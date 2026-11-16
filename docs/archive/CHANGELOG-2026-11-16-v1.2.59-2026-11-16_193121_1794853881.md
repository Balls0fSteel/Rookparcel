# Changelog archive: 2026-11-16, version 1.2.59 (feature wave 1)

1 entry moved verbatim from the root changelog on 2026-11-16_193121_1794853881 to keep it under budget.

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
