# Changelog archive: 2026-11-15, version 1.2.55

1 entry moved verbatim from the root changelog on 2026-11-16_001242_1794784362 to keep it under budget.

## 2026-11-15_140244_1794747764: Version 1.2.55: settings not taken on trust (D70, part 1)

- **Asked for:** the owner remembered qBittorrent's update that lost users' proxy setting and asked for a smart guard rather than hand tests of every upgrade path. The owner chose all three layers of the plan (close the loss paths, a fixture test, a check after a version change) and holding the transfers when the proxy cannot be trusted (D70). This is layer 1.
- **Loss paths found and closed:**
  - **An unreadable settings file.** A `rookparcel.ini` that could not be read, or was not UTF-8, loaded empty, opened the Setup Guide and was saved over. Now a missing file is a first start. An unreadable one is never saved over and holds the transfers. A damaged one is read with the bad bytes replaced, after a `.damaged-<secs>` copy is kept. Notepad's UTF-16 reads as it is.
  - **Lines this build cannot read.** `settings::store` erased such a line (`lt.proxy_type=6` from a newer build) the next time it stored. Now the line stays until the user confirms (`settings::unreadable`, `forget_unreadable`).
  - **An unknown proxy type.** It became "none", a direct connection. Now `guard::on_start` starts the engine held.
  - **A damaged `webui.json`.** nox replaced it with defaults and saved over it. Now nox stops with an error that names the file.
- **The hold (core):** `SessionHandle::hold_transfers` and `SessionParams::hold_transfers`, through the D54 machinery.
  - While held, torrent traffic is bound to nothing: no listen socket, DHT, uTP, LSD or port mapping, and torrents wait. The hold is set before the first socket and the first restored torrent.
  - The status is `NetworkStatus::Held`, so the remote protocol is 19. The status bar says "Transfers held: confirm the settings" in every GUI and the Web UI, and the Logger says why.
  - Preferences' OK or Apply confirms the settings and lifts the hold. In nox a preference change through the Web UI or the API does the same.
- **nox, found in the same review:**
  - The network rule's rows were stored as `net.*` but read back as `lt.net.*`, so a `--network` rule was lost at a restart. They are now read under their own keys, and the old keys are migrated.
  - The start's `* proxy:` line shows the proxy in force after the stored settings.
- **How it was built:** two worktree subagents (the engine hold, the nox fixes) from a contracts commit. Their branches were reviewed and merged here. 6 new strings in all 35 tables (machine translations).
- **Validation:**
  - Tests: core lib 940 (the subagent's run, with three new hold tests); frontend 394; the nox suite with its new damaged-file, network-rule and held-start tests; Win32 150 on one thread.
  - Lint: clippy `-D warnings` for core, frontend, nox and Win32; GTK and macOS cross-linted.
  - End to end: nox in scratch folders with `lt.proxy_type=7` stored. It started held (no listen socket, the DHT not started); `setPreferences` lifted the hold and stored `lt.proxy_type=0` in place of the unreadable 7.
- **Not done:**
  - The status bar's look in the GUIs, by hand.
  - A remote GUI's OK without a change lifts nox's hold for that run only.
  - A GUI-hosted Web UI confirms through the host's Preferences path.
