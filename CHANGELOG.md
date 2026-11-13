# Changelog

Recent completed changes, issues and decisions. Timestamps: `YYYY-MM-DD_HHMMSS_<unix epoch>` (Europe/Budapest).

Earlier history: [archive index](docs/archive/README.md), including all 148 original entries from 2026-10-15 through 2026-10-23. Open archived entries only for a specific historical question. Current work: [PLAN.md](PLAN.md).

## 2026-11-13_214643_1794602803: Version 1.2.48: the UI lock in the GTK GUI (D69)

- **Menus:** File > Lock Rookparcel (Ctrl+L) above Exit; Options > Lock PIN with Set, Change and Remove PIN, greyed by whether a PIN is set; Lock without a PIN opens Set PIN first. The tray offers Lock Rookparcel with a PIN set, and only Show and Exit while locked (a pure `tray::visible` also drops leftover separators). Built in a worktree, merged here.
- **Locking:** every visible window but the Auto Shutdown countdown hides and comes back after the right PIN; `ui.locked` is saved at once. Showing the windows (the tray, a second start) asks in one prompt (new `lock.rs`: hidden digit-only boxes, the filter on the entry's delegate, the shared `ui_lock::submit`). While locked, message boxes, magnets' Add dialogs, downloaded torrents' dialogs and update offers wait; completion notifications are dropped; torrents from a second start wait for the PIN and are dropped on Cancel.
- **Starting locked:** the main window is never shown; a start by hand asks at once, `--minimized` stays in the tray. Without a tray (plain GNOME) the prompt stands in for the window: no Cancel, and closing it quits. Exit closes the never-shown main window by emitting `close-request` itself (`gtk_window_close` ignores unrealized windows). On the way: Minimize to tray now also works for a window first shown after the tray started.
- **Reset:** `--reset-lock` asks (Cancel/OK, as the GTK GUI's other questions; the tables have no Yes/No) before the window is built, or in the running window when another start forwards it; OK removes the PIN and logs it; with no PIN it says "No lock PIN is set."
- **Validation:** clippy for Linux from Windows (stand-in pkg-config); on the Xubuntu VM `cargo test -p rookparcel-gtk` (22, 2 new) and an Xvfb run with a private bus (no tray) and with the Xfce panel's tray under a profile: lock, the tray menus read over D-Bus (locked: Show, Exit; unlocked with a PIN: Lock above Exit), five wrong PINs then the 30 s wait, `12ab34` typed as `1234`, Ctrl+Q from the prompt, a locked restart showing only the prompt, a second start bringing the same prompt forward, Escape keeping it locked in the tray, `--reset-lock` at start (Cancel, OK, no PIN) and forwarded. The VM's own autostarted AppImage shares the default application id, so the tray run used `--profile locktest`.

## 2026-11-13_210657_1794600417: Version 1.2.47: the shared part of the UI lock (D69)

- **Asked by the owner:** a PIN lock for the desktop windows, as qBittorrent's "Lock qBittorrent", so someone at the computer cannot see the torrent list; easy to reset, with a wiki page on how. Owner's answers: a PIN of 4 to 6 digits, the windows hidden in the tray, locking by hand only (the state survives restarts), reset by holding Ctrl (Windows) or Option (macOS) at start or with `--reset-lock`. Plan: this shared slice, then one slice per GUI.
- **Moved:** the PBKDF2-HMAC-SHA256 password hash, HMAC-SHA256, the constant-time compare and `random_hex` from nox's `webui::auth` to `core::crypto::password` (with their RFC tests), so the GTK and Mac GUIs, which do not link nox, can hash the PIN; `webui::auth` re-exports them.
- **New `frontend::ui_lock`:** the keys `ui.lock_pin` (a salted hash in the Web UI password's format) and `ui.locked`, outside `GuiOptions` so Preferences never writes them; PIN rules; lock, unlock and remove; `Guard` (from the fifth wrong PIN on, each one starts a 30 s wait, in memory); `Form` and `submit` for the four PIN dialogs, so each GUI only lays out boxes; `--reset-lock`; `without_lock_keys` for the Web UI a Windows window hosts. `Events::take_popups` lets the Mac hold its popups while locked.
- **Strings:** 20 new keys (menus, the prompt, the PIN dialogs, the reset question) in all 35 tables; the 34 translations are machine-made.
- **Validation:** tests on Windows: core lib (935, 4 new), frontend (390 + 4, 9 new), nox (all suites, a new hosted test that the Web UI never shows or changes the lock keys); workspace fmt and clippy clean; the GTK and Mac crates lint from Windows (stand-in pkg-config, the 1.90 toolchain's targets). No GUI uses the lock yet.

## 2026-11-11_085623_1794383783: The nox search test no longer races the torrent list

- **Failure:** CI for `05f93fdd` (no code changed), Windows only: `sources_search_and_add` got 200 where it expects 409 for a second add of the same magnet.
- **Cause (read from the code, not reproduced here):** `/pa/search/add` answers 409 from `by_hash`, which is filled when a torrent's first status arrives. The test added a `.torrent` and a magnet, waited only for the `.torrent` to be listed, then added the magnet again; on a slow runner the magnet's status had not come yet.
- **Fix (test only, no version bump):** the test waits until both torrents are listed. `cargo test -p rookparcel-nox --test search`: 3 passed, and the test alone passed 8 more times.
- **Left:** the gap itself (PLAN "Current status"): a real second add inside it returns 200.

## 2026-11-11_085223_1794383543: Wiki source and the FAQ's client ID brought up to 1.2.46

- **Wiki (`docs/wiki`, 20 pages):** every "not in a published release yet" mark is gone (all of it shipped by 1.2.46; the "New in x" versions stay); the macOS notes say "new in 1.2.19"; Home and Installing name 1.2.46, with 1.2.46 file names in the examples; Profiles says "New in 1.2.0". BRIEF.md now tells writers to add the mark only for unreleased features and remove it at the release.
- **Site:** `site/faq.html` gives the client ID as `RP` (peer ID `-RP1014-`), was `PA`. Site tests 37 of 37.
- **Pushed (2026-11-11_085425_1794383665, the owner said so):** `main`, and the 19 changed pages to the GitHub wiki (compared page by page first; only these differed). Checked live: the site FAQ says `RP`, the wiki's Home names 1.2.46.
- **Seen on the way:** CI for `05f93fdd` (documentation and benchmark files only) failed on Windows, `sources_search_and_add` at `crates/nox/tests/search.rs:341`; Linux, macOS and FreeBSD passed. Not looked into.

## 2026-11-11_085003_1794383403: README brought up to 1.2.46

- **Asked by the owner:** the README still named 1.1.40 as the latest stable release.
- **Changed (documentation only, no version bump):** the note box and the Download link say 1.2.46 (`gh release view`: `v1.2.46`, not a pre-release); Status says 1.2 and lists what 1.2 added (profiles, the Files tab folder tree, Web UI themes, Statistics, trackers for new downloads, Run Program, the Mac app, the Docker image, Win32 dark mode) plus 1.1.40's keep-ping-low and smart disk I/O; search is "in every interface"; the old arm64 dmg is dated "before 1.2.19"; the GTK row no longer says "not yet used on a desktop" (checked by hand on the Xubuntu VM; GNOME and KDE still open).
- **Found and left:** `docs/wiki/Home.md` and `docs/wiki/Installing-Rookparcel.md` still say 1.1.63 / 1.1.40, and `site/faq.html` gives the client ID as `PA` (it is `RP` since 1.2.2). PLAN "Current status".

## 2026-11-11_083809_1794382689: Benchmarks run again on the published 1.2.46

- **Asked by the owner:** the run of 2026-11-09 found the idle CPU regression and its site page was held back; run it all again overnight.
- **Daemon run (Xubuntu VM, 8 GB, 00:11-04:30, host left alone):** `rookparcel-1.2.46-linux-x86_64.tar.gz` (checked against `SHA256SUMS`) with 1.0.1, 1.1.40 and 1.2.29 as `--control`, beside qBittorrent, Transmission, Deluge and rTorrent; all 88 runs good. 10,000 torrents: idle 0.07% of a core (1.2.29 in the same rounds 0.35%, 1.0.1 0.13%, qBittorrent 0.10%), 67 MB after a restart; download 3,148 MiB/s and 1.01 CPU s/GiB, as every older version.
- **Odd figure, explained as far as checked:** the session's first download (Rookparcel's, minutes after the VM booted) ran at 349 MiB/s with 21.6 s of system time in the disk threads. Two short runs afterwards: 1.0.1 put first is the slow one too (1,822-1,955 MiB/s, 1.2.46 right after it 3,161-3,599). The position, not the version; medians untouched; cause not looked into (BACKLOG).
- **Development PC, 100,000 torrents:** engine alone (three `lt-swarm` builds, three rounds) no difference; Win32 GUI from the published zips, three rounds each: the process idles at 0.21-0.42% (1.2.29: 0.81-0.86%, 1.0.1: 0.29-0.44%), the list after 1.37-1.49 s (1.2.29: 1.36-1.38, 1.0.1: 1.27). One engine idle figure of 0.16% was three repeats at 0.03-0.05%: noise.
- **Mac VM:** release build of 1.2.46, `--demo 100000`, three runs: the list after 0.50-0.57 s, 188-189 MB (these three ran after the owner was back at the PC).
- **Written:** BENCHMARKS.md (new results first, 2026-11-09 kept as earlier), raw files under `docs/benchmarks/2026-11-11-*`, `site/benchmarks.html` filled by `summarize.py --fill` with a "Since 1.0.1" section and its prose rechecked by hand (site tests 37 of 37). **Not done:** the page is committed but not pushed (looked at once in a local build: six sections, the version table's 13 rows, no overflow); no program changed, so no version bump.
- The Claude app was found closed in the morning, cause unknown; every run had finished and nothing was lost.

## 2026-11-11_000208_1794351728: Release 1.2.46 published

- **Asked by the owner** after the change below. `main` at `965b54ca` (the notes `docs/releases/1.2.46.md`, the Flatpak metainfo entry) with CI, FreeBSD and Fuzz green; tag `v1.2.46`; the Release run built 13 files and `SHA256SUMS` and both container images; published as the latest release (2026-11-10 23:02 UTC). Then the site's news entry pushed to `main`. No wiki page changed.
- **Checked on the draft:** every file against `SHA256SUMS`; the zip's `rookparcel-nox.exe --version`.
- **Skipped of RELEASING.md step 5:** the setup and the zip on a clean Windows VM, the update through Help > Check for Updates, the tarballs, AppImages and Flatpaks on a Linux desktop, the dmg on the Mac VM, the container image pulled and run. No program of 1.2.46 was started on a profile with a missing file; the tests are what showed the change.
- The oldest entry moved to the archive.

## 2026-11-10_230554_1794348354: Version 1.2.46: a file that is gone costs only its own pieces; the Logger without "received N peers"

- **Reported by the owner** after updating to 1.2.45: "it rechecked a bunch again", and the Logger full of "received N peers". The update had nothing to do with it.
- **Found in the owner's profile (read only: `resume.log`, `metadata.pack`, the files, Defender's detections):** every record had its full piece set. One torrent, The Plucky Squire (12.3 GB), lacked `setup.exe` (5 MB, pieces 3151-3153): Windows Defender removed it at 08:30 the day before, at 22:00 and at 22:55, each time right after it was downloaded again. So each start refused that torrent's resume data, hashed all 3,154 pieces, fetched the three again, and Defender took the file again. The other "Checking 0.0 %" rows were not hashed: they waited behind it for their turn on the same drive (the start order shows it: everything from that torrent on).
- **Changed (`core::disk`, our addition, libtorrent hashes everything):** resume data whose only fault is files that are gone keeps its other pieces; the pieces touching a gone file are dropped and nothing is hashed (`ResumeFiles::Missing`, `CheckResult::missing`). A file that is too short, cannot be looked at, or a part file without a piece still means a full check. The Logger says "setup.exe is missing: 3 pieces to download again".
- **Logger:** "received N peers" (`TrackerReply`, `DhtReply`) is left out in the Win32, GTK and Mac clients and the Web UI; the Trackers tab has the numbers. `rookparcel-nox --verbose` still prints them.
- **Validation:** tests on Windows: core lib (931), core's integration suites with the new `a_file_removed_while_stopped_loses_only_its_own_pieces` (a restart after deleting one file: 39 of 41 pieces, the line, no "rechecked"), frontend (381 + 4), nox (all suites), Win32 (140, one thread); clippy `-D warnings` for the workspace; fmt. **Not done:** no run of the built client on the owner's profile; the GTK and Mac one-line changes did not run on the VMs; no release. Not ours to change: Defender's detection (an exclusion for the folder, or `setup.exe` set to Skip, ends the loop).
- **Seen in the same journal, not explained (BACKLOG "Disk and storage"):** earlier that day two seeds (Phonopolis, Lost in Play) came out of a check with no pieces and downloaded everything again, and several others showed no pieces for one save and all of them at the next.
- The oldest entry moved to the archive.

## 2026-11-10_223619_1794346579: Release 1.2.45 published

- **Asked by the owner** after the change below. `main` at `86b0ff66` (the notes `docs/releases/1.2.45.md`, the Flatpak metainfo entry) with CI, FreeBSD and Fuzz green; tag `v1.2.45`; the Release run built 13 files and `SHA256SUMS` and both container images; published as the latest release (2026-11-10 21:36 UTC). Then the site's news entry pushed to `main` and the changed `docs/wiki` pages copied to the wiki.
- **Checked on the draft:** every file against `SHA256SUMS`; the zip's `rookparcel-nox.exe --version`; that program started on three scratch folders, read through the shim's `app/preferences` and its `webui.json` afterwards: a stored 150 % / 0 with the box off stays off (`max_ratio_enabled` false), a stored 200 % gets the box ticked with 0 (true), a new folder is off; each has `queue.seed_goal_rule=2`.
- **Skipped of RELEASING.md step 5:** the setup and the zip on a clean Windows VM, the update through Help > Check for Updates, the tarballs, AppImages and Flatpaks on a Linux desktop, the dmg on the Mac VM, the container image pulled and run. No GUI of 1.2.45 was run by hand.

## 2026-11-10_220442_1794344682: Version 1.2.45: the default seeding goal is off until its box is ticked (D68)

- **Reported by the owner:** of 41 seeds started together about half went back to paused at once. Cause: Preferences > Queueing's default goal (150 %) was always in force, and with "Limit the upload rate" off it stopped every queued seed past it, again after each Start. µTorrent 2.2.1 has the same 150 % with that box off and stops nothing.
- **Changed (`frontend::seeding`):** the default goal acts only while "Limit the upload rate to (kB/s): [0: stop]" is ticked (0 stops, a rate caps); off, as at first, torrents seed on. A torrent's own goal (Properties, the Web API's share limits, an import) always acts, which the *arr apps' Remove Completed needs. A goal acts on a torrent once (`torrent.<key>.goal_done` = the goal that acted): a torrent started again seeds on, a changed goal acts again. `SeedingGoal::run` is the one implementation the Win32, GTK, Mac (shared model) and nox loops call; they had four copies.
- **Carried over once (`seeding::migrate`, marker `queue.seed_goal_rule=2`, called where each program reads its settings):** a profile at the untouched 150 % / 0 minutes is left with the box off, so its goal is gone; a profile whose goal was changed gets the box ticked with 0 and keeps stopping. A new profile has nothing to carry.
- **Logger:** the line names what was reached: "seeding goal reached, stopped (ratio 1.523, goal 1.500)", with the seeding time when that target is set.
- **Web UI and shim:** the box reads "Stop the torrent or limit its upload rate" (`PrefGoalAct`, replacing `PrefGoalLimitInstead`); `app/preferences` reports `max_ratio_enabled` / `max_seeding_time_enabled` only for a default goal ticked with 0.
- **Also:** a removed torrent's own goal keys are dropped in every client (only nox did). Three strings in 34 tables, machine-made; help texts, the wiki's Queueing, Preferences reference and Auto Shutdown pages, INTEGRATIONS, PORTING and `tools/arr-test`.
- **Validation:** tests on Windows: frontend (381 + 4), nox (all suites), Win32 (140, one thread); new tests for the box-off default, the once-only rule and the carry-over; clippy `-D warnings` for the workspace, the GTK crate (Linux target) and the Mac crate (x86_64 darwin); fmt. **Not done:** no run of a real updated profile or of any GUI page; the GTK and Mac crates were only linted from Windows, not run; the wiki not copied to the wiki repository; no release.
