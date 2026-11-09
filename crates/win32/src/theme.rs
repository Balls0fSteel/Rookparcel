//! Dark mode for the Win32 GUI (D64, docs/DARKMODE.md).
//!
//! The preference `ui.theme` is System (follow Windows' "Choose your mode" for apps), Light or
//! Dark. Dark needs Windows 10 1809 (build 17763) or later and `uxtheme.dll`'s undocumented
//! exports, which are looked up by ordinal and feature-tested: without them, or under High
//! Contrast, the program stays with the system colours it always had. Light mode never calls
//! any of this at the first show, so it stays pixel-identical to the program before dark mode.
//!
//! What dark changes: the window caption (DWM), the popup menus (`SetPreferredAppMode`), the
//! common controls (`SetWindowTheme` with the `DarkMode_*` classes) and everything the program
//! draws itself, which asks [`sys`] for the system-colour stand-ins and [`draw`] for the drawn
//! parts' palette (`frontend::palette`).

// The colours, brushes and control theming come into use slice by slice (DARKMODE.md §7); the
// allowance goes once the last slice has landed.
#![allow(dead_code)]

use std::ffi::c_void;
use std::ptr::{null, null_mut};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use rookparcel_frontend::lang::S;
use rookparcel_frontend::palette::DrawPalette;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::Graphics::Dwm::{DWMWA_USE_IMMERSIVE_DARK_MODE, DwmSetWindowAttribute};
use windows_sys::Win32::Graphics::Gdi::{
    COLOR_BTNFACE, COLOR_BTNSHADOW, COLOR_BTNTEXT, COLOR_GRAYTEXT, COLOR_HIGHLIGHT, COLOR_HIGHLIGHTTEXT,
    COLOR_WINDOW, COLOR_WINDOWTEXT, CreateSolidBrush, GetSysColor, GetSysColorBrush, HBRUSH,
};
use windows_sys::Win32::System::LibraryLoader::{
    GetProcAddress, LOAD_LIBRARY_SEARCH_SYSTEM32, LoadLibraryExW,
};
use windows_sys::Win32::System::Registry::HKEY_CURRENT_USER;
use windows_sys::Win32::System::SystemInformation::{GetVersionExW, OSVERSIONINFOW};
use windows_sys::Win32::UI::Accessibility::{HCF_HIGHCONTRASTON, HIGHCONTRASTW};
use windows_sys::Win32::UI::Controls::SetWindowTheme;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    SPI_GETHIGHCONTRAST, SendMessageW, SystemParametersInfoW, WM_APP, WM_THEMECHANGED,
};
use windows_sys::core::w;

/// Posted to the main window when the effective theme flips (a preference change, or Windows'
/// setting under System); the main window re-themes itself and tells the open dialogs.
pub const WM_APP_THEME: u32 = WM_APP + 11;

/// The first Windows 10 with the dark common controls (1809).
pub const MIN_BUILD: u32 = 17763;
/// From this build `DWMWA_USE_IMMERSIVE_DARK_MODE` is 20; before, the attribute was 19.
const DWM_ATTRIBUTE_20_BUILD: u32 = 18985;
/// From this build ordinal 135 is `SetPreferredAppMode`; on 17763 it is `AllowDarkModeForApp`.
const PREFERRED_APP_MODE_BUILD: u32 = 18362;

/// The `ui.theme` preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Mode {
    /// Follow Windows' "Choose your mode" for apps.
    #[default]
    System,
    Light,
    Dark,
}

impl Mode {
    pub const ALL: [Mode; 3] = [Mode::System, Mode::Light, Mode::Dark];

    /// The stored value (the Web UI's theme ids): anything but `light` and `dark` is System.
    pub fn from_key(key: &str) -> Mode {
        match key.trim() {
            "light" => Mode::Light,
            "dark" => Mode::Dark,
            _ => Mode::System,
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Mode::System => "system",
            Mode::Light => "light",
            Mode::Dark => "dark",
        }
    }

    /// The Preferences drop-down's text.
    pub fn label(self) -> S {
        match self {
            Mode::System => S::ThemeSystem,
            Mode::Light => S::ThemeLight,
            Mode::Dark => S::ThemeDark,
        }
    }

    fn index(self) -> u8 {
        Mode::ALL.iter().position(|m| *m == self).unwrap_or(0) as u8
    }
}

/// What decides the effective mode, gathered once per decision so the rule is a pure function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Inputs {
    pub pref: Mode,
    /// Windows' "Choose your mode" for apps, when it could be read.
    pub windows_dark: Option<bool>,
    pub high_contrast: bool,
    pub build: u32,
    /// `uxtheme.dll`'s dark-mode exports are all there.
    pub uxtheme: bool,
}

/// Whether the program shows dark: never under High Contrast, never without the exports or
/// before 1809; then the preference, System following Windows (unknown counts as light).
pub fn resolve(i: &Inputs) -> bool {
    if i.high_contrast || !i.uxtheme || i.build < MIN_BUILD {
        return false;
    }
    match i.pref {
        Mode::Dark => true,
        Mode::Light => false,
        Mode::System => i.windows_dark == Some(true),
    }
}

static DARK_NOW: AtomicBool = AtomicBool::new(false);
static PREF: AtomicU8 = AtomicU8::new(0);
/// Something flipped since the start: from then on light has to undo what dark set, so the
/// "never touched" guarantee only holds for a process that never left light.
static SWITCHED: AtomicBool = AtomicBool::new(false);

/// Whether the window is dark right now.
pub fn is_dark() -> bool {
    DARK_NOW.load(Ordering::Relaxed)
}

/// The preference in force.
pub fn preference() -> Mode {
    Mode::ALL[PREF.load(Ordering::Relaxed) as usize]
}

/// Whether this Windows can show dark at all (the build and the exports).
pub fn available() -> bool {
    uxtheme().is_some()
}

/// The palette of what the program draws itself, for the mode in force.
pub fn draw() -> &'static DrawPalette {
    DrawPalette::for_dark(is_dark())
}

/// At start, before any window exists: decides the mode and tells uxtheme, so the menus and
/// the common controls are born in it.
pub fn init(pref: Mode) {
    PREF.store(pref.index(), Ordering::Relaxed);
    let dark = resolve(&inputs());
    DARK_NOW.store(dark, Ordering::Relaxed);
    if let Some(u) = uxtheme() {
        u.set_app_mode(dark);
    }
}

/// A new preference; `true` when the effective mode flipped (the caller re-themes the windows).
pub fn set_preference(pref: Mode) -> bool {
    PREF.store(pref.index(), Ordering::Relaxed);
    reconsider()
}

/// Windows' setting may have changed (`WM_SETTINGCHANGE`); `true` when the mode flipped.
pub fn system_changed() -> bool {
    reconsider()
}

fn reconsider() -> bool {
    let dark = resolve(&inputs());
    if dark == is_dark() {
        return false;
    }
    DARK_NOW.store(dark, Ordering::Relaxed);
    SWITCHED.store(true, Ordering::Relaxed);
    if let Some(u) = uxtheme() {
        u.set_app_mode(dark);
        // SAFETY: plain calls into uxtheme with no arguments.
        unsafe {
            (u.refresh_immersive_color_policy_state)();
            (u.flush_menu_themes)();
        }
    }
    true
}

fn inputs() -> Inputs {
    let u = uxtheme();
    Inputs {
        pref: preference(),
        windows_dark: windows_dark(u),
        high_contrast: high_contrast(),
        build: u.map_or_else(build_number, |u| u.build),
        uxtheme: u.is_some(),
    }
}

// ---- Windows' own state ---------------------------------------------------------------------

/// Windows' "Choose your mode" for apps: `ROOKPARCEL_FORCE_WINDOWS_DARK=0|1` for tests, else
/// uxtheme's answer, else the registry value the setting writes.
fn windows_dark(u: Option<&Uxtheme>) -> Option<bool> {
    match std::env::var("ROOKPARCEL_FORCE_WINDOWS_DARK").ok().as_deref() {
        Some("1") => return Some(true),
        Some("0") => return Some(false),
        _ => {}
    }
    if let Some(u) = u {
        // SAFETY: a no-argument call into uxtheme.
        return Some(unsafe { (u.should_apps_use_dark_mode)() });
    }
    crate::integration::get_dword_in(
        HKEY_CURRENT_USER,
        r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
        Some("AppsUseLightTheme"),
    )
    .map(|light| light == 0)
}

/// Windows High Contrast is on (`ROOKPARCEL_FORCE_HIGH_CONTRAST=1` for tests).
fn high_contrast() -> bool {
    if std::env::var_os("ROOKPARCEL_FORCE_HIGH_CONTRAST").is_some_and(|v| v == "1") {
        return true;
    }
    // SAFETY: a zeroed HIGHCONTRASTW with its size set, as SPI_GETHIGHCONTRAST asks.
    unsafe {
        let mut hc: HIGHCONTRASTW = std::mem::zeroed();
        hc.cbSize = size_of::<HIGHCONTRASTW>() as u32;
        let ok = SystemParametersInfoW(SPI_GETHIGHCONTRAST, hc.cbSize, &mut hc as *mut _ as *mut c_void, 0);
        ok != 0 && hc.dwFlags & HCF_HIGHCONTRASTON != 0
    }
}

/// The Windows build number (the manifest declares Windows 10, so it is the real one).
fn build_number() -> u32 {
    // SAFETY: a zeroed OSVERSIONINFOW with its size set.
    unsafe {
        let mut v: OSVERSIONINFOW = std::mem::zeroed();
        v.dwOSVersionInfoSize = size_of::<OSVERSIONINFOW>() as u32;
        if GetVersionExW(&mut v) != 0 { v.dwBuildNumber } else { 0 }
    }
}

// ---- uxtheme's undocumented exports -------------------------------------------------------

type ShouldAppsUseDarkModeFn = unsafe extern "system" fn() -> bool;
type AllowDarkModeForWindowFn = unsafe extern "system" fn(HWND, bool) -> bool;
type SetPreferredAppModeFn = unsafe extern "system" fn(i32) -> i32;
type AllowDarkModeForAppFn = unsafe extern "system" fn(bool) -> bool;
type VoidFn = unsafe extern "system" fn();

/// Ordinal 135, whose meaning changed with 1903.
#[derive(Clone, Copy)]
enum AppMode {
    /// `SetPreferredAppMode(Default 0 | AllowDark 1 | ForceDark 2 | ForceLight 3)`, 18362+.
    Preferred(SetPreferredAppModeFn),
    /// `AllowDarkModeForApp(bool)`, 17763.
    Allow(AllowDarkModeForAppFn),
}

/// The exports dark mode needs, by ordinal: 132 `ShouldAppsUseDarkMode`, 133
/// `AllowDarkModeForWindow`, 135 (see [`AppMode`]), 136 `FlushMenuThemes`, 104
/// `RefreshImmersiveColorPolicyState`. Explorer, Notepad++ and Chromium use the same ones.
pub struct Uxtheme {
    should_apps_use_dark_mode: ShouldAppsUseDarkModeFn,
    allow_dark_mode_for_window: AllowDarkModeForWindowFn,
    app_mode: AppMode,
    flush_menu_themes: VoidFn,
    refresh_immersive_color_policy_state: VoidFn,
    pub build: u32,
}

impl Uxtheme {
    fn set_app_mode(&self, dark: bool) {
        // SAFETY: the exports' documented-by-use signatures (DARKMODE.md §2).
        unsafe {
            match self.app_mode {
                AppMode::Preferred(f) => {
                    f(if dark { 2 } else { 0 });
                }
                AppMode::Allow(f) => {
                    f(dark);
                }
            }
        }
    }
}

/// uxtheme's dark-mode exports, looked up once; `None` before 1809, when any export is missing
/// (a Windows that dropped them: the program stays light) or with `ROOKPARCEL_NO_DARK_UXTHEME=1`.
pub fn uxtheme() -> Option<&'static Uxtheme> {
    static U: OnceLock<Option<Uxtheme>> = OnceLock::new();
    U.get_or_init(load_uxtheme).as_ref()
}

fn load_uxtheme() -> Option<Uxtheme> {
    if std::env::var_os("ROOKPARCEL_NO_DARK_UXTHEME").is_some_and(|v| v == "1") {
        return None;
    }
    let build = build_number();
    if build < MIN_BUILD {
        return None;
    }
    // SAFETY: loading the system's uxtheme.dll from System32 only, then reading exports by
    // ordinal; each pointer is cast to the signature the export has had since 1809.
    unsafe {
        let h = LoadLibraryExW(w!("uxtheme.dll"), null_mut(), LOAD_LIBRARY_SEARCH_SYSTEM32);
        if h.is_null() {
            return None;
        }
        let ordinal = |n: u16| GetProcAddress(h, n as usize as *const u8);
        let should_apps_use_dark_mode: ShouldAppsUseDarkModeFn = std::mem::transmute(ordinal(132)?);
        let allow_dark_mode_for_window: AllowDarkModeForWindowFn = std::mem::transmute(ordinal(133)?);
        let p135 = ordinal(135)?;
        let app_mode = if build >= PREFERRED_APP_MODE_BUILD {
            AppMode::Preferred(std::mem::transmute::<
                unsafe extern "system" fn() -> isize,
                SetPreferredAppModeFn,
            >(p135))
        } else {
            AppMode::Allow(
                std::mem::transmute::<unsafe extern "system" fn() -> isize, AllowDarkModeForAppFn>(p135),
            )
        };
        let flush_menu_themes: VoidFn = std::mem::transmute(ordinal(136)?);
        let refresh_immersive_color_policy_state: VoidFn = std::mem::transmute(ordinal(104)?);
        Some(Uxtheme {
            should_apps_use_dark_mode,
            allow_dark_mode_for_window,
            app_mode,
            flush_menu_themes,
            refresh_immersive_color_policy_state,
            build,
        })
    }
}

// ---- colours --------------------------------------------------------------------------------

/// The system colours the program paints with, as `COLORREF`s: Windows' own when light, fixed
/// dark values aligned with `frontend::palette::DARK` and the Web UI's dark tokens when dark.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SysPalette {
    pub window: u32,
    pub window_text: u32,
    pub gray_text: u32,
    pub highlight: u32,
    pub highlight_text: u32,
    pub button_face: u32,
    pub button_text: u32,
    /// Frames and rules (`COLOR_BTNSHADOW` when light).
    pub shadow: u32,
    /// Lines between parts (headers, status bar, tab strip; dark only differs from `shadow`).
    pub border: u32,
    /// Column headers and the tab strip's unselected tabs.
    pub header: u32,
    /// The hot (hovered) toolbar button, header and menu item.
    pub hot: u32,
    /// The pressed or checked toolbar button, and an unfocused selection.
    pub pressed: u32,
}

/// `0xRRGGBB` to a `COLORREF`.
pub const fn bgr(rgb: u32) -> u32 {
    ((rgb & 0xff) << 16) | (rgb & 0xff00) | (rgb >> 16)
}

/// The dark system colours, in RGB before [`bgr`].
pub const DARK_SYS_RGB: SysPalette = SysPalette {
    window: 0x1e1f22,
    window_text: 0xe0e0e0,
    gray_text: 0x9a9a9a,
    highlight: 0x2d4f7a,
    highlight_text: 0xffffff,
    button_face: 0x2b2d31,
    button_text: 0xe0e0e0,
    shadow: 0x5a5e66,
    border: 0x3a3d42,
    header: 0x26272b,
    hot: 0x3a3d42,
    pressed: 0x45484f,
};

const DARK_SYS: SysPalette = SysPalette {
    window: bgr(DARK_SYS_RGB.window),
    window_text: bgr(DARK_SYS_RGB.window_text),
    gray_text: bgr(DARK_SYS_RGB.gray_text),
    highlight: bgr(DARK_SYS_RGB.highlight),
    highlight_text: bgr(DARK_SYS_RGB.highlight_text),
    button_face: bgr(DARK_SYS_RGB.button_face),
    button_text: bgr(DARK_SYS_RGB.button_text),
    shadow: bgr(DARK_SYS_RGB.shadow),
    border: bgr(DARK_SYS_RGB.border),
    header: bgr(DARK_SYS_RGB.header),
    hot: bgr(DARK_SYS_RGB.hot),
    pressed: bgr(DARK_SYS_RGB.pressed),
};

/// The colours for the mode in force. Light reads `GetSysColor` on each call, as the program
/// always did, so it follows `WM_SYSCOLORCHANGE` for free.
pub fn sys() -> SysPalette {
    if is_dark() {
        return DARK_SYS;
    }
    // SAFETY: GetSysColor takes a colour index and cannot fail.
    unsafe {
        SysPalette {
            window: GetSysColor(COLOR_WINDOW),
            window_text: GetSysColor(COLOR_WINDOWTEXT),
            gray_text: GetSysColor(COLOR_GRAYTEXT),
            highlight: GetSysColor(COLOR_HIGHLIGHT),
            highlight_text: GetSysColor(COLOR_HIGHLIGHTTEXT),
            button_face: GetSysColor(COLOR_BTNFACE),
            button_text: GetSysColor(COLOR_BTNTEXT),
            shadow: GetSysColor(COLOR_BTNSHADOW),
            border: GetSysColor(COLOR_BTNSHADOW),
            header: GetSysColor(COLOR_WINDOW),
            hot: GetSysColor(COLOR_HIGHLIGHT),
            pressed: GetSysColor(COLOR_BTNFACE),
        }
    }
}

/// A brush of the mode in force.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brush {
    Window,
    Face,
    Header,
    Hot,
    Pressed,
    Border,
}

thread_local! {
    /// The dark brushes, made once and kept for the thread's life (the colours never change).
    static DARK_BRUSHES: std::cell::Cell<[HBRUSH; 6]> = const { std::cell::Cell::new([null_mut(); 6]) };
}

/// Light: the system's own brush (never deleted); dark: a solid brush kept for good.
pub fn brush(which: Brush) -> HBRUSH {
    if !is_dark() {
        let index = match which {
            Brush::Window | Brush::Header => COLOR_WINDOW,
            Brush::Face | Brush::Pressed => COLOR_BTNFACE,
            Brush::Hot => COLOR_HIGHLIGHT,
            Brush::Border => COLOR_BTNSHADOW,
        };
        // SAFETY: a colour index; the brush belongs to the system.
        return unsafe { GetSysColorBrush(index) };
    }
    let i = which as usize;
    DARK_BRUSHES.with(|cell| {
        let mut all = cell.get();
        if all[i].is_null() {
            let color = match which {
                Brush::Window => DARK_SYS.window,
                Brush::Face => DARK_SYS.button_face,
                Brush::Header => DARK_SYS.header,
                Brush::Hot => DARK_SYS.hot,
                Brush::Pressed => DARK_SYS.pressed,
                Brush::Border => DARK_SYS.border,
            };
            // SAFETY: a GDI brush that is kept for the thread's life, so no double free.
            all[i] = unsafe { CreateSolidBrush(color) };
            cell.set(all);
        }
        all[i]
    })
}

// ---- windows and controls -------------------------------------------------------------------

/// A top-level window (the main one, a dialog): its caption follows the mode and its dark
/// common controls are allowed. Light at the first show does nothing at all.
///
/// # Safety
/// `hwnd` is a window of this thread.
pub unsafe fn theme_window(hwnd: HWND) {
    let dark = is_dark();
    if !dark && !SWITCHED.load(Ordering::Relaxed) {
        return;
    }
    let Some(u) = uxtheme() else { return };
    // SAFETY: the caller's window; the exports' signatures as in DARKMODE.md §2.
    unsafe {
        (u.allow_dark_mode_for_window)(hwnd, dark);
        let value: i32 = dark as i32;
        let attribute =
            if u.build >= DWM_ATTRIBUTE_20_BUILD { DWMWA_USE_IMMERSIVE_DARK_MODE as u32 } else { 19 };
        DwmSetWindowAttribute(
            hwnd,
            attribute,
            &value as *const i32 as *const c_void,
            size_of::<i32>() as u32,
        );
    }
}

/// Which `DarkMode_*` visual-style class a control takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// List views, tree views, scroll bars, buttons, edits, tooltips, up-downs.
    Explorer,
    /// Headers (and list views that want Explorer's item look).
    ItemsView,
    /// Combo boxes and their drop lists.
    Cfd,
}

/// A common control follows the mode. Light at the first show does nothing; after a switch it
/// takes the default visual style back.
///
/// # Safety
/// `hwnd` is a control of this thread.
pub unsafe fn theme_control(hwnd: HWND, kind: Kind) {
    if hwnd.is_null() {
        return;
    }
    let dark = is_dark();
    if !dark && !SWITCHED.load(Ordering::Relaxed) {
        return;
    }
    if uxtheme().is_none() {
        return;
    }
    // SAFETY: the caller's control; SetWindowTheme takes the class name or null for the default.
    unsafe {
        if let Some(u) = uxtheme() {
            (u.allow_dark_mode_for_window)(hwnd, dark);
        }
        if dark {
            let class = match kind {
                Kind::Explorer => w!("DarkMode_Explorer"),
                Kind::ItemsView => w!("DarkMode_ItemsView"),
                Kind::Cfd => w!("DarkMode_CFD"),
            };
            SetWindowTheme(hwnd, class, null());
        } else {
            SetWindowTheme(hwnd, null(), null());
        }
        SendMessageW(hwnd, WM_THEMECHANGED, 0, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs(pref: Mode) -> Inputs {
        Inputs { pref, windows_dark: Some(true), high_contrast: false, build: 22631, uxtheme: true }
    }

    #[test]
    fn the_rule_follows_the_preference_then_windows() {
        assert!(resolve(&inputs(Mode::Dark)));
        assert!(!resolve(&inputs(Mode::Light)));
        assert!(resolve(&inputs(Mode::System)));
        assert!(!resolve(&Inputs { windows_dark: Some(false), ..inputs(Mode::System) }));
        assert!(!resolve(&Inputs { windows_dark: None, ..inputs(Mode::System) }), "unknown counts as light");
        assert!(resolve(&Inputs { windows_dark: None, ..inputs(Mode::Dark) }));
    }

    #[test]
    fn high_contrast_old_windows_and_missing_exports_stay_light() {
        for pref in Mode::ALL {
            assert!(!resolve(&Inputs { high_contrast: true, ..inputs(pref) }), "{pref:?}");
            assert!(!resolve(&Inputs { uxtheme: false, ..inputs(pref) }), "{pref:?}");
            assert!(!resolve(&Inputs { build: MIN_BUILD - 1, ..inputs(pref) }), "{pref:?}");
        }
        assert!(resolve(&Inputs { build: MIN_BUILD, ..inputs(Mode::Dark) }));
    }

    #[test]
    fn keys_round_trip_and_anything_else_is_system() {
        for m in Mode::ALL {
            assert_eq!(Mode::from_key(m.key()), m);
            assert_eq!(Mode::ALL[m.index() as usize], m);
        }
        assert_eq!(Mode::from_key(""), Mode::System);
        assert_eq!(Mode::from_key(" dark "), Mode::Dark);
        assert_eq!(Mode::from_key("DARK"), Mode::System, "the stored value is lower case");
        assert_eq!(Mode::from_key("midnight"), Mode::System, "a Web UI theme name is not a mode here");
        assert_eq!(Mode::default(), Mode::System);
    }

    fn luminance(rgb: u32) -> f64 {
        let ch = |v: u32| {
            let c = v as f64 / 255.0;
            if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * ch(rgb >> 16) + 0.7152 * ch((rgb >> 8) & 0xff) + 0.0722 * ch(rgb & 0xff)
    }

    fn contrast(a: u32, b: u32) -> f64 {
        let (x, y) = (luminance(a) + 0.05, luminance(b) + 0.05);
        if x > y { x / y } else { y / x }
    }

    /// WCAG AA for normal text over every dark surface the text sits on.
    #[test]
    fn the_dark_palette_keeps_text_readable() {
        let p = DARK_SYS_RGB;
        for surface in [p.window, p.button_face, p.header, p.hot, p.pressed] {
            assert!(contrast(p.window_text, surface) >= 4.5, "text over {surface:06x}");
            assert!(contrast(p.gray_text, surface) >= 3.0, "grey text over {surface:06x}");
        }
        assert!(contrast(p.highlight_text, p.highlight) >= 4.5);
        assert!(
            contrast(p.window_text, p.highlight) >= 4.5,
            "selected rows keep their text colour unfocused"
        );
    }

    #[test]
    fn bgr_swaps_red_and_blue() {
        assert_eq!(bgr(0x112233), 0x332211);
        assert_eq!(bgr(0xffffff), 0xffffff);
        assert_eq!(DARK_SYS.window, bgr(0x1e1f22));
    }

    /// Light is the system's own brushes and colours: nothing of the program's.
    #[test]
    fn light_is_the_systems_own() {
        assert!(!is_dark());
        unsafe {
            assert_eq!(brush(Brush::Face), GetSysColorBrush(COLOR_BTNFACE));
            assert_eq!(brush(Brush::Window), GetSysColorBrush(COLOR_WINDOW));
            assert_eq!(sys().window, GetSysColor(COLOR_WINDOW));
            assert_eq!(sys().button_text, GetSysColor(COLOR_BTNTEXT));
        }
        assert_eq!(draw(), &rookparcel_frontend::palette::LIGHT);
    }

    #[test]
    fn the_exports_are_looked_up_without_panicking() {
        if let Some(u) = uxtheme() {
            assert!(u.build >= MIN_BUILD);
        }
    }
}
