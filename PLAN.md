# Rookparcel plan

Last updated: 2026-11-11_000208_1794351728

Read this file for current work. Load only the relevant sections of [detailed follow-ups](docs/BACKLOG.md) or [decisions D1-D68](docs/DECISIONS.md) when needed. Completed phase checklists and old reports are in the [history index](docs/archive/README.md); they are not the active plan.

## Vision

A lean recreation of µTorrent 2.2.1: native look, small single-file binaries, and 100,000 torrents with low idle CPU and a responsive UI on a modest PC. The engine is our Rust port of libtorrent-rasterbar 2.0.11, used as a reference and never linked.

| Binary | Platform | Role |
|---|---|---|
| `rookparcel` | Windows 10+ x64 | Native Win32 GUI |
| `rookparcel-nox` | All supported platforms, Docker | Daemon, native push API, Web UI, qBittorrent-compatible shim |
| `rookparcel-gtk` | Linux / BSD | GTK 4 GUI on the shared frontend model |
| `rookparcel-mac` | macOS 11+ (universal) | Native AppKit GUI on `frontend::app` (D61), shipped as `Rookparcel.app` |

Design: [ARCHITECTURE.md](docs/ARCHITECTURE.md), [PORTING.md](docs/PORTING.md), [WEBUI.md](docs/WEBUI.md), [INTEGRATIONS.md](docs/INTEGRATIONS.md). Release procedure: [RELEASING.md](docs/RELEASING.md).

## Current status

- **Native macOS GUI (D52, D61, [MACOS_GUI.md](docs/MACOS_GUI.md)):** all 12 slices merged (1.2.6-1.2.18) and validated on the unlocked VM with real input (slice L): every window and dialog; 100,000 demo torrents listed in 0.48 s, sorted in 47-66 ms, 194 MB. **1.2.19, the first release with it, is published (2026-11-06)**; `update.txt` says `stable=1.2.19`. Open (BACKLOG "macOS"): arm64 and Retina by hand (the owner has only the Intel VM).

- **Renamed PaTorrent to Rookparcel** (D59, 1.2.2; first released as 1.2.5): names, peer ID `-RP1014-`, remote protocol 16, a one-time takeover of PaTorrent's folders, registry, desktop entries and services. Open: upgrading a real PaTorrent 1.2 install on each platform.

- **Artwork (2026-11-05):** the Postal Rook won ([concepts](docs/art/rookparcel-candidates-2026-11-05/README.md), [small marks](docs/art/postal-rook-small-2026-11-05/README.md)); the [96 UI icon studies](docs/art/rookparcel-ui-2026-11-05/README.md) await selection and 16/20/24 px redraws.

- **Releases:** 1.0.1 to 1.2.46 ([notes](docs/releases/)); hand checks are below. 1.2.0 (2026-11-05) shipped profiles (D42), D50-D55 and the bug-hunt fixes.
- **Benchmarks:** [2026-11-09 results](docs/BENCHMARKS.md) for 1.2.29, `tools/bench`, with 1.0.1 and 1.1.40 in the same rounds: 10,000 torrents after restart use 67 MB (qBittorrent 612 MB); one-peer download 3,154 MiB/s, 0.99 CPU s/GiB, both as in 1.0.1. **Regression found there, fixed in 1.2.30:** idle CPU with 10,000 torrents was 0.33% in 1.2.29 (1.0.1: 0.07%), 0.12% with the fix; the Win32 process at 100,000 idles at 0.36-0.39% again (1.2.29: 0.8-1.0%). Causes: the Web UI's metadata pass for the private flag after each start, and LSD's walk over every torrent per tick. The site's page of that run is held back on the branch `site-benchmarks-1.2.29` (owner, 2026-11-10: benchmarks again after the release).
- CI, FreeBSD, Fuzz and the Release run were green for `v1.2.44` (recorded 2026-11-10, not a live check).
- Scale baseline on this development PC (2026-10-23): Win32 lists 100,000 torrents in 1.16 s, GUI-thread idle CPU 0.05-0.10%, working set 232 MB. Name sort 49 ms, Ctrl+A 4.7 ms, add 19 ms, remove one 14 ms. 1.2.29 on 2026-11-10: 1.37 s, GUI thread 0.16-0.18%, 264-269 MB, Name sort 32-36 ms (the published 1.0.1 the same day: 1.27 s, 0.05-0.18%, 250 MB, 49-53 ms). The low-end-machine gate remains open.
- The app icon is the Postal Rook (D60): drawn marks below 48 px, the generated stamp from 48 px (`assets/rookparcel-icons`).

## Next steps

**1.2.29, published 2026-11-09:** the theme pass and the Win32 dark mode (1.2.22-1.2.29, D63, D64), tested on the Xubuntu, Windows 11 and macOS VMs and this PC ([notes](docs/releases/1.2.29.md)); `update.txt` says `stable=1.2.29`; the wiki has the Dark-mode page. Open hand checks in BACKLOG "Themes" (a Windows 10 1809 VM, a phone and Safari for the Web UI themes) and "Files tab folder tree"; the nox Docker image's first push is still untested. Next on macOS: drop the legacy notification centre (BACKLOG "macOS" (A)).

**1.2.44, published 2026-11-10 (2026-11-10_210923_1794341363):** `update.txt` says `stable=1.2.44`; work goes on `main` (the branch `rename-rookparcel` was deleted after the release, 2026-11-10). Since 1.2.30: Win32 dark-list fixes (1.2.31-1.2.32); no full recheck after closing during a resume check (1.2.33, an engine fix for every platform); Help > Donate (1.2.34; `site/donate.html` is on `main`); Win32 notes that grow with their text (1.2.35); the Logger without tracker noise (1.2.36); lower GTK buttons (1.2.37); the Mac first-run guide and `site/install.html` (1.2.38, on `main`); Help > Statistics (1.2.39, D65, **remote protocol 18**); trackers for new downloads (1.2.40, D66); Run Program (1.2.41, D67); the VM runs and their fixes (1.2.42, 1.2.43: shell lines with codes refused, nox applies stored UPnP/DHT/LSD settings before the engine starts). A bug review of everything since 1.2.30 (1.2.44) fixed five things: the "added" program lost when a torrent finished at once, `-c` behind a shell option with a value, an empty %L, the host's Run Program lines readable in a hosted Web UI, GTK Statistics outliving the main window; what it found and left is in BACKLOG "Statistics, trackers and Run Program", the same-computer proxy case first (documented, the owner's choice). Every client ran the three new features with real transfers, and nox's local-only rule held from a second machine (changelog). Release text: `docs/releases/1.2.44.md` (protocol 18 means a 1.2.44 GUI and an older daemon do not mix). The site's news entry and the wiki are updated. Not done of RELEASING.md step 5 (changelog): the setup, zip, AppImages, Flatpaks, dmg and container image tried by hand on the VMs, and the update from 1.2.30 through Help > Check for Updates. Open: the 33 translations of 1.2.34-1.2.43 are machine-made; 1.2.44's GTK line and shared-frontend change have not run on the VMs; a try of 1.2.33's fix with the installed client (force a recheck of a big torrent, exit, start: only that one is hashed); a real session's Logger with private torrents (1.2.36); the owner's look at the GTK sizes on a real desktop (1.2.37); the dmg step and real Macs for the guide (1.2.38); the rest in BACKLOG "Themes", "Localisation", "GTK desktop checks", "macOS" and "Statistics, trackers and Run Program".

**1.2.45, published 2026-11-10 (2026-11-10_223619_1794346579, D68):** the default seeding goal no longer stops anything until "Limit the upload rate to [0: stop]" is ticked (as µTorrent ships it); a goal acts on a torrent once, so a torrent started again seeds on; the Logger says what was reached; settings from before are carried over once. The released `rookparcel-nox.exe` carried three scratch profiles over as designed (changelog). Open: the owner's installed copy updating through Help > Check for Updates, and the hand checks in BACKLOG "Seeding goals" (the pages in the three GUIs and the Web UI); the same RELEASING.md step 5 checks as 1.2.44 were skipped.

**1.2.46, published 2026-11-11 (2026-11-11_000208_1794351728):** resume data whose only fault is a file that is gone keeps the other pieces (the owner's 12 GB torrent was hashed at every start because Defender removes its `setup.exe`); the Logger leaves out "received N peers". `update.txt` should say `stable=1.2.46` after the Pages run. Open: the owner's installed copy updating and starting on its profile (one line "setup.exe is missing", no long check); BACKLOG "Disk and storage" has what the journal showed and did not explain (seeds that came out of a check empty) and the "Queued" status for a waiting check; the same RELEASING.md step 5 checks as 1.2.44 were skipped.

**Feature order chosen by the owner (2026-10-30_085652_1793347012):**

- **C, D.** RSS in hosted mode and Torznab search (D41, 1.1.12-1.1.19): done; hand checks in BACKLOG. No alternative speed limits in the Win32 GUI (owner, 2026-10-30).
- **E.** Profiles and portable mode (D42): shipped in **1.2.0** (D58), built and checked on VMs, GNOME, KDE, Flatpak, AppImages and a real Mac. Remaining hand checks: [BACKLOG](docs/BACKLOG.md#profiles-and-portable-mode-d42).
- **F.** Microsoft Store release: D46 approved 2026-11-04 (free MSIX, individual account, settings shared with the setup copy, all markets and 34 languages, the Postal Rook icon, D60); build per [STORE.md](docs/STORE.md).
- **G.** Smart disk I/O (D48, [DISKIO.md](docs/DISKIO.md)) shipped in 1.1.36-1.1.40; real-disk busy-throttle measurements remain in BACKLOG "Disk and storage". **Keep ping low** (D49, [LOWPING.md](docs/LOWPING.md)) shipped in 1.1.29-1.1.31; 1.1.32 passed a bloated-line test (~480 ms to 9 ms at 80-92% line use). Next: the owner's gaming line, a Mac run, and BACKLOG items including 2.2.1's "Stop transfers on user interaction".
- **H.** Approved 2026-11-04 (D56): 2.2.1's web search engines (empty default list), icon skins, and dark mode; research in [BACKLOG](docs/BACKLOG.md#themes-researched-2026-11-02_110639_1793613999-theme-pass-2026-11-09_152030_1794234030), each designed before code. **Theme pass (2026-11-09, D63):** Web UI themes (nine built-in, third-party CSS files, server and per-browser choice), the shared `frontend::palette`, GTK dark fixes. **Win32 dark mode (D64, 1.2.24-1.2.29):** built as [DARKMODE.md](docs/DARKMODE.md) designed it: System/Light/Dark on UI Settings, every window and dialog, light untouched; hand checks in BACKLOG "Themes". Icon skins and web search engines remain.
- **I.** The owner's six points (2026-11-04, [PROPOSALS.md](docs/PROPOSALS.md)): D50, D51, D53, D54 and D55 shipped in 1.2.0; D52 is the native macOS GUI (D61, shipped 1.2.19). [BUGHUNT-2026-11-05.md](docs/BUGHUNT-2026-11-05.md) records all 38 findings. **Open:** a real VPN run, a real first start through the guide, the GUI looks; the Win32 test binary's rare heap fault under parallel load (BACKLOG "Validation"; run its tests on one thread meanwhile).

0. **Owner hand-test findings (2026-10-27):** fixed in 1.0.2-1.0.8; re-test on a fresh Win32 build ([BACKLOG](docs/BACKLOG.md#owner-hand-test-findings-2026-10-27_225501_1793138101)).
1. Run the installer's wizard and the all-users UAC path once on a real PC, and register `RP` with the client-ID lists (item 14).
2. A full `tools/bench` run of 1.2.30 (released 2026-11-10 with the idle CPU fixes) for the site's tables; the benchmark page of the 1.2.29 run waits on the branch `site-benchmarks-1.2.29` until then. Rerun `tools/bench` before each release and on bare metal when one is free ([BACKLOG](docs/BACKLOG.md#validation-and-development-environment)).
3. Hand-check V1.1 (below, implemented) and keep the 1.0.x validation going. Refresh the hand-test set with the latest Win32 dialog changes, then check the remaining dialogs for clipped translations and layout problems in the new font.
4. Complete the manual checks in Road to 1.0 item 9, starting with Add Torrent from URL, Preferences and the changed Add dialog. Use [BACKLOG.md](docs/BACKLOG.md) for the relevant feature's exact gaps.
5. Finish the outstanding platform, scale and interop checks in items 5-8 and 11-13. Record what was actually run and what remains unverified.
6. **Security fixes** (done 2026-10-27): every audit finding is Fixed or Ruled out ([SECURITY.md](SECURITY.md); the full audit is the untracked `SECURITY-AUDIT.md`). Native review still wanted for `RemotePlaintext`, `ForwardRefused`, `PrefCurrentPassword`, `PrefNeedCurrentPassword`.
7. **Sonarr/Radarr/Prowlarr** ([INTEGRATIONS.md](docs/INTEGRATIONS.md) part 1): gaps 1-3 are fixed or decided (1.1.3-1.1.4); real apps passed against nox and the hosted Web UI (2026-10-29, `tools/arr-test`). Still open: a magnet grab with real peers, and "First" priority / Force Start. Torznab search is item D above.
8. Keep Windows, Linux, macOS and FreeBSD CI green. MSRV is 1.85; avoid let-chains. Before pushing code, use stable clippy and rustfmt, including the separate fuzz workspace. Interop reruns are needed after peer-wire, picker or disk changes.

## Road to 1.0

Original gate numbers are retained for references in source comments and release docs. "Implemented" does not close a pending hand check. D19-D23 define the approved scope. Since D34 the open checks are 1.0.x validation, not release blockers; 1.0.1 shipped with them open (1.0.0's tagged build failed).

| # | Gate | Remaining work |
|---|---|---|
| 1 | BitTorrent v2 and hybrid (BEP 52) | Implemented, including v2 magnets, dual-hash announces and creation. Linux plain/RC4 interop passed with qBittorrent and Deluge. Keep the regression and final interop checks in item 12. |
| 2 | GTK 4 GUI | All five slices implemented, built and tested in CI; the owner approved a Linux run. Complete targeted GTK checks in the backlog, icon validation and packaging checks. |
| 3 | HTTPS Web UI and GUI remote mode | Implemented. Hand-check certificate pinning, reconnect/authentication failures and both GUIs driving nox. |
| 4 | Localisation | 34 tables compiled in, language selection and RTL implemented. Check GTK in RTL, remaining long-text layouts and machine translations with native speakers when available. |
| 5 | 100,000-torrent scale gate (D21) | Measure on low-end Windows and Linux (proposal: 2 cores, 4 GB RAM, HDD); measure GTK, the Web UI's per-client model and idle CPU, and bulk removal of 1,000 torrents (last observed 0.5-1.4 s of engine alerts). Fix measured bottlenecks. |
| 6 | Windows release zip | Done: the release job builds and smoke-tests the zip and the NSIS installer (D31). 1.1.2's zip and setup passed on a clean Windows 11 VM (2026-10-29, `tools/windows-clean`: start, static CRT, upgrade, uninstall, all users). Remaining, needing a desktop: SmartScreen on a real download, the wizard and UAC pages, "Run Rookparcel" not elevated, drag and drop, double-click associations. |
| 7 | Linux packages | x86_64/aarch64 tarballs, GUI/nox AppImages and Flatpaks build. Run desktop AppImage and Flatpak checks, including schemas, icons, tray, HTTPS, associations and the Background portal. Flathub still needs screenshots, submission changes and the owner's exception text; see RELEASING.md. |
| 8 | macOS (D61) | The native AppKit GUI replaces the GTK bundle from 1.2.18: one universal `Rookparcel-<version>-macos.dmg`, macOS 11, validated on the Intel VM (slice L) and, as the 1.2.19 release dmg, on GitHub's arm64 runner. The owner has no real Mac: arm64 by hand and Retina stay unchecked; notifications show on the VM once allowed. |
| 9 | Manual GUI checks | Preferences (all pages, OK/Cancel/Apply, proxy and Advanced), Add (files/folders, priorities, labels, recent paths, real magnet OK/Cancel), Create, Properties (multi-select and DPI), RSS with a public feed, scheduler/hour boundary, Auto Shutdown with `ROOKPARCEL_POWER_DRY_RUN=1`, Windows integration on the real registry, moving completed data / Set Download Location across drives, seeding goals and .torrent folders. Check Pieces progress and newly resized/font-changed dialogs. |
| 10 | Icon integration and validation | Bold UI family (D27) integrated in both GUIs. Remaining: non-100% DPI, the Trackers tab with live trackers, GTK light/dark, tray/About and macOS Retina/light/dark menu bars. |
| 11 | Service integration | Run the nox Windows service under the real elevated SCM, including restart-on-failure and shutdown behavior. Install and exercise FreeBSD rc.d with root (only syntax-checked so far). |
| 12 | Final interop rerun | Use release builds: plain, RC4, TCP, uTP, uTP-only and super-seed with qBittorrent, Transmission and Deluge; v2/hybrid where supported. Transmission/Deluge uTP and super-seed switches remain unverified. Known Linux results: Transmission rejects v2-only and stalls on hybrid pad-file expectations; qBittorrent/Deluge v1/v2/hybrid plain/RC4 passed. |
| 13 | Web UI on a real phone | Android/iOS: detail/action sheets, drawer, sort, long-press multi-select, Back behavior, Properties and reconnection. Also verify desktop keyboard, selection, columns, Preferences and dark mode on the real engine. |
| 14 | Release engineering | rc.1 and 1.0.1 (D34) are out; `docs/releases/<version>.md` is the release text. Keep the parser/security review (SECURITY-REVIEW.md) at each release. Register `RP` (D59) with libtorrent, Transmission, WebTorrent and the BEP 20 list. |

## Decisions and open questions

The approved decisions D1-D40 remain active in [DECISIONS.md](docs/DECISIONS.md). Keep the one-thread mio engine, OS TLS, lazy shared-store persistence, minimal dependencies, static portable Windows binary (an installer beside the zip, D31) and faithful 2.2.1 layout. Scale takes priority over empty-session memory. The PolyForm Noncommercial licence, Rookparcel identity (D59) and honest peer ID remain fixed. New dependencies need a written justification and changelog entry.

Q1-Q15 are resolved; their answers and the original Q10 interop setup are in the [plan snapshot](docs/archive/PLAN-2026-10-23.md). Current testing constraints and setup notes are in [BACKLOG.md](docs/BACKLOG.md#validation-and-development-environment).

- UI artwork (D27-D30): the bold family passed its revision 6 review (131/131) and is integrated (gate 10); revisions 3-6 and their reviews are in the [changelog archive](docs/archive/README.md). Optional: hand-drawn 20 px globe and pylon, dedicated 48/64 px drawings, and a web-ui globe matching the network globe.
- Compare Properties > Advanced and `rss.*` settings with the owner's real µTorrent 2.2.1 in `z_exclude_uTorrent/` (gitignored). Its Add dialog was inspected via scripted captures/control dumps; the remaining wording and behavior need comparison. The owner captured all 15 Preferences pages (General through Advanced > Run Program) as `z_exclude_uTorrent/screenshots/*.webp` on 2026-10-26: compare each page of our Preferences against them (Road to 1.0 item 9).
- D46 and D50-D68 are settled; no decision is proposed.
- Decide the Flathub permission/submission approach (item 7); the recorded exception policy requires the owner's own text.


## V1.1

Released in 1.1.0 (D35: the name search, and importers from qBittorrent / µTorrent with D36). Hand checks are still open: the Win32 box at 100-200% DPI and its keys, the import dialogs in both GUIs, the µTorrent `started` mapping, and native review of the new strings. See [BACKLOG.md](docs/BACKLOG.md#v11-importers-search-and-hooks).

## V2.0 goals

Owner priorities recorded 2026-10-21 and reaffirmed 2026-10-23. Goals 1 and 3 are designed in [PROFILES.md](docs/PROFILES.md) and planned in [PROFILES_PLAN.md](docs/PROFILES_PLAN.md) (D42, approved 2026-11-03); step E above.

1. Profiles: separate settings, torrents and state, simultaneous instances, per-profile single-instance handling, ports and nox services.
2. Downloadable binaries on GitHub Releases: build on the tagged packaging workflow already started for 1.0 (items 6-8).
3. Portable mode on every platform: settings, state and downloads relative to the portable folder. Research the model first and revisit D7 together with profiles.
4. A multilingual wiki/user guide in the D24 languages. The English guide is done (2026-11-04_211029_1793823029): 28 pages on the [GitHub wiki](https://github.com/Balls0fSteel/Rookparcel/wiki), source in `docs/wiki` (copy it to `Rookparcel.wiki.git` after edits; `docs/wiki/BRIEF.md` has the conventions). Open: translations; F1 could open the wiki (`help::WIKI_URL` is `None`); keep pages current as features change.
5. Compiled-in Rust plugin hooks (moved from V1.1, D35), once a completion action, search source or custom column needs one.

## Future improvements

- BEP 44 DHT items/ed25519, BEP 51 sample-infohash querying and BEP 55 holepunch after 1.0 unless cheap. Detailed remaining engine/test gaps are in the backlog.
- Split `core` only if compile times justify it; consider io_uring disk/network on Linux and IOCP file I/O on Windows.
- Continuous work: meaningful scale regression baselines, parser fuzzing, per-release security review and a wider interop matrix (including rTorrent and µTorrent).
- Theming (owner's wish: included and third-party themes, no extra resource use): the Web UI has them (D63) and the Windows program has dark mode (D64, [DARKMODE.md](docs/DARKMODE.md)); icon skins and the open checks are in [BACKLOG](docs/BACKLOG.md#themes-researched-2026-11-02_110639_1793613999-theme-pass-2026-11-09_152030_1794234030).

## Known TODOs

Use the topic headings in [BACKLOG.md](docs/BACKLOG.md) for unresolved implementation gaps, behavioral differences and specific unverified checks. Update the relevant topic when work lands; summarize its outcome in the changelog. Historical completed milestones belong in the [archive](docs/archive/README.md), not in this plan.
