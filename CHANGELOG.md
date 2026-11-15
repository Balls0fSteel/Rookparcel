# Changelog

Recent completed changes, issues and decisions. Timestamps: `YYYY-MM-DD_HHMMSS_<unix epoch>` (Europe/Budapest).

Earlier history: [archive index](docs/archive/README.md), including all 148 original entries from 2026-10-15 through 2026-10-23. Open archived entries only for a specific historical question. Current work: [PLAN.md](PLAN.md).

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

## 2026-11-15_143820_1794749900: CI red on 1.2.56: two tests depended on the version (tests only)

- **What failed** (CI on all three systems, FreeBSD): `guard::tests::a_different_reading_of_the_stamped_settings_holds_until_confirmed` used `1.2.56` as "an older version", which the bump made the current one. On macOS, which runs with `--no-fail-fast`, `the_newest_fixture_is_this_builds_format` also failed: the fixture generator built text settings from their defaults, and `user_agent`'s default names the version, so every bump looked like a new format.
- **Cause:** the tests ran before the bump to 1.2.56, not after.
- **Fixed:**
  - The test's older version is `1.0.0`.
  - Text rows get `fixture-<name>`, and the test fails if the snapshot names the version. `1.2.56.ini` is added; `1.2.55.ini` stays as that version's file.
- **Validation:** `cargo test --workspace --no-fail-fast` green (40 test binaries); the guard and fixture tests green again under a scratch version of 1.2.99.

## 2026-11-15_141701_1794748621: Version 1.2.56: the start compares the settings with how the last version read them (D70, part 2)

- **The stamp:** every save writes `check.version` and `check.guard` (`guard::summary`). The summary is a frozen `v1` text of the proxy (type, host, port, scopes), the network rule, the listen interfaces, the encryption and the IP filter. It holds no user name or password.
  - `Prefs::save` adds the stamp once `guard::on_start` has turned it on, so a remote window's file and a helper's file are never stamped. nox adds it to `webui.json` (`guard::stamp_lines`).
- **At start** (`guard::on_start`, before anything writes):
  - After a version change, the file is first copied to `<name>.<old>.bak`. One copy per version, the newest three kept, and the version from the file is sanitised so it never names a path.
  - The stamp is then compared with this build's reading. Each differing field goes to the Logger.
  - Losing a proxy, a network rule or the listen interfaces holds the transfers. The file then keeps the old stamp until the user confirms, so the next start asks again.
  - An IP filter or encryption difference is only logged.
- **nox:** the check runs on `webui.json` before the engine starts (the backup, the hold) and again for the Web UI's Logger. A preference change confirms.
- 3 new strings in all 35 tables (machine translations). The wiki's Updating page has a section on it ("not in a published release yet").
- **Validation:**
  - Tests: frontend 397 (the stamp, the backups, the summary); the fixture tests unchanged; nox and Win32 suites green; clippy `-D warnings` and the GTK and macOS cross-lint clean.
  - End to end with the real nox in scratch folders:
    - Run 1 stamped `check.version=1.2.56` with a SOCKS5 proxy.
    - Run 2, with the stamp made to look written by 1.2.40 and the proxy line read as none, kept `webui.json.1.2.40.bak`, logged `proxy: socks5 10.8.0.1:1080 peers trackers names → none` and started held, with nothing listening. The file kept the old stamp while held.
    - `setPreferences` lifted the hold and stamped 1.2.56's reading.
- **Not done:** the GUIs by hand (BACKLOG "Settings guard").

## 2026-11-15_140719_1794748039: D70 layer 3: the settings fixture test (tests only, no version)

- **`crates/frontend/tests/settings_fixtures.rs`:** builds the preferences with every engine row and GUI key away from its default. Then three tests:
  - **Every setting survives the file.** A row that comes back at its default fails.
  - **The newest fixture is this build's format.** A changed format fails until `ROOKPARCEL_WRITE_FIXTURE=1` adds `<version>.ini` with its `.expect`. A version that leaves the settings alone costs nothing.
  - **Older settings read the same.** Every fixture must read as its own version read it: the rows, the engine's fields by what they mean (`proxy_type=Socks5`) and the GUI keys. Only what that version read away from its defaults is compared, so a later build may still change a default.
- **Fixtures:** `1.2.55` (147 rows, every GUI key that survives a change, the folders, the IP filter, Run Program) and a hand-written `legacy-keys` for the keys the existing migrations handle (`net.port`, a seeding goal from before 1.2.45, D68).
- **Shown to catch the bug class** with two scratch edits, reverted afterwards:
  - SOCKS4 and SOCKS5 renumbered in both directions, so the stored "2" is unchanged but read as SOCKS4: `field proxy_type: then "Socks5", now Some("Socks4")`.
  - LSD's default changed: the format test asks for a new fixture. A value stored before the change still compares, with no false alarm.
- **RELEASING.md** step 3 says what to do when the test fails: add the fixture in the change set itself, write a migration, and never edit an old fixture.

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
