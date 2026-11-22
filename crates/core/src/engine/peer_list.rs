// Ported from libtorrent-rasterbar 2.0.11: src/peer_list.cpp, src/torrent_peer.cpp
// (peer_priority / BEP 40, the trust points and hash-fail fields)
// Copyright (c) 2003-2020, Arvid Norberg and contributors. BSD-3-Clause, see THIRD-PARTY-NOTICES.md.
// Changes: a Vec plus an address index instead of a sorted vector of pooled `torrent_peer`
// pointers. Connect candidates are chosen by a scan with libtorrent's ordering (fewest failures,
// then BEP 40 rank); the list is capped at `max_peerlist_size` by libtorrent's erase-candidate
// rule, except that a peer banned as ourselves is kept. Entries are per address (ip:port); the
// IPs of peers banned for bad data are kept in a set of their own, which refuses their
// connections, and their entries are the first a full list gives up.
// Peers restored from resume data stay in their compact saved bytes until the torrent starts.

//! Every peer a torrent knows about, connected or not.

use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};

use super::smart_ban::Trust;
use crate::crypto::crc32c::Crc32c;
use crate::net::ip_voter::ExternalIp;
use crate::resume::SavedPeers;
use crate::status::PeerSource;

/// When an unconnected peer may be tried (again): libtorrent's `max_failcount` and
/// `min_reconnect_time` settings.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Reconnect {
    /// Peers that failed this often are no longer tried.
    pub max_failcount: u8,
    /// How long after its last connection closed a peer waits, times its fail count plus one.
    pub min_reconnect_time: Duration,
}

impl Reconnect {
    pub fn from_settings(s: &crate::Settings) -> Reconnect {
        Reconnect {
            max_failcount: s.max_failcount.clamp(0, i32::from(FAILCOUNT_CAP)) as u8,
            min_reconnect_time: Duration::from_secs(u64::from(s.min_reconnect_time)),
        }
    }
}

impl Default for Reconnect {
    /// libtorrent's defaults: 3 failures, 60 s.
    fn default() -> Reconnect {
        Reconnect { max_failcount: 3, min_reconnect_time: Duration::from_secs(60) }
    }
}

/// libtorrent's `torrent_peer::failcount` is five bits.
const FAILCOUNT_CAP: u8 = 31;

#[derive(Debug)]
pub(crate) struct KnownPeer {
    pub addr: SocketAddr,
    pub source: PeerSource,
    pub failcount: u8,
    /// Connection id while connected.
    pub connection: Option<u64>,
    pub last_connected: Option<Instant>,
    pub seed: bool,
    pub banned: bool,
    /// Hash failures, trust points and parole (libtorrent `torrent_peer::trust_points`,
    /// `hashfails`, `on_parole`).
    pub trust: Trust,
    /// Try an encrypted handshake first (libtorrent `torrent_peer::pe_support`): set by a PEX
    /// `e` flag or a completed encrypted handshake, toggled as `EncPolicy::Enabled` alternates.
    pub pe_support: bool,
    /// We think this peer supports uTP (libtorrent `torrent_peer::supports_utp`): assumed at
    /// first, set by a PEX `utp` flag, cleared when a uTP connect fails or succeeds (then
    /// `confirmed_supports_utp` says so).
    pub supports_utp: bool,
    /// We have been connected to this peer over uTP at least once.
    pub confirmed_supports_utp: bool,
    /// Learned under a hybrid's v2 hash (a v2 tracker announce, the DHT, PEX `lt_v2`): we
    /// connect with that hash (libtorrent `torrent_peer::protocol_v2`).
    pub protocol_v2: bool,
    /// BEP 40 rank against our external address (higher connects first).
    rank: u32,
}

/// A torrent's known peers (libtorrent `peer_list`). Most torrents of a large library are
/// stopped, so the list takes the least room it can for what it holds:
///
/// * `Saved`: only the peers restored from resume data, kept as their compact bytes (6 or 18
///   each, as saved) until the torrent starts: a torrent that ran saves up to 100, which as
///   `KnownPeer`s and index entries take about 15 KB. They read as the list they stand for
///   (fresh peers from resume data: no failures, not connected, not seeds), so the status,
///   the resume data and the filters see no difference; anything that changes a peer makes the
///   list `Live` first. No peers at all is an empty `Saved`, which allocates nothing
///   (libtorrent makes `torrent::m_peer_list` in `need_peer_list`, with the first peer).
/// * `Live`: the full list, boxed.
///
/// 16 bytes (a third, `Empty` variant would make it 24).
pub(crate) enum PeerList {
    Saved(SavedPeers),
    Live(Box<Peers>),
}

impl Default for PeerList {
    fn default() -> PeerList {
        PeerList::Saved(SavedPeers::default())
    }
}

impl PeerList {
    fn live(&self) -> Option<&Peers> {
        match self {
            PeerList::Live(l) => Some(l),
            _ => None,
        }
    }

    /// The full list, made on first use; saved peers are expanded into it (unranked, as
    /// `clear_peer_prio` would rank them).
    fn live_mut(&mut self) -> &mut Peers {
        match self {
            PeerList::Saved(s) if s.is_empty() => *self = PeerList::Live(Box::default()),
            PeerList::Saved(_) => self.expand(&ExternalIp::default()),
            PeerList::Live(_) => {}
        }
        match self {
            PeerList::Live(l) => l,
            _ => unreachable!("expanded above"),
        }
    }

    /// The torrent starts: saved peers become the full list, ranked against our external
    /// address (libtorrent computes `torrent_peer::rank` on first use).
    pub fn expand(&mut self, external_ip: &ExternalIp) {
        if let PeerList::Saved(saved) = self {
            if saved.is_empty() {
                return;
            }
            let saved = std::mem::take(saved);
            let mut l = Box::<Peers>::default();
            l.peers.reserve_exact(saved.len());
            l.index.reserve(saved.len());
            for addr in saved.iter() {
                l.index.insert(addr, l.peers.len());
                l.peers.push(KnownPeer::new(addr, PeerSource::RESUME_DATA, rank(external_ip, addr)));
            }
            *self = PeerList::Live(l);
        }
    }

    /// Peers from resume data (libtorrent `add_torrent_params::peers`, added with
    /// `peer_info::resume_data` as the source). Into an empty list they go as they are.
    pub fn restore(&mut self, saved: SavedPeers) {
        if saved.is_empty() {
            return;
        }
        match self {
            PeerList::Saved(s) if s.is_empty() => *s = saved,
            _ => {
                for addr in saved.iter() {
                    self.add(addr, PeerSource::RESUME_DATA, ExternalIp::default(), usize::MAX);
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            PeerList::Saved(s) => s.len(),
            PeerList::Live(l) => l.peers.len(),
        }
    }

    pub fn num_seeds(&self) -> usize {
        self.live().map_or(0, Peers::num_seeds)
    }

    /// The peer at `addr`. Saved peers are not found until the list is expanded (the torrent
    /// starting does that): nothing asks for a peer of a stopped torrent.
    pub fn get(&self, addr: &SocketAddr) -> Option<&KnownPeer> {
        self.live()?.get(addr)
    }

    pub fn get_mut(&mut self, addr: &SocketAddr) -> Option<&mut KnownPeer> {
        match self {
            PeerList::Saved(s) if !s.iter().any(|a| a == *addr) => None,
            _ => self.live_mut().get_mut(addr),
        }
    }

    /// libtorrent `peer_list::add_peer`. Returns false if the peer was rejected (full list,
    /// invalid address). A full list makes room as [`add_capped`](Self::add_capped) does for
    /// a torrent that is not finished, under the default reconnect policy.
    pub fn add(
        &mut self,
        addr: SocketAddr,
        source: PeerSource,
        external_ip: ExternalIp,
        max_size: usize,
    ) -> bool {
        self.add_capped(addr, source, external_ip, max_size, false, Reconnect::default())
    }

    /// libtorrent `peer_list::add_peer` into a list of at most `max_size` peers. A full list
    /// gives up a peer that is no connect candidate (`finished` and `policy` say which ones
    /// are) and has failed or only came from resume data (libtorrent `erase_peers` and
    /// `is_erase_candidate`); without one the new peer is rejected. Banned peers are kept.
    pub fn add_capped(
        &mut self,
        addr: SocketAddr,
        source: PeerSource,
        external_ip: ExternalIp,
        max_size: usize,
        finished: bool,
        policy: Reconnect,
    ) -> bool {
        if !SavedPeers::usable(&addr) {
            return false;
        }
        if matches!(self, PeerList::Saved(_)) {
            self.expand(&external_ip);
        }
        let added = self.live_mut().add(addr, source, external_ip, max_size, finished, policy);
        self.drop_if_empty();
        added
    }

    pub fn set_protocol_v2(&mut self, addr: &SocketAddr) {
        if let Some(p) = self.get_mut(addr) {
            p.protocol_v2 = true;
        }
    }

    /// Forgets every peer `f` picks (libtorrent `apply_ip_filter` / `apply_port_filter`).
    /// Returns how many went. A connected peer's connection is the caller's to close. Saved
    /// peers stay saved.
    pub fn remove_if(&mut self, f: impl Fn(&KnownPeer) -> bool) -> usize {
        let removed = match self {
            PeerList::Saved(s) => s.retain(|addr| !f(&KnownPeer::new(addr, PeerSource::RESUME_DATA, 0))),
            PeerList::Live(l) => l.remove_if(f),
        };
        self.drop_if_empty();
        removed
    }

    /// libtorrent `peer_list::num_connect_candidates` (`is_connect_candidate`): peers that could
    /// be connected to, now or once their reconnect delay is over. A finished torrent does not
    /// connect to seeds.
    pub fn num_connect_candidates(&self, finished: bool, policy: Reconnect) -> usize {
        match self {
            // fresh peers: none failed, none connected, none known as a seed
            PeerList::Saved(s) if policy.max_failcount > 0 => s.len(),
            PeerList::Saved(_) => 0,
            PeerList::Live(l) => l.num_connect_candidates(finished, policy),
        }
    }

    /// libtorrent `find_connect_candidates`: up to `n` peers to connect to now. A peer waits
    /// `min_reconnect_time` times its fail count plus one after its last connection. A
    /// `finished` torrent passes over seeds (libtorrent `is_connect_candidate`) before taking
    /// the best `n`, so leechers ranked behind them are reached.
    pub fn connect_candidates(
        &self,
        n: usize,
        now: Instant,
        policy: Reconnect,
        finished: bool,
    ) -> Vec<SocketAddr> {
        match self {
            // all equal in the ordering, so in their saved order (none known to be a seed)
            PeerList::Saved(s) if policy.max_failcount > 0 => s.iter().take(n).collect(),
            PeerList::Saved(_) => Vec::new(),
            PeerList::Live(l) => l.connect_candidates(n, now, policy, finished),
        }
    }

    /// A connection to or from `addr` started. Incoming peers are added on the fly.
    pub fn connecting(&mut self, addr: SocketAddr, conn: u64, source: PeerSource, now: Instant) {
        self.live_mut().connecting(addr, conn, source, now);
    }

    /// An incoming connection from `addr` joins the torrent (libtorrent
    /// `peer_list::new_connection`). A peer not in the list yet goes into it within
    /// `max_size`: a full list gives up an erase candidate as [`add_capped`](Self::add_capped)
    /// does (an entry banned for bad data first), else (libtorrent's `force_erase`) the least
    /// useful unconnected peer, never one banned as ourselves. Returns false, adding nothing,
    /// when every other peer is connected or banned as ourselves: the connection is refused
    /// (`too_many_connections`).
    pub fn incoming(
        &mut self,
        addr: SocketAddr,
        conn: u64,
        now: Instant,
        max_size: usize,
        finished: bool,
        policy: Reconnect,
    ) -> bool {
        self.live_mut().incoming(addr, conn, now, max_size, finished, policy)
    }

    /// Up to `n` peers worth persisting in resume data (connectable, not failing).
    pub fn resume_peers(&self, n: usize) -> SavedPeers {
        match self {
            PeerList::Saved(s) => s.first(n),
            PeerList::Live(l) => SavedPeers::from_addrs(l.resume_peers(n)),
        }
    }

    /// Drops unconnected peers beyond `max` (libtorrent `max_paused_peerlist_size`), the least
    /// useful first and banned ones last. Saved peers all rank the same, and the eviction
    /// takes the last of equals: the list is cut.
    pub fn trim(&mut self, max: usize) {
        match self {
            PeerList::Saved(s) => {
                if s.len() > max {
                    *s = s.first(max);
                }
            }
            PeerList::Live(l) => l.trim(max),
        }
        self.drop_if_empty();
    }

    /// Our external address changed: rank every peer again (libtorrent `clear_peer_prio`,
    /// which makes `torrent_peer::rank` recompute on next use). Saved peers are ranked when
    /// they are expanded.
    pub fn clear_peer_prio(&mut self, external_ip: &ExternalIp) {
        if let PeerList::Live(l) = self {
            l.clear_peer_prio(external_ip);
        }
    }

    /// Leaving upload mode (libtorrent `torrent::set_upload_mode` resets every `last_connected`
    /// to force a fast reconnect): no peer waits out its reconnect delay. Saved peers have
    /// never been connected.
    pub fn forget_last_connected(&mut self) {
        if let PeerList::Live(l) = self {
            l.forget_last_connected(false);
        }
    }

    /// As [`forget_last_connected`](Self::forget_last_connected), only for peers with no
    /// failures: those that keep failing wait out their growing delay as before.
    pub fn forget_last_connected_of_sound(&mut self) {
        if let PeerList::Live(l) = self {
            l.forget_last_connected(true);
        }
    }

    /// Frees the list once it holds no peers (a stopped torrent trimmed to none) and no bans.
    fn drop_if_empty(&mut self) {
        if self.len() == 0 && self.live().is_none_or(|l| l.banned_ips.is_empty()) {
            *self = PeerList::default();
        }
    }

    /// The connection closed at `now` (libtorrent `connection_closed`), which starts the peer's
    /// reconnect delay. `failed` counts against the peer (could not connect, protocol error); a
    /// clean disconnect resets its fail count. With `fast_reconnect` (an encrypted attempt the
    /// peer closed on us) a clean close instead forgets the connect time, so the peer is a
    /// candidate again at once for the plaintext retry (libtorrent rewinds it by
    /// `min_reconnect_time * max_failcount`).
    pub fn disconnected(
        &mut self,
        addr: &SocketAddr,
        failed: bool,
        seed: bool,
        fast_reconnect: bool,
        now: Instant,
    ) {
        if let Some(p) = self.get_mut(addr) {
            p.connection = None;
            p.seed |= seed;
            if failed {
                p.failcount = (p.failcount + 1).min(FAILCOUNT_CAP);
                p.last_connected = Some(now);
            } else {
                p.failcount = 0;
                p.last_connected = if fast_reconnect { None } else { Some(now) };
            }
        }
    }

    pub fn pe_support(&self, addr: &SocketAddr) -> bool {
        self.get(addr).is_some_and(|p| p.pe_support)
    }

    pub fn set_pe_support(&mut self, addr: &SocketAddr, pe_support: bool) {
        if let Some(p) = self.get_mut(addr) {
            p.pe_support = pe_support;
        }
    }

    /// Whether to try uTP with this peer (libtorrent `torrent::connect_to_peer`): assumed
    /// until a uTP connect failed, or known once one succeeded.
    pub fn wants_utp(&self, addr: &SocketAddr) -> bool {
        self.get(addr).is_none_or(|p| p.supports_utp || p.confirmed_supports_utp)
    }

    /// A uTP connect failed (libtorrent `on_connection_failed`): never assume uTP again.
    /// Returns whether it was assumed, i.e. whether a TCP retry is due.
    pub fn utp_failed(&mut self, addr: &SocketAddr) -> bool {
        match self.get_mut(addr) {
            Some(p) if p.supports_utp => {
                p.supports_utp = false;
                true
            }
            _ => false,
        }
    }

    /// A uTP connection to this peer was established (libtorrent `on_connection_complete`).
    pub fn utp_confirmed(&mut self, addr: &SocketAddr) {
        if let Some(p) = self.get_mut(addr) {
            p.confirmed_supports_utp = true;
            p.supports_utp = false;
        }
    }

    /// A PEX message flagged this peer as a uTP peer.
    pub fn set_supports_utp(&mut self, addr: &SocketAddr) {
        if let Some(p) = self.get_mut(addr) {
            p.supports_utp = true;
        }
    }

    /// Never connect to this peer again (it turned out to be ourselves). Only this address:
    /// other clients on our host are other peers.
    pub fn ban(&mut self, addr: &SocketAddr) {
        if let Some(p) = self.get_mut(addr) {
            p.banned = true;
        }
    }

    /// Bans a peer that sent bad data: never connect to its IP again, and refuse incoming
    /// connections from it, whatever their port (libtorrent bans the `torrent_peer`, which
    /// stands for the whole IP while `allow_multiple_connections_per_ip` is off, its default;
    /// `new_connection` refuses a banned one). A peer that reconnects comes from a new source
    /// port, which is a new entry here. The IP stays banned when the entry leaves the list.
    pub fn ban_ip(&mut self, addr: &SocketAddr) {
        let Some(p) = self.get_mut(addr) else { return };
        p.banned = true;
        self.live_mut().banned_ips.insert(addr.ip());
    }

    /// An incoming connection from `ip` is refused: a peer there sent bad data.
    pub fn ip_banned(&self, ip: IpAddr) -> bool {
        self.live().is_some_and(|l| l.banned_ips.contains(&ip))
    }

    pub fn set_seed(&mut self, addr: &SocketAddr, seed: bool) {
        if let Some(p) = self.get_mut(addr) {
            p.seed = seed;
        }
    }
}

impl KnownPeer {
    fn new(addr: SocketAddr, source: PeerSource, rank: u32) -> KnownPeer {
        KnownPeer {
            addr,
            source,
            failcount: 0,
            connection: None,
            last_connected: None,
            seed: false,
            banned: false,
            trust: Trust::default(),
            pe_support: false,
            supports_utp: true,
            confirmed_supports_utp: false,
            protocol_v2: false,
            rank,
        }
    }

    /// libtorrent `peer_list::is_connect_candidate`.
    fn connect_candidate(&self, finished: bool, policy: Reconnect) -> bool {
        self.connection.is_none()
            && !self.banned
            && self.failcount < policy.max_failcount
            && !(self.seed && finished)
    }

    /// libtorrent `peer_list::is_erase_candidate`: an unconnected peer that is no connect
    /// candidate and failed or is known from resume data only. A banned peer is not: one
    /// banned for bad data goes before any candidate (`Peers::evict_ip_banned`, its IP stays
    /// banned), and one banned as ourselves stays.
    fn erase_candidate(&self, finished: bool, policy: Reconnect) -> bool {
        self.connection.is_none()
            && !self.banned
            && !self.connect_candidate(finished, policy)
            && (self.failcount > 0 || self.source == PeerSource::RESUME_DATA)
    }

    /// libtorrent `compare_peer_erase`, greater goes first: more failures, then known from
    /// resume data only, then fewer trust points; then the lower BEP 40 rank.
    fn erase_order(&self) -> (u8, bool, std::cmp::Reverse<i8>, std::cmp::Reverse<u32>) {
        (
            self.failcount,
            self.source == PeerSource::RESUME_DATA,
            std::cmp::Reverse(self.trust.trust_points),
            std::cmp::Reverse(self.rank),
        )
    }
}

/// The full list.
#[derive(Default)]
pub(crate) struct Peers {
    peers: Vec<KnownPeer>,
    index: HashMap<SocketAddr, usize>,
    /// IPs of peers banned for bad data ([`PeerList::ban_ip`]): incoming connections from
    /// them are refused and no peer there is connected to. Kept apart from the entries, so
    /// no eviction lifts a ban.
    banned_ips: HashSet<IpAddr>,
}

impl Peers {
    pub fn num_seeds(&self) -> usize {
        self.peers.iter().filter(|p| p.seed).count()
    }

    pub fn get(&self, addr: &SocketAddr) -> Option<&KnownPeer> {
        self.index.get(addr).map(|&i| &self.peers[i])
    }

    pub fn get_mut(&mut self, addr: &SocketAddr) -> Option<&mut KnownPeer> {
        self.index.get(addr).map(|&i| &mut self.peers[i])
    }

    /// libtorrent `peer_list::add_peer`. Returns false if the peer was rejected (full list,
    /// invalid address).
    pub fn add(
        &mut self,
        addr: SocketAddr,
        source: PeerSource,
        external_ip: ExternalIp,
        max_size: usize,
        finished: bool,
        policy: Reconnect,
    ) -> bool {
        if let Some(p) = self.get_mut(&addr) {
            p.source |= source;
            // libtorrent `update_peer`: a tracker listing a peer that failed us gives it another
            // try, since somebody apparently reaches it (only trackers are trusted with that)
            if source.contains(PeerSource::TRACKER) {
                p.failcount = p.failcount.saturating_sub(1);
            }
            return true;
        }
        if self.peers.len() >= max_size
            && !self.evict_ip_banned()
            && !self.evict(|p| p.erase_candidate(finished, policy))
        {
            return false;
        }
        let rank = rank(&external_ip, addr);
        self.index.insert(addr, self.peers.len());
        self.peers.push(KnownPeer::new(addr, source, rank));
        true
    }

    /// Drops the least useful peer `eligible` allows (libtorrent `erase_peers`, the last of
    /// equals). Returns false if there is none.
    fn evict(&mut self, eligible: impl Fn(&KnownPeer) -> bool) -> bool {
        let victim = self
            .peers
            .iter()
            .enumerate()
            .filter(|(_, p)| eligible(p))
            .max_by_key(|(_, p)| p.erase_order())
            .map(|(i, _)| i);
        match victim {
            Some(i) => {
                self.remove_at(i);
                true
            }
            None => false,
        }
    }

    /// Drops an unconnected entry whose IP is banned for bad data, the first to go from a full
    /// list (libtorrent erases a banned peer that failed). `banned_ips` keeps the ban, so the
    /// entry is no use; without this, peers banned from enough addresses would fill the list
    /// and keep every new peer out. A peer banned as ourselves ([`PeerList::ban`]) has no IP
    /// entry and stays.
    fn evict_ip_banned(&mut self) -> bool {
        if self.banned_ips.is_empty() {
            return false;
        }
        let banned_ips = &self.banned_ips;
        let victim = self
            .peers
            .iter()
            .position(|p| p.banned && p.connection.is_none() && banned_ips.contains(&p.addr.ip()));
        match victim {
            Some(i) => {
                self.remove_at(i);
                true
            }
            None => false,
        }
    }

    fn remove_at(&mut self, i: usize) {
        let p = self.peers.swap_remove(i);
        self.index.remove(&p.addr);
        if i < self.peers.len() {
            self.index.insert(self.peers[i].addr, i);
        }
    }

    /// Forgets every peer `f` picks (libtorrent `apply_ip_filter` / `apply_port_filter`).
    /// Returns how many went. A connected peer's connection is the caller's to close.
    pub fn remove_if(&mut self, f: impl Fn(&KnownPeer) -> bool) -> usize {
        let before = self.peers.len();
        self.peers.retain(|p| !f(p));
        if self.peers.len() != before {
            self.index = self.peers.iter().enumerate().map(|(i, p)| (p.addr, i)).collect();
        }
        before - self.peers.len()
    }

    /// libtorrent `peer_list::num_connect_candidates` (`is_connect_candidate`): peers that could
    /// be connected to, now or once their reconnect delay is over. A finished torrent does not
    /// connect to seeds.
    pub fn num_connect_candidates(&self, finished: bool, policy: Reconnect) -> usize {
        self.peers.iter().filter(|p| self.connect_candidate(p, finished, policy)).count()
    }

    /// [`KnownPeer::connect_candidate`], and not at an IP banned for bad data: libtorrent's one
    /// `torrent_peer` per IP is banned there, whichever port a tracker or PEX lists next.
    fn connect_candidate(&self, p: &KnownPeer, finished: bool, policy: Reconnect) -> bool {
        p.connect_candidate(finished, policy)
            && (self.banned_ips.is_empty() || !self.banned_ips.contains(&p.addr.ip()))
    }

    /// libtorrent `find_connect_candidates`: up to `n` peers to connect to now. A peer waits
    /// `min_reconnect_time` times its fail count plus one after its last connection.
    pub fn connect_candidates(
        &self,
        n: usize,
        now: Instant,
        policy: Reconnect,
        finished: bool,
    ) -> Vec<SocketAddr> {
        let mut c: Vec<&KnownPeer> = self
            .peers
            .iter()
            .filter(|p| self.connect_candidate(p, finished, policy))
            .filter(|p| {
                p.last_connected.is_none_or(|t| {
                    now.duration_since(t) >= policy.min_reconnect_time * (u32::from(p.failcount) + 1)
                })
            })
            .collect();
        c.sort_by_key(|p| (p.failcount, std::cmp::Reverse(p.rank), p.last_connected));
        c.into_iter().take(n).map(|p| p.addr).collect()
    }

    /// See [`PeerList::incoming`].
    pub fn incoming(
        &mut self,
        addr: SocketAddr,
        conn: u64,
        now: Instant,
        max_size: usize,
        finished: bool,
        policy: Reconnect,
    ) -> bool {
        if self.get(&addr).is_none()
            && self.peers.len() >= max_size
            && !self.evict_ip_banned()
            && !self.evict(|p| p.erase_candidate(finished, policy))
            && !self.evict(|p| p.connection.is_none() && !p.banned)
        {
            return false;
        }
        self.connecting(addr, conn, PeerSource::INCOMING, now);
        true
    }

    /// A connection to or from `addr` started. A peer not in the list is added without the
    /// size cap: outgoing connections go to listed peers, and incoming ones come through
    /// [`incoming`](Self::incoming), which makes room first.
    pub fn connecting(&mut self, addr: SocketAddr, conn: u64, source: PeerSource, now: Instant) {
        if self.get(&addr).is_none() {
            self.index.insert(addr, self.peers.len());
            self.peers.push(KnownPeer::new(addr, source, 0));
        }
        let p = self.get_mut(&addr).expect("just added");
        p.connection = Some(conn);
        p.last_connected = Some(now);
    }

    /// Up to `n` peers worth persisting in resume data (connectable, not failing).
    pub fn resume_peers(&self, n: usize) -> Vec<SocketAddr> {
        self.peers
            .iter()
            .filter(|p| !p.banned && p.failcount == 0 && !p.source.contains(PeerSource::INCOMING))
            .take(n)
            .map(|p| p.addr)
            .collect()
    }

    /// Drops unconnected peers beyond `max` (libtorrent `max_paused_peerlist_size`, where a
    /// paused torrent's `erase_peers` takes erase candidates only): the least useful first, in
    /// libtorrent's erase order, banned ones last.
    pub fn trim(&mut self, max: usize) {
        while self.peers.len() > max && self.evict(|p| p.connection.is_none() && !p.banned) {}
        while self.peers.len() > max && self.evict(|p| p.connection.is_none()) {}
    }

    /// Our external address changed: rank every peer again (libtorrent `clear_peer_prio`,
    /// which makes `torrent_peer::rank` recompute on next use).
    pub fn clear_peer_prio(&mut self, external_ip: &ExternalIp) {
        for p in &mut self.peers {
            p.rank = rank(external_ip, p.addr);
        }
    }

    /// Forgets every peer's last connection time (with `only_sound`, of peers without a
    /// failure), so none waits out its reconnect delay.
    pub fn forget_last_connected(&mut self, only_sound: bool) {
        for p in self.peers.iter_mut().filter(|p| !only_sound || p.failcount == 0) {
            p.last_connected = None;
        }
    }
}

/// libtorrent `torrent_peer::rank`: BEP 40 priority against the address the peer sees us at
/// (0 while we don't know it).
fn rank(external_ip: &ExternalIp, addr: SocketAddr) -> u32 {
    external_ip.external_address(addr.ip()).map_or(0, |ip| peer_priority(SocketAddr::new(ip, 0), addr))
}

/// BEP 40 canonical peer priority (libtorrent `peer_priority`): CRC32-C of the two masked
/// addresses in ascending order (or of the two ports, if the addresses are equal), so both ends
/// compute the same rank.
pub(crate) fn peer_priority(e1: SocketAddr, e2: SocketAddr) -> u32 {
    let (e1, e2) = if e1 > e2 { (e2, e1) } else { (e1, e2) };
    let mut c = Crc32c::new();
    if e1.ip() == e2.ip() {
        let (p1, p2) = (e1.port().min(e2.port()), e1.port().max(e2.port()));
        c.update(&p1.to_be_bytes());
        c.update(&p2.to_be_bytes());
        return c.checksum();
    }
    match (e1.ip(), e2.ip()) {
        (IpAddr::V4(a), IpAddr::V4(b)) => {
            const V4MASK: [[u8; 4]; 3] = [[0xff, 0xff, 0x55, 0x55], [0xff, 0xff, 0xff, 0x55], [0xff; 4]];
            let (mut b1, mut b2) = (a.octets(), b.octets());
            let mask = if b1[..2] != b2[..2] {
                0
            } else if b1[..3] != b2[..3] {
                1
            } else {
                2
            };
            for i in 0..4 {
                b1[i] &= V4MASK[mask][i];
                b2[i] &= V4MASK[mask][i];
            }
            c.update(&b1);
            c.update(&b2);
        }
        (IpAddr::V6(a), IpAddr::V6(b)) => {
            let (mut b1, mut b2) = (a.octets(), b.octets());
            // The first 6 bytes are never masked; masking starts after the first differing
            // byte (libtorrent's loop, verbatim).
            let mut offset = usize::MAX;
            for i in 0..16 {
                if offset == usize::MAX && b1[i] != b2[i] {
                    offset = (i + 1).max(5);
                } else if i > offset {
                    b1[i] &= 0x55;
                    b2[i] &= 0x55;
                }
            }
            c.update(&b1);
            c.update(&b2);
        }
        _ => return 0,
    }
    c.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ep(s: &str, port: u16) -> SocketAddr {
        SocketAddr::new(s.parse().unwrap(), port)
    }

    fn hash_buffer(hex: &str) -> u32 {
        let mut buf = vec![0u8; hex.len() / 2];
        assert!(crate::util::hex::decode_into(hex, &mut buf));
        crate::crypto::crc32c(&buf)
    }

    // libtorrent test_peer_priority.cpp
    #[test]
    fn bep40_peer_priority() {
        assert_eq!(
            peer_priority(ep("230.12.123.3", 0x4d2), ep("230.12.123.3", 0x12c)),
            hash_buffer("012c04d2")
        );
        assert_eq!(
            peer_priority(ep("230.12.123.1", 0x4d2), ep("230.12.123.3", 0x12c)),
            hash_buffer("e60c7b01e60c7b03")
        );
        assert_eq!(
            peer_priority(ep("230.12.23.1", 0x4d2), ep("230.12.123.3", 0x12c)),
            hash_buffer("e60c1701e60c7b01")
        );
        assert_eq!(
            peer_priority(ep("230.120.23.1", 0x4d2), ep("230.12.123.3", 0x12c)),
            hash_buffer("e60c5101e6781501")
        );
        assert_eq!(peer_priority(ep("123.213.32.10", 0), ep("98.76.54.32", 0)), 0xec2d7224);
        assert_eq!(peer_priority(ep("123.213.32.10", 0), ep("123.213.32.234", 0)), 0x99568189);

        let all = "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff";
        assert_eq!(peer_priority(ep(all, 0x4d2), ep(all, 0x12c)), hash_buffer("012c04d2"));
        assert_eq!(peer_priority(ep(all, 0x12c), ep(all, 0x4d2)), hash_buffer("012c04d2"));
        let cases = [
            (
                "ffff:0fff:ffff:ffff:ffff:ffff:ffff:ffff",
                "ffff0fffffff55555555555555555555ffffffffffff55555555555555555555",
            ),
            (
                "ffff:ffff:0fff:ffff:ffff:ffff:ffff:ffff",
                "ffffffff0fff55555555555555555555ffffffffffff55555555555555555555",
            ),
            (
                "ffff:ffff:ff0f:ffff:ffff:ffff:ffff:ffff",
                "ffffffffff0fff555555555555555555ffffffffffffff555555555555555555",
            ),
            (
                "ffff:ffff:ffff:0fff:ffff:ffff:ffff:ffff",
                "ffffffffffff0fff5555555555555555ffffffffffffffff5555555555555555",
            ),
        ];
        for (other, expected) in cases {
            assert_eq!(peer_priority(ep(all, 0x4d2), ep(other, 0x12c)), hash_buffer(expected), "{other}");
        }
    }

    #[test]
    fn candidates_failures_and_cap() {
        let mut pl = PeerList::default();
        let now = Instant::now();
        let a: SocketAddr = "10.0.0.1:1".parse().unwrap();
        let b: SocketAddr = "10.0.0.2:1".parse().unwrap();
        assert!(pl.add(a, PeerSource::TRACKER, ExternalIp::default(), 2));
        assert!(pl.add(b, PeerSource::TRACKER, ExternalIp::default(), 2));
        assert!(pl.add(a, PeerSource::PEX, ExternalIp::default(), 2), "duplicates merge sources");
        assert!(pl.get(&a).unwrap().source.contains(PeerSource::PEX));
        assert!(!pl.add("0.0.0.0:5".parse().unwrap(), PeerSource::TRACKER, ExternalIp::default(), 9));
        let r = Reconnect::default();
        assert_eq!(pl.connect_candidates(5, now, r, false).len(), 2);

        pl.connecting(a, 1, PeerSource::TRACKER, now);
        assert_eq!(pl.connect_candidates(5, now, r, false), [b]);
        assert_eq!(pl.num_connect_candidates(false, r), 1, "a connected peer is no candidate");
        pl.set_seed(&b, true);
        assert_eq!(pl.num_connect_candidates(true, r), 0, "a finished torrent skips seeds");
        assert_eq!(pl.connect_candidates(5, now, r, true), [] as [SocketAddr; 0]);
        // ... before taking the best: a leecher ranked behind a seed is still reached
        let d: SocketAddr = "10.0.0.4:1".parse().unwrap();
        let mut two = PeerList::default();
        for x in [b, d] {
            assert!(two.add(x, PeerSource::TRACKER, ExternalIp::default(), 9));
        }
        two.set_seed(&b, true);
        assert_eq!(two.connect_candidates(1, now, r, false), [b]);
        assert_eq!(two.connect_candidates(1, now, r, true), [d]);
        pl.set_seed(&b, false);
        pl.disconnected(&a, true, false, false, now);
        assert_eq!(pl.connect_candidates(5, now, r, false), [b], "reconnects wait");
        assert_eq!(pl.num_connect_candidates(false, r), 2, "but they count");
        assert_eq!(
            pl.connect_candidates(5, now + Duration::from_secs(121), r, false),
            [b, a],
            "failed peers last"
        );

        // Full list: a peer that failed once is still a connect candidate, which libtorrent's
        // `is_erase_candidate` keeps; the new peer is turned away instead.
        let c: SocketAddr = "10.0.0.3:1".parse().unwrap();
        assert!(!pl.add(c, PeerSource::TRACKER, ExternalIp::default(), 2));
        assert_eq!(pl.len(), 2);
        // Once it failed `max_failcount` times it makes room.
        for conn in [2, 3] {
            pl.connecting(a, conn, PeerSource::TRACKER, now);
            pl.disconnected(&a, true, false, false, now);
        }
        assert!(pl.add(c, PeerSource::TRACKER, ExternalIp::default(), 2));
        assert!(pl.get(&a).is_none() && pl.get(&b).is_some() && pl.get(&c).is_some());
        pl.ban(&b);
        assert_eq!(pl.connect_candidates(5, now, r, false), [c]);
        pl.trim(1);
        assert_eq!(pl.len(), 1);
        assert!(pl.get(&b).is_some_and(|p| p.banned), "trimming drops banned peers last");
    }

    /// Bug-hunt 2026-11-21 items 5 and 13: a ban for bad data covers the peer's IP for incoming
    /// connections, and no eviction lifts it.
    #[test]
    fn bans_cover_the_ip_and_survive_a_full_list() {
        let mut pl = PeerList::default();
        let r = Reconnect::default();
        let now = Instant::now();
        let (bad, x, y) = (ep("10.0.0.9", 50000), ep("10.0.0.1", 1), ep("10.0.0.2", 1));
        // a peer that came in from a source port sent bad data
        pl.connecting(bad, 1, PeerSource::INCOMING, now);
        pl.ban_ip(&bad);
        pl.disconnected(&bad, true, false, false, now);
        assert!(pl.ip_banned(bad.ip()), "whatever port it comes from next");
        assert!(!pl.ip_banned(x.ip()));
        // a full list gives up the banned entry first (release review of 1.2.75: banned
        // entries that stayed filled the list from enough addresses); the IP set keeps the ban
        assert!(pl.add(x, PeerSource::TRACKER, ExternalIp::default(), 2));
        assert!(pl.add(y, PeerSource::PEX, ExternalIp::default(), 2));
        assert!(pl.get(&bad).is_none() && pl.ip_banned(bad.ip()));
        assert_eq!(pl.num_connect_candidates(false, r), 2);
        // x and y are wanted: a third peer is refused
        assert!(!pl.add(ep("10.0.0.3", 1), PeerSource::PEX, ExternalIp::default(), 2));
        // its listen port, listed by a tracker, is not connected to
        let listen = ep("10.0.0.9", 6881);
        assert!(pl.add(listen, PeerSource::TRACKER, ExternalIp::default(), 3));
        assert!(!pl.connect_candidates(5, now, r, false).contains(&listen));
        assert_eq!(pl.num_connect_candidates(false, r), 2);
        // trimmed to nothing, the IP ban remains and the list with it
        pl.trim(0);
        assert_eq!(pl.len(), 0);
        assert!(pl.ip_banned(bad.ip()));
        assert!(matches!(pl, PeerList::Live(_)));
        // a ban for connecting to ourselves is for that address only (other clients on this
        // host are other peers), and an unknown address bans nothing
        let me = ep("10.0.0.5", 6881);
        assert!(pl.add(me, PeerSource::LSD, ExternalIp::default(), 9));
        pl.ban(&me);
        assert!(pl.get(&me).is_some_and(|p| p.banned) && !pl.ip_banned(me.ip()));
        pl.ban_ip(&ep("10.0.0.6", 1));
        assert!(!pl.ip_banned(ep("10.0.0.6", 1).ip()));
        let mut fresh = PeerList::default();
        fresh.ban_ip(&x);
        assert!(matches!(&fresh, PeerList::Saved(s) if s.is_empty()), "nothing to ban");
    }

    /// libtorrent `is_erase_candidate` and `compare_peer_erase`: a finished torrent gives up a
    /// seed known from resume data only, never a fresh peer, a connected one or a banned one;
    /// among candidates the one that failed most goes first.
    #[test]
    fn a_full_list_gives_up_erase_candidates_only() {
        let r = Reconnect::default();
        let now = Instant::now();
        let (s, t, n) = (ep("10.0.1.1", 1), ep("10.0.1.2", 1), ep("10.0.1.3", 1));
        let mut pl = PeerList::default();
        pl.restore(SavedPeers::from_addrs([s]));
        assert!(pl.add(t, PeerSource::TRACKER, ExternalIp::default(), 9));
        pl.set_seed(&s, true);
        pl.set_seed(&t, true);
        assert!(!pl.add_capped(n, PeerSource::DHT, ExternalIp::default(), 2, false, r), "still wanted");
        assert!(pl.add_capped(n, PeerSource::DHT, ExternalIp::default(), 2, true, r));
        assert!(pl.get(&s).is_none() && pl.get(&t).is_some(), "the resume-data seed went");
        // connected and banned peers never go, however often they failed
        let mut pl = PeerList::default();
        for (i, a) in [s, t].into_iter().enumerate() {
            for conn in 0..4 {
                pl.connecting(a, conn, PeerSource::TRACKER, now);
                pl.disconnected(&a, true, false, false, now);
            }
            pl.connecting(a, 9, PeerSource::TRACKER, now);
            if i == 1 {
                pl.disconnected(&a, true, false, false, now);
                pl.ban(&a);
            }
        }
        assert!(!pl.add(n, PeerSource::DHT, ExternalIp::default(), 2));
        // among candidates: more failures first
        let mut pl = PeerList::default();
        for (a, fails) in [(s, 5), (t, 3)] {
            for conn in 0..fails {
                pl.connecting(a, conn, PeerSource::TRACKER, now);
                pl.disconnected(&a, true, false, false, now);
            }
        }
        assert!(pl.add(n, PeerSource::DHT, ExternalIp::default(), 2));
        assert!(pl.get(&s).is_none() && pl.get(&t).is_some());
    }

    /// libtorrent `peer_list::new_connection`: an incoming peer goes into a full list in place
    /// of an erase candidate first, else of the least useful unconnected peer
    /// (`force_erase`), never of a connected or banned one; with none, it is refused.
    #[test]
    fn incoming_peers_keep_to_the_cap() {
        let r = Reconnect::default();
        let now = Instant::now();
        let (fresh, failed, banned) = (ep("10.0.2.1", 1), ep("10.0.2.2", 1), ep("10.0.2.3", 1));
        let (i1, i2, i3) = (ep("10.0.3.1", 40001), ep("10.0.3.2", 40002), ep("10.0.3.3", 40003));
        let mut pl = PeerList::default();
        for a in [fresh, failed, banned] {
            assert!(pl.add(a, PeerSource::TRACKER, ExternalIp::default(), 3));
        }
        for conn in 0..3 {
            pl.connecting(failed, conn, PeerSource::TRACKER, now);
            pl.disconnected(&failed, true, false, false, now);
        }
        pl.connecting(banned, 7, PeerSource::TRACKER, now);
        pl.ban_ip(&banned);
        pl.disconnected(&banned, true, false, false, now);
        // the entry banned for bad data goes first (its IP stays banned)
        assert!(pl.incoming(i1, 10, now, 3, false, r));
        assert!(pl.get(&banned).is_none() && pl.ip_banned(banned.ip()));
        assert!(pl.get(&i1).is_some_and(|p| p.connection == Some(10) && p.source == PeerSource::INCOMING));
        // then the erase candidate, though it failed: the fresh peer is a connect candidate
        assert!(pl.incoming(i2, 11, now, 3, false, r));
        assert!(pl.get(&failed).is_none() && pl.get(&fresh).is_some());
        // then, forced, the fresh one
        assert!(pl.incoming(i3, 12, now, 3, false, r));
        assert!(pl.get(&fresh).is_none());
        // every other peer connected: refused, nothing added
        let i4 = ep("10.0.3.4", 40004);
        assert!(!pl.incoming(i4, 13, now, 3, false, r));
        assert!(pl.get(&i4).is_none());
        assert_eq!(pl.len(), 3);
        // a listed peer connecting in needs no room
        pl.disconnected(&i1, false, false, false, now);
        assert!(pl.incoming(i1, 14, now, 3, false, r));
        assert_eq!(pl.get(&i1).and_then(|p| p.connection), Some(14));
        assert_eq!(pl.len(), 3);
    }

    /// Peers restored from resume data stay compact until the torrent starts, and read as the
    /// list they stand for: the same peers added one by one.
    #[test]
    fn saved_peers_read_as_the_list_they_stand_for() {
        assert_eq!(std::mem::size_of::<PeerList>(), 16);
        let addrs: Vec<SocketAddr> =
            ["10.0.0.1:1", "[2001:db8::1]:2", "10.0.0.2:3", "10.0.0.3:4", "[2001:db8::2]:5"]
                .iter()
                .map(|a| a.parse().unwrap())
                .collect();
        let live = || {
            // resume data lists the IPv4 peers (`peers`), then the IPv6 ones (`peers6`)
            let mut l = PeerList::default();
            for &a in addrs.iter().filter(|a| a.is_ipv4()).chain(addrs.iter().filter(|a| a.is_ipv6())) {
                l.add(a, PeerSource::RESUME_DATA, ExternalIp::default(), usize::MAX);
            }
            l
        };
        let saved = || {
            let mut l = PeerList::default();
            l.restore(SavedPeers::from_addrs(addrs.iter().copied()));
            assert!(matches!(l, PeerList::Saved(_)));
            l
        };
        let now = Instant::now();
        let (a, b) = (live(), saved());
        let r = Reconnect::default();
        let none = Reconnect { max_failcount: 0, ..r };
        assert_eq!((a.len(), a.num_seeds()), (b.len(), b.num_seeds()));
        for policy in [r, none] {
            for finished in [false, true] {
                assert_eq!(
                    a.num_connect_candidates(finished, policy),
                    b.num_connect_candidates(finished, policy)
                );
            }
            for n in [0, 3, 9] {
                assert_eq!(
                    a.connect_candidates(n, now, policy, false),
                    b.connect_candidates(n, now, policy, false)
                );
            }
        }
        for n in [0, 2, 3, 100] {
            assert_eq!(a.resume_peers(n), b.resume_peers(n));
        }
        for &addr in &addrs {
            assert_eq!(a.pe_support(&addr), b.pe_support(&addr));
            assert_eq!(a.wants_utp(&addr), b.wants_utp(&addr));
        }
        for max in [0, 1, 4, 9] {
            let (mut a, mut b) = (live(), saved());
            a.trim(max);
            b.trim(max);
            assert_eq!(a.resume_peers(100), b.resume_peers(100), "trim({max})");
            assert!(max > 0 || matches!(&b, PeerList::Saved(s) if s.is_empty()));
        }
        let (mut a, mut b) = (live(), saved());
        let v6 = |k: &KnownPeer| k.addr.is_ipv6();
        assert_eq!(a.remove_if(v6), b.remove_if(v6));
        assert!(matches!(b, PeerList::Saved(_)), "filtering keeps them saved");
        assert_eq!(a.resume_peers(100), b.resume_peers(100));

        // starting expands them, ranked; changing one expands them too
        let (a, mut b) = (live(), saved());
        b.expand(&ExternalIp::default());
        assert!(matches!(b, PeerList::Live(_)));
        for &addr in &addrs {
            assert_eq!(format!("{:?}", a.get(&addr)), format!("{:?}", b.get(&addr)));
        }
        let mut b = saved();
        b.ban(&"10.9.9.9:9".parse().unwrap());
        assert!(matches!(b, PeerList::Saved(_)), "an unknown peer expands nothing");
        b.ban(&addrs[0]);
        assert!(b.get(&addrs[0]).unwrap().banned);
        let mut b = saved();
        assert!(b.add("10.9.9.9:9".parse().unwrap(), PeerSource::TRACKER, ExternalIp::default(), 99));
        assert_eq!(b.len(), addrs.len() + 1);
        // an empty list stays unallocated when the torrent starts
        let mut e = PeerList::default();
        e.expand(&ExternalIp::default());
        e.restore(SavedPeers::default());
        assert!(matches!(&e, PeerList::Saved(s) if s.is_empty()));
    }

    #[test]
    fn the_reconnect_delay_runs_from_the_close() {
        // libtorrent `connection_closed` stamps `last_connected`: a peer connected for an hour
        // waits `min_reconnect_time` after it leaves, not after it came
        let mut pl = PeerList::default();
        let r = Reconnect::default();
        let t0 = Instant::now();
        let a: SocketAddr = "10.0.0.1:1".parse().unwrap();
        pl.connecting(a, 1, PeerSource::TRACKER, t0);
        let closed = t0 + Duration::from_secs(3600);
        pl.disconnected(&a, false, false, false, closed);
        assert!(pl.connect_candidates(5, closed + Duration::from_secs(59), r, false).is_empty());
        assert_eq!(pl.connect_candidates(5, closed + Duration::from_secs(60), r, false), [a]);
        // the plaintext retry after an encrypted attempt goes at once
        pl.connecting(a, 2, PeerSource::TRACKER, closed);
        pl.disconnected(&a, false, false, true, closed);
        assert_eq!(pl.connect_candidates(5, closed, r, false), [a]);
        // leaving upload mode forgets the delay (libtorrent `set_upload_mode`)
        pl.connecting(a, 3, PeerSource::TRACKER, closed);
        pl.disconnected(&a, false, false, false, closed);
        assert!(pl.connect_candidates(5, closed, r, false).is_empty());
        pl.forget_last_connected();
        assert_eq!(pl.connect_candidates(5, closed, r, false), [a]);
        // a file turned on again forgets it only for peers without failures
        let b: SocketAddr = "10.0.0.2:1".parse().unwrap();
        assert!(pl.add(b, PeerSource::TRACKER, ExternalIp::default(), 9));
        for (addr, conn, failed) in [(a, 4, false), (b, 5, true)] {
            pl.connecting(addr, conn, PeerSource::TRACKER, closed);
            pl.disconnected(&addr, failed, false, false, closed);
        }
        assert!(pl.connect_candidates(5, closed, r, false).is_empty());
        pl.forget_last_connected_of_sound();
        assert_eq!(pl.connect_candidates(5, closed, r, false), [a], "the failing peer waits its delay");
    }

    #[test]
    fn a_tracker_gives_a_failed_peer_another_try() {
        // libtorrent `update_peer`: only a tracker's word takes a failure off
        let mut pl = PeerList::default();
        let r = Reconnect { max_failcount: 2, min_reconnect_time: Duration::from_secs(1) };
        let now = Instant::now();
        let a: SocketAddr = "10.0.0.1:1".parse().unwrap();
        assert!(pl.add(a, PeerSource::DHT, ExternalIp::default(), 9));
        for _ in 0..2 {
            pl.connecting(a, 1, PeerSource::DHT, now);
            pl.disconnected(&a, true, false, false, now);
        }
        let later = now + Duration::from_secs(10);
        assert_eq!(pl.get(&a).unwrap().failcount, 2);
        assert!(pl.connect_candidates(5, later, r, false).is_empty(), "max_failcount reached");
        assert_eq!(pl.num_connect_candidates(false, r), 0);
        pl.add(a, PeerSource::PEX, ExternalIp::default(), 9);
        assert_eq!(pl.get(&a).unwrap().failcount, 2, "a peer's word is not enough");
        pl.add(a, PeerSource::TRACKER, ExternalIp::default(), 9);
        assert_eq!(pl.get(&a).unwrap().failcount, 1);
        assert_eq!(pl.connect_candidates(5, later, r, false), [a]);
        // the settings' names and defaults are libtorrent's
        let d = Reconnect::from_settings(&crate::Settings::default());
        assert_eq!((d.max_failcount, d.min_reconnect_time), (3, Duration::from_secs(60)));
    }
}
