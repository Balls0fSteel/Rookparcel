//! The Torrent menu's and the list's context menu's commands on the selected torrents (GTK
//! `torrent_action`, `remove`, `open_folder`, `set_download_location`, `set_label`, `set_limit`,
//! `toggle_super_seeding`, `menu_state`), and Start All / Stop All of the Dock and status item.
//! Tags ▸ (D76) too. The logic is `Core`'s; this asks the questions (Remove's sheet, the folder
//! panel, New Label, New Tag) and gives the menus their check marks and greying ([`state`],
//! [`mixed`]).

use std::path::{Path, PathBuf};

use objc2::MainThreadMarker;
use objc2_app_kit::{NSPasteboard, NSPasteboardTypeString, NSWorkspace};
use objc2_foundation::{NSArray, NSString, NSURL};
use rookparcel_core::TorrentId;
use rookparcel_frontend::actions::LIMITS;
use rookparcel_frontend::app::TorrentAction;
use rookparcel_frontend::export::{ExportTarget, export_file_name};
use rookparcel_frontend::tags::TagState;
use rookparcel_frontend::tr;

use crate::app::{apply, peek, selected, ui, with_core};
use crate::commands::Command;
use crate::menu::{built_label, built_tag};
use crate::ui::{self as mui, Pick, Style};

/// Runs a torrent command on the selection (Start All and Stop All on every torrent).
pub fn run(cmd: Command) {
    use Command::*;
    let ids = selected();
    let action = |a: TorrentAction, ids: &[TorrentId]| {
        apply(with_core(|c| {
            c.torrent_action(a, ids);
            c.events()
        }))
    };
    match cmd {
        StartAll | StopAll => {
            let all = peek(|c| c.ids()).unwrap_or_default();
            action(if cmd == StartAll { TorrentAction::Start } else { TorrentAction::Stop }, &all);
        }
        _ if ids.is_empty() => {}
        ForceStart => action(TorrentAction::ForceStart, &ids),
        Start => action(TorrentAction::Start, &ids),
        Pause => action(TorrentAction::Pause, &ids),
        Stop => action(TorrentAction::Stop, &ids),
        QueueUp => action(TorrentAction::QueueUp, &ids),
        QueueDown => action(TorrentAction::QueueDown, &ids),
        QueueTop => action(TorrentAction::QueueTop, &ids),
        QueueBottom => action(TorrentAction::QueueBottom, &ids),
        Recheck => action(TorrentAction::Recheck, &ids),
        UpdateTracker => action(TorrentAction::Reannounce, &ids),
        OpenFolder => open_folder(&ids),
        Remove => remove(ids, false),
        RemoveAndDelete => remove(ids, true),
        SetLocation => set_location(ids),
        InitialSeeding => apply(with_core(|c| {
            c.toggle_super_seeding(&ids);
            c.events()
        })),
        Sequential => apply(with_core(|c| {
            c.toggle_sequential(&ids);
            c.events()
        })),
        FirstLast => apply(with_core(|c| {
            c.toggle_first_last(&ids);
            c.events()
        })),
        CopyMagnet => copy_magnets(&ids),
        ExportTorrent => export(ids),
        DownLimit(i) | UpLimit(i) => {
            let Some(&kib) = LIMITS.get(usize::from(i)) else { return };
            let download = matches!(cmd, DownLimit(_));
            apply(with_core(|c| {
                c.set_limit(&ids, download, kib.saturating_mul(1024));
                c.events()
            }));
        }
        NewLabel => {
            crate::dialogs::input::ask(
                &mui::text(tr!(NewLabelTitle)),
                &mui::text(tr!(NewLabelPrompt)),
                "",
                move |l| {
                    let l = l.trim().to_owned();
                    if !l.is_empty() {
                        set_label(&ids, &l);
                    }
                },
            );
        }
        RemoveLabel => set_label(&ids, ""),
        // the names the menu showed (M1): a label or tag learned while it was open shifts no item
        Label(i) => {
            if let Some(l) = built_label(i) {
                set_label(&ids, &l);
            }
        }
        Tag(i) => {
            if let Some(t) = built_tag(i) {
                apply(with_core(|c| {
                    c.toggle_tag(&ids, &t);
                    c.events()
                }));
            }
        }
        NewTag => new_tag(ids),
        ClearTags => apply(with_core(|c| {
            c.clear_tags(&ids);
            c.events()
        })),
        _ => {}
    }
}

/// Whether a command needing a selection is enabled now, and its check mark (Label ▸, Tags ▸,
/// Bandwidth ▸, Initial Seeding, the sequential and first-and-last toggles), from `Core::menu_state`;
/// Tags ▸'s items from `Core::tag_state` and `Core::can_clear_tags`, each looking at one tag or
/// none (every menu and toolbar item is validated on its own).
pub fn state(cmd: Command) -> (bool, Option<bool>) {
    use Command::*;
    let ids = selected();
    match cmd {
        _ if ids.is_empty() && matches!(cmd, Tag(_) | ClearTags) => return (false, None),
        Tag(i) => {
            return tag_state(&ids, i).map_or((false, None), |s| (true, Some(s == TagState::On)));
        }
        ClearTags => return peek(|c| c.can_clear_tags(&ids)).map_or((false, None), |on| (on, None)),
        _ => {}
    }
    let Some(m) = peek(|c| c.menu_state(&ids)).flatten() else { return (false, None) };
    let limit = |i: u16, kib: u32| Some(LIMITS.get(usize::from(i)) == Some(&kib));
    match cmd {
        OpenFolder => (m.can_open_folder, None),
        SetLocation => (m.can_set_location, None),
        InitialSeeding => (m.can_super_seed, Some(m.super_seeding)),
        Sequential => (m.can_sequential, Some(m.sequential)),
        FirstLast => (m.can_sequential, Some(m.first_last)),
        CopyMagnet => (true, None),
        ExportTorrent => (m.can_export, None),
        DownLimit(i) => (true, limit(i, m.download_kib)),
        UpLimit(i) => (true, limit(i, m.upload_kib)),
        RemoveLabel => (m.can_remove_label, None),
        Label(i) => (true, Some(m.label.is_some() && m.label == built_label(i))),
        _ => (true, None),
    }
}

/// The state across `ids` of the tag Tags ▸'s item `i` shows (`menu::built_tag`, M1); `None`
/// past the menu's tags or without a core.
fn tag_state(ids: &[TorrentId], i: u16) -> Option<TagState> {
    let tag = built_tag(i)?;
    peek(|c| rookparcel_frontend::tags::state(c.list(), ids, &tag))
}

/// Whether a tag's item in Tags ▸ shows the mixed mark: some of the selection have it (D76).
pub fn mixed(cmd: Command) -> bool {
    let Command::Tag(i) = cmd else { return false };
    let ids = selected();
    tag_state(&ids, i) == Some(TagState::Mixed)
}

/// Tags ▸ > New Tag: a name as New Label asks it; the selection gets the tag (made known when
/// new). An empty name does nothing, as New Label's; a refused one says why.
fn new_tag(ids: Vec<TorrentId>) {
    ask_tag(move |name| {
        let made = with_core(|c| c.create_tag(&ids, name).map(|_| c.events()))?;
        Some(made.map(|ev| apply(Some(ev))))
    });
}

/// Asks a tag's name (New Tag's prompt) and hands it to `make`, which gives `Some(Err)` for a
/// name refused, whose reason is then shown.
pub fn ask_tag(make: impl FnOnce(&str) -> Option<Result<(), rookparcel_frontend::tags::TagError>> + 'static) {
    crate::dialogs::input::ask(
        &mui::text(tr!(NewTagTitle)),
        &mui::text(tr!(NewTagPrompt)),
        "",
        move |name| {
            if name.trim().is_empty() {
                return;
            }
            if let Some(Err(e)) = make(&name) {
                if let Some(mtm) = MainThreadMarker::new() {
                    crate::dialogs::tell(None, Style::Warning, &e.text(), "", mtm);
                }
            }
        },
    );
}

/// Open Containing Folder's text: one finished torrent's own folder, or the torrents'.
pub fn open_folder_title() -> &'static str {
    let ids = selected();
    match peek(|c| c.menu_state(&ids)).flatten() {
        Some(m) if m.single_done => tr!(MenuOpenFolder),
        _ => tr!(MenuOpenFolderMany),
    }
}

fn set_label(ids: &[TorrentId], label: &str) {
    apply(with_core(|c| {
        c.set_label(ids, label);
        c.events()
    }));
}

/// Shows the first torrent's file or folder in the Finder (or, before it exists, the nearest
/// folder that does). Nothing in a remote window: the files are on the server.
pub fn open_folder(ids: &[TorrentId]) {
    let Some(path) = peek(|c| c.folder_of(ids)).flatten() else { return };
    reveal(&path);
}

/// Shows `path` selected in the Finder, or opens its nearest existing folder.
pub fn reveal(path: &Path) {
    let ws = NSWorkspace::sharedWorkspace();
    let url = |p: &Path| NSURL::fileURLWithPath(&NSString::from_str(&p.to_string_lossy()));
    if path.exists() {
        ws.activateFileViewerSelectingURLs(&NSArray::from_retained_slice(&[url(path)]));
        return;
    }
    if let Some(dir) = path.ancestors().skip(1).find(|p| p.is_dir()) {
        ws.openURL(&url(dir));
    }
}

/// Remove and Remove And Delete: Preferences' question as a sheet (`Core::remove_question`),
/// then `Core::remove`.
fn remove(ids: Vec<TorrentId>, with_data: bool) {
    let Some(mtm) = MainThreadMarker::new() else { return };
    let question = peek(|c| c.remove_question(&ids, with_data)).flatten();
    let go = move |ids: &[TorrentId]| {
        apply(with_core(|c| {
            c.remove(ids, with_data);
            c.events()
        }))
    };
    let Some((message, detail)) = question else {
        go(&ids);
        return;
    };
    let window = ui().map(|u| u.window.clone());
    mui::sheet(
        window.as_deref(),
        if with_data { Style::Warning } else { Style::Info },
        &message,
        &detail,
        &[tr!(BtnRemove), tr!(BtnCancel)],
        move |b| {
            if b == 0 {
                go(&ids);
            }
        },
        mtm,
    );
}

/// Advanced > Set Download Location: a folder from the open panel (a remote window's is typed:
/// a folder on the server), then the files move there.
fn set_location(ids: Vec<TorrentId>) {
    let Some(mtm) = MainThreadMarker::new() else { return };
    let Some((start, remote)) = peek(|c| (c.download_location_start(&ids), c.is_remote())) else { return };
    let start = start.unwrap_or_default();
    let move_to = move |dir: String| {
        apply(with_core(|c| {
            c.set_download_location(&ids, &dir);
            c.events()
        }));
        crate::detail::refresh();
    };
    if remote {
        let text = tr!(WebSetLocation);
        let (title, prompt) = text.split_once("\n\n").unwrap_or((text, text));
        crate::dialogs::input::ask(&mui::text(title), &mui::text(prompt), &start, move_to);
        return;
    }
    let pick = Pick {
        folders: true,
        start: Some(PathBuf::from(&start)).filter(|p| p.is_dir()),
        message: mui::text(tr!(PickLocation)),
        ..Pick::default()
    };
    let window = ui().map(|u| u.window.clone());
    mui::open_panel(
        window.as_deref(),
        pick,
        move |paths| {
            if let Some(p) = paths.first() {
                move_to(p.to_string_lossy().into_owned());
            }
        },
        mtm,
    );
}

/// Copy Magnet Link: the selection's links, one per line, on the general pasteboard.
fn copy_magnets(ids: &[TorrentId]) {
    let Some(text) = with_core(|c| c.copy_magnets(ids)) else { return };
    // (popped alerts wait for the next tick)
    if text.is_empty() {
        return;
    }
    let board = NSPasteboard::generalPasteboard();
    board.clearContents();
    // SAFETY: AppKit's pasteboard type constant.
    board.setString_forType(&NSString::from_str(&text), unsafe { NSPasteboardTypeString });
}

/// Export .torrent: one torrent to the file the save panel names (`<name>.torrent` offered),
/// several into a folder from the open panel. `Core` writes them when the engine hands the
/// files over, logs each, and reports a failed file export among its events' errors.
fn export(ids: Vec<TorrentId>) {
    let Some(mtm) = MainThreadMarker::new() else { return };
    // only the torrents with metadata (a magnet still fetching it has no .torrent yet)
    let Some(named) = peek(|c| {
        ids.iter()
            .filter_map(|&id| c.list().get(id).filter(|s| s.has_metadata).map(|s| (id, s.name.clone())))
            .collect::<Vec<_>>()
    }) else {
        return;
    };
    let window = ui().map(|u| u.window.clone());
    let ids: Vec<TorrentId> = named.iter().map(|(id, _)| *id).collect();
    let go = move |target: ExportTarget| {
        apply(with_core(|c| {
            c.export_torrents(&ids, target);
            c.events()
        }))
    };
    match named.as_slice() {
        [] => {}
        [(_, name)] => crate::dialogs::save_panel(
            window.as_deref(),
            &export_file_name(name),
            None,
            move |path| {
                if let Some(p) = path {
                    go(ExportTarget::File(p));
                }
            },
            mtm,
        ),
        _ => {
            let pick = Pick { folders: true, message: mui::text(tr!(ExportFolderTitle)), ..Pick::default() };
            crate::dialogs::open_panel(
                window.as_deref(),
                pick,
                move |paths| {
                    if let Some(p) = paths.into_iter().next() {
                        go(ExportTarget::Folder(p));
                    }
                },
                mtm,
            );
        }
    }
}
