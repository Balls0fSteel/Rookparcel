//! µTorrent's torrent actions expressed with libtorrent's flags.
//!
//! | µTorrent    | libtorrent                          | Shows as                         |
//! |-------------|-------------------------------------|----------------------------------|
//! | Start       | set `AUTO_MANAGED`, `resume()`      | Downloading/Seeding or Queued    |
//! | Force Start | unset `AUTO_MANAGED`, `resume()`    | [F] Downloading / [F] Seeding    |
//! | Pause       | unset `AUTO_MANAGED`, `pause()`     | Paused (frontend remembers)      |
//! | Stop        | unset `AUTO_MANAGED`, `pause()`     | Stopped / Finished               |
//!
//! Pause and Stop are the same in the engine. The frontend records which one the user chose
//! (see [`crate::status::status_text`]).

use rookparcel_core::{SessionHandle, TorrentFlags, TorrentHandle, TorrentId};

use crate::model::TorrentList;

/// The context menu's Bandwidth Allocation presets in kB/s; 0 = unlimited.
pub const LIMITS: [u32; 10] = [0, 10, 25, 50, 100, 250, 500, 1000, 2500, 5000];

/// A preset's menu text: "Unlimited" or "50 kB/s".
pub fn limit_text(kib: u32) -> String {
    if kib == 0 {
        crate::tr!(MenuUnlimited).to_owned()
    } else {
        format!("{kib} {}", crate::tr!(UnitKbPerSec))
    }
}

pub fn start(t: &TorrentHandle) {
    t.set_flags(TorrentFlags::AUTO_MANAGED);
    t.resume();
}

pub fn force_start(t: &TorrentHandle) {
    t.unset_flags(TorrentFlags::AUTO_MANAGED);
    t.resume();
}

/// Recheck. The engine checks a torrent only while it runs, so a stopped one is started as by
/// Start, or the check would wait for a Start nobody guesses; the user may stop it after. The
/// caller clears the Pause mark when `stopped`.
pub fn recheck(t: &TorrentHandle, stopped: bool) {
    t.force_recheck();
    if stopped {
        start(t);
    }
}

/// Also used for Pause; the caller records the difference.
pub fn stop(t: &TorrentHandle) {
    t.unset_flags(TorrentFlags::AUTO_MANAGED);
    t.pause();
}

/// Every torrent of a non-empty selection has `flag`: the check mark of a flag's menu item
/// (Initial Seeding, Download in Sequential Order, First and Last Pieces First).
pub fn all_flagged(list: &TorrentList, ids: &[TorrentId], flag: TorrentFlags) -> bool {
    !ids.is_empty() && ids.iter().all(|&id| list.get(id).is_some_and(|s| s.flags.contains(flag)))
}

/// A flag's menu item toggles as in µTorrent: off for all when every torrent has it, else on
/// for all.
pub fn toggle_flag(handle: &SessionHandle, list: &TorrentList, ids: &[TorrentId], flag: TorrentFlags) {
    let on = all_flagged(list, ids, flag);
    for &id in ids {
        let t = handle.torrent(id);
        if on {
            t.unset_flags(flag);
        } else {
            t.set_flags(flag);
        }
    }
    handle.post_torrent_updates();
}

/// Some torrent of the selection is not finished: the download order items are enabled.
pub fn any_unfinished(list: &TorrentList, ids: &[TorrentId]) -> bool {
    ids.iter().any(|&id| list.get(id).is_some_and(|s| !s.is_finished))
}

/// Some torrent of the selection has its metadata: Export .torrent is enabled.
pub fn any_has_metadata(list: &TorrentList, ids: &[TorrentId]) -> bool {
    ids.iter().any(|&id| list.get(id).is_some_and(|s| s.has_metadata))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rookparcel_core::{AddTorrentParams, Alert, MockConfig, Session, Settings, TorrentSource};
    use std::time::{Duration, Instant};

    #[test]
    fn actions_drive_engine_flags() {
        let s =
            Session::start_mock(Settings { tick_interval: 50, ..Settings::default() }, MockConfig::default());
        let h = s.handle();
        let magnet = "magnet:?xt=urn:btih:0123456789abcdef0123456789abcdef01234567&dn=t".to_owned();
        h.add_torrent(AddTorrentParams::new(TorrentSource::Magnet(magnet), "/dl"));

        let mut alerts = Vec::new();
        let mut id = None;
        let mut flags = None;
        let mut step = 0;
        let deadline = Instant::now() + Duration::from_secs(10);
        while step < 3 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
            h.post_torrent_updates();
            h.pop_alerts(&mut alerts);
            for a in &alerts {
                match a {
                    Alert::TorrentAdded { id: x, .. } => id = Some(*x),
                    Alert::StateUpdate(v) => {
                        if let Some(st) = v.iter().find(|st| Some(st.id) == id) {
                            flags = Some(st.flags);
                        }
                    }
                    _ => {}
                }
            }
            let (Some(id), Some(f)) = (id, flags) else { continue };
            let t = h.torrent(id);
            let auto = f.contains(TorrentFlags::AUTO_MANAGED);
            let paused = f.contains(TorrentFlags::PAUSED);
            match step {
                0 if auto && !paused => {
                    force_start(&t);
                    step = 1;
                }
                1 if !auto && !paused => {
                    stop(&t);
                    step = 2;
                }
                2 if !auto && paused => step = 3,
                _ => {}
            }
        }
        assert_eq!(step, 3, "start -> force start -> stop did not complete");
    }
}
