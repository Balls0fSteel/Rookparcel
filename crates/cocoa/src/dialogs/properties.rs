//! Torrent Properties (µTorrent 2.2.1's General tab, as the GTK and Win32 dialogs): Trackers,
//! Bandwidth Settings, Seeding Goal and Other Settings for the selected torrents, as a sheet on
//! the main window.
//!
//! The fields come from `frontend::properties` through `Core::properties`, which waits up to 2 s
//! for the tracker lists. With several torrents the check boxes have a mixed state and fields
//! whose values differ start blank; what the user leaves that way stays as it is on every
//! torrent. OK validates (`Properties::changes`: a wrong field gets a message and the focus) and
//! hands the changes to `Core::apply_properties`.

use std::cell::RefCell;
use std::ptr::NonNull;
use std::rc::Rc;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{MainThreadMarker, Message};
use objc2_app_kit::{
    NSAppearance, NSButton, NSColor, NSControlStateValueMixed, NSControlStateValueOff, NSControlStateValueOn,
    NSFont, NSScrollView, NSTextField, NSTextView, NSView, NSWindow,
};
use rookparcel_core::TorrentId;
use rookparcel_frontend::properties::{Field, Fields, Properties, Tri};
use rookparcel_frontend::tr;

use super::{NOTHING, cmd};
use crate::app::{apply, with_core};
use crate::ui::{self, layout, ns};

const OK: u8 = 1;
const CANCEL: u8 = 2;
/// An override check box changed: the fields it governs follow.
const SYNC: u8 = 3;

/// Torrent > Properties for the torrents `ids` (the list's selection).
pub fn open(ids: &[u64]) {
    let Some(mtm) = MainThreadMarker::new() else { return };
    let ids: Vec<TorrentId> = ids.iter().map(|&i| TorrentId::from_raw(i)).collect();
    let Some(Some((names, props))) = with_core(|c| c.properties(&ids)) else { return };
    // the alerts popped while it waited for the tracker lists
    crate::mainq::on_main(crate::app::on_alerts);
    let Some(parent) = crate::app::ui().map(|u| u.window.clone()) else { return };
    show(&parent, ids, &title(&names), props, mtm);
}

/// "Properties - name" for one torrent, the number of torrents for several.
pub fn title(names: &[String]) -> String {
    match names {
        [one] => tr!(PropsTitle, name = one),
        _ => tr!(PropsTitleMany, count = names.len()),
    }
}

struct Dialog {
    id: u16,
    ids: Vec<TorrentId>,
    props: RefCell<Properties>,
    parent: Retained<NSWindow>,
    sheet: Retained<NSWindow>,
    trackers: Retained<NSTextView>,
    upload: Retained<NSTextField>,
    download: Retained<NSTextField>,
    override_slots: Retained<NSButton>,
    slots: Retained<NSTextField>,
    override_goal: Retained<NSButton>,
    ratio: Retained<NSTextField>,
    minutes: Retained<NSTextField>,
    initial_seeding: Retained<NSButton>,
    dht: Retained<NSButton>,
    pex: Retained<NSButton>,
}

/// A multi-line plain text box in a scroll view: no rich text, nothing substituted or detected
/// in what is typed or pasted, the text fields' font, and Tab / Shift-Tab move the focus on as
/// in a field (`RPTarget`'s `textView:doCommandBySelector:`). Preferences' tracker list and
/// Create Torrent's boxes are ones too.
pub(crate) fn text_box(
    text: &str,
    height: f64,
    mtm: MainThreadMarker,
) -> (Retained<NSScrollView>, Retained<NSTextView>) {
    let scroll = NSTextView::scrollableTextView(mtm);
    ui::auto(&scroll);
    layout::height(&scroll, height);
    scroll.setBorderType(objc2_app_kit::NSBorderType::BezelBorder);
    let view = scroll
        .documentView()
        .and_then(|v| v.downcast::<NSTextView>().ok())
        .expect("a scrollable text view holds a text view");
    view.setRichText(false);
    view.setAllowsUndo(true);
    view.setAutomaticQuoteSubstitutionEnabled(false);
    view.setAutomaticDashSubstitutionEnabled(false);
    view.setAutomaticTextReplacementEnabled(false);
    view.setAutomaticLinkDetectionEnabled(false);
    view.setAutomaticSpellingCorrectionEnabled(false);
    view.setString(&ns(text));
    // a text view's own font (Helvetica 12) is smaller than the fields' (the system font, 13)
    view.setFont(Some(&NSFont::systemFontOfSize(NSFont::systemFontSize())));
    view.setDelegate(Some(ProtocolObject::from_ref(&*crate::ui::target(mtm))));
    (scroll, view)
}

/// Turns a [`text_box`] on or off. A text view is no control: off, it is read-only and looks as
/// a disabled text field does (grey text on a greyed background).
pub(crate) fn enable_text_box(scroll: &NSScrollView, view: &NSTextView, enabled: bool) {
    view.setEditable(enabled);
    view.setSelectable(enabled);
    let (text, background) = if enabled {
        (NSColor::textColor(), NSColor::textBackgroundColor())
    } else {
        (NSColor::disabledControlTextColor(), disabled_background())
    };
    view.setTextColor(Some(&text));
    view.setBackgroundColor(&background);
    scroll.setBackgroundColor(&background);
}

/// A disabled text field's background, which AppKit draws as its enabled fill at half strength
/// over the window: white at 50 % in light mode (246 -> 251 on macOS 15), white at 2.5 % in dark
/// mode (40 -> 45), a shade lighter than the window, as the field beside the box shows.
fn disabled_background() -> Retained<NSColor> {
    let provider = RcBlock::new(|appearance: NonNull<NSAppearance>| {
        // SAFETY: AppKit hands the provider a valid appearance for the call.
        let dark = crate::images::dark_of(unsafe { appearance.as_ref() });
        let color = NSColor::colorWithWhite_alpha(1.0, if dark { 0.025 } else { 0.5 });
        // the provider returns an autoreleased color (AppKit resolves it inside a pool)
        NonNull::new(Retained::autorelease_ptr(color)).expect("a color")
    });
    // SAFETY: the provider returns a valid color for every appearance.
    unsafe { NSColor::colorWithName_dynamicProvider(None, &provider) }
}

/// A number field.
fn number(text: &str, mtm: MainThreadMarker) -> Retained<NSTextField> {
    let f = ui::field(text, "", mtm);
    layout::width(&f, 90.0);
    f
}

fn set_tri(b: &NSButton, v: Tri) {
    b.setState(match v {
        Tri::On => NSControlStateValueOn,
        Tri::Off => NSControlStateValueOff,
        Tri::Mixed => NSControlStateValueMixed,
    });
}

fn tri(b: &NSButton) -> Tri {
    match b.state() {
        s if s == NSControlStateValueOn => Tri::On,
        s if s == NSControlStateValueMixed => Tri::Mixed,
        _ => Tri::Off,
    }
}

fn show(parent: &NSWindow, ids: Vec<TorrentId>, title: &str, props: Properties, mtm: MainThreadMarker) {
    let id = super::new_id();
    let multi = props.count > 1;
    let f = props.fields.clone();
    let check = |s: &str, v: Tri, act: u8| {
        let b = ui::check(&ui::text(s), false, cmd(id, act), mtm);
        // several torrents: a third state, "as each one is"
        b.setAllowsMixedState(multi);
        set_tri(&b, v);
        b
    };
    let (tracker_scroll, trackers) = text_box(&f.trackers, 110.0, mtm);
    layout::min_width(&tracker_scroll, 460.0);
    let upload = number(&f.upload_kib, mtm);
    let download = number(&f.download_kib, mtm);
    let slots = number(&f.upload_slots, mtm);
    let override_slots = check(tr!(PropsOverrideSlots), f.override_slots, SYNC);
    let override_goal = check(tr!(PropsOverrideGoal), f.override_goal, SYNC);
    let ratio = number(&f.goal_ratio, mtm);
    let minutes = number(&f.goal_minutes, mtm);
    let initial_seeding = check(tr!(PropsInitialSeeding), f.initial_seeding, NOTHING);
    let dht = check(tr!(PropsDht), f.dht, NOTHING);
    let pex = check(tr!(PropsPex), f.pex, NOTHING);
    initial_seeding.setEnabled(props.can_initial_seed);
    dht.setEnabled(props.can_dht_pex);
    pex.setEnabled(props.can_dht_pex);

    let caption = |s: &str| ui::label_tr(s, mtm);
    let (c_up, c_down, c_slots, c_ratio, c_minutes) = (
        caption(tr!(PropsUpRate)),
        caption(tr!(PropsDownRate)),
        caption(tr!(PropsSlots)),
        caption(tr!(PropsRatio)),
        caption(tr!(PropsSeedTime)),
    );
    let bandwidth = layout::grid(
        &[
            vec![Some(&*c_up), Some(&*upload)],
            vec![Some(&*c_down), Some(&*download)],
            vec![Some(&*c_slots), Some(&*slots)],
            vec![Some(&*override_slots), None],
        ],
        mtm,
    );
    let goal = layout::grid(
        &[
            vec![Some(&*override_goal), None],
            vec![Some(&*c_ratio), Some(&*ratio)],
            vec![Some(&*c_minutes), Some(&*minutes)],
        ],
        mtm,
    );
    let other = layout::grid(&[vec![Some(&*initial_seeding), Some(&*pex)], vec![Some(&*dht), None]], mtm);
    other.setColumnSpacing(24.0);
    let ok = ui::button_tr(tr!(BtnOk), cmd(id, OK), mtm);
    let cancel = ui::button_tr(tr!(BtnCancel), cmd(id, CANCEL), mtm);
    let buttons = super::buttons(&[], &cancel, &ok, mtm);
    let parts: [(&str, &NSView); 4] = [
        (tr!(PropsTrackers), &tracker_scroll),
        (tr!(PropsBandwidth), &bandwidth),
        (tr!(PropsSeedingGoal), &goal),
        (tr!(PropsOther), &other),
    ];
    let sections: Vec<Retained<objc2_app_kit::NSStackView>> =
        parts.iter().map(|(t, v)| super::section(t, v, mtm)).collect();
    let mut views: Vec<&NSView> = sections.iter().map(|s| &***s).collect();
    views.push(&buttons);
    let root = super::column(&views, mtm);
    root.setSpacing(14.0);

    let sheet = super::window(title, &root, None, false, mtm);
    // the tracker box first, with a key view loop: Tab leaves the box for the number fields
    super::focus_first(&sheet, &trackers);
    let d = Rc::new(Dialog {
        id,
        ids,
        props: RefCell::new(props),
        parent: parent.retain(),
        sheet: sheet.clone(),
        trackers,
        upload,
        download,
        override_slots,
        slots,
        override_goal,
        ratio,
        minutes,
        initial_seeding,
        dht,
        pex,
    });
    d.sync();
    let handler = d.clone();
    super::set_handler(id, move |act| handler.act(act));
    super::test_accept(id, OK);
    parent.beginSheet_completionHandler(&sheet, None);
}

impl Dialog {
    fn act(&self, act: u8) {
        let Some(mtm) = MainThreadMarker::new() else { return };
        match act {
            OK => self.ok(mtm),
            CANCEL => self.end(),
            SYNC => self.sync(),
            _ => {}
        }
    }

    fn end(&self) {
        self.parent.endSheet(&self.sheet);
        self.sheet.orderOut(None);
        super::retire(self.id);
    }

    fn ok(&self, mtm: MainThreadMarker) {
        self.props.borrow_mut().fields = self.read();
        let result = self.props.borrow().changes();
        match result {
            Ok(changes) => {
                self.end();
                if !changes.is_empty() {
                    let ids = self.ids.clone();
                    apply(with_core(|c| {
                        c.apply_properties(&ids, &changes);
                        c.events()
                    }));
                }
            }
            Err(e) => {
                super::tell(Some(&self.sheet), ui::Style::Warning, &e.message, "", mtm);
                self.focus(e.field);
            }
        }
    }

    fn read(&self) -> Fields {
        Fields {
            trackers: self.trackers.string().to_string(),
            upload_kib: ui::value(&self.upload),
            download_kib: ui::value(&self.download),
            override_slots: tri(&self.override_slots),
            upload_slots: ui::value(&self.slots),
            override_goal: tri(&self.override_goal),
            goal_ratio: ui::value(&self.ratio),
            goal_minutes: ui::value(&self.minutes),
            initial_seeding: tri(&self.initial_seeding),
            dht: tri(&self.dht),
            pex: tri(&self.pex),
        }
    }

    /// The slot count and the goal's values are edited only while their override is not off.
    fn sync(&self) {
        self.slots.setEnabled(tri(&self.override_slots) != Tri::Off);
        let goal = tri(&self.override_goal) != Tri::Off;
        self.ratio.setEnabled(goal);
        self.minutes.setEnabled(goal);
    }

    /// Puts the cursor in the field a validation error is about, its text selected.
    fn focus(&self, field: Field) {
        let f = match field {
            Field::Trackers => {
                self.sheet.makeFirstResponder(Some(&*self.trackers));
                return;
            }
            Field::UploadRate => &self.upload,
            Field::DownloadRate => &self.download,
            Field::UploadSlots => &self.slots,
            Field::GoalRatio => &self.ratio,
            Field::GoalMinutes => &self.minutes,
        };
        self.sheet.makeFirstResponder(Some(&**f));
        // SAFETY: no sender.
        unsafe { f.selectText(None) };
    }
}
