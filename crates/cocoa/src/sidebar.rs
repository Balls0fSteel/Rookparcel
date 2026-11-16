//! The sidebar, µTorrent 2.2.1's category tree as a source list (GTK `sidebar.rs`): Torrents
//! with Downloading, Completed, Active and Inactive under it, the Labels heading with No Label
//! and every label in use, the Tags heading (D76) with No Tag and every known tag, then Feeds
//! (every feed's items) with each feed (slice H's `rss::sidebar_rows`). Each row shows its count;
//! a pick shows those torrents (`Core::show_torrents`) or a feed's items (`rss::show_feed`).
//! The context menu ([`menu_items`]) is the feeds' on the Feeds rows, New Tag (and Delete Tag on
//! a tag's row) on the Tags rows ([`run`]).
//!
//! Rows are `RPTableSource` outline items keyed by a `u64`: fixed ids for the fixed rows, a label
//! or tag keeps the id it first got while the process runs (so the selection survives a label
//! coming or going), a feed's id is its `FeedId` above [`FEED_BASE`].

use std::cell::RefCell;

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSOutlineView, NSScrollView};
use objc2_foundation::NSNumber;
use rookparcel_frontend::icons::{UiIcon, category_icon};
use rookparcel_frontend::model::Filter;
use rookparcel_frontend::rss::FeedId;
use rookparcel_frontend::status::Category;
use rookparcel_frontend::tr;

use crate::app::{apply, peek, ui, with_core};
use crate::commands::Command;
use crate::menu::Item;
use crate::table::{self, CellKind, CellValue, ColumnSpec, Hooks, RPTableSource, TableKind};
use crate::{images, mainq};

/// Torrents (every torrent); the other categories follow it: `TORRENTS + i` is
/// `Category::ALL[i]`.
const TORRENTS: u64 = 1;
/// The Labels heading.
const LABELS: u64 = 10;
const NO_LABEL: u64 = 11;
/// The Tags heading (D76).
const TAGS: u64 = 12;
const NO_TAG: u64 = 13;
/// The Feeds row: every feed's items.
const FEEDS: u64 = 20;
/// A label's row: `LABEL_BASE` + its index in `State::known`.
const LABEL_BASE: u64 = 1 << 20;
/// A tag's row: `TAG_BASE` + its index in `State::known_tags`.
const TAG_BASE: u64 = 1 << 30;
/// A feed's row: `FEED_BASE` + its id.
const FEED_BASE: u64 = 1 << 40;

/// What a row shows in the list.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Pick {
    Torrents(Filter),
    /// A feed's items (`None`: every feed's).
    Feed(Option<FeedId>),
}

#[derive(Default)]
struct State {
    /// Every label name given a row id so far (the id is `LABEL_BASE` + the index).
    known: Vec<String>,
    /// The labels in use, by name, with their counts.
    labels: Vec<(String, usize)>,
    /// How many torrents have no label.
    unlabeled: usize,
    /// Every tag name given a row id so far (the id is `TAG_BASE` + the index).
    known_tags: Vec<String>,
    /// The known tags with their counts (D76).
    tags: Vec<(String, usize)>,
    /// How many torrents have no tag.
    untagged: usize,
    feeds: Vec<(FeedId, String)>,
    /// The tag whose row the context menu was opened on (Delete Tag's).
    menu_tag: Option<String>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

impl State {
    fn label_id(&mut self, name: &str) -> u64 {
        LABEL_BASE + id_in(&mut self.known, name)
    }

    fn label_name(&self, id: u64) -> Option<&str> {
        name_in(&self.known, id, LABEL_BASE)
    }

    fn tag_id(&mut self, name: &str) -> u64 {
        TAG_BASE + id_in(&mut self.known_tags, name)
    }

    fn tag_name(&self, id: u64) -> Option<&str> {
        name_in(&self.known_tags, id, TAG_BASE)
    }
}

/// `name`'s index in `known`, added at the end when it is new.
fn id_in(known: &mut Vec<String>, name: &str) -> u64 {
    let i = match known.iter().position(|k| k == name) {
        Some(i) => i,
        None => {
            known.push(name.to_owned());
            known.len() - 1
        }
    };
    i as u64
}

/// The name of row `id` in `known`, whose ids start at `base`.
fn name_in(known: &[String], id: u64, base: u64) -> Option<&str> {
    let i = usize::try_from(id.checked_sub(base)?).ok()?;
    known.get(i).map(String::as_str)
}

/// What row `id` picks; `None` for the heading.
fn pick_of(id: u64) -> Option<Pick> {
    match id {
        TORRENTS..=5 => Some(Pick::Torrents(Filter::Category(Category::ALL[(id - TORRENTS) as usize]))),
        NO_LABEL => Some(Pick::Torrents(Filter::Label(String::new()))),
        NO_TAG => Some(Pick::Torrents(Filter::Tag(String::new()))),
        FEEDS => Some(Pick::Feed(None)),
        _ if id >= FEED_BASE => FeedId::try_from(id - FEED_BASE).ok().map(|f| Pick::Feed(Some(f))),
        _ if id >= TAG_BASE => {
            STATE.with(|s| s.borrow().tag_name(id).map(str::to_owned)).map(|t| Pick::Torrents(Filter::Tag(t)))
        }
        _ if id >= LABEL_BASE => STATE
            .with(|s| s.borrow().label_name(id).map(str::to_owned))
            .map(|l| Pick::Torrents(Filter::Label(l))),
        _ => None,
    }
}

/// A row's children (`None`: the top level).
fn children(parent: Option<u64>) -> Vec<u64> {
    match parent {
        None => vec![TORRENTS, LABELS, TAGS, FEEDS],
        Some(TORRENTS) => (TORRENTS + 1..TORRENTS + Category::ALL.len() as u64).collect(),
        Some(LABELS) => STATE.with(|s| {
            let mut s = s.borrow_mut();
            let names: Vec<String> = s.labels.iter().map(|(l, _)| l.clone()).collect();
            let mut ids = vec![NO_LABEL];
            ids.extend(names.iter().map(|l| s.label_id(l)));
            ids
        }),
        Some(TAGS) => STATE.with(|s| {
            let mut s = s.borrow_mut();
            let names: Vec<String> = s.tags.iter().map(|(t, _)| t.clone()).collect();
            let mut ids = vec![NO_TAG];
            ids.extend(names.iter().map(|t| s.tag_id(t)));
            ids
        }),
        Some(FEEDS) => {
            STATE.with(|s| s.borrow().feeds.iter().map(|(f, _)| FEED_BASE + u64::from(*f)).collect())
        }
        Some(_) => Vec::new(),
    }
}

/// "Name (count)".
fn counted(name: &str, n: usize) -> String {
    format!("{name} ({n})")
}

fn cell(id: u64, dark: bool) -> CellValue {
    let icon = |i: UiIcon| Some(images::ui(i, dark));
    match id {
        TORRENTS..=5 => {
            let c = Category::ALL[(id - TORRENTS) as usize];
            let n = peek(|core| core.list().count(c)).unwrap_or(0);
            CellValue::IconText(icon(category_icon(c)), counted(&crate::ui::text(c.label()), n))
        }
        LABELS => CellValue::Text(crate::ui::text(tr!(CatLabels))),
        NO_LABEL => {
            let n = STATE.with(|s| s.borrow().unlabeled);
            CellValue::IconText(icon(UiIcon::NoLabel), counted(&crate::ui::text(tr!(CatNoLabel)), n))
        }
        TAGS => CellValue::Text(crate::ui::text(tr!(CatTags))),
        NO_TAG => {
            let n = STATE.with(|s| s.borrow().untagged);
            CellValue::IconText(icon(UiIcon::NoLabel), counted(&crate::ui::text(tr!(CatNoTag)), n))
        }
        FEEDS => CellValue::IconText(icon(UiIcon::Feeds), crate::ui::text(tr!(CatFeeds))),
        _ if id >= FEED_BASE => {
            let name = STATE.with(|s| {
                s.borrow().feeds.iter().find(|(f, _)| FEED_BASE + u64::from(*f) == id).map(|(_, n)| n.clone())
            });
            name.map_or(CellValue::Empty, |n| CellValue::IconText(icon(UiIcon::Feeds), n))
        }
        _ if id >= TAG_BASE => {
            // a tag is the user's text, shown as it is (the label icon: there is no tag icon)
            let text = STATE.with(|s| {
                let s = s.borrow();
                let name = s.tag_name(id)?;
                let n = s.tags.iter().find(|(t, _)| t == name).map_or(0, |(_, n)| *n);
                Some(counted(name, n))
            });
            text.map_or(CellValue::Empty, |t| CellValue::IconText(icon(UiIcon::Labels), t))
        }
        _ if id >= LABEL_BASE => {
            // a label is the user's text, shown as it is
            let text = STATE.with(|s| {
                let s = s.borrow();
                let name = s.label_name(id)?;
                let n = s.labels.iter().find(|(l, _)| l == name).map_or(0, |(_, n)| *n);
                Some(counted(name, n))
            });
            text.map_or(CellValue::Empty, |t| CellValue::IconText(icon(UiIcon::Labels), t))
        }
        _ => CellValue::Empty,
    }
}

/// The rows showing these ids (the outline's rows as they are now, expanded or not).
fn rows_of(ids: &[u64]) -> Vec<usize> {
    let Some(u) = ui() else { return Vec::new() };
    let ov = &u.sidebar_view;
    (0..ov.numberOfRows())
        .filter(|&r| {
            ov.itemAtRow(r)
                .and_then(|i| i.downcast_ref::<NSNumber>().map(|n| n.as_u64()))
                .is_some_and(|id| ids.contains(&id))
        })
        .map(|r| r as usize)
        .collect()
}

/// The sidebar outline (a source list) with its rows from `Core`.
pub fn build(
    mtm: MainThreadMarker,
) -> (Retained<NSScrollView>, Retained<NSOutlineView>, Retained<RPTableSource>) {
    refresh_state();
    let mut spec = ColumnSpec::text("name", "", 160.0);
    spec.kind = CellKind::IconText;
    spec.hideable = false;
    let mut hooks = Hooks::flat(|| 0, |id, _, dark| cell(id, dark));
    hooks.children = Some(Box::new(children));
    hooks.is_group = Some(Box::new(|id| id == LABELS || id == TAGS));
    hooks.rows_of = Box::new(rows_of);
    hooks.selection = Some(Box::new(select));
    let (scroll, outline, source) = table::outline(TableKind("sidebar"), vec![spec], hooks, true, mtm);
    outline.setAllowsEmptySelection(false);
    // the Feeds rows' (slice H) and the Tags rows' (D76) context menu (empty, so not shown, on
    // the other rows)
    // SAFETY: the outline keeps its menu.
    unsafe { outline.setMenu(Some(&crate::menu::live(crate::menu::Role::Sidebar, mtm))) };
    (scroll, outline, source)
}

/// Selects Torrents (at the start).
pub fn select_all_torrents() {
    if let Some(u) = ui() {
        u.sidebar.select(&[TORRENTS]);
    }
}

/// The id of the row a context click is on.
fn clicked_id() -> Option<u64> {
    let u = ui()?;
    let row = u.sidebar_view.clickedRow();
    u.sidebar_view.itemAtRow(row)?.downcast_ref::<NSNumber>().map(|n| n.as_u64())
}

/// Slice H: the feed of the row a context click is on (`Some(None)`: the Feeds row); `None`
/// for the other rows.
pub fn clicked_feed() -> Option<Option<FeedId>> {
    match pick_of(clicked_id()?)? {
        Pick::Feed(f) => Some(f),
        Pick::Torrents(_) => None,
    }
}

/// The context menu as it opens (`menuNeedsUpdate:`): the feeds' on the Feeds rows, New Tag and
/// Delete Tag on the Tags rows (Delete Tag greyed but on a tag's, [`menu_on_tag`]), nothing (no
/// menu) on the others.
pub fn menu_items() -> Vec<Item> {
    let Some(id) = clicked_id() else { return Vec::new() };
    let tag = match id {
        TAGS | NO_TAG => None,
        _ if (TAG_BASE..FEED_BASE).contains(&id) => {
            STATE.with(|s| s.borrow().tag_name(id).map(str::to_owned))
        }
        _ => return crate::rss::sidebar_menu_items(),
    };
    STATE.with(|s| s.borrow_mut().menu_tag = tag);
    crate::menu::sidebar_tag_items()
}

/// Whether the sidebar's open menu is on a tag's own row: Delete Tag is enabled (D76).
pub fn menu_on_tag() -> bool {
    STATE.with(|s| s.borrow().menu_tag.is_some())
}

/// The Tags rows' context menu (D76): New Tag makes a tag known with no torrent; Delete Tag
/// asks first, then takes the tag off every torrent (a list showing it shows all torrents).
pub fn run(cmd: Command) {
    match cmd {
        Command::SidebarNewTag => crate::torrent_menu::ask_tag(|name| {
            let made = with_core(|c| c.create_tag(&[], name).map(|_| c.events()))?;
            Some(made.map(|ev| apply(Some(ev))))
        }),
        Command::DeleteTag => {
            let Some(name) = STATE.with(|s| s.borrow().menu_tag.clone()) else { return };
            let Some(mtm) = MainThreadMarker::new() else { return };
            let window = ui().map(|u| u.window.clone());
            crate::ui::sheet(
                window.as_deref(),
                crate::ui::Style::Warning,
                &tr!(AskDeleteTag, name = name),
                "",
                &[tr!(BtnOk), tr!(BtnCancel)],
                move |button| {
                    if button != 0 {
                        return;
                    }
                    apply(with_core(|c| {
                        c.delete_tag(&name);
                        c.events()
                    }));
                    select_filter();
                },
                mtm,
            );
        }
        _ => {}
    }
}

/// Selects the row of the list's filter (after Delete Tag reset it), unless a feed's items are
/// shown.
fn select_filter() {
    let Some((filter, feeds)) = peek(|c| (c.list().filter().clone(), c.feed_view().is_some())) else {
        return;
    };
    if feeds {
        return;
    }
    let id = match filter {
        Filter::Category(c) => Category::ALL.iter().position(|&x| x == c).map(|i| TORRENTS + i as u64),
        Filter::Label(l) if l.is_empty() => Some(NO_LABEL),
        Filter::Tag(t) if t.is_empty() => Some(NO_TAG),
        Filter::Label(l) => Some(STATE.with(|s| s.borrow_mut().label_id(&l))),
        Filter::Tag(t) => Some(STATE.with(|s| s.borrow_mut().tag_id(&t))),
    };
    let Some(u) = ui() else { return };
    if let Some(id) = id.filter(|id| !u.sidebar.selected().contains(id)) {
        u.sidebar.select(&[id]);
    }
}

/// Slice H: picks the Feeds row (`None`) or a feed's row, as a click would.
pub fn select_feed(feed: Option<FeedId>) {
    if let Some(u) = ui() {
        u.sidebar.select(&[feed.map_or(FEEDS, |f| FEED_BASE + u64::from(f))]);
    }
}

/// Reads the labels, the tags, their counts and the feeds; true when the rows change.
fn refresh_state() -> bool {
    let (labels, unlabeled) = peek(|c| (c.list().labels(), c.list().unlabeled())).unwrap_or_default();
    let (tags, untagged) = peek(|c| (c.list().tag_counts(), c.list().untagged())).unwrap_or_default();
    let feeds = crate::rss::sidebar_rows();
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        let rows_changed = s.feeds != feeds
            || s.labels.len() != labels.len()
            || s.labels.iter().zip(&labels).any(|((a, _), (b, _))| a != b)
            || s.tags.len() != tags.len()
            || s.tags.iter().zip(&tags).any(|((a, _), (b, _))| a != b);
        s.labels = labels;
        s.unlabeled = unlabeled;
        s.tags = tags;
        s.untagged = untagged;
        s.feeds = feeds;
        rows_changed
    })
}

/// `Events::sidebar`: new counts; the rows again when the labels, tags or feeds changed.
pub fn update() {
    let changed = refresh_state();
    let Some(u) = ui() else { return };
    if changed {
        u.sidebar.reload_keep_selection();
        // SAFETY: nil expands every top-level item and its children (new labels and feeds show).
        unsafe { u.sidebar_view.expandItem_expandChildren(None, true) };
    } else {
        u.sidebar.reload_visible();
    }
}

/// The user (or a reload that lost the row) picked a row.
pub fn select(ids: &[u64]) {
    let Some(&id) = ids.first() else {
        // what was shown went away (a label no torrent has now, a deleted tag): back to
        // Torrents, once this selection change is over
        mainq::on_main(select_all_torrents);
        return;
    };
    match pick_of(id) {
        Some(Pick::Torrents(f)) => apply(with_core(|c| {
            c.show_torrents(f);
            c.events()
        })),
        Some(Pick::Feed(f)) => crate::rss::show_feed(f),
        None => {}
    }
}
