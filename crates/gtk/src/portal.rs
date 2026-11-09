//! XDG desktop portals, over GDBus as the tray does (no library beyond GIO). What a Flatpak
//! needs from them: autostart through the Background portal (`integration`, Preferences >
//! General's "Start Rookparcel on system startup"); and what every desktop offers through them:
//! the user's light-or-dark preference from the Settings portal ([`color_scheme`], which the
//! icons and the drawn widgets follow).
//!
//! A portal request answers twice: the method call returns the path of an
//! `org.freedesktop.portal.Request` object at once, and the outcome arrives later on its
//! `Response` signal, after the desktop has perhaps asked the user. The path is
//! `/org/freedesktop/portal/desktop/request/<sender>/<handle_token>` (the caller's unique bus
//! name without the `:` and with `.` as `_`), so the signal is subscribed to before the call,
//! as the portal documentation says, or a fast answer would be missed; an older portal that
//! returns another path gets a second subscription there. Portals need no `--talk-name` in the
//! manifest: `org.freedesktop.portal.*` is always reachable from a sandbox.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{gio, glib};

const BUS: &str = "org.freedesktop.portal.Desktop";
const PATH: &str = "/org/freedesktop/portal/desktop";
const SETTINGS: &str = "org.freedesktop.portal.Settings";
/// The Settings portal's namespace and key of the light-or-dark preference.
const APPEARANCE: &str = "org.freedesktop.appearance";
const COLOR_SCHEME: &str = "color-scheme";
/// How long a Settings read may take: a missing portal must not hold up the start.
const SETTINGS_TIMEOUT_MS: i32 = 300;

thread_local! {
    /// Makes each request's `handle_token` unique within this connection.
    static NEXT_TOKEN: Cell<u32> = const { Cell::new(1) };
}

/// The `Request` object a call with `token` gets, from the caller's unique bus name.
fn request_path(unique_name: &str, token: &str) -> String {
    let sender = unique_name.trim_start_matches(':').replace('.', "_");
    format!("{PATH}/request/{sender}/{token}")
}

/// What a `Response` signal's `(ua{sv})` says: the results, or why there are none.
fn response(parameters: &glib::Variant) -> Result<glib::VariantDict, String> {
    let (code, results) = match parameters.get::<(u32, glib::VariantDict)>() {
        Some(r) => r,
        None => return Err(format!("unexpected portal response {parameters}")),
    };
    match code {
        0 => Ok(results),
        1 => Err("the request was cancelled".into()),
        _ => Err("the desktop portal refused the request".into()),
    }
}

/// What a request's caller gets: the `Response`'s results, or why there are none.
type Done = Box<dyn FnOnce(Result<glib::VariantDict, String>)>;

/// One request in flight: its answer goes to `done` once, then the subscriptions go.
struct Pending {
    done: RefCell<Option<Done>>,
    subscriptions: RefCell<Vec<gio::SignalSubscription>>,
}

impl Pending {
    fn finish(self: &Rc<Self>, result: Result<glib::VariantDict, String>) {
        let Some(done) = self.done.borrow_mut().take() else { return };
        // unsubscribed after this signal's handler has returned
        let me = self.clone();
        glib::idle_add_local_once(move || drop(me.subscriptions.take()));
        done(result);
    }

    fn subscribe(self: &Rc<Self>, bus: &gio::DBusConnection, path: &str) {
        let weak = Rc::downgrade(self);
        let s = bus.subscribe_to_signal(
            Some(BUS),
            Some("org.freedesktop.portal.Request"),
            Some("Response"),
            Some(path),
            None,
            gio::DBusSignalFlags::NONE,
            move |signal| {
                if let Some(me) = weak.upgrade() {
                    me.finish(response(signal.parameters));
                }
            },
        );
        self.subscriptions.borrow_mut().push(s);
    }
}

/// Calls `method` of portal `interface` with `args` before its `options` (`a{sv}`, which gets
/// the `handle_token`); `done` gets the `Response`'s results on the main loop.
fn request(
    interface: &str,
    method: &str,
    args: Vec<glib::Variant>,
    mut options: HashMap<String, glib::Variant>,
    done: impl FnOnce(Result<glib::VariantDict, String>) + 'static,
) {
    let bus = match gio::bus_get_sync(gio::BusType::Session, None::<&gio::Cancellable>) {
        Ok(b) => b,
        Err(e) => return done(Err(e.to_string())),
    };
    let Some(unique) = bus.unique_name() else {
        return done(Err("not connected to the session bus".into()));
    };
    let token = format!("rookparcel{}", NEXT_TOKEN.with(|n| n.replace(n.get().wrapping_add(1))));
    let expected = request_path(&unique, &token);
    let pending =
        Rc::new(Pending { done: RefCell::new(Some(Box::new(done))), subscriptions: RefCell::default() });
    pending.subscribe(&bus, &expected);
    options.insert("handle_token".into(), token.to_variant());
    let params = glib::Variant::tuple_from_iter(args.into_iter().chain([options.to_variant()]));
    let reply = glib::VariantTy::new("(o)").expect("a valid type");
    let b = bus.clone();
    bus.call(
        Some(BUS),
        PATH,
        interface,
        method,
        Some(&params),
        Some(reply),
        gio::DBusCallFlags::NONE,
        -1,
        None::<&gio::Cancellable>,
        move |r| match r {
            Ok(v) => {
                let path = v.child_value(0).str().map(str::to_owned).unwrap_or_default();
                if !path.is_empty() && path != expected {
                    pending.subscribe(&b, &path);
                }
            }
            Err(e) => pending.finish(Err(e.to_string())),
        },
    );
}

/// Asks the Background portal to start Rookparcel at login with `commandline` (the sandbox's
/// command and its arguments), or to stop doing so; `done` gets whether it will now be
/// autostarted. The desktop may ask the user first (GNOME does once), and nothing can be read
/// back later: the caller keeps the answer.
pub fn request_autostart(
    autostart: bool,
    commandline: Vec<String>,
    reason: String,
    done: impl FnOnce(Result<bool, String>) + 'static,
) {
    let mut options = HashMap::new();
    options.insert("reason".to_owned(), reason.to_variant());
    options.insert("autostart".to_owned(), autostart.to_variant());
    options.insert("commandline".to_owned(), commandline.to_variant());
    // started as a program, not by D-Bus activation (the exported entry has no DBusActivatable)
    options.insert("dbus-activatable".to_owned(), false.to_variant());
    // the parent window's handle: none (a Wayland one needs xdg-foreign, which GTK 4 keeps
    // private to the GDK backend); the portal's dialog then has no parent
    request(
        "org.freedesktop.portal.Background",
        "RequestBackground",
        vec!["".to_variant()],
        options,
        move |r| {
            done(r.map(|results| {
                // missing only from a portal older than the spec: take the request as granted
                results.lookup::<bool>("autostart").ok().flatten().unwrap_or(autostart)
            }))
        },
    );
}

/// What a `color-scheme` value says: `Some(true)` prefers dark, `Some(false)` light, `None` no
/// preference (0) or something unexpected. The value arrives boxed: `ReadOne` replies `(v)` and
/// `SettingChanged` carries a `v`, an older portal's `Read` replies `(v)` around another `v`.
fn prefers_dark(value: &glib::Variant) -> Option<bool> {
    let mut v = value.clone();
    loop {
        if v.is_container() && v.type_().is_tuple() {
            if v.n_children() != 1 {
                return None;
            }
            v = v.child_value(0);
        } else if v.type_().is_variant() {
            v = v.as_variant()?;
        } else {
            break;
        }
    }
    match v.get::<u32>()? {
        1 => Some(true),
        2 => Some(false),
        _ => None,
    }
}

/// The desktop's light-or-dark preference from the Settings portal: `Some(true)` for dark,
/// `Some(false)` for light, `None` when there is no preference or no portal (within
/// [`SETTINGS_TIMEOUT_MS`]). `ReadOne` first (portal version 2), then the older `Read`.
pub fn color_scheme() -> Option<bool> {
    let bus = gio::bus_get_sync(gio::BusType::Session, None::<&gio::Cancellable>).ok()?;
    let args = (APPEARANCE, COLOR_SCHEME).to_variant();
    let reply = glib::VariantTy::new("(v)").expect("a valid type");
    ["ReadOne", "Read"].iter().find_map(|method| {
        let v = bus
            .call_sync(
                Some(BUS),
                PATH,
                SETTINGS,
                method,
                Some(&args),
                Some(reply),
                gio::DBusCallFlags::NONE,
                SETTINGS_TIMEOUT_MS,
                None::<&gio::Cancellable>,
            )
            .ok()?;
        prefers_dark(&v)
    })
}

/// Calls `changed` with the new preference whenever the Settings portal announces a change of
/// the `color-scheme` key (`SettingChanged(namespace, key, value)`). The subscription lives as
/// long as the returned value; `None` without a session bus.
pub fn watch_color_scheme(changed: impl Fn(Option<bool>) + 'static) -> Option<gio::SignalSubscription> {
    let bus = gio::bus_get_sync(gio::BusType::Session, None::<&gio::Cancellable>).ok()?;
    Some(bus.subscribe_to_signal(
        Some(BUS),
        Some(SETTINGS),
        Some("SettingChanged"),
        Some(PATH),
        Some(APPEARANCE),
        gio::DBusSignalFlags::NONE,
        move |signal| {
            let Some((namespace, key, value)) = signal.parameters.get::<(String, String, glib::Variant)>()
            else {
                return;
            };
            if namespace == APPEARANCE && key == COLOR_SCHEME {
                changed(prefers_dark(&value));
            }
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_colour_scheme_is_read_out_of_its_boxes() {
        // ReadOne's reply: (v) around the u32
        let read_one =
            |n: u32| glib::Variant::tuple_from_iter([glib::Variant::from_variant(&n.to_variant())]);
        assert_eq!(read_one(1).type_().as_str(), "(v)");
        assert_eq!(prefers_dark(&read_one(1)), Some(true));
        assert_eq!(prefers_dark(&read_one(2)), Some(false));
        assert_eq!(prefers_dark(&read_one(0)), None);
        assert_eq!(prefers_dark(&read_one(7)), None);
        // an older portal's Read: (v) around a v around the u32
        let read = glib::Variant::tuple_from_iter([glib::Variant::from_variant(
            &glib::Variant::from_variant(&1u32.to_variant()),
        )]);
        assert_eq!(prefers_dark(&read), Some(true));
        // SettingChanged's value: a bare v, or the u32 itself
        assert_eq!(prefers_dark(&glib::Variant::from_variant(&2u32.to_variant())), Some(false));
        assert_eq!(prefers_dark(&1u32.to_variant()), Some(true));
        // not a colour scheme at all
        assert_eq!(prefers_dark(&"dark".to_variant()), None);
        assert_eq!(prefers_dark(&(1u32, 2u32).to_variant()), None);
    }

    #[test]
    fn the_request_path_comes_from_the_unique_name() {
        assert_eq!(
            request_path(":1.42", "rookparcel1"),
            "/org/freedesktop/portal/desktop/request/1_42/rookparcel1"
        );
    }

    /// A `Response` as the portal sends it: `(ua{sv})`. (A tuple's `to_variant` would box the
    /// dictionary as `v`, which is not what arrives.)
    fn signal(code: u32, results: &glib::VariantDict) -> glib::Variant {
        glib::Variant::tuple_from_iter([code.to_variant(), results.end()])
    }

    #[test]
    fn responses_carry_results_only_on_success() {
        let results = glib::VariantDict::new(None);
        results.insert("autostart", true);
        let ok = signal(0, &results);
        assert_eq!(ok.type_().as_str(), "(ua{sv})");
        assert_eq!(response(&ok).unwrap().lookup::<bool>("autostart").unwrap(), Some(true));
        assert!(response(&signal(1, &glib::VariantDict::new(None))).is_err());
        assert!(response(&signal(2, &glib::VariantDict::new(None))).is_err());
        assert!(response(&"nonsense".to_variant()).is_err());
    }
}
