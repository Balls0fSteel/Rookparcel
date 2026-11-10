//! The seeding goal (µTorrent 2.2.1 Preferences > Queueing "Seeding Goal", and the per-torrent
//! "Override default settings" in Properties): once a queued torrent has seeded to a minimum
//! ratio and for a minimum time, it is stopped, or held to an upload rate.
//!
//! libtorrent's `share_ratio_limit` and `seed_time_limit` only rank seeds in the queue, so
//! this is the client's job, as in µTorrent and qBittorrent: [`SeedingGoal::run`] looks at
//! each status update and says what to do. Only torrents the queue manages are touched; a
//! force-started torrent seeds on, as in µTorrent.
//!
//! The default goal (150 % and no minimum time, as in µTorrent) does nothing by itself: it
//! acts only while "Limit the upload rate to X kB/s [0: stop]" is ticked, which is off by
//! default as in µTorrent. Ticked, 0 stops the torrent and X caps its upload rate. A goal of
//! the torrent's own (Properties, the Web API's share limits, an import) always acts: it was
//! set for that torrent on purpose. 0 switches a target off, and with both off there is no
//! goal. With both set, both must be reached ("minimum" ratio and "minimum" time).
//!
//! A goal acts on a torrent once: `torrent.<info-hash>.goal_done` records the goal it was
//! done for, so a torrent the user starts again seeds on. A changed goal acts again.

use rookparcel_core::{TorrentFlags, TorrentHandle, TorrentState, TorrentStatus};

use crate::prefs::Prefs;
use crate::{fmt, tr};

/// One goal: share ratio in percent and seeding time in minutes; 0 = that target is off.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SeedGoal {
    pub ratio_pct: u32,
    pub minutes: u32,
}

impl Default for SeedGoal {
    fn default() -> SeedGoal {
        SeedGoal { ratio_pct: 150, minutes: 0 }
    }
}

impl SeedGoal {
    /// At least one target is set.
    pub fn is_set(&self) -> bool {
        self.ratio_pct > 0 || self.minutes > 0
    }

    /// Every target that is set has been reached.
    pub fn reached(&self, st: &TorrentStatus) -> bool {
        if !self.is_set() {
            return false;
        }
        let ratio_ok = self.ratio_pct == 0 || st.ratio() * 100.0 >= self.ratio_pct as f64;
        let time_ok = self.minutes == 0 || st.seeding_duration >= self.minutes as u64 * 60;
        ratio_ok && time_ok
    }

    /// A torrent's own goal, if it overrides the default (`torrent.<info-hash>.seed_ratio` and
    /// `.seed_time` in the preferences, beside its label).
    pub fn load_override(prefs: &Prefs, key: &str) -> Option<SeedGoal> {
        let ratio = prefs.get_i32(&format!("torrent.{key}.seed_ratio"))?;
        let minutes = prefs.get_i32(&format!("torrent.{key}.seed_time")).unwrap_or(0);
        Some(SeedGoal { ratio_pct: ratio.max(0) as u32, minutes: minutes.max(0) as u32 })
    }

    /// Sets (`Some`) or removes a torrent's own goal.
    pub fn store_override(prefs: &mut Prefs, key: &str, goal: Option<SeedGoal>) {
        let (r, t) = (format!("torrent.{key}.seed_ratio"), format!("torrent.{key}.seed_time"));
        match goal {
            Some(g) => {
                prefs.set(&r, g.ratio_pct);
                prefs.set(&t, g.minutes);
            }
            None => {
                prefs.remove(&r);
                prefs.remove(&t);
            }
        }
    }

    /// Drops everything kept for a removed torrent: its own goal and the mark of a goal done.
    /// Returns whether there was anything.
    pub fn forget(prefs: &mut Prefs, key: &str) -> bool {
        let done = done_key(key);
        let had = SeedGoal::load_override(prefs, key).is_some() || prefs.get(&done).is_some();
        SeedGoal::store_override(prefs, key, None);
        prefs.remove(&done);
        had
    }

    /// What the Logger shows of a reached goal: "ratio 1.523, goal 1.500", the seeding time
    /// after it when that target is set.
    fn reason(&self, st: &TorrentStatus) -> String {
        let mut parts = Vec::new();
        if self.ratio_pct > 0 {
            let goal = fmt::ratio(self.ratio_pct as f64 / 100.0);
            parts.push(tr!(SeedingGoalRatio, ratio = fmt::ratio(st.ratio()), goal));
        }
        if self.minutes > 0 {
            let goal = fmt::duration(self.minutes as u64 * 60);
            parts.push(tr!(SeedingGoalTime, time = fmt::duration(st.seeding_duration), goal));
        }
        parts.join("; ")
    }
}

/// `torrent.<info-hash>.goal_done`: the goal ("ratio:minutes") that has acted on the torrent.
fn done_key(key: &str) -> String {
    format!("torrent.{key}.goal_done")
}

/// `queue.seed_goal_rule`: 2 since the default goal acts only with its box ticked (1.2.45).
/// [`migrate`] and every `GuiOptions::store` write it.
pub const RULE_KEY: &str = "queue.seed_goal_rule";
pub const RULE: u32 = 2;

/// Carries settings written before 1.2.45 over, once, right after they are read. Until then
/// "Limit the upload rate" off meant "stop the torrent", so every profile stopped its seeds at
/// the default 150 % whether its user had asked for that or not. Now the box off means the
/// default goal does nothing. A profile still at 150 % and no time is left like that (nobody
/// chose it); one whose goal was changed keeps stopping: its box is ticked with a rate of 0.
/// A profile without the keys is new and has nothing to carry over. Returns whether the
/// preferences changed.
pub fn migrate(prefs: &mut Prefs) -> bool {
    if prefs.get(RULE_KEY).is_some() {
        return false;
    }
    let (ratio, minutes) = (prefs.get_i32("queue.seed_ratio"), prefs.get_i32("queue.seed_time"));
    if ratio.is_none() && minutes.is_none() {
        return false;
    }
    let goal = SeedGoal {
        ratio_pct: ratio.map_or(SeedGoal::default().ratio_pct, |v| v.max(0) as u32),
        minutes: minutes.unwrap_or(0).max(0) as u32,
    };
    if goal.is_set() && goal != SeedGoal::default() && !prefs.get_bool("queue.seed_limit", false) {
        prefs.set_bool("queue.seed_limit", true);
        prefs.set("queue.seed_limit_kib", 0);
    }
    prefs.set(RULE_KEY, RULE);
    true
}

/// What to do with a torrent that reached its goal.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GoalAction {
    /// µTorrent's Stop: out of the queue and paused (it then shows as "Finished").
    Stop,
    /// Cap the torrent's upload rate, in bytes per second.
    LimitUpload(u32),
}

impl GoalAction {
    pub fn apply(self, t: &TorrentHandle) {
        match self {
            GoalAction::Stop => crate::actions::stop(t),
            GoalAction::LimitUpload(bytes) => t.set_upload_limit(bytes),
        }
    }
}

/// The default goal and what happens when a torrent reaches one (Preferences > Queueing).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SeedingGoal {
    pub goal: SeedGoal,
    /// "Limit the upload rate to X [0: stop]": off, the default goal does nothing.
    pub limit_upload: bool,
    /// kB/s; 0 = stop.
    pub limit_kib: u32,
}

impl SeedingGoal {
    pub fn action(&self) -> GoalAction {
        if self.limit_upload && self.limit_kib > 0 {
            GoalAction::LimitUpload(self.limit_kib.saturating_mul(1024))
        } else {
            GoalAction::Stop
        }
    }

    /// The default goal stops torrents: its box is ticked with a rate of 0. (A torrent's own
    /// goal stops it whenever [`SeedingGoal::action`] is `Stop`.)
    pub fn default_stops(&self) -> bool {
        self.limit_upload && self.limit_kib == 0
    }

    /// The goal in force for a torrent: its own, else the default while that one's box is on.
    pub fn goal_for(&self, own: Option<SeedGoal>) -> Option<SeedGoal> {
        own.or(self.limit_upload.then_some(self.goal))
    }

    /// What to do with this torrent now, given its own goal (`own`, else the default): `None`
    /// unless it is a queued torrent seeding past a goal in force.
    pub fn check(&self, st: &TorrentStatus, own: Option<SeedGoal>) -> Option<GoalAction> {
        let managed =
            st.flags.contains(TorrentFlags::AUTO_MANAGED) && !st.flags.contains(TorrentFlags::PAUSED);
        let checking = matches!(st.state, TorrentState::CheckingFiles | TorrentState::CheckingResumeData);
        if !managed || checking || !st.is_finished || !st.error.is_empty() || st.moving_storage {
            return None;
        }
        if !self.goal_for(own)?.reached(st) {
            return None;
        }
        match self.action() {
            GoalAction::LimitUpload(b) if st.upload_limit > 0 && st.upload_limit <= b => None,
            a => Some(a),
        }
    }

    /// One status update of a torrent: applies the goal it reached, if that goal has not acted
    /// on it before, and returns the Logger's line. `prefs` holds the torrent's own goal and
    /// the mark of the goal done, which keeps a torrent the user starts again seeding.
    pub fn run(&self, prefs: &mut Prefs, st: &TorrentStatus, t: &TorrentHandle) -> Option<String> {
        if !st.is_finished {
            return None;
        }
        let key = st.info_hashes.key_hex();
        let own = SeedGoal::load_override(prefs, &key);
        let action = self.check(st, own)?;
        let goal = self.goal_for(own)?;
        let (done, mark) = (done_key(&key), format!("{}:{}", goal.ratio_pct, goal.minutes));
        if prefs.get(&done) == Some(mark.as_str()) {
            return None;
        }
        prefs.set(&done, mark);
        action.apply(t);
        let what = match action {
            GoalAction::Stop => tr!(SeedingGoalStopped).to_owned(),
            GoalAction::LimitUpload(b) => tr!(SeedingGoalLimited, rate = fmt::speed_always(b)),
        };
        Some(format!("{} ({})", tr!(LogSeedingGoal, name = st.name, what), goal.reason(st)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed(up: u64, size: u64, seeded_secs: u64) -> TorrentStatus {
        TorrentStatus {
            flags: TorrentFlags::AUTO_MANAGED,
            state: TorrentState::Seeding,
            is_finished: true,
            is_seeding: true,
            all_time_upload: up,
            all_time_download: size,
            total_wanted_done: size,
            seeding_duration: seeded_secs,
            ..Default::default()
        }
    }

    #[test]
    fn goals_are_reached_when_every_target_set_is() {
        let g = SeedGoal::default();
        assert_eq!((g.ratio_pct, g.minutes), (150, 0));
        assert!(!g.reached(&seed(140, 100, 0)));
        assert!(g.reached(&seed(150, 100, 0)));
        let both = SeedGoal { ratio_pct: 100, minutes: 10 };
        assert!(!both.reached(&seed(200, 100, 599)), "the time is not up yet");
        assert!(both.reached(&seed(200, 100, 600)));
        let time_only = SeedGoal { ratio_pct: 0, minutes: 1 };
        assert!(time_only.reached(&seed(0, 100, 60)));
        assert!(!SeedGoal { ratio_pct: 0, minutes: 0 }.reached(&seed(1000, 1, 1_000_000)), "no goal");
    }

    #[test]
    fn only_queued_seeds_past_their_goal_are_stopped_or_limited() {
        let st = seed(300, 100, 10);
        // the default goal does nothing until its box is ticked; a torrent's own goal acts
        let off = SeedingGoal::default();
        assert_eq!(off.check(&st, None), None);
        assert!(!off.default_stops());
        assert_eq!(off.check(&st, Some(SeedGoal { ratio_pct: 200, minutes: 0 })), Some(GoalAction::Stop));
        let stop = SeedingGoal { limit_upload: true, ..off };
        assert!(stop.default_stops());
        assert_eq!(stop.check(&st, None), Some(GoalAction::Stop));
        // a torrent's own goal wins over the default
        assert_eq!(stop.check(&st, Some(SeedGoal { ratio_pct: 400, minutes: 0 })), None);
        assert_eq!(stop.check(&st, Some(SeedGoal { ratio_pct: 0, minutes: 0 })), None, "seed forever");
        // forced, stopped, still downloading or checking: left alone
        let forced = TorrentStatus { flags: TorrentFlags::empty(), ..st.clone() };
        assert_eq!(stop.check(&forced, None), None);
        let stopped =
            TorrentStatus { flags: TorrentFlags::PAUSED | TorrentFlags::AUTO_MANAGED, ..st.clone() };
        assert_eq!(stop.check(&stopped, None), None);
        let unfinished = TorrentStatus { is_finished: false, state: TorrentState::Downloading, ..st.clone() };
        assert_eq!(stop.check(&unfinished, None), None);
        let checking = TorrentStatus { state: TorrentState::CheckingFiles, ..st.clone() };
        assert_eq!(stop.check(&checking, None), None);

        // "Limit the upload rate to 0" stops too; another rate caps it once
        let cap = SeedingGoal { limit_kib: 5, ..stop };
        assert!(!cap.default_stops());
        assert_eq!(cap.check(&st, None), Some(GoalAction::LimitUpload(5120)));
        let capped = TorrentStatus { upload_limit: 5120, ..st.clone() };
        assert_eq!(cap.check(&capped, None), None);
        let lower = TorrentStatus { upload_limit: 1024, ..st.clone() };
        assert_eq!(cap.check(&lower, None), None, "a lower limit of its own stays");
        let higher = TorrentStatus { upload_limit: 50 * 1024, ..st };
        assert_eq!(cap.check(&higher, None), Some(GoalAction::LimitUpload(5120)));
    }

    #[test]
    fn overrides_live_in_the_preferences() {
        let mut prefs = Prefs::default();
        assert_eq!(SeedGoal::load_override(&prefs, "ab"), None);
        SeedGoal::store_override(&mut prefs, "ab", Some(SeedGoal { ratio_pct: 200, minutes: 30 }));
        assert!(prefs.to_text().contains("torrent.ab.seed_ratio=200\n"));
        assert_eq!(SeedGoal::load_override(&prefs, "ab"), Some(SeedGoal { ratio_pct: 200, minutes: 30 }));
        SeedGoal::store_override(&mut prefs, "ab", None);
        assert_eq!(SeedGoal::load_override(&prefs, "ab"), None);
        assert!(!prefs.to_text().contains("torrent.ab"));
    }

    #[test]
    fn settings_from_before_the_box_meant_on_are_carried_over_once() {
        let of = |pairs: &[(&str, &str)]| {
            let mut p = Prefs::default();
            for (k, v) in pairs {
                p.set(k, v);
            }
            let changed = migrate(&mut p);
            (changed, p.get_bool("queue.seed_limit", false), p.get_i32("queue.seed_limit_kib"), p)
        };
        // a new profile: nothing to carry over, nothing written
        let (changed, on, _, p) = of(&[]);
        assert!(!changed && !on && p.get(RULE_KEY).is_none());
        // the old default, which nobody chose: the goal is off from now on
        let (changed, on, _, p) =
            of(&[("queue.seed_ratio", "150"), ("queue.seed_time", "0"), ("queue.seed_limit", "0")]);
        assert!(changed && !on);
        assert_eq!(p.get_i32(RULE_KEY), Some(RULE as i32));
        // a goal somebody set keeps stopping
        let (_, on, kib, _) =
            of(&[("queue.seed_ratio", "200"), ("queue.seed_time", "0"), ("queue.seed_limit", "0")]);
        assert!(on);
        assert_eq!(kib, Some(0));
        let (_, on, _, _) = of(&[("queue.seed_ratio", "150"), ("queue.seed_time", "60")]);
        assert!(on);
        // no goal at all, or a cap already chosen: as they were
        let (_, on, _, _) = of(&[("queue.seed_ratio", "0"), ("queue.seed_time", "0")]);
        assert!(!on);
        let (_, on, kib, mut p) =
            of(&[("queue.seed_ratio", "300"), ("queue.seed_limit", "1"), ("queue.seed_limit_kib", "8")]);
        assert!(on);
        assert_eq!(kib, Some(8));
        // once: a box unticked afterwards stays unticked
        p.set_bool("queue.seed_limit", false);
        assert!(!migrate(&mut p));
        assert!(!p.get_bool("queue.seed_limit", true));
    }
}
