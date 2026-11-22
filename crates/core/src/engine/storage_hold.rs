// Rookparcel's own code, not ported (D79). libtorrent stops a torrent whose files are not there
// with a file error and leaves it to the user; qBittorrent shows that as "Missing files".

//! Torrents held because their storage is missing: a resume check that found none of the
//! files its pieces are in, or a write whose save path vanished (a drive pulled, a share or
//! mount not there). The torrent stops with an error and keeps its pieces; every
//! [`PROBE_INTERVAL`] the paths are looked at on a thread of their own, and a torrent whose
//! path is back starts again, its resume check finding the files as they were.

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use super::Engine;
use crate::alert::Alert;
use crate::types::TorrentId;

/// How often the paths of held torrents are looked at.
pub(super) const PROBE_INTERVAL: Duration = Duration::from_secs(30);

/// Kept here, not in `Torrent`, which stays compact for 100,000 stopped ones.
#[derive(Default)]
pub(super) struct StorageHolds {
    /// Held torrent → the path whose return ends the hold.
    pub held: HashMap<TorrentId, String>,
    /// A probe is out ([`crate::disk::DiskIo::async_probe_paths`]).
    pub probing: bool,
    next_probe: Option<Instant>,
}

/// The torrent's error while it is held.
pub(super) fn hold_message(path: &str) -> String {
    format!(
        "{path}: files missing (the drive, share or folder is not available). Started again when \
         it is back; Force Recheck downloads them again"
    )
}

impl Engine {
    /// Stops `id` with the "files missing" error until `path` exists again. Its pieces are
    /// untouched: the stored set of a check that failed stays (`stop_torrent`), and a running
    /// torrent's picker set is kept as at any stop.
    pub(super) fn hold_for_storage(&mut self, id: TorrentId, path: String) {
        let Some(t) = self.torrent(id) else { return };
        t.error = Some(hold_message(&path));
        t.status_dirty = true;
        self.alert(Alert::Log {
            id: Some(id),
            message: format!(
                "{path} is not there: held until it is back (every {} s)",
                PROBE_INTERVAL.as_secs()
            ),
        });
        self.holds.held.insert(id, path);
        if self.holds.next_probe.is_none() {
            self.holds.next_probe = Some(Instant::now() + PROBE_INTERVAL);
        }
        self.update_running(id);
        self.needs_auto_manage = true;
    }

    /// The hold ends without the path coming back: the user started or rechecked the torrent,
    /// or removed it.
    pub(super) fn release_storage_hold(&mut self, id: TorrentId) {
        self.holds.held.remove(&id);
    }

    pub(super) fn storage_hold_tick(&mut self, now: Instant) {
        if self.holds.held.is_empty() {
            self.holds.next_probe = None;
            return;
        }
        if self.holds.probing || self.holds.next_probe.is_some_and(|at| now < at) {
            return;
        }
        let paths: HashSet<&String> = self.holds.held.values().collect();
        let paths = paths.into_iter().cloned().collect();
        self.holds.probing = self.disk.async_probe_paths(paths);
        self.holds.next_probe = Some(now + PROBE_INTERVAL);
    }

    pub(super) fn on_paths_probed(&mut self, present: Vec<String>) {
        self.holds.probing = false;
        let present: HashSet<String> = present.into_iter().collect();
        let back: Vec<TorrentId> =
            self.holds.held.iter().filter(|(_, p)| present.contains(*p)).map(|(&id, _)| id).collect();
        for id in back {
            let Some(path) = self.holds.held.remove(&id) else { continue };
            let Some(t) = self.torrent(id) else { continue };
            // a hold whose error the user replaced meanwhile (a Start that failed otherwise)
            if t.error.as_deref() != Some(hold_message(&path).as_str()) {
                continue;
            }
            t.error = None;
            t.status_dirty = true;
            self.alert(Alert::Log { id: Some(id), message: format!("{path} is back: checking the files") });
            self.update_running(id);
            self.needs_auto_manage = true;
        }
    }
}
