//! Preferences > UI Settings > "Do not download files named" (D74): files whose names match the
//! list start unchecked when a torrent is added. µTorrent 2.2.1 has no such option; qBittorrent
//! has it under Downloads ("Excluded file names").
//!
//! A file is matched by its own name (the last part of its path), without case, against
//! [`SkipPatterns`]. Pad files never count. When every file of a torrent would be left out, none
//! is: a torrent made only of such files is wanted for them.
//!
//! Where it applies, in every interface:
//! - the Add New Torrent window starts those files unchecked ([`crate::add::FileTree::for_add`]),
//!   and the user can tick them back;
//! - a torrent added without the window gets the priorities in its add parameters, a `.torrent`
//!   at once and a magnet once its metadata has arrived ([`Pending`]);
//! - never a torrent restored at start, an imported one, or one from the Add window (the user's
//!   choice there stands).

use std::collections::HashMap;

use rookparcel_core::metainfo::{TorrentInfo, parse_magnet_uri};
use rookparcel_core::{
    AddTorrentParams, Alert, DownloadPriority, FileEntry, InfoHashes, SessionHandle, TorrentId, TorrentSource,
};

use crate::options::GuiOptions;
use crate::patterns::SkipPatterns;

/// The most patterns the list keeps.
pub const MAX_PATTERNS: usize = 256;
/// The longest the kept list may be, in bytes.
pub const MAX_BYTES: usize = 4096;

/// The list as the preferences keep it and the Preferences show it: the patterns, trimmed,
/// separated by `; ` (whatever separated them: `;`, `|` or line breaks). At most
/// [`MAX_PATTERNS`] patterns and [`MAX_BYTES`] bytes are kept, cut after the last whole pattern
/// that fits; the rest is dropped, without an error, so that no client (a qBittorrent API
/// client of nox, say) can make every add slow with a huge list.
pub fn to_stored(text: &str) -> String {
    let mut out = String::new();
    for p in text.split(['|', ';', '\n', '\r']).map(str::trim).filter(|p| !p.is_empty()).take(MAX_PATTERNS) {
        let sep = if out.is_empty() { "" } else { "; " };
        if out.len() + sep.len() + p.len() > MAX_BYTES {
            break;
        }
        out.push_str(sep);
        out.push_str(p);
    }
    out
}

/// The patterns in force ([`to_stored`]'s bounds apply): `None` while the option is off or the
/// list empty.
pub fn patterns(o: &GuiOptions) -> Option<SkipPatterns> {
    if !o.exclude_names {
        return None;
    }
    Some(SkipPatterns::parse(&to_stored(&o.exclude_list))).filter(|p| !p.is_empty())
}

/// The last part of a path (`/` or `\` separated).
fn file_name(path: &str) -> &str {
    path.rsplit(['/', '\\']).find(|s| !s.is_empty()).unwrap_or(path)
}

/// Per file, (path or name, is a pad file) in torrent order: whether `p` leaves it out. Pad files
/// never; nothing when every other file would be.
pub fn mask<'a>(files: impl IntoIterator<Item = (&'a str, bool)>, p: &SkipPatterns) -> Vec<bool> {
    let mut out = Vec::new();
    let mut kept = false;
    for (path, pad) in files {
        let excluded = !pad && p.matches(file_name(path));
        kept |= !pad && !excluded;
        out.push(excluded);
    }
    if !kept {
        out.fill(false);
    }
    out
}

/// Per file of `ti`: whether patterns `p` leave it out ([`mask`]).
pub fn excluded_by(ti: &TorrentInfo, p: &SkipPatterns) -> Vec<bool> {
    let fs = ti.files();
    mask((0..fs.num_files()).map(|i| (fs.file_name(i), fs.pad_file_at(i))), p)
}

/// Per file of `ti`: whether the preferences leave it out (all `false` while the option is off).
pub fn excluded(ti: &TorrentInfo, o: &GuiOptions) -> Vec<bool> {
    match patterns(o) {
        Some(p) => excluded_by(ti, &p),
        None => vec![false; ti.files().num_files()],
    }
}

/// Priorities from a [`mask`]: Don't Download for the files left out and pad files, Normal for
/// the rest; `None` when nothing is left out.
fn priorities_of(mask: &[bool], pads: impl Iterator<Item = bool>) -> Option<Vec<DownloadPriority>> {
    if !mask.contains(&true) {
        return None;
    }
    Some(
        mask.iter()
            .zip(pads)
            .map(
                |(&out, pad)| {
                    if out || pad { DownloadPriority::DONT_DOWNLOAD } else { DownloadPriority::NORMAL }
                },
            )
            .collect(),
    )
}

/// The file priorities a torrent added without the Add New Torrent window gets: `None` while
/// the option is off or no file matches.
pub fn priorities(ti: &TorrentInfo, o: &GuiOptions) -> Option<Vec<DownloadPriority>> {
    priorities_by(ti, &patterns(o)?)
}

fn priorities_by(ti: &TorrentInfo, p: &SkipPatterns) -> Option<Vec<DownloadPriority>> {
    let fs = ti.files();
    priorities_of(&excluded_by(ti, p), (0..fs.num_files()).map(|i| fs.pad_file_at(i)))
}

/// [`priorities`] for the engine's file list of a torrent in the session.
fn priorities_of_entries(files: &[FileEntry], p: &SkipPatterns) -> Option<Vec<DownloadPriority>> {
    let mask = mask(files.iter().map(|f| (f.path.as_str(), f.pad_file)), p);
    priorities_of(&mask, files.iter().map(|f| f.pad_file))
}

/// For an add without the Add New Torrent window: fills `p.file_priorities` from the
/// preferences when `p` is a `.torrent` and its caller set no priorities. True when it did.
/// A magnet has no file list yet: see [`Pending::add`].
pub fn apply_to_new(p: &mut AddTorrentParams, o: &GuiOptions) -> bool {
    if !p.file_priorities.is_empty() {
        return false;
    }
    let Some(patterns) = patterns(o) else { return false };
    let TorrentSource::TorrentFile(bytes) = &p.source else { return false };
    // a file the engine cannot read is handed to it all the same, so it reports why
    let Some(v) = TorrentInfo::from_bytes(bytes).ok().and_then(|ti| priorities_by(&ti, &patterns)) else {
        return false;
    };
    p.file_priorities = v;
    true
}

/// Whether two sets of info-hashes name the same torrent: they share a hash the engine knows
/// torrents by (v1, or the truncated v2), as its duplicate check does.
pub fn same_torrent(a: &InfoHashes, b: &InfoHashes) -> bool {
    a.wire_hashes().any(|x| b.wire_hashes().any(|y| x == y))
}

/// Torrents added without the Add New Torrent window whose files are not known yet (magnets):
/// once a magnet's metadata has arrived, its files are asked for (`post_file_list`) and the ones
/// the list leaves out are set to Don't Download (`prioritize_files`). A GUI owns one, calls
/// [`Pending::add`] for each such add and hands it every alert ([`Pending::on_alert`]). The
/// patterns are the ones in force at the add. Not kept over a restart.
#[derive(Debug, Default)]
pub struct Pending {
    /// By info-hash (`InfoHashes::key_hex`), until the engine reports the torrent added.
    keys: HashMap<String, SkipPatterns>,
    /// By torrent, until its file list came; whether its metadata has arrived since the add.
    /// A magnet already in the session is reported added under its existing id, so only a file
    /// list after `MetadataReceived` counts: an existing torrent's (the Files tab's) never does,
    /// and its file choices stand.
    ids: HashMap<TorrentId, (SkipPatterns, bool)>,
}

impl Pending {
    pub fn new() -> Pending {
        Pending::default()
    }

    /// A torrent added without the Add New Torrent window, before it goes to the engine: a
    /// `.torrent` gets its priorities now ([`apply_to_new`]), a magnet is remembered for its
    /// metadata. Never for a magnet that waits for the Add window
    /// ([`crate::add::AwaitingMetadata`]): its window starts the files unchecked.
    ///
    /// `in_session` answers whether the owner already has a torrent of these info-hashes (its
    /// torrent list, [`same_torrent`]). Such a magnet is not remembered: the engine reports the
    /// add under the existing torrent's id, and that torrent (restored at start, or waiting for
    /// the Add window with its own choices to come) keeps its files as they are, even when its
    /// metadata arrives later. A duplicate `.torrent` needs no such care: the engine leaves the
    /// existing torrent as it is and ignores the new parameters, priorities included.
    pub fn add(
        &mut self,
        p: &mut AddTorrentParams,
        o: &GuiOptions,
        in_session: impl FnOnce(&InfoHashes) -> bool,
    ) {
        if apply_to_new(p, o) || !p.file_priorities.is_empty() {
            return;
        }
        let Some(patterns) = patterns(o) else { return };
        let hashes = match &p.source {
            TorrentSource::Magnet(uri) => parse_magnet_uri(uri).ok().map(|m| m.info_hashes),
            TorrentSource::InfoHash(h) => Some(*h),
            _ => None,
        };
        if let Some(h) = hashes.filter(|h| !in_session(h)) {
            self.keys.insert(h.key_hex(), patterns);
        }
    }

    /// Follows the engine: an added magnet's id, its metadata (its files are asked for), its
    /// file list (the priorities are set), and torrents removed or not added (forgotten).
    pub fn on_alert(&mut self, alert: &Alert, handle: &SessionHandle) {
        if self.keys.is_empty() && self.ids.is_empty() {
            return;
        }
        match alert {
            Alert::TorrentAdded { id, info_hashes, .. } => {
                if let Some(p) = self.keys.remove(&info_hashes.key_hex()) {
                    self.ids.insert(*id, (p, false));
                }
            }
            Alert::AddTorrentFailed { info_hashes: Some(h), .. } => {
                self.keys.remove(&h.key_hex());
            }
            Alert::MetadataReceived { id } => {
                if let Some((_, arrived)) = self.ids.get_mut(id) {
                    *arrived = true;
                    handle.torrent(*id).post_file_list();
                }
            }
            // an empty list: asked for before the metadata came (the Files tab); one before
            // `MetadataReceived`: a torrent that was in the session already
            Alert::FileList { id, files } if !files.is_empty() && self.ids.get(id).is_some_and(|e| e.1) => {
                if let Some((p, _)) = self.ids.remove(id) {
                    if let Some(v) = priorities_of_entries(files, &p) {
                        handle.torrent(*id).prioritize_files(v);
                    }
                }
            }
            Alert::TorrentRemoved { id, .. } => {
                self.ids.remove(id);
            }
            _ => {}
        }
    }

    /// Nothing waits.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty() && self.ids.is_empty()
    }

    /// Whether the magnet `id` waits for its file list here.
    pub fn holds(&self, id: TorrentId) -> bool {
        self.ids.contains_key(&id)
    }
}

#[cfg(test)]
mod tests {
    use rookparcel_core::{InfoHashes, Sha1Hash, Sha256Hash};

    use super::*;
    use crate::prefs::Prefs;

    fn gui(on: bool, list: &str) -> GuiOptions {
        let mut o = GuiOptions::load(&Prefs::default(), "/dl");
        o.exclude_names = on;
        o.exclude_list = list.into();
        o
    }

    /// A `.torrent` of these files (path components below the torrent's folder, pad file).
    fn torrent(files: &[(&[&str], bool)]) -> Vec<u8> {
        let bstr = |s: &str| format!("{}:{s}", s.len());
        let mut list = String::new();
        for (parts, pad) in files {
            let path: String = parts.iter().map(|p| bstr(p)).collect();
            let attr = if *pad { "4:attr1:p" } else { "" };
            list.push_str(&format!("d{attr}6:lengthi16384e4:pathl{path}ee"));
        }
        let n = files.len();
        let mut b = format!("d4:infod5:filesl{list}e4:name4:Show12:piece lengthi16384e6:pieces{}:", 20 * n)
            .into_bytes();
        b.extend(vec![b'a'; 20 * n]);
        b.extend(b"ee");
        b
    }

    fn info(files: &[(&[&str], bool)]) -> TorrentInfo {
        TorrentInfo::from_bytes(&torrent(files)).unwrap()
    }

    const N: DownloadPriority = DownloadPriority::NORMAL;
    const X: DownloadPriority = DownloadPriority::DONT_DOWNLOAD;

    #[test]
    fn the_list_is_stored_tidy() {
        assert_eq!(to_stored(" *.lnk |Thumbs.db;;\r\n*sample* \n"), "*.lnk; Thumbs.db; *sample*");
        assert_eq!(to_stored(" ; \n"), "");
        assert_eq!(to_stored(&to_stored("a;b")), "a; b");
    }

    /// A huge list is cut at a pattern: at most 256 patterns and 4096 bytes.
    #[test]
    fn the_list_is_bounded() {
        let many: Vec<String> = (0..1000).map(|i| format!("p{i}")).collect();
        let kept = to_stored(&many.join("\n"));
        assert_eq!(kept.split("; ").count(), MAX_PATTERNS);
        assert!(kept.ends_with("; p255"));
        // long patterns: the last whole one that fits
        let long = "x".repeat(1000);
        let kept = to_stored(&[long.as_str(); 10].join(";"));
        assert_eq!(kept, [long.as_str(); 4].join("; "));
        assert!(kept.len() <= MAX_BYTES);
        // a single pattern beyond the bound: nothing
        assert_eq!(to_stored(&"y".repeat(MAX_BYTES + 1)), "");
        // exactly at the bound: kept whole
        let edge = format!("{}; {}", "a".repeat(2000), "b".repeat(MAX_BYTES - 2002));
        assert_eq!(to_stored(&edge), edge);
        // the patterns in force are bounded too, whatever the options hold
        let p = patterns(&gui(true, &many.join(";"))).unwrap();
        assert!(p.matches("p255") && !p.matches("p256"));
    }

    #[test]
    fn a_file_is_matched_by_its_own_name() {
        let ti = info(&[
            (&["Season 1", "Sample", "e01.sample.mkv"], false),
            (&["Season 1", "e01.mkv"], false),
            (&["Desktop.LNK"], false),
            (&["thumbs.db"], false),
            (&["lnk", "readme.txt"], false),
        ]);
        let o = gui(true, "*.lnk; Thumbs.db; *sample*");
        // a folder's name does not count: Sample/ and lnk/ hold files that are kept
        assert_eq!(excluded(&ti, &o), [true, false, true, true, false]);
        assert_eq!(priorities(&ti, &o), Some(vec![X, N, X, X, N]));
        // the off switch, an empty list, a list that matches nothing
        assert_eq!(excluded(&ti, &gui(false, "*.lnk")), [false; 5]);
        assert_eq!(priorities(&ti, &gui(false, "*.lnk")), None);
        assert_eq!(priorities(&ti, &gui(true, " ; ")), None);
        assert_eq!(priorities(&ti, &gui(true, "*.exe")), None);
        assert_eq!(excluded(&ti, &gui(true, "*.exe")), [false; 5]);
    }

    #[test]
    fn pad_files_never_count() {
        let ti = info(&[(&["a.mkv"], false), (&[".pad", "16384"], true), (&["b.nfo"], false)]);
        let o = gui(true, "*");
        // every real file matches: the torrent is left alone
        assert_eq!(excluded(&ti, &o), [false, false, false]);
        assert_eq!(priorities(&ti, &o), None);
        let o = gui(true, "*.nfo; 16384");
        assert_eq!(excluded(&ti, &o), [false, false, true]);
        assert_eq!(priorities(&ti, &o), Some(vec![N, X, X]));
    }

    /// A torrent of nothing but matching files (a pack of shortcuts) still downloads.
    #[test]
    fn a_torrent_whose_every_file_matches_is_left_alone() {
        let ti = info(&[(&["one.lnk"], false), (&["two.LNK"], false)]);
        let o = gui(true, "*.lnk");
        assert_eq!(excluded(&ti, &o), [false, false]);
        assert_eq!(priorities(&ti, &o), None);
        let mut p =
            AddTorrentParams::new(TorrentSource::TorrentFile(torrent(&[(&["one.lnk"], false)])), "/dl");
        assert!(!apply_to_new(&mut p, &o));
        assert!(p.file_priorities.is_empty());
    }

    #[test]
    fn an_add_without_the_window_gets_the_priorities() {
        let bytes = torrent(&[(&["movie.mkv"], false), (&["Thumbs.db"], false)]);
        let o = gui(true, "thumbs.db");
        let mut p = AddTorrentParams::new(TorrentSource::TorrentFile(bytes.clone()), "/dl");
        assert!(apply_to_new(&mut p, &o));
        assert_eq!(p.file_priorities, [N, X]);
        // priorities the caller set win
        let mut p = AddTorrentParams::new(TorrentSource::TorrentFile(bytes.clone()), "/dl");
        p.file_priorities = vec![N, N];
        assert!(!apply_to_new(&mut p, &o));
        assert_eq!(p.file_priorities, [N, N]);
        // off; a file that is no torrent; a magnet (no files yet)
        let mut p = AddTorrentParams::new(TorrentSource::TorrentFile(bytes), "/dl");
        assert!(!apply_to_new(&mut p, &gui(false, "thumbs.db")));
        let mut p = AddTorrentParams::new(TorrentSource::TorrentFile(b"junk".to_vec()), "/dl");
        assert!(!apply_to_new(&mut p, &o));
        let mut p = AddTorrentParams::new(TorrentSource::Magnet("magnet:?xt=urn:btih:".into()), "/dl");
        assert!(!apply_to_new(&mut p, &o));
    }

    #[test]
    fn a_magnet_waits_for_its_files() {
        let mut pending = Pending::new();
        let h = InfoHashes::v1(Sha1Hash([7; 20]));
        let uri = format!("magnet:?xt=urn:btih:{}", h.key_hex());
        let mut p = AddTorrentParams::new(TorrentSource::Magnet(uri.clone()), "/dl");
        let new = |_: &InfoHashes| false;
        pending.add(&mut p, &gui(false, "*.lnk"), new);
        assert!(pending.is_empty(), "off: nothing to wait for");
        pending.add(&mut p, &gui(true, "*.lnk"), |q| {
            assert_eq!(*q, h);
            false
        });
        assert!(!pending.is_empty() && p.file_priorities.is_empty());
        // another torrent's alerts change nothing; this one's id is learnt at its add
        let other = InfoHashes::v1(Sha1Hash([8; 20]));
        let (id, other_id) = (TorrentId::from_raw(1), TorrentId::from_raw(2));
        let session = rookparcel_core::Session::start_mock(Default::default(), Default::default());
        let handle = session.handle();
        pending.on_alert(
            &Alert::TorrentAdded { id: other_id, info_hashes: other, name: String::new() },
            &handle,
        );
        assert!(!pending.holds(other_id));
        pending.on_alert(&Alert::TorrentAdded { id, info_hashes: h, name: String::new() }, &handle);
        assert!(pending.holds(id));
        let entry = |path: &str, pad| FileEntry {
            path: path.into(),
            size: 1,
            offset: 0,
            first_piece: 0,
            last_piece: 0,
            pad_file: pad,
            priority: N,
        };
        // an empty list (asked for before the metadata) is not the one waited for
        pending.on_alert(&Alert::FileList { id, files: Vec::new().into() }, &handle);
        assert!(pending.holds(id));
        let files = vec![entry("x/a.mkv", false), entry("x/b.lnk", false)];
        // nor is a full one before the metadata arrived: a torrent that was in the session
        // already (a duplicate add reports its id), whose file choices stand
        pending.on_alert(&Alert::FileList { id, files: files.clone().into() }, &handle);
        assert!(pending.holds(id));
        pending.on_alert(&Alert::MetadataReceived { id }, &handle);
        pending.on_alert(&Alert::FileList { id, files: files.into() }, &handle);
        assert!(pending.is_empty());
        // removed before its metadata: forgotten
        pending.add(&mut p, &gui(true, "*.lnk"), new);
        pending.on_alert(&Alert::TorrentAdded { id, info_hashes: h, name: String::new() }, &handle);
        pending.on_alert(&Alert::TorrentRemoved { id, info_hashes: h }, &handle);
        assert!(pending.is_empty());
        // refused by the engine: forgotten
        pending.add(&mut p, &gui(true, "*.lnk"), new);
        let failed = Alert::AddTorrentFailed {
            name: String::new(),
            info_hashes: Some(h),
            error: rookparcel_core::Error::DuplicateTorrent,
        };
        pending.on_alert(&failed, &handle);
        assert!(pending.is_empty());
        // the engine's list: a path, matched by its file name, pad files Don't Download
        let p = SkipPatterns::parse("*.lnk");
        let files = [entry("Show/a.mkv", false), entry("Show/.pad/1", true), entry("Show/lnk/b.LNK", false)];
        assert_eq!(priorities_of_entries(&files, &p), Some(vec![N, X, X]));
        assert_eq!(priorities_of_entries(&files[2..], &p), None);
    }

    /// A magnet of a torrent already in the session (a restored one, one waiting for the Add
    /// window) is reported added under that torrent's id: its file choices stand, even when its
    /// metadata arrives after the add.
    #[test]
    fn a_magnet_already_in_the_session_is_left_alone() {
        let session = rookparcel_core::Session::start_mock(Default::default(), Default::default());
        let handle = session.handle();
        let h = InfoHashes::v1(Sha1Hash([7; 20]));
        let magnet = || {
            let uri = format!("magnet:?xt=urn:btih:{}", h.key_hex());
            AddTorrentParams::new(TorrentSource::Magnet(uri), "/dl")
        };
        let id = TorrentId::from_raw(1);
        let o = gui(true, "*.lnk");
        let in_list = |list: &[InfoHashes]| {
            let list = list.to_vec();
            move |q: &InfoHashes| list.iter().any(|l| same_torrent(l, q))
        };
        let added = Alert::TorrentAdded { id, info_hashes: h, name: String::new() };
        let files: Vec<FileEntry> = ["x/a.mkv", "x/b.lnk"]
            .iter()
            .map(|p| FileEntry {
                path: (*p).into(),
                size: 1,
                offset: 0,
                first_piece: 0,
                last_piece: 0,
                pad_file: false,
                priority: N,
            })
            .collect();
        // a duplicate of a torrent with its metadata, then of one still waiting for it
        for metadata_later in [false, true] {
            let mut pending = Pending::new();
            let mut p = magnet();
            pending.add(&mut p, &o, in_list(&[InfoHashes::v1(Sha1Hash([1; 20])), h]));
            assert!(pending.is_empty() && p.file_priorities.is_empty());
            pending.on_alert(&added, &handle);
            if metadata_later {
                pending.on_alert(&Alert::MetadataReceived { id }, &handle);
            }
            pending.on_alert(&Alert::FileList { id, files: files.clone().into() }, &handle);
            assert!(!pending.holds(id) && pending.is_empty());
        }
        // a new magnet: waited for, and its files set once its metadata came
        let mut pending = Pending::new();
        pending.add(&mut magnet(), &o, in_list(&[InfoHashes::v1(Sha1Hash([1; 20]))]));
        pending.on_alert(&added, &handle);
        pending.on_alert(&Alert::MetadataReceived { id }, &handle);
        assert!(pending.holds(id));
        pending.on_alert(&Alert::FileList { id, files: files.into() }, &handle);
        assert!(pending.is_empty());

        // the engine's duplicate check: a shared v1 or truncated v2 hash
        let v2 = Sha256Hash([9; 32]);
        let hybrid = InfoHashes { v1: Some(Sha1Hash([7; 20])), v2: Some(v2) };
        assert!(same_torrent(&hybrid, &h));
        assert!(same_torrent(&hybrid, &InfoHashes { v1: None, v2: Some(v2) }));
        assert!(!same_torrent(&hybrid, &InfoHashes::v1(Sha1Hash([5; 20]))));
        assert!(!same_torrent(&InfoHashes::default(), &InfoHashes::default()));
    }
}
