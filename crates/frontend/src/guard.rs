//! Settings the program does not take on trust (D70). A torrent client that quietly falls back to
//! a direct connection where the user had a proxy shows the user's address to every peer and
//! tracker, so:
//!
//! * a preferences file that is there but cannot be read ([`LoadProblem::Unreadable`]), and a
//!   proxy type this build cannot read ([`HOLD_ROWS`]), start the engine with the transfers
//!   held (`SessionParams::hold_transfers`) until the user confirms the settings: Preferences'
//!   OK or Apply in a GUI, a save in the Web UI. The Logger says why, the status bar says
//!   "Transfers held" (`NetworkStatus::Held`);
//! * the lines this build cannot read stay in the file until then ([`settings::store`]), so a
//!   newer build that wrote them finds them again;
//! * every save stamps the file with `check.version`, the version that wrote it, and
//!   `check.guard`, how that version read the settings that decide where traffic goes
//!   ([`summary`]). A start compares the stamp with its own reading of the same file: every
//!   difference goes to the Logger, and one that loses a proxy, a network rule or the listen
//!   interfaces holds the transfers. Both readings come from the builds themselves, so a newer
//!   build that reads an old key differently is caught without knowing the old code. After a
//!   version change the file is first copied to `<name>.<old version>.bak` (the last
//!   [`BACKUPS`] are kept).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rookparcel_core::util::private_file;
use rookparcel_core::{EncLevel, EncPolicy, ProxyType};

use crate::prefs::{LoadProblem, Prefs, Stamp};
use crate::settings;
use crate::tr;

/// The settings rows that hold the transfers while this build cannot read their line: read as
/// its default, the proxy type is "none", a direct connection.
pub const HOLD_ROWS: &[&str] = &["proxy_type"];

/// The stamp's keys: the version that wrote the file, and its [`summary`].
pub const VERSION_KEY: &str = "check.version";
pub const GUARD_KEY: &str = "check.guard";

/// The [`summary`] fields whose change holds the transfers (when they had a value before):
/// the ones that decide which network torrent traffic leaves through.
const HOLD_FIELDS: &[&str] = &["proxy", "rule", "listen"];

/// How many `.bak` copies of the file a version change keeps.
pub const BACKUPS: usize = 3;

/// What freshly loaded preferences ask for at start.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StartCheck {
    /// Start the engine with the transfers held.
    pub hold: bool,
    /// Logger lines saying what was found, in order.
    pub log: Vec<String>,
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Looks at `prefs` as loaded from their file, before anything is written to them or the engine
/// starts, and turns on their stamp ([`stamp_lines`]).
pub fn on_start(prefs: &mut Prefs) -> StartCheck {
    let file = prefs.path().map(Path::to_path_buf);
    on_start_for(prefs, file.as_deref())
}

/// [`on_start`] for preferences kept in another file (nox's `webui.json`): `file` is what a
/// version change copies to a `.bak`, `None` for no copy.
pub fn on_start_for(prefs: &mut Prefs, file: Option<&Path>) -> StartCheck {
    let mut c = StartCheck::default();
    let name = prefs.path().map(|p| p.display().to_string()).unwrap_or_default();
    match prefs.problem() {
        Some(LoadProblem::Unreadable(error)) => {
            // whatever the file says about a proxy is unknown
            c.hold = true;
            c.log.push(tr!(LogSettingsUnreadable, file = name, error));
        }
        Some(LoadProblem::Damaged { copy: Some(copy) }) => {
            c.log.push(tr!(LogSettingsDamaged, file = name, copy = copy.display()));
        }
        Some(LoadProblem::Damaged { copy: None }) => c.log.push(tr!(LogSettingsNoCopy, file = name)),
        None => {}
    }
    for (e, value) in settings::unreadable(prefs) {
        c.log.push(tr!(LogSettingUnreadable, name = settings::pref_key(e.name), value));
        c.hold |= HOLD_ROWS.contains(&e.name);
    }
    prefs.set_stamp(Stamp::On);
    let old = prefs.get(VERSION_KEY).map(str::to_owned);
    if let (Some(old), Some(file)) = (&old, file) {
        if old != version() && prefs.problem().is_none() {
            if let Some(copy) = backup(file, old) {
                c.log.push(tr!(LogSettingsBackup, old, copy = copy.display()));
            }
        }
    }
    let stamped = prefs.get(GUARD_KEY).map(str::to_owned);
    if let (Some(old), Some(before)) = (old, stamped.as_deref().and_then(fields)) {
        let now_text = summary(prefs);
        let now = fields(&now_text).unwrap_or_default();
        let changed: Vec<(&str, &str, &str)> = before
            .iter()
            .filter_map(|(&k, &b)| {
                let n = now.get(k).copied().unwrap_or("");
                (b != n).then_some((k, b, n))
            })
            .collect();
        if !changed.is_empty() {
            c.log.push(tr!(LogSettingsChanged, old));
            let mut hold = false;
            for (field, before, now) in changed {
                c.log.push(tr!(LogSettingChanged, field, before, now));
                hold |= HOLD_FIELDS.contains(&field) && before != "none";
            }
            if hold {
                // the next start compares again until the user confirms
                c.hold = true;
                prefs.set_stamp(Stamp::Kept);
            }
        }
    }
    if c.hold {
        c.log.push(tr!(LogSettingsHeld).to_owned());
    }
    c
}

/// The user confirmed the settings (after they were stored): the lines this build cannot read
/// go, and the next save stamps this version's reading. The caller lifts the hold
/// (`SessionHandle::hold_transfers(false)`).
pub fn confirm(prefs: &mut Prefs) {
    settings::forget_unreadable(prefs);
    if prefs.stamp() == Stamp::Kept {
        prefs.set_stamp(Stamp::On);
    }
}

/// The stamp a save writes over the file's own (`Prefs::save`, nox's state file): this
/// version and its [`summary`] once [`on_start`] turned it on; nothing before that, nor while
/// a difference awaits the user's confirmation (the file's stamp stays).
pub fn stamp_lines(prefs: &Prefs) -> Vec<(&'static str, String)> {
    match prefs.stamp() {
        Stamp::On => vec![(VERSION_KEY, version().to_owned()), (GUARD_KEY, summary(prefs))],
        Stamp::Off | Stamp::Kept => Vec::new(),
    }
}

/// The settings that decide where traffic goes, as this build reads `prefs`, in the frozen `v1`
/// form every version writes and reads:
///
/// ```text
/// v1; proxy=socks5 10.8.0.1:1080 peers trackers names; rule=vpn wait; listen=0.0.0.0:6881,[::]:6881; enc=enabled both; ipfilter=off
/// ```
///
/// `proxy` is `none` or the type (`socks4`, `socks5`, `socks5_pw`, `http`, `http_pw`), the host
/// and port and the three scopes (`no-peers`... when off); `rule` is `none` or the network rule
/// with `wait` or `idle`; `enc` is the outgoing policy (`forced`, `enabled`, `disabled`) and the
/// allowed level (`plaintext`, `rc4`, `both`). Never change this form: a new field means a `v2`
/// written beside it. It holds no secret: no user name, no password.
pub fn summary(prefs: &Prefs) -> String {
    let s = settings::load(prefs);
    let clean = |v: &str| v.trim().replace(';', ",");
    let scope = |on: bool, name: &str| if on { name.to_owned() } else { format!("no-{name}") };
    let proxy = match proxy_name(s.proxy_type) {
        None => "none".to_owned(),
        Some(kind) => format!(
            "{kind} {}:{} {} {} {}",
            clean(&s.proxy_hostname),
            s.proxy_port,
            scope(s.proxy_peer_connections, "peers"),
            scope(s.proxy_tracker_connections, "trackers"),
            scope(s.proxy_hostnames, "names")
        ),
    };
    let rule = match clean(&s.network_rule) {
        r if r.is_empty() => "none".to_owned(),
        r => format!("{r} {}", if s.network_pause_when_missing { "wait" } else { "idle" }),
    };
    let policy = match s.out_enc_policy {
        EncPolicy::Forced => "forced",
        EncPolicy::Enabled => "enabled",
        EncPolicy::Disabled => "disabled",
    };
    let level = match s.allowed_enc_level {
        EncLevel::Plaintext => "plaintext",
        EncLevel::Rc4 => "rc4",
        EncLevel::Both => "both",
    };
    let ipfilter = if prefs.get_bool("ipfilter.enable", false) { "on" } else { "off" };
    format!(
        "v1; proxy={proxy}; rule={rule}; listen={}; enc={policy} {level}; ipfilter={ipfilter}",
        clean(&s.listen_interfaces)
    )
}

fn proxy_name(t: ProxyType) -> Option<&'static str> {
    match t {
        ProxyType::None => None,
        ProxyType::Socks4 => Some("socks4"),
        ProxyType::Socks5 => Some("socks5"),
        ProxyType::Socks5Pw => Some("socks5_pw"),
        ProxyType::Http => Some("http"),
        ProxyType::HttpPw => Some("http_pw"),
    }
}

/// A `v1` [`summary`]'s fields by name; `None` for anything else.
fn fields(summary: &str) -> Option<BTreeMap<&str, &str>> {
    let mut parts = summary.split("; ");
    (parts.next()? == "v1").then(|| parts.filter_map(|p| p.split_once('=')).collect())
}

/// Copies `file` to `<name>.<old>.bak` beside it, unless that copy is there already, and keeps
/// the newest [`BACKUPS`] such copies. The copy's path, when one was made.
fn backup(file: &Path, old: &str) -> Option<PathBuf> {
    let name = file.file_name()?.to_string_lossy().into_owned();
    // the version comes from the file: nothing in it may reach outside the folder
    let old: String = old
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '_' })
        .collect();
    let copy = file.with_file_name(format!("{name}.{old}.bak"));
    if copy.exists() {
        return None;
    }
    let bytes = std::fs::read(file).ok()?;
    private_file::write(&copy, &bytes).ok()?;
    prune(file, &name);
    Some(copy)
}

/// Removes all but the newest [`BACKUPS`] `<name>.*.bak` copies beside `file`.
fn prune(file: &Path, name: &str) {
    let Some(dir) = file.parent() else { return };
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let prefix = format!("{name}.");
    let mut copies: Vec<(std::time::SystemTime, PathBuf)> = entries
        .filter_map(Result::ok)
        .filter(|e| {
            let n = e.file_name().to_string_lossy().into_owned();
            n.starts_with(&prefix) && n.ends_with(".bak")
        })
        .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
        .collect();
    copies.sort_by_key(|c| std::cmp::Reverse(c.0));
    for (_, path) in copies.into_iter().skip(BACKUPS) {
        let _ = std::fs::remove_file(path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rp-guard-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn an_unreadable_proxy_type_holds_the_transfers() {
        assert_eq!(on_start(&mut Prefs::from_text("lt.proxy_hostname=h\n")), StartCheck::default());

        let mut prefs = Prefs::from_text("lt.connections_limit=lots\n");
        let c = on_start(&mut prefs);
        assert!(!c.hold, "only the rows that decide where traffic goes hold it");
        assert_eq!(c.log.len(), 1);
        assert!(c.log[0].contains("lt.connections_limit=lots"), "{:?}", c.log);

        prefs.set("lt.proxy_type", "6");
        let c = on_start(&mut prefs);
        assert!(c.hold);
        assert_eq!(c.log.len(), 3, "{:?}", c.log);
        assert!(c.log[1].contains("lt.proxy_type=6"), "{:?}", c.log);

        confirm(&mut prefs);
        assert_eq!(on_start(&mut prefs), StartCheck::default());
    }

    #[test]
    fn a_file_that_cannot_be_read_holds_the_transfers() {
        let dir = temp("unreadable");
        // a folder where the file should be cannot be read
        std::fs::create_dir_all(dir.join("rookparcel.ini")).unwrap();
        let c = on_start(&mut Prefs::load(dir.join("rookparcel.ini")));
        assert!(c.hold);
        assert_eq!(c.log.len(), 2, "{:?}", c.log);
        // not valid UTF-8: read, a copy kept, no hold
        std::fs::write(dir.join("other.ini"), b"ui.toolbar=0\nx=\xff\n").unwrap();
        let c = on_start(&mut Prefs::load(dir.join("other.ini")));
        assert!(!c.hold);
        assert!(c.log[0].contains("other.ini.damaged-"), "{:?}", c.log);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_summary_names_where_traffic_goes_and_no_secret() {
        let prefs = Prefs::from_text(
            "lt.proxy_type=3\nlt.proxy_hostname=10.8.0.1\nlt.proxy_port=1080\nlt.proxy_username=me\n\
             lt.proxy_password=s3cret\nlt.proxy_tracker_connections=false\nnet.network_rule=vpn\n\
             lt.out_enc_policy=0\nipfilter.enable=1\n",
        );
        let s = summary(&prefs);
        assert_eq!(
            s,
            "v1; proxy=socks5_pw 10.8.0.1:1080 peers no-trackers names; rule=vpn wait; \
             listen=0.0.0.0:6881,[::]:6881; enc=forced both; ipfilter=on"
        );
        assert!(!s.contains("me ") && !s.contains("s3cret"));
        assert_eq!(
            summary(&Prefs::default()),
            "v1; proxy=none; rule=none; listen=0.0.0.0:6881,[::]:6881; enc=enabled both; ipfilter=off"
        );
        assert_eq!(fields(&s).unwrap().get("rule"), Some(&"vpn wait"));
        assert_eq!(fields("v2; proxy=none"), None);
    }

    /// The stamp a save writes, read back by the next start: the same reading is quiet; a newer
    /// build's reading that loses the proxy holds the transfers until the user confirms, and the
    /// file keeps the old stamp meanwhile; a version change keeps a backup.
    #[test]
    fn a_different_reading_of_the_stamped_settings_holds_until_confirmed() {
        let dir = temp("stamp");
        let path = dir.join("rookparcel.ini");
        std::fs::write(&path, "lt.proxy_type=2\nlt.proxy_hostname=10.8.0.1\nlt.proxy_port=1080\n").unwrap();
        let mut prefs = Prefs::load(&path);
        assert_eq!(on_start(&mut prefs), StartCheck::default(), "no stamp yet: nothing to compare");
        prefs.save().unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains(&format!("check.version={}\n", version())), "{text}");
        assert!(text.contains("check.guard=v1; proxy=socks5 10.8.0.1:1080 peers trackers names;"), "{text}");
        assert_eq!(on_start(&mut Prefs::load(&path)), StartCheck::default(), "the same reading");

        // an older version wrote the stamp, and this one reads the proxy differently (as
        // qBittorrent's update did): the stamp says SOCKS5, the line now reads as none
        let edited = text
            .replace(&format!("check.version={}", version()), "check.version=1.0.0")
            .replace("lt.proxy_type=2", "lt.proxy_type=0");
        std::fs::write(&path, &edited).unwrap();
        let mut prefs = Prefs::load(&path);
        let c = on_start(&mut prefs);
        assert!(c.hold, "{:?}", c.log);
        assert!(c.log[0].contains("1.0.0") && c.log[0].contains("rookparcel.ini.1.0.0.bak"), "{:?}", c.log);
        assert!(
            c.log.iter().any(|l| l.contains("proxy: socks5 10.8.0.1:1080 peers trackers names → none")),
            "{:?}",
            c.log
        );
        assert_eq!(std::fs::read_to_string(dir.join("rookparcel.ini.1.0.0.bak")).unwrap(), edited);
        // saved before the user confirms: the old stamp stays, so the next start asks again
        prefs.set("ui.toolbar", "0");
        prefs.save().unwrap();
        assert!(std::fs::read_to_string(&path).unwrap().contains("check.version=1.0.0"));
        let again = on_start(&mut Prefs::load(&path));
        assert!(again.hold && !again.log.iter().any(|l| l.contains(".bak")), "one backup per version");
        // confirmed: this version's reading is stamped, and the next start is quiet
        confirm(&mut prefs);
        prefs.save().unwrap();
        assert_eq!(on_start(&mut Prefs::load(&path)), StartCheck::default());

        // a difference that changes no network (the IP filter) is logged, not held
        let text = std::fs::read_to_string(&path).unwrap();
        std::fs::write(&path, format!("{text}ipfilter.enable=1\n")).unwrap();
        let c = on_start(&mut Prefs::load(&path));
        assert!(!c.hold && c.log.len() == 2, "{:?}", c.log);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn backups_keep_the_newest_three_and_stay_in_the_folder() {
        let dir = temp("backups");
        let file = dir.join("rookparcel.ini");
        std::fs::write(&file, "a=1\n").unwrap();
        for v in ["1.0.0", "1.1.0", "1.2.0", "1.3.0"] {
            assert!(backup(&file, v).is_some());
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        assert!(backup(&file, "1.3.0").is_none(), "one copy per version");
        let mut names: Vec<String> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        assert_eq!(
            names,
            [
                "rookparcel.ini",
                "rookparcel.ini.1.1.0.bak",
                "rookparcel.ini.1.2.0.bak",
                "rookparcel.ini.1.3.0.bak"
            ]
        );
        // a version from the file never names a path
        assert_eq!(backup(&file, "../../x").unwrap(), dir.join("rookparcel.ini..._.._x.bak"));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
