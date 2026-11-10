//! Help > Statistics: the all-time and session totals in a small window that stays open beside
//! the main one and follows the figures once a second, as in the Win32 client. The rows come
//! from `rookparcel_frontend::statistics::rows`; their number and order never change, so the
//! labels are built once and only the value texts are replaced after that.

use std::cell::RefCell;

use gtk::prelude::*;
use rookparcel_core::SessionStats;
use rookparcel_frontend::lang::{self, gtk_mnemonic};
use rookparcel_frontend::statistics::{self, Row};
use rookparcel_frontend::tr;

/// The open window and its value labels, in the order of the value rows. The window is held
/// weakly: GTK emits `destroy` only when the last reference goes, so a strong one here would
/// keep a closed window as the open one, to be presented again destroyed.
struct Open {
    window: gtk::glib::WeakRef<gtk::Window>,
    values: Vec<gtk::Label>,
}

thread_local! {
    static OPEN: RefCell<Option<Open>> = const { RefCell::new(None) };
}

/// Shows the window over `parent` with `stats`; a window already open comes to the front.
pub fn show(parent: &impl IsA<gtk::Window>, stats: &SessionStats) {
    let open = OPEN.with(|o| o.borrow().as_ref().and_then(|o| o.window.upgrade()));
    if let Some(window) = open {
        update(stats);
        window.present();
        return;
    }
    let window = gtk::Window::builder()
        .title(lang::plain(tr!(MenuStatistics)))
        .transient_for(parent)
        // it goes with the main window: left open it would stay, with figures that stand still,
        // for as long as the program takes to close
        .destroy_with_parent(true)
        .modal(false)
        .resizable(false)
        .build();
    let grid = gtk::Grid::builder().row_spacing(4).column_spacing(24).build();
    let mut values = Vec::new();
    for (i, row) in statistics::rows(stats).into_iter().enumerate() {
        let line = i as i32;
        match row {
            Row::Heading(title) => {
                let l = gtk::Label::builder().label(title).xalign(0.0).build();
                l.add_css_class("heading");
                if i > 0 {
                    l.set_margin_top(10);
                }
                grid.attach(&l, 0, line, 2, 1);
            }
            Row::Value { label, value } => {
                let l = gtk::Label::builder().label(label).xalign(0.0).margin_start(12).build();
                grid.attach(&l, 0, line, 1, 1);
                // wide enough for a size or a date, so the window keeps its width as they grow
                let v = gtk::Label::builder()
                    .label(value)
                    .xalign(1.0)
                    .halign(gtk::Align::End)
                    .hexpand(true)
                    .width_chars(16)
                    .selectable(true)
                    .build();
                grid.attach(&v, 1, line, 1, 1);
                values.push(v);
            }
        }
    }
    let close = gtk::Button::with_mnemonic(&gtk_mnemonic(tr!(BtnClose)));
    close.set_halign(gtk::Align::End);
    close.connect_clicked({
        let window = window.downgrade();
        move |_| {
            if let Some(w) = window.upgrade() {
                w.close();
            }
        }
    });
    let body = gtk::Box::new(gtk::Orientation::Vertical, 14);
    body.set_margin_top(12);
    body.set_margin_bottom(12);
    body.set_margin_start(12);
    body.set_margin_end(12);
    body.append(&grid);
    body.append(&close);
    window.set_child(Some(&body));
    window.set_default_widget(Some(&close));
    let keys = gtk::ShortcutController::new();
    keys.add_shortcut(gtk::Shortcut::new(
        gtk::ShortcutTrigger::parse_string("Escape"),
        Some(gtk::NamedAction::new("window.close")),
    ));
    window.add_controller(keys);
    // closed (by the button, Escape, the title bar or with the main window): the next
    // Help > Statistics builds a new one
    window.connect_destroy(|_| {
        // (try: never a panic in a signal handler, also not when the window goes at the
        // program's end, after the thread's own storage)
        let _ = OPEN.try_with(|o| {
            if let Ok(mut o) = o.try_borrow_mut() {
                *o = None;
            }
        });
    });
    OPEN.with(|o| *o.borrow_mut() = Some(Open { window: window.downgrade(), values }));
    // (a selectable label that takes the first focus shows its whole text selected, and keeps
    // it selected when the focus moves on: the button has the focus before the window is shown)
    GtkWindowExt::set_focus(&window, Some(&close));
    window.present();
}

/// The engine's new figures (once a second): the open window's values follow. A text that did
/// not change is left alone, so a selection in it stays.
pub fn update(stats: &SessionStats) {
    let Some(labels) = OPEN.with(|o| o.try_borrow().ok()?.as_ref().map(|o| o.values.clone())) else {
        return;
    };
    let texts = statistics::rows(stats).into_iter().filter_map(|r| match r {
        Row::Value { value, .. } => Some(value),
        Row::Heading(_) => None,
    });
    for (label, text) in labels.iter().zip(texts) {
        if label.label() != text {
            label.set_label(&text);
        }
    }
}
