//! The desktop windows' lock (D69, as qBittorrent's "Lock qBittorrent"): a PIN of 4 to 6 digits
//! hides every window of the GUI in the tray, and showing them again asks for it. It keeps the
//! torrent list from someone at the same computer; it is not a password for the data, and
//! editing the settings file removes it (the owner's terms).
//!
//! What is kept, in the window's own settings file (`rookparcel.ini`, so each profile and each
//! remote window has its own): `ui.lock_pin`, the PIN as a salted PBKDF2 hash
//! ([`PasswordHash`], the Web UI password's format), and `ui.locked`, set while the windows are
//! locked, so a locked Rookparcel starts locked. Neither is a [`crate::options::GuiOptions`]
//! field: Preferences never shows or writes them, and the Web UI a Windows window hosts never
//! gets them ([`without_lock_keys`]).
//!
//! A forgotten PIN is removed by starting the program with [`RESET_SWITCH`] or, on Windows and
//! macOS, with Ctrl or Option held; each GUI asks first.

use std::time::{Duration, Instant};

use rookparcel_core::crypto::password::{DEFAULT_ITERATIONS, PasswordHash};

use crate::prefs::{Change, Prefs};
use crate::tr;

/// The PIN's hash: `pbkdf2-sha256:<iterations>:<salt>:<hash>`.
pub const PIN_KEY: &str = "ui.lock_pin";
/// `1` while the windows are locked (no key otherwise).
pub const LOCKED_KEY: &str = "ui.locked";
/// The command-line switch that asks to remove the PIN.
pub const RESET_SWITCH: &str = "--reset-lock";

/// A PIN's length in digits.
pub const MIN_DIGITS: usize = 4;
pub const MAX_DIGITS: usize = 6;

/// Wrong PINs allowed before each further one starts a wait.
pub const FREE_TRIES: u32 = 4;
/// The wait after too many wrong PINs.
pub const WAIT: Duration = Duration::from_secs(30);

/// Whether `pin` can be a PIN: 4 to 6 ASCII digits.
pub fn valid_pin(pin: &str) -> bool {
    (MIN_DIGITS..=MAX_DIGITS).contains(&pin.len()) && pin.bytes().all(|b| b.is_ascii_digit())
}

/// Whether `arg` is [`RESET_SWITCH`] (in any case).
pub fn is_reset_switch(arg: &str) -> bool {
    arg.eq_ignore_ascii_case(RESET_SWITCH)
}

/// Whether the command line asks to remove the PIN.
pub fn reset_requested<S: AsRef<str>>(args: &[S]) -> bool {
    args.iter().any(|a| is_reset_switch(a.as_ref()))
}

fn stored(prefs: &Prefs) -> Option<PasswordHash> {
    prefs.get(PIN_KEY).and_then(PasswordHash::parse)
}

/// Whether a usable PIN is set (a hash that does not parse is no PIN).
pub fn has_pin(prefs: &Prefs) -> bool {
    stored(prefs).is_some()
}

/// PBKDF2 rounds for a new PIN: the Web UI password's count (this crate's own tests use a few,
/// since a debug build takes seconds for the full count; a stored hash names its count).
const PIN_ROUNDS: u32 = if cfg!(test) { 10 } else { DEFAULT_ITERATIONS };

/// Sets the PIN (the caller checked it with [`valid_pin`]). The caller saves.
pub fn set_pin(prefs: &mut Prefs, pin: &str) {
    set_pin_with(prefs, pin, PIN_ROUNDS);
}

/// [`set_pin`] with a given number of PBKDF2 rounds, for other crates' tests and test runs (a
/// debug build takes seconds for the full count).
#[doc(hidden)]
pub fn set_pin_with(prefs: &mut Prefs, pin: &str, iterations: u32) {
    prefs.set(PIN_KEY, PasswordHash::new(pin, iterations));
}

/// Whether `pin` is the PIN. False without a PIN, and for anything that cannot be one.
pub fn verify(prefs: &Prefs, pin: &str) -> bool {
    valid_pin(pin) && stored(prefs).is_some_and(|h| h.verify(pin))
}

/// Removes the PIN and the locked mark. True when there was a PIN. The caller saves.
pub fn remove(prefs: &mut Prefs) -> bool {
    let had = has_pin(prefs);
    prefs.remove(PIN_KEY);
    prefs.remove(LOCKED_KEY);
    had
}

/// Whether the windows are locked: the mark is set and there is a PIN to unlock with (a mark
/// without one, from a settings file edited by hand, does not lock).
pub fn is_locked(prefs: &Prefs) -> bool {
    prefs.get_bool(LOCKED_KEY, false) && has_pin(prefs)
}

/// Sets or clears the locked mark. The caller saves (at once: a lock must survive a crash).
pub fn set_locked(prefs: &mut Prefs, locked: bool) {
    if locked {
        prefs.set_bool(LOCKED_KEY, true);
    } else {
        prefs.remove(LOCKED_KEY);
    }
}

/// Locks if there is a PIN; false (and nothing changed) without one, when the GUI asks for a
/// PIN first.
pub fn lock(prefs: &mut Prefs) -> bool {
    let ok = has_pin(prefs);
    if ok {
        set_locked(prefs, true);
    }
    ok
}

/// Whether a preferences key is one of the lock's.
pub fn is_lock_key(key: &str) -> bool {
    key == PIN_KEY || key == LOCKED_KEY
}

/// `changes` without the lock's keys: what may pass between the window's preferences and the
/// Web UI it hosts, in either direction.
pub fn without_lock_keys(changes: Vec<Change>) -> Vec<Change> {
    changes.into_iter().filter(|(k, _)| !is_lock_key(k)).collect()
}

/// What a PIN check says.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Check {
    Right,
    Wrong,
    /// Too many wrong PINs: none is tried for this many more seconds.
    Wait(u64),
}

/// Counts wrong PINs, in memory (a restart forgets them, which the lock's terms allow): from
/// the fifth on, each starts a [`WAIT`] in which no PIN is even tried. A right PIN clears it.
#[derive(Debug, Default)]
pub struct Guard {
    fails: u32,
    until: Option<Instant>,
}

impl Guard {
    /// The seconds left of a wait, rounded up; `None` when a PIN may be tried.
    pub fn wait_left(&self, now: Instant) -> Option<u64> {
        let left = self.until?.checked_duration_since(now).filter(|d| !d.is_zero())?;
        Some(left.as_secs() + u64::from(left.subsec_nanos() > 0))
    }

    /// Checks `pin` against the PIN.
    pub fn check(&mut self, prefs: &Prefs, pin: &str, now: Instant) -> Check {
        if let Some(secs) = self.wait_left(now) {
            return Check::Wait(secs);
        }
        if verify(prefs, pin) {
            *self = Guard::default();
            return Check::Right;
        }
        self.fails = self.fails.saturating_add(1);
        if self.fails > FREE_TRIES {
            self.until = Some(now + WAIT);
            return Check::Wait(WAIT.as_secs());
        }
        Check::Wrong
    }
}

/// The PIN dialogs. Each GUI lays out the boxes [`Form::asks_current`] and [`Form::asks_new`]
/// name, and hands what was typed to [`submit`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Form {
    /// "Rookparcel is locked": the PIN, to show the windows again.
    Unlock,
    /// Options > Lock PIN > Set PIN... (or File > Lock Rookparcel without a PIN): the new PIN
    /// twice.
    Set,
    /// Change PIN...: the current PIN, then the new one twice.
    Change,
    /// Remove PIN...: the current PIN.
    Remove,
}

impl Form {
    /// The dialog's title.
    pub fn title(self) -> &'static str {
        match self {
            Form::Unlock => tr!(LockTitle),
            Form::Set => tr!(PinSetTitle),
            Form::Change => tr!(PinChangeTitle),
            Form::Remove => tr!(PinRemoveTitle),
        }
    }

    /// Whether it has the box for the current PIN ([`Form::current_label`]).
    pub fn asks_current(self) -> bool {
        self != Form::Set
    }

    /// The current PIN box's label: the unlock prompt's, or "Current PIN:".
    pub fn current_label(self) -> &'static str {
        if self == Form::Unlock { tr!(LockPrompt) } else { tr!(PinCurrent) }
    }

    /// Whether it has the two boxes for a new PIN (labels `PinNew`, `PinRepeat`).
    pub fn asks_new(self) -> bool {
        matches!(self, Form::Set | Form::Change)
    }
}

/// Does what `form`'s OK does, with what was typed in its boxes (an empty string for a box it
/// does not have): unlocks, sets or changes the PIN, or removes it. The `Err` is the message
/// the dialog shows under its boxes, staying open. The caller saves after an `Ok`.
pub fn submit(
    form: Form,
    prefs: &mut Prefs,
    guard: &mut Guard,
    current: &str,
    new: &str,
    repeat: &str,
    now: Instant,
) -> Result<(), String> {
    if form.asks_current() {
        match guard.check(prefs, current, now) {
            Check::Right => {}
            Check::Wrong => return Err(tr!(LockWrong).to_owned()),
            Check::Wait(secs) => return Err(tr!(LockWait, secs)),
        }
    }
    if form.asks_new() {
        if !valid_pin(new) {
            return Err(tr!(PinBadLength).to_owned());
        }
        if new != repeat {
            return Err(tr!(PinMismatch).to_owned());
        }
    }
    match form {
        Form::Unlock => set_locked(prefs, false),
        Form::Set | Form::Change => set_pin(prefs, new),
        Form::Remove => {
            remove(prefs);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROUNDS: u32 = 10;

    fn with_pin(pin: &str) -> Prefs {
        let mut p = Prefs::default();
        set_pin_with(&mut p, pin, ROUNDS);
        p
    }

    #[test]
    fn a_pin_has_four_to_six_ascii_digits() {
        for good in ["0000", "1234", "12345", "999999"] {
            assert!(valid_pin(good), "{good}");
        }
        for bad in ["", "123", "1234567", "12a4", " 1234", "1234 ", "١٢٣٤", "１２３４", "12.4", "-123"]
        {
            assert!(!valid_pin(bad), "{bad}");
        }
    }

    #[test]
    fn the_pin_is_kept_as_a_salted_hash() {
        let p = with_pin("2468");
        let stored = p.get(PIN_KEY).unwrap();
        assert!(stored.starts_with("pbkdf2-sha256:10:"));
        assert!(!stored.contains("2468"));
        assert!(has_pin(&p) && verify(&p, "2468"));
        assert!(!verify(&p, "2469") && !verify(&p, "") && !verify(&p, "24680"));
        assert_ne!(stored, with_pin("2468").get(PIN_KEY).unwrap(), "salted");
        let mut q = Prefs::default();
        set_pin(&mut q, "1357");
        assert!(q.get(PIN_KEY).unwrap().starts_with(&format!("pbkdf2-sha256:{PIN_ROUNDS}:")));
        assert!(verify(&q, "1357"));
    }

    #[test]
    fn a_hash_that_does_not_parse_is_no_pin_and_no_lock() {
        let mut p = Prefs::default();
        assert!(!has_pin(&p) && !verify(&p, "1234") && !is_locked(&p));
        assert!(!lock(&mut p) && p.get(LOCKED_KEY).is_none(), "no lock without a PIN");
        p.set(PIN_KEY, "1234");
        p.set_bool(LOCKED_KEY, true);
        assert!(!has_pin(&p) && !verify(&p, "1234"));
        assert!(!is_locked(&p), "a mark without a usable PIN does not lock");
    }

    #[test]
    fn locking_unlocking_and_removing() {
        let mut p = with_pin("1234");
        assert!(!is_locked(&p));
        assert!(lock(&mut p) && is_locked(&p));
        assert_eq!(p.get(LOCKED_KEY), Some("1"));
        set_locked(&mut p, false);
        assert!(!is_locked(&p) && p.get(LOCKED_KEY).is_none());
        lock(&mut p);
        assert!(remove(&mut p));
        assert!(!has_pin(&p) && !is_locked(&p) && p.get(LOCKED_KEY).is_none());
        assert!(!remove(&mut p), "nothing left to remove");
        // the keys survive a save and a load
        let mut q = with_pin("4321");
        lock(&mut q);
        let back = Prefs::from_text(&q.to_text());
        assert!(is_locked(&back) && verify(&back, "4321"));
    }

    #[test]
    fn five_wrong_pins_start_a_wait_and_a_right_one_clears_it() {
        let p = with_pin("1234");
        let mut g = Guard::default();
        let t0 = Instant::now();
        for _ in 0..FREE_TRIES {
            assert_eq!(g.check(&p, "0000", t0), Check::Wrong);
        }
        assert_eq!(g.check(&p, "0000", t0), Check::Wait(30));
        // during the wait not even the right PIN is tried
        assert_eq!(g.check(&p, "1234", t0 + Duration::from_millis(500)), Check::Wait(30));
        assert_eq!(g.wait_left(t0 + Duration::from_secs(29)), Some(1));
        assert_eq!(g.check(&p, "1234", t0 + Duration::from_secs(10)), Check::Wait(20));
        // after it, one more wrong PIN starts another
        let t1 = t0 + WAIT;
        assert_eq!(g.wait_left(t1), None);
        assert_eq!(g.check(&p, "9999", t1), Check::Wait(30));
        let t2 = t1 + WAIT;
        assert_eq!(g.check(&p, "1234", t2), Check::Right);
        // the count starts again
        for _ in 0..FREE_TRIES {
            assert_eq!(g.check(&p, "0000", t2), Check::Wrong);
        }
        assert_eq!(g.check(&p, "1234", t2), Check::Right);
    }

    #[test]
    fn the_forms_do_what_their_ok_does() {
        let t = Instant::now();
        let mut g = Guard::default();
        let mut p = Prefs::default();

        // Set: the new PIN twice, checked
        assert_eq!(submit(Form::Set, &mut p, &mut g, "", "12", "12", t), Err(tr!(PinBadLength).into()));
        assert_eq!(submit(Form::Set, &mut p, &mut g, "", "1234", "1243", t), Err(tr!(PinMismatch).into()));
        assert!(!has_pin(&p));
        assert_eq!(submit(Form::Set, &mut p, &mut g, "", "1234", "1234", t), Ok(()));
        assert!(verify(&p, "1234"));

        // Unlock
        lock(&mut p);
        assert_eq!(submit(Form::Unlock, &mut p, &mut g, "1111", "", "", t), Err(tr!(LockWrong).into()));
        assert!(is_locked(&p));
        assert_eq!(submit(Form::Unlock, &mut p, &mut g, "1234", "", "", t), Ok(()));
        assert!(!is_locked(&p));

        // Change: the current PIN first, then the new one
        assert_eq!(
            submit(Form::Change, &mut p, &mut g, "1111", "5678", "5678", t),
            Err(tr!(LockWrong).into())
        );
        assert_eq!(
            submit(Form::Change, &mut p, &mut g, "1234", "5678", "567", t),
            Err(tr!(PinMismatch).into())
        );
        assert!(verify(&p, "1234"));
        assert_eq!(submit(Form::Change, &mut p, &mut g, "1234", "567890", "567890", t), Ok(()));
        assert!(verify(&p, "567890") && !verify(&p, "1234"));

        // Remove
        assert_eq!(submit(Form::Remove, &mut p, &mut g, "1234", "", "", t), Err(tr!(LockWrong).into()));
        assert_eq!(submit(Form::Remove, &mut p, &mut g, "567890", "", "", t), Ok(()));
        assert!(!has_pin(&p));

        // the wait shows in the message
        let mut p = with_pin("1234");
        let mut g = Guard::default();
        for _ in 0..FREE_TRIES {
            submit(Form::Unlock, &mut p, &mut g, "0000", "", "", t).unwrap_err();
        }
        assert_eq!(submit(Form::Unlock, &mut p, &mut g, "0000", "", "", t), Err(tr!(LockWait, secs = 30)));

        // what each form asks
        assert!(!Form::Set.asks_current() && Form::Set.asks_new());
        assert!(Form::Change.asks_current() && Form::Change.asks_new());
        assert!(Form::Unlock.asks_current() && !Form::Unlock.asks_new());
        assert!(Form::Remove.asks_current() && !Form::Remove.asks_new());
        assert_eq!(Form::Unlock.current_label(), tr!(LockPrompt));
        assert_eq!(Form::Remove.current_label(), tr!(PinCurrent));
    }

    #[test]
    fn the_reset_switch() {
        assert!(reset_requested(&["--reset-lock"]));
        assert!(reset_requested(&["x.torrent", "--RESET-LOCK"]));
        assert!(!reset_requested(&["--reset", "--lock", "reset-lock"]));
        assert!(!reset_requested::<&str>(&[]));
        assert!(is_reset_switch("--Reset-Lock"));
    }

    #[test]
    fn the_lock_keys_never_pass_to_the_web_ui() {
        let changes = vec![
            (PIN_KEY.to_owned(), Some("pbkdf2-sha256:1:00:00".to_owned())),
            ("ui.balloons".to_owned(), Some("0".to_owned())),
            (LOCKED_KEY.to_owned(), None),
            ("torrent.abc.label".to_owned(), Some("x".to_owned())),
        ];
        let kept = without_lock_keys(changes);
        assert_eq!(
            kept.iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>(),
            ["ui.balloons", "torrent.abc.label"]
        );
        assert!(is_lock_key("ui.lock_pin") && is_lock_key("ui.locked") && !is_lock_key("ui.lock"));
    }
}
