//! The windows' lock (D69) in the Win32 GUI: what `app.rs` keeps while Rookparcel is locked.
//! Locking hides every visible window of the GUI thread that has a caption (the main window,
//! dialogs, message boxes, Statistics, Search Results) and remembers them; unlocking shows the
//! same ones again, in their order. What would pop up meanwhile is held ([`Held`]) and shown
//! after the PIN. A reset of a forgotten PIN is asked for at start, by Ctrl held or
//! `--reset-lock` ([`ResetBy`]); a second start forwards it to the running copy
//! ([`COPYDATA_RESET_LOCK`]). The PIN logic itself is `rookparcel_frontend::ui_lock`'s.

use std::collections::VecDeque;

use rookparcel_frontend::ui_lock::Guard;
use rookparcel_frontend::update::Version;
use windows_sys::Win32::Foundation::{HWND, LPARAM};
use windows_sys::Win32::System::DataExchange::COPYDATASTRUCT;
use windows_sys::Win32::System::Threading::GetCurrentThreadId;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumThreadWindows, GWL_STYLE, GetWindowLongW, IsIconic, IsWindow, IsWindowVisible, SW_HIDE,
    SW_SHOWMINNOACTIVE, SW_SHOWNA, ShowWindow, WS_CAPTION,
};

/// How a reset of the PIN was asked for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetBy {
    /// Ctrl held while Rookparcel started: nothing is said when there is no PIN.
    Key = 1,
    /// `--reset-lock`: "No lock PIN is set." when there is none.
    Switch = 2,
}

impl ResetBy {
    /// The code a forwarded request and `WM_APP_RESET_LOCK`'s `wParam` carry.
    pub fn code(self) -> u32 {
        self as u32
    }

    pub fn from_code(code: u32) -> Option<ResetBy> {
        match code {
            1 => Some(ResetBy::Key),
            2 => Some(ResetBy::Switch),
            _ => None,
        }
    }
}

/// The reset a start asks for: the switch when it is there (it says when there is no PIN),
/// else the held key.
pub fn reset_by(key_held: bool, switch: bool) -> Option<ResetBy> {
    if switch {
        Some(ResetBy::Switch)
    } else if key_held {
        Some(ResetBy::Key)
    } else {
        None
    }
}

/// `COPYDATASTRUCT::dwData` of a reset request a second start forwards before its arguments
/// (`instance::forward` in `main.rs`), so the running copy asks first; its data is the
/// [`ResetBy`] code as a `u32`. A build from before the lock answers it as any message it does
/// not know and takes the arguments that follow as before.
pub const COPYDATA_RESET_LOCK: usize = 0x4c54_524c;

/// The reset request in a `WM_COPYDATA`, or `None` for another message or bad data.
pub fn copydata_reset(cds: &COPYDATASTRUCT) -> Option<ResetBy> {
    if cds.dwData != COPYDATA_RESET_LOCK || cds.lpData.is_null() || cds.cbData != 4 {
        return None;
    }
    // SAFETY: the sender's 4 bytes at lpData, valid while the message is handled
    let code = unsafe { std::ptr::read_unaligned(cds.lpData as *const u32) };
    ResetBy::from_code(code)
}

/// Error and information boxes held while locked, at most: the first ones are kept (the Logger
/// has every line).
pub const MAX_MESSAGES: usize = 20;

/// Something that would have popped up while locked, shown after the PIN. `U` is a finished
/// URL download (`app::UrlResult`).
pub enum Popup<U> {
    Error(String),
    Info(String),
    /// Magnets have their metadata: their Add New Torrent dialogs (`WM_APP_MAGNET` again).
    Magnet,
    Url(U),
    /// A newer version's box (`offer_update`).
    Update(Version),
}

/// What waits for the PIN: popups in the order they came, and the sources a second start, a
/// double-click or the command line handed over (dropped when the prompt is cancelled).
pub struct Held<U> {
    popups: VecDeque<Popup<U>>,
    sources: Vec<String>,
}

impl<U> Default for Held<U> {
    fn default() -> Self {
        Held { popups: VecDeque::new(), sources: Vec::new() }
    }
}

impl<U> Held<U> {
    /// Holds `p`: messages up to [`MAX_MESSAGES`], one Magnet, the newest Update only.
    pub fn push(&mut self, p: Popup<U>) {
        match &p {
            Popup::Error(_) | Popup::Info(_) => {
                let n = self.popups.iter().filter(|q| matches!(q, Popup::Error(_) | Popup::Info(_))).count();
                if n >= MAX_MESSAGES {
                    return;
                }
            }
            Popup::Magnet if self.popups.iter().any(|q| matches!(q, Popup::Magnet)) => return,
            Popup::Update(_) => self.popups.retain(|q| !matches!(q, Popup::Update(_))),
            _ => {}
        }
        self.popups.push_back(p);
    }

    /// The oldest popup.
    pub fn pop(&mut self) -> Option<Popup<U>> {
        self.popups.pop_front()
    }

    pub fn hold_sources(&mut self, sources: impl IntoIterator<Item = String>) {
        self.sources.extend(sources);
    }

    pub fn take_sources(&mut self) -> Vec<String> {
        std::mem::take(&mut self.sources)
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.popups.len()
    }
}

/// The lock's state beside `App`'s preferences, which say whether it is locked
/// (`ui_lock::is_locked`).
pub struct LockUi<U> {
    /// The windows hidden by the lock, top first, and whether each was minimized.
    pub hidden: Vec<(HWND, bool)>,
    pub held: Held<U>,
    /// Wrong PINs, in memory (the unlock prompt and Change / Remove PIN share it).
    pub guard: Guard,
    /// A forwarded reset's question is open, and the arguments that came after it wait for
    /// its answer: handled inside its message box, they would open their dialog (the PIN
    /// prompt, Add New Torrent) before the box ever showed (a dialog shows once its loop is
    /// idle).
    pub asking_reset: bool,
    pub deferred_args: bool,
}

impl<U> Default for LockUi<U> {
    fn default() -> Self {
        LockUi {
            hidden: Vec::new(),
            held: Held::default(),
            guard: Guard::default(),
            asking_reset: false,
            deferred_args: false,
        }
    }
}

/// Hides every visible window with a caption of this thread but those in `keep`, and returns
/// them top first (`EnumThreadWindows` goes down the Z order) with whether each was minimized.
///
/// # Safety
/// Run on the GUI thread.
pub unsafe fn hide_windows(keep: &[HWND]) -> Vec<(HWND, bool)> {
    struct Found<'a> {
        keep: &'a [HWND],
        windows: Vec<(HWND, bool)>,
    }
    unsafe extern "system" fn found(hwnd: HWND, lp: LPARAM) -> i32 {
        // SAFETY: `lp` is the `Found` below, alive during the enumeration
        unsafe {
            let f = &mut *(lp as *mut Found<'_>);
            let style = GetWindowLongW(hwnd, GWL_STYLE) as u32;
            if !f.keep.contains(&hwnd) && IsWindowVisible(hwnd) != 0 && style & WS_CAPTION == WS_CAPTION {
                f.windows.push((hwnd, IsIconic(hwnd) != 0));
            }
        }
        1
    }
    let mut f = Found { keep, windows: Vec::new() };
    unsafe {
        EnumThreadWindows(GetCurrentThreadId(), Some(found), &mut f as *mut Found<'_> as LPARAM);
        for &(hwnd, _) in &f.windows {
            ShowWindow(hwnd, SW_HIDE);
        }
    }
    f.windows
}

/// Shows `hidden` again bottom up, without activating them (a minimized one stays minimized),
/// skipping windows closed meanwhile; the top one that is not minimized, to activate.
///
/// # Safety
/// Run on the GUI thread.
pub unsafe fn restore_windows(hidden: &[(HWND, bool)]) -> Option<HWND> {
    let mut top = None;
    unsafe {
        for &(hwnd, iconic) in hidden.iter().rev() {
            if IsWindow(hwnd) == 0 {
                continue;
            }
            ShowWindow(hwnd, if iconic { SW_SHOWMINNOACTIVE } else { SW_SHOWNA });
            if !iconic {
                top = Some(hwnd);
            }
        }
    }
    top
}

#[cfg(test)]
mod tests {
    use std::ffi::c_void;
    use std::ptr::{null, null_mut};

    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassW, WNDCLASSW, WS_EX_NOACTIVATE,
        WS_EX_TOOLWINDOW, WS_OVERLAPPEDWINDOW, WS_POPUP,
    };
    use windows_sys::core::w;

    use super::*;

    #[test]
    fn the_reset_comes_from_the_switch_or_the_key() {
        assert_eq!(reset_by(false, false), None);
        assert_eq!(reset_by(true, false), Some(ResetBy::Key));
        assert_eq!(reset_by(false, true), Some(ResetBy::Switch));
        assert_eq!(reset_by(true, true), Some(ResetBy::Switch), "the switch says when there is no PIN");
        for by in [ResetBy::Key, ResetBy::Switch] {
            assert_eq!(ResetBy::from_code(by.code()), Some(by));
        }
        assert_eq!(ResetBy::from_code(0), None);
        assert_eq!(ResetBy::from_code(3), None);
    }

    /// A forwarded reset is read only when it is one, with its 4 bytes; the arguments' reader
    /// (and so a build from before the lock) does not take it.
    #[test]
    fn a_forwarded_reset_checks_its_data() {
        let cds = |data: usize, bytes: u32, ptr: *const u32| COPYDATASTRUCT {
            dwData: data,
            cbData: bytes,
            lpData: ptr as *mut c_void,
        };
        let key = ResetBy::Key.code();
        let switch = ResetBy::Switch.code();
        assert_eq!(copydata_reset(&cds(COPYDATA_RESET_LOCK, 4, &key)), Some(ResetBy::Key));
        assert_eq!(copydata_reset(&cds(COPYDATA_RESET_LOCK, 4, &switch)), Some(ResetBy::Switch));
        assert_eq!(copydata_reset(&cds(COPYDATA_RESET_LOCK, 4, null())), None, "a null pointer");
        assert_eq!(copydata_reset(&cds(COPYDATA_RESET_LOCK, 0, null())), None, "no data");
        assert_eq!(copydata_reset(&cds(COPYDATA_RESET_LOCK, 2, &key)), None, "too short");
        assert_eq!(copydata_reset(&cds(COPYDATA_RESET_LOCK, 4, &7)), None, "an unknown code");
        assert_eq!(copydata_reset(&cds(crate::app::COPYDATA_ARGS, 4, &key)), None, "the arguments");
        assert_ne!(COPYDATA_RESET_LOCK, crate::app::COPYDATA_ARGS);
        assert_eq!(crate::app::copydata_args(&cds(COPYDATA_RESET_LOCK, 4, &key)), None);
    }

    #[test]
    fn held_popups_keep_their_order_and_limits() {
        let mut h: Held<u8> = Held::default();
        h.push(Popup::Magnet);
        for i in 0..MAX_MESSAGES + 5 {
            h.push(if i % 2 == 0 { Popup::Error(format!("e{i}")) } else { Popup::Info(format!("i{i}")) });
        }
        h.push(Popup::Magnet);
        h.push(Popup::Url(1));
        h.push(Popup::Update(Version::parse("1.2.50").unwrap()));
        h.push(Popup::Url(2));
        h.push(Popup::Update(Version::parse("1.2.51").unwrap()));
        assert_eq!(h.len(), 1 + MAX_MESSAGES + 2 + 1, "one magnet, the first messages, one update");
        assert!(matches!(h.pop(), Some(Popup::Magnet)));
        assert!(matches!(h.pop(), Some(Popup::Error(e)) if e == "e0"));
        assert!(matches!(h.pop(), Some(Popup::Info(i)) if i == "i1"));
        let rest: Vec<Popup<u8>> = std::iter::from_fn(|| h.pop()).collect();
        let last_message = rest.iter().rev().find_map(|p| match p {
            Popup::Error(t) | Popup::Info(t) => Some(t.clone()),
            _ => None,
        });
        assert_eq!(last_message.as_deref(), Some("i19"), "the first 20");
        let tail: Vec<String> = rest
            .iter()
            .filter_map(|p| match p {
                Popup::Url(n) => Some(format!("url{n}")),
                Popup::Update(v) => Some(v.to_string()),
                Popup::Magnet => Some("magnet".into()),
                _ => None,
            })
            .collect();
        assert_eq!(tail, ["url1", "url2", "1.2.51"]);
        assert!(h.pop().is_none());
        // sources wait apart, and go at once
        h.hold_sources(["a.torrent".to_owned()]);
        h.hold_sources(["magnet:?xt=urn:btih:x".to_owned()]);
        assert_eq!(h.take_sources(), ["a.torrent", "magnet:?xt=urn:btih:x"]);
        assert!(h.take_sources().is_empty());
    }

    unsafe extern "system" fn plain(hwnd: HWND, msg: u32, wp: usize, lp: LPARAM) -> isize {
        unsafe { DefWindowProcW(hwnd, msg, wp, lp) }
    }

    /// A test window far off the screen, without a taskbar button or activation, so a test run
    /// on the user's desktop shows nothing.
    fn window(style: u32) -> HWND {
        static CLASS: std::sync::Once = std::sync::Once::new();
        let class = w!("RookparcelLockTest");
        unsafe {
            CLASS.call_once(|| {
                let wc = WNDCLASSW {
                    lpfnWndProc: Some(plain),
                    hInstance: GetModuleHandleW(null()),
                    lpszClassName: class,
                    ..std::mem::zeroed()
                };
                assert_ne!(RegisterClassW(&wc), 0);
            });
            let hwnd = CreateWindowExW(
                WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
                class,
                w!("lock test"),
                style,
                -32000,
                -32000,
                120,
                80,
                null_mut(),
                null_mut(),
                GetModuleHandleW(null()),
                null(),
            );
            assert!(!hwnd.is_null());
            hwnd
        }
    }

    /// Locking hides the visible captioned windows, not the others; unlocking shows the same
    /// ones, a minimized one minimized, and skips one closed meanwhile.
    #[test]
    fn the_lock_hides_and_brings_back_the_same_windows() {
        unsafe {
            let shown = window(WS_OVERLAPPEDWINDOW);
            let minimized = window(WS_OVERLAPPEDWINDOW);
            let closed = window(WS_OVERLAPPEDWINDOW);
            let popup = window(WS_POPUP);
            let hidden = window(WS_OVERLAPPEDWINDOW);
            let kept = window(WS_OVERLAPPEDWINDOW);
            for h in [shown, closed, popup, kept] {
                ShowWindow(h, SW_SHOWNA);
            }
            ShowWindow(minimized, SW_SHOWMINNOACTIVE);
            assert!(IsIconic(minimized) != 0);

            let got = hide_windows(&[kept]);
            let mine: Vec<(HWND, bool)> = got
                .iter()
                .copied()
                .filter(|(h, _)| [shown, minimized, closed, popup, hidden, kept].contains(h))
                .collect();
            assert_eq!(mine.len(), 3, "{mine:?}");
            for h in [shown, minimized, closed] {
                assert!(mine.contains(&(h, h == minimized)), "{h:?}");
                assert_eq!(IsWindowVisible(h), 0);
            }
            assert_ne!(IsWindowVisible(popup), 0, "no caption: not one of the GUI's windows");
            assert_ne!(IsWindowVisible(kept), 0, "kept (the Auto Shutdown countdown)");

            DestroyWindow(closed);
            let top = restore_windows(&mine);
            assert!(top == Some(shown), "the top one that is not minimized");
            assert_ne!(IsWindowVisible(shown), 0);
            assert_ne!(IsWindowVisible(minimized), 0);
            assert_ne!(IsIconic(minimized), 0, "minimized it was, minimized it comes back");
            assert_eq!(IsWindowVisible(hidden), 0, "hidden before, hidden after");
            assert_eq!(restore_windows(&[]), None);
            for h in [shown, minimized, popup, hidden, kept] {
                DestroyWindow(h);
            }
        }
    }
}
