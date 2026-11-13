//! The windows' lock (D69): the PIN dialog, hiding the windows and showing them again, and what
//! waits for the unlock. What a lock means for the main window, the tray and what arrives
//! meanwhile is `crate::app`'s; the PIN rules and the dialogs' texts are
//! `rookparcel_frontend::ui_lock`'s.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk::glib;
use gtk::prelude::*;
use rookparcel_frontend::lang::gtk_mnemonic;
use rookparcel_frontend::tr;
use rookparcel_frontend::ui_lock::{Form, MAX_DIGITS};

/// A PIN dialog: the Unlock prompt, or Set, Change or Remove PIN.
#[derive(Clone)]
pub struct Pin {
    pub window: gtk::Window,
    cancel: gtk::Button,
}

impl Pin {
    /// Shows or hides Cancel (and Escape with it). The Unlock prompt has none without a tray:
    /// it stands in for the main window then, and closing it quits.
    pub fn set_cancel(&self, on: bool) {
        self.cancel.set_visible(on);
    }
}

/// Builds the PIN dialog for `form`, a window of `app`; the caller shows it with `present`.
/// `parent`: the window it is over, modal; `None` for the Unlock prompt, which stands alone
/// while every other window is hidden.
///
/// OK hands what the boxes hold (the current PIN, the new one, the new one again; "" for a box
/// the form does not have) to `submit`: its `Err` shows under the boxes and the dialog stays;
/// after an `Ok` the dialog closes, then `done` runs. Cancel, Escape while Cancel shows, and
/// the title bar's close run `cancelled` as the dialog closes.
pub fn dialog(
    app: &gtk::Application,
    form: Form,
    parent: Option<&gtk::Window>,
    submit: impl Fn(&str, &str, &str) -> Result<(), String> + 'static,
    done: impl FnOnce() + 'static,
    cancelled: impl FnOnce() + 'static,
) -> Pin {
    let window = gtk::Window::builder()
        .application(app)
        .title(form.title())
        .modal(parent.is_some())
        .default_width(320)
        .resizable(false)
        .build();
    window.set_transient_for(parent);
    let current = form.asks_current().then(pin_entry);
    let new = form.asks_new().then(pin_entry);
    let repeat = form.asks_new().then(pin_entry);
    let body = gtk::Box::new(gtk::Orientation::Vertical, 6);
    body.set_margin_top(12);
    body.set_margin_bottom(12);
    body.set_margin_start(12);
    body.set_margin_end(12);
    for (text, entry) in [(form.current_label(), &current), (tr!(PinNew), &new), (tr!(PinRepeat), &repeat)] {
        let Some(entry) = entry else { continue };
        let label = gtk::Label::new(Some(text));
        label.set_xalign(0.0);
        label.set_wrap(true);
        // (the box is "labelled by" it for screen readers)
        label.set_mnemonic_widget(Some(entry));
        body.append(&label);
        body.append(entry);
    }
    // "Wrong PIN." and the others, under the boxes
    let error = gtk::Label::new(None);
    error.set_xalign(0.0);
    error.set_wrap(true);
    error.set_max_width_chars(40);
    error.add_css_class("error");
    error.set_visible(false);
    body.append(&error);
    let ok = gtk::Button::with_mnemonic(&gtk_mnemonic(tr!(BtnOk)));
    ok.add_css_class("suggested-action");
    let cancel = gtk::Button::with_mnemonic(&gtk_mnemonic(tr!(BtnCancel)));
    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    buttons.set_halign(gtk::Align::End);
    buttons.set_margin_top(6);
    buttons.append(&cancel);
    buttons.append(&ok);
    body.append(&buttons);
    window.set_child(Some(&body));
    window.set_default_widget(Some(&ok));
    // the keyboard starts in the first box
    GtkWindowExt::set_focus(&window, current.as_ref().or(new.as_ref()));

    // set once OK or Cancel has decided: the close that follows is not a cancel
    let finished = Rc::new(Cell::new(false));
    let w = window.downgrade();
    cancel.connect_clicked(move |_| {
        if let Some(w) = w.upgrade() {
            w.close();
        }
    });
    let keys = gtk::EventControllerKey::new();
    let (w, c) = (window.downgrade(), cancel.downgrade());
    keys.connect_key_pressed(move |_, key, _, _| {
        if key != gtk::gdk::Key::Escape {
            return glib::Propagation::Proceed;
        }
        if let (Some(w), Some(c)) = (w.upgrade(), c.upgrade()) {
            if c.is_visible() {
                w.close();
            }
        }
        glib::Propagation::Stop
    });
    window.add_controller(keys);
    let cancelled = RefCell::new(Some(cancelled));
    let f = finished.clone();
    window.connect_close_request(move |_| {
        if !f.replace(true) {
            if let Some(cancelled) = cancelled.take() {
                cancelled();
            }
        }
        glib::Propagation::Proceed
    });
    let done = RefCell::new(Some(done));
    let weak = |e: &Option<gtk::Entry>| e.as_ref().map(|e| e.downgrade());
    let (cur, nw, rep) = (weak(&current), weak(&new), weak(&repeat));
    let (w, err) = (window.downgrade(), error.downgrade());
    ok.connect_clicked(move |_| {
        let entry = |e: &Option<glib::WeakRef<gtk::Entry>>| e.as_ref().and_then(|e| e.upgrade());
        let (cur, nw, rep) = (entry(&cur), entry(&nw), entry(&rep));
        let text = |e: &Option<gtk::Entry>| e.as_ref().map(|e| e.text().to_string()).unwrap_or_default();
        match submit(&text(&cur), &text(&nw), &text(&rep)) {
            Ok(()) => {
                finished.set(true);
                if let Some(w) = w.upgrade() {
                    w.close();
                }
                if let Some(done) = done.take() {
                    done();
                }
            }
            Err(message) => {
                if let Some(err) = err.upgrade() {
                    err.set_text(&message);
                    err.set_visible(true);
                }
                // the boxes the message is about are emptied, and the first gets the keyboard
                let about_new = message == tr!(PinBadLength) || message == tr!(PinMismatch);
                let wrong = if about_new { [nw, rep] } else { [cur, None] };
                for e in wrong.iter().flatten() {
                    e.set_text("");
                }
                if let Some(e) = wrong.iter().flatten().next() {
                    e.grab_focus();
                }
            }
        }
    });
    Pin { window, cancel }
}

/// A box for a PIN: dots, a digit keyboard where the input method has one, at most six
/// digits and nothing else.
fn pin_entry() -> gtk::Entry {
    let entry = gtk::Entry::builder()
        .visibility(false)
        .input_purpose(gtk::InputPurpose::Pin)
        .max_length(MAX_DIGITS as i32)
        .width_chars(MAX_DIGITS as i32 + 2)
        .activates_default(true)
        .build();
    // typed and pasted text keeps only its digits. Typing inserts into the entry's delegate
    // (its GtkText), and GTK does not pass that signal on to the entry, so the filter sits there
    if let Some(text) = entry.delegate() {
        text.connect_insert_text(|text, new, position| {
            if new.bytes().all(|b| b.is_ascii_digit()) {
                return;
            }
            text.stop_signal_emission_by_name("insert-text");
            let digits: String = new.chars().filter(char::is_ascii_digit).collect();
            if !digits.is_empty() {
                // (this emission is all digits, so it goes through)
                text.insert_text(&digits, position);
            }
        });
    }
    entry
}

/// A window the lock hid.
pub struct Hidden {
    window: glib::WeakRef<gtk::Window>,
    /// It was minimized: it comes back minimized.
    minimized: bool,
}

/// Hides every window on screen but those `keep` names, and returns them in the order to show
/// them again: as they were created (the main window first, a dialog after the window it
/// belongs to), with the active one last, so that it ends on top with the keyboard.
pub fn hide_windows(keep: impl Fn(&gtk::Window) -> bool) -> Vec<Hidden> {
    let mut windows: Vec<gtk::Window> =
        toplevels().into_iter().filter(|w| w.is_visible() && !keep(w)).collect();
    if let Some(i) = windows.iter().position(|w| w.is_active()) {
        let active = windows.remove(i);
        windows.push(active);
    }
    windows
        .into_iter()
        .map(|w| {
            let minimized = minimized(&w);
            w.set_visible(false);
            Hidden { window: w.downgrade(), minimized }
        })
        .collect()
}

/// Whether `w` is minimized, as far as the desktop has said so.
pub fn minimized(w: &gtk::Window) -> bool {
    w.surface()
        .and_downcast::<gtk::gdk::Toplevel>()
        .is_some_and(|t| t.state().contains(gtk::gdk::ToplevelState::MINIMIZED))
}

/// Shows again the windows [`hide_windows`] hid that are still there; the last one that was not
/// minimized comes to the front. False when none came back on screen (none had been hidden,
/// or each was minimized).
pub fn show_windows(hidden: Vec<Hidden>) -> bool {
    // a window destroyed meanwhile has left GTK's list, even while something still holds it
    let alive = toplevels();
    let mut front = None;
    for h in hidden {
        let Some(w) = h.window.upgrade().filter(|w| alive.contains(w)) else { continue };
        w.set_visible(true);
        if h.minimized {
            w.minimize();
        } else {
            front = Some(w);
        }
    }
    if let Some(w) = &front {
        w.present();
    }
    front.is_some()
}

/// The application's windows, as they were created.
fn toplevels() -> Vec<gtk::Window> {
    let list = gtk::Window::toplevels();
    (0..list.n_items()).filter_map(|i| list.item(i).and_downcast::<gtk::Window>()).collect()
}

/// At most this many message boxes wait for the unlock (the newest are kept).
pub const MAX_MESSAGES: usize = 20;

/// What waits for the unlock, in the order it came: message boxes (at most [`MAX_MESSAGES`],
/// the newest kept) and results asked for before the lock (none dropped).
pub struct Held<T> {
    /// (a message box, the item)
    items: Vec<(bool, T)>,
}

impl<T> Default for Held<T> {
    fn default() -> Self {
        Held { items: Vec::new() }
    }
}

impl<T> Held<T> {
    /// A result asked for before the lock.
    pub fn push(&mut self, item: T) {
        self.items.push((false, item));
    }

    /// A message box; the oldest one waiting goes when there are too many.
    pub fn push_message(&mut self, item: T) {
        if self.items.iter().filter(|(message, _)| *message).count() >= MAX_MESSAGES {
            if let Some(i) = self.items.iter().position(|(message, _)| *message) {
                self.items.remove(i);
            }
        }
        self.items.push((true, item));
    }

    /// Everything, in order, leaving nothing.
    pub fn take(&mut self) -> Vec<T> {
        std::mem::take(&mut self.items).into_iter().map(|(_, item)| item).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn held_message_boxes_keep_the_newest_and_results_stay() {
        let mut h = Held::default();
        h.push(-1);
        for i in 0..25 {
            h.push_message(i);
        }
        h.push(-2);
        let got = h.take();
        assert_eq!(got.len(), MAX_MESSAGES + 2);
        assert_eq!((got[0], got[MAX_MESSAGES + 1]), (-1, -2));
        let newest: Vec<i32> = (25 - MAX_MESSAGES as i32..25).collect();
        assert_eq!(got[1..=MAX_MESSAGES], newest[..]);
        assert!(h.take().is_empty());
        // fewer than the limit: all of them, in order
        h.push_message(7);
        h.push(8);
        h.push_message(9);
        assert_eq!(h.take(), [7, 8, 9]);
    }
}
