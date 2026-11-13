//! The windows' lock (D69, `frontend::ui_lock`): a PIN hides every window of Rookparcel until it
//! is entered again. It keeps the transfer list from someone at the same Mac; it is not a
//! password for the data.
//!
//! - **Locking** (File > Lock Rookparcel ⌘L, the Dock and status-item menus; Set PIN first
//!   without a PIN): every window that is on screen or in the Dock, not a sheet (it goes with
//!   its window) and at the normal level (the status item's and the floating Auto Shutdown
//!   countdown stay) is ordered out, and remembered front to back. A minimized one first
//!   leaves the Dock, unseen, since ordered out from there it would keep its tile. Not
//!   `NSApp.hide`, which ⌘-Tab undoes. `ui.locked=1` is saved at once and the menu bar shrinks
//!   to `menu::locked_tree` (Show Rookparcel, Hide, Quit and Edit).
//! - **While locked**: Show Rookparcel, the Dock icon, the status item's click, a second start
//!   and a double-clicked torrent ask for the PIN ([`request`]); torrents handed over wait for
//!   it ([`hold_targets`]) and are dropped when the prompt is cancelled. `app::apply` holds
//!   the messages, Add dialogs and the update offer and drops completion notifications
//!   ([`hold`]); `app::run` and `app::state` refuse what the locked menus leave out
//!   (`Command::works_locked`).
//! - **Unlocking** (the right PIN in `dialogs::pin`): the full menu bar, the same windows in
//!   their order (minimized ones back into the Dock), the main window
//!   (`lifecycle::show_window`), then what was held.
//! - **Starting locked**: the main window is never shown (`app::open`); a start by hand
//!   (without `--minimized`) asks for the PIN at once, Cancel leaves Rookparcel in the menu bar.
//! - **Reset** (a forgotten PIN, [`Reset`]): Option held at the start, or `--reset-lock`, asks
//!   `LockResetAsk` before the window (`app::open`); with a copy running, the second start
//!   forwards it and that copy asks (`mac::open`), as it does when its Dock icon is clicked with
//!   Option held (a Finder start of a running application starts no second process).
//!
//! The pure parts here are compiled and tested on every system.

use rookparcel_frontend::ui_lock;

/// How a reset of the PIN was asked for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    /// Option held at the start (or on a click of the running copy's Dock icon).
    Key,
    /// `--reset-lock` on the command line.
    Switch,
}

/// What a reset request does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetStep {
    /// Ask `LockResetAsk` (a PIN is set).
    Ask,
    /// Nothing: Option was held by chance, there is no PIN.
    Nothing,
    /// Say `LockResetNone`: the switch asked, there is no PIN.
    SayNone,
}

impl Reset {
    /// The request at a start: the switch on the command line, else Option held.
    pub fn at_start<S: AsRef<str>>(option_held: bool, args: &[S]) -> Option<Reset> {
        if ui_lock::reset_requested(args) {
            Some(Reset::Switch)
        } else if option_held {
            Some(Reset::Key)
        } else {
            None
        }
    }

    /// Its name in the forwarding notification's user info (`mac::open`).
    pub fn name(self) -> &'static str {
        match self {
            Reset::Key => "key",
            Reset::Switch => "switch",
        }
    }

    pub fn from_name(s: &str) -> Option<Reset> {
        [Reset::Key, Reset::Switch].into_iter().find(|r| r.name() == s)
    }

    /// What it does with or without a PIN set.
    pub fn step(self, has_pin: bool) -> ResetStep {
        match (self, has_pin) {
            (_, true) => ResetStep::Ask,
            (Reset::Key, false) => ResetStep::Nothing,
            (Reset::Switch, false) => ResetStep::SayNone,
        }
    }
}

/// What a PIN box keeps of what was typed or pasted: its ASCII digits, at most
/// `ui_lock::MAX_DIGITS` of them.
pub fn pin_digits(s: &str) -> String {
    s.chars().filter(char::is_ascii_digit).take(ui_lock::MAX_DIGITS).collect()
}

#[cfg(target_os = "macos")]
pub use sys::*;

#[cfg(target_os = "macos")]
mod sys {
    use std::cell::{Cell, RefCell};
    use std::ptr::NonNull;
    use std::time::Instant;

    use block2::RcBlock;
    use objc2::MainThreadMarker;
    use objc2::rc::Retained;
    use objc2::runtime::AnyObject;
    use objc2_app_kit::{
        NSAlertFirstButtonReturn, NSApplication, NSEvent, NSEventModifierFlags, NSWindow,
        NSWindowDidDeminiaturizeNotification, NSWindowNumberListOptions,
    };
    use objc2_foundation::{NSNotification, NSNotificationCenter};
    use rookparcel_frontend::app::Events;
    use rookparcel_frontend::integration::mac::Target;
    use rookparcel_frontend::prefs::Prefs;
    use rookparcel_frontend::tr;
    use rookparcel_frontend::ui_lock::{self, Form, Guard};

    use super::{Reset, ResetStep};
    use crate::app::{peek, with_core};
    use crate::mac::lifecycle;
    use crate::{mainq, ui};

    /// A window the lock hid.
    struct Hidden {
        window: Retained<NSWindow>,
        /// It was in the Dock, and goes back there.
        minimized: bool,
        /// Ordered out (a minimized one only once it has left the Dock).
        out: bool,
    }

    thread_local! {
        /// The windows are locked (as `ui.locked` says; read without borrowing the app).
        static LOCKED: Cell<bool> = const { Cell::new(false) };
        /// The PIN prompt is up or on its way.
        static ASKING: Cell<bool> = const { Cell::new(false) };
        /// The windows hidden, front to back.
        static HIDDEN: RefCell<Vec<Hidden>> = const { RefCell::new(Vec::new()) };
        /// What `app::apply` held back.
        static HELD: RefCell<Option<Events>> = const { RefCell::new(None) };
        /// Torrents and links handed over while locked.
        static TARGETS: RefCell<Vec<Target>> = const { RefCell::new(Vec::new()) };
        /// The wrong-PIN count and wait, for the process's life.
        static GUARD: RefCell<Guard> = RefCell::new(Guard::default());
        /// The observer of windows leaving the Dock is installed.
        static WATCHING: Cell<bool> = const { Cell::new(false) };
    }

    /// Whether the windows are locked.
    pub fn is_locked() -> bool {
        LOCKED.with(Cell::get)
    }

    /// Whether a PIN is set (false before the start or while the app is borrowed).
    pub fn has_pin() -> bool {
        peek(|c| ui_lock::has_pin(c.prefs())).unwrap_or(false)
    }

    /// Whether Option is held now (the class method: the keyboard's state, not an event's).
    pub fn option_held() -> bool {
        NSEvent::modifierFlags_class().contains(NSEventModifierFlags::Option)
    }

    /// `ui.locked` as the settings have it.
    fn locked_in_prefs() -> Option<bool> {
        peek(|c| ui_lock::is_locked(c.prefs()))
    }

    /// `app::apply`: while locked, what would open a window by itself (messages, Add New Torrent
    /// dialogs, a magnet's dialog, the update offer) waits for the unlock, and completion
    /// notifications, which name torrents, are dropped.
    pub fn hold(ev: &mut Events) {
        if !is_locked() {
            return;
        }
        let popups = ev.take_popups();
        HELD.with(|h| h.borrow_mut().get_or_insert_with(Events::default).merge(popups));
    }

    /// An information message to show after the unlock.
    pub fn hold_info(text: &str) {
        HELD.with(|h| h.borrow_mut().get_or_insert_with(Events::default).infos.push(text.to_owned()));
    }

    /// Torrents and links handed over while locked (a second start, a double click): they wait
    /// for the PIN, which is asked for, and go if the prompt is cancelled.
    pub fn hold_targets(targets: Vec<Target>) {
        eprintln!("lock: {} file(s) and link(s) wait for the PIN", targets.len());
        TARGETS.with(|t| t.borrow_mut().extend(targets));
        request();
    }

    /// `app::open` found the settings locked: the window was never shown; the menu bar is the
    /// locked one.
    pub fn start_locked(mtm: MainThreadMarker) {
        LOCKED.with(|l| l.set(true));
        crate::menu::locked_bar(mtm);
        eprintln!("lock: started locked");
    }

    /// `Command::Lock`: without a PIN, Set PIN first.
    pub fn lock_command() {
        // (a modal prompt is never run inside a menu's action, the status item's click among them)
        mainq::on_main(|| {
            let Some(mtm) = MainThreadMarker::new() else { return };
            if is_locked() {
                return;
            }
            if !has_pin() && !crate::dialogs::pin::run(Form::Set, mtm) {
                return;
            }
            lock_now(mtm);
        });
    }

    /// Options > Lock PIN's items: Set (without a PIN), Change and Remove (with one).
    pub fn pin_command(form: Form) {
        mainq::on_main(move || {
            let Some(mtm) = MainThreadMarker::new() else { return };
            // as `app::state` greys them: Set never replaces a PIN without asking for it
            let fits = if form == Form::Set { !has_pin() } else { has_pin() };
            if fits && !is_locked() {
                crate::dialogs::pin::run(form, mtm);
            }
        });
    }

    fn lock_now(mtm: MainThreadMarker) {
        let ok = with_core(|c| {
            let ok = ui_lock::lock(c.prefs_mut());
            if ok {
                // at once: a lock must survive a crash
                c.save_prefs();
            }
            ok
        })
        .unwrap_or(false);
        if !ok {
            return;
        }
        LOCKED.with(|l| l.set(true));
        let n = hide_windows(mtm);
        crate::menu::locked_bar(mtm);
        eprintln!("lock: locked, {n} window(s) hidden");
    }

    fn same(a: &NSWindow, b: &NSWindow) -> bool {
        std::ptr::eq(a, b)
    }

    /// A window the lock hides: on screen or in the Dock, ordinary (`lifecycle::ordinary`), and
    /// not a child window (those go with their parent).
    fn hideable(w: &NSWindow) -> bool {
        (w.isVisible() || w.isMiniaturized()) && lifecycle::ordinary(w) && w.parentWindow().is_none()
    }

    /// Orders out every window the lock hides, front to back; the number hidden.
    fn hide_windows(mtm: MainThreadMarker) -> usize {
        let app = NSApplication::sharedApplication(mtm);
        // front to back on every space, then the rest (minimized ones are in no such list)
        let mut order: Vec<Retained<NSWindow>> =
            NSWindow::windowNumbersWithOptions(NSWindowNumberListOptions::AllSpaces, mtm)
                .map(|numbers| {
                    numbers.iter().filter_map(|n| app.windowWithWindowNumber(n.integerValue())).collect()
                })
                .unwrap_or_default();
        for w in app.windows().iter() {
            if !order.iter().any(|o| same(o, &w)) {
                order.push(w);
            }
        }
        let mut hidden = Vec::new();
        let mut docked = Vec::new();
        for w in order.into_iter().filter(|w| hideable(w)) {
            let minimized = w.isMiniaturized();
            if minimized {
                // ordered out from the Dock it would keep its tile (with the list in it): out
                // of the Dock first, unseen, then ordered out (`left_dock`)
                w.setAlphaValue(0.0);
                docked.push(w.clone());
            } else {
                w.orderOut(None);
            }
            hidden.push(Hidden { window: w, minimized, out: !minimized });
        }
        let n = hidden.len();
        HIDDEN.with(|h| h.borrow_mut().extend(hidden));
        if !docked.is_empty() {
            watch_dock();
            // (no borrow held: the notification may come at once)
            for w in docked {
                w.deminiaturize(None);
            }
        }
        n
    }

    /// Watches windows leaving the Dock (`NSWindowDidDeminiaturizeNotification`, any window), for
    /// the minimized ones the lock takes out of it.
    fn watch_dock() {
        if WATCHING.with(|w| w.replace(true)) {
            return;
        }
        let block = RcBlock::new(|n: NonNull<NSNotification>| {
            // SAFETY: the center hands a valid notification for the block's call.
            let n = unsafe { n.as_ref() };
            let Some(object) = n.object() else { return };
            let object: *const AnyObject = Retained::as_ptr(&object);
            let ours = HIDDEN.with(|h| {
                h.borrow().iter().any(|e| !e.out && std::ptr::eq(Retained::as_ptr(&e.window).cast(), object))
            });
            if ours {
                // after the notification: ordered out from within it, the Dock tile stayed
                mainq::on_main(left_dock);
            }
        });
        // SAFETY: AppKit's notification name; with no object it is every window's, with no
        // queue the block runs where it is posted (the main thread, as windows post theirs);
        // the observer is kept for the process's life.
        let observer = unsafe {
            NSNotificationCenter::defaultCenter().addObserverForName_object_queue_usingBlock(
                Some(NSWindowDidDeminiaturizeNotification),
                None,
                None,
                &block,
            )
        };
        lifecycle::keep_observer(observer);
    }

    /// Windows the lock took out of the Dock are on screen, unseen: they go now.
    fn left_dock() {
        let ready: Vec<Retained<NSWindow>> = HIDDEN.with(|h| {
            h.borrow_mut()
                .iter_mut()
                .filter(|e| !e.out && !e.window.isMiniaturized())
                .map(|e| {
                    e.out = true;
                    e.window.clone()
                })
                .collect()
        });
        for w in ready {
            w.orderOut(None);
            w.setAlphaValue(1.0);
        }
    }

    /// Asks for the PIN while locked (Show Rookparcel, the Dock icon, the status item's click, a
    /// second start, a double-clicked torrent): once; another request while the prompt is up
    /// brings it forward. Nothing when unlocked.
    pub fn request() {
        if !is_locked() {
            return;
        }
        if ASKING.with(|a| a.replace(true)) {
            if let Some(mtm) = MainThreadMarker::new() {
                crate::dialogs::pin::bring_forward(mtm);
            }
            return;
        }
        // never inside the caller: a delegate callback, a notification block, a menu's action
        mainq::on_main(|| {
            let Some(mtm) = MainThreadMarker::new() else { return };
            // (a reset may have come first)
            if locked_in_prefs() == Some(true) {
                crate::dialogs::pin::run(Form::Unlock, mtm);
            }
            ASKING.with(|a| a.set(false));
            // the right PIN, or a reset while the prompt was up
            if locked_in_prefs() == Some(false) {
                unlocked(mtm);
            } else {
                let dropped = TARGETS.with(|t| std::mem::take(&mut *t.borrow_mut())).len();
                eprintln!("lock: the PIN prompt was cancelled ({dropped} file(s) and link(s) dropped)");
            }
        });
    }

    /// The windows are unlocked (the right PIN, or a reset): the full menu bar, the same windows
    /// in their order, the main window, then what waited.
    fn unlocked(mtm: MainThreadMarker) {
        if !LOCKED.with(|l| l.replace(false)) {
            return;
        }
        crate::menu::unlocked_bar(mtm);
        let hidden = HIDDEN.with(|h| std::mem::take(&mut *h.borrow_mut()));
        // back to front, so the front one ends in front
        for e in hidden.iter().rev() {
            e.window.setAlphaValue(1.0);
            e.window.orderFront(None);
        }
        // the main window (also when it was in the status item), the front one with the keyboard
        lifecycle::show_window();
        let main = crate::app::ui().map(|u| u.window.clone());
        for e in hidden.iter().filter(|e| e.minimized) {
            if !main.as_deref().is_some_and(|m| same(m, &e.window)) {
                e.window.miniaturize(None);
            }
        }
        eprintln!("lock: unlocked, {} window(s) back", hidden.len());
        let held = HELD.with(|h| h.borrow_mut().take());
        if held.is_some() {
            crate::app::apply(held);
        }
        let targets = TARGETS.with(|t| std::mem::take(&mut *t.borrow_mut()));
        crate::mac::open::open_held(targets);
    }

    /// What a PIN dialog's OK does (`ui_lock::submit`, with the wrong-PIN count kept for the
    /// process); the settings are saved after an `Ok`. `None` while the app is busy.
    pub fn submit(form: Form, current: &str, new: &str, repeat: &str) -> Option<Result<(), String>> {
        let now = Instant::now();
        GUARD.with(|g| {
            let mut g = g.borrow_mut();
            with_core(|c| {
                let r = ui_lock::submit(form, c.prefs_mut(), &mut g, current, new, repeat, now);
                if r.is_ok() {
                    c.save_prefs();
                }
                r
            })
        })
    }

    /// "Remove the lock PIN?": true for Remove.
    fn reset_question(mtm: MainThreadMarker) -> bool {
        lifecycle::activate(mtm);
        let alert =
            ui::make_alert(ui::Style::Warning, tr!(LockResetAsk), "", &[tr!(BtnRemove), tr!(BtnCancel)], mtm);
        // Escape is Cancel's in every language (AppKit gives it only to a button titled "Cancel")
        if let Some(cancel) = alert.buttons().iter().nth(1) {
            cancel.setKeyEquivalent(&ui::ns("\u{1b}"));
        }
        alert.runModal() == NSAlertFirstButtonReturn
    }

    fn say_none(mtm: MainThreadMarker) {
        lifecycle::activate(mtm);
        ui::alert(ui::Style::Info, tr!(LockResetNone), "", &[], mtm);
    }

    /// A reset asked for at the start (`app::open`, before the window and `Core`): after Remove,
    /// `prefs` has no PIN and no lock, saved; true then (the Logger's line comes once there is
    /// one).
    pub fn reset_at_start(how: Reset, prefs: &mut Prefs, mtm: MainThreadMarker) -> bool {
        match how.step(ui_lock::has_pin(prefs)) {
            ResetStep::Nothing => false,
            ResetStep::SayNone => {
                say_none(mtm);
                false
            }
            ResetStep::Ask => {
                if !reset_question(mtm) {
                    return false;
                }
                ui_lock::remove(prefs);
                if let Err(e) = prefs.save() {
                    eprintln!("lock: the settings were not saved: {e}");
                }
                true
            }
        }
    }

    /// A reset in the running copy (a second start forwarded it, or the Dock icon was clicked
    /// with Option held); call it from `mainq::on_main`. After Remove the PIN is gone (saved,
    /// a Logger line) and the windows are unlocked.
    pub fn reset(how: Reset) {
        let Some(mtm) = MainThreadMarker::new() else { return };
        match how.step(has_pin()) {
            ResetStep::Nothing => {}
            ResetStep::SayNone => say_none(mtm),
            ResetStep::Ask => {
                if !reset_question(mtm) {
                    return;
                }
                with_core(|c| {
                    ui_lock::remove(c.prefs_mut());
                    c.save_prefs();
                });
                crate::app::log_line(tr!(LogLockPinRemoved));
                // with the prompt up, its end unlocks (`request`)
                if !crate::dialogs::pin::abort(mtm, None) {
                    unlocked(mtm);
                }
            }
        }
    }

    /// `applicationShouldHandleReopen:` (the Dock icon, or a Finder start of this running copy):
    /// with Option held and a PIN set, the reset question first, then as a click; while locked,
    /// the PIN prompt. True when this took care of it.
    pub fn reopen() -> bool {
        if option_held() && has_pin() {
            mainq::on_main(|| {
                reset(Reset::Key);
                // (asks for the PIN if still locked)
                lifecycle::show_window();
            });
            return true;
        }
        if is_locked() {
            request();
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_reset_is_asked_for_by_the_switch_or_option() {
        assert_eq!(Reset::at_start(false, &["--reset-lock"]), Some(Reset::Switch));
        assert_eq!(Reset::at_start(true, &["x.torrent", "--RESET-LOCK"]), Some(Reset::Switch));
        assert_eq!(Reset::at_start(true, &["--minimized"]), Some(Reset::Key));
        assert_eq!(Reset::at_start::<&str>(false, &[]), None);
        assert_eq!(Reset::at_start(false, &["--reset", "reset-lock"]), None);
    }

    #[test]
    fn what_a_reset_does() {
        assert_eq!(Reset::Key.step(true), ResetStep::Ask);
        assert_eq!(Reset::Switch.step(true), ResetStep::Ask);
        // Option held by chance says nothing; the switch says there is no PIN
        assert_eq!(Reset::Key.step(false), ResetStep::Nothing);
        assert_eq!(Reset::Switch.step(false), ResetStep::SayNone);
    }

    #[test]
    fn reset_names_round_trip() {
        for r in [Reset::Key, Reset::Switch] {
            assert_eq!(Reset::from_name(r.name()), Some(r));
        }
        assert_eq!(Reset::from_name(""), None);
        assert_eq!(Reset::from_name("1"), None);
    }

    #[test]
    fn a_pin_box_keeps_six_ascii_digits() {
        assert_eq!(pin_digits("1234"), "1234");
        assert_eq!(pin_digits("12a3-4 "), "1234");
        assert_eq!(pin_digits("1234567890"), "123456");
        assert_eq!(pin_digits("١٢٣٤１２"), "", "only ASCII digits");
        assert_eq!(pin_digits(""), "");
        assert!(ui_lock::valid_pin(&pin_digits(" 9876 54 ")));
    }
}
