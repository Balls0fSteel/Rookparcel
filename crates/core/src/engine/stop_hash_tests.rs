//! A piece whose last block is still being written when the torrent stops: the disk thread
//! hashes it behind the stop and the engine learns it has the piece then. The partial piece the
//! stop saved (its other blocks) must go with it, or the blocks count twice in the bytes done
//! and the next resume check drops one restored block (the macOS CI flake of
//! `a_download_stopped_midway_resumes_with_its_names_and_pieces`). The engine is driven by hand,
//! so the completions come exactly where the race puts them.

use super::*;
use crate::disk::CheckResult;
use crate::disk::tests::{make_torrent, temp_dir};
use crate::session::Session;

const PIECE: u32 = 32 * 1024;

fn check_ok(have: Bitfield) -> Result<CheckResult, (String, Error)> {
    Ok(CheckResult { have, resume_failed: false, inconsistent: None, missing: Vec::new(), names: Vec::new() })
}

fn hash_behind_the_stop(verdict: HashVerdict) {
    let (torrent, _) = make_torrent(&[("a.bin", 3 * PIECE as usize)], PIECE);
    let dir = temp_dir("stop-hash");
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
        let old = storage_of(&mut e);
        e.on_disk(Completion::Checked { storage: old, result: check_ok(Bitfield::new(3, false)) });
        assert!(e.torrent(id).unwrap().checked);

        // piece 1: block 0 on disk, block 1 still being written when the torrent stops
        let peer = SocketAddr::from(([127, 0, 0, 2], 6881));
        let picker = e.torrent(id).unwrap().picker.as_mut().unwrap();
        assert!(picker.mark_as_writing(1, 0, peer));
        assert!(picker.mark_as_writing(1, 1, peer));
        e.on_disk(Completion::Write { storage: old, piece: 1, offset: 0, error: None });
        assert_eq!(e.torrent(id).unwrap().total_done(), BLOCK_SIZE as u64);
        e.stop_torrent(id, false);
        let t = e.torrent(id).unwrap();
        assert_eq!(t.unfinished.len(), 1, "the block on disk is saved; the one in flight is not");
        assert_eq!(t.total_done(), BLOCK_SIZE as u64);

        // behind the stop: the last write, and the piece's hash
        e.on_disk(Completion::Write { storage: old, piece: 1, offset: BLOCK_SIZE, error: None });
        e.on_disk(Completion::Hashed { storage: old, piece: 1, verdict });
        let passed = verdict == HashVerdict::Passed;
        let t = e.torrent(id).unwrap();
        let want = if passed { PIECE as u64 } else { 0 };
        assert_eq!(t.total_done(), want, "the piece's blocks count once");
        let saved = t.resume_data();
        assert!(saved.unfinished.is_empty(), "{:?}", saved.unfinished);
        assert_eq!(saved.pieces.as_ref().map(|p| p.get(1)), Some(passed));

        // the restart's resume check finds what the stop left: the bytes done stay as they were
        e.start_torrent(id);
        let new = storage_of(&mut e);
        assert_ne!(new, old);
        let have = saved.pieces.clone().unwrap();
        e.on_disk(Completion::Checked { storage: new, result: check_ok(have) });
        let t = e.torrent(id).unwrap();
        assert!(t.checked);
        assert_eq!(t.total_done(), want, "after the check");
        done_tx.send(()).unwrap();
    });
    let finished = done_rx.recv_timeout(Duration::from_secs(30));
    drop(session);
    let _ = std::fs::remove_dir_all(dir);
    finished.expect("the engine thread ran (a panic there is printed above)");
}

#[test]
fn a_piece_hashed_behind_the_stop_is_had_not_partial() {
    hash_behind_the_stop(HashVerdict::Passed);
}

#[test]
fn a_piece_failed_behind_the_stop_is_downloaded_again_whole() {
    hash_behind_the_stop(HashVerdict::Failed);
}

/// Resume data written so by 1.2.73 and earlier: the piece had and partial too counts once.
#[test]
fn resume_data_with_a_had_piece_partial_too_counts_it_once() {
    let (bytes, _) = make_torrent(&[("a.bin", 3 * PIECE as usize)], PIECE);
    let ti = Arc::new(TorrentInfo::from_bytes(&bytes).unwrap());
    let mut t = Torrent::new(
        TorrentId::default(),
        ti.info_hashes(),
        String::new(),
        "/x".into(),
        TorrentFlags::DEFAULT,
    );
    t.set_metadata(ti);
    let mut r = ResumeData::default();
    let mut have = Bitfield::new(3, false);
    have.set(1, true);
    r.pieces = Some(have);
    r.unfinished = vec![(1, Bitfield::new(2, true)), (2, Bitfield::from_bytes(&[0b1000_0000], 2).unwrap())];
    apply_resume(&mut t, &r);
    assert_eq!(t.unfinished, [(2, Bitfield::from_bytes(&[0b1000_0000], 2).unwrap())]);
    assert_eq!(t.total_done(), PIECE as u64 + BLOCK_SIZE as u64);
}
