# Changelog archive: 2026-11-15, version 1.2.56 and the settings fixture test

3 entries moved verbatim from the root changelog on 2026-11-16_122332_1794828212 to keep it under budget.

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
