//! Bug hunt 17: the progress a stopped check still reports is dropped. The engine is driven by
//! hand on its own thread, so the stale report comes exactly where the race puts it: after the
//! restart, before the new check begins.

use super::*;
use crate::disk::tests::{make_torrent, temp_dir};
use crate::session::Session;

/// A recheck (stop and start) while the old check runs: the old storage stays in `storages`
/// until its fence completes, and its progress must not overwrite the new check's "queued" or
/// its own progress; progress after the check is done is ignored too.
#[test]
fn a_stopped_checks_progress_is_ignored() {
    let (torrent, _) = make_torrent(&[("a.bin", 200_000), ("b.bin", 70_000)], 16 * 1024);
    let dir = temp_dir("check-progress");
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
    let session = Session::spawn(settings, Box::new(|| {}), move |rx, shared, settings| {
        let poll = Poll::new().unwrap();
        let waker = Arc::new(Waker::new(poll.registry(), WAKER_TOKEN).unwrap());
        let mut e = Engine::new(poll, waker, rx, shared, settings);
        let mut p = crate::params::AddTorrentParams::new(
            crate::params::TorrentSource::TorrentFile(torrent),
            save_path,
        );
        p.flags = crate::params::TorrentFlags::empty();
        e.add_torrent(p);
        let id = e
            .out
            .iter()
            .find_map(|a| match a {
                Alert::TorrentAdded { id, .. } => Some(*id),
                _ => None,
            })
            .expect("added");
        let storage_of =
            |e: &mut Engine| e.torrent(id).and_then(|t| t.storage.as_ref()).map(|s| s.id).unwrap();
        let checked = |e: &mut Engine| e.torrent(id).unwrap().checked_pieces;
        let old = storage_of(&mut e);
        assert!(e.torrent(id).unwrap().checking);
        e.on_disk(Completion::CheckStarted { storage: old });
        e.on_disk(Completion::CheckProgress { storage: old, checked: 16 });
        assert_eq!(checked(&mut e), 16, "the current check's progress counts");

        // the recheck: a new storage, its check queued; the old one is still known (no
        // completion is drained here, so its fence has not come back)
        e.force_recheck(id);
        let new = storage_of(&mut e);
        assert_ne!(new, old);
        assert!(e.storages.contains_key(&old));
        assert_eq!(checked(&mut e), torrent::CHECK_QUEUED);
        e.on_disk(Completion::CheckProgress { storage: old, checked: 32 });
        assert_eq!(checked(&mut e), torrent::CHECK_QUEUED, "the stopped check's report is dropped");
        // the new check begins at 0 and its reports count; the old one's still do not
        e.on_disk(Completion::CheckStarted { storage: new });
        assert_eq!(checked(&mut e), 0);
        e.on_disk(Completion::CheckProgress { storage: new, checked: 16 });
        e.on_disk(Completion::CheckProgress { storage: old, checked: 48 });
        assert_eq!(checked(&mut e), 16);
        // and none counts once the torrent is not checking
        e.torrent(id).unwrap().checking = false;
        e.on_disk(Completion::CheckProgress { storage: new, checked: 17 });
        assert_eq!(checked(&mut e), 16, "no progress outside a check");
        done_tx.send(()).unwrap();
    });
    let finished = done_rx.recv_timeout(Duration::from_secs(30));
    drop(session);
    let _ = std::fs::remove_dir_all(dir);
    finished.expect("the engine thread ran the checks (a panic there is printed above)");
}
