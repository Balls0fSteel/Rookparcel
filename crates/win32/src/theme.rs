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
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, SIZE, WPARAM};
use windows_sys::Win32::Graphics::Dwm::{DWMWA_USE_IMMERSIVE_DARK_MODE, DwmSetWindowAttribute};
use windows_sys::Win32::Graphics::Gdi::{
    BeginPaint, COLOR_BTNFACE, COLOR_BTNSHADOW, COLOR_BTNTEXT, COLOR_GRAYTEXT, COLOR_HIGHLIGHT,
    COLOR_HIGHLIGHTTEXT, COLOR_WINDOW, COLOR_WINDOWTEXT, CreateFontIndirectW, CreateSolidBrush, DFC_BUTTON,
    DFCS_BUTTONCHECK, DFCS_BUTTONRADIO, DFCS_CHECKED, DFCS_INACTIVE, DT_CALCRECT, DT_CENTER, DT_END_ELLIPSIS,
    DT_HIDEPREFIX, DT_LEFT, DT_NOCLIP, DT_NOPREFIX, DT_RIGHT, DT_SINGLELINE, DT_VCENTER, DT_WORDBREAK,
    DeleteObject, DrawFocusRect, DrawFrameControl, DrawTextW, EndPaint, FillRect, FrameRect, GetSysColor,
    GetSysColorBrush, GetWindowDC, HBRUSH, HDC, HFONT, HGDIOBJ, InvalidateRect, OPAQUE, PAINTSTRUCT,
    ReleaseDC, SelectObject, SetBkColor, SetBkMode, SetTextColor, TRANSPARENT,
};
use windows_sys::Win32::System::LibraryLoader::{
    GetProcAddress, LOAD_LIBRARY_SEARCH_SYSTEM32, LoadLibraryExW,
};
use windows_sys::Win32::System::Registry::HKEY_CURRENT_USER;
use windows_sys::Win32::System::SystemInformation::{GetVersionExW, OSVERSIONINFOW};
use windows_sys::Win32::UI::Accessibility::{HCF_HIGHCONTRASTON, HIGHCONTRASTW};
use windows_sys::Win32::UI::Controls::{
    BP_CHECKBOX, BP_RADIOBUTTON, BST_CHECKED, BST_INDETERMINATE, CDDS_ITEMPREPAINT, CDDS_PREPAINT,
    CDIS_CHECKED, CDIS_DISABLED, CDIS_HOT, CDIS_SELECTED, CDRF_DODEFAULT, CDRF_NOTIFYITEMDRAW,
    CDRF_SKIPDEFAULT, CloseThemeData, DRAWITEMSTRUCT, DrawThemeBackground, GetThemePartSize, HDF_CENTER,
    HDF_JUSTIFYMASK, HDF_RIGHT, HDF_SORTDOWN, HDF_SORTUP, HDI_FORMAT, HDI_TEXT, HDITEMW, HDM_GETITEMW,
    LVM_GETHEADER, LVM_SETBKCOLOR, LVM_SETTEXTBKCOLOR, LVM_SETTEXTCOLOR, NM_CUSTOMDRAW, NMCUSTOMDRAW, NMHDR,
    NMTBCUSTOMDRAW, ODS_DISABLED, ODS_GRAYED, ODS_HOTLIGHT, ODS_INACTIVE, ODS_NOACCEL, ODS_SELECTED,
    OpenThemeData, SB_GETICON, SB_GETPARTS, SB_GETRECT, SB_GETTEXTLENGTHW, SB_GETTEXTW, SB_ISSIMPLE,
    SB_SIMPLEID, SBARS_SIZEGRIP, SetWindowTheme, TBCDRF_NOBACKGROUND, TBCDRF_NOEDGES, TBCDRF_USECDCOLORS,
    TS_DRAW, TVM_SETBKCOLOR, TVM_SETLINECOLOR, TVM_SETTEXTCOLOR,
};
use windows_sys::Win32::UI::HiDpi::{GetDpiForWindow, GetSystemMetricsForDpi, SystemParametersInfoForDpi};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetFocus, IsWindowEnabled};
use windows_sys::Win32::UI::Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    BM_GETSTATE, BS_AUTORADIOBUTTON, BS_MULTILINE, BS_RADIOBUTTON, BST_PUSHED, DI_NORMAL, DrawIconEx,
    GWL_EXSTYLE, GWL_STYLE, GetClientRect, GetMenuBarInfo, GetMenuItemInfoW, GetParent, GetWindowLongW,
    GetWindowRect, GetWindowTextLengthW, GetWindowTextW, HICON, HMENU, MENUBARINFO, MENUITEMINFOW,
    MIIM_STRING, NONCLIENTMETRICSW, OBJID_MENU, SM_CXSMICON, SPI_GETHIGHCONTRAST, SPI_GETNONCLIENTMETRICS,
    SendMessageW, SystemParametersInfoW, UISF_HIDEACCEL, UISF_HIDEFOCUS, WM_APP, WM_CTLCOLORBTN,
    WM_CTLCOLORDLG, WM_CTLCOLOREDIT, WM_CTLCOLORLISTBOX, WM_CTLCOLORSTATIC, WM_ERASEBKGND, WM_GETFONT,
    WM_NCDESTROY, WM_NOTIFY, WM_PAINT, WM_QUERYUISTATE, WM_THEMECHANGED, WS_EX_LAYOUTRTL,
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
    /// The text colour, for glyphs drawn as rectangles.
    Text,
}

thread_local! {
    /// The dark brushes, made once and kept for the thread's life (the colours never change).
    static DARK_BRUSHES: std::cell::Cell<[HBRUSH; 7]> = const { std::cell::Cell::new([null_mut(); 7]) };
}

/// Light: the system's own brush (never deleted); dark: a solid brush kept for good.
pub fn brush(which: Brush) -> HBRUSH {
    if !is_dark() {
        let index = match which {
            Brush::Window | Brush::Header => COLOR_WINDOW,
            Brush::Face | Brush::Pressed => COLOR_BTNFACE,
            Brush::Hot => COLOR_HIGHLIGHT,
            Brush::Border => COLOR_BTNSHADOW,
            Brush::Text => COLOR_WINDOWTEXT,
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
                Brush::Text => DARK_SYS.window_text,
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

// ---- subclasses and painted parts -----------------------------------------------------------

const SUBCLASS_ID: usize = 0x5244; // "RD"

type SubclassFn = unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM, usize, usize) -> LRESULT;

/// Installs `proc` on `hwnd` once (a second call replaces it). The procs do nothing while light,
/// so installing them changes no pixel.
unsafe fn subclass(hwnd: HWND, proc: SubclassFn) {
    if hwnd.is_null() {
        return;
    }
    // SAFETY: the caller's window; the id is ours.
    unsafe {
        SetWindowSubclass(hwnd, Some(proc), SUBCLASS_ID, 0);
    }
}

/// A list view (its header too): the dark classes, the dark background and text colours, and
/// the header painted by [`list_proc`] in dark.
///
/// # Safety
/// `lv` is a list view of this thread.
pub unsafe fn list(lv: HWND) {
    // SAFETY: the caller's list view and its header.
    unsafe {
        subclass(lv, list_proc);
        let dark = is_dark();
        if !dark && !SWITCHED.load(Ordering::Relaxed) {
            return;
        }
        theme_control(lv, Kind::Explorer);
        let header = SendMessageW(lv, LVM_GETHEADER, 0, 0) as HWND;
        theme_control(header, Kind::ItemsView);
        let s = sys();
        SendMessageW(lv, LVM_SETBKCOLOR, 0, s.window as LPARAM);
        SendMessageW(lv, LVM_SETTEXTBKCOLOR, 0, s.window as LPARAM);
        SendMessageW(lv, LVM_SETTEXTCOLOR, 0, s.window_text as LPARAM);
        InvalidateRect(lv, null(), 1);
    }
}

/// A tree view: the dark class and colours.
///
/// # Safety
/// `tree` is a tree view of this thread.
pub unsafe fn tree(tree: HWND) {
    let dark = is_dark();
    if !dark && !SWITCHED.load(Ordering::Relaxed) {
        return;
    }
    // SAFETY: the caller's tree view.
    unsafe {
        theme_control(tree, Kind::Explorer);
        let s = sys();
        SendMessageW(tree, TVM_SETBKCOLOR, 0, s.window as LPARAM);
        SendMessageW(tree, TVM_SETTEXTCOLOR, 0, s.window_text as LPARAM);
        SendMessageW(tree, TVM_SETLINECOLOR, 0, s.gray_text as LPARAM);
        InvalidateRect(tree, null(), 1);
    }
}

/// A status bar: painted by [`status_proc`] in dark.
///
/// # Safety
/// `status` is a status bar of this thread.
pub unsafe fn status_bar(status: HWND) {
    // SAFETY: the caller's status bar.
    unsafe {
        subclass(status, status_proc);
        if is_dark() || SWITCHED.load(Ordering::Relaxed) {
            InvalidateRect(status, null(), 1);
        }
    }
}

/// A check box or radio button: the dark glyph from the `DarkMode_Explorer` theme and its text
/// painted by [`check_proc`] in the window's text colour (the themed button ignores
/// `WM_CTLCOLORSTATIC` for its caption).
///
/// # Safety
/// `button` is a check box or radio button of this thread.
pub unsafe fn check_box(button: HWND) {
    // SAFETY: the caller's button.
    unsafe {
        subclass(button, check_proc);
        theme_control(button, Kind::Explorer);
    }
}

/// The `WM_CTLCOLOR*` family in dark: the text and background colours, and the brush. `None`
/// while light (the caller falls through to the default), and for other messages.
///
/// # Safety
/// `wparam` is the message's device context and `lparam` its control.
pub unsafe fn ctl_color(msg: u32, wparam: WPARAM, lparam: LPARAM) -> Option<LRESULT> {
    if !is_dark() {
        return None;
    }
    let s = sys();
    let (bg, which) = match msg {
        WM_CTLCOLORDLG | WM_CTLCOLORSTATIC | WM_CTLCOLORBTN => (s.button_face, Brush::Face),
        WM_CTLCOLOREDIT | WM_CTLCOLORLISTBOX => (s.window, Brush::Window),
        _ => return None,
    };
    // SAFETY: the message's device context; the control may be null for WM_CTLCOLORDLG.
    unsafe {
        let hdc = wparam as HDC;
        let ctl = lparam as HWND;
        let disabled = !ctl.is_null() && IsWindowEnabled(ctl) == 0;
        SetTextColor(hdc, if disabled { s.gray_text } else { s.window_text });
        SetBkColor(hdc, bg);
        SetBkMode(hdc, OPAQUE as i32);
    }
    Some(brush(which) as LRESULT)
}

/// A 1 px line as a filled rectangle.
unsafe fn line(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, which: Brush) {
    let rc = RECT { left, top, right, bottom };
    // SAFETY: a plain fill on the caller's device context.
    unsafe {
        FillRect(hdc, &rc, brush(which));
    }
}

/// The text of a window, for painting.
unsafe fn text_of(hwnd: HWND) -> Vec<u16> {
    // SAFETY: the buffer is as long as the text plus the terminator.
    unsafe {
        let n = GetWindowTextLengthW(hwnd).max(0) as usize;
        let mut buf = vec![0u16; n + 1];
        let got = GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32).max(0) as usize;
        buf.truncate(got);
        buf
    }
}

fn scaled(px: i32, dpi: u32) -> i32 {
    (px * dpi as i32 + 48) / 96
}

/// The list view's subclass: in dark, its header's items are painted here (`DarkMode_ItemsView`
/// gives the dark surface but black text).
unsafe extern "system" fn list_proc(
    hwnd: HWND,
    msg: u32,
    wp: WPARAM,
    lp: LPARAM,
    _: usize,
    _: usize,
) -> LRESULT {
    // SAFETY: the subclassed list view's messages; `lp` is an NMHDR for WM_NOTIFY.
    unsafe {
        if msg == WM_NOTIFY && is_dark() {
            let nm = &*(lp as *const NMHDR);
            if nm.code == NM_CUSTOMDRAW && nm.hwndFrom == SendMessageW(hwnd, LVM_GETHEADER, 0, 0) as HWND {
                return header_custom_draw(&*(lp as *const NMCUSTOMDRAW));
            }
        }
        if msg == WM_NCDESTROY {
            RemoveWindowSubclass(hwnd, Some(list_proc), SUBCLASS_ID);
        }
        DefSubclassProc(hwnd, msg, wp, lp)
    }
}

/// A header item in dark: the surface (hot or pressed when so), a divider on its right, the
/// text with the column's alignment and the sort arrow.
unsafe fn header_custom_draw(cd: &NMCUSTOMDRAW) -> LRESULT {
    if cd.dwDrawStage == CDDS_PREPAINT {
        return CDRF_NOTIFYITEMDRAW as LRESULT;
    }
    if cd.dwDrawStage != CDDS_ITEMPREPAINT {
        return CDRF_DODEFAULT as LRESULT;
    }
    // SAFETY: the header's paint device context and item; the text buffer is ours.
    unsafe {
        let header = cd.hdr.hwndFrom;
        let hdc = cd.hdc;
        let rc = cd.rc;
        let fill = if cd.uItemState & CDIS_SELECTED != 0 {
            Brush::Pressed
        } else if cd.uItemState & CDIS_HOT != 0 {
            Brush::Hot
        } else {
            Brush::Header
        };
        FillRect(hdc, &rc, brush(fill));
        line(hdc, rc.right - 1, rc.top, rc.right, rc.bottom, Brush::Border);
        line(hdc, rc.left, rc.bottom - 1, rc.right, rc.bottom, Brush::Border);
        let mut text = [0u16; 256];
        let mut item: HDITEMW = std::mem::zeroed();
        item.mask = HDI_TEXT | HDI_FORMAT;
        item.pszText = text.as_mut_ptr();
        item.cchTextMax = text.len() as i32;
        SendMessageW(header, HDM_GETITEMW, cd.dwItemSpec, &mut item as *mut _ as LPARAM);
        let font = SendMessageW(header, WM_GETFONT, 0, 0) as HGDIOBJ;
        let old = SelectObject(hdc, font);
        SetBkMode(hdc, TRANSPARENT as i32);
        SetTextColor(hdc, sys().window_text);
        let dpi = GetDpiForWindow(header);
        let pad = scaled(6, dpi);
        let mut text_rc = RECT { left: rc.left + pad, top: rc.top, right: rc.right - pad, bottom: rc.bottom };
        let align = match item.fmt & HDF_JUSTIFYMASK {
            HDF_RIGHT => DT_RIGHT,
            HDF_CENTER => DT_CENTER,
            _ => DT_LEFT,
        };
        let len = text.iter().position(|&c| c == 0).unwrap_or(text.len()) as i32;
        DrawTextW(
            hdc,
            text.as_mut_ptr(),
            len,
            &mut text_rc,
            align | DT_SINGLELINE | DT_VCENTER | DT_END_ELLIPSIS | DT_NOPREFIX,
        );
        if item.fmt & (HDF_SORTUP | HDF_SORTDOWN) != 0 {
            // a small triangle at the top centre, as the theme draws it
            let rows = scaled(4, dpi).max(3);
            let cx = (rc.left + rc.right) / 2;
            for i in 0..rows {
                let y = if item.fmt & HDF_SORTUP != 0 { rc.top + 1 + i } else { rc.top + rows - i };
                line(hdc, cx - i, y, cx + i + 1, y + 1, Brush::Border);
            }
        }
        SelectObject(hdc, old);
    }
    CDRF_SKIPDEFAULT as LRESULT
}

/// The status bar's subclass: in dark, painted here (the themed one is light).
unsafe extern "system" fn status_proc(
    hwnd: HWND,
    msg: u32,
    wp: WPARAM,
    lp: LPARAM,
    _: usize,
    _: usize,
) -> LRESULT {
    // SAFETY: the subclassed status bar's messages.
    unsafe {
        match msg {
            WM_ERASEBKGND if is_dark() => 1,
            WM_PAINT if is_dark() => {
                paint_status(hwnd);
                0
            }
            WM_NCDESTROY => {
                RemoveWindowSubclass(hwnd, Some(status_proc), SUBCLASS_ID);
                DefSubclassProc(hwnd, msg, wp, lp)
            }
            _ => DefSubclassProc(hwnd, msg, wp, lp),
        }
    }
}

unsafe fn paint_status(hwnd: HWND) {
    // SAFETY: a paint cycle on the status bar; the buffers are sized from the control's answers.
    unsafe {
        let mut ps: PAINTSTRUCT = std::mem::zeroed();
        let hdc = BeginPaint(hwnd, &mut ps);
        let mut client: RECT = std::mem::zeroed();
        GetClientRect(hwnd, &mut client);
        FillRect(hdc, &client, brush(Brush::Face));
        let s = sys();
        let dpi = GetDpiForWindow(hwnd);
        let font = SendMessageW(hwnd, WM_GETFONT, 0, 0) as HGDIOBJ;
        let old = SelectObject(hdc, font);
        SetBkMode(hdc, TRANSPARENT as i32);
        SetTextColor(hdc, s.window_text);
        let pad = scaled(4, dpi);
        let icon_px = GetSystemMetricsForDpi(SM_CXSMICON, dpi);
        let draw_part = |part: usize, rc: RECT| {
            let mut x = rc.left + pad;
            let icon = SendMessageW(hwnd, SB_GETICON, part, 0) as HICON;
            if !icon.is_null() {
                let y = rc.top + (rc.bottom - rc.top - icon_px) / 2;
                DrawIconEx(hdc, x, y, icon, icon_px, icon_px, 0, null_mut(), DI_NORMAL);
                x += icon_px + pad;
            }
            let len = (SendMessageW(hwnd, SB_GETTEXTLENGTHW, part, 0) & 0xffff) as usize;
            let mut buf = vec![0u16; len + 1];
            SendMessageW(hwnd, SB_GETTEXTW, part, buf.as_mut_ptr() as LPARAM);
            let mut text_rc = RECT { left: x, top: rc.top, right: rc.right - pad, bottom: rc.bottom };
            DrawTextW(
                hdc,
                buf.as_mut_ptr(),
                len as i32,
                &mut text_rc,
                DT_LEFT | DT_SINGLELINE | DT_VCENTER | DT_END_ELLIPSIS | DT_NOPREFIX,
            );
        };
        if SendMessageW(hwnd, SB_ISSIMPLE, 0, 0) != 0 {
            draw_part(SB_SIMPLEID as usize, client);
        } else {
            let n = SendMessageW(hwnd, SB_GETPARTS, 0, 0).max(0) as usize;
            for part in 0..n {
                let mut rc: RECT = std::mem::zeroed();
                SendMessageW(hwnd, SB_GETRECT, part, &mut rc as *mut _ as LPARAM);
                draw_part(part, rc);
                if part + 1 < n {
                    line(hdc, rc.right, rc.top + 1, rc.right + 1, rc.bottom - 1, Brush::Border);
                }
            }
        }
        if GetWindowLongW(hwnd, GWL_STYLE) as u32 & SBARS_SIZEGRIP != 0 {
            // the size grip: three rows of dots in the corner
            let dot = scaled(2, dpi).max(1);
            let step = dot * 2;
            let grip = brush(Brush::Border);
            for row in 0..3 {
                for col in 0..=row {
                    let x = client.right - step * (col + 1) - dot;
                    let y = client.bottom - step * (3 - row) - dot;
                    let rc = RECT { left: x, top: y, right: x + dot, bottom: y + dot };
                    FillRect(hdc, &rc, grip);
                }
            }
        }
        SelectObject(hdc, old);
        EndPaint(hwnd, &ps);
    }
}

const BST_HOT_STATE: usize = 0x0200;

/// The check box and radio button subclass: in dark, painted here (the themed glyph from
/// `DarkMode_Explorer`, the caption in the window's text colour).
unsafe extern "system" fn check_proc(
    hwnd: HWND,
    msg: u32,
    wp: WPARAM,
    lp: LPARAM,
    _: usize,
    _: usize,
) -> LRESULT {
    // SAFETY: the subclassed button's messages.
    unsafe {
        match msg {
            WM_ERASEBKGND if is_dark() => 1,
            WM_PAINT if is_dark() => {
                paint_check(hwnd);
                0
            }
            WM_NCDESTROY => {
                RemoveWindowSubclass(hwnd, Some(check_proc), SUBCLASS_ID);
                DefSubclassProc(hwnd, msg, wp, lp)
            }
            _ => DefSubclassProc(hwnd, msg, wp, lp),
        }
    }
}

unsafe fn paint_check(hwnd: HWND) {
    // SAFETY: a paint cycle on the button; the theme handle is closed before returning.
    unsafe {
        let mut ps: PAINTSTRUCT = std::mem::zeroed();
        let hdc = BeginPaint(hwnd, &mut ps);
        let mut rc: RECT = std::mem::zeroed();
        GetClientRect(hwnd, &mut rc);
        // the background is the parent's: what it answers for a static
        let parent = GetParent(hwnd);
        let bg = SendMessageW(parent, WM_CTLCOLORSTATIC, hdc as WPARAM, hwnd as LPARAM) as HBRUSH;
        FillRect(hdc, &rc, if bg.is_null() { brush(Brush::Face) } else { bg });
        let style = GetWindowLongW(hwnd, GWL_STYLE) as u32;
        let radio = matches!((style & 0xf) as i32, BS_RADIOBUTTON | BS_AUTORADIOBUTTON);
        let part = if radio { BP_RADIOBUTTON } else { BP_CHECKBOX };
        let state = SendMessageW(hwnd, BM_GETSTATE, 0, 0) as usize;
        let enabled = IsWindowEnabled(hwnd) != 0;
        // CBS_UNCHECKEDNORMAL 1 .. CBS_MIXEDDISABLED 12: four states per check value
        let base = if state & BST_INDETERMINATE as usize != 0 {
            8
        } else if state & BST_CHECKED as usize != 0 {
            4
        } else {
            0
        };
        let variant = if !enabled {
            4
        } else if state & BST_PUSHED as usize != 0 {
            3
        } else if state & BST_HOT_STATE != 0 {
            2
        } else {
            1
        };
        let theme = OpenThemeData(hwnd, w!("Button"));
        let mut glyph = SIZE { cx: 13, cy: 13 };
        if theme != 0 {
            GetThemePartSize(theme, hdc, part, (base + variant) as i32, null(), TS_DRAW, &mut glyph);
        }
        let rtl = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32 & WS_EX_LAYOUTRTL != 0;
        let gy = rc.top + (rc.bottom - rc.top - glyph.cy) / 2;
        let gx = if rtl { rc.right - glyph.cx } else { rc.left };
        let glyph_rc = RECT { left: gx, top: gy, right: gx + glyph.cx, bottom: gy + glyph.cy };
        if theme != 0 {
            DrawThemeBackground(theme, hdc, part, (base + variant) as i32, &glyph_rc, null());
            CloseThemeData(theme);
        } else {
            let flags = if radio { DFCS_BUTTONRADIO } else { DFCS_BUTTONCHECK }
                | if state & BST_CHECKED as usize != 0 { DFCS_CHECKED } else { 0 }
                | if enabled { 0 } else { DFCS_INACTIVE };
            let mut frame = glyph_rc;
            DrawFrameControl(hdc, &mut frame, DFC_BUTTON, flags);
        }
        let dpi = GetDpiForWindow(hwnd);
        let gap = scaled(3, dpi);
        let mut text_rc = rc;
        if rtl {
            text_rc.right = gx - gap;
        } else {
            text_rc.left = glyph_rc.right + gap;
        }
        let font = SendMessageW(hwnd, WM_GETFONT, 0, 0) as HGDIOBJ;
        let old = SelectObject(hdc, font);
        SetBkMode(hdc, TRANSPARENT as i32);
        let s = sys();
        SetTextColor(hdc, if enabled { s.window_text } else { s.gray_text });
        let mut text = text_of(hwnd);
        let multiline = style as i32 & BS_MULTILINE != 0;
        let flags = DT_LEFT
            | if multiline { DT_WORDBREAK } else { DT_SINGLELINE | DT_VCENTER }
            | if SendMessageW(hwnd, WM_QUERYUISTATE, 0, 0) as u32 & UISF_HIDEACCEL != 0 {
                DT_HIDEPREFIX
            } else {
                0
            };
        DrawTextW(hdc, text.as_mut_ptr(), text.len() as i32, &mut text_rc, flags);
        if GetFocus() == hwnd && SendMessageW(hwnd, WM_QUERYUISTATE, 0, 0) as u32 & UISF_HIDEFOCUS == 0 {
            let mut focus = text_rc;
            DrawTextW(hdc, text.as_mut_ptr(), text.len() as i32, &mut focus, flags | DT_CALCRECT);
            focus.left -= 1;
            focus.right += 1;
            focus.top = text_rc.top + (text_rc.bottom - text_rc.top - (focus.bottom - focus.top)) / 2 - 1;
            focus.bottom = focus.top + (text_rc.bottom - text_rc.top).min(focus.bottom - focus.top + 2);
            DrawFocusRect(hdc, &focus);
        }
        SelectObject(hdc, old);
        EndPaint(hwnd, &ps);
    }
}

// ---- toolbar, tooltips and the menu bar -----------------------------------------------------

/// A control's tooltip window (`TB_GETTOOLTIPS`, `LVM_GETTOOLTIPS`, ...) follows the mode.
///
/// # Safety
/// `owner` is a control of this thread and `get` its message that answers the tooltip.
pub unsafe fn tooltip(owner: HWND, get: u32) {
    if owner.is_null() {
        return;
    }
    // SAFETY: the control's own tooltip window, or null.
    unsafe {
        let tip = SendMessageW(owner, get, 0, 0) as HWND;
        theme_control(tip, Kind::Explorer);
    }
}

/// A toolbar's `NM_CUSTOMDRAW` in dark: the hot, pressed and checked buttons get their own
/// fills (the themed ones are light), the images stay; `dropdown` is the search box's
/// chevron-only button, drawn whole. Light: the default drawing.
///
/// # Safety
/// `cd` is the notification's `NMTBCUSTOMDRAW`.
pub unsafe fn toolbar_custom_draw(cd: &mut NMTBCUSTOMDRAW, dropdown: bool) -> LRESULT {
    if !is_dark() {
        return CDRF_DODEFAULT as LRESULT;
    }
    match cd.nmcd.dwDrawStage {
        CDDS_PREPAINT => CDRF_NOTIFYITEMDRAW as LRESULT,
        CDDS_ITEMPREPAINT => {
            let hdc = cd.nmcd.hdc;
            let rc = cd.nmcd.rc;
            let st = cd.nmcd.uItemState;
            let fill = if st & (CDIS_SELECTED | CDIS_CHECKED) != 0 {
                Some(Brush::Pressed)
            } else if st & CDIS_HOT != 0 {
                Some(Brush::Hot)
            } else {
                None
            };
            // SAFETY: the toolbar's paint device context and the item's rectangle.
            unsafe {
                if let Some(b) = fill {
                    FillRect(hdc, &rc, brush(b));
                    FrameRect(hdc, &rc, brush(Brush::Border));
                }
                if dropdown {
                    // a chevron in the text colour, as wide as the themed arrow
                    let dpi = GetDpiForWindow(cd.nmcd.hdr.hwndFrom);
                    let rows = scaled(3, dpi).max(2);
                    let cx = (rc.left + rc.right) / 2;
                    let top = (rc.top + rc.bottom) / 2 - rows / 2;
                    let which = if st & CDIS_DISABLED != 0 { Brush::Border } else { Brush::Text };
                    for i in 0..rows {
                        line(hdc, cx - (rows - i), top + i, cx + (rows - i), top + i + 1, which);
                    }
                    return CDRF_SKIPDEFAULT as LRESULT;
                }
            }
            cd.clrText = sys().window_text;
            (TBCDRF_NOBACKGROUND | TBCDRF_NOEDGES | TBCDRF_USECDCOLORS) as LRESULT
        }
        _ => CDRF_DODEFAULT as LRESULT,
    }
}

/// The undocumented messages Windows sends a window whose menu bar it would paint itself:
/// answering them paints the bar (Explorer, Notepad++ and Chromium do the same).
pub const WM_UAHDRAWMENU: u32 = 0x0091;
pub const WM_UAHDRAWMENUITEM: u32 = 0x0092;

#[repr(C)]
struct UahMenu {
    hmenu: HMENU,
    hdc: HDC,
    flags: u32,
}

#[repr(C)]
struct UahMenuItem {
    position: i32,
    /// `UAHMENUITEMMETRICS`: two or four size pairs.
    metrics: [u32; 8],
    /// `UAHMENUPOPUPMETRICS`: four widths and a flag word.
    popup: [u32; 5],
}

#[repr(C)]
struct UahDrawMenuItem {
    dis: DRAWITEMSTRUCT,
    um: UahMenu,
    umi: UahMenuItem,
}

/// The menu bar's rectangle in window coordinates (for `GetWindowDC` and the UAH device
/// context), or `None` without a bar.
unsafe fn menu_bar_rect(hwnd: HWND) -> Option<RECT> {
    // SAFETY: a zeroed MENUBARINFO with its size set, and the window's rectangle.
    unsafe {
        let mut mbi: MENUBARINFO = std::mem::zeroed();
        mbi.cbSize = size_of::<MENUBARINFO>() as u32;
        if GetMenuBarInfo(hwnd, OBJID_MENU, 0, &mut mbi) == 0 {
            return None;
        }
        let mut win: RECT = std::mem::zeroed();
        GetWindowRect(hwnd, &mut win);
        Some(RECT {
            left: mbi.rcBar.left - win.left,
            top: mbi.rcBar.top - win.top,
            right: mbi.rcBar.right - win.left,
            bottom: mbi.rcBar.bottom - win.top,
        })
    }
}

/// The menu bar in dark: `WM_UAHDRAWMENU` fills the bar, `WM_UAHDRAWMENUITEM` draws an item.
/// `None` while light, or for other messages.
///
/// # Safety
/// `lparam` is the message's structure.
pub unsafe fn menu_bar_message(hwnd: HWND, msg: u32, lparam: LPARAM) -> Option<LRESULT> {
    if !is_dark() {
        return None;
    }
    // SAFETY: the structures Windows passes with these messages (DARKMODE.md §2).
    unsafe {
        match msg {
            WM_UAHDRAWMENU => {
                let um = &*(lparam as *const UahMenu);
                if let Some(rc) = menu_bar_rect(hwnd) {
                    FillRect(um.hdc, &rc, brush(Brush::Face));
                }
                Some(1)
            }
            WM_UAHDRAWMENUITEM => {
                let d = &*(lparam as *const UahDrawMenuItem);
                let hdc = d.dis.hDC;
                let rc = d.dis.rcItem;
                let st = d.dis.itemState;
                let mut text = [0u16; 256];
                let mut mii: MENUITEMINFOW = std::mem::zeroed();
                mii.cbSize = size_of::<MENUITEMINFOW>() as u32;
                mii.fMask = MIIM_STRING;
                mii.dwTypeData = text.as_mut_ptr();
                mii.cch = text.len() as u32 - 1;
                GetMenuItemInfoW(d.um.hmenu, d.umi.position as u32, 1, &mut mii);
                let fill = if st & (ODS_HOTLIGHT | ODS_SELECTED) != 0 && st & ODS_INACTIVE == 0 {
                    Brush::Hot
                } else {
                    Brush::Face
                };
                FillRect(hdc, &rc, brush(fill));
                let s = sys();
                let gray = st & (ODS_GRAYED | ODS_DISABLED | ODS_INACTIVE) != 0;
                SetTextColor(hdc, if gray { s.gray_text } else { s.window_text });
                SetBkMode(hdc, TRANSPARENT as i32);
                let font = menu_font(hwnd);
                let old = SelectObject(hdc, font as HGDIOBJ);
                let mut text_rc = rc;
                let flags = DT_CENTER
                    | DT_SINGLELINE
                    | DT_VCENTER
                    | DT_NOCLIP
                    | if st & ODS_NOACCEL != 0 { DT_HIDEPREFIX } else { 0 };
                DrawTextW(hdc, text.as_mut_ptr(), -1, &mut text_rc, flags);
                SelectObject(hdc, old);
                DeleteObject(font as HGDIOBJ);
                Some(1)
            }
            _ => None,
        }
    }
}

/// The menu font of the window's DPI; the caller deletes it.
unsafe fn menu_font(hwnd: HWND) -> HFONT {
    // SAFETY: a zeroed NONCLIENTMETRICSW with its size set.
    unsafe {
        let mut ncm: NONCLIENTMETRICSW = std::mem::zeroed();
        ncm.cbSize = size_of::<NONCLIENTMETRICSW>() as u32;
        SystemParametersInfoForDpi(
            SPI_GETNONCLIENTMETRICS,
            ncm.cbSize,
            &mut ncm as *mut _ as *mut c_void,
            0,
            GetDpiForWindow(hwnd),
        );
        CreateFontIndirectW(&ncm.lfMenuFont)
    }
}

/// After `WM_NCPAINT` and `WM_NCACTIVATE` in dark: the one-pixel line Windows leaves under the
/// menu bar in its own colour is painted over.
///
/// # Safety
/// `hwnd` is a window of this thread with a menu bar.
pub unsafe fn menu_bar_line(hwnd: HWND) {
    if !is_dark() {
        return;
    }
    // SAFETY: the window's own device context, released before returning.
    unsafe {
        let Some(bar) = menu_bar_rect(hwnd) else { return };
        let hdc = GetWindowDC(hwnd);
        if hdc.is_null() {
            return;
        }
        let rc = RECT { left: bar.left, top: bar.bottom, right: bar.right, bottom: bar.bottom + 1 };
        FillRect(hdc, &rc, brush(Brush::Face));
        ReleaseDC(hwnd, hdc);
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
