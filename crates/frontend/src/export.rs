//! Torrent menu > Copy Magnet Link and Export .torrent (D73), for every GUI.
//!
//! [`magnet_for`] builds a torrent's magnet link from its status and tracker list;
//! [`magnet_links`] gives Copy Magnet Link's text for a selection. [`Exports`] asks the engine
//! for the `.torrent` files (`post_torrent_file`) and writes each where the user chose when
//! its `Alert::TorrentFile` comes. Other parts of the frontend ask for `.torrent` files too
//! (the Add dialog's magnets, the `.torrent` folders): every one of them sees every such alert
//! and takes only the torrents it asked for.

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use rookparcel_core::metainfo::{MagnetUri, make_magnet_uri};
use rookparcel_core::{Alert, AnnounceEntry, SessionHandle, TorrentId, TorrentStatus};

use crate::model::TorrentList;
use crate::torrent_folders::file_name;
use crate::tr;

/// A torrent's magnet link: its v1 (`btih`) and v2 (`btmh`) info-hashes (both for a hybrid),
/// its name and its trackers. No peers and no file selection.
pub fn magnet_for(status: &TorrentStatus, trackers: &[AnnounceEntry]) -> String {
    let m = MagnetUri {
        info_hashes: status.info_hashes,
        name: status.name.clone(),
        trackers: trackers.iter().map(|t| t.url.clone()).collect(),
        tracker_tiers: trackers.iter().map(|t| t.tier).collect(),
        ..MagnetUri::default()
    };
    make_magnet_uri(&m)
}

/// Copy Magnet Link's text for `ids`: one link per line (`\n`), in their order. The tracker
/// lists come from the engine, waiting up to `wait` ([`crate::properties::fetch_trackers`]); a
/// torrent whose list is late goes out without trackers. The alerts popped meanwhile are
/// appended to `popped`, for the GUI's alert loop.
pub fn magnet_links(
    handle: &SessionHandle,
    list: &TorrentList,
    ids: &[TorrentId],
    popped: &mut Vec<Alert>,
    wait: Duration,
) -> String {
    let ids: Vec<TorrentId> = ids.iter().copied().filter(|&id| list.get(id).is_some()).collect();
    let trackers = crate::properties::fetch_trackers(handle, &ids, popped, wait);
    let links: Vec<String> = ids
        .iter()
        .filter_map(|id| Some(magnet_for(list.get(*id)?, trackers.get(id).map_or(&[][..], Vec::as_slice))))
        .collect();
    links.join("\n")
}

/// The file name a Save As dialog offers for torrent `name`: `<name>.torrent`, with what file
/// systems refuse replaced ([`file_name`]).
pub fn export_file_name(name: &str) -> String {
    format!("{}.torrent", file_name(name))
}

/// Where Export .torrent writes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExportTarget {
    /// One torrent: the file chosen in a Save As dialog, replaced when it exists (the dialog
    /// asked).
    File(PathBuf),
    /// Several: a folder, each torrent as `<name>.torrent` ([`export_file_name`]), then
    /// `<name> (2).torrent`, `<name> (3).torrent`... when the name is taken. Nothing there is
    /// replaced.
    Folder(PathBuf),
}

/// One export done: the torrent's name, and the file written or why not.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exported {
    pub name: String,
    pub result: Result<PathBuf, String>,
    /// An export to a chosen file ([`ExportTarget::File`]): a GUI shows a failure in a message
    /// box (`CannotExport` and the error); a folder export's failures are only logged.
    pub to_file: bool,
}

impl Exported {
    /// The Logger line: `ExportDone` or `ExportFailed`.
    pub fn log_line(&self) -> String {
        match &self.result {
            Ok(path) => tr!(ExportDone, name = self.name, path = path.display()),
            Err(error) => tr!(ExportFailed, name = self.name, error),
        }
    }
}

/// The exports asked for and not written yet. A GUI owns one, calls [`Exports::request`] from
/// the menu item and hands it every alert ([`Exports::on_alert`]).
#[derive(Debug, Default)]
pub struct Exports {
    pending: HashMap<TorrentId, (String, ExportTarget)>,
}

impl Exports {
    pub fn new() -> Exports {
        Exports::default()
    }

    /// Asks the engine for the `.torrent` of each of `torrents` (id and name, the name for the
    /// file in a folder and for the Logger), to be written to `target`. A torrent asked for
    /// again before its file came is written once, to the new target.
    pub fn request(
        &mut self,
        handle: &SessionHandle,
        torrents: &[(TorrentId, String)],
        target: &ExportTarget,
    ) {
        for (id, name) in torrents {
            self.expect(*id, name, target);
            handle.torrent(*id).post_torrent_file();
        }
    }

    fn expect(&mut self, id: TorrentId, name: &str, target: &ExportTarget) {
        self.pending.insert(id, (name.to_owned(), target.clone()));
    }

    /// Whether an export of `id` waits for its file.
    pub fn is_pending(&self, id: TorrentId) -> bool {
        self.pending.contains_key(&id)
    }

    /// The engine handed out `id`'s `.torrent` (`None`: a magnet without its metadata yet):
    /// written if it was asked for. `None` when it was not.
    pub fn on_torrent_file(&mut self, id: TorrentId, data: Option<&[u8]>) -> Option<Exported> {
        let (name, target) = self.pending.remove(&id)?;
        let result = match data {
            None => Err(tr!(ExportNoMetadata).to_owned()),
            Some(bytes) => write(&target, &name, bytes).map_err(|e| e.to_string()),
        };
        Some(Exported { name, result, to_file: matches!(target, ExportTarget::File(_)) })
    }

    /// Follows the session's alerts: the `.torrent` files asked for, and torrents removed before
    /// theirs came (forgotten).
    pub fn on_alert(&mut self, alert: &Alert) -> Option<Exported> {
        match alert {
            Alert::TorrentFile { id, data } => self.on_torrent_file(*id, data.as_deref()),
            Alert::TorrentRemoved { id, .. } => {
                self.pending.remove(id);
                None
            }
            _ => None,
        }
    }
}

/// Writes `bytes` as torrent `name`'s file at `target`; the path written.
fn write(target: &ExportTarget, name: &str, bytes: &[u8]) -> io::Result<PathBuf> {
    match target {
        ExportTarget::File(path) => {
            fs::write(path, bytes)?;
            Ok(path.clone())
        }
        ExportTarget::Folder(dir) => write_new(dir, name, bytes),
    }
}

/// `bytes` in `dir` under the first free name of `<name>.torrent`, `<name> (2).torrent`, ...
fn write_new(dir: &Path, name: &str, bytes: &[u8]) -> io::Result<PathBuf> {
    let base = file_name(name);
    for i in 1..1000 {
        let file = if i == 1 { format!("{base}.torrent") } else { format!("{base} ({i}).torrent") };
        let path = dir.join(file);
        match fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut f) => {
                if let Err(e) = f.write_all(bytes) {
                    drop(f);
                    let _ = fs::remove_file(&path);
                    return Err(e);
                }
                return Ok(path);
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    Err(io::Error::new(io::ErrorKind::AlreadyExists, "too many files of the same name"))
}

#[cfg(test)]
mod tests {
    use rookparcel_core::metainfo::{TorrentInfo, parse_magnet_uri};
    use rookparcel_core::{InfoHashes, Sha1Hash, Sha256Hash};

    use super::*;

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("rookparcel-export-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn status(hashes: InfoHashes, name: &str) -> TorrentStatus {
        TorrentStatus { info_hashes: hashes, name: name.into(), ..TorrentStatus::default() }
    }

    #[test]
    fn magnet_links_for_v1_v2_and_hybrid() {
        let v1 = Sha1Hash([0xab; 20]);
        let v2 = Sha256Hash([0x12; 32]);
        let trackers =
            [AnnounceEntry::new("udp://t:1/announce", 0), AnnounceEntry::new("https://u/announce?k=1", 1)];

        let link = magnet_for(&status(InfoHashes::v1(v1), "a b"), &[]);
        assert_eq!(link, format!("magnet:?xt=urn:btih:{}&dn=a%20b", "ab".repeat(20)));
        let link = magnet_for(&status(InfoHashes::v1(v1), "a b"), &trackers);
        assert_eq!(
            link,
            format!(
                "magnet:?xt=urn:btih:{}&dn=a%20b&tr=udp%3a%2f%2ft%3a1%2fannounce&tr=https%3a%2f%2fu%2fannounce%3fk%3d1",
                "ab".repeat(20)
            )
        );

        let only_v2 = InfoHashes { v1: None, v2: Some(v2) };
        let link = magnet_for(&status(only_v2, "v2"), &trackers);
        assert!(
            link.starts_with(&format!("magnet:?xt=urn:btmh:1220{}&dn=v2&tr=", "12".repeat(32))),
            "{link}"
        );
        let m = parse_magnet_uri(&link).unwrap();
        assert_eq!((m.info_hashes, m.name.as_str()), (only_v2, "v2"));
        assert_eq!(m.trackers, ["udp://t:1/announce", "https://u/announce?k=1"]);

        let hybrid = InfoHashes { v1: Some(v1), v2: Some(v2) };
        let link = magnet_for(&status(hybrid, "ünï"), &trackers);
        assert!(link.contains("xt=urn:btih:") && link.contains("&xt=urn:btmh:1220"), "{link}");
        assert!(!link.contains("x.pe") && !link.contains("so="), "{link}");
        let m = parse_magnet_uri(&link).unwrap();
        assert_eq!((m.info_hashes, m.name.as_str(), m.trackers.len()), (hybrid, "ünï", 2));

        // no name: no dn
        assert_eq!(
            magnet_for(&status(InfoHashes::v1(v1), ""), &[]),
            format!("magnet:?xt=urn:btih:{}", "ab".repeat(20))
        );
    }

    /// A small multi-file `.torrent` named `name`.
    fn torrent(name: &str) -> Vec<u8> {
        let mut b = format!(
            "d8:announce9:udp://t:14:infod5:filesld6:lengthi3e4:pathl1:aeed6:lengthi4e4:pathl1:beee4:name{}:{name}12:piece lengthi16384e6:pieces20:",
            name.len()
        )
        .into_bytes();
        b.extend([b'p'; 20]);
        b.extend(b"ee");
        b
    }

    #[test]
    fn an_export_writes_the_torrent_file() {
        let dir = temp("file");
        let bytes = torrent("Show");
        let want = TorrentInfo::from_bytes(&bytes).unwrap().info_hashes();
        let mut ex = Exports::new();
        let id = TorrentId::from_raw(1);
        let path = dir.join("chosen.torrent");
        fs::write(&path, b"old").unwrap();
        ex.expect(id, "Show", &ExportTarget::File(path.clone()));
        assert!(ex.is_pending(id));
        // another torrent's file is not this export's
        assert!(ex.on_torrent_file(TorrentId::from_raw(2), Some(&bytes)).is_none());
        let done = ex.on_alert(&Alert::TorrentFile { id, data: Some(bytes.clone().into()) }).unwrap();
        assert_eq!(done, Exported { name: "Show".into(), result: Ok(path.clone()), to_file: true });
        assert_eq!(done.log_line(), format!("Exported Show to {}", path.display()));
        // the chosen file is replaced, and parses back to the same torrent
        let back = TorrentInfo::from_bytes(&fs::read(&path).unwrap()).unwrap();
        assert_eq!(back.info_hashes(), want);
        assert!(!ex.is_pending(id), "written once");
        assert!(ex.on_torrent_file(id, Some(&bytes)).is_none());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn a_folder_export_numbers_the_names_it_finds_taken() {
        let dir = temp("folder");
        let mut ex = Exports::new();
        let target = ExportTarget::Folder(dir.clone());
        fs::write(dir.join("a_b.torrent"), b"someone else's").unwrap();
        let (one, two, three) = (TorrentId::from_raw(1), TorrentId::from_raw(2), TorrentId::from_raw(3));
        ex.expect(one, "a/b", &target);
        ex.expect(two, "a/b", &target);
        ex.expect(three, "c", &target);
        let first = ex.on_torrent_file(one, Some(&torrent("x"))).unwrap();
        let second = ex.on_torrent_file(two, Some(&torrent("y"))).unwrap();
        assert_eq!(first.result, Ok(dir.join("a_b (2).torrent")));
        assert_eq!(second.result, Ok(dir.join("a_b (3).torrent")));
        assert!(!first.to_file);
        assert_eq!(fs::read(dir.join("a_b.torrent")).unwrap(), b"someone else's", "never overwritten");
        assert_eq!(fs::read(dir.join("a_b (3).torrent")).unwrap(), torrent("y"));
        // a removed torrent is forgotten
        assert!(
            ex.on_alert(&Alert::TorrentRemoved { id: three, info_hashes: InfoHashes::default() }).is_none()
        );
        assert!(!ex.is_pending(three));
        assert_eq!(export_file_name("a:b?"), "a_b_.torrent");
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn a_magnet_without_metadata_cannot_be_exported() {
        let dir = temp("none");
        let mut ex = Exports::new();
        let id = TorrentId::from_raw(7);
        ex.expect(id, "magnet", &ExportTarget::File(dir.join("m.torrent")));
        let done = ex.on_alert(&Alert::TorrentFile { id, data: None }).unwrap();
        assert_eq!(done.result, Err(tr!(ExportNoMetadata).to_owned()));
        assert_eq!(done.log_line(), format!("Could not export magnet: {}", tr!(ExportNoMetadata)));
        assert!(!dir.join("m.torrent").exists());
        // a write that fails says why
        ex.expect(id, "x", &ExportTarget::File(dir.join("no such folder").join("x.torrent")));
        let done = ex.on_torrent_file(id, Some(b"d4:infodee")).unwrap();
        assert!(done.result.is_err() && done.to_file);
        let _ = fs::remove_dir_all(dir);
    }
}
