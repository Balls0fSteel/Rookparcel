# Changelog

Recent completed changes, issues and decisions. Timestamps: `YYYY-MM-DD_HHMMSS_<unix epoch>` (Europe/Budapest).

Earlier history: [archive index](docs/archive/README.md), including all 148 original entries from 2026-10-15 through 2026-10-23. Open archived entries only for a specific historical question. Current work: [PLAN.md](PLAN.md).

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

## 2026-11-15_132051_1794745251: Release 1.2.54 published; the site's news and the wiki pushed

- **Published by the owner** (`gh release edit v1.2.54 --draft=false --latest`, 2026-11-15 13:18 Budapest; this session's permission check had refused the command). GitHub's latest release is `v1.2.54`.
- **Then, on the owner's "do the rest":** `main` fast-forwarded to the publish branch and pushed (`03c6f3ed..b7a841fe`: the README and wiki version lines, the news entry dated 2026-11-15); the branch deleted. The GitHub wiki got the seven changed pages (`a244cf5`), after a page-by-page comparison showed every wiki page still equal to the 2026-11-13 sync (`1f4ccedb`).
- **Checked live:** the three Pages runs green; `update.txt` says `stable=1.2.54`; the news page, `news.atom` and the home page's latest-news line have 1.2.54; the download page links the 1.2.54 setup; the wiki's Home says 1.2.54 and Locking Rookparcel "New in 1.2.54.".
- **Still open:** the rest of RELEASING.md step 5 by hand (the GUI, the setup and the update from an installed 1.2.46, the Linux and Mac files, the container image) and the UI lock's hand checks (BACKLOG "UI lock").

## 2026-11-14_235326_1794696806: Release 1.2.54 tagged; the draft is built, not published

- **On the owner's word ("Push and tag"):** `main` pushed (`9fb7e42d..03c6f3ed`); CI, FreeBSD and Fuzz green on `03c6f3ed`; tag `v1.2.54` pushed.
- **Release run [43649073477](https://github.com/Balls0fSteel/Rookparcel/actions/runs/43649073477):** the first attempt failed in `flatpak (x86_64)` only: inside Flathub's `gnome-51` builder container, flatpak-builder's update of `org.gnome.Sdk` (and the GL and locale extensions) failed with "Invalid cross-device link", then `xvfb-run` segfaulted (exit 139). The aarch64 leg passed. Rerunning the failed job passed (attempt 2), so it was the runner, not the build; if it comes back, look at the builder image and the restored flatpak-builder cache.
- **Draft:** the same 14 files as 1.2.46 under the new version; its text is `docs/releases/1.2.54.md`. `ghcr.io/balls0fsteel/rookparcel-nox:1.2.54` and `:latest` are pushed (they exist before publishing, RELEASING.md "Container image").
- **Checked on the draft (2026-11-15, on the owner's "Check files first"):** the Windows zip downloaded matches `SHA256SUMS`, and GitHub's own SHA-256 digests of all 13 files match the list; the zip holds `rookparcel.exe`, `rookparcel-nox.exe`, `LICENSE.md` and `THIRD-PARTY-NOTICES.md`; both exes say 1.2.54 in their version resources, the manifest 1.2.54.0; `rookparcel-nox.exe --version` prints `rookparcel-nox 1.2.54` (no `-dev`).
- **Not done:** the rest of RELEASING.md step 5 (the GUI, the setup, the Linux and Mac files and the image were not started); publishing (the session's permission check refused `gh release edit --draft=false`); then `main` fast-forwarded to `publish-1.2.54` (its news entry now dated 2026-11-15) with the wiki pushed. The tagged Flatpak metainfo says 2026-11-14.

## 2026-11-14_163756_1794670676: Version 1.2.54: the release review's two findings; release 1.2.54 prepared

- **Asked for:** write the docs and prepare the release. The next release covers the UI lock (D69, 1.2.47-1.2.50), clippy 1.100 (1.2.51), the bug hunt's inherited quirks (1.2.52) and the GTK quit signals (1.2.53). The security review of `v1.2.46..HEAD` (Road to 1.0 item 14; [SECURITY-REVIEW.md](docs/SECURITY-REVIEW.md) "Release 1.2.54", code read by a subagent, both findings checked here) found two things, fixed here, so the release is **1.2.54**:
  - **Set PIN replaced an existing PIN** without the current one; on Windows only the greyed menu item stood in the way of a posted `WM_COMMAND`. `ui_lock::submit` now treats Set with a PIN as needing the current one ("Wrong PIN.", counted by the guard), and Win32's `on_command` opens Set, Change and Remove only in the state their menu items are enabled in, as the Mac's `pin_command` did. Test: `the_forms_do_what_their_ok_does`.
  - **1.2.52's web seed rule reached v1 and hybrid torrents:** a `files` list of one file and a pad (never made by libtorrent, possible from other tools) became a single file, so its seed was asked for the directory URL and a bad piece banned it at once. `serves_one_file` and `WebPeer::new` take `v1` now; the pad counts as no second file only in a v2-only torrent. Test: `a_tail_pad_is_a_second_file_only_with_v1_metadata`, which fails with the 1.2.52 rule ("unknown path /web_seed"). PORTING's row says so.
  - Left open (BACKLOG "UI lock"): GTK's window actions have no central lock check and are exported on the session bus (not run); a second forwarded `--reset-lock` on Windows stacks a question.
- **Docs written:** `docs/releases/1.2.54.md` (the lock, the v2-only web seed fix, the second SIGTERM, Minimize to tray for a window shown late, the LSD floor, the stricter `\u`; the remote protocol is still 18; 1.2.46 and older ignore the lock; the hand checks not done and the lock's known faults); the Flatpak metainfo's first `<release>` (1.2.54, 2026-11-14); the wiki's lock marks name 1.2.54 instead of 1.2.50, never published; Troubleshooting's "Closing Rookparcel takes a long time" says what a second signal does on Linux now.
- **Branch `publish-1.2.54`** (not on `main`; fast-forward `main` to it after publishing, RELEASING.md step 2): the site news entry dated 2026-11-14, the README's and the wiki's version lines at 1.2.54 and the "not in a published release yet" marks dropped. If the release goes out on another day, change the entry's and the metainfo's date.
- **Validation (Rust 1.100, CI's stable):** rustfmt and clippy `--all-targets -D warnings` clean for the workspace and the fuzz workspace; `cargo test --workspace` 1,874 passed, none failed (the Win32 crate on one thread); the same run on 1.2.53 before the fixes passed 1,873. CI has not run 1.2.52-1.2.54 yet (not pushed); `9fb7e42d` (1.2.51) was green on CI, FreeBSD and Fuzz.
- **Not done:** `tools/bench` (no hot-path change since 1.2.46's run), an interop rerun (no peer-wire change), RELEASING.md step 5 (needs the draft); the GTK and Mac crates were not linted again (their code did not change).
- **Plan and backlog:** the release's status in PLAN; the µTorrent 2.2.1 comparison's detail moved to BACKLOG "Win32 layout and dialogs" (PLAN 17.5 KB to 16.9 KB).
