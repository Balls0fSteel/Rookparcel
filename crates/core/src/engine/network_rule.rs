// Rookparcel's own code, not ported (D54). The binding itself follows libtorrent 2.0.11's
// `session_impl::bind_outgoing_socket` (src/session_impl.cpp): an outgoing socket is bound to
// the address of a listen socket of its family before it connects, and a connection with no
// such address fails. libtorrent takes the addresses from `outgoing_interfaces`; here a rule
// picks them from the interfaces at every network change, and the listen sockets follow.

//! "Use only this network": the rule evaluated against the interfaces, the engine's binding,
//! and the hold on running torrents while the network is missing. The frontend's hold on the
//! transfers (D70) uses the same machinery: bound to nothing, running torrents held.

use std::io;
use std::net::{IpAddr, SocketAddr};

use super::Engine;
use crate::alert::Alert;
use crate::net::bind::{self, Binding};
use crate::net::enum_net::IpInterface;
use crate::settings::NetworkRule;
use crate::status::{NetAdapter, NetworkStatus};

/// The rule's state in the engine.
#[derive(Default)]
pub(super) struct RuleState {
    /// The addresses torrent traffic is bound to; `None` without a rule. Empty when the rule
    /// matches nothing.
    pub binding: Option<Binding>,
    pub status: NetworkStatus,
    /// Running torrents are held until the network comes back or the transfers are let go.
    pub hold: bool,
    /// The status `hold` was last set for, so the log says why it ends.
    hold_for: NetworkStatus,
    /// The frontend holds the transfers (D70, [`crate::SessionHandle::hold_transfers`]):
    /// bound to nothing whatever the rule says.
    pub held: bool,
    /// The adapters last sent in [`Alert::NetworkAdapters`].
    pub adapters: Option<Vec<NetAdapter>>,
}

/// How the rule reads in the log.
fn describe(rule: &str) -> String {
    match NetworkRule::parse(rule) {
        Ok(NetworkRule::Vpn) => "\"vpn\" (any VPN adapter)".into(),
        Ok(NetworkRule::Adapter(id)) => format!("adapter {id}"),
        Ok(NetworkRule::Range(a, b)) => format!("range {a}/{b}"),
        Ok(NetworkRule::Off) => "off".into(),
        Err(_) => format!("\"{rule}\""),
    }
}

impl Engine {
    /// Evaluates `network_rule` against `ifs` and returns the `listen_interfaces` the listen
    /// sockets are opened for: the configured ones without a rule, the matched addresses on
    /// the configured port with one, none when it matches nothing or the transfers are held
    /// (D70). The adapters are sent to the frontends when they changed.
    pub(super) fn evaluate_network_rule(&mut self, ifs: &[IpInterface]) -> String {
        self.send_network_adapters(bind::adapters(ifs), false);
        let rule_text = self.settings.network_rule.clone();
        let (binding, status) = if self.rules.held {
            // held transfers bind to nothing, whatever the rule says (D70)
            (Some(Binding::default()), NetworkStatus::Held)
        } else {
            let binding = match NetworkRule::parse(&rule_text) {
                Ok(rule) => bind::match_rule(&rule, ifs),
                Err(e) => {
                    // a rule that cannot be read binds to nothing: torrent traffic stays off
                    // rather than leaving through any network
                    if self.rules.status == NetworkStatus::Held
                        || self.rules.binding.as_ref().is_none_or(|b| !b.is_empty())
                    {
                        self.alert(Alert::Log { id: None, message: format!("network rule: {e}") });
                    }
                    Some(Binding::default())
                }
            };
            let status = match &binding {
                None => NetworkStatus::Any,
                Some(b) if !b.is_empty() => NetworkStatus::Bound,
                Some(_) if self.settings.network_pause_when_missing => NetworkStatus::Waiting,
                Some(_) => NetworkStatus::Missing,
            };
            (binding, status)
        };
        if binding != self.rules.binding || status != self.rules.status {
            let rule = describe(&rule_text);
            let message = match (&binding, status) {
                (None, _) => "network rule off: torrent traffic may use any network".to_owned(),
                (Some(_), NetworkStatus::Held) => {
                    "transfers held: torrent traffic bound to nothing, nothing listens".to_owned()
                }
                (Some(b), NetworkStatus::Bound) => {
                    let addrs: Vec<String> = b.v4.iter().chain(&b.v6).map(IpAddr::to_string).collect();
                    let v6 = if b.v6.is_none() { "; no IPv6 address there, so no IPv6" } else { "" };
                    format!(
                        "network rule {rule}: torrent traffic bound to {} on {}{v6}",
                        addrs.join(", "),
                        b.adapters.join(", ")
                    )
                }
                (Some(_), NetworkStatus::Waiting) => {
                    format!("network rule {rule} matches no network adapter: torrents wait for the network")
                }
                (Some(_), _) => {
                    format!("network rule {rule} matches no network adapter: no torrent traffic")
                }
            };
            self.alert(Alert::Log { id: None, message });
        }
        let listen = match &binding {
            None => self.settings.listen_interfaces.clone(),
            Some(b) => bind::bound_listen_interfaces(&self.settings.listen_interfaces, b),
        };
        self.rules.binding = binding;
        self.rules.status = status;
        listen
    }

    /// After the listen sockets followed the rule: the trackers' sockets follow, and running
    /// torrents are held or let go.
    pub(super) fn network_rule_applied(&mut self) {
        self.trackers.set_bind(self.rules.binding.clone(), self.poll.registry());
        self.set_network_hold();
    }

    /// `network_pause_when_missing` changed: the hold follows (nothing to reopen). Held
    /// transfers stay held (D70).
    pub(super) fn network_pause_setting_changed(&mut self) {
        if let Some(b) = &self.rules.binding {
            if b.is_empty() && !self.rules.held {
                self.rules.status = if self.settings.network_pause_when_missing {
                    NetworkStatus::Waiting
                } else {
                    NetworkStatus::Missing
                };
            }
        }
        self.set_network_hold();
    }

    /// Running torrents are held while the rule waits for the network or the transfers are
    /// held (D70), and run again otherwise.
    fn set_network_hold(&mut self) {
        let status = self.rules.status;
        let hold = matches!(status, NetworkStatus::Waiting | NetworkStatus::Held);
        let was_for = std::mem::replace(&mut self.rules.hold_for, status);
        if self.rules.hold == hold {
            return;
        }
        self.rules.hold = hold;
        // the transfers' hold has its own message (`set_held`)
        let message = match (hold, if hold { status } else { was_for }) {
            (true, NetworkStatus::Held) => None,
            (true, _) => Some("waiting for the network: running torrents are held"),
            (false, NetworkStatus::Held) => Some("held torrents run again"),
            (false, _) => Some("the network is back: held torrents run again"),
        };
        if let Some(message) = message {
            self.alert(Alert::Log { id: None, message: message.into() });
        }
        for id in self.torrent_ids() {
            if let Some(t) = self.torrent(id) {
                t.status_dirty = true;
            }
            self.update_running(id);
        }
    }

    /// [`crate::SessionHandle::hold_transfers`] (D70): the binding follows as on a change of
    /// the rule, so the listen sockets close or open again.
    pub(super) fn hold_transfers(&mut self, held: bool) {
        if self.rules.held == held {
            return;
        }
        self.set_held(held);
        self.on_listen_settings_changed(false);
    }

    /// Sets the transfers' hold without reopening anything: at startup before the first
    /// listen sockets (`SessionParams::hold_transfers`), or before a reopen.
    pub(super) fn set_held(&mut self, held: bool) {
        self.rules.held = held;
        let message = if held {
            "transfers held until the settings are confirmed (D70)"
        } else {
            "the settings hold is lifted"
        };
        self.alert(Alert::Log { id: None, message: message.into() });
    }

    /// Torrents do not run: the session is paused, waits for the network or is held (D70).
    pub(super) fn session_halted(&self) -> bool {
        self.session_paused || self.rules.hold
    }

    /// The local address an outgoing connection to `remote` binds to (see
    /// [`bind::outgoing_from`]).
    pub(super) fn outgoing_from(&self, remote: SocketAddr) -> io::Result<Option<IpAddr>> {
        bind::outgoing_from(self.rules.binding.as_ref(), remote)
    }

    /// Answers `post_network_adapters` with the interfaces as they are now.
    pub(super) fn post_network_adapters(&mut self) {
        match self.net_interfaces() {
            Ok(ifs) => self.send_network_adapters(bind::adapters(&ifs), true),
            Err(e) => {
                self.alert(Alert::Log {
                    id: None,
                    message: format!("cannot list the network interfaces: {e}"),
                });
                let last = self.rules.adapters.clone().unwrap_or_default();
                self.send_network_adapters(last, true);
            }
        }
    }

    fn send_network_adapters(&mut self, adapters: Vec<NetAdapter>, always: bool) {
        if !always && self.rules.adapters.as_ref() == Some(&adapters) {
            return;
        }
        self.rules.adapters = Some(adapters.clone());
        self.alert(Alert::NetworkAdapters(adapters));
    }
}

/// A real session bound by a rule to a made-up adapter on 127.0.0.2 (the fake notifier,
/// `SessionHandle::set_fake_network`, makes it come and go); peers, trackers and DHT nodes
/// listen on 127.0.0.1 and see where the traffic comes from.
#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
    use std::sync::mpsc;
    use std::time::{Duration, Instant};

    use crate::alert::{Alert, Protocol};
    use crate::bencode::Entry;
    use crate::crypto::Sha1;
    use crate::net::enum_net::{IpInterface, build_netmask};
    use crate::params::{AddTorrentParams, TorrentFlags, TorrentSource};
    use crate::session::{Session, SessionHandle, SessionParams};
    use crate::settings::Settings;
    use crate::status::{NetworkStatus, SessionStats, TorrentStatus};
    use crate::types::TorrentId;

    const BOUND: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2));

    /// The made-up tunnel adapter, with the bound address.
    fn tunnel() -> IpInterface {
        IpInterface {
            addr: BOUND,
            netmask: build_netmask(32, true),
            scope_id: 0,
            index: 77,
            name: "wg-test".into(),
            friendly: "Test Tunnel".into(),
            description: String::new(),
            if_type: 0,
            up: true,
            loopback: false,
            multicast: false,
            pointopoint: true,
            preferred: true,
        }
    }

    /// Whether this host can bind 127.0.0.2 (Windows and Linux route all of 127/8 to
    /// loopback; macOS only 127.0.0.1 unless an alias is added).
    fn can_bind_bound() -> bool {
        UdpSocket::bind(SocketAddr::new(BOUND, 0)).is_ok()
    }

    fn start(dht_node: Option<SocketAddr>) -> Session {
        let settings = Settings {
            listen_interfaces: "0.0.0.0:0,[::]:0".into(),
            network_rule: "range:127.0.0.2/32".into(),
            tick_interval: 100,
            aio_threads: 2,
            enable_dht: dht_node.is_some(),
            alert_mask: u32::MAX,
            dht_bootstrap_nodes: dht_node.map(|a| a.to_string()).unwrap_or_default(),
            enable_lsd: false,
            enable_upnp: false,
            enable_natpmp: false,
            ..Settings::default()
        };
        Session::start(SessionParams { settings, state_dir: None, ..Default::default() })
            .expect("session starts")
    }

    /// Takes alerts until `f` returns `Some`, failing after `secs`.
    fn wait<T>(h: &SessionHandle, secs: u64, what: &str, mut f: impl FnMut(&Alert) -> Option<T>) -> T {
        let deadline = Instant::now() + Duration::from_secs(secs);
        let mut alerts = Vec::new();
        let mut seen = Vec::new();
        loop {
            h.pop_alerts(&mut alerts);
            for a in alerts.drain(..) {
                if let Some(v) = f(&a) {
                    return v;
                }
                if !matches!(a, Alert::StateUpdate(_) | Alert::SessionStats(_)) {
                    seen.push(a.message());
                }
            }
            if Instant::now() >= deadline {
                panic!("timed out waiting for {what}; alerts: {seen:#?}");
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    /// The torrent's status, polled until `f` holds.
    fn status_until(h: &SessionHandle, id: TorrentId, what: &str, f: impl Fn(&TorrentStatus) -> bool) {
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut last = None;
        loop {
            h.post_torrent_updates();
            let mut alerts = Vec::new();
            std::thread::sleep(Duration::from_millis(50));
            h.pop_alerts(&mut alerts);
            for a in alerts {
                if let Alert::StateUpdate(v) = a {
                    if let Some(st) = v.into_iter().find(|s| s.id == id) {
                        last = Some(st);
                    }
                }
            }
            if last.as_ref().is_some_and(&f) {
                return;
            }
            if Instant::now() >= deadline {
                panic!("timed out waiting for {what}; last status: {last:?}");
            }
        }
    }

    fn network_status(h: &SessionHandle) -> NetworkStatus {
        h.post_session_stats();
        wait(h, 5, "session stats", |a| match a {
            Alert::SessionStats(s) => Some(s.network),
            _ => None,
        })
    }

    fn session_stats(h: &SessionHandle) -> SessionStats {
        h.post_session_stats();
        wait(h, 5, "session stats", |a| match a {
            Alert::SessionStats(s) => Some(*s),
            _ => None,
        })
    }

    /// Polls the network status until it is `want`.
    fn network_status_until(h: &SessionHandle, want: NetworkStatus) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let st = network_status(h);
            if st == want {
                return;
            }
            assert!(Instant::now() < deadline, "network status {st:?}, not {want:?}");
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    /// Fails on any alert `bad` matches for `ms` milliseconds.
    fn none_for(h: &SessionHandle, ms: u64, what: &str, bad: impl Fn(&Alert) -> bool) {
        let deadline = Instant::now() + Duration::from_millis(ms);
        let mut alerts = Vec::new();
        while Instant::now() < deadline {
            h.pop_alerts(&mut alerts);
            if let Some(a) = alerts.drain(..).find(&bad) {
                panic!("{what}: {a:?}");
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn listening(a: &Alert) -> bool {
        matches!(a, Alert::ListenSucceeded { .. })
    }

    fn tcp_listening(a: &Alert) -> Option<SocketAddr> {
        match a {
            Alert::ListenSucceeded { address, protocol: Protocol::Tcp } => Some(*address),
            _ => None,
        }
    }

    fn logged(text: &str) -> impl Fn(&Alert) -> Option<()> + '_ {
        move |a| matches!(a, Alert::Log { message, .. } if message.contains(text)).then_some(())
    }

    /// No rule, listening on 127.0.0.1 only.
    fn unbound(listen: &str) -> Settings {
        Settings {
            listen_interfaces: listen.into(),
            tick_interval: 100,
            aio_threads: 2,
            enable_dht: false,
            alert_mask: u32::MAX,
            enable_lsd: false,
            enable_upnp: false,
            enable_natpmp: false,
            ..Settings::default()
        }
    }

    /// An HTTP tracker on 127.0.0.1 that reports where each announce comes from.
    fn http_tracker() -> (String, mpsc::Receiver<SocketAddr>) {
        let tracker = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/announce", tracker.local_addr().unwrap());
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            for c in tracker.incoming() {
                let Ok(mut c) = c else { return };
                let from = c.peer_addr().unwrap();
                let mut buf = vec![0u8; 4096];
                let _ = c.read(&mut buf);
                let body = b"d8:intervali1800e5:peers0:e";
                let head =
                    format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
                let _ = c.write_all(head.as_bytes());
                let _ = c.write_all(body);
                if tx.send(from).is_err() {
                    return;
                }
            }
        });
        (url, rx)
    }

    /// A DHT node on 127.0.0.1 that reports where the first packet comes from.
    fn dht_node() -> (SocketAddr, mpsc::Receiver<SocketAddr>) {
        let node = UdpSocket::bind("127.0.0.1:0").unwrap();
        node.set_read_timeout(Some(Duration::from_secs(60))).unwrap();
        let addr = node.local_addr().unwrap();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let mut buf = [0u8; 1500];
            if let Ok((_, from)) = node.recv_from(&mut buf) {
                let _ = tx.send(from);
            }
        });
        (addr, rx)
    }

    fn torrent_file(announce: &str) -> Vec<u8> {
        let data = vec![9u8; 16 * 1024];
        let mut info = Entry::new_dict();
        *info.key("length") = Entry::Int(data.len() as i64);
        *info.key("name") = Entry::Bytes(b"network-rule-test.bin".to_vec());
        *info.key("piece length") = Entry::Int(16 * 1024);
        *info.key("pieces") = Entry::Bytes(Sha1::digest(&data).0.to_vec());
        let mut t = Entry::new_dict();
        *t.key("announce") = Entry::Bytes(announce.as_bytes().to_vec());
        *t.key("info") = info;
        t.bencode()
    }

    fn add(h: &SessionHandle, announce: &str, dir: &std::path::Path) -> TorrentId {
        let mut p =
            AddTorrentParams::new(TorrentSource::TorrentFile(torrent_file(announce)), dir.to_string_lossy());
        p.flags = TorrentFlags::empty();
        h.add_torrent(p);
        wait(h, 15, "the torrent", |a| match a {
            Alert::TorrentAdded { id, .. } => Some(*id),
            _ => None,
        })
    }

    #[test]
    fn bound_traffic_leaves_from_the_matched_address() {
        if !can_bind_bound() {
            eprintln!("skipped: 127.0.0.2 cannot be bound here");
            return;
        }
        // a DHT node, an HTTP tracker and a peer on 127.0.0.1, each reporting its first sender
        let (node_addr, dht_rx) = dht_node();
        let (url, tr_rx) = http_tracker();
        let peer = TcpListener::bind("127.0.0.1:0").unwrap();
        let peer_addr = peer.local_addr().unwrap();
        let (peer_tx, peer_rx) = mpsc::channel();
        std::thread::spawn(move || {
            if let Ok((_c, from)) = peer.accept() {
                let _ = peer_tx.send(from);
                std::thread::sleep(Duration::from_secs(1));
            }
        });

        let s = start(Some(node_addr));
        let h = s.handle();
        // no adapter has 127.0.0.2 yet: nothing listens and torrents wait
        wait(&h, 5, "the rule finding nothing", |a| match a {
            Alert::Log { message, .. } if message.contains("matches no network adapter") => Some(()),
            Alert::ListenSucceeded { address, .. } => panic!("listening unbound on {address}"),
            _ => None,
        });
        assert_eq!(network_status(&h), NetworkStatus::Waiting);
        let dir = std::env::temp_dir().join(format!("lt-netrule-bound-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let id = add(&h, &url, &dir);
        status_until(&h, id, "waiting for the network", |st| st.waiting_for_network);
        assert!(tr_rx.recv_timeout(Duration::from_millis(500)).is_err(), "no announce while waiting");

        // the tunnel comes up
        let lo = crate::net::sys::interfaces()
            .unwrap()
            .into_iter()
            .filter(|i| i.addr.is_loopback())
            .collect::<Vec<_>>();
        let mut ifs = lo.clone();
        ifs.push(tunnel());
        h.set_fake_network(Some((ifs, Vec::new())));
        let listen = wait(&h, 5, "the bound listen socket", |a| match a {
            Alert::ListenSucceeded { address, .. } => Some(*address),
            _ => None,
        });
        assert_eq!(listen.ip(), BOUND);
        assert_eq!(network_status(&h), NetworkStatus::Bound);
        status_until(&h, id, "running again", |st| !st.waiting_for_network);

        let from = tr_rx.recv_timeout(Duration::from_secs(15)).expect("an announce");
        assert_eq!(from.ip(), BOUND, "the tracker request leaves from the bound address");
        h.torrent(id).connect_peer(peer_addr);
        let from = peer_rx.recv_timeout(Duration::from_secs(10)).expect("a peer connection");
        assert_eq!(from.ip(), BOUND, "the peer connection leaves from the bound address");
        let from = dht_rx.recv_timeout(Duration::from_secs(20)).expect("a DHT packet");
        assert_eq!(from, SocketAddr::new(BOUND, listen.port()), "the DHT speaks from the bound socket");

        // an IPv6 peer: the binding has no IPv6 address, so it is not reached
        h.torrent(id).connect_peer("[::1]:9".parse().unwrap());
        let reason = wait(&h, 5, "the IPv6 peer refused", |a| match a {
            Alert::PeerDisconnected { addr, reason, .. } if addr.is_ipv6() => Some(reason.clone()),
            _ => None,
        });
        assert!(reason.contains("no address of this family"), "{reason}");
        s.shutdown();
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn torrents_wait_for_the_network_and_the_user_keeps_control() {
        let s = start(None);
        let h = s.handle();
        let dir = std::env::temp_dir().join(format!("lt-netrule-wait-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let id = add(&h, "http://127.0.0.1:9/announce", &dir);
        status_until(&h, id, "waiting", |st| st.waiting_for_network);

        // the user stops it while waiting: stopped, not waiting
        h.torrent(id).pause();
        status_until(&h, id, "stopped", |st| {
            st.flags.contains(TorrentFlags::PAUSED) && !st.waiting_for_network
        });
        // and starts it again: waiting again, not running
        h.torrent(id).resume();
        status_until(&h, id, "waiting again", |st| {
            !st.flags.contains(TorrentFlags::PAUSED) && st.waiting_for_network
        });

        // the network comes: it runs by itself
        h.set_fake_network(Some((vec![tunnel()], Vec::new())));
        status_until(&h, id, "running", |st| {
            !st.waiting_for_network && !st.flags.contains(TorrentFlags::PAUSED)
        });
        assert_eq!(network_status(&h), NetworkStatus::Bound);

        // it goes: held again, and the log names the rule
        h.set_fake_network(Some((Vec::new(), Vec::new())));
        wait(&h, 5, "the log naming the rule", |a| match a {
            Alert::Log { message, .. }
                if message.contains("range 127.0.0.2/32 matches no network adapter") =>
            {
                Some(())
            }
            _ => None,
        });
        status_until(&h, id, "held", |st| st.waiting_for_network);

        // without "pause while missing": nothing is held (and nothing is bound either)
        let mut settings = Settings {
            listen_interfaces: "0.0.0.0:0,[::]:0".into(),
            network_rule: "range:127.0.0.2/32".into(),
            tick_interval: 100,
            aio_threads: 2,
            enable_dht: false,
            enable_lsd: false,
            enable_upnp: false,
            enable_natpmp: false,
            ..Settings::default()
        };
        settings.network_pause_when_missing = false;
        h.apply_settings(settings.clone());
        status_until(&h, id, "not held", |st| !st.waiting_for_network);
        assert_eq!(network_status(&h), NetworkStatus::Missing);

        // the rule off: any network
        settings.network_rule.clear();
        h.apply_settings(settings);
        let st = (0..50)
            .map(|_| {
                std::thread::sleep(Duration::from_millis(50));
                network_status(&h)
            })
            .find(|s| *s == NetworkStatus::Any);
        assert_eq!(st, Some(NetworkStatus::Any));

        // the adapters are listed on request
        h.post_network_adapters();
        let adapters = wait(&h, 5, "the adapters", |a| match a {
            Alert::NetworkAdapters(v) => Some(v.clone()),
            _ => None,
        });
        assert!(adapters.is_empty(), "the fake network has none: {adapters:?}");
        s.shutdown();
        let _ = std::fs::remove_dir_all(dir);
    }

    /// D70: without a rule, held transfers close the listen sockets and hold the running
    /// torrent; settings and network changes keep the hold, and letting go restores it all.
    #[test]
    fn held_transfers_bind_nothing_until_let_go() {
        let (url, announces) = http_tracker();
        let params =
            SessionParams { settings: unbound("127.0.0.1:0"), state_dir: None, ..Default::default() };
        let s = Session::start(params).expect("session starts");
        let h = s.handle();
        let listen = wait(&h, 5, "the listen socket", tcp_listening);
        let dir = std::env::temp_dir().join(format!("lt-held-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let id = add(&h, &url, &dir);
        announces.recv_timeout(Duration::from_secs(15)).expect("an announce before the hold");
        assert_eq!(session_stats(&h).num_downloading, 1);

        // held: nothing listens, the torrent waits and says so, and nothing is announced
        h.hold_transfers(true);
        wait(&h, 5, "the hold", logged("transfers held until the settings are confirmed"));
        status_until(&h, id, "held", |st| st.waiting_for_network);
        assert_eq!(network_status(&h), NetworkStatus::Held);
        assert_eq!(session_stats(&h).num_downloading, 0, "the held torrent is not active");
        assert!(
            TcpStream::connect_timeout(&listen, Duration::from_secs(3)).is_err(),
            "the listen port is closed"
        );
        assert!(announces.recv_timeout(Duration::from_millis(500)).is_err(), "no announce while held");

        // new listen interfaces and "pause while missing" off: still held, nothing listens
        let changed = Settings { network_pause_when_missing: false, ..unbound("127.0.0.1:0,127.0.0.3:0") };
        h.apply_settings(changed);
        none_for(&h, 500, "a listen socket while held", listening);
        assert_eq!(network_status(&h), NetworkStatus::Held);
        assert_eq!(session_stats(&h).num_downloading, 0, "the torrent is still held");

        // a network change (forced): still held
        h.reopen_network_sockets();
        wait(&h, 5, "the network change", |a| {
            assert!(!listening(a), "a listen socket while held: {a:?}");
            logged("network change: reopening")(a)
        });
        none_for(&h, 300, "a listen socket while held", listening);
        assert_eq!(network_status(&h), NetworkStatus::Held);

        // let go: listening again, any network, the torrent runs and announces
        h.hold_transfers(false);
        let again = wait(&h, 5, "listening again", |a| tcp_listening(a).filter(|x| x.ip() == listen.ip()));
        drop(TcpStream::connect(again).expect("the listen port is open again"));
        assert_eq!(network_status(&h), NetworkStatus::Any);
        status_until(&h, id, "running again", |st| !st.waiting_for_network);
        announces.recv_timeout(Duration::from_secs(15)).expect("an announce once let go");
        assert_eq!(session_stats(&h).num_downloading, 1);
        s.shutdown();
        let _ = std::fs::remove_dir_all(dir);
    }

    /// D70 over a rule: held whatever the rule matches; let go, the rule's own status returns.
    #[test]
    fn held_transfers_override_the_rule() {
        let s = start(None);
        let h = s.handle();
        let dir = std::env::temp_dir().join(format!("lt-held-rule-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let id = add(&h, "http://127.0.0.1:9/announce", &dir);
        status_until(&h, id, "waiting", |st| st.waiting_for_network);
        assert_eq!(network_status(&h), NetworkStatus::Waiting);

        // the rule matches nothing: held, then waiting again
        h.hold_transfers(true);
        network_status_until(&h, NetworkStatus::Held);
        h.hold_transfers(false);
        network_status_until(&h, NetworkStatus::Waiting);
        assert_eq!(session_stats(&h).num_downloading, 0, "the torrent still waits");

        // the tunnel comes: bound, the torrent runs
        let bindable = can_bind_bound();
        h.set_fake_network(Some((vec![tunnel()], Vec::new())));
        let bound = bindable.then(|| wait(&h, 5, "the bound listen socket", tcp_listening));
        network_status_until(&h, NetworkStatus::Bound);
        status_until(&h, id, "running", |st| !st.waiting_for_network);

        // held over the matching rule: nothing listens, the torrent waits
        h.hold_transfers(true);
        network_status_until(&h, NetworkStatus::Held);
        status_until(&h, id, "held", |st| st.waiting_for_network);
        if let Some(a) = bound {
            assert!(TcpStream::connect_timeout(&a, Duration::from_secs(3)).is_err(), "{a} is closed");
        }
        // the tunnel goes and comes back: still held
        h.set_fake_network(Some((Vec::new(), Vec::new())));
        wait(&h, 5, "the tunnel gone", logged("network change: reopening"));
        assert_eq!(network_status(&h), NetworkStatus::Held);
        h.set_fake_network(Some((vec![tunnel()], Vec::new())));
        wait(&h, 5, "the tunnel back", |a| {
            assert!(!listening(a), "a listen socket while held: {a:?}");
            logged("network change: reopening")(a)
        });
        none_for(&h, 300, "a listen socket while held", listening);
        assert_eq!(network_status(&h), NetworkStatus::Held);

        // let go: bound again
        h.hold_transfers(false);
        if bindable {
            let a = wait(&h, 5, "the bound listen socket again", tcp_listening);
            assert_eq!(a.ip(), BOUND);
        }
        network_status_until(&h, NetworkStatus::Bound);
        status_until(&h, id, "running again", |st| !st.waiting_for_network);
        s.shutdown();
        let _ = std::fs::remove_dir_all(dir);
    }

    /// D70 from the start (`SessionParams::hold_transfers`): a restored torrent never runs and
    /// nothing listens, announces or speaks to the DHT until the hold is lifted.
    #[test]
    fn a_session_started_held_runs_nothing_until_let_go() {
        let (url, announces) = http_tracker();
        let (node, dht_rx) = dht_node();
        let base = std::env::temp_dir().join(format!("lt-held-start-{}", std::process::id()));
        let (dir, state) = (base.join("data"), base.join("state"));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::create_dir_all(&state).unwrap();
        let settings =
            Settings { enable_dht: true, dht_bootstrap_nodes: node.to_string(), ..unbound("127.0.0.1:0") };
        let params = SessionParams {
            settings,
            state_dir: Some(state.clone()),
            hold_transfers: true,
            ..Default::default()
        };

        // the torrent goes into the state (added while held too)
        let s = Session::start(params.clone()).expect("session starts");
        add(&s.handle(), &url, &dir);
        s.shutdown();

        let s = Session::start(params).expect("session starts again");
        let h = s.handle();
        let count = wait(&h, 10, "the restored torrent", |a| match a {
            Alert::TorrentsRestored { count, .. } => Some(*count),
            Alert::ListenSucceeded { address, .. } => panic!("listening on {address} while held"),
            _ => None,
        });
        assert_eq!(count, 1);
        h.post_torrent_updates();
        let st = wait(&h, 5, "its status", |a| match a {
            Alert::StateUpdate(v) => v.first().cloned(),
            _ => None,
        });
        assert!(st.waiting_for_network && !st.flags.contains(TorrentFlags::PAUSED), "{st:?}");
        let id = st.id;
        assert_eq!(network_status(&h), NetworkStatus::Held);
        assert_eq!(session_stats(&h).num_downloading, 0, "the held torrent is not active");
        none_for(&h, 1000, "a listen socket while held", listening);
        assert!(announces.recv_timeout(Duration::from_millis(200)).is_err(), "no announce while held");
        assert!(dht_rx.recv_timeout(Duration::from_millis(200)).is_err(), "no DHT packet while held");

        // let go: it listens, runs, announces and bootstraps the DHT
        h.hold_transfers(false);
        let listen = wait(&h, 5, "the listen socket", tcp_listening);
        assert_eq!(listen.ip(), IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(network_status(&h), NetworkStatus::Any);
        status_until(&h, id, "running", |st| !st.waiting_for_network);
        announces.recv_timeout(Duration::from_secs(15)).expect("an announce once let go");
        dht_rx.recv_timeout(Duration::from_secs(20)).expect("a DHT packet once let go");
        s.shutdown();
        let _ = std::fs::remove_dir_all(base);
    }
}
