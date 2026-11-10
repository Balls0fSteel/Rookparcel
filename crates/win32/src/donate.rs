//! Help > Donate: a small dialog with a few lines on supporting the project, a button that
//! opens the website's donation page (`update::DONATE_URL`) in the browser, and Close (Escape
//! closes too). The ways to give are on that page, so they can change without a new program.
//! The dialog sizes itself to its text, which differs in length between the languages.

use std::mem::zeroed;
use std::ptr::{null, null_mut};

use rookparcel_frontend::{lang, tr, update};
use windows_sys::Win32::Foundation::{HWND, LPARAM, RECT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{
    DT_CALCRECT, DT_NOPREFIX, DT_SINGLELINE, DT_WORDBREAK, DrawTextW, GetDC, HFONT, ReleaseDC, SelectObject,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AdjustWindowRectEx, BS_DEFPUSHBUTTON, BS_PUSHBUTTON, DLGTEMPLATE, DialogBoxIndirectParamW, EndDialog,
    GWL_EXSTYLE, GWL_STYLE, GetDlgItem, GetWindowLongW, GetWindowRect, IDCANCEL, IDOK, MapDialogRect,
    MoveWindow, SWP_NOACTIVATE, SWP_NOZORDER, SetWindowPos, SetWindowTextW, WM_COMMAND, WM_GETFONT,
    WM_INITDIALOG, WS_CHILD, WS_TABSTOP, WS_VISIBLE,
};

use crate::dialogs::{BUTTON, STATIC, Template};
use crate::util::{loword, send, wide};

const ID_TEXT: u16 = 100;
/// Open Donation Page is the dialog's OK (Enter), Close its Cancel (Escape).
const ID_OPEN: u16 = IDOK as u16;
const ID_CLOSE: u16 = IDCANCEL as u16;

/// `SS_NOPREFIX`: the text is shown as it is.
const SS_NOPREFIX: u32 = 0x80;

/// In dialog units: the margins, the text's width, the gap above the buttons and between
/// them, a button's height, its smallest width and the room around its label.
const MARGIN: i32 = 7;
const TEXT_W: i32 = 230;
const GAP_Y: i32 = 10;
const GAP_X: i32 = 4;
const BUTTON_H: i32 = 14;
const BUTTON_MIN_W: i32 = 50;
const BUTTON_PAD: i32 = 12;
/// The template's size; `layout` gives the dialog the one its texts need.
const WIDTH: i16 = (TEXT_W + 2 * MARGIN) as i16;
const HEIGHT: i16 = 110;

/// The window title: the menu item without its mnemonic.
pub fn title() -> String {
    lang::plain(tr!(MenuDonate))
}

/// Shows the dialog, modal to `owner`, and opens the donation page if asked to.
pub fn show(owner: HWND) {
    let template = template();
    let pressed = unsafe {
        DialogBoxIndirectParamW(
            GetModuleHandleW(null()),
            template.as_ptr() as *const DLGTEMPLATE,
            owner,
            Some(proc_),
            0,
        )
    };
    if pressed == IDOK as isize {
        crate::app::open_url(owner, update::DONATE_URL);
    }
}

fn template() -> Vec<u16> {
    let mut t = Template::new(WIDTH, HEIGHT);
    let button = WS_CHILD | WS_VISIBLE | WS_TABSTOP;
    let y = HEIGHT as i32 - MARGIN - BUTTON_H;
    t.item(
        STATIC,
        ID_TEXT,
        WS_CHILD | WS_VISIBLE | SS_NOPREFIX,
        [MARGIN, MARGIN, TEXT_W, y - GAP_Y - MARGIN].map(|v| v as i16),
        tr!(DonateText),
    );
    // the first button has the focus when the dialog opens
    t.item(
        BUTTON,
        ID_OPEN,
        button | BS_DEFPUSHBUTTON as u32,
        [MARGIN, y, BUTTON_MIN_W, BUTTON_H].map(|v| v as i16),
        tr!(DonateOpen),
    );
    t.item(
        BUTTON,
        ID_CLOSE,
        button | BS_PUSHBUTTON as u32,
        [MARGIN + BUTTON_MIN_W + GAP_X, y, BUTTON_MIN_W, BUTTON_H].map(|v| v as i16),
        tr!(BtnClose),
    );
    t.finish()
}

/// Dialog units to pixels for this dialog's font.
unsafe fn pixels(dlg: HWND, x: i32, y: i32) -> (i32, i32) {
    let mut r = RECT { left: 0, top: 0, right: x, bottom: y };
    unsafe { MapDialogRect(dlg, &mut r) };
    (r.right, r.bottom)
}

/// The size `text` takes in the dialog's font: on one line (a button's label, its & not
/// counted), or wrapped at `wrap` pixels (the text).
unsafe fn measure(dlg: HWND, text: &str, wrap: Option<i32>) -> (i32, i32) {
    unsafe {
        let dc = GetDC(dlg);
        let old = SelectObject(dc, send(dlg, WM_GETFONT, 0, 0) as HFONT);
        let mut r = RECT { left: 0, top: 0, right: wrap.unwrap_or(0), bottom: 0 };
        let flags = match wrap {
            Some(_) => DT_CALCRECT | DT_WORDBREAK | DT_NOPREFIX,
            None => DT_CALCRECT | DT_SINGLELINE,
        };
        let w = wide(text);
        DrawTextW(dc, w.as_ptr(), -1, &mut r, flags);
        SelectObject(dc, old);
        ReleaseDC(dlg, dc);
        (r.right, r.bottom)
    }
}

/// Sizes the dialog to its texts: each button as wide as its label needs, both in the bottom
/// right corner, the text above them at the height its lines take, and the window around
/// them, kept centred where the template put it.
unsafe fn layout(dlg: HWND) {
    unsafe {
        let (m, my) = pixels(dlg, MARGIN, MARGIN);
        let (gap_x, gap_y) = pixels(dlg, GAP_X, GAP_Y);
        let (min_w, bh) = pixels(dlg, BUTTON_MIN_W, BUTTON_H);
        let (pad, _) = pixels(dlg, BUTTON_PAD, 0);
        let button_w = |text: &str| (measure(dlg, text, None).0 + pad).max(min_w);
        let (open_w, close_w) = (button_w(tr!(DonateOpen)), button_w(tr!(BtnClose)));
        let inner = pixels(dlg, TEXT_W, 0).0.max(open_w + gap_x + close_w);
        let (_, text_h) = measure(dlg, tr!(DonateText), Some(inner));
        let (cw, ch) = (inner + 2 * m, my + text_h + gap_y + bh + my);

        let mut want = RECT { left: 0, top: 0, right: cw, bottom: ch };
        let style = GetWindowLongW(dlg, GWL_STYLE) as u32;
        let ex = GetWindowLongW(dlg, GWL_EXSTYLE) as u32;
        AdjustWindowRectEx(&mut want, style, 0, ex);
        let (w, h) = (want.right - want.left, want.bottom - want.top);
        let mut now: RECT = zeroed();
        GetWindowRect(dlg, &mut now);
        let x = now.left + (now.right - now.left - w) / 2;
        let y = now.top + (now.bottom - now.top - h) / 2;
        SetWindowPos(dlg, null_mut(), x, y, w, h, SWP_NOZORDER | SWP_NOACTIVATE);

        MoveWindow(GetDlgItem(dlg, ID_TEXT as i32), m, my, inner, text_h, 1);
        let by = ch - my - bh;
        MoveWindow(GetDlgItem(dlg, ID_CLOSE as i32), cw - m - close_w, by, close_w, bh, 1);
        MoveWindow(GetDlgItem(dlg, ID_OPEN as i32), cw - m - close_w - gap_x - open_w, by, open_w, bh, 1);
    }
}

unsafe extern "system" fn proc_(dlg: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> isize {
    unsafe {
        if let Some(r) = crate::theme::dialog_message(dlg, msg, wp, lp) {
            return r;
        }
        match msg {
            WM_INITDIALOG => {
                SetWindowTextW(dlg, wide(&title()).as_ptr());
                layout(dlg);
                1
            }
            WM_COMMAND if matches!(loword(wp), ID_OPEN | ID_CLOSE) => {
                EndDialog(dlg, loword(wp) as isize);
                1
            }
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    //! The dialog built from its template, hidden and modeless.

    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::Graphics::Gdi::MapWindowPoints;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CreateDialogIndirectParamW, DestroyWindow, GetClientRect,
    };

    use super::*;
    use crate::util::window_text;

    struct Dialog(HWND);

    impl Drop for Dialog {
        fn drop(&mut self) {
            unsafe { DestroyWindow(self.0) };
        }
    }

    fn open() -> Dialog {
        let template = template();
        let dlg = unsafe {
            CreateDialogIndirectParamW(
                GetModuleHandleW(null()),
                template.as_ptr() as *const DLGTEMPLATE,
                null_mut(),
                Some(proc_),
                0,
            )
        };
        assert!(!dlg.is_null(), "the template does not load");
        Dialog(dlg)
    }

    /// A control's rectangle in the dialog's client area.
    unsafe fn rect(dlg: HWND, id: u16) -> RECT {
        unsafe {
            let mut r: RECT = zeroed();
            GetWindowRect(GetDlgItem(dlg, id as i32), &mut r);
            MapWindowPoints(null_mut(), dlg, &mut r as *mut RECT as *mut POINT, 2);
            // a mirrored dialog maps left and right swapped
            if r.left > r.right {
                std::mem::swap(&mut r.left, &mut r.right);
            }
            r
        }
    }

    #[test]
    fn shows_the_text_and_both_buttons() {
        let d = open();
        unsafe {
            assert_eq!(window_text(d.0), title());
            assert_eq!(title(), "Donate");
            assert_eq!(window_text(GetDlgItem(d.0, ID_TEXT as i32)), tr!(DonateText));
            assert_eq!(window_text(GetDlgItem(d.0, ID_OPEN as i32)), tr!(DonateOpen));
            assert_eq!(window_text(GetDlgItem(d.0, ID_CLOSE as i32)), tr!(BtnClose));
            assert!(update::DONATE_URL.starts_with(update::WEBSITE_URL));
        }
    }

    #[test]
    fn the_dialog_fits_its_texts() {
        let d = open();
        unsafe {
            let mut c: RECT = zeroed();
            GetClientRect(d.0, &mut c);
            let (text, open, close) = (rect(d.0, ID_TEXT), rect(d.0, ID_OPEN), rect(d.0, ID_CLOSE));
            let (m, my) = pixels(d.0, MARGIN, MARGIN);
            // the text at the size its wrapped lines take
            let (_, need) = measure(d.0, tr!(DonateText), Some(text.right - text.left));
            assert_eq!(text.bottom - text.top, need);
            assert!(need > 4 * measure(d.0, "X", None).1, "several lines: {need}");
            // the buttons wide enough for their labels, side by side under the text
            for (r, label) in [(open, tr!(DonateOpen)), (close, tr!(BtnClose))] {
                assert!(r.right - r.left >= measure(d.0, label, None).0 + 8, "{label}");
                assert!(r.top > text.bottom && r.bottom == c.bottom - my, "{label}");
                assert!(r.left >= m && r.right <= c.right - m, "{label} inside the margins");
            }
            assert!(open.right < close.left, "Open before Close");
            assert_eq!((text.left, text.top, text.right), (m, my, c.right - m));
        }
    }
}
