//! "Run Program" (D67; µTorrent's Advanced > Run Program, qBittorrent's "Run external
//! program"): starts a program the user named when a torrent is added and when one finishes
//! downloading.
//!
//! libtorrent has no such thing; clients do it on their alerts. [`RunProgram`] is that client
//! side, so every program that owns an engine behaves the same: feed it every alert.
//!
//! What is run is the user's choice and nothing else's:
//!
//! - **No shell.** The command line is split into arguments first ([`split_command`]) and the
//!   placeholders are filled in afterwards, inside each argument ([`expand`]). A torrent's name
//!   or label, which a stranger chose, can therefore never add an argument, a second command
//!   or a redirection.
//! - **The program itself has no placeholders**: the first argument is taken as written, so a
//!   torrent cannot choose what runs.
//! - **No shell told to run a string with a placeholder in it** (`cmd /c ... %N`,
//!   `powershell -Command`, `sh -c`): such a shell re-reads the name as commands, so the line
//!   is refused ([`shell_reads_codes`]); the script is named itself instead.
//! - At most [`MAX_RUNNING`] programs run at once and [`MAX_QUEUED`] wait, so a thousand
//!   torrents finishing together cannot start a thousand processes.
//!
//! A value can still begin with `-`, which a program may read as an option: write the
//! placeholder after `--` or inside a longer argument where that matters.

use std::collections::{HashMap, VecDeque};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::alert::Alert;
use crate::status::TorrentStatus;
use crate::types::TorrentId;

/// Programs running at once.
pub const MAX_RUNNING: usize = 4;
/// Programs waiting for one of those to end; more are dropped, with a Logger line.
pub const MAX_QUEUED: usize = 1000;
/// The longest command line accepted.
pub const MAX_COMMAND: usize = 4096;
/// A torrent added this long ago or longer is not new: its `TorrentAdded` is a duplicate add.
const NEW_FOR: Duration = Duration::from_secs(120);

/// The placeholders and what they stand for, for the Preferences page and the documentation:
/// qBittorrent's letters, as far as this engine knows the value.
pub const PLACEHOLDERS: &[(&str, &str)] = &[
    ("%N", "the torrent's name"),
    ("%L", "its label"),
    ("%F", "where its data is: the file, or the folder of a torrent with several files"),
    ("%D", "the folder it was saved in"),
    ("%Z", "its size in bytes"),
    ("%T", "its current tracker"),
    ("%I", "its info hash (v1), or - when it has none"),
    ("%J", "its info hash (v2), or - when it has none"),
    ("%K", "its id: the v1 hash, or the shortened v2 hash of a v2-only torrent"),
    ("%%", "a percent sign"),
];

/// Why a command line cannot be used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandError {
    /// A `"` that is never closed.
    OpenQuote,
    /// Longer than [`MAX_COMMAND`], or with a control character in it.
    Malformed,
    /// The program (the first argument) holds a placeholder.
    ProgramPlaceholder,
    /// The program is a command shell told to run a string as commands (`cmd /c`,
    /// `powershell -Command`, `sh -c`) and a placeholder is in its arguments: the shell would
    /// read a torrent's name as commands ([`shell_reads_codes`]).
    ShellPlaceholder,
}

/// Splits a command line into the program and its arguments. Blanks separate arguments; double
/// quotes keep blanks together (`"C:\Program Files\x.exe" "%F"`), and `""` inside quotes is one
/// quote character. A backslash is an ordinary character, so Windows paths need no doubling.
/// Nothing else is special: no variables, no pipes, no wildcards, no single quotes. An empty
/// line gives no arguments (the feature is off).
pub fn split_command(line: &str) -> Result<Vec<String>, CommandError> {
    if line.len() > MAX_COMMAND || line.chars().any(char::is_control) {
        return Err(CommandError::Malformed);
    }
    let mut args = Vec::new();
    let mut cur = String::new();
    let mut started = false;
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' if quoted && chars.peek() == Some(&'"') => {
                chars.next();
                cur.push('"');
            }
            '"' => {
                quoted = !quoted;
                started = true;
            }
            c if c.is_whitespace() && !quoted => {
                if started {
                    args.push(std::mem::take(&mut cur));
                    started = false;
                }
            }
            c => {
                cur.push(c);
                started = true;
            }
        }
    }
    if quoted {
        return Err(CommandError::OpenQuote);
    }
    if started {
        args.push(cur);
    }
    if args.first().is_some_and(|p| has_placeholder(p)) {
        return Err(CommandError::ProgramPlaceholder);
    }
    if shell_reads_codes(&args) {
        return Err(CommandError::ShellPlaceholder);
    }
    Ok(args)
}

/// Whether `args` start a command shell that would read a filled-in code as commands. Every
/// argument reaches the program whole, but a shell told to run a string re-reads it, so a name
/// such as `a&calc` would start a second program:
///
/// - `cmd` with a code anywhere (`cmd /c log.bat %N`: cmd parses the whole line again);
/// - PowerShell (`powershell`, `pwsh`) with a code and no `-File` before it (Windows
///   PowerShell's default is `-Command`);
/// - a Unix shell (`sh`, `bash`, `dash`, `zsh`, `ksh`, `mksh`, `ash`, `fish`, `csh`, `tcsh`)
///   with a code and `-c` (or `--command`) among its options.
///
/// Naming the script itself (`"C:\x\done.bat" "%N"`, `powershell -File x.ps1 "%N"`,
/// `sh x.sh "%N"`) passes the name as an argument and is fine. `%%` is no code here.
pub fn shell_reads_codes(args: &[String]) -> bool {
    let Some((program, rest)) = args.split_first() else {
        return false;
    };
    let Some(first_code) = rest.iter().position(|a| has_code(a)) else {
        return false;
    };
    let name = program.rsplit(['/', '\\']).next().unwrap_or(program);
    let name = name.trim_end_matches(['.', ' ']).to_ascii_lowercase();
    let name = name.strip_suffix(".exe").unwrap_or(&name);
    match name {
        "cmd" => true,
        "powershell" | "pwsh" => {
            // `-File` (or a prefix of it, `-f`, `/file`) before the first code: a script, whose
            // arguments stay arguments. PowerShell takes the first of `-File` and `-Command`
            // (`-c`) and reads everything after it as that one's, so a `-File` behind a
            // `-Command` is part of the command.
            for a in &rest[..first_code] {
                let o = a.strip_prefix(['-', '/']).unwrap_or("").to_ascii_lowercase();
                if o.is_empty() {
                    continue;
                }
                if "file".starts_with(&o) {
                    return false;
                }
                if "command".starts_with(&o) {
                    return true;
                }
            }
            true
        }
        "sh" | "bash" | "dash" | "zsh" | "ksh" | "mksh" | "ash" | "fish" | "csh" | "tcsh" => {
            // the options before the first operand (the script, or -c's string)
            let mut options = rest.iter();
            while let Some(a) = options.next() {
                if a.len() < 2 || !a.starts_with(['-', '+']) {
                    break;
                }
                if let Some(long) = a.strip_prefix("--") {
                    let long = long.split_once('=').map_or(long, |(name, _)| name);
                    match long {
                        // fish's `--init-command` runs a string as well
                        "command" | "init-command" => return true,
                        // these take a value, which is no operand: `--rcfile x -c "..."`
                        "rcfile" | "init-file" if !a.contains('=') => {
                            options.next();
                        }
                        _ => {}
                    }
                    continue;
                }
                let letters = &a[1..];
                if letters.contains('c') || (name == "fish" && letters.contains('C')) {
                    return true;
                }
                // `-o pipefail`, `-O extglob`: the next argument is the option's name, and
                // `-c` may follow it
                if letters.contains(['o', 'O']) {
                    options.next();
                }
            }
            false
        }
        _ => false,
    }
}

/// A placeholder that stands for a torrent's value (`%%` is only a percent sign).
fn has_code(arg: &str) -> bool {
    let mut chars = arg.chars();
    while let Some(c) = chars.next() {
        // `%%` is consumed whole, so `%%N` is a percent sign and an N
        if c == '%'
            && chars.next().is_some_and(|n| PLACEHOLDERS.iter().any(|(p, _)| *p != "%%" && p.ends_with(n)))
        {
            return true;
        }
    }
    false
}

fn has_placeholder(arg: &str) -> bool {
    let mut chars = arg.chars();
    while let Some(c) = chars.next() {
        if c == '%' && chars.next().is_some_and(|n| PLACEHOLDERS.iter().any(|(p, _)| p.ends_with(n))) {
            return true;
        }
    }
    false
}

/// What the placeholders stand for, for one torrent.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Values {
    pub name: String,
    pub label: String,
    pub content_path: String,
    pub save_path: String,
    pub size: u64,
    pub tracker: String,
    pub hash_v1: String,
    pub hash_v2: String,
    pub id: String,
}

impl Values {
    pub fn of(st: &TorrentStatus, label: &str) -> Values {
        let hex = |h: Option<String>| h.unwrap_or_else(|| "-".to_owned());
        let sep = if st.save_path.contains('\\') && !st.save_path.contains('/') { '\\' } else { '/' };
        let content_path = match st.save_path.trim_end_matches(['/', '\\']) {
            "" => st.name.clone(),
            dir => format!("{dir}{sep}{}", st.name),
        };
        Values {
            name: st.name.clone(),
            label: label.to_owned(),
            content_path,
            save_path: st.save_path.clone(),
            size: st.total,
            tracker: st.current_tracker.clone(),
            hash_v1: hex(st.info_hashes.v1.map(|h| h.to_hex())),
            hash_v2: hex(st.info_hashes.v2.map(|h| h.to_hex())),
            id: st.info_hashes.best_v1_compatible().to_hex(),
        }
    }
}

/// Fills the placeholders of one argument. A `%` before any other character stays as it is.
/// Control characters in a value (a name from a hostile torrent) are dropped.
pub fn expand(arg: &str, v: &Values) -> String {
    let mut out = String::with_capacity(arg.len());
    let mut chars = arg.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        let value: Option<String> = match chars.peek() {
            Some('N') => Some(v.name.clone()),
            Some('L') => Some(v.label.clone()),
            Some('F') => Some(v.content_path.clone()),
            Some('D') => Some(v.save_path.clone()),
            Some('Z') => Some(v.size.to_string()),
            Some('T') => Some(v.tracker.clone()),
            Some('I') => Some(v.hash_v1.clone()),
            Some('J') => Some(v.hash_v2.clone()),
            Some('K') => Some(v.id.clone()),
            Some('%') => Some("%".to_owned()),
            _ => None,
        };
        match value {
            Some(value) => {
                chars.next();
                out.extend(value.chars().filter(|c| !c.is_control()));
            }
            None => out.push('%'),
        }
    }
    out
}

/// The program and arguments for a torrent: the first argument as written, the rest expanded.
pub fn arguments(command: &[String], v: &Values) -> Vec<String> {
    command.iter().enumerate().map(|(i, a)| if i == 0 { a.clone() } else { expand(a, v) }).collect()
}

fn spawn(args: &[String]) -> std::io::Result<std::process::Child> {
    let mut c = Command::new(&args[0]);
    c.args(&args[1..]).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    #[cfg(windows)]
    {
        // a console program gets no window of its own; a batch file's arguments are escaped
        // by the standard library, which refuses the ones it cannot make safe
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        c.creation_flags(CREATE_NO_WINDOW);
    }
    c.spawn()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Event {
    Added,
    Finished,
}

/// Runs the user's programs on `TorrentAdded` and `TorrentFinished`.
pub struct RunProgram {
    on_added: Vec<String>,
    on_finished: Vec<String>,
    /// Torrents whose alert came before the status update that describes them: whether
    /// "added" and "finished" wait (both, when a torrent finishes within one update of its
    /// add), and since when.
    waiting: HashMap<TorrentId, (bool, bool, Instant)>,
    /// New torrents a program was started for: their id is not run twice (a duplicate add).
    recent: HashMap<TorrentId, Instant>,
    queue: VecDeque<Vec<String>>,
    running: usize,
    done_tx: mpsc::Sender<String>,
    done_rx: mpsc::Receiver<String>,
    /// For tests: called instead of starting a process.
    spawner: fn(&[String]) -> std::io::Result<std::process::Child>,
}

impl RunProgram {
    /// The two command lines; an empty one is off. A line that cannot be split
    /// ([`split_command`]) is off too: the Preferences refuse it before it gets here.
    pub fn new(on_added: &str, on_finished: &str) -> RunProgram {
        let (done_tx, done_rx) = mpsc::channel();
        RunProgram {
            on_added: split_command(on_added).unwrap_or_default(),
            on_finished: split_command(on_finished).unwrap_or_default(),
            waiting: HashMap::new(),
            recent: HashMap::new(),
            queue: VecDeque::new(),
            running: 0,
            done_tx,
            done_rx,
            spawner: spawn,
        }
    }

    /// New command lines; programs already queued still run.
    pub fn set(&mut self, on_added: &str, on_finished: &str) {
        self.on_added = split_command(on_added).unwrap_or_default();
        self.on_finished = split_command(on_finished).unwrap_or_default();
    }

    /// Whether either program is set.
    pub fn enabled(&self) -> bool {
        !self.on_added.is_empty() || !self.on_finished.is_empty()
    }

    /// Looks at one alert; returns lines for the Logger (a program started, ended or failed).
    /// Status updates ([`Alert::StateUpdate`]) say what a torrent is, so the caller keeps
    /// posting them. `label_of` gives a torrent's label; it is asked only when a program runs.
    pub fn on_alert(&mut self, alert: &Alert, label_of: impl Fn(TorrentId) -> Option<String>) -> Vec<String> {
        let mut log = Vec::new();
        let now = Instant::now();
        match alert {
            Alert::TorrentAdded { id, .. } if !self.on_added.is_empty() => {
                let w = self.waiting.entry(*id).or_insert((false, false, now));
                (w.0, w.2) = (true, now);
            }
            Alert::TorrentFinished { id } if !self.on_finished.is_empty() => {
                let w = self.waiting.entry(*id).or_insert((false, false, now));
                (w.1, w.2) = (true, now);
            }
            Alert::TorrentRemoved { id, .. } => {
                self.waiting.remove(id);
                self.recent.remove(id);
            }
            Alert::StateUpdate(v) if !self.waiting.is_empty() => {
                for st in v {
                    if let Some((added, finished, _)) = self.waiting.remove(&st.id) {
                        if added {
                            self.event(Event::Added, st, &label_of, now, &mut log);
                        }
                        if finished {
                            self.event(Event::Finished, st, &label_of, now, &mut log);
                        }
                    }
                }
            }
            _ => {}
        }
        self.waiting.retain(|_, (_, _, at)| now.duration_since(*at) < NEW_FOR);
        self.recent.retain(|_, at| now.duration_since(*at) < NEW_FOR);
        while let Ok(line) = self.done_rx.try_recv() {
            self.running = self.running.saturating_sub(1);
            log.push(line);
        }
        self.start_queued(&mut log);
        log
    }

    fn event(
        &mut self,
        event: Event,
        st: &TorrentStatus,
        label_of: &impl Fn(TorrentId) -> Option<String>,
        now: Instant,
        log: &mut Vec<String>,
    ) {
        let command = match event {
            Event::Added => {
                // a duplicate add posts `TorrentAdded` for the torrent that was there
                let age = crate::util::time::unix_now().saturating_sub(st.added_time);
                if age >= NEW_FOR.as_secs() as i64 || self.recent.insert(st.id, now).is_some() {
                    return;
                }
                &self.on_added
            }
            // a torrent whose data was there already when it was added downloaded nothing
            Event::Finished if st.all_time_download == 0 => return,
            Event::Finished => &self.on_finished,
        };
        if command.is_empty() {
            return;
        }
        let values = Values::of(st, label_of(st.id).as_deref().unwrap_or(""));
        if self.queue.len() >= MAX_QUEUED {
            log.push(format!("Run Program: too many programs are waiting, none started for {}", st.name));
            return;
        }
        self.queue.push_back(arguments(command, &values));
    }

    fn start_queued(&mut self, log: &mut Vec<String>) {
        while self.running < MAX_RUNNING {
            let Some(args) = self.queue.pop_front() else { return };
            let program = args[0].clone();
            match (self.spawner)(&args) {
                Ok(mut child) => {
                    self.running += 1;
                    log.push(format!("Run Program: started {program} ({} arguments)", args.len() - 1));
                    let tx = self.done_tx.clone();
                    let waiter = std::thread::Builder::new().name("run-program".into()).spawn(move || {
                        let line = match child.wait() {
                            Ok(s) if s.success() => format!("Run Program: {program} ended"),
                            Ok(s) => format!("Run Program: {program} ended with {s}"),
                            Err(e) => format!("Run Program: {program}: {e}"),
                        };
                        let _ = tx.send(line);
                    });
                    if waiter.is_err() {
                        // nobody waits for it: it runs on, uncounted
                        self.running -= 1;
                    }
                }
                Err(e) => log.push(format!("Run Program: cannot start {program}: {e}")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{InfoHashes, Sha1Hash};

    fn split(s: &str) -> Vec<String> {
        split_command(s).unwrap()
    }

    #[test]
    fn a_command_line_splits_at_blanks_outside_quotes() {
        assert!(split("").is_empty());
        assert!(split("   ").is_empty());
        assert_eq!(split("prog"), ["prog"]);
        assert_eq!(
            split(r#"  "C:\Program Files\x\run.exe"   "%F"  -n %N "#),
            [r"C:\Program Files\x\run.exe", "%F", "-n", "%N"]
        );
        // an empty argument, a quote inside quotes, quotes inside a word
        assert_eq!(split(r#"p "" "a ""b"" c" x"y z"w"#), ["p", "", r#"a "b" c"#, "xy zw"]);
        // nothing a shell would act on is special
        assert_eq!(
            split("p a;b | c > d && e $HOME `x` 'q r' *"),
            ["p", "a;b", "|", "c", ">", "d", "&&", "e", "$HOME", "`x`", "'q", "r'", "*"]
        );
        assert_eq!(split_command(r#"p "open"#), Err(CommandError::OpenQuote));
        assert_eq!(split_command("p a\nb"), Err(CommandError::Malformed));
        assert_eq!(split_command(&"a".repeat(MAX_COMMAND + 1)), Err(CommandError::Malformed));
        // the program is never a placeholder
        assert_eq!(split_command("%F"), Err(CommandError::ProgramPlaceholder));
        assert_eq!(split_command(r#""C:\tools\%N.exe" x"#), Err(CommandError::ProgramPlaceholder));
        assert_eq!(split("100%done %x"), ["100%done", "%x"]);
    }

    /// A shell told to run a string would read a torrent's name as commands (`a&calc`); the
    /// script named itself gets the name as an argument.
    #[test]
    fn a_shell_that_runs_a_string_gets_no_codes() {
        let shell = Err(CommandError::ShellPlaceholder);
        for line in [
            r#"cmd /c "C:\x\log.bat" %N"#,
            r#"C:\Windows\System32\CMD.EXE /c echo "%F""#,
            "cmd.exe /k type %D",
            r#"powershell -Command "Write-Host '%N'""#,
            r#"powershell.exe -NoProfile "& C:\x\a.ps1 %N""#,
            r#"pwsh -c "echo %N""#,
            r#"sh -c "echo %N""#,
            r#"/bin/bash -lc "notify-send %N""#,
            r#"/usr/bin/zsh --command "echo %I""#,
            r#"fish -c "echo %Z""#,
            // an option with a value before -c
            r#"bash -o pipefail -c "notify-send %N""#,
            r#"/bin/sh -eo errexit -c "echo %N""#,
            r#"bash -O extglob --rcfile /x/rc -c "echo %N""#,
            r#"fish --command="echo %N" %N"#,
            r#"fish -C "echo %N""#,
            // -File behind -Command belongs to the command
            r#"powershell -Command C:\x\a.ps1 -File %N"#,
            r#"pwsh -c echo -f %N"#,
        ] {
            assert_eq!(split_command(line), shell, "{line}");
        }
        for line in [
            r#""C:\x\log.bat" %N "%F""#,
            r#"powershell -NoProfile -ExecutionPolicy Bypass -File C:\x\log.ps1 "%N""#,
            r#"pwsh -f /x/log.ps1 "%N""#,
            r#"powershell.exe /File C:\x\log.ps1 %N"#,
            "/bin/sh /x/args.sh added %N",
            "bash -e /x/args.sh %N -c",
            "bash --norc /x/args.sh %N",
            "bash -o pipefail /x/args.sh %N -c",
            "bash --rcfile /x/rc /x/args.sh %N",
            // no code, or only a percent sign: nothing of a torrent's reaches the shell
            "cmd /c echo done",
            "cmd /c echo 100%%",
            r#"sh -c "echo finished""#,
            // not a shell
            "/usr/bin/notify-send %N",
            "cmdline-tool %N",
        ] {
            assert!(split_command(line).is_ok(), "{line}");
        }
    }

    fn values() -> Values {
        Values {
            name: "A \"name\" & more; $(rm -rf ~) | x > y %F\n--delete".into(),
            label: "lab el".into(),
            content_path: "/dl/A".into(),
            save_path: "/dl".into(),
            size: 1234,
            tracker: "udp://t/a".into(),
            hash_v1: "ab".repeat(20),
            hash_v2: "-".into(),
            id: "ab".repeat(20),
        }
    }

    #[test]
    fn a_hostile_name_stays_one_argument() {
        let v = values();
        let args = arguments(&split(r#"/usr/bin/notify --name=%N %N "%L in %D" %Z %I %J %K 100%% %Q %"#), &v);
        assert_eq!(args.len(), 11);
        assert_eq!(args[0], "/usr/bin/notify");
        // the whole name, without its line break, in one argument; its own %F is not expanded
        let name = "A \"name\" & more; $(rm -rf ~) | x > y %F--delete";
        assert_eq!(args[1], format!("--name={name}"));
        assert_eq!(args[2], name);
        assert_eq!(args[3], "lab el in /dl");
        assert_eq!(&args[4..], ["1234", &"ab".repeat(20), "-", &"ab".repeat(20), "100%", "%Q", "%"]);
    }

    #[test]
    fn the_values_of_a_status() {
        let st = TorrentStatus {
            name: "Some Torrent".into(),
            save_path: r"D:\Downloads\".into(),
            total: 42,
            current_tracker: "http://t/announce".into(),
            info_hashes: InfoHashes::v1(Sha1Hash([0xab; 20])),
            ..TorrentStatus::default()
        };
        let v = Values::of(&st, "films");
        assert_eq!(v.content_path, r"D:\Downloads\Some Torrent");
        assert_eq!((v.hash_v1.as_str(), v.hash_v2.as_str()), ("ab".repeat(20).as_str(), "-"));
        assert_eq!(v.id, "ab".repeat(20));
        assert_eq!((v.label.as_str(), v.size), ("films", 42));
        let st = TorrentStatus { save_path: "/srv/dl".into(), name: "x".into(), ..st };
        assert_eq!(Values::of(&st, "").content_path, "/srv/dl/x");
    }

    fn no_spawn(_: &[String]) -> std::io::Result<std::process::Child> {
        Err(std::io::Error::other("not started in a test"))
    }

    fn status(id: TorrentId, added_secs_ago: i64, downloaded: u64) -> TorrentStatus {
        TorrentStatus {
            id,
            name: format!("t{}", id.index()),
            save_path: "/dl".into(),
            added_time: crate::util::time::unix_now() - added_secs_ago,
            all_time_download: downloaded,
            ..TorrentStatus::default()
        }
    }

    fn feed(r: &mut RunProgram, a: Alert) -> Vec<String> {
        r.on_alert(&a, |_| Some("lbl".into()))
    }

    fn added(id: TorrentId) -> Alert {
        Alert::TorrentAdded { id, info_hashes: InfoHashes::default(), name: String::new() }
    }

    #[test]
    fn programs_run_for_new_and_really_finished_torrents_only() {
        let mut r = RunProgram::new("on-added %N %L", "on-finished %F");
        r.spawner = no_spawn;
        assert!(r.enabled());
        let (a, b, c) = (TorrentId::new(1, 0), TorrentId::new(2, 0), TorrentId::new(3, 0));
        let started = |log: &[String]| -> Vec<String> {
            log.iter().filter(|l| l.contains("cannot start")).cloned().collect()
        };

        // a new torrent: once, when its status is known
        assert!(feed(&mut r, added(a)).is_empty());
        let log = feed(&mut r, Alert::StateUpdate(vec![status(a, 1, 0)]));
        assert_eq!(started(&log).len(), 1, "{log:?}");
        assert!(log[0].contains("on-added"));
        // added again (a duplicate): nothing
        feed(&mut r, added(a));
        assert!(feed(&mut r, Alert::StateUpdate(vec![status(a, 2, 0)])).is_empty());
        // a duplicate of a torrent that has been there for an hour: nothing
        feed(&mut r, added(b));
        assert!(feed(&mut r, Alert::StateUpdate(vec![status(b, 3600, 0)])).is_empty());

        // finished after downloading: once
        feed(&mut r, Alert::TorrentFinished { id: b });
        let log = feed(&mut r, Alert::StateUpdate(vec![status(b, 3600, 10)]));
        assert_eq!(started(&log).len(), 1, "{log:?}");
        assert!(log[0].contains("on-finished"));
        // "finished" with nothing downloaded (the data was there): nothing
        feed(&mut r, Alert::TorrentFinished { id: c });
        assert!(feed(&mut r, Alert::StateUpdate(vec![status(c, 5, 0)])).is_empty());
        // removed before its status came: nothing
        feed(&mut r, Alert::TorrentFinished { id: c });
        feed(&mut r, Alert::TorrentRemoved { id: c, info_hashes: InfoHashes::default() });
        assert!(feed(&mut r, Alert::StateUpdate(vec![status(c, 5, 10)])).is_empty());

        // added and finished within one status update (a small torrent, or one whose data
        // was there): both programs, the added one first; with nothing downloaded only that
        let d = TorrentId::new(4, 0);
        feed(&mut r, added(d));
        feed(&mut r, Alert::TorrentFinished { id: d });
        let log = started(&feed(&mut r, Alert::StateUpdate(vec![status(d, 1, 10)])));
        assert_eq!(log.len(), 2, "{log:?}");
        assert!(log[0].contains("on-added") && log[1].contains("on-finished"), "{log:?}");
        let e = TorrentId::new(5, 0);
        feed(&mut r, added(e));
        feed(&mut r, Alert::TorrentFinished { id: e });
        let log = started(&feed(&mut r, Alert::StateUpdate(vec![status(e, 1, 0)])));
        assert_eq!(log.len(), 1, "{log:?}");
        assert!(log[0].contains("on-added"), "{log:?}");

        // switched off: nothing waits
        r.set("", "");
        assert!(!r.enabled());
        feed(&mut r, Alert::TorrentFinished { id: b });
        assert!(feed(&mut r, Alert::StateUpdate(vec![status(b, 3600, 10)])).is_empty());
        // a line that cannot be split is off
        assert!(!RunProgram::new("\"open", "%N").enabled());
    }

    #[test]
    fn a_flood_of_finished_torrents_is_bounded() {
        let mut r = RunProgram::new("", "p %N");
        // every start "succeeds" without a process: count what would run at once
        r.running = MAX_RUNNING;
        let mut dropped = 0;
        for i in 0..(MAX_QUEUED as u32 + 10) {
            let id = TorrentId::new(i, 0);
            feed(&mut r, Alert::TorrentFinished { id });
            let log = feed(&mut r, Alert::StateUpdate(vec![status(id, 9999, 1)]));
            dropped += log.iter().filter(|l| l.contains("too many")).count();
        }
        assert_eq!(r.queue.len(), MAX_QUEUED);
        assert_eq!(dropped, 10);
        assert_eq!(r.running, MAX_RUNNING);
    }

    /// A real process gets the arguments exactly: the test binary runs itself as the child.
    #[test]
    fn a_real_program_gets_each_argument_whole() {
        const MARK: &str = "RP_RUN_PROGRAM_CHILD";
        if let Ok(out) = std::env::var(MARK) {
            // the child: write the arguments after `--`, one per line, and leave
            let args: Vec<String> = std::env::args().skip_while(|a| a != "--").skip(1).collect();
            std::fs::write(out, args.join("\n")).unwrap();
            return;
        }
        let out = std::env::temp_dir().join(format!("rp-run-program-{}.txt", std::process::id()));
        let _ = std::fs::remove_file(&out);
        let exe = std::env::current_exe().unwrap();
        let line = format!(
            r#""{}" run_program::tests::a_real_program_gets_each_argument_whole --exact --nocapture -- %N "%L / %F" end"#,
            exe.display()
        );
        let mut v = values();
        v.name = "it's a \"name\" & more; $(x) | y > z %TEMP% ^ !".into();
        let args = arguments(&split_command(&line).unwrap(), &v);
        // SAFETY: the variable is read by the child only; tests of this module set no other
        unsafe { std::env::set_var(MARK, &out) };
        let status = spawn(&args).unwrap().wait().unwrap();
        unsafe { std::env::remove_var(MARK) };
        assert!(status.success());
        let got = std::fs::read_to_string(&out).unwrap();
        assert_eq!(got.lines().collect::<Vec<_>>(), [v.name.as_str(), "lab el / /dl/A", "end"]);
        let _ = std::fs::remove_file(&out);
    }

    /// A `.bat` or `.cmd` program runs through cmd.exe, whose rules are not the ones of other
    /// programs: the standard library escapes its arguments for cmd (Rust 1.77.2's fix of
    /// CVE-2024-24576). Pinned here, so a toolchain that loses it fails a test: a name with `&`,
    /// `|`, `>`, `^`, `%` or `"` reaches the batch file as one argument and runs nothing.
    #[cfg(windows)]
    #[test]
    fn a_batch_file_gets_each_argument_whole_and_runs_nothing_else() {
        let dir = std::env::temp_dir().join(format!("rp-run-program-bat-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let log = dir.join("args.txt");
        let ran = dir.join("ran.txt");
        // a careful batch file quotes its arguments: "%~1"
        let script = "@echo off\r\n>>\"%~dp0args.txt\" echo \"%~1\"\r\n>>\"%~dp0args.txt\" echo \"%~2\"\r\n";
        for program in ["log args.bat", "LOG.CMD"] {
            std::fs::write(dir.join(program), script).unwrap();
        }
        let r = ran.display();
        let names = [
            "Rook's test & more (1) %TEMP% ;x --delete.bin".to_owned(),
            format!("a&echo.X>{r}"),
            format!("a\"&echo.X>{r}&\"b"),
            format!("^\"&echo.X>{r}"),
            format!("x|echo.X>{r}"),
            "%PATH% %cd:~,% !TEMP! (a) <b> %%".to_owned(),
        ];
        for program in ["log args.bat", "LOG.CMD"] {
            for name in &names {
                let _ = std::fs::remove_file(&log);
                let line = format!(r#""{}" %N "%L / %F""#, dir.join(program).display());
                let v = Values { name: name.clone(), ..values() };
                let args = arguments(&split_command(&line).unwrap(), &v);
                let status = spawn(&args).unwrap().wait().unwrap();
                assert!(status.success(), "{program} {name}");
                assert!(!ran.exists(), "{program}: {name} ran a second command");
                // inside the argument cmd.exe sees a quote doubled, as the escaping writes it
                let want =
                    [name.replace('"', "\"\""), "lab el / /dl/A".to_owned()].map(|s| format!("\"{s}\""));
                let got = std::fs::read_to_string(&log).unwrap();
                assert_eq!(got.lines().collect::<Vec<_>>(), want, "{program}");
            }
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
