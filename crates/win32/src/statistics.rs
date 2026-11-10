//! Help > Statistics (D65): a small modeless window with the all-time and session totals
//! (`rookparcel_frontend::statistics::rows`), one group box per heading with its labels on the
//! left and their figures right-aligned in a second column, and Close (Enter and Escape close
//! too). The main window pushes every `Alert::SessionStats` to it ([`update`]), so the figures
//! follow the session about once a second. There is one window: [`show`] brings the open one
//! forward. It sizes itself to its texts, which differ in length between the languages, and
//! grows when a figure outgrows its column.

use std::cell::Cell;
use std::mem::zeroed;
use std::ptr::{null, null_mut};

use rookparcel_core::SessionStats;
use rookparcel_frontend::statistics::{Row, rows};
use rookparcel_frontend::{lang, tr};
use windows_sys::Win32::Foundation::{HWND, LPARAM, RECT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{
    DT_CALCRECT, DT_NOPREFIX, DT_SINGLELINE, DrawTextW, GetDC, HFONT, ReleaseDC, SelectObject,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AdjustWindowRectEx, BS_DEFPUSHBUTTON, BS_GROUPBOX, CreateDialogIndirectParamW, DLGTEMPLATE,
    DestroyWindow, GWL_EXSTYLE, GWL_STYLE, GetClientRect, GetDlgItem, GetWindowLongW, GetWindowRect,
    IDCANCEL, IDOK, IsIconic, MapDialogRect, MoveWindow, SW_RESTORE, SW_SHOW, SWP_NOACTIVATE, SWP_NOZORDER,
    SetForegroundWindow, SetWindowPos, SetWindowTextW, ShowWindow, WM_COMMAND, WM_DESTROY, WM_GETFONT,
    WM_INITDIALOG, WS_CHILD, WS_TABSTOP, WS_VISIBLE,
};

use crate::dialogs::{BUTTON, STATIC, Template};
use crate::util::{loword, send, wide, window_text};

/// Close is the dialog's Cancel (Escape); Enter presses it as the default button.
const ID_CLOSE: u16 = IDCANCEL as u16;
/// A row's controls, by its index in `rows`: a heading's group box, a figure's label and value.
const ID_GROUP: u16 = 100;
const ID_LABEL: u16 = 200;
const ID_VALUE: u16 = 300;

/// Static styles (in `System::SystemServices`, a feature we do not otherwise need): the value
/// right-aligned, the label on one line, both shown as they are.
const SS_RIGHT: u32 = 2;
const SS_LEFTNOWORDWRAP: u32 = 0xc;
const SS_NOPREFIX: u32 = 0x80;

/// In dialog units: the margins, a group box's room left and right of its rows, above the first
/// (its title) and under the last, the gap between two rows, between the label and the value
/// column, between two boxes and above the button, the value column's smallest width, a
/// button's height, its smallest width and the room around its label.
const MARGIN: i32 = 7;
const GROUP_PAD: i32 = 8;
const GROUP_TOP: i32 = 12;
const GROUP_BOTTOM: i32 = 7;
const ROW_GAP: i32 = 3;
const COLUMN_GAP: i32 = 14;
const GROUP_GAP: i32 = 4;
const BUTTON_GAP: i32 = 7;
const VALUE_MIN_W: i32 = 60;
const BUTTON_H: i32 = 14;
const BUTTON_MIN_W: i32 = 50;
const BUTTON_PAD: i32 = 12;
/// The template's size; `layout` gives the window the one its texts need.
const WIDTH: i16 = 200;
const HEIGHT: i16 = 200;

thread_local! {
    /// The open window (null: none). Windows belong to the GUI thread.
    static OPEN: Cell<HWND> = const { Cell::new(null_mut()) };
}

/// The window title: the menu item without its mnemonic.
pub fn title() -> String {
    lang::plain(tr!(MenuStatistics))
}

/// The open Statistics window, or null: the message loop offers it its keys
/// (`IsDialogMessageW`).
pub fn window() -> HWND {
    OPEN.with(Cell::get)
}

/// Opens the window owned by `owner` with `stats`, or brings the open one forward; returns it
/// (null when it could not be made).
pub fn show(owner: HWND, stats: &SessionStats) -> HWND {
    let open = window();
    unsafe {
        if !open.is_null() {
            update(stats);
            if !cfg!(test) {
                if IsIconic(open) != 0 {
                    ShowWindow(open, SW_RESTORE);
                }
                SetForegroundWindow(open);
            }
            return open;
        }
        let template = template(&rows(stats));
        let dlg = CreateDialogIndirectParamW(
            GetModuleHandleW(null()),
            template.as_ptr() as *const DLGTEMPLATE,
            owner,
            Some(proc_),
            stats as *const SessionStats as LPARAM,
        );
        if !dlg.is_null() && !cfg!(test) {
            ShowWindow(dlg, SW_SHOW);
        }
        dlg
    }
}

/// The session's new figures: into the open window, if there is one.
pub fn update(stats: &SessionStats) {
    let dlg = window();
    if dlg.is_null() {
        return;
    }
    let rows = rows(stats);
    unsafe {
        if fill(dlg, &rows) {
            layout(dlg, &rows);
        }
    }
}

/// Figures about as long as they get, so the value column is wide enough from the start and
/// the window does not grow while it is watched.
fn roomy() -> SessionStats {
    let big = (1u64 << 40) - (1 << 27);
    SessionStats {
        alltime_upload: big,
        alltime_download: big + 1,
        alltime_since: 1_794_300_000,
        sent_payload_bytes: big,
        recv_payload_bytes: big,
        wasted_bytes: big,
        num_peers_connected: 88_888,
        disk_write_queue: 65_535,
        read_cache_bytes: big,
        disk_blocks_read: 1,
        disk_cache_hits: 1,
        ..SessionStats::default()
    }
}

fn template(rows: &[Row]) -> Vec<u16> {
    const C: u32 = WS_CHILD | WS_VISIBLE;
    let mut t = Template::new(WIDTH, HEIGHT);
    // positions are set by `layout`; the only button has the focus when the window opens
    t.item(BUTTON, ID_CLOSE, C | WS_TABSTOP | BS_DEFPUSHBUTTON as u32, [0, 0, 50, 14], tr!(BtnClose));
    for (i, row) in rows.iter().enumerate() {
        let i = i as u16;
        match row {
            // a box comes before the rows inside it
            Row::Heading(text) => t.item(BUTTON, ID_GROUP + i, C | BS_GROUPBOX as u32, [0, 0, 10, 10], text),
            Row::Value { label, .. } => {
                t.item(STATIC, ID_LABEL + i, C | SS_LEFTNOWORDWRAP | SS_NOPREFIX, [0, 0, 10, 8], label);
                t.item(STATIC, ID_VALUE + i, C | SS_RIGHT | SS_NOPREFIX, [0, 0, 10, 8], "");
            }
        }
    }
    t.finish()
}

/// Dialog units to pixels for this dialog's font.
unsafe fn pixels(dlg: HWND, x: i32, y: i32) -> (i32, i32) {
    let mut r = RECT { left: 0, top: 0, right: x, bottom: y };
    unsafe { MapDialogRect(dlg, &mut r) };
    (r.right, r.bottom)
}

/// The size `text` takes on one line in the dialog's font: as it is (a label, a value), or
/// with its & not counted (`prefix`: a button's label, a group box's title).
unsafe fn measure(dlg: HWND, text: &str, prefix: bool) -> (i32, i32) {
    unsafe {
        let dc = GetDC(dlg);
        let old = SelectObject(dc, send(dlg, WM_GETFONT, 0, 0) as HFONT);
        let mut r: RECT = zeroed();
        let flags = DT_CALCRECT | DT_SINGLELINE | if prefix { 0 } else { DT_NOPREFIX };
        let w = wide(text);
        DrawTextW(dc, w.as_ptr(), -1, &mut r, flags);
        SelectObject(dc, old);
        ReleaseDC(dlg, dc);
        (r.right, r.bottom)
    }
}

/// Puts the rows' values into their controls (only the changed ones, so nothing flickers);
/// true when one no longer fits its control.
unsafe fn fill(dlg: HWND, rows: &[Row]) -> bool {
    let mut outgrown = false;
    for (i, row) in rows.iter().enumerate() {
        let Row::Value { value, .. } = row else { continue };
        unsafe {
            let ctl = GetDlgItem(dlg, (ID_VALUE + i as u16) as i32);
            if ctl.is_null() || window_text(ctl) == *value {
                continue;
            }
            SetWindowTextW(ctl, wide(value).as_ptr());
            let mut rc: RECT = zeroed();
            GetClientRect(ctl, &mut rc);
            outgrown |= measure(dlg, value, false).0 > rc.right;
        }
    }
    outgrown
}

/// Sizes the window to its texts: the label column as wide as the longest label, the value
/// column as wide as the longest figure shown or to be expected, every group box around its
/// rows at the same width, Close in the bottom right corner, and the window around them, kept
/// centred where it is.
unsafe fn layout(dlg: HWND, shown: &[Row]) {
    unsafe {
        let (m, my) = pixels(dlg, MARGIN, MARGIN);
        let (pad, top) = pixels(dlg, GROUP_PAD, GROUP_TOP);
        let (column_gap, row_gap) = pixels(dlg, COLUMN_GAP, ROW_GAP);
        let (_, bottom) = pixels(dlg, 0, GROUP_BOTTOM);
        let (_, group_gap) = pixels(dlg, 0, GROUP_GAP);
        let (_, button_gap) = pixels(dlg, 0, BUTTON_GAP);
        let (min_w, bh) = pixels(dlg, BUTTON_MIN_W, BUTTON_H);
        let (button_pad, _) = pixels(dlg, BUTTON_PAD, 0);
        let line = measure(dlg, "Xg", false).1;

        let (mut label_w, mut value_w, mut heading_w) = (0, pixels(dlg, VALUE_MIN_W, 0).0, 0);
        let roomy = rows(&roomy());
        for row in shown.iter().chain(&roomy) {
            match row {
                Row::Heading(text) => heading_w = heading_w.max(measure(dlg, text, true).0),
                Row::Value { label, value } => {
                    label_w = label_w.max(measure(dlg, label, false).0);
                    value_w = value_w.max(measure(dlg, value, false).0);
                }
            }
        }
        let close_w = (measure(dlg, tr!(BtnClose), true).0 + button_pad).max(min_w);
        // a box's title starts and ends a little inside its frame
        let inner = (label_w + column_gap + value_w).max(heading_w + pad).max(close_w - 2 * pad);
        let group_w = inner + 2 * pad;
        let cw = group_w + 2 * m;

        // (control, x, y, width, height), top to bottom
        let mut places: Vec<(u16, i32, i32, i32, i32)> = Vec::new();
        let mut y = my;
        // the open box: its place in `places` and its top
        let mut open: Option<(usize, i32)> = None;
        let mut rows_in_box = 0;
        // ends the open box under its last row; the next thing comes `group_gap` below it
        let close_box = |open: &mut Option<(usize, i32)>,
                         places: &mut Vec<(u16, i32, i32, i32, i32)>,
                         y: &mut i32,
                         rows_in_box: i32| {
            if let Some((at, box_top)) = open.take() {
                if rows_in_box > 0 {
                    *y -= row_gap;
                }
                *y += bottom;
                places[at].4 = *y - box_top;
            }
            *y += group_gap;
        };
        for (i, row) in shown.iter().enumerate() {
            let i = i as u16;
            match row {
                Row::Heading(_) => {
                    if open.is_some() {
                        close_box(&mut open, &mut places, &mut y, rows_in_box);
                    }
                    open = Some((places.len(), y));
                    rows_in_box = 0;
                    places.push((ID_GROUP + i, m, y, group_w, 0));
                    y += top;
                }
                Row::Value { .. } => {
                    places.push((ID_LABEL + i, m + pad, y, label_w, line));
                    places.push((ID_VALUE + i, m + pad + inner - value_w, y, value_w, line));
                    y += line + row_gap;
                    rows_in_box += 1;
                }
            }
        }
        close_box(&mut open, &mut places, &mut y, rows_in_box);
        let by = y - group_gap + button_gap;
        places.push((ID_CLOSE, cw - m - close_w, by, close_w, bh));
        let ch = by + bh + my;

        let mut want = RECT { left: 0, top: 0, right: cw, bottom: ch };
        let style = GetWindowLongW(dlg, GWL_STYLE) as u32;
        let ex = GetWindowLongW(dlg, GWL_EXSTYLE) as u32;
        AdjustWindowRectEx(&mut want, style, 0, ex);
        let (w, h) = (want.right - want.left, want.bottom - want.top);
        let mut now: RECT = zeroed();
        GetWindowRect(dlg, &mut now);
        let x = now.left + (now.right - now.left - w) / 2;
        let top_y = now.top + (now.bottom - now.top - h) / 2;
        SetWindowPos(dlg, null_mut(), x, top_y, w, h, SWP_NOZORDER | SWP_NOACTIVATE);
        for (id, x, y, w, h) in places {
            MoveWindow(GetDlgItem(dlg, id as i32), x, y, w, h, 1);
        }
    }
}

unsafe extern "system" fn proc_(dlg: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> isize {
    unsafe {
        if let Some(r) = crate::theme::dialog_message(dlg, msg, wp, lp) {
            return r;
        }
        match msg {
            WM_INITDIALOG => {
                OPEN.with(|o| o.set(dlg));
                SetWindowTextW(dlg, wide(&title()).as_ptr());
                // `show`'s figures; they live for the length of its call
                let stats = (lp as *const SessionStats).as_ref().copied().unwrap_or_default();
                let rows = rows(&stats);
                fill(dlg, &rows);
                layout(dlg, &rows);
                1
            }
            // Close, Escape and the caption's X (IDCANCEL), Enter (the default button, or IDOK)
            WM_COMMAND if matches!(loword(wp) as i32, IDOK | IDCANCEL) => {
                DestroyWindow(dlg);
                1
            }
            WM_DESTROY => {
                OPEN.with(|o| {
                    if o.get() == dlg {
                        o.set(null_mut());
                    }
                });
                0
            }
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    //! The window built from its template, hidden.

    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::Graphics::Gdi::MapWindowPoints;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetFocus;
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetDlgCtrlID, IsWindow, SendMessageW};

    use super::*;

    struct Dialog(HWND);

    impl Drop for Dialog {
        fn drop(&mut self) {
            unsafe { DestroyWindow(self.0) };
        }
    }

    fn open(stats: &SessionStats) -> Dialog {
        assert!(window().is_null(), "a window left open by another test");
        let dlg = show(null_mut(), stats);
        assert!(!dlg.is_null(), "the template does not load");
        Dialog(dlg)
    }

    fn busy() -> SessionStats {
        SessionStats {
            alltime_upload: 3 << 30,
            alltime_download: 1 << 30,
            alltime_since: 1_794_300_000,
            sent_payload_bytes: 5 << 20,
            recv_payload_bytes: 7 << 20,
            wasted_bytes: 16_384,
            num_peers_connected: 1234,
            disk_write_queue: 2,
            read_cache_bytes: 8 << 20,
            disk_blocks_read: 10,
            disk_cache_hits: 5,
            ..SessionStats::default()
        }
    }

    unsafe fn text(dlg: HWND, id: u16) -> String {
        unsafe { window_text(GetDlgItem(dlg, id as i32)) }
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

    /// What the window shows for `expected`, row by row.
    unsafe fn assert_shows(dlg: HWND, expected: &[Row]) {
        for (i, row) in expected.iter().enumerate() {
            let i = i as u16;
            unsafe {
                match row {
                    Row::Heading(h) => assert_eq!(text(dlg, ID_GROUP + i), *h),
                    Row::Value { label, value } => {
                        assert_eq!(text(dlg, ID_LABEL + i), *label);
                        assert_eq!(text(dlg, ID_VALUE + i), *value, "{label}");
                    }
                }
            }
        }
    }

    #[test]
    fn shows_every_row_and_close() {
        let stats = busy();
        let d = open(&stats);
        unsafe {
            assert_eq!(window_text(d.0), title());
            assert_eq!(title(), "Statistics");
            let expected = rows(&stats);
            assert!(expected.iter().any(|r| matches!(r, Row::Heading(_))));
            assert_shows(d.0, &expected);
            assert_eq!(text(d.0, ID_CLOSE), tr!(BtnClose));
            // the only button has the focus, so Enter and Space close
            assert_eq!(GetDlgCtrlID(GetFocus()), ID_CLOSE as i32);
        }
    }

    #[test]
    fn the_values_follow_the_session() {
        // no window: nothing to do
        update(&busy());
        assert!(window().is_null());

        let d = open(&SessionStats::default());
        unsafe {
            assert_shows(d.0, &rows(&SessionStats::default()));
            let before = rect(d.0, ID_CLOSE);
            let stats = busy();
            update(&stats);
            assert_shows(d.0, &rows(&stats));
            let later = SessionStats { sent_payload_bytes: 6 << 20, num_peers_connected: 1235, ..stats };
            update(&later);
            assert_shows(d.0, &rows(&later));
            assert_ne!(rows(&later), rows(&stats));
            // ordinary figures leave the layout alone
            let after = rect(d.0, ID_CLOSE);
            assert_eq!((before.left, before.top), (after.left, after.top));
        }
    }

    /// Every text has the room it needs, the two columns line up and each box holds its rows.
    unsafe fn assert_fits(dlg: HWND, shown: &[Row]) {
        unsafe {
            let mut c: RECT = zeroed();
            GetClientRect(dlg, &mut c);
            let (m, my) = pixels(dlg, MARGIN, MARGIN);
            let mut group: Option<RECT> = None;
            let mut columns: Option<(i32, i32)> = None;
            let mut last_bottom = 0;
            let mut values = 0;
            for (i, row) in shown.iter().enumerate() {
                let i = i as u16;
                match row {
                    Row::Heading(h) => {
                        let g = rect(dlg, ID_GROUP + i);
                        assert_eq!((g.left, g.right), (m, c.right - m), "{h}");
                        assert!(g.top >= last_bottom, "{h} under the box before it");
                        assert!(g.right - g.left >= measure(dlg, h, true).0 + 16, "{h}");
                        last_bottom = g.bottom;
                        group = Some(g);
                    }
                    Row::Value { label, value } => {
                        let (l, v) = (rect(dlg, ID_LABEL + i), rect(dlg, ID_VALUE + i));
                        assert!(l.right - l.left >= measure(dlg, label, false).0, "{label}");
                        assert!(v.right - v.left >= measure(dlg, value, false).0, "{label}: {value}");
                        assert!(l.bottom - l.top >= measure(dlg, label, false).1, "{label}");
                        assert_eq!((l.top, l.bottom), (v.top, v.bottom), "{label}");
                        assert!(l.right < v.left, "{label} before its value");
                        // the columns: every label starts, every value ends at the same place
                        let (left, right) = *columns.get_or_insert((l.left, v.right));
                        assert_eq!((l.left, v.right), (left, right), "{label}");
                        let g = group.expect("a heading comes first");
                        assert!(l.left > g.left && v.right < g.right, "{label} inside its box");
                        assert!(l.top > g.top && l.bottom < g.bottom, "{label} inside its box");
                        values += 1;
                    }
                }
            }
            assert!(values > 0);
            let close = rect(dlg, ID_CLOSE);
            assert!(close.right - close.left >= measure(dlg, tr!(BtnClose), true).0 + 8);
            assert!(close.top > last_bottom, "Close under the boxes");
            assert_eq!((close.right, close.bottom), (c.right - m, c.bottom - my));
        }
    }

    #[test]
    fn the_window_fits_its_texts() {
        for stats in [SessionStats::default(), busy(), roomy()] {
            let d = open(&stats);
            unsafe { assert_fits(d.0, &rows(&stats)) };
        }
    }

    #[test]
    fn a_figure_that_outgrows_its_column_widens_the_window() {
        let stats = busy();
        let d = open(&stats);
        unsafe {
            let mut long = rows(&stats);
            let at = long.iter().position(|r| matches!(r, Row::Value { .. })).unwrap();
            if let Row::Value { value, .. } = &mut long[at] {
                *value = "1,234,567,890,123,456,789,012,345,678,901,234.5 GiB".to_owned();
            }
            let mut before: RECT = zeroed();
            GetClientRect(d.0, &mut before);
            assert!(fill(d.0, &long), "the long figure does not fit");
            layout(d.0, &long);
            let mut after: RECT = zeroed();
            GetClientRect(d.0, &mut after);
            assert!(after.right > before.right);
            assert_eq!(after.bottom, before.bottom);
            assert_fits(d.0, &long);
            // the same texts again: nothing changed, nothing to lay out
            assert!(!fill(d.0, &long));
        }
    }

    #[test]
    fn there_is_one_window_and_close_enter_and_escape_end_it() {
        // Close and Escape send IDCANCEL, Enter the default button's id or IDOK
        for id in [IDCANCEL, IDOK] {
            let dlg = show(null_mut(), &busy());
            assert!(!dlg.is_null());
            assert_eq!(window(), dlg);
            // asked for again: the same window, with the newer figures
            let newer = SessionStats { num_peers_connected: 7, ..busy() };
            assert_eq!(show(null_mut(), &newer), dlg);
            unsafe {
                assert_shows(dlg, &rows(&newer));
                SendMessageW(dlg, WM_COMMAND, id as usize, 0);
                assert_eq!(IsWindow(dlg), 0);
            }
            assert!(window().is_null());
        }
    }
}
