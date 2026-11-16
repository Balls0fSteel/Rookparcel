//! D77: incomplete files under `<name>.!ut`, on the disk threads alone: the names the storage
//! gives its files, finding files left under another name (ours, µTorrent's `.!ut`,
//! qBittorrent's `.!qB`), renames that fail, and the jobs that must keep working with them
//! (reads, writes, checks, user renames, moves, deletes, the part file).

use super::tests::{make_torrent, temp_dir};
use super::*;
use crate::bencode::Entry;
use crate::engine::torrent::{HaveRef, incomplete_files};

/// 16 KiB pieces: a is pieces 0..=2, b 2..=5, c 5..=7 (pieces 2 and 5 are shared).
const FILES: &[(&str, usize)] = &[("a", 40_000), ("b", 50_000), ("c", 30_000)];

struct Rig {
    disk: DiskIo,
    rx: mpsc::Receiver<Completion>,
    storage: Arc<Storage>,
    info: Arc<TorrentInfo>,
    dir: PathBuf,
    all: Vec<u8>,
    contents: Vec<Vec<u8>>,
}

impl Drop for Rig {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

fn rig_from(name: &str, bytes: Vec<u8>, contents: Vec<Vec<u8>>, priorities: Vec<u8>) -> Rig {
    let info = Arc::new(TorrentInfo::from_bytes(&bytes).unwrap());
    let dir = temp_dir(name);
    let (tx, rx) = mpsc::channel();
    let mut disk = DiskIo::new(3, 8, tx, Arc::new(|| {}));
    let storage = disk.new_storage(dir.to_string_lossy().into_owned(), info.clone(), priorities);
    let all = info_bytes(&info, &contents);
    Rig { disk, rx, storage, info, dir, all, contents }
}

fn rig(name: &str, files: &[(&str, usize)], priorities: Vec<u8>) -> Rig {
    let (bytes, contents) = make_torrent(files, BLOCK_SIZE);
    rig_from(name, bytes, contents, priorities)
}

/// The torrent's bytes in piece order, pad files as zeros.
fn info_bytes(info: &TorrentInfo, contents: &[Vec<u8>]) -> Vec<u8> {
    let fs = info.files();
    let mut all = Vec::new();
    let mut next = contents.iter();
    for f in 0..fs.num_files() {
        if fs.pad_file_at(f) {
            all.extend(std::iter::repeat_n(0u8, fs.file_size(f) as usize));
        } else {
            all.extend_from_slice(next.next().expect("contents"));
        }
    }
    all
}

/// The names of the files under `root` (and its subfolders), `/`-separated and sorted.
fn files_in(root: &Path) -> Vec<String> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) {
        for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(root, &p, out);
            } else {
                out.push(p.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/"));
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort();
    out
}

fn next(rx: &mpsc::Receiver<Completion>) -> Completion {
    rx.recv_timeout(std::time::Duration::from_secs(10)).expect("disk completion")
}

impl Rig {
    fn path(&self, rel: &str) -> PathBuf {
        self.dir.join("test").join(rel)
    }

    /// The names of the files in `test/` (and its subfolders), sorted.
    fn listing(&self) -> Vec<String> {
        files_in(&self.dir.join("test"))
    }

    fn check(&self, mode: CheckMode) -> CheckResult {
        self.disk.async_check(&self.storage, mode);
        loop {
            if let Completion::Checked { result, .. } = next(&self.rx) {
                break result.unwrap();
            }
        }
    }

    /// Writes `pieces` (whole) and waits for each to pass.
    fn write(&self, pieces: &[u32]) {
        let pl = self.info.piece_length() as usize;
        for &p in pieces {
            let start = p as usize * pl;
            let end = (start + pl).min(self.all.len());
            for (i, block) in self.all[start..end].chunks(BLOCK_SIZE as usize).enumerate() {
                self.disk.async_write(&self.storage, p, i as u32 * BLOCK_SIZE, block.to_vec());
            }
        }
        let mut left: std::collections::HashSet<u32> = pieces.iter().copied().collect();
        while !left.is_empty() {
            match next(&self.rx) {
                Completion::Hashed { piece, verdict, .. } => {
                    assert_eq!(verdict, HashVerdict::Passed, "piece {piece}");
                    left.remove(&piece);
                }
                Completion::Write { error: None, .. } | Completion::FilePriority { error: None, .. } => {}
                c => panic!("unexpected {c:?}"),
            }
        }
    }

    fn read(&self, piece: u32) -> Vec<u8> {
        let size = self.info.piece_size(piece);
        self.disk.async_read(&self.storage, piece, 0, size.min(BLOCK_SIZE), 7);
        loop {
            match next(&self.rx) {
                Completion::Read { result, .. } => break result.unwrap(),
                Completion::Write { error: None, .. } => {}
                c => panic!("unexpected {c:?}"),
            }
        }
    }

    fn piece_data(&self, piece: u32) -> Vec<u8> {
        let pl = self.info.piece_length() as usize;
        let size = self.info.piece_size(piece).min(BLOCK_SIZE) as usize;
        self.all[piece as usize * pl..][..size].to_vec()
    }

    /// Sends the names for `incomplete` and waits for the answer: the failures reported and
    /// whether any is pending.
    fn names(&mut self, incomplete: &[bool]) -> (Vec<NameFailure>, bool) {
        self.disk.async_set_names(&self.storage, incomplete.to_vec());
        loop {
            match next(&self.rx) {
                Completion::Names { failed, pending, .. } => break (failed, pending),
                Completion::Write { error: None, .. } | Completion::Hashed { .. } => {}
                c => panic!("unexpected {c:?}"),
            }
        }
    }

    /// The names the engine would send for the pieces in `have` with the option on.
    fn incomplete(&self, have: &Bitfield) -> Vec<bool> {
        incomplete_files(&self.info, Some(HaveRef::Bits(have)))
    }

    fn have(&self, pieces: &[u32]) -> Bitfield {
        let mut b = Bitfield::new(self.info.num_pieces() as usize, false);
        for &p in pieces {
            b.set(p as usize, true);
        }
        b
    }

    /// A new storage for the same files, as a restart makes, with `incomplete` as its names.
    fn restart(&mut self, incomplete: &[bool]) {
        let prio = self.storage.lock_files().priority.clone();
        self.restart_with(incomplete, prio);
    }

    /// [`Rig::restart`] with these file priorities (changed while stopped).
    fn restart_with(&mut self, incomplete: &[bool], prio: Vec<u8>) {
        self.disk.async_fence(&self.storage, FenceAction::Release);
        while !matches!(next(&self.rx), Completion::Fenced { .. }) {}
        self.storage = self.disk.new_storage(self.storage.save_path(), self.info.clone(), prio);
        self.storage.init_names(incomplete);
    }
}

#[test]
fn incomplete_files_carry_ut_until_every_piece_passed() {
    let mut r = rig("names-download", FILES, Vec::new());
    r.storage.init_names(&[true; 3]);
    assert!(r.check(CheckMode::Full).have.none_set());
    assert!(r.listing().is_empty(), "nothing is created by the check");

    // a's first pieces: a.!ut
    r.write(&[0, 1]);
    assert_eq!(r.listing(), ["a.!ut"]);
    assert_eq!(r.read(0), r.piece_data(0));
    // piece 2 completes a and starts b; the names follow the pieces
    r.write(&[2]);
    let have = r.have(&[0, 1, 2]);
    assert_eq!(r.incomplete(&have), [false, true, true]);
    assert_eq!(r.listing(), ["a.!ut", "b.!ut"]);
    let (failed, pending) = r.names(&r.incomplete(&have));
    assert!(failed.is_empty() && !pending);
    assert_eq!(r.listing(), ["a", "b.!ut"]);
    assert_eq!(std::fs::read(r.path("a")).unwrap(), r.contents[0]);
    // reads and writes after the rename use the new names (the handles were closed for it)
    assert_eq!(r.read(0), r.piece_data(0));
    assert_eq!(r.read(2), r.piece_data(2));
    r.write(&[3, 4, 5, 6, 7]);
    assert_eq!(r.listing(), ["a", "b.!ut", "c.!ut"]);
    assert_eq!(r.names(&[false; 3]), (Vec::new(), false));
    assert_eq!(r.listing(), ["a", "b", "c"]);
    for (i, n) in ["a", "b", "c"].iter().enumerate() {
        assert_eq!(std::fs::read(r.path(n)).unwrap(), r.contents[i], "{n}");
    }
    assert!(r.check(CheckMode::Full).have.all_set());

    // incomplete again (a failed piece, a recheck that finds a file short): .!ut comes back
    let mut have = Bitfield::new(8, true);
    have.set(6, false);
    assert!(r.names(&r.incomplete(&have)).0.is_empty());
    assert_eq!(r.listing(), ["a", "b", "c.!ut"]);
    assert_eq!(r.read(6), r.piece_data(6), "seeding works from the suffixed file");
    // the option off: plain again
    assert!(r.names(&[false; 3]).0.is_empty());
    assert_eq!(r.listing(), ["a", "b", "c"]);
}

/// PartialEq for the asserts above.
impl PartialEq for NameFailure {
    fn eq(&self, o: &NameFailure) -> bool {
        (&self.from, &self.to) == (&o.from, &o.to)
    }
}

#[test]
fn files_already_complete_on_disk_are_not_renamed_back_and_forth() {
    let mut r = rig("names-complete", FILES, Vec::new());
    for (i, n) in ["a", "b", "c"].iter().enumerate() {
        std::fs::create_dir_all(r.dir.join("test")).unwrap();
        std::fs::write(r.path(n), &r.contents[i]).unwrap();
    }
    // a new torrent: nothing known, so every file should carry .!ut until the check says more
    r.storage.init_names(&[true; 3]);
    let res = r.check(CheckMode::Full);
    assert!(res.have.all_set());
    assert_eq!(r.listing(), ["a", "b", "c"], "found under the plain names, not renamed");
    assert_eq!(res.names.len(), 3, "{:?}", res.names);
    // the check's answer: every file complete, no names job is needed
    assert!(!r.storage.names_differ(&r.incomplete(&res.have)));
    assert!(r.storage.names_differ(&[true, false, false]));

    // seed mode / resume data with every piece: the plain names are expected and found
    let all = res.have.clone();
    r.restart(&r.incomplete(&all));
    let res = r.check(CheckMode::Resume(all));
    assert!(res.have.all_set() && !res.resume_failed && res.names.is_empty());
    assert_eq!(r.listing(), ["a", "b", "c"]);
}

#[test]
fn files_of_ours_and_of_other_clients_are_picked_up_under_their_names() {
    let mut r = rig("names-adopt", FILES, Vec::new());
    std::fs::create_dir_all(r.dir.join("test")).unwrap();
    // qBittorrent's whole a, µTorrent's partial b (its first 20,000 bytes), our plain c
    std::fs::write(r.path("a.!qB"), &r.contents[0]).unwrap();
    std::fs::write(r.path("b.!ut"), &r.contents[1][..20_000]).unwrap();
    std::fs::write(r.path("c"), &r.contents[2]).unwrap();
    // the option off: the plain names are expected
    r.storage.init_names(&[false; 3]);
    let res = r.check(CheckMode::Full);
    assert_eq!(wanted(&res.have), [0, 1, 2, 6, 7], "only b's missing part is to download");
    assert!(res.names.iter().any(|l| l.contains("a.!qB")), "{:?}", res.names);
    assert_eq!(r.storage.suffix(0), Suffix::Qb);
    assert_eq!(r.storage.suffix(1), Suffix::Ut);
    assert_eq!(r.read(0), r.piece_data(0), "read under the name found");
    // the names the pieces call for with the option off: all plain
    assert!(r.names(&[false; 3]).0.is_empty());
    assert_eq!(r.listing(), ["a", "b", "c"]);
    r.write(&[3, 4, 5]);
    assert_eq!(std::fs::read(r.path("b")).unwrap(), r.contents[1], "b completed in place");

    // with the option on, a partial qBittorrent file becomes ours
    let mut r = rig("names-adopt-on", FILES, Vec::new());
    std::fs::create_dir_all(r.dir.join("test")).unwrap();
    std::fs::write(r.path("b.!qB"), &r.contents[1][..45_000]).unwrap();
    r.storage.init_names(&[true; 3]);
    let res = r.check(CheckMode::Full);
    assert_eq!(wanted(&res.have), [3, 4]);
    let inc = r.incomplete(&res.have);
    assert!(r.names(&inc).0.is_empty());
    assert_eq!(r.listing(), ["b.!ut"]);
    r.write(&[0, 1, 2, 5, 6, 7]);
    assert!(r.names(&[false; 3]).0.is_empty());
    assert_eq!(r.listing(), ["a", "b", "c"]);
    assert!(r.check(CheckMode::Full).have.all_set());
}

fn wanted(have: &Bitfield) -> Vec<usize> {
    (0..have.len()).filter(|&p| have.get(p)).collect()
}

#[test]
fn a_restart_heals_names_that_disagree_with_the_pieces() {
    // As a crash leaves it: a was renamed to its plain name, but the resume data saved before
    // says it is incomplete; b was given .!ut, the resume data says complete.
    let mut r = rig("names-crash", FILES, Vec::new());
    std::fs::create_dir_all(r.dir.join("test")).unwrap();
    std::fs::write(r.path("a"), &r.contents[0]).unwrap();
    std::fs::write(r.path("b.!ut"), &r.contents[1]).unwrap();
    std::fs::write(r.path("c.!ut"), &r.contents[2][..10_000]).unwrap();
    let resume = r.have(&[0, 1, 3, 4]);
    r.storage.init_names(&r.incomplete(&resume));
    assert_eq!(r.storage.suffix(0), Suffix::Ut);
    assert_eq!(r.storage.suffix(1), Suffix::Ut);
    let res = r.check(CheckMode::Resume(resume.clone()));
    assert!(!res.resume_failed, "every file is found, under the name it has");
    assert_eq!(res.have, resume);
    assert_eq!(r.storage.suffix(0), Suffix::Plain);
    // the engine's names for that set
    assert!(r.names(&r.incomplete(&resume)).0.is_empty());
    assert_eq!(r.listing(), ["a.!ut", "b.!ut", "c.!ut"]);
    r.write(&[2, 5, 6, 7]);
    assert!(r.names(&[false; 3]).0.is_empty());
    assert_eq!(r.listing(), ["a", "b", "c"]);
    assert!(r.check(CheckMode::Full).have.all_set());
}

#[test]
fn an_existing_file_is_never_overwritten_and_a_failed_rename_is_reported_once() {
    let mut r = rig("names-taken", FILES, Vec::new());
    std::fs::create_dir_all(r.dir.join("test")).unwrap();
    std::fs::write(r.path("a"), &r.contents[0]).unwrap();
    std::fs::write(r.path("a.!ut"), b"someone else's").unwrap();
    r.storage.init_names(&[false; 3]);
    let res = r.check(CheckMode::Full);
    assert!(res.names.iter().any(|l| l.contains("left alone")), "{:?}", res.names);
    assert_eq!(wanted(&res.have), [0, 1]);
    // a should get .!ut (piece 2 is missing), but the name is taken: it stays a, in use, and
    // a name taken by another file is not tried again on a timer (not pending)
    let inc = r.incomplete(&res.have);
    let (failed, pending) = r.names(&inc);
    assert!(!pending);
    assert_eq!(failed.len(), 1, "{failed:?}");
    assert!(failed[0].to.ends_with("a.!ut") && !failed[0].retry, "{failed:?}");
    assert_eq!(std::fs::read(r.path("a.!ut")).unwrap(), b"someone else's");
    assert_eq!(r.read(0), r.piece_data(0));
    // tried again with the next names job: still refused, not reported again
    let (failed, pending) = r.names(&inc);
    assert!(failed.is_empty() && !pending);
    // the other file gone: the rename works, and a later failure is reported anew
    std::fs::remove_file(r.path("a.!ut")).unwrap();
    assert_eq!(r.names(&inc), (Vec::new(), false));
    assert_eq!(r.storage.suffix(0), Suffix::Ut);
    std::fs::write(r.path("a"), b"in the way").unwrap();
    r.write(&[2]);
    let (failed, pending) = r.names(&[false; 3]);
    assert!(!pending && failed.len() == 1, "{failed:?}");
    assert_eq!(std::fs::read(r.path("a")).unwrap(), b"in the way");
    assert_eq!(std::fs::read(r.path("a.!ut")).unwrap(), r.contents[0]);
}

/// Another program holds the file open without sharing deletion (a media player, a virus
/// scanner): the rename fails, the file keeps being used under its name, and the next try after
/// the program let go works.
#[cfg(windows)]
#[test]
fn a_file_held_by_another_program_keeps_its_name_until_it_is_let_go() {
    use std::os::windows::fs::OpenOptionsExt;
    let mut r = rig("names-held", FILES, Vec::new());
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    r.write(&[0, 1, 2]);
    // FILE_SHARE_READ | FILE_SHARE_WRITE, not FILE_SHARE_DELETE
    let held = std::fs::OpenOptions::new().read(true).share_mode(1 | 2).open(r.path("a.!ut")).unwrap();
    let inc = r.incomplete(&r.have(&[0, 1, 2]));
    let (failed, pending) = r.names(&inc);
    assert!(pending && failed.len() == 1 && failed[0].retry, "a sharing violation passes: {failed:?}");
    assert_eq!(r.listing(), ["a.!ut", "b.!ut"]);
    assert_eq!(r.read(1), r.piece_data(1), "seeding from the suffixed file");
    r.write(&[3]);
    drop(held);
    assert_eq!(r.names(&inc), (Vec::new(), false));
    assert_eq!(r.listing(), ["a", "b.!ut"]);
    assert_eq!(r.read(1), r.piece_data(1));
}

#[test]
fn empty_and_pad_files_stay_plain_and_long_names_and_paths_work() {
    // an empty file, a pad file, a long name, a path past MAX_PATH
    let long = format!("{}.bin", "n".repeat(250));
    let deep = format!("{}/{}/f.bin", "d".repeat(120), "e".repeat(120));
    let sizes = [20_000usize, 0, 12_768, 30_000, 25_000];
    let names: [&str; 5] = ["first.bin", "empty", ".pad/12768", &long, &deep];
    let mut info = Entry::new_dict();
    let mut list = Entry::new_list();
    let mut contents = Vec::new();
    let mut all = Vec::new();
    for (i, (&name, &size)) in names.iter().zip(&sizes).enumerate() {
        let mut f = Entry::new_dict();
        *f.key("length") = Entry::Int(size as i64);
        *f.key("path") = Entry::List(name.split('/').map(|p| Entry::Bytes(p.as_bytes().to_vec())).collect());
        if name.starts_with(".pad") {
            *f.key("attr") = Entry::Bytes(b"p".to_vec());
            all.extend(std::iter::repeat_n(0u8, size));
        } else {
            let data: Vec<u8> = (0..size).map(|j| ((j * 13 + i) % 251) as u8).collect();
            all.extend_from_slice(&data);
            contents.push(data);
        }
        list.list_mut().push(f);
    }
    let pl = 32 * 1024;
    let pieces: Vec<u8> = all.chunks(pl).flat_map(|c| Sha1::digest(c).0).collect();
    *info.key("files") = list;
    *info.key("name") = Entry::Bytes(b"test".to_vec());
    *info.key("piece length") = Entry::Int(pl as i64);
    *info.key("pieces") = Entry::Bytes(pieces);
    let mut t = Entry::new_dict();
    *t.key("info") = info;
    let mut r = rig_from("names-odd", t.bencode(), contents, Vec::new());
    let fs = r.info.files();
    assert!(fs.pad_file_at(2) && fs.file_size(1) == 0);
    r.storage.init_names(&[true; 5]);
    assert_eq!(r.incomplete(&r.have(&[])), [true, false, false, true, true]);
    assert_eq!(r.storage.suffix(1), Suffix::Plain, "an empty file");
    assert_eq!(r.storage.suffix(2), Suffix::Plain, "a pad file");
    // a name from the metadata is cut to 240 bytes and its extension (libtorrent's
    // sanitizing): `.!ut` would fit it, but not with the part file's export name after it
    // (`.pa-export`, for a file unskipped later), so it stays plain
    let cut = Path::new(&fs.file_path(3, "")).file_name().unwrap().to_string_lossy().into_owned();
    assert!(cut.len() < long.len() && cut.len() + 4 <= NAME_MAX, "{}", cut.len());
    assert!(cut.len() + 4 + EXPORT_EXT.len() > NAME_MAX, "{}", cut.len());
    assert_eq!(r.storage.suffix(3), Suffix::Plain);
    assert_eq!(r.storage.suffix(4), Suffix::Ut);
    r.check(CheckMode::Full);
    let n = r.info.num_pieces();
    r.write(&(0..n - 1).collect::<Vec<_>>());
    let mut listing = r.listing();
    listing.sort();
    let mut want = vec!["empty".to_owned(), "first.bin.!ut".into(), cut.clone(), format!("{deep}.!ut")];
    want.sort();
    assert_eq!(listing, want, "no pad file on disk, the empty one plain");
    assert!(r.path(&deep).as_os_str().len() > 260);
    r.write(&[n - 1]);
    assert_eq!(r.names(&[false; 5]), (Vec::new(), false));
    assert_eq!(std::fs::read(r.path(&deep)).unwrap(), r.contents[3], "renamed past MAX_PATH");
    assert_eq!(std::fs::read(r.path(&cut)).unwrap(), r.contents[2]);
    assert!(r.check(CheckMode::Full).have.all_set());
}

#[test]
fn a_single_file_torrent() {
    let data: Vec<u8> = (0..70_000).map(|j| (j % 249) as u8).collect();
    let mut info = Entry::new_dict();
    *info.key("length") = Entry::Int(data.len() as i64);
    *info.key("name") = Entry::Bytes(b"single.iso".to_vec());
    *info.key("piece length") = Entry::Int(BLOCK_SIZE as i64);
    *info.key("pieces") =
        Entry::Bytes(data.chunks(BLOCK_SIZE as usize).flat_map(|c| Sha1::digest(c).0).collect());
    let mut t = Entry::new_dict();
    *t.key("info") = info;
    let mut r = rig_from("names-single", t.bencode(), vec![data.clone()], Vec::new());
    r.storage.init_names(&[true]);
    r.check(CheckMode::Full);
    r.write(&[0, 1, 2]);
    assert!(r.dir.join("single.iso.!ut").exists() && !r.dir.join("single.iso").exists());
    r.write(&[3, 4]);
    assert_eq!(r.names(&[false]), (Vec::new(), false));
    assert_eq!(std::fs::read(r.dir.join("single.iso")).unwrap(), data);
    assert!(!r.dir.join("single.iso.!ut").exists());
}

fn renamed(rx: &mpsc::Receiver<Completion>) -> (String, Option<(String, Error)>) {
    loop {
        if let Completion::Renamed { name, error, .. } = next(rx) {
            break (name, error);
        }
    }
}

#[test]
fn a_user_rename_keeps_the_suffix_on_top_of_the_new_name() {
    let mut r = rig("names-user-rename", FILES, Vec::new());
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    r.write(&[0, 1, 2]);
    assert!(r.names(&r.incomplete(&r.have(&[0, 1, 2]))).0.is_empty());
    assert_eq!(r.listing(), ["a", "b.!ut"]);

    // an incomplete file: the new name gets the suffix, the alert the name without it
    r.disk.async_rename_file(&r.storage, 1, "test/sub/b2".into());
    assert_eq!(renamed(&r.rx), ("test/sub/b2".into(), None));
    assert_eq!(r.listing(), ["a", "sub/b2.!ut"]);
    // a complete one: plain
    r.disk.async_rename_file(&r.storage, 0, "test/a2".into());
    assert!(renamed(&r.rx).1.is_none());
    assert_eq!(r.listing(), ["a2", "sub/b2.!ut"]);
    // a file not on disk yet changes its name only, and is created with the suffix
    r.disk.async_rename_file(&r.storage, 2, "test/c2".into());
    assert!(renamed(&r.rx).1.is_none());
    r.write(&[5]);
    assert_eq!(r.listing(), ["a2", "c2.!ut", "sub/b2.!ut"]);
    // a name a file already has, with or without the suffix: refused, nothing moves
    std::fs::write(r.path("taken"), b"x").unwrap();
    r.disk.async_rename_file(&r.storage, 1, "test/taken".into());
    let (_, e) = renamed(&r.rx);
    assert!(e.is_some_and(|(p, e)| p.ends_with("taken") && e.to_string().contains("exists")));
    std::fs::rename(r.path("taken"), r.path("taken.!ut")).unwrap();
    r.disk.async_rename_file(&r.storage, 1, "test/taken".into());
    assert!(renamed(&r.rx).1.is_some());
    std::fs::remove_file(r.path("taken.!ut")).unwrap();
    assert_eq!(r.listing(), ["a2", "c2.!ut", "sub/b2.!ut"]);
    // a new name too long for the suffix: plain, and it stays so
    let long = format!("test/{}", "x".repeat(253));
    r.disk.async_rename_file(&r.storage, 2, long.clone());
    assert!(renamed(&r.rx).1.is_none());
    assert_eq!(r.storage.suffix(2), Suffix::Plain);
    assert!(r.dir.join(&long).exists());
    // the rest of the pieces, then the names: every file under its new plain name
    r.write(&[3, 4, 6, 7]);
    assert_eq!(r.names(&[false; 3]), (Vec::new(), false));
    assert_eq!(r.listing(), ["a2".to_owned(), "sub/b2".into(), "x".repeat(253)]);
    assert_eq!(std::fs::read(r.path("sub/b2")).unwrap(), r.contents[1]);
    assert!(r.check(CheckMode::Full).have.all_set());
}

/// Windows file systems ignore case: a rename that only changes the case of a suffixed file
/// works, as it does for a plain one.
#[cfg(windows)]
#[test]
fn a_change_of_case_only_renames_a_suffixed_file() {
    let mut r = rig("names-case", FILES, Vec::new());
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    r.write(&[0, 1]);
    r.disk.async_rename_file(&r.storage, 0, "test/A".into());
    assert!(renamed(&r.rx).1.is_none());
    assert_eq!(r.listing(), ["A.!ut"]);
    assert_eq!(r.read(0), r.piece_data(0));
    // a suffix in another case is the same file for the probe
    let mut r2 = rig("names-case-qb", FILES, Vec::new());
    std::fs::create_dir_all(r2.dir.join("test")).unwrap();
    std::fs::write(r2.path("a.!QB"), &r2.contents[0]).unwrap();
    r2.storage.init_names(&[false; 3]);
    assert_eq!(wanted(&r2.check(CheckMode::Full).have), [0, 1]);
    assert_eq!(r2.names(&[false; 3]), (Vec::new(), false));
    assert_eq!(r2.listing(), ["a"]);
}

fn moved(rx: &mpsc::Receiver<Completion>) -> (MoveStatus, String, Option<MoveError>) {
    loop {
        if let Completion::Moved { status, save_path, error, .. } = next(rx) {
            break (status, save_path, error);
        }
    }
}

#[test]
fn moves_take_the_files_under_the_names_they_have() {
    let mut r = rig("names-move", FILES, Vec::new());
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    r.write(&[0, 1, 2, 3]);
    assert!(r.names(&r.incomplete(&r.have(&[0, 1, 2, 3]))).0.is_empty());
    assert_eq!(r.listing(), ["a", "b.!ut"]);
    let to = r.dir.join("moved");
    r.disk.async_move_storage(&r.storage, to.to_string_lossy().into_owned(), MoveFlags::AlwaysReplaceFiles);
    assert_eq!(moved(&r.rx).0, MoveStatus::NoError);
    assert!(to.join("test/a").exists() && to.join("test/b.!ut").exists());
    assert!(!r.path("b.!ut").exists());
    assert_eq!(r.read(3), r.piece_data(3));
    r.write(&[4, 5, 6, 7]);
    assert_eq!(r.names(&[false; 3]), (Vec::new(), false));
    for (i, n) in ["a", "b", "c"].iter().enumerate() {
        assert_eq!(std::fs::read(to.join("test").join(n)).unwrap(), r.contents[i]);
    }

    // a stopped torrent's move (a storage made for it): the names come from the disk
    let back = r.dir.join("back");
    std::fs::rename(to.join("test/c"), to.join("test/c.!ut")).unwrap();
    r.restart(&[false; 3]);
    r.disk.async_move_storage(&r.storage, back.to_string_lossy().into_owned(), MoveFlags::AlwaysReplaceFiles);
    assert_eq!(moved(&r.rx).0, MoveStatus::NoError);
    assert!(back.join("test/c.!ut").exists(), "found under its name and moved with it");
    assert!(!to.join("test").exists(), "nothing left behind");

    // the download location changed by hand (reset, unchecked): the files there by their names
    let elsewhere = r.dir.join("elsewhere");
    std::fs::create_dir_all(elsewhere.join("test")).unwrap();
    std::fs::rename(back.join("test/a"), elsewhere.join("test/a.!ut")).unwrap();
    std::fs::rename(back.join("test/b"), elsewhere.join("test/b")).unwrap();
    std::fs::rename(back.join("test/c.!ut"), elsewhere.join("test/c")).unwrap();
    r.disk.async_move_storage(
        &r.storage,
        elsewhere.to_string_lossy().into_owned(),
        MoveFlags::ResetSavePathUnchecked,
    );
    assert_eq!(moved(&r.rx).0, MoveStatus::NoError);
    assert_eq!(r.storage.suffix(0), Suffix::Ut);
    assert_eq!(r.storage.suffix(2), Suffix::Plain);
    assert_eq!(r.read(0), r.piece_data(0));
    assert!(r.check(CheckMode::Full).have.all_set());
}

#[test]
fn deleting_removes_suffixed_files_and_our_leftovers() {
    let r = rig("names-delete", FILES, Vec::new());
    std::fs::create_dir_all(r.dir.join("test")).unwrap();
    std::fs::write(r.path("a.!ut"), &r.contents[0][..100]).unwrap();
    std::fs::write(r.path("b"), &r.contents[1]).unwrap();
    std::fs::write(r.path("c"), &r.contents[2]).unwrap();
    std::fs::write(r.path("c.!ut"), b"left over").unwrap();
    std::fs::write(r.dir.join("keep.txt"), b"not the torrent's").unwrap();
    // a storage made for the delete, with names the engine would guess
    r.storage.init_names(&[false; 3]);
    r.disk.async_fence(&r.storage, FenceAction::Delete);
    loop {
        if let Completion::Fenced { error, .. } = next(&r.rx) {
            assert!(error.is_none(), "{error:?}");
            break;
        }
    }
    assert!(!r.dir.join("test").exists(), "every file gone, and the folder");
    assert!(r.dir.join("keep.txt").exists());
}

#[test]
fn a_skipped_file_unskipped_mid_download_is_exported_under_its_name() {
    let mut r = rig("names-part", FILES, vec![4, 0, 4]);
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    // the pieces shared with b go to the part file as before
    r.write(&[0, 1, 2, 5, 6, 7]);
    let have = r.have(&[0, 1, 2, 5, 6, 7]);
    assert!(r.names(&r.incomplete(&have)).0.is_empty());
    let part = format!(".{}.parts", r.info.info_hash().to_hex());
    assert!(r.dir.join(&part).exists());
    assert_eq!(r.listing(), ["a", "c"], "b, skipped, has no file");
    // 0 -> normal: b's blocks leave the part file into b.!ut
    r.disk.async_set_file_priority(&r.storage, vec![4, 4, 4]);
    r.write(&[3]);
    assert_eq!(r.listing(), ["a", "b.!ut", "c"]);
    r.write(&[4]);
    assert_eq!(r.names(&[false; 3]), (Vec::new(), false));
    assert_eq!(std::fs::read(r.path("b")).unwrap(), r.contents[1]);
    assert!(!r.dir.join(&part).exists());
    assert!(r.check(CheckMode::Full).have.all_set());
}

/// The review's F1, on the disk threads: a new storage's check runs on a checking thread and is
/// not ordered against the old storage's barriers on the I/O threads, so the engine starts the
/// new storage only once the old one's names job has answered (`Engine::start_torrent`). What
/// that relies on: the answer comes once the rename is done, before the release's, however many
/// writes the job waited behind, and a storage made after it finds the file under its new name
/// (made before it, it found `a.!ut`, which the old storage then renamed away).
#[test]
fn a_storage_made_after_the_names_answer_finds_the_new_names() {
    let mut r = rig("names-race", FILES, Vec::new());
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    r.write(&[0, 1, 2]);
    assert_eq!(r.listing(), ["a.!ut", "b.!ut"]);
    // a backlog of writes (b's piece 3, again and again), a's name, the release
    let block = r.all[3 * BLOCK_SIZE as usize..4 * BLOCK_SIZE as usize].to_vec();
    for _ in 0..6000 {
        r.disk.async_write(&r.storage, 3, 0, block.clone());
    }
    let have = r.have(&[0, 1, 2]);
    r.disk.async_set_names(&r.storage, r.incomplete(&have));
    r.disk.async_fence(&r.storage, FenceAction::Release);
    loop {
        match next(&r.rx) {
            Completion::Names { failed, pending, .. } => {
                assert!(failed.is_empty() && !pending, "{failed:?}");
                break;
            }
            Completion::Fenced { .. } => panic!("released before the names answer"),
            _ => {}
        }
    }
    assert!(r.path("a").exists() && !r.path("a.!ut").exists());
    // the start the engine held back: a is complete in the pieces it saved
    let prio = r.storage.lock_files().priority.clone();
    r.storage = r.disk.new_storage(r.storage.save_path(), r.info.clone(), prio);
    r.storage.init_names(&r.incomplete(&have));
    r.disk.async_check(&r.storage, CheckMode::Resume(have.clone()));
    let (mut res, mut released) = (None, false);
    while res.is_none() || !released {
        match next(&r.rx) {
            Completion::Checked { result, .. } => res = Some(result.unwrap()),
            // (the old storage's writes end before its release)
            Completion::Fenced { .. } => released = true,
            _ => {}
        }
    }
    let res = res.unwrap();
    assert!(!res.resume_failed && res.missing.is_empty() && res.names.is_empty(), "{:?}", res.names);
    assert_eq!(res.have, have);
    assert_eq!(r.storage.suffix(0), Suffix::Plain);
    assert!(!r.storage.names_differ(&r.incomplete(&have)));
    assert_eq!(r.read(0), r.piece_data(0));
    assert_eq!(r.listing(), ["a", "b.!ut"]);
}

/// The review's F2: Relocate of a file that is not on disk points the torrent at the file the
/// user picked, under the name it has there (plain, or another client's partial); a check
/// counts it, and it ends under the plain name with nothing beside it.
#[test]
fn relocating_a_file_not_on_disk_onto_an_existing_file_uses_it() {
    let mut r = rig("names-relocate", FILES, Vec::new());
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    let other = r.dir.join("other");
    std::fs::create_dir_all(&other).unwrap();
    std::fs::write(other.join("a-copy"), &r.contents[0]).unwrap();
    std::fs::write(other.join("c-copy.!qB"), &r.contents[2]).unwrap();
    for (f, name) in [(0, "a-copy"), (2, "c-copy")] {
        r.disk.async_rename_file(&r.storage, f, other.join(name).to_string_lossy().into_owned());
        assert!(renamed(&r.rx).1.is_none());
    }
    assert_eq!(r.storage.suffix(0), Suffix::Plain, "the plain file there is used");
    assert_eq!(r.storage.suffix(2), Suffix::Qb);
    // Force Re-Check: a's pieces and c's (piece 2 and 5 also need b's)
    let res = r.check(CheckMode::Full);
    assert_eq!(wanted(&res.have), [0, 1, 6, 7]);
    assert!(r.names(&r.incomplete(&res.have)).0.is_empty());
    assert_eq!(files_in(&other), ["a-copy.!ut", "c-copy.!ut"]);
    r.write(&[2, 3, 4, 5]);
    assert_eq!(r.names(&[false; 3]), (Vec::new(), false));
    assert_eq!(files_in(&other), ["a-copy", "c-copy"], "nothing beside them");
    assert_eq!(r.listing(), ["b"]);
    assert_eq!(std::fs::read(other.join("a-copy")).unwrap(), r.contents[0]);
    assert_eq!(std::fs::read(other.join("c-copy")).unwrap(), r.contents[2]);
    assert!(r.check(CheckMode::Full).have.all_set());
}

/// The review's F4: a move that replaces files takes the place of a plain file of the same name
/// with our `.!ut` one (as it replaced it before `.!ut`), a file not on disk here is used under
/// the name it has there, and one that does not replace keeps a file there under any name.
#[test]
fn moves_replace_files_of_the_same_name_and_use_those_they_keep() {
    let mut r = rig("names-move-replace", FILES, Vec::new());
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    r.write(&[0, 1]);
    assert_eq!(r.listing(), ["a.!ut"]);
    let to = r.dir.join("to");
    std::fs::create_dir_all(to.join("test")).unwrap();
    std::fs::write(to.join("test/a"), b"replaced").unwrap();
    std::fs::write(to.join("test/c"), &r.contents[2]).unwrap();
    let path = to.to_string_lossy().into_owned();
    r.disk.async_move_storage(&r.storage, path, MoveFlags::AlwaysReplaceFiles);
    assert_eq!(moved(&r.rx).0, MoveStatus::NoError);
    assert_eq!(files_in(&to.join("test")), ["a.!ut", "c"]);
    assert_eq!((r.storage.suffix(0), r.storage.suffix(2)), (Suffix::Ut, Suffix::Plain));
    assert_eq!(r.read(0), r.piece_data(0));
    assert_eq!(wanted(&r.check(CheckMode::Full).have), [0, 1, 6, 7]);
    r.write(&[2, 3, 4, 5]);
    assert_eq!(r.names(&[false; 3]), (Vec::new(), false));
    assert_eq!(files_in(&to.join("test")), ["a", "b", "c"]);
    assert!(r.check(CheckMode::Full).have.all_set());

    // not replacing: a file there under its plain name is kept (ours stays behind) and used
    let mut r = rig("names-move-keep", FILES, Vec::new());
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    r.write(&[0, 1]);
    let to = r.dir.join("to");
    std::fs::create_dir_all(to.join("test")).unwrap();
    std::fs::write(to.join("test/a"), &r.contents[0]).unwrap();
    let path = to.to_string_lossy().into_owned();
    r.disk.async_move_storage(&r.storage, path, MoveFlags::DontReplace);
    assert_eq!(moved(&r.rx).0, MoveStatus::NeedFullCheck);
    assert_eq!(files_in(&to.join("test")), ["a"]);
    assert_eq!(r.storage.suffix(0), Suffix::Plain);
    assert_eq!(wanted(&r.check(CheckMode::Full).have), [0, 1]);

    // failing if anything exists: a plain file there counts
    let mut r = rig("names-move-fail", FILES, Vec::new());
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    r.write(&[0, 1]);
    let to = r.dir.join("to");
    std::fs::create_dir_all(to.join("test")).unwrap();
    std::fs::write(to.join("test/a"), b"someone's").unwrap();
    let path = to.to_string_lossy().into_owned();
    r.disk.async_move_storage(&r.storage, path, MoveFlags::FailIfExist);
    let (status, _, error) = moved(&r.rx);
    assert_eq!(status, MoveStatus::FileExist);
    assert!(error.is_some_and(|e| e.file.ends_with('a')));
    assert_eq!(r.listing(), ["a.!ut"]);
}

/// A Unix rename to the name a file should have is a hard link plus an unlink: one cut short
/// between them (a crash) leaves both names on one file. The next names job finishes it.
#[cfg(unix)]
#[test]
fn a_rename_cut_short_between_link_and_unlink_is_finished() {
    let mut r = rig("names-link", FILES, Vec::new());
    std::fs::create_dir_all(r.dir.join("test")).unwrap();
    std::fs::write(r.path("a.!ut"), &r.contents[0]).unwrap();
    std::fs::hard_link(r.path("a.!ut"), r.path("a")).unwrap();
    // the resume data saved before the crash: a incomplete
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    assert_eq!(r.storage.suffix(0), Suffix::Ut);
    assert_eq!(r.names(&[false, true, true]), (Vec::new(), false));
    assert_eq!(r.listing(), ["a"]);
    assert_eq!(std::fs::read(r.path("a")).unwrap(), r.contents[0]);
    // a different file of the name: refused as before, not deleted
    std::fs::write(r.path("a.!ut"), b"another").unwrap();
    r.storage.set_suffix(0, Suffix::Ut);
    let (failed, pending) = r.names(&[false, true, true]);
    assert!(!pending && failed.len() == 1, "{failed:?}");
    assert_eq!(std::fs::read(r.path("a.!ut")).unwrap(), b"another");
}

/// The review's F6: the probe's look at each file serves the check of the resume data, but a
/// file the probe did not find is looked at again: one unskipped while the torrent was stopped
/// is taken out of the part file between the two, and counts.
#[test]
fn a_file_unskipped_while_stopped_counts_for_the_resume_data() {
    let mut r = rig("names-unskip-stopped", FILES, vec![4, 0, 4]);
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    r.write(&[0, 1, 2, 5, 6, 7]);
    let have = r.have(&[0, 1, 2, 5, 6, 7]);
    assert!(r.names(&r.incomplete(&have)).0.is_empty());
    assert_eq!(r.listing(), ["a", "c"]);
    r.restart_with(&r.incomplete(&have), vec![4, 4, 4]);
    let res = r.check(CheckMode::Resume(have.clone()));
    assert!(!res.resume_failed && res.missing.is_empty(), "{:?}", res.missing);
    assert_eq!(res.have, have);
    assert_eq!(r.listing(), ["a", "b.!ut", "c"]);
    r.write(&[3, 4]);
    assert_eq!(r.names(&[false; 3]), (Vec::new(), false));
    assert_eq!(std::fs::read(r.path("b")).unwrap(), r.contents[1]);
}

/// The review's F7: a file is taken out of the part file under its name plus `.pa-export`, so a
/// name too long for that after `.!ut` (here a user's 243-byte one) stays plain.
#[test]
fn a_long_name_unskipped_is_exported_within_the_name_limit() {
    let mut r = rig("names-long-export", FILES, vec![4, 0, 4]);
    r.storage.init_names(&[true; 3]);
    r.check(CheckMode::Full);
    let long = format!("test/{}.bin", "l".repeat(239));
    r.disk.async_rename_file(&r.storage, 1, long.clone());
    assert!(renamed(&r.rx).1.is_none());
    assert_eq!(r.storage.suffix(1), Suffix::Plain);
    r.write(&[0, 1, 2, 5, 6, 7]);
    r.disk.async_set_file_priority(&r.storage, vec![4, 4, 4]);
    r.write(&[3, 4]);
    assert_eq!(std::fs::read(r.dir.join(&long)).unwrap(), r.contents[1]);
    // a `.!ut` partial of such a name (another client's) is still found, and loses it once complete
    let mut r = rig("names-long-found", FILES, Vec::new());
    std::fs::create_dir_all(r.dir.join("test")).unwrap();
    let name = format!("{}.bin", "m".repeat(240));
    r.disk.async_rename_file(&r.storage, 0, format!("test/{name}"));
    assert!(renamed(&r.rx).1.is_none());
    std::fs::write(r.path(&format!("{name}.!ut")), &r.contents[0]).unwrap();
    r.storage.probed.store(false, Ordering::Relaxed);
    let res = r.check(CheckMode::Full);
    assert_eq!(wanted(&res.have), [0, 1]);
    assert_eq!(r.storage.suffix(0), Suffix::Ut);
    assert!(r.names(&r.incomplete(&res.have)).0.is_empty());
    assert_eq!(r.listing(), [format!("{name}.!ut")], "kept while incomplete");
    r.write(&[2]);
    assert!(r.names(&r.incomplete(&r.have(&[0, 1, 2]))).0.is_empty());
    assert_eq!(r.listing(), ["b.!ut".to_owned(), name]);
}
