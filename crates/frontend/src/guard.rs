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
//!   newer build that wrote them finds them again.

use crate::prefs::{LoadProblem, Prefs};
use crate::settings;
use crate::tr;

/// The settings rows that hold the transfers while this build cannot read their line: read as
/// its default, the proxy type is "none", a direct connection.
pub const HOLD_ROWS: &[&str] = &["proxy_type"];

/// What freshly loaded preferences ask for at start.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StartCheck {
    /// Start the engine with the transfers held.
    pub hold: bool,
    /// Logger lines saying what was found, in order.
    pub log: Vec<String>,
}

/// Looks at `prefs` as loaded, before anything is written to them or the engine starts.
pub fn on_start(prefs: &Prefs) -> StartCheck {
    let mut c = StartCheck::default();
    let file = prefs.path().map(|p| p.display().to_string()).unwrap_or_default();
    match prefs.problem() {
        Some(LoadProblem::Unreadable(error)) => {
            // whatever the file says about a proxy is unknown
            c.hold = true;
            c.log.push(tr!(LogSettingsUnreadable, file, error));
        }
        Some(LoadProblem::Damaged { copy: Some(copy) }) => {
            c.log.push(tr!(LogSettingsDamaged, file, copy = copy.display()));
        }
        Some(LoadProblem::Damaged { copy: None }) => c.log.push(tr!(LogSettingsNoCopy, file)),
        None => {}
    }
    for (e, value) in settings::unreadable(prefs) {
        c.log.push(tr!(LogSettingUnreadable, name = settings::pref_key(e.name), value));
        c.hold |= HOLD_ROWS.contains(&e.name);
    }
    if c.hold {
        c.log.push(tr!(LogSettingsHeld).to_owned());
    }
    c
}

/// The user confirmed the settings (after they were stored): the lines this build cannot read
/// go. The caller lifts the hold (`SessionHandle::hold_transfers(false)`).
pub fn confirm(prefs: &mut Prefs) {
    settings::forget_unreadable(prefs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unreadable_proxy_type_holds_the_transfers() {
        assert_eq!(on_start(&Prefs::from_text("lt.proxy_hostname=h\n")), StartCheck::default());

        let mut prefs = Prefs::from_text("lt.connections_limit=lots\n");
        let c = on_start(&prefs);
        assert!(!c.hold, "only the rows that decide where traffic goes hold it");
        assert_eq!(c.log.len(), 1);
        assert!(c.log[0].contains("lt.connections_limit=lots"), "{:?}", c.log);

        prefs.set("lt.proxy_type", "6");
        let c = on_start(&prefs);
        assert!(c.hold);
        assert_eq!(c.log.len(), 3, "{:?}", c.log);
        assert!(c.log[1].contains("lt.proxy_type=6"), "{:?}", c.log);

        confirm(&mut prefs);
        assert_eq!(on_start(&prefs), StartCheck::default());
    }

    #[test]
    fn a_file_that_cannot_be_read_holds_the_transfers() {
        let dir = std::env::temp_dir().join(format!("rp-guard-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        // a folder where the file should be cannot be read
        std::fs::create_dir_all(dir.join("rookparcel.ini")).unwrap();
        let c = on_start(&Prefs::load(dir.join("rookparcel.ini")));
        assert!(c.hold);
        assert_eq!(c.log.len(), 2, "{:?}", c.log);
        // not valid UTF-8: read, a copy kept, no hold
        std::fs::write(dir.join("other.ini"), b"ui.toolbar=0\nx=\xff\n").unwrap();
        let c = on_start(&Prefs::load(dir.join("other.ini")));
        assert!(!c.hold);
        assert!(c.log[0].contains("other.ini.damaged-"), "{:?}", c.log);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
