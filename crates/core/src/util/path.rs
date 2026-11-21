// Ported from libtorrent-rasterbar 2.0.11: src/path.cpp (rsplit_path, lsplit_path, append_path,
// combine_path, is_complete, has_parent_path, filename, extension, remove_extension, path_compare)
// Copyright (c) 2003-2020, Arvid Norberg and contributors. BSD-3-Clause, see THIRD-PARTY-NOTICES.md.
// Changes: torrent-internal paths always use '/' on every platform (libtorrent uses the native
// separator); only `is_complete` knows about Windows path forms, for absolute file renames.

//! Helpers for torrent-relative paths. Paths are UTF-8 with '/' separators; conversion to native
//! paths happens at the disk layer.

use std::cmp::Ordering;

pub const SEPARATOR: char = '/';

/// Splits off the last element: `"a/b/c"` -> `("a/b", "c")`. A trailing separator is ignored.
pub fn rsplit_path(p: &str) -> (&str, &str) {
    let p = p.strip_suffix(SEPARATOR).unwrap_or(p);
    match p.rfind(SEPARATOR) {
        None => ("", p),
        Some(i) => (&p[..i], &p[i + 1..]),
    }
}

/// Splits off the first element: `"a/b/c"` -> `("a", "b/c")`. A leading separator is ignored.
pub fn lsplit_path(p: &str) -> (&str, &str) {
    let p = p.strip_prefix(SEPARATOR).unwrap_or(p);
    match p.find(SEPARATOR) {
        None => (p, ""),
        Some(i) => (&p[..i], &p[i + 1..]),
    }
}

/// Like [`lsplit_path`] but searches for the separator starting at byte `pos`.
pub fn lsplit_path_at(p: &str, mut pos: usize) -> (&str, &str) {
    let p = match p.strip_prefix(SEPARATOR) {
        Some(rest) => {
            pos = pos.saturating_sub(1);
            rest
        }
        None => p,
    };
    match p.get(pos..).and_then(|tail| tail.find(SEPARATOR)) {
        None => (p, ""),
        Some(i) => (&p[..pos + i], &p[pos + i + 1..]),
    }
}

/// Appends `leaf` to `branch` with exactly one separator between them.
pub fn append_path(branch: &mut String, leaf: &str) {
    if branch.is_empty() || branch == "." {
        branch.clear();
        branch.push_str(leaf);
        return;
    }
    if leaf.is_empty() {
        return;
    }
    if !branch.ends_with(SEPARATOR) {
        branch.push(SEPARATOR);
    }
    branch.push_str(leaf);
}

pub fn combine_path(lhs: &str, rhs: &str) -> String {
    if lhs.is_empty() || lhs == "." {
        return rhs.to_owned();
    }
    if rhs.is_empty() || rhs == "." {
        return lhs.to_owned();
    }
    let mut out = String::with_capacity(lhs.len() + rhs.len() + 1);
    out.push_str(lhs);
    append_path(&mut out, rhs);
    out
}

/// True for absolute paths: `C:\` / `C:/` / `\\server` on Windows, `/...` elsewhere.
pub fn is_complete(f: &str) -> bool {
    if cfg!(windows) {
        let b = f.as_bytes();
        let letters = b.iter().take_while(|c| c.is_ascii_alphabetic()).count();
        if letters + 1 < b.len() && b[letters] == b':' && matches!(b[letters + 1], b'\\' | b'/') {
            return true;
        }
        f.starts_with("\\\\")
    } else {
        f.starts_with('/')
    }
}

/// Whether `name` is a file name inside the save folder: what a torrent's file may be renamed
/// to by someone who may not name other folders (a remote session, the Web UI). Relative (no
/// leading separator, no drive, no `\\server`), every part between `/` and `\` separators
/// neither empty, `.` nor `..`, no control characters; on Windows also no `:` (a drive-relative
/// path, an alternate data stream), none of the characters Windows refuses in names, and no
/// part that Windows trims to nothing (trailing dots and spaces, as in `.. `). The engine itself
/// also takes absolute names (libtorrent's `rename_file`), for its local owner.
pub fn is_relative_name(name: &str) -> bool {
    let windows_only_bad = |c: char| cfg!(windows) && matches!(c, ':' | '<' | '>' | '"' | '|' | '?' | '*');
    !name.is_empty()
        && !name.chars().any(|c| c.is_control() || windows_only_bad(c))
        && name.split(['/', '\\']).all(|part| {
            let kept = if cfg!(windows) { part.trim_end_matches(['.', ' ']) } else { part };
            !part.is_empty() && part != "." && part != ".." && !kept.is_empty()
        })
}

/// True if `f` has a directory part (`"a/b"` yes, `"b"` and `"b/"` no).
pub fn has_parent_path(f: &str) -> bool {
    if f.is_empty() || f == "/" {
        return false;
    }
    let f = f.strip_suffix(['/', '\\']).unwrap_or(f);
    f.contains(['/', '\\'])
}

/// The last path element, ignoring one trailing separator.
pub fn filename(f: &str) -> &str {
    rsplit_path(f).1
}

/// The extension including the dot (`"a/b.tar.gz"` -> `".gz"`), or `""`.
pub fn extension(f: &str) -> &str {
    let name_start = f.rfind(SEPARATOR).map_or(0, |i| i + 1);
    match f[name_start..].rfind('.') {
        Some(i) => &f[name_start + i..],
        None => "",
    }
}

/// `f` without its extension. A leading dot (".hidden") is not treated as an extension.
pub fn remove_extension(f: &str) -> &str {
    let slash = f.rfind(SEPARATOR);
    match f.rfind('.') {
        Some(0) | None => f,
        Some(dot) if slash.is_some_and(|s| dot < s) => f,
        Some(dot) => &f[..dot],
    }
}

/// Orders `lhs/lfile` against `rhs/rfile` element by element (libtorrent `path_compare`).
pub fn path_compare(lhs: &str, lfile: &str, rhs: &str, rfile: &str) -> Ordering {
    let (mut l, mut r) = (lsplit_path(lhs), lsplit_path(rhs));
    while !l.0.is_empty() || !r.0.is_empty() {
        if l.0.is_empty() || r.0.is_empty() {
            let le = if l.0.is_empty() { lfile } else { l.0 };
            let re = if r.0.is_empty() { rfile } else { r.0 };
            return le.cmp(re);
        }
        match l.0.cmp(r.0) {
            Ordering::Equal => {}
            other => return other,
        }
        l = lsplit_path(l.1);
        r = lsplit_path(r.1);
    }
    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitting() {
        assert_eq!(rsplit_path("a/b/c"), ("a/b", "c"));
        assert_eq!(rsplit_path("a/b/c/"), ("a/b", "c"));
        assert_eq!(rsplit_path("c"), ("", "c"));
        assert_eq!(rsplit_path(""), ("", ""));
        assert_eq!(lsplit_path("a/b/c"), ("a", "b/c"));
        assert_eq!(lsplit_path("/a/b"), ("a", "b"));
        assert_eq!(lsplit_path("a"), ("a", ""));
        assert_eq!(lsplit_path_at("a/b/c", 2), ("a/b", "c"));
        // the leading "/" is stripped and pos shifts with it (libtorrent semantics)
        assert_eq!(lsplit_path_at("/a/b/c", 2), ("a", "b/c"));
        assert_eq!(lsplit_path_at("a/b/c", 4), ("a/b/c", ""));
    }

    #[test]
    fn joining() {
        let mut p = String::new();
        append_path(&mut p, "a");
        append_path(&mut p, "b");
        append_path(&mut p, "");
        assert_eq!(p, "a/b");
        assert_eq!(combine_path("a/", "b"), "a/b");
        assert_eq!(combine_path(".", "b"), "b");
        assert_eq!(combine_path("a", ""), "a");
    }

    #[test]
    fn names_and_extensions() {
        assert!(has_parent_path("a/b"));
        assert!(!has_parent_path("b"));
        assert!(!has_parent_path("b/"));
        assert!(!has_parent_path(""));
        assert_eq!(filename("a/b/c.txt"), "c.txt");
        assert_eq!(filename("a/b/"), "b");
        assert_eq!(extension("a/b.tar.gz"), ".gz");
        assert_eq!(extension("a.b/c"), "");
        assert_eq!(remove_extension("a/b.tar.gz"), "a/b.tar");
        assert_eq!(remove_extension("a.b/c"), "a.b/c");
        assert_eq!(remove_extension(".hidden"), ".hidden");
    }

    #[test]
    fn complete_paths() {
        if cfg!(windows) {
            assert!(is_complete("C:\\x") && is_complete("c:/x") && is_complete("\\\\srv\\share"));
            assert!(!is_complete("x/y") && !is_complete("C:"));
        } else {
            assert!(is_complete("/x") && !is_complete("x/y"));
        }
    }

    /// Bug hunt 2026-11-21, 2: a name a remote session or the Web UI renames a file to stays
    /// inside the save folder.
    #[test]
    fn relative_names_stay_inside_the_save_folder() {
        for good in ["a", "a/b.txt", "Show S01/e 1.mkv", "a\\b", ".hidden", "a..b", "\u{e9}t\u{e9}"] {
            assert!(is_relative_name(good), "{good}");
        }
        for bad in [
            "",
            "/etc/x",
            "\\x",
            "//srv/share/x",
            "\\\\srv\\share\\x",
            "..",
            "../x",
            "a/../../x",
            "a\\..\\x",
            "a/./b",
            "a//b",
            "a/",
            ".",
            "a\nb",
            "a\0b",
        ] {
            assert!(!is_relative_name(bad), "{bad:?}");
        }
        // drive paths, streams and the names Windows trims or refuses; ordinary names elsewhere
        for windows_bad in ["C:\\x", "C:x", "c:/x", "a:stream", "a/.. ", "a/...", "a/ .", "a?", "a*b", "a|b"]
        {
            assert_eq!(is_relative_name(windows_bad), !cfg!(windows), "{windows_bad:?}");
        }
    }

    #[test]
    fn compare() {
        assert_eq!(path_compare("a/b", "f", "a/b", "f"), Ordering::Equal);
        assert_eq!(path_compare("a", "z", "a/b", "f"), Ordering::Greater); // "z" vs "b"
        assert_eq!(path_compare("a/b", "f", "a/c", "f"), Ordering::Less);
        assert_eq!(path_compare("", "a", "", "b"), Ordering::Equal); // both empty: equal paths
    }
}
