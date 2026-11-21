//! The RSS Downloader's network side, shared by the frontends: feeds and `.torrent` files are
//! fetched on worker threads (one per feed refresh, one per batch of downloads), and
//! [`Worker::collect`] hands the results to the [`RssDownloader`] on the frontend's own thread
//! and returns the torrents to add. The worker starts threads and calls the frontend's
//! `notify` when one finishes; it never touches the model from another thread, so the
//! frontend decides when to look (the Win32 window on a posted message, nox on its status
//! tick).
//!
//! A download that fails is forgotten ([`RssDownloader::forget`]), so the item is tried again
//! at the next refresh; one the engine refuses (a duplicate, a broken file) stays added.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};

use rookparcel_core::fetch::{FetchOptions, FetchProxy, Reach, http_get};
use rookparcel_core::metainfo::{TorrentInfo, parse_magnet_uri};
use rookparcel_core::{AddTorrentParams, InfoHashes, TorrentFlags, TorrentSource};

use super::{AddRequest, FeedId, Fetched, RssDownloader, Validators, fetch_feed};
use crate::tr;

/// What an item's URL gave: a `.torrent` file, or a magnet link (named by the item, or where
/// the download redirected, as indexers do).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Download {
    Torrent(Vec<u8>),
    Magnet(String),
}

impl Download {
    /// The torrent's info-hashes, when the file parses or the link names them.
    pub fn info_hashes(&self) -> Option<InfoHashes> {
        match self {
            Download::Torrent(b) => TorrentInfo::from_bytes(b).ok().map(|ti| ti.info_hashes()),
            Download::Magnet(m) => parse_magnet_uri(m).ok().map(|m| m.info_hashes),
        }
    }

    pub fn into_source(self) -> TorrentSource {
        match self {
            Download::Torrent(b) => TorrentSource::TorrentFile(b),
            Download::Magnet(m) => TorrentSource::Magnet(m),
        }
    }
}

impl AddRequest {
    /// Fills in what the request decides, after the frontend's defaults: the filter's Save in
    /// and "Add in paused state" (µTorrent's Stop: paused and outside the queue). The label and
    /// "highest priority" (top of the queue once added) are the frontend's to apply.
    pub fn apply_to(&self, p: &mut AddTorrentParams) {
        if !self.save_path.trim().is_empty() {
            p.save_path = self.save_path.trim().to_owned();
        }
        if self.paused {
            p.flags = p.flags & !TorrentFlags::AUTO_MANAGED | TorrentFlags::PAUSED;
        }
    }
}

/// Fetches what an item names (blocking: call it from a worker thread): a magnet link as it
/// is, else the `.torrent` behind the URL, sent with the feed's cookies. The feed operator
/// chose the URL: it reaches the addresses `reach` allows ([`AddRequest::links_reach`]: this
/// machine or network only when the server that sent the feed is there too, SEC-26).
pub fn fetch_download(
    url: &str,
    cookies: &str,
    reach: Reach,
    proxy: Option<FetchProxy>,
) -> Result<Download, String> {
    let url = url.trim();
    if url.get(..7).is_some_and(|s| s.eq_ignore_ascii_case("magnet:")) {
        return Ok(Download::Magnet(url.to_owned()));
    }
    let opts = FetchOptions { cookies: cookies.to_owned(), proxy, reach, ..FetchOptions::default() };
    let resp = http_get(url, &opts).map_err(|e| e.to_string())?;
    match resp.redirect() {
        Some(m) if m.get(..7).is_some_and(|s| s.eq_ignore_ascii_case("magnet:")) => {
            Ok(Download::Magnet(m.to_owned()))
        }
        Some(other) => Err(tr!(RssErrRedirect, url = other)),
        None if resp.body.first() == Some(&b'd') => Ok(Download::Torrent(resp.body)),
        // an HTML page (a login form, a "download will start" page) is no torrent
        None => Err(tr!(RssErrNotTorrent, url)),
    }
}

/// How the worker fetches; the network by default, a fake in tests.
pub trait Fetcher: Send + Sync {
    /// `since` is the feed's [`Validators`] (empty: fetch the whole document).
    fn feed(&self, url: &str, since: &Validators, proxy: Option<FetchProxy>) -> Result<Fetched, String>;
    fn download(&self, req: &AddRequest, proxy: Option<FetchProxy>) -> Result<Download, String>;
}

/// [`fetch_feed`] and [`fetch_download`].
pub struct Net;

impl Fetcher for Net {
    fn feed(&self, url: &str, since: &Validators, proxy: Option<FetchProxy>) -> Result<Fetched, String> {
        fetch_feed(url, since, proxy)
    }

    fn download(&self, req: &AddRequest, proxy: Option<FetchProxy>) -> Result<Download, String> {
        fetch_download(&req.url, &req.cookies, req.links_reach, proxy)
    }
}

enum Done {
    /// A feed fetched, with the URL it was fetched from ([`super::RssFeed::url`]) and the
    /// validators the request sent.
    Feed(FeedId, String, Validators, Result<Fetched, String>),
    Download(AddRequest, Result<Download, String>),
    /// A download batch is through.
    Batch,
}

/// What [`Worker::collect`] found.
#[derive(Debug, Default)]
pub struct Collected {
    /// The torrents to add now, with the requests that asked for them.
    pub adds: Vec<(AddRequest, Download)>,
    /// Lines for the Logger.
    pub log: Vec<String>,
    /// The downloader changed (a feed fetched, an item forgotten): save `rss.dat`, refresh
    /// what shows it.
    pub changed: bool,
}

/// Starts the fetches and collects their results; see the module documentation.
pub struct Worker {
    tx: Sender<Done>,
    rx: Receiver<Done>,
    notify: Arc<dyn Fn() + Send + Sync>,
    fetcher: Arc<dyn Fetcher>,
    /// Feeds being fetched, with the URL the newest fetch asked for: a feed whose URL was
    /// edited meanwhile is fetched again at once, and the old fetch's result is dropped.
    busy: HashMap<FeedId, String>,
    /// Feed fetches under way, those of an old URL included.
    feeds_out: usize,
    /// Download batches under way.
    batches: usize,
}

impl Worker {
    /// A worker on the network; `notify` runs on the worker thread after each result.
    pub fn new(notify: impl Fn() + Send + Sync + 'static) -> Worker {
        Worker::with_fetcher(Arc::new(Net), notify)
    }

    pub fn with_fetcher(fetcher: Arc<dyn Fetcher>, notify: impl Fn() + Send + Sync + 'static) -> Worker {
        let (tx, rx) = channel();
        Worker { tx, rx, notify: Arc::new(notify), fetcher, busy: HashMap::new(), feeds_out: 0, batches: 0 }
    }

    /// Whether feed `id` is being fetched.
    pub fn is_busy(&self, id: FeedId) -> bool {
        self.busy.contains_key(&id)
    }

    /// Whether anything is under way.
    pub fn is_idle(&self) -> bool {
        self.feeds_out == 0 && self.batches == 0
    }

    /// Starts fetching the feeds due at `now` that are not being fetched already; returns how
    /// many it started.
    pub fn refresh_due(&mut self, rss: &RssDownloader, now: i64, proxy: Option<&FetchProxy>) -> usize {
        rss.due_feeds(now).into_iter().filter(|&id| self.refresh(rss, id, proxy)).count()
    }

    /// Starts fetching feed `id` now (the dialog's Update); false when it is being fetched
    /// already from its current URL, or does not exist. A fetch from a URL the feed no longer
    /// has (the Edit Feed dialog changed it) does not hold up the new one.
    pub fn refresh(&mut self, rss: &RssDownloader, id: FeedId, proxy: Option<&FetchProxy>) -> bool {
        let Some(feed) = rss.feed(id) else { return false };
        if self.busy.get(&id) == Some(&feed.url) {
            return false;
        }
        self.busy.insert(id, feed.url.clone());
        self.feeds_out += 1;
        let (url, since) = (feed.url.clone(), feed.validators());
        let (tx, notify, fetcher, proxy) =
            (self.tx.clone(), self.notify.clone(), self.fetcher.clone(), proxy.cloned());
        std::thread::spawn(move || {
            let r = fetcher.feed(&url, &since, proxy);
            if tx.send(Done::Feed(id, url, since, r)).is_ok() {
                notify();
            }
        });
        true
    }

    /// Starts the downloads `reqs` ask for. Magnet links need no fetching: they come back at
    /// once, to add now.
    pub fn download(
        &mut self,
        reqs: Vec<AddRequest>,
        proxy: Option<&FetchProxy>,
    ) -> Vec<(AddRequest, Download)> {
        let (magnets, files): (Vec<AddRequest>, Vec<AddRequest>) =
            reqs.into_iter().partition(AddRequest::is_magnet);
        if !files.is_empty() {
            self.batches += 1;
            let (tx, notify, fetcher, proxy) =
                (self.tx.clone(), self.notify.clone(), self.fetcher.clone(), proxy.cloned());
            std::thread::spawn(move || {
                for req in files {
                    let r = fetcher.download(&req, proxy.clone());
                    if tx.send(Done::Download(req, r)).is_err() {
                        return;
                    }
                    notify();
                }
                if tx.send(Done::Batch).is_ok() {
                    notify();
                }
            });
        }
        magnets
            .into_iter()
            .map(|r| {
                let m = Download::Magnet(r.url.trim().to_owned());
                (r, m)
            })
            .collect()
    }

    /// Takes the finished fetches into `rss` at `now`: a fetched feed runs its subscription or
    /// the filters and starts the downloads they want; a finished download comes back to add,
    /// a failed one is forgotten so the next refresh tries it again.
    pub fn collect(&mut self, rss: &mut RssDownloader, now: i64, proxy: Option<&FetchProxy>) -> Collected {
        let mut out = Collected::default();
        while let Ok(done) = self.rx.try_recv() {
            match done {
                Done::Batch => self.batches = self.batches.saturating_sub(1),
                Done::Feed(id, url, since, result) => {
                    self.feeds_out = self.feeds_out.saturating_sub(1);
                    if self.busy.get(&id) == Some(&url) {
                        self.busy.remove(&id);
                    }
                    let Some(feed) = rss.feed(id) else {
                        continue; // removed while it was being fetched
                    };
                    // its URL was edited while it was being fetched: the document is the old
                    // URL's, so neither its items nor its error are the feed's (bug hunt
                    // 2026-11-21 #23). Edited away and back, the URL is the same, but a "not
                    // modified" then speaks of the items the edit dropped.
                    let stale = feed.url != url
                        || (matches!(result, Ok(Fetched::NotModified)) && since != feed.validators());
                    if stale {
                        // the current URL is fetched, unless a fetch of it is under way or has
                        // been taken since the edit
                        if !self.busy.contains_key(&id) && feed.last_refresh.is_none() {
                            self.refresh(rss, id, proxy);
                        }
                        continue;
                    }
                    if let Err(e) = &result {
                        let name = rss.feed(id).map(|f| f.name().to_owned()).unwrap_or_default();
                        out.log.push(tr!(RssLogError, name, error = e));
                    }
                    let reqs = rss.feed_fetched(id, now, result);
                    out.changed = true;
                    let name = rss.feed(id).map(|f| f.name().to_owned()).unwrap_or_default();
                    for r in &reqs {
                        out.log.push(tr!(RssLogDownloading, name, title = r.title));
                    }
                    out.adds.extend(self.download(reqs, proxy));
                }
                Done::Download(req, Ok(d)) => out.adds.push((req, d)),
                Done::Download(req, Err(e)) => {
                    out.log.push(tr!(RssLogDownloadFailed, title = req.title, error = e));
                    rss.forget(&req);
                    out.changed = true;
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    use super::super::{Feed, FeedItem, RssFilter};
    use super::*;

    const NOW: i64 = 1_800_000_000;

    /// Feeds and downloads from tables; a URL missing from them fails.
    #[derive(Default)]
    struct Fake {
        feeds: Mutex<Vec<(String, Feed)>>,
        files: Mutex<Vec<(String, Vec<u8>)>>,
        cookies: Mutex<Vec<String>>,
        /// The validators each feed fetch was sent.
        since: Mutex<Vec<Validators>>,
    }

    impl Fetcher for Fake {
        fn feed(&self, url: &str, since: &Validators, _: Option<FetchProxy>) -> Result<Fetched, String> {
            self.since.lock().unwrap().push(since.clone());
            let url = super::super::split_cookie(url).0;
            let feeds = self.feeds.lock().unwrap();
            feeds
                .iter()
                .find(|(u, _)| u == url)
                .map(|(_, f)| f.clone().into())
                .ok_or_else(|| "HTTP 404".into())
        }

        fn download(&self, req: &AddRequest, _: Option<FetchProxy>) -> Result<Download, String> {
            self.cookies.lock().unwrap().push(req.cookies.clone());
            let files = self.files.lock().unwrap();
            files
                .iter()
                .find(|(u, _)| *u == req.url)
                .map(|(_, b)| Download::Torrent(b.clone()))
                .ok_or_else(|| "HTTP 404".into())
        }
    }

    fn item(title: &str, url: &str) -> FeedItem {
        FeedItem { title: title.into(), guid: title.into(), link: url.into(), ..FeedItem::default() }
    }

    /// Collects until `want` adds came back (or a second passed).
    fn collect_until(w: &mut Worker, d: &mut RssDownloader, want: usize) -> Collected {
        let mut all = Collected::default();
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            let c = w.collect(d, NOW, None);
            all.adds.extend(c.adds);
            all.log.extend(c.log);
            all.changed |= c.changed;
            if all.adds.len() >= want && w.is_idle() {
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        all
    }

    #[test]
    fn a_refresh_adds_what_the_filters_take_and_retries_failed_downloads() {
        let fake = Arc::new(Fake::default());
        fake.feeds.lock().unwrap().push((
            "https://f/rss".into(),
            Feed {
                title: "Shows".into(),
                items: vec![
                    item("Show.S01E01.720p", "https://f/1.torrent"),
                    item("Show.S01E02.720p", "https://f/2.torrent"),
                    item("Show.S01E03.720p", "magnet:?xt=urn:btih:0123456789abcdef0123456789abcdef01234567"),
                    item("Other.S01E01", "https://f/other.torrent"),
                ],
                ..Feed::default()
            },
        ));
        fake.files.lock().unwrap().push(("https://f/1.torrent".into(), b"d4:infod4:name1:aee".to_vec()));
        let notified = Arc::new(AtomicUsize::new(0));
        let n = notified.clone();
        let mut w = Worker::with_fetcher(fake.clone(), move || {
            n.fetch_add(1, Ordering::SeqCst);
        });
        let mut d = RssDownloader::default();
        let id = d.add_feed("https://f/rss:COOKIE:uid=5", "");
        d.add_filter(RssFilter { pattern: "show".into(), ..RssFilter::default() });
        assert_eq!(w.refresh_due(&d, NOW, None), 1);
        assert!(w.is_busy(id));
        assert_eq!(w.refresh_due(&d, NOW, None), 0, "a feed being fetched is not started twice");
        assert!(!w.refresh(&d, id, None));
        let c = collect_until(&mut w, &mut d, 2);
        assert!(c.changed);
        assert!(!w.is_busy(id) && w.is_idle());
        assert!(notified.load(Ordering::SeqCst) >= 2);
        let mut titles: Vec<&str> = c.adds.iter().map(|(r, _)| r.title.as_str()).collect();
        titles.sort_unstable();
        assert_eq!(titles, ["Show.S01E01.720p", "Show.S01E03.720p"], "the magnet at once, file 1 fetched");
        assert!(c.adds.iter().any(|(_, d)| matches!(d, Download::Magnet(_))));
        assert!(
            c.log.iter().any(|l| l.contains("could not download Show.S01E02.720p: HTTP 404")),
            "{:?}",
            c.log
        );
        assert!(fake.cookies.lock().unwrap().iter().all(|c| c == "uid=5"), "the feed's cookies go along");
        assert_eq!(d.feed(id).unwrap().title, "Shows");
        // the failed one is forgotten: the next refresh takes it again
        assert!(!d.feed(id).unwrap().downloaded.contains("Show.S01E02.720p"));
        assert!(d.feed(id).unwrap().downloaded.contains("Show.S01E01.720p"));
        fake.files.lock().unwrap().push(("https://f/2.torrent".into(), b"d4:infod4:name1:bee".to_vec()));
        assert!(w.refresh(&d, id, None));
        let c = collect_until(&mut w, &mut d, 1);
        let titles: Vec<&str> = c.adds.iter().map(|(r, _)| r.title.as_str()).collect();
        assert_eq!(titles, ["Show.S01E02.720p"]);
    }

    /// Bug hunt 2026-11-21 #8: a smart filter's episode whose `.torrent` could not be fetched
    /// is fetched again at the next refresh, as the module promises.
    #[test]
    fn a_smart_filters_failed_episode_is_tried_again() {
        let fake = Arc::new(Fake::default());
        let items = vec![
            FeedItem { published: Some(NOW - 10), ..item("Show.S01E05.1080p", "https://f/5-1080.torrent") },
            FeedItem { published: Some(NOW - 20), ..item("Show.S01E05.720p", "https://f/5.torrent") },
        ];
        fake.feeds.lock().unwrap().push(("https://f/rss".into(), Feed { items, ..Feed::default() }));
        let mut w = Worker::with_fetcher(fake.clone(), || {});
        let mut d = RssDownloader::default();
        let id = d.add_feed("https://f/rss", "");
        d.add_filter(RssFilter { pattern: "show".into(), smart: true, ..RssFilter::default() });
        w.refresh(&d, id, None);
        let c = collect_until(&mut w, &mut d, 0);
        assert!(c.adds.is_empty() && c.log.iter().any(|l| l.contains("Show.S01E05.720p")), "{:?}", c.log);
        fake.files.lock().unwrap().push(("https://f/5.torrent".into(), b"d4:infod4:name1:aee".to_vec()));
        w.refresh(&d, id, None);
        let c = collect_until(&mut w, &mut d, 1);
        let titles: Vec<&str> = c.adds.iter().map(|(r, _)| r.title.as_str()).collect();
        assert_eq!(titles, ["Show.S01E05.720p"]);
    }

    /// Feeds from a table, each answered with the validators `"v1"` (and "not modified" when
    /// they are sent back); the fetch of the URL in `hold` waits for `gate`.
    struct Gated {
        feeds: Vec<(String, Feed)>,
        hold: Mutex<Option<String>>,
        gate: Mutex<std::sync::mpsc::Receiver<()>>,
        asked: Mutex<Vec<(String, Validators)>>,
    }

    impl Gated {
        fn new(feeds: Vec<(&str, Feed)>) -> (Arc<Gated>, std::sync::mpsc::Sender<()>) {
            let (tx, rx) = channel();
            let feeds = feeds.into_iter().map(|(u, f)| (u.to_owned(), f)).collect();
            let g = Gated { feeds, hold: Mutex::new(None), gate: Mutex::new(rx), asked: Mutex::default() };
            (Arc::new(g), tx)
        }

        fn v1() -> Validators {
            Validators { etag: "\"v1\"".into(), last_modified: String::new() }
        }
    }

    impl Fetcher for Gated {
        fn feed(&self, url: &str, since: &Validators, _: Option<FetchProxy>) -> Result<Fetched, String> {
            self.asked.lock().unwrap().push((url.to_owned(), since.clone()));
            if self.hold.lock().unwrap().take_if(|h| h == url).is_some() {
                let _ = self.gate.lock().unwrap().recv();
            }
            if *since == Gated::v1() {
                return Ok(Fetched::NotModified);
            }
            let feed = self.feeds.iter().find(|(u, _)| u == url).map(|(_, f)| f.clone());
            feed.map(|f| Fetched::Changed(f, Gated::v1())).ok_or_else(|| format!("HTTP 404 from {url}"))
        }

        fn download(&self, _: &AddRequest, _: Option<FetchProxy>) -> Result<Download, String> {
            Err("no downloads here".into())
        }
    }

    fn magnet(n: u8) -> String {
        format!("magnet:?xt=urn:btih:{}", format!("{n:02x}").repeat(20))
    }

    /// The old and the new address of an edited feed: the old one's items are local, so their
    /// links would reach this network (SEC-26), and its show is one the filter takes.
    fn old_and_new() -> Vec<(&'static str, Feed)> {
        let old = Feed {
            title: "Old".into(),
            items: vec![item("Show.S01E01", &magnet(1))],
            served_locally: true,
            ..Feed::default()
        };
        let new =
            Feed { title: "New".into(), items: vec![item("Show.S02E01", &magnet(2))], ..Feed::default() };
        vec![("https://old/rss", old), ("https://new/rss", new)]
    }

    fn assert_new_document(d: &RssDownloader, id: FeedId, c: &Collected) {
        let titles: Vec<&str> = c.adds.iter().map(|(r, _)| r.title.as_str()).collect();
        assert_eq!(titles, ["Show.S02E01"], "{:?}", c.log);
        let f = d.feed(id).unwrap();
        assert_eq!((f.url.as_str(), f.title.as_str()), ("https://new/rss", "New"));
        assert_eq!(f.items.iter().map(|i| i.title.as_str()).collect::<Vec<_>>(), ["Show.S02E01"]);
        assert!(!f.items_local, "the old server's reach is not the new feed's");
        assert!(!f.downloaded.contains("Show.S01E01") && f.last_error.is_empty());
        assert!(c.adds.iter().all(|(r, _)| r.links_reach == Reach::Public));
        assert!(!c.log.iter().any(|l| l.contains("S01E01") || l.contains("404")), "{:?}", c.log);
    }

    /// Bug hunt 2026-11-21 #23: a feed whose URL is edited while it is being fetched does not
    /// take the old URL's document; the Edit dialog's refresh fetches the new one at once.
    #[test]
    fn a_feed_edited_during_its_fetch_takes_the_new_urls_document_only() {
        let (fake, gate) = Gated::new(old_and_new());
        *fake.hold.lock().unwrap() = Some("https://old/rss".into());
        let mut w = Worker::with_fetcher(fake.clone(), || {});
        let mut d = RssDownloader::default();
        let id = d.add_feed("https://old/rss", "");
        d.add_filter(RssFilter { pattern: "show".into(), ..RssFilter::default() });
        assert!(w.refresh(&d, id, None));
        assert!(d.edit_feed(id, "https://new/rss", ""));
        // what the frontends' Edit Feed does: refresh, which the old fetch does not hold up
        assert!(w.refresh(&d, id, None), "the new URL is fetched at once");
        assert!(!w.refresh(&d, id, None), "but not twice");
        // the new document lands first, while the old fetch is still out
        let mut c = Collected::default();
        let deadline = Instant::now() + Duration::from_secs(5);
        while c.adds.is_empty() && Instant::now() < deadline {
            let more = w.collect(&mut d, NOW, None);
            c.adds.extend(more.adds);
            c.log.extend(more.log);
            std::thread::sleep(Duration::from_millis(5));
        }
        assert_new_document(&d, id, &c);
        assert!(!w.is_idle(), "the old fetch is still out");
        gate.send(()).unwrap();
        let rest = collect_until(&mut w, &mut d, 0);
        assert!(!w.is_busy(id) && w.is_idle());
        assert!(rest.adds.is_empty() && rest.log.is_empty() && !rest.changed, "{:?}", rest.log);
        assert_new_document(&d, id, &c);
        assert_eq!(fake.asked.lock().unwrap().len(), 2, "nothing fetched a third time");
    }

    /// The same when nothing refreshes after the edit: the stale result starts the new fetch.
    #[test]
    fn a_stale_fetch_result_starts_the_fetch_of_the_new_url() {
        let (fake, gate) = Gated::new(old_and_new());
        *fake.hold.lock().unwrap() = Some("https://old/rss".into());
        let mut w = Worker::with_fetcher(fake.clone(), || {});
        let mut d = RssDownloader::default();
        let id = d.add_feed("https://old/rss", "");
        d.add_filter(RssFilter { pattern: "show".into(), ..RssFilter::default() });
        assert!(w.refresh(&d, id, None));
        d.edit_feed(id, "https://new/rss", "");
        gate.send(()).unwrap();
        let c = collect_until(&mut w, &mut d, 1);
        assert_new_document(&d, id, &c);
        let asked: Vec<String> = fake.asked.lock().unwrap().iter().map(|(u, _)| u.clone()).collect();
        assert_eq!(asked, ["https://old/rss", "https://new/rss"]);
    }

    /// Edited away and back during a fetch that sent the validators: the "not modified" that
    /// comes back speaks of items the edit dropped, so the document is fetched whole again.
    #[test]
    fn a_not_modified_for_items_an_edit_dropped_fetches_the_feed_again() {
        let (fake, gate) = Gated::new(old_and_new());
        let mut w = Worker::with_fetcher(fake.clone(), || {});
        let mut d = RssDownloader::default();
        let id = d.add_feed("https://new/rss", "");
        assert!(w.refresh(&d, id, None));
        collect_until(&mut w, &mut d, 0);
        assert_eq!(d.feed(id).unwrap().validators(), Gated::v1());
        *fake.hold.lock().unwrap() = Some("https://new/rss".into());
        assert!(w.refresh(&d, id, None));
        d.edit_feed(id, "https://old/rss", "");
        d.edit_feed(id, "https://new/rss", "");
        assert!(d.feed(id).unwrap().items.is_empty());
        gate.send(()).unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        while d.feed(id).unwrap().items.is_empty() && Instant::now() < deadline {
            w.collect(&mut d, NOW, None);
            std::thread::sleep(Duration::from_millis(5));
        }
        collect_until(&mut w, &mut d, 0);
        let f = d.feed(id).unwrap();
        assert_eq!(f.items.iter().map(|i| i.title.as_str()).collect::<Vec<_>>(), ["Show.S02E01"]);
        let sent: Vec<Validators> = fake.asked.lock().unwrap().iter().map(|(_, v)| v.clone()).collect();
        assert_eq!(sent, [Validators::default(), Gated::v1(), Validators::default()]);
    }

    #[test]
    fn a_failed_feed_is_logged_and_kept() {
        let fake = Arc::new(Fake::default());
        let mut w = Worker::with_fetcher(fake, || {});
        let mut d = RssDownloader::default();
        let id = d.add_feed("https://nowhere/rss", "Nowhere");
        w.refresh(&d, id, None);
        let c = collect_until(&mut w, &mut d, 0);
        assert!(c.changed && c.adds.is_empty());
        assert_eq!(c.log, ["RSS: Nowhere: HTTP 404"]);
        assert_eq!(d.feed(id).unwrap().last_error, "HTTP 404");
        assert_eq!(d.feed(id).unwrap().last_refresh, Some(NOW));
    }

    /// A refresh sends the feed's validators once its items are in memory.
    #[test]
    fn a_refresh_sends_the_validators() {
        let fake = Arc::new(Fake::default());
        let mut w = Worker::with_fetcher(fake.clone(), || {});
        let mut d = RssDownloader::default();
        let id = d.add_feed("https://nowhere/rss", "");
        let v = Validators { etag: "\"v1\"".into(), last_modified: String::new() };
        d.feed_mut(id).unwrap().etag = v.etag.clone();
        w.refresh(&d, id, None);
        collect_until(&mut w, &mut d, 0);
        d.feed_fetched(
            id,
            NOW,
            Ok(Fetched::Changed(Feed { items: vec![item("a", "x")], ..Feed::default() }, v.clone())),
        );
        w.refresh(&d, id, None);
        collect_until(&mut w, &mut d, 0);
        assert_eq!(*fake.since.lock().unwrap(), [Validators::default(), v]);
    }

    #[test]
    fn requests_apply_save_path_and_paused() {
        let req = AddRequest {
            feed: 1,
            filter: Some(2),
            item_id: "x".into(),
            title: "x".into(),
            url: "magnet:?xt=urn:btih:0123456789abcdef0123456789abcdef01234567".into(),
            cookies: String::new(),
            links_reach: Reach::Public,
            recorded: Default::default(),
            save_path: " D:\\tv ".into(),
            label: "tv".into(),
            top_priority: true,
            paused: true,
        };
        let mut p = AddTorrentParams::new(TorrentSource::Magnet(req.url.clone()), "C:\\dl");
        p.flags = TorrentFlags::DEFAULT;
        req.apply_to(&mut p);
        assert_eq!(p.save_path, "D:\\tv");
        assert!(p.flags.contains(TorrentFlags::PAUSED) && !p.flags.contains(TorrentFlags::AUTO_MANAGED));
        let plain = AddRequest { save_path: String::new(), paused: false, ..req.clone() };
        let mut q = AddTorrentParams::new(TorrentSource::Magnet(req.url.clone()), "C:\\dl");
        q.flags = TorrentFlags::DEFAULT;
        plain.apply_to(&mut q);
        assert_eq!((q.save_path.as_str(), q.flags), ("C:\\dl", TorrentFlags::DEFAULT));
        let m = Download::Magnet(req.url.clone());
        assert_eq!(m.info_hashes().unwrap().key_hex(), "0123456789abcdef0123456789abcdef01234567");
        assert_eq!(Download::Torrent(b"junk".to_vec()).info_hashes(), None);
        assert_eq!(
            fetch_download(" MAGNET:?xt=x ", "", Reach::Public, None),
            Ok(Download::Magnet("MAGNET:?xt=x".into()))
        );
    }
}
