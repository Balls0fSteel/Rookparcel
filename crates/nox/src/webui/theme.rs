//! The Web UI's themes (D63): a theme is one small CSS file that sets the page's custom
//! properties (`style.css`'s `:root` colours and `color-scheme`), loaded after `style.css` from
//! `/themes/<id>.css`. The built-in ones are embedded; third-party ones are `*.css` files in
//! `webui/themes` under the settings directory, listed by the name in their first comment
//! (`/* name: ... */`) or their file name. `system` is the default and has no file: the page
//! follows the browser's light or dark setting then. The choice is the `webui.theme` preference
//! for everyone, and the page can keep one of its own per browser (docs/WEBUI.md "Themes").

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use rookparcel_core::crypto::Sha1;
use rookparcel_frontend::lang::{self, S};

use super::api::{StaticFile, etag_matches, serve_static};
use super::http::{Head, Response};

/// The default: no theme file, the browser's light or dark setting.
pub const SYSTEM: &str = "system";

/// The built-in themes, in the picker's order: the id (its file is `themes/<id>.css`) and the
/// name shown.
pub const BUILT_IN: &[(&str, S)] = &[
    (SYSTEM, S::ThemeSystem),
    ("light", S::ThemeLight),
    ("dark", S::ThemeDark),
    ("midnight", S::ThemeMidnight),
    ("slate", S::ThemeSlate),
    ("solarized-dark", S::ThemeSolarizedDark),
    ("solarized-light", S::ThemeSolarizedLight),
    ("high-contrast", S::ThemeHighContrast),
    ("postal-rook", S::ThemePostalRook),
];

/// The longest theme id taken.
pub const MAX_ID: usize = 40;
/// The largest third-party theme file served (a theme is a few hundred bytes of variables).
pub const MAX_BYTES: u64 = 64 * 1024;
/// How much of a third-party file is read for its `/* name: ... */` comment.
const NAME_SCAN: usize = 512;
/// The longest display name taken from that comment.
const MAX_NAME: usize = 60;

const CSS: &str = "text/css; charset=UTF-8";

macro_rules! theme_file {
    ($id:literal) => {
        StaticFile::new(
            concat!("/themes/", $id, ".css"),
            CSS,
            include_bytes!(concat!("themes/", $id, ".css")),
        )
    };
}

static FILES: LazyLock<[StaticFile; 8]> = LazyLock::new(|| {
    [
        theme_file!("light"),
        theme_file!("dark"),
        theme_file!("midnight"),
        theme_file!("slate"),
        theme_file!("solarized-dark"),
        theme_file!("solarized-light"),
        theme_file!("high-contrast"),
        theme_file!("postal-rook"),
    ]
});

/// Whether `s` can name a theme: 1 to [`MAX_ID`] lower-case ASCII letters, digits and dashes,
/// not starting or ending with a dash. Nothing else reaches a file name, so `..`, separators,
/// percent escapes and case variants are refused before any path is built.
pub fn valid_id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= MAX_ID
        && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !s.starts_with('-')
        && !s.ends_with('-')
}

fn is_built_in(id: &str) -> bool {
    BUILT_IN.iter().any(|(b, _)| *b == id)
}

/// Where third-party themes are looked for.
pub fn dir(settings_dir: &Path) -> PathBuf {
    settings_dir.join("webui").join("themes")
}

/// The display name in a theme file's first comment, `/* name: Mine */`, if any.
fn name_of(head: &[u8]) -> Option<String> {
    let text = String::from_utf8_lossy(head);
    let start = text.find("/*")?;
    let body = &text[start + 2..];
    let end = body.find("*/")?;
    let comment = body[..end].trim();
    let name = comment.strip_prefix("name:")?.trim();
    let name: String = name.chars().filter(|c| !c.is_control()).take(MAX_NAME).collect();
    let name = name.trim().to_owned();
    if name.is_empty() { None } else { Some(name) }
}

/// The third-party themes as `(id, name)`, sorted by name: the `*.css` files in [`dir`] whose
/// stem is a [`valid_id`] that is not a built-in's and whose size is within [`MAX_BYTES`]. A
/// missing folder is simply no themes.
pub fn third_party(settings_dir: &Path) -> Vec<(String, String)> {
    let Ok(entries) = fs::read_dir(dir(settings_dir)) else { return Vec::new() };
    let mut out = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "css") {
            continue;
        }
        let Some(id) = path.file_stem().and_then(|s| s.to_str()) else { continue };
        if !valid_id(id) || is_built_in(id) {
            continue;
        }
        let Ok(meta) = entry.metadata() else { continue };
        if !meta.is_file() || meta.len() > MAX_BYTES {
            continue;
        }
        let name = fs::File::open(&path)
            .ok()
            .and_then(|mut f| {
                use std::io::Read;
                let mut head = vec![0u8; NAME_SCAN];
                let n = f.read(&mut head).ok()?;
                head.truncate(n);
                name_of(&head)
            })
            .unwrap_or_else(|| id.to_owned());
        out.push((id.to_owned(), name));
    }
    out.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()).then_with(|| a.0.cmp(&b.0)));
    out
}

/// The Preferences list as `(id, name)`: the built-ins in the current language, then the
/// third-party files, then `current` when it is none of them (a file since removed), so the
/// drop-down still shows what is set.
pub fn choices(settings_dir: &Path, current: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> =
        BUILT_IN.iter().map(|(id, name)| ((*id).to_owned(), lang::text(*name).to_owned())).collect();
    out.extend(third_party(settings_dir));
    let current = current.trim();
    if !current.is_empty() && !out.iter().any(|(id, _)| id == current) {
        out.push((current.to_owned(), current.to_owned()));
    }
    out
}

/// `GET /themes/<id>.css`: a built-in file like the other statics, else the third-party file
/// of that name (read on each request, within [`MAX_BYTES`]) with an `ETag` and `no-cache` like
/// them; anything else, `system` included, is 404.
pub fn serve(settings_dir: &Path, id: &str, head: &Head) -> Response {
    if let Some(f) = FILES
        .iter()
        .find(|f| f.path.strip_prefix("/themes/").and_then(|p| p.strip_suffix(".css")) == Some(id))
    {
        return serve_static(f, head);
    }
    if !valid_id(id) || is_built_in(id) {
        return Response::text(404, "Not Found");
    }
    let path = dir(settings_dir).join(format!("{id}.css"));
    let body = match fs::metadata(&path) {
        Ok(m) if m.is_file() && m.len() <= MAX_BYTES => fs::read(&path),
        _ => return Response::text(404, "Not Found"),
    };
    let Ok(body) = body else { return Response::text(404, "Not Found") };
    let etag = format!("\"{}\"", Sha1::digest(&body).to_hex());
    let r =
        if etag_matches(head, &etag) { Response::new(304, "", "") } else { Response::new(200, CSS, body) };
    r.with_header("ETag", etag).with_header("Cache-Control", "no-cache")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_lower_case_names_without_separators() {
        for ok in ["light", "postal-rook", "a", "x1", &"a".repeat(MAX_ID)] {
            assert!(valid_id(ok), "{ok}");
        }
        for bad in
            ["", "Light", "../x", "a/b", "a\\b", "a%2fb", "a b", "-a", "a-", "a.css", &"a".repeat(MAX_ID + 1)]
        {
            assert!(!valid_id(bad), "{bad}");
        }
    }

    #[test]
    fn every_built_in_has_a_valid_id_and_a_file_but_system() {
        for (id, _) in BUILT_IN {
            assert!(valid_id(id), "{id}");
            let has_file = FILES.iter().any(|f| f.path == format!("/themes/{id}.css"));
            assert_eq!(has_file, *id != SYSTEM, "{id}");
        }
        assert_eq!(FILES.len(), BUILT_IN.len() - 1);
    }

    /// Every built-in theme sets every colour of `style.css`'s `:root` (not the layout sizes),
    /// `color-scheme`, and nothing the page's CSP would refuse; and stays small.
    #[test]
    fn every_built_in_theme_defines_every_token() {
        let style = include_str!("style.css");
        let root = &style[style.find(":root {").unwrap()..];
        let root = &root[..root.find('}').unwrap()];
        let tokens: Vec<&str> = root
            .lines()
            .filter_map(|l| l.trim().strip_suffix(';').and_then(|l| l.split_once(':')).map(|(k, _)| k.trim()))
            .filter(|k| k.starts_with("--") && !matches!(*k, "--row-h" | "--tree-w" | "--detail-h"))
            .collect();
        assert_eq!(tokens.len(), 29, "{tokens:?}");
        for f in FILES.iter() {
            let css = std::str::from_utf8(f.body).unwrap();
            assert!(css.len() < 4 * 1024, "{}: {} bytes", f.path, css.len());
            assert!(css.starts_with("/* name: "), "{}", f.path);
            assert!(css.contains("color-scheme:"), "{}", f.path);
            for t in &tokens {
                assert!(css.contains(&format!("  {t}:")), "{}: no {t}", f.path);
            }
            for refused in ["url(", "@import", "@font-face", "<"] {
                assert!(!css.contains(refused), "{}: {refused}", f.path);
            }
        }
    }

    #[test]
    fn third_party_files_are_listed_by_name_and_filtered() {
        let dir = std::env::temp_dir().join(format!("rp-themes-{}", std::process::id()));
        let themes = dir.join("webui").join("themes");
        fs::create_dir_all(&themes).unwrap();
        fs::write(themes.join("mine.css"), "/* name: Mine, really */\n:root { --bg: #000; }\n").unwrap();
        fs::write(themes.join("zed.css"), ":root { --bg: #111; }\n").unwrap();
        fs::write(themes.join("Bad Name.css"), ":root {}").unwrap();
        fs::write(themes.join("dark.css"), ":root {}").unwrap();
        fs::write(themes.join("x.txt"), ":root {}").unwrap();
        fs::write(themes.join("big.css"), vec![b' '; MAX_BYTES as usize + 1]).unwrap();
        let list = third_party(&dir);
        assert_eq!(
            list,
            vec![("mine".to_owned(), "Mine, really".to_owned()), ("zed".to_owned(), "zed".to_owned())]
        );
        let all = choices(&dir, "gone");
        assert_eq!(all[0].0, SYSTEM);
        assert_eq!(all.len(), BUILT_IN.len() + 3);
        assert_eq!(all.last().unwrap(), &("gone".to_owned(), "gone".to_owned()));
        assert!(third_party(&dir.join("nowhere")).is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_name_comment_is_optional_and_trimmed() {
        assert_eq!(name_of(b"/* name: Mine */"), Some("Mine".to_owned()));
        assert_eq!(name_of(b"/*name:  Two words  */ :root{}"), Some("Two words".to_owned()));
        assert_eq!(name_of(b"/* Mine */"), None);
        assert_eq!(name_of(b":root { --bg: #000 }"), None);
        assert_eq!(name_of(b"/* name: */"), None);
        let long = format!("/* name: {} */", "n".repeat(100));
        assert_eq!(name_of(long.as_bytes()).unwrap().len(), MAX_NAME);
    }
}
