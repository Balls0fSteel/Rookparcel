//! Preferences (docs/WEBUI.md slice 6): the model behind `GET` and `POST /pa/prefs`, generated
//! from `frontend::options` and `frontend::settings` and laid out as µTorrent 2.2.1's pages,
//! the way the Win32 dialog (`crates/win32/src/preferences.rs`) lays them out.
//!
//! Three kinds of field share one table: every engine setting by its libtorrent name (the
//! `frontend::settings::TABLE` rows, the Advanced page), the frontend's options nox has an
//! equivalent for (`GuiOptions`: the folders, the per-torrent defaults, the IP filter) and the
//! Web UI's own preferences (`webui.*`). The page learns the fields and their placement from
//! `GET /pa/prefs`, so a new field needs no page code except a place on a page.
//!
//! A `POST` carries a partial object of ids to values. Everything is parsed and set on a
//! [`Working`] copy first (with `frontend::settings::Entry::parse` for the engine settings and
//! the Win32 dialog's checks for the rest), then committed at once: the engine settings go out
//! through `apply_settings`, a new password is hashed, a changed domain list or CSRF flag holds
//! for the next request, and everything is stored in `State::options` (the frontend's
//! preferences keys, saved in the state file) for [`restore`] at the next start. The compat
//! shim's `app/setPreferences` maps its names onto these ids and takes the same path.
//!
//! Hosted by a desktop client (`Config::hosted`, D40): the same parsing, checks and `store`
//! into `State::options`, which is the mirror of the host's preferences, and `Shared::save`
//! hands the journal to the host; nothing goes to the engine from here (no `apply_settings`,
//! IP filter, peer-exchange flags or language switch: the host applies its preferences
//! itself), and `State::settings` means the host's configured settings (`settings::load` of
//! the `lt.*` keys), which [`read_back`] refreshes when the host's journal comes in. The
//! alternative speed limits do not exist there ([`HOSTED_NO_ALT`]).

use std::io;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

use rookparcel_core::{IpFilter, IpFilterLoad, ProxyType, Settings, TorrentFlags};
use rookparcel_frontend::exclude;
use rookparcel_frontend::help;
use rookparcel_frontend::ip_filter_list;
use rookparcel_frontend::lang::{self, H, S};
use rookparcel_frontend::network;
use rookparcel_frontend::options::{self, GuiOptions};
use rookparcel_frontend::pref_pages;
use rookparcel_frontend::scheduler;
use rookparcel_frontend::settings::{self, Entry, Kind as SettingKind, TABLE, Value};
use rookparcel_frontend::tr;
use rookparcel_frontend::tracker_list;

use super::auth::{self, PasswordHash};
use super::json::Json;
use super::path_policy::PathPolicy;
use super::{Prefs, Shared, State, theme, unix_now};

/// A field's type, as the page sees it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Bool,
    Int {
        min: i64,
        max: i64,
    },
    Text,
    /// Several lines of text (the typed tracker list, D66): a `<textarea>` on the page.
    Lines,
    Folder,
    /// A choice among labels; the value is the index.
    Enum(&'static [S]),
    /// Write-only: the value read back is always empty, an empty value sent changes nothing.
    Password,
    /// The scheduler's week: 168 `frontend::scheduler::Mode` codes, Monday 00:00 first.
    Schedule,
    /// The user-interface language (`frontend::lang`): a code, empty for the system's; the
    /// model lists the choices as `[value, text]`.
    Language,
    /// "Use only this network"'s drop-down (D54, `frontend::network::rule_choices`): the
    /// engine host's adapters, listed as `[value, text]` like [`Kind::Language`].
    Network,
    /// The Web UI's theme (D63, `super::theme`): an id, listed as `[value, text]` like
    /// [`Kind::Language`], the built-ins and the third-party files.
    Theme,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Bool => "bool",
            Kind::Int { .. } => "int",
            Kind::Text => "text",
            Kind::Lines => "lines",
            Kind::Folder => "folder",
            Kind::Enum(_) => "enum",
            Kind::Password => "password",
            Kind::Schedule => "schedule",
            Kind::Language | Kind::Network | Kind::Theme => "choice",
        }
    }
}

/// The copy a `POST` edits before anything is committed. `settings` carries the *normal*
/// global rate limits (`Prefs::dl_limit` / `up_limit`), not the ones in force when the
/// alternative limits are on, so the `download_rate_limit` and `upload_rate_limit` rows edit
/// what the Bandwidth page shows.
#[derive(Clone)]
pub(crate) struct Working {
    pub settings: Settings,
    pub prefs: Prefs,
    pub username: String,
    pub new_password: Option<String>,
    /// A new API key (`webui.api_key`, write-only like the password).
    pub new_api_key: Option<String>,
    pub listen: String,
}

impl Working {
    pub fn from_state(st: &State) -> Working {
        let mut settings = st.settings.clone();
        settings.download_rate_limit = st.prefs.dl_limit;
        settings.upload_rate_limit = st.prefs.up_limit;
        Working {
            settings,
            prefs: st.prefs.clone(),
            username: st.username.clone(),
            new_password: None,
            new_api_key: None,
            listen: st.listen_addr.to_string(),
        }
    }

    /// The defaults every field is compared with.
    fn defaults(st: &State) -> Working {
        Working {
            settings: Settings::default(),
            prefs: Prefs::defaults(&st.default_save_path, &Settings::default()),
            username: "admin".to_owned(),
            new_password: None,
            new_api_key: None,
            listen: st.listen_addr.to_string(),
        }
    }
}

/// A field that is not an engine setting row.
pub(crate) struct Field {
    pub id: &'static str,
    /// `None`: the field shows with the control before it (a folder under its check box).
    pub label: Option<S>,
    pub kind: Kind,
    pub readonly: bool,
    get: fn(&Working) -> Value,
    set: fn(&mut Working, Value),
}

fn boolean(v: &Value) -> bool {
    matches!(v, Value::Bool(true)) || matches!(v, Value::Int(i) if *i != 0)
}

fn int(v: &Value) -> i64 {
    match v {
        Value::Int(i) => *i,
        Value::Bool(b) => *b as i64,
        Value::Str(s) => s.trim().parse().unwrap_or(0),
    }
}

fn string(v: Value) -> String {
    match v {
        Value::Str(s) => s,
        other => other.to_string(),
    }
}

/// The proxy "Type" combo: the GUIs' list (`frontend::pref_pages::PROXY_TYPES`).
const PROXY_TYPES: [S; 4] = pref_pages::PROXY_TYPES;
/// "Outgoing" encryption, in the combo's order.
const ENC_OUT: [S; 3] = [S::EncDisabled, S::EncEnabled, S::EncForced];

fn proxy_to_ui(s: &Settings) -> (i64, bool) {
    let (kind, auth) = pref_pages::proxy_to_ui(s);
    (kind as i64, auth)
}

fn proxy_from_ui(kind: i64, auth: bool) -> ProxyType {
    pref_pages::proxy_from_ui(usize::try_from(kind).unwrap_or(0), auth)
}

fn enc_out(s: &Settings) -> i64 {
    match s.out_enc_policy {
        rookparcel_core::EncPolicy::Disabled => 0,
        rookparcel_core::EncPolicy::Enabled => 1,
        rookparcel_core::EncPolicy::Forced => 2,
    }
}

fn enc_policy(i: i64) -> rookparcel_core::EncPolicy {
    match i {
        0 => rookparcel_core::EncPolicy::Disabled,
        2 => rookparcel_core::EncPolicy::Forced,
        _ => rookparcel_core::EncPolicy::Enabled,
    }
}

const RATE_MAX: i64 = u32::MAX as i64;
const YEAR: i64 = 86_400 * 365;

macro_rules! field {
    ($id:expr, $label:expr, $kind:expr, |$w:ident| $get:expr, |$m:ident, $v:ident| $set:expr) => {
        Field { id: $id, label: $label, kind: $kind, readonly: false, get: |$w| $get, set: |$m, $v| $set }
    };
}

/// The fields that are not engine setting rows: the frontend's options nox has an equivalent
/// for (their `GuiOptions` preferences keys as ids), the page's composite controls over several
/// settings (as the Win32 dialog composes them) and the Web UI's own preferences.
pub(crate) static FIELDS: &[Field] = &[
    // General
    field!(
        "ui.language",
        Some(S::PrefUiLanguage),
        Kind::Language,
        |w| Value::Str(w.prefs.language.clone()),
        |w, v| w.prefs.language = string(v).trim().to_owned()
    ),
    // nox's update check (D43; not on the page hosted: the host's own options)
    field!(
        "update.check",
        Some(S::PrefCheckUpdates),
        Kind::Bool,
        |w| Value::Bool(w.prefs.check_updates),
        |w, v| w.prefs.check_updates = boolean(&v)
    ),
    field!(
        "update.beta",
        Some(S::PrefUpdateBeta),
        Kind::Bool,
        |w| Value::Bool(w.prefs.update_beta),
        |w, v| w.prefs.update_beta = boolean(&v)
    ),
    // Directories
    field!(
        "dir.download",
        Some(S::PrefNewDownloadsIn),
        Kind::Folder,
        |w| Value::Str(w.prefs.save_path.clone()),
        |w, v| w.prefs.save_path = string(v).trim().to_owned()
    ),
    field!(
        "dir.move_completed",
        Some(S::PrefMoveCompleted),
        Kind::Bool,
        |w| Value::Bool(w.prefs.move_completed),
        |w, v| w.prefs.move_completed = boolean(&v)
    ),
    field!(
        "dir.move_add_label",
        Some(S::PrefMoveAddLabel),
        Kind::Bool,
        |w| Value::Bool(w.prefs.move_add_label),
        |w, v| w.prefs.move_add_label = boolean(&v)
    ),
    field!("dir.completed", None, Kind::Folder, |w| Value::Str(w.prefs.completed_dir.clone()), |w, v| w
        .prefs
        .completed_dir =
        string(v).trim().to_owned()),
    field!(
        "dir.move_only_default",
        Some(S::PrefMoveOnlyDefault),
        Kind::Bool,
        |w| Value::Bool(w.prefs.move_only_default),
        |w, v| w.prefs.move_only_default = boolean(&v)
    ),
    field!(
        "dir.store_torrents",
        Some(S::PrefStoreTorrents),
        Kind::Bool,
        |w| Value::Bool(w.prefs.torrent_folders.store),
        |w, v| w.prefs.torrent_folders.store = boolean(&v)
    ),
    field!(
        "dir.torrents",
        None,
        Kind::Folder,
        |w| Value::Str(w.prefs.torrent_folders.store_dir.clone()),
        |w, v| w.prefs.torrent_folders.store_dir = string(v).trim().to_owned()
    ),
    field!(
        "dir.move_torrents",
        Some(S::PrefMoveTorrents),
        Kind::Bool,
        |w| Value::Bool(w.prefs.torrent_folders.move_finished),
        |w, v| w.prefs.torrent_folders.move_finished = boolean(&v)
    ),
    field!(
        "dir.finished_torrents",
        None,
        Kind::Folder,
        |w| Value::Str(w.prefs.torrent_folders.finished_dir.clone()),
        |w, v| w.prefs.torrent_folders.finished_dir = string(v).trim().to_owned()
    ),
    field!(
        "dir.autoload",
        Some(S::PrefAutoload),
        Kind::Bool,
        |w| Value::Bool(w.prefs.torrent_folders.autoload),
        |w, v| w.prefs.torrent_folders.autoload = boolean(&v)
    ),
    field!(
        "dir.autoload_dir",
        None,
        Kind::Folder,
        |w| Value::Str(w.prefs.torrent_folders.autoload_dir.clone()),
        |w, v| w.prefs.torrent_folders.autoload_dir = string(v).trim().to_owned()
    ),
    field!(
        "dir.autoload_delete",
        Some(S::PrefAutoloadDelete),
        Kind::Bool,
        |w| Value::Bool(w.prefs.torrent_folders.autoload_delete),
        |w, v| w.prefs.torrent_folders.autoload_delete = boolean(&v)
    ),
    // UI Settings
    field!(
        "ui.confirm_delete",
        Some(S::PrefConfirmDelete),
        Kind::Bool,
        |w| Value::Bool(w.prefs.confirm_delete),
        |w, v| w.prefs.confirm_delete = boolean(&v)
    ),
    field!(
        "ui.add_stopped",
        Some(S::PrefDontStart),
        Kind::Bool,
        |w| Value::Bool(w.prefs.start_paused),
        |w, v| w.prefs.start_paused = boolean(&v)
    ),
    field!(
        "ui.show_add_dialog",
        Some(S::PrefShowAddDialog),
        Kind::Bool,
        |w| Value::Bool(w.prefs.show_add_dialog),
        |w, v| w.prefs.show_add_dialog = boolean(&v)
    ),
    // D74: "Do not download files named" and its patterns, kept tidy (`exclude::to_stored`)
    field!(
        pref_pages::EXCLUDE_NAMES,
        Some(S::PrefExcludeNames),
        Kind::Bool,
        |w| Value::Bool(w.prefs.exclude_names),
        |w, v| w.prefs.exclude_names = boolean(&v)
    ),
    field!(
        pref_pages::EXCLUDE_LIST,
        None,
        Kind::Text,
        |w| Value::Str(w.prefs.exclude_list.clone()),
        |w, v| w.prefs.exclude_list = exclude::to_stored(&string(v))
    ),
    // Connection
    field!(
        "listen_port",
        Some(S::PrefPort),
        Kind::Int { min: 1, max: 65535 },
        |w| Value::Int(settings::listen_port(&w.settings).map_or(0, i64::from)),
        |w, v| settings::set_listen_port(&mut w.settings, int(&v) as u16)
    ),
    field!(
        "net.random_port",
        Some(S::PrefRandomize),
        Kind::Bool,
        |w| Value::Bool(w.prefs.random_port),
        |w, v| w.prefs.random_port = boolean(&v)
    ),
    field!(
        "proxy_kind",
        Some(S::PrefProxyType),
        Kind::Enum(&PROXY_TYPES),
        |w| Value::Int(proxy_to_ui(&w.settings).0),
        |w, v| w.settings.proxy_type = proxy_from_ui(int(&v), proxy_to_ui(&w.settings).1)
    ),
    field!(
        "proxy_auth",
        Some(S::PrefAuthentication),
        Kind::Bool,
        |w| Value::Bool(proxy_to_ui(&w.settings).1),
        |w, v| w.settings.proxy_type = proxy_from_ui(proxy_to_ui(&w.settings).0, boolean(&v))
    ),
    // Connection > Use Only This Network (D54): the drop-down before the range box (changes
    // apply in this order), both composing `network_rule`
    field!(
        "net.network_choice",
        Some(S::PrefNetworkRule),
        Kind::Network,
        |w| Value::Str(network::choice_of(&w.settings.network_rule)),
        |w, v| w.settings.network_rule = network::rule_for_choice(&w.settings.network_rule, &string(v))
    ),
    field!(
        "net.network_range",
        Some(S::PrefNetworkRange),
        Kind::Text,
        |w| Value::Str(network::range_of(&w.settings.network_rule)),
        |w, v| w.settings.network_rule = network::rule_for_range(&w.settings.network_rule, &string(v))
    ),
    // Disk Cache: each box before its number (changes apply in this order)
    field!(
        "cache.override",
        Some(S::PrefCacheOverride),
        Kind::Bool,
        |w| Value::Bool(settings::cache_size_overridden(&w.settings)),
        |w, v| {
            let mb = settings::cache_size_mb(&w.settings);
            settings::set_cache_size(&mut w.settings, boolean(&v), mb)
        }
    ),
    field!(
        "cache.size_mb",
        None,
        Kind::Int { min: 1, max: 65535 },
        |w| Value::Int(settings::cache_size_mb(&w.settings) as i64),
        |w, v| {
            let on = settings::cache_size_overridden(&w.settings);
            settings::set_cache_size(&mut w.settings, on, int(&v) as u32)
        }
    ),
    field!(
        "cache.expire",
        Some(S::PrefCacheExpire),
        Kind::Bool,
        |w| Value::Bool(settings::cache_expiry_on(&w.settings)),
        |w, v| {
            let secs = settings::cache_expiry_secs(&w.settings);
            settings::set_cache_expiry(&mut w.settings, boolean(&v), secs)
        }
    ),
    field!(
        "cache.expiry_s",
        None,
        Kind::Int { min: 1, max: 86400 },
        |w| Value::Int(settings::cache_expiry_secs(&w.settings) as i64),
        |w, v| {
            let on = settings::cache_expiry_on(&w.settings);
            settings::set_cache_expiry(&mut w.settings, on, int(&v) as u32)
        }
    ),
    // Bandwidth
    field!(
        "bt.max_peers",
        Some(S::PrefPeersPerTorrent),
        Kind::Int { min: 0, max: 65535 },
        |w| Value::Int(w.prefs.max_peers_per_torrent as i64),
        |w, v| w.prefs.max_peers_per_torrent = int(&v) as u32
    ),
    field!(
        "bt.upload_slots",
        Some(S::PrefSlotsPerTorrent),
        Kind::Int { min: 0, max: 65535 },
        |w| Value::Int(w.prefs.upload_slots_per_torrent as i64),
        |w, v| w.prefs.upload_slots_per_torrent = int(&v) as u32
    ),
    // Queueing > Seeding Goal (`frontend::seeding`)
    field!(
        "queue.seed_ratio",
        Some(S::PrefRatio),
        Kind::Int { min: 0, max: 1_000_000 },
        |w| Value::Int(w.prefs.seeding_goal.goal.ratio_pct as i64),
        |w, v| w.prefs.seeding_goal.goal.ratio_pct = int(&v) as u32
    ),
    field!(
        "queue.seed_time",
        Some(S::PrefSeedTime),
        Kind::Int { min: 0, max: 1_000_000 },
        |w| Value::Int(w.prefs.seeding_goal.goal.minutes as i64),
        |w, v| w.prefs.seeding_goal.goal.minutes = int(&v) as u32
    ),
    field!(
        "queue.seed_limit",
        Some(S::PrefGoalAct),
        Kind::Bool,
        |w| Value::Bool(w.prefs.seeding_goal.limit_upload),
        |w, v| w.prefs.seeding_goal.limit_upload = boolean(&v)
    ),
    field!(
        "queue.seed_limit_kib",
        Some(S::PrefGoalLimit),
        Kind::Int { min: 0, max: RATE_MAX / 1024 },
        |w| Value::Int(w.prefs.seeding_goal.limit_kib as i64),
        |w, v| w.prefs.seeding_goal.limit_kib = int(&v) as u32
    ),
    field!(
        "alt_up_limit",
        Some(S::PrefAltUpRate),
        Kind::Int { min: 0, max: RATE_MAX },
        |w| Value::Int(w.prefs.alt_up_limit as i64),
        |w, v| w.prefs.alt_up_limit = int(&v) as u32
    ),
    field!(
        "alt_dl_limit",
        Some(S::PrefAltDownRate),
        Kind::Int { min: 0, max: RATE_MAX },
        |w| Value::Int(w.prefs.alt_dl_limit as i64),
        |w, v| w.prefs.alt_dl_limit = int(&v) as u32
    ),
    field!(
        "alt_enabled",
        Some(S::PrefUseAltRates),
        Kind::Bool,
        |w| Value::Bool(w.prefs.alt_enabled),
        |w, v| w.prefs.alt_enabled = boolean(&v)
    ),
    // BitTorrent
    field!("bt.pex", Some(S::PrefPex), Kind::Bool, |w| Value::Bool(w.prefs.pex), |w, v| w.prefs.pex =
        boolean(&v)),
    // uTP both ways; the four transports one by one are bt.transp_disposition on Advanced
    field!(
        "utp",
        Some(S::PrefUtp),
        Kind::Bool,
        |w| Value::Bool(w.settings.enable_outgoing_utp || w.settings.enable_incoming_utp),
        |w, v| (w.settings.enable_outgoing_utp, w.settings.enable_incoming_utp) = (boolean(&v), boolean(&v))
    ),
    field!(
        "enc_out",
        Some(S::PrefOutgoing),
        Kind::Enum(&ENC_OUT),
        |w| Value::Int(enc_out(&w.settings)),
        |w, v| {
            let legacy = settings::allow_legacy_incoming(&w.settings);
            settings::set_encryption(&mut w.settings, enc_policy(int(&v)), legacy)
        }
    ),
    field!(
        "enc_legacy",
        Some(S::PrefLegacy),
        Kind::Bool,
        |w| Value::Bool(settings::allow_legacy_incoming(&w.settings)),
        |w, v| {
            let out = w.settings.out_enc_policy;
            settings::set_encryption(&mut w.settings, out, boolean(&v))
        }
    ),
    field!(
        "ipfilter.enable",
        Some(S::PrefIpFilterEnable),
        Kind::Bool,
        |w| Value::Bool(w.prefs.ip_filter),
        |w, v| w.prefs.ip_filter = boolean(&v)
    ),
    field!("ipfilter.path", None, Kind::Text, |w| Value::Str(w.prefs.ip_filter_path.clone()), |w, v| w
        .prefs
        .ip_filter_path =
        string(v).trim().to_owned()),
    // D75: the list from an `https://` address instead of the file; security fields, see
    // `SECURITY_FIELDS`
    field!(
        pref_pages::IPFILTER_FROM_URL,
        Some(S::PrefIpFilterFromUrl),
        Kind::Bool,
        |w| Value::Bool(w.prefs.ip_filter_from_url),
        |w, v| w.prefs.ip_filter_from_url = boolean(&v)
    ),
    field!(
        pref_pages::IPFILTER_URL,
        None,
        Kind::Text,
        |w| Value::Str(w.prefs.ip_filter_url.clone()),
        |w, v| w.prefs.ip_filter_url = string(v).trim().to_owned()
    ),
    // Scheduler
    field!(
        "sched.enable",
        Some(S::PrefScheduler),
        Kind::Bool,
        |w| Value::Bool(w.prefs.schedule.enabled),
        |w, v| w.prefs.schedule.enabled = boolean(&v)
    ),
    field!("sched.table", None, Kind::Schedule, |w| Value::Str(w.prefs.schedule.table_text()), |w, v| {
        if let Some(t) = scheduler::parse_table(&string(v)) {
            w.prefs.schedule.table = t;
        }
    }),
    field!(
        "sched.ul_rate",
        Some(S::PrefSchedUp),
        Kind::Int { min: 0, max: RATE_MAX / 1024 },
        |w| Value::Int(w.prefs.schedule.limited_up_kib as i64),
        |w, v| w.prefs.schedule.limited_up_kib = int(&v) as u32
    ),
    field!(
        "sched.dl_rate",
        Some(S::PrefSchedDown),
        Kind::Int { min: 0, max: RATE_MAX / 1024 },
        |w| Value::Int(w.prefs.schedule.limited_down_kib as i64),
        |w, v| w.prefs.schedule.limited_down_kib = int(&v) as u32
    ),
    field!(
        "sched.dis_dht",
        Some(S::PrefSchedDht),
        Kind::Bool,
        |w| Value::Bool(w.prefs.schedule.disable_dht),
        |w, v| w.prefs.schedule.disable_dht = boolean(&v)
    ),
    // Web UI
    Field {
        id: "webui.listen",
        label: Some(S::WebListenAddress),
        kind: Kind::Text,
        readonly: true,
        get: |w| Value::Str(w.listen.clone()),
        set: |_, _| {},
    },
    field!(
        "webui.username",
        Some(S::PrefUsername),
        Kind::Text,
        |w| Value::Str(w.username.clone()),
        |w, v| w.username = string(v).trim().to_owned()
    ),
    field!(
        "webui.password",
        Some(S::PrefPassword),
        Kind::Password,
        |_w| Value::Str(String::new()),
        |w, v| {
            let p = string(v);
            if !p.is_empty() {
                w.new_password = Some(p);
            }
        }
    ),
    field!(
        "webui.current_password",
        Some(S::PrefCurrentPassword),
        Kind::Password,
        |_w| Value::Str(String::new()),
        // never stored: needed to change the login and the Web UI's security settings (SEC-03),
        // [`Asker::new`] takes it from the request's changes
        |_w, _v| {}
    ),
    field!(
        "webui.bypass_local_auth",
        Some(S::WebBypassLocal),
        Kind::Bool,
        |w| Value::Bool(w.prefs.bypass_local_auth),
        |w, v| w.prefs.bypass_local_auth = boolean(&v)
    ),
    field!(
        "webui.api_key_enabled",
        Some(S::WebApiKeyEnabled),
        Kind::Bool,
        |w| Value::Bool(w.prefs.api_key_enabled),
        |w, v| w.prefs.api_key_enabled = boolean(&v)
    ),
    field!("webui.api_key", Some(S::WebApiKey), Kind::Password, |_w| Value::Str(String::new()), |w, v| {
        let k = string(v).trim().to_owned();
        if !k.is_empty() {
            w.new_api_key = Some(k);
        }
    }),
    field!(
        "webui.bypass_auth_subnets",
        Some(S::WebAuthSubnets),
        Kind::Text,
        |w| Value::Str(w.prefs.auth_subnets.clone()),
        |w, v| w.prefs.auth_subnets = string(v).trim().to_owned()
    ),
    field!(
        "webui.host_header_validation",
        Some(S::WebHostHeader),
        Kind::Bool,
        |w| Value::Bool(w.prefs.host_header_validation),
        |w, v| w.prefs.host_header_validation = boolean(&v)
    ),
    field!(
        "webui.domain_list",
        Some(S::WebDomains),
        Kind::Text,
        |w| Value::Str(w.prefs.domain_list.clone()),
        |w, v| w.prefs.domain_list = string(v).trim().to_owned()
    ),
    field!("webui.csrf", Some(S::WebCsrf), Kind::Bool, |w| Value::Bool(w.prefs.csrf_protection), |w, v| w
        .prefs
        .csrf_protection =
        boolean(&v)),
    field!(
        "webui.max_auth_fail_count",
        Some(S::WebBanAfter),
        Kind::Int { min: 0, max: 1000 },
        |w| Value::Int(w.prefs.max_auth_fail_count as i64),
        |w, v| w.prefs.max_auth_fail_count = int(&v) as u32
    ),
    field!(
        "webui.ban_duration",
        Some(S::WebBanFor),
        Kind::Int { min: 1, max: YEAR },
        |w| Value::Int(w.prefs.ban_duration as i64),
        |w, v| w.prefs.ban_duration = int(&v) as u64
    ),
    field!(
        "webui.session_timeout",
        Some(S::WebSessionTimeout),
        Kind::Int { min: 1, max: YEAR },
        |w| Value::Int(w.prefs.session_timeout as i64),
        |w, v| w.prefs.session_timeout = int(&v) as u64
    ),
    field!("webui.theme", Some(S::PrefTheme), Kind::Theme, |w| Value::Str(w.prefs.theme.clone()), |w, v| w
        .prefs
        .theme =
        string(v).trim().to_owned()),
    // Advanced > Trackers (D66): security fields, see `SECURITY_FIELDS`
    field!(
        pref_pages::TRACKERS_APPEND,
        Some(S::PrefTrackersAppend),
        Kind::Bool,
        |w| Value::Bool(w.prefs.trackers_append),
        |w, v| w.prefs.trackers_append = boolean(&v)
    ),
    // as typed until the commit, which checks every line and then keeps the list's own form
    field!(
        pref_pages::TRACKERS_LIST,
        None,
        Kind::Lines,
        |w| Value::Str(w.prefs.trackers_list.clone()),
        |w, v| w.prefs.trackers_list = string(v)
    ),
    field!(
        pref_pages::TRACKERS_FROM_URL,
        Some(S::PrefTrackersFromUrl),
        Kind::Bool,
        |w| Value::Bool(w.prefs.trackers_from_url),
        |w, v| w.prefs.trackers_from_url = boolean(&v)
    ),
    field!(
        pref_pages::TRACKERS_URL,
        None,
        Kind::Text,
        |w| Value::Str(w.prefs.trackers_url.clone()),
        |w, v| w.prefs.trackers_url = string(v).trim().to_owned()
    ),
    // Advanced > Run Program (D67): changed only through `commit_request` with an open
    // [`RunGate`] and the current password, see `RUN_FIELDS`
    field!(
        pref_pages::RUN_ADDED_ON,
        Some(S::PrefRunAdded),
        Kind::Bool,
        |w| Value::Bool(w.prefs.run_added_on),
        |w, v| w.prefs.run_added_on = boolean(&v)
    ),
    field!(pref_pages::RUN_ADDED, None, Kind::Text, |w| Value::Str(w.prefs.run_added.clone()), |w, v| w
        .prefs
        .run_added =
        string(v).trim().to_owned()),
    field!(
        pref_pages::RUN_FINISHED_ON,
        Some(S::PrefRunFinished),
        Kind::Bool,
        |w| Value::Bool(w.prefs.run_finished_on),
        |w, v| w.prefs.run_finished_on = boolean(&v)
    ),
    field!(
        pref_pages::RUN_FINISHED,
        None,
        Kind::Text,
        |w| Value::Str(w.prefs.run_finished.clone()),
        |w, v| w.prefs.run_finished = string(v).trim().to_owned()
    ),
];

/// The 2.2.1 wording of the engine settings the pages show (the Advanced table shows the
/// libtorrent names); a row not listed here is labelled by its name.
const LABELS: &[(&str, S)] = &[
    ("enable_upnp", S::PrefUpnp),
    ("enable_natpmp", S::PrefNatPmp),
    ("proxy_hostname", S::PrefProxy),
    ("proxy_port", S::PrefProxyPort),
    ("proxy_username", S::PrefUsername),
    ("proxy_password", S::PrefPassword),
    ("proxy_hostnames", S::PrefProxyNames),
    ("proxy_peer_connections", S::PrefProxyPeers),
    ("proxy_tracker_connections", S::PrefProxyTrackers),
    ("net.pause_when_missing", S::PrefNetworkPause),
    ("upload_rate_limit", S::PrefUpRate),
    ("download_rate_limit", S::PrefDownRate),
    ("rate_limit_ip_overhead", S::PrefRateOverhead),
    ("rate_limit_utp", S::PrefRateUtp),
    ("auto_rate_limit", S::PrefKeepPingLow),
    ("connections_limit", S::PrefMaxConnections),
    ("unchoke_slots_limit", S::PrefGlobalSlots),
    ("enable_dht", S::PrefDht),
    ("enable_lsd", S::PrefLsd),
    ("active_limit", S::PrefActive),
    ("active_downloads", S::PrefActiveDownloads),
    ("active_seeds", S::PrefActiveSeeds),
    ("dont_count_slow_torrents", S::PrefSlow),
    ("max_queued_disk_bytes", S::PrefDiskWriteQueue),
    ("file_pool_size", S::PrefOpenFiles),
    ("aio_threads", S::PrefAioThreads),
    ("hashing_threads", S::PrefHashThreads),
    ("use_read_cache", S::PrefReadCacheEnable),
    ("check_gently", S::PrefCheckGently),
    ("check_rate_limit", S::PrefCheckRateLimit),
    ("active_checking", S::PrefActiveChecking),
    ("tune_for_drive", S::PrefTuneForDrive),
    ("disk_slow_downloads", S::PrefDiskSlowDownloads),
    ("disk_slow_uploads", S::PrefDiskSlowUploads),
    ("append_incomplete_ext", S::PrefAppendIncomplete),
];

/// Engine settings the pages show without a caption: a number beside its check box, whose
/// text ends in the unit (as `cache.size_mb` beside `cache.override`).
const UNLABELLED: &[&str] = &["check_rate_mb"];

/// Help texts (D55) a field shows instead of its label's: a field without a label shows its
/// check box's (as `frontend::pref_pages` gives its label-less fields), and a label two
/// fields share has a text for each.
const HELP: &[(&str, H)] = &[
    ("dir.completed", H::PrefMoveCompleted),
    ("dir.torrents", H::PrefStoreTorrents),
    ("dir.finished_torrents", H::PrefMoveTorrents),
    ("dir.autoload_dir", H::PrefAutoload),
    ("ipfilter.path", H::PrefIpFilterEnable),
    (pref_pages::IPFILTER_URL, H::PrefIpFilterFromUrl),
    (pref_pages::EXCLUDE_LIST, H::PrefExcludeNames),
    ("queue.seed_limit_kib", H::PrefGoalLimit),
    ("cache.size_mb", H::PrefCacheOverride),
    ("cache.expiry_s", H::PrefCacheExpire),
    ("check_rate_mb", H::PrefCheckRateLimit),
    ("sched.table", H::PrefSchedTable),
    ("proxy_username", H::PrefProxyUsername),
    ("proxy_password", H::PrefProxyPassword),
    ("webui.username", H::PrefWebUiUsername),
    ("webui.password", H::PrefWebUiPassword),
    (pref_pages::TRACKERS_LIST, H::PrefTrackersAppend),
    (pref_pages::TRACKERS_URL, H::PrefTrackersFromUrl),
    (pref_pages::RUN_ADDED_ON, H::PrefRunAdded),
    (pref_pages::RUN_ADDED, H::PrefRunAdded),
    (pref_pages::RUN_FINISHED_ON, H::PrefRunFinished),
    (pref_pages::RUN_FINISHED, H::PrefRunFinished),
];

/// One editable thing: an engine setting row or one of [`FIELDS`].
#[derive(Clone, Copy)]
pub(crate) enum Def {
    Setting(&'static Entry),
    Own(&'static Field),
}

impl Def {
    pub fn id(&self) -> &'static str {
        match self {
            Def::Setting(e) => e.name,
            Def::Own(f) => f.id,
        }
    }

    /// The caption, without the desktop dialogs' `&` mnemonics (`lang::plain`: "Append .!&ut
    /// to incomplete files", D77).
    fn label(&self) -> String {
        let text = match self {
            Def::Setting(e) if UNLABELLED.contains(&e.name) => "",
            Def::Setting(e) => {
                LABELS.iter().find(|(n, _)| *n == e.name).map_or(e.name, |(_, l)| lang::text(*l))
            }
            Def::Own(f) => f.label.map_or("", lang::text),
        };
        lang::plain(text)
    }

    /// The field's help key (D55): its own ([`HELP`]), else its label's, else for an engine
    /// row the setting's (`frontend::help::for_setting`).
    fn help_key(&self) -> Option<H> {
        let id = self.id();
        if let Some((_, h)) = HELP.iter().find(|(n, _)| *n == id) {
            return Some(*h);
        }
        let label = match self {
            Def::Setting(e) => LABELS.iter().find(|(n, _)| *n == e.name).map(|(_, l)| *l),
            Def::Own(f) => f.label,
        };
        label.and_then(H::for_label).or_else(|| match self {
            Def::Setting(e) => help::for_setting(e.name),
            Def::Own(_) => None,
        })
    }

    pub fn kind(&self) -> Kind {
        match self {
            Def::Setting(e) => match e.kind {
                SettingKind::Bool => Kind::Bool,
                SettingKind::Int { min, max } => Kind::Int { min, max },
                SettingKind::Str => Kind::Text,
            },
            Def::Own(f) => f.kind,
        }
    }

    fn readonly(&self) -> bool {
        matches!(self, Def::Own(f) if f.readonly)
    }

    pub fn get(&self, w: &Working) -> Value {
        match self {
            Def::Setting(e) => e.get(&w.settings),
            Def::Own(f) => (f.get)(w),
        }
    }

    fn set(&self, w: &mut Working, v: Value) {
        match self {
            Def::Setting(e) => {
                let _ = e.set(&mut w.settings, &v.to_string());
            }
            Def::Own(f) => (f.set)(w, v),
        }
    }

    /// Parses a value for this field: the engine settings through `Entry::parse`, the rest by
    /// their kind the same way. The error says why, without the field's name.
    pub fn parse(&self, text: &str) -> Result<Value, String> {
        if let Def::Setting(e) = self {
            return e
                .parse(text)
                .map_err(|m| m.strip_prefix(&format!("{}: ", e.name)).unwrap_or(&m).to_owned());
        }
        let t = text.trim();
        match self.kind() {
            Kind::Bool => match t.to_ascii_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => Ok(Value::Bool(true)),
                "false" | "0" | "no" | "off" => Ok(Value::Bool(false)),
                _ => Err(tr!(SettingNotBool, value = t)),
            },
            Kind::Int { min, max } => match t.parse::<i64>() {
                Ok(i) if (min..=max).contains(&i) => Ok(Value::Int(i)),
                Ok(_) => Err(tr!(SettingOutOfRange, value = t, min, max)),
                Err(_) => Err(tr!(SettingNotNumber, value = t)),
            },
            Kind::Enum(options) => match t.parse::<i64>() {
                Ok(i) if (0..options.len() as i64).contains(&i) => Ok(Value::Int(i)),
                Ok(_) => Err(tr!(SettingOutOfRange, value = t, min = 0, max = options.len() - 1)),
                // by name: the English one (what scripts send) or the current language's
                Err(_) => options
                    .iter()
                    .position(|&o| {
                        o.english().eq_ignore_ascii_case(t) || lang::text(o).eq_ignore_ascii_case(t)
                    })
                    .map(|i| Value::Int(i as i64))
                    .ok_or_else(|| {
                        let names: Vec<&str> = options.iter().map(|&o| lang::text(o)).collect();
                        tr!(SettingNotOneOf, value = t, options = names.join(", "))
                    }),
            },
            Kind::Text | Kind::Folder | Kind::Password | Kind::Network => Ok(Value::Str(text.to_owned())),
            Kind::Lines => Ok(Value::Str(text.replace("\r\n", "\n").replace('\r', "\n"))),
            Kind::Theme if theme::valid_id(t) => Ok(Value::Str(t.to_owned())),
            Kind::Theme => Err(tr!(PrefBadTheme).to_owned()),
            Kind::Schedule => match scheduler::parse_table(t) {
                Some(_) => Ok(Value::Str(t.to_owned())),
                None => Err(tr!(SettingBadSchedule, count = scheduler::CELLS)),
            },
            // a shipped language, the system's (empty) or the pseudo-language
            Kind::Language => match lang::LANGUAGES.iter().find(|l| l.code.eq_ignore_ascii_case(t)) {
                Some(l) => Ok(Value::Str(l.code.to_owned())),
                None if t.is_empty() || t == "pseudo" => Ok(Value::Str(t.to_owned())),
                None => {
                    let codes: Vec<&str> = lang::LANGUAGES.iter().map(|l| l.code).collect();
                    Err(tr!(SettingNotOneOf, value = t, options = codes.join(", ")))
                }
            },
        }
    }
}

pub(crate) fn find(id: &str) -> Option<Def> {
    FIELDS.iter().find(|f| f.id == id).map(Def::Own).or_else(|| settings::find(id).map(Def::Setting))
}

/// Every field once: the engine settings in the table's order, then [`FIELDS`].
fn all() -> impl Iterator<Item = Def> {
    TABLE.iter().map(Def::Setting).chain(FIELDS.iter().map(Def::Own))
}

fn json_of(v: &Value) -> Json {
    match v {
        Value::Bool(b) => Json::Bool(*b),
        Value::Int(i) => Json::Int(*i),
        Value::Str(s) => Json::Str(s.clone()),
    }
}

/// A JSON value as the text the parsers take.
fn text_of(v: &Json) -> Option<String> {
    match v {
        Json::Bool(b) => Some(b.to_string()),
        Json::Int(i) => Some(i.to_string()),
        Json::Float(_) => v.as_i64().map(|i| i.to_string()),
        Json::Str(s) => Some(s.clone()),
        _ => None,
    }
}

// ---- the page layout -----------------------------------------------------------------------

fn f(id: &str) -> Json {
    Json::obj().with("f", id)
}

/// A field shown in kB/s (or kB) on the page; the value is bytes.
fn kb(id: &str) -> Json {
    f(id).with("kb", true)
}

fn ranged(id: &str, min: i64, max: i64) -> Json {
    f(id).with("min", min).with("max", max)
}

/// Greys `item` while the check box `id` is off (several ids, space-separated: while any is).
fn enabled_by(item: Json, id: &str) -> Json {
    item.with("enabled_by", id)
}

/// Greys `item` while the check box `id` is on (several ids, space-separated: while any is),
/// as `frontend::pref_pages::Field::unless` (the IP filter's file while the list comes from an
/// address, D75).
fn disabled_by(item: Json, id: &str) -> Json {
    item.with("disabled_by", id)
}

fn row(items: Vec<Json>) -> Json {
    Json::obj().with("row", Json::Arr(items))
}

/// A control that is a placeholder until its feature exists (a disabled check box, or a
/// disabled text field with `text`).
fn stub(label: &str) -> Json {
    Json::obj().with("stub", label)
}

fn note(text: impl Into<String>) -> Json {
    Json::obj().with("note", text.into())
}

/// A control the page keeps for itself (nothing is sent): `theme`, the browser's own theme.
fn local(what: &str, label: &str, help: H) -> Json {
    Json::obj().with("local", what).with("label", label).with("help", lang::help(help))
}

fn group(title: Option<&str>, items: Vec<Json>) -> Json {
    Json::obj().with("title", title.map_or(Json::Null, Json::from)).with("items", Json::Arr(items))
}

fn page(id: &str, title: &str, groups: Vec<Json>) -> Json {
    Json::obj().with("id", id).with("title", title).with("groups", Json::Arr(groups))
}

/// The tree, in µTorrent 2.2.1's order; Disk Cache and Rookparcel's Disk I/O and Trackers sit
/// under Advanced. `hosted`: without the alternative speed limits (the host has none), the
/// update check, the Trackers page and the IP filter's list from an address (the host's own).
fn pages(settings_dir: &Path, hosted: bool, run_open: bool) -> Vec<Json> {
    let ipf_default = settings_dir.join("ipfilter.dat");
    // the file greys while the list comes from an address (D75)
    let mut ip_filter = vec![
        f(pref_pages::IPFILTER_ENABLE),
        disabled_by(
            enabled_by(f(pref_pages::IPFILTER_PATH), pref_pages::IPFILTER_ENABLE),
            pref_pages::IPFILTER_FROM_URL,
        ),
        note(tr!(PrefIpFilterEmptyPath, path = ipf_default.display())),
    ];
    if !hosted {
        // the list from an address (D75); hosted, the host's own, which a browser neither sees
        // nor changes (its file greys all the same while the host takes the list from one)
        let both = format!("{} {}", pref_pages::IPFILTER_ENABLE, pref_pages::IPFILTER_FROM_URL);
        ip_filter.extend([
            enabled_by(f(pref_pages::IPFILTER_FROM_URL), pref_pages::IPFILTER_ENABLE),
            enabled_by(f(pref_pages::IPFILTER_URL), &both),
            enabled_by(
                Json::obj()
                    .with("button", tr!(PrefTrackersUpdate))
                    .with("action", "update_ipfilter")
                    .with("help", lang::help(H::PrefIpFilterUpdate)),
                &both,
            ),
            Json::obj().with("status", "ipfilter"),
        ]);
    }
    let mut bandwidth = vec![
        group(Some(tr!(PrefUpLimiting)), vec![kb("upload_rate_limit")]),
        group(Some(tr!(PrefDownLimiting)), vec![kb("download_rate_limit")]),
        group(Some(tr!(PrefRateOptions)), vec![f("rate_limit_ip_overhead"), f("rate_limit_utp")]),
        group(
            Some(tr!(PrefConnections)),
            vec![
                ranged("connections_limit", 2, 65535),
                f("bt.max_peers"),
                f("bt.upload_slots"),
                ranged("unchoke_slots_limit", -1, 65535),
            ],
        ),
        group(None, vec![note(tr!(PrefPerTorrentNote))]),
    ];
    if !hosted {
        bandwidth.push(group(
            Some(tr!(PrefAltRates)),
            vec![kb("alt_up_limit"), kb("alt_dl_limit"), f("alt_enabled")],
        ));
    }
    let mut general = vec![
        group(Some(tr!(PrefLanguage)), vec![f("ui.language")]),
        group(
            Some(tr!(PrefIntegration)),
            vec![
                stub(tr!(PrefAssocTorrent)),
                stub(tr!(PrefAssocMagnet)),
                stub(tr!(PrefAutostart)),
                note(tr!(PrefDesktopOnly)),
            ],
        ),
    ];
    if !hosted {
        // nox's update check (D43); hosted, the host's Preferences have its own
        general.push(group(Some(tr!(PrefPrivacy)), vec![f("update.check"), f("update.beta")]));
    }
    // D77: µTorrent 2.2.1's group, last on the page (`frontend::pref_pages`); an engine
    // setting, so hosted it goes to the host like the others
    general.push(group(Some(tr!(PrefWhenDownloading)), vec![f("append_incomplete_ext")]));
    let mut pages = vec![
        page("general", tr!(PageGeneral), general),
        page(
            "ui",
            tr!(PageUi),
            vec![
                group(
                    Some(tr!(PrefDisplay)),
                    vec![
                        f("ui.confirm_delete"),
                        f("webui.theme"),
                        local("theme", tr!(PrefThemeBrowser), H::PrefThemeBrowser),
                    ],
                ),
                group(
                    Some(tr!(PrefWhenAdding)),
                    vec![
                        f("ui.add_stopped"),
                        f("ui.show_add_dialog"),
                        // D74: the patterns under their box
                        f(pref_pages::EXCLUDE_NAMES),
                        enabled_by(f(pref_pages::EXCLUDE_LIST), pref_pages::EXCLUDE_NAMES),
                    ],
                ),
            ],
        ),
        page(
            "directories",
            tr!(PageDirectories),
            vec![
                group(
                    Some(tr!(PrefDownloadLocation)),
                    vec![
                        f("dir.download"),
                        // "Append the torrent's label" to its right, as in µTorrent
                        row(vec![
                            f("dir.move_completed"),
                            enabled_by(f("dir.move_add_label"), "dir.move_completed"),
                        ]),
                        enabled_by(f("dir.completed"), "dir.move_completed"),
                        enabled_by(f("dir.move_only_default"), "dir.move_completed"),
                    ],
                ),
                group(
                    Some(tr!(PrefTorrentLocation)),
                    vec![
                        f("dir.store_torrents"),
                        enabled_by(f("dir.torrents"), "dir.store_torrents"),
                        enabled_by(f("dir.move_torrents"), "dir.store_torrents"),
                        enabled_by(f("dir.finished_torrents"), "dir.store_torrents dir.move_torrents"),
                        f("dir.autoload"),
                        enabled_by(f("dir.autoload_dir"), "dir.autoload"),
                        enabled_by(f("dir.autoload_delete"), "dir.autoload"),
                    ],
                ),
            ],
        ),
        page(
            "connection",
            tr!(PageConnection),
            vec![
                group(
                    Some(tr!(PrefListeningPort)),
                    vec![
                        row(vec![
                            f("listen_port"),
                            Json::obj().with("button", tr!(PrefRandomPort)).with("action", "random_port"),
                        ]),
                        f("net.random_port"),
                        f("enable_upnp"),
                        f("enable_natpmp"),
                    ],
                ),
                group(
                    Some(tr!(PrefProxyServer)),
                    vec![
                        // greyed while the type is "(none)" (0), the name and password while
                        // "Authentication" is off, as in µTorrent and the GUIs
                        row(vec![
                            f("proxy_kind"),
                            enabled_by(f("proxy_hostname"), "proxy_kind"),
                            enabled_by(ranged("proxy_port", 0, 65535), "proxy_kind"),
                        ]),
                        enabled_by(f("proxy_auth"), "proxy_kind"),
                        row(vec![
                            enabled_by(f("proxy_username"), "proxy_kind proxy_auth"),
                            enabled_by(f("proxy_password").with("secret", true), "proxy_kind proxy_auth"),
                        ]),
                        enabled_by(f("proxy_hostnames"), "proxy_kind"),
                        enabled_by(f("proxy_peer_connections"), "proxy_kind"),
                        enabled_by(f("proxy_tracker_connections"), "proxy_kind"),
                    ],
                ),
                group(
                    Some(tr!(PrefUseNetwork)),
                    vec![
                        f("net.network_choice"),
                        enabled_by(f("net.network_range"), "net.network_choice"),
                        enabled_by(f(settings::NETWORK_PAUSE), "net.network_choice"),
                    ],
                ),
            ],
        ),
        page("bandwidth", tr!(PageBandwidth), bandwidth),
        page(
            "bittorrent",
            tr!(PageBitTorrent),
            vec![
                group(
                    Some(tr!(PrefBasicFeatures)),
                    vec![f("enable_dht"), f("enable_lsd"), f("bt.pex"), f("utp")],
                ),
                group(Some(tr!(PrefEncryption)), vec![f("enc_out"), f("enc_legacy")]),
                group(Some(tr!(PrefIpFilter)), ip_filter),
            ],
        ),
        page(
            "queueing",
            tr!(PageQueueing),
            vec![
                group(
                    Some(tr!(PrefQueue)),
                    vec![
                        ranged("active_limit", -1, 65535),
                        ranged("active_downloads", -1, 65535),
                        ranged("active_seeds", -1, 65535),
                        f("dont_count_slow_torrents"),
                    ],
                ),
                group(
                    Some(tr!(PrefSeedingGoal)),
                    vec![
                        f("queue.seed_ratio"),
                        f("queue.seed_time"),
                        note(tr!(PrefGoalReached)),
                        f("queue.seed_limit"),
                        enabled_by(f("queue.seed_limit_kib"), "queue.seed_limit"),
                    ],
                ),
            ],
        ),
        page(
            "scheduler",
            tr!(PageScheduler),
            vec![
                group(None, vec![f("sched.enable")]),
                group(Some(tr!(PrefSchedTable)), vec![enabled_by(f("sched.table"), "sched.enable")]),
                group(
                    Some(tr!(PrefSchedSettings)),
                    vec![
                        enabled_by(f("sched.ul_rate"), "sched.enable"),
                        enabled_by(f("sched.dl_rate"), "sched.enable"),
                        enabled_by(f("sched.dis_dht"), "sched.enable"),
                    ],
                ),
            ],
        ),
        page(
            "webui",
            tr!(PageWebUi),
            vec![
                group(None, vec![stub(tr!(PrefWebUi)).with("on", true)]),
                group(
                    Some(tr!(PrefAuthentication)),
                    vec![
                        f("webui.username"),
                        f("webui.password"),
                        f("webui.current_password"),
                        f("webui.bypass_local_auth"),
                        f("webui.api_key_enabled"),
                        enabled_by(f("webui.api_key"), "webui.api_key_enabled"),
                        f("webui.bypass_auth_subnets"),
                    ],
                ),
                group(
                    Some(tr!(PrefConnectivity)),
                    vec![
                        f("webui.listen"),
                        f("webui.host_header_validation"),
                        f("webui.domain_list"),
                        f("webui.csrf"),
                    ],
                ),
                group(
                    Some(tr!(PrefSecurity)),
                    vec![f("webui.max_auth_fail_count"), f("webui.ban_duration"), f("webui.session_timeout")],
                ),
            ],
        ),
        // what keeps games, calls and the computer responsive (D80): keep ping low (D49) and
        // smart disk I/O (D48); the per-drive types (drive_overrides) are on Advanced only
        page(
            "gaming",
            tr!(PageGaming),
            vec![
                group(Some(tr!(PrefPingGroup)), vec![f("auto_rate_limit"), note(tr!(PrefKeepPingLowNote))]),
                group(
                    Some(tr!(PrefChecking)),
                    vec![
                        f("check_gently"),
                        row(vec![f("check_rate_limit"), enabled_by(f("check_rate_mb"), "check_rate_limit")]),
                        // the engine checks with at most 8 threads; -1 (no limit) is Advanced's
                        ranged("active_checking", 1, 8),
                    ],
                ),
                group(Some(tr!(PrefDrives)), vec![f("tune_for_drive")]),
                group(Some(tr!(PrefDiskBusy)), vec![f("disk_slow_downloads"), f("disk_slow_uploads")]),
                group(None, vec![note(tr!(PrefDiskIoNote))]),
            ],
        ),
        page("advanced", tr!(PageAdvanced), vec![group(None, vec![note(tr!(PrefAdvancedNoteWeb))])])
            .with("advanced", true),
        page(
            "diskcache",
            tr!(PageDiskCache),
            vec![
                group(
                    Some(tr!(PrefReadCache)),
                    vec![
                        f("use_read_cache"),
                        row(vec![
                            enabled_by(f("cache.override"), "use_read_cache"),
                            enabled_by(f("cache.size_mb"), "use_read_cache cache.override"),
                        ]),
                        row(vec![
                            enabled_by(f("cache.expire"), "use_read_cache"),
                            enabled_by(f("cache.expiry_s"), "use_read_cache cache.expire"),
                        ]),
                    ],
                ),
                group(
                    Some(tr!(PrefDiskQueue)),
                    vec![
                        kb("max_queued_disk_bytes").with("min", 1024),
                        ranged("file_pool_size", 1, 65535),
                        ranged("aio_threads", 1, 64),
                        ranged("hashing_threads", 1, 64),
                    ],
                ),
                group(None, vec![note(tr!(PrefReadCacheNote, mb = settings::AUTO_CACHE_MB))]),
            ],
        )
        .with("under", "advanced"),
    ];
    if !hosted {
        // the trackers added to new downloads (D66); hosted, the host's Preferences have its own
        // list, which a browser neither sees nor changes
        pages.push(
            page(
                "trackers",
                tr!(PageTrackers),
                vec![
                    group(
                        Some(tr!(PrefTrackersGroup)),
                        vec![
                            f(pref_pages::TRACKERS_APPEND),
                            enabled_by(f(pref_pages::TRACKERS_LIST), pref_pages::TRACKERS_APPEND),
                            f(pref_pages::TRACKERS_FROM_URL),
                            enabled_by(f(pref_pages::TRACKERS_URL), pref_pages::TRACKERS_FROM_URL),
                            enabled_by(
                                Json::obj()
                                    .with("button", tr!(PrefTrackersUpdate))
                                    .with("action", "update_trackers")
                                    .with("help", lang::help(H::PrefTrackersUpdate)),
                                pref_pages::TRACKERS_FROM_URL,
                            ),
                            Json::obj().with("status", "trackers"),
                        ],
                    ),
                    group(None, vec![note(tr!(PrefTrackersNote))]),
                ],
            )
            .with("under", "advanced"),
        );
        // the programs started on torrent events (D67); hosted, the host's own. A session that
        // may not change them (`RunGate`) sees them read-only (`model`), with the reason
        let mut notes = vec![note(tr!(PrefRunNote)), note(tr!(PrefRunSafetyNote))];
        if !run_open {
            notes.push(note(tr!(RunErrRemote)));
        }
        pages.push(
            page(
                "run_program",
                tr!(PageRunProgram),
                vec![
                    group(
                        Some(tr!(PageRunProgram)),
                        vec![
                            f(pref_pages::RUN_ADDED_ON),
                            enabled_by(f(pref_pages::RUN_ADDED), pref_pages::RUN_ADDED_ON),
                            f(pref_pages::RUN_FINISHED_ON),
                            enabled_by(f(pref_pages::RUN_FINISHED), pref_pages::RUN_FINISHED_ON),
                        ],
                    ),
                    group(None, notes),
                ],
            )
            .with("under", "advanced"),
        );
    }
    pages
}

/// `GET /pa/prefs`: `pages` (the tree in order, each with its groups of items: `{"f": id}`
/// with optional `kb`, `min`, `max`, `enabled_by` (space-separated ids, greyed while any is
/// off), `disabled_by` (greyed while any is on), `secret`; `{"row": [items]}`;
/// `{"stub": label}`; `{"note": text}`; `{"button": label, "action": name}`;
/// `{"status": "trackers"}` and `{"status": "ipfilter"}`, the lines that show
/// `trackers_status` and `ipfilter_status`), `fields` (id to
/// `label`, `type`, `value`, `default`, `min` / `max` for ints, `options` for enums,
/// `readonly`, `help`: the current language's help text, `wiki`: its wiki page once there is
/// one), `advanced` (every engine setting's id, the table's order), `trackers_status` (the
/// fetched tracker list's status line, D66; `GET /pa/prefs/trackers` gives it alone) and
/// `ipfilter_status` (the IP filter's list from an address, D75; `GET /pa/prefs/ipfilter`).
///
/// `run`: whether the asking session may change the Run Program lines (D67, [`RunGate`]). When
/// not, the four fields are `readonly` in this answer, the two command lines are left empty
/// and their page carries the reason; the gate itself is [`commit_request`]'s, whatever a page
/// shows. The lists' addresses ([`ADDRESS_FIELDS`]) are sent without their credentials, query
/// and fragment, hosted not at all (N2).
pub(crate) fn model(shared: &Shared, st: &State, run: RunGate) -> Json {
    // hosted: the lines are the host's own, which no browser changes ([`HOSTED_NO_RUN`])
    let run_open = run == RunGate::Open && !st.hosted;
    let w = Working::from_state(st);
    let d = Working::defaults(st);
    let mut fields = Json::obj();
    for def in all() {
        // a session that may not change the Run Program lines does not read them either, as
        // the qBittorrent API's `autorun_*` go only to a client on this computer: a line can
        // hold an address or a token (D67). Whether they are on is shown.
        let hidden = !run_open && [pref_pages::RUN_ADDED, pref_pages::RUN_FINISHED].contains(&def.id());
        let mut value = def.get(if hidden { &d } else { &w });
        // the lists' addresses (D66, D75) may carry a subscription's credentials: sent without
        // them ([`shown_address`]), as the search sources' (D41); the field left as it is keeps
        // the stored address ([`keep_addresses`]). Hosted, the host's own: never sent
        if let (true, Value::Str(url)) = (ADDRESS_FIELDS.contains(&def.id()), &value) {
            value = Value::Str(if st.hosted { String::new() } else { shown_address(url) });
        }
        let mut j = Json::obj()
            .with("label", def.label())
            .with("type", def.kind().name())
            .with("value", json_of(&value))
            .with("default", json_of(&def.get(&d)));
        match def.kind() {
            Kind::Int { min, max } => {
                j.set("min", min);
                j.set("max", max);
            }
            Kind::Enum(options) => {
                j.set("options", Json::Arr(options.iter().map(|&o| Json::from(lang::text(o))).collect()));
            }
            Kind::Network => {
                let choices = network::rule_choices(&w.settings.network_rule);
                let options = choices.into_iter().map(|(v, t)| Json::Arr(vec![v.into(), t.into()])).collect();
                j.set("options", Json::Arr(options));
            }
            Kind::Language => {
                let current = w.prefs.language.as_str();
                let choices = lang::choices(current, &lang::system_languages());
                let options = choices.into_iter().map(|(v, t)| Json::Arr(vec![v.into(), t.into()])).collect();
                j.set("options", Json::Arr(options));
            }
            Kind::Theme => {
                let choices = theme::choices(&shared.settings_dir, &w.prefs.theme);
                let options = choices.into_iter().map(|(v, t)| Json::Arr(vec![v.into(), t.into()])).collect();
                j.set("options", Json::Arr(options));
            }
            _ => {}
        }
        if def.readonly() || (!run_open && RUN_FIELDS.contains(&def.id())) {
            j.set("readonly", true);
        }
        if let Some(h) = def.help_key() {
            j.set("help", lang::help(h));
            if let Some(url) = help::wiki_url(h) {
                j.set("wiki", url);
            }
        }
        fields.set(def.id(), j);
    }
    let advanced: Vec<Json> = TABLE.iter().map(|e| Json::from(e.name)).collect();
    Json::obj()
        .with("pages", Json::Arr(pages(&shared.settings_dir, st.hosted, run_open)))
        .with("fields", fields)
        .with("advanced", Json::Arr(advanced))
        .with("trackers_status", trackers_status(shared))
        .with("ipfilter_status", ipfilter_status(shared))
}

/// The status line of the IP filter's list from an address (D75, `IpFilterList::status`):
/// empty while that is switched off, and hosted (the host's own list).
pub(crate) fn ipfilter_status(shared: &Shared) -> String {
    shared.ip_list().map_or(String::new(), |l| l.status())
}

/// `POST /pa/prefs/ipfilter/update`, the page's Update Now (D75): fetches the block list again
/// from the address stored in the preferences, as [`update_trackers`] does the tracker list;
/// nothing is taken from the request. Hosted, refused (the host's own list). Returns the
/// status line.
pub(crate) fn update_ip_filter(shared: &Shared) -> Result<String, &'static str> {
    let mut st = shared.lock();
    match shared.ip_list() {
        Some(mut l) => l.refresh_now(),
        None => return Err(HOSTED_NO_IPFILTER_URL),
    }
    // starts the fetch now rather than at the next status tick
    shared.ip_list_tick(&mut st);
    Ok(ipfilter_status(shared))
}

/// The status line of the tracker list fetched from the web (`TrackerList::status`): empty
/// while that is switched off, and hosted (the host's own list).
pub(crate) fn trackers_status(shared: &Shared) -> String {
    shared.trackers().map_or(String::new(), |t| t.status())
}

/// `POST /pa/prefs/trackers/update`, the page's Update Now: fetches the list again from the
/// address stored in the preferences. It takes nothing from the request, so it needs no
/// password: the address is changed only through [`commit`], with the current one. Hosted,
/// refused (the host's own list). Returns the status line.
pub(crate) fn update_trackers(shared: &Shared) -> Result<String, &'static str> {
    let mut st = shared.lock();
    match shared.trackers() {
        Some(mut t) => t.refresh_now(),
        None => return Err(HOSTED_NO_TRACKERS),
    }
    // starts the fetch now rather than at the next status tick
    shared.trackers_tick(&mut st);
    Ok(trackers_status(shared))
}

// ---- applying a POST -----------------------------------------------------------------------

/// A refused change: the field and why.
pub(crate) type Refusal = (String, String);

/// One parsed change.
pub(crate) type Change = (Def, Value);

/// Why the alternative speed limits are refused while a desktop client hosts the Web UI (it
/// has no such mode): `POST /pa/prefs` answers 400 with it, `POST /pa/session/limits` and the
/// shim's `toggleSpeedLimitsMode` / `setSpeedLimitsMode` 409.
pub(crate) const HOSTED_NO_ALT: &str =
    "alternative speed limits are not available: a desktop client hosts the Web UI";

/// Why the trackers added to new downloads (D66) are refused while a desktop client hosts the
/// Web UI: the list is the host's own, set in its Preferences; `POST /pa/prefs` answers 400
/// with it, `POST /pa/prefs/trackers/update` 409.
pub(crate) const HOSTED_NO_TRACKERS: &str =
    "the trackers for new downloads are set in the desktop client that hosts the Web UI";

/// The four tracker preferences (D66), in the page's order.
const TRACKER_FIELDS: [&str; 4] = [
    pref_pages::TRACKERS_APPEND,
    pref_pages::TRACKERS_LIST,
    pref_pages::TRACKERS_FROM_URL,
    pref_pages::TRACKERS_URL,
];

/// Why the IP filter's list from an address (D75) is refused while a desktop client hosts the
/// Web UI: the address and its switch are the host's own, set in its Preferences, as the
/// trackers' list (D66) is; `POST /pa/prefs` answers 400 with it,
/// `POST /pa/prefs/ipfilter/update` 409.
pub(crate) const HOSTED_NO_IPFILTER_URL: &str =
    "the IP filter's list address is set in the desktop client that hosts the Web UI";

/// The IP filter's list from an address (D75): its switch and the address.
const IPFILTER_URL_FIELDS: [&str; 2] = [pref_pages::IPFILTER_FROM_URL, pref_pages::IPFILTER_URL];

/// The addresses the daemon fetches lists from (D75's block list, D66's trackers): a
/// subscription's user name, password or PIN may be part of them, so [`model`] sends them as
/// [`shown_address`] gives them, and [`keep_addresses`] reads that form sent back as no change.
const ADDRESS_FIELDS: [&str; 2] = [pref_pages::IPFILTER_URL, pref_pages::TRACKERS_URL];

/// A stored list address as a page gets it: without the user name and password before the host,
/// the query and the fragment (`ip_filter_list::redact`, the Logger's form); empty stays empty.
fn shown_address(url: &str) -> String {
    if url.trim().is_empty() { String::new() } else { ip_filter_list::redact(url) }
}

/// A list address sent back as [`shown_address`] showed it (a client that sends every field) is
/// the stored one: kept, and no change (no current password asked). Any other value replaces
/// it; an empty one clears it.
fn keep_addresses(w: &mut Working, st: &State) {
    for (new, stored) in [
        (&mut w.prefs.ip_filter_url, &st.prefs.ip_filter_url),
        (&mut w.prefs.trackers_url, &st.prefs.trackers_url),
    ] {
        if *new != *stored && !new.is_empty() && *new == shown_address(stored) {
            new.clone_from(stored);
        }
    }
}

/// Why the Run Program lines (D67) are refused while a desktop client hosts the Web UI: the
/// host starts the programs and owns the setting; `POST /pa/prefs` answers 400 with it.
pub(crate) const HOSTED_NO_RUN: &str =
    "the programs to run are set in the desktop client that hosts the Web UI";

/// The four Run Program preferences (D67), in the page's order. Whoever changes them runs
/// programs under the daemon's account, so a request changes them only when its [`RunGate`] is
/// open and it carries the current password ([`SECURITY_FIELDS`]); nothing else does at all:
/// not the qBittorrent shim, not a remote GUI, not nox itself.
pub(crate) const RUN_FIELDS: [&str; 4] =
    [pref_pages::RUN_ADDED_ON, pref_pages::RUN_ADDED, pref_pages::RUN_FINISHED_ON, pref_pages::RUN_FINISHED];

/// Whether a request may change the Run Program lines (D67). qBittorrent's CVE-2023-30801 was
/// a Web UI login turned into code execution through "Run external program"; here a login from
/// elsewhere changes nothing of it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RunGate {
    /// A browser on this computer (`api::is_local`: a loopback peer, not passed on by a
    /// proxy), or the daemon was started with `--webui-remote-run-program`. The current
    /// password is still needed.
    Open,
    /// Anything else: refused before the password is looked at, so the answer says nothing
    /// about it and the session stays. The client's address, for the Logger line; `None` for
    /// a change that came from no page request (the shim, nox's own).
    Closed(Option<std::net::IpAddr>),
}

impl RunGate {
    /// The gate of a `/pa/` request.
    pub(crate) fn of(shared: &Shared, head: &super::http::Head, peer: std::net::IpAddr) -> RunGate {
        if shared.remote_run_program || super::api::is_local(head, peer) {
            RunGate::Open
        } else {
            RunGate::Closed(Some(super::api::client_ip(head, peer)))
        }
    }
}

/// The fields that guard the Web UI: changed only with the current password (SEC-03). The
/// trackers added to new downloads (D66) are among them: they make the daemon fetch a file
/// from the net and announce every public download to the addresses named. So is the IP
/// filter's list from an address (D75): it makes the daemon fetch a file from the net and
/// decides whom it talks to. So are the Run Program lines (D67, [`RUN_FIELDS`]), behind their
/// [`RunGate`] as well.
const SECURITY_FIELDS: [&str; 20] = [
    "webui.username",
    "webui.bypass_local_auth",
    "webui.api_key_enabled",
    "webui.bypass_auth_subnets",
    "webui.host_header_validation",
    "webui.domain_list",
    "webui.csrf",
    "webui.max_auth_fail_count",
    "webui.ban_duration",
    "webui.session_timeout",
    pref_pages::TRACKERS_APPEND,
    pref_pages::TRACKERS_LIST,
    pref_pages::TRACKERS_FROM_URL,
    pref_pages::TRACKERS_URL,
    pref_pages::IPFILTER_FROM_URL,
    pref_pages::IPFILTER_URL,
    pref_pages::RUN_ADDED_ON,
    pref_pages::RUN_ADDED,
    pref_pages::RUN_FINISHED_ON,
    pref_pages::RUN_FINISHED,
];

/// Whether `w` changes the password, the API key or a [`SECURITY_FIELDS`] value.
fn security_changed(w: &Working, old: &Working) -> bool {
    w.new_password.is_some()
        || w.new_api_key.is_some()
        || SECURITY_FIELDS.iter().filter_map(|id| find(id)).any(|d| d.get(w) != d.get(old))
}

/// Parses one id and value.
pub(crate) fn parse_one(id: &str, v: &Json) -> Result<Change, Refusal> {
    let def = find(id).ok_or_else(|| (id.to_owned(), "unknown preference".to_owned()))?;
    if def.readonly() {
        return Err((id.to_owned(), "read-only".to_owned()));
    }
    let text = text_of(v).ok_or_else(|| (id.to_owned(), format!("unexpected value {}", v.to_json())))?;
    let value = def.parse(&text).map_err(|why| (id.to_owned(), why))?;
    Ok((def, value))
}

/// Parses a `POST /pa/prefs` body: a partial object of ids to values. Stops at the first bad
/// field.
pub(crate) fn parse(body: &Json) -> Result<Vec<Change>, Refusal> {
    let mut changes: Vec<Change> =
        body.entries().iter().map(|(k, v)| parse_one(k, v)).collect::<Result<_, _>>()?;
    // in table order, whatever the body's: a composite's box applies before its number
    changes.sort_by_key(|(def, _)| all().position(|d| d.id() == def.id()));
    Ok(changes)
}

/// The Win32 dialog's checks across fields.
pub(crate) fn validate(w: &Working) -> Result<(), Refusal> {
    let refuse = |id: &str, why: &str| Err((id.to_owned(), why.to_owned()));
    if w.settings.proxy_type != ProxyType::None {
        if w.settings.proxy_hostname.trim().is_empty() {
            return refuse("proxy_hostname", tr!(PrefNoProxyHost));
        }
        if w.settings.proxy_port == 0 {
            return refuse("proxy_port", tr!(PrefNoProxyPort));
        }
    }
    if let Err(why) = network::check_rule(&w.settings.network_rule) {
        return refuse("net.network_range", &why);
    }
    if !theme::valid_id(&w.prefs.theme) {
        return refuse("webui.theme", tr!(PrefBadTheme));
    }
    if w.prefs.save_path.trim().is_empty() {
        return refuse("dir.download", tr!(PrefNoDownloadDir));
    }
    if w.prefs.move_completed && w.prefs.completed_dir.is_empty() {
        return refuse("dir.completed", tr!(PrefNoCompletedDir));
    }
    let tf = &w.prefs.torrent_folders;
    if tf.store && tf.store_dir.is_empty() {
        return refuse("dir.torrents", tr!(PrefNoStoreDir));
    }
    if tf.store && tf.move_finished && tf.finished_dir.is_empty() {
        return refuse("dir.finished_torrents", tr!(PrefNoFinishedDir));
    }
    if tf.autoload && tf.autoload_dir.is_empty() {
        return refuse("dir.autoload_dir", tr!(PrefNoAutoloadDir));
    }
    if w.username.is_empty() {
        return refuse("webui.username", tr!(PrefNoUsername));
    }
    if w.new_api_key.as_ref().is_some_and(|k| k.chars().count() < auth::API_KEY_MIN_LEN) {
        return refuse("webui.api_key", &tr!(PrefApiKeyShort, count = auth::API_KEY_MIN_LEN));
    }
    if let Err(entry) = auth::parse_subnets(&w.prefs.auth_subnets) {
        return refuse("webui.bypass_auth_subnets", &tr!(PrefBadSubnet, entry = entry));
    }
    // `frontend::pref_pages::check`'s rules (D66): a typed line that is no tracker address
    // would be dropped silently otherwise, and a list is fetched over https only
    if w.prefs.trackers_append && tracker_list::parse(&w.prefs.trackers_list).invalid > 0 {
        return refuse(pref_pages::TRACKERS_LIST, tr!(TrackersErrLine));
    }
    if w.prefs.trackers_from_url && !tracker_list::is_list_url(&w.prefs.trackers_url) {
        return refuse(pref_pages::TRACKERS_URL, tr!(TrackersErrHttps));
    }
    // the IP filter's list from an address (D75), https only while the filter takes it (the
    // address is greyed while the filter is off)
    let p = &w.prefs;
    if p.ip_filter && p.ip_filter_from_url && !tracker_list::is_list_url(&p.ip_filter_url) {
        return refuse(pref_pages::IPFILTER_URL, tr!(TrackersErrHttps));
    }
    // a Run Program line that is switched on must be one that can be started (D67)
    for (on, line, id) in [
        (w.prefs.run_added_on, &w.prefs.run_added, pref_pages::RUN_ADDED),
        (w.prefs.run_finished_on, &w.prefs.run_finished, pref_pages::RUN_FINISHED),
    ] {
        if let Some(why) = pref_pages::run_command_error(line).filter(|_| on) {
            return refuse(id, &why);
        }
    }
    Ok(())
}

/// The folders and files a change names, against the path policy (SEC-05). Only paths that
/// change are checked, so a relative folder given on the command line keeps working.
fn check_paths(w: &Working, old: &Working, policy: &PathPolicy) -> Result<(), Refusal> {
    let paths = |w: &Working| {
        let tf = &w.prefs.torrent_folders;
        [
            ("dir.download", w.prefs.save_path.clone()),
            ("dir.completed", w.prefs.completed_dir.clone()),
            ("dir.torrents", tf.store_dir.clone()),
            ("dir.finished_torrents", tf.finished_dir.clone()),
            ("dir.autoload_dir", tf.autoload_dir.clone()),
            ("ipfilter.path", w.prefs.ip_filter_path.clone()),
        ]
    };
    for ((id, new), (_, was)) in paths(w).into_iter().zip(paths(old)) {
        if new != was && !new.trim().is_empty() {
            policy.check(&new).map_err(|why| (id.to_owned(), why.to_owned()))?;
        }
    }
    Ok(())
}

/// A block list read ahead of a commit, without the State lock: the file and what reading it
/// gave.
type Read = Option<(PathBuf, io::Result<IpFilterLoad>)>;

/// The IP filter's on switch and the block list file it loads in `w`: the path field's (or
/// `ipfilter.dat`), or with the list from an address (D75) the copy kept of that address's list,
/// `None` while none is (`ip_filter_list::list_file`).
fn ip_filter(shared: &Shared, w: &Working) -> (bool, Option<PathBuf>) {
    (w.prefs.ip_filter, ip_filter_list::list_file(&w.prefs.gui(), &shared.settings_dir))
}

/// What decides the IP filter's rules in `w`: [`ip_filter`], and whether and where from the
/// list comes from an address.
type IpSource = (bool, Option<PathBuf>, bool, String);

fn ip_source(shared: &Shared, w: &Working) -> IpSource {
    let (on, file) = ip_filter(shared, w);
    (on, file, w.prefs.ip_filter_from_url, w.prefs.ip_filter_url.trim().to_owned())
}

/// The block list committing `changes` onto `st` would read: the IP filter turned on or pointed
/// at another list by changes that pass the checks (a path the policy refuses is never read).
fn ip_filter_to_read(shared: &Shared, st: &State, changes: &[Change]) -> Option<PathBuf> {
    let mut w = Working::from_state(st);
    for (def, v) in changes {
        def.set(&mut w, v.clone());
    }
    validate(&w).ok()?;
    let old = Working::from_state(st);
    check_paths(&w, &old, &shared.path_policy).ok()?;
    let (on, file) = ip_filter(shared, &w);
    let file = file.filter(|_| on)?;
    (ip_source(shared, &w) != ip_source(shared, &old)).then_some(file)
}

/// The request that sent changes to [`commit`]: its session (kept when the password changes,
/// ended by a wrong current password) and its client, whose ban a wrong current password
/// counts towards, as a failed login does (bug hunt 2026-11-21, 11: the bypass sessions come
/// back with the next request, so ending the session alone stops nobody).
pub(crate) struct Asker<'a> {
    session: &'a str,
    ip: IpAddr,
    current: Current,
}

/// The current password a request gave (`webui.current_password`).
enum Current {
    Absent,
    /// Not checked yet: it is checked once the change is found to need it, and only then
    /// counts (a change that needs none, or one the Run Program gate refuses, leaves it).
    Unchecked(String),
    /// Right, against this hash (a password changed since does not count).
    Right(PasswordHash),
    /// Wrong, or the client was banned meanwhile.
    Wrong,
}

impl<'a> Asker<'a> {
    pub(crate) fn new(session: &'a str, ip: IpAddr, changes: &[Change]) -> Asker<'a> {
        let given = changes
            .iter()
            .find(|(d, _)| d.id() == "webui.current_password")
            .map(|(_, v)| string(v.clone()))
            .filter(|p| !p.is_empty());
        Asker { session, ip, current: given.map_or(Current::Absent, Current::Unchecked) }
    }

    /// Checks the password, outside the State lock.
    fn check(&mut self, shared: &Shared) {
        if let Current::Unchecked(p) = &self.current {
            let found = super::api::check_current_password(shared, p, self.ip);
            self.current = found.map_or(Current::Wrong, Current::Right);
        }
    }
}

/// The hashes of a new Web UI password and API key, each with the plain text it was made of,
/// made outside the State lock ([`Stop::Hash`]): PBKDF2 is slow on purpose, and every client
/// waits for the lock. One request's commits share them, so each is made once.
#[derive(Default)]
pub(crate) struct NewHashes {
    password: Option<(String, PasswordHash)>,
    api_key: Option<(String, PasswordHash)>,
}

impl NewHashes {
    /// Makes the hashes of `password` and `api_key` not made yet.
    fn make(&mut self, password: Option<String>, api_key: Option<String>, iterations: u32) {
        for (made, plain) in [(&mut self.password, password), (&mut self.api_key, api_key)] {
            if let Some(p) = plain.filter(|p| made.as_ref().is_none_or(|(q, _)| q != p)) {
                let hash = auth::new_hash(&p, iterations);
                *made = Some((p, hash));
            }
        }
    }

    /// The hash made of `plain` in `made`: `Ok(None)` without a plain text, `Err` when it is not
    /// made yet.
    fn of(made: &Option<(String, PasswordHash)>, plain: Option<&str>) -> Result<Option<PasswordHash>, ()> {
        let Some(plain) = plain else { return Ok(None) };
        made.as_ref().filter(|(p, _)| p == plain).map(|(_, h)| Some(h.clone())).ok_or(())
    }
}

/// Why [`commit_read`] stopped.
enum Stop {
    Refused(Refusal),
    /// The change needs the current password, which is not checked yet: nothing was changed.
    Check,
    /// A new password and/or API key (these plain texts) are not hashed yet
    /// ([`NewHashes::make`]): nothing was changed.
    Hash(Option<String>, Option<String>),
}

impl From<Refusal> for Stop {
    fn from(r: Refusal) -> Stop {
        Stop::Refused(r)
    }
}

/// [`commit`] for a request: a block list the changes turn on is read without the State lock,
/// which every request and the alert loop wait for (SEC-05: a big list takes a while), with
/// `read`; so is the current password checked when the change needs it, and a new password or
/// API key hashed (PBKDF2 is slow on purpose); then the changes are committed under the lock.
/// When the preferences changed in the meantime so that another file is wanted, that one is
/// read the same way.
///
/// `run` is the request's [`RunGate`]: the only way the Run Program lines (D67) change. `ip` is
/// the request's client ([`super::api::client_ip`]).
pub(crate) fn commit_request(
    shared: &Shared,
    changes: &[Change],
    from: &str,
    ip: IpAddr,
    run: RunGate,
    read: impl Fn(&Path) -> io::Result<IpFilterLoad>,
) -> Result<(), Refusal> {
    let mut asker = Asker::new(from, ip, changes);
    let mut hashes = NewHashes::default();
    let mut done: Read = None;
    let mut reads = 0;
    loop {
        let mut st = shared.lock();
        let wanted = ip_filter_to_read(shared, &st, changes);
        // after three reads that the preferences outran, the file is read under the lock
        if reads < 3 && wanted.as_ref() != done.as_ref().map(|(f, _)| f) {
            drop(st);
            reads += 1;
            done = wanted.map(|f| {
                let r = read(&f);
                (f, r)
            });
            continue;
        }
        match commit_read(shared, &mut st, changes, Some(&asker), run, &mut done, &hashes) {
            Ok(()) => {
                confirm(shared, &mut st);
                return Ok(());
            }
            Err(Stop::Refused(r)) => return Err(r),
            // checked once: the next round finds it right or wrong
            Err(Stop::Check) => {
                drop(st);
                asker.check(shared);
            }
            // hashed once: the next round finds them
            Err(Stop::Hash(password, api_key)) => {
                drop(st);
                hashes.make(password, api_key, shared.iterations);
            }
        }
    }
}

/// [`commit`] for a request that holds no lock (the qBittorrent shim's `setPreferences`): the
/// State lock is taken for the commit and let go while the current password is checked and a
/// new password or API key is hashed, which then commit in a later round. `asker` and `hashes`
/// serve every commit of one request, so the password is checked (and a wrong one counted)
/// once and each hash made once; a password that changed in between counts as wrong.
pub(crate) fn commit_asked(
    shared: &Shared,
    changes: &[Change],
    asker: &mut Asker<'_>,
    hashes: &mut NewHashes,
) -> Result<(), Refusal> {
    loop {
        let mut st = shared.lock();
        match commit_read(shared, &mut st, changes, Some(asker), RunGate::Closed(None), &mut None, hashes) {
            Ok(()) => return Ok(()),
            Err(Stop::Refused(r)) => return Err(r),
            Err(Stop::Check) => {
                drop(st);
                asker.check(shared);
            }
            Err(Stop::Hash(password, api_key)) => {
                drop(st);
                hashes.make(password, api_key, shared.iterations);
            }
        }
    }
}

/// The Web UI's Preferences page saved, or a remote window's Preferences sent OK or Apply: the
/// user saw the settings and confirmed them (D70). After the settings reached the engine, so a
/// held start goes on with them: the lines this build cannot read go, this version's reading is
/// stamped, the web fetches and then the engine's transfers go on. Nothing else confirms: not
/// the qBittorrent API (automation calls it), not a remote window's scheduler, not the limits.
/// Hosted, the host confirms in its own Preferences.
pub(crate) fn confirm(shared: &Shared, st: &mut State) {
    if st.hosted {
        return;
    }
    rookparcel_frontend::guard::confirm(&mut st.options);
    shared.save(st);
    shared.handle.hold_transfers(false);
}

/// Applies parsed changes to a working copy of `st`, checks it, and commits: engine settings
/// through `apply_settings` (with the global limits of the mode in force), the IP filter loaded
/// or cleared, peer exchange toggled on every torrent, a new password hashed, the folders
/// handed to the owner's `MoveCompleted`, and everything stored in the state file. A block list
/// is read here, and a new password hashed, under the State lock the caller holds: this is for
/// nox's own changes (the state file at start, before the server accepts anyone; a remote GUI's
/// engine settings; the limits), and requests go through [`commit_request`] or
/// [`commit_asked`], which do the slow parts without the lock.
///
/// A request changes the login and the Web UI's security settings only with the current
/// password; a wrong one ends its session and counts towards its client's ban (SEC-03), and
/// one [`Asker`] checks it once, however many commits it is given to. A new password ends every
/// other login: the other sessions, their WebSockets and the remote connections (SEC-24).
///
/// The Run Program lines (D67) never change here: the gate is closed for everything that is
/// not a page's request ([`commit_request`]), whoever calls and with whatever password.
pub(crate) fn commit(shared: &Shared, st: &mut State, changes: &[Change]) -> Result<(), Refusal> {
    let mut hashes = NewHashes::default();
    loop {
        match commit_read(shared, st, changes, None, RunGate::Closed(None), &mut None, &hashes) {
            Ok(()) => return Ok(()),
            Err(Stop::Refused(r)) => return Err(r),
            Err(Stop::Check) => unreachable!("only a request's changes need the current password"),
            Err(Stop::Hash(password, api_key)) => hashes.make(password, api_key, shared.iterations),
        }
    }
}

/// [`commit`], with the block list `read` already when it is the file wanted (taken from
/// `read` when used) and the new password's and API key's `hashes` when made. Changes nothing
/// in `st` before the current password is known to be needed and right and the hashes are
/// made; [`Stop::Check`] when the password is needed and not checked yet, [`Stop::Hash`] when a
/// hash is not made yet.
fn commit_read(
    shared: &Shared,
    st: &mut State,
    changes: &[Change],
    from: Option<&Asker<'_>>,
    run: RunGate,
    read: &mut Read,
    hashes: &NewHashes,
) -> Result<(), Stop> {
    let keep = from.map(|a| a.session);
    let mut w = Working::from_state(st);
    for (def, v) in changes {
        def.set(&mut w, v.clone());
    }
    keep_addresses(&mut w, st);
    // Run Program (D67) before anything else is looked at, the current password above all: a
    // request from elsewhere learns nothing here, not even whether its password was right
    let now = Working::from_state(st);
    if let Some(id) = RUN_FIELDS.iter().find(|id| find(id).is_some_and(|d| d.get(&w) != d.get(&now))) {
        if st.hosted {
            // the host's own programs: never changed from a browser
            return Err(((*id).to_owned(), HOSTED_NO_RUN.to_owned()).into());
        }
        if let RunGate::Closed(client) = run {
            let line = match client {
                // a loopback client here is a proxy on this computer that passed the request on
                // (`X-Real-IP` or `Forwarded` alone, or `X-Forwarded-For` naming loopback)
                Some(addr) if addr.is_loopback() => {
                    format!("Run Program: a change from {addr} was refused (passed on by a proxy)")
                }
                Some(addr) => {
                    format!("Run Program: a change from {addr} was refused (not this computer)")
                }
                None => {
                    "Run Program: a change was refused (not a request of the Preferences page)".to_owned()
                }
            };
            println!("* {line}");
            st.push_log(None, line, unix_now());
            return Err(((*id).to_owned(), tr!(RunErrRemote).to_owned()).into());
        }
    }
    validate(&w)?;
    if st.hosted && w.prefs.alt_enabled {
        return Err(("alt_enabled".to_owned(), HOSTED_NO_ALT.to_owned()).into());
    }
    // the typed tracker list in its own form (as the preferences file gives it back), so the
    // same list sent again is no change
    w.prefs.trackers_list = tracker_list::to_text(&tracker_list::parse(&w.prefs.trackers_list).tiers);
    let old = Working::from_state(st);
    if st.hosted {
        // the host's own lists (D66, D75): never changed from a browser
        let changed = |ids: &[&'static str]| {
            ids.iter().copied().find(|id| find(id).is_some_and(|d| d.get(&w) != d.get(&old)))
        };
        if let Some(id) = changed(&TRACKER_FIELDS) {
            return Err((id.to_owned(), HOSTED_NO_TRACKERS.to_owned()).into());
        }
        if let Some(id) = changed(&IPFILTER_URL_FIELDS) {
            return Err((id.to_owned(), HOSTED_NO_IPFILTER_URL.to_owned()).into());
        }
    }
    if let Some(asker) = from.filter(|_| security_changed(&w, &old)) {
        let need =
            || Stop::Refused(("webui.current_password".to_owned(), tr!(PrefNeedCurrentPassword).to_owned()));
        match &asker.current {
            Current::Right(hash) if *hash == st.password => {}
            Current::Unchecked(_) => return Err(Stop::Check),
            Current::Absent => return Err(need()),
            // a stolen session cannot guess the password here: one wrong try ends it, and its
            // check ([`Asker::check`]) counted it towards the client's ban, which stops the
            // bypass sessions too
            Current::Right(_) | Current::Wrong => {
                st.sessions.remove(asker.session);
                return Err(need());
            }
        }
    }
    check_paths(&w, &old, &shared.path_policy)?;
    // the new password and API key are hashed without the lock, once the change may be made
    let new_password = NewHashes::of(&hashes.password, w.new_password.as_deref());
    let new_api_key = NewHashes::of(&hashes.api_key, w.new_api_key.as_deref());
    let (Ok(new_password), Ok(new_api_key)) = (new_password, new_api_key) else {
        return Err(Stop::Hash(w.new_password.clone(), w.new_api_key.clone()));
    };

    // the IP filter first: a block list file that does not load refuses the whole change
    // (hosted: the host loads its own; nothing here goes to the engine)
    if !st.hosted && ip_source(shared, &w) != ip_source(shared, &old) {
        let (on, file) = ip_filter(shared, &w);
        let from_url = w.prefs.ip_filter_from_url;
        let list = file.filter(|_| on).map(|file| {
            let r = match read.take() {
                Some((f, r)) if f == file => r,
                _ => IpFilter::load(&file),
            };
            (file, r)
        });
        if let (Some((file, Err(e))), false) = (&list, from_url) {
            return Err((pref_pages::IPFILTER_PATH.to_owned(), format!("{}: {e}", file.display())).into());
        }
        // the list from an address follows: a new address is fetched at the next tick (D75);
        // the copy kept of another stays on disk, unused, until a list of this one is saved
        shared.configure_ip_list(&w.prefs);
        let now = unix_now();
        match list {
            Some((file, Ok(l))) => {
                shared.handle.set_ip_filter(l.filter);
                st.push_log(None, tr!(LogIpFilterRules, count = l.rules, path = file.display()), now)
            }
            // a kept copy that no longer loads: the rules in force stay until the next fetch
            Some((file, Err(e))) => {
                st.push_log(None, tr!(LogIpFilterKept, path = file.display(), error = e), now)
            }
            // nothing fetched from the address yet: the rules in force stay, the fetch brings
            // the new ones (said in the Logger)
            None if on => {
                if let Some(line) = ip_filter_list::not_loaded_line(&w.prefs.gui()) {
                    st.push_log(None, line, now);
                }
            }
            None => {
                shared.handle.set_ip_filter(IpFilter::new());
                st.push_log(None, tr!(LogIpFilterOff).to_owned(), now);
            }
        }
    }
    if !st.hosted && w.prefs.pex != old.prefs.pex {
        for id in st.torrents.keys() {
            let t = shared.handle.torrent(*id);
            if w.prefs.pex {
                t.unset_flags(TorrentFlags::DISABLE_PEX);
            } else {
                t.set_flags(TorrentFlags::DISABLE_PEX);
            }
        }
    }
    let folders = |p: &Prefs| {
        (
            p.move_completed,
            p.completed_dir.clone(),
            p.move_only_default,
            p.move_add_label,
            p.save_path.clone(),
        )
    };
    if folders(&w.prefs) != folders(&old.prefs) {
        st.gui_version += 1;
    }
    w.prefs.dl_limit = w.settings.download_rate_limit;
    w.prefs.up_limit = w.settings.upload_rate_limit;
    let limits = |p: &Prefs| (p.dl_limit, p.up_limit, p.alt_dl_limit, p.alt_up_limit, p.alt_enabled);
    let engine_changed = w.settings != old.settings
        || limits(&w.prefs) != limits(&old.prefs)
        || w.prefs.schedule != old.prefs.schedule;
    st.prefs = w.prefs.clone();
    st.settings = w.settings.clone();
    if engine_changed {
        // sets the limits of the mode in force and sends every setting
        shared.apply_limits(st);
    }
    st.username = w.username.clone();
    if let Some(hash) = new_password {
        st.password = hash;
        // a WebSocket ends when its session is gone, at the next status tick
        st.sessions.retain(|k, _| Some(k.as_str()) == keep);
        shared.end_remotes();
    }
    if let Some(hash) = new_api_key {
        st.set_api_key(Some(hash));
    }
    if !w.prefs.api_key_enabled {
        st.sessions.remove(super::api::APIKEY_SESSION);
    }
    if w.prefs.auth_subnets != old.prefs.auth_subnets {
        st.sessions.remove(super::api::SUBNET_SESSION);
    }
    if st.hosted {
        // the host stores the login (the hash; a login it hands back is taken as it is)
        st.options.set("webui.username", &st.username);
        st.options.set("webui.password", st.password.to_string());
    }
    let changed: Vec<&'static str> =
        TABLE.iter().filter(|e| e.get(&w.settings) != e.get(&old.settings)).map(|e| e.name).collect();
    store(st, &changed);
    shared.save(st);
    // the trackers added to new downloads (D66) follow at once: the engine has the new list
    // before the next torrent is added, and a new address is fetched (hosted: the host's list)
    shared.configure_trackers(st);
    shared.trackers_tick(st);
    // so does the IP filter's list from an address (D75): a new address is fetched now
    shared.configure_ip_list(&st.prefs);
    shared.ip_list_tick(st);
    // the server's texts switch now; the page reloads for its own (`lang.js`); hosted, the
    // host's language is the process's and the host switches it
    if !st.hosted && w.prefs.language != old.prefs.language {
        for p in lang::init(&w.prefs.language) {
            st.push_log(None, p, unix_now());
        }
    }
    Ok(())
}

/// A remote GUI's `apply_settings`: the engine settings that differ from the current ones,
/// committed like the Preferences dialog's changes (checked, applied, stored). Settings
/// without a row in the table, and read-only ones, are not taken.
pub(crate) fn commit_settings(shared: &Shared, st: &mut State, new: &Settings) -> Result<(), Refusal> {
    let changes: Vec<Change> = TABLE
        .iter()
        .filter(|e| e.get(new) != e.get(&st.settings))
        .map(|e| (Def::Setting(e), e.get(new)))
        .filter(|(d, _)| !d.readonly())
        .collect();
    if changes.is_empty() {
        return Ok(());
    }
    commit(shared, st, &changes)
}

/// Writes the preferences into `State::options` (the frontend's keys): the engine settings
/// under their keys (`settings::pref_key`: `lt.<name>`, Rookparcel's own `net.` rows by their
/// names) with the normal rate limits (the ones that differ from the defaults, as the Win32
/// client stores them, plus every row `changed` here or stored before, so a value set back to
/// its default still wins over the command line at the next start), the `GuiOptions` keys
/// (without the desktop-only tray ones), and the Web UI's own as `webui.<name>`. The old
/// `lt.net.*` keys ([`migrate_keys`]) are written beside the new ones, so nox 1.2.54 and older,
/// which read only those, keep the network rule after a downgrade; hosted, they are dropped.
fn store(st: &mut State, changed: &[&'static str]) {
    let mut normal = st.settings.clone();
    normal.download_rate_limit = st.prefs.dl_limit;
    normal.upload_rate_limit = st.prefs.up_limit;
    let kept: Vec<&'static str> = TABLE
        .iter()
        .filter(|e| !e.name.starts_with("bt.") && st.options.get(&settings::pref_key(e.name)).is_some())
        .map(|e| e.name)
        .collect();
    settings::store(&mut st.options, &normal);
    for name in kept.iter().chain(changed) {
        if let Some(e) = settings::find(name).filter(|e| !e.name.starts_with("bt.")) {
            let key = settings::pref_key(e.name);
            // a line this build cannot read stays until the user confirms (D70) or changes it
            if !changed.contains(name) && st.options.get(&key).is_some_and(|v| e.parse(v).is_err()) {
                continue;
            }
            st.options.set(&key, e.get(&normal));
        }
    }
    for e in TABLE.iter() {
        let Some(old) = legacy_key(e.name) else { continue };
        match st.options.get(&settings::pref_key(e.name)).map(str::to_owned) {
            Some(v) if !st.hosted => st.options.set(&old, v),
            _ => st.options.remove(&old),
        }
    }
    if st.hosted {
        // the host's own options (tray, updates) stay as they are; no alternative limits
        let mut g = GuiOptions::load(&st.options, &st.default_save_path);
        st.prefs.apply_to(&mut g);
        g.store(&mut st.options);
    } else {
        st.prefs.gui().store(&mut st.options);
        for k in ["ui.close_to_tray", "ui.minimize_to_tray", "ui.balloons"] {
            st.options.remove(k);
        }
    }
    let p = &st.prefs;
    let o = &mut st.options;
    if !st.hosted {
        o.set("webui.alt_dl_limit", p.alt_dl_limit);
        o.set("webui.alt_up_limit", p.alt_up_limit);
        o.set_bool("webui.alt_enabled", p.alt_enabled);
    }
    o.set_bool("webui.bypass_local_auth", p.bypass_local_auth);
    o.set_bool("webui.api_key_enabled", p.api_key_enabled);
    o.set("webui.bypass_auth_subnets", &p.auth_subnets);
    match &st.api_key {
        Some(k) => o.set("webui.api_key", k.to_string()),
        None => o.remove("webui.api_key"),
    }
    o.set_bool("webui.csrf", p.csrf_protection);
    o.set_bool("webui.host_header_validation", p.host_header_validation);
    o.set("webui.domain_list", &p.domain_list);
    o.set("webui.max_auth_fail_count", p.max_auth_fail_count);
    o.set("webui.ban_duration", p.ban_duration);
    o.set("webui.session_timeout", p.session_timeout);
    o.set("webui.theme", &p.theme);
}

/// Reads the stored preferences (`State::options`) back into `st.settings` and `st.prefs`,
/// applying nothing: the engine settings from their keys, the `GuiOptions` keys and the
/// Web UI's `webui.*` numbers. nox lays them over the current values (the command line's), so
/// a value never stored keeps the command line's; hosted, they are the host's whole
/// configuration, over the defaults (a key the host removed is back at its default). Returns
/// whether an engine setting or a limit was read (nox applies them then).
pub(crate) fn read_back(st: &mut State) -> bool {
    let mut changed = false;
    if st.hosted {
        st.settings = settings::load(&st.options);
        st.prefs = Prefs::defaults(&st.default_save_path, &st.settings);
        st.prefs.set_gui(&GuiOptions::load(&st.options, &st.default_save_path));
    } else {
        let o = &st.options;
        for e in TABLE.iter().filter(|e| !e.name.starts_with("bt.")) {
            if let Some(v) = o.get(&settings::pref_key(e.name)) {
                changed |= e.set(&mut st.settings, v).is_ok();
            }
        }
        // the GuiOptions keys: the stored ones over the current values (the command line's)
        let mut filled = rookparcel_frontend::prefs::Prefs::default();
        st.prefs.gui().store(&mut filled);
        for (k, v) in o.entries() {
            filled.set(k, v);
        }
        let gui = GuiOptions::load(&filled, &st.default_save_path);
        st.prefs.set_gui(&gui);
    }
    st.prefs.dl_limit = st.settings.download_rate_limit;
    st.prefs.up_limit = st.settings.upload_rate_limit;
    let o = &st.options;
    let u32_of = |k: &str| o.get(k).and_then(|v| v.trim().parse::<u32>().ok());
    let u64_of = |k: &str| o.get(k).and_then(|v| v.trim().parse::<u64>().ok());
    let p = &st.prefs;
    let prefs = Prefs {
        alt_dl_limit: u32_of("webui.alt_dl_limit").unwrap_or(p.alt_dl_limit),
        alt_up_limit: u32_of("webui.alt_up_limit").unwrap_or(p.alt_up_limit),
        // hosted: no alternative limits (the host has none)
        alt_enabled: !st.hosted && o.get_bool("webui.alt_enabled", p.alt_enabled),
        bypass_local_auth: o.get_bool("webui.bypass_local_auth", p.bypass_local_auth),
        api_key_enabled: o.get_bool("webui.api_key_enabled", p.api_key_enabled),
        auth_subnets: o.get("webui.bypass_auth_subnets").map_or(p.auth_subnets.clone(), str::to_owned),
        csrf_protection: o.get_bool("webui.csrf", p.csrf_protection),
        host_header_validation: o.get_bool("webui.host_header_validation", p.host_header_validation),
        domain_list: o.get("webui.domain_list").map_or(p.domain_list.clone(), str::to_owned),
        max_auth_fail_count: u32_of("webui.max_auth_fail_count").unwrap_or(p.max_auth_fail_count),
        ban_duration: u64_of("webui.ban_duration").unwrap_or(p.ban_duration),
        session_timeout: u64_of("webui.session_timeout").unwrap_or(p.session_timeout),
        // a name that could not be set through the page is left out (an edited file)
        theme: o.get("webui.theme").filter(|t| theme::valid_id(t)).map_or(p.theme.clone(), str::to_owned),
        ..p.clone()
    };
    changed |=
        prefs.alt_enabled || prefs.alt_dl_limit != p.alt_dl_limit || prefs.alt_up_limit != p.alt_up_limit;
    st.prefs = prefs;
    // the API key's hash, stored like the password's (hosted: the host's)
    let key = st.options.get("webui.api_key").and_then(PasswordHash::parse);
    st.set_api_key(key);
    changed
}

/// The engine settings a state file's `options` hold (what the Web UI's Preferences changed),
/// put over `s`: the same entries [`read_back`] takes, read before the engine starts, so a
/// stored "off" (UPnP, NAT-PMP, DHT, LSD, a listen port) holds from its first moment instead of
/// after the Web UI's start.
pub(crate) fn stored_engine_settings(stored: &Json, s: &mut Settings) {
    let Some(o) = stored.get("options") else {
        return;
    };
    let get = |key: &str| o.get(key).and_then(Json::as_str);
    for e in TABLE.iter().filter(|e| !e.name.starts_with("bt.")) {
        // the old key when the row's own is not there ([`migrate_keys`])
        let v = get(&settings::pref_key(e.name)).or_else(|| legacy_key(e.name).and_then(|k| get(&k)));
        if let Some(v) = v {
            let _ = e.set(s, v);
        }
    }
}

/// The key nox up to 1.2.54 stored a row under when it is not the row's own: `lt.<name>` for
/// Rookparcel's own `net.` rows (the network rule, D54), which `settings::pref_key` keeps by
/// their names.
fn legacy_key(name: &str) -> Option<String> {
    let old = format!("{}{name}", settings::PREF_PREFIX);
    (!name.starts_with("bt.") && old != settings::pref_key(name)).then_some(old)
}

/// At start (nox): a row stored under its old key ([`legacy_key`]) and not under its own is
/// taken under its own; [`store`] drops the old keys.
pub(crate) fn migrate_keys(o: &mut rookparcel_frontend::prefs::Prefs) {
    for e in TABLE {
        let Some(old) = legacy_key(e.name) else { continue };
        let key = settings::pref_key(e.name);
        if o.get(&key).is_some() {
            continue;
        }
        if let Some(v) = o.get(&old).map(str::to_owned) {
            o.set(&key, v);
        }
    }
}

/// At start (nox): [`read_back`], then re-applies the stored preferences onto the settings the
/// daemon was started with (so a stored value wins over the command line, a value never stored
/// keeps the command line's), picks a random listen port when asked, and loads the IP filter
/// unless the daemon loaded that same file already (`loaded_ip_filter`): the file, or the copy
/// kept of the list from an address (D75), whose list it configures first.
pub(crate) fn restore(shared: &Shared, st: &mut State, loaded_ip_filter: Option<&Path>) {
    let mut changed = read_back(st);
    if st.prefs.random_port {
        settings::set_listen_port(&mut st.settings, options::random_port());
        changed = true;
    }
    if changed {
        shared.apply_limits(st);
    }
    // the list from an address (D75) first
    shared.configure_ip_list(&st.prefs);
    // with the list from an address, the copy kept of it, so the filter is in force before any
    // fetch; none kept of this address (a copy of another is not its list): no rules until the
    // fetch brings them, which the Logger says
    let file = ip_filter_list::list_file(&st.prefs.gui(), &shared.settings_dir);
    if st.prefs.ip_filter {
        let Some(file) = file else {
            if let Some(line) = ip_filter_list::not_loaded_line(&st.prefs.gui()) {
                st.push_log(None, line, unix_now());
            }
            return;
        };
        if loaded_ip_filter != Some(file.as_path()) {
            match shared.handle.load_ip_filter(&file) {
                Ok(n) => {
                    st.push_log(None, tr!(LogIpFilterRules, count = n, path = file.display()), unix_now())
                }
                Err(e) => {
                    st.push_log(None, tr!(LogIpFilterError, path = file.display(), error = e), unix_now())
                }
            }
        }
    } else if loaded_ip_filter.is_some() {
        shared.handle.set_ip_filter(IpFilter::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_IP: IpAddr = IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);

    /// [`commit_asked`] for the changes of session `s` from [`TEST_IP`].
    fn commit_as(shared: &Shared, changes: &[Change]) -> Result<(), Refusal> {
        commit_asked(shared, changes, &mut Asker::new("s", TEST_IP, changes), &mut NewHashes::default())
    }

    #[test]
    fn stored_engine_settings_come_before_the_engine() {
        let stored = super::super::json::parse(
            r#"{"options":{"lt.enable_upnp":"false","lt.enable_natpmp":"false","lt.enable_dht":"false","lt.enable_lsd":"false","lt.download_rate_limit":"junk","webui.alt_enabled":"true"}}"#,
        )
        .unwrap();
        let mut s = Settings { download_rate_limit: 5000, ..Settings::default() };
        assert!(s.enable_upnp && s.enable_natpmp && s.enable_dht && s.enable_lsd);
        stored_engine_settings(&stored, &mut s);
        assert!(!s.enable_upnp && !s.enable_natpmp && !s.enable_dht && !s.enable_lsd);
        // a value that is not one keeps what was there (the command line's)
        assert_eq!(s.download_rate_limit, 5000);
        // no file, or one without options: nothing changes
        let mut d = Settings::default();
        stored_engine_settings(&Json::obj(), &mut d);
        assert!(d.enable_upnp && d.enable_dht);
    }

    /// D70: a stored proxy type this build cannot read (a newer build's) asks for the engine to
    /// start held, and says why; a readable one does not.
    #[test]
    fn an_unreadable_stored_proxy_type_holds_the_start() {
        let dir = std::env::temp_dir().join(format!("nox-hold-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let check = |options: &str| {
            std::fs::write(dir.join(super::super::STATE_FILE), format!(r#"{{"options":{options}}}"#))
                .unwrap();
            let mut s = Settings::default();
            let c = super::super::stored_settings(&dir, &mut s).unwrap();
            (c, s.proxy_type)
        };
        let (c, proxy) = check(r#"{"lt.proxy_type":"6","lt.proxy_hostname":"10.8.0.1"}"#);
        assert!(c.hold, "{c:?}");
        assert_eq!(proxy, rookparcel_core::ProxyType::None);
        assert!(c.log.iter().any(|l| l.contains("lt.proxy_type=6")), "{c:?}");
        let (c, proxy) = check(r#"{"lt.proxy_type":"2","lt.proxy_hostname":"10.8.0.1"}"#);
        assert!(!c.hold && c.log.is_empty(), "{c:?}");
        assert_eq!(proxy, rookparcel_core::ProxyType::Socks5);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    /// D70: only the Preferences page confirms. A commit from the qBittorrent API (automation
    /// posts `listen_port`) keeps a line this build cannot read and the old stamp; the page's
    /// save drops the line and stamps this version's reading.
    #[test]
    fn only_the_preferences_page_confirms_the_settings() {
        let dir = std::env::temp_dir().join(format!("nox-prefs-confirm-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let session = rookparcel_core::Session::start_mock(Settings::default(), Default::default());
        let shared = State::shared_for_tests(session.handle(), dir.clone());
        *shared.lock() = state();
        {
            let mut st = shared.lock();
            st.options.set("lt.proxy_type", "6");
            st.options.set("lt.proxy_hostname", "10.8.0.1");
            let check = rookparcel_frontend::guard::on_start_for(&mut st.options, None);
            assert!(check.hold, "{check:?}");
            let changes = parse(&Json::obj().with("listen_port", 51413)).unwrap();
            commit(&shared, &mut st, &changes).unwrap();
            assert_eq!(st.options.get("lt.proxy_type"), Some("6"), "kept: nobody confirmed");
            assert_eq!(st.settings.listen_interfaces, "0.0.0.0:51413,[::]:51413");
        }
        let read = |_: &Path| -> io::Result<IpFilterLoad> { panic!("no block list here") };
        shared.lock().sessions.insert(
            "s".to_owned(),
            super::super::ClientSession { last_used: std::time::Instant::now(), maindata: None, peers: None },
        );
        commit_request(&shared, &parse(&Json::obj()).unwrap(), "s", TEST_IP, RunGate::Open, read).unwrap();
        let st = shared.lock();
        assert_eq!(st.options.get("lt.proxy_type"), None, "the page's save confirms");
        assert!(
            rookparcel_frontend::guard::stamp_lines(&st.options).iter().any(|(k, _)| *k == "check.guard")
        );
        drop(st);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The network rule's rows (D54) by their own keys, as `settings::store` writes them; the
    /// `lt.net.*` keys of nox up to 1.2.54 when the own ones are not there.
    #[test]
    fn the_network_rule_is_read_by_its_own_key_and_the_old_one() {
        let read = |options: &str| {
            let stored = super::super::json::parse(&format!(r#"{{"options":{options}}}"#)).unwrap();
            let mut s = Settings::default();
            stored_engine_settings(&stored, &mut s);
            (s.network_rule, s.network_pause_when_missing)
        };
        let own = r#"{"net.network_rule":"vpn","net.pause_when_missing":"false"}"#;
        assert_eq!(read(own), ("vpn".to_owned(), false));
        let old = r#"{"lt.net.network_rule":"vpn","lt.net.pause_when_missing":"false"}"#;
        assert_eq!(read(old), ("vpn".to_owned(), false));
        let both = r#"{"net.network_rule":"range:10.0.0.0/8","lt.net.network_rule":"vpn"}"#;
        assert_eq!(read(both), ("range:10.0.0.0/8".to_owned(), true), "the own key wins");
        assert_eq!(legacy_key("enable_dht"), None);
        assert_eq!(legacy_key("bt.max_peers"), None);
        assert_eq!(legacy_key(settings::NETWORK_RULE).as_deref(), Some("lt.net.network_rule"));
    }

    /// The old keys are taken at start; the rows are then stored under their own keys, and the
    /// old ones are written beside them with the same values, so nox 1.2.54 and older keep the
    /// rule after a downgrade (D70).
    #[test]
    fn old_network_keys_are_migrated_and_kept_for_older_versions() {
        let mut st = state();
        st.options.set("lt.net.network_rule", "vpn");
        st.options.set("lt.net.pause_when_missing", "false");
        st.options.set("net.pause_when_missing", "true");
        migrate_keys(&mut st.options);
        assert_eq!(st.options.get("net.network_rule"), Some("vpn"));
        assert_eq!(st.options.get("net.pause_when_missing"), Some("true"), "the own key is kept");
        read_back(&mut st);
        assert_eq!(st.settings.network_rule, "vpn");
        assert!(st.settings.network_pause_when_missing);
        store(&mut st, &[]);
        assert_eq!(st.options.get("net.network_rule"), Some("vpn"));
        assert_eq!(st.options.get("net.pause_when_missing"), Some("true"), "stored before, so kept");
        assert_eq!(st.options.get("lt.net.network_rule"), Some("vpn"));
        assert_eq!(st.options.get("lt.net.pause_when_missing"), Some("true"));
        // a rule changed to off is stored as off under both keys
        st.settings.network_rule.clear();
        store(&mut st, &[settings::NETWORK_RULE]);
        assert_eq!(st.options.get("net.network_rule"), Some(""));
        assert_eq!(st.options.get("lt.net.network_rule"), Some(""));
        // hosted: the host's file gets no old keys
        st.hosted = true;
        store(&mut st, &[]);
        assert!(st.options.entries().all(|(k, _)| !k.starts_with("lt.net.")));
    }

    fn state() -> State {
        let mut st = State::for_tests();
        st.default_save_path = "/dl".to_owned();
        st.prefs = Prefs::defaults("/dl", &Settings::default());
        st
    }

    /// A working copy with `changes` applied (no engine, no commit).
    fn edited(st: &State, body: &str) -> Result<Working, Refusal> {
        let changes = parse(&super::super::json::parse(body).unwrap())?;
        let mut w = Working::from_state(st);
        for (def, v) in &changes {
            def.set(&mut w, v.clone());
        }
        validate(&w)?;
        Ok(w)
    }

    #[test]
    fn every_field_once_and_the_pages_stable() {
        let mut ids: Vec<&str> = all().map(|d| d.id()).collect();
        let n = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), n, "duplicate ids");
        assert_eq!(n, TABLE.len() + FIELDS.len());
        for e in TABLE {
            assert!(matches!(find(e.name), Some(Def::Setting(_))), "{}", e.name);
        }
        for f in FIELDS {
            assert!(matches!(find(f.id), Some(Def::Own(_))), "{}", f.id);
            assert!(settings::find(f.id).is_none(), "{} shadows an engine setting", f.id);
        }
        assert!(find("nope").is_none());

        let st = state();
        // the layout without a Shared: the page list is static but for one note
        let names: Vec<&str> = TABLE.iter().map(|e| e.name).collect();
        let w = Working::from_state(&st);
        for def in all() {
            // every field reads back its own default on a fresh state
            let d = Working::defaults(&st);
            assert_eq!(def.get(&w), def.get(&d), "{}", def.id());
        }
        assert_eq!(find("enable_dht").unwrap().get(&w), Value::Bool(true));
        assert_eq!(find("webui.max_auth_fail_count").unwrap().get(&w), Value::Int(5));
        assert_eq!(find("alt_dl_limit").unwrap().get(&w), Value::Int(10 * 1024));
        assert_eq!(find("dir.download").unwrap().get(&w), Value::Str("/dl".into()));
        assert_eq!(find("listen_port").unwrap().get(&w), Value::Int(6881));
        assert_eq!(find("webui.listen").unwrap().get(&w), Value::Str("127.0.0.1:8080".into()));
        assert!(names.contains(&"user_agent") && names.contains(&"bt.transp_disposition"));
    }

    /// D55: every field of `GET /pa/prefs` carries a help text, so a new field without one
    /// fails here; a field's own key wins over its label's.
    #[test]
    fn every_field_has_help() {
        let dir = std::env::temp_dir().join(format!("nox-prefs-help-{}", std::process::id()));
        let session = rookparcel_core::Session::start_mock(Settings::default(), Default::default());
        let shared = State::shared_for_tests(session.handle(), dir);
        let m = model(&shared, &state(), RunGate::Open);
        let fields = m.get("fields").unwrap();
        let mut missing = Vec::new();
        for def in all() {
            let f = fields.get(def.id()).unwrap();
            let help = f.get("help").and_then(Json::as_str).unwrap_or("");
            if help.trim().is_empty() {
                missing.push(def.id());
            }
            assert_eq!(f.get("wiki").is_some(), help::WIKI_URL.is_some(), "{}", def.id());
        }
        assert!(missing.is_empty(), "fields without a help text: {missing:?}");
        let text = |id: &str| fields.get(id).and_then(|f| f.get("help")).and_then(Json::as_str).unwrap();
        assert_eq!(text("webui.username"), lang::help(H::PrefWebUiUsername));
        assert_eq!(text("proxy_password"), lang::help(H::PrefProxyPassword));
        assert_eq!(text("cache.size_mb"), text("cache.override"));
        assert_eq!(text("enable_dht"), lang::help(H::PrefDht));
        assert_eq!(text("connection_speed"), lang::help(H::AdvConnectionSpeed));
        assert_eq!(text("webui.ban_duration"), lang::help(H::WebBanFor));
    }

    #[test]
    fn model_json_groups_every_page_field() {
        // the pages reference existing fields only, in 2.2.1's order
        let st = state();
        let w = Working::from_state(&st);
        let d = Working::defaults(&st);
        let mut fields = Json::obj();
        for def in all() {
            fields.set(def.id(), json_of(&def.get(&w)));
        }
        let pages_json = pages(Path::new("S"), false, true);
        let ids: Vec<String> =
            pages_json.iter().map(|p| p.get("id").and_then(Json::as_str).unwrap().to_owned()).collect();
        assert_eq!(
            ids,
            [
                "general",
                "ui",
                "directories",
                "connection",
                "bandwidth",
                "bittorrent",
                "queueing",
                "scheduler",
                "webui",
                "gaming",
                "advanced",
                "diskcache",
                "trackers",
                "run_program"
            ]
        );
        let titles: Vec<&str> =
            pages_json.iter().map(|p| p.get("title").and_then(Json::as_str).unwrap()).collect();
        assert_eq!(titles[1], "UI Settings");
        assert_eq!(titles[9], "Gaming");
        assert_eq!(pages_json[9].get("under"), None, "a page of its own above Advanced (D80)");
        assert_eq!(titles[11], "Disk Cache");
        assert_eq!(titles[12], "Trackers");
        assert_eq!(pages_json[11].get("under").and_then(Json::as_str), Some("advanced"));
        assert_eq!(pages_json[12].get("under").and_then(Json::as_str), Some("advanced"));
        assert_eq!(titles[13], "Run Program");
        assert_eq!(pages_json[13].get("under").and_then(Json::as_str), Some("advanced"));
        assert_eq!(pages_json[10].get("advanced"), Some(&Json::Bool(true)));
        let mut placed = Vec::new();
        fn walk(items: &[Json], placed: &mut Vec<String>) {
            for it in items {
                if let Some(id) = it.get("f").and_then(Json::as_str) {
                    placed.push(id.to_owned());
                }
                if let Some(Json::Arr(r)) = it.get("row") {
                    walk(r, placed);
                }
            }
        }
        for p in &pages_json {
            let Some(Json::Arr(groups)) = p.get("groups") else { panic!() };
            for g in groups {
                let Some(Json::Arr(items)) = g.get("items") else { panic!() };
                walk(items, &mut placed);
            }
        }
        for id in &placed {
            assert!(fields.get(id).is_some(), "page field {id} is not a field");
        }
        let mut sorted = placed.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), placed.len(), "a field placed twice");
        // every non-setting field is on some page
        for f in FIELDS {
            assert!(placed.contains(&f.id.to_owned()), "{} is on no page", f.id);
        }
        assert!(d.prefs.show_add_dialog);
        assert_eq!(json_of(&Value::Int(3)), Json::Int(3));
    }

    #[test]
    fn parses_and_refuses() {
        let st = state();
        let w = edited(
            &st,
            r#"{"enable_dht":false,"listen_port":"7000","connections_limit":42,"ui.add_stopped":1}"#,
        )
        .unwrap();
        assert!(!w.settings.enable_dht && w.prefs.start_paused);
        assert_eq!(settings::listen_port(&w.settings), Some(7000));
        assert_eq!(w.settings.connections_limit, 42);
        assert_eq!(w.settings.listen_interfaces, "0.0.0.0:7000,[::]:7000");

        let bad = |body: &str| edited(&st, body).err().unwrap();
        assert_eq!(bad(r#"{"no_such":1}"#), ("no_such".to_owned(), "unknown preference".to_owned()));
        assert_eq!(bad(r#"{"connections_limit":"lots"}"#).0, "connections_limit");
        assert!(bad(r#"{"connections_limit":"lots"}"#).1.contains("not a number"));
        assert!(bad(r#"{"upload_rate_limit":-1}"#).1.contains("outside"));
        assert_eq!(bad(r#"{"listen_port":0}"#).1, "0 is outside 1..65535");
        assert_eq!(bad(r#"{"webui.listen":"x"}"#).1, "read-only");
        assert_eq!(bad(r#"{"enable_dht":"maybe"}"#).1, "expected true or false, not \"maybe\"");
        assert_eq!(bad(r#"{"enable_dht":null}"#).1, "unexpected value null");
        assert_eq!(bad(r#"{"proxy_kind":9}"#).1, "9 is outside 0..3");
        // the first bad field is named, in body order
        assert_eq!(bad(r#"{"enable_dht":false,"zzz":1,"yyy":2}"#).0, "zzz");

        // the Win32 dialog's checks
        assert_eq!(bad(r#"{"proxy_kind":2}"#).0, "proxy_hostname");
        assert_eq!(bad(r#"{"proxy_kind":"Socks5","proxy_hostname":"p"}"#).0, "proxy_port");
        let w =
            edited(&st, r#"{"proxy_kind":"HTTP","proxy_hostname":"p","proxy_port":8080,"proxy_auth":true}"#)
                .unwrap();
        assert_eq!(w.settings.proxy_type, ProxyType::HttpPw);
        let w = edited(&st, r#"{"proxy_kind":3,"proxy_hostname":"p","proxy_port":8080}"#).unwrap();
        assert_eq!(w.settings.proxy_type, ProxyType::Http);
        // Disk Cache: a number before its box in the body still counts
        assert_eq!(st.settings.cache_size, -1, "automatic by default");
        let w = edited(&st, r#"{"cache.size_mb":100,"cache.override":true}"#).unwrap();
        assert_eq!(w.settings.cache_size, 6400);
        let w = edited(&st, r#"{"cache.size_mb":100}"#).unwrap();
        assert_eq!(w.settings.cache_size, -1, "the size alone keeps the automatic size");
        let w = edited(&st, r#"{"cache.expiry_s":60,"cache.expire":false,"use_read_cache":false}"#).unwrap();
        assert_eq!((w.settings.cache_expiry, w.settings.use_read_cache), (0, false));
        let w = edited(&st, r#"{"cache.expiry_s":60}"#).unwrap();
        assert_eq!(w.settings.cache_expiry, 60);
        assert_eq!(bad(r#"{"dir.download":"  "}"#).0, "dir.download");
        assert_eq!(bad(r#"{"dir.move_completed":true}"#).0, "dir.completed");
        assert_eq!(bad(r#"{"webui.username":""}"#).0, "webui.username");

        // composites
        let w = edited(&st, r#"{"enc_out":"Forced","enc_legacy":false}"#).unwrap();
        assert_eq!(w.settings.out_enc_policy, rookparcel_core::EncPolicy::Forced);
        assert_eq!(w.settings.in_enc_policy, rookparcel_core::EncPolicy::Forced);
        assert_eq!(w.settings.allowed_enc_level, rookparcel_core::EncLevel::Rc4);
        let w = edited(&st, r#"{"utp":false}"#).unwrap();
        assert!(
            !w.settings.enable_outgoing_utp
                && !w.settings.enable_incoming_utp
                && w.settings.enable_outgoing_tcp
        );
        // the password: write-only, empty means unchanged
        let w = edited(&st, r#"{"webui.password":""}"#).unwrap();
        assert_eq!(w.new_password, None);
        let w = edited(&st, r#"{"webui.password":"s3cret"}"#).unwrap();
        assert_eq!(w.new_password.as_deref(), Some("s3cret"));
        assert_eq!(find("webui.password").unwrap().get(&w), Value::Str(String::new()));
        // the rate limit rows edit the normal limits
        let w = edited(&st, r#"{"download_rate_limit":2048}"#).unwrap();
        assert_eq!(w.settings.download_rate_limit, 2048);
    }

    #[test]
    fn storing_keeps_the_frontend_keys() {
        let mut st = state();
        st.settings.enable_dht = false;
        st.prefs.dl_limit = 4096;
        st.settings.download_rate_limit = 1; // the alternative mode's value, not stored
        st.prefs.completed_dir = "/done".to_owned();
        st.prefs.move_completed = true;
        st.prefs.move_add_label = true;
        st.prefs.domain_list = "*.example".to_owned();
        store(&mut st, &["enable_lsd"]);
        let o = &st.options;
        assert_eq!(o.get("lt.enable_dht"), Some("false"));
        assert_eq!(o.get("lt.download_rate_limit"), Some("4096"));
        assert_eq!(o.get("lt.upload_rate_limit"), None, "defaults are not written");
        assert_eq!(o.get("lt.enable_lsd"), Some("true"), "a row set to its default on purpose is");
        assert_eq!(o.get("dir.completed"), Some("/done"));
        assert_eq!(o.get("dir.move_completed"), Some("1"));
        assert_eq!(o.get("dir.move_add_label"), Some("1"));
        assert_eq!(o.get("ui.close_to_tray"), None, "desktop-only keys are not written");
        assert_eq!(o.get("webui.domain_list"), Some("*.example"));
        assert_eq!(o.get("webui.csrf"), Some("1"));
        // the model's values follow what is stored
        let w = Working::from_state(&st);
        assert_eq!(find("download_rate_limit").unwrap().get(&w), Value::Int(4096));
    }

    /// Preferences > Gaming (D80): keep ping low and smart disk I/O, the engine's rows under
    /// 2.2.1-style captions, the MB/s number beside its box, stored as `lt.*` keys like any
    /// setting; gentle checks are off by default.
    #[test]
    fn the_gaming_page_edits_the_disk_settings() {
        let label = |id: &str| find(id).unwrap().label();
        assert_eq!(label("check_gently"), tr!(PrefCheckGently));
        assert_eq!(label("check_rate_limit"), tr!(PrefCheckRateLimit));
        assert_eq!(label("check_rate_mb"), "", "the number beside its box has no caption");
        assert_eq!(label("active_checking"), tr!(PrefActiveChecking));
        assert_eq!(label("disk_slow_uploads"), tr!(PrefDiskSlowUploads));
        assert_eq!(label("drive_overrides"), "drive_overrides", "Advanced only");

        let mut st = state();
        let w = Working::from_state(&st);
        let get = |w: &Working, id: &str| find(id).unwrap().get(w);
        assert_eq!(get(&w, "check_gently"), Value::Bool(false));
        assert_eq!(get(&w, "auto_rate_limit"), Value::Bool(false));
        assert_eq!(get(&w, "check_rate_mb"), Value::Int(100));
        assert_eq!(get(&w, "active_checking"), Value::Int(4));
        let body = r#"{"check_gently":true,"check_rate_limit":true,"check_rate_mb":250,"active_checking":2,
            "tune_for_drive":false,"disk_slow_downloads":true,"disk_slow_uploads":true}"#;
        let w = edited(&st, body).unwrap();
        let s = &w.settings;
        assert!(s.check_gently && s.check_rate_limit && !s.tune_for_drive);
        assert_eq!((s.check_rate_mb, s.active_checking), (250, 2));
        assert!(s.disk_slow_downloads && s.disk_slow_uploads);
        assert!(edited(&st, r#"{"check_rate_mb":0}"#).err().unwrap().1.contains("outside"));
        st.settings = w.settings;
        store(&mut st, &[]);
        assert_eq!(st.options.get("lt.check_rate_mb"), Some("250"));
        assert_eq!(st.options.get("lt.disk_slow_uploads"), Some("true"));
        assert_eq!(st.options.get("lt.check_gently"), Some("true"));
        // the page above Advanced holds keep ping low too, Bandwidth no longer
        let pages = pages(Path::new("."), false, true);
        let page = |id: &str| {
            pages.iter().find(|p| p.get("id").and_then(Json::as_str) == Some(id)).unwrap().to_json()
        };
        assert!(page("gaming").contains(r#"{"f":"auto_rate_limit"}"#), "{}", page("gaming"));
        assert!(page("gaming").contains(r#"{"f":"check_gently"}"#));
        assert!(!page("bandwidth").contains("auto_rate_limit"), "{}", page("bandwidth"));
    }

    /// Preferences > General > When Downloading (D77): µTorrent's caption without its mnemonic,
    /// the engine setting edited and stored as an `lt.*` key like any other.
    #[test]
    fn append_incomplete_ext_is_on_the_general_page() {
        let def = find("append_incomplete_ext").unwrap();
        assert_eq!(def.label(), "Append .!ut to incomplete files");
        assert_eq!(def.help_key(), Some(H::PrefAppendIncomplete));
        let mut st = state();
        let general = pages(Path::new("."), false, true).remove(0).to_json();
        assert!(general.contains(r#"{"f":"append_incomplete_ext"}"#), "{general}");
        let hosted = pages(Path::new("."), true, true).remove(0).to_json();
        assert!(hosted.contains(r#"{"f":"append_incomplete_ext"}"#), "hosted too");
        assert_eq!(find("append_incomplete_ext").unwrap().get(&Working::from_state(&st)), Value::Bool(false));
        let w = edited(&st, r#"{"append_incomplete_ext":true}"#).unwrap();
        assert!(w.settings.append_incomplete_ext);
        st.settings = w.settings;
        store(&mut st, &[]);
        assert_eq!(st.options.get("lt.append_incomplete_ext"), Some("true"));
    }

    /// Preferences > Advanced > Trackers (D66): the four fields are security fields, checked by
    /// `frontend::pref_pages::check`'s rules and stored as the frontend's keys; hosted, the page
    /// is not there and nothing of it is taken (the host's own list).
    #[test]
    fn the_trackers_page_is_guarded_and_the_hosts_own_when_hosted() {
        const A: &str = "udp://a.example:6969/announce";
        const B: &str = "https://b.example/announce";
        let has = |hosted: bool| {
            pages(Path::new("S"), hosted, true)
                .iter()
                .any(|p| p.get("id").and_then(Json::as_str) == Some("trackers"))
        };
        assert!(has(false) && !has(true));
        for id in TRACKER_FIELDS {
            assert!(SECURITY_FIELDS.contains(&id), "{id}");
            assert!(matches!(find(id), Some(Def::Own(_))), "{id}");
        }
        assert_eq!(find("trackers.list").unwrap().kind().name(), "lines");
        assert_eq!(find("trackers.list").unwrap().help_key(), Some(H::PrefTrackersAppend));
        assert_eq!(find("trackers.url").unwrap().help_key(), Some(H::PrefTrackersFromUrl));

        let st = state();
        let w = Working::from_state(&st);
        assert!(!w.prefs.trackers_append && !w.prefs.trackers_from_url, "off by default");
        let bad = |body: &str| edited(&st, body).err().unwrap();
        let junk =
            Json::obj().with("trackers.append", true).with("trackers.list", format!("{A}\nws://x.example/"));
        assert_eq!(bad(&junk.to_json()), ("trackers.list".to_owned(), tr!(TrackersErrLine).to_owned()));
        for url in ["http://lists.example/t.txt", "lists.example/t.txt", "", "https://"] {
            let body = Json::obj().with("trackers.from_url", true).with("trackers.url", url).to_json();
            assert_eq!(bad(&body), ("trackers.url".to_owned(), tr!(TrackersErrHttps).to_owned()), "{url}");
        }
        // an address that is not switched on is not checked (nothing fetches it)
        assert!(edited(&st, r#"{"trackers.url":"http://lists.example/t.txt"}"#).is_ok());

        let dir = std::env::temp_dir().join(format!("nox-prefs-trackers-{}", std::process::id()));
        let session = rookparcel_core::Session::start_mock(Settings::default(), Default::default());
        let shared = State::shared_for_tests(session.handle(), dir);
        *shared.lock() = state();
        let typed = format!("# a comment\r\n{A}\r\n\r\n\r\n{B}\r\n{A}\r\n");
        let body = |password: Option<&str>| {
            let mut j = Json::obj().with("trackers.append", true).with("trackers.list", typed.as_str());
            if let Some(p) = password {
                j.set("webui.current_password", p);
            }
            parse(&j).unwrap()
        };
        shared.lock().sessions.insert(
            "s".to_owned(),
            super::super::ClientSession { last_used: std::time::Instant::now(), maindata: None, peers: None },
        );
        // a request: only with the current password (the test state's is "x")
        let refused = commit_as(&shared, &body(None)).unwrap_err();
        assert_eq!(refused.0, "webui.current_password");
        let st = shared.lock();
        assert!(!st.prefs.trackers_append && st.sessions.contains_key("s"));
        drop(st);
        let refused = commit_as(&shared, &body(Some("guess"))).unwrap_err();
        assert_eq!(refused.0, "webui.current_password");
        let st = shared.lock();
        assert!(
            !st.prefs.trackers_append && !st.sessions.contains_key("s"),
            "a wrong guess ends the session"
        );
        drop(st);
        commit_as(&shared, &body(Some("x"))).unwrap();
        // the same list again, typed differently, is no change: no password is asked
        let same = parse(&Json::obj().with("trackers.list", format!("{A}\n\n{B}\n"))).unwrap();
        commit_as(&shared, &same).unwrap();
        let mut st = shared.lock();
        assert!(st.prefs.trackers_append);
        assert_eq!(st.prefs.trackers_list, format!("{A}\n\n{B}"), "the list's own form");
        assert_eq!(st.options.get("trackers.list"), Some(format!("{A} - {B}").as_str()));
        assert_eq!(st.options.get("trackers.append"), Some("1"));
        // the daemon's list follows the commit
        assert_eq!(
            shared.trackers().unwrap().tiers(),
            vec![vec![A.to_owned()], vec![B.to_owned()]],
            "configured at the commit"
        );
        // read back at the next start
        st.prefs = Prefs::defaults("/dl", &Settings::default());
        read_back(&mut st);
        assert!(st.prefs.trackers_append);
        assert_eq!(st.prefs.trackers_list, format!("{A}\n\n{B}"));
        drop(st);
        assert_eq!(update_trackers(&shared), Ok(String::new()), "nothing to fetch: no status");

        // hosted: refused whatever the password, and the host's values stay in its options
        shared.lock().hosted = true;
        let off =
            parse(&Json::obj().with("trackers.append", false).with("webui.current_password", "x")).unwrap();
        let refused = commit_as(&shared, &off).unwrap_err();
        assert_eq!(refused, ("trackers.append".to_owned(), HOSTED_NO_TRACKERS.to_owned()));
        let st = shared.lock();
        assert!(st.prefs.trackers_append);
        let mut g = GuiOptions::load(&rookparcel_frontend::prefs::Prefs::default(), "/dl");
        g.trackers_url = "https://host.example/list".to_owned();
        st.prefs.apply_to(&mut g);
        assert!(!g.trackers_append && g.trackers_list.is_empty(), "a host's list is never overwritten");
        assert_eq!(g.trackers_url, "https://host.example/list");
    }

    /// Preferences > Advanced > Run Program (D67). Whoever changes these lines runs programs
    /// under the daemon's account (qBittorrent's CVE-2023-30801), so they change through one
    /// door only: a page's request whose [`RunGate`] is open, with the current password.
    #[test]
    fn run_program_changes_only_through_an_open_gate_with_the_password() {
        use std::net::{IpAddr, Ipv4Addr};
        // a program that does not exist: nothing could start even if an event came
        const LINE: &str = r#""/nonexistent/rookparcel-test/none" "%N" %I"#;
        let page = |hosted: bool, open: bool| {
            pages(Path::new("S"), hosted, open)
                .into_iter()
                .find(|p| p.get("id").and_then(Json::as_str) == Some("run_program"))
        };
        // F: no page hosted
        assert!(page(false, true).is_some() && page(true, true).is_none() && page(true, false).is_none());
        // C: a session that may not change them is told why on the page
        let open = page(false, true).unwrap().to_json();
        let closed = page(false, false).unwrap().to_json();
        let reason = Json::from(tr!(RunErrRemote)).to_json();
        assert!(!open.contains(&reason) && closed.contains(&reason), "{closed}");
        for text in [tr!(PrefRunNote), tr!(PrefRunSafetyNote)] {
            let note = Json::from(text).to_json();
            assert!(open.contains(&note) && closed.contains(&note));
        }
        for id in RUN_FIELDS {
            assert!(SECURITY_FIELDS.contains(&id), "{id}");
            assert!(matches!(find(id), Some(Def::Own(_))), "{id}");
            assert!(TABLE.iter().all(|e| e.name != id), "{id} is no engine setting");
            assert!(find(id).unwrap().help_key().is_some(), "{id}");
        }
        assert_eq!(find("run.added_enabled").unwrap().kind().name(), "bool");
        assert_eq!(find("run.finished").unwrap().kind().name(), "text");
        assert_eq!(find("run.added").unwrap().help_key(), Some(H::PrefRunAdded));
        assert_eq!(find("run.finished").unwrap().help_key(), Some(H::PrefRunFinished));

        // I: a line that is switched on must be one that can be started
        let st = state();
        let w = Working::from_state(&st);
        assert!(!w.prefs.run_added_on && !w.prefs.run_finished_on, "off by default");
        assert_eq!(w.prefs.run_commands(), ("", ""));
        let bad = |on: &str, id: &str, line: &str| {
            edited(&st, &Json::obj().with(on, true).with(id, line).to_json()).err().unwrap()
        };
        let long = format!("prog {}", "x".repeat(5000));
        for (line, why) in [
            ("", tr!(RunErrEmpty)),
            ("   ", tr!(RunErrEmpty)),
            (r#""C:\Program Files\x.exe %N"#, tr!(RunErrQuote)),
            ("%N.exe --now", tr!(RunErrProgram)),
            (long.as_str(), tr!(RunErrLong)),
        ] {
            assert_eq!(
                bad("run.added_enabled", "run.added", line),
                ("run.added".to_owned(), why.to_owned()),
                "{line}"
            );
            assert_eq!(
                bad("run.finished_enabled", "run.finished", line),
                ("run.finished".to_owned(), why.to_owned()),
                "{line}"
            );
        }
        // a line that is not switched on is not checked (nothing starts it)
        assert!(edited(&st, r#"{"run.finished":"\"unclosed"}"#).is_ok());

        let dir = std::env::temp_dir().join(format!("nox-prefs-run-{}", std::process::id()));
        let session = rookparcel_core::Session::start_mock(Settings::default(), Default::default());
        let mut shared = State::shared_for_tests(session.handle(), dir);
        *shared.lock() = state();
        let body = |password: Option<&str>| {
            let mut j = Json::obj().with("run.finished_enabled", true).with("run.finished", LINE);
            if let Some(p) = password {
                j.set("webui.current_password", p);
            }
            parse(&j).unwrap()
        };
        let read = |_: &Path| -> io::Result<IpFilterLoad> { panic!("no block list here") };
        let login = |shared: &Shared| {
            shared.lock().sessions.insert(
                "s".to_owned(),
                super::super::ClientSession {
                    last_used: std::time::Instant::now(),
                    maindata: None,
                    peers: None,
                },
            );
        };
        let unchanged = |shared: &Shared| {
            let st = shared.lock();
            !st.prefs.run_finished_on
                && st.prefs.run_finished.is_empty()
                && st.options.get("run.finished").is_none()
                && st.options.get("run.finished_enabled").is_none()
        };
        let lines =
            |shared: &Shared| shared.lock().log.iter().filter(|l| l.text.contains("Run Program")).count();
        login(&shared);

        // B: from another computer, refused whatever the password: the right one, a wrong one
        // and none get the same answer, the session stays, and each try leaves a Logger line
        let there = RunGate::Closed(Some(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 7))));
        for (n, password) in [Some("x"), Some("guess"), None].into_iter().enumerate() {
            let refused = commit_request(&shared, &body(password), "s", TEST_IP, there, read).unwrap_err();
            assert_eq!(refused, ("run.finished_enabled".to_owned(), tr!(RunErrRemote).to_owned()));
            assert!(unchanged(&shared));
            assert!(shared.lock().sessions.contains_key("s"), "the password was not looked at");
            assert_eq!(lines(&shared), n + 1);
        }
        let logged = shared.lock().log.back().unwrap().text.clone();
        assert_eq!(logged, "Run Program: a change from 203.0.113.7 was refused (not this computer)");
        // the same request with something else in it changes nothing of that either
        let mut mixed = body(Some("x"));
        mixed.extend(parse(&Json::obj().with("ui.add_stopped", true)).unwrap());
        assert!(commit_request(&shared, &mixed, "s", TEST_IP, there, read).is_err());
        assert!(!shared.lock().prefs.start_paused);

        // D, E, H: `commit_asked` (the shim's setPreferences) and `commit` (a remote GUI's
        // settings, nox's own changes) never change them: with the password, from a session or
        // from nox itself
        let refused = commit_as(&shared, &body(Some("x"))).unwrap_err();
        assert_eq!(refused.1, tr!(RunErrRemote));
        assert!(unchanged(&shared));
        let refused = commit(&shared, &mut shared.lock(), &body(Some("x"))).unwrap_err();
        assert_eq!(refused.1, tr!(RunErrRemote));
        assert!(unchanged(&shared));

        // A: this computer, and only with the current password (the test state's is "x")
        let refused = commit_request(&shared, &body(None), "s", TEST_IP, RunGate::Open, read).unwrap_err();
        assert_eq!(refused.0, "webui.current_password");
        assert!(unchanged(&shared) && shared.lock().sessions.contains_key("s"));
        let refused =
            commit_request(&shared, &body(Some("guess")), "s", TEST_IP, RunGate::Open, read).unwrap_err();
        assert_eq!(refused.0, "webui.current_password");
        assert!(unchanged(&shared));
        assert!(!shared.lock().sessions.contains_key("s"), "a wrong guess ends the session");
        login(&shared);
        // I, at the commit: refused with the field and the text, password or not
        let quote = Json::obj()
            .with("run.added_enabled", true)
            .with("run.added", "\"unclosed")
            .with("webui.current_password", "x");
        let refused =
            commit_request(&shared, &parse(&quote).unwrap(), "s", TEST_IP, RunGate::Open, read).unwrap_err();
        assert_eq!(refused, ("run.added".to_owned(), tr!(RunErrQuote).to_owned()));
        commit_request(&shared, &body(Some("x")), "s", TEST_IP, RunGate::Open, read).unwrap();
        {
            // the same values again are no change: no password is asked, from anywhere
            let same = parse(&Json::obj().with("run.finished", format!("  {LINE} "))).unwrap();
            commit_as(&shared, &same).unwrap();
            let mut st = shared.lock();
            assert!(st.prefs.run_finished_on && !st.prefs.run_added_on);
            assert_eq!(st.prefs.run_finished, LINE);
            assert_eq!(st.prefs.run_commands(), ("", LINE));
            assert_eq!(st.options.get("run.finished"), Some(LINE));
            assert_eq!(st.options.get("run.finished_enabled"), Some("1"));
            // H: read back at the next start
            st.prefs = Prefs::defaults("/dl", &Settings::default());
            read_back(&mut st);
            assert!(st.prefs.run_finished_on);
            assert_eq!(st.prefs.run_finished, LINE);
        }
        // switching it off is a change like any other: not from elsewhere
        let off = parse(&Json::obj().with("run.finished_enabled", false).with("webui.current_password", "x"))
            .unwrap();
        assert!(commit_request(&shared, &off, "s", TEST_IP, there, read).is_err());
        assert!(shared.lock().prefs.run_finished_on);

        // C: the model says who may change them
        let readonly = |m: &Json, id: &str| {
            m.get("fields").and_then(|f| f.get(id)).and_then(|f| f.get("readonly")) == Some(&Json::Bool(true))
        };
        let (mine, theirs) = {
            let st = shared.lock();
            (model(&shared, &st, RunGate::Open), model(&shared, &st, there))
        };
        for id in RUN_FIELDS {
            assert!(!readonly(&mine, id) && readonly(&theirs, id), "{id}");
        }
        assert!(!readonly(&theirs, "trackers.url"));
        // the line itself only for a session that may change it; whether it is on, for both
        let value = |m: &Json, id: &str| {
            m.get("fields").and_then(|f| f.get(id)).and_then(|f| f.get("value")).cloned()
        };
        assert_eq!(value(&mine, "run.finished"), Some(Json::Str(LINE.into())));
        assert_eq!(value(&theirs, "run.finished"), Some(Json::Str(String::new())));
        assert_eq!(value(&theirs, "run.finished_enabled"), Some(Json::Bool(true)));

        // C: the gate of a request. A loopback peer is this computer unless a proxy passed the
        // request on; --webui-remote-run-program opens it for everyone
        let head = |extra: &str| {
            let text = format!("GET /pa/prefs HTTP/1.1\r\nHost: localhost\r\n{extra}\r\n");
            super::super::http::parse_head(text.as_bytes(), &super::super::http::Limits::default())
                .unwrap()
                .unwrap()
                .0
        };
        let loopback = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let lan = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 20));
        assert_eq!(RunGate::of(&shared, &head(""), loopback), RunGate::Open);
        assert_eq!(RunGate::of(&shared, &head(""), IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)), RunGate::Open);
        assert_eq!(RunGate::of(&shared, &head(""), lan), RunGate::Closed(Some(lan)));
        let via = IpAddr::V4(Ipv4Addr::new(198, 51, 100, 9));
        assert_eq!(
            RunGate::of(&shared, &head("X-Forwarded-For: 10.0.0.1, 198.51.100.9\r\n"), loopback),
            RunGate::Closed(Some(via)),
            "the address the ban list counts"
        );
        for h in ["Forwarded: for=198.51.100.9\r\n", "X-Real-IP: 198.51.100.9\r\n"] {
            assert!(matches!(RunGate::of(&shared, &head(h), loopback), RunGate::Closed(_)), "{h}");
        }
        // a proxy that names no address the Logger can show is logged as a proxy, not as a
        // loopback client that is "not this computer"
        let proxied = RunGate::of(&shared, &head("X-Real-IP: 198.51.100.9\r\n"), loopback);
        assert!(commit_request(&shared, &off, "s", TEST_IP, proxied, read).is_err());
        let logged = shared.lock().log.back().unwrap().text.clone();
        assert_eq!(logged, "Run Program: a change from 127.0.0.1 was refused (passed on by a proxy)");
        shared.remote_run_program = true;
        assert_eq!(RunGate::of(&shared, &head(""), lan), RunGate::Open);
        assert_eq!(RunGate::of(&shared, &head("X-Forwarded-For: 198.51.100.9\r\n"), loopback), RunGate::Open);
        shared.remote_run_program = false;

        // F: hosted, refused whatever the gate and the password, and the host's lines stay
        let mut st = shared.lock();
        st.hosted = true;
        let asker = Asker::new("s", TEST_IP, &off);
        let hashes = NewHashes::default();
        let refused = commit_read(&shared, &mut st, &off, Some(&asker), RunGate::Open, &mut None, &hashes);
        let Err(Stop::Refused(refused)) = refused else { panic!("not refused") };
        assert_eq!(refused, ("run.finished_enabled".to_owned(), HOSTED_NO_RUN.to_owned()));
        assert!(st.prefs.run_finished_on);
        // nor read: the host's line goes to no browser, this computer's included
        let hosted = model(&shared, &st, RunGate::Open);
        assert_eq!(value(&hosted, "run.finished"), Some(Json::Str(String::new())));
        assert!(readonly(&hosted, "run.finished"));
        let mut g = GuiOptions::load(&rookparcel_frontend::prefs::Prefs::default(), "/dl");
        g.run_added = "host-added".to_owned();
        st.prefs.apply_to(&mut g);
        assert!(!g.run_finished_on && g.run_finished.is_empty(), "a host's lines are never overwritten");
        assert_eq!(g.run_added, "host-added");
    }

    #[test]
    fn the_scheduler_fields_edit_the_schedule() {
        use rookparcel_frontend::scheduler::{CELLS, Mode};
        let st = state();
        let mut table = "0".repeat(CELLS);
        table.replace_range(9..10, "2");
        table.replace_range(CELLS - 1.., "3");
        let body = format!(
            r#"{{"sched.enable":true,"sched.table":"{table}","sched.ul_rate":20,"sched.dl_rate":300,"sched.dis_dht":false}}"#
        );
        let w = edited(&st, &body).unwrap();
        let s = &w.prefs.schedule;
        assert!(s.enabled && !s.disable_dht);
        assert_eq!((s.limited_up_kib, s.limited_down_kib), (20, 300));
        assert_eq!((s.get(0, 9), s.get(6, 23), s.get(0, 0)), (Mode::Limited, Mode::SeedOnly, Mode::Full));
        assert_eq!(find("sched.table").unwrap().get(&w), Value::Str(table));
        assert_eq!(find("sched.table").unwrap().kind().name(), "schedule");

        let bad = |body: &str| edited(&st, body).err().unwrap();
        assert_eq!(bad(r#"{"sched.table":"0123"}"#).0, "sched.table");
        assert!(bad(&format!(r#"{{"sched.table":"{}9"}}"#, "0".repeat(CELLS - 1))).1.contains("168 hours"));
        assert!(bad(r#"{"sched.ul_rate":-1}"#).1.contains("outside"));

        // stored as the frontend's keys, as the Win32 client stores them
        let mut st = state();
        st.prefs.schedule = w.prefs.schedule.clone();
        store(&mut st, &[]);
        assert_eq!(st.options.get("sched.enable"), Some("1"));
        assert_eq!(st.options.get("sched.ul_rate"), Some("20"));
        assert_eq!(st.options.get("sched.table").map(str::len), Some(CELLS));
    }

    /// Preferences > General's language: a choice of `[value, text]` with the system's first,
    /// a shipped code (any case), empty or `pseudo`, stored as `ui.language`. (Nothing here
    /// switches the process's language: the other tests read English.)
    #[test]
    fn the_language_is_a_choice_of_codes() {
        let st = state();
        let def = find("ui.language").unwrap();
        assert_eq!(def.kind().name(), "choice");
        assert_eq!(def.get(&Working::from_state(&st)), Value::Str(String::new()));
        assert_eq!(edited(&st, r#"{"ui.language": "EN"}"#).unwrap().prefs.language, "en");
        assert_eq!(edited(&st, r#"{"ui.language": "pseudo"}"#).unwrap().prefs.language, "pseudo");
        assert_eq!(edited(&st, r#"{"ui.language": ""}"#).unwrap().prefs.language, "");
        let (id, why) = edited(&st, r#"{"ui.language": "xx"}"#).err().unwrap();
        assert_eq!(id, "ui.language");
        assert!(why.contains("\"xx\" is not one of en"), "{why}");
        let general = &pages(Path::new("S"), false, true)[0];
        assert!(general.to_json().contains(r#"{"f":"ui.language"}"#), "{}", general.to_json());
        let mut w = Working::from_state(&st);
        w.prefs.language = "en".to_owned();
        let mut options = rookparcel_frontend::prefs::Prefs::default();
        w.prefs.gui().store(&mut options);
        assert_eq!(options.get("ui.language"), Some("en"));
    }

    /// SEC-05: a request's block list is read while the State lock is free, once, and only
    /// when the path policy allows it; the rules are in force after the commit.
    #[test]
    fn a_requests_ip_filter_is_read_without_the_lock() {
        let dir = std::env::temp_dir().join(format!("nox-prefs-ipfilter-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let list = dir.join("list.dat");
        std::fs::write(&list, "1.2.3.0 - 1.2.3.255 , 0 , x\n").unwrap();
        let session = rookparcel_core::Session::start_mock(Settings::default(), Default::default());
        let shared = State::shared_for_tests(session.handle(), dir.clone());
        *shared.lock() = state();
        let reads = std::cell::Cell::new(0);
        let read = |f: &Path| {
            assert!(shared.state.try_lock().is_ok(), "read under the State lock");
            reads.set(reads.get() + 1);
            IpFilter::load(f)
        };
        let body =
            Json::obj().with("ipfilter.enable", true).with("ipfilter.path", list.to_string_lossy().as_ref());
        let changes = parse(&body).unwrap();
        commit_request(&shared, &changes, "s", TEST_IP, RunGate::Open, read).unwrap();
        assert_eq!(reads.get(), 1);
        assert!(shared.lock().prefs.ip_filter);
        assert!(shared.lock().log.iter().any(|l| l.text.contains("list.dat")));
        // the same again: nothing to read
        commit_request(&shared, &changes, "s", TEST_IP, RunGate::Open, read).unwrap();
        assert_eq!(reads.get(), 1);
        // a path the policy refuses is not read, and is refused
        let changes = parse(&Json::obj().with("ipfilter.path", "relative.dat")).unwrap();
        assert!(commit_request(&shared, &changes, "s", TEST_IP, RunGate::Open, read).is_err());
        assert_eq!(reads.get(), 1);
        // a list that does not load refuses the change
        let missing = dir.join("missing.dat");
        let body = Json::obj().with("ipfilter.path", missing.to_string_lossy().as_ref());
        let refused =
            commit_request(&shared, &parse(&body).unwrap(), "s", TEST_IP, RunGate::Open, read).unwrap_err();
        assert_eq!(refused.0, "ipfilter.path");
        assert_eq!(reads.get(), 2);
        assert_eq!(shared.lock().prefs.ip_filter_path, list.to_string_lossy());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// "Append the torrent's label" reaches the owner's mover: committing it bumps the
    /// version `WebUi::sync_move_completed` watches.
    #[test]
    fn append_label_reaches_the_mover() {
        let dir = std::env::temp_dir().join(format!("nox-prefs-add-label-{}", std::process::id()));
        let session = rookparcel_core::Session::start_mock(Settings::default(), Default::default());
        let shared = State::shared_for_tests(session.handle(), dir.clone());
        *shared.lock() = state();
        let done = dir.join("done");
        let body = Json::obj()
            .with("dir.move_completed", true)
            .with("dir.completed", done.to_string_lossy().as_ref())
            .with("dir.move_add_label", true);
        let read = |f: &Path| IpFilter::load(f);
        commit_request(&shared, &parse(&body).unwrap(), "s", TEST_IP, RunGate::Open, read).unwrap();
        let seen = shared.lock().gui_version;
        let m = shared.lock().prefs.gui().move_completed().expect("a mover");
        assert!(m.append_label);
        let body = Json::obj().with("dir.move_add_label", false);
        commit_request(&shared, &parse(&body).unwrap(), "s", TEST_IP, RunGate::Open, read).unwrap();
        assert!(shared.lock().gui_version > seen, "the mover is told");
        assert!(!shared.lock().prefs.move_add_label);
    }

    /// Bug hunt 2026-11-21, follow-up of 11: a request's PBKDF2 runs without the State lock,
    /// which every Web UI, API and WebSocket client waits for: the current password's check and
    /// a new password's or API key's hash, in the shim's `setPreferences` (its one-by-one
    /// fallback included, each made once) and in the Preferences page's commit. Each hash looks
    /// at the lock from the thread that makes it: `try_lock` fails if that thread holds it.
    #[test]
    fn requests_hash_without_the_state_lock() {
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::sync::{Arc, TryLockError};

        let dir = std::env::temp_dir().join(format!("nox-prefs-hash-lock-{}", std::process::id()));
        let session = rookparcel_core::Session::start_mock(Settings::default(), Default::default());
        let shared = Arc::new(State::shared_for_tests(session.handle(), dir));
        *shared.lock() = state();
        let locked = Rc::new(RefCell::new(Vec::new()));
        let (s, seen) = (shared.clone(), locked.clone());
        auth::tests::on_hash(Some(Box::new(move || {
            seen.borrow_mut().push(matches!(s.state.try_lock(), Err(TryLockError::WouldBlock)))
        })));

        // the shim: a proxy type without a host fails the set, so each change goes on its own
        let set = super::super::json::parse(
            r#"{"web_ui_password":"new-pass","proxy_type":2,"web_ui_current_password":"x"}"#,
        )
        .unwrap();
        super::super::api::apply_preferences(&shared, &set, "s", TEST_IP);
        assert_eq!(*locked.borrow(), [false, false], "the check and the new hash, once each, unlocked");
        assert!(shared.lock().password.verify("new-pass"), "the new password is in force");

        // the Preferences page: a new API key
        const KEY: &str = "0123456789abcdef0123";
        let changes =
            parse(&Json::obj().with("webui.api_key", KEY).with("webui.current_password", "new-pass"))
                .unwrap();
        commit_request(&shared, &changes, "s", TEST_IP, RunGate::Open, |f: &Path| IpFilter::load(f)).unwrap();
        assert_eq!(*locked.borrow(), [false; 4]);
        assert!(shared.lock().api_key.as_ref().is_some_and(|k| k.verify(KEY)), "the new key is in force");
        auth::tests::on_hash(None);
    }
}
