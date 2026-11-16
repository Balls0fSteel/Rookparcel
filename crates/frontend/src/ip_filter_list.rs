//! The IP filter's block list from an `https://` address (D75), under Preferences > BitTorrent:
//! with "Get the block list from this address" ticked, the list fetched from the address
//! replaces the file (`ipfilter.path`); "Enable IP filter" stays the switch for both.
//!
//! Shaped like [`crate::tracker_list`] (D66): an owner (a GUI's `App`, `Core`, nox) keeps one
//! [`IpFilterList`], [`IpFilterList::configure`]s it from the preferences, calls
//! [`IpFilterList::tick`] about once a second and hands the engine what
//! [`IpFilterList::take_filter`] gives (`SessionHandle::set_ip_filter`). The fetch and the
//! parsing of a list of many megabytes run on a worker thread.
//!
//! The answer is kept as it came ([`CACHE_FILE`], gzip or not: [`IpFilter::load`] reads both),
//! beside a small [`INFO_FILE`] naming its address, when it was fetched and its rule count. At
//! start the owner loads the kept copy first ([`list_file`]), so the filter is in force before
//! any fetch; a fetch is due when the copy is a day old or belongs to another address. What is
//! not a block list (an error page, nothing that parses as a rule) is refused, and the rules in
//! force and the kept copy stay.
//!
//! While the D70 settings guard holds the transfers (`rookparcel_core::fetch::held`), a due
//! fetch waits; it starts at the first tick after the hold is lifted, and a fetch that failed
//! because the hold came meanwhile is tried again then, not an hour later.

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use rookparcel_core::fetch::{FetchError, FetchOptions, FetchProxy, http_get};
use rookparcel_core::util::gzip::{inflate_gzip, is_gzip};
use rookparcel_core::util::private_file;
use rookparcel_core::{IpFilter, IpFilterLoad};

use crate::options::GuiOptions;
use crate::tracker_list::is_list_url;
use crate::{fmt, tr};

/// The largest answer fetched (block lists are several MB, often gzipped).
pub const MAX_BYTES: usize = 32 << 20;
/// The largest list a gzipped answer may inflate to.
pub const MAX_INFLATED: usize = 256 << 20;
/// How long one fetch may take, redirects included.
pub const TIMEOUT: Duration = Duration::from_secs(120);
/// How often a list that was fetched is fetched again.
pub const REFRESH: Duration = Duration::from_secs(24 * 3600);
/// How long after a failed fetch the next one is tried.
pub const RETRY: Duration = Duration::from_secs(3600);
/// The fetched list, as it came, in the settings folder.
pub const CACHE_FILE: &str = "ipfilter-url.dat";
/// Beside it: its address, fetch time and rule count.
pub const INFO_FILE: &str = "ipfilter-url.txt";

/// Why a fetched list was not used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reject {
    /// Not `https://`, or not an address at all.
    NotHttps,
    /// Not a block list in a known format (or a broken gzip file).
    NotAList,
    /// The fetch failed.
    Fetch(String),
}

impl Reject {
    pub fn text(&self) -> String {
        match self {
            Reject::NotHttps => tr!(TrackersErrHttps).to_owned(),
            Reject::NotAList => tr!(IpFilterErrNotList).to_owned(),
            Reject::Fetch(e) => e.clone(),
        }
    }
}

/// Reads a fetched answer as a block list ([`IpFilter::parse`], inflated first when it is
/// gzip): refused when no line is a rule or more lines are something else than rules.
pub fn parse_list(body: &[u8]) -> Result<IpFilterLoad, Reject> {
    let inflated;
    let text = if is_gzip(body) {
        inflated = inflate_gzip(body, MAX_INFLATED).map_err(|_| Reject::NotAList)?;
        &inflated[..]
    } else {
        body
    };
    let l = IpFilter::parse(text);
    if l.rules == 0 || l.invalid > l.rules {
        return Err(Reject::NotAList);
    }
    Ok(l)
}

/// Fetches `url` (blocking: call it on a worker thread). The body comes as the server sent it:
/// `identity` is asked for, so a `.gz` file stays one, and [`parse_list`] inflates it.
pub fn fetch(url: &str, proxy: Option<FetchProxy>) -> Result<Vec<u8>, Reject> {
    if !is_list_url(url) {
        return Err(Reject::NotHttps);
    }
    let opts = FetchOptions {
        proxy,
        max_size: MAX_BYTES,
        timeout: TIMEOUT,
        no_downgrade: true,
        ..FetchOptions::default()
    };
    let resp = http_get(url, &opts).map_err(|e| {
        Reject::Fetch(match e {
            // a redirect's target the fetcher could not read: named without what it may carry
            FetchError::InvalidUrl(u) => FetchError::InvalidUrl(redact(&u)).to_string(),
            e => e.to_string(),
        })
    })?;
    Ok(resp.body)
}

/// The address as the Logger and error texts name it: scheme, host, port and
/// path, without the user name and password before the host and without the query and fragment
/// (`https://lists.example/level1.gz` for `https://u:p@lists.example/level1.gz?pin=1`), since a
/// subscription's address often carries its credentials. Only the Preferences field shows it
/// whole.
pub fn redact(url: &str) -> String {
    let url = url.trim();
    let (scheme, rest) = match url.find("://") {
        Some(i) => (&url[..i + 3], &url[i + 3..]),
        None => ("", url),
    };
    let rest = &rest[..rest.find(['?', '#']).unwrap_or(rest.len())];
    let (authority, path) = rest.split_at(rest.find('/').unwrap_or(rest.len()));
    let host = authority.rsplit_once('@').map_or(authority, |(_, h)| h);
    format!("{scheme}{host}{path}")
}

/// The three preferences ([`GuiOptions`]' `ip_filter`, `ip_filter_from_url`, `ip_filter_url`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Config {
    pub enabled: bool,
    pub from_url: bool,
    pub url: String,
}

impl Config {
    pub fn of(g: &GuiOptions) -> Config {
        Config {
            enabled: g.ip_filter,
            from_url: g.ip_filter_from_url,
            url: g.ip_filter_url.trim().to_owned(),
        }
    }
}

/// What [`INFO_FILE`] says of the kept list.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Info {
    url: String,
    /// Unix seconds.
    at: i64,
    rules: usize,
}

fn read_info(dir: &Path) -> Option<Info> {
    let text = std::fs::read_to_string(dir.join(INFO_FILE)).ok()?;
    let value = |key: &str| text.lines().find_map(|l| l.strip_prefix(key)?.strip_prefix('=')).map(str::trim);
    let info = Info {
        url: value("url")?.to_owned(),
        at: value("fetched")?.parse().ok()?,
        rules: value("rules")?.parse().ok()?,
    };
    (is_list_url(&info.url) && info.at > 0 && dir.join(CACHE_FILE).is_file()).then_some(info)
}

/// The block list file the IP filter loads for `gui` (the box itself aside): the path field's
/// file (`ipfilter.dat` in `settings_dir` when empty), or, with the list from an address, the
/// copy fetched from that address; `None` while there is none, so the fetch brings the rules.
pub fn list_file(gui: &GuiOptions, settings_dir: &Path) -> Option<PathBuf> {
    if !gui.ip_filter_from_url {
        return Some(gui.ip_filter_file(settings_dir));
    }
    let info = read_info(settings_dir)?;
    (info.url == gui.ip_filter_url.trim()).then(|| settings_dir.join(CACHE_FILE))
}

/// The page's IP filter line ([`crate::pref_pages::Item::IpFilterStatus`]), for every GUI:
/// `reload` is Reload's result when it was just pressed (the rule count or the message);
/// otherwise `url_status` ([`IpFilterList::status_for`] of the page's box and address) while the
/// page takes the list from an address, else the file an empty path field means.
pub fn page_status(
    reload: Option<&Result<usize, String>>,
    url_status: Option<String>,
    path_field: &str,
    settings_dir: &Path,
) -> String {
    match (reload, url_status) {
        (Some(Ok(n)), _) => tr!(PrefIpFilterLoaded, count = n),
        (Some(Err(e)), _) => e.clone(),
        (None, Some(s)) => s,
        (None, None) if path_field.trim().is_empty() => {
            tr!(PrefIpFilterDefault, path = settings_dir.join("ipfilter.dat").display())
        }
        (None, None) => String::new(),
    }
}

/// A finished fetch: the address, the parsed list (written to the cache's `.part` file when
/// there is a cache), and whether the transfers were held when it failed.
type Fetched = (String, Result<IpFilterLoad, Reject>, bool);

/// The list from the web and its fetch schedule, for one program.
pub struct IpFilterList {
    /// The settings folder, where the list is kept; `None` keeps nothing between runs.
    dir: Option<PathBuf>,
    config: Config,
    /// The address of the last good list, when it was fetched (Unix seconds, 0 = never) and
    /// its rule count.
    fetched_url: String,
    fetched_at: i64,
    rules: usize,
    error: Option<Reject>,
    next: Option<Instant>,
    running: Option<mpsc::Receiver<Fetched>>,
    /// A new list for the engine.
    ready: Option<IpFilter>,
    fetcher: fn(&str, Option<FetchProxy>) -> Result<Vec<u8>, Reject>,
    held: fn() -> bool,
}

impl IpFilterList {
    /// `settings_dir` keeps [`CACHE_FILE`] and [`INFO_FILE`]; `None` keeps nothing between runs.
    pub fn new(settings_dir: Option<PathBuf>, config: Config) -> IpFilterList {
        let mut l = IpFilterList {
            dir: settings_dir,
            config: Config::default(),
            fetched_url: String::new(),
            fetched_at: 0,
            rules: 0,
            error: None,
            next: None,
            running: None,
            ready: None,
            fetcher: fetch,
            held: rookparcel_core::fetch::held,
        };
        if let Some(info) = l.dir.as_deref().and_then(read_info) {
            l.fetched_url = info.url;
            l.fetched_at = info.at;
            l.rules = info.rules;
        }
        l.configure(config);
        l
    }

    #[cfg(test)]
    fn with_fetcher(mut self, f: fn(&str, Option<FetchProxy>) -> Result<Vec<u8>, Reject>) -> IpFilterList {
        self.fetcher = f;
        self
    }

    fn cache(&self) -> Option<PathBuf> {
        self.dir.as_ref().map(|d| d.join(CACHE_FILE))
    }

    fn part(&self) -> Option<PathBuf> {
        self.dir.as_ref().map(|d| d.join(format!("{CACHE_FILE}.part")))
    }

    fn forget_cache(&self) {
        if let Some(dir) = &self.dir {
            let _ = std::fs::remove_file(dir.join(CACHE_FILE));
            let _ = std::fs::remove_file(dir.join(INFO_FILE));
        }
    }

    /// New preferences. A list fetched from another address than the new one is forgotten at
    /// once, its copy too (the rules in force stay until the owner loads others), and the new
    /// address is fetched at the next tick.
    pub fn configure(&mut self, config: Config) {
        if config == self.config && self.next.is_some() == self.wants_fetch() {
            return;
        }
        let changed = config != self.config;
        self.config = config;
        if self.fetched_url != self.config.url {
            if !self.fetched_url.is_empty() {
                self.fetched_url.clear();
                self.fetched_at = 0;
                self.rules = 0;
                self.forget_cache();
            }
            self.error = None;
        }
        if !self.wants_fetch() {
            self.next = None;
            let c = &self.config;
            self.error = (c.enabled && c.from_url && !c.url.is_empty()).then_some(Reject::NotHttps);
        } else if changed || self.next.is_none() {
            // a kept list that is still fresh waits for its day
            let age = rookparcel_core::util::time::unix_now().saturating_sub(self.fetched_at);
            let fresh = self.fetched_at > 0 && (0..REFRESH.as_secs() as i64).contains(&age);
            let wait = if fresh { REFRESH - Duration::from_secs(age as u64) } else { Duration::ZERO };
            self.next = Some(Instant::now() + wait);
        }
    }

    fn wants_fetch(&self) -> bool {
        self.config.enabled && self.config.from_url && is_list_url(&self.config.url)
    }

    /// Fetch at the next tick (the page's Update Now button).
    pub fn refresh_now(&mut self) {
        if self.wants_fetch() {
            self.next = Some(Instant::now());
        }
    }

    /// Takes a finished fetch and starts one that is due, on a worker thread, unless the
    /// transfers are held. `proxy` is asked only when a fetch starts. Returns lines for the
    /// Logger; a new list waits in [`IpFilterList::take_filter`].
    pub fn tick(&mut self, proxy: impl FnOnce() -> Option<FetchProxy>) -> Vec<String> {
        let mut log = Vec::new();
        if let Some(rx) = &self.running {
            match rx.try_recv() {
                Ok((url, result, held)) => {
                    self.running = None;
                    // an answer for an address that is no longer the one set is dropped
                    if url == self.config.url && self.wants_fetch() {
                        self.fetch_done(url, result, held, &mut log);
                    } else if let Some(part) = self.part() {
                        let _ = std::fs::remove_file(part);
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.running = None;
                    self.next = Some(Instant::now() + RETRY);
                }
            }
        }
        let due = self.next.is_some_and(|t| Instant::now() >= t);
        if self.running.is_none() && self.wants_fetch() && due && !(self.held)() {
            self.next = None;
            let (tx, rx) = mpsc::channel();
            let (url, proxy, fetcher, held, part) =
                (self.config.url.clone(), proxy(), self.fetcher, self.held, self.part());
            let spawned = std::thread::Builder::new().name("ip-filter-list".into()).spawn(move || {
                let result = fetcher(&url, proxy).and_then(|body| {
                    let list = parse_list(&body)?;
                    // a `.part` this answer could not be written to must not stand in for it
                    // (one an earlier run left would be kept as this address's list)
                    if let Some(part) = part.filter(|p| private_file::write(p, &body).is_err()) {
                        let _ = std::fs::remove_file(part);
                    }
                    Ok(list)
                });
                let held = result.is_err() && held();
                let _ = tx.send((url, result, held));
            });
            match spawned {
                Ok(_) => self.running = Some(rx),
                Err(_) => self.next = Some(Instant::now() + RETRY),
            }
        }
        log
    }

    fn fetch_done(
        &mut self,
        url: String,
        result: Result<IpFilterLoad, Reject>,
        held: bool,
        log: &mut Vec<String>,
    ) {
        let shown = redact(&url);
        match result {
            Ok(l) => {
                log.push(tr!(LogIpFilterRules, count = l.rules, path = shown));
                self.ready = Some(l.filter);
                self.fetched_url = url;
                self.fetched_at = rookparcel_core::util::time::unix_now();
                self.rules = l.rules;
                self.error = None;
                self.next = Some(Instant::now() + REFRESH);
                self.save_cache();
            }
            // the hold came meanwhile: tried again once it is lifted
            Err(_) if held => self.next = Some(Instant::now()),
            Err(e) => {
                // the rules in force stay
                let kept = self.fetched_at > 0 && self.fetched_url == url;
                log.push(if kept {
                    tr!(LogIpFilterKept, path = shown, error = e.text())
                } else {
                    tr!(LogIpFilterError, path = shown, error = e.text())
                });
                self.error = Some(e);
                self.next = Some(Instant::now() + RETRY);
            }
        }
    }

    /// The `.part` the worker wrote becomes the kept list, with its info beside it.
    fn save_cache(&self) {
        let (Some(dir), Some(part), Some(cache)) = (&self.dir, self.part(), self.cache()) else { return };
        if std::fs::rename(&part, &cache).is_err() {
            let _ = std::fs::remove_file(&part);
            return;
        }
        let text = format!("url={}\nfetched={}\nrules={}\n", self.fetched_url, self.fetched_at, self.rules);
        let _ = private_file::write(&dir.join(INFO_FILE), text.as_bytes());
    }

    /// A list fetched since the last call, for `SessionHandle::set_ip_filter`.
    pub fn take_filter(&mut self) -> Option<IpFilter> {
        self.ready.take()
    }

    /// The page's status line for the list from the web; empty while it is switched off.
    pub fn status(&self) -> String {
        self.status_for(self.config.from_url, &self.config.url)
    }

    /// The status line for what a Preferences page shows now, applied or not: empty while its
    /// box is off or its address empty; an address other than the one in use has no list and no
    /// error yet.
    pub fn status_for(&self, from_url: bool, url: &str) -> String {
        let url = url.trim();
        if !from_url || url.is_empty() {
            return String::new();
        }
        let have = (self.fetched_at > 0 && self.fetched_url == url)
            .then(|| tr!(IpFilterStatusOk, count = self.rules, time = fmt::date_time(self.fetched_at)));
        let error = self.error.as_ref().filter(|_| url == self.config.url);
        match (error, have) {
            (Some(e), Some(have)) => format!("{} {have}", tr!(TrackersStatusFailed, error = e.text())),
            (Some(e), None) => tr!(TrackersStatusFailed, error = e.text()),
            (None, Some(have)) => have,
            (None, None) => tr!(TrackersStatusNone).to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use super::*;
    use crate::prefs::Prefs;

    const URL: &str = "https://lists.example/level1.dat";
    const LIST: &str =
        "# a list\n1.2.3.0 - 1.2.3.255 , 000 , Bad Corp\nOther:5.6.7.0-5.6.7.255\n10.0.0.0/8\n";

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rp-ipfilter-list-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn config(url: &str) -> Config {
        Config { enabled: true, from_url: true, url: url.into() }
    }

    fn run(l: &mut IpFilterList) -> Vec<String> {
        let mut log = Vec::new();
        for _ in 0..1000 {
            log.extend(l.tick(|| None));
            if l.running.is_none() {
                return log;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        panic!("the fetch never finished");
    }

    fn never_held() -> bool {
        false
    }

    fn always_held() -> bool {
        true
    }

    fn list(l: IpFilterList) -> IpFilterList {
        IpFilterList { held: never_held, ..l }
    }

    fn good(url: &str, _: Option<FetchProxy>) -> Result<Vec<u8>, Reject> {
        assert!(url.starts_with("https://"));
        Ok(LIST.as_bytes().to_vec())
    }

    fn html(_: &str, _: Option<FetchProxy>) -> Result<Vec<u8>, Reject> {
        Ok(b"<!DOCTYPE html>\n<html><head><title>404</title></head>\n<body>Not found 1.2.3.4</body></html>\n"
            .to_vec())
    }

    fn down(_: &str, _: Option<FetchProxy>) -> Result<Vec<u8>, Reject> {
        Err(Reject::Fetch("timed out".into()))
    }

    fn never(_: &str, _: Option<FetchProxy>) -> Result<Vec<u8>, Reject> {
        panic!("fetched");
    }

    fn blocked(f: &IpFilter, a: &str) -> bool {
        f.is_blocked(a.parse::<IpAddr>().unwrap())
    }

    /// `LIST` as a gzip file: one stored (uncompressed) deflate block.
    fn gzipped() -> Vec<u8> {
        let data = LIST.as_bytes();
        let mut out = vec![0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 0xff];
        out.push(1);
        out.extend((data.len() as u16).to_le_bytes());
        out.extend((!(data.len() as u16)).to_le_bytes());
        out.extend(data);
        out.extend(crc32(data).to_le_bytes());
        out.extend((data.len() as u32).to_le_bytes());
        out
    }

    fn crc32(data: &[u8]) -> u32 {
        let mut crc = !0u32;
        for &b in data {
            crc ^= u32::from(b);
            for _ in 0..8 {
                crc = if crc & 1 != 0 { (crc >> 1) ^ 0xedb8_8320 } else { crc >> 1 };
            }
        }
        !crc
    }

    #[test]
    fn what_is_not_a_block_list_is_refused() {
        let l = parse_list(LIST.as_bytes()).unwrap();
        assert_eq!((l.rules, l.invalid), (3, 0));
        assert!(blocked(&l.filter, "10.9.8.7") && blocked(&l.filter, "5.6.7.8"));
        let l = parse_list(&gzipped()).unwrap();
        assert_eq!(l.rules, 3);
        assert!(matches!(html("", None).map(|b| parse_list(&b)), Ok(Err(Reject::NotAList))));
        assert_eq!(parse_list(b"").unwrap_err(), Reject::NotAList);
        assert_eq!(parse_list(b"# only a comment\n").unwrap_err(), Reject::NotAList);
        // a broken gzip file
        assert_eq!(parse_list(&gzipped()[..20]).unwrap_err(), Reject::NotAList);
        // a page with one address-like line among many others
        let page = "<html>\n<p>\nblocked:\n1.2.3.4\n</p>\n</html>\n";
        assert_eq!(parse_list(page.as_bytes()).unwrap_err(), Reject::NotAList);
    }

    #[test]
    fn a_fetched_list_is_kept_and_a_fresh_one_not_fetched_again() {
        let dir = temp("keep");
        let mut l = list(IpFilterList::new(Some(dir.clone()), config(URL)).with_fetcher(good));
        assert_eq!(l.status(), tr!(TrackersStatusNone));
        let log = run(&mut l);
        assert_eq!(log, [tr!(LogIpFilterRules, count = 3, path = URL)]);
        let f = l.take_filter().expect("a filter");
        assert!(blocked(&f, "1.2.3.200") && !blocked(&f, "1.2.4.0"));
        assert!(l.take_filter().is_none());
        assert_eq!(l.status(), tr!(IpFilterStatusOk, count = 3, time = fmt::date_time(l.fetched_at)));
        // kept as it came, beside its info; the owner loads it with the engine's loader
        assert_eq!(std::fs::read(dir.join(CACHE_FILE)).unwrap(), LIST.as_bytes());
        assert!(!dir.join(format!("{CACHE_FILE}.part")).exists());
        let mut gui = GuiOptions::load(&Prefs::default(), "/dl");
        (gui.ip_filter, gui.ip_filter_from_url, gui.ip_filter_url) = (true, true, URL.into());
        let file = list_file(&gui, &dir).expect("the kept list");
        assert_eq!(IpFilter::load(&file).unwrap().rules, 3);

        // the next start: the kept list is fresh, nothing is fetched for a day
        let mut l = list(IpFilterList::new(Some(dir.clone()), config(URL)).with_fetcher(never));
        assert!(run(&mut l).is_empty());
        assert!(l.next.is_some_and(|t| t > Instant::now() + Duration::from_secs(23 * 3600)));
        assert!(l.status().contains('3'));
        // Update Now fetches all the same
        l.fetcher = good;
        l.refresh_now();
        assert_eq!(run(&mut l).len(), 1);

        // another address: the kept list is forgotten and the new one fetched at once
        let other = "https://other.example/list.p2p";
        l.configure(config(other));
        assert!(!dir.join(CACHE_FILE).exists() && !dir.join(INFO_FILE).exists());
        gui.ip_filter_url = other.into();
        assert_eq!(list_file(&gui, &dir), None, "nothing fetched from it yet");
        assert_eq!(l.status(), tr!(TrackersStatusNone));
        assert_eq!(run(&mut l).len(), 1);
        assert_eq!(list_file(&gui, &dir), Some(dir.join(CACHE_FILE)));
        // a kept list of another address than the preferences' is not loaded
        gui.ip_filter_url = URL.into();
        assert_eq!(list_file(&gui, &dir), None);
        // the file source ignores it
        gui.ip_filter_from_url = false;
        assert_eq!(list_file(&gui, &dir), Some(dir.join("ipfilter.dat")));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_failed_fetch_keeps_the_rules_and_tries_again_in_an_hour() {
        let dir = temp("fail");
        let mut l = list(IpFilterList::new(Some(dir.clone()), config(URL)).with_fetcher(good));
        run(&mut l);
        l.take_filter().unwrap();
        // an error page: refused, the old list stays in use and on disk
        l.fetcher = html;
        l.refresh_now();
        let log = run(&mut l);
        assert_eq!(log, [tr!(LogIpFilterKept, path = URL, error = tr!(IpFilterErrNotList))]);
        assert!(l.take_filter().is_none());
        assert_eq!(l.error, Some(Reject::NotAList));
        assert_eq!(std::fs::read(dir.join(CACHE_FILE)).unwrap(), LIST.as_bytes());
        let next = l.next.unwrap();
        assert!(next > Instant::now() + Duration::from_secs(3500) && next <= Instant::now() + RETRY);
        assert!(l.status().starts_with(&tr!(TrackersStatusFailed, error = tr!(IpFilterErrNotList))));
        assert!(l.status().contains('3'), "the list in use is still named");
        // a fetch that fails without any list: an error, not "the old rules stay"
        let mut l = list(IpFilterList::new(None, config("https://down.example/x.gz")).with_fetcher(down));
        let log = run(&mut l);
        assert_eq!(log, [tr!(LogIpFilterError, path = "https://down.example/x.gz", error = "timed out")]);
        assert_eq!(l.status(), tr!(TrackersStatusFailed, error = "timed out"));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_gzipped_list_is_kept_gzipped() {
        fn gz(_: &str, _: Option<FetchProxy>) -> Result<Vec<u8>, Reject> {
            Ok(gzipped())
        }
        let dir = temp("gzip");
        let mut l = list(IpFilterList::new(Some(dir.clone()), config(URL)).with_fetcher(gz));
        run(&mut l);
        assert!(blocked(&l.take_filter().unwrap(), "5.6.7.1"));
        assert_eq!(std::fs::read(dir.join(CACHE_FILE)).unwrap(), gzipped());
        assert_eq!(IpFilter::load(dir.join(CACHE_FILE)).unwrap().rules, 3);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn nothing_is_fetched_while_held_or_switched_off() {
        let mut l = IpFilterList::new(None, config(URL)).with_fetcher(never);
        l.held = always_held;
        // due, but held: it waits, and starts at the first tick after the hold
        assert!(run(&mut l).is_empty() && l.next.is_some());
        l.held = never_held;
        l.fetcher = good;
        assert_eq!(run(&mut l).len(), 1);
        // a fetch that failed because the hold came meanwhile is due again at once, unlogged
        let mut l = list(IpFilterList::new(None, config(URL)).with_fetcher(down));
        let mut log = Vec::new();
        l.fetch_done(URL.into(), Err(Reject::Fetch("held".into())), true, &mut log);
        assert!(log.is_empty() && l.error.is_none());
        assert!(l.next.is_some_and(|t| t <= Instant::now()));

        // off, the file source, an http address: never fetched
        for c in [
            Config { enabled: false, ..config(URL) },
            Config { from_url: false, ..config(URL) },
            config("http://lists.example/level1.dat"),
        ] {
            let mut l = list(IpFilterList::new(None, c.clone()).with_fetcher(never));
            assert!(run(&mut l).is_empty(), "{c:?}");
        }
        let l = IpFilterList::new(None, config("http://lists.example/level1.dat"));
        assert_eq!(l.error, Some(Reject::NotHttps));
        assert_eq!(l.status(), tr!(TrackersStatusFailed, error = tr!(TrackersErrHttps)));
        assert_eq!(fetch("http://lists.example/x", None), Err(Reject::NotHttps));
    }

    /// The page's line follows its own box and address, applied or not.
    #[test]
    fn the_status_lines() {
        let mut l = list(IpFilterList::new(None, config(URL)).with_fetcher(good));
        run(&mut l);
        let ok = l.status();
        assert_eq!(l.status_for(true, &format!(" {URL} ")), ok);
        assert_eq!(l.status_for(false, URL), "");
        assert_eq!(l.status_for(true, " "), "");
        assert_eq!(l.status_for(true, "https://other.example/x"), tr!(TrackersStatusNone));
        // the page's line: Reload's result first, then the list's, then the default file
        let dir = Path::new("S");
        assert_eq!(page_status(Some(&Ok(7)), Some(ok.clone()), "", dir), tr!(PrefIpFilterLoaded, count = 7));
        assert_eq!(page_status(Some(&Err("bad".into())), None, "", dir), "bad");
        assert_eq!(page_status(None, Some(ok.clone()), "", dir), ok);
        let default = tr!(PrefIpFilterDefault, path = dir.join("ipfilter.dat").display());
        assert_eq!(page_status(None, None, " ", dir), default);
        assert_eq!(page_status(None, None, "D:\\level1.dat", dir), "");
    }

    #[test]
    fn the_address_is_named_without_its_credentials() {
        for (url, shown) in [
            (URL, URL),
            ("https://lists.example/level1.gz?username=me&pin=123", "https://lists.example/level1.gz"),
            ("https://me:secret@lists.example:8443/l.gz", "https://lists.example:8443/l.gz"),
            ("https://me@x@lists.example/l.gz#part", "https://lists.example/l.gz"),
            ("https://u:p@[2001:db8::1]:8443/l.p2p?k=v", "https://[2001:db8::1]:8443/l.p2p"),
            ("https://[::1]/a/b.dat", "https://[::1]/a/b.dat"),
            ("https://lists.example", "https://lists.example"),
            ("https://u:p@lists.example?pin=1", "https://lists.example"),
            (" https://lists.example/x#y ", "https://lists.example/x"),
            ("lists.example/x?pin=1", "lists.example/x"),
        ] {
            assert_eq!(redact(url), shown, "{url}");
        }
        // the Logger gets the short form, the fetch the whole address
        fn secret(url: &str, _: Option<FetchProxy>) -> Result<Vec<u8>, Reject> {
            assert!(url.ends_with("?pin=123"));
            Err(Reject::Fetch("HTTP 403 Forbidden".into()))
        }
        let url = "https://me:pw@lists.example/level1.gz?pin=123";
        let mut l = list(IpFilterList::new(None, config(url)).with_fetcher(secret));
        let log = run(&mut l);
        let shown = "https://lists.example/level1.gz";
        assert_eq!(log, [tr!(LogIpFilterError, path = shown, error = "HTTP 403 Forbidden")]);
        assert!(!l.status().contains("pin") && !l.status().contains("pw"));
        l.fetcher = good;
        l.refresh_now();
        assert_eq!(run(&mut l), [tr!(LogIpFilterRules, count = 3, path = shown)]);
    }

    #[test]
    fn a_kept_list_that_was_tampered_with_is_not_used() {
        let dir = temp("tamper");
        std::fs::write(dir.join(CACHE_FILE), LIST).unwrap();
        let mut gui = GuiOptions::load(&Prefs::default(), "/dl");
        (gui.ip_filter, gui.ip_filter_from_url, gui.ip_filter_url) = (true, true, URL.into());
        for info in [
            String::new(),
            "url=http://lists.example/level1.dat\nfetched=5\nrules=3\n".to_owned(),
            format!("url={URL}\nfetched=soon\nrules=3\n"),
            format!("url={URL}\nfetched=0\nrules=3\n"),
            format!("url={URL}\nrules=3\n"),
        ] {
            std::fs::write(dir.join(INFO_FILE), &info).unwrap();
            assert_eq!(list_file(&gui, &dir), None, "{info:?}");
            let l = IpFilterList::new(Some(dir.clone()), config(URL));
            assert_eq!(l.fetched_at, 0, "{info:?}");
        }
        std::fs::write(dir.join(INFO_FILE), format!("url={URL}\nfetched=5\nrules=3\n")).unwrap();
        assert_eq!(list_file(&gui, &dir), Some(dir.join(CACHE_FILE)));
        // without the list itself, the info alone is nothing
        std::fs::remove_file(dir.join(CACHE_FILE)).unwrap();
        assert_eq!(list_file(&gui, &dir), None);
        let _ = std::fs::remove_dir_all(dir);
    }
}
