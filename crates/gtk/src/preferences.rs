//! Options > Preferences: µTorrent 2.2.1's dialog, the page list on the left and the page on the
//! right, OK / Cancel / Apply, built from the frontend's description of the pages
//! ([`pref_pages`]): every page the Win32 dialog has but the Web UI (the remote mode); General
//! has the desktop integration in place of the Windows one, and UI Settings the tray boxes
//! while a tray is on screen.
//!
//! The pages and the Advanced table edit one working [`Options`]. The controls are written from
//! it ([`Form::write`]) and only the ones the user changed are read back ([`Form::read`]), on
//! Apply, on OK and when the Advanced table opens (it shows what the pages say).
//!
//! Help (D55): every control has its help text as a tooltip; the pane under the pages shows the
//! help of the control under the pointer, else of the focused one (the Advanced table's: the
//! selected row's); F1 pops the focused control's help up beside it.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{gio, glib};
use rookparcel_frontend::help;
use rookparcel_frontend::icons::page_icon;
use rookparcel_frontend::ip_filter_list;
use rookparcel_frontend::lang::{self, H, S, gtk_mnemonic};
use rookparcel_frontend::options::{self, GuiOptions, Options};
use rookparcel_frontend::pref_pages::{
    self, Action, Control, Features, Field, Form, IPFILTER_ENABLE, IPFILTER_FROM_URL, IPFILTER_PATH,
    IPFILTER_URL, Item, Page, SCHED_ENABLE, TRACKERS_APPEND, TRACKERS_FROM_URL, TRACKERS_LIST, TRACKERS_URL,
    TextKind, advanced_name, advanced_rows,
};
use rookparcel_frontend::scheduler::{CELLS, Mode};
use rookparcel_frontend::settings::{Entry, Kind, Value};
use rookparcel_frontend::tr;
use rookparcel_frontend::ui_lock::{self, Form as LockForm};

use crate::sched_grid::SchedGrid;
use crate::table::Table;

/// Loads a block list: the number of rules or an error.
pub type ReloadIpFilter = Box<dyn Fn(&Path) -> Result<usize, String>>;
/// The Trackers page's status line for its "from the web" box and address.
pub type TrackerStatus = Box<dyn Fn(bool, &str) -> String>;

/// What the dialog asks of its owner.
pub struct Hooks {
    /// OK and Apply: store the options and put them into effect.
    pub apply: Box<dyn Fn(&Options)>,
    /// "Reload" on the BitTorrent page: load this block list; the number of rules or an error.
    pub reload_ip_filter: ReloadIpFilter,
    /// "Update Now" on the Trackers page: store these options' tracker settings (the four
    /// `trackers_*` values, nothing else of them), put them into effect and fetch the list.
    pub update_tracker_list: Box<dyn Fn(&GuiOptions)>,
    /// The Trackers page's status line for the list from the web, for the page's "from the
    /// web" box and address as they stand (`TrackerList::status_for`).
    pub tracker_status: TrackerStatus,
    /// "Update Now" in the BitTorrent page's IP Filter group (D75): store these options' four
    /// IP filter values (`ip_filter`, `ip_filter_path`, `ip_filter_from_url`, `ip_filter_url`,
    /// nothing else of them), put them into effect and fetch the list.
    pub update_ip_filter: Box<dyn Fn(&GuiOptions)>,
    /// The IP filter's status line for the list from the web, for the page's box and address
    /// as they stand (`IpFilterList::status_for`); empty in a remote window.
    pub ip_filter_status: TrackerStatus,
    /// Where `ipfilter.dat` is looked for when no path is set.
    pub settings_dir: PathBuf,
    /// A tray is on screen: UI Settings shows the tray and notification boxes.
    pub tray: bool,
    /// Advanced > UI Lock (D71), none of it an [`Options`] value: it acts at once. The lock's
    /// state as stored, which the page's switch, buttons and status line follow.
    pub lock_state: Box<dyn Fn() -> ui_lock::State>,
    /// The switch: turn the lock on, or off without a PIN set; saved at once.
    pub lock_enable: Box<dyn Fn(bool)>,
    /// A PIN dialog over the Preferences window: Set, Change or Remove PIN, or [`LockForm::Off`]
    /// for the switch turned off with a PIN set. Its OK does it and saves; the last closure runs
    /// once the dialog is gone, after an OK or a Cancel.
    pub pin_form: PinForm,
}

/// [`Hooks::pin_form`]: the form, the Preferences window, and what runs once the dialog is gone.
pub type PinForm = Box<dyn Fn(LockForm, &gtk::Window, Box<dyn Fn()>)>;

/// Advanced > UI Lock's controls (D71).
struct LockPage {
    switch: gtk::CheckButton,
    /// Set, Change and Remove PIN, each with the PIN dialog it opens.
    buttons: Vec<(gtk::Button, LockForm)>,
    status: gtk::Label,
    /// The switch is being set to the stored state: its `toggled` is no click.
    quiet: Cell<bool>,
}

/// What a greyed widget follows.
#[derive(Clone, Copy)]
pub(crate) enum Grey {
    /// A page's field: enabled while the boxes it needs are on and the ones that grey it off
    /// ([`Field::enabled`]: `needs` and `unless`).
    Field(Field),
    /// Enabled while these boxes are on (the Scheduler's table).
    Needs(&'static [&'static str]),
}

impl Grey {
    /// For a field: `None` when nothing greys it.
    fn of(f: &Field) -> Option<Grey> {
        (!f.needs.is_empty() || !f.unless.is_empty()).then_some(Grey::Field(*f))
    }

    /// Whether the widget is enabled, with `on` telling whether a field is on now.
    pub(crate) fn enabled(&self, on: impl Fn(&str) -> bool) -> bool {
        match self {
            Grey::Field(f) => f.enabled(on),
            Grey::Needs(keys) => keys.iter().all(|k| on(k)),
        }
    }
}

/// A field's control.
pub(crate) enum Ctl {
    Check(gtk::CheckButton),
    Entry(gtk::Entry),
    /// A multi-line box (`TextKind::Lines`) and the scrolled window around it.
    Lines(gtk::TextView, gtk::ScrolledWindow),
    /// The drop-down and its choices' values.
    Choice(gtk::DropDown, Vec<String>),
}

impl Ctl {
    pub(crate) fn value(&self) -> String {
        match self {
            Ctl::Check(c) => c.is_active().to_string(),
            Ctl::Entry(e) => e.text().to_string(),
            Ctl::Lines(t, _) => {
                let b = t.buffer();
                b.text(&b.start_iter(), &b.end_iter(), false).to_string()
            }
            Ctl::Choice(d, values) => values.get(d.selected() as usize).cloned().unwrap_or_default(),
        }
    }

    pub(crate) fn set(&self, value: &str) {
        match self {
            Ctl::Check(c) => c.set_active(value == "true"),
            Ctl::Entry(e) => e.set_text(value),
            // (written only when it differs: the cursor and the scroll position stay)
            Ctl::Lines(t, _) => {
                if self.value() != value {
                    t.buffer().set_text(value);
                }
            }
            Ctl::Choice(d, values) => {
                d.set_selected(values.iter().position(|v| v == value).unwrap_or(0) as u32)
            }
        }
    }

    /// The widget a page lays out (a multi-line box's scrolled window).
    pub(crate) fn widget(&self) -> gtk::Widget {
        match self {
            Ctl::Check(c) => c.clone().upcast(),
            Ctl::Entry(e) => e.clone().upcast(),
            Ctl::Lines(_, s) => s.clone().upcast(),
            Ctl::Choice(d, _) => d.clone().upcast(),
        }
    }

    /// The widget that takes the focus.
    pub(crate) fn focus_widget(&self) -> gtk::Widget {
        match self {
            Ctl::Lines(t, _) => t.clone().upcast(),
            other => other.widget(),
        }
    }
}

/// The Advanced page's table and value controls.
struct Advanced {
    filter: gtk::Entry,
    table: Table,
    value: gtk::Entry,
    yes: gtk::CheckButton,
    no: gtk::CheckButton,
    set: gtk::Button,
    reset: gtk::Button,
}

/// What the handlers share. It holds no window, so the widgets' closures can keep it without
/// a cycle (they hold it weakly); the window's close handler keeps it alive.
struct Dialog {
    pages: Vec<Page>,
    /// The working copy every page edits.
    opts: RefCell<Options>,
    form: RefCell<Form>,
    controls: HashMap<&'static str, Ctl>,
    /// Rows greyed unless the check boxes named are on (or while others are).
    needs: Vec<(gtk::Widget, Grey)>,
    stack: gtk::Stack,
    /// The page on screen, to go back to when the pages cannot be read.
    page: RefCell<String>,
    ip_status: Option<gtk::Label>,
    /// Reload's result, which the IP filter's line shows until the page's source changes.
    ip_reload: RefCell<Option<Result<usize, String>>>,
    /// The Trackers page's status line for the list from the web.
    tracker_status: Option<gtk::Label>,
    advanced: Option<Advanced>,
    /// The Scheduler page's table and what it showed when written, so an untouched table keeps
    /// the stored one.
    sched: Option<SchedGrid>,
    sched_shown: RefCell<[Mode; CELLS]>,
    /// Advanced > UI Lock's switch, buttons and status line.
    lock: Option<LockPage>,
    /// The Advanced table's rows.
    rows: RefCell<Vec<&'static Entry>>,
    /// The controls with a help text (D55); the Advanced table's is its selected row's.
    helps: Vec<(gtk::Widget, H)>,
    /// The help pane's text.
    pane: gtk::Label,
    /// The control under the pointer, and the one focused last outside the pane.
    hovered: RefCell<Option<gtk::Widget>>,
    focused: RefCell<Option<gtk::Widget>>,
    hooks: Hooks,
}

/// Shows the dialog over `parent`, editing `options` as stored, at the page whose key is `page`
/// (the first page for `None` or a key it does not have).
pub fn show(parent: &gtk::ApplicationWindow, options: Options, hooks: Hooks, page: Option<&str>) {
    let window = gtk::Window::builder()
        .title(tr!(PrefTitle))
        .transient_for(parent)
        .modal(true)
        .default_width(760)
        .default_height(560)
        .build();
    let mut pages = pref_pages::pages(Features {
        desktop: crate::integration::AVAILABLE,
        packaged: crate::integration::packaged(),
        tray: hooks.tray,
        // a Flatpak is updated by Flathub
        updates: rookparcel_frontend::update::AVAILABLE && !crate::integration::packaged(),
        alternate_rows: true,
        // GTK follows the desktop's theme (D63); no drop-down of its own
        theme: false,
    });
    if !crate::integration::associations_settable() {
        // a Flatpak: the package's own entry declares the types, autostart still works
        fix_fields(&mut pages, &["assoc.torrent", "assoc.magnet", "ui.check_assoc"]);
    }
    let stack = gtk::Stack::new();
    stack.set_hexpand(true);
    stack.set_vexpand(true);

    let mut b = Builder::default();
    // the page list: a page's symbol and title (a StackSidebar shows titles only)
    let list = gtk::ListBox::new();
    list.set_selection_mode(gtk::SelectionMode::Single);
    list.add_css_class("navigation-sidebar");
    for page in &pages {
        let child = b.page(page, &options);
        let title = lang::text(page.title);
        stack.add_titled(&child, Some(page.key), title);
        let label = gtk::Label::new(Some(title));
        label.set_xalign(0.0);
        let content = match page_icon(page.title) {
            Some(icon) => crate::icons::with_icon(icon, &label).upcast::<gtk::Widget>(),
            None => label.upcast(),
        };
        if page.parent.is_some() {
            // one level down, as under Advanced in µTorrent's tree
            content.set_margin_start(18);
        }
        let row = gtk::ListBoxRow::new();
        row.set_child(Some(&content));
        row.set_widget_name(page.key);
        list.append(&row);
    }
    list.connect_row_selected({
        let stack = stack.clone();
        move |_, row| {
            if let Some(row) = row {
                stack.set_visible_child_name(row.widget_name().as_str());
            }
        }
    });
    // and back: a page shown by the dialog itself (an error's) selects its row
    stack.connect_visible_child_name_notify({
        let list = list.clone();
        move |stack| {
            let Some(name) = stack.visible_child_name() else { return };
            let mut i = 0;
            while let Some(row) = list.row_at_index(i) {
                if row.widget_name() == name && !row.is_selected() {
                    list.select_row(Some(&row));
                }
                i += 1;
            }
        }
    });
    let first = page.and_then(|key| pages.iter().position(|p| p.key == key)).unwrap_or(0);
    list.select_row(list.row_at_index(first as i32).as_ref());
    let sidebar = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .child(&list)
        .width_request(170)
        .build();
    let body_pages = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    body_pages.set_vexpand(true);
    body_pages.append(&sidebar);
    body_pages.append(&stack);

    let ok = gtk::Button::with_label(tr!(BtnOk));
    ok.add_css_class("suggested-action");
    let cancel = gtk::Button::with_label(tr!(BtnCancel));
    let apply = gtk::Button::with_mnemonic(&gtk_mnemonic(tr!(BtnApply)));
    // the hint at the start of the buttons' row
    let hint = gtk::Label::new(Some(tr!(PrefHelpHint)));
    hint.set_xalign(0.0);
    hint.set_hexpand(true);
    hint.set_ellipsize(gtk::pango::EllipsizeMode::End);
    hint.add_css_class("dim-label");
    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    buttons.append(&hint);
    buttons.append(&ok);
    buttons.append(&cancel);
    buttons.append(&apply);
    // the help pane under the pages: about three lines, scrolling when a text is longer
    let pane = gtk::Label::new(None);
    pane.set_xalign(0.0);
    pane.set_yalign(0.0);
    pane.set_wrap(true);
    pane.set_wrap_mode(gtk::pango::WrapMode::WordChar);
    pane.set_selectable(true);
    pane.set_margin_top(4);
    pane.set_margin_bottom(4);
    pane.set_margin_start(6);
    pane.set_margin_end(6);
    let line = pane.create_pango_layout(Some("Ag")).pixel_size().1.max(1);
    let pane_box = gtk::ScrolledWindow::builder()
        .child(&pane)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_height(line * 3 + 8)
        .max_content_height(line * 3 + 8)
        .has_frame(true)
        .build();
    let body = gtk::Box::new(gtk::Orientation::Vertical, 12);
    body.set_margin_top(12);
    body.set_margin_bottom(12);
    body.set_margin_start(12);
    body.set_margin_end(12);
    body.append(&body_pages);
    body.append(&pane_box);
    body.append(&buttons);
    window.set_child(Some(&body));
    window.set_default_widget(Some(&ok));
    let keys = gtk::ShortcutController::new();
    keys.add_shortcut(gtk::Shortcut::new(
        gtk::ShortcutTrigger::parse_string("Escape"),
        Some(gtk::NamedAction::new("window.close")),
    ));
    window.add_controller(keys);

    let Builder {
        controls,
        needs,
        ip_status,
        tracker_status,
        advanced,
        sched,
        browse,
        actions,
        helps,
        lock_switch,
        lock_buttons,
        lock_status,
        ..
    } = b;
    let lock = match (lock_switch, lock_status) {
        (Some(switch), Some(status)) => {
            Some(LockPage { switch, buttons: lock_buttons, status, quiet: Cell::new(false) })
        }
        _ => None,
    };
    let d = Rc::new(Dialog {
        page: RefCell::new(pages.first().map(|p| p.key.to_owned()).unwrap_or_default()),
        pages,
        opts: RefCell::new(options),
        form: RefCell::default(),
        controls,
        needs,
        stack: stack.clone(),
        ip_status,
        ip_reload: RefCell::default(),
        tracker_status,
        advanced,
        sched,
        sched_shown: RefCell::new([Mode::Full; CELLS]),
        lock,
        rows: RefCell::default(),
        helps,
        pane,
        hovered: RefCell::default(),
        focused: RefCell::default(),
        hooks,
    });
    DIALOG.with(|x| *x.borrow_mut() = Some(Rc::downgrade(&d)));

    // help (D55): the pane follows the pointer over the controls and the focus
    let mut hoverable: Vec<gtk::Widget> = d.helps.iter().map(|(w, _)| w.clone()).collect();
    if let Some(a) = &d.advanced {
        hoverable.push(a.table.view.clone().upcast());
    }
    for widget in hoverable {
        let motion = gtk::EventControllerMotion::new();
        let (w, over) = (Rc::downgrade(&d), widget.downgrade());
        motion.connect_enter(move |_, _, _| {
            if let (Some(d), Some(over)) = (w.upgrade(), over.upgrade()) {
                *d.hovered.borrow_mut() = Some(over);
                d.show_help();
            }
        });
        let (w, over) = (Rc::downgrade(&d), widget.downgrade());
        motion.connect_leave(move |_| {
            let (Some(d), Some(over)) = (w.upgrade(), over.upgrade()) else { return };
            let left = d.hovered.borrow().as_ref() == Some(&over);
            if left {
                *d.hovered.borrow_mut() = None;
                d.show_help();
            }
        });
        widget.add_controller(motion);
    }
    window.connect_focus_widget_notify({
        let w = Rc::downgrade(&d);
        move |window| {
            let Some(d) = w.upgrade() else { return };
            let focus = GtkWindowExt::focus(window);
            // selecting text in the pane keeps the text it shows
            if focus.as_ref().is_some_and(|f| *f == d.pane || f.is_ancestor(&d.pane)) {
                return;
            }
            *d.focused.borrow_mut() = focus;
            d.show_help();
        }
    });
    // F1: the focused control's help beside it, then its wiki page when there is one
    let f1 = gtk::ShortcutController::new();
    f1.add_shortcut(gtk::Shortcut::new(
        gtk::ShortcutTrigger::parse_string("F1"),
        Some(gtk::CallbackAction::new({
            let w = Rc::downgrade(&d);
            move |widget, _| {
                let Some(d) = w.upgrade() else { return glib::Propagation::Proceed };
                let Some(window) = widget.downcast_ref::<gtk::Window>() else {
                    return glib::Propagation::Proceed;
                };
                d.f1(window);
                glib::Propagation::Stop
            }
        })),
    ));
    window.add_controller(f1);

    // the check boxes and the proxy type grey what depends on them
    for ctl in d.controls.values() {
        let w = Rc::downgrade(&d);
        let update = move || {
            if let Some(d) = w.upgrade() {
                d.update_sensitivity();
            }
        };
        match ctl {
            Ctl::Check(c) => {
                c.connect_toggled(move |_| update());
            }
            Ctl::Choice(dd, _) => {
                dd.connect_selected_notify(move |_| update());
            }
            Ctl::Entry(_) | Ctl::Lines(..) => {}
        }
    }
    // the list's status line follows the page's box and address at once
    if let Some(Ctl::Check(c)) = d.controls.get(TRACKERS_FROM_URL) {
        let w = Rc::downgrade(&d);
        c.connect_toggled(move |_| {
            if let Some(d) = w.upgrade() {
                d.update_tracker_status();
            }
        });
    }
    if let Some(Ctl::Entry(e)) = d.controls.get(TRACKERS_URL) {
        let w = Rc::downgrade(&d);
        e.connect_changed(move |_| {
            if let Some(d) = w.upgrade() {
                d.update_tracker_status();
            }
        });
    }
    // the IP filter's line follows the page's source at once (D75); a Reload result goes
    for key in [IPFILTER_FROM_URL, IPFILTER_URL, IPFILTER_PATH] {
        let w = Rc::downgrade(&d);
        let changed = move || {
            if let Some(d) = w.upgrade() {
                d.ip_reload.replace(None);
                d.update_ip_status();
            }
        };
        match d.controls.get(key) {
            Some(Ctl::Check(c)) => {
                c.connect_toggled(move |_| changed());
            }
            Some(Ctl::Entry(e)) => {
                e.connect_changed(move |_| changed());
            }
            _ => {}
        }
    }
    // "..." beside folders and files
    for (button, key, kind) in browse {
        let (w, window) = (Rc::downgrade(&d), window.downgrade());
        button.connect_clicked(move |_| {
            let (Some(d), Some(window)) = (w.upgrade(), window.upgrade()) else { return };
            let Some(Ctl::Entry(entry)) = d.controls.get(key) else { return };
            pick(&window, entry, kind);
        });
    }
    for (button, action) in actions {
        let (w, window) = (Rc::downgrade(&d), window.downgrade());
        button.connect_clicked(move |_| {
            if let (Some(d), Some(window)) = (w.upgrade(), window.upgrade()) {
                d.act(action, &window);
            }
        });
    }
    // Advanced > UI Lock's switch (D71): it acts at once
    if let Some(l) = &d.lock {
        let (w, window) = (Rc::downgrade(&d), window.downgrade());
        l.switch.connect_toggled(move |c| {
            let (Some(d), Some(window)) = (w.upgrade(), window.upgrade()) else { return };
            if d.lock.as_ref().is_some_and(|l| !l.quiet.get()) {
                d.lock_switched(c.is_active(), &window);
            }
        });
    }
    if let Some(a) = &d.advanced {
        let w = Rc::downgrade(&d);
        a.filter.connect_changed(move |_| {
            if let Some(d) = w.upgrade() {
                d.fill_advanced(None);
            }
        });
        let w = Rc::downgrade(&d);
        a.table.selection.connect_selection_changed(move |_, _, _| {
            if let Some(d) = w.upgrade() {
                d.show_value();
                d.show_help();
            }
        });
        for (button, reset) in [(&a.set, false), (&a.reset, true)] {
            let w = Rc::downgrade(&d);
            button.connect_clicked(move |_| {
                if let Some(d) = w.upgrade() {
                    d.advanced_set(reset);
                }
            });
        }
        // Enter in the value sets it instead of closing the dialog
        let w = Rc::downgrade(&d);
        a.value.connect_activate(move |_| {
            if let Some(d) = w.upgrade() {
                d.advanced_set(false);
            }
        });
    }
    // the Advanced table shows what the pages say: they are read when it opens
    stack.connect_visible_child_name_notify({
        let w = Rc::downgrade(&d);
        move |stack| {
            let Some(d) = w.upgrade() else { return };
            let Some(name) = stack.visible_child_name() else { return };
            if *d.page.borrow() == name.as_str() {
                return;
            }
            if name == "advanced" && !d.read_or_complain() {
                return;
            }
            if name == "advanced" {
                d.fill_advanced(None);
            }
            *d.page.borrow_mut() = name.to_string();
        }
    });

    apply.connect_clicked({
        let (w, window) = (Rc::downgrade(&d), window.downgrade());
        move |_| {
            if let (Some(d), Some(window)) = (w.upgrade(), window.upgrade()) {
                d.apply(&window);
            }
        }
    });
    ok.connect_clicked({
        let (w, window) = (Rc::downgrade(&d), window.downgrade());
        move |_| {
            let (Some(d), Some(window)) = (w.upgrade(), window.upgrade()) else { return };
            if d.apply(&window) {
                window.close();
            }
        }
    });
    cancel.connect_clicked({
        let window = window.downgrade();
        move |_| {
            if let Some(w) = window.upgrade() {
                w.close();
            }
        }
    });
    window.connect_close_request({
        let d = d.clone();
        move |_| {
            // the shared state goes with the window
            let _ = &d;
            glib::Propagation::Proceed
        }
    });

    d.write_controls();
    d.update_ip_status();
    d.update_tracker_status();
    d.update_lock();
    if d.tracker_status.is_some() || d.ip_status.is_some() {
        // the lists are fetched on their own time: a line follows while its page is on screen,
        // until the dialog is gone
        let w = Rc::downgrade(&d);
        glib::timeout_add_seconds_local(1, move || {
            let Some(d) = w.upgrade() else { return glib::ControlFlow::Break };
            if d.tracker_status.as_ref().is_some_and(|l| l.is_mapped()) {
                d.update_tracker_status();
            }
            if d.ip_status.as_ref().is_some_and(|l| l.is_mapped()) {
                d.update_ip_status();
            }
            glib::ControlFlow::Continue
        });
    }
    window.present();
}

impl Dialog {
    /// The help of `widget` or the control holding it (an entry's text, a drop-down's button),
    /// with that control: the Advanced table's is its selected row's.
    fn help_for(&self, widget: &gtk::Widget) -> Option<(gtk::Widget, H)> {
        let view = self.advanced.as_ref().map(|a| a.table.view.clone().upcast::<gtk::Widget>());
        let mut at = Some(widget.clone());
        while let Some(w) = at {
            if view.as_ref() == Some(&w) {
                return self.row_help().map(|h| (w, h));
            }
            if let Some((_, h)) = self.helps.iter().find(|(c, _)| *c == w) {
                return Some((w, *h));
            }
            at = w.parent();
        }
        None
    }

    /// The help of the Advanced table's selected row.
    fn row_help(&self) -> Option<H> {
        let i = self.selected_row()?;
        let name = self.rows.borrow().get(i)?.name;
        help::for_setting(name)
    }

    /// The pane: the help of the control under the pointer, else of the focused one; and the
    /// Advanced table's tooltip, its selected row's help.
    fn show_help(&self) {
        let of = |w: &RefCell<Option<gtk::Widget>>| w.borrow().as_ref().and_then(|w| self.help_for(w));
        let h = of(&self.hovered).or_else(|| of(&self.focused)).map(|(_, h)| h);
        self.pane.set_text(h.map_or("", lang::help));
        if let Some(a) = &self.advanced {
            a.table.view.set_tooltip_text(self.row_help().map(lang::help));
        }
    }

    /// F1: the focused control's help in the pane and beside the control (GTK shows no tooltip
    /// on demand), then its wiki page when there is one.
    fn f1(&self, window: &gtk::Window) {
        let Some(focus) = GtkWindowExt::focus(window) else { return };
        let Some((control, h)) = self.help_for(&focus) else { return };
        *self.hovered.borrow_mut() = None;
        *self.focused.borrow_mut() = Some(focus);
        self.show_help();
        let text = gtk::Label::new(Some(lang::help(h)));
        text.set_wrap(true);
        text.set_max_width_chars(60);
        text.set_xalign(0.0);
        let popover = gtk::Popover::new();
        popover.set_child(Some(&text));
        popover.set_parent(&control);
        popover.connect_closed(|p| {
            // unparent once the popover is down
            let p = p.clone();
            glib::idle_add_local_once(move || p.unparent());
        });
        popover.popup();
        if let Some(url) = help::wiki_url(h) {
            gtk::UriLauncher::new(&url).launch(Some(window), None::<&gio::Cancellable>, |_| {});
        }
    }

    /// The controls from the working options.
    fn write_controls(&self) {
        let values = self.form.borrow_mut().write(&self.pages, &self.opts.borrow());
        for (key, value) in values {
            if let Some(c) = self.controls.get(key) {
                c.set(&value);
            }
        }
        if let Some(g) = &self.sched {
            let table = self.opts.borrow().gui.schedule.table;
            g.set(&table);
            *self.sched_shown.borrow_mut() = table;
        }
        self.update_sensitivity();
    }

    fn update_sensitivity(&self) {
        let on = |k: &str| {
            let Some(ctl) = self.controls.get(k) else { return false };
            let value = ctl.value();
            pref_pages::fields(&self.pages).any(|f| f.key == k && pref_pages::is_on(&f.control, &value))
        };
        for (widget, grey) in &self.needs {
            widget.set_sensitive(grey.enabled(on));
        }
    }

    /// The value the control of the field `key` holds now.
    fn value(&self, key: &str) -> String {
        self.controls.get(key).map(Ctl::value).unwrap_or_default()
    }

    /// The typed tracker list's message when a line of it is no tracker address: reading the
    /// box tidies the list, which would drop such a line without a word.
    fn bad_tracker_lines(&self) -> Option<(&'static str, String)> {
        let append = self.value(TRACKERS_APPEND) == "true";
        pref_pages::live::bad_tracker_lines(append, &self.value(TRACKERS_LIST)).map(|m| (TRACKERS_LIST, m))
    }

    /// Shows the page of the field `key`, says what is wrong with it and focuses its control.
    fn complain(&self, key: &str, message: String) {
        if let Some(page) = pref_pages::page_of(&self.pages, key) {
            *self.page.borrow_mut() = page.key.to_owned();
            self.stack.set_visible_child_name(page.key);
        }
        let parent = self.stack.root().and_downcast::<gtk::Window>();
        gtk::AlertDialog::builder().message(message).modal(true).build().show(parent.as_ref());
        if let Some(c) = self.controls.get(key) {
            c.focus_widget().grab_focus();
        }
    }

    /// Reads the changed controls into the working options; on a bad value shows its page,
    /// says what is wrong and focuses the control.
    fn read_or_complain(&self) -> bool {
        let read = match self.bad_tracker_lines() {
            Some(bad) => Err(bad),
            None => {
                let form = self.form.borrow();
                let opts = self.opts.borrow();
                form.read(&self.pages, &opts, |k| self.value(k))
            }
        };
        match read {
            Ok(mut o) => {
                if let Some(g) = &self.sched {
                    let table = g.table();
                    if table != *self.sched_shown.borrow() {
                        o.gui.schedule.table = table;
                    }
                }
                *self.opts.borrow_mut() = o;
                true
            }
            Err((key, message)) => {
                self.complain(key, message);
                false
            }
        }
    }

    /// OK and Apply: false when a value is wrong.
    fn apply(&self, window: &gtk::Window) -> bool {
        let language = self.opts.borrow().gui.language.clone();
        if !self.read_or_complain() {
            return false;
        }
        (self.hooks.apply)(&self.opts.borrow());
        // the windows are built in the language of the start
        if self.opts.borrow().gui.language != language {
            gtk::AlertDialog::builder()
                .message(tr!(LangRestart))
                .modal(true)
                .build()
                .show(window.transient_for().as_ref());
        }
        self.write_controls();
        self.fill_advanced(self.selected_row());
        self.update_tracker_status();
        self.update_ip_status();
        true
    }

    /// A page's button; `window` is the Preferences window.
    fn act(self: &Rc<Self>, action: Action, window: &gtk::Window) {
        match action {
            Action::SetPin | Action::ChangePin | Action::RemovePin => {
                if let Some(form) = action.pin_form() {
                    self.pin_form(form, window);
                }
            }
            Action::RandomPort => {
                if let Some(c) = self.controls.get("port") {
                    c.set(&options::random_port().to_string());
                }
            }
            Action::ReloadIpFilter => {
                if self.value(IPFILTER_ENABLE) != "true" {
                    self.ip_reload.replace(Some(Err(tr!(PrefIpFilterOff).to_owned())));
                    self.update_ip_status();
                    return;
                }
                let mut gui = self.opts.borrow().gui.clone();
                gui.ip_filter_path = self.value(IPFILTER_PATH).trim().to_owned();
                gui.ip_filter_from_url = self.value(IPFILTER_FROM_URL) == "true";
                gui.ip_filter_url = self.value(IPFILTER_URL).trim().to_owned();
                // the page's file, or the copy kept of its address's list (none yet: the line
                // says how the fetch stands)
                let result = ip_filter_list::list_file(&gui, &self.hooks.settings_dir).map(|file| {
                    (self.hooks.reload_ip_filter)(&file).map_err(|e| format!("{}: {e}", file.display()))
                });
                self.ip_reload.replace(result);
                self.update_ip_status();
            }
            Action::UpdateTrackerList => self.update_tracker_list(),
            Action::UpdateIpFilter => self.update_ip_filter(),
        }
    }

    /// IP Filter > Update Now (D75): the group's four values are stored and put into effect, as
    /// Trackers' Update Now does for its own, then the list is fetched from the page's address.
    /// The status line and the Logger say how it went.
    fn update_ip_filter(&self) {
        const KEYS: [&str; 4] = [IPFILTER_ENABLE, IPFILTER_PATH, IPFILTER_FROM_URL, IPFILTER_URL];
        let mut o = self.opts.borrow().clone();
        for f in pref_pages::fields(&self.pages).filter(|f| KEYS.contains(&f.key)) {
            if let Err(message) = f.apply(&mut o, &self.value(f.key)) {
                self.complain(f.key, message);
                return;
            }
        }
        // what `pref_pages::check` asks of them (the other pages may be half filled in)
        let g = &o.gui;
        if g.ip_filter
            && g.ip_filter_from_url
            && !rookparcel_frontend::tracker_list::is_list_url(&g.ip_filter_url)
        {
            self.complain(IPFILTER_URL, tr!(TrackersErrHttps).to_owned());
            return;
        }
        (self.hooks.update_ip_filter)(&o.gui);
        {
            let mut opts = self.opts.borrow_mut();
            opts.gui.ip_filter = o.gui.ip_filter;
            opts.gui.ip_filter_path = o.gui.ip_filter_path;
            opts.gui.ip_filter_from_url = o.gui.ip_filter_from_url;
            opts.gui.ip_filter_url = o.gui.ip_filter_url;
        }
        // (the other controls keep what was typed into them, as after Trackers' Update Now)
        let values = self.form.borrow_mut().write(&self.pages, &self.opts.borrow());
        for (key, value) in values.iter().filter(|(k, _)| KEYS.contains(k)) {
            if let Some(c) = self.controls.get(key) {
                c.set(value);
            }
        }
        self.ip_reload.replace(None);
        self.update_ip_status();
    }

    /// The PIN dialog `form` over the Preferences window, modal on it (D71); the page follows
    /// what it did once it is gone (a Cancel of [`LockForm::Off`] ticks the switch again).
    fn pin_form(self: &Rc<Self>, form: LockForm, window: &gtk::Window) {
        let w = Rc::downgrade(self);
        (self.hooks.pin_form)(
            form,
            window,
            Box::new(move || {
                if let Some(d) = w.upgrade() {
                    d.update_lock();
                }
            }),
        );
    }

    /// UI Lock's switch clicked (D71). Off with a PIN set asks for it first; otherwise the
    /// lock goes on or off at once.
    fn lock_switched(self: &Rc<Self>, on: bool, window: &gtk::Window) {
        if !on && (self.hooks.lock_state)() == ui_lock::State::Pin {
            self.pin_form(LockForm::Off, window);
        } else {
            (self.hooks.lock_enable)(on);
            self.update_lock();
        }
    }

    /// UI Lock's switch, buttons and status line from the lock's state as stored.
    fn update_lock(&self) {
        let Some(l) = &self.lock else { return };
        let state = (self.hooks.lock_state)();
        l.quiet.set(true);
        l.switch.set_active(state != ui_lock::State::Off);
        l.quiet.set(false);
        for (button, form) in &l.buttons {
            button.set_sensitive(state.allows(*form));
        }
        l.status.set_text(state.text());
    }

    /// Trackers > Update Now: the page's four values are stored and put into effect (as Apply
    /// does, for them alone: a list is fetched from the address the page shows, not from the
    /// one stored before), then the list is fetched. The status line and the Logger say how
    /// it went.
    fn update_tracker_list(&self) {
        if let Some((key, message)) = self.bad_tracker_lines() {
            self.complain(key, message);
            return;
        }
        const KEYS: [&str; 4] = [TRACKERS_APPEND, TRACKERS_LIST, TRACKERS_FROM_URL, TRACKERS_URL];
        let mut o = self.opts.borrow().clone();
        for f in pref_pages::fields(&self.pages).filter(|f| KEYS.contains(&f.key)) {
            if let Err(message) = f.apply(&mut o, &self.value(f.key)) {
                self.complain(f.key, message);
                return;
            }
        }
        // what `pref_pages::check` asks of them (the other pages may be half filled in)
        if o.gui.trackers_from_url && !rookparcel_frontend::tracker_list::is_list_url(&o.gui.trackers_url) {
            self.complain(TRACKERS_URL, tr!(TrackersErrHttps).to_owned());
            return;
        }
        (self.hooks.update_tracker_list)(&o.gui);
        // they are stored now: the working copy and the controls have them as applied
        {
            let mut opts = self.opts.borrow_mut();
            opts.gui.trackers_append = o.gui.trackers_append;
            opts.gui.trackers_list = o.gui.trackers_list;
            opts.gui.trackers_from_url = o.gui.trackers_from_url;
            opts.gui.trackers_url = o.gui.trackers_url;
        }
        // (the other controls keep what was typed into them: they still differ from the
        // working copy, so Apply reads them)
        let values = self.form.borrow_mut().write(&self.pages, &self.opts.borrow());
        for (key, value) in values.iter().filter(|(k, _)| KEYS.contains(k)) {
            if let Some(c) = self.controls.get(key) {
                c.set(value);
            }
        }
        self.update_tracker_status();
    }

    /// The status line for the page's box and address as they stand, applied or not: a box
    /// switched off has no line even while OK is refused for another field.
    fn update_tracker_status(&self) {
        let Some(label) = &self.tracker_status else { return };
        let from_url = self.value(TRACKERS_FROM_URL) == "true";
        let text = (self.hooks.tracker_status)(from_url, &self.value(TRACKERS_URL));
        if label.text() != text {
            label.set_text(&text);
        }
    }

    /// The IP filter's line (`ip_filter_list::page_status`): Reload's result, else how the list
    /// from the page's address stands while its box is ticked, else the default file.
    fn update_ip_status(&self) {
        let Some(label) = &self.ip_status else { return };
        let url_status = (self.value(IPFILTER_FROM_URL) == "true")
            .then(|| (self.hooks.ip_filter_status)(true, &self.value(IPFILTER_URL)));
        let text = ip_filter_list::page_status(
            self.ip_reload.borrow().as_ref(),
            url_status,
            &self.value(IPFILTER_PATH),
            &self.hooks.settings_dir,
        );
        if label.text() != text {
            label.set_text(&text);
        }
    }

    fn selected_row(&self) -> Option<usize> {
        self.advanced.as_ref()?.table.selected().first().copied()
    }

    /// Fills the table with the rows matching the filter; selects `select` (a row) again.
    fn fill_advanced(&self, select: Option<usize>) {
        let Some(a) = &self.advanced else { return };
        let keep = select.and_then(|i| self.rows.borrow().get(i).map(|e| e.name));
        *self.rows.borrow_mut() = advanced_rows(&a.filter.text());
        let n = self.rows.borrow().len();
        // a new filter can keep the count, so the rows are redrawn either way
        a.table.update(n);
        a.table.unselect_all();
        if let Some(i) = keep.and_then(|name| self.rows.borrow().iter().position(|e| e.name == name)) {
            a.table.selection.select_item(i as u32, true);
            a.table.view.scroll_to(i as u32, None, gtk::ListScrollFlags::NONE, None);
        }
        self.show_value();
    }

    /// The value controls for the selected row: True / False for switches, a field otherwise.
    fn show_value(&self) {
        let Some(a) = &self.advanced else { return };
        let entry = self.selected_row().and_then(|i| self.rows.borrow().get(i).copied());
        let is_bool = entry.is_some_and(|e| e.kind == Kind::Bool);
        a.value.set_visible(!is_bool);
        a.yes.set_visible(is_bool);
        a.no.set_visible(is_bool);
        match entry.map(|e| e.get(&self.opts.borrow().settings)) {
            Some(Value::Bool(v)) => {
                a.yes.set_active(v);
                a.no.set_active(!v);
            }
            Some(v) => a.value.set_text(&v.to_string()),
            None => a.value.set_text(""),
        }
        for w in [a.value.upcast_ref::<gtk::Widget>(), a.set.upcast_ref(), a.reset.upcast_ref()] {
            w.set_sensitive(entry.is_some());
        }
    }

    /// "Set" (or "Reset" to the default) for the selected row; the pages follow.
    fn advanced_set(&self, reset: bool) {
        let Some(a) = &self.advanced else { return };
        let Some(i) = self.selected_row() else { return };
        let Some(e) = self.rows.borrow().get(i).copied() else { return };
        let text = if reset {
            e.get(&rookparcel_core::Settings::default()).to_string()
        } else if e.kind == Kind::Bool {
            a.yes.is_active().to_string()
        } else {
            a.value.text().to_string()
        };
        let result = e.set(&mut self.opts.borrow_mut().settings, &text);
        if let Err(message) = result {
            let parent = self.stack.root().and_downcast::<gtk::Window>();
            gtk::AlertDialog::builder().message(message).modal(true).build().show(parent.as_ref());
            a.value.grab_focus();
            return;
        }
        // the pages show the table's values (they were read when Advanced opened)
        self.write_controls();
        self.fill_advanced(Some(i));
    }

    /// A cell of the Advanced table.
    fn cell(&self, row: usize, column: usize) -> String {
        let Some(e) = self.rows.borrow().get(row).copied() else { return String::new() };
        let opts = self.opts.borrow();
        match column {
            0 => advanced_name(e, &opts.settings),
            _ => e.get(&opts.settings).to_string(),
        }
    }
}

/// Greys the fields `keys` ([`Field::fixed`]): shown, never edited.
fn fix_fields(pages: &mut [Page], keys: &[&str]) {
    for page in pages {
        for item in &mut page.items {
            if let Item::Field(f) = item {
                if keys.contains(&f.key) {
                    f.fixed = true;
                }
            }
        }
    }
}

/// A folder or file picker for `entry`.
pub(crate) fn pick(window: &gtk::Window, entry: &gtk::Entry, kind: TextKind) {
    let (title, folder) = match kind {
        TextKind::Folder(t) => (t, true),
        TextKind::File(t) => (t, false),
        _ => return,
    };
    let dialog = gtk::FileDialog::builder().title(lang::text(title)).modal(true).build();
    let now = entry.text();
    let now = Path::new(now.trim());
    if folder && now.is_dir() {
        dialog.set_initial_folder(Some(&gio::File::for_path(now)));
    } else if !folder && now.is_file() {
        dialog.set_initial_file(Some(&gio::File::for_path(now)));
    }
    let entry = entry.downgrade();
    let done = move |r: Result<gio::File, glib::Error>| {
        if let (Some(path), Some(entry)) = (r.ok().and_then(|f| f.path()), entry.upgrade()) {
            entry.set_text(&path.to_string_lossy());
        }
    };
    if folder {
        dialog.select_folder(Some(window), None::<&gio::Cancellable>, done);
    } else {
        dialog.open(Some(window), None::<&gio::Cancellable>, done);
    }
}

/// Builds the pages' widgets and collects what the dialog needs from them.
#[derive(Default)]
pub(crate) struct Builder {
    pub(crate) controls: HashMap<&'static str, Ctl>,
    pub(crate) needs: Vec<(gtk::Widget, Grey)>,
    ip_status: Option<gtk::Label>,
    tracker_status: Option<gtk::Label>,
    advanced: Option<Advanced>,
    sched: Option<SchedGrid>,
    /// "..." buttons: the button, its field and the picker.
    pub(crate) browse: Vec<(gtk::Button, &'static str, TextKind)>,
    actions: Vec<(gtk::Button, Action)>,
    /// Advanced > UI Lock (D71): the switch, the PIN buttons (on one row) and the status line.
    lock_switch: Option<gtk::CheckButton>,
    lock_buttons: Vec<(gtk::Button, LockForm)>,
    lock_status: Option<gtk::Label>,
    /// The row the next `beside` field or button joins.
    last_row: Option<gtk::Box>,
    /// The controls with a help text, which is also their tooltip.
    helps: Vec<(gtk::Widget, H)>,
}

impl Builder {
    /// Gives `widget` the help `h`: its tooltip, and the pane's text while it is hovered or
    /// focused.
    pub(crate) fn help(&mut self, widget: &impl IsA<gtk::Widget>, h: Option<H>) {
        let Some(h) = h else { return };
        widget.set_tooltip_text(Some(lang::help(h)));
        self.helps.push((widget.clone().upcast(), h));
    }

    fn page(&mut self, page: &Page, o: &Options) -> gtk::Widget {
        self.last_row = None;
        let content = gtk::Box::new(gtk::Orientation::Vertical, 12);
        content.set_margin_end(6);
        // where fields go: the page, or the group box opened last
        let mut target = content.clone();
        for item in &page.items {
            match item {
                Item::Group(title) => {
                    let inner = gtk::Box::new(gtk::Orientation::Vertical, 6);
                    inner.set_margin_top(6);
                    inner.set_margin_bottom(8);
                    inner.set_margin_start(8);
                    inner.set_margin_end(8);
                    let frame = gtk::Frame::new(Some(&lang::plain(lang::text(*title))));
                    frame.set_child(Some(&inner));
                    content.append(&frame);
                    target = inner;
                }
                Item::Field(f) => self.field(&target, f, o),
                Item::Note(text) => {
                    let l = gtk::Label::new(Some(&text()));
                    l.set_xalign(0.0);
                    l.set_wrap(true);
                    l.add_css_class("dim-label");
                    target.append(&l);
                }
                Item::Button { action, label } => {
                    let b = gtk::Button::with_mnemonic(&gtk_mnemonic(lang::text(*label)));
                    self.help(&b, pref_pages::item_help(item));
                    if let Some(form) = action.pin_form() {
                        // UI Lock's Set, Change and Remove PIN share a row
                        if self.lock_buttons.is_empty() {
                            let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
                            target.append(&row);
                            self.last_row = Some(row);
                        }
                        self.lock_buttons.push((b.clone(), form));
                    }
                    match &self.last_row {
                        Some(row) => row.append(&b),
                        None => target.append(&b),
                    }
                    self.actions.push((b, *action));
                }
                Item::LockSwitch => {
                    let c = gtk::CheckButton::with_mnemonic(&gtk_mnemonic(tr!(PrefLockEnable)));
                    self.help(&c, pref_pages::item_help(item));
                    target.append(&c);
                    self.last_row = None;
                    self.lock_switch = Some(c);
                }
                Item::LockStatus => {
                    let l = gtk::Label::new(None);
                    l.set_xalign(0.0);
                    l.set_wrap(true);
                    target.append(&l);
                    self.last_row = None;
                    self.lock_status = Some(l);
                }
                Item::IpFilterStatus => {
                    let l = gtk::Label::new(None);
                    l.set_xalign(0.0);
                    l.set_wrap(true);
                    target.append(&l);
                    self.ip_status = Some(l);
                }
                Item::TrackerListStatus => {
                    let l = gtk::Label::new(None);
                    l.set_xalign(0.0);
                    l.set_wrap(true);
                    target.append(&l);
                    self.tracker_status = Some(l);
                }
                Item::AdvancedTable => {
                    let (widget, advanced) = advanced_page();
                    // the table's own help is its selected row's (Dialog::help_for)
                    self.help(&advanced.filter, H::for_label(S::PrefFilter));
                    self.help(&advanced.value, H::for_label(S::PrefValue));
                    self.help(&advanced.yes, H::for_label(S::PrefTrue));
                    self.help(&advanced.no, H::for_label(S::PrefFalse));
                    self.help(&advanced.set, H::for_label(S::PrefSet));
                    self.help(&advanced.reset, H::for_label(S::PrefReset));
                    target.append(&widget);
                    self.advanced = Some(advanced);
                }
                Item::SchedTable => {
                    let grid = SchedGrid::new();
                    self.help(&grid.widget, pref_pages::item_help(item));
                    target.append(&grid.widget);
                    self.needs.push((grid.widget.clone().upcast(), Grey::Needs(&[SCHED_ENABLE])));
                    self.sched = Some(grid);
                }
            }
        }
        if page.items.iter().any(|i| matches!(i, Item::AdvancedTable)) {
            // the table scrolls itself
            return content.upcast();
        }
        gtk::ScrolledWindow::builder()
            .child(&content)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .build()
            .upcast()
    }

    pub(crate) fn field(&mut self, target: &gtk::Box, f: &Field, o: &Options) {
        let caption = |text: S| {
            let l = gtk::Label::with_mnemonic(&gtk_mnemonic(lang::text(text)));
            l.set_xalign(0.0);
            l.set_wrap(true);
            l.set_hexpand(true);
            l
        };
        let (ctl, grey): (Ctl, gtk::Widget) = match f.control {
            Control::Check { .. } => {
                let c = gtk::CheckButton::with_mnemonic(&gtk_mnemonic(f.label.map_or("", lang::text)));
                c.set_hexpand(true);
                (Ctl::Check(c.clone()), c.upcast())
            }
            Control::Text { kind: TextKind::Lines, .. } => {
                // a list to type or paste, as Properties' tracker box: about six lines
                let t = gtk::TextView::new();
                t.set_wrap_mode(gtk::WrapMode::None);
                t.set_accepts_tab(false);
                t.set_left_margin(4);
                t.set_right_margin(4);
                let line = t.create_pango_layout(Some("Ag")).pixel_size().1.max(1);
                let scroll = gtk::ScrolledWindow::builder()
                    .child(&t)
                    .min_content_height(line * 6 + 8)
                    .hexpand(true)
                    .has_frame(true)
                    .build();
                (Ctl::Lines(t, scroll.clone()), scroll.upcast())
            }
            Control::Number { .. } | Control::Text { .. } => {
                let e = gtk::Entry::new();
                e.set_activates_default(true);
                match f.control {
                    Control::Number { .. } => {
                        e.set_width_chars(8);
                        e.set_max_width_chars(10);
                        e.set_input_purpose(gtk::InputPurpose::Number);
                    }
                    Control::Text { kind, .. } => {
                        e.set_hexpand(true);
                        if kind == TextKind::Password {
                            e.set_visibility(false);
                            e.set_input_purpose(gtk::InputPurpose::Password);
                        }
                    }
                    _ => {}
                }
                (Ctl::Entry(e.clone()), e.upcast())
            }
            Control::Choice { choices, .. } => {
                let list = choices(o);
                let texts: Vec<&str> = list.iter().map(|(_, t)| t.as_str()).collect();
                let d = gtk::DropDown::from_strings(&texts);
                (Ctl::Choice(d.clone(), list.into_iter().map(|(v, _)| v).collect()), d.upcast())
            }
        };
        let control = ctl.widget();
        let h = f.help_key();
        self.help(&control, h);
        if f.beside {
            if let Some(row) = &self.last_row {
                row.append(&control);
                if f.fixed {
                    grey.set_sensitive(false);
                } else if let Some(rule) = Grey::of(f) {
                    self.needs.push((grey, rule));
                }
                self.controls.insert(f.key, ctl);
                return;
            }
        }
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        if f.indent {
            row.set_margin_start(20);
        }
        if let (Some(label), false) = (f.label, matches!(f.control, Control::Check { .. })) {
            let l = caption(label);
            l.set_mnemonic_widget(Some(&control));
            self.help(&l, h);
            row.append(&l);
        }
        row.append(&control);
        if let Control::Text { kind: kind @ (TextKind::Folder(_) | TextKind::File(_)), .. } = f.control {
            let b = gtk::Button::with_label("...");
            self.help(&b, h);
            row.append(&b);
            self.browse.push((b, f.key, kind));
        }
        target.append(&row);
        if f.fixed {
            // what a package manages: shown, never edited
            row.set_sensitive(false);
        } else if let Some(rule) = Grey::of(f) {
            // a button beside the field (Reload, Update Now) greys with it
            self.needs.push((row.clone().upcast(), rule));
        }
        self.last_row = Some(row);
        self.controls.insert(f.key, ctl);
    }
}

/// The Advanced page: Filter, the table of every engine setting, the value with Set and Reset.
fn advanced_page() -> (gtk::Widget, Advanced) {
    let filter = gtk::Entry::new();
    filter.set_hexpand(true);
    let filter_label = gtk::Label::with_mnemonic(&gtk_mnemonic(tr!(PrefFilter)));
    filter_label.set_mnemonic_widget(Some(&filter));
    let top = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    top.append(&filter_label);
    top.append(&filter);

    let text: crate::table::CellText = Rc::new(|row, column| {
        DIALOG
            .with(|d| d.borrow().as_ref().and_then(|w| w.upgrade()).map(|d| d.cell(row, column)))
            .unwrap_or_default()
    });
    let table = Table::new(&[(S::ColName, 300, false), (S::ColValue, 140, false)], text, None, None);
    let scroll = gtk::ScrolledWindow::builder().child(&table.view).vexpand(true).has_frame(true).build();

    let value = gtk::Entry::new();
    value.set_hexpand(true);
    let yes = gtk::CheckButton::with_label(tr!(PrefTrue));
    let no = gtk::CheckButton::with_label(tr!(PrefFalse));
    no.set_group(Some(&yes));
    let set = gtk::Button::with_label(tr!(PrefSet));
    let reset = gtk::Button::with_label(tr!(PrefReset));
    let value_label = gtk::Label::new(Some(tr!(PrefValue)));
    let bottom = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    for w in [
        value_label.upcast_ref::<gtk::Widget>(),
        value.upcast_ref(),
        yes.upcast_ref(),
        no.upcast_ref(),
        set.upcast_ref(),
        reset.upcast_ref(),
    ] {
        bottom.append(w);
    }
    let page = gtk::Box::new(gtk::Orientation::Vertical, 8);
    page.set_vexpand(true);
    page.append(&top);
    page.append(&scroll);
    page.append(&bottom);
    (page.upcast(), Advanced { filter, table, value, yes, no, set, reset })
}

thread_local! {
    /// The open dialog, for the Advanced table's cells (the table is built before the dialog's
    /// shared state exists).
    static DIALOG: RefCell<Option<std::rc::Weak<Dialog>>> = const { RefCell::new(None) };
}

#[cfg(test)]
mod tests {
    use rookparcel_frontend::pref_pages::{EXCLUDE_LIST, EXCLUDE_NAMES};

    use super::*;

    fn grey(pages: &[Page], key: &str) -> Option<Grey> {
        Grey::of(pref_pages::fields(pages).find(|f| f.key == key).unwrap())
    }

    /// The rows follow `needs` and `unless` (D74, D75): the IP filter's path (with Reload beside
    /// it) greys while the list comes from an address; the address (with Update Now) needs both
    /// boxes; the exclusion list needs its box.
    #[test]
    fn rows_grey_by_needs_and_unless() {
        let pages = pref_pages::pages(Features::default());
        let on = |keys: &'static [&'static str]| move |k: &str| keys.contains(&k);
        let path = grey(&pages, IPFILTER_PATH).expect("the path greys");
        assert!(path.enabled(on(&[])) && path.enabled(on(&[IPFILTER_ENABLE])));
        assert!(!path.enabled(on(&[IPFILTER_ENABLE, IPFILTER_FROM_URL])));
        let url = grey(&pages, IPFILTER_URL).unwrap();
        assert!(!url.enabled(on(&[])) && !url.enabled(on(&[IPFILTER_FROM_URL])));
        assert!(url.enabled(on(&[IPFILTER_ENABLE, IPFILTER_FROM_URL])));
        let from_url = grey(&pages, IPFILTER_FROM_URL).unwrap();
        assert!(!from_url.enabled(on(&[])) && from_url.enabled(on(&[IPFILTER_ENABLE])));
        let list = grey(&pages, EXCLUDE_LIST).unwrap();
        assert!(!list.enabled(on(&[])) && list.enabled(on(&[EXCLUDE_NAMES])));
        assert!(grey(&pages, IPFILTER_ENABLE).is_none() && grey(&pages, EXCLUDE_NAMES).is_none());
        assert!(Grey::Needs(&[SCHED_ENABLE]).enabled(on(&[SCHED_ENABLE])));
        assert!(!Grey::Needs(&[SCHED_ENABLE]).enabled(on(&[])));
    }
}
