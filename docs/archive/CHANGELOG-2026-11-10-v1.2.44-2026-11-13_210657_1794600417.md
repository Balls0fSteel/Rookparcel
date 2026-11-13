# Changelog archive: 2026-11-10, 1.2.44

3 entries moved verbatim from the root changelog on 2026-11-13_210657_1794600417 to keep it under budget.

## 2026-11-10_210923_1794341363: Release 1.2.44 published

- **Done:** `main` at `8d04e05d` with CI, FreeBSD and Fuzz green; tag `v1.2.44`; the Release run built 13 files and `SHA256SUMS` and both container images (`:1.2.44`, `:latest`); published as the latest release (2026-11-10 20:09 UTC). Then the site's news entry pushed to `main` and `docs/wiki` copied to the wiki (seven pages changed).
- **Checked on the draft:** every file against `SHA256SUMS`; the zip's `rookparcel-nox.exe --version`; `tools/nox-e2e/e2e.py` with the release `rookparcel-nox.exe` and the real https list: 31 of 31. Its first run said 26 of 27: the script read the Logger the moment the program's file appeared, before the "ended" line of the next alert; it waits for the line now (a race in the script, not in the program).
- **Skipped of RELEASING.md step 5:** the setup and the zip on a clean Windows VM, the wizard and UAC, the update from 1.2.30 through Help > Check for Updates, the tarballs, AppImages and Flatpaks on a Linux desktop, the dmg on the Mac VM (the run's own start-up checks passed), the container image pulled and run.
- The two oldest entries moved to the archive.

## 2026-11-10_204108_1794339668: Release 1.2.44 prepared

- **Asked by the owner:** prepare, commit and release, with the notes and the website's news.
- **Written:** `docs/releases/1.2.44.md` (everything since 1.2.30: Statistics, trackers for new downloads, Run Program, the recheck fix, nox's stored settings, the quieter Logger, Donate, the Mac guide, the Win32 and GTK looks; remote protocol 18 means both ends update; the limits the review left). `docs/releases/1.2.38.md` removed: 1.2.38 was never tagged and its text is in the new file. The Flatpak metainfo's first release is 1.2.44 in place of 1.2.38.
- **Website:** the 1.2.38 news entry, never deployed, is taken off the branch so `main` can be pushed; the 1.2.44 entry is a commit of its own, pushed after publishing (RELEASING.md step 2).
- **Order:** `main` fast-forwarded to this branch, CI and FreeBSD green, tag `v1.2.44`, the Release run's draft, published, then the news and the wiki. What ran and what was skipped of step 5 is in the entry that follows the publication.

## 2026-11-10_202226_1794338546: Version 1.2.44: a bug review of everything since 1.2.30; five fixes

- **Asked by the owner:** look over the changes waiting for the release and fix the bugs found. `git diff v1.2.30..HEAD` read in five parts: core here, and four read-only agents for the frontend, nox and the Web UI, Win32, and GTK with the Mac crate.
- **Run Program, fixed (`core::run_program`):** a torrent that finished within one status update of being added (a small one, or one whose data was there) lost its "added" program: one waiting slot per torrent held the later event only. Both wait now. `shell_reads_codes` missed `-c` behind an option that takes a value (`bash -o pipefail -c "... %N"`, `--rcfile x -c`), fish's `--command=...`, `-C` and `--init-command`, and PowerShell's `-File` behind a `-Command` (part of the command there).
- **%L, fixed (frontend `app/alerts.rs`, Win32 `app.rs`):** the "added" program got an empty label for a .torrent added with one in the Add dialog, and for imports: the runner read the list's label on the torrent's first status update, a few lines before that update restored it from the preferences. The label is restored first.
- **Hosted Web UI, fixed:** a loopback session read the host's Run Program lines (`/pa/prefs` and the shim's `autorun_*`), which no browser may change there; hidden like for a remote session.
- **Small:** a fetched tracker list with more valid lines than the 200 kept was weighed against the kept ones only and could be refused as "not a list"; GTK's Statistics window closes with the main window (`destroy_with_parent`).
- **Looked at, left:** a reviewer's guess that the dark list paint (1.2.31) leaves lists without `LVS_EX_DOUBLEBUFFER` black was pictured before and after a fix on the hidden desktop (RSS filters, empty): identical and right, nothing changed. Refusing a remote request that names a Run Program line with its stored value was tried and taken back: two tests pin "the same value is no change" as chosen. The reviewers' other findings are in BACKLOG "Statistics, trackers and Run Program" (found 2026-11-10 in review).
- **Documented (owner's choice, 2026-11-10_203900):** a proxy or tunnel on the server itself that adds no forwarding header makes every visitor local, Run Program's gate included; WEBUI.md "Behind a reverse proxy", the wiki's Preferences reference and nox page say so and give nginx's `X-Forwarded-For` line. No code.
- **Validation:** tests on Windows: core lib (931), frontend (380 + 4), nox (all suites, 211), Win32 (140, one thread); `cargo check` of the workspace with the GTK and Mac crates. **Not done:** the GTK and Mac changes (one builder line; the shared `alerts.rs`) did not run on the VMs; no real transfer was repeated; the two Web UI changes ran in tests only.
