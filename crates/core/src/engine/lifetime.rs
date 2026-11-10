//! All-time transfer totals (D65): the payload bytes of every session, kept in `stats.dat`
//! beside the session store so they outlive the torrents they were counted on.
//!
//! The file holds the totals up to the last save; the running session's own counters are added
//! on top. A missing or unreadable file never stops the session: the totals start again, from
//! the sum of the saved torrents' own totals on the first start that has no file.

use std::path::{Path, PathBuf};

use crate::util::{private_file, time::unix_now};

/// The file's name in the session's state folder.
pub(crate) const FILE: &str = "stats.dat";

/// A file larger than this is not ours.
const MAX_FILE: u64 = 4096;

#[derive(Debug, Default)]
pub(super) struct Lifetime {
    path: Option<PathBuf>,
    /// Payload bytes of the sessions before this one.
    base_uploaded: u64,
    base_downloaded: u64,
    /// When counting began, Unix seconds; 0 when unknown (no state folder).
    since: i64,
    /// The session totals at the last save: nothing is written while they stand still.
    saved: Option<(u64, u64)>,
}

/// What [`Lifetime::load`] found.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Loaded {
    /// The file was read.
    File,
    /// No file: a first start, the totals were seeded.
    New,
    /// The file is there and is not readable as ours.
    Damaged(String),
}

impl Lifetime {
    /// Reads `dir`'s file. Without one the totals start at `seed` (the saved torrents' own
    /// uploaded and downloaded sums), so an upgraded install does not begin at zero.
    pub(super) fn load(dir: &Path, seed: (u64, u64)) -> (Lifetime, Loaded) {
        let path = dir.join(FILE);
        let mut l = Lifetime { path: Some(path.clone()), since: unix_now(), ..Lifetime::default() };
        let text = match std::fs::metadata(&path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                (l.base_uploaded, l.base_downloaded) = seed;
                return (l, Loaded::New);
            }
            Err(e) => return (l, Loaded::Damaged(e.to_string())),
            Ok(m) if m.len() > MAX_FILE => return (l, Loaded::Damaged("too large".into())),
            Ok(_) => match std::fs::read_to_string(&path) {
                Ok(t) => t,
                Err(e) => return (l, Loaded::Damaged(e.to_string())),
            },
        };
        match parse(&text) {
            Some((up, down, since)) => {
                l.base_uploaded = up;
                l.base_downloaded = down;
                // a clock set back, or a file from the future: keep the earlier of the two
                l.since = if since > 0 { since.min(l.since) } else { l.since };
                // the file matches a session that moved nothing yet
                l.saved = Some((0, 0));
                (l, Loaded::File)
            }
            None => (l, Loaded::Damaged("not a statistics file".into())),
        }
    }

    /// All-time (uploaded, downloaded) with this session's payload totals on top.
    pub(super) fn totals(&self, session_up: u64, session_down: u64) -> (u64, u64) {
        (self.base_uploaded.saturating_add(session_up), self.base_downloaded.saturating_add(session_down))
    }

    pub(super) fn since(&self) -> i64 {
        self.since
    }

    /// Writes the file if the session moved anything since the last save.
    pub(super) fn save(&mut self, session_up: u64, session_down: u64) -> std::io::Result<()> {
        let Some(path) = &self.path else { return Ok(()) };
        if self.saved == Some((session_up, session_down)) {
            return Ok(());
        }
        let (up, down) = self.totals(session_up, session_down);
        private_file::write(path, format(up, down, self.since).as_bytes())?;
        self.saved = Some((session_up, session_down));
        Ok(())
    }
}

fn format(up: u64, down: u64, since: i64) -> String {
    format!("rookparcel-stats 1\nuploaded={up}\ndownloaded={down}\nsince={since}\n")
}

/// (uploaded, downloaded, since) of a file's text; `None` when it is not ours or a number is
/// missing or malformed. Unknown keys are skipped, for later versions.
fn parse(text: &str) -> Option<(u64, u64, i64)> {
    let mut lines = text.lines();
    if lines.next()?.trim() != "rookparcel-stats 1" {
        return None;
    }
    let (mut up, mut down, mut since) = (None, None, None);
    for line in lines {
        let Some((key, value)) = line.split_once('=') else { continue };
        let value = value.trim();
        match key.trim() {
            "uploaded" => up = Some(value.parse::<u64>().ok()?),
            "downloaded" => down = Some(value.parse::<u64>().ok()?),
            "since" => since = Some(value.parse::<i64>().ok()?),
            _ => {}
        }
    }
    Some((up?, down?, since?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rp-lifetime-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn a_first_start_is_seeded_and_the_totals_round_trip() {
        let dir = temp("round");
        let (mut l, how) = Lifetime::load(&dir, (100, 200));
        assert_eq!(how, Loaded::New);
        assert_eq!(l.totals(0, 0), (100, 200));
        assert!(l.since() > 0);
        let since = l.since();
        // a first start writes its file even when nothing moved, so the seed is taken once
        l.save(0, 0).unwrap();
        assert!(dir.join(FILE).exists());
        l.save(5, 7).unwrap();

        let (mut l, how) = Lifetime::load(&dir, (9999, 9999));
        assert_eq!(how, Loaded::File);
        assert_eq!(l.totals(0, 0), (105, 207));
        assert_eq!(l.totals(1, 2), (106, 209));
        assert_eq!(l.since(), since);
        // nothing moved: nothing is written
        std::fs::remove_file(dir.join(FILE)).unwrap();
        l.save(0, 0).unwrap();
        assert!(!dir.join(FILE).exists());
        l.save(1, 0).unwrap();
        assert!(dir.join(FILE).exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_damaged_file_starts_again_without_the_seed() {
        let dir = temp("damaged");
        for bad in [
            "",
            "<html>",
            "rookparcel-stats 1\nuploaded=1\n",
            "rookparcel-stats 1\nuploaded=x\ndownloaded=1\nsince=1\n",
            "rookparcel-stats 1\nuploaded=-1\ndownloaded=1\nsince=1\n",
        ] {
            std::fs::write(dir.join(FILE), bad).unwrap();
            let (l, how) = Lifetime::load(&dir, (100, 200));
            assert!(matches!(how, Loaded::Damaged(_)), "{bad:?}");
            assert_eq!(l.totals(3, 4), (3, 4), "{bad:?}");
        }
        std::fs::write(dir.join(FILE), vec![b'1'; 5000]).unwrap();
        assert!(matches!(Lifetime::load(&dir, (0, 0)).1, Loaded::Damaged(_)));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn unknown_keys_are_skipped_and_the_totals_saturate() {
        let text = "rookparcel-stats 1\nlater=abc\nuploaded=18446744073709551615\ndownloaded=2\nsince=5\n";
        assert_eq!(parse(text), Some((u64::MAX, 2, 5)));
        let dir = temp("saturate");
        std::fs::write(dir.join(FILE), text).unwrap();
        let (l, how) = Lifetime::load(&dir, (0, 0));
        assert_eq!(how, Loaded::File);
        assert_eq!(l.totals(10, 10), (u64::MAX, 12));
        assert_eq!(l.since(), 5);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn without_a_state_folder_nothing_is_written() {
        let mut l = Lifetime::default();
        l.save(1, 2).unwrap();
        assert_eq!(l.totals(1, 2), (1, 2));
        assert_eq!(l.since(), 0);
    }
}
