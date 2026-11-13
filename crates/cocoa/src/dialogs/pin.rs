//! The PIN dialogs of the windows' lock (D69, `crate::lock`): "Rookparcel is locked" (the unlock
//! prompt), Set PIN, Change PIN and Remove PIN. Each is an application-modal alert (`runModal`:
//! the windows may all be hidden) with OK and Cancel and the secure boxes its
//! `ui_lock::Form` asks for: the current PIN, and the new one twice. The boxes keep to six
//! digits as they are typed in or pasted into (`controlTextDidChange:` through `RPTarget`).
//! OK hands what was typed to `ui_lock::submit` (`lock::submit`); its message (a wrong PIN, the
//! wait after five, a PIN too short or not repeated) runs the alert again with the message
//! under the title.

use std::cell::{Cell, RefCell};

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_app_kit::{
    NSAlertFirstButtonReturn, NSApplication, NSEvent, NSEventModifierFlags, NSEventType,
    NSModalResponseAbort, NSSecureTextField, NSView, NSWindow,
};
use objc2_foundation::{NSPoint, NSRect};
use rookparcel_frontend::tr;
use rookparcel_frontend::ui_lock::Form;

use crate::ui::{self, layout, ns};

/// The boxes' least width, in points.
const BOX_WIDTH: f64 = 220.0;

/// A box's text changed: every box keeps its digits.
const FILTER: u8 = 1;

thread_local! {
    /// The alert's window while one runs.
    static OPEN: RefCell<Option<Retained<NSWindow>>> = const { RefCell::new(None) };
    /// [`abort`] ended it: it is not run again.
    static ABORTED: Cell<bool> = const { Cell::new(false) };
    /// What [`abort`] was given to do once the modal loop has ended.
    static THEN: Cell<Option<fn()>> = const { Cell::new(None) };
}

/// Whether a PIN dialog is up.
pub fn is_open() -> bool {
    OPEN.with(|o| o.borrow().is_some())
}

/// Brings the dialog that is up to the front (a second request for the PIN); false when none is.
pub fn bring_forward(mtm: MainThreadMarker) -> bool {
    let Some(w) = OPEN.with(|o| o.borrow().clone()) else { return false };
    crate::mac::lifecycle::activate(mtm);
    w.makeKeyAndOrderFront(None);
    true
}

/// Ends the dialog that is up as if cancelled (Quit from Auto Shutdown, a logout, a reset that
/// came meanwhile), then runs `then` once its modal loop has ended (AppKit ends no application
/// while one runs). False when none is up.
pub fn abort(mtm: MainThreadMarker, then: Option<fn()>) -> bool {
    if !is_open() {
        return false;
    }
    ABORTED.with(|a| a.set(true));
    if then.is_some() {
        THEN.with(|t| t.set(then));
    }
    let app = NSApplication::sharedApplication(mtm);
    app.stopModalWithCode(NSModalResponseAbort);
    // the modal loop sees the stop once it has handled an event: from a timer or a notification
    // block, one that does nothing (the button's handling would have been that event)
    let wake =
        NSEvent::otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2(
            NSEventType::ApplicationDefined,
            NSPoint::new(0.0, 0.0),
            NSEventModifierFlags::empty(),
            0.0,
            0,
            None,
            0,
            0,
            0,
        );
    if let Some(e) = wake {
        app.postEvent_atStart(&e, true);
    }
    true
}

/// Runs `form`'s dialog until its OK goes through (true; the settings are saved) or it is
/// cancelled (false). One at a time: while one is up, this brings it forward.
pub fn run(form: Form, mtm: MainThreadMarker) -> bool {
    if bring_forward(mtm) {
        return false;
    }
    ABORTED.with(|a| a.set(false));
    let mut message = String::new();
    loop {
        let Some([current, new, repeat]) = ask(form, &message, mtm) else { return false };
        match crate::lock::submit(form, &current, &new, &repeat) {
            Some(Ok(())) => return true,
            Some(Err(m)) => message = m,
            None => return false,
        }
    }
}

/// A secure box of the dialog `id`, keeping to digits as it is typed in (`FILTER`).
fn pin_box(id: u16, mtm: MainThreadMarker) -> Retained<NSSecureTextField> {
    let b = ui::secure_field("", mtm);
    // no action, so Return stays the OK button's; its text's changes come to `RPTarget`, which
    // runs the tag's command
    b.setTag(super::cmd(id, FILTER).tag());
    // SAFETY: the shared `RPTarget` lives as long as the main thread; the box does not retain
    // its delegate.
    unsafe { b.setDelegate(Some(ProtocolObject::from_ref(&*ui::target(mtm)))) };
    b
}

/// Keeps a box to `lock::pin_digits` of what is in it: in its field editor while it is being
/// typed in (the box keeps the keyboard), else in the box.
fn keep_digits(b: &NSSecureTextField) {
    let typed = ui::value(b);
    let digits = crate::lock::pin_digits(&typed);
    if digits == typed {
        return;
    }
    match b.currentEditor() {
        Some(editor) => editor.setString(&ns(&digits)),
        None => b.setStringValue(&ns(&digits)),
    }
}

/// Shows the dialog once; what was typed in the current, new and repeated PIN boxes (empty for
/// one it does not have), or `None` when cancelled or aborted.
fn ask(form: Form, message: &str, mtm: MainThreadMarker) -> Option<[String; 3]> {
    let alert =
        ui::make_alert(ui::Style::Info, &ui::text(form.title()), message, &[tr!(BtnOk), tr!(BtnCancel)], mtm);
    let id = super::new_id();
    let mut labels: Vec<&str> = Vec::new();
    if form.asks_current() {
        labels.push(form.current_label());
    }
    if form.asks_new() {
        labels.extend([tr!(PinNew), tr!(PinRepeat)]);
    }
    let boxes: Vec<Retained<NSSecureTextField>> = labels.iter().map(|_| pin_box(id, mtm)).collect();
    let label_views: Vec<_> = labels.iter().map(|l| ui::label(&ui::text(l), mtm)).collect();
    let mut views: Vec<&NSView> = Vec::new();
    for (l, b) in label_views.iter().zip(&boxes) {
        views.push(l);
        views.push(b);
    }
    // an alert's accessory view is laid out by its frame: a plain view as large as the stack
    // (Auto Layout inside) needs
    let stack = super::column(&views, mtm);
    stack.setSpacing(6.0);
    layout::min_width(&stack, BOX_WIDTH);
    let host = NSView::new(mtm);
    host.addSubview(&stack);
    layout::pin(&stack, &host, 0.0);
    let size = host.fittingSize();
    host.setFrame(NSRect::new(NSPoint::new(0.0, 0.0), size));
    alert.setAccessoryView(Some(&host));
    // Escape is Cancel's in every language (AppKit gives it only to a button titled "Cancel")
    if let Some(cancel) = alert.buttons().iter().nth(1) {
        cancel.setKeyEquivalent(&ns("\u{1b}"));
    }
    let filtered = boxes.clone();
    super::set_handler(id, move |act| {
        if act == FILTER {
            filtered.iter().for_each(|b| keep_digits(b));
        }
    });
    alert.layout();
    let window = alert.window();
    if let Some(first) = boxes.first() {
        // Tab goes through the boxes
        super::focus_first(&window, first);
    }
    OPEN.with(|o| *o.borrow_mut() = Some(window));
    crate::mac::lifecycle::activate(mtm);
    let r = alert.runModal();
    OPEN.with(|o| o.borrow_mut().take());
    super::retire(id);
    if let Some(then) = THEN.with(Cell::take) {
        crate::mainq::on_main(then);
    }
    if r != NSAlertFirstButtonReturn || ABORTED.with(Cell::get) {
        return None;
    }
    let mut typed = boxes.iter().map(|b| ui::value(b));
    let current = if form.asks_current() { typed.next().unwrap_or_default() } else { String::new() };
    let new = typed.next().unwrap_or_default();
    let repeat = typed.next().unwrap_or_default();
    Some([current, new, repeat])
}
