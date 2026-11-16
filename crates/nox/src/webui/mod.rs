//! The Web UI server (PLAN.md Phase 6, D18): an embedded HTTP/1.1 server with the native push
//! API (`/pa/ws`, a WebSocket carrying the session protocol of `docs/WEBUI.md`) and, as a
//! compatibility shim (D10), a subset of the qBittorrent WebAPI v2 written from qBittorrent's
//! published WebAPI documentation only.
//!
//! Threads: one acceptor, one thread per connection (at most [`MAX_CONNECTIONS`]); a WebSocket
//! keeps its connection thread. They never touch the engine directly. The owner of the
//! session's alert queue (nox's main loop, or the desktop client hosting the Web UI over its
//! own engine, [`Host`]) hands every batch of alerts to [`WebUi::on_alerts`], which keeps a
//! cache of torrent status, session stats and replies to `post_*` requests and counts a status
//! tick per batch; requests read that cache and post commands through the [`SessionHandle`],
//! WebSocket threads send what changed since their last tick.
//!
//! Hosted (D40, [`Config::hosted`]): the host owns the preferences, the labels, the pause
//! marks and the automations; the Web UI keeps a mirror of its preferences in
//! `State::options`, writes what a browser changed there and hands the journal to the host
//! ([`Shared::save`]), and takes the host's changes through [`WebUi::options_changed`]. Nothing
//! is written to disk and nothing but a request's own command goes to the engine. The RSS
//! Downloader the pages edit is the host's own model ([`Hosted::rss`], `rss.rs`).

mod api;
pub mod auth;
pub(crate) mod commands;
mod fetch;
mod helper;
pub mod http;
pub mod https;
pub mod json;
pub mod native;
pub mod path_policy;
pub(crate) mod prefs;
mod remote;
pub(crate) mod rss;
mod search;
mod server;
pub(crate) mod theme;

pub use rss::Rss;
pub mod torrent;
mod update;
pub mod ws;

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, Weak};
use std::thread::JoinHandle;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use rookparcel_core::fetch::FetchProxy;
use rookparcel_core::run_program::RunProgram;
use rookparcel_core::util::private_file;
use rookparcel_core::util::time::local_utc_offset;
use rookparcel_core::{
    Alert, AnnounceEntry, Bitfield, FileEntry, MoveCompleted, PartialPiece, PeerInfo, SessionHandle,
    SessionStats, Settings, TorrentFlags, TorrentId, TorrentStatus,
};
use rookparcel_frontend::exclude::Pending;
use rookparcel_frontend::ip_filter_list::{self, IpFilterList};
use rookparcel_frontend::options::GuiOptions;
use rookparcel_frontend::scheduler::{Mode, Schedule, Scheduler};
use rookparcel_frontend::scrape::Scrapes;
use rookparcel_frontend::seeding::{SeedGoal, SeedingGoal};
use rookparcel_frontend::tags::{self, TagError};
use rookparcel_frontend::torrent_folders::{self, Poller, TorrentFiles, TorrentFolders};
use rookparcel_frontend::tr;
use rookparcel_frontend::tracker_list::{self, TrackerList};

use auth::{BanList, PasswordHash};
use json::Json;

/// Most connections served at once; more are answered 503 and closed.
pub const MAX_CONNECTIONS: usize = 64;
/// Most logged-in sessions kept; the least recently used is dropped beyond this.
pub const MAX_SESSIONS: usize = 256;
/// How long a request waits for the engine to answer a `post_*` call.
const REPLY_TIMEOUT: Duration = Duration::from_secs(5);
/// Requests `Shared::query_many` has out at once: well under what `State::reply` keeps after
/// trimming (the last 64), so other clients' replies cannot push ours out before they are read.
const QUERY_GROUP: usize = 32;
/// The name of the state file in the state directory.
pub const STATE_FILE: &str = "webui.json";
/// Log lines kept for the Logger tab (the alert loop's lines, session-wide and per torrent).
pub const LOG_LINES: usize = 500;
/// Speed samples kept per watched torrent: 10 minutes at the engine's one status tick a second.
pub const SPEED_SAMPLES: usize = 600;
/// A speed ring no client looked at for this many ticks is dropped.
const SPEED_IDLE_TICKS: u64 = 600;
/// Torrents restored from records older than 1.2.30, which do not say whether the torrent is
/// private (`TorrentStatus::private` is `None`), asked of the engine per status tick: one
/// blocking engine round trip and a read of the metadata each, on the thread feeding the alerts
/// (a hosting window's GUI thread), so few per tick; 100k such torrents take a quarter of an
/// hour of ticks, once (the engine saves the answer).
const META_PER_TICK: usize = 128;

/// How to start the Web UI.
#[derive(Clone, Debug)]
pub struct Config {
    pub addr: SocketAddr,
    pub username: String,
    /// A new password (from the command line): hashed and, with a state directory, stored as
    /// the hash. `None` keeps the stored one; with none stored, a random temporary password is
    /// made and shown once ([`WebUi::temporary_password`]).
    pub password: Option<String>,
    pub state_dir: Option<PathBuf>,
    /// The default save path for added torrents.
    pub save_path: String,
    /// The settings the session was started with; the Web UI changes them through
    /// `apply_settings`.
    pub settings: Settings,
    pub pbkdf2_iterations: u32,
    pub limits: http::Limits,
    /// `--move-on-complete`: the Directories page's "Move completed downloads to" starts on
    /// with this folder (the stored preferences win over it).
    pub move_completed_dir: Option<String>,
    /// `--ip-filter`: the block list the daemon loaded at start, so the BitTorrent page shows
    /// it and the Web UI does not load it a second time.
    pub ip_filter: Option<PathBuf>,
    /// Serve HTTPS with this certificate instead of plain HTTP.
    pub https: Option<https::Certificate>,
    /// `--webui-save-root`: the folders requests may save, move and read in (none = any
    /// absolute local path; see [`path_policy`]).
    pub save_roots: Vec<PathBuf>,
    /// A desktop client hosting the Web UI over its own engine (D40): see [`Hosted`].
    pub hosted: Option<Hosted>,
    /// The folder the search helper (D51, `helper.rs`) lives in (`<dir>/search-helper`): nox's
    /// state directory, or hosted the window's own profile folder, so the pages act on the
    /// window's helper. `None`: the pages offer no helper.
    pub helper_dir: Option<PathBuf>,
    /// `--webui-remote-run-program`: the Run Program lines (D67) may be changed from another
    /// computer too (still only with the current password). Off unless the command line says
    /// so: it is never stored and no request can set it.
    pub remote_run_program: bool,
}

/// A desktop client that hosts the Web UI over its own engine (D40: the Windows GUI, as
/// µTorrent and qBittorrent serve theirs). It owns the preferences (`rookparcel.ini`), the
/// labels, the pause marks and the automations (scheduler, seeding goal, scrapes, `.torrent`
/// folders, RSS, moving completed downloads); the Web UI keeps a mirror of its preferences,
/// shows and drives the same torrents, and reports what a browser changed. The Web UI stores
/// nothing itself then and runs none of its own automations; its RSS endpoints edit the
/// host's RSS Downloader ([`Hosted::rss`]).
///
/// No method may wait on the host's own thread (queue, post a message and return): a request
/// thread may hold the Web UI's lock while it calls one.
pub trait Host: Send + Sync {
    /// Preferences keys a request changed (`None`: removed), as the host's own keys
    /// (`rookparcel_frontend::prefs`): a label or a pause mark as `torrent.<key>.label` /
    /// `torrent.<key>.paused` (`InfoHashes::key_hex`), a Preferences page's `lt.*` and option
    /// keys, a seeding goal, the login as `webui.username` / `webui.password` (the hash).
    /// Called on a request thread with the Web UI's lock held (or from [`WebUi::on_alerts`],
    /// for a removed torrent's keys): queue them for the host's thread and return.
    fn options_changed(&self, changes: Vec<rookparcel_frontend::prefs::Change>);
    /// `app/shutdown` (the shim) or `session/shutdown` (the page): the host exits. Called on
    /// a request thread without the lock.
    fn shutdown(&self);
    /// A browser changed the host's RSS Downloader ([`Hosted::rss`]: a feed, a filter, the
    /// History, a refresh started): the host redraws what shows it, and saves `rss.dat` when
    /// [`Rss::dirty`] is set. Called on a request thread without either lock.
    fn rss_changed(&self);
    /// A browser asked for these RSS downloads (an item's Download, the filters run when the
    /// dialog closed, a shim rule): the host adds them as it adds its own RSS downloads (its
    /// worker fetches the `.torrent` files; its Add dialog's rules apply). The model has
    /// marked them added and put them in the History already. Called on a request thread
    /// without either lock.
    fn rss_download(&self, reqs: Vec<rookparcel_frontend::rss::AddRequest>);
}

/// How a desktop client hosts the Web UI ([`Config::hosted`]).
#[derive(Clone)]
pub struct Hosted {
    pub host: Arc<dyn Host>,
    /// The host's preferences, mirrored: the login (`webui.username`, `webui.password` as a
    /// [`PasswordHash`] string), the labels and pause marks (`torrent.<key>.*`), the engine
    /// settings (`lt.*`) and the options the Preferences pages show. The host hands later
    /// changes to [`WebUi::options_changed`].
    pub options: rookparcel_frontend::prefs::Prefs,
    /// The host's RSS Downloader, which the RSS endpoints (`/pa/rss...`, the shim's `rss/*`)
    /// read and edit: one model, the host's, never a second downloader. The host keeps running
    /// its worker, collecting the fetches, adding the torrents and saving `rss.dat`
    /// ([`Rss::dirty`]), and calls [`WebUi::rss_changed`] when it changed the model itself.
    /// Lock order on every thread: this lock before the Web UI's own, never the other way
    /// round.
    pub rss: Arc<Mutex<Rss>>,
}

impl std::fmt::Debug for Hosted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hosted").finish_non_exhaustive()
    }
}

impl Config {
    pub fn new(addr: SocketAddr, settings: Settings, save_path: impl Into<String>) -> Config {
        Config {
            addr,
            username: "admin".into(),
            password: None,
            state_dir: None,
            save_path: save_path.into(),
            settings,
            pbkdf2_iterations: auth::DEFAULT_ITERATIONS,
            limits: http::Limits::default(),
            move_completed_dir: None,
            ip_filter: None,
            https: None,
            save_roots: Vec::new(),
            hosted: None,
            helper_dir: None,
            remote_run_program: false,
        }
    }
}

/// Parses `[ADDR:]PORT` (`8080`, `0.0.0.0:8080`, `[::1]:8080`); the address defaults to
/// 127.0.0.1.
pub fn parse_addr(s: &str) -> Result<SocketAddr, String> {
    if let Ok(port) = s.parse::<u16>() {
        return Ok(SocketAddr::from(([127, 0, 0, 1], port)));
    }
    s.parse().map_err(|_| format!("--webui {s}: expected [ADDR:]PORT"))
}

/// Preferences that live in the Web UI rather than in the engine's [`Settings`]: the frontend's
/// options nox has an equivalent for (`frontend::options::GuiOptions` without the tray and
/// balloon ones), the global limits and the Web UI's own settings. One source of truth for
/// `/pa/prefs` and the shim's `app/preferences`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Prefs {
    /// Where new downloads go (`GuiOptions::download_dir`).
    pub save_path: String,
    /// Bytes per second, 0 = unlimited: the normal and the alternative global limits.
    pub dl_limit: u32,
    pub up_limit: u32,
    pub alt_dl_limit: u32,
    pub alt_up_limit: u32,
    pub alt_enabled: bool,
    /// Add torrents stopped (`GuiOptions::add_stopped`).
    pub start_paused: bool,
    /// Move finished downloads to `completed_dir` (Directories page).
    pub move_completed: bool,
    pub completed_dir: String,
    pub move_only_default: bool,
    /// Into the label's folder under `completed_dir` (`GuiOptions::move_add_label`).
    pub move_add_label: bool,
    /// "Location of .torrents" (Directories page): stored copies, finished jobs, auto-load.
    pub torrent_folders: TorrentFolders,
    /// The page asks before removing torrents.
    pub confirm_delete: bool,
    /// The page shows the Add New Torrent dialog for `.torrent` files (`GET /pa/add`'s
    /// `show_dialog`).
    pub show_add_dialog: bool,
    /// UI Settings > When Adding Torrents (D74, `GuiOptions::exclude_names`, `exclude_list`,
    /// kept as `frontend::exclude::to_stored` gives it): files whose names match the list start
    /// unchecked in the page's Add dialog, and are not downloaded in torrents added without it
    /// (`commands::add`). Hosted, the host's, which a browser changes as its other options.
    pub exclude_names: bool,
    pub exclude_list: String,
    /// A new listen port at every start.
    pub random_port: bool,
    /// Per-torrent defaults: peer exchange, connected peers and upload slots per new torrent
    /// (0 = no limit).
    pub pex: bool,
    pub max_peers_per_torrent: u32,
    pub upload_slots_per_torrent: u32,
    /// Preferences > Queueing's seeding goal (`GuiOptions::seeding_goal`).
    pub seeding_goal: SeedingGoal,
    /// The IP filter: on, and the block list (empty = `ipfilter.dat` in the state directory).
    pub ip_filter: bool,
    pub ip_filter_path: String,
    /// The block list from an `https://` address instead of the file (D75,
    /// `GuiOptions::ip_filter_from_url`, `ip_filter_url`), fetched by [`Shared::ip_list`].
    /// Changed only with the current password (`prefs::SECURITY_FIELDS`); hosted, the host's
    /// own, which a browser never changes.
    pub ip_filter_from_url: bool,
    pub ip_filter_url: String,
    /// Preferences > Scheduler (`GuiOptions::schedule`).
    pub schedule: Schedule,
    /// Preferences > General > Privacy: nox's update check (`GuiOptions::check_updates`,
    /// `update_beta`; D43). Hosted, the host's own.
    pub check_updates: bool,
    pub update_beta: bool,
    /// Preferences > General's language (`GuiOptions::language`): the Web UI's, the server's.
    pub language: String,
    /// Preferences > Advanced > Trackers (D66, `GuiOptions::trackers_*`): the typed list added
    /// to every new public download, and the `https://` address a list is fetched from. Changed
    /// only with the current password (`prefs::SECURITY_FIELDS`); hosted, the host's own.
    pub trackers_append: bool,
    pub trackers_list: String,
    pub trackers_from_url: bool,
    pub trackers_url: String,
    /// Preferences > Advanced > Run Program (D67, `GuiOptions::run_*`): the command lines the
    /// daemon starts when a torrent is added and when one finishes downloading. Changed only
    /// through `prefs::commit_request`, from this computer (or with
    /// `--webui-remote-run-program`) and with the current password (`prefs::RunGate`,
    /// `prefs::SECURITY_FIELDS`); hosted, the host's own.
    pub run_added_on: bool,
    pub run_added: String,
    pub run_finished_on: bool,
    pub run_finished: String,
    pub bypass_local_auth: bool,
    /// `Authorization: Bearer <key>` is accepted (with a key stored: `State::api_key`).
    pub api_key_enabled: bool,
    /// `;`-separated subnets (CIDR or bare addresses, IPv4 and IPv6) whose clients skip the
    /// login (qBittorrent's `bypass_auth_subnet_whitelist`); empty = off.
    pub auth_subnets: String,
    pub csrf_protection: bool,
    pub host_header_validation: bool,
    /// `;`-separated host names accepted in `Host` besides IP addresses and `localhost`
    /// (`*` wildcards allowed).
    pub domain_list: String,
    pub max_auth_fail_count: u32,
    pub ban_duration: u64,
    pub session_timeout: u64,
    /// The page's theme for everyone (D63, [`theme`]): a built-in id or a third-party file's
    /// name; `system` follows the browser.
    pub theme: String,
}

impl Prefs {
    /// The defaults for a daemon saving to `save_path` with `settings`' limits.
    pub fn defaults(save_path: &str, settings: &Settings) -> Prefs {
        let gui = GuiOptions::load(&rookparcel_frontend::prefs::Prefs::default(), save_path);
        let mut p = Prefs {
            save_path: save_path.to_owned(),
            dl_limit: settings.download_rate_limit,
            up_limit: settings.upload_rate_limit,
            alt_dl_limit: 10 * 1024,
            alt_up_limit: 10 * 1024,
            alt_enabled: false,
            start_paused: false,
            move_completed: false,
            completed_dir: String::new(),
            move_only_default: true,
            move_add_label: false,
            torrent_folders: TorrentFolders::default(),
            confirm_delete: true,
            show_add_dialog: true,
            exclude_names: false,
            exclude_list: String::new(),
            random_port: false,
            pex: true,
            max_peers_per_torrent: 0,
            upload_slots_per_torrent: 0,
            seeding_goal: SeedingGoal::default(),
            ip_filter: false,
            ip_filter_path: String::new(),
            ip_filter_from_url: false,
            ip_filter_url: String::new(),
            schedule: Schedule::default(),
            check_updates: true,
            update_beta: false,
            language: String::new(),
            trackers_append: false,
            trackers_list: String::new(),
            trackers_from_url: false,
            trackers_url: String::new(),
            run_added_on: false,
            run_added: String::new(),
            run_finished_on: false,
            run_finished: String::new(),
            bypass_local_auth: false,
            api_key_enabled: false,
            auth_subnets: String::new(),
            csrf_protection: true,
            host_header_validation: true,
            domain_list: String::new(),
            max_auth_fail_count: 5,
            ban_duration: 3600,
            session_timeout: 3600,
            theme: theme::SYSTEM.to_owned(),
        };
        p.set_gui(&gui);
        p
    }

    /// The frontend's view of these preferences (the tray and balloon options at their
    /// defaults), for `GuiOptions`' helpers and its preferences keys.
    pub fn gui(&self) -> GuiOptions {
        GuiOptions {
            download_dir: self.save_path.clone(),
            theme: String::new(),
            move_completed: self.move_completed,
            completed_dir: self.completed_dir.clone(),
            move_only_default: self.move_only_default,
            move_add_label: self.move_add_label,
            torrent_folders: self.torrent_folders.clone(),
            confirm_delete: self.confirm_delete,
            alternate_rows: true,
            title_speed: false,
            check_association: false,
            close_to_tray: true,
            minimize_to_tray: false,
            balloons: true,
            activate_on_add: true,
            add_stopped: self.start_paused,
            show_add_dialog: self.show_add_dialog,
            random_port: self.random_port,
            pex: self.pex,
            max_peers_per_torrent: self.max_peers_per_torrent,
            upload_slots_per_torrent: self.upload_slots_per_torrent,
            seed_ratio: self.seeding_goal.goal.ratio_pct,
            seed_minutes: self.seeding_goal.goal.minutes,
            seed_limit_upload: self.seeding_goal.limit_upload,
            seed_limit_kib: self.seeding_goal.limit_kib,
            ip_filter: self.ip_filter,
            ip_filter_path: self.ip_filter_path.clone(),
            schedule: self.schedule.clone(),
            language: self.language.clone(),
            check_updates: self.check_updates,
            update_beta: self.update_beta,
            trackers_append: self.trackers_append,
            trackers_list: self.trackers_list.clone(),
            trackers_from_url: self.trackers_from_url,
            trackers_url: self.trackers_url.clone(),
            run_added_on: self.run_added_on,
            run_added: self.run_added.clone(),
            run_finished_on: self.run_finished_on,
            run_finished: self.run_finished.clone(),
            exclude_names: self.exclude_names,
            exclude_list: self.exclude_list.clone(),
            ip_filter_from_url: self.ip_filter_from_url,
            ip_filter_url: self.ip_filter_url.clone(),
        }
    }

    /// The Run Program command lines in force (D67), (added, finished): empty for one that is
    /// switched off (`GuiOptions::run_commands`, without building the whole `GuiOptions`).
    pub fn run_commands(&self) -> (&str, &str) {
        (
            if self.run_added_on { self.run_added.trim() } else { "" },
            if self.run_finished_on { self.run_finished.trim() } else { "" },
        )
    }

    /// Writes these preferences into `g`, leaving the desktop-only options (tray, balloons,
    /// window activation and title, updates, the trackers added to new downloads, the Run
    /// Program lines, the IP filter's list from an address) as they are: a host's options with
    /// the Web UI's changes on them.
    pub fn apply_to(&self, g: &mut GuiOptions) {
        let mine = self.gui();
        *g = GuiOptions {
            alternate_rows: g.alternate_rows,
            theme: g.theme.clone(),
            title_speed: g.title_speed,
            check_association: g.check_association,
            close_to_tray: g.close_to_tray,
            minimize_to_tray: g.minimize_to_tray,
            balloons: g.balloons,
            activate_on_add: g.activate_on_add,
            check_updates: g.check_updates,
            update_beta: g.update_beta,
            // the host's own list (D66): a browser never changes it
            trackers_append: g.trackers_append,
            trackers_list: g.trackers_list.clone(),
            trackers_from_url: g.trackers_from_url,
            trackers_url: g.trackers_url.clone(),
            // the host's own programs (D67): a browser never changes them
            run_added_on: g.run_added_on,
            run_added: g.run_added.clone(),
            run_finished_on: g.run_finished_on,
            run_finished: g.run_finished.clone(),
            // the host's own block list address (D75): a browser never changes it
            ip_filter_from_url: g.ip_filter_from_url,
            ip_filter_url: g.ip_filter_url.clone(),
            ..mine
        };
    }

    pub fn set_gui(&mut self, g: &GuiOptions) {
        self.save_path = g.download_dir.clone();
        self.move_completed = g.move_completed;
        self.completed_dir = g.completed_dir.clone();
        self.move_only_default = g.move_only_default;
        self.move_add_label = g.move_add_label;
        self.torrent_folders = g.torrent_folders.clone();
        self.confirm_delete = g.confirm_delete;
        self.start_paused = g.add_stopped;
        self.show_add_dialog = g.show_add_dialog;
        self.random_port = g.random_port;
        self.pex = g.pex;
        self.max_peers_per_torrent = g.max_peers_per_torrent;
        self.upload_slots_per_torrent = g.upload_slots_per_torrent;
        self.seeding_goal = g.seeding_goal();
        self.ip_filter = g.ip_filter;
        self.ip_filter_path = g.ip_filter_path.clone();
        self.ip_filter_from_url = g.ip_filter_from_url;
        self.ip_filter_url = g.ip_filter_url.clone();
        self.exclude_names = g.exclude_names;
        self.exclude_list = g.exclude_list.clone();
        self.schedule = g.schedule.clone();
        self.check_updates = g.check_updates;
        self.update_beta = g.update_beta;
        self.language = g.language.clone();
        self.trackers_append = g.trackers_append;
        self.trackers_list = g.trackers_list.clone();
        self.trackers_from_url = g.trackers_from_url;
        self.trackers_url = g.trackers_url.clone();
        self.run_added_on = g.run_added_on;
        self.run_added = g.run_added.clone();
        self.run_finished_on = g.run_finished_on;
        self.run_finished = g.run_finished.clone();
    }
}

/// One logged-in client.
pub(crate) struct ClientSession {
    pub last_used: Instant,
    pub maindata: Option<api::Snapshot>,
    pub peers: Option<api::PeersSnapshot>,
}

/// A cached torrent.
pub(crate) struct Entry {
    pub status: TorrentStatus,
    pub hash: String,
    /// Bumped on every change, so `sync/maindata` diffs only changed torrents.
    pub version: u64,
    pub last_activity: i64,
}

/// What a `post_*` call answered.
#[derive(Clone)]
pub(crate) enum Reply {
    Files(Arc<[FileEntry]>),
    Progress(Vec<u64>),
    Trackers(Vec<AnnounceEntry>),
    /// Our pieces, the availability per piece, the pieces being downloaded with their blocks.
    Pieces(Bitfield, Vec<u32>, Vec<PartialPiece>),
    Peers(Vec<PeerInfo>),
    /// The torrent's `.torrent` file (`post_torrent_file`); `None` for a magnet whose metadata
    /// is not in yet. Cached only while an export waits for it, and taken out of the cache when
    /// the last one is done (`commands::torrent_file`, N4).
    TorrentFile(Option<Arc<[u8]>>),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum ReplyKind {
    Files,
    Progress,
    Trackers,
    Pieces,
    Peers,
    TorrentFile,
}

/// One Logger line: the alert's time, the torrent it concerns (none for session-wide ones) and
/// the text, as `rookparcel-nox` prints it.
pub(crate) struct LogLine {
    pub seq: u64,
    pub time: i64,
    pub torrent: Option<TorrentId>,
    pub text: String,
}

/// The Speed tab's history of one torrent, sampled once per status tick while some client
/// looks at it ([`State::watch_speed`]), and for [`SPEED_IDLE_TICKS`] after.
pub(crate) struct SpeedRing {
    /// (download, upload) payload bytes per second, oldest first.
    pub samples: VecDeque<(u32, u32)>,
    /// Samples pushed so far; the newest has this sequence number.
    pub pushed: u64,
    /// The tick a client last asked for it.
    pub last_seen: u64,
}

/// Torrent metadata the engine status does not carry, for torrents added through the API.
#[derive(Clone, Debug, Default)]
pub(crate) struct Meta {
    pub comment: String,
    pub created_by: String,
    pub creation_date: Option<i64>,
    pub private: bool,
}

pub(crate) struct State {
    pub torrents: HashMap<TorrentId, Entry>,
    pub by_hash: HashMap<String, TorrentId>,
    /// `InfoHashes::key_hex` (the host's and the preferences' key, `torrent.<key>.*`) to id;
    /// it differs from [`torrent::hash_of`] for a v2-only torrent.
    pub by_key: HashMap<String, TorrentId>,
    /// A desktop client hosts the Web UI ([`Config::hosted`]): `options` mirrors its
    /// preferences, the labels and pause marks are kept there as its keys too.
    pub hosted: bool,
    pub next_version: u64,
    pub stats: SessionStats,
    pub settings: Settings,
    pub prefs: Prefs,
    /// Where third-party themes are looked for ([`Shared::settings_dir`]).
    pub settings_dir: PathBuf,
    pub username: String,
    pub password: PasswordHash,
    /// The API key (`webui.api_key`, hashed like the password), when one is set.
    pub api_key: Option<PasswordHash>,
    /// The SHA-256 of the last key that verified against `api_key`, so the slow hash runs once
    /// per key, not per request; cleared when the key changes.
    pub api_key_verified: Option<[u8; 32]>,
    /// Category name -> save path.
    pub categories: BTreeMap<String, String>,
    pub categories_version: u64,
    /// Torrent hash -> category.
    pub torrent_categories: HashMap<String, String>,
    /// The known tags (D76), in `tags::cmp` order, at most `tags::MAX_TAGS`: `tags.list` in
    /// `options` and every tag a torrent has. A tag exists without torrents until deleted.
    pub tags: Vec<String>,
    /// Bumped when [`State::tags`] changed: the page's Tags group and `sync/maindata`'s `tags`
    /// / `tags_removed` follow it.
    pub tags_version: u64,
    /// Torrent hash -> its tags (never empty), in `tags::cmp` order. Stored in `options` as
    /// `torrent.<key>.tags` (`webui.json` under nox, the host's preferences hosted), read at
    /// a torrent's first sight.
    pub torrent_tags: HashMap<String, Vec<String>>,
    pub meta: HashMap<String, Meta>,
    /// Torrents with metadata whose status does not say whether they are private (restored
    /// from a record older than 1.2.30) and without an entry in `meta` yet: asked of the engine
    /// a few per status tick ([`META_PER_TICK`]), so their `isPrivate` is right without a
    /// Properties visit.
    pub meta_wanted: VecDeque<TorrentId>,
    /// The stored preferences, as the frontend's preferences keys (what the Win32 client keeps
    /// in `rookparcel.ini`): the engine settings that differ from the defaults as `lt.<name>`
    /// (Rookparcel's own `net.` rows by their names, `settings::pref_key`),
    /// the `GuiOptions` keys, the Web UI's own as `webui.<name>`, and the Add dialog's
    /// recent folders (`add.recent_dirs`). Saved as the `options` object of the state file and
    /// re-applied on the next start.
    pub options: rookparcel_frontend::prefs::Prefs,
    /// The download folder the daemon was started with: the Directories page's default.
    pub default_save_path: String,
    /// The address the server is bound to (the Web UI page shows it).
    pub listen_addr: SocketAddr,
    /// Bumped when the folder options change, so the owner's `MoveCompleted` follows.
    pub gui_version: u64,
    /// The weekly scheduler's mode in force (Preferences > Scheduler is `prefs.schedule`).
    pub scheduler: Scheduler,
    /// The periodic scrape that gives stopped and queued torrents their swarm counts, run on
    /// the status tick.
    pub scrapes: Scrapes,
    /// Stores and moves the `.torrent` copies (`prefs.torrent_folders`), fed every alert, and
    /// watches the auto-load folder on the status tick.
    pub torrent_files: TorrentFiles,
    pub autoload: Poller,
    replies: HashMap<(TorrentId, ReplyKind), (u64, Reply)>,
    serial: u64,
    /// The exports waiting for a torrent's `.torrent` bytes, by torrent: only these are cached
    /// from `Alert::TorrentFile` (the `.torrent` folders and the waiting magnets cause the same
    /// alert, whose bytes must not stay here, N4).
    exports: HashMap<TorrentId, usize>,
    pub sessions: HashMap<String, ClientSession>,
    pub bans: BanList,
    pub shutdown: bool,
    /// Status ticks so far: one per alert batch that carried a state update or session stats.
    /// WebSocket threads send a `tick` when it moved past the one they last sent.
    pub tick: u64,
    /// Torrents removed so far, so a client's list only scans for removals when there were any.
    pub removals: u64,
    /// The last [`LOG_LINES`] alert lines, oldest first.
    pub log: VecDeque<LogLine>,
    log_seq: u64,
    /// Speed history per torrent some client is (or was lately) looking at.
    pub speed: HashMap<TorrentId, SpeedRing>,
    /// Torrents (by hash) the user paused rather than stopped, so the cells say "Paused"
    /// (`frontend::status`); kept in the state file.
    pub user_paused: HashSet<String>,
    /// Torrents (by hash) added with "Add to top of queue", moved there when the engine
    /// reports them added.
    pub pending_top: commands::PendingTop,
    /// Bumped when the RSS Downloader changed (`rss.rs`); a WebSocket `tick` carries it.
    pub rss_version: u64,
    /// Bumped when the search's results or status, or the search sources, changed
    /// (`search.rs`); a WebSocket `tick` carries it.
    pub search_version: u64,
    /// Magnets added through the Add dialog, waiting for their metadata or for the user's
    /// choice (kept in `options`), and a version a `tick` carries as `adds`.
    pub awaiting: rookparcel_frontend::add::AwaitingMetadata,
    pub adds_version: u64,
    /// Magnets added without the Add dialog (the shim, URL adds with the dialog off, RSS),
    /// until their files are known: the ones "Do not download files named" leaves out are set
    /// to Don't Download then (D74). Fed every alert by [`WebUi::on_alerts`]; not kept over a
    /// restart.
    pub excluded: Pending,
    /// The newer Rookparcel nox's update check found (`update.rs`), for the page's status bar;
    /// never set hosted (the host checks).
    pub update: Option<rookparcel_frontend::update::Version>,
}

pub(crate) struct Shared {
    pub handle: SessionHandle,
    pub state: Mutex<State>,
    replied: Condvar,
    pub limits: http::Limits,
    pub iterations: u32,
    pub state_file: Option<PathBuf>,
    /// Where `ipfilter.dat` is looked for when the IP filter has no path: the state directory,
    /// else the working directory.
    pub settings_dir: PathBuf,
    /// The RSS Downloader (hosted, the host's: [`Hosted::rss`]); its lock comes before the
    /// State lock, never after it.
    pub rss: Arc<Mutex<rss::Rss>>,
    /// The torrent search (`search.rs`): one for the server, hosted too; its lock is never
    /// held with the State lock.
    pub(crate) search: Mutex<rookparcel_frontend::search::Search>,
    /// The qBittorrent shim's searches (`search/start`); never held with the State lock or
    /// the search's.
    pub(crate) shim_searches: Mutex<search::ShimSearches>,
    /// The certificate when serving HTTPS.
    pub tls: Option<rookparcel_core::tls::Identity>,
    /// The paths requests may name; fixed at start.
    pub path_policy: path_policy::PathPolicy,
    /// The connected remote sessions' alert queues ([`Shared::fan_out`]).
    pub(crate) remotes: Mutex<Vec<Arc<remote::Client>>>,
    /// The desktop client hosting the Web UI ([`Config::hosted`]), given the preferences
    /// journal at every [`Shared::save`].
    pub(crate) host: Option<Arc<dyn Host>>,
    /// [`Config::helper_dir`].
    pub(crate) helper_dir: Option<PathBuf>,
    /// The search helper's background steps (`helper.rs`); never held with another lock.
    pub(crate) helper: Mutex<helper::Tasks>,
    /// This, for the threads a request starts (the helper's download).
    pub(crate) this: Weak<Shared>,
    /// The trackers added to every new download (D66): the one list of the daemon, configured
    /// from `Prefs::trackers_*`, ticked once a second and sent to the engine when it changes
    /// ([`Shared::trackers_tick`]). `None` hosted: the host owns its own list. Its lock is taken
    /// alone or after the State lock, never before it.
    pub(crate) trackers: Option<Mutex<TrackerList>>,
    /// The IP filter's block list from an `https://` address (D75): configured from
    /// `Prefs::ip_filter*`, ticked with [`Shared::trackers_tick`] ([`Shared::ip_list_tick`]),
    /// a fetched list handed to the engine. `None` hosted: the host keeps its own. Its lock is
    /// taken alone or after the State lock, never before it.
    pub(crate) ip_list: Option<Mutex<IpFilterList>>,
    /// Run Program (D67): the daemon's one starter of the user's programs, fed every alert by
    /// [`WebUi::on_alerts`] and pointed at `Prefs::run_commands` there, so before the first
    /// alert after a commit. `None` hosted: the host runs its own. Only the alert thread takes
    /// this lock, and always before the State lock (never while holding it).
    pub(crate) run_program: Option<Mutex<RunPrograms>>,
    /// [`Config::remote_run_program`]: the command line's alone; nothing changes it later.
    pub(crate) remote_run_program: bool,
}

/// [`Shared::run_program`]: the starter and the command lines it was last given.
pub(crate) struct RunPrograms {
    runner: RunProgram,
    added: String,
    finished: String,
}

impl RunPrograms {
    pub(crate) fn new() -> RunPrograms {
        RunPrograms { runner: RunProgram::new("", ""), added: String::new(), finished: String::new() }
    }
}

/// Hosted, a category is the host's `webui.category.<name>` key (its save folder as the value),
/// so one that no torrent uses yet survives a restart. `%` and `=` in the name are escaped:
/// the preferences file is `key=value` lines.
const CATEGORY_KEY: &str = "webui.category.";

pub(crate) fn category_key(name: &str) -> String {
    format!("{CATEGORY_KEY}{}", name.replace('%', "%25").replace('=', "%3D"))
}

fn category_from_key(key: &str) -> Option<String> {
    let name = key.strip_prefix(CATEGORY_KEY)?.replace("%3D", "=").replace("%25", "%");
    (!name.is_empty()).then_some(name)
}

pub fn unix_now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64)
}

impl Shared {
    pub fn lock(&self) -> MutexGuard<'_, State> {
        // A panicking request thread must not take the whole Web UI down with it.
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub(crate) fn rss(&self) -> MutexGuard<'_, rss::Rss> {
        self.rss.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub(crate) fn search(&self) -> MutexGuard<'_, rookparcel_frontend::search::Search> {
        self.search.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub(crate) fn shim_searches(&self) -> MutexGuard<'_, search::ShimSearches> {
        self.shim_searches.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub(crate) fn helper(&self) -> MutexGuard<'_, helper::Tasks> {
        self.helper.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// The desktop client hosting the Web UI, if one does ([`Config::hosted`]).
    pub(crate) fn hosted(&self) -> Option<&Arc<dyn Host>> {
        self.host.as_ref()
    }

    pub(crate) fn is_hosted(&self) -> bool {
        self.host.is_some()
    }

    /// Posts a request with `post` and waits for the engine's answer.
    pub fn query(&self, id: TorrentId, kind: ReplyKind, post: impl FnOnce(&SessionHandle)) -> Option<Reply> {
        let start = self.lock().serial;
        post(&self.handle);
        let deadline = Instant::now() + REPLY_TIMEOUT;
        let mut st = self.lock();
        loop {
            if let Some((serial, r)) = st.replies.get(&(id, kind)) {
                if *serial > start {
                    return Some(r.clone());
                }
            }
            let now = Instant::now();
            if now >= deadline || !st.torrents.contains_key(&id) {
                return None;
            }
            st = self.replied.wait_timeout(st, deadline - now).unwrap_or_else(|e| e.into_inner()).0;
        }
    }

    /// [`query`](Self::query) for several torrents (Torrent Properties on a whole selection):
    /// the requests go out in groups of `QUERY_GROUP` and the answers are taken as they come.
    /// The torrents that do not answer within `REPLY_TIMEOUT`, all groups together, are left
    /// out.
    pub fn query_many(
        &self,
        ids: &[TorrentId],
        kind: ReplyKind,
        post: impl Fn(&SessionHandle, TorrentId),
    ) -> HashMap<TorrentId, Reply> {
        let deadline = Instant::now() + REPLY_TIMEOUT;
        let mut out = HashMap::new();
        for group in ids.chunks(QUERY_GROUP) {
            let start = self.lock().serial;
            for &id in group {
                post(&self.handle, id);
            }
            let mut waiting = group.to_vec();
            let mut st = self.lock();
            loop {
                waiting.retain(|id| match st.replies.get(&(*id, kind)) {
                    Some((serial, r)) if *serial > start => {
                        out.insert(*id, r.clone());
                        false
                    }
                    _ => st.torrents.contains_key(id),
                });
                let now = Instant::now();
                if waiting.is_empty() || now >= deadline {
                    break;
                }
                st = self.replied.wait_timeout(st, deadline - now).unwrap_or_else(|e| e.into_inner()).0;
            }
            if Instant::now() >= deadline {
                break;
            }
        }
        out
    }

    /// Writes the state file (credentials as a hash, stored preferences, categories). Hosted:
    /// hands the preferences journal (what requests changed in `options` since the last call)
    /// to the host instead; nothing is written.
    pub fn save(&self, st: &mut State) {
        if let Some(host) = &self.host {
            let changes = st.options.take_changes();
            if !changes.is_empty() {
                host.options_changed(changes);
            }
            return;
        }
        let Some(path) = &self.state_file else { return };
        let mut cats = Json::obj();
        for (name, path) in &st.categories {
            cats.set(name, path.as_str());
        }
        let mut tc = Json::obj();
        let mut assigned: Vec<_> = st.torrent_categories.iter().collect();
        assigned.sort();
        for (hash, name) in assigned {
            tc.set(hash, name.as_str());
        }
        let mut paused: Vec<&String> = st.user_paused.iter().collect();
        paused.sort();
        let paused: Vec<Json> = paused.into_iter().map(|h| Json::Str(h.clone())).collect();
        let mut options = Json::obj();
        for (k, v) in st.options.entries() {
            options.set(k, v);
        }
        // the version that wrote the file and its reading of the settings (D70)
        for (k, v) in rookparcel_frontend::guard::stamp_lines(&st.options) {
            options.set(k, v.as_str());
        }
        let doc = Json::obj()
            .with("username", st.username.as_str())
            .with("password", st.password.to_string())
            .with("options", options)
            .with("categories", cats)
            .with("torrent_categories", tc)
            .with("paused", Json::Arr(paused));
        if let Some(dir) = path.parent() {
            let _ = private_file::create_dir_all(dir);
        }
        // Write then rename, so a crash never leaves half a file; owner-only on Unix (the
        // password hash, and the proxy password, which is plain there).
        let _ = private_file::write(path, doc.to_json().as_bytes());
    }

    pub(crate) fn trackers(&self) -> Option<MutexGuard<'_, TrackerList>> {
        self.trackers.as_ref().map(|t| t.lock().unwrap_or_else(|e| e.into_inner()))
    }

    /// The tracker list (D66) follows the preferences: after a commit and at start. Hosted:
    /// nothing (the host's list).
    pub(crate) fn configure_trackers(&self, st: &State) {
        if let Some(mut t) = self.trackers() {
            t.configure(tracker_list::Config::of(&st.prefs.gui()));
        }
    }

    /// Once a second, and right after the preferences changed: a finished fetch of the list is
    /// taken and a due one started (through the session's proxy), its lines go to the Logger,
    /// and a list that is not the one the engine has is sent to it.
    pub(crate) fn trackers_tick(&self, st: &mut State) {
        let Some(mut t) = self.trackers() else { return };
        let log = t.tick(|| FetchProxy::from_settings(&st.settings));
        let tiers = t.take_tiers();
        drop(t);
        let now = unix_now();
        for line in log {
            st.push_log(None, line, now);
        }
        if let Some(tiers) = tiers {
            self.handle.set_extra_trackers(tiers);
        }
    }

    pub(crate) fn ip_list(&self) -> Option<MutexGuard<'_, IpFilterList>> {
        self.ip_list.as_ref().map(|l| l.lock().unwrap_or_else(|e| e.into_inner()))
    }

    /// The IP filter's list from an address (D75) follows the preferences: at start, before
    /// the filter is loaded, and with a commit (a kept list of another address stays on disk,
    /// unused, until one of the new address is saved).
    /// Hosted: nothing (the host's list).
    pub(crate) fn configure_ip_list(&self, p: &Prefs) {
        if let Some(mut l) = self.ip_list() {
            l.configure(ip_filter_list::Config::of(&p.gui()));
        }
    }

    /// With [`Shared::trackers_tick`]: a finished fetch of the block list is taken and a due one
    /// started (through the session's proxy, held while the D70 guard holds the transfers), its
    /// lines go to the Logger, and a new list goes to the engine.
    pub(crate) fn ip_list_tick(&self, st: &mut State) {
        let Some(mut l) = self.ip_list() else { return };
        let log = l.tick(|| FetchProxy::from_settings(&st.settings));
        let filter = l.take_filter();
        drop(l);
        if let Some(f) = filter {
            self.handle.set_ip_filter(f);
        }
        let now = unix_now();
        for line in log {
            st.push_log(None, line, now);
        }
    }

    /// Applies the global limits for the current alternative-speed mode, through the
    /// scheduler (its Limited hours override them). Hosted: nothing (the host applies its
    /// settings itself; a browser's limits reach it through the journal).
    pub fn apply_limits(&self, st: &mut State) {
        if self.is_hosted() {
            return;
        }
        let (dl, up) = if st.prefs.alt_enabled {
            (st.prefs.alt_dl_limit, st.prefs.alt_up_limit)
        } else {
            (st.prefs.dl_limit, st.prefs.up_limit)
        };
        st.settings.download_rate_limit = dl;
        st.settings.upload_rate_limit = up;
        self.run_scheduler(st, true);
    }

    /// Puts the scheduler's mode for this hour into effect; with `force` the settings are sent
    /// even if the mode stays (they changed). Hosted: nothing (the scheduler is the host's, so
    /// the mode here stays Normal).
    pub(crate) fn run_scheduler(&self, st: &mut State, force: bool) {
        if self.is_hosted() {
            return;
        }
        let now = unix_now();
        let offset = local_utc_offset(now);
        let old = st.scheduler.mode();
        let Some(c) = st.scheduler.update(&st.prefs.schedule, &st.settings, now, offset, force) else {
            return;
        };
        if let Some(s) = c.settings {
            self.handle.apply_settings(s);
        }
        match c.pause_session {
            Some(true) => self.handle.pause(),
            Some(false) => self.handle.resume(),
            None => {}
        }
        if let Some(on) = c.upload_mode {
            for id in st.torrents.keys() {
                let t = self.handle.torrent(*id);
                if on {
                    t.set_flags(TorrentFlags::UPLOAD_MODE);
                } else if !st.awaiting.holds(*id) {
                    // a magnet waiting for the Add dialog stays in upload mode until its OK
                    t.unset_flags(TorrentFlags::UPLOAD_MODE);
                }
            }
        }
        if c.mode != old {
            st.push_log(None, tr!(LogScheduler, mode = c.mode.label()), now);
        }
    }
}

/// The running Web UI. Dropping it stops the server.
pub struct WebUi {
    shared: Arc<Shared>,
    addr: SocketAddr,
    stop: Arc<AtomicBool>,
    acceptor: Option<JoinHandle<()>>,
    temporary_password: Option<String>,
    https: Option<HttpsInfo>,
}

/// The certificate an HTTPS Web UI presents, for the start-up message.
#[derive(Clone, Debug)]
pub struct HttpsInfo {
    pub fingerprint: rookparcel_core::tls::Fingerprint,
    /// Unix seconds.
    pub not_after: i64,
    /// The certificate file (none for an in-memory one).
    pub cert_path: Option<PathBuf>,
    /// The self-signed certificate was made (or renewed) by this start.
    pub generated: bool,
}

/// The engine settings stored in `state_dir`'s `webui.json`, put over `s`; for nox to call
/// before it starts the engine, so the engine never runs with defaults the owner switched off
/// (UPnP and NAT-PMP mapping the router's ports, the DHT bootstrapping). [`WebUi::start`] applies
/// the same entries again, with everything else it restores. Returns what the stored settings
/// ask for at start (D70: an unreadable proxy type holds the transfers); an error, for nox to
/// stop on, when the file is there but cannot be read ([`WebUi::start`] refuses it too).
pub fn stored_settings(
    state_dir: &std::path::Path,
    s: &mut Settings,
) -> Result<rookparcel_frontend::guard::StartCheck, String> {
    let file = state_dir.join(STATE_FILE);
    let stored = read_state(&file)?;
    prefs::stored_engine_settings(&stored, s);
    let mut options = rookparcel_frontend::prefs::Prefs::default();
    for (k, v) in stored.get("options").map(Json::entries).unwrap_or_default() {
        if let Some(v) = v.as_str() {
            options.set(k, v);
        }
    }
    prefs::migrate_keys(&mut options);
    // a version change keeps a copy of the file before anything is written to it
    Ok(rookparcel_frontend::guard::on_start_for(&mut options, Some(&file)))
}

/// The state file at `path`: an empty object when there is none. A file that cannot be read or
/// is not a JSON object is an error naming it, so that the start stops instead of saving over
/// the settings, labels and login it may still hold.
fn read_state(path: &std::path::Path) -> Result<Json, String> {
    let damaged = |why: &dyn std::fmt::Display| {
        format!(
            "cannot read {}: {why}. Fix the file or move it away; nox did not start, so that it would not \
             overwrite it.",
            path.display()
        )
    };
    // a file another program holds for a moment (a virus scanner) is tried again a few times
    let mut tries = 0;
    let text = loop {
        match std::fs::read_to_string(path) {
            Ok(t) => break t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Json::obj()),
            Err(e) if e.kind() == std::io::ErrorKind::InvalidData => return Err(damaged(&e)),
            Err(e) => {
                tries += 1;
                if tries == 5 {
                    return Err(damaged(&e));
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    };
    // a byte order mark, as some editors write one
    match json::parse(text.strip_prefix('\u{feff}').unwrap_or(&text)) {
        Ok(doc @ Json::Obj(_)) => Ok(doc),
        Ok(_) => Err(damaged(&"not a JSON object")),
        Err(e) => Err(damaged(&e)),
    }
}

impl WebUi {
    /// Binds `config.addr` and starts serving. A state file that is there but cannot be read is
    /// an `InvalidData` error naming it, and nothing is written.
    pub fn start(handle: SessionHandle, config: Config) -> std::io::Result<WebUi> {
        // hosted: no state file; the login, labels and pause marks come from the host's
        // preferences (`torrent.<key>.*`, the host's keys)
        let hosted = config.hosted.as_ref();
        let state_file = match hosted {
            Some(_) => None,
            None => config.state_dir.as_ref().map(|d| d.join(STATE_FILE)),
        };
        // a damaged file stops the start before anything is saved over it
        let stored = match &state_file {
            Some(p) => read_state(p).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
            None => Json::obj(),
        };
        let stored_login = |key: &str| -> Option<String> {
            match hosted {
                Some(h) => h.options.get(&format!("webui.{key}")).map(str::to_owned),
                None => stored.get(key).and_then(Json::as_str).map(str::to_owned),
            }
        };

        let mut temporary_password = None;
        let password = match (&config.password, stored_login("password")) {
            (Some(p), _) => PasswordHash::new(p, config.pbkdf2_iterations),
            (None, Some(h)) if PasswordHash::parse(&h).is_some() => PasswordHash::parse(&h).expect("checked"),
            _ => {
                let p = auth::random_hex(6);
                let h = PasswordHash::new(&p, config.pbkdf2_iterations);
                temporary_password = Some(p);
                h
            }
        };
        // A stored user name is the one set through setPreferences; the command line wins when
        // it gives a password too.
        let username = match stored_login("username") {
            Some(u) if config.password.is_none() && temporary_password.is_none() => u,
            _ => config.username.clone(),
        };
        let mut categories = BTreeMap::new();
        for (name, path) in stored.get("categories").map(Json::entries).unwrap_or_default() {
            categories.insert(name.clone(), path.as_str().unwrap_or_default().to_owned());
        }
        let mut torrent_categories = HashMap::new();
        for (hash, name) in stored.get("torrent_categories").map(Json::entries).unwrap_or_default() {
            if let Some(name) = name.as_str() {
                torrent_categories.insert(hash.clone(), name.to_owned());
            }
        }
        let user_paused: HashSet<String> = match stored.get("paused") {
            Some(Json::Arr(v)) => v.iter().filter_map(Json::as_str).map(str::to_owned).collect(),
            _ => HashSet::new(),
        };
        // the stored preferences (the frontend's keys), and the older `recent_dirs` list; hosted,
        // the host's preferences, mirrored (its labels make the label list complete at once)
        let mut options = match hosted {
            Some(h) => {
                let mut o = h.options.clone();
                o.track_changes();
                for (k, v) in o.entries() {
                    if k.starts_with("torrent.") && k.ends_with(".label") && !v.is_empty() {
                        categories.entry(v.to_owned()).or_default();
                    } else if let Some(name) = category_from_key(k) {
                        categories.insert(name, v.to_owned());
                    }
                }
                o
            }
            None => rookparcel_frontend::prefs::Prefs::default(),
        };
        for (k, v) in stored.get("options").map(Json::entries).unwrap_or_default() {
            if let Some(v) = v.as_str() {
                options.set(k, v);
            }
        }
        if hosted.is_none() {
            // a goal stored before 1.2.45 (the host carries its own over)
            rookparcel_frontend::seeding::migrate(&mut options);
            // the network rule's rows as nox up to 1.2.54 stored them (`lt.net.*`)
            prefs::migrate_keys(&mut options);
        }
        if let Some(Json::Arr(v)) = stored.get("recent_dirs") {
            // most recent first in the file; remember_dir puts each first, so go backwards
            for d in v.iter().rev().filter_map(Json::as_str) {
                rookparcel_frontend::add::remember_dir(&mut options, d);
            }
        }
        // the known tags (D76): `tags.list` only, as the GUIs read it. A torrent's
        // `torrent.<key>.tags` (an import's, an add's) makes its names known at its first
        // sight; a key of a torrent that is gone must not bring a deleted tag back
        let known_tags = tags::load_known(&options);
        // hosted: the host handles its pending magnets after a restart
        let awaiting = match hosted {
            Some(_) => rookparcel_frontend::add::AwaitingMetadata::default(),
            None => rookparcel_frontend::add::AwaitingMetadata::load(&options),
        };
        let settings = config.settings.clone();
        let mut prefs = Prefs::defaults(&config.save_path, &settings);
        if let Some(d) = &config.move_completed_dir {
            prefs.move_completed = true;
            prefs.completed_dir = d.clone();
            prefs.move_only_default = false;
        }
        if let Some(p) = &config.ip_filter {
            prefs.ip_filter = true;
            prefs.ip_filter_path = p.to_string_lossy().into_owned();
        }
        // the certificate before the socket: a broken one fails the start without binding
        let loaded = match &config.https {
            Some(source) => Some(https::load(source, unix_now())?),
            None => None,
        };
        let https = loaded.as_ref().map(|l| HttpsInfo {
            fingerprint: l.identity.fingerprint(),
            not_after: l.identity.info().not_after,
            cert_path: l.cert_path.clone(),
            generated: l.generated,
        });
        let listener = std::net::TcpListener::bind(config.addr)?;
        let addr = listener.local_addr()?;
        let settings_dir = config.state_dir.clone().unwrap_or_else(|| PathBuf::from("."));
        let shared = Arc::new_cyclic(|this| Shared {
            handle,
            state: Mutex::new(State {
                settings_dir: settings_dir.clone(),
                torrents: HashMap::new(),
                by_hash: HashMap::new(),
                by_key: HashMap::new(),
                hosted: hosted.is_some(),
                next_version: 1,
                stats: SessionStats::default(),
                settings,
                prefs,
                username,
                password,
                api_key: None,
                api_key_verified: None,
                categories,
                categories_version: 1,
                torrent_categories,
                tags: known_tags,
                tags_version: 1,
                torrent_tags: HashMap::new(),
                meta: HashMap::new(),
                meta_wanted: VecDeque::new(),
                options,
                default_save_path: config.save_path.clone(),
                listen_addr: addr,
                gui_version: 1,
                scheduler: Scheduler::new(),
                scrapes: Scrapes::default(),
                torrent_files: TorrentFiles::new(),
                autoload: Poller::new(),
                replies: HashMap::new(),
                serial: 0,
                exports: HashMap::new(),
                sessions: HashMap::new(),
                bans: BanList::default(),
                shutdown: false,
                tick: 0,
                removals: 0,
                log: VecDeque::new(),
                log_seq: 0,
                speed: HashMap::new(),
                user_paused,
                pending_top: HashSet::new(),
                rss_version: 1,
                search_version: 1,
                awaiting,
                adds_version: 1,
                excluded: Pending::new(),
                update: None,
            }),
            replied: Condvar::new(),
            limits: config.limits,
            iterations: config.pbkdf2_iterations,
            state_file,
            settings_dir,
            // rss.dat beside webui.json, in the config directory; hosted, the host's downloader
            rss: match hosted {
                Some(h) => h.rss.clone(),
                None => Arc::new(Mutex::new(rss::Rss::load(
                    config.state_dir.as_ref().map(|d| d.join(rookparcel_frontend::rss::store::FILE_NAME)),
                ))),
            },
            search: Mutex::new(search::new_search()),
            shim_searches: Mutex::default(),
            tls: loaded.map(|l| l.identity),
            path_policy: path_policy::PathPolicy::new(config.save_roots.clone()),
            remotes: Mutex::new(Vec::new()),
            host: hosted.map(|h| h.host.clone()),
            helper_dir: config.helper_dir.clone(),
            helper: Mutex::default(),
            this: this.clone(),
            // trackers-url.txt beside webui.json; none without a config directory (--no-state);
            // hosted, the host's own list
            trackers: match hosted {
                Some(_) => None,
                None => Some(Mutex::new(TrackerList::new(
                    config.state_dir.as_ref().map(|d| d.join(tracker_list::CACHE_FILE)),
                    tracker_list::Config::default(),
                ))),
            },
            // ipfilter-url.dat and .txt beside webui.json; none kept without a config directory;
            // hosted, the host's own list. Configured by `prefs::restore`.
            ip_list: match hosted {
                Some(_) => None,
                None => Some(Mutex::new(IpFilterList::new(
                    config.state_dir.clone(),
                    ip_filter_list::Config::default(),
                ))),
            },
            // hosted, the host starts its own programs (D67): none here
            run_program: match hosted {
                Some(_) => None,
                None => Some(Mutex::new(RunPrograms::new())),
            },
            remote_run_program: config.remote_run_program && hosted.is_none(),
        });
        // the helper's source is routed to the running helper from the first search on
        helper::configure(&shared);
        {
            let mut st = shared.lock();
            if st.hosted {
                // the host's settings and options, read back for the Preferences pages; the
                // host applied them itself, and its language is the process's already
                prefs::read_back(&mut st);
                // the mirror shows the login in force (not journaled: the host gave it, or it
                // knows the password it passed and reads `temporary_password`)
                let login = [
                    ("webui.username".to_owned(), Some(st.username.clone())),
                    ("webui.password".to_owned(), Some(st.password.to_string())),
                ];
                st.options.apply_changes(&login);
            } else {
                // what the Web UI's preferences changed before this start, re-applied; the
                // `preferences` object of state files written before slice 6 holds the shim's
                // keys
                prefs::restore(&shared, &mut st, config.ip_filter.as_deref());
                // the trackers added to new downloads (D66): the engine has the typed list (and
                // the cached one) before anything is added; a due fetch starts here too
                shared.configure_trackers(&st);
                shared.trackers_tick(&mut st);
                // the IP filter's list from an address (D75), configured by `restore`: a due
                // fetch starts here (the kept copy is in force already)
                shared.ip_list_tick(&mut st);
                // the Web UI's language (Preferences > General); ROOKPARCEL_LANG over it
                for p in rookparcel_frontend::lang::init(&st.prefs.language) {
                    st.push_log(None, p, unix_now());
                }
                // D70: what the stored settings asked for at start (nox started the engine
                // with the transfers held if so)
                // (and this file's stamp, written at every save, D70)
                for line in rookparcel_frontend::guard::on_start_for(&mut st.options, None).log {
                    st.push_log(None, line, unix_now());
                }
                if let Some(legacy @ Json::Obj(_)) = stored.get("preferences") {
                    api::apply_preferences(&shared, &mut st, legacy, None);
                }
                // keeps the stored file current (a new password hash, a first start)
                shared.save(&mut st);
            }
        }
        let stop = Arc::new(AtomicBool::new(false));
        let acceptor = server::spawn(listener, shared.clone(), stop.clone())?;
        // the adapters for Preferences > Connection (D54)
        shared.handle.post_network_adapters();
        Ok(WebUi { shared, addr, stop, acceptor: Some(acceptor), temporary_password, https })
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.addr
    }

    /// The certificate, when serving HTTPS.
    pub fn https(&self) -> Option<&HttpsInfo> {
        self.https.as_ref()
    }

    /// The page's address: `http://` or `https://`, with a loopback host for a wildcard
    /// listen address.
    pub fn url(&self) -> String {
        let scheme = if self.https.is_some() { "https" } else { "http" };
        let ip = match self.addr.ip() {
            IpAddr::V4(ip) if ip.is_unspecified() => IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
            IpAddr::V6(ip) if ip.is_unspecified() => IpAddr::V6(std::net::Ipv6Addr::LOCALHOST),
            ip => ip,
        };
        format!("{scheme}://{}/", SocketAddr::new(ip, self.addr.port()))
    }

    /// The random password made because none was given or stored.
    pub fn temporary_password(&self) -> Option<&str> {
        self.temporary_password.as_deref()
    }

    /// `run --import`: the imported torrents' labels (by hash; a torrent with one already keeps
    /// it), their tags (by `InfoHashes::key_hex`, the import's `torrent.<key>.tags`: added to
    /// those a torrent has, made known; D76), their µTorrent Paused marks and the Logger lines,
    /// saved at once.
    pub fn assign_imported(
        &self,
        labels: &[(String, String)],
        tags: &[(String, Vec<String>)],
        paused: &[String],
        log: &[String],
    ) {
        let mut st = self.shared.lock();
        for (hash, label) in labels {
            if st.torrent_categories.contains_key(hash) {
                continue;
            }
            st.set_label(hash, label);
        }
        for (key, names) in tags {
            // a v1 torrent's key is its hash; another one is found by its key
            let _ = st.add_tags_keyed(key, Some(key), names);
        }
        for hash in paused {
            st.set_user_paused(hash, true);
        }
        let now = unix_now();
        for line in log {
            st.push_log(None, line.clone(), now);
        }
        self.shared.save(&mut st);
    }

    /// True once a client called `app/shutdown`.
    pub fn shutdown_requested(&self) -> bool {
        self.shared.lock().shutdown
    }

    /// Reads the IP filter's block list again (the daemon's SIGHUP) when the filter is on: the
    /// BitTorrent page's file, else `ipfilter.dat` in the state directory; with the list from an
    /// address (D75), the copy kept of it (nothing while none was fetched from that address:
    /// nothing is fetched here). A list that does not load leaves the old rules in force.
    /// Returns the line it logged.
    pub fn reload_ip_filter(&self) -> Option<String> {
        let file = {
            let st = self.shared.lock();
            if !st.prefs.ip_filter {
                return None;
            }
            ip_filter_list::list_file(&st.prefs.gui(), &self.shared.settings_dir)?
        };
        // read without the State lock (SEC-05)
        let line = match self.shared.handle.load_ip_filter(&file) {
            Ok(n) => tr!(LogIpFilterRules, count = n, path = file.display()),
            Err(e) => tr!(LogIpFilterKept, path = file.display(), error = e),
        };
        self.shared.lock().push_log(None, line.clone(), unix_now());
        Some(line)
    }

    /// Points the owner's "Move completed downloads" helper at the Directories preferences
    /// when they changed since the last call (`seen` is the caller's copy of the version).
    /// Returns whether it changed anything.
    pub fn sync_move_completed(&self, seen: &mut u64, current: &mut Option<MoveCompleted>) -> bool {
        let st = self.shared.lock();
        if st.gui_version == *seen {
            return false;
        }
        *seen = st.gui_version;
        st.prefs.gui().update_move_completed(current);
        true
    }

    /// A torrent's label (`None`: none, or not in the lists yet), for "Append the torrent's
    /// label" ([`MoveCompleted::on_alert`]).
    pub fn label_of(&self, id: TorrentId) -> Option<String> {
        let st = self.shared.lock();
        st.torrents.get(&id).and_then(|e| st.torrent_categories.get(&e.hash)).cloned()
    }

    /// Hosted ([`Config::hosted`]): the host changed its RSS Downloader ([`Hosted::rss`])
    /// itself (a fetch came in, the user edited a feed or a filter): the pages reload it after
    /// the next tick. Takes the Web UI's lock only, so the RSS lock may be held.
    pub fn rss_changed(&self) {
        self.shared.lock().rss_version += 1;
    }

    /// Hosted ([`Config::hosted`]): the host's preferences changed (its journal,
    /// `Prefs::take_changes`); the mirror follows. A label or a pause mark updates the lists
    /// (clients see it at the next tick), the login replaces the one in force, settings and
    /// option keys refresh what the Preferences pages show. Nothing goes to the engine: the
    /// host applied it.
    pub fn options_changed(&self, changes: &[rookparcel_frontend::prefs::Change]) {
        let mut st = self.shared.lock();
        if !st.hosted {
            return;
        }
        st.options.apply_changes(changes);
        let mut other = false;
        let mut known_changed = false;
        if changes.iter().any(|(k, _)| k.starts_with("search.")) {
            // the host's search sources: an open search window reloads them
            st.search_version += 1;
        }
        for (key, value) in changes {
            if let Some(list) = st.host_tags_change(key, value.as_deref()) {
                // `tags.list` is read once every change is in: a deleted tag leaves the
                // torrents first
                known_changed |= list;
                continue;
            }
            if let Some(k) = key.strip_prefix("torrent.") {
                if let Some(k) = k.strip_suffix(".label") {
                    let label = value.as_deref().unwrap_or("");
                    match st.by_key.get(k).and_then(|id| st.torrents.get(id)).map(|e| e.hash.clone()) {
                        Some(hash) => st.label_cached(&hash, label),
                        // not seen yet: the label joins the list now, the torrent takes it
                        // at its first sight
                        None if !label.is_empty() && !st.categories.contains_key(label) => {
                            st.categories.insert(label.to_owned(), String::new());
                            st.categories_version += 1;
                        }
                        None => {}
                    }
                } else if let Some(k) = k.strip_suffix(".paused") {
                    let Some(hash) =
                        st.by_key.get(k).and_then(|id| st.torrents.get(id)).map(|e| e.hash.clone())
                    else {
                        continue;
                    };
                    st.paused_cached(&hash, value.as_deref() == Some("1"));
                } else if let Some(k) = k.strip_suffix(".seed_ratio").or_else(|| k.strip_suffix(".seed_time"))
                {
                    // the override is read from `options`; `sync/maindata` resends the torrent's
                    // `ratio_limit` / `seeding_time_limit` only when its version moves
                    let hash = st.by_key.get(k).and_then(|id| st.torrents.get(id)).map(|e| e.hash.clone());
                    if let Some(hash) = hash {
                        st.touch(&hash);
                    }
                }
                continue;
            }
            match key.as_str() {
                "webui.username" => {
                    if let Some(u) = value.as_deref().map(str::trim).filter(|u| !u.is_empty()) {
                        st.username = u.to_owned();
                    }
                }
                "webui.password" => {
                    let Some(h) = value.as_deref().and_then(PasswordHash::parse) else { continue };
                    if h.to_string() != st.password.to_string() {
                        st.password = h;
                        // every login ends: the sessions, their WebSockets, the remotes (SEC-24)
                        st.sessions.clear();
                        self.shared.end_remotes();
                    }
                }
                _ => other = true,
            }
        }
        if known_changed {
            // the host's known tags (New Tag, Delete Tag), plus every tag a torrent has
            st.reload_known_tags();
        }
        if other {
            // the Preferences pages show the host's values
            prefs::read_back(&mut st);
        }
    }

    /// Feeds a batch of alerts popped from the session.
    pub fn on_alerts(&self, alerts: &[Alert]) {
        self.shared.fan_out(alerts);
        // the adapters and the network status (D54), for the preferences and the status bar
        for a in alerts {
            rookparcel_frontend::network::on_alert(a);
        }
        // the search's answers, taken before the State lock (never held together)
        let searched = search::collect(&self.shared);
        search::collect_shim(&self.shared);
        let mut st = self.shared.lock();
        if searched {
            st.search_version += 1;
        }
        let mut replied = false;
        let mut ticked = false;
        let now = unix_now();
        // hosted: the automations (`.torrent` folders, seeding goal, scheduler, scrapes,
        // auto-load, RSS) are the host's; the cache, the replies and the Web UI's own adds stay
        let hosted = st.hosted;
        let folders = st.prefs.torrent_folders.clone();
        for a in alerts {
            if !hosted {
                for line in st.torrent_files.on_alert(a, &folders, &self.shared.handle) {
                    st.push_log(None, line, now);
                }
            }
            // magnets added without the Add dialog: their files once known (D74); hosted too,
            // for the Web UI's own adds
            st.excluded.on_alert(a, &self.shared.handle);
            let left = matches!(a, Alert::TorrentRemoved { id, .. } if st.awaiting.holds(*id));
            let s = &mut *st;
            if s.awaiting.on_alert(a, &self.shared.handle, &mut s.options) || left {
                s.adds_version += 1;
            }
            if left {
                self.shared.save(&mut st);
            }
            match a {
                Alert::StateUpdate(v) => {
                    let goal = st.prefs.seeding_goal;
                    for s in v {
                        // first sight of a restored torrent (they post no `TorrentAdded`) during
                        // Seeding only hours
                        if !hosted
                            && !st.torrents.contains_key(&s.id)
                            && st.scheduler.mode() == Mode::SeedOnly
                            && !s.flags.contains(TorrentFlags::UPLOAD_MODE)
                        {
                            self.shared.handle.torrent(s.id).set_flags(TorrentFlags::UPLOAD_MODE);
                        }
                        st.update(s.clone(), now);
                        // Queueing > Seeding Goal: stop or limit a queued seed past its goal
                        if s.is_finished && !hosted {
                            let t = self.shared.handle.torrent(s.id);
                            if let Some(line) = goal.run(&mut st.options, s, &t) {
                                st.push_log(Some(s.id), line, now);
                                // the mark of the goal done
                                self.shared.save(&mut st);
                            }
                        }
                    }
                    ticked = true;
                }
                Alert::TorrentAdded { id, info_hashes, name } => {
                    if !st.torrents.contains_key(id) {
                        let s = TorrentStatus {
                            id: *id,
                            info_hashes: *info_hashes,
                            name: name.clone(),
                            ..TorrentStatus::default()
                        };
                        st.update(s, now);
                    }
                    // added by anyone during Seeding only hours (the host's scheduler, hosted)
                    if !hosted && st.scheduler.mode() == Mode::SeedOnly {
                        self.shared.handle.torrent(*id).set_flags(TorrentFlags::UPLOAD_MODE);
                    }
                    // "Add to top of queue" from the Add dialog, now that the id is known
                    if st.pending_top.remove(&torrent::hash_of(info_hashes)) {
                        self.shared.handle.torrent(*id).queue_position(rookparcel_core::QueueMove::Top);
                    }
                }
                Alert::TorrentRemoved { id, .. } => {
                    if let Some(e) = st.torrents.remove(id) {
                        st.by_hash.remove(&e.hash);
                        st.meta.remove(&e.hash);
                        st.removals += 1;
                        let key = e.status.info_hashes.key_hex();
                        st.by_key.remove(&key);
                        let had_goal = SeedGoal::forget(&mut st.options, &key);
                        let forget = st.torrent_categories.remove(&e.hash).is_some() | had_goal;
                        // its tags leave with it (D76); the tags stay known
                        st.torrent_tags.remove(&e.hash);
                        let tags_key = tags::torrent_key(&key);
                        let forget = forget | st.options.get(&tags_key).is_some();
                        st.options.remove(&tags_key);
                        if hosted {
                            // the host drops its own keys too; removing an absent key is not
                            // journaled, and an empty journal is not handed on
                            st.options.remove(&format!("torrent.{key}.label"));
                            st.options.remove(&format!("torrent.{key}.paused"));
                        }
                        if st.user_paused.remove(&e.hash) | forget | hosted {
                            self.shared.save(&mut st);
                        }
                    }
                    st.replies.retain(|(t, _), _| t != id);
                }
                Alert::SessionStats(s) => {
                    st.stats = *s;
                    ticked = true;
                }
                Alert::FileList { id, files } => {
                    st.reply(*id, ReplyKind::Files, Reply::Files(files.clone()));
                    replied = true;
                }
                Alert::FileProgress { id, done } => {
                    st.reply(*id, ReplyKind::Progress, Reply::Progress(done.clone()));
                    replied = true;
                }
                Alert::TrackerList { id, trackers } => {
                    st.reply(*id, ReplyKind::Trackers, Reply::Trackers(trackers.clone()));
                    replied = true;
                }
                Alert::PieceInfo { id, have, availability, downloading } => {
                    let r = Reply::Pieces(have.clone(), availability.clone(), downloading.clone());
                    st.reply(*id, ReplyKind::Pieces, r);
                    replied = true;
                }
                Alert::PeerInfo { id, peers } => {
                    st.reply(*id, ReplyKind::Peers, Reply::Peers(peers.clone()));
                    replied = true;
                }
                // Export .torrent (`GET /pa/torrents/export`, the shim's `torrents/export`); the
                // `.torrent` folders and the waiting magnets above see the same alert
                Alert::TorrentFile { id, data } => replied |= st.torrent_file_answer(*id, data),
                _ => {}
            }
            if logged(a) {
                st.log_line(a, now);
            }
        }
        let mut found = Vec::new();
        if ticked {
            st.tick += 1;
            st.sample_speeds();
        }
        if ticked && !hosted {
            self.shared.run_scheduler(&mut st, false);
            // no scrapes while the scheduler has the session turned off
            if st.scheduler.mode() != Mode::Off {
                let s = &mut *st;
                s.scrapes.run(s.torrents.values().map(|e| &e.status), Instant::now(), &self.shared.handle);
            }
            // the tracker list (D66) before this tick's adds (the auto-load folder, RSS)
            self.shared.trackers_tick(&mut st);
            // the IP filter's list from an address (D75)
            self.shared.ip_list_tick(&mut st);
            found = autoload(&mut st, &self.shared.path_policy, now);
        }
        let meta_wanted: Vec<TorrentId> = if ticked {
            let n = st.meta_wanted.len().min(META_PER_TICK);
            st.meta_wanted.drain(..n).collect()
        } else {
            Vec::new()
        };
        drop(st);
        if replied {
            self.shared.replied.notify_all();
        }
        // restored torrents' private flag (`isPrivate`), asked outside the State lock
        for id in meta_wanted {
            commands::meta(&self.shared, id);
        }
        for (path, source) in found {
            if let Err(e) = commands::add(&self.shared, source, &commands::AddOptions::default()) {
                let line = tr!(FoldersNotLoaded, path = path.display(), error = e.msg);
                self.shared.lock().push_log(None, line, now);
            }
        }
        if ticked && !hosted {
            rss::tick(&self.shared);
        }
        // last: the cache knows the batch's torrents and their labels by now
        self.run_programs(alerts);
    }

    /// Run Program (D67): points the daemon's starter at the command lines in force, then
    /// shows it every alert of the batch; it starts the programs itself. Its lines (a program
    /// started, ended, failed) go to the Logger and to the daemon's output. Hosted: nothing
    /// (the host runs its own).
    fn run_programs(&self, alerts: &[Alert]) {
        let Some(run) = &self.shared.run_program else { return };
        let mut run = run.lock().unwrap_or_else(|e| e.into_inner());
        let changed = {
            let st = self.shared.lock();
            let (added, finished) = st.prefs.run_commands();
            ((added, finished) != (run.added.as_str(), run.finished.as_str()))
                .then(|| (added.to_owned(), finished.to_owned()))
        };
        if let Some((added, finished)) = changed {
            run.runner.set(&added, &finished);
            run.added = added;
            run.finished = finished;
        }
        let mut lines = Vec::new();
        for a in alerts {
            // the label is asked only when a program is about to run (the State lock, briefly)
            lines.extend(run.runner.on_alert(a, |id| self.label_of(id)));
        }
        drop(run);
        if lines.is_empty() {
            return;
        }
        let now = unix_now();
        let mut st = self.shared.lock();
        for line in lines {
            println!("* {line}");
            st.push_log(None, line, now);
        }
    }

    /// Whether a Run Program line (D67) is set and switched on, for the warning at start.
    pub fn run_program_enabled(&self) -> bool {
        let st = self.shared.lock();
        let (added, finished) = st.prefs.run_commands();
        !st.hosted && (!added.is_empty() || !finished.is_empty())
    }

    /// Whether the daemon's starter has a program to run: what [`WebUi::on_alerts`] last gave
    /// it. For tests.
    #[doc(hidden)]
    pub fn run_program_armed(&self) -> bool {
        self.shared
            .run_program
            .as_ref()
            .is_some_and(|r| r.lock().unwrap_or_else(|e| e.into_inner()).runner.enabled())
    }

    /// Stops accepting connections (also done on drop). Open connections end on their own
    /// after their current request.
    pub fn stop(mut self) {
        self.stop_now();
    }

    fn stop_now(&mut self) {
        if let Some(t) = self.acceptor.take() {
            self.stop.store(true, Ordering::SeqCst);
            server::wake(self.addr);
            let _ = t.join();
        }
    }
}

impl Drop for WebUi {
    fn drop(&mut self) {
        self.stop_now();
    }
}

/// Preferences > Directories "Automatically load .torrents from": the new files in the folder,
/// to add without the Add New Torrent dialog (they are renamed to `.torrent.loaded`, or
/// deleted, once the engine has them). Files whose torrent is in the session already are
/// finished at once.
///
/// A folder the path policy refuses (one set before `--webui-save-root` was given) is not
/// watched: loaded files may be deleted there (SEC-05).
fn autoload(st: &mut State, policy: &path_policy::PathPolicy, now: i64) -> Vec<(PathBuf, commands::Source)> {
    let Some(dir) = st.prefs.torrent_folders.loads_from().map(PathBuf::from) else { return Vec::new() };
    let paths = match policy.check(&dir.to_string_lossy()) {
        Ok(()) => st.autoload.poll(&dir, Instant::now()),
        Err(why) => {
            st.autoload.refuse(&dir, why);
            Vec::new()
        }
    };
    if let Some(e) = st.autoload.take_error() {
        st.push_log(None, tr!(LogAutoloadError, error = e), now);
    }
    let mut found = Vec::new();
    for path in paths {
        let (bytes, ti) = match torrent_folders::read(&path) {
            Ok(r) => r,
            Err(e) => {
                st.push_log(None, tr!(FoldersNotLoaded, path = path.display(), error = e), now);
                continue;
            }
        };
        if st.by_hash.contains_key(&torrent::hash_of(&ti.info_hashes())) {
            let key = ti.info_hashes().key_hex();
            let line =
                torrent_folders::loaded(ti.name(), &key, &path, st.prefs.torrent_folders.autoload_delete);
            st.push_log(None, tr!(LogAlreadyInList, line), now);
            continue;
        }
        st.torrent_files.loading(&ti, path.clone());
        found.push((path, commands::Source::Torrent(bytes, Box::new(ti))));
    }
    found
}

/// Whether the Logger tab shows an alert: everything but the replies to `post_*` requests
/// (the Win32 Logger's rule), the port mappers' step-by-step chatter, scrape results (one
/// per stopped or queued torrent every half hour, `frontend::scrape`; the counts show in the
/// Seeds and Peers columns) and failed announces (on the Trackers tab; the engine logs a line
/// of its own when a private torrent has no tracker left).
fn logged(a: &Alert) -> bool {
    !matches!(
        a,
        Alert::StateUpdate(_)
            | Alert::SessionStats(_)
            | Alert::NetworkAdapters(_)
            | Alert::PeerInfo { .. }
            | Alert::FileList { .. }
            | Alert::FileProgress { .. }
            | Alert::TrackerList { .. }
            | Alert::PieceInfo { .. }
            | Alert::WebSeeds { .. }
            | Alert::TorrentFile { .. }
            | Alert::PortmapLog { .. }
            | Alert::ScrapeReply { .. }
            | Alert::ScrapeFailed { .. }
            | Alert::TrackerError { .. }
            | Alert::TrackerReply { .. }
            | Alert::DhtReply { .. }
    )
}

impl State {
    /// Appends an alert's line to the Logger ring: `name: message` for a torrent's alert.
    fn log_line(&mut self, a: &Alert, now: i64) {
        let torrent = a.torrent();
        let message = a.message();
        let text = match torrent.and_then(|id| self.torrents.get(&id)) {
            // some messages (added, failed to add) name the torrent themselves
            Some(e) if !e.status.name.is_empty() && !message.starts_with(&e.status.name) => {
                format!("{}: {message}", e.status.name)
            }
            _ => message,
        };
        self.push_log(torrent, text, now);
    }

    pub(crate) fn push_log(&mut self, torrent: Option<TorrentId>, text: String, time: i64) {
        if self.log.len() == LOG_LINES {
            self.log.pop_front();
        }
        self.log_seq += 1;
        self.log.push_back(LogLine { seq: self.log_seq, time, torrent, text });
    }

    /// Starts (or keeps) the speed history of a torrent a client is looking at.
    pub(crate) fn watch_speed(&mut self, id: TorrentId) -> &SpeedRing {
        let tick = self.tick;
        let ring = self.speed.entry(id).or_insert_with(|| SpeedRing {
            samples: VecDeque::with_capacity(SPEED_SAMPLES),
            pushed: 0,
            last_seen: tick,
        });
        ring.last_seen = tick;
        ring
    }

    /// Once per status tick: one sample per watched torrent, and the rings nobody asked for
    /// in [`SPEED_IDLE_TICKS`] (or whose torrent is gone) dropped.
    fn sample_speeds(&mut self) {
        let tick = self.tick;
        let torrents = &self.torrents;
        self.speed.retain(|id, ring| {
            let Some(e) = torrents.get(id) else { return false };
            if tick.saturating_sub(ring.last_seen) > SPEED_IDLE_TICKS {
                return false;
            }
            if ring.samples.len() == SPEED_SAMPLES {
                ring.samples.pop_front();
            }
            ring.samples.push_back((e.status.download_payload_rate, e.status.upload_payload_rate));
            ring.pushed += 1;
            true
        });
    }

    fn update(&mut self, s: TorrentStatus, now: i64) {
        let version = self.next_version;
        self.next_version += 1;
        let hash = torrent::hash_of(&s.info_hashes);
        let key = s.info_hashes.key_hex();
        let active = s.download_payload_rate > 0 || s.upload_payload_rate > 0;
        let had_metadata = self.torrents.get(&s.id).is_some_and(|e| e.status.has_metadata);
        if s.has_metadata && s.private.is_none() && !had_metadata && !self.meta.contains_key(&hash) {
            self.meta_wanted.push_back(s.id);
        }
        match self.torrents.get_mut(&s.id) {
            Some(e) => {
                if active {
                    e.last_activity = now;
                }
                if e.hash != hash {
                    self.by_hash.remove(&e.hash);
                    e.hash = hash.clone();
                }
                let old_key = e.status.info_hashes.key_hex();
                if old_key != key {
                    self.by_key.remove(&old_key);
                }
                e.status = s;
                e.version = version;
                self.by_hash.insert(hash, e.status.id);
                self.by_key.insert(key, e.status.id);
            }
            None => {
                let id = s.id;
                let last_activity = if active { now } else { 0 };
                self.by_hash.insert(hash.clone(), id);
                self.by_key.insert(key.clone(), id);
                self.torrents.insert(id, Entry { status: s, hash: hash.clone(), version, last_activity });
                if self.hosted {
                    // first sight: the host's label and pause mark for it
                    let label = self.options.get(&format!("torrent.{key}.label")).map(str::to_owned);
                    if let Some(label) = label.filter(|l| !l.is_empty()) {
                        self.label_cached(&hash, &label);
                    }
                    if self.options.get_bool(&format!("torrent.{key}.paused"), false) {
                        self.user_paused.insert(hash.clone());
                    }
                }
                // first sight, nox and hosted: the tags kept for it (D76)
                self.tags_first_sight(&hash, &key);
            }
        }
    }

    /// A torrent's first sight: the tags `options` keeps for it (`torrent.<key>.tags`) go into
    /// the lists, a name not known yet becomes known. Under nox an untidy value (a name the
    /// list refuses, or one that did not fit) and a new name are written back, as
    /// `tags::on_first_sight` does for the GUIs; hosted the host does that for its own keys.
    fn tags_first_sight(&mut self, hash: &str, key: &str) {
        let k = tags::torrent_key(key);
        let Some(text) = self.options.get(&k).map(str::to_owned) else { return };
        let before = self.tags_version;
        self.tags_cached(hash, &tags::parse_list(&text));
        if self.hosted {
            return;
        }
        if self.tags_version != before {
            self.store_known_tags();
        }
        let now = self.torrent_tags.get(hash).map(|v| tags::join(v)).unwrap_or_default();
        if now != text {
            self.store_torrent_tags(hash, key);
        }
    }

    /// Whether `name` is a known tag.
    pub(crate) fn is_known_tag(&self, name: &str) -> bool {
        self.tags.binary_search_by(|k| tags::cmp(k, name)).is_ok()
    }

    /// A torrent's tags (empty: none).
    pub(crate) fn tags_of(&self, hash: &str) -> &[String] {
        self.torrent_tags.get(hash).map_or(&[], Vec::as_slice)
    }

    /// Makes the new names of `names` (valid already) known while fewer than `tags::MAX_TAGS`
    /// are; true (and `tags_version` bumped) when one was added. Nothing is stored.
    fn learn_tags<'a>(&mut self, names: impl IntoIterator<Item = &'a String>) -> bool {
        let mut added = false;
        for t in names {
            if self.tags.len() >= tags::MAX_TAGS {
                break;
            }
            if let Err(at) = self.tags.binary_search_by(|k| tags::cmp(k, t)) {
                self.tags.insert(at, t.clone());
                added = true;
            }
        }
        if added {
            self.tags_version += 1;
        }
        added
    }

    /// Hosted: one of the host's preferences changes when it is a tag's (D76; `None` for any
    /// other key). A torrent's `torrent.<key>.tags` sets its tags in the lists (one not seen
    /// yet: its names become known, and it takes them at its first sight); `Some(true)` for
    /// `tags.list`, which the caller reads with [`State::reload_known_tags`] once every change
    /// of the batch is in. Nothing is stored (the mirror has them already).
    fn host_tags_change(&mut self, key: &str, value: Option<&str>) -> Option<bool> {
        if key == tags::KNOWN_KEY {
            return Some(true);
        }
        let k = tags::key_of(key)?;
        let names = tags::parse_list(value.unwrap_or(""));
        match self.by_key.get(k).and_then(|id| self.torrents.get(id)).map(|e| e.hash.clone()) {
            Some(hash) => {
                self.tags_cached(&hash, &names);
            }
            None => {
                self.learn_tags(&names);
            }
        }
        Some(false)
    }

    /// The known tags again from `options`' `tags.list` plus every tag a torrent has (the host
    /// changed them: a deleted tag leaves once no torrent has it).
    fn reload_known_tags(&mut self) {
        let mut known = tags::parse_list(self.options.get(tags::KNOWN_KEY).unwrap_or_default());
        known.extend(self.torrent_tags.values().flatten().cloned());
        tags::sort(&mut known);
        known.truncate(tags::MAX_TAGS);
        if known != self.tags {
            self.tags = known;
            self.tags_version += 1;
        }
    }

    /// Writes the known tags into `options` as `tags.list` (none removes the key).
    fn store_known_tags(&mut self) {
        if self.tags.is_empty() {
            self.options.remove(tags::KNOWN_KEY);
        } else {
            self.options.set(tags::KNOWN_KEY, tags::join(&self.tags));
        }
    }

    /// Writes a cached torrent's tags into `options` as `torrent.<key>.tags` (none removes it).
    fn store_torrent_tags(&mut self, hash: &str, key: &str) {
        let k = tags::torrent_key(key);
        match self.torrent_tags.get(hash) {
            Some(v) => {
                let text = tags::join(v);
                self.options.set(&k, text);
            }
            None => self.options.remove(&k),
        }
    }

    /// Sets a cached torrent's tags in the lists: each name normalized, duplicates dropped, a
    /// new name made known while there is room (one that does not fit is dropped), and bumps
    /// its version. Nothing is stored. True when its tags changed.
    fn tags_cached(&mut self, hash: &str, names: &[String]) -> bool {
        let mut v: Vec<String> = names.iter().filter_map(|t| tags::normalize(t).ok()).collect();
        tags::sort(&mut v);
        self.learn_tags(&v);
        v.retain(|t| self.is_known_tag(t));
        self.replace_cached(hash, &v)
    }

    /// A cached torrent's tags become `v`, which is as the cache keeps them (normalized,
    /// `tags::sort`ed, known); its version bumped when they changed. Nothing is stored.
    fn replace_cached(&mut self, hash: &str, v: &[String]) -> bool {
        if v == self.tags_of(hash) {
            return false;
        }
        if v.is_empty() {
            self.torrent_tags.remove(hash);
        } else {
            self.torrent_tags.insert(hash.to_owned(), v.to_vec());
        }
        self.touch(hash);
        true
    }

    /// [`State::set_tags`] for names that are as the cache keeps them already: normalized,
    /// `tags::sort`ed (so each once) and known. Nothing is normalized, sorted or learnt again,
    /// so one request's names cost each torrent no more than its own tags (N1).
    pub(crate) fn put_tags(&mut self, hash: &str, v: &[String]) -> bool {
        debug_assert!(
            v.windows(2).all(|w| tags::cmp(&w[0], &w[1]).is_lt()) && v.iter().all(|t| self.is_known_tag(t))
        );
        let changed = self.replace_cached(hash, v);
        if changed {
            if let Some(key) = self.key_of(hash) {
                self.store_torrent_tags(hash, &key);
            }
        }
        changed
    }

    /// Sets a cached torrent's tags (none clears them, [`State::tags_cached`]'s rules) and
    /// writes them into `options` as `torrent.<key>.tags`, with `tags.list` when a name became
    /// known: `webui.json` under nox, the host's keys hosted (the next [`Shared::save`]).
    /// True when they changed.
    pub(crate) fn set_tags(&mut self, hash: &str, names: &[String]) -> bool {
        let before = self.tags_version;
        let changed = self.tags_cached(hash, names);
        if self.tags_version != before {
            self.store_known_tags();
        }
        if changed {
            if let Some(key) = self.key_of(hash) {
                self.store_torrent_tags(hash, &key);
            }
        }
        changed
    }

    /// New Tag (the shim's `createTags`): makes `name` known, with no torrent; its normalized
    /// name. An existing tag is no error. `Err` for an invalid name, or a new one with
    /// `tags::MAX_TAGS` known.
    pub(crate) fn create_tag(&mut self, name: &str) -> Result<String, TagError> {
        let tag = tags::normalize(name)?;
        if self.is_known_tag(&tag) {
            return Ok(tag);
        }
        if !self.learn_tags([&tag]) {
            return Err(TagError::TooMany);
        }
        self.store_known_tags();
        Ok(tag)
    }

    /// Delete Tag for each of `names` (as the tags keep them): no longer known, and every
    /// torrent loses them, those not seen yet too (their `torrent.<key>.tags`); in one pass over
    /// the torrents and the stored keys however many names there are (N1). How many were known.
    pub(crate) fn delete_tags(&mut self, names: &[String]) -> usize {
        let mut gone: Vec<String> = names.iter().filter(|n| self.is_known_tag(n)).cloned().collect();
        if gone.is_empty() {
            return 0;
        }
        tags::sort(&mut gone);
        let is_gone = |t: &str| gone.binary_search_by(|g| tags::cmp(g, t)).is_ok();
        self.tags.retain(|t| !is_gone(t));
        self.tags_version += 1;
        let lost: Vec<(String, Vec<String>)> = self
            .torrent_tags
            .iter()
            .filter(|(_, v)| v.iter().any(|t| is_gone(t)))
            .map(|(h, v)| (h.clone(), v.iter().filter(|t| !is_gone(t)).cloned().collect()))
            .collect();
        for (h, v) in lost {
            self.put_tags(&h, &v);
        }
        // a torrent added with them that the engine has not reported yet, or one that is gone
        tags::forget_all_everywhere(&mut self.options, is_gone);
        self.store_known_tags();
        gone.len()
    }

    /// Gives a torrent the tags `names` beside those it has (each made known when new; an
    /// invalid one, or a new one that does not fit, is skipped): one in the cache by `hash`
    /// (or by `key`), else, for one being added, `torrent.<key>.tags` in `options`, taken at
    /// its first sight. The first name refused, if any.
    pub(crate) fn add_tags_keyed(
        &mut self,
        hash: &str,
        key: Option<&str>,
        names: &[String],
    ) -> Result<(), TagError> {
        let mut first_err = None;
        let mut add = Vec::new();
        for n in names {
            match self.create_tag(n) {
                Ok(t) => add.push(t),
                Err(e) => {
                    first_err.get_or_insert(e);
                }
            }
        }
        let cached = match self.by_hash.get(hash) {
            Some(id) => self.torrents.get(id).map(|e| e.hash.clone()),
            None => key
                .and_then(|k| self.by_key.get(k))
                .and_then(|id| self.torrents.get(id))
                .map(|e| e.hash.clone()),
        };
        if let Some(h) = cached {
            let mut v = self.tags_of(&h).to_vec();
            v.extend(add);
            self.set_tags(&h, &v);
        } else if let (Some(key), false) = (key, add.is_empty()) {
            let k = tags::torrent_key(key);
            let mut v = tags::parse_list(self.options.get(&k).unwrap_or_default());
            v.extend(add);
            tags::sort(&mut v);
            self.options.set(&k, tags::join(&v));
        }
        first_err.map_or(Ok(()), Err)
    }

    /// The host's key (`InfoHashes::key_hex`) of a cached torrent.
    pub(crate) fn key_of(&self, hash: &str) -> Option<String> {
        self.by_hash.get(hash).and_then(|id| self.torrents.get(id)).map(|e| e.status.info_hashes.key_hex())
    }

    /// Sets a cached torrent's label in the lists (empty removes it; a new label joins the
    /// label list) and bumps its version. Nothing is stored.
    fn label_cached(&mut self, hash: &str, label: &str) {
        if label.is_empty() {
            self.torrent_categories.remove(hash);
        } else {
            if !self.categories.contains_key(label) {
                self.categories.insert(label.to_owned(), String::new());
                self.categories_version += 1;
            }
            self.torrent_categories.insert(hash.to_owned(), label.to_owned());
        }
        self.touch(hash);
    }

    /// Records (or clears) the user's Pause of a cached torrent and bumps its version. Nothing
    /// is stored. Returns whether it changed.
    fn paused_cached(&mut self, hash: &str, paused: bool) -> bool {
        let changed =
            if paused { self.user_paused.insert(hash.to_owned()) } else { self.user_paused.remove(hash) };
        if changed {
            self.touch(hash);
        }
        changed
    }

    /// Sets a torrent's label (empty removes it) and, hosted, writes it into the mirror as the
    /// host's `torrent.<key>.label` for the next [`Shared::save`]. For a torrent in the cache;
    /// [`State::set_label_keyed`] for one being added.
    pub(crate) fn set_label(&mut self, hash: &str, label: &str) {
        let key = self.key_of(hash);
        self.set_label_keyed(hash, key.as_deref(), label);
    }

    /// [`State::set_label`] with the host's key given (a torrent not in the cache yet).
    pub(crate) fn set_label_keyed(&mut self, hash: &str, key: Option<&str>, label: &str) {
        self.label_cached(hash, label);
        if let (true, Some(key)) = (self.hosted, key) {
            let k = format!("torrent.{key}.label");
            if label.is_empty() {
                self.options.remove(&k);
            } else {
                self.options.set(&k, label);
            }
        }
    }

    /// Records (or clears) the user's Pause of a torrent and, hosted, the host's
    /// `torrent.<key>.paused` mark in the mirror. Returns whether it changed.
    pub(crate) fn set_user_paused(&mut self, hash: &str, paused: bool) -> bool {
        let changed = self.paused_cached(hash, paused);
        if self.hosted {
            if let Some(key) = self.key_of(hash) {
                let k = format!("torrent.{key}.paused");
                if paused {
                    self.options.set(&k, "1");
                } else {
                    self.options.remove(&k);
                }
            }
        }
        changed
    }

    /// The cached answer to a `post_*` request, if the engine answered one.
    pub(crate) fn cached(&self, id: TorrentId, kind: ReplyKind) -> Option<&Reply> {
        self.replies.get(&(id, kind)).map(|(_, r)| r)
    }

    /// An export starts waiting for torrent `id`'s `.torrent` bytes ([`State::torrent_file_answer`]
    /// caches them for it).
    pub(crate) fn want_torrent_file(&mut self, id: TorrentId) {
        *self.exports.entry(id).or_default() += 1;
    }

    /// An export of torrent `id` is done (answered, timed out or failed): once no other waits,
    /// the bytes leave the cache.
    pub(crate) fn torrent_file_done(&mut self, id: TorrentId) {
        let left = match self.exports.get_mut(&id) {
            Some(n) => {
                *n = n.saturating_sub(1);
                *n
            }
            None => 0,
        };
        if left == 0 {
            self.exports.remove(&id);
            self.replies.remove(&(id, ReplyKind::TorrentFile));
        }
    }

    /// `Alert::TorrentFile`: cached only while an export waits for it (N4). True when cached.
    fn torrent_file_answer(&mut self, id: TorrentId, data: &Option<Arc<[u8]>>) -> bool {
        if !self.exports.contains_key(&id) {
            return false;
        }
        self.reply(id, ReplyKind::TorrentFile, Reply::TorrentFile(data.clone()));
        true
    }

    pub(crate) fn reply(&mut self, id: TorrentId, kind: ReplyKind, r: Reply) {
        self.serial += 1;
        self.replies.insert((id, kind), (self.serial, r));
        if self.replies.len() > 256 {
            let keep_after = self.serial.saturating_sub(64);
            self.replies.retain(|_, (s, _)| *s > keep_after);
        }
    }

    /// Bumps a torrent's version after a change the engine does not report (its category).
    pub fn touch(&mut self, hash: &str) {
        if let Some(e) = self.by_hash.get(hash).and_then(|id| self.torrents.get_mut(id)) {
            e.version = self.next_version;
            self.next_version += 1;
        }
    }

    pub fn is_loopback_bypass(&self, ip: IpAddr) -> bool {
        self.prefs.bypass_local_auth && ip.is_loopback()
    }

    /// Whether `ip` is in the subnet whitelist (`Prefs::auth_subnets`).
    pub fn is_subnet_bypass(&self, ip: IpAddr) -> bool {
        !self.prefs.auth_subnets.trim().is_empty() && auth::subnets_contain(&self.prefs.auth_subnets, ip)
    }

    /// Sets the API key; a different one forgets the verified key and ends the sessions the
    /// old one opened.
    pub fn set_api_key(&mut self, key: Option<PasswordHash>) {
        if key != self.api_key {
            self.api_key = key;
            self.api_key_verified = None;
            self.sessions.remove(api::APIKEY_SESSION);
        }
    }

    /// A Web UI without a server for unit tests: `handle`'s engine, an empty cache, and
    /// `settings_dir` for the default `ipfilter.dat`.
    #[cfg(test)]
    pub(crate) fn shared_for_tests(handle: SessionHandle, settings_dir: PathBuf) -> Shared {
        Shared {
            handle,
            state: Mutex::new(State::for_tests()),
            replied: Condvar::new(),
            limits: http::Limits::default(),
            iterations: 1,
            state_file: None,
            settings_dir,
            rss: Arc::new(Mutex::new(rss::Rss::load(None))),
            search: Mutex::new(search::new_search()),
            shim_searches: Mutex::default(),
            tls: None,
            path_policy: path_policy::PathPolicy::default(),
            remotes: Mutex::new(Vec::new()),
            host: None,
            helper_dir: None,
            helper: Mutex::default(),
            this: Weak::new(),
            trackers: Some(Mutex::new(TrackerList::new(None, tracker_list::Config::default()))),
            ip_list: Some(Mutex::new(IpFilterList::new(None, ip_filter_list::Config::default()))),
            run_program: Some(Mutex::new(RunPrograms::new())),
            remote_run_program: false,
        }
    }

    /// An empty cache for unit tests of the protocol code (no server, no engine).
    #[cfg(test)]
    pub(crate) fn for_tests() -> State {
        State {
            settings_dir: PathBuf::from("."),
            torrents: HashMap::new(),
            by_hash: HashMap::new(),
            by_key: HashMap::new(),
            hosted: false,
            next_version: 1,
            stats: SessionStats::default(),
            settings: Settings::default(),
            prefs: Prefs { alt_dl_limit: 0, alt_up_limit: 0, ..Prefs::defaults("", &Settings::default()) },
            username: "admin".into(),
            password: PasswordHash::new("x", 1),
            api_key: None,
            api_key_verified: None,
            categories: BTreeMap::new(),
            categories_version: 1,
            torrent_categories: HashMap::new(),
            tags: Vec::new(),
            tags_version: 1,
            torrent_tags: HashMap::new(),
            meta: HashMap::new(),
            meta_wanted: VecDeque::new(),
            options: rookparcel_frontend::prefs::Prefs::default(),
            default_save_path: String::new(),
            listen_addr: SocketAddr::from(([127, 0, 0, 1], 8080)),
            gui_version: 1,
            scheduler: Scheduler::new(),
            scrapes: Scrapes::default(),
            torrent_files: TorrentFiles::new(),
            autoload: Poller::new(),
            replies: HashMap::new(),
            serial: 0,
            exports: HashMap::new(),
            sessions: HashMap::new(),
            bans: BanList::default(),
            shutdown: false,
            tick: 0,
            removals: 0,
            log: VecDeque::new(),
            log_seq: 0,
            speed: HashMap::new(),
            user_paused: HashSet::new(),
            pending_top: HashSet::new(),
            rss_version: 1,
            search_version: 1,
            awaiting: Default::default(),
            adds_version: 1,
            excluded: Pending::new(),
            update: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rookparcel_core::InfoHashes;
    use rookparcel_core::Sha1Hash;

    fn status(id: u64, v1: u8) -> TorrentStatus {
        TorrentStatus {
            id: TorrentId::from_raw(id),
            info_hashes: InfoHashes::v1(Sha1Hash([v1; 20])),
            name: format!("t{id}"),
            ..TorrentStatus::default()
        }
    }

    /// N4: a `.torrent` the engine reports is cached only while an export waits for it (the
    /// `.torrent` folders and the waiting magnets cause the same alert), and leaves the cache
    /// when the last waiting export is done, answered or timed out.
    #[test]
    fn torrent_files_are_cached_only_for_a_waiting_export() {
        let mut st = State::for_tests();
        let (a, b) = (TorrentId::from_raw(1), TorrentId::from_raw(2));
        let bytes: Option<Arc<[u8]>> = Some(Arc::from(&b"d4:infod4:name1:xee"[..]));
        // nobody asked: not kept
        assert!(!st.torrent_file_answer(a, &bytes));
        assert!(st.cached(a, ReplyKind::TorrentFile).is_none());
        // two exports of `a`, one of `b`
        st.want_torrent_file(a);
        st.want_torrent_file(a);
        st.want_torrent_file(b);
        assert!(st.torrent_file_answer(a, &bytes));
        assert!(st.cached(a, ReplyKind::TorrentFile).is_some());
        st.torrent_file_done(a);
        assert!(st.cached(a, ReplyKind::TorrentFile).is_some(), "the other export of it still waits");
        st.torrent_file_done(a);
        assert!(st.cached(a, ReplyKind::TorrentFile).is_none());
        assert!(!st.torrent_file_answer(a, &bytes), "a late answer is not kept");
        // `b` timed out: its answer, had it come, goes with it; one after that is not kept
        st.torrent_file_done(b);
        assert!(!st.torrent_file_answer(b, &bytes));
        assert!(st.replies.is_empty() && st.exports.is_empty());
        // a stray `done` changes nothing
        st.torrent_file_done(b);
        assert!(st.exports.is_empty());
    }

    /// A torrent seen with metadata whose status does not say whether it is private (a record
    /// older than 1.2.30) is queued once for the flag; one whose metadata the shim already has,
    /// or whose status says it, is not.
    #[test]
    fn torrents_with_new_metadata_are_queued_for_their_private_flag() {
        let mut st = State::for_tests();
        let (mut a, mut b) = (status(1, 1), status(2, 2));
        st.update(a.clone(), 0);
        assert!(st.meta_wanted.is_empty(), "no metadata yet");
        a.has_metadata = true;
        st.update(a.clone(), 0);
        st.update(a, 0);
        assert_eq!(st.meta_wanted, [TorrentId::from_raw(1)]);
        b.has_metadata = true;
        st.meta.insert(torrent::hash_of(&b.info_hashes), Meta::default());
        st.update(b, 0);
        assert_eq!(st.meta_wanted.len(), 1, "the shim has its metadata");
        let mut c = status(3, 3);
        c.has_metadata = true;
        c.private = Some(true);
        st.update(c, 0);
        assert_eq!(st.meta_wanted.len(), 1, "the engine said it");
    }

    /// Hosted: a torrent's first sight takes the host's label and pause mark, `by_key` finds
    /// it by the host's key, and the Web UI's own changes land in the mirror as the host's keys
    /// (only real changes, as the journal records them).
    #[test]
    fn hosted_state_mirrors_labels_and_pause_marks() {
        let mut st = State::for_tests();
        st.hosted = true;
        st.options.track_changes();
        let key = Sha1Hash([7; 20]).to_hex();
        st.options.apply_changes(&[
            (format!("torrent.{key}.label"), Some("tv".to_owned())),
            (format!("torrent.{key}.paused"), Some("1".to_owned())),
        ]);
        st.update(status(1, 7), 0);
        st.update(status(2, 8), 0);
        let hash = torrent::hash_of(&InfoHashes::v1(Sha1Hash([7; 20])));
        assert_eq!(hash, key, "a v1 torrent's key is its hash");
        assert_eq!(st.by_key.get(&key), Some(&TorrentId::from_raw(1)));
        assert_eq!(st.torrent_categories.get(&hash).map(String::as_str), Some("tv"));
        assert!(st.categories.contains_key("tv") && st.user_paused.contains(&hash));
        assert!(st.options.take_changes().is_empty(), "the host's own keys are not journaled");

        // the Web UI's changes: the host's keys, removed keys as None
        let other = torrent::hash_of(&InfoHashes::v1(Sha1Hash([8; 20])));
        st.set_label(&other, "film");
        st.set_label(&hash, "");
        assert!(st.set_user_paused(&hash, false));
        assert!(!st.set_user_paused(&hash, false), "unchanged");
        assert!(st.set_user_paused(&other, true));
        let other_key = Sha1Hash([8; 20]).to_hex();
        assert_eq!(
            st.options.take_changes(),
            [
                (format!("torrent.{other_key}.label"), Some("film".to_owned())),
                (format!("torrent.{key}.label"), None),
                (format!("torrent.{key}.paused"), None),
                (format!("torrent.{other_key}.paused"), Some("1".to_owned())),
            ]
        );
        assert_eq!(st.torrent_categories.get(&other).map(String::as_str), Some("film"));
        assert!(!st.torrent_categories.contains_key(&hash) && !st.user_paused.contains(&hash));

        // nox: the same calls keep the lists and write no keys
        let mut st = State::for_tests();
        st.options.track_changes();
        st.update(status(1, 7), 0);
        st.set_label(&hash, "tv");
        st.set_user_paused(&hash, true);
        assert_eq!(st.torrent_categories.get(&hash).map(String::as_str), Some("tv"));
        assert!(st.options.take_changes().is_empty());
    }

    fn names(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_owned()).collect()
    }

    /// D76: a torrent's first sight takes `torrent.<key>.tags` (its new names made known; under
    /// nox an untidy value written back), the Web UI's changes land in `options` as
    /// `torrent.<key>.tags` and `tags.list` (the host's keys hosted), deleting a tag takes it
    /// off every torrent, those not seen yet too, and the known list stays within the limit.
    #[test]
    fn tags_are_kept_as_the_preferences_keys() {
        let key = Sha1Hash([7; 20]).to_hex();
        let other = Sha1Hash([8; 20]).to_hex();
        for hosted in [false, true] {
            let mut st = State::for_tests();
            st.hosted = hosted;
            st.options.set(&tags::torrent_key(&key), " tv,,Movies,a\u{1}b ");
            st.options.set(tags::KNOWN_KEY, "old");
            // a torrent removed while the program was closed, with a tag deleted since
            st.options.set(&tags::torrent_key(&Sha1Hash([6; 20]).to_hex()), "gone");
            st.options.track_changes();
            // what a start reads: `tags.list` only
            st.tags = tags::load_known(&st.options);
            assert_eq!(st.tags, names(&["old"]), "a gone torrent's key brings nothing back");
            st.update(status(1, 7), 0);
            st.update(status(2, 8), 0);
            assert_eq!(st.tags_of(&key), names(&["Movies", "tv"]));
            assert!(st.tags_of(&other).is_empty());
            assert_eq!(st.tags, names(&["Movies", "old", "tv"]), "the first sight makes them known");
            let tidy = st.options.take_changes();
            if hosted {
                assert!(tidy.is_empty(), "the host tidies its own keys: {tidy:?}");
            } else {
                assert_eq!(
                    tidy,
                    [
                        (tags::KNOWN_KEY.to_owned(), Some("Movies,old,tv".to_owned())),
                        (tags::torrent_key(&key), Some("Movies,tv".to_owned())),
                    ]
                );
            }

            // the Web UI's changes: the keys, a new name in `tags.list` too
            let v0 = st.tags_version;
            assert!(st.set_tags(&other, &names(&["new", " tv "])));
            assert!(!st.set_tags(&other, &names(&["tv", "new"])), "the same tags");
            assert!(st.tags_version > v0);
            assert_eq!(st.create_tag(" x "), Ok("x".to_owned()));
            assert_eq!(st.create_tag("x"), Ok("x".to_owned()), "an existing one is fine");
            for bad in ["", "  ", "a,b", "a\nb", "\t"] {
                assert_eq!(st.create_tag(bad), Err(TagError::Invalid), "{bad:?}");
            }
            assert_eq!(
                st.options.take_changes(),
                [
                    (tags::KNOWN_KEY.to_owned(), Some("Movies,new,old,tv".to_owned())),
                    (tags::torrent_key(&other), Some("new,tv".to_owned())),
                    (tags::KNOWN_KEY.to_owned(), Some("Movies,new,old,tv,x".to_owned())),
                ]
            );
            // one being added: its key, read at its first sight
            let third = Sha1Hash([9; 20]).to_hex();
            assert_eq!(
                st.add_tags_keyed(&third, Some(&third), &names(&["tv", "a,b"])),
                Err(TagError::Invalid)
            );
            assert_eq!(st.options.get(&tags::torrent_key(&third)), Some("tv"));

            // Delete Tag: off every torrent, the unseen one's key too
            assert_eq!(st.delete_tags(&names(&["tv"])), 1);
            assert_eq!(st.delete_tags(&names(&["tv"])), 0);
            assert_eq!(st.tags_of(&key), names(&["Movies"]));
            assert_eq!(st.tags_of(&other), names(&["new"]));
            assert_eq!(st.options.get(&tags::torrent_key(&third)), None);
            assert_eq!(st.options.get(tags::KNOWN_KEY), Some("Movies,new,old,x"));
            st.update(status(3, 9), 0);
            assert!(st.tags_of(&third).is_empty());
            assert_eq!(st.options.get(&tags::torrent_key(&Sha1Hash([6; 20]).to_hex())), Some("gone"));
            st.options.take_changes();

            // tagging with known names, taking tags off and an existing New Tag leave the known
            // list alone: `tags.list` is not written (hosted, the mirror journals only what
            // changed, so a tag the host made meanwhile is not overwritten by these)
            assert!(st.set_tags(&key, &names(&["Movies", "new", "x"])));
            assert!(st.set_tags(&other, &[]));
            assert_eq!(st.create_tag("old"), Ok("old".to_owned()));
            assert!(st.add_tags_keyed(&key, None, &names(&["old"])).is_ok());
            let changes = st.options.take_changes();
            assert!(!changes.is_empty());
            assert!(changes.iter().all(|(k, _)| k != tags::KNOWN_KEY), "{changes:?}");

            // at most MAX_TAGS known; a known one still goes on
            for i in 0..tags::MAX_TAGS {
                let _ = st.create_tag(&format!("t{i}"));
            }
            assert_eq!(st.tags.len(), tags::MAX_TAGS);
            assert_eq!(st.create_tag("one more"), Err(TagError::TooMany));
            assert!(!st.set_tags(&third, &names(&["one more"])), "a new name past the limit is dropped");
            assert!(st.set_tags(&third, &names(&["t5"])));
        }
    }

    /// Hosted, the host's changes: a torrent's key sets its tags, `tags.list` replaces the known
    /// ones (a tag a torrent still has stays), a deleted tag leaves.
    #[test]
    fn the_hosts_tag_changes_reach_the_lists() {
        let mut st = State::for_tests();
        st.hosted = true;
        st.options.track_changes();
        st.update(status(1, 7), 0);
        let key = Sha1Hash([7; 20]).to_hex();
        // what `WebUi::options_changed` does with them, without a server
        let apply = |st: &mut State, changes: &[(String, Option<String>)]| {
            st.options.apply_changes(changes);
            let mut list = false;
            for (k, v) in changes {
                list |= st.host_tags_change(k, v.as_deref()).unwrap_or(false);
            }
            if list {
                st.reload_known_tags();
            }
        };
        apply(
            &mut st,
            &[
                (tags::torrent_key(&key), Some("b,a".to_owned())),
                (tags::KNOWN_KEY.to_owned(), Some("a,b,c".to_owned())),
            ],
        );
        assert_eq!(st.tags_of(&key), names(&["a", "b"]));
        assert_eq!(st.tags, names(&["a", "b", "c"]));
        // the host deletes "a": the list first, then the torrent's key (either order works)
        apply(
            &mut st,
            &[
                (tags::KNOWN_KEY.to_owned(), Some("b,c".to_owned())),
                (tags::torrent_key(&key), Some("b".to_owned())),
            ],
        );
        assert_eq!(st.tags_of(&key), names(&["b"]));
        assert_eq!(st.tags, names(&["b", "c"]));
        // a torrent not seen yet: its names become known, it takes them at its first sight
        let later = Sha1Hash([9; 20]).to_hex();
        apply(&mut st, &[(tags::torrent_key(&later), Some("d".to_owned()))]);
        assert_eq!(st.tags, names(&["b", "c", "d"]));
        st.update(status(2, 9), 0);
        assert_eq!(st.tags_of(&later), names(&["d"]));
        assert_eq!(st.host_tags_change("torrent.x.label", Some("y")), None);
        assert!(st.options.take_changes().is_empty(), "nothing goes back");
    }
}
