//! Settings written by older versions still read the same (D70). Each `<version>.ini` in
//! `tests/fixtures/settings` is a preferences file with every setting away from its default, as
//! that version wrote it, and `<version>.expect` is what that version read from it: the
//! settings rows, the engine's fields by what they mean (`proxy_type=Socks5`) and the GUI keys.
//! Every later version must read the same: a renamed key, or a number read as another proxy
//! type, without a migration fails here, before a release, instead of resetting a user's proxy
//! after an update. `legacy-keys.ini` is hand-written: keys that existing migrations handle.
//!
//! When this build writes the settings differently from the newest fixture (a new setting, a
//! renamed key, another format), `the_newest_fixture_is_this_builds_format` fails. Then run
//!
//! ```text
//! ROOKPARCEL_WRITE_FIXTURE=1 cargo test -p rookparcel-frontend --test settings_fixtures
//! ```
//!
//! to add this version's fixture, and give an old key a migration (as `seeding::migrate` does)
//! so the older fixtures still pass. A version that leaves the settings alone costs nothing.

use std::collections::BTreeMap;
use std::path::PathBuf;

use rookparcel_core::Settings;
use rookparcel_frontend::options::{GuiOptions, Options};
use rookparcel_frontend::prefs::Prefs;
use rookparcel_frontend::settings::{Entry, Kind, TABLE, Value};

/// The default download folder the fixtures are read with.
const DOWNLOADS: &str = "/fixture/Downloads";

/// GUI keys that are absent while at their defaults, so the defaults do not list them.
const ABSENT_AT_DEFAULT: &[(&str, &str)] = &[
    ("dir.completed", "/fixture/Completed"),
    ("dir.torrents", "/fixture/Torrents"),
    ("dir.finished_torrents", "/fixture/Finished torrents"),
    ("dir.autoload_dir", "/fixture/Autoload"),
    ("ipfilter.path", "/fixture/ipfilter.dat"),
    ("ui.language", "de"),
    ("ui.theme", "dark"),
    ("trackers.list", "udp://tracker.example:1337/announce"),
    ("trackers.url", "https://example.org/trackers.txt"),
    ("run.added", "/fixture/added.sh %F"),
    ("run.finished", "/fixture/finished.sh %F"),
];

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("settings")
}

/// The engine rows the preferences keep (the composed µTorrent `bt.` rows are views of others).
fn rows() -> impl Iterator<Item = &'static Entry> {
    TABLE.iter().filter(|e| !e.name.starts_with("bt."))
}

/// A text for `e` that is not its default.
fn away_from_default(e: &Entry) -> String {
    match (e.kind, e.get(&Settings::default())) {
        (Kind::Bool, Value::Bool(b)) => (!b).to_string(),
        (Kind::Int { max, .. }, Value::Int(i)) => (if i < max { i + 1 } else { i - 1 }).to_string(),
        (_, v) => format!("{v}fixture"),
    }
}

/// `prefs` as `key=value` lines, sorted, with the plain values of the sealed keys (a sealed
/// value differs at every sealing).
fn plain_text(prefs: &Prefs) -> String {
    prefs.entries().map(|(k, _)| format!("{k}={}\n", prefs.get(k).unwrap_or_default())).collect()
}

/// This build's preferences with every setting it can move away from its default.
fn every_setting_away_from_default() -> Prefs {
    let mut s = Settings::default();
    for e in rows() {
        let text = away_from_default(e);
        e.set(&mut s, &text).unwrap_or_else(|why| panic!("{}: {text}: {why}", e.name));
    }
    let mut defaults = Prefs::default();
    GuiOptions::load(&Prefs::default(), DOWNLOADS).store(&mut defaults);
    let mut gui = Prefs::default();
    for (k, v) in defaults.entries() {
        let v = match v {
            "0" => "1".to_owned(),
            "1" => "0".to_owned(),
            v => v.parse::<i64>().map_or_else(|_| format!("{v}fixture"), |i| (i + 1).to_string()),
        };
        gui.set(k, v);
    }
    for (k, v) in ABSENT_AT_DEFAULT {
        gui.set(k, v);
    }
    let mut prefs = Prefs::default();
    Options { settings: s, gui: GuiOptions::load(&gui, DOWNLOADS), integration: Default::default() }
        .store(&mut prefs);
    prefs
}

/// What this build reads from `prefs` at a start (the migrations, then the options): the engine
/// rows by name, the engine's fields by what they mean, the GUI keys as stored again. With
/// `away`, only what is away from this build's defaults (an `.expect` file's lines); without,
/// everything, so a value kept while a later build changed the default still compares.
fn read(mut prefs: Prefs, away: bool) -> BTreeMap<String, String> {
    rookparcel_frontend::seeding::migrate(&mut prefs);
    let o = Options::load(&prefs, DOWNLOADS);
    let mut m = BTreeMap::new();
    let defaults = Settings::default();
    for e in rows() {
        let v = e.get(&o.settings);
        if !away || v != e.get(&defaults) {
            m.insert(format!("engine {}", e.name), v.to_string());
        }
    }
    // the engine's own fields too, by what they mean: a number read as another proxy type
    // (the row's text unchanged) shows here
    let default_fields = fields(&defaults);
    for (k, v) in fields(&o.settings) {
        if !away || default_fields.get(&k) != Some(&v) {
            m.insert(format!("field {k}"), v);
        }
    }
    let mut default_gui = Prefs::default();
    GuiOptions::load(&Prefs::default(), DOWNLOADS).store(&mut default_gui);
    let mut gui = Prefs::default();
    o.gui.store(&mut gui);
    for (k, _) in gui.entries() {
        let v = gui.get(k).unwrap_or_default();
        if !away || default_gui.get(k) != Some(v) {
            m.insert(format!("gui {k}"), v.to_owned());
        }
    }
    m
}

/// The engine settings' fields as `Debug` prints them (`proxy_type: Socks5`), one per field.
fn fields(s: &Settings) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    let mut current: Option<(String, String)> = None;
    for line in format!("{s:#?}").lines().skip(1) {
        let top = line.strip_prefix("    ").filter(|rest| !rest.starts_with(' '));
        if let Some((k, v)) = top.and_then(|rest| rest.split_once(": ")) {
            if let Some((k, v)) = current.take() {
                m.insert(k, v);
            }
            current = Some((k.to_owned(), v.trim_end_matches(',').to_owned()));
        } else if let Some((_, v)) = current.as_mut().filter(|_| line != "}") {
            // a field written over several lines
            v.push(' ');
            v.push_str(line.trim());
        }
    }
    if let Some((k, v)) = current {
        m.insert(k, v);
    }
    m
}

fn expect_text(m: &BTreeMap<String, String>) -> String {
    m.iter().map(|(k, v)| format!("{k}={v}\n")).collect()
}

fn parse_expect(text: &str) -> BTreeMap<String, String> {
    text.lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter_map(|l| l.split_once('='))
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect()
}

/// The fixtures by version, oldest first; the ones not named by a version (hand-written old
/// formats) under `None`.
fn all_fixtures() -> Vec<(Option<Vec<u32>>, PathBuf)> {
    let mut out: Vec<(Option<Vec<u32>>, PathBuf)> = std::fs::read_dir(fixtures())
        .map(|d| d.filter_map(Result::ok).map(|e| e.path()).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        .filter(|p| p.extension().is_some_and(|x| x == "ini"))
        .map(|p| {
            let stem = p.file_stem().unwrap_or_default().to_string_lossy().into_owned();
            let version: Option<Vec<u32>> = stem.split('.').map(|n| n.parse().ok()).collect();
            (version, p)
        })
        .collect();
    out.sort();
    out
}

fn read_fixture(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Every engine row can be moved away from its default and is read back so, through the file:
/// a setting that does not survive its own build's file fails here.
#[test]
fn every_setting_survives_the_file() {
    let prefs = every_setting_away_from_default();
    let back = read(Prefs::from_text(&plain_text(&prefs)), true);
    let lost: Vec<&str> =
        rows().map(|e| e.name).filter(|n| !back.contains_key(&format!("engine {n}"))).collect();
    assert!(lost.is_empty(), "these settings come back at their defaults: {lost:?}");
    let gui_lost: Vec<&str> = ABSENT_AT_DEFAULT
        .iter()
        .map(|(k, _)| *k)
        .filter(|k| !back.contains_key(&format!("gui {k}")))
        .collect();
    assert!(gui_lost.is_empty(), "these GUI keys are lost: {gui_lost:?}");
}

/// The newest fixture is what this build writes; if not, the format changed (see the module).
#[test]
fn the_newest_fixture_is_this_builds_format() {
    let version = env!("CARGO_PKG_VERSION");
    let text = format!(
        "# Written by Rookparcel {version} (tests/settings_fixtures.rs).\n{}",
        plain_text(&every_setting_away_from_default())
    );
    let newest = all_fixtures().into_iter().filter_map(|(v, p)| v.map(|_| p)).next_back();
    let same = newest.as_ref().is_some_and(|p| {
        // the comment line names the version that wrote it
        let old = read_fixture(p);
        old.lines().skip(1).eq(text.lines().skip(1))
    });
    if same {
        return;
    }
    if std::env::var_os("ROOKPARCEL_WRITE_FIXTURE").is_some() {
        std::fs::create_dir_all(fixtures()).unwrap();
        let ini = fixtures().join(format!("{version}.ini"));
        std::fs::write(&ini, &text).unwrap();
        let expect = read(Prefs::from_text(&text), true);
        let head = format!("# What Rookparcel {version} reads from {version}.ini.\n");
        std::fs::write(fixtures().join(format!("{version}.expect")), head + &expect_text(&expect)).unwrap();
        return;
    }
    panic!(
        "this build writes the settings differently from {}: run \
         ROOKPARCEL_WRITE_FIXTURE=1 cargo test -p rookparcel-frontend --test settings_fixtures \
         to add {version}.ini, and give a renamed or changed key a migration so the older \
         fixtures still read the same",
        newest.map_or_else(|| "no fixture".to_owned(), |p| p.display().to_string())
    );
}

/// Every fixture, the hand-written old formats included, reads as its version read it.
#[test]
fn older_settings_read_the_same() {
    let all = all_fixtures();
    assert!(!all.is_empty(), "no fixtures in {}", fixtures().display());
    let mut failures = Vec::new();
    for (_, ini) in all {
        let expect_path = ini.with_extension("expect");
        let want = parse_expect(&read_fixture(&expect_path));
        // every value that version read away from its defaults must read the same now; the
        // rest were at their defaults then, and a later build may change a default
        let have = read(Prefs::from_text(&read_fixture(&ini)), false);
        let mut diff = Vec::new();
        for (k, v) in &want {
            if have.get(k) != Some(v) {
                diff.push(format!("  {k}: then {v:?}, now {:?}", have.get(k)));
            }
        }
        if !diff.is_empty() {
            failures.push(format!("{}:\n{}", ini.display(), diff.join("\n")));
        }
    }
    assert!(
        failures.is_empty(),
        "settings read differently from how their version read them:\n{}",
        failures.join("\n")
    );
}
