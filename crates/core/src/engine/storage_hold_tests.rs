//! D79: a torrent held for its missing storage stays stopped while the probe does not find its
//! path, and starts (checking its files) once it does; a Start releases the hold. The engine is
//! driven by hand on its own thread, so the probe's interval is stepped over with `now`.

use super::*;
use crate::disk::tests::{make_torrent, temp_dir};
use crate::session::Session;

#[test]
fn a_held_torrent_starts_when_its_path_is_back() {
    let (torrent, _) = make_torrent(&[("a.bin", 70_000)], 16 * 1024);
    let dir = temp_dir("storage-hold");
    let settings = Settings {
        listen_interfaces: "127.0.0.1:0".into(),
        enable_dht: false,
        enable_lsd: false,
        enable_upnp: false,
        enable_natpmp: false,
        ..Settings::default()
    };
    let (done_tx, done_rx) = mpsc::channel();
    let save_path = dir.to_string_lossy().into_owned();
    let drive = dir.join("drive");
    let session = Session::spawn(settings, Box::new(|| {}), move |rx, shared, settings| {
        let poll = Poll::new().unwrap();
        let waker = Arc::new(Waker::new(poll.registry(), WAKER_TOKEN).unwrap());
        let mut e = Engine::new(poll, waker, rx, shared, settings);
        let mut p = crate::params::AddTorrentParams::new(
            crate::params::TorrentSource::TorrentFile(torrent),
            save_path,
        );
        p.flags = crate::params::TorrentFlags::AUTO_MANAGED;
        e.add_torrent(p);
        let id = e
            .out
            .iter()
            .find_map(|a| match a {
                Alert::TorrentAdded { id, .. } => Some(*id),
                _ => None,
            })
            .expect("added");
        assert!(e.torrent(id).unwrap().storage.is_some(), "started");
        let path = drive.to_string_lossy().into_owned();
        // the probe's answer, past the other completions (fed to the engine as they come)
        let probe = |e: &mut Engine, now: Instant| {
            e.storage_hold_tick(now);
            assert!(e.holds.probing, "a probe is out");
            loop {
                match e.disk_rx.recv_timeout(Duration::from_secs(10)).expect("disk completion") {
                    Completion::PathsProbed { present } => {
                        break e.on_disk(Completion::PathsProbed { present });
                    }
                    c => e.on_disk(c),
                }
            }
        };

        e.hold_for_storage(id, path.clone());
        let t = e.torrent(id).unwrap();
        assert_eq!(t.error.as_deref(), Some(storage_hold::hold_message(&path).as_str()));
        assert!(t.storage.is_none(), "stopped");
        let start = Instant::now();
        e.storage_hold_tick(start);
        assert!(!e.holds.probing, "not before the interval");

        // not there: still held
        probe(&mut e, start + storage_hold::PROBE_INTERVAL);
        assert!(e.holds.held.contains_key(&id));
        assert!(e.torrent(id).unwrap().error.is_some());

        // back: the error is gone and the torrent checks its files
        std::fs::create_dir_all(&drive).unwrap();
        probe(&mut e, start + storage_hold::PROBE_INTERVAL * 2);
        assert!(e.holds.held.is_empty());
        let t = e.torrent(id).unwrap();
        assert!(t.error.is_none() && t.storage.is_some(), "started again");

        // a user Start (pause, resume) ends a hold too; Remove forgets it
        e.hold_for_storage(id, path.clone());
        e.set_flags(id, TorrentFlags::PAUSED, TorrentFlags::PAUSED);
        e.set_flags(id, TorrentFlags::empty(), TorrentFlags::PAUSED);
        assert!(e.holds.held.is_empty());
        e.hold_for_storage(id, path);
        e.remove_torrent(id, crate::params::RemoveFlags::empty());
        assert!(e.holds.held.is_empty());
        done_tx.send(()).unwrap();
    });
    let finished = done_rx.recv_timeout(Duration::from_secs(30));
    drop(session);
    let _ = std::fs::remove_dir_all(dir);
    finished.expect("the engine thread ran (a panic there is printed above)");
}
