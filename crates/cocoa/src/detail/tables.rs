//! The Trackers, Peers and Logger tabs, and the table under the piece map and in the Files
//! tab: virtual tables (`table.rs`) over `Detail::rows` and `Detail::cell` with the tab's
//! columns (`Tab::columns`). Trackers shows each tracker's status icon before its URL; the
//! Logger is one column, scrolled to its newest line while it was at the end.
//!
//! A click on a Trackers or Peers column's header sorts by it (D78, `Core::toggle_sort_of`):
//! the column's natural direction, reversed, then the engine's order (no indicator); the sort
//! is kept (`detail::sort_key`) and the header's indicator is always the model's. The
//! selection stays on the same trackers and peers when their rows move (a header click, an
//! update of a sorted list, a peer coming or going): `Detail::row_keys` taken at each
//! selection change, found again with `Detail::rows_of_keys` at each refresh.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSScrollView, NSTableViewColumnAutoresizingStyle};
use rookparcel_core::TorrentId;
use rookparcel_frontend::detail::{ColumnDef, FileSort, Tab, file_sort_pref, sort_key};
use rookparcel_frontend::icons::tracker_icon;
use rookparcel_frontend::lang;

use crate::app::{apply, peek, with_core};
use crate::images;
use crate::table::{
    self, Align, CellKind, CellValue, ColumnSpec, Hooks, RPTableSource, RPTableView, TableKind,
};
use crate::ui;

/// One detail table and what it showed last.
pub struct Table {
    pub scroll: Retained<NSScrollView>,
    pub view: Retained<RPTableView>,
    pub source: Retained<RPTableSource>,
    tab: Tab,
    /// The rows at the last refresh.
    rows: Cell<usize>,
    /// The torrent at the last refresh.
    shown: Cell<Option<TorrentId>>,
    /// A sortable tab's selection by what its rows show (`Detail::row_keys`), as of its last
    /// change; `None` for a tab that does not sort.
    keys: Option<Rc<RefCell<Vec<String>>>>,
}

/// The column specs of a tab's columns (titles from the string table, widths, alignment).
pub fn specs(defs: &[ColumnDef]) -> Vec<ColumnSpec> {
    defs.iter()
        .enumerate()
        .map(|(i, &(title, width, right))| {
            let mut s = ColumnSpec::text(&format!("c{i}"), &ui::text(lang::text(title)), f64::from(width));
            s.align = if right { Align::Right } else { Align::Left };
            // the first column (name, address, message) stays
            s.hideable = i > 0;
            s
        })
        .collect()
}

/// The hooks of a plain detail table: its rows and text cells from `Detail`.
pub fn hooks(tab: Tab) -> Hooks {
    Hooks::flat(
        move || peek(|c| c.detail().rows(tab)).unwrap_or(0),
        move |row, col, _dark| {
            peek(|c| CellValue::Text(c.detail().cell(tab, row as usize, col))).unwrap_or(CellValue::Empty)
        },
    )
}

impl Table {
    pub fn new(tab: Tab, specs: Vec<ColumnSpec>, hooks: Hooks, mtm: MainThreadMarker) -> Table {
        let (scroll, view, source) = table::table(TableKind(tab.key()), specs, hooks, None, mtm);
        Table { scroll, view, source, tab, rows: Cell::new(0), shown: Cell::new(None), keys: None }
    }

    /// A Trackers or Peers table: its columns sort by a header click ([`Table::sort_clicked`]),
    /// its header shows the model's sort, and its selection is kept by what the rows show.
    fn sortable(tab: Tab, mut specs: Vec<ColumnSpec>, mut hooks: Hooks, mtm: MainThreadMarker) -> Table {
        for s in &mut specs {
            s.sortable = true;
        }
        hooks.sort_click = Some(Box::new(move |col| sort_clicked(tab, col)));
        let keys = Rc::new(RefCell::new(Vec::new()));
        let k = keys.clone();
        hooks.selection = Some(Box::new(move |ids| {
            let rows: Vec<usize> = ids.iter().map(|&r| r as usize).collect();
            *k.borrow_mut() = peek(|c| c.detail().row_keys(tab, &rows)).unwrap_or_default();
        }));
        let mut t = Table::new(tab, specs, hooks, mtm);
        t.keys = Some(keys);
        t.show_sort(peek(|c| c.detail().sort_of(tab)).flatten());
        t
    }

    /// The Trackers tab: the status icon before each URL.
    pub fn trackers(mtm: MainThreadMarker) -> Table {
        let mut specs = specs(Tab::Trackers.columns());
        specs[0].kind = CellKind::IconText;
        let mut hooks = hooks(Tab::Trackers);
        hooks.cell = Box::new(|row, col, dark| {
            peek(|c| {
                let d = c.detail();
                let Some(t) = d.trackers.get(row as usize) else { return CellValue::Empty };
                let text = d.cell(Tab::Trackers, row as usize, col);
                if col == 0 {
                    CellValue::IconText(Some(images::state(tracker_icon(t.status), dark)), text)
                } else {
                    CellValue::Text(text)
                }
            })
            .unwrap_or(CellValue::Empty)
        });
        Table::sortable(Tab::Trackers, specs, hooks, mtm)
    }

    /// The Peers tab.
    pub fn peers(mtm: MainThreadMarker) -> Table {
        Table::sortable(Tab::Peers, specs(Tab::Peers.columns()), hooks(Tab::Peers), mtm)
    }

    /// The Logger: one column across the width, no header.
    pub fn logger(mtm: MainThreadMarker) -> Table {
        let t = Table::new(Tab::Logger, specs(Tab::Logger.columns()), hooks(Tab::Logger), mtm);
        t.view.setHeaderView(None);
        t.view.setUsesAlternatingRowBackgroundColors(false);
        t.view
            .setColumnAutoresizingStyle(NSTableViewColumnAutoresizingStyle::LastColumnOnlyAutoresizingStyle);
        t.scroll.setHasHorizontalScroller(false);
        t.view.sizeLastColumnToFit();
        t
    }

    /// The selected rows.
    pub fn selected(&self) -> Vec<usize> {
        self.source.selected().into_iter().map(|r| r as usize).collect()
    }

    /// Drops the selection (the rows mean other files now).
    pub fn deselect(&self) {
        self.source.select(&[]);
    }

    /// Shows the rows `Detail` has now: another torrent starts at the top with nothing
    /// selected, a changed count reloads every row, else the rows on screen are redrawn. A
    /// sortable tab's selection follows its trackers or peers to the rows they show now (one
    /// gone leaves it), without scrolling.
    pub fn refresh(&self) {
        let Some((n, id)) = peek(|c| (c.detail().rows(self.tab), c.detail().id)) else { return };
        let before = self.rows.replace(n);
        let kept = self.keys.as_ref().map(|k| k.borrow().clone()).filter(|k| !k.is_empty());
        if self.shown.replace(id) != id {
            self.source.select(&[]);
            self.source.reload_keep_selection();
            if n > 0 {
                self.view.scrollRowToVisible(0);
            }
        } else if let Some(keys) = kept {
            let mut rows = peek(|c| c.detail().rows_of_keys(self.tab, &keys)).unwrap_or_default();
            rows.sort_unstable();
            let ids: Vec<u64> = rows.iter().map(|&r| r as u64).collect();
            if n != before {
                self.source.reload_select(&ids);
            } else {
                self.source.reload_visible();
                if table::rows_of_set(&self.view.selectedRowIndexes()) != rows {
                    self.source.select(&ids);
                }
            }
        } else if n != before {
            // the Logger follows its newest line while it was at the end
            let range = self.view.rowsInRect(self.view.visibleRect());
            let at_end = range.location + range.length >= before;
            self.source.reload_keep_selection();
            if self.tab == Tab::Logger && at_end && n > 0 {
                self.view.scrollRowToVisible(n as isize - 1);
            }
        } else {
            self.source.reload_visible();
        }
    }

    /// The header's sort indicator for `sort` (`None`: none, the engine's order).
    fn show_sort(&self, sort: Option<FileSort>) {
        self.source.set_sort(sort.map(|(col, desc)| (col, !desc)));
    }

    /// A click on the header of column `col`: the next sort of the column's cycle, kept in the
    /// preferences and shown by the header's indicator; every row reloads with the selection on
    /// the same trackers or peers, its first row scrolled into view.
    fn sort_clicked(&self, col: usize) {
        let tab = self.tab;
        let Some(key) = sort_key(tab) else { return };
        let sort = with_core(|c| {
            let sort = c.toggle_sort_of(tab, col);
            match file_sort_pref(sort) {
                Some(v) => c.prefs_mut().set(key, v),
                None => c.prefs_mut().remove(key),
            }
            sort
        });
        let Some(sort) = sort else { return };
        // AppKit's own change of the descriptors (the clicked column reversed, or put first)
        // gives way to the model's
        self.show_sort(sort);
        let keys = self.keys.as_ref().map(|k| k.borrow().clone()).unwrap_or_default();
        let Some((mut rows, n)) = peek(|c| (c.detail().rows_of_keys(tab, &keys), c.detail().rows(tab)))
        else {
            return;
        };
        rows.sort_unstable();
        let ids: Vec<u64> = rows.iter().map(|&r| r as u64).collect();
        self.rows.set(n);
        self.source.reload_select(&ids);
        if let Some(&first) = rows.first() {
            self.view.scrollRowToVisible(first as isize);
        }
        // the rest of what the change asks for; this tab's refresh finds nothing moved
        apply(with_core(|c| c.events()));
    }
}

/// A click on the header of column `col` of the Trackers or the Peers tab.
fn sort_clicked(tab: Tab, col: usize) {
    let Some(t) = super::tabs() else { return };
    match tab {
        Tab::Trackers => t.trackers.sort_clicked(col),
        Tab::Peers => t.peers.sort_clicked(col),
        _ => {}
    }
}
