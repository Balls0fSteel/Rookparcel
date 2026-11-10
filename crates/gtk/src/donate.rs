//! Help > Donate: a small window with a few lines on supporting the project, a button that
//! opens the website's donation page (`update::DONATE_URL`) in the browser, and Close (Escape
//! closes too), as in the Win32 client. The ways to give are on that page, so they can change
//! without a new program.

use gtk::gio;
use gtk::prelude::*;
use rookparcel_frontend::lang::{self, gtk_mnemonic};
use rookparcel_frontend::{tr, update};

/// Shows the window over `parent`.
pub fn show(parent: &impl IsA<gtk::Window>) {
    let window = gtk::Window::builder()
        .title(lang::plain(tr!(MenuDonate)))
        .transient_for(parent)
        .modal(true)
        .resizable(false)
        .build();
    let text = gtk::Label::builder()
        .label(tr!(DonateText))
        .wrap(true)
        .max_width_chars(60)
        .xalign(0.0)
        .selectable(false)
        .build();
    let open = gtk::Button::with_mnemonic(&gtk_mnemonic(tr!(DonateOpen)));
    open.add_css_class("suggested-action");
    open.connect_clicked({
        let window = window.downgrade();
        move |_| {
            let Some(w) = window.upgrade() else { return };
            gtk::UriLauncher::new(update::DONATE_URL).launch(
                w.transient_for().as_ref(),
                None::<&gio::Cancellable>,
                |r| {
                    if let Err(e) = r {
                        eprintln!("cannot open the link: {e}");
                    }
                },
            );
            w.close();
        }
    });
    let close = gtk::Button::with_mnemonic(&gtk_mnemonic(tr!(BtnClose)));
    close.connect_clicked({
        let window = window.downgrade();
        move |_| {
            if let Some(w) = window.upgrade() {
                w.close();
            }
        }
    });
    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    buttons.set_halign(gtk::Align::End);
    buttons.append(&open);
    buttons.append(&close);
    let body = gtk::Box::new(gtk::Orientation::Vertical, 14);
    body.set_margin_top(12);
    body.set_margin_bottom(12);
    body.set_margin_start(12);
    body.set_margin_end(12);
    body.append(&text);
    body.append(&buttons);
    window.set_child(Some(&body));
    window.set_default_widget(Some(&open));
    let keys = gtk::ShortcutController::new();
    keys.add_shortcut(gtk::Shortcut::new(
        gtk::ShortcutTrigger::parse_string("Escape"),
        Some(gtk::NamedAction::new("window.close")),
    ));
    window.add_controller(keys);
    window.present();
    open.grab_focus();
}
