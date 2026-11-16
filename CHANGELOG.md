# Changelog

Recent completed changes, issues and decisions. Timestamps: `YYYY-MM-DD_HHMMSS_<unix epoch>` (Europe/Budapest).

Earlier history: [archive index](docs/archive/README.md), including all 148 original entries from 2026-10-15 through 2026-10-23. Open archived entries only for a specific historical question. Current work: [PLAN.md](PLAN.md).

## 2026-11-16_140743_1794834463: Version 1.2.60: excluded file names (D74), the IP filter's block list from an https address with CIDR (D75); feature wave 2

- **Asked for (owner, 2026-11-16):** wave 2 of the feature plan (PLAN.md): "Do not download files named" as qBittorrent has it, and the IP filter's list from a web address, with CIDR lines.
- **Engine:** `IpFilter` reads CIDR lines (IPv4 and IPv6) beside ipfilter.dat and P2P lines, in a local file too.
- **Shared:**
  - `patterns.rs` (Create's Skip Files matcher moved; `;`, `|` and line breaks separate). `exclude.rs`: `excluded`, `priorities`, `apply_to_new`, and `Pending` for magnets (`post_file_list` once the metadata is in).
  - `ip_filter_list.rs`: https only, 32 MiB fetched and 256 MiB inflated (bounded while inflating), parsed on the worker, written to a `.part` then renamed; the kept copy `ipfilter-url.dat` with `ipfilter-url.txt`; every 24 h, an hour after a failure; a fetch held by D70 starts at the first tick after the hold lifts.
  - `Field.unless` in `pref_pages`, `FileTree::for_add`; 4 strings and 3 help texts; the settings fixture 1.2.60, with `files.exclude_names` and `ipfilter.url` absent at their defaults; the D70 guard's summary unchanged.
- **Win32:** the fields on UI Settings and BitTorrent (groups grown to 89 and 92 DU), `add_without_dialog` for every add without the dialog, `App.ip_list` with its timer. Hidden-desktop captures light and dark; the cut-off text audit clean in 35 languages for both pages (70 runs).
- **GTK:** `Grey` so greying honours `unless`. On the Xubuntu VM 25 tests and live runs with two instances and a local tracker: the file unticked in the dialog and tickable; a .torrent and a magnet added without the dialog left the file skipped and absent from disk. Live fetches from a local HTTPS server (the repo's TLS fixture trusted through `SSL_CERT_FILE`): plain and gzip lists with P2P, dat, IPv4 and IPv6 CIDR lines (4 rules), an HTML page refused, an http address refused, the kept copy loaded at restart without a fetch.
- **Mac:** a page's buttons grey with the field whose row they share (so Trackers' Update Now greys while its box is off, as on Win32). On the macOS VM 23 tests in light, dark and German: the pages, greying, the https error, the Add dialog's unticked file ticked back by a real click, an add without the dialog giving `file_priority [0,4,4]`. No live fetch there.
- **nox:** the fields on UI Settings and BitTorrent, `disabled_by` in the page model, `GET /pa/prefs/ipfilter` and `POST /pa/prefs/ipfilter/update`; both address fields need the current password (`SECURITY_FIELDS`), and a hosted Web UI leaves them out and refuses them. Shim: `excluded_file_names_enabled` / `excluded_file_names` writable, `ip_filter_enabled` / `ip_filter_path` read-only; `torrent-info` marks `excluded`, and the page then sends every file's priority. Files are left out on shim adds, native adds with the dialog off, RSS, auto-load and search adds.
- **Review** by a separate subagent. Two defects fixed in the wave: a duplicate magnet add could override file choices through `Pending` (now it waits for that id's `MetadataReceived`), and a failed write could leave a stale `.part` to become the kept list. Its five follow-ups, fixed after:
  - The Logger and error texts name the block-list address as scheme, host, port and path (`ip_filter_list::redact`: no user name, password, query or fragment), a bad redirect target too. D66's tracker list names no address in its lines (only a bad redirect target inside a fetch error); left as it is.
  - A magnet add duplicating a torrent already in the session (restored, or waiting for the Add window) is not remembered by `Pending`: `Pending::add` asks the owner's list (`TorrentList::has`, nox's torrents) through `exclude::same_torrent`. A duplicate `.torrent` needed nothing: the engine leaves the existing torrent as it is.
  - `PrefIpFilterEnable` names CIDR lines; en.lng, HELP_TEXTS.md and the wiki row follow.
  - The excluded-names list keeps at most 256 patterns and 4,096 bytes, cut after a whole pattern; the rest is dropped.
  - Win32's IP filter Update Now no longer applies the dialog: it checks the https rule and fetches with the page's fields, as the Trackers page's does. Closing without OK puts the stored fields back and, when they differ from those tried, the stored rules.
  - Found sound: CIDR parsing (checked shifts and bounds, existing lines unchanged), the inflation bound, an address change mid-fetch, the held-fetch loop, the wildcard matcher's cost (not exponential), the new routes' session, CSRF and Host rules, no innerHTML. Left as is, like D66: the daemon fetches any https address its password holder sets (SEC-26), and Update Now has no rate limit.
- **Checked:** clippy `-D warnings` clean for core, frontend, nox and Win32, both Mac cross-lints and GTK's (after the follow-ups, before the bump). After the bump: fmt clean; tests core 1,101, frontend 437, Win32 161 (one thread), nox 227, all passing; the settings fixtures pass unchanged. The fuzz workspace was checked before the follow-ups, which touched no engine code.
- **Open:** BACKLOG "Excluded file names and the IP filter address": a magnet whose metadata comes after a restart, no Win32 or Mac live fetch, the Web UI on a phone, Safari and Firefox, the machine translations, zip lists, conditional fetches.

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
