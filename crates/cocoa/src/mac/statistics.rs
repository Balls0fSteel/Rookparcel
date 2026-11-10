//! Help > Statistics: the all-time and session totals in a small window that stays open beside
//! the main one and follows the figures once a second (`app::tick`), as in the other clients.
//! The rows come from `rookparcel_frontend::statistics::rows`; their number and order never
//! change, so the labels are built once and only the value texts are replaced after that.
//! The title bar's button and Close Window (Command-W) close it, as other Mac utility windows.

use std::cell::RefCell;

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSFont, NSGridCellPlacement, NSTextField, NSView, NSWindow, NSWindowStyleMask};
use objc2_foundation::NSRange;
use rookparcel_core::SessionStats;
use rookparcel_frontend::statistics::{self, Row};
use rookparcel_frontend::{lang, tr};

use crate::ui::{self, layout};

/// The window (kept once built: closing only hides it) and its value labels, in the order of
/// the value rows.
struct Open {
    window: Retained<NSWindow>,
    values: Vec<Retained<NSTextField>>,
}

thread_local! {
    static OPEN: RefCell<Option<Open>> = const { RefCell::new(None) };
}

/// Help > Statistics: shows the window, or brings the open one to the front.
pub fn open() {
    let Some(mtm) = MainThreadMarker::new() else { return };
    let stats = crate::app::peek(|c| *c.stats()).unwrap_or_default();
    if OPEN.with(|o| o.borrow().is_none()) {
        let built = build(&stats, mtm);
        built.window.center();
        OPEN.with(|o| *o.borrow_mut() = Some(built));
    }
    fill(&stats);
    if let Some(w) = OPEN.with(|o| o.borrow().as_ref().map(|o| o.window.clone())) {
        w.makeKeyAndOrderFront(None);
    }
    super::lifecycle::activate(mtm);
}

/// The 1 s timer: a window that shows follows `Core`'s last statistics.
pub fn refresh() {
    let shows = OPEN.with(|o| o.try_borrow().ok().and_then(|o| o.as_ref().map(|o| o.window.isVisible())));
    if shows != Some(true) {
        return;
    }
    if let Some(stats) = crate::app::peek(|c| *c.stats()) {
        fill(&stats);
    }
}

/// Writes the values. A text that did not change is left alone, so a selection in it stays.
fn fill(stats: &SessionStats) {
    let Some(labels) = OPEN.with(|o| o.try_borrow().ok()?.as_ref().map(|o| o.values.clone())) else {
        return;
    };
    let texts = statistics::rows(stats).into_iter().filter_map(|r| match r {
        Row::Value { value, .. } => Some(value),
        Row::Heading(_) => None,
    });
    for (label, text) in labels.iter().zip(texts) {
        if label.stringValue().to_string() != text {
            label.setStringValue(&ui::ns(&text));
        }
    }
}

/// The window: a grid with the group titles in bold over both columns, the labels at the
/// leading edge and the values at the trailing one, selectable.
fn build(stats: &SessionStats, mtm: MainThreadMarker) -> Open {
    let mut cells: Vec<(Retained<NSTextField>, Option<Retained<NSTextField>>)> = Vec::new();
    let mut values = Vec::new();
    for row in statistics::rows(stats) {
        match row {
            Row::Heading(title) => {
                let l = ui::label(&title, mtm);
                l.setFont(Some(&NSFont::boldSystemFontOfSize(NSFont::systemFontSize())));
                cells.push((l, None));
            }
            Row::Value { label, value } => {
                let v = ui::label(&value, mtm);
                ui::align_right(&v);
                v.setSelectable(true);
                values.push(v.clone());
                cells.push((ui::label(&label, mtm), Some(v)));
            }
        }
    }
    let rows: Vec<Vec<Option<&NSView>>> = cells
        .iter()
        .map(|(l, v)| {
            let label: &NSView = l;
            vec![Some(label), v.as_ref().map(|v| -> &NSView { v })]
        })
        .collect();
    let grid = layout::grid(&rows, mtm);
    grid.setColumnSpacing(24.0);
    if grid.numberOfColumns() > 1 {
        grid.columnAtIndex(1).setXPlacement(NSGridCellPlacement::Trailing);
    }
    for (i, (_, value)) in cells.iter().enumerate() {
        if value.is_some() {
            continue;
        }
        grid.mergeCellsInHorizontalRange_verticalRange(NSRange::new(0, 2), NSRange::new(i, 1));
        if i > 0 {
            grid.rowAtIndex(i as isize).setTopPadding(12.0);
        }
    }
    // wide enough for a size or a date, so the window keeps its width as the figures grow
    if let Some(first) = values.first() {
        layout::min_width(first, 150.0);
    }
    let grid_view: &NSView = &grid;
    let body = layout::vstack(&[grid_view], mtm);
    layout::insets(&body, layout::MARGIN, layout::MARGIN, layout::MARGIN, layout::MARGIN);
    let style = NSWindowStyleMask::Titled | NSWindowStyleMask::Closable | NSWindowStyleMask::Miniaturizable;
    let window = super::window(&lang::plain(tr!(MenuStatistics)), &body, style, mtm);
    Open { window, values }
}
