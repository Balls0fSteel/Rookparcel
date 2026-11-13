//! Common dialogs and a small text-input dialog built from an in-memory template (no resource
//! compiler).

use std::mem::{size_of, zeroed};
use std::path::{Path, PathBuf};
use std::ptr::{null, null_mut};

use rookparcel_frontend::tr;
use rookparcel_frontend::ui_lock::Form;
use windows_sys::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows_sys::Win32::System::DataExchange::{CloseClipboard, GetClipboardData, OpenClipboard};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::Memory::{GlobalLock, GlobalUnlock};
use windows_sys::Win32::System::Ole::CF_UNICODETEXT;
use windows_sys::Win32::UI::Controls::Dialogs::{
    GetOpenFileNameW, GetSaveFileNameW, OFN_ALLOWMULTISELECT, OFN_EXPLORER, OFN_FILEMUSTEXIST,
    OFN_HIDEREADONLY, OFN_NOCHANGEDIR, OFN_PATHMUSTEXIST, OPENFILENAMEW,
};
use windows_sys::Win32::UI::HiDpi::SystemParametersInfoForDpi;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    BS_DEFPUSHBUTTON, BS_PUSHBUTTON, DLGTEMPLATE, DS_CENTER, DS_MODALFRAME, DestroyIcon,
    DialogBoxIndirectParamW, ES_AUTOHSCROLL, ES_NUMBER, ES_PASSWORD, EndDialog, GetDlgItem,
    GetWindowLongPtrW, HICON, ICON_BIG, ICON_SMALL, IDCANCEL, IDOK, IDYES, MB_ICONERROR, MB_ICONINFORMATION,
    MB_ICONQUESTION, MB_ICONWARNING, MB_OK, MB_RIGHT, MB_RTLREADING, MB_SETFOREGROUND, MB_USERICON, MB_YESNO,
    MSGBOXPARAMSW, MessageBoxIndirectW, MessageBoxW, NONCLIENTMETRICSW, SPI_GETNONCLIENTMETRICS,
    SendMessageW, SetForegroundWindow, SetWindowLongPtrW, SetWindowTextW, WM_COMMAND, WM_INITDIALOG,
    WM_NEXTDLGCTL, WM_SETICON, WS_BORDER, WS_CAPTION, WS_CHILD, WS_EX_LAYOUTRTL, WS_MAXIMIZEBOX,
    WS_MINIMIZEBOX, WS_POPUP, WS_SYSMENU, WS_TABSTOP, WS_THICKFRAME, WS_VISIBLE,
};

use windows_sys::Win32::UI::Controls::{
    EM_LIMITTEXT, EM_SETSEL, TASKDIALOG_BUTTON, TASKDIALOGCONFIG, TDCBF_CANCEL_BUTTON,
    TDF_ALLOW_DIALOG_CANCELLATION, TDF_POSITION_RELATIVE_TO_WINDOW, TDF_RTL_LAYOUT, TDF_SIZE_TO_CONTENT,
    TaskDialogIndirect,
};

use crate::util::{from_wide, from_wide_ptr, loword, rtl, wide, window_text};

/// File → Add Torrent: one or more `.torrent` files.
pub fn open_torrents(owner: HWND) -> Vec<String> {
    let filter = wide(&torrent_filter());
    let title = wide(tr!(PickTorrent));
    let mut buf = vec![0u16; 64 * 1024];
    unsafe {
        let mut ofn: OPENFILENAMEW = zeroed();
        ofn.lStructSize = size_of::<OPENFILENAMEW>() as u32;
        ofn.hwndOwner = owner;
        ofn.lpstrFilter = filter.as_ptr();
        ofn.lpstrFile = buf.as_mut_ptr();
        ofn.nMaxFile = buf.len() as u32;
        ofn.lpstrTitle = title.as_ptr();
        ofn.Flags = OFN_EXPLORER | OFN_ALLOWMULTISELECT | OFN_FILEMUSTEXIST | OFN_HIDEREADONLY;
        if GetOpenFileNameW(&mut ofn) == 0 {
            return Vec::new();
        }
    }
    // One file: its full path. Several: the folder, then each name, NUL-separated.
    let parts: Vec<String> =
        buf.split(|&c| c == 0).take_while(|s| !s.is_empty()).map(String::from_utf16_lossy).collect();
    match parts.as_slice() {
        [] => Vec::new(),
        [one] => vec![one.clone()],
        [dir, names @ ..] => names.iter().map(|n| format!("{}\\{n}", dir.trim_end_matches('\\'))).collect(),
    }
}

/// The Files tab's Relocate: where file `current` should be, starting at it (its folder when
/// that exists). Choosing an existing file does not ask to replace it: the torrent is pointed
/// at it, and the engine never overwrites one (see `TorrentHandle::rename_file`).
pub fn relocate_file(owner: HWND, current: &Path) -> Option<PathBuf> {
    let filter = wide(&all_files_filter());
    let title = wide(tr!(PickRelocate));
    let dir = current.parent().filter(|d| d.is_dir()).map(|d| wide(&d.to_string_lossy()));
    let mut buf = vec![0u16; 32 * 1024];
    let name = current.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    for (i, c) in name.encode_utf16().take(buf.len() - 1).enumerate() {
        buf[i] = c;
    }
    unsafe {
        let mut ofn: OPENFILENAMEW = zeroed();
        ofn.lStructSize = size_of::<OPENFILENAMEW>() as u32;
        ofn.hwndOwner = owner;
        ofn.lpstrFilter = filter.as_ptr();
        ofn.lpstrFile = buf.as_mut_ptr();
        ofn.nMaxFile = buf.len() as u32;
        ofn.lpstrTitle = title.as_ptr();
        ofn.lpstrInitialDir = dir.as_ref().map_or(null(), |d| d.as_ptr());
        ofn.Flags = OFN_EXPLORER | OFN_PATHMUSTEXIST | OFN_HIDEREADONLY | OFN_NOCHANGEDIR;
        if GetSaveFileNameW(&mut ofn) == 0 {
            return None;
        }
    }
    Some(PathBuf::from(from_wide(&buf)))
}

/// The file dialogs' filter: `.torrent` files, then all files.
pub fn torrent_filter() -> String {
    format!("{}\0*.torrent\0{}\0*.*\0", tr!(FilterTorrents), tr!(FilterAllFiles))
}

/// The file dialogs' filter for any file.
pub fn all_files_filter() -> String {
    format!("{}\0*.*\0", tr!(FilterAllFiles))
}

pub fn error(owner: HWND, text: &str) {
    message(owner, text, MB_OK | MB_ICONERROR);
}

/// Help > About: the text with the application icon.
pub fn about(owner: HWND, text: &str) {
    let (t, c) = (wide(text), wide("Rookparcel"));
    unsafe {
        let mut p: MSGBOXPARAMSW = zeroed();
        p.cbSize = size_of::<MSGBOXPARAMSW>() as u32;
        p.hwndOwner = owner;
        p.hInstance = GetModuleHandleW(null());
        p.lpszText = t.as_ptr();
        p.lpszCaption = c.as_ptr();
        p.dwStyle = MB_OK | MB_USERICON | rtl(MB_RTLREADING | MB_RIGHT);
        p.lpszIcon = crate::icons::app_icon_resource();
        MessageBoxIndirectW(&p);
    }
}

pub fn info(owner: HWND, text: &str) {
    message(owner, text, MB_OK | MB_ICONINFORMATION);
}

pub fn confirm(owner: HWND, text: &str) -> bool {
    message(owner, text, MB_YESNO | MB_ICONWARNING) == IDYES
}

/// A Yes / No question that risks nothing (Help > Check for Updates).
pub fn ask(owner: HWND, text: &str) -> bool {
    message(owner, text, MB_YESNO | MB_ICONQUESTION) == IDYES
}

/// A question with buttons of its own, the first the default, and Cancel: the index of the
/// button chosen, `None` for Cancel, Escape or the close box (a task dialog, with the
/// application icon; the update's "Download and install" / "Open download page").
pub fn choose(owner: HWND, text: &str, buttons: &[&str]) -> Option<usize> {
    if cfg!(test) {
        told(text);
        return None;
    }
    let (t, c) = (wide(text), wide("Rookparcel"));
    let texts: Vec<Vec<u16>> = buttons.iter().map(|b| wide(b)).collect();
    let buttons: Vec<TASKDIALOG_BUTTON> = texts
        .iter()
        .enumerate()
        .map(|(i, b)| TASKDIALOG_BUTTON { nButtonID: 100 + i as i32, pszButtonText: b.as_ptr() })
        .collect();
    unsafe {
        let mut cfg: TASKDIALOGCONFIG = zeroed();
        cfg.cbSize = size_of::<TASKDIALOGCONFIG>() as u32;
        cfg.hwndParent = owner;
        cfg.hInstance = GetModuleHandleW(null());
        cfg.dwFlags = TDF_ALLOW_DIALOG_CANCELLATION
            | TDF_POSITION_RELATIVE_TO_WINDOW
            | TDF_SIZE_TO_CONTENT
            | rtl(TDF_RTL_LAYOUT as u32) as i32;
        cfg.dwCommonButtons = TDCBF_CANCEL_BUTTON;
        cfg.pszWindowTitle = c.as_ptr();
        cfg.Anonymous1.pszMainIcon = crate::icons::app_icon_resource();
        cfg.pszContent = t.as_ptr();
        cfg.cButtons = buttons.len() as u32;
        cfg.pButtons = buttons.as_ptr();
        cfg.nDefaultButton = 100;
        let mut pressed = 0i32;
        if TaskDialogIndirect(&cfg, &mut pressed, null_mut(), null_mut()) < 0 {
            return None;
        }
        (pressed >= 100).then(|| (pressed - 100) as usize)
    }
}

fn message(owner: HWND, text: &str, flags: u32) -> i32 {
    // a test has nobody to close a modal box: it is recorded and answered OK (never Yes)
    if cfg!(test) {
        told(text);
        return IDOK;
    }
    let (t, c) = (wide(text), wide("Rookparcel"));
    unsafe { MessageBoxW(owner, t.as_ptr(), c.as_ptr(), flags | rtl(MB_RTLREADING | MB_RIGHT)) }
}

#[cfg(test)]
thread_local! {
    static TOLD: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
fn told(text: &str) {
    TOLD.with(|t| t.borrow_mut().push(text.to_owned()));
}

#[cfg(not(test))]
fn told(_: &str) {}

/// What this test thread's message boxes said since the last call.
#[cfg(test)]
pub fn take_told() -> Vec<String> {
    TOLD.with(|t| t.take())
}

/// The clipboard's text, if any.
pub fn clipboard_text(owner: HWND) -> Option<String> {
    unsafe {
        if OpenClipboard(owner) == 0 {
            return None;
        }
        let h = GetClipboardData(CF_UNICODETEXT as u32);
        let text = if h.is_null() {
            None
        } else {
            let p = GlobalLock(h) as *const u16;
            let s = from_wide_ptr(p);
            GlobalUnlock(h);
            Some(s)
        };
        CloseClipboard();
        text
    }
}

/// Puts `text` on the clipboard; false when the clipboard is busy.
pub fn set_clipboard_text(owner: HWND, text: &str) -> bool {
    use windows_sys::Win32::Foundation::GlobalFree;
    use windows_sys::Win32::System::DataExchange::{EmptyClipboard, SetClipboardData};
    use windows_sys::Win32::System::Memory::{GMEM_MOVEABLE, GlobalAlloc};
    let w = wide(text);
    unsafe {
        if OpenClipboard(owner) == 0 {
            return false;
        }
        EmptyClipboard();
        let h = GlobalAlloc(GMEM_MOVEABLE, w.len() * 2);
        let mut done = false;
        if !h.is_null() {
            let p = GlobalLock(h) as *mut u16;
            if !p.is_null() {
                std::ptr::copy_nonoverlapping(w.as_ptr(), p, w.len());
                GlobalUnlock(h);
                done = !SetClipboardData(CF_UNICODETEXT as u32, h).is_null();
            }
            if !done {
                GlobalFree(h);
            }
        }
        CloseClipboard();
        done
    }
}

/// The template names its font (`message_font`).
const DS_SETFONT: u32 = 0x40;

/// The system's message font as a template names it: face and point size at 96 DPI (Segoe UI
/// 9 pt since Vista; the dialog manager scales it to the window's DPI). The main window and
/// µTorrent 2.2.1's dialogs use the same, where `MS Shell Dlg` would be Tahoma 8 pt.
fn message_font() -> (String, u16) {
    unsafe {
        let mut ncm: NONCLIENTMETRICSW = zeroed();
        ncm.cbSize = size_of::<NONCLIENTMETRICSW>() as u32;
        let ok = SystemParametersInfoForDpi(
            SPI_GETNONCLIENTMETRICS,
            ncm.cbSize,
            &mut ncm as *mut _ as *mut _,
            0,
            96,
        );
        let lf = &ncm.lfMessageFont;
        if ok == 0 || lf.lfFaceName[0] == 0 || lf.lfHeight >= 0 {
            return ("MS Shell Dlg".to_owned(), 8);
        }
        // a negative height is the character height in pixels at 96 DPI
        ((from_wide(&lf.lfFaceName)), ((-lf.lfHeight * 72 + 48) / 96).max(1) as u16)
    }
}
/// `GetWindowLongPtr` index of the dialog's user data (after the message result and the
/// dialog procedure).
pub const DWLP_USER: i32 = (2 * size_of::<usize>()) as i32;
/// Where a dialog procedure leaves the result of a message such as `WM_NOTIFY` (it returns
/// `TRUE` to say it did).
pub const DWLP_MSGRESULT: i32 = 0;

const ID_EDIT: u16 = 100;
const ID_PROMPT: u16 = 101;

struct InputState {
    title: String,
    prompt: String,
    initial: String,
    result: Option<String>,
}

/// A modal dialog with a prompt, one text field, OK and Cancel. Returns the trimmed text, or
/// `None` on Cancel.
pub fn input(owner: HWND, title: &str, prompt: &str, initial: &str) -> Option<String> {
    let mut state =
        InputState { title: title.into(), prompt: prompt.into(), initial: initial.into(), result: None };
    let template = input_template();
    unsafe {
        DialogBoxIndirectParamW(
            GetModuleHandleW(null()),
            template.as_ptr() as *const DLGTEMPLATE,
            owner,
            Some(input_proc),
            &mut state as *mut InputState as LPARAM,
        );
    }
    state.result.map(|s| s.trim().to_owned())
}

unsafe extern "system" fn input_proc(dlg: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> isize {
    unsafe {
        if let Some(r) = crate::theme::dialog_message(dlg, msg, wp, lp) {
            return r;
        }
        match msg {
            WM_INITDIALOG => {
                SetWindowLongPtrW(dlg, DWLP_USER, lp);
                let st = &*(lp as *const InputState);
                let (t, p, i) = (wide(&st.title), wide(&st.prompt), wide(&st.initial));
                SetWindowTextW(dlg, t.as_ptr());
                SetWindowTextW(GetDlgItem(dlg, ID_PROMPT as i32), p.as_ptr());
                let edit = GetDlgItem(dlg, ID_EDIT as i32);
                SetWindowTextW(edit, i.as_ptr());
                SendMessageW(edit, EM_SETSEL, 0, -1);
                crate::fit::fit_texts(dlg, |_| Some(0));
                // a prompt longer than the dialog is wide (Add Torrent from URL in German)
                // takes a second line, and the dialog grows by it
                use windows_sys::Win32::Foundation::RECT;
                use windows_sys::Win32::UI::WindowsAndMessaging::{
                    GetWindowRect, MapDialogRect, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOZORDER, SetWindowPos,
                };
                let mut line = RECT { left: 0, top: 0, right: 0, bottom: 9 };
                MapDialogRect(dlg, &mut line);
                let grown = crate::fit::fit_note_height(dlg, ID_PROMPT, line.bottom, |_| Some(0));
                if grown > 0 {
                    let mut r: RECT = zeroed();
                    GetWindowRect(dlg, &mut r);
                    let flags = SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE;
                    SetWindowPos(dlg, null_mut(), 0, 0, r.right - r.left, r.bottom - r.top + grown, flags);
                }
                1
            }
            WM_COMMAND => {
                let id = loword(wp) as i32;
                if id == IDOK || id == IDCANCEL {
                    let st = GetWindowLongPtrW(dlg, DWLP_USER) as *mut InputState;
                    if id == IDOK && !st.is_null() {
                        (*st).result = Some(window_text(GetDlgItem(dlg, ID_EDIT as i32)));
                    }
                    EndDialog(dlg, id as isize);
                    return 1;
                }
                0
            }
            _ => 0,
        }
    }
}

fn input_template() -> Vec<u16> {
    let mut t = Template::new(280, 62);
    t.item(STATIC, ID_PROMPT, WS_CHILD | WS_VISIBLE, [7, 7, 266, 9], "");
    t.item(
        EDIT,
        ID_EDIT,
        WS_CHILD | WS_VISIBLE | WS_BORDER | WS_TABSTOP | ES_AUTOHSCROLL as u32,
        [7, 19, 266, 13],
        "",
    );
    t.item(
        BUTTON,
        IDOK as u16,
        WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_DEFPUSHBUTTON as u32,
        [169, 40, 50, 14],
        tr!(BtnOk),
    );
    t.item(
        BUTTON,
        IDCANCEL as u16,
        WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_PUSHBUTTON as u32,
        [223, 40, 50, 14],
        tr!(BtnCancel),
    );
    t.finish()
}

// ---- The windows' lock (D69): one dialog for the unlock prompt and Set / Change / Remove PIN

const ID_PIN_CURRENT: u16 = 110;
const ID_PIN_NEW: u16 = 111;
const ID_PIN_REPEAT: u16 = 112;
/// The line under the boxes that says what was wrong ("Wrong PIN.", the wait).
const ID_PIN_MESSAGE: u16 = 113;
/// The boxes' labels, by box index.
const ID_PIN_LABEL: u16 = 120;

/// What the PIN dialog's OK does with what was typed in the current, new and repeated PIN's
/// boxes (empty for a box the form lacks): `None` when there is nothing to act on any more (the
/// program is exiting; the dialog closes as Cancel), else `ui_lock::submit`'s answer, whose
/// `Err` the dialog shows under its boxes, staying open.
pub type PinSubmit<'a> = &'a mut dyn FnMut(&str, &str, &str) -> Option<Result<(), String>>;

struct PinState<'a> {
    form: Form,
    submit: PinSubmit<'a>,
    done: bool,
    /// The application icon for the caption and the taskbar button of the unlock prompt,
    /// which has no owner (destroyed once the dialog has closed).
    icons: [HICON; 2],
}

thread_local! {
    /// The unlock prompt while it is open.
    static PROMPT: std::cell::Cell<HWND> = const { std::cell::Cell::new(null_mut()) };
}

/// The unlock prompt ([`Form::Unlock`]) while it is open, else null.
pub fn pin_prompt() -> HWND {
    PROMPT.with(|p| p.get())
}

/// Closes the open unlock prompt as Cancel (Exit chosen meanwhile, or the PIN reset).
pub fn close_pin_prompt() {
    let dlg = pin_prompt();
    if !dlg.is_null() {
        unsafe { EndDialog(dlg, IDCANCEL as isize) };
        // ended: a request from now on is a new one
        PROMPT.with(|p| p.set(null_mut()));
    }
}

/// The PIN dialog for `form` (`ui_lock::Form`: the boxes it asks), modal on `owner`; the unlock
/// prompt has none, so that it never enables a window a hidden dialog disabled, and comes to
/// the front. True when OK's `submit` succeeded.
pub fn pin(owner: HWND, form: Form, submit: PinSubmit<'_>) -> bool {
    let mut state = PinState { form, submit, done: false, icons: [null_mut(); 2] };
    let template = pin_template(form);
    unsafe {
        DialogBoxIndirectParamW(
            GetModuleHandleW(null()),
            template.as_ptr() as *const DLGTEMPLATE,
            owner,
            Some(pin_proc),
            &mut state as *mut PinState<'_> as LPARAM,
        );
        if form == Form::Unlock {
            PROMPT.with(|p| p.set(null_mut()));
        }
        for icon in state.icons.into_iter().filter(|i| !i.is_null()) {
            DestroyIcon(icon);
        }
    }
    state.done
}

/// The ids of `form`'s boxes, top down.
fn pin_boxes(form: Form) -> Vec<u16> {
    let mut boxes = Vec::new();
    if form.asks_current() {
        boxes.push(ID_PIN_CURRENT);
    }
    if form.asks_new() {
        boxes.extend([ID_PIN_NEW, ID_PIN_REPEAT]);
    }
    boxes
}

fn pin_label(form: Form, id: u16) -> &'static str {
    match id {
        ID_PIN_CURRENT => form.current_label(),
        ID_PIN_NEW => tr!(PinNew),
        _ => tr!(PinRepeat),
    }
}

/// A label above each box (a line of its own, so a long translation wraps and the dialog
/// grows), the message line, OK and Cancel.
fn pin_template(form: Form) -> Vec<u16> {
    const W: i16 = 230;
    let boxes = pin_boxes(form);
    let height = 7 + 28 * boxes.len() as i16 + 20 + 14 + 7;
    let mut t = Template::new(W, height);
    let mut y = 7;
    for (i, &id) in boxes.iter().enumerate() {
        t.item(
            STATIC,
            ID_PIN_LABEL + i as u16,
            WS_CHILD | WS_VISIBLE | SS_NOPREFIX,
            [7, y, W - 14, 9],
            pin_label(form, id),
        );
        let style = WS_CHILD
            | WS_VISIBLE
            | WS_BORDER
            | WS_TABSTOP
            | ES_AUTOHSCROLL as u32
            | ES_PASSWORD as u32
            | ES_NUMBER as u32;
        t.item(EDIT, id, style, [7, y + 11, 70, 13], "");
        y += 28;
    }
    t.item(STATIC, ID_PIN_MESSAGE, WS_CHILD | WS_VISIBLE | SS_NOPREFIX, [7, y, W - 14, 18], "");
    y += 20;
    t.item(
        BUTTON,
        IDOK as u16,
        WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_DEFPUSHBUTTON as u32,
        [W - 111, y, 50, 14],
        tr!(BtnOk),
    );
    t.item(
        BUTTON,
        IDCANCEL as u16,
        WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_PUSHBUTTON as u32,
        [W - 57, y, 50, 14],
        tr!(BtnCancel),
    );
    t.finish()
}

/// `SS_NOPREFIX`: an `&` in a translation shows as itself.
const SS_NOPREFIX: u32 = 0x80;

unsafe extern "system" fn pin_proc(dlg: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> isize {
    unsafe {
        if let Some(r) = crate::theme::dialog_message(dlg, msg, wp, lp) {
            return r;
        }
        match msg {
            WM_INITDIALOG => {
                SetWindowLongPtrW(dlg, DWLP_USER, lp);
                let st = &mut *(lp as *mut PinState<'_>);
                let title = wide(st.form.title());
                SetWindowTextW(dlg, title.as_ptr());
                let boxes = pin_boxes(st.form);
                for &id in &boxes {
                    SendMessageW(
                        GetDlgItem(dlg, id as i32),
                        EM_LIMITTEXT,
                        rookparcel_frontend::ui_lock::MAX_DIGITS,
                        0,
                    );
                }
                crate::fit::fit_texts(dlg, |_| Some(0));
                // a label longer than the dialog is wide takes more lines, and the dialog grows
                use windows_sys::Win32::Foundation::RECT;
                use windows_sys::Win32::UI::WindowsAndMessaging::{
                    GetWindowRect, MapDialogRect, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOZORDER, SetWindowPos,
                };
                let mut line = RECT { left: 0, top: 0, right: 0, bottom: 9 };
                MapDialogRect(dlg, &mut line);
                let mut grown = 0;
                for i in 0..boxes.len() {
                    grown +=
                        crate::fit::fit_note_height(dlg, ID_PIN_LABEL + i as u16, line.bottom, |_| Some(0));
                }
                if grown > 0 {
                    let mut r: RECT = zeroed();
                    GetWindowRect(dlg, &mut r);
                    let flags = SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE;
                    SetWindowPos(dlg, null_mut(), 0, 0, r.right - r.left, r.bottom - r.top + grown, flags);
                }
                if st.form == Form::Unlock {
                    PROMPT.with(|p| p.set(dlg));
                    let dpi = windows_sys::Win32::UI::HiDpi::GetDpiForWindow(dlg);
                    st.icons = [
                        crate::icons::app_icon(crate::util::scale(32, dpi) as usize),
                        crate::icons::app_icon(crate::util::scale(16, dpi) as usize),
                    ];
                    SendMessageW(dlg, WM_SETICON, ICON_BIG as usize, st.icons[0] as LPARAM);
                    SendMessageW(dlg, WM_SETICON, ICON_SMALL as usize, st.icons[1] as LPARAM);
                    // started from a second start or the tray: in front of the window it came from
                    SetForegroundWindow(dlg);
                }
                1
            }
            WM_COMMAND => {
                let id = loword(wp) as i32;
                if id == IDCANCEL {
                    EndDialog(dlg, IDCANCEL as isize);
                    return 1;
                }
                if id != IDOK {
                    return 0;
                }
                let st = GetWindowLongPtrW(dlg, DWLP_USER) as *mut PinState<'_>;
                if st.is_null() {
                    return 0;
                }
                let text = |id: u16| {
                    let h = GetDlgItem(dlg, id as i32);
                    if h.is_null() { String::new() } else { window_text(h) }
                };
                let (current, new, repeat) = (text(ID_PIN_CURRENT), text(ID_PIN_NEW), text(ID_PIN_REPEAT));
                match ((*st).submit)(&current, &new, &repeat) {
                    None => {
                        EndDialog(dlg, IDCANCEL as isize);
                    }
                    Some(Ok(())) => {
                        (*st).done = true;
                        EndDialog(dlg, IDOK as isize);
                    }
                    Some(Err(message)) => {
                        let m = wide(&message);
                        SetWindowTextW(GetDlgItem(dlg, ID_PIN_MESSAGE as i32), m.as_ptr());
                        // the boxes the message is about are emptied, the first takes the keys
                        let new_pin = message == tr!(PinBadLength) || message == tr!(PinMismatch);
                        let again: &[u16] =
                            if new_pin { &[ID_PIN_NEW, ID_PIN_REPEAT] } else { &[ID_PIN_CURRENT] };
                        for &b in again {
                            SetWindowTextW(GetDlgItem(dlg, b as i32), [0u16].as_ptr());
                        }
                        let first = GetDlgItem(dlg, again[0] as i32);
                        if !first.is_null() {
                            SendMessageW(dlg, WM_NEXTDLGCTL, first as WPARAM, 1);
                        }
                    }
                }
                1
            }
            _ => 0,
        }
    }
}

/// A Yes / No question without an owner, in front (asked before the main window exists, or for
/// a forwarded reset while every window is hidden).
pub fn ask_front(text: &str) -> bool {
    message(null_mut(), text, MB_YESNO | MB_ICONQUESTION | MB_SETFOREGROUND) == IDYES
}

/// An information box without an owner, in front.
pub fn info_front(text: &str) {
    message(null_mut(), text, MB_OK | MB_ICONINFORMATION | MB_SETFOREGROUND);
}

/// Predefined dialog control classes (atoms) and common-control classes by name.
pub enum Class {
    Atom(u16),
    Name(&'static str),
}
pub const BUTTON: Class = Class::Atom(0x0080);
pub const EDIT: Class = Class::Atom(0x0081);
pub const STATIC: Class = Class::Atom(0x0082);
pub const COMBOBOX: Class = Class::Atom(0x0085);

/// `DLGTEMPLATE` + items, as the dialog manager reads them (UTF-16 words, items DWORD-aligned).
/// Positions are dialog units; the dialog uses the system's message font and is centred on its
/// owner.
pub struct Template {
    t: Vec<u16>,
    count_at: usize,
}

impl Template {
    pub fn new(cx: i16, cy: i16) -> Template {
        let mut t = Template { t: Vec::new(), count_at: 0 };
        let style = WS_POPUP | WS_CAPTION | WS_SYSMENU | DS_MODALFRAME as u32 | DS_CENTER as u32 | DS_SETFONT;
        t.dword(style);
        // a dialog has an owner, so the process's right-to-left default does not reach it
        t.dword(rtl(WS_EX_LAYOUTRTL));
        t.count_at = t.t.len();
        t.t.push(0); // items, set by `item`
        t.t.extend([0, 0, cx, cy].map(|v: i16| v as u16));
        t.t.push(0); // no menu
        t.t.push(0); // default class
        t.text("");
        let (face, points) = message_font();
        t.t.push(points);
        t.text(&face);
        t
    }

    /// One control: class, id, style, [x, y, cx, cy] and its text.
    pub fn item(&mut self, class: Class, id: u16, style: u32, rect: [i16; 4], label: &str) {
        self.t[self.count_at] += 1;
        if self.t.len() % 2 == 1 {
            self.t.push(0);
        }
        self.dword(style);
        self.dword(0);
        self.t.extend(rect.map(|v| v as u16));
        self.t.push(id);
        match class {
            Class::Atom(a) => self.t.extend([0xffff, a]),
            Class::Name(n) => self.text(n),
        }
        self.text(label);
        self.t.push(0); // no creation data
    }

    /// A window the user can resize and maximize (the Search Results window) instead of a
    /// fixed dialog frame; its procedure lays the controls out on `WM_SIZE`.
    pub fn resizable(&mut self) {
        let style = (self.t[0] as u32 | (self.t[1] as u32) << 16) & !(DS_MODALFRAME as u32)
            | WS_THICKFRAME
            | WS_MAXIMIZEBOX
            | WS_MINIMIZEBOX;
        self.t[0] = style as u16;
        self.t[1] = (style >> 16) as u16;
    }

    pub fn finish(self) -> Vec<u16> {
        self.t
    }

    fn dword(&mut self, v: u32) {
        self.t.push(v as u16);
        self.t.push((v >> 16) as u16);
    }

    fn text(&mut self, s: &str) {
        self.t.extend(s.encode_utf16());
        self.t.push(0);
    }
}

/// Reads a NUL-separated list (WM_COPYDATA payloads) into strings.
pub fn split_nul(buf: &[u16]) -> Vec<String> {
    buf.split(|&c| c == 0).filter(|s| !s.is_empty()).map(String::from_utf16_lossy).collect()
}

#[cfg(test)]
mod tests {
    use std::ptr::null_mut;

    use windows_sys::Win32::Graphics::Gdi::{GetObjectW, HFONT, LOGFONTW};
    use windows_sys::Win32::UI::Controls::EM_GETLIMITTEXT;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CreateDialogIndirectParamW, DestroyWindow, WM_GETFONT,
    };

    use super::*;

    unsafe extern "system" fn nop(_: HWND, _: u32, _: WPARAM, _: LPARAM) -> isize {
        0
    }

    #[test]
    fn templates_use_the_system_message_font() {
        let (face, points) = message_font();
        assert!(!face.is_empty() && face != "MS Shell Dlg", "{face}");
        assert!((8..=12).contains(&points), "{points} pt");
        let mut t = Template::new(200, 100);
        t.item(BUTTON, IDOK as u16, WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON as u32, [7, 7, 50, 14], "OK");
        let template = t.finish();
        unsafe {
            let dlg = CreateDialogIndirectParamW(
                GetModuleHandleW(null()),
                template.as_ptr() as *const DLGTEMPLATE,
                null_mut(),
                Some(nop),
                0,
            );
            assert!(!dlg.is_null());
            let font = SendMessageW(dlg, WM_GETFONT, 0, 0) as HFONT;
            let mut lf: LOGFONTW = zeroed();
            GetObjectW(font, size_of::<LOGFONTW>() as i32, &mut lf as *mut _ as *mut _);
            assert_eq!(from_wide(&lf.lfFaceName), face);
            // the button's font too
            let bf = SendMessageW(GetDlgItem(dlg, IDOK), WM_GETFONT, 0, 0) as HFONT;
            assert_eq!(bf, font);
            DestroyWindow(dlg);
        }
    }

    /// The PIN dialog (D69) has the boxes its form asks for, takes 6 digits at most, hands OK
    /// what was typed and shows the answer's message, emptying the box it is about. Made
    /// modeless and never shown (Change PIN: the unlock prompt would come to the front).
    #[test]
    fn the_pin_dialog_asks_its_boxes_and_says_what_was_wrong() {
        assert_eq!(pin_boxes(Form::Unlock), [ID_PIN_CURRENT]);
        assert_eq!(pin_boxes(Form::Set), [ID_PIN_NEW, ID_PIN_REPEAT]);
        assert_eq!(pin_boxes(Form::Change), [ID_PIN_CURRENT, ID_PIN_NEW, ID_PIN_REPEAT]);
        assert_eq!(pin_boxes(Form::Remove), [ID_PIN_CURRENT]);
        let seen = std::cell::RefCell::new(Vec::new());
        let mut submit = |c: &str, n: &str, r: &str| {
            seen.borrow_mut().push(format!("{c}|{n}|{r}"));
            Some(match (c, n == r) {
                ("1234", true) => Ok(()),
                ("1234", false) => Err(tr!(PinMismatch).to_owned()),
                _ => Err(tr!(LockWrong).to_owned()),
            })
        };
        let mut state =
            PinState { form: Form::Change, submit: &mut submit, done: false, icons: [null_mut(); 2] };
        let template = pin_template(Form::Change);
        unsafe {
            let dlg = CreateDialogIndirectParamW(
                GetModuleHandleW(null()),
                template.as_ptr() as *const DLGTEMPLATE,
                null_mut(),
                Some(pin_proc),
                &mut state as *mut PinState<'_> as LPARAM,
            );
            assert!(!dlg.is_null());
            let text = |id: u16| window_text(GetDlgItem(dlg, id as i32));
            let set = |id: u16, s: &str| {
                let w = wide(s);
                SetWindowTextW(GetDlgItem(dlg, id as i32), w.as_ptr());
            };
            assert_eq!(window_text(dlg), tr!(PinChangeTitle));
            assert_eq!(text(ID_PIN_LABEL), tr!(PinCurrent));
            assert_eq!(text(ID_PIN_LABEL + 1), tr!(PinNew));
            assert_eq!(text(ID_PIN_LABEL + 2), tr!(PinRepeat));
            assert_eq!(SendMessageW(GetDlgItem(dlg, ID_PIN_CURRENT as i32), EM_GETLIMITTEXT, 0, 0), 6);
            assert!(text(ID_PIN_MESSAGE).is_empty());
            set(ID_PIN_CURRENT, "1111");
            set(ID_PIN_NEW, "5678");
            set(ID_PIN_REPEAT, "5678");
            SendMessageW(dlg, WM_COMMAND, IDOK as usize, 0);
            assert_eq!(text(ID_PIN_MESSAGE), tr!(LockWrong));
            assert!(text(ID_PIN_CURRENT).is_empty(), "the wrong PIN goes");
            assert_eq!(text(ID_PIN_NEW), "5678", "the new one stays");
            set(ID_PIN_CURRENT, "1234");
            set(ID_PIN_REPEAT, "567");
            SendMessageW(dlg, WM_COMMAND, IDOK as usize, 0);
            assert_eq!(text(ID_PIN_MESSAGE), tr!(PinMismatch));
            assert!(text(ID_PIN_NEW).is_empty() && text(ID_PIN_REPEAT).is_empty());
            assert_eq!(text(ID_PIN_CURRENT), "1234");
            assert!(!state_done(dlg));
            set(ID_PIN_NEW, "5678");
            set(ID_PIN_REPEAT, "5678");
            SendMessageW(dlg, WM_COMMAND, IDOK as usize, 0);
            assert!(state_done(dlg));
            DestroyWindow(dlg);
        }
        assert_eq!(*seen.borrow(), ["1111|5678|5678", "1234|5678|567", "1234|5678|5678"]);
        assert!(pin_prompt().is_null(), "only the unlock prompt is the prompt");
    }

    unsafe fn state_done(dlg: HWND) -> bool {
        unsafe { (*(GetWindowLongPtrW(dlg, DWLP_USER) as *const PinState<'_>)).done }
    }
}
