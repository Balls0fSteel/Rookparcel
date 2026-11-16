// Ported from libtorrent-rasterbar 2.0.11: src/mmap_disk_io.cpp (job, fence and thread-pool
// structure), src/posix_storage.cpp, src/storage_utils.cpp, src/disk_job_fence.cpp;
// fixes from 2.0.12 (3417f6479d) and 2.1.2 (36582475fc)
// Copyright (c) 2003-2020, Arvid Norberg and contributors. BSD-3-Clause, see THIRD-PARTY-NOTICES.md.
// Changes: positional read/write instead of mmap (PLAN.md D3). Jobs for a piece always run on
// the same thread (piece % threads), so per-piece hash state needs no locks; a fence is a job
// broadcast to every thread whose last finisher runs the fenced action. Blocks are hashed as
// they are written, in order, so a completed piece is never read back unless its blocks arrived
// out of order and did not fit in the small reorder cache. Blocks restored from resume data
// (`unfinished`) are on disk but not in any hash state, so they are read back as the cursor
// reaches them. File priority changes (libtorrent `set_file_priority`, a fence job) are a
// barrier: every thread parks the storage's later jobs until the last one has run the change.
// Renames (libtorrent `rename_file`, a fence job) are barriers too, and keep the renamed layout
// beside the shared `TorrentInfo` as `posix_storage::m_mapped_files` does. Reads for peers go
// through each thread's read cache (`read_cache`, after libtorrent 1.2), which writes, fences
// and barriers of the storage clear. Incomplete files may carry µTorrent's `.!ut` (D77, not in
// libtorrent): the suffix is a per-file state of the storage, found on disk by the first job and
// changed only by barriers, never saved. A file never takes a suffixed name that is another path
// of the torrent (`Clash`), and is deleted under one only when it holds data known to be ours.

//! Disk I/O: a thread pool doing file reads, writes, piece hashing and file checking for the
//! network thread.

pub(crate) mod drive;
pub(crate) mod file_pool;
pub(crate) mod gentle;
pub(crate) mod part_file;
pub(crate) mod read_cache;
pub(crate) mod smart;

use std::collections::{HashMap, HashSet};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, RwLock, mpsc};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::alert::FileOperation;
use crate::crypto::{Sha1, Sha256};
use crate::error::Error;
use crate::metainfo::TorrentInfo;
use crate::metainfo::file_storage::{FileFlags, FileStorage};
use crate::metainfo::merkle::merkle_piece_root;
use crate::params::MoveFlags;
use crate::types::Sha256Hash;
use crate::util::bitfield::Bitfield;
use file_pool::{FilePool, read_at, write_at};
use part_file::{PartFile, PartFiles};
use read_cache::ReadCache;
use smart::{CheckThreads, Config, Drive, Drives, MAX_CHECKS, Pacer};

/// The BitTorrent block size (libtorrent `default_block_size`).
pub(crate) const BLOCK_SIZE: u32 = 16 * 1024;

/// Out-of-order blocks each thread may keep in memory while waiting to hash them in order.
const REORDER_CACHE_BYTES: usize = 8 * 1024 * 1024;

/// One torrent's files on disk (libtorrent `storage_interface` / `posix_storage`).
pub(crate) struct Storage {
    /// Unique for the life of the process; a restarted torrent gets a new one, so completions
    /// for an older instance can be told apart.
    pub id: u64,
    /// Changed only by a move, which runs with no other job of the storage in flight.
    save_path: RwLock<String>,
    pub info: Arc<TorrentInfo>,
    /// `info`'s files with the renames done by this storage (libtorrent `m_mapped_files`);
    /// `None` until a file is renamed. Changed only by a rename, which runs with no other job of
    /// the storage in flight.
    mapped: RwLock<Option<Arc<FileStorage>>>,
    /// Set by the network thread to abandon queued and running work (checks, hashing).
    pub abort: AtomicBool,
    files: Mutex<FileState>,
    part_files: Arc<PartFiles>,
    /// The drive the save path is on, with the overrides' generation it was found under;
    /// dropped when the save path changes.
    drive: RwLock<Option<(u64, Arc<Drive>)>>,
    /// D77: per file, the [`Suffix`] its name carries on disk now (the low bits, [`SUFFIX_BITS`]).
    /// Set by [`Storage::init_names`] before the first job, by the probe
    /// ([`Storage::probe_names`]) and by renames, all of which run with no other job of the
    /// storage in flight (the check is the storage's first job; renames are barriers), so the
    /// other jobs read it without a lock. The high bits: the suffixes the file may not carry
    /// ([`NO_UT`], [`NO_QB`]: the name would be another path of the torrent, [`Clash`]) and
    /// whether the file holds data known to be this torrent's ([`OURS`]).
    names: Vec<AtomicU8>,
    /// D77: the torrent's paths that a suffixed name could take; `None` when no path of the
    /// torrent ends like one (nearly every torrent). Rebuilt when a rename or a move changes
    /// the paths.
    clash: RwLock<Option<Clash>>,
    /// The torrent's storage before this one (a running torrent's, released before its files
    /// are deleted with a storage made for that): what it wrote counts as ours too
    /// ([`Storage::note_data`]).
    data_from: Mutex<Option<Arc<Storage>>>,
    /// The names were looked for on disk ([`Storage::probe_names`]).
    probed: AtomicBool,
    /// Files whose failed rename to the name they should have was reported; reported again only
    /// after a rename of theirs worked.
    rename_reported: Mutex<Vec<bool>>,
}

/// D77: what a file's name carries after its own while the file is incomplete: µTorrent's
/// `.!ut`, which Rookparcel writes, or qBittorrent's `.!qB`, which it only picks up. Neither is
/// stored anywhere: the engine derives which files should carry `.!ut` from the pieces it has.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Suffix {
    Plain = 0,
    Ut = 1,
    Qb = 2,
}

impl Suffix {
    fn ext(self) -> &'static str {
        match self {
            Suffix::Plain => "",
            Suffix::Ut => INCOMPLETE_EXT,
            Suffix::Qb => QBITTORRENT_EXT,
        }
    }

    fn from_u8(v: u8) -> Suffix {
        match v {
            1 => Suffix::Ut,
            2 => Suffix::Qb,
            _ => Suffix::Plain,
        }
    }
}

/// µTorrent's extension for incomplete files (`Settings::append_incomplete_ext`, D77).
pub(crate) const INCOMPLETE_EXT: &str = ".!ut";
/// qBittorrent's ("Append .!qB extension to incomplete files"): picked up, never written.
pub(crate) const QBITTORRENT_EXT: &str = ".!qB";
/// The longest file name (one path component) the common file systems take: 255 bytes on
/// ext4, APFS and most others, 255 UTF-16 units on NTFS (never more than the UTF-8 bytes). A
/// file whose name would pass it with the suffix keeps its plain name.
const NAME_MAX: usize = 255;
/// What [`export_file`] puts after a file's name (the suffix included) while it writes it.
const EXPORT_EXT: &str = ".pa-export";

/// [`Storage::names`]: the bits of the [`Suffix`].
const SUFFIX_BITS: u8 = 0b11;
/// [`Storage::names`]: the file may not carry `.!ut` ([`Clash`]).
const NO_UT: u8 = 1 << 2;
/// [`Storage::names`]: the file may not carry `.!qB` ([`Clash`]).
const NO_QB: u8 = 1 << 3;
/// [`Storage::names`]: the file holds data known to be this torrent's: the storage wrote to
/// it, a piece of it passed a check, or the engine knows pieces or blocks of it
/// ([`Storage::note_data`]). Only such a file is deleted under a suffixed name.
const OURS: u8 = 1 << 4;

fn forbid_bit(suffix: Suffix) -> u8 {
    match suffix {
        Suffix::Plain => 0,
        Suffix::Ut => NO_UT,
        Suffix::Qb => NO_QB,
    }
}

/// A path as the file systems compare it, for telling whether two of the torrent's paths are
/// one (D77): `/` for `\`, and case folded, on every system (the torrent's own duplicate names
/// are found that way too, and a Linux download may be on an NTFS, exFAT or SMB volume). Each
/// character is folded alone, so a key of `a` followed by `b` is `a`'s key followed by `b`'s.
/// Windows also ignores a name's trailing dots and spaces. Folding more than a file system
/// does only keeps a suffix off a file that could have had it.
fn fold_key(path: &Path) -> String {
    let s = path.to_string_lossy();
    let mut out = String::with_capacity(s.len());
    for (i, part) in s.split(['/', '\\']).enumerate() {
        if i > 0 {
            out.push('/');
        }
        let part = if cfg!(windows) { part.trim_end_matches(['.', ' ']) } else { part };
        out.extend(part.chars().flat_map(char::to_uppercase).flat_map(char::to_lowercase));
    }
    out
}

/// Whether a [`fold_key`] ends like a suffixed name or an export's: a file or directory of
/// the torrent with such a name may be what another file's suffixed name would be.
fn looks_suffixed(key: &str) -> bool {
    [".!ut", ".!qb", EXPORT_EXT].iter().any(|e| key.ends_with(e))
}

/// Whether any element of `path` (separated by `/` or `\`) ends like a suffixed name. (Case
/// folding never makes a `!` or a `-`: an element without either is passed over unfolded.)
fn any_looks_suffixed(path: &str) -> bool {
    path.split(['/', '\\']).any(|p| p.contains(['!', '-']) && looks_suffixed(&fold_key(Path::new(p))))
}

/// D77: the torrent's paths a suffixed name could collide with: a file `x` may not be stored as
/// `x.!ut` when the torrent has a file or directory `x.!ut` (or `x.!ut.pa-export`, the name
/// [`export_file`] would write it under), and so on for `.!qB` (also in another case). Built only
/// for a torrent with such a path ([`Clash::of`]).
struct Clash {
    /// The [`fold_key`]s of the torrent's files and directories that [`looks_suffixed`].
    taken: HashSet<String>,
    /// Every file's plain path ([`fold_key`]) and the files at it (two only when a rename gave
    /// two files one name).
    bases: HashMap<String, Vec<u32>>,
}

impl Clash {
    /// The paths of `fs` in `save_path`; `None` when none [`looks_suffixed`].
    fn of(fs: &FileStorage, save_path: &str) -> Option<Clash> {
        let n = fs.num_files();
        let any = any_looks_suffixed(save_path)
            || any_looks_suffixed(fs.name())
            || fs.paths().iter().any(|p| any_looks_suffixed(p))
            || (0..n).any(|i| any_looks_suffixed(fs.file_name(i)));
        if !any {
            return None;
        }
        let mut taken = HashSet::new();
        let mut bases: HashMap<String, Vec<u32>> = HashMap::new();
        for i in 0..n {
            let key = fold_key(Path::new(&fs.file_path(i, save_path)));
            // the file, and the directories it is in
            let mut end = key.len();
            loop {
                let k = &key[..end];
                if looks_suffixed(k) {
                    taken.insert(k.to_owned());
                }
                match k.rfind('/') {
                    Some(j) if j > 0 => end = j,
                    _ => break,
                }
            }
            bases.entry(key).or_default().push(i as u32);
        }
        Some(Clash { taken, bases })
    }

    /// Whether a file at `base` (a [`fold_key`]) may not carry `suffix`: the name, or the name
    /// an export writes it under first, is a file or directory of the torrent.
    fn forbids(&self, base: &str, suffix: Suffix) -> bool {
        if suffix == Suffix::Plain {
            return false;
        }
        let mut k = format!("{base}{}", fold_key(Path::new(suffix.ext())));
        if self.taken.contains(&k) {
            return true;
        }
        k.push_str(EXPORT_EXT);
        self.taken.contains(&k)
    }
}

/// `path` with `suffix`'s extension after its file name.
fn with_suffix(path: PathBuf, suffix: Suffix) -> PathBuf {
    if suffix == Suffix::Plain {
        return path;
    }
    let mut s = path.into_os_string();
    s.push(suffix.ext());
    PathBuf::from(s)
}

/// File `file`'s plain path in `fs` (a torrent's files, renamed or not) under `save_path`.
fn base_path_in(fs: &FileStorage, save_path: &str, file: usize) -> PathBuf {
    PathBuf::from(fs.file_path(file, save_path))
}

fn name_len(path: &Path) -> Option<usize> {
    path.file_name().map(|n| n.to_string_lossy().len())
}

/// Whether we may give `path`'s file name a suffix (all are 4 bytes): it still fits
/// [`NAME_MAX`] with one, and with the part file's export name after that ([`EXPORT_EXT`]).
fn suffix_fits(path: &Path) -> bool {
    name_len(path).is_some_and(|n| n + INCOMPLETE_EXT.len() + EXPORT_EXT.len() <= NAME_MAX)
}

/// Whether a file of `path`'s name with a suffix can exist at all (another client's, or one we
/// are given): looked for by the probes, which then give it the name we would.
fn suffix_possible(path: &Path) -> bool {
    name_len(path).is_some_and(|n| n + INCOMPLETE_EXT.len() <= NAME_MAX)
}

fn found(path: &Path) -> bool {
    matches!(path.try_exists(), Ok(true))
}

/// libtorrent `posix_storage::m_file_priority`, `m_use_partfile` and `m_part_file`.
struct FileState {
    priority: Vec<u8>,
    /// False for a skipped file that existed on disk when it was skipped: it keeps its blocks
    /// (libtorrent's compatibility with versions without part files).
    use_partfile: Vec<bool>,
    part: Option<Arc<PartFile>>,
}

impl Storage {
    fn new(
        id: u64,
        save_path: String,
        info: Arc<TorrentInfo>,
        mut priority: Vec<u8>,
        part_files: Arc<PartFiles>,
    ) -> Storage {
        let n = info.files().num_files();
        priority.resize(n.max(priority.len()), DEFAULT_PRIORITY);
        let files = Mutex::new(FileState { priority, use_partfile: vec![true; n], part: None });
        let save_path = RwLock::new(save_path);
        let mapped = RwLock::new(None);
        let drive = RwLock::new(None);
        let storage = Storage {
            id,
            save_path,
            info,
            mapped,
            abort: AtomicBool::new(false),
            files,
            part_files,
            drive,
            names: (0..n).map(|_| AtomicU8::new(Suffix::Plain as u8)).collect(),
            clash: RwLock::new(None),
            data_from: Mutex::new(None),
            probed: AtomicBool::new(false),
            rename_reported: Mutex::new(vec![false; n]),
        };
        storage.rebuild_clash();
        storage
    }

    /// D77: finds again which suffixes each file may not carry ([`Clash`]), for the paths the
    /// files have now. A file whose suffix became one of those and that is not on disk under
    /// it loses it; one that is keeps it until the next names job renames it.
    fn rebuild_clash(&self) {
        let save_path = self.save_path();
        let mapped = self.mapped();
        let fs = mapped.as_deref().unwrap_or(self.info.files());
        let clash = Clash::of(fs, &save_path);
        let mut slot = self.clash.write().unwrap_or_else(|e| e.into_inner());
        if clash.is_none() && slot.is_none() {
            return;
        }
        for f in 0..self.names.len() {
            let mut bits = 0;
            if let Some(c) = &clash {
                let base = fold_key(&base_path_in(fs, &save_path, f));
                for s in [Suffix::Ut, Suffix::Qb] {
                    if c.forbids(&base, s) {
                        bits |= forbid_bit(s);
                    }
                }
            }
            self.update_bits(f, |v| (v & !(NO_UT | NO_QB)) | bits);
        }
        *slot = clash;
        drop(slot);
        for f in 0..self.names.len() {
            let now = self.suffix(f);
            if self.forbidden(f, now) && !found(&self.file_path(f)) {
                self.set_suffix(f, Suffix::Plain);
            }
        }
    }

    fn update_bits(&self, file: usize, change: impl Fn(u8) -> u8) {
        if let Some(n) = self.names.get(file) {
            // A compare-exchange loop, not `fetch_update`: newer toolchains deprecate that
            // name for `try_update`, which the MSRV does not have.
            let mut old = n.load(Ordering::Relaxed);
            while let Err(seen) =
                n.compare_exchange_weak(old, change(old), Ordering::Relaxed, Ordering::Relaxed)
            {
                old = seen;
            }
        }
    }

    /// D77: whether file `file` may not carry `suffix`: its name with it would be another path
    /// of the torrent ([`Clash`]).
    fn forbidden(&self, file: usize, suffix: Suffix) -> bool {
        let bit = forbid_bit(suffix);
        bit != 0 && self.names.get(file).is_some_and(|n| n.load(Ordering::Relaxed) & bit != 0)
    }

    /// D77: whether `path` is where another file than `file` is or would be: another file's
    /// plain path or the name it has now, or a directory of the torrent. Never true for a
    /// torrent none of whose paths [`looks_suffixed`], where only a rename that gave two files
    /// one name could make it so.
    fn clashes(&self, file: usize, path: &Path) -> bool {
        let clash = self.clash.read().unwrap_or_else(|e| e.into_inner());
        let Some(c) = clash.as_ref() else { return false };
        let own = fold_key(&self.base_path(file));
        self.clashes_in(c, file, &own, path)
    }

    /// [`Storage::clashes`] in `c`, for file `file` at `own` (its plain path's [`fold_key`]).
    fn clashes_in(&self, c: &Clash, file: usize, own: &str, path: &Path) -> bool {
        let k = fold_key(path);
        let other = |v: Option<&Vec<u32>>, suffix: Option<Suffix>| {
            v.is_some_and(|v| {
                v.iter().any(|&j| j as usize != file && suffix.is_none_or(|s| self.suffix(j as usize) == s))
            })
        };
        // another file's plain path or a directory (a file's own plain path is in `taken` too)
        if (k != own && c.taken.contains(&k)) || other(c.bases.get(&k), None) {
            return true;
        }
        // the name another file has now
        [Suffix::Ut, Suffix::Qb].into_iter().any(|s| {
            let ext = fold_key(Path::new(s.ext()));
            k.strip_suffix(&ext).is_some_and(|stem| other(c.bases.get(stem), Some(s)))
        })
    }

    /// D77: whether `path` is a file or directory of the torrent; only paths that
    /// [`looks_suffixed`] are known.
    fn taken(&self, path: &Path) -> bool {
        let clash = self.clash.read().unwrap_or_else(|e| e.into_inner());
        clash.as_ref().is_some_and(|c| c.taken.contains(&fold_key(path)))
    }

    /// D77: notes which files hold data known to be this torrent's, so that deleting the
    /// torrent's files takes them under a suffixed name ([`delete_files`]): those a piece
    /// `has_piece` says we have overlaps, those a block of `unfinished` (pieces and their
    /// blocks on disk) lands in, and what `from`, the torrent's storage before this one, wrote
    /// or noted (security review, 2026-11-16).
    pub fn note_data(
        &self,
        has_piece: &dyn Fn(u32) -> bool,
        unfinished: &[(u32, Bitfield)],
        from: Option<Arc<Storage>>,
    ) {
        let fs = self.info.files();
        for f in 0..fs.num_files() {
            if fs.pad_file_at(f) || fs.file_size(f) == 0 {
                continue;
            }
            let (first, end) = crate::metainfo::file_storage::file_piece_range_inclusive(fs, f);
            if (first..end).any(has_piece) {
                self.note_ours(f);
            }
        }
        for (piece, blocks) in unfinished {
            if *piece >= self.info.num_pieces() {
                continue;
            }
            let size = self.info.piece_size(*piece);
            for b in (0..blocks.len()).filter(|&b| blocks.get(b)) {
                let offset = b as u32 * BLOCK_SIZE;
                if offset >= size {
                    break;
                }
                let len = (size - offset).min(BLOCK_SIZE);
                for s in fs.map_block(*piece, offset as u64, len as u64) {
                    self.note_ours(s.file_index);
                }
            }
        }
        if let Some(from) = from.filter(|s| !std::ptr::eq(Arc::as_ptr(s), self)) {
            *self.data_from.lock().unwrap_or_else(|e| e.into_inner()) = Some(from);
        }
    }

    /// The files `piece` overlaps hold data of ours (it passed a check).
    fn note_piece(&self, piece: u32) {
        for s in self.info.files().map_block(piece, 0, self.info.piece_size(piece) as u64) {
            self.note_ours(s.file_index);
        }
    }

    fn note_ours(&self, file: usize) {
        if let Some(n) = self.names.get(file) {
            if n.load(Ordering::Relaxed) & OURS == 0 {
                n.fetch_or(OURS, Ordering::Relaxed);
            }
        }
    }

    /// Whether file `file` holds data known to be ours ([`OURS`]), here or in the storage
    /// before this one.
    fn ours(&self, file: usize) -> bool {
        let here = self.names.get(file).is_some_and(|n| n.load(Ordering::Relaxed) & OURS != 0);
        here || self
            .data_from
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .is_some_and(|s| s.ours(file))
    }

    pub fn save_path(&self) -> String {
        self.save_path.read().unwrap_or_else(|e| e.into_inner()).clone()
    }

    /// The renamed layout, if a file was renamed (libtorrent `posix_storage::files()`).
    fn mapped(&self) -> Option<Arc<FileStorage>> {
        self.mapped.read().unwrap_or_else(|e| e.into_inner()).clone()
    }

    /// Where file `file` is: its name in the torrent (or the one it was renamed to) in the save
    /// path, with the suffix it carries on disk now (D77).
    fn file_path(&self, file: usize) -> PathBuf {
        with_suffix(self.base_path(file), self.suffix(file))
    }

    /// File `file`'s path without a suffix: its name in the torrent, or the one it was renamed
    /// to, in the save path.
    fn base_path(&self, file: usize) -> PathBuf {
        let save_path = self.save_path.read().unwrap_or_else(|e| e.into_inner());
        match &*self.mapped.read().unwrap_or_else(|e| e.into_inner()) {
            Some(fs) => base_path_in(fs, &save_path, file),
            None => base_path_in(self.info.files(), &save_path, file),
        }
    }

    /// The suffix file `file`'s name carries on disk now (D77).
    pub fn suffix(&self, file: usize) -> Suffix {
        self.names
            .get(file)
            .map_or(Suffix::Plain, |n| Suffix::from_u8(n.load(Ordering::Relaxed) & SUFFIX_BITS))
    }

    fn set_suffix(&self, file: usize, suffix: Suffix) {
        self.update_bits(file, |v| (v & !SUFFIX_BITS) | suffix as u8);
    }

    /// Whether file `file` (at `base`, its plain path) may carry `.!ut`: not a pad file or a
    /// symlink, not empty, its name still fits [`NAME_MAX`] with one, and that name is no other
    /// path of the torrent ([`Clash`]).
    fn may_suffix(&self, file: usize, base: &Path) -> bool {
        !self.skip_file(file)
            && self.info.files().file_size(file) > 0
            && suffix_fits(base)
            && !self.forbidden(file, Suffix::Ut)
    }

    /// The suffixes besides its plain name that file `file` (at `base`) may be found under:
    /// those a name of its length can carry ([`suffix_possible`]) and that are no other path of
    /// the torrent ([`Clash`]).
    fn may_find(&self, file: usize, base: &Path, suffix: Suffix) -> bool {
        suffix == Suffix::Plain || (suffix_possible(base) && !self.forbidden(file, suffix))
    }

    /// D77: the names the files should have, before the storage's first job: `incomplete`
    /// files carry `.!ut` where they may ([`Storage::may_suffix`]). What is on disk is looked
    /// at by the first job that touches the files ([`Storage::probe_names`]).
    pub fn init_names(&self, incomplete: &[bool]) {
        for (f, &inc) in incomplete.iter().enumerate().take(self.names.len()) {
            let suffix =
                if inc && self.may_suffix(f, &self.base_path(f)) { Suffix::Ut } else { Suffix::Plain };
            self.set_suffix(f, suffix);
        }
    }

    /// D77: whether a file is not under the name [`DiskIo::async_set_names`] with `incomplete`
    /// would give it. Only meaningful while no names job of the storage is in flight (its
    /// check's answer is such a time).
    pub fn names_differ(&self, incomplete: &[bool]) -> bool {
        incomplete
            .iter()
            .enumerate()
            .take(self.names.len())
            .any(|(f, &inc)| self.name_change(f, inc).is_some())
    }

    /// D77: when file `f` is not under the name it should have (`.!ut` while `incomplete` where
    /// it may carry one, else plain): the suffix it has, the one it should have, and its plain
    /// path. Most files have the right one, which is told without building a path: a file keeps
    /// a `.!ut` it has while incomplete (we give one only where it fits; one found on disk keeps
    /// it too, and loses it once complete).
    fn name_change(&self, f: usize, incomplete: bool) -> Option<(Suffix, Suffix, PathBuf)> {
        let now = self.suffix(f);
        let settled = match now {
            Suffix::Plain => !incomplete,
            // (a `.!ut` that became another path of the torrent by a rename goes)
            Suffix::Ut => incomplete && !self.forbidden(f, Suffix::Ut),
            Suffix::Qb => false,
        };
        if settled || self.skip_file(f) {
            return None;
        }
        let base = self.base_path(f);
        let want = if incomplete && self.may_suffix(f, &base) { Suffix::Ut } else { Suffix::Plain };
        (now != want).then_some((now, want, base))
    }

    /// D77: finds each file on disk, once per storage, before anything else uses the files. A
    /// file not under the name it should have is looked for under its plain name, then with
    /// `.!ut`, then with `.!qB` (qBittorrent's), and the first one found is used under that name
    /// (a crash between a rename and the next save of the resume data, the option turned on
    /// or off while Rookparcel was closed, or µTorrent's or qBittorrent's partial file). Nothing
    /// is renamed here: the engine sends the names the files should have once the pieces are
    /// known ([`DiskIo::async_set_names`]), so a complete file found under its plain name is
    /// never renamed back and forth. With `thorough` (a full check), a file found under one name
    /// while another candidate exists too is noted: the other is left alone. Returns the
    /// Logger lines.
    fn probe_names(&self, thorough: bool) -> Vec<String> {
        self.probe(thorough, None)
    }

    /// [`Storage::probe_names`], keeping in `sizes` the length of each file found: the look
    /// at a file under the name it should have is the only one when it is there, and the
    /// check's test of the resume data needs no second one ([`resume_files`]).
    fn probe(&self, thorough: bool, mut sizes: Option<&mut [Option<u64>]>) -> Vec<String> {
        if self.probed.swap(true, Ordering::Relaxed) {
            return Vec::new();
        }
        let mut log = Vec::new();
        for f in 0..self.names.len() {
            if self.skip_file(f) || self.info.files().file_size(f) == 0 {
                continue;
            }
            let base = self.base_path(f);
            let now = self.suffix(f);
            // D77: never a name that is another path of the torrent: not a suffix the file may
            // not carry, nor (should a rename have made it so) another file's name
            let mut order = Vec::with_capacity(3);
            for s in [now, Suffix::Plain, Suffix::Ut, Suffix::Qb] {
                if !order.contains(&s) && (s == now || self.may_find(f, &base, s)) && !self.forbidden(f, s) {
                    order.push(s);
                }
            }
            let mut first = None;
            for s in order {
                let p = with_suffix(base.clone(), s);
                if self.clashes(f, &p) {
                    continue;
                }
                let Ok(meta) = std::fs::metadata(&p) else { continue };
                match first {
                    None => {
                        first = Some(s);
                        if let Some(size) = sizes.as_deref_mut().and_then(|v| v.get_mut(f)) {
                            *size = Some(meta.len());
                        }
                        if !thorough {
                            break;
                        }
                    }
                    Some(used) => log.push(format!(
                        "\"{}\" is used; \"{}\" is there too and is left alone",
                        with_suffix(base.clone(), used).display(),
                        p.display()
                    )),
                }
            }
            if first.is_none() && self.forbidden(f, now) {
                self.set_suffix(f, Suffix::Plain);
            }
            if let Some(s) = first.filter(|&s| s != now) {
                self.set_suffix(f, s);
                log.push(format!(
                    "\"{}\" is not there; using \"{}\"",
                    with_suffix(base.clone(), now).display(),
                    with_suffix(base, s).display()
                ));
            }
        }
        log
    }

    fn skip_file(&self, file: usize) -> bool {
        let fs = self.info.files();
        fs.file_flags(file).intersects(FileFlags::PAD_FILE | FileFlags::SYMLINK)
    }

    fn lock_files(&self) -> MutexGuard<'_, FileState> {
        self.files.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// libtorrent `m_part_file_name`.
    fn part_file_name(&self) -> String {
        format!(".{}.parts", self.info.info_hash().to_hex())
    }

    fn part_file_path(&self) -> PathBuf {
        PathBuf::from(self.save_path()).join(self.part_file_name())
    }

    /// libtorrent `need_partfile`: the part file object (the file itself is created lazily).
    fn need_partfile(&self, st: &mut FileState) -> Arc<PartFile> {
        st.part
            .get_or_insert_with(|| {
                self.part_files.get_or_open(
                    std::path::Path::new(&self.save_path()),
                    &self.part_file_name(),
                    self.info.num_pieces(),
                    self.info.piece_length(),
                )
            })
            .clone()
    }

    /// The part file, if `file`'s blocks go there: it has priority 0 and was not on disk.
    fn part_file_for(&self, file: usize) -> Option<Arc<PartFile>> {
        let mut st = self.lock_files();
        if st.priority.get(file).copied() == Some(0) && st.use_partfile.get(file).copied().unwrap_or(true) {
            Some(self.need_partfile(&mut st))
        } else {
            None
        }
    }

    fn part_error(&self, e: io::Error) -> (String, Error) {
        (self.part_file_path().to_string_lossy().into_owned(), Error::from(e))
    }
}

/// libtorrent `default_priority`.
const DEFAULT_PRIORITY: u8 = 4;

#[derive(Debug)]
pub(crate) enum CheckMode {
    /// Hash every piece (libtorrent `check_files` without resume data, or `force_recheck`).
    Full,
    /// Trust these pieces if every file holding them exists and is long enough; otherwise fall
    /// back to a full check (libtorrent `verify_resume_data`).
    Resume(Bitfield),
}

enum Job {
    Write {
        storage: Arc<Storage>,
        piece: u32,
        offset: u32,
        data: Vec<u8>,
    },
    Read {
        storage: Arc<Storage>,
        piece: u32,
        offset: u32,
        length: u32,
        cookie: u64,
    },
    /// Hash a piece already on disk (a seed-mode piece before its first upload).
    Verify {
        storage: Arc<Storage>,
        piece: u32,
    },
    Restore {
        storage: Arc<Storage>,
        piece: u32,
        blocks: Vec<bool>,
    },
    Check {
        storage: Arc<Storage>,
        mode: CheckMode,
    },
    Fence {
        storage: Arc<Storage>,
        fence: Arc<Fence>,
    },
    Barrier {
        storage: Arc<Storage>,
        barrier: Arc<Barrier>,
    },
    /// The barrier `seq` of `storage` is done: run the jobs parked behind it.
    Unblock {
        storage: u64,
        seq: u64,
    },
    Shutdown,
}

impl Job {
    fn storage_id(&self) -> Option<u64> {
        match self {
            Job::Write { storage, .. }
            | Job::Read { storage, .. }
            | Job::Verify { storage, .. }
            | Job::Restore { storage, .. }
            | Job::Check { storage, .. }
            | Job::Fence { storage, .. }
            | Job::Barrier { storage, .. } => Some(storage.id),
            Job::Unblock { .. } | Job::Shutdown => None,
        }
    }
}

/// A file priority change or a move, run by the last thread to reach it while the others hold
/// back the storage's later jobs (libtorrent's fence jobs that keep the storage).
struct Barrier {
    remaining: AtomicUsize,
    seq: u64,
    action: BarrierAction,
}

enum BarrierAction {
    FilePriority(Vec<u8>),
    Move {
        path: String,
        flags: MoveFlags,
    },
    Rename {
        file: u32,
        name: String,
    },
    /// D77: per file, whether it should carry `.!ut` ([`DiskIo::async_set_names`]).
    Names(Vec<bool>),
}

/// libtorrent `status_t`, the values `move_storage` returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MoveStatus {
    NoError,
    /// Moved (or not, for `ResetSavePath`), but the files must be checked at the new path.
    NeedFullCheck,
    /// `FailIfExist` found a file at the destination; nothing was moved.
    FileExist,
    FatalDiskError,
}

/// Why a move failed (libtorrent `storage_error`, with the file resolved to a path).
#[derive(Clone, Debug)]
pub(crate) struct MoveError {
    pub file: String,
    pub operation: FileOperation,
    pub error: Error,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FenceAction {
    /// Close the files and drop all cached state (libtorrent `release_files`, `stop_torrent`).
    Release,
    /// Release, then delete the files, any directories left empty and the part file
    /// (libtorrent `delete_files`).
    Delete,
    /// Release, then delete only the part file (libtorrent `delete_partfile`).
    DeletePartFile,
}

struct Fence {
    remaining: AtomicUsize,
    action: FenceAction,
}

#[derive(Debug)]
pub(crate) struct CheckResult {
    pub have: Bitfield,
    /// True when resume data did not match the files and a full check was done instead.
    pub resume_failed: bool,
    /// A hybrid piece whose v1 and v2 hashes disagreed; the check stopped there (libtorrent
    /// `handle_inconsistent_hashes`).
    pub inconsistent: Option<u32>,
    /// Files the resume data counted on that are gone, when that was all that was wrong with
    /// it: `have` is the resume data's set without their pieces, and nothing was hashed.
    pub missing: Vec<usize>,
    /// D77: files found under another name than the one they should have, for the Logger
    /// ([`Storage::probe_names`]).
    pub names: Vec<String>,
}

/// How a piece compares with the metadata. v1 pieces are checked against their SHA-1, v2
/// pieces against the merkle root of their blocks from the piece layers (both known on the disk
/// thread), hybrids against both.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HashVerdict {
    Passed,
    Failed,
    /// A hybrid piece that passed one of its hashes and failed the other (libtorrent
    /// `torrent_inconsistent_hashes`).
    Inconsistent,
}

/// Results sent back to the network thread.
#[derive(Debug)]
pub(crate) enum Completion {
    Write {
        storage: u64,
        piece: u32,
        offset: u32,
        error: Option<(String, Error)>,
    },
    /// Every block of the piece was written and the piece hashed.
    Hashed {
        storage: u64,
        piece: u32,
        verdict: HashVerdict,
    },
    Read {
        storage: u64,
        cookie: u64,
        piece: u32,
        offset: u32,
        result: Result<Vec<u8>, Error>,
    },
    /// Answer to [`DiskIo::async_verify`].
    Verified {
        storage: u64,
        piece: u32,
        verdict: HashVerdict,
    },
    CheckProgress {
        storage: u64,
        checked: u32,
    },
    Checked {
        storage: u64,
        result: Result<CheckResult, (String, Error)>,
    },
    Fenced {
        storage: u64,
        action: FenceAction,
        error: Option<Error>,
        /// For the Logger: files a delete took or kept under a suffixed name (D77).
        log: Vec<String>,
    },
    /// Answer to [`DiskIo::async_set_file_priority`]; on error the change stopped at that file.
    FilePriority {
        storage: u64,
        error: Option<(String, Error)>,
    },
    /// Answer to [`DiskIo::async_move_storage`]: the storage's save path is now `save_path`
    /// (unchanged on error).
    Moved {
        storage: u64,
        status: MoveStatus,
        save_path: String,
        error: Option<MoveError>,
    },
    /// Answer to [`DiskIo::async_rename_file`]: file `file` is stored as `name` unless there
    /// is an error (the path it is about, and the error).
    Renamed {
        storage: u64,
        file: u32,
        name: String,
        error: Option<(String, Error)>,
    },
    /// Answer to [`DiskIo::async_set_names`]: the renames that failed for the first time
    /// (what was to be renamed to what, the error), and whether a file is still not under the
    /// name it should have for a reason that may pass (`pending`: tried again later).
    Names {
        storage: u64,
        failed: Vec<NameFailure>,
        pending: bool,
    },
}

/// A rename to the name a file should have that failed (D77): the file keeps the name it has.
#[derive(Debug)]
pub(crate) struct NameFailure {
    pub from: String,
    pub to: String,
    pub error: Error,
    /// Tried again later ([`rename_may_work_later`]); a name taken by another file is not.
    pub retry: bool,
}

pub(crate) struct DiskIo {
    threads: Vec<mpsc::Sender<Job>>,
    /// The checking threads' router: checks, fences and barriers, in order.
    hasher: mpsc::Sender<Job>,
    handles: Vec<JoinHandle<()>>,
    pool: Arc<FilePool>,
    part_files: Arc<PartFiles>,
    queued_write_bytes: Arc<AtomicU64>,
    read_cache: Arc<read_cache::Limits>,
    stats: Arc<DiskStats>,
    moves: Arc<Moves>,
    config: Arc<Config>,
    drives: Arc<Drives>,
    next_storage_id: u64,
    next_barrier: u64,
}

/// The storage moves queued and running on the disk threads, for a closing session's window
/// (D47), and the switch that cancels them. Empty while nothing moves, so it costs a torrent
/// nothing.
#[derive(Default)]
struct Moves {
    running: Mutex<Vec<MoveProgress>>,
    cancel: AtomicBool,
}

/// A move asked for: bytes moved of the files' total, to `to`; `started` once a disk thread
/// runs it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MoveProgress {
    pub storage: u64,
    pub to: String,
    pub done: u64,
    pub total: u64,
    pub started: bool,
}

impl Moves {
    fn running(&self) -> MutexGuard<'_, Vec<MoveProgress>> {
        self.running.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// A move of `storage` was queued.
    fn queue(&self, storage: u64, to: String, total: u64) {
        self.running().push(MoveProgress { storage, to, done: 0, total, started: false });
    }

    /// A disk thread runs `storage`'s first queued move, of `total` bytes, to `to`.
    fn start(&self, storage: u64, to: &str, total: u64) {
        let mut running = self.running();
        let m = MoveProgress { storage, to: to.to_owned(), done: 0, total, started: true };
        match running.iter_mut().find(|m| m.storage == storage && !m.started) {
            Some(queued) => *queued = m,
            None => running.push(m),
        }
    }

    /// The running move of `storage`.
    fn find(running: &mut [MoveProgress], storage: u64) -> Option<&mut MoveProgress> {
        running.iter_mut().find(|m| m.storage == storage && m.started)
    }

    /// `bytes` more of `storage`'s move are across.
    fn add(&self, storage: u64, bytes: u64) {
        if let Some(m) = Moves::find(&mut self.running(), storage) {
            m.done = (m.done + bytes).min(m.total);
        }
    }

    fn done(&self, storage: u64) -> u64 {
        Moves::find(&mut self.running(), storage).map_or(0, |m| m.done)
    }

    fn set_done(&self, storage: u64, done: u64) {
        if let Some(m) = Moves::find(&mut self.running(), storage) {
            m.done = done.min(m.total);
        }
    }

    fn end(&self, storage: u64) {
        let mut running = self.running();
        if let Some(i) = running.iter().position(|m| m.storage == storage && m.started) {
            running.remove(i);
        }
    }

    fn cancelled(&self) -> bool {
        self.cancel.load(Ordering::Relaxed)
    }
}

/// Counters of reads for peers (libtorrent `disk.num_blocks_read`, `disk.num_read_ops`,
/// `disk.num_blocks_cache_hits`), and the read cache's size per thread.
struct DiskStats {
    blocks_read: AtomicU64,
    read_ops: AtomicU64,
    cache_hits: AtomicU64,
    cache_bytes: Vec<AtomicUsize>,
}

impl DiskStats {
    fn new(threads: usize) -> DiskStats {
        DiskStats {
            blocks_read: AtomicU64::new(0),
            read_ops: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_bytes: (0..threads).map(|_| AtomicUsize::new(0)).collect(),
        }
    }
}

/// A snapshot of the disk counters, see [`DiskIo::counters`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct DiskCounters {
    /// Requests from peers read, from the cache or the files.
    pub blocks_read: u64,
    /// File reads made for them (a read cache line counts once).
    pub read_ops: u64,
    /// Requests answered from the read cache.
    pub cache_hits: u64,
    /// Bytes in the read cache.
    pub cache_bytes: u64,
}

struct Ctx {
    pool: Arc<FilePool>,
    done: mpsc::Sender<Completion>,
    wake: Arc<dyn Fn() + Send + Sync>,
    queued_write_bytes: Arc<AtomicU64>,
    read_cache: Arc<read_cache::Limits>,
    stats: Arc<DiskStats>,
    moves: Arc<Moves>,
    config: Arc<Config>,
    drives: Arc<Drives>,
    /// Every thread's queue but the router's, for [`Job::Unblock`].
    queues: Arc<OnceLock<Vec<mpsc::Sender<Job>>>>,
    /// Index of this thread in `queues`.
    index: usize,
}

impl Ctx {
    fn send(&self, c: Completion) {
        if self.done.send(c).is_ok() {
            (self.wake)();
        }
    }

    fn drive(&self, storage: &Storage) -> Arc<Drive> {
        let generation = self.config.generation();
        if let Some((g, d)) = &*storage.drive.read().unwrap_or_else(|e| e.into_inner()) {
            if *g == generation {
                return d.clone();
            }
        }
        let d = self.drives.for_path(&self.config, &storage.save_path());
        *storage.drive.write().unwrap_or_else(|e| e.into_inner()) = Some((generation, d.clone()));
        d
    }

    /// The drive of `storage` when a job there needs it: to wait for a place among the drive's
    /// jobs in flight, or to be timed for the busy-disk throttle. `None` costs nothing.
    fn drive_for_job(&self, storage: &Storage) -> Option<Arc<Drive>> {
        (self.config.tune() || self.config.measure()).then(|| self.drive(storage))
    }
}

impl DiskIo {
    /// Starts `threads` I/O threads, [`MAX_CHECKS`] checking threads and the router that hands
    /// them checks by drive. Completions go to `done`, followed by a call to `wake`.
    pub fn new(
        threads: usize,
        file_pool_size: usize,
        done: mpsc::Sender<Completion>,
        wake: Arc<dyn Fn() + Send + Sync>,
    ) -> DiskIo {
        let pool = Arc::new(FilePool::new(file_pool_size));
        let queued_write_bytes = Arc::new(AtomicU64::new(0));
        let queues = Arc::new(OnceLock::new());
        let n = threads.clamp(1, 64);
        let read_cache = Arc::new(read_cache::Limits::default());
        let stats = Arc::new(DiskStats::new(n + MAX_CHECKS));
        let moves = Arc::new(Moves::default());
        let config = Arc::new(Config::default());
        let drives = Arc::new(Drives::default());
        let ctx = |index| Ctx {
            pool: pool.clone(),
            done: done.clone(),
            wake: wake.clone(),
            queued_write_bytes: queued_write_bytes.clone(),
            read_cache: read_cache.clone(),
            stats: stats.clone(),
            moves: moves.clone(),
            config: config.clone(),
            drives: drives.clone(),
            queues: queues.clone(),
            index,
        };
        let mut senders = Vec::new();
        let mut handles = Vec::new();
        let spawn = |name: String, rx: mpsc::Receiver<Job>, ctx: Ctx| {
            std::thread::Builder::new()
                .name(name)
                .spawn(move || disk_thread(rx, ctx))
                .expect("failed to spawn a disk thread")
        };
        for i in 0..n {
            let (tx, rx) = mpsc::channel();
            handles.push(spawn(format!("lt-disk-{i}"), rx, ctx(i)));
            senders.push(tx);
        }
        let mut checkers = Vec::new();
        for i in 0..MAX_CHECKS {
            let (tx, rx) = mpsc::channel();
            handles.push(spawn(format!("lt-check-{i}"), rx, ctx(n + i)));
            checkers.push(tx);
        }
        let _ = queues.set(senders.iter().chain(&checkers).cloned().collect());
        let (hasher, rx) = mpsc::channel();
        let router = ctx(usize::MAX);
        handles.push(
            std::thread::Builder::new()
                .name("lt-hash".into())
                .spawn(move || check_router(rx, router, checkers))
                .expect("failed to spawn a disk thread"),
        );
        DiskIo {
            threads: senders,
            hasher,
            handles,
            pool,
            part_files: Arc::new(PartFiles::default()),
            queued_write_bytes,
            read_cache,
            stats,
            moves,
            config,
            drives,
            next_storage_id: 1,
            next_barrier: 1,
        }
    }

    /// The smart disk I/O settings (D48): gentle checks, the checking cap in MB/s (`None`: no
    /// cap), checks at once (`active_checking`), tuning for the drive type and its overrides.
    /// `measure`: time writes and reads for the busy-disk throttle.
    pub fn set_smart(
        &self,
        gentle: bool,
        check_rate_mb: Option<u32>,
        checks: i32,
        tune: bool,
        overrides: &str,
        measure: bool,
    ) {
        self.config.set(gentle, check_rate_mb, checks, tune, overrides);
        self.config.set_measure(measure);
    }

    /// Drives seen for the first time since the last call, and notes, for the Logger.
    pub fn drives_found(&self) -> Vec<String> {
        self.drives.take_found()
    }

    /// A drive whose writes, and one whose reads, are slower than usual now (described).
    pub fn busy_drives(&self, now: Instant) -> [Option<String>; 2] {
        let mut out = [None, None];
        for d in self.drives.all() {
            let busy = d.busy(now);
            for (slot, b) in out.iter_mut().zip(busy) {
                if b && slot.is_none() {
                    *slot = Some(d.id.describe.clone());
                }
            }
        }
        out
    }

    /// Fences and barriers count every thread that sees them: the I/O and checking threads.
    fn fenced_threads(&self) -> usize {
        self.threads.len() + MAX_CHECKS
    }

    /// A storage for `info` in `save_path`. Files whose `file_priority` is 0 keep their blocks
    /// in the part file.
    pub fn new_storage(
        &mut self,
        save_path: String,
        info: Arc<TorrentInfo>,
        file_priority: Vec<u8>,
    ) -> Arc<Storage> {
        let id = self.next_storage_id;
        self.next_storage_id += 1;
        Arc::new(Storage::new(id, save_path, info, file_priority, self.part_files.clone()))
    }

    pub fn set_file_pool_size(&self, n: usize) {
        self.pool.set_limit(n);
    }

    /// Sizes the read cache (libtorrent 1.2 `use_read_cache`, `cache_size` and
    /// `read_cache_line_size` in 16 KiB blocks, `cache_expiry` in seconds). `cache_blocks` is
    /// shared by the I/O threads; a negative one is the default, 2048 (32 MiB). Off when
    /// `enabled` is false or a thread's share is less than one line.
    pub fn set_read_cache(&self, enabled: bool, cache_blocks: i32, line_blocks: i32, expiry_secs: u32) {
        let blocks = if cache_blocks < 0 { 2048 } else { cache_blocks as usize };
        let per_thread = if enabled { blocks * BLOCK_SIZE as usize / self.threads.len() } else { 0 };
        let line = (line_blocks.max(0) as u32).saturating_mul(BLOCK_SIZE);
        self.read_cache.set(per_thread, line, Duration::from_secs(expiry_secs.into()));
    }

    pub fn counters(&self) -> DiskCounters {
        let s = &self.stats;
        DiskCounters {
            blocks_read: s.blocks_read.load(Ordering::Relaxed),
            read_ops: s.read_ops.load(Ordering::Relaxed),
            cache_hits: s.cache_hits.load(Ordering::Relaxed),
            cache_bytes: s.cache_bytes.iter().map(|b| b.load(Ordering::Relaxed) as u64).sum(),
        }
    }

    fn thread_for(&self, piece: u32) -> &mpsc::Sender<Job> {
        &self.threads[piece as usize % self.threads.len()]
    }

    pub fn async_write(&self, storage: &Arc<Storage>, piece: u32, offset: u32, data: Vec<u8>) {
        self.queued_write_bytes.fetch_add(data.len() as u64, Ordering::Relaxed);
        let _ = self.thread_for(piece).send(Job::Write { storage: storage.clone(), piece, offset, data });
    }

    pub fn async_read(&self, storage: &Arc<Storage>, piece: u32, offset: u32, length: u32, cookie: u64) {
        let _ = self.thread_for(piece).send(Job::Read {
            storage: storage.clone(),
            piece,
            offset,
            length,
            cookie,
        });
    }

    /// Hashes `piece` as it is on disk; answered by [`Completion::Verified`]. Seed mode trusts
    /// a torrent's pieces until each is checked like this before its first upload (SEC-20).
    pub fn async_verify(&self, storage: &Arc<Storage>, piece: u32) {
        let _ = self.thread_for(piece).send(Job::Verify { storage: storage.clone(), piece });
    }

    /// Tells the piece's hash state that `blocks` are already on disk (restored from resume
    /// data). Must be queued before any write to the piece. A piece that is complete this way is
    /// hashed at once and answered by [`Completion::Hashed`].
    pub fn async_restore_partial(&self, storage: &Arc<Storage>, piece: u32, blocks: Vec<bool>) {
        let _ = self.thread_for(piece).send(Job::Restore { storage: storage.clone(), piece, blocks });
    }

    pub fn async_check(&self, storage: &Arc<Storage>, mode: CheckMode) {
        let _ = self.hasher.send(Job::Check { storage: storage.clone(), mode });
    }

    /// Runs `action` after every job queued so far for `storage` has finished (libtorrent
    /// `disk_job_fence`). Answered by [`Completion::Fenced`].
    pub fn async_fence(&self, storage: &Arc<Storage>, action: FenceAction) {
        storage.abort.store(true, Ordering::Relaxed);
        let fence = Arc::new(Fence { remaining: AtomicUsize::new(self.fenced_threads()), action });
        for tx in self.threads.iter().chain([&self.hasher]) {
            let _ = tx.send(Job::Fence { storage: storage.clone(), fence: fence.clone() });
        }
    }

    /// Applies new file priorities after every job queued so far for `storage`, and before any
    /// queued later (libtorrent `async_set_file_priority`, a fence job). A file leaving priority
    /// 0 gets its data moved out of the part file; one going to 0 starts using the part file if
    /// it does not exist. Answered by [`Completion::FilePriority`].
    pub fn async_set_file_priority(&mut self, storage: &Arc<Storage>, priorities: Vec<u8>) {
        self.barrier(storage, BarrierAction::FilePriority(priorities));
    }

    /// Moves the storage's files to `path` after every job queued so far for `storage`, and
    /// before any queued later (libtorrent `async_move_storage`, a fence job). Answered by
    /// [`Completion::Moved`].
    pub fn async_move_storage(&mut self, storage: &Arc<Storage>, path: String, flags: MoveFlags) {
        self.moves.queue(storage.id, complete(&path), storage.info.total_size());
        self.barrier(storage, BarrierAction::Move { path, flags });
    }

    /// Renames file `file` to `name` (relative to the save path, or absolute) after every job
    /// queued so far for `storage`, and before any queued later (libtorrent `async_rename_file`,
    /// a fence job). Answered by [`Completion::Renamed`].
    pub fn async_rename_file(&mut self, storage: &Arc<Storage>, file: u32, name: String) {
        self.barrier(storage, BarrierAction::Rename { file, name });
    }

    /// D77: gives each file the name it should have, after every job queued so far for
    /// `storage` and before any queued later (a barrier, as a rename): `.!ut` after the files
    /// whose `incomplete` entry is true (where they may carry one), the plain name for the rest.
    /// A file is closed before it is renamed and never renamed over an existing one; one that
    /// cannot be renamed keeps its name and is used under it. Answered by [`Completion::Names`].
    pub fn async_set_names(&mut self, storage: &Arc<Storage>, incomplete: Vec<bool>) {
        self.barrier(storage, BarrierAction::Names(incomplete));
    }

    fn barrier(&mut self, storage: &Arc<Storage>, action: BarrierAction) {
        let seq = self.next_barrier;
        self.next_barrier += 1;
        let barrier = Arc::new(Barrier { remaining: AtomicUsize::new(self.fenced_threads()), seq, action });
        for tx in self.threads.iter().chain([&self.hasher]) {
            let _ = tx.send(Job::Barrier { storage: storage.clone(), barrier: barrier.clone() });
        }
    }

    /// Bytes handed to `async_write` and not written yet. Peers stop reading from their sockets
    /// while this exceeds `max_queued_disk_bytes` (libtorrent's disk-queue back-pressure).
    pub fn queued_write_bytes(&self) -> u64 {
        self.queued_write_bytes.load(Ordering::Relaxed)
    }

    /// The counter behind [`DiskIo::queued_write_bytes`], which outlives the engine.
    pub fn queued_write_counter(&self) -> Arc<AtomicU64> {
        self.queued_write_bytes.clone()
    }

    /// The moves queued and running now.
    pub fn moves(&self) -> Vec<MoveProgress> {
        self.moves.running.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    /// Cancels the moves running and every later one: each stops at the next file or copied
    /// chunk, deletes its copies and keeps the old folder (a closing session's "Quit now").
    pub fn cancel_moves(&self) {
        self.moves.cancel.store(true, Ordering::Relaxed);
    }
}

impl Drop for DiskIo {
    fn drop(&mut self) {
        for tx in self.threads.iter().chain([&self.hasher]) {
            let _ = tx.send(Job::Shutdown);
        }
        for h in self.handles.drain(..) {
            let _ = h.join();
        }
    }
}

/// Hashes one piece as its torrent needs it, fed whole blocks in order: SHA-1 over the piece,
/// pad bytes included (v1), and SHA-256 over each 16 KiB block of the file data in it (v2,
/// when the piece's v2 hash is known).
struct PieceHasher {
    sha1: Option<Sha1>,
    /// The block hashes so far, and the piece's v2 size (it ends at the end of its file).
    v2: Option<(Vec<Sha256Hash>, u32)>,
    fed: u32,
}

impl PieceHasher {
    fn new(info: &TorrentInfo, piece: u32) -> PieceHasher {
        let sha1 = info.v1().then(Sha1::new);
        let v2 = info.v2_piece_hash(piece).map(|_| (Vec::new(), info.orig_files().piece_size2(piece)));
        PieceHasher { sha1, v2, fed: 0 }
    }

    fn update(&mut self, block: &[u8]) {
        if let Some(h) = self.sha1.as_mut() {
            h.update(block);
        }
        if let Some((blocks, size2)) = self.v2.as_mut() {
            if self.fed < *size2 {
                let n = (block.len() as u32).min(*size2 - self.fed) as usize;
                blocks.push(Sha256::digest(&block[..n]));
            }
        }
        self.fed += block.len() as u32;
    }

    /// libtorrent `on_piece_verified` (downloads) and `on_piece_hashed` (`checking`: a v1
    /// failure is final there, the v2 hash is not looked at).
    fn verdict(self, info: &TorrentInfo, piece: u32, checking: bool) -> HashVerdict {
        let v1 = self.sha1.map(|h| Some(h.finalize()) == info.hash_for_piece(piece));
        if checking && v1 == Some(false) {
            return HashVerdict::Failed;
        }
        let v2 = self.v2.and_then(|(blocks, _)| {
            let (expected, leafs) = info.v2_piece_hash(piece)?;
            Some(
                !blocks.is_empty()
                    && blocks.len() <= leafs
                    && merkle_piece_root(&blocks, leafs, &mut Vec::new()) == expected,
            )
        });
        match (v1, v2) {
            (Some(a), Some(b)) if a != b => HashVerdict::Inconsistent,
            (Some(true), _) | (_, Some(true)) => HashVerdict::Passed,
            _ => HashVerdict::Failed,
        }
    }
}

/// Incremental hash of one piece being downloaded.
struct PartialHash {
    hasher: PieceHasher,
    /// Next block to feed to the hasher.
    cursor: u32,
    received: Vec<bool>,
    /// Blocks written ahead of the cursor, kept to avoid reading them back.
    cached: HashMap<u32, Vec<u8>>,
    /// Blocks restored from resume data. A read error on one of them fails the piece (the data
    /// is downloaded again) instead of failing the torrent.
    restored: Vec<bool>,
}

impl PartialHash {
    fn new(info: &TorrentInfo, piece: u32, num_blocks: u32) -> PartialHash {
        // blocks wholly in pad files are never downloaded (the piece picker finishes them at
        // once); they hash as the zeros they read as
        let pad_blocks = info.files().pad_bytes_in_piece(piece) / BLOCK_SIZE;
        let payload = num_blocks.saturating_sub(pad_blocks) as usize;
        let mut received = vec![false; num_blocks as usize];
        received[payload..].fill(true);
        PartialHash {
            hasher: PieceHasher::new(info, piece),
            cursor: 0,
            received,
            cached: HashMap::new(),
            restored: vec![false; num_blocks as usize],
        }
    }
}

/// One disk thread's state.
#[derive(Default)]
struct Worker {
    partial: HashMap<(u64, u32), PartialHash>,
    cached_bytes: usize,
    /// Storages behind a barrier this thread has passed, by barrier sequence number.
    blocked: HashMap<u64, u64>,
    /// Their jobs, in arrival order.
    parked: Vec<Job>,
    cache: ReadCache,
    /// The thread's queue; a running check takes from it between pieces (`None` in tests).
    rx: Option<mpsc::Receiver<Job>>,
    /// Jobs taken during a check that wait for it to end, in arrival order.
    deferred: Vec<Job>,
    /// A shutdown was taken during a check: the thread ends after the check's job.
    shutdown: bool,
}

fn disk_thread(rx: mpsc::Receiver<Job>, ctx: Ctx) {
    let mut w = Worker { rx: Some(rx), ..Worker::default() };
    loop {
        // Wake up for the read cache's expiry while it holds lines (an expiry of 0 is never).
        let expiry = ctx.read_cache.expiry();
        let check = if expiry.is_zero() { None } else { w.cache.next_check(expiry) };
        let rx = w.rx.as_ref().expect("the thread's queue");
        let job = match check {
            Some(at) => match rx.recv_timeout(at.saturating_duration_since(Instant::now())) {
                Ok(job) => Some(job),
                Err(mpsc::RecvTimeoutError::Timeout) => None,
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            },
            None => match rx.recv() {
                Ok(job) => Some(job),
                Err(_) => break,
            },
        };
        match job {
            Some(Job::Shutdown) => break,
            Some(job) => w.handle(job, &ctx),
            None => w.cache.expire(Instant::now(), expiry),
        }
        if let Some(b) = ctx.stats.cache_bytes.get(ctx.index) {
            b.store(w.cache.bytes(), Ordering::Relaxed);
        }
        if w.shutdown {
            break;
        }
    }
}

/// The checking threads' router (`lt-hash`): a check goes to the thread of its drive, so a drive
/// checks one torrent at a time while other drives check theirs, up to `active_checking` at once
/// (D48). Fences, barriers and the shutdown go to every checking thread in the order they came,
/// so each thread sees a storage's jobs in order.
fn check_router(rx: mpsc::Receiver<Job>, ctx: Ctx, checkers: Vec<mpsc::Sender<Job>>) {
    let mut threads = CheckThreads::default();
    while let Ok(job) = rx.recv() {
        match job {
            Job::Check { storage, mode } => {
                // a stopped torrent's check is dropped unread; no need to find its drive
                let t = if aborted(&storage) {
                    0
                } else {
                    threads.thread_for(&ctx.drive(&storage).id.key, ctx.config.checks())
                };
                let _ = checkers[t].send(Job::Check { storage, mode });
            }
            Job::Fence { storage, fence } => {
                for tx in &checkers {
                    let _ = tx.send(Job::Fence { storage: storage.clone(), fence: fence.clone() });
                }
            }
            Job::Barrier { storage, barrier } => {
                for tx in &checkers {
                    let _ = tx.send(Job::Barrier { storage: storage.clone(), barrier: barrier.clone() });
                }
            }
            Job::Shutdown => {
                for tx in &checkers {
                    let _ = tx.send(Job::Shutdown);
                }
                break;
            }
            // nothing else is sent to the router
            other => {
                let _ = checkers[0].send(other);
            }
        }
    }
}

impl Worker {
    fn handle(&mut self, job: Job, ctx: &Ctx) {
        if job.storage_id().is_some_and(|id| self.blocked.contains_key(&id)) {
            self.parked.push(job);
            return;
        }
        let partial = &mut self.partial;
        let cached_bytes = &mut self.cached_bytes;
        match job {
            Job::Write { storage, piece, offset, data } => {
                self.cache.drop_piece(storage.id, piece);
                let length = data.len() as u32;
                let drive = ctx.drive_for_job(&storage);
                let slot = drive.as_ref().map(|d| d.enter(ctx.config.tune()));
                let started = Instant::now();
                let result = write_block(&storage, ctx, partial, cached_bytes, piece, offset, data);
                if let Some(d) = drive.as_ref().filter(|_| ctx.config.measure()) {
                    d.record(true, started.elapsed());
                }
                drop(slot);
                ctx.queued_write_bytes.fetch_sub(length as u64, Ordering::Relaxed);
                match result {
                    Ok(hashed) => {
                        ctx.send(Completion::Write { storage: storage.id, piece, offset, error: None });
                        if let Some(verdict) = hashed {
                            ctx.send(Completion::Hashed { storage: storage.id, piece, verdict });
                        }
                    }
                    Err(e) => {
                        drop_partial(partial, cached_bytes, storage.id, piece);
                        ctx.send(Completion::Write { storage: storage.id, piece, offset, error: Some(e) });
                    }
                }
            }
            Job::Restore { storage, piece, blocks } => {
                let n = storage.info.piece_size(piece).div_ceil(BLOCK_SIZE);
                let state = partial
                    .entry((storage.id, piece))
                    .or_insert_with(|| PartialHash::new(&storage.info, piece, n));
                for (b, &on_disk) in blocks.iter().enumerate().take(n as usize) {
                    if on_disk && !state.received[b] {
                        state.received[b] = true;
                        state.restored[b] = true;
                    }
                }
                match advance_hash(&storage, ctx, partial, cached_bytes, piece) {
                    Ok(Some(verdict)) => ctx.send(Completion::Hashed { storage: storage.id, piece, verdict }),
                    Ok(None) => {}
                    Err(_) => {
                        // Only possible if writes were queued before the restore: fail the piece.
                        drop_partial(partial, cached_bytes, storage.id, piece);
                        let verdict = HashVerdict::Failed;
                        ctx.send(Completion::Hashed { storage: storage.id, piece, verdict });
                    }
                }
            }
            Job::Read { storage, piece, offset, length, cookie } => {
                let drive = ctx.drive_for_job(&storage);
                let result =
                    read_for_peer(&mut self.cache, &storage, ctx, drive.as_deref(), piece, offset, length);
                ctx.send(Completion::Read { storage: storage.id, cookie, piece, offset, result });
            }
            Job::Verify { storage, piece } => {
                let mut buf = vec![0u8; storage.info.piece_size(piece) as usize];
                let drive = ctx.drive_for_job(&storage);
                let slot = drive.as_ref().map(|d| d.enter(ctx.config.tune()));
                let read = read_into(&storage, &ctx.pool, piece, 0, &mut buf);
                drop(slot);
                let verdict = match read {
                    Ok(_) => {
                        let mut hasher = PieceHasher::new(&storage.info, piece);
                        for block in buf.chunks(BLOCK_SIZE as usize) {
                            hasher.update(block);
                        }
                        hasher.verdict(&storage.info, piece, true)
                    }
                    Err(_) => HashVerdict::Failed,
                };
                ctx.send(Completion::Verified { storage: storage.id, piece, verdict });
            }
            Job::Check { storage, mode } => {
                if !aborted(&storage) {
                    let result =
                        check_files(&storage, ctx, mode, &mut || self.poll_during_check(&storage, ctx));
                    ctx.send(Completion::Checked { storage: storage.id, result });
                }
                self.run_deferred(ctx);
            }
            Job::Fence { storage, fence } => {
                let id = storage.id;
                self.cache.drop_storage(id);
                let keys: Vec<_> = partial.keys().filter(|k| k.0 == id).copied().collect();
                for (_, piece) in keys {
                    drop_partial(partial, cached_bytes, id, piece);
                }
                if fence.remaining.fetch_sub(1, Ordering::AcqRel) == 1 {
                    ctx.pool.release(id);
                    if let Some(part) = storage.lock_files().part.take() {
                        // libtorrent ignores this error too; the header is rewritten next time.
                        let _ = part.release();
                    }
                    let (error, log) = match fence.action {
                        FenceAction::Release => (None, Vec::new()),
                        FenceAction::Delete => {
                            let (r, log) = delete_files(&storage);
                            (r.err(), log)
                        }
                        FenceAction::DeletePartFile => {
                            (delete_part_file(&storage).err().map(Error::from), Vec::new())
                        }
                    };
                    ctx.send(Completion::Fenced { storage: id, action: fence.action, error, log });
                }
            }
            Job::Barrier { storage, barrier } => {
                // A move, rename or priority change may put the storage's blocks elsewhere.
                self.cache.drop_storage(storage.id);
                if barrier.remaining.fetch_sub(1, Ordering::AcqRel) != 1 {
                    self.blocked.insert(storage.id, barrier.seq);
                    return;
                }
                // Every thread has passed it: nothing else of this storage is running.
                match &barrier.action {
                    BarrierAction::FilePriority(priorities) => {
                        let error = set_file_priority(&storage, ctx, priorities).err();
                        ctx.send(Completion::FilePriority { storage: storage.id, error });
                    }
                    BarrierAction::Move { path, flags } => {
                        let (status, save_path, error) = move_storage(&storage, ctx, path, *flags);
                        ctx.send(Completion::Moved { storage: storage.id, status, save_path, error });
                    }
                    BarrierAction::Rename { file, name } => {
                        let error = rename_file(&storage, ctx, *file as usize, name).err();
                        let (file, name) = (*file, name.clone());
                        ctx.send(Completion::Renamed { storage: storage.id, file, name, error });
                    }
                    BarrierAction::Names(incomplete) => {
                        let (failed, pending) = set_names(&storage, ctx, incomplete);
                        ctx.send(Completion::Names { storage: storage.id, failed, pending });
                    }
                }
                for (i, tx) in ctx.queues.get().into_iter().flatten().enumerate() {
                    if i != ctx.index {
                        let _ = tx.send(Job::Unblock { storage: storage.id, seq: barrier.seq });
                    }
                }
            }
            Job::Unblock { storage, seq } => {
                if self.blocked.get(&storage) != Some(&seq) {
                    return;
                }
                self.blocked.remove(&storage);
                let (mine, rest) = std::mem::take(&mut self.parked)
                    .into_iter()
                    .partition(|j| j.storage_id() == Some(storage));
                self.parked = rest;
                // A parked barrier blocks the storage again and parks the rest anew, in order.
                for job in mine {
                    self.handle(job, ctx);
                }
            }
            Job::Shutdown => {}
        }
    }

    /// Called by a running check between pieces: takes the jobs queued meanwhile. A fence or
    /// barrier of another storage runs now, so a stop, a move or a priority change does not
    /// wait for the check, which may take hours (the I/O threads park that storage's writes
    /// behind the barrier, and the parked bytes would hold the engine's write queue shut until
    /// the check ended). Everything else waits in `deferred` for the check to end: the checked
    /// storage's jobs, other checks, unblocks (one would replay a parked check inside this one)
    /// and a fence or barrier whose storage already has a job waiting, so each storage's jobs
    /// still run in order.
    fn poll_during_check(&mut self, checking: &Storage, ctx: &Ctx) {
        loop {
            let job = match self.rx.as_ref().map(|rx| rx.try_recv()) {
                Some(Ok(job)) => job,
                _ => return,
            };
            let id = job.storage_id();
            let now = matches!(job, Job::Fence { .. } | Job::Barrier { .. })
                && id != Some(checking.id)
                && !self.deferred.iter().any(|j| j.storage_id() == id);
            if now {
                self.handle(job, ctx);
            } else {
                self.deferred.push(job);
            }
        }
    }

    /// Runs the jobs a check held back, in order. A check among them may hold back more, which
    /// go behind the rest. A shutdown ends the thread as it would have from the queue.
    fn run_deferred(&mut self, ctx: &Ctx) {
        while !self.shutdown && !self.deferred.is_empty() {
            match self.deferred.remove(0) {
                Job::Shutdown => self.shutdown = true,
                job => self.handle(job, ctx),
            }
        }
    }
}

fn drop_partial(
    partial: &mut HashMap<(u64, u32), PartialHash>,
    cached_bytes: &mut usize,
    storage: u64,
    piece: u32,
) {
    if let Some(p) = partial.remove(&(storage, piece)) {
        *cached_bytes -= p.cached.values().map(Vec::len).sum::<usize>();
    }
}

fn io_error(storage: &Storage, file: usize, e: io::Error) -> (String, Error) {
    (storage.file_path(file).to_string_lossy().into_owned(), Error::from(e))
}

/// Writes one block and advances the piece's hash. Returns the verdict once the whole piece
/// has been hashed.
fn write_block(
    storage: &Storage,
    ctx: &Ctx,
    partial: &mut HashMap<(u64, u32), PartialHash>,
    cached_bytes: &mut usize,
    piece: u32,
    offset: u32,
    data: Vec<u8>,
) -> Result<Option<HashVerdict>, (String, Error)> {
    let fs = storage.info.files();
    let piece_size = storage.info.piece_size(piece);
    let num_blocks = piece_size.div_ceil(BLOCK_SIZE);
    let block = offset / BLOCK_SIZE;
    let state = partial
        .entry((storage.id, piece))
        .or_insert_with(|| PartialHash::new(&storage.info, piece, num_blocks));
    if block >= num_blocks || state.received[block as usize] {
        // Out of range or a duplicate (end-game): nothing new to write.
        return Ok(None);
    }

    let mut pos = 0usize;
    for slice in fs.map_block(piece, offset as u64, data.len() as u64) {
        let n = slice.size as usize;
        if storage.skip_file(slice.file_index) {
            // pad files are not written
        } else if let Some(part) = storage.part_file_for(slice.file_index) {
            part.write(&data[pos..pos + n], piece, offset + pos as u32).map_err(|e| storage.part_error(e))?;
        } else {
            let file = ctx
                .pool
                .open(storage.id, slice.file_index as u32, &storage.file_path(slice.file_index), true)
                .map_err(|e| io_error(storage, slice.file_index, e))?;
            write_at(&file, &data[pos..pos + n], slice.offset)
                .map_err(|e| io_error(storage, slice.file_index, e))?;
            storage.note_ours(slice.file_index);
        }
        pos += n;
    }
    state.received[block as usize] = true;

    if block == state.cursor {
        state.hasher.update(&data);
        state.cursor += 1;
    } else if *cached_bytes + data.len() <= REORDER_CACHE_BYTES {
        *cached_bytes += data.len();
        state.cached.insert(block, data);
    }
    advance_hash(storage, ctx, partial, cached_bytes, piece)
}

/// Feeds the hasher every received block from the cursor on, reading back blocks that are not
/// cached. Returns the verdict once the whole piece has been hashed; the state is dropped then.
fn advance_hash(
    storage: &Storage,
    ctx: &Ctx,
    partial: &mut HashMap<(u64, u32), PartialHash>,
    cached_bytes: &mut usize,
    piece: u32,
) -> Result<Option<HashVerdict>, (String, Error)> {
    let fs = storage.info.files();
    let piece_size = storage.info.piece_size(piece);
    let num_blocks = piece_size.div_ceil(BLOCK_SIZE);
    let Some(state) = partial.get_mut(&(storage.id, piece)) else { return Ok(None) };
    while state.cursor < num_blocks && state.received[state.cursor as usize] {
        let b = state.cursor;
        match state.cached.remove(&b) {
            Some(buf) => {
                *cached_bytes -= buf.len();
                state.hasher.update(&buf);
            }
            None => {
                let len = (piece_size - b * BLOCK_SIZE).min(BLOCK_SIZE);
                match read_block(storage, &ctx.pool, piece, b * BLOCK_SIZE, len) {
                    Ok(buf) => {
                        state.hasher.update(&buf);
                    }
                    Err(_) if state.restored[b as usize] => {
                        // The resume data promised a block the files do not hold.
                        drop_partial(partial, cached_bytes, storage.id, piece);
                        return Ok(Some(HashVerdict::Failed));
                    }
                    Err(e) => {
                        let file = storage.file_path(fs.file_index_at_piece(piece));
                        return Err((file.to_string_lossy().into_owned(), e));
                    }
                }
            }
        }
        state.cursor += 1;
    }
    if state.cursor < num_blocks {
        return Ok(None);
    }
    let state = partial.remove(&(storage.id, piece)).expect("present");
    Ok(Some(state.hasher.verdict(&storage.info, piece, false)))
}

/// Reads a block a peer asked for: from the read cache, else the cache line holding it, read
/// with one call and kept, else (cache off, or the line could not be read) just the block.
fn read_for_peer(
    cache: &mut ReadCache,
    storage: &Storage,
    ctx: &Ctx,
    drive: Option<&Drive>,
    piece: u32,
    offset: u32,
    length: u32,
) -> Result<Vec<u8>, Error> {
    let stats = &ctx.stats;
    stats.blocks_read.fetch_add(1, Ordering::Relaxed);
    // A file read: within the drive's jobs in flight, and timed for its busy signal.
    let file_read = |buf: &mut [u8], at: u32| {
        stats.read_ops.fetch_add(1, Ordering::Relaxed);
        let _slot = drive.map(|d| d.enter(ctx.config.tune()));
        let started = Instant::now();
        let r = read_into(storage, &ctx.pool, piece, at, buf);
        if let Some(d) = drive.filter(|_| ctx.config.measure()) {
            d.record(false, started.elapsed());
        }
        r
    };
    let block = |buf: &mut Vec<u8>| file_read(buf, offset);
    let Some((limit, line)) = ctx.read_cache.get() else {
        cache.trim(0);
        let mut buf = vec![0u8; length as usize];
        return block(&mut buf).map(|_| buf);
    };
    cache.trim(limit);
    let piece_size = storage.info.piece_size(piece);
    let start = offset / line * line;
    let end = start.saturating_add(line).min(piece_size);
    if offset.saturating_add(length) > end || length == 0 {
        let mut buf = vec![0u8; length as usize];
        return block(&mut buf).map(|_| buf);
    }
    let key = (storage.id, piece, start);
    let now = Instant::now();
    if let Some(data) = cache.read(key, offset, length, now) {
        stats.cache_hits.fetch_add(1, Ordering::Relaxed);
        return Ok(data);
    }
    let mut buf = vec![0u8; (end - start) as usize];
    if file_read(&mut buf, start).is_err() {
        let mut buf = vec![0u8; length as usize];
        return block(&mut buf).map(|_| buf);
    }
    let at = (offset - start) as usize;
    let out = buf[at..at + length as usize].to_vec();
    cache.insert(key, buf, offset, limit, now);
    Ok(out)
}

/// Reads `length` bytes of `piece` at `offset`. Pad files read as zeros.
fn read_block(
    storage: &Storage,
    pool: &FilePool,
    piece: u32,
    offset: u32,
    length: u32,
) -> Result<Vec<u8>, Error> {
    let mut buf = vec![0u8; length as usize];
    read_into(storage, pool, piece, offset, &mut buf)?;
    Ok(buf)
}

fn read_into(
    storage: &Storage,
    pool: &FilePool,
    piece: u32,
    offset: u32,
    buf: &mut [u8],
) -> Result<(), Error> {
    let fs = storage.info.files();
    let mut pos = 0usize;
    for slice in fs.map_block(piece, offset as u64, buf.len() as u64) {
        let n = slice.size as usize;
        if storage.skip_file(slice.file_index) {
            // pad files read as zeros (the buffer may hold an earlier piece)
            buf[pos..pos + n].fill(0);
        } else if let Some(part) = storage.part_file_for(slice.file_index) {
            part.read(&mut buf[pos..pos + n], piece, offset + pos as u32)?;
        } else {
            let file =
                pool.open(storage.id, slice.file_index as u32, &storage.file_path(slice.file_index), false)?;
            if read_at(&file, &mut buf[pos..pos + n], slice.offset)? != n {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "file too short").into());
            }
        }
        pos += n;
    }
    if pos != buf.len() {
        return Err(Error::Io {
            kind: io::ErrorKind::InvalidInput,
            message: "read past the end of the torrent".into(),
        });
    }
    Ok(())
}

/// `poll` is called before the pieces are hashed and after each one: the thread's chance to take
/// the jobs queued meanwhile ([`Worker::poll_during_check`]).
fn check_files(
    storage: &Storage,
    ctx: &Ctx,
    mode: CheckMode,
    poll: &mut dyn FnMut(),
) -> Result<CheckResult, (String, Error)> {
    let fs = storage.info.files();
    let num_pieces = storage.info.num_pieces();
    let mut resume_failed = false;
    // D77: every file under the name it has on disk, before anything looks at the files; the
    // lengths of those found serve the resume data's test below
    let mut sizes = vec![None; fs.num_files()];
    let names = storage.probe(matches!(mode, CheckMode::Full), Some(&mut sizes));

    // libtorrent `posix_storage::initialize`: a skipped file that exists keeps its blocks; the
    // others go to the part file.
    let skipped = {
        let mut st = storage.lock_files();
        let skipped: Vec<bool> =
            (0..fs.num_files()).map(|f| st.priority[f] == 0 && !fs.pad_file_at(f)).collect();
        for (f, _) in skipped.iter().enumerate().filter(|(_, s)| **s) {
            st.use_partfile[f] = !storage.file_path(f).exists();
            if st.use_partfile[f] && fs.file_size(f) > 0 {
                storage.need_partfile(&mut st);
            }
        }
        skipped
    };

    export_unskipped(storage, ctx)?;
    poll();

    // Empty files hold no pieces, so nothing else would ever create them. Skipped ones are not
    // created.
    for (f, &skip) in skipped.iter().enumerate() {
        if aborted(storage) {
            break;
        }
        if fs.file_size(f) == 0 && !storage.skip_file(f) && !skip {
            let path = storage.file_path(f);
            if !path.exists() {
                ctx.pool.open(storage.id, f as u32, &path, true).map_err(|e| io_error(storage, f, e))?;
            }
        }
    }

    if let CheckMode::Resume(mut have) = mode {
        let files = if have.len() == num_pieces as usize {
            resume_files(storage, &have, &sizes)
        } else {
            ResumeFiles::Mismatch
        };
        // the pieces the resume data has are ours (those of a file that is gone go below)
        let have_files = |have: &Bitfield| storage.note_data(&|p| have.get(p as usize), &[], None);
        match files {
            ResumeFiles::Match => {
                have_files(&have);
                return Ok(CheckResult {
                    have,
                    resume_failed: false,
                    inconsistent: None,
                    missing: Vec::new(),
                    names,
                });
            }
            // Our addition (libtorrent checks every file): a file that is gone took only its
            // own pieces with it. The other files are as the resume data left them, so they
            // are not hashed again because of it (a virus scanner that removes one small file
            // of a large torrent after each download made every start hash all of it).
            ResumeFiles::Missing(missing) => {
                for piece in 0..num_pieces {
                    if have.get(piece as usize)
                        && fs
                            .map_block(piece, 0, storage.info.piece_size(piece) as u64)
                            .iter()
                            .any(|s| missing.contains(&s.file_index))
                    {
                        have.set(piece as usize, false);
                    }
                }
                have_files(&have);
                return Ok(CheckResult { have, resume_failed: false, inconsistent: None, missing, names });
            }
            ResumeFiles::Mismatch => resume_failed = true,
        }
    }

    let mut have = Bitfield::new(num_pieces as usize, false);
    // Nothing on disk: nothing to hash. Stopped meanwhile: nobody wants the result.
    let any_file = storage.lock_files().part.as_ref().is_some_and(|p| !p.is_empty())
        || (0..fs.num_files()).any(|f| {
            !aborted(storage) && fs.file_size(f) > 0 && !storage.skip_file(f) && storage.file_path(f).exists()
        });
    if !any_file || aborted(storage) {
        return Ok(CheckResult { have, resume_failed, inconsistent: None, missing: Vec::new(), names });
    }
    let mut buf = vec![0u8; storage.info.piece_length() as usize];
    let report_every = (num_pieces / 100).max(16);
    let mut inconsistent = None;
    // Gently (D48): background I/O priority, the check's own file handles so the system can be
    // told to read ahead and then forget what was read, and waits while the drive is slow.
    let gentle = ctx.config.gentle();
    let background = gentle.then(gentle::BackgroundIo::begin);
    if background.as_ref().is_some_and(|b| !b.active()) {
        ctx.drives.no_background_io();
    }
    let pool = FilePool::new(8);
    let mut advised = std::collections::HashSet::new();
    let mut pacer = Pacer::new(&ctx.config, Instant::now());
    for piece in 0..num_pieces {
        if aborted(storage) {
            break;
        }
        let size = storage.info.piece_size(piece) as usize;
        let started = Instant::now();
        let read = read_into(storage, &pool, piece, 0, &mut buf[..size]);
        let now = Instant::now();
        let wait = pacer.after_read(size, now.duration_since(started), now);
        if gentle {
            for slice in fs.map_block(piece, 0, size as u64) {
                let f = slice.file_index;
                if storage.skip_file(f) || storage.part_file_for(f).is_some() {
                    continue;
                }
                if let Ok(file) = pool.open(storage.id, f as u32, &storage.file_path(f), false) {
                    if advised.insert(f) {
                        gentle::advise_sequential(&file);
                    }
                    gentle::advise_done(&file, slice.offset, slice.size);
                }
            }
        }
        if read.is_ok() {
            let mut hasher = PieceHasher::new(&storage.info, piece);
            for block in buf[..size].chunks(BLOCK_SIZE as usize) {
                hasher.update(block);
            }
            match hasher.verdict(&storage.info, piece, true) {
                HashVerdict::Passed => {
                    have.set(piece as usize, true);
                    storage.note_piece(piece);
                }
                HashVerdict::Failed => {}
                HashVerdict::Inconsistent => {
                    inconsistent = Some(piece);
                    break;
                }
            }
        }
        if (piece + 1) % report_every == 0 {
            ctx.send(Completion::CheckProgress { storage: storage.id, checked: piece + 1 });
        }
        pause(storage, wait);
        poll();
    }
    // Checking may open hundreds of files; don't keep them all.
    ctx.pool.release(storage.id);
    Ok(CheckResult { have, resume_failed, inconsistent, missing: Vec::new(), names })
}

/// The torrent was stopped or removed ([`DiskIo::async_fence`]), at shutdown too: a check
/// stops at the next file or piece, so one on a slow share does not hold up the exit. Its
/// result is dropped (the storage is no longer the torrent's).
fn aborted(storage: &Storage) -> bool {
    storage.abort.load(Ordering::Relaxed)
}

/// Waits `wait`, or less if the torrent is stopped meanwhile.
fn pause(storage: &Storage, wait: Duration) {
    let until = Instant::now() + wait;
    loop {
        let left = until.saturating_duration_since(Instant::now());
        if left.is_zero() || aborted(storage) {
            return;
        }
        std::thread::sleep(left.min(Duration::from_millis(100)));
    }
}

/// libtorrent `posix_storage::verify_resume_data`, reduced to what pread storage needs: every
/// file that holds part of a piece we claim to have must exist and reach that far. Where
/// libtorrent exempts skipped files, the part file must hold the piece.
enum ResumeFiles {
    Match,
    /// These files are gone and nothing else is wrong: only their pieces are lost.
    Missing(Vec<usize>),
    /// A file is shorter than the pieces need, cannot be looked at, or the part file lacks a
    /// piece: the data is not what the resume data says, so every piece is hashed.
    Mismatch,
}

/// `sizes`: the lengths of the files the probe found ([`Storage::probe`]); the others are looked
/// at here.
fn resume_files(storage: &Storage, have: &Bitfield, sizes: &[Option<u64>]) -> ResumeFiles {
    let fs = storage.info.files();
    let piece_len = storage.info.piece_length() as u64;
    let mut need = vec![0u64; fs.num_files()];
    for piece in (0..have.len()).filter(|&p| have.get(p)) {
        let size = storage.info.piece_size(piece as u32) as u64;
        for s in fs.map_block(piece as u32, 0, size) {
            if storage.skip_file(s.file_index) {
                continue;
            }
            match storage.part_file_for(s.file_index) {
                Some(part) if !part.has_piece(piece as u32) => return ResumeFiles::Mismatch,
                Some(_) => {}
                None => need[s.file_index] = need[s.file_index].max(s.offset + s.size),
            }
        }
        debug_assert!(piece as u64 * piece_len < fs.total_size());
    }
    let mut missing = Vec::new();
    for (f, &n) in need.iter().enumerate() {
        if n == 0 || storage.skip_file(f) {
            continue;
        }
        if aborted(storage) {
            return ResumeFiles::Mismatch;
        }
        let len = match sizes.get(f).copied().flatten() {
            Some(len) => Ok(len),
            None => std::fs::metadata(storage.file_path(f)).map(|m| m.len()),
        };
        match len {
            Ok(len) if len >= n => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => missing.push(f),
            _ => return ResumeFiles::Mismatch,
        }
    }
    if missing.is_empty() { ResumeFiles::Match } else { ResumeFiles::Missing(missing) }
}

/// Deletes the torrent's files, then the directories that became empty (deepest first),
/// never the save path itself. Returns the Logger lines too.
///
/// D77: a file is deleted under a suffixed name (`.!ut`, `.!qB`) only when it holds data known
/// to be this torrent's ([`OURS`]: written by it, a piece of it checked, or pieces or blocks
/// of it the engine has): the probe takes a same-named partial file in the folder for the
/// file's, and that may be another client's, untouched by this torrent. A file under its
/// plain name is deleted as it always was.
fn delete_files(storage: &Storage) -> (Result<(), Error>, Vec<String>) {
    let fs = storage.info.files();
    let mut dirs = std::collections::BTreeSet::new();
    let mut first_error = None;
    let mut log = Vec::new();
    let save = PathBuf::from(storage.save_path());
    storage.probe_names(false);
    for f in 0..fs.num_files() {
        if fs.pad_file_at(f) {
            continue;
        }
        let path = storage.file_path(f);
        let (suffix, ours) = (storage.suffix(f), storage.ours(f));
        // the name the file has, and a `.!ut` one of ours left beside it (an incomplete
        // file's while the plain one is used); a plain file beside a `.!ut` one is not ours
        let mut paths = Vec::new();
        if suffix == Suffix::Plain || ours {
            paths.push((path.clone(), suffix != Suffix::Plain));
        } else if found(&path) {
            log.push(format!("\"{}\" is not known to be this torrent's data and is kept", path.display()));
        }
        let base = storage.base_path(f);
        if ours && suffix != Suffix::Ut && storage.may_suffix(f, &base) {
            let leftover = with_suffix(base, Suffix::Ut);
            if !storage.clashes(f, &leftover) {
                paths.push((leftover, true));
            }
        }
        for (p, suffixed) in paths {
            match std::fs::remove_file(&p) {
                Ok(()) if suffixed => log.push(format!("deleted \"{}\"", p.display())),
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) => {
                    first_error.get_or_insert(Error::from(e));
                }
            }
        }
        let mut p = path.parent();
        while let Some(dir) = p {
            if !dir.starts_with(&save) || dir == save {
                break;
            }
            dirs.insert(dir.to_path_buf());
            p = dir.parent();
        }
    }
    for dir in dirs.iter().rev() {
        let _ = std::fs::remove_dir(dir);
    }
    if let Err(e) = delete_part_file(storage) {
        first_error.get_or_insert(Error::from(e));
    }
    (first_error.map_or(Ok(()), Err), log)
}

/// Deletes the part file. One still open elsewhere forgets its pieces, so it is not written
/// back.
fn delete_part_file(storage: &Storage) -> io::Result<()> {
    let part = storage.part_file_path();
    let removed = match storage.part_files.get(&part) {
        Some(pf) => pf.delete(),
        None => std::fs::remove_file(&part),
    };
    match removed {
        Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e),
        _ => Ok(()),
    }
}

/// libtorrent `posix_storage::set_file_priority`. Runs with no other job of `storage` in
/// flight. Pieces left in the part file that no skipped file needs any more are freed.
fn set_file_priority(storage: &Storage, ctx: &Ctx, priorities: &[u8]) -> Result<(), (String, Error)> {
    let fs = storage.info.files();
    storage.probe_names(false);
    let mut st = storage.lock_files();
    for (f, &new) in priorities.iter().enumerate().take(fs.num_files()) {
        // pad files always have priority 0
        if fs.pad_file_at(f) {
            continue;
        }
        let old = st.priority[f];
        if old == 0 && new != 0 {
            if let (true, Some(part)) = (st.use_partfile[f], st.part.clone()) {
                export_file(storage, ctx, &part, f)?;
            }
        } else if old != 0 && new == 0 {
            // libtorrent does not move existing data into the part file: a file already on
            // disk keeps its blocks.
            st.use_partfile[f] = !storage.file_path(f).exists();
        }
        st.priority[f] = new;
        if new == 0 && st.use_partfile[f] && fs.file_size(f) > 0 {
            storage.need_partfile(&mut st);
        }
    }
    if let Some(part) = st.part.clone() {
        free_unneeded(storage, &st, &part)?;
    }
    Ok(())
}

/// Moves file `f`'s blocks out of the part file into the file. A file not on disk yet is
/// written under a temporary name and renamed once whole, and the part file frees the blocks
/// only then: an interrupted export leaves no short file that the next start would take for
/// the exported one (and free the rest of its blocks); it runs again instead.
fn export_file(storage: &Storage, ctx: &Ctx, part: &PartFile, f: usize) -> Result<(), (String, Error)> {
    let fs = storage.info.files();
    let (offset, size) = (fs.file_offset(f), fs.file_size(f));
    let path = storage.file_path(f);
    if path.exists() {
        return part
            .export_file(
                offset,
                size,
                |offset, buf| {
                    let file = ctx.pool.open(storage.id, f as u32, &path, true)?;
                    write_at(&file, buf, offset)
                },
                || Ok(()),
            )
            .map_err(|e| io_error(storage, f, e));
    }
    let tmp = export_temp(storage, &path);
    // what an earlier interrupted export left
    let _ = std::fs::remove_file(&tmp);
    let file = std::cell::RefCell::new(None);
    let r = part.export_file(
        offset,
        size,
        |offset, buf| {
            let mut file = file.borrow_mut();
            if file.is_none() {
                *file = Some(file_pool::open_file(&tmp, true)?);
            }
            write_at(file.as_ref().expect("opened above"), buf, offset)
        },
        || match file.borrow_mut().take() {
            Some(file) => {
                file.sync_data()?;
                // closed before the rename, which Windows refuses for an open file
                drop(file);
                file_pool::rename_no_replace(&tmp, &path)
            }
            // nothing was in the part file: no file, as before
            None => Ok(()),
        },
    );
    if r.is_err() {
        drop(file.borrow_mut().take());
        let _ = std::fs::remove_file(&tmp);
    }
    r.map_err(|e| io_error(storage, f, e))
}

/// The name [`export_file`] writes `path` under: `path` with [`EXPORT_EXT`] (a suffix is given
/// only where that still fits, [`suffix_fits`]), unless a file or directory of the torrent has
/// that name, which the export would delete first and then take: then `.1.pa-export`,
/// `.2.pa-export`, ... beside it, the first such name the torrent has no path of (each ends
/// like an export, so [`Storage::taken`] knows it if the torrent has it).
fn export_temp(storage: &Storage, path: &Path) -> PathBuf {
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(EXPORT_EXT);
    let tmp = PathBuf::from(tmp);
    if !storage.taken(&tmp) {
        return tmp;
    }
    (1u32..)
        .map(|n| path.with_file_name(format!(".{n}{EXPORT_EXT}")))
        .find(|p| !storage.taken(p))
        .expect("the torrent has fewer paths than names")
}

/// Frees the part file's pieces that overlap no file kept in it, then flushes the header (which
/// deletes the file once it is empty).
fn free_unneeded(storage: &Storage, st: &FileState, part: &PartFile) -> Result<(), (String, Error)> {
    let fs = storage.info.files();
    let uses_part = |f: usize| st.priority[f] == 0 && st.use_partfile[f] && !fs.pad_file_at(f);
    part.retain(|piece| {
        piece < storage.info.num_pieces()
            && fs
                .map_block(piece, 0, storage.info.piece_size(piece) as u64)
                .iter()
                .any(|s| uses_part(s.file_index))
    });
    part.flush_metadata().map_err(|e| storage.part_error(e))
}

/// A torrent has no storage while it is stopped, so a file unskipped then still has its
/// blocks in the part file: move them out before the check, as `set_file_priority` would have.
fn export_unskipped(storage: &Storage, ctx: &Ctx) -> Result<(), (String, Error)> {
    let fs = storage.info.files();
    let mut st = storage.lock_files();
    if st.part.is_none() && !storage.part_file_path().exists() {
        return Ok(());
    }
    let part = storage.need_partfile(&mut st);
    if part.is_empty() {
        return Ok(());
    }
    #[allow(clippy::needless_range_loop)] // `f` is the file index of the storage calls too
    for f in 0..fs.num_files() {
        // the part file keeps what was not exported yet; the next check goes on
        if aborted(storage) {
            return Ok(());
        }
        if st.priority[f] != 0
            && !storage.skip_file(f)
            && fs.file_size(f) > 0
            && !storage.file_path(f).exists()
        {
            export_file(storage, ctx, &part, f)?;
        }
    }
    free_unneeded(storage, &st, &part)
}

/// libtorrent `complete`: `path` made absolute against the working directory.
pub(crate) fn complete(path: &str) -> String {
    let p = if path.is_empty() { "." } else { path };
    std::path::absolute(p).map_or_else(|_| p.to_owned(), |a| a.to_string_lossy().into_owned())
}

/// The error of a move cancelled by [`DiskIo::cancel_moves`].
fn cancelled() -> io::Error {
    io::Error::new(io::ErrorKind::Interrupted, "move cancelled at exit")
}

/// Copies `from` to `to` (a move across volumes), passing the bytes copied to `progress` as
/// they go. Once `cancel` is set it stops with [`cancelled`] and leaves no partial copy.
#[cfg(windows)]
fn copy_file(from: &Path, to: &Path, cancel: &AtomicBool, progress: &mut dyn FnMut(u64)) -> io::Result<()> {
    use windows_sys::Win32::Foundation::{ERROR_REQUEST_ABORTED, HANDLE};
    use windows_sys::Win32::Storage::FileSystem::{
        CopyFileExW, LPPROGRESS_ROUTINE_CALLBACK_REASON, PROGRESS_CANCEL, PROGRESS_CONTINUE,
    };

    struct Progress<'a> {
        cancel: &'a AtomicBool,
        progress: &'a mut dyn FnMut(u64),
        reported: u64,
    }
    #[allow(clippy::too_many_arguments)]
    unsafe extern "system" fn routine(
        _total: i64,
        transferred: i64,
        _stream_size: i64,
        _stream_transferred: i64,
        _stream: u32,
        _reason: LPPROGRESS_ROUTINE_CALLBACK_REASON,
        _src: HANDLE,
        _dst: HANDLE,
        data: *const std::ffi::c_void,
    ) -> u32 {
        // SAFETY: `data` is the `Progress` below, alive for the whole CopyFileExW call, which
        // runs this routine on the calling thread.
        let p = unsafe { &mut *(data as *mut Progress<'_>) };
        let now = transferred.max(0) as u64;
        (p.progress)(now.saturating_sub(p.reported));
        p.reported = now;
        if p.cancel.load(Ordering::Relaxed) { PROGRESS_CANCEL } else { PROGRESS_CONTINUE }
    }

    let (src, dst) = (file_pool::wide_path(from)?, file_pool::wide_path(to)?);
    let mut p = Progress { cancel, progress, reported: 0 };
    // CopyFileExW is what `std::fs::copy` calls: the same attributes, and a share may copy on
    // the server. A cancelled copy deletes its partial file.
    let ok = unsafe {
        CopyFileExW(
            src.as_ptr(),
            dst.as_ptr(),
            Some(routine),
            &mut p as *mut Progress<'_> as *const std::ffi::c_void,
            std::ptr::null_mut(),
            0,
        )
    };
    if ok != 0 {
        return Ok(());
    }
    let e = io::Error::last_os_error();
    if e.raw_os_error() == Some(ERROR_REQUEST_ABORTED as i32) { Err(cancelled()) } else { Err(e) }
}

/// Copies `from` to `to` (a move across volumes), passing the bytes copied to `progress` as
/// they go. Once `cancel` is set it stops with [`cancelled`] and leaves no partial copy.
#[cfg(not(windows))]
fn copy_file(from: &Path, to: &Path, cancel: &AtomicBool, progress: &mut dyn FnMut(u64)) -> io::Result<()> {
    use std::io::{Read, Write};
    let mut copy = || -> io::Result<()> {
        let mut src = std::fs::File::open(from)?;
        let mut dst = std::fs::File::create(to)?;
        let mut buf = vec![0u8; 4 << 20];
        loop {
            if cancel.load(Ordering::Relaxed) {
                return Err(cancelled());
            }
            let n = match src.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            dst.write_all(&buf[..n])?;
            progress(n as u64);
        }
        dst.set_permissions(src.metadata()?.permissions())
    };
    copy().inspect_err(|_| {
        let _ = std::fs::remove_file(to);
    })
}

/// libtorrent `move_file`: creates the destination's directory, then renames.
fn move_file(old: &Path, new: &Path) -> Result<(), (FileOperation, io::Error)> {
    std::fs::metadata(old).map_err(|e| (FileOperation::FileStat, e))?;
    if let Some(parent) = new.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|e| (FileOperation::Mkdir, e))?;
    }
    if old == new {
        return Ok(());
    }
    std::fs::rename(old, new).map_err(|e| (FileOperation::FileRename, e))
}

fn is_not_found<T>(r: io::Result<T>) -> bool {
    matches!(r, Err(e) if e.kind() == io::ErrorKind::NotFound)
}

/// libtorrent `posix_storage::rename_file`: renames the file if it is on disk (making the
/// directories the new name lands in), then uses the new name for every later job. A file not
/// downloaded yet only changes its name. A file on disk is never renamed over an existing one:
/// that fails with `AlreadyExists` (libtorrent `file_exists`). Runs with no other job of
/// `storage` in flight.
fn rename_file(storage: &Storage, ctx: &Ctx, file: usize, name: &str) -> Result<(), (String, Error)> {
    let fs = storage.info.files();
    if file >= fs.num_files() {
        let e = io::Error::new(io::ErrorKind::InvalidInput, "no such file in the torrent");
        return Err((String::new(), e.into()));
    }
    storage.probe_names(false);
    let old = storage.file_path(file);
    let fail = |path: &Path, e: io::Error| Err((path.to_string_lossy().into_owned(), Error::from(e)));
    let new_base = if crate::util::path::is_complete(name) {
        PathBuf::from(name)
    } else {
        PathBuf::from(storage.save_path()).join(name)
    };
    // D77: the torrent's paths with the new name, when one of them may collide with a suffixed
    // name ([`Clash`]): the new name may not be another file's (its plain one, the one it has
    // now) or a directory of the torrent, and keeps no suffix that would make it one
    let had_clash = storage.clash.read().unwrap_or_else(|e| e.into_inner()).is_some();
    let next = (had_clash || any_looks_suffixed(&new_base.to_string_lossy())).then(|| {
        let mut next_fs = storage.mapped().map_or_else(|| fs.clone(), |m| (*m).clone());
        next_fs.rename_file(file, name);
        let save_path = storage.save_path();
        let own = fold_key(&base_path_in(&next_fs, &save_path, file));
        (Clash::of(&next_fs, &save_path), own)
    });
    let next = next.as_ref().and_then(|(c, own)| c.as_ref().map(|c| (c, own.as_str())));
    let clashes = |p: &Path| next.is_some_and(|(c, own)| storage.clashes_in(c, file, own, p));
    let forbids = |s: Suffix| next.is_some_and(|(c, own)| c.forbids(own, s));
    if clashes(&new_base) {
        let e = io::Error::new(io::ErrorKind::AlreadyExists, "another file of the torrent has that name");
        return fail(&new_base, e);
    }
    // D77: the suffix goes on top of the new name, unless the new name is too long for one or
    // would be another path of the torrent with it
    let mut suffix = match storage.suffix(file) {
        s if s != Suffix::Plain && suffix_fits(&new_base) && !forbids(s) => s,
        _ => Suffix::Plain,
    };
    match old.try_exists() {
        Ok(true) => {
            let new = with_suffix(new_base.clone(), suffix);
            // An existing file is never overwritten (libtorrent 2.0.12), not even by itself. On
            // Windows a change of case only finds the file itself, and is let through
            // (libtorrent refuses it).
            let key = |p: &Path| p.to_string_lossy().replace('/', "\\").to_lowercase();
            let case_only = cfg!(windows) && old != new && key(&old) == key(&new);
            if !case_only && matches!(new.try_exists(), Ok(true)) {
                let e = io::Error::new(io::ErrorKind::AlreadyExists, "file already exists");
                return fail(&new, e);
            }
            // D77: nor is a suffixed file put beside a file of the new name: without the suffix
            // that rename is refused, and the file there would never get its name (Relocate to
            // a file that is already there refuses as it always did)
            let base = storage.base_path(file);
            let same_base = base == new_base || (cfg!(windows) && key(&base) == key(&new_base));
            if suffix != Suffix::Plain && !same_base && matches!(new_base.try_exists(), Ok(true)) {
                let e = io::Error::new(io::ErrorKind::AlreadyExists, "file already exists");
                return fail(&new_base, e);
            }
            if let Some(dir) = new.parent().filter(|d| !d.as_os_str().is_empty()) {
                if let Err(e) = std::fs::create_dir_all(dir) {
                    return fail(dir, e);
                }
            }
            // Open files cannot be renamed on Windows.
            ctx.pool.release(storage.id);
            match file_pool::rename_no_replace(&old, &new) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => return fail(&new, e),
                Err(e) => return fail(&old, e),
            }
        }
        // D77: not on disk, so the new name is where to look for it (Relocate to a file that is
        // there): under the plain name, then with `.!ut`, then `.!qB`, the first found is used
        // under the name it has; a check counts it, and the names follow its pieces
        // (never a name that is another path of the torrent: another file's, which it would
        // then share)
        Ok(false) => {
            let may = suffix_possible(&new_base);
            let there = [Suffix::Plain, Suffix::Ut, Suffix::Qb]
                .into_iter()
                .filter(|&s| s == Suffix::Plain || (may && !forbids(s)))
                .map(|s| (s, with_suffix(new_base.clone(), s)))
                .find(|(_, p)| !clashes(p) && found(p));
            if let Some((s, _)) = there {
                suffix = s;
            }
        }
        Err(e) => return fail(&old, e),
    }
    {
        let mut mapped = storage.mapped.write().unwrap_or_else(|e| e.into_inner());
        let files = mapped.get_or_insert_with(|| Arc::new(fs.clone()));
        Arc::make_mut(files).rename_file(file, name);
    }
    storage.set_suffix(file, suffix);
    if next.is_some() || had_clash {
        storage.rebuild_clash();
    }
    // a failed rename to the name it should have is reported anew under the new one
    if let Some(r) = storage.rename_reported.lock().unwrap_or_else(|e| e.into_inner()).get_mut(file) {
        *r = false;
    }
    Ok(())
}

/// D77: renames each file whose name differs from the one it should have: `.!ut` after an
/// `incomplete` file's name where it may carry one, the plain name otherwise. Runs with no
/// other job of `storage` in flight, after the files were found on disk. A file not on disk
/// only changes its name. A file is closed first and never renamed over an existing one; a
/// rename that fails (another program has the file open without sharing it, a permission)
/// leaves the file under the name it has, where every job keeps using it. Returns the failures
/// not reported before (once per file until one of its renames works, or a user's rename of
/// it) and whether a failure is worth trying again later ([`rename_may_work_later`]): a name
/// taken by another file is not; it is tried again only by the next names job sent for another
/// reason (a file completed, the option toggled, a start).
fn set_names(storage: &Storage, ctx: &Ctx, incomplete: &[bool]) -> (Vec<NameFailure>, bool) {
    storage.probe_names(false);
    let mut failed = Vec::new();
    let mut pending = false;
    let mut report = |f: usize, from: &Path, to: &Path, e: io::Error, pending: &mut bool| {
        let retry = rename_may_work_later(&e);
        *pending |= retry;
        let mut reported = storage.rename_reported.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(r) = reported.get_mut(f).filter(|r| !**r) {
            *r = true;
            failed.push(NameFailure {
                from: from.to_string_lossy().into_owned(),
                to: to.to_string_lossy().into_owned(),
                error: e.into(),
                retry,
            });
        }
    };
    for (f, &inc) in incomplete.iter().enumerate().take(storage.names.len()) {
        // (most files have the name they should: no path is built for them)
        let Some((now, want, base)) = storage.name_change(f, inc) else { continue };
        let (from, to) = (with_suffix(base.clone(), now), with_suffix(base, want));
        // D77: never to another path of the torrent (only a rename that gave two files one name
        // makes one the name a file should have): as a name taken by another file, not retried
        if storage.clashes(f, &to) {
            let e = io::Error::new(io::ErrorKind::AlreadyExists, "another file of the torrent has that name");
            report(f, &from, &to, e, &mut pending);
            continue;
        }
        match from.try_exists() {
            // nothing on disk under that name: the name alone changes
            Ok(false) => {
                storage.set_suffix(f, want);
                continue;
            }
            Ok(true) => {}
            Err(e) => {
                report(f, &from, &to, e, &mut pending);
                continue;
            }
        }
        // our handles share deletion, but a file is closed before it is renamed all the same
        ctx.pool.release_file(storage.id, f as u32);
        match file_pool::rename_no_replace(&from, &to) {
            Ok(()) => {
                storage.set_suffix(f, want);
                if let Some(r) = storage.rename_reported.lock().unwrap_or_else(|e| e.into_inner()).get_mut(f)
                {
                    *r = false;
                }
            }
            // gone between the look and the rename: as not on disk
            Err(e) if e.kind() == io::ErrorKind::NotFound && !found(&from) => storage.set_suffix(f, want),
            // Unix renames are a link plus an unlink: a crash between them leaves both names on
            // one file, and the old one only has to go
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists && same_file(&from, &to) => {
                match std::fs::remove_file(&from) {
                    Ok(()) => {
                        storage.set_suffix(f, want);
                        if let Some(r) =
                            storage.rename_reported.lock().unwrap_or_else(|e| e.into_inner()).get_mut(f)
                        {
                            *r = false;
                        }
                    }
                    Err(e) => report(f, &from, &to, e, &mut pending),
                }
            }
            Err(e) => report(f, &from, &to, e, &mut pending),
        }
    }
    (failed, pending)
}

/// D77: whether a rename to the name a file should have that failed with `e` may work later
/// without anything being done to the torrent: another program has the file open without
/// sharing it, or a permission or a read-only volume may come back. A name taken by another
/// file (`AlreadyExists`) is not tried again on a timer.
fn rename_may_work_later(e: &io::Error) -> bool {
    // ERROR_SHARING_VIOLATION, ERROR_LOCK_VIOLATION
    if cfg!(windows) && matches!(e.raw_os_error(), Some(32 | 33)) {
        return true;
    }
    matches!(
        e.kind(),
        io::ErrorKind::PermissionDenied | io::ErrorKind::ResourceBusy | io::ErrorKind::ReadOnlyFilesystem
    )
}

/// Whether `a` and `b` are one file under two names (hard links; never on Windows, whose
/// renames are one step).
#[cfg(unix)]
fn same_file(a: &Path, b: &Path) -> bool {
    use std::os::unix::fs::MetadataExt;
    match (std::fs::symlink_metadata(a), std::fs::symlink_metadata(b)) {
        (Ok(a), Ok(b)) => a.dev() == b.dev() && a.ino() == b.ino(),
        _ => false,
    }
}

#[cfg(not(unix))]
fn same_file(_: &Path, _: &Path) -> bool {
    false
}

/// libtorrent `posix_storage::move_storage` and `aux::move_storage`: moves every file (rename,
/// or copy and delete across volumes) and the part file to `dest`, rolls the renames back on
/// an error, then removes the directories left empty. Runs with no other job of `storage` in
/// flight. Returns the status, the save path now in use and the error, if any.
fn move_storage(
    storage: &Storage,
    ctx: &Ctx,
    dest: &str,
    flags: MoveFlags,
) -> (MoveStatus, String, Option<MoveError>) {
    // Open files cannot be renamed on Windows (libtorrent `mmap_storage` releases them too).
    ctx.pool.release(storage.id);
    // D77: the files move under the names they have, a suffix included
    storage.probe_names(false);
    let mapped = storage.mapped();
    let fs = mapped.as_deref().unwrap_or(storage.info.files());
    let at = |i: usize, root: &str| with_suffix(PathBuf::from(fs.file_path(i, root)), storage.suffix(i));
    let save_path = storage.save_path();
    let new_save_path = complete(dest);
    let fail = |status, file: String, operation, e: io::Error| {
        (status, save_path.clone(), Some(MoveError { file, operation, error: e.into() }))
    };
    let movable = |i: &usize| !fs.file_absolute_path(*i) && !fs.pad_file_at(*i);

    // D77: a file of the torrent at `root` under any name it may have there (plain, `.!ut`,
    // `.!qB`), the name it has here first
    // (not under a suffix that makes it another path of the torrent: the paths are the same
    // relative to any root, [`Clash`])
    let there = |i: usize, root: &str| {
        let base = PathBuf::from(fs.file_path(i, root));
        let mut order = vec![storage.suffix(i)];
        order.extend(
            [Suffix::Plain, Suffix::Ut, Suffix::Qb].into_iter().filter(|&s| storage.may_find(i, &base, s)),
        );
        order.into_iter().map(|s| with_suffix(base.clone(), s)).find(|p| found(p))
    };

    if flags == MoveFlags::FailIfExist && !is_not_found(std::fs::metadata(&new_save_path)) {
        // The directory exists: so may the files.
        for i in (0..fs.num_files()).filter(movable) {
            let path = there(i, &new_save_path).unwrap_or_else(|| at(i, &new_save_path));
            let path = path.to_string_lossy().into_owned();
            match std::fs::metadata(&path) {
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) => return fail(MoveStatus::FileExist, path, FileOperation::FileStat, e),
                Ok(_) => {
                    let e = io::Error::new(io::ErrorKind::AlreadyExists, "file already exists");
                    return fail(MoveStatus::FileExist, path, FileOperation::FileStat, e);
                }
            }
        }
    }

    match std::fs::metadata(&new_save_path) {
        Ok(_) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            if let Err(e) = std::fs::create_dir_all(&new_save_path) {
                return fail(MoveStatus::FatalDiskError, new_save_path.clone(), FileOperation::Mkdir, e);
            }
        }
        Err(e) => {
            return fail(MoveStatus::FatalDiskError, new_save_path.clone(), FileOperation::FileStat, e);
        }
    }

    let set_save_path = |p: &str| {
        *storage.save_path.write().unwrap_or_else(|e| e.into_inner()) = p.to_owned();
        *storage.drive.write().unwrap_or_else(|e| e.into_inner()) = None;
        // D77: the torrent's paths are elsewhere now (a file renamed to an absolute path
        // stays where it is)
        storage.rebuild_clash();
    };
    // D77: the files at the new path go by the names they have there
    let probe_again = || {
        storage.probed.store(false, Ordering::Relaxed);
        storage.probe_names(false);
    };
    match flags {
        MoveFlags::ResetSavePath => {
            set_save_path(&new_save_path);
            probe_again();
            return (MoveStatus::NeedFullCheck, new_save_path, None);
        }
        MoveFlags::ResetSavePathUnchecked => {
            set_save_path(&new_save_path);
            probe_again();
            return (MoveStatus::NoError, new_save_path, None);
        }
        _ => {}
    }

    let mut status = MoveStatus::NoError;
    // Files we renamed (rolled back on an error) and files we copied (their sources are deleted
    // once everything is across, the copies on an error).
    let mut renamed = vec![false; fs.num_files()];
    let mut copied = vec![false; fs.num_files()];
    let mut error = None;
    let (moves, id) = (&ctx.moves, storage.id);
    moves.start(id, &new_save_path, (0..fs.num_files()).filter(movable).map(|i| fs.file_size(i)).sum());
    for i in (0..fs.num_files()).filter(movable) {
        if moves.cancelled() {
            error = Some((i, FileOperation::FileCopy, cancelled()));
            break;
        }
        let old = at(i, &save_path);
        let new = at(i, &new_save_path);
        let done = moves.done(id) + fs.file_size(i);
        // (D77: the file there under any of its names is kept, and used under it after the
        // check; see the probe below)
        if flags == MoveFlags::DontReplace && there(i, &new_save_path).is_some() {
            status = MoveStatus::NeedFullCheck;
            moves.set_done(id, done);
            continue;
        }
        // D77: a file of the same name there is replaced as it always was, whatever name ours
        // carries now: our `x.!ut` takes the place of a plain `x` (which would keep it from
        // ever getting its name)
        if flags == MoveFlags::AlwaysReplaceFiles
            && storage.suffix(i) != Suffix::Plain
            && save_path != new_save_path
            && found(&old)
        {
            let plain = PathBuf::from(fs.file_path(i, &new_save_path));
            match std::fs::remove_file(&plain) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) => {
                    error = Some((i, FileOperation::FileRename, e));
                    break;
                }
            }
        }
        match move_file(&old, &new) {
            Ok(()) => renamed[i] = old != new,
            // A file that is not there (not downloaded yet) is not a problem.
            Err((_, e)) if e.kind() == io::ErrorKind::NotFound => {}
            Err((op, e))
                if matches!(e.kind(), io::ErrorKind::InvalidInput | io::ErrorKind::PermissionDenied) =>
            {
                error = Some((i, op, e));
                break;
            }
            // Renaming across volumes fails (EXDEV, ERROR_NOT_SAME_DEVICE): copy instead.
            Err(_) => match copy_file(&old, &new, &moves.cancel, &mut |n| moves.add(id, n)) {
                Ok(()) => copied[i] = true,
                Err(e) => {
                    error = Some((i, FileOperation::FileCopy, e));
                    break;
                }
            },
        }
        // the file's whole size, whatever the copy reported
        moves.set_done(id, done);
    }
    moves.end(id);
    let mut error = error.map(|(i, operation, e)| MoveError {
        file: at(i, &save_path).to_string_lossy().into_owned(),
        operation,
        error: e.into(),
    });

    if error.is_none() {
        let mut st = storage.lock_files();
        if st.part.is_none() && storage.part_file_path().exists() {
            storage.need_partfile(&mut st);
        }
        if let Some(part) = st.part.clone() {
            let old = storage.part_file_path();
            let new_dir = Path::new(&new_save_path);
            match part.move_partfile(new_dir) {
                Ok(()) => storage.part_files.rekey(&old, &new_dir.join(storage.part_file_name())),
                Err(e) => {
                    error = Some(MoveError {
                        file: old.to_string_lossy().into_owned(),
                        operation: FileOperation::PartfileMove,
                        error: e.into(),
                    })
                }
            }
        }
    }

    if let Some(error) = error {
        // Roll back. libtorrent moves back every file before the failing one; only the ones
        // renamed are moved here, so a destination file kept by `DontReplace` stays put. The
        // copies are deleted: their sources are all still there.
        for i in (0..fs.num_files()).rev() {
            let new = at(i, &new_save_path);
            if renamed[i] {
                let _ = move_file(&new, &at(i, &save_path));
            } else if copied[i] {
                let _ = std::fs::remove_file(&new);
            }
        }
        return (MoveStatus::FatalDiskError, save_path, Some(error));
    }
    set_save_path(&new_save_path);

    // Delete the sources of copied files, then the directories left empty (deepest first along
    // each file's path, stopping at the first that is not empty, never the save path).
    let mut subdirs = std::collections::BTreeSet::new();
    for i in (0..fs.num_files()).filter(movable) {
        let rel = PathBuf::from(fs.file_path(i, ""));
        if let Some(parent) = rel.parent().filter(|p| !p.as_os_str().is_empty()) {
            subdirs.insert(parent.to_path_buf());
        }
        if copied[i] {
            let _ = std::fs::remove_file(at(i, &save_path));
        }
    }
    let root = Path::new(&save_path);
    for sub in subdirs {
        let mut dir = root.join(sub);
        while dir != root && std::fs::remove_dir(&dir).is_ok() {
            match dir.parent() {
                Some(p) => dir = p.to_path_buf(),
                None => break,
            }
        }
    }
    // D77: a file that was not on disk here, or that `DontReplace` left there, is used under
    // the name it has there (as Relocate does); the moved ones are found under theirs
    probe_again();
    (status, new_save_path, None)
}

#[cfg(test)]
mod names_tests;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::bencode::Entry;
    use std::time::Duration;

    pub(crate) fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "lt-test-{name}-{}-{}",
            std::process::id(),
            crate::util::random::random_u64()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// A multi-file torrent with deterministic contents. Returns (metadata, file contents).
    pub(crate) fn make_torrent(files: &[(&str, usize)], piece_length: u32) -> (Vec<u8>, Vec<Vec<u8>>) {
        let mut contents = Vec::new();
        let mut all = Vec::new();
        for (i, &(_, size)) in files.iter().enumerate() {
            let data: Vec<u8> = (0..size).map(|j| ((j * 31 + i * 7) % 251) as u8).collect();
            all.extend_from_slice(&data);
            contents.push(data);
        }
        let mut pieces = Vec::new();
        for chunk in all.chunks(piece_length as usize) {
            pieces.extend_from_slice(&Sha1::digest(chunk).0);
        }
        let mut info = Entry::new_dict();
        let mut list = Entry::new_list();
        for &(path, size) in files {
            let mut f = Entry::new_dict();
            *f.key("length") = Entry::Int(size as i64);
            let mut p = Entry::new_list();
            for part in path.split('/') {
                p.list_mut().push(Entry::Bytes(part.as_bytes().to_vec()));
            }
            *f.key("path") = p;
            list.list_mut().push(f);
        }
        *info.key("files") = list;
        *info.key("name") = Entry::Bytes(b"test".to_vec());
        *info.key("piece length") = Entry::Int(piece_length as i64);
        *info.key("pieces") = Entry::Bytes(pieces);
        let mut t = Entry::new_dict();
        *t.key("info") = info;
        (t.bencode(), contents)
    }

    struct Rig {
        disk: DiskIo,
        rx: mpsc::Receiver<Completion>,
        storage: Arc<Storage>,
        dir: PathBuf,
        contents: Vec<Vec<u8>>,
    }

    fn rig(name: &str, files: &[(&str, usize)], piece_length: u32) -> Rig {
        rig_with(name, files, piece_length, Vec::new())
    }

    fn rig_with(name: &str, files: &[(&str, usize)], piece_length: u32, priorities: Vec<u8>) -> Rig {
        let (bytes, contents) = make_torrent(files, piece_length);
        let info = Arc::new(TorrentInfo::from_bytes(&bytes).unwrap());
        let dir = temp_dir(name);
        let (tx, rx) = mpsc::channel();
        let mut disk = DiskIo::new(3, 8, tx, Arc::new(|| {}));
        let storage = disk.new_storage(dir.to_string_lossy().into_owned(), info, priorities);
        Rig { disk, rx, storage, dir, contents }
    }

    fn next(rx: &mpsc::Receiver<Completion>) -> Completion {
        rx.recv_timeout(Duration::from_secs(10)).expect("disk completion")
    }

    fn torrent_bytes(contents: &[Vec<u8>]) -> Vec<u8> {
        contents.concat()
    }

    #[test]
    fn reads_for_peers_go_through_the_read_cache() {
        // 3 pieces of 32 KiB, piece 1 across the boundary of files "one" and "two"
        let r = rig("read_cache", &[("one", 40_000), ("two", 40_000)], 32 * 1024);
        let all = torrent_bytes(&r.contents);
        let block = |p: usize, b: usize| all[p * 32768 + b * 16384..][..16384].to_vec();
        for p in 0..2u32 {
            r.disk.async_write(&r.storage, p, 0, block(p as usize, 0));
            r.disk.async_write(&r.storage, p, BLOCK_SIZE, block(p as usize, 1));
        }
        let mut hashed = 0;
        while hashed < 2 {
            if let Completion::Hashed { verdict, .. } = next(&r.rx) {
                assert_eq!(verdict, HashVerdict::Passed);
                hashed += 1;
            }
        }
        let read = |piece: u32, offset: u32| {
            r.disk.async_read(&r.storage, piece, offset, BLOCK_SIZE, 1);
            loop {
                if let Completion::Read { result, .. } = next(&r.rx) {
                    break result.unwrap();
                }
            }
        };
        let counters = |blocks_read, read_ops, cache_hits| DiskCounters {
            blocks_read,
            read_ops,
            cache_hits,
            cache_bytes: r.disk.counters().cache_bytes,
        };

        // Lines of two blocks: the whole piece is read at the first request.
        r.disk.set_read_cache(true, 2048, 2, 300);
        assert_eq!(read(1, BLOCK_SIZE), block(1, 1));
        assert_eq!(read(1, 0), block(1, 0));
        assert_eq!(read(0, 0), block(0, 0));
        assert_eq!(r.disk.counters(), counters(3, 2, 1));
        let deadline = Instant::now() + Duration::from_secs(5);
        while r.disk.counters().cache_bytes != 2 * 32768 {
            assert!(Instant::now() < deadline, "the threads report their lines");
            std::thread::sleep(Duration::from_millis(1));
        }

        // A write to the piece clears its line: the next read sees the new data.
        let mut changed = block(1, 0);
        changed[100] ^= 0xff;
        r.disk.async_write(&r.storage, 1, 0, changed.clone());
        assert!(matches!(next(&r.rx), Completion::Write { error: None, .. }));
        assert_eq!(read(1, 0), changed);
        assert_eq!(r.disk.counters(), counters(4, 3, 1));
        assert_eq!(read(1, BLOCK_SIZE), block(1, 1));
        assert_eq!(r.disk.counters(), counters(5, 3, 2));

        // A fence clears the storage's lines.
        r.disk.async_fence(&r.storage, FenceAction::Release);
        loop {
            if let Completion::Fenced { .. } = next(&r.rx) {
                break;
            }
        }
        assert_eq!(read(0, BLOCK_SIZE), block(0, 1));
        assert_eq!(r.disk.counters(), counters(6, 4, 2));

        // Off: every request is a read of its own.
        r.disk.set_read_cache(false, 2048, 2, 300);
        assert_eq!(read(0, 0), block(0, 0));
        assert_eq!(read(0, 0), block(0, 0));
        assert_eq!(r.disk.counters(), counters(8, 6, 2));
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn write_hash_read_check_and_delete() {
        // 3 pieces of 32 KiB over three files, the last piece short.
        let r = rig("disk", &[("a/one", 20_000), ("two", 40_000), ("a/b/three", 15_000)], 32 * 1024);
        let all = torrent_bytes(&r.contents);
        let info = r.storage.info.clone();
        assert_eq!(info.num_pieces(), 3);

        // Write piece 1 out of order (block 1 first), piece 2 in order, piece 0 corrupted.
        let block = |p: u32, b: u32| {
            let start = p as usize * 32768 + b as usize * BLOCK_SIZE as usize;
            let end = (start + BLOCK_SIZE as usize)
                .min(all.len())
                .min(p as usize * 32768 + info.piece_size(p) as usize);
            all[start..end].to_vec()
        };
        r.disk.async_write(&r.storage, 1, BLOCK_SIZE, block(1, 1));
        r.disk.async_write(&r.storage, 1, 0, block(1, 0));
        r.disk.async_write(&r.storage, 2, 0, block(2, 0));
        let mut bad = block(0, 0);
        bad[5] ^= 1;
        r.disk.async_write(&r.storage, 0, 0, bad);
        r.disk.async_write(&r.storage, 0, BLOCK_SIZE, block(0, 1));

        let mut hashed = HashMap::new();
        let mut writes = 0;
        while hashed.len() < 3 {
            match next(&r.rx) {
                Completion::Write { error: None, .. } => writes += 1,
                Completion::Hashed { piece, verdict, .. } => {
                    let passed = verdict == HashVerdict::Passed;
                    hashed.insert(piece, passed);
                }
                c => panic!("unexpected {c:?}"),
            }
        }
        assert_eq!(hashed, HashMap::from([(0, false), (1, true), (2, true)]));
        assert!(writes >= 4);
        assert_eq!(r.disk.queued_write_bytes(), 0);

        // Fix piece 0, then read across the file boundary in piece 0 / 1.
        r.disk.async_write(&r.storage, 0, 0, block(0, 0));
        r.disk.async_write(&r.storage, 0, BLOCK_SIZE, block(0, 1));
        loop {
            if let Completion::Hashed { piece: 0, verdict, .. } = next(&r.rx) {
                let passed = verdict == HashVerdict::Passed;
                assert!(passed);
                break;
            }
        }
        r.disk.async_read(&r.storage, 0, BLOCK_SIZE, BLOCK_SIZE, 7);
        match next(&r.rx) {
            Completion::Read { cookie: 7, result: Ok(data), .. } => assert_eq!(data, block(0, 1)),
            c => panic!("unexpected {c:?}"),
        }
        assert_eq!(std::fs::read(r.dir.join("test/two")).unwrap(), r.contents[1]);

        // A full check finds all three pieces; resume data that matches is trusted.
        r.disk.async_check(&r.storage, CheckMode::Full);
        let have = loop {
            if let Completion::Checked { result, .. } = next(&r.rx) {
                break result.unwrap().have;
            }
        };
        assert!(have.all_set());
        r.disk.async_check(&r.storage, CheckMode::Resume(have.clone()));
        match next(&r.rx) {
            Completion::Checked { result: Ok(res), .. } => assert!(!res.resume_failed && res.have.all_set()),
            c => panic!("unexpected {c:?}"),
        }

        // Truncate a file: resume verification falls back to a full check and loses pieces.
        let f = std::fs::OpenOptions::new().write(true).open(r.dir.join("test/a/b/three")).unwrap();
        f.set_len(100).unwrap();
        drop(f);
        r.disk.async_fence(&r.storage, FenceAction::Release);
        assert!(matches!(next(&r.rx), Completion::Fenced { action: FenceAction::Release, error: None, .. }));
        let storage2 =
            Arc::new(Storage::new(99, r.storage.save_path(), info.clone(), Vec::new(), Arc::default()));
        r.disk.async_check(&storage2, CheckMode::Resume(have));
        match next(&r.rx) {
            Completion::Checked { result: Ok(res), .. } => {
                assert!(res.resume_failed);
                // Pieces 1 and 2 both reach into the truncated file.
                assert_eq!(res.have.iter().collect::<Vec<_>>(), [true, false, false]);
            }
            c => panic!("unexpected {c:?}"),
        }

        // A file that is gone takes only its own pieces: nothing is hashed, so piece 0, which
        // the truncated file's check above would not vouch for again, is not looked at either.
        std::fs::remove_file(r.dir.join("test/a/b/three")).unwrap();
        let mut resume = Bitfield::new(3, true);
        resume.set(2, false);
        r.disk.async_check(&storage2, CheckMode::Resume(resume.clone()));
        match next(&r.rx) {
            Completion::Checked { result: Ok(res), .. } => {
                assert!(!res.resume_failed);
                assert_eq!(res.missing.len(), 1);
                let lost: Vec<usize> = (0..3).filter(|&p| resume.get(p) && !res.have.get(p)).collect();
                assert!(!lost.is_empty() && res.have.get(0), "{lost:?}");
            }
            c => panic!("unexpected {c:?}"),
        }

        r.disk.async_fence(&storage2, FenceAction::Delete);
        assert!(matches!(next(&r.rx), Completion::Fenced { action: FenceAction::Delete, error: None, .. }));
        assert!(!r.dir.join("test").exists(), "files and empty directories are gone");
        assert!(r.dir.exists(), "the save path stays");
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn restored_blocks_are_read_back_for_the_hash() {
        // One piece of 4 blocks, one of 2.
        let r = rig("disk-restore", &[("f", 6 * BLOCK_SIZE as usize)], 4 * BLOCK_SIZE);
        let all = torrent_bytes(&r.contents);
        let block = |p: u32, b: u32| {
            let start = (p * 4 + b) as usize * BLOCK_SIZE as usize;
            all[start..start + BLOCK_SIZE as usize].to_vec()
        };
        let hashed = |rx: &mpsc::Receiver<Completion>| loop {
            if let Completion::Hashed { piece, verdict, .. } = next(rx) {
                let passed = verdict == HashVerdict::Passed;
                break (piece, passed);
            }
        };
        // A previous session wrote blocks 0 and 2 of piece 0, then stopped.
        let old = Arc::new(Storage::new(
            50,
            r.storage.save_path(),
            r.storage.info.clone(),
            Vec::new(),
            Arc::default(),
        ));
        r.disk.async_write(&old, 0, 0, block(0, 0));
        r.disk.async_write(&old, 0, 2 * BLOCK_SIZE, block(0, 2));
        r.disk.async_fence(&old, FenceAction::Release);
        while !matches!(next(&r.rx), Completion::Fenced { .. }) {}

        // This session restores them and downloads the other two, last block first.
        r.disk.async_restore_partial(&r.storage, 0, vec![true, false, true, false]);
        r.disk.async_write(&r.storage, 0, 3 * BLOCK_SIZE, block(0, 3));
        r.disk.async_write(&r.storage, 0, BLOCK_SIZE, block(0, 1));
        assert_eq!(hashed(&r.rx), (0, true));

        // A piece that was complete but not hashed at shutdown is hashed on restore.
        r.disk.async_write(&old, 1, 0, block(1, 0));
        r.disk.async_write(&old, 1, BLOCK_SIZE, block(1, 1));
        r.disk.async_fence(&old, FenceAction::Release);
        while !matches!(next(&r.rx), Completion::Fenced { .. }) {}
        r.disk.async_restore_partial(&r.storage, 1, vec![true, true]);
        assert_eq!(hashed(&r.rx), (1, true));

        // A restored block the file does not hold fails the piece instead of the torrent.
        r.disk.async_fence(&r.storage, FenceAction::Release);
        while !matches!(next(&r.rx), Completion::Fenced { .. }) {}
        std::fs::OpenOptions::new().write(true).open(r.dir.join("test/f")).unwrap().set_len(100).unwrap();
        let fresh = Arc::new(Storage::new(
            51,
            r.storage.save_path(),
            r.storage.info.clone(),
            Vec::new(),
            Arc::default(),
        ));
        r.disk.async_restore_partial(&fresh, 0, vec![true, true, false, false]);
        r.disk.async_write(&fresh, 0, 2 * BLOCK_SIZE, block(0, 2));
        r.disk.async_write(&fresh, 0, 3 * BLOCK_SIZE, block(0, 3));
        assert_eq!(hashed(&r.rx), (0, false));
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn check_without_files_is_instant() {
        let r = rig("disk-empty", &[("x", 100_000), ("empty", 0)], 16 * 1024);
        r.disk.async_check(&r.storage, CheckMode::Full);
        match next(&r.rx) {
            Completion::Checked { result: Ok(res), .. } => assert!(res.have.none_set()),
            c => panic!("unexpected {c:?}"),
        }
        assert_eq!(std::fs::metadata(r.dir.join("test/empty")).unwrap().len(), 0, "empty files are created");
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn a_drive_checks_one_torrent_at_a_time_and_a_cap_paces_it() {
        // 1 MB in each of two folders on one drive, at most 1 MB/s a check: two seconds, as the
        // drive keeps its checks on one thread however many may run at once.
        let r = rig("disk-checks-a", &[("data", 1_000_000)], 64 * 1024);
        let dir_b = temp_dir("disk-checks-b");
        let mut disk = r.disk;
        let a = r.storage.clone();
        let b = disk.new_storage(dir_b.to_string_lossy().into_owned(), a.info.clone(), Vec::new());
        for dir in [&r.dir, &dir_b] {
            std::fs::create_dir_all(dir.join("test")).unwrap();
            std::fs::write(dir.join("test/data"), &r.contents[0]).unwrap();
        }
        disk.set_smart(true, Some(1), 4, true, "", false);
        let started = Instant::now();
        disk.async_check(&a, CheckMode::Full);
        disk.async_check(&b, CheckMode::Full);
        assert!(checked(&r.rx).have.all_set());
        let first = started.elapsed();
        assert!(checked(&r.rx).have.all_set());
        let both = started.elapsed();
        assert!(first >= Duration::from_millis(900), "the cap holds: {first:?}");
        assert!(both >= Duration::from_millis(1800), "one after the other: {both:?}");
        let found = disk.drives_found();
        assert_eq!(found.iter().filter(|m| m.starts_with("drive ")).count(), 1, "{found:?}");
        drop(disk);
        let _ = std::fs::remove_dir_all(&r.dir);
        let _ = std::fs::remove_dir_all(&dir_b);
    }

    #[test]
    fn a_barrier_for_another_torrent_does_not_wait_for_a_check() {
        // T: 2 MB checked at 1 MB/s. A priority change for S, sent while T is being checked, is
        // answered before T's check ends: the checking thread takes it between two pieces.
        let r = rig("disk-check-barrier-t", &[("data", 2_000_000)], 64 * 1024);
        let dir_s = temp_dir("disk-check-barrier-s");
        let mut disk = r.disk;
        let t = r.storage.clone();
        let s = disk.new_storage(dir_s.to_string_lossy().into_owned(), t.info.clone(), Vec::new());
        std::fs::create_dir_all(r.dir.join("test")).unwrap();
        std::fs::write(r.dir.join("test/data"), &r.contents[0]).unwrap();
        disk.set_smart(true, Some(1), 4, true, "", false);
        disk.async_check(&t, CheckMode::Full);
        // The check is under way once it reports progress (16 pieces, about a second).
        match next(&r.rx) {
            Completion::CheckProgress { .. } => {}
            c => panic!("unexpected {c:?}"),
        }
        disk.async_set_file_priority(&s, vec![4]);
        let answered = loop {
            match next(&r.rx) {
                Completion::CheckProgress { .. } => {}
                Completion::FilePriority { storage, error: None } => break storage,
                c => panic!("the priority change waited for the check: {c:?}"),
            }
        };
        assert_eq!(answered, s.id);
        assert!(checked(&r.rx).have.all_set());
        drop(disk);
        let _ = std::fs::remove_dir_all(&r.dir);
        let _ = std::fs::remove_dir_all(&dir_s);
    }

    fn checked(rx: &mpsc::Receiver<Completion>) -> CheckResult {
        loop {
            if let Completion::Checked { result, .. } = next(rx) {
                break result.unwrap();
            }
        }
    }

    fn wanted(have: &Bitfield) -> Vec<usize> {
        (0..have.len()).filter(|&p| have.get(p)).collect()
    }

    /// Writes every piece in `pieces` (one block each) and waits for their hashes.
    fn write_pieces(r: &Rig, storage: &Arc<Storage>, pieces: &[u32]) {
        let all = torrent_bytes(&r.contents);
        let pl = r.storage.info.piece_length() as usize;
        for &p in pieces {
            let start = p as usize * pl;
            let end = (start + pl).min(all.len());
            r.disk.async_write(storage, p, 0, all[start..end].to_vec());
        }
        let mut left: std::collections::HashSet<u32> = pieces.iter().copied().collect();
        while !left.is_empty() {
            match next(&r.rx) {
                Completion::Hashed { piece, verdict, .. } => {
                    let passed = verdict == HashVerdict::Passed;
                    assert!(passed, "piece {piece} failed");
                    left.remove(&piece);
                }
                Completion::Write { error: None, .. } | Completion::FilePriority { error: None, .. } => {}
                c => panic!("unexpected {c:?}"),
            }
        }
    }

    // a: pieces 0..=2, b: 2..=5, c: 5..=7, with 16 KiB pieces.
    const SKIP_MIDDLE: &[(&str, usize)] = &[("a", 40_000), ("b", 50_000), ("c", 30_000)];

    #[test]
    fn a_skipped_file_lives_in_the_part_file_until_it_is_wanted() {
        let r = rig_with("disk-part", SKIP_MIDDLE, BLOCK_SIZE, vec![4, 0, 4]);
        let info = r.storage.info.clone();
        let save = r.storage.save_path();
        let part = r.dir.join(format!(".{}.parts", info.info_hash().to_hex()));
        let b = r.dir.join("test/b");
        assert_eq!(info.num_pieces(), 8);
        r.disk.async_check(&r.storage, CheckMode::Full);
        assert!(checked(&r.rx).have.none_set());
        assert!(!part.exists(), "the part file is created lazily");

        // The boundary pieces 2 and 5 pass; b is never created.
        write_pieces(&r, &r.storage, &[0, 1, 2, 5, 6, 7]);
        assert!(!b.exists(), "a skipped file is not created");
        assert_eq!(std::fs::read(r.dir.join("test/a")).unwrap(), r.contents[0]);
        assert_eq!(std::fs::read(r.dir.join("test/c")).unwrap(), r.contents[2]);
        r.disk.async_read(&r.storage, 2, 0, BLOCK_SIZE, 1);
        match next(&r.rx) {
            Completion::Read { result: Ok(data), .. } => {
                assert_eq!(data, torrent_bytes(&r.contents)[2 * 16384..3 * 16384])
            }
            c => panic!("unexpected {c:?}"),
        }

        // A recheck finds the boundary pieces through the part file; resume data is trusted.
        r.disk.async_check(&r.storage, CheckMode::Full);
        let have = checked(&r.rx).have;
        assert_eq!(wanted(&have), [0, 1, 2, 5, 6, 7]);
        r.disk.async_check(&r.storage, CheckMode::Resume(have.clone()));
        assert!(!checked(&r.rx).resume_failed);

        // Restart: the part file keeps its contents.
        r.disk.async_fence(&r.storage, FenceAction::Release);
        while !matches!(next(&r.rx), Completion::Fenced { .. }) {}
        assert!(part.exists());
        let Rig { mut disk, rx, storage, dir, contents } = r;
        drop(storage);
        let s2 = disk.new_storage(save.clone(), info.clone(), vec![4, 0, 4]);
        disk.async_check(&s2, CheckMode::Resume(have.clone()));
        let res = checked(&rx);
        assert!(!res.resume_failed);
        assert_eq!(wanted(&res.have), [0, 1, 2, 5, 6, 7]);
        disk.async_check(&s2, CheckMode::Full);
        assert_eq!(wanted(&checked(&rx).have), [0, 1, 2, 5, 6, 7]);

        // Unskip b, with its pieces queued right behind the change: they wait for the export.
        disk.async_set_file_priority(&s2, vec![4, 1, 4]);
        let mut r = Rig { disk, rx, storage: s2.clone(), dir, contents };
        write_pieces(&r, &s2, &[3, 4]);
        assert_eq!(std::fs::read(&b).unwrap(), r.contents[1], "exported and downloaded");
        assert!(!part.exists(), "no skipped file needs the part file any more");
        r.disk.async_check(&s2, CheckMode::Full);
        assert!(checked(&r.rx).have.all_set());

        // Skipping a file that exists keeps its data where it is.
        r.disk.async_set_file_priority(&s2, vec![4, 4, 0]);
        assert!(matches!(next(&r.rx), Completion::FilePriority { error: None, .. }));
        r.disk.async_check(&s2, CheckMode::Full);
        assert!(checked(&r.rx).have.all_set());
        assert!(!part.exists());

        r.disk.async_fence(&s2, FenceAction::Delete);
        assert!(matches!(next(&r.rx), Completion::Fenced { action: FenceAction::Delete, error: None, .. }));
        assert!(!r.dir.join("test").exists());
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn deleting_removes_the_part_file_and_resume_needs_its_pieces() {
        let files = &[("a", 40_000), ("b", 50_000), ("c", 30_000), ("empty", 0)];
        let mut r = rig_with("disk-part-delete", files, BLOCK_SIZE, vec![4, 0, 4, 0]);
        let info = r.storage.info.clone();
        let part = r.dir.join(format!(".{}.parts", info.info_hash().to_hex()));
        r.disk.async_check(&r.storage, CheckMode::Full);
        checked(&r.rx);
        assert!(!r.dir.join("test/empty").exists(), "a skipped empty file is not created");
        let storage = r.storage.clone();
        write_pieces(&r, &storage, &[1, 2]);
        assert!(part.exists());
        assert!(!r.dir.join("test/b").exists());

        // Resume data claiming piece 5 (partly in b) fails: the part file does not have it.
        let mut have = Bitfield::new(8, false);
        have.set(2, true);
        r.disk.async_check(&r.storage, CheckMode::Resume(have.clone()));
        assert!(!checked(&r.rx).resume_failed);
        have.set(5, true);
        r.disk.async_check(&r.storage, CheckMode::Resume(have));
        let res = checked(&r.rx);
        assert!(res.resume_failed);
        assert_eq!(wanted(&res.have), [1, 2]);

        // Remove deleting the part file only, while the old storage still holds it.
        let gone = r.disk.new_storage(r.storage.save_path(), info.clone(), Vec::new());
        r.disk.async_fence(&gone, FenceAction::DeletePartFile);
        assert!(matches!(
            next(&r.rx),
            Completion::Fenced { action: FenceAction::DeletePartFile, error: None, .. }
        ));
        assert!(!part.exists());
        assert!(r.dir.join("test/a").exists(), "the files stay");

        // Remove with data.
        write_pieces(&r, &storage, &[2]);
        assert!(part.exists());
        let gone = r.disk.new_storage(r.storage.save_path(), info, Vec::new());
        r.disk.async_fence(&gone, FenceAction::Delete);
        assert!(matches!(next(&r.rx), Completion::Fenced { action: FenceAction::Delete, error: None, .. }));
        assert!(!part.exists());
        assert!(!r.dir.join("test").exists());
        r.disk.async_fence(&r.storage, FenceAction::Release);
        while !matches!(next(&r.rx), Completion::Fenced { .. }) {}
        assert!(!part.exists(), "a released part file is not written back after the delete");
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    /// A check of a stopped torrent (shutdown) touches no more files: an abort is seen before
    /// each file's stat, not only between pieces.
    #[test]
    fn a_stopped_check_touches_no_more_files() {
        let r = rig("disk-check-abort", &[("empty", 0), ("data", 40_000)], 32 * 1024);
        std::fs::create_dir_all(r.dir.join("test")).unwrap();
        std::fs::write(r.dir.join("test/data"), &r.contents[1]).unwrap();
        let (tx, _rx) = mpsc::channel();
        let ctx = Ctx {
            pool: Arc::new(FilePool::new(4)),
            done: tx,
            wake: Arc::new(|| {}),
            queued_write_bytes: Arc::new(AtomicU64::new(0)),
            read_cache: Arc::default(),
            stats: Arc::new(DiskStats::new(1)),
            moves: Arc::default(),
            config: Arc::default(),
            drives: Arc::default(),
            queues: Arc::new(OnceLock::new()),
            index: 0,
        };
        let s = &r.storage;
        let all = Bitfield::new(s.info.num_pieces() as usize, true);
        let fresh = || Storage::new(78, s.save_path(), s.info.clone(), Vec::new(), Arc::default());

        let stopped = fresh();
        stopped.abort.store(true, Ordering::Relaxed);
        assert!(matches!(resume_files(&stopped, &all, &[]), ResumeFiles::Mismatch));
        let res = check_files(&stopped, &ctx, CheckMode::Resume(all.clone()), &mut || {}).unwrap();
        assert!(res.have.none_set());
        assert!(!r.dir.join("test/empty").exists(), "the empty file was not made");

        let res = check_files(&fresh(), &ctx, CheckMode::Resume(all.clone()), &mut || {}).unwrap();
        assert!(!res.resume_failed && res.have.all_set());
        assert!(r.dir.join("test/empty").exists());
        ctx.pool.release(78);
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn jobs_behind_a_priority_barrier_wait_for_it() {
        let r = rig("disk-barrier", &[("f", 2 * BLOCK_SIZE as usize)], BLOCK_SIZE);
        let all = torrent_bytes(&r.contents);
        let (tx, rx) = mpsc::channel();
        let ctx = Ctx {
            pool: Arc::new(FilePool::new(4)),
            done: tx,
            wake: Arc::new(|| {}),
            queued_write_bytes: Arc::new(AtomicU64::new(0)),
            read_cache: Arc::default(),
            stats: Arc::new(DiskStats::new(1)),
            moves: Arc::default(),
            config: Arc::default(),
            drives: Arc::default(),
            queues: Arc::new(OnceLock::new()),
            index: 0,
        };
        let mut w = Worker::default();
        let s = r.storage.clone();
        let other = Arc::new(Storage::new(77, s.save_path(), s.info.clone(), Vec::new(), Arc::default()));
        let barrier = |seq| {
            Arc::new(Barrier {
                remaining: AtomicUsize::new(2),
                seq,
                action: BarrierAction::FilePriority(vec![4]),
            })
        };
        // Another thread has yet to reach barrier 1: this one parks the storage's jobs.
        w.handle(Job::Barrier { storage: s.clone(), barrier: barrier(1) }, &ctx);
        w.handle(Job::Write { storage: s.clone(), piece: 0, offset: 0, data: all[..16384].to_vec() }, &ctx);
        w.handle(Job::Barrier { storage: s.clone(), barrier: barrier(2) }, &ctx);
        w.handle(Job::Write { storage: s.clone(), piece: 1, offset: 0, data: all[16384..].to_vec() }, &ctx);
        // Other storages are not held up.
        w.handle(Job::Read { storage: other, piece: 0, offset: 0, length: 1, cookie: 9 }, &ctx);
        assert!(matches!(rx.try_recv(), Ok(Completion::Read { cookie: 9, .. })));
        assert!(rx.try_recv().is_err());
        // An unblock for another barrier does nothing.
        w.handle(Job::Unblock { storage: s.id, seq: 2 }, &ctx);
        assert!(rx.try_recv().is_err());
        // Barrier 1 is done: piece 0 is written, then barrier 2 parks piece 1 again.
        w.handle(Job::Unblock { storage: s.id, seq: 1 }, &ctx);
        assert!(matches!(rx.try_recv(), Ok(Completion::Write { piece: 0, error: None, .. })));
        assert!(matches!(
            rx.try_recv(),
            Ok(Completion::Hashed { piece: 0, verdict: HashVerdict::Passed, .. })
        ));
        assert!(rx.try_recv().is_err());
        w.handle(Job::Unblock { storage: s.id, seq: 2 }, &ctx);
        assert!(matches!(rx.try_recv(), Ok(Completion::Write { piece: 1, error: None, .. })));
        assert!(matches!(
            rx.try_recv(),
            Ok(Completion::Hashed { piece: 1, verdict: HashVerdict::Passed, .. })
        ));
        assert!(w.parked.is_empty() && w.blocked.is_empty());
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn a_file_unskipped_while_stopped_is_exported_on_start() {
        let mut r = rig_with("disk-part-stopped", SKIP_MIDDLE, BLOCK_SIZE, vec![4, 0, 4]);
        let info = r.storage.info.clone();
        let part = r.dir.join(format!(".{}.parts", info.info_hash().to_hex()));
        r.disk.async_check(&r.storage, CheckMode::Full);
        checked(&r.rx);
        let storage = r.storage.clone();
        write_pieces(&r, &storage, &[2, 5]);
        r.disk.async_fence(&storage, FenceAction::Release);
        while !matches!(next(&r.rx), Completion::Fenced { .. }) {}
        drop(storage);
        assert!(part.exists());

        // Started again with b wanted: its boundary blocks move into b before the check.
        let s2 = r.disk.new_storage(r.storage.save_path(), info, vec![4, 4, 4]);
        r.disk.async_check(&s2, CheckMode::Full);
        assert_eq!(wanted(&checked(&r.rx).have), [2, 5]);
        assert!(!part.exists());
        let b = std::fs::read(r.dir.join("test/b")).unwrap();
        let all = torrent_bytes(&r.contents);
        assert_eq!(b[..49152 - 40_000], all[40_000..49152]);
        assert_eq!(b[81920 - 40_000..], all[81920..90_000]);
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    /// BUGHUNT 25: an export that fails part way leaves no short file and frees no block, so the
    /// next start exports the whole file instead of taking the short one and freeing the rest.
    #[test]
    fn an_interrupted_export_runs_again_on_the_next_start() {
        let mut r = rig_with("disk-part-interrupted", SKIP_MIDDLE, BLOCK_SIZE, vec![4, 0, 4]);
        let info = r.storage.info.clone();
        let part = r.dir.join(format!(".{}.parts", info.info_hash().to_hex()));
        r.disk.async_check(&r.storage, CheckMode::Full);
        checked(&r.rx);
        let storage = r.storage.clone();
        write_pieces(&r, &storage, &[2, 5]);
        r.disk.async_fence(&storage, FenceAction::Release);
        while !matches!(next(&r.rx), Completion::Fenced { .. }) {}
        drop(storage);

        // a folder where the export's temporary file goes makes it fail
        let blocker = r.dir.join("test/b.pa-export");
        std::fs::create_dir_all(&blocker).unwrap();
        let s2 = r.disk.new_storage(r.storage.save_path(), info.clone(), vec![4, 4, 4]);
        r.disk.async_check(&s2, CheckMode::Full);
        match next(&r.rx) {
            Completion::Checked { result, .. } => assert!(result.is_err(), "the export failed"),
            c => panic!("unexpected {c:?}"),
        }
        assert!(!r.dir.join("test/b").exists(), "no short file");
        r.disk.async_fence(&s2, FenceAction::Release);
        while !matches!(next(&r.rx), Completion::Fenced { .. }) {}
        drop(s2);
        assert!(part.exists(), "the blocks stay in the part file");

        std::fs::remove_dir(&blocker).unwrap();
        let s3 = r.disk.new_storage(r.storage.save_path(), info, vec![4, 4, 4]);
        r.disk.async_check(&s3, CheckMode::Full);
        assert_eq!(wanted(&checked(&r.rx).have), [2, 5]);
        assert!(!part.exists());
        assert!(!blocker.exists(), "no temporary file is left");
        let b = std::fs::read(r.dir.join("test/b")).unwrap();
        let all = torrent_bytes(&r.contents);
        assert_eq!(b[..49152 - 40_000], all[40_000..49152]);
        assert_eq!(b[81920 - 40_000..], all[81920..90_000]);
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    // test_storage.cpp `setup_torrent`: 0x8000, 0x8000, 0, 0, 0x8000 bytes in 16 KiB pieces.
    const MOVE_FILES: &[(&str, usize)] = &[
        ("test1.tmp", 0x8000),
        ("folder1/test2.tmp", 0x8000),
        ("folder2/test3.tmp", 0),
        ("_folder3/test4.tmp", 0),
        ("_folder3/subfolder/test5.tmp", 0x8000),
    ];

    /// A storage in `<dir>/save` with every piece written and the empty files created.
    fn move_rig(name: &str, priorities: Vec<u8>) -> Rig {
        let (bytes, contents) = make_torrent(MOVE_FILES, BLOCK_SIZE);
        let info = Arc::new(TorrentInfo::from_bytes(&bytes).unwrap());
        let dir = temp_dir(name);
        let (tx, rx) = mpsc::channel();
        let mut disk = DiskIo::new(3, 8, tx, Arc::new(|| {}));
        let save = dir.join("save");
        let storage = disk.new_storage(save.to_string_lossy().into_owned(), info, priorities);
        let r = Rig { disk, rx, storage, dir, contents };
        r.disk.async_check(&r.storage, CheckMode::Full);
        checked(&r.rx);
        let storage = r.storage.clone();
        let wanted: Vec<u32> = (0..6).filter(|&p| r.storage.info.piece_size(p) > 0).collect();
        write_pieces(&r, &storage, &wanted);
        r
    }

    fn moved(rx: &mpsc::Receiver<Completion>) -> (MoveStatus, String, Option<MoveError>) {
        loop {
            match next(rx) {
                Completion::Moved { status, save_path, error, .. } => break (status, save_path, error),
                Completion::Write { error: None, .. }
                | Completion::Hashed { verdict: HashVerdict::Passed, .. } => {}
                c => panic!("unexpected {c:?}"),
            }
        }
    }

    fn s(p: &Path) -> String {
        p.to_string_lossy().into_owned()
    }

    fn renamed(rx: &mpsc::Receiver<Completion>) -> (u32, String, Option<(String, Error)>) {
        loop {
            match next(rx) {
                Completion::Renamed { file, name, error, .. } => break (file, name, error),
                Completion::Write { error: None, .. }
                | Completion::Hashed { verdict: HashVerdict::Passed, .. } => {}
                c => panic!("unexpected {c:?}"),
            }
        }
    }

    #[test]
    fn rename_file_on_disk_then_read_and_move() {
        // test_torrent.cpp `test_rename_file` / test_storage.cpp's rename cases, on one storage.
        let mut r = move_rig("rename", Vec::new());
        let save = r.dir.join("save");
        // A relative name lands under the save path, directories made; the file keeps its data.
        r.disk.async_rename_file(&r.storage, 1, "test/renamed/two.tmp".into());
        assert_eq!(renamed(&r.rx), (1, "test/renamed/two.tmp".into(), None));
        assert!(!save.join("test/folder1/test2.tmp").exists());
        assert_eq!(std::fs::read(save.join("test/renamed/two.tmp")).unwrap(), r.contents[1]);
        // An absolute name, and a name outside the torrent's folder.
        let abs = r.dir.join("elsewhere/one.tmp");
        r.disk.async_rename_file(&r.storage, 0, s(&abs));
        assert!(renamed(&r.rx).2.is_none());
        assert_eq!(std::fs::read(&abs).unwrap(), r.contents[0]);
        r.disk.async_rename_file(&r.storage, 4, "five.tmp".into());
        assert!(renamed(&r.rx).2.is_none());
        assert_eq!(std::fs::read(save.join("five.tmp")).unwrap(), r.contents[4]);

        // Later jobs use the new names: file 1 starts at piece 2.
        r.disk.async_read(&r.storage, 2, 0, BLOCK_SIZE, 1);
        match next(&r.rx) {
            Completion::Read { result: Ok(data), .. } => assert_eq!(data, r.contents[1][..16384]),
            c => panic!("unexpected {c:?}"),
        }
        r.disk.async_check(&r.storage, CheckMode::Full);
        assert!(checked(&r.rx).have.all_set());

        // A move takes the renamed relative files along and leaves the absolute one.
        let new = r.dir.join("save2");
        r.disk.async_move_storage(&r.storage, s(&new), MoveFlags::AlwaysReplaceFiles);
        assert_eq!(moved(&r.rx).0, MoveStatus::NoError);
        assert_eq!(std::fs::read(new.join("test/renamed/two.tmp")).unwrap(), r.contents[1]);
        assert_eq!(std::fs::read(new.join("five.tmp")).unwrap(), r.contents[4]);
        assert_eq!(std::fs::read(&abs).unwrap(), r.contents[0]);
        assert!(!save.join("test/renamed").exists(), "emptied directories are removed");

        // A file that is not on disk only changes its name.
        std::fs::remove_file(new.join("five.tmp")).unwrap();
        r.disk.async_rename_file(&r.storage, 4, "test/later.tmp".into());
        assert!(renamed(&r.rx).2.is_none());
        assert!(!new.join("test/later.tmp").exists());

        // A directory that cannot be made fails, and the name stays.
        std::fs::write(new.join("blocker"), b"x").unwrap();
        r.disk.async_rename_file(&r.storage, 1, "blocker/two.tmp".into());
        let (_, _, error) = renamed(&r.rx);
        let (path, _) = error.expect("an error");
        assert!(path.ends_with("blocker"), "{path}");
        assert_eq!(std::fs::read(new.join("test/renamed/two.tmp")).unwrap(), r.contents[1]);
        r.disk.async_rename_file(&r.storage, 99, "x".into());
        assert!(renamed(&r.rx).2.is_some());

        // An existing file is not replaced (libtorrent 2.0.12), nor is the file itself.
        std::fs::write(new.join("taken.tmp"), b"mine").unwrap();
        for name in ["taken.tmp", "test/renamed/two.tmp"] {
            r.disk.async_rename_file(&r.storage, 1, name.into());
            let (path, error) = renamed(&r.rx).2.expect("file exists");
            assert!(Path::new(&path).ends_with(name), "{path}");
            assert!(matches!(error, Error::Io { kind: io::ErrorKind::AlreadyExists, .. }), "{error:?}");
        }
        assert_eq!(std::fs::read(new.join("taken.tmp")).unwrap(), b"mine");
        assert_eq!(std::fs::read(new.join("test/renamed/two.tmp")).unwrap(), r.contents[1]);
        // The race-free primitive refuses too.
        let (a, b) = (new.join("test/renamed/two.tmp"), new.join("taken.tmp"));
        let e = file_pool::rename_no_replace(&a, &b).unwrap_err();
        assert_eq!(e.kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(std::fs::read(&a).unwrap(), r.contents[1]);
        // On Windows a change of case is a rename of the file itself, and goes through.
        if cfg!(windows) {
            r.disk.async_rename_file(&r.storage, 1, "test/renamed/TWO.tmp".into());
            let e = renamed(&r.rx).2;
            assert!(e.is_none(), "{e:?}");
            let names: Vec<_> = std::fs::read_dir(new.join("test/renamed"))
                .unwrap()
                .map(|e| e.unwrap().file_name())
                .collect();
            assert!(names.iter().any(|n| n == "TWO.tmp"), "{names:?}");
        }
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn move_storage_to_another_directory() {
        // test_storage.cpp `dont_move_intermingled_files`, plus the hash state kept across.
        let mut r = move_rig("move", Vec::new());
        let (old, new) = (r.dir.join("save"), r.dir.join("save2"));
        std::fs::create_dir_all(old.join("test/_folder3/alien_folder1")).unwrap();
        std::fs::write(old.join("test/alien1.tmp"), b"x").unwrap();
        std::fs::write(old.join("test/folder1/alien2.tmp"), b"x").unwrap();

        r.disk.async_move_storage(&r.storage, s(&new), MoveFlags::AlwaysReplaceFiles);
        let (status, path, error) = moved(&r.rx);
        assert_eq!((status, path.as_str()), (MoveStatus::NoError, s(&new).as_str()), "{error:?}");
        assert_eq!(r.storage.save_path(), s(&new));
        for (i, (name, _)) in MOVE_FILES.iter().enumerate() {
            assert_eq!(std::fs::read(new.join("test").join(name)).unwrap(), r.contents[i], "{name}");
            assert!(!old.join("test").join(name).exists(), "{name}");
        }
        // Intermingled files and directories stay; directories left empty go.
        assert!(old.join("test/alien1.tmp").exists());
        assert!(old.join("test/folder1/alien2.tmp").exists());
        assert!(old.join("test/_folder3/alien_folder1").exists());
        assert!(!new.join("test/alien1.tmp").exists());
        assert!(!old.join("test/folder2").exists());
        assert!(!old.join("test/_folder3/subfolder").exists());
        assert!(old.exists(), "the old save path stays");

        // Reads, writes and checks use the new path.
        r.disk.async_read(&r.storage, 0, 0, 16, 3);
        assert!(matches!(next(&r.rx), Completion::Read { cookie: 3, result: Ok(_), .. }));
        r.disk.async_check(&r.storage, CheckMode::Full);
        assert!(checked(&r.rx).have.all_set());
        let storage = r.storage.clone();
        r.disk.async_fence(&storage, FenceAction::Delete);
        assert!(matches!(next(&r.rx), Completion::Fenced { error: None, .. }));
        assert!(!new.join("test").exists());
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    /// D47: "Quit now" at exit cancels a move; the torrent keeps its folder and its files.
    #[test]
    fn a_cancelled_move_keeps_the_old_folder() {
        let mut r = move_rig("move-cancel", Vec::new());
        let (old, new) = (r.dir.join("save"), r.dir.join("save2"));
        r.disk.cancel_moves();
        r.disk.async_move_storage(&r.storage, s(&new), MoveFlags::AlwaysReplaceFiles);
        let (status, path, error) = moved(&r.rx);
        assert_eq!((status, path.as_str()), (MoveStatus::FatalDiskError, s(&old).as_str()));
        let kind = error.map(|e| e.error);
        assert!(matches!(kind, Some(Error::Io { kind: io::ErrorKind::Interrupted, .. })), "{kind:?}");
        assert_eq!(r.storage.save_path(), s(&old));
        for (i, (name, _)) in MOVE_FILES.iter().enumerate() {
            assert_eq!(std::fs::read(old.join("test").join(name)).unwrap(), r.contents[i], "{name}");
            assert!(!new.join("test").join(name).exists(), "{name}");
        }
        assert!(r.disk.moves().is_empty(), "nothing left running");
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    /// D47's list of moves: queued ones wait with no progress; a running one counts its bytes;
    /// a second move of the same storage stays queued behind the first.
    #[test]
    fn moves_are_listed_queued_then_running() {
        let m = Moves::default();
        m.queue(1, "D:\\a".into(), 100);
        m.queue(2, "D:\\b".into(), 50);
        m.queue(1, "E:\\a".into(), 100);
        assert!(m.running().iter().all(|p| !p.started && p.done == 0));
        m.start(1, "D:\\a", 90);
        m.add(1, 30);
        m.add(1, 100);
        let list = m.running().clone();
        assert_eq!((list[0].started, list[0].done, list[0].total), (true, 90, 90), "capped at the total");
        assert!(!list[1].started && !list[2].started);
        m.end(1);
        let left: Vec<(u64, String, bool)> =
            m.running().iter().map(|p| (p.storage, p.to.clone(), p.started)).collect();
        assert_eq!(left, [(2, "D:\\b".to_owned(), false), (1, "E:\\a".to_owned(), false)]);
        // a move nobody queued (a test calling move_storage directly) is listed while it runs
        m.start(3, "F:\\", 10);
        m.end(3);
        assert_eq!(m.running().len(), 2);
    }

    #[test]
    fn copies_report_progress_and_stop_when_cancelled() {
        let dir = temp_dir("copy-file");
        let (src, dst) = (dir.join("a"), dir.join("b"));
        let data: Vec<u8> = (0..300_000u32).map(|i| (i % 251) as u8).collect();
        std::fs::write(&src, &data).unwrap();
        let cancel = AtomicBool::new(false);
        let mut copied = 0;
        copy_file(&src, &dst, &cancel, &mut |n| copied += n).unwrap();
        assert_eq!(copied, data.len() as u64);
        assert_eq!(std::fs::read(&dst).unwrap(), data);
        std::fs::remove_file(&dst).unwrap();

        cancel.store(true, Ordering::Relaxed);
        let e = copy_file(&src, &dst, &cancel, &mut |_| {}).unwrap_err();
        assert_eq!(e.kind(), io::ErrorKind::Interrupted);
        assert!(!dst.exists(), "no partial copy");
        assert!(src.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// BUGHUNT 24: a copy into a path past MAX_PATH (CopyFileExW without the extended form fails).
    #[test]
    fn copies_work_past_max_path() {
        let dir = temp_dir("copy-long");
        let deep = dir.join("d".repeat(120)).join("e".repeat(120));
        std::fs::create_dir_all(&deep).unwrap();
        let (src, dst) = (deep.join("a".repeat(100)), deep.join("b".repeat(100)));
        std::fs::write(&src, b"long").unwrap();
        let mut copied = 0;
        copy_file(&src, &dst, &AtomicBool::new(false), &mut |n| copied += n).unwrap();
        assert_eq!(std::fs::read(&dst).unwrap(), b"long");
        assert_eq!(copied, 4);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn move_storage_to_self() {
        // test_storage.cpp `test_move_storage_to_self`: a no-op.
        let mut r = move_rig("move-self", Vec::new());
        let save = r.storage.save_path();
        r.disk.async_move_storage(&r.storage, save.clone(), MoveFlags::AlwaysReplaceFiles);
        let (status, path, error) = moved(&r.rx);
        assert_eq!(status, MoveStatus::NoError, "{error:?}");
        assert_eq!(path, save);
        for (i, (name, _)) in MOVE_FILES.iter().enumerate() {
            assert_eq!(std::fs::read(Path::new(&save).join("test").join(name)).unwrap(), r.contents[i]);
        }
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn move_storage_into_self() {
        // test_storage.cpp `test_move_storage_into_self`: into a folder of the torrent itself.
        let mut r = move_rig("move-into", Vec::new());
        let inner = r.dir.join("save/test/folder1");
        r.disk.async_move_storage(&r.storage, s(&inner), MoveFlags::AlwaysReplaceFiles);
        let (status, _, error) = moved(&r.rx);
        assert_eq!(status, MoveStatus::NoError, "{error:?}");
        assert_eq!(std::fs::read(inner.join("test/folder1/test2.tmp")).unwrap(), r.contents[1]);
        assert!(inner.join("test/folder2/test3.tmp").exists());
        assert!(inner.join("test/_folder3/test4.tmp").exists());
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn move_storage_reset() {
        // test_storage.cpp `test_move_storage_reset`: the directory is made, nothing moves.
        for (flags, want) in [
            (MoveFlags::ResetSavePath, MoveStatus::NeedFullCheck),
            (MoveFlags::ResetSavePathUnchecked, MoveStatus::NoError),
        ] {
            let mut r = move_rig("move-reset", Vec::new());
            let new = r.dir.join("save2");
            r.disk.async_move_storage(&r.storage, s(&new), flags);
            let (status, path, error) = moved(&r.rx);
            assert_eq!((status, path), (want, s(&new)), "{error:?}");
            assert!(new.is_dir());
            assert!(!new.join("test").exists());
            assert!(r.dir.join("save/test/test1.tmp").exists());
            assert_eq!(r.storage.save_path(), s(&new));
            drop(r.disk);
            let _ = std::fs::remove_dir_all(&r.dir);
        }
    }

    #[test]
    fn fail_if_exist_and_dont_replace() {
        let mut r = move_rig("move-exist", Vec::new());
        let (old, new) = (r.dir.join("save"), r.dir.join("save2"));
        std::fs::create_dir_all(new.join("test/folder1")).unwrap();
        std::fs::write(new.join("test/folder1/test2.tmp"), b"theirs").unwrap();

        // FailIfExist moves nothing.
        r.disk.async_move_storage(&r.storage, s(&new), MoveFlags::FailIfExist);
        let (status, path, error) = moved(&r.rx);
        assert_eq!((status, path), (MoveStatus::FileExist, s(&old)));
        let error = error.unwrap();
        assert_eq!(error.operation, FileOperation::FileStat);
        assert!(error.file.ends_with("test2.tmp"), "{}", error.file);
        assert!(old.join("test/test1.tmp").exists() && !new.join("test/test1.tmp").exists());
        assert_eq!(r.storage.save_path(), s(&old));

        // DontReplace keeps their file and asks for a recheck; ours is left behind.
        r.disk.async_move_storage(&r.storage, s(&new), MoveFlags::DontReplace);
        let (status, path, error) = moved(&r.rx);
        assert_eq!((status, path), (MoveStatus::NeedFullCheck, s(&new)), "{error:?}");
        assert_eq!(std::fs::read(new.join("test/folder1/test2.tmp")).unwrap(), b"theirs");
        assert_eq!(std::fs::read(new.join("test/test1.tmp")).unwrap(), r.contents[0]);
        assert!(old.join("test/folder1/test2.tmp").exists());
        r.disk.async_check(&r.storage, CheckMode::Full);
        let have = checked(&r.rx).have;
        assert_eq!(wanted(&have), [0, 1, 4, 5], "test2.tmp's pieces fail");

        // AlwaysReplaceFiles (the default) overwrites.
        r.disk.async_move_storage(&r.storage, s(&old), MoveFlags::DontReplace);
        moved(&r.rx);
        r.disk.async_move_storage(&r.storage, s(&new), MoveFlags::AlwaysReplaceFiles);
        assert_eq!(moved(&r.rx).0, MoveStatus::NoError);
        assert_eq!(std::fs::read(new.join("test/folder1/test2.tmp")).unwrap(), r.contents[1]);
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn a_failed_move_is_rolled_back() {
        let mut r = move_rig("move-fail", Vec::new());
        let (old, new) = (r.dir.join("save"), r.dir.join("save2"));
        // test5.tmp cannot go where a non-empty directory of that name is.
        std::fs::create_dir_all(new.join("test/_folder3/subfolder/test5.tmp/x")).unwrap();
        r.disk.async_move_storage(&r.storage, s(&new), MoveFlags::AlwaysReplaceFiles);
        let (status, path, error) = moved(&r.rx);
        assert_eq!((status, path), (MoveStatus::FatalDiskError, s(&old)));
        let error = error.unwrap();
        assert!(error.file.ends_with("test5.tmp"), "{}", error.file);
        assert!(matches!(error.operation, FileOperation::FileRename | FileOperation::FileCopy));
        for (i, (name, _)) in MOVE_FILES.iter().enumerate() {
            assert_eq!(std::fs::read(old.join("test").join(name)).unwrap(), r.contents[i], "{name} back");
        }
        assert!(!new.join("test/test1.tmp").exists());
        assert_eq!(r.storage.save_path(), s(&old));
        r.disk.async_check(&r.storage, CheckMode::Full);
        assert!(checked(&r.rx).have.all_set());
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn the_part_file_moves_with_the_files() {
        // test2.tmp (pieces 2..=3) skipped: its pieces stay in the part file.
        let (bytes, contents) = make_torrent(MOVE_FILES, BLOCK_SIZE);
        let info = Arc::new(TorrentInfo::from_bytes(&bytes).unwrap());
        let dir = temp_dir("move-part");
        let (tx, rx) = mpsc::channel();
        let mut disk = DiskIo::new(3, 8, tx, Arc::new(|| {}));
        let (old, new) = (dir.join("save"), dir.join("save2"));
        let prio = vec![4, 0, 4, 4, 4];
        let storage = disk.new_storage(s(&old), info.clone(), prio.clone());
        let mut r = Rig { disk, rx, storage: storage.clone(), dir, contents };
        r.disk.async_check(&storage, CheckMode::Full);
        checked(&r.rx);
        write_pieces(&r, &storage, &[0, 1, 2, 3, 4, 5]);
        let part = format!(".{}.parts", info.info_hash().to_hex());
        assert!(old.join(&part).exists() && !old.join("test/folder1/test2.tmp").exists());

        // Writes queued behind the move go to the new place.
        r.disk.async_move_storage(&storage, s(&new), MoveFlags::AlwaysReplaceFiles);
        let all = torrent_bytes(&r.contents);
        r.disk.async_write(&storage, 0, 0, all[..BLOCK_SIZE as usize].to_vec());
        let (status, _, error) = moved(&r.rx);
        assert_eq!(status, MoveStatus::NoError, "{error:?}");
        assert!(!old.join(&part).exists());
        assert!(new.join(&part).exists());
        assert!(!old.join("test").exists(), "every directory left empty is gone");
        assert!(r.storage.part_files.get(&new.join(&part)).is_some(), "the registry follows");
        assert!(r.storage.part_files.get(&old.join(&part)).is_none());
        loop {
            if let Completion::Hashed { piece: 0, verdict, .. } = next(&r.rx) {
                let passed = verdict == HashVerdict::Passed;
                assert!(passed);
                break;
            }
        }
        r.disk.async_check(&storage, CheckMode::Full);
        assert!(checked(&r.rx).have.all_set(), "the boundary pieces are read from the part file");

        // Unskipping exports into the file at the new place.
        r.disk.async_set_file_priority(&storage, vec![4; 5]);
        assert!(matches!(next(&r.rx), Completion::FilePriority { error: None, .. }));
        assert_eq!(std::fs::read(new.join("test/folder1/test2.tmp")).unwrap()[..16], r.contents[1][..16]);
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }

    #[test]
    fn a_stopped_torrents_part_file_moves_too() {
        // No part file object is open (a fresh storage): the file on disk is moved anyway.
        let mut r = move_rig("move-part-stopped", vec![4, 0, 4, 4, 4]);
        let info = r.storage.info.clone();
        let part = format!(".{}.parts", info.info_hash().to_hex());
        let (old, new) = (r.dir.join("save"), r.dir.join("save2"));
        let storage = r.storage.clone();
        r.disk.async_fence(&storage, FenceAction::Release);
        while !matches!(next(&r.rx), Completion::Fenced { .. }) {}
        drop(storage);
        r.storage = r.disk.new_storage(s(&old), info, vec![4, 0, 4, 4, 4]);
        assert!(old.join(&part).exists());
        r.disk.async_move_storage(&r.storage, s(&new), MoveFlags::AlwaysReplaceFiles);
        assert_eq!(moved(&r.rx).0, MoveStatus::NoError);
        assert!(new.join(&part).exists() && !old.join(&part).exists());
        r.disk.async_check(&r.storage, CheckMode::Full);
        assert!(checked(&r.rx).have.all_set());
        drop(r.disk);
        let _ = std::fs::remove_dir_all(&r.dir);
    }
}
