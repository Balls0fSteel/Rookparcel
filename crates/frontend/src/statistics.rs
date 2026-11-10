//! Help > Statistics (D65): the all-time and session totals as rows of text, the same in every
//! client. The engine counts them ([`SessionStats`]); nothing here is kept.

use rookparcel_core::SessionStats;

use crate::{fmt, tr};

/// The engine's block: `SessionStats::disk_write_queue` counts these.
const BLOCK: u64 = 16 * 1024;

/// One line of the window.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Row {
    /// A group's title.
    Heading(String),
    /// A label (with its colon) and its value.
    Value { label: String, value: String },
}

/// Uploaded over downloaded: infinite when only uploading happened, 0 when nothing did.
pub fn share_ratio(uploaded: u64, downloaded: u64) -> f64 {
    match (uploaded, downloaded) {
        (0, 0) => 0.0,
        (_, 0) => f64::INFINITY,
        (up, down) => up as f64 / down as f64,
    }
}

/// The share of the blocks read for peers that the read cache answered, in parts per million;
/// `None` before the first read.
pub fn cache_hit_ppm(s: &SessionStats) -> Option<u32> {
    (s.disk_blocks_read > 0).then(|| {
        let hits = s.disk_cache_hits.min(s.disk_blocks_read) as u128;
        (hits * 1_000_000 / s.disk_blocks_read as u128) as u32
    })
}

/// The window's lines, top to bottom. The number and order of the lines do not depend on the
/// figures, so a client can fill its controls once and only change the values after that.
pub fn rows(s: &SessionStats) -> Vec<Row> {
    let value = |label: &str, value: String| Row::Value { label: label.to_owned(), value };
    vec![
        Row::Heading(tr!(StatsAllTime).to_owned()),
        value(tr!(GenUploaded), fmt::size(s.alltime_upload)),
        value(tr!(GenDownloaded), fmt::size(s.alltime_download)),
        value(tr!(GenShareRatio), fmt::ratio(share_ratio(s.alltime_upload, s.alltime_download))),
        value(tr!(StatsSince), fmt::date_time(s.alltime_since)),
        Row::Heading(tr!(StatsSession).to_owned()),
        value(tr!(GenUploaded), fmt::size(s.sent_payload_bytes)),
        value(tr!(GenDownloaded), fmt::size(s.recv_payload_bytes)),
        value(tr!(GenWasted), fmt::size(s.wasted_bytes)),
        value(tr!(StatsPeers), fmt::count(s.num_peers_connected as usize)),
        Row::Heading(tr!(StatsDisk).to_owned()),
        value(tr!(StatsWriteQueue), fmt::size(s.disk_write_queue as u64 * BLOCK)),
        value(tr!(StatsReadCache), fmt::size(s.read_cache_bytes)),
        value(tr!(StatsCacheHits), fmt::percent(cache_hit_ppm(s).unwrap_or(0))),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ratio_has_no_division_by_zero() {
        assert_eq!(share_ratio(0, 0), 0.0);
        assert!(share_ratio(1, 0).is_infinite());
        assert_eq!(share_ratio(3, 2), 1.5);
        assert_eq!(fmt::ratio(share_ratio(1, 0)), "∞");
    }

    #[test]
    fn cache_hits_never_pass_the_whole() {
        let mut s = SessionStats::default();
        assert_eq!(cache_hit_ppm(&s), None);
        s.disk_blocks_read = 4;
        s.disk_cache_hits = 1;
        assert_eq!(cache_hit_ppm(&s), Some(250_000));
        s.disk_cache_hits = 9;
        assert_eq!(cache_hit_ppm(&s), Some(1_000_000));
        s.disk_blocks_read = u64::MAX;
        s.disk_cache_hits = u64::MAX;
        assert_eq!(cache_hit_ppm(&s), Some(1_000_000));
    }

    #[test]
    fn the_rows_keep_their_shape_whatever_the_figures() {
        let empty = rows(&SessionStats::default());
        let busy = rows(&SessionStats {
            alltime_upload: 3 << 30,
            alltime_download: 1 << 30,
            alltime_since: 1_794_300_000,
            sent_payload_bytes: 5 << 20,
            recv_payload_bytes: 7 << 20,
            wasted_bytes: 16_384,
            num_peers_connected: 1234,
            disk_write_queue: 2,
            disk_blocks_read: 10,
            disk_cache_hits: 5,
            ..SessionStats::default()
        });
        assert_eq!(empty.len(), busy.len());
        for (a, b) in empty.iter().zip(&busy) {
            match (a, b) {
                (Row::Heading(x), Row::Heading(y)) => assert_eq!(x, y),
                (Row::Value { label: x, .. }, Row::Value { label: y, .. }) => assert_eq!(x, y),
                _ => panic!("{a:?} / {b:?}"),
            }
        }
        let text = |rows: &[Row], i: usize| match &rows[i] {
            Row::Value { value, .. } => value.clone(),
            Row::Heading(h) => h.clone(),
        };
        assert_eq!(text(&busy, 3), "3.000");
        assert_eq!(text(&busy, 9), "1,234");
        assert_eq!(text(&busy, 11), fmt::size(32_768));
        assert_eq!(text(&busy, 13), "50.0 %");
        // nothing counted yet: no date
        assert_eq!(text(&empty, 3), "0.000");
        assert_eq!(text(&empty, 4), "");
        assert_eq!(text(&empty, 13), "0.0 %");
    }
}
