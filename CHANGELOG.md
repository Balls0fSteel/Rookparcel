# Changelog

Recent completed changes, issues and decisions. Timestamps: `YYYY-MM-DD_HHMMSS_<unix epoch>` (Europe/Budapest).

Earlier history: [archive index](docs/archive/README.md), including all 148 original entries from 2026-10-15 through 2026-10-23. Open archived entries only for a specific historical question. Current work: [PLAN.md](PLAN.md).

## 2026-11-16_122332_1794828212: Version 1.2.59: download order (D72), Copy Magnet Link and Export .torrent (D73); feature wave 1

- **Asked for (owner, 2026-11-16):** after a comparison with qBittorrent, Transmission, Deluge and BiglyBT, six features in four waves for one release (PLAN.md). Wave 1 is download order (D72) and Copy Magnet Link / Export .torrent (D73). The owner chose the torrent menu, the Add dialog and Properties for the switches, and no Preferences default.
- **Engine:** `TorrentFlags::FIRST_LAST_PIECES` (bit 30, resume key `rp-first-last`). `piece_priorities` gives the first and last 1 % of each wanted file, at least one piece, priority 7, pushed to the picker on a flag change, at start and on file-priority changes; sequential download already existed. A swarm test with a scripted seed records the request order: first and last pieces, then the rest in order.
- **Shared:**
  - `actions::{all_flagged, toggle_flag, any_unfinished, any_has_metadata}` are free functions, since Win32 and GTK have their own `App`; `MenuState` and `Core` methods serve the Mac.
  - `export.rs` (`magnet_for`, `magnet_links`, `Exports`, `ExportTarget`): a chosen file is replaced; a folder export numbers ` (2)`, ` (3)` with `create_new`.
  - Properties `Tri` fields and `AddChoice` flags. 15 strings and 8 help texts, translated into the 34 tables by a subagent; HELP_TEXTS.md gained a "Dialogs" section.
- **Win32:** ids 159-162 (159-161 reused from the removed Lock PIN items). Copy Magnet Link and Export .torrent sit under Open Containing Folder without icons (the bold set has no copy or export symbol); the toggles are in Advanced after a separator. `torrent_menu` split into `TorrentMenu` + `build_torrent_menu` so it can be tested. Properties grew to 300×352 DU with two full-width rows; the Add dialog grew 14 DU for a third option row.
- **GTK:** stateful `win.sequential` and `win.first-last`, plain `win.copy-magnet` and `win.export-torrent`; Properties puts sequential beside Enable DHT.
- **Mac:** command kinds 84-87; the Add dialog's left column widens for a long option text (German was clipped at the fixed 260 pt).
- **nox:** native `POST /pa/torrents/sequential` and `/first-last`, `GET /pa/torrents/magnet` and `/export` (an attachment header with an ASCII fallback and `filename*`, a new `ReplyKind::TorrentFile`, 409 for a magnet without metadata); Properties tri-states; add `first_last`.
  - Shim: `toggleSequentialDownload`, `toggleFirstLastPiecePrio` (POST only), `torrents/export`, `firstLastPiecePrio` and a real `f_l_piece_prio`.
  - The page copies through a ClipboardItem, falling back to a dialog with the links, and exports by fetch and a download link, at most 50 per click.
- **Checked:**
  - fmt and clippy `-D warnings` clean for core, frontend, nox and Win32; tests core 1,100, frontend 415, Win32 155 (one thread), nox 222, all passing; the fuzz workspace checks; both Mac cross-lints and the GTK cross-lint clean.
  - A separate subagent's review of the combined diff found no defect: a crafted torrent name cannot leave the export folder, the new endpoints follow their neighbours' session, Host and Origin rules, no innerHTML.
  - Win32 on the hidden desktop, light and dark; the cut-off text audit clean in all 35 tables for the menu, Advanced, Properties (one and several) and Add. In bg, fr, ru and hu the Add dialog's second box moves right of its column (cosmetic).
  - GTK on the Xubuntu VM under Xvfb: 24 tests, menu states, the clipboard read back, single and folder exports with matching info-hashes, the error box for a read-only folder, Properties mixed state, flags surviving a restart.
  - The Mac on the macOS VM in light, dark and German: 23 tests, menu marks and greying, `pbpaste`, exports through the test aids, Properties mixed state.
  - The Web UI against a mock daemon in the built-in browser, desktop and 375 px: the menu items and the phone sheet, a toggle reaching the engine, the magnet and export endpoints.
- **Open** (BACKLOG "Download order, magnet links and export"): Win32 Copy and the Save As / folder dialogs not clicked live; the Web UI's Copy and Export not clicked in a real browser (only the endpoints); a real video previewed while downloading; machine translations.
- **Side effects:** a `cargo +1.90` call auto-installed a second toolchain, `1.90-x86_64-pc-windows-msvc`, on this PC (still installed; removing it is the owner's call). The Xubuntu VM was started headless and left running.

## 2026-11-16_001242_1794784362: The Mac's dark Preferences sidebar symbols: the VM's drawing, not a bug (documentation only)

- **Asked for:** a fix for the Preferences sidebar's SF Symbols drawn nearly black on the dark sidebar on the macOS VM (BACKLOG "UI lock", older than D71).
- **Found:** a source-list row (`NSTableViewStyle::SourceList`) tints its template symbols with an effect that VMware's software renderer draws black. On the VM they are black in light mode too.
  - An `NSTableCellView` subclass that set `contentTintColor` (`secondaryLabelColor`, by background style) changed nothing. At run time the image was a template with the tint set under the dark appearance, and still black.
  - A small Swift app with the same outline in a sidebar split item showed the same, even with a red tint. Finder's own sidebar icons on the VM are black instead of the accent colour.
  - The same symbols outside a source list draw right in light and dark: plain and sidebar image views in that app, the browse buttons on Directories.
- **Kept:** no code change; the subclass was dropped, so no version bump. BACKLOG, `tools/mac-vm/README.md` ("Not seen right") and MACOS_GUI.md L (a check for the owner's real Mac) say so.

## 2026-11-15_232343_1794781423: Version 1.2.58: Preferences > Advanced > UI Lock, the lock's off switch, a narrower PIN dialog (D71)

- **Asked for:** a place in Advanced for the UI lock, a way to change the PIN and to turn the lock off, and a fix for the oddly sized Set PIN window on Windows. The owner chose an off switch that hides every Lock command and dropping Options > Lock PIN.
- **Shared:**
  - `ui_lock` has `ui.lock_enabled` (`enabled`, `set_enabled`; off removes the PIN), `Form::Off` and `State` (the page's status line and buttons).
  - `pref_pages` has a fifth Advanced page, `ui_lock`, with `Item::LockSwitch`, `Item::LockStatus` and three PIN actions. `UiIcon::Lock` is the bold set's lock.
  - The menu strings became `PrefLockSetPin`/`ChangePin`/`RemovePin`, renamed in every table so the translations stayed; `MenuLockPin` went. Eight new strings were translated into the 34 tables by a subagent, reusing each table's own words for PIN, File and Lock Rookparcel.
- **Win32:** the page's controls act through `Hooks::ui_lock`, with the PIN dialog modal on Preferences. `show_lock_item` deletes File > Lock while off and puts it back above Exit. `lock_command` ignores Ctrl+L and posted commands while off, and the tray offers Lock only when the lock is on and a PIN is set.
- **The PIN dialog:** it is 170 dialog units wide instead of 230, and the message line is one line, growing with the dialog when a message needs more (`fit_pin_message`). Set PIN went from 419×234 to 314×219 px at 100 %.
- **GTK and Mac:** subagents built them, and both cross-lint clean. GTK hides File > Lock with `hidden-when=action-disabled`. The Mac hides it in validation, refuses ⌘L while off, and drops `Command::SetPin/ChangePin/RemovePin` (kinds 81-83).
- **Checked:**
  - frontend 401 tests and Win32 151 tests (one thread), including new ones for the page, the switch and the dialog's growth.
  - Clippy with `-D warnings` for Win32, frontend and nox; GTK (Linux target) and the Mac (both targets) cross-linted.
  - The hidden desktop, light and dark: Set → Change with a wrong PIN → lock → wrong and right PIN in the prompt → switch off with the PIN → File > Lock gone and a posted Lock ignored → on again.
- **VM runs (same evening):**
  - **GTK on the Xubuntu VM** (Xvfb, private bus, no tray): the page, Set PIN, the switch with Cancel and with the PIN, File > Lock hidden and Ctrl+L ignored while off, both back once ticked, and Ctrl+L without a PIN asking for one before it locks.
  - **The Mac on the macOS VM** (a subagent, light and dark), the same plus Change PIN with a wrong PIN, no Lock in the status item's or the Dock's menu while off, and a lock and unlock.
  - No D71 bug. Found and left in BACKLOG "UI lock": "tray" in the Mac's status line, and the Preferences sidebar's symbols nearly invisible in dark mode (older than D71).

## 2026-11-15_180859_1794762539: Release 1.2.57 published; the site's news and the wiki pushed

- **On the owner's "make a release":** CI, FreeBSD and Fuzz green on `b0f3852b`; tag `v1.2.57`; the Release run [43738460424](https://github.com/Balls0fSteel/Rookparcel/actions/runs/43738460424) passed every job on the first attempt (Windows, macOS, both Linux arches, both Flatpaks, both Docker images and the manifest).
- **Checked on the draft:**
  - All 13 files' GitHub SHA-256 digests match `SHA256SUMS`, and the downloaded zip matches too.
  - The zip holds `rookparcel.exe`, `rookparcel-nox.exe`, `LICENSE.md` and `THIRD-PARTY-NOTICES.md`.
  - Both exes say 1.2.57 in their version resources and the manifest says 1.2.57.0; `rookparcel-nox --version` prints `rookparcel-nox 1.2.57`.
- **Published** with `gh release edit v1.2.57 --draft=false --latest` (18:07 Budapest).
- **Then:**
  - `main` fast-forwarded to the publish commit (`d032d7ed`: the news entry, the README's and the wiki's version lines, the settings check's "new in 1.2.57") and pushed.
  - The GitHub wiki got the four changed pages (`19a4266`); a page-by-page comparison showed nothing else differing.
- **Checked live:** `update.txt` says `stable=1.2.57`; the news page and `news.atom` lead with 1.2.57; the download page links the 1.2.57 setup.
- **Still open:** RELEASING.md step 5 by hand (the GUIs, the setup and the update from an installed 1.2.54, the Linux and Mac files, the image), and the settings guard's hand checks (BACKLOG "Settings guard").

## 2026-11-15_174308_1794760988: Version 1.2.57: the release review's findings in the settings guard; release 1.2.57 prepared

- **Asked for:** make a release. The security review of `v1.2.54..HEAD` (code read by a subagent, every finding checked here; [SECURITY-REVIEW.md](docs/SECURITY-REVIEW.md) "Release 1.2.57") found eleven things in D70, fixed here, so the release is **1.2.57**.
- **High, the order of OK:**
  - Preferences' OK used to lift the hold before the settings just entered reached the engine. Now `confirm_settings` runs after `apply_options` in all three GUIs.
  - The Setup Guide and the hosted Web UI's changes no longer confirm.
- **Medium:**
  - **The frontends' own fetches are held too:** RSS, `.torrent` links, the tracker list, search and the update check (`fetch::set_held`).
  - **Only a person's review confirms.** That means the Preferences dialog or the Web UI's Preferences page; a remote window's OK reaches nox's own confirmation. The scheduler, the qBittorrent API (a VPN port-forward helper posting `listen_port`), the limits and the Setup Guide do not confirm. nox's `store` keeps a line it cannot read.
  - **A file without settings is no first start.** Text without a setting (zeroes after a crash) counts as damaged; no settings beside saved torrents is `NoSettings`, held with no guide; a dangling link counts as unreadable. `private_file::write` now syncs before the rename.
- **Low:**
  - `prune` spares the user's own `.bak` files.
  - One `.damaged-` copy is kept per content.
  - The old `lt.net.*` keys are written beside the new ones, so going back to 1.2.54 keeps the network rule.
  - Damage near the proxy holds, and so does a damaged file whose copy failed.
  - A blocked save is an error the user sees.
  - nox's `read_state` retries.
  - Only the stamp's known fields are compared, and printed without control characters.
- **Release:** `docs/releases/1.2.57.md` covers 1.2.55-1.2.57. The Flatpak metainfo's first `<release>` is 1.2.57 (2026-11-15). The wiki's Updating page and D70 say what confirms. 1 new string (34 machine translations).
- **Validation:**
  - Tests: `cargo test --workspace --no-fail-fast` (40 binaries, run after the bump); clippy `-D warnings`, fmt, and the GTK and macOS cross-lint clean.
  - End to end, real nox with `lt.proxy_type=7` stored: it started held. `setPreferences` with `listen_port` applied the port, kept the hold and kept the line. The Web UI's `/pa/prefs` lifted the hold after the new port listened, and dropped the line.
- **Not done:** the GUIs' status bar and Preferences OK by hand. The fetch hold has no test of its own: the flag is process-wide, so a test would race the other fetch tests.
