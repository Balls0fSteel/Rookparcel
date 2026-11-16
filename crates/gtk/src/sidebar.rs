//! The category list on the left, µTorrent 2.2.1's tree as a flat list: Torrents, Downloading,
//! Completed, Active and Inactive with their counts, then the Labels: "No Label" and every
//! label in use, then the Tags (D76): "No Tag" and every known tag, then the Feeds (all feeds'
//! items, then each feed's), as the Win32 tree shows them.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk::prelude::*;
use rookparcel_frontend::icons::{UiIcon, category_icon};
use rookparcel_frontend::model::{Filter, TorrentList};
use rookparcel_frontend::rss::FeedId;
use rookparcel_frontend::status::Category;
use rookparcel_frontend::tr;

/// What a row shows in the main list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pick {
    Torrents(Filter),
    /// A feed's items (`None`: every feed's), µTorrent's way of listing them.
    Feed(Option<FeedId>),
}

/// Called for a right click on a feed row (after it was picked): the feed (`None`: the Feeds
/// row), the widget and where.
pub type FeedMenu = Rc<dyn Fn(Option<FeedId>, &gtk::Widget, f64, f64)>;

/// Called for a right click on the Tags group (D76): the tag of the row (picked first; `None`
/// for the Tags heading and No Tag), the widget and where.
pub type TagMenu = Rc<dyn Fn(Option<String>, &gtk::Widget, f64, f64)>;

pub struct Sidebar {
    pub widget: gtk::ListBox,
    /// What each row picks, by row index (`None` for the Labels and Tags headings).
    rows: RefCell<Vec<(Option<Pick>, gtk::Label)>>,
    /// The Tags heading's row index.
    tags_heading: Cell<usize>,
    /// The labels, tags and feeds the rows were built for.
    labels: RefCell<Vec<String>>,
    tags: RefCell<Vec<String>>,
    feeds: RefCell<Vec<(FeedId, String)>>,
    /// The row picked last.
    current: RefCell<Pick>,
    /// Rows are being rebuilt: their selection changes are not the user's.
    rebuilding: Cell<bool>,
}

impl Sidebar {
    /// `on_select` gets what each row the user picks shows; `feed_menu` the right clicks on the
    /// Feeds rows, `tag_menu` those on the Tags group.
    pub fn new(on_select: impl Fn(Pick) + 'static, feed_menu: FeedMenu, tag_menu: TagMenu) -> Rc<Sidebar> {
        let widget = gtk::ListBox::new();
        widget.set_selection_mode(gtk::SelectionMode::Single);
        widget.add_css_class("navigation-sidebar");
        let sidebar = Rc::new(Sidebar {
            widget,
            rows: RefCell::default(),
            tags_heading: Cell::new(usize::MAX),
            labels: RefCell::default(),
            tags: RefCell::default(),
            feeds: RefCell::default(),
            current: RefCell::new(Pick::Torrents(Filter::default())),
            rebuilding: Cell::new(false),
        });
        let weak = Rc::downgrade(&sidebar);
        sidebar.widget.connect_row_selected(move |_, row| {
            let (Some(s), Some(row)) = (weak.upgrade(), row) else { return };
            if s.rebuilding.get() {
                return;
            }
            let pick = s.rows.borrow().get(row.index() as usize).and_then(|(p, _)| p.clone());
            if let Some(p) = pick {
                *s.current.borrow_mut() = p.clone();
                on_select(p);
            }
        });
        let click = gtk::GestureClick::new();
        click.set_button(3);
        let weak = Rc::downgrade(&sidebar);
        click.connect_pressed(move |g, _, x, y| {
            let Some(s) = weak.upgrade() else { return };
            let Some(row) = s.widget.row_at_y(y as i32) else { return };
            let index = row.index() as usize;
            let pick = s.rows.borrow().get(index).and_then(|(p, _)| p.clone());
            let Some(w) = g.widget() else { return };
            match pick {
                Some(Pick::Feed(feed)) => {
                    s.widget.select_row(Some(&row));
                    feed_menu(feed, &w, x, y);
                }
                Some(Pick::Torrents(Filter::Tag(tag))) => {
                    s.widget.select_row(Some(&row));
                    tag_menu((!tag.is_empty()).then_some(tag), &w, x, y);
                }
                None if index == s.tags_heading.get() => tag_menu(None, &w, x, y),
                _ => {}
            }
        });
        sidebar.widget.add_controller(click);
        sidebar.rebuild(&[], &[], &[]);
        sidebar
    }

    /// What the list shows now.
    pub fn current(&self) -> Pick {
        self.current.borrow().clone()
    }

    /// Updates the counts; rebuilds the rows when the labels in use, the known tags or the
    /// feeds changed.
    pub fn update(&self, list: &TorrentList, feeds: &[(FeedId, String)]) {
        let labels = list.labels();
        let names: Vec<String> = labels.iter().map(|(l, _)| l.clone()).collect();
        let tags = list.tag_counts();
        let changed = *self.labels.borrow() != names
            || *self.feeds.borrow() != feeds
            || !self.tags.borrow().iter().eq(tags.iter().map(|(t, _)| t));
        if changed {
            let tag_names: Vec<String> = tags.iter().map(|(t, _)| t.clone()).collect();
            self.rebuild(&names, &tag_names, feeds);
        }
        let labelled: usize = labels.iter().map(|(_, n)| n).sum();
        for (pick, label) in self.rows.borrow().iter() {
            let text = match pick {
                Some(Pick::Torrents(Filter::Category(c))) => format!("{} ({})", c.label(), list.count(*c)),
                Some(Pick::Torrents(Filter::Label(l))) if l.is_empty() => {
                    format!("{} ({})", tr!(CatNoLabel), list.len() - labelled)
                }
                Some(Pick::Torrents(Filter::Label(l))) => {
                    let n = labels.iter().find(|(x, _)| x == l).map_or(0, |(_, n)| *n);
                    format!("{l} ({n})")
                }
                Some(Pick::Torrents(Filter::Tag(t))) if t.is_empty() => {
                    format!("{} ({})", tr!(CatNoTag), list.untagged())
                }
                Some(Pick::Torrents(Filter::Tag(t))) => {
                    let n = tags.iter().find(|(x, _)| x == t).map_or(0, |(_, n)| *n);
                    format!("{t} ({n})")
                }
                _ => continue,
            };
            if label.text() != text {
                label.set_text(&text);
            }
        }
    }

    fn rebuild(&self, labels: &[String], tags: &[String], feeds: &[(FeedId, String)]) {
        self.rebuilding.set(true);
        while let Some(row) = self.widget.row_at_index(0) {
            self.widget.remove(&row);
        }
        let mut rows = Vec::new();
        let mut add = |pick: Option<Pick>, text: &str, icon: UiIcon, indent: bool, heading: bool| {
            let label = gtk::Label::new(Some(text));
            label.set_xalign(0.0);
            label.set_ellipsize(gtk::pango::EllipsizeMode::End);
            let content = crate::icons::with_icon(icon, &label);
            if indent {
                content.set_margin_start(14);
            }
            let row = gtk::ListBoxRow::new();
            row.set_child(Some(&content));
            if heading {
                label.add_css_class("heading");
                content.set_margin_top(8);
            }
            if pick.is_none() {
                row.set_selectable(false);
                row.set_activatable(false);
            }
            self.widget.append(&row);
            rows.push((pick, label));
        };
        for (i, c) in Category::ALL.iter().enumerate() {
            add(Some(Pick::Torrents(Filter::Category(*c))), c.label(), category_icon(*c), i > 0, false);
        }
        add(None, tr!(CatLabels), UiIcon::Labels, false, true);
        let no_label = Some(Pick::Torrents(Filter::Label(String::new())));
        add(no_label, tr!(CatNoLabel), UiIcon::NoLabel, true, false);
        for l in labels {
            add(Some(Pick::Torrents(Filter::Label(l.clone()))), l, UiIcon::Labels, true, false);
        }
        // the Tags group (D76): no symbol of its own yet, the Labels' stand in
        // after the categories, the Labels heading, No Label and the labels
        let tags_heading = Category::ALL.len() + 2 + labels.len();
        add(None, tr!(CatTags), UiIcon::Labels, false, true);
        let no_tag = Some(Pick::Torrents(Filter::Tag(String::new())));
        add(no_tag, tr!(CatNoTag), UiIcon::NoLabel, true, false);
        for t in tags {
            add(Some(Pick::Torrents(Filter::Tag(t.clone()))), t, UiIcon::Labels, true, false);
        }
        // the Feeds row lists every feed's items, as the Win32 tree's Feeds node
        add(Some(Pick::Feed(None)), tr!(CatFeeds), UiIcon::Feeds, false, true);
        for (id, name) in feeds {
            add(Some(Pick::Feed(Some(*id))), name, UiIcon::Feeds, true, false);
        }
        let current = self.current.borrow().clone();
        let index = rows.iter().position(|(p, _)| p.as_ref() == Some(&current));
        if index.is_none() {
            // what was shown went away
            *self.current.borrow_mut() = Pick::Torrents(Filter::default());
        }
        *self.rows.borrow_mut() = rows;
        *self.labels.borrow_mut() = labels.to_vec();
        *self.tags.borrow_mut() = tags.to_vec();
        self.tags_heading.set(tags_heading);
        *self.feeds.borrow_mut() = feeds.to_vec();
        self.widget.select_row(self.widget.row_at_index(index.unwrap_or(0) as i32).as_ref());
        self.rebuilding.set(false);
    }
}
