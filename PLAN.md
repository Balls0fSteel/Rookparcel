# Rookparcel plan

Last updated: 2026-11-25_092421_1795595061

Read this file for current work. Load only the relevant sections of [detailed follow-ups](docs/BACKLOG.md) or [decisions D1-D80](docs/DECISIONS.md) when needed. Completed phase checklists and old reports are in the [history index](docs/archive/README.md); they are not the active plan.

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

- **1.2.80 (published 2026-11-25):** the desktop entry says `Version=1.0` for the AppImage catalog's older validator (`/retest` posted on AppImage/appimage.github.io#12506). **1.2.79 (published 2026-11-25; 1.2.78 not released; RELEASING step 5 done on the three VMs, all passed):** a Gaming page in Preferences above Advanced (Keep ping low and smart disk I/O; D80), gentle checks off by default; checked on Win32, the Web UI and the GTK and macOS VMs. **1.2.77 (published 2026-11-23):** adding no longer waits for a settings flush per torrent (the 3x slower add since 1.2.57) and its review's fix. **1.2.76 (published 2026-11-22):** the release review's fixes (banned peers no longer fill the peer list). **1.2.70-1.2.75:** the storage hold (D79), the 2026-11-21 bug hunt's fixes, remote `PROTOCOL` 21 (details in the release notes).
- **Settings guard (D70, 1.2.55-1.2.57):** loss paths closed, the engine's hold (fetches too), the fixture test, a stamp compared at each start with backups; 1.2.57 has the release review's fixes. Hand checks: BACKLOG "Settings guard".
- **Native macOS GUI (D52, D61, [MACOS_GUI.md](docs/MACOS_GUI.md)):** since 1.2.19; arm64 and Retina by hand are open (BACKLOG "macOS").
- **Renamed PaTorrent to Rookparcel** (D59, 1.2.5). Open: upgrading a real PaTorrent 1.2 install on each platform.
- **Artwork:** the Postal Rook won ([small marks](docs/art/postal-rook-small-2026-11-05/README.md)); the [96 UI icon studies](docs/art/rookparcel-ui-2026-11-05/README.md) await selection and 16/20/24 px redraws.
- **Releases:** 1.0.1 to 1.2.80 published ([notes](docs/releases/)); the feature history is in the notes and CHANGELOG. RELEASING.md step 5 by hand is open for 1.2.63-1.2.69. Open in BACKLOG: those features' sections, "Security review 1.2.63", "Files tab sort and the unfocused selection" (the owner's retest), "Trackers and Peers tab sort", a macOS swarm-test flake ("Validation").
- **Version texts:** the README, `docs/wiki` (pushed to the GitHub wiki) and the site's news are at 1.2.80. At each release: update the version lines in `docs/wiki/Home.md`, `Installing-Rookparcel.md` and their fr/es/de/hu copies (`grep -rl 1.2.80 docs/wiki`). A small nox add race is in BACKLOG "Remote mode and Web UI".
- **UI lock (D69, released in 1.2.54; D71's off switch in 1.2.63):** a 4-6 digit PIN hides all three GUIs' windows in the tray; Ctrl or Option at start, or `--reset-lock`, removes a forgotten one; wiki page `Locking-Rookparcel`. Open: BACKLOG "UI lock".
- **Benchmarks:** the website shows [the version history](docs/BENCHMARKS.md) on a Hetzner CCX23 (`launch-hetzner.ps1 -Published`): 10,000 torrents use 67.5 MB after restart (qBittorrent 612 MB), idle 0.03 %; adding 10,000 takes 1.59 s in 1.2.77 (1.2.57-1.2.76 were 3x slower, a flushed settings write per add; qBittorrent 2.16 s). Win32 lists 100,000 in 1.37-1.49 s (2026-11-11). BACKLOG "Disk and storage" has the SHA-1 suspect on CPUs without SHA-NI and an unreproduced 10,000-listing delay.
- CI, FreeBSD and Fuzz green for `2ae717e3` (1.2.75, 2026-11-22, first try on every platform; 1.2.74 green too).
- Scale baselines on this PC: [plan snapshot](docs/archive/PLAN-2026-11-13.md), [BENCHMARKS.md](docs/BENCHMARKS.md); the low-end-machine gate remains open.
- The app icon is the Postal Rook (D60): drawn marks below 48 px, the generated stamp from 48 px (`assets/rookparcel-icons`).

## Next steps

**Bug hunt 2026-11-21** ([BUGHUNT-2026-11-21.md](docs/BUGHUNT-2026-11-21.md)): all fixed (1.2.71, 1.2.72), GTK and macOS ones passed on the VMs; follow-ups decided, A-H fixed in 1.2.73.

**Open from releases 1.2.29-1.2.54** (notes in `docs/releases/`, full paragraphs in the [2026-11-13 snapshot](docs/archive/PLAN-2026-11-13.md)): RELEASING.md step 5 by hand (setup, zip, AppImages, Flatpaks, dmg and image on the VMs; updates from 1.2.30 and from an installed 1.2.46-1.2.54, the owner's copy starting with one "setup.exe is missing" line and no long check; the UI lock); the machine translations of 1.2.34-1.2.43 and D69; 1.2.44's GTK line and frontend change on the VMs; 1.2.33's fix with the installed client; a real Logger with private torrents; the owner's look at the GTK sizes; the dmg step and real Macs for the first-run guide; the Docker image's first push; then dropping the Mac's legacy notification centre. BACKLOG: "Themes", "Files tab folder tree", "Localisation", "GTK desktop checks", "macOS", "Statistics, trackers and Run Program", "Seeding goals", "Disk and storage", "UI lock".

**Feature order chosen by the owner (2026-10-30_085652_1793347012):**

- **C-E** done: RSS in hosted mode and Torznab search (D41), no alternative speed limits in Win32 (owner); profiles and portable mode (D42, 1.2.0). Hand checks in BACKLOG ([profiles](docs/BACKLOG.md#profiles-and-portable-mode-d42)).
- **F.** Microsoft Store release (D46, approved 2026-11-04): build per [STORE.md](docs/STORE.md).
- **G.** Smart disk I/O (D48) and Keep ping low (D49) shipped in 1.1.29-1.1.40. Open: real-disk busy-throttle measurements (BACKLOG "Disk and storage"), the owner's gaming line, a Mac run, 2.2.1's "Stop transfers on user interaction".
- **H.** D56: the theme pass (D63) and Win32 dark mode (D64) shipped by 1.2.29, hand checks in [BACKLOG "Themes"](docs/BACKLOG.md#themes-researched-2026-11-02_110639_1793613999-theme-pass-2026-11-09_152030_1794234030). Icon skins and web search engines remain, each designed before code.
- **I.** The owner's six points (D50-D55, [PROPOSALS.md](docs/PROPOSALS.md)) shipped; the 2026-11-05 hunt's 38 closed. **Open:** a real VPN run, a real first start through the guide, the GUI looks; the Win32 test binary's rare heap fault under parallel load (BACKLOG "Validation"; run its tests on one thread).

0. **Owner hand-test findings (2026-10-27):** fixed in 1.0.2-1.0.8; re-test on a fresh Win32 build ([BACKLOG](docs/BACKLOG.md#owner-hand-test-findings-2026-10-27_225501_1793138101)).
1. Run the installer's wizard and the all-users UAC path once on a real PC, and register `RP` with the client-ID lists (item 14).
2. Rerun `tools/bench` before each release and on bare metal when one is free ([BACKLOG](docs/BACKLOG.md#validation-and-development-environment)).
3. Hand-check V1.1 and keep the 1.0.x validation going. Refresh the hand-test set with the latest Win32 dialog changes, then check the remaining dialogs for clipped translations and layout problems in the new font.
4. Complete the manual checks in Road to 1.0 item 9, starting with Add Torrent from URL, Preferences and the changed Add dialog. Use [BACKLOG.md](docs/BACKLOG.md) for the relevant feature's exact gaps.
5. Finish the outstanding platform, scale and interop checks in items 5-8 and 11-13. Record what was actually run and what remains unverified.
6. **Security:** every audit finding is Fixed or Ruled out ([SECURITY.md](SECURITY.md)). Native review still wanted for `RemotePlaintext`, `ForwardRefused`, `PrefCurrentPassword`, `PrefNeedCurrentPassword`.
7. **Sonarr/Radarr/Prowlarr** ([INTEGRATIONS.md](docs/INTEGRATIONS.md)): real apps passed against nox and the hosted Web UI (`tools/arr-test`). Still open: a magnet grab with real peers, and "First" priority / Force Start.
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
| 6 | Windows release zip | Done: the release job smoke-tests the zip and the installer (D31); both passed on a clean Windows 11 VM (2026-10-29, `tools/windows-clean`). Remaining, needing a desktop: SmartScreen on a real download, the wizard and UAC pages, "Run Rookparcel" not elevated, drag and drop, double-click associations. |
| 7 | Linux packages | x86_64/aarch64 tarballs, GUI/nox AppImages and Flatpaks build. Run desktop AppImage and Flatpak checks, including schemas, icons, tray, HTTPS, associations and the Background portal. Flathub still needs screenshots, submission changes and the owner's exception text; see RELEASING.md. |
| 8 | macOS (D61) | The native AppKit GUI (one universal dmg, macOS 11) since 1.2.18, validated on the Intel VM and GitHub's arm64 runner. The owner has no real Mac: arm64 by hand and Retina stay unchecked. |
| 9 | Manual GUI checks | Preferences (all pages, OK/Cancel/Apply, proxy and Advanced), Add (files/folders, priorities, labels, recent paths, real magnet OK/Cancel), Create, Properties (multi-select and DPI), RSS with a public feed, scheduler/hour boundary, Auto Shutdown with `ROOKPARCEL_POWER_DRY_RUN=1`, Windows integration on the real registry, moving completed data / Set Download Location across drives, seeding goals and .torrent folders. Check Pieces progress and newly resized/font-changed dialogs. |
| 10 | Icon integration and validation | Bold UI family (D27) integrated in both GUIs. Remaining: non-100% DPI, the Trackers tab with live trackers, GTK light/dark, tray/About and macOS Retina/light/dark menu bars. |
| 11 | Service integration | Run the nox Windows service under the real elevated SCM, including restart-on-failure and shutdown behavior. Install and exercise FreeBSD rc.d with root (only syntax-checked so far). |
| 12 | Final interop rerun | Use release builds: plain, RC4, TCP, uTP, uTP-only and super-seed with qBittorrent, Transmission and Deluge; v2/hybrid where supported. Transmission/Deluge uTP and super-seed switches remain unverified. Known Linux results: Transmission rejects v2-only and stalls on hybrid pad-file expectations; qBittorrent/Deluge v1/v2/hybrid plain/RC4 passed. |
| 13 | Web UI on a real phone | Android/iOS: detail/action sheets, drawer, sort, long-press multi-select, Back behavior, Properties and reconnection. Also verify desktop keyboard, selection, columns, Preferences and dark mode on the real engine. |
| 14 | Release engineering | `docs/releases/<version>.md` is the release text. Keep the parser/security review (SECURITY-REVIEW.md) at each release. Register `RP` (D59) with libtorrent, Transmission, WebTorrent and the BEP 20 list. |

## Decisions and open questions

The approved decisions D1-D40 remain active in [DECISIONS.md](docs/DECISIONS.md). Keep the one-thread mio engine, OS TLS, lazy shared-store persistence, minimal dependencies, static portable Windows binary (an installer beside the zip, D31) and faithful 2.2.1 layout. Scale takes priority over empty-session memory. The PolyForm Noncommercial licence, Rookparcel identity (D59) and honest peer ID remain fixed. New dependencies need a written justification and changelog entry.

Q1-Q15 are resolved; their answers and the original Q10 interop setup are in the [plan snapshot](docs/archive/PLAN-2026-10-23.md). Current testing constraints and setup notes are in [BACKLOG.md](docs/BACKLOG.md#validation-and-development-environment).

- UI artwork (D27-D30): the bold family is integrated (gate 10). Optional: hand-drawn 20 px globe and pylon, dedicated 48/64 px drawings, and a web-ui globe matching the network globe.
- Compare Properties > Advanced, `rss.*`, the Add dialog's wording and all 15 Preferences pages with the owner's µTorrent 2.2.1 captures in `z_exclude_uTorrent/` (BACKLOG "Win32 layout and dialogs"; Road to 1.0 item 9).
- D46 and D50-D78 are settled; no decision is proposed.
- Decide the Flathub permission/submission approach (item 7); the recorded exception policy requires the owner's own text.

## V1.1

Released in 1.1.0 (D35: the name search, and importers from qBittorrent / µTorrent with D36). Hand checks are still open: the Win32 box at 100-200% DPI and its keys, the import dialogs in both GUIs, the µTorrent `started` mapping, and native review of the new strings. See [BACKLOG.md](docs/BACKLOG.md#v11-importers-search-and-hooks).

## V2.0 goals

Owner priorities recorded 2026-10-21 and reaffirmed 2026-10-23. Goals 1-3 are done: profiles and portable mode on every platform (D42, [PROFILES.md](docs/PROFILES.md), shipped in 1.2.0) and binaries on GitHub Releases.

4. A multilingual wiki/user guide in the D24 languages. The English guide is done (2026-11-04_211029_1793823029): 28 pages on the [GitHub wiki](https://github.com/Balls0fSteel/Rookparcel/wiki), source in `docs/wiki` (copy it to `Rookparcel.wiki.git` after edits; `docs/wiki/BRIEF.md` has the conventions). French, Spanish, German and Hungarian translations (`docs/wiki/<code>/`, names in BRIEF.md) are done (2026-11-22); when an English page changes, change its four copies too. Open: the other D24 languages; F1 could open the wiki (`help::WIKI_URL` is `None`); keep pages current as features change.
5. Compiled-in Rust plugin hooks (moved from V1.1, D35), once a completion action, search source or custom column needs one.

## Future improvements

- BEP 44 DHT items/ed25519, BEP 51 sample-infohash querying and BEP 55 holepunch after 1.0 unless cheap. Detailed remaining engine/test gaps are in the backlog.
- Split `core` only if compile times justify it; consider io_uring disk/network on Linux and IOCP file I/O on Windows.
- Continuous work: meaningful scale regression baselines, parser fuzzing, per-release security review and a wider interop matrix (including rTorrent and µTorrent).
- Theming (owner's wish: included and third-party themes, no extra resource use): the Web UI has them (D63) and the Windows program has dark mode (D64, [DARKMODE.md](docs/DARKMODE.md)); icon skins and the open checks are in [BACKLOG](docs/BACKLOG.md#themes-researched-2026-11-02_110639_1793613999-theme-pass-2026-11-09_152030_1794234030).

## Known TODOs

Use the topic headings in [BACKLOG.md](docs/BACKLOG.md) for unresolved implementation gaps, behavioral differences and specific unverified checks. Update the relevant topic when work lands; summarize its outcome in the changelog. Historical completed milestones belong in the [archive](docs/archive/README.md), not in this plan.
