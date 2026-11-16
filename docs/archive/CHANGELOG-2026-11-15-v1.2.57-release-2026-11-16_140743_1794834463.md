# Changelog archive: 2026-11-15, version 1.2.57 and its release

2 entries moved verbatim from the root changelog on 2026-11-16_140743_1794834463 to keep it under budget.

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
