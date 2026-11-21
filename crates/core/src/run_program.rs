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
//! - **No shell or interpreter told to run a string with a placeholder in it** (`cmd /c ...
//!   %N`, `powershell -Command`, `sh -c`, `python -c`, `ssh host ...`, also behind `env`,
//!   `nohup`, `sudo`, `wsl` and other wrappers): it would re-read the name as commands, so the
//!   line is refused ([`shell_reads_codes`]); the script is named itself instead.
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
    /// The program (the first argument, or the program a wrapper such as `env` or `nohup`
    /// starts) holds a placeholder.
    ProgramPlaceholder,
    /// The program is a command shell or interpreter told to run a string as commands
    /// (`cmd /c`, `powershell -Command`, `sh -c`, `python -c`, `ssh host ...`, also behind a
    /// wrapper) and a placeholder is in its arguments: it would read a torrent's name as
    /// commands ([`shell_reads_codes`]).
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
    check_program(&args)?;
    Ok(args)
}

/// Whether `args` hand a filled-in code to something that would read it as commands: a shell
/// or interpreter told to run a string (`cmd /c`, `powershell -Command`, `sh -c`, `python -c`,
/// `ssh host ...`, also behind wrappers such as `env`, `nohup` or `sudo`), or a wrapper whose
/// program holds a code ([`split_command`]'s `ShellPlaceholder` and `ProgramPlaceholder`).
pub fn shell_reads_codes(args: &[String]) -> bool {
    check_program(args).is_err()
}

/// Refuses a line whose codes something would read as commands. Every argument reaches the
/// program whole, but a shell or interpreter told to run a string re-reads it, so a name such
/// as `a&calc` or `x$(calc)` would start a second program:
///
/// - `cmd` with a code anywhere (`cmd /c log.bat %N`: cmd parses the whole line again);
/// - PowerShell (`powershell`, `pwsh`) with a code and no `-File` before it (Windows
///   PowerShell's default is `-Command`);
/// - a Unix shell (`sh`, `bash`, `dash`, `zsh`, `fish`, `nu`, ...) with a code and `-c` (or
///   `--command`) among its options;
/// - an interpreter told to run a string (`python -c`, `perl -e`, `ruby -e`, `node -e`,
///   `php -r`, `lua -e`, `osascript -e`, `Rscript -e`, `julia -e`, `deno eval`) with a code;
///   `awk` and `sed` with a code in their program text, and `find -exec` by its command;
/// - programs that hand their whole line to a shell (`ssh`, `wsl` without `-e`/`--exec`,
///   `tmux`, `parallel`, `runas`, `mshta`, `wt`, `watch` without `-x`, `su -c`, `flock -c`,
///   `env -S`, `sudo -s`/`-i`) with a code anywhere;
/// - wrappers that start another program (`env`, `nohup`, `nice`, `timeout`, `sudo`,
///   `busybox`, `xargs`, `wsl -e`, ...) are looked through, and their program may hold no code
///   (`env %N`: [`CommandError::ProgramPlaceholder`]);
/// - any other program with a shell or interpreter further on in its line told to run a string
///   (`gnome-terminal -- bash -lc ...`), unless the program is a script (`.sh`, `.bat`, `.cmd`,
///   `.ps1`, `.py`, ...), whose arguments are its own. Its own `-c` or `-e` is its own business
///   (`notify-send -e %N`, `grep -e %N`, `rsync -c %F`).
///
/// Naming the script itself (`"C:\x\done.bat" "%N"`, `powershell -File x.ps1 "%N"`,
/// `sh x.sh "%N"`, `python x.py "%N"`) passes the name as an argument and is fine. `%%` is no
/// code here. This is a guard against the usual ways, not a proof: a program that evaluates an
/// argument in a way of its own is still the user's choice.
fn check_program(args: &[String]) -> Result<(), CommandError> {
    let Some((program, rest)) = args.split_first() else {
        return Ok(());
    };
    if !rest.iter().any(|a| has_code(a)) {
        return Ok(());
    }
    let shell = Err(CommandError::ShellPlaceholder);
    let name = program_name(program);
    if let Some(inner) = wrapped(&name, rest) {
        return match inner {
            Inner::Line => shell,
            Inner::Nothing => Ok(()),
            Inner::At(k) => {
                if has_placeholder(&rest[k]) {
                    return Err(CommandError::ProgramPlaceholder);
                }
                // the value of an option of the wrapper's that is not modelled may have taken
                // the program's place: a known program further on is the one that runs (a path
                // is a program, though)
                let slot = &rest[k];
                let k = if is_known(&program_name(slot)) || slot.contains(['/', '\\']) || is_script(slot) {
                    k
                } else {
                    rest[k..].iter().position(|a| is_known(&program_name(a))).map_or(k, |p| k + p)
                };
                check_program(&rest[k..])
            }
        };
    }
    if shell_runs_codes(&name, rest) || interpreter_runs_codes(&name, rest) {
        return shell;
    }
    match name.as_str() {
        "awk" | "gawk" | "mawk" | "nawk" if awk_program_has_code(rest) => shell,
        "sed" | "gsed" if sed_script_has_code(rest) => shell,
        "find" | "gfind" => find_commands(rest),
        _ if RE_PARSE_LINE.contains(&name.as_str()) => shell,
        _ if is_known(&name) || is_script(program) => Ok(()),
        _ if nested_shell_runs_codes(rest) => shell,
        _ => Ok(()),
    }
}

/// A Unix shell or an interpreter named further on in the line of a program no rule here knows
/// (`gnome-terminal -- bash -lc "..."`, `konsole -e python3 -c "..."`), told to run a string,
/// with a code after it.
fn nested_shell_runs_codes(rest: &[String]) -> bool {
    (0..rest.len()).any(|j| {
        let (name, tail) = (program_name(&rest[j]), &rest[j + 1..]);
        (SHELLS.contains(&name.as_str()) && shell_runs_codes(&name, tail))
            || (tail.iter().any(|a| has_code(a)) && interpreter_runs_codes(&name, tail))
    })
}

/// A program's name as the rules know it: the file name, lowercase, without `.exe`.
fn program_name(program: &str) -> String {
    let name = program.rsplit(['/', '\\']).next().unwrap_or(program);
    let name = name.trim_end_matches(['.', ' ']).to_ascii_lowercase();
    name.strip_suffix(".exe").map(str::to_owned).unwrap_or(name)
}

/// Unix shells: `-c` (in any option cluster) or `--command` runs a string.
const SHELLS: &[&str] = &[
    "sh", "bash", "rbash", "dash", "zsh", "ksh", "ksh93", "mksh", "lksh", "oksh", "loksh", "pdksh", "posh",
    "yash", "ash", "hush", "fish", "csh", "tcsh", "nu", "xonsh", "elvish", "osh", "ysh", "rc", "es", "ion",
];

/// Programs that hand the rest of their line to a shell, whatever their options.
const RE_PARSE_LINE: &[&str] = &[
    "ssh", "slogin", "rsh", "rlogin", "plink", "mosh", "dbclient", "autossh", "mshta", "wt", "runas",
    "parallel", "tmux",
];

/// Programs some rule here knows, so that none of the guesses for unknown ones applies.
fn is_known(name: &str) -> bool {
    matches!(
        name,
        "cmd" | "powershell" | "pwsh" | "awk" | "gawk" | "mawk" | "nawk" | "sed" | "gsed" | "find" | "gfind"
    ) || SHELLS.contains(&name)
        || RE_PARSE_LINE.contains(&name)
        || interpreter(name).is_some()
        || wrapped(name, &[]).is_some()
}

/// A script named as the program: its options are its own.
fn is_script(program: &str) -> bool {
    let file = program.rsplit(['/', '\\']).next().unwrap_or(program).trim_end_matches(['.', ' ']);
    let Some((_, ext)) = file.rsplit_once('.') else { return false };
    let ext = ext.to_ascii_lowercase();
    [
        "sh", "bash", "zsh", "ksh", "fish", "command", "bat", "cmd", "ps1", "py", "pyw", "pl", "rb", "js",
        "mjs", "cjs", "php", "lua", "tcl", "vbs", "wsf", "jl", "r",
    ]
    .contains(&ext.as_str())
}

/// Where a wrapper's program is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Inner {
    /// At this index of the wrapper's arguments.
    At(usize),
    /// The wrapper hands its line to a shell: no code may be in it.
    Line,
    /// It starts no program (`sudo -l`, `wsl --list`).
    Nothing,
}

/// A wrapper's options: which take a value, and which make it run a string through a shell.
#[derive(Default)]
struct Wrapper {
    /// Short options that take a value (glued, or the next argument).
    value: &'static str,
    value_long: &'static [&'static str],
    /// Options after which the line goes through a shell.
    line: &'static str,
    line_long: &'static [&'static str],
    /// `line` counts anywhere in the line, not only before the program (`su user -c ...`).
    line_anywhere: bool,
    /// Arguments before the program that are not options (`timeout`'s duration).
    positional: usize,
    /// `NAME=value` arguments before the program (`env`).
    assignments: bool,
    /// An option without which the program runs through a shell (`watch -x`).
    exec: &'static str,
    exec_long: &'static [&'static str],
}

/// The wrapper `name` with arguments `rest`, if `name` is one ([`Inner`]).
fn wrapped(name: &str, rest: &[String]) -> Option<Inner> {
    let w = match name {
        "env" => Wrapper {
            value: "uC",
            value_long: &["unset", "chdir"],
            line: "S",
            line_long: &["split-string"],
            assignments: true,
            ..Wrapper::default()
        },
        "nohup" | "setsid" | "busybox" | "toybox" | "firejail" | "unbuffer" => Wrapper::default(),
        "nice" => Wrapper { value: "n", value_long: &["adjustment"], ..Wrapper::default() },
        "ionice" => Wrapper { value: "cnpPu", value_long: &["class", "classdata"], ..Wrapper::default() },
        "timeout" => Wrapper {
            value: "ks",
            value_long: &["kill-after", "signal"],
            positional: 1,
            ..Wrapper::default()
        },
        "stdbuf" => Wrapper { value: "ioe", value_long: &["input", "output", "error"], ..Wrapper::default() },
        "chrt" => Wrapper { value: "TPD", positional: 1, ..Wrapper::default() },
        "taskset" => Wrapper { positional: 1, ..Wrapper::default() },
        "time" => Wrapper { value: "fo", value_long: &["format", "output"], ..Wrapper::default() },
        "caffeinate" => Wrapper { value: "wt", ..Wrapper::default() },
        "torsocks" => Wrapper { value: "uapP", line_long: &["shell"], ..Wrapper::default() },
        "proxychains" | "proxychains4" => Wrapper { value: "f", ..Wrapper::default() },
        // `-X` sends screen commands (`stuff` types into a shell)
        "screen" => Wrapper { value: "cehpSsTt", line: "X", ..Wrapper::default() },
        "sshpass" => Wrapper { value: "pfdP", ..Wrapper::default() },
        "xargs" => Wrapper {
            value: "aEdILnPs",
            value_long: &["arg-file", "delimiter", "max-args", "max-procs", "max-chars", "process-slot-var"],
            ..Wrapper::default()
        },
        "sudo" => Wrapper {
            value: "ugpCDrtThUac",
            value_long: &[
                "user",
                "group",
                "host",
                "prompt",
                "close-from",
                "chdir",
                "role",
                "type",
                "command-timeout",
                "other-user",
                "login-class",
                "auth-type",
            ],
            line: "si",
            line_long: &["shell", "login"],
            ..Wrapper::default()
        },
        "doas" => Wrapper { value: "uC", line: "s", ..Wrapper::default() },
        "su" | "runuser" => Wrapper {
            value: "gGsuw",
            value_long: &["group", "supp-group", "shell", "user", "whitelist-environment"],
            line: "c",
            line_long: &["command", "session-command"],
            line_anywhere: true,
            ..Wrapper::default()
        },
        "flock" => Wrapper {
            value: "wE",
            value_long: &["timeout", "conflict-exit-code"],
            line: "c",
            line_long: &["command"],
            line_anywhere: true,
            positional: 1,
            ..Wrapper::default()
        },
        "watch" => Wrapper {
            value: "nq",
            value_long: &["interval", "equexit"],
            exec: "x",
            exec_long: &["exec"],
            ..Wrapper::default()
        },
        "wsl" => return Some(wsl_program(rest)),
        _ => return None,
    };
    Some(wrapper_program(&w, rest))
}

fn wrapper_program(w: &Wrapper, rest: &[String]) -> Inner {
    // (is a line option, is an exec option) of one option argument
    let kinds = |a: &str| -> (bool, bool) {
        if let Some(long) = a.strip_prefix("--") {
            let long = long.split_once('=').map_or(long, |(n, _)| n);
            return (w.line_long.contains(&long), w.exec_long.contains(&long));
        }
        let mut line = false;
        let mut exec = false;
        for c in a[1..].chars() {
            line |= w.line.contains(c);
            exec |= w.exec.contains(c);
            if w.value.contains(c) {
                break;
            }
        }
        (line, exec)
    };
    let is_option = |a: &str| a.len() >= 2 && a.starts_with('-') && a != "--";
    if w.line_anywhere
        && rest.iter().take_while(|a| *a != "--").any(|a| is_option(a.as_str()) && kinds(a.as_str()).0)
    {
        return Inner::Line;
    }
    let mut positional = w.positional;
    let mut exec = false;
    let mut i = 0;
    while i < rest.len() {
        let a = rest[i].as_str();
        if a == "--" {
            i += 1;
            break;
        }
        if is_option(a) {
            let (line, x) = kinds(a);
            if line {
                return Inner::Line;
            }
            exec |= x;
            if let Some(long) = a.strip_prefix("--") {
                if !long.contains('=') && w.value_long.contains(&long) {
                    i += 1;
                }
            } else if let Some(pos) = a[1..].find(|c| w.value.contains(c)) {
                // the value is the rest of the argument, or the next one
                if pos + 2 == a.len() {
                    i += 1;
                }
            }
            i += 1;
            continue;
        }
        if a == "-" || (w.assignments && a.find('=').is_some_and(|e| e > 0 && !a[..e].contains(['/', '\\'])))
        {
            i += 1;
            continue;
        }
        if positional > 0 {
            positional -= 1;
            i += 1;
            continue;
        }
        break;
    }
    let i = i + positional;
    if !w.exec.is_empty() && !exec {
        return Inner::Line;
    }
    if i < rest.len() { Inner::At(i) } else { Inner::Nothing }
}

/// `wsl`: the line goes to the distribution's shell, unless `-e` / `--exec` (or
/// `--shell-type none`) names the program.
fn wsl_program(rest: &[String]) -> Inner {
    let mut no_shell = false;
    let mut i = 0;
    while i < rest.len() {
        match rest[i].to_ascii_lowercase().as_str() {
            "-e" | "--exec" => return if i + 1 < rest.len() { Inner::At(i + 1) } else { Inner::Nothing },
            "--" => return Inner::Line,
            "--shell-type" => {
                no_shell = rest.get(i + 1).is_some_and(|t| t.eq_ignore_ascii_case("none"));
                i += 2;
            }
            "-d" | "--distribution" | "-u" | "--user" | "--cd" | "--distribution-id" => i += 2,
            // management options (`--list`, `--install`, ...) run no command line
            a if a.starts_with('-') => i += 1,
            _ => return if no_shell { Inner::At(i) } else { Inner::Line },
        }
    }
    Inner::Nothing
}

/// How an interpreter's options read.
#[derive(Default)]
struct Interpreter {
    /// Short options that run their value, and long ones.
    code: &'static str,
    code_long: &'static [&'static str],
    /// Short options whose value is glued or the next argument.
    value: &'static str,
    /// Short options whose value can only be glued (`-Mstrict`, `-i.bak`).
    glued: &'static str,
    /// Short options with optional glued digits (`-l0`, `-0777`).
    digits: &'static str,
    /// Short options after which the rest are the program's arguments (`python -m mod`).
    end: &'static str,
    value_long: &'static [&'static str],
    /// A first operand that runs the next one (`deno eval "..."`).
    eval_words: &'static [&'static str],
}

/// `name` with a version after it: `python3.12`, `lua5.4`, `php8.2`.
fn versioned(name: &str, base: &str) -> bool {
    name.strip_prefix(base).is_some_and(|v| v.bytes().all(|c| c.is_ascii_digit() || c == b'.'))
}

fn interpreter(name: &str) -> Option<Interpreter> {
    let i = if versioned(name, "python")
        || versioned(name, "pypy")
        || matches!(name, "py" | "pyw" | "pythonw")
    {
        Interpreter { code: "c", value: "WX", end: "m", ..Interpreter::default() }
    } else if versioned(name, "perl") {
        Interpreter { code: "eE", glued: "MmIidDxCF", digits: "l0", ..Interpreter::default() }
    } else if versioned(name, "ruby") || name == "jruby" {
        Interpreter { code: "e", value: "IrCE", glued: "xFK", digits: "0TW", ..Interpreter::default() }
    } else if matches!(name, "node" | "nodejs" | "bun") {
        Interpreter {
            code: "ep",
            code_long: &["eval", "print"],
            value: "rC",
            value_long: &[
                "require",
                "import",
                "loader",
                "experimental-loader",
                "input-type",
                "title",
                "conditions",
            ],
            ..Interpreter::default()
        }
    } else if name == "deno" {
        Interpreter { eval_words: &["eval"], ..Interpreter::default() }
    } else if versioned(name, "php") || name == "php-cgi" {
        Interpreter { code: "rBRE", value: "cdfFzSt", ..Interpreter::default() }
    } else if versioned(name, "lua") || name == "luajit" {
        Interpreter { code: "e", value: "l", ..Interpreter::default() }
    } else if name == "osascript" {
        Interpreter { code: "e", value: "ls", ..Interpreter::default() }
    } else if versioned(name, "julia") {
        Interpreter { code: "eE", code_long: &["eval", "print"], value: "LJptCH", ..Interpreter::default() }
    } else if matches!(name, "rscript" | "r") {
        Interpreter { code: "e", ..Interpreter::default() }
    } else {
        return None;
    };
    Some(i)
}

/// Whether interpreter `name` is told by `rest` to run a string (the caller knows a code is in
/// `rest`, so it is refused wherever it is, as for a shell's `-c`).
fn interpreter_runs_code_options(s: &Interpreter, rest: &[String]) -> bool {
    let mut i = 0;
    while i < rest.len() {
        let a = rest[i].as_str();
        if a == "--" {
            i += 1;
            break;
        }
        if let Some(long) = a.strip_prefix("--") {
            let (long, glued) = long.split_once('=').map_or((long, false), |(n, _)| (n, true));
            if s.code_long.contains(&long) {
                return true;
            }
            if !glued && s.value_long.contains(&long) {
                i += 1;
            }
            i += 1;
            continue;
        }
        if a.len() < 2 || !a.starts_with('-') {
            break;
        }
        let letters = &a.as_bytes()[1..];
        let mut k = 0;
        while k < letters.len() {
            let c = char::from(letters[k]);
            if s.code.contains(c) {
                return true;
            }
            if s.end.contains(c) {
                return false;
            }
            if s.value.contains(c) {
                if k + 1 == letters.len() {
                    i += 1;
                }
                break;
            }
            if s.glued.contains(c) {
                break;
            }
            k += 1;
            if s.digits.contains(c) {
                while letters.get(k).is_some_and(u8::is_ascii_digit) {
                    k += 1;
                }
            }
        }
        i += 1;
    }
    rest.get(i).is_some_and(|op| s.eval_words.contains(&op.to_ascii_lowercase().as_str()))
}

fn interpreter_runs_codes(name: &str, rest: &[String]) -> bool {
    interpreter(name).is_some_and(|s| interpreter_runs_code_options(&s, rest))
}

/// `awk`: the program text is the first operand, unless `-f` names a file; gawk's `-e` /
/// `--source` add text too.
fn awk_program_has_code(rest: &[String]) -> bool {
    let mut from_file = false;
    let mut i = 0;
    while i < rest.len() {
        let a = rest[i].as_str();
        if a == "--" {
            i += 1;
            break;
        }
        if let Some(long) = a.strip_prefix("--") {
            let (long, value) = long.split_once('=').map_or((long, None), |(n, v)| (n, Some(v)));
            let takes = matches!(
                long,
                "file" | "exec" | "source" | "assign" | "field-separator" | "include" | "load"
            );
            let value = match value {
                Some(v) => Some(v),
                None if takes => {
                    i += 1;
                    rest.get(i).map(String::as_str)
                }
                None => None,
            };
            match long {
                "file" | "exec" => from_file = true,
                "source" if value.is_some_and(has_code) => return true,
                _ => {}
            }
            i += 1;
            continue;
        }
        if a.len() < 2 || !a.starts_with('-') {
            break;
        }
        let c = a.as_bytes()[1];
        if matches!(c, b'f' | b'E' | b'e' | b'F' | b'v' | b'i' | b'l') {
            let value = if a.len() > 2 {
                Some(&a[2..])
            } else {
                i += 1;
                rest.get(i).map(String::as_str)
            };
            match c {
                b'f' | b'E' => from_file = true,
                b'e' if value.is_some_and(has_code) => return true,
                _ => {}
            }
        }
        i += 1;
    }
    !from_file && rest.get(i).is_some_and(|p| has_code(p))
}

/// `sed`: the script is `-e`'s values, or the first operand without `-e` and `-f` (GNU sed's
/// `e` command and `s///e` run a shell).
fn sed_script_has_code(rest: &[String]) -> bool {
    let mut scripts: Vec<&str> = Vec::new();
    let mut from_file = false;
    let mut i = 0;
    while i < rest.len() {
        let a = rest[i].as_str();
        if a == "--" {
            i += 1;
            break;
        }
        if let Some(long) = a.strip_prefix("--") {
            let (long, value) = long.split_once('=').map_or((long, None), |(n, v)| (n, Some(v)));
            let takes = matches!(long, "expression" | "file" | "line-length");
            let value = match value {
                Some(v) => Some(v),
                None if takes => {
                    i += 1;
                    rest.get(i).map(String::as_str)
                }
                None => None,
            };
            match long {
                "expression" => scripts.extend(value),
                "file" => from_file = true,
                _ => {}
            }
            i += 1;
            continue;
        }
        if a.len() < 2 || !a.starts_with('-') {
            break;
        }
        for (k, c) in a[1..].char_indices() {
            if !matches!(c, 'e' | 'f' | 'l' | 'i') {
                continue;
            }
            let glued = &a[k + 2..];
            let value = if !glued.is_empty() || c == 'i' {
                Some(glued)
            } else {
                i += 1;
                rest.get(i).map(String::as_str)
            };
            match c {
                'e' => scripts.extend(value),
                'f' => from_file = true,
                _ => {}
            }
            break;
        }
        i += 1;
    }
    if scripts.is_empty() && !from_file {
        scripts.extend(rest.get(i).map(String::as_str));
    }
    scripts.into_iter().any(has_code)
}

/// `find`: each `-exec` / `-execdir` / `-ok` / `-okdir` command is a line of its own.
fn find_commands(rest: &[String]) -> Result<(), CommandError> {
    let mut i = 0;
    while i < rest.len() {
        if matches!(rest[i].as_str(), "-exec" | "-execdir" | "-ok" | "-okdir") {
            let start = i + 1;
            let end =
                rest[start..].iter().position(|a| a == ";" || a == "+").map_or(rest.len(), |p| start + p);
            let command = &rest[start..end];
            if command.first().is_some_and(|p| has_placeholder(p)) {
                return Err(CommandError::ProgramPlaceholder);
            }
            check_program(command)?;
            i = end;
        }
        i += 1;
    }
    Ok(())
}

/// Whether Unix shell, PowerShell or cmd `name` would run a code of `rest` as commands.
fn shell_runs_codes(name: &str, rest: &[String]) -> bool {
    let Some(first_code) = rest.iter().position(|a| has_code(a)) else {
        return false;
    };
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
        _ if SHELLS.contains(&name) => {
            // the options before the first operand (the script, or -c's string)
            let mut options = rest.iter();
            while let Some(a) = options.next() {
                if a.len() < 2 || !a.starts_with(['-', '+']) {
                    break;
                }
                if let Some(long) = a.strip_prefix("--") {
                    let long = long.split_once('=').map_or(long, |(name, _)| name);
                    match long {
                        // fish's `--init-command` runs a string as well, nu's `--commands`
                        "command" | "init-command" | "commands" => return true,
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
    /// The two lines as given, to tell a new line from the one in force.
    lines: [String; 2],
    /// Lines that cannot run, for the Logger at the next alert: once each time they are given.
    refused: Vec<(String, CommandError)>,
}

/// What a frontend says about a refused line in its language ([`set_error_text`]).
static ERROR_TEXT: std::sync::OnceLock<fn(CommandError) -> String> = std::sync::OnceLock::new();

/// The text the Logger shows for a stored line that cannot run (the Preferences' own error
/// texts, in the frontend's language). Set once, by the frontend; without it the line is
/// logged with the error's name.
pub fn set_error_text(text: fn(CommandError) -> String) {
    let _ = ERROR_TEXT.set(text);
}

impl RunProgram {
    /// The two command lines; an empty one is off. A line that cannot be split
    /// ([`split_command`]) is off too: the Preferences refuse it before it gets here, but one
    /// stored before a rule refused it (a settings file of an older version, or edited by hand)
    /// is reported in the Logger at the next alert ([`RunProgram::on_alert`]).
    pub fn new(on_added: &str, on_finished: &str) -> RunProgram {
        let (done_tx, done_rx) = mpsc::channel();
        let mut r = RunProgram {
            on_added: Vec::new(),
            on_finished: Vec::new(),
            waiting: HashMap::new(),
            recent: HashMap::new(),
            queue: VecDeque::new(),
            running: 0,
            done_tx,
            done_rx,
            spawner: spawn,
            lines: Default::default(),
            refused: Vec::new(),
        };
        r.take_lines(on_added, on_finished, true);
        r
    }

    /// New command lines; programs already queued still run. A changed line that cannot run is
    /// reported as at the start.
    pub fn set(&mut self, on_added: &str, on_finished: &str) {
        self.take_lines(on_added, on_finished, false);
    }

    fn take_lines(&mut self, on_added: &str, on_finished: &str, all: bool) {
        for (i, line) in [on_added, on_finished].into_iter().enumerate() {
            if !all && self.lines[i] == line {
                continue;
            }
            self.lines[i] = line.to_owned();
            let args = match split_command(line) {
                Ok(args) => args,
                Err(e) => {
                    self.refused.push((line.to_owned(), e));
                    Vec::new()
                }
            };
            *(if i == 0 { &mut self.on_added } else { &mut self.on_finished }) = args;
        }
    }

    /// The Logger's line for a refused line: the line (control characters out, cut short) and
    /// why.
    fn refused_line(line: &str, e: CommandError) -> String {
        let shown: String = line.trim().chars().filter(|c| !c.is_control()).take(200).collect();
        let why = ERROR_TEXT.get().map_or_else(|| format!("{e:?}"), |text| text(e));
        format!("Run Program: {shown}: {why}")
    }

    /// Whether either program is set.
    pub fn enabled(&self) -> bool {
        !self.on_added.is_empty() || !self.on_finished.is_empty()
    }

    /// Looks at one alert; returns lines for the Logger (a program started, ended or failed, a
    /// line given that cannot run).
    /// Status updates ([`Alert::StateUpdate`]) say what a torrent is, so the caller keeps
    /// posting them. `label_of` gives a torrent's label; it is asked only when a program runs.
    pub fn on_alert(&mut self, alert: &Alert, label_of: impl Fn(TorrentId) -> Option<String>) -> Vec<String> {
        let mut log: Vec<String> =
            self.refused.drain(..).map(|(line, e)| RunProgram::refused_line(&line, e)).collect();
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

    /// Bug hunt 2026-11-21 #22: wrappers, interpreters and remote shells re-read a name such
    /// as `x$(calc)` too; a wrapper's own program takes no code; the direct forms still run.
    #[test]
    fn wrappers_and_interpreters_that_run_a_string_get_no_codes() {
        let shell = Err(CommandError::ShellPlaceholder);
        for line in [
            // wrappers, looked through to the shell
            r#"env sh -c "echo %N""#,
            r#"/usr/bin/env -i PATH=/bin:/usr/bin sh -c "echo %N""#,
            r#"env -u HOME -C /tmp bash -lc "notify-send %N""#,
            r#"env -S "sh -c 'echo %N'""#,
            r#"env --split-string="/x/run %N""#,
            r#"busybox sh -c "echo %N""#,
            r#"nohup sh -c "echo %N""#,
            r#"nice -n 10 bash -c "echo %N""#,
            r#"timeout -s KILL 30 sh -c "echo %N""#,
            r#"sudo -u media bash -c "echo %N""#,
            r#"sudo -n -u media -- zsh -c "echo %N""#,
            "sudo -i /x/args.sh %N",
            r#"doas -u media sh -c "echo %N""#,
            r#"setsid -f nohup env sh -c "echo %N""#,
            r#"xargs -0 -I{} sh -c "echo %N""#,
            r#"ionice -c 3 nice sh -c "echo %N""#,
            r#"su media -c "echo %N""#,
            r#"su -c "echo %N" media"#,
            r#"flock /tmp/lock -c "echo %N""#,
            "watch notify-send %N",
            r#"watch -n 5 -x sh -c "echo %N""#,
            // WSL hands the line to the distribution's shell unless -e names the program
            "wsl notify-send %N",
            r#"wsl bash -c "echo %N""#,
            r#"C:\Windows\System32\wsl.exe -d Ubuntu -- ls %N"#,
            r#"wsl -e sh -c "echo %N""#,
            r#"wsl --exec bash -c "echo %N""#,
            // remote and multiplexing shells join their arguments into one command
            r#"ssh host "notify-send %N""#,
            "ssh -p 2222 -i /x/key host /x/args.sh %N",
            r#"sshpass -f /x/pw ssh host /x/args.sh %N"#,
            "plink.exe host notify %N",
            r#"tmux new-session -d "echo %N""#,
            "parallel echo ::: %N",
            r#"runas /user:x "C:\x\run.exe %N""#,
            r#"mshta "javascript:alert('%N')""#,
            "wt -d . cmd.exe /c echo %N",
            // interpreters told to run a string
            r#"python -c "import os; os.system('%N')""#,
            r#"python3 -c "print(1)" %N"#,
            r#"/usr/bin/python3.12 -Ic "print('%N')""#,
            r#"py -3 -c "print('%N')""#,
            r#"perl -e "system '%N'""#,
            r#"perl -Mstrict -lne "print" %N"#,
            r#"ruby -e "system('%N')""#,
            r#"node -e "require('child_process').exec('%N')""#,
            r#"node --eval="console.log('%N')""#,
            r#"bun --print "'%N'""#,
            r#"deno eval "console.log('%N')""#,
            r#"php -r "system('%N');""#,
            r#"lua5.4 -e "os.execute('%N')""#,
            r#"osascript -e "display notification ""%N""""#,
            r#"Rscript -e "system('%N')""#,
            r#"julia -e "run(`%N`)""#,
            r#"nu -c "echo %N""#,
            r#"xonsh -c "echo %N""#,
            // program text with a code in it
            r#"awk "BEGIN { system(""echo %N"") }""#,
            r#"gawk -v x=1 -e "BEGIN { system(""%N"") }""#,
            r#"sed -e "s/x/%N/e" /x/f"#,
            r#"sed -n "s/x/%N/p" /x/f"#,
            r#"sed --expression=s/x/%N/ /x/f"#,
            r#"find %D -name x -exec sh -c "echo %N" ;"#,
            r#"find %D -execdir bash -c "echo %N" +"#,
            // a shell or interpreter further on in an unknown program's line
            r#"screen -dm bash -lc "echo %N""#,
            r#"gnome-terminal -- bash -c "echo %N""#,
            r#"gnome-terminal -- bash -lc "echo %N""#,
            r#"xterm -e sh -c "echo %N""#,
            r#"konsole -e python3 -c "print('%N')""#,
            r#"alacritty -e perl -le "print '%N'""#,
        ] {
            assert_eq!(split_command(line), shell, "{line}");
        }
        // the program a wrapper starts is chosen by no torrent either
        for line in [
            "env %N",
            "nohup %F --now",
            "/usr/bin/env FOO=1 %D/run.sh",
            "sudo -u x %F",
            "wsl -e %F",
            "find %D -exec %N {} ;",
            "timeout 30 %F",
        ] {
            assert_eq!(split_command(line), Err(CommandError::ProgramPlaceholder), "{line}");
        }
        for line in [
            // wrappers around a script or a program that takes the name as an argument
            "env /x/args.sh %N",
            "/usr/bin/env FOO=%N TORRENT=%L /x/args.sh",
            "nohup /x/args.sh %N",
            "nice -n 10 ionice -c 3 /x/args.sh %N",
            "timeout 30 /x/args.sh %N",
            // a wrapper's program given as a path is the one that runs, whatever follows it
            "nohup /x/notify -t time %N",
            "nice /x/translate es %N",
            "sudo -u media /x/args.sh %N",
            "sudo -u %L /x/args.sh",
            "busybox sh /x/args.sh %N",
            "env sh /x/args.sh %N",
            "flock /tmp/lock /x/args.sh %N",
            "watch -x /x/args.sh %N",
            "xargs -0 /x/args.sh %N",
            "su media",
            "wsl -e /x/args.sh %N",
            "wsl --exec /usr/bin/notify-send %N",
            "wsl -d Ubuntu --shell-type none /x/args.sh %N",
            // interpreters running a script
            "python /x/args.py %N",
            "python3 -u /x/args.py -c %N",
            "python -m notify %N -c",
            "python -X utf8 /x/args.py %N",
            "perl /x/args.pl %N",
            "perl -Mstrict -I/x/lib /x/args.pl %N",
            "ruby -r json /x/args.rb %N",
            "node /x/args.js %N",
            "node -r dotenv/config /x/args.js %N",
            "deno run /x/args.ts %N",
            "php -f /x/args.php %N",
            "lua /x/args.lua %N",
            "osascript /x/notify.scpt %N",
            "Rscript /x/args.R %N",
            // awk and sed with the name as data
            "awk -f /x/prog.awk %F",
            r#"awk "{ print }" %F"#,
            r#"awk -v name=%N "BEGIN { print name }""#,
            "sed -n p %F",
            "sed -f /x/prog.sed %F",
            r#"sed -i.bak "s/a/b/" %F"#,
            // find with the name as data, or a script for -exec
            r#"find %D -name "*.nfo" -delete"#,
            "find %D -exec /x/args.sh {} %N ;",
            // a script's own options
            "/x/done.sh -c %L",
            r#""C:\x\done.cmd" -c %L /e %N"#,
            "notify.py -e %N",
            // other programs: codes elsewhere than after -c / -e
            r#"notify-send -e "Download finished" %N"#,
            r#"curl -d "name=%N" -e http://x/ http://x/hook"#,
            "rsync -avc %F /backup/",
            r#"ffmpeg -i %F -c copy "%D/out.mkv""#,
            "tool --command-file /x/c %N",
            // another program's own -c, -e, /c or --command is not a shell's: no guess is made
            // about programs no rule here knows
            "notify-send -e %N",
            "grep -e %N /x/list.txt",
            "rsync -c %F /backup/",
            "curl -e %T http://x/hook",
            r#"C:\tools\mover.exe /c "%F""#,
            "beet import --command %F",
            "mytool -c %L %F",
            // words that name a shell or a wrapper, without a string to run
            "notify-send sh %N",
            "notify-send Time %N",
            r#"notify-send "%N" python -c"#,
        ] {
            assert!(split_command(line).is_ok(), "{line}: {:?}", split_command(line));
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

    /// A stored line that cannot run (stored before a rule refused it) is off, and says so in
    /// the Logger once, at the next alert; a new refused line is reported again.
    #[test]
    fn a_stored_line_that_cannot_run_is_reported_once() {
        let mut r = RunProgram::new(r#"ssh host "notify %N""#, "/x/done.sh %N");
        r.spawner = no_spawn;
        assert!(r.on_finished.len() == 2 && r.on_added.is_empty(), "only the refused line is off");
        let log = feed(&mut r, Alert::StateUpdate(Vec::new()));
        assert_eq!(log.len(), 1, "{log:?}");
        assert!(log[0].starts_with(r#"Run Program: ssh host "notify %N": "#), "{log:?}");
        assert!(feed(&mut r, Alert::StateUpdate(Vec::new())).is_empty(), "once");
        // the same lines again (nox sets them when its settings change): nothing new
        r.set(r#"ssh host "notify %N""#, "/x/done.sh %N");
        assert!(feed(&mut r, Alert::StateUpdate(Vec::new())).is_empty());
        // a new line that cannot run, with a control character that is not logged
        r.set("env %N\u{7}", "/x/done.sh %N");
        let log = feed(&mut r, Alert::StateUpdate(Vec::new()));
        assert_eq!(log.len(), 1, "{log:?}");
        assert!(log[0].starts_with("Run Program: ") && !log[0].contains('\u{7}'), "{log:?}");
        // lines that are fine, or empty, say nothing
        r.set("", "/x/other.sh %N");
        assert!(feed(&mut r, Alert::StateUpdate(Vec::new())).is_empty());
        assert!(RunProgram::new("", "").on_alert(&Alert::StateUpdate(Vec::new()), |_| None).is_empty());
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
