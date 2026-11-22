# Changelog

Recent completed changes, issues and decisions. Timestamps: `YYYY-MM-DD_HHMMSS_<unix epoch>` (Europe/Budapest).

Earlier history: [archive index](docs/archive/README.md), including all 148 original entries from 2026-10-15 through 2026-10-23. Open archived entries only for a specific historical question. Current work: [PLAN.md](PLAN.md).

## 2026-11-22_210021_1795377621: the website's benchmarks now show the Hetzner run

- **On the owner's word** ("update the benchmark on the site if you still have the VPS run results"): `site/benchmarks.html` filled with `summarize.py --fill` from the 2026-11-19 Hetzner CCX23 run, which replaces the 2026-11-11 VM run. The Rookparcel there was built from source (a 1.2.70 pre-release and the 1.2.69 tag), not a published tarball, and the page says so; 1.2.76 is not measured yet. Prose rechecked against the numbers: "Since 1.0.1" became "Against the previous release" (the older history points at BENCHMARKS.md), the method names the server, and a new limit says Rookparcel's download CPU depends on SHA extensions (about twice the others' on the Xeon run). Site tests pass.

## 2026-11-22_205639_1795377399: the wiki checked against 1.2.76; French, Spanish, German and Hungarian wikis

- **English pages:** seven agents checked every page against the code and the changelogs since each page's last edit. Fixes include: the Web UI does offer the search helper (since 1.1.67; the page said no buttons), GTK has Start/Pause/Stop/Properties keys and Ctrl+A/Ctrl+F in the Files tab, **Hashing threads (not used):**, GTK has the alternate row colour, a Web UI setting and a Sonarr/Radarr setting name that did not exist, the files-missing hold and Recheck changes (marked 1.2.76, since 1.2.75 was not published), remote protocol 21, macOS sections on several pages.
- **Translations:** `docs/wiki/fr`, `es`, `de`, `hu`, 34 pages each with its own sidebar and footer; page names are a table in `docs/wiki/BRIEF.md` (unique across the flat GitHub wiki). UI names come from the app's `.lng` tables by key; engine messages that the app shows in English stay English. Sixteen agents translated four page groups per language; a script restored 193 cross-page anchors by heading position (every page kept the English heading structure) and a link check found no broken page, anchor or image link. Key names in `<kbd>` stay English.
- **Found in the app's tables, not fixed here:** `de.lng` gives the Web UI port (`PrefWebUiPort`) the same text as the connection port; `hu.lng` uses *Tulajdonságok* for Preferences and has a few informal (tegező) strings such as `GenExternal`.

## 2026-11-22_184404_1795369444: 1.2.76, the release review's fixes; release notes for 1.2.76

- **Release review** of 1.2.71-1.2.75 (SECURITY-REVIEW.md "Release 1.2.76"; one fresh reviewer, findings checked here), nothing critical or high. **Medium, fixed:** 1.2.71 kept banned peers in the list for good, so peers banned from enough addresses (one IPv6 /64) could fill it and keep every new peer out; an entry banned for bad data is now the first a full list gives up, its IP staying banned. **Low, fixed:** Run Program's list knows `script`, `expect`, `forfiles`, `wmic`, editors, debuggers and `dc`; a Stop ends a requested check's place outside the queue, and a magnet without metadata takes none. Open: Run Program stays a list (environment variables would end the guessing).
- **Release 1.2.76, published 2026-11-22** after CI, FreeBSD and Fuzz passed on `c12e3fe5`; the draft's digests matched SHA256SUMS, the zip held the four files and both programs said 1.2.76; nothing started on a clean machine or a Linux desktop (1.2.75 was not published, so its notes, metainfo and news name 1.2.76 and cover 1.2.70-1.2.75); remote protocol 21 since 1.2.70, so a window and `rookparcel-nox` both need 1.2.76.

## 2026-11-22_142957_1795354197: 1.2.75, a torrent whose storage is gone is held; Recheck starts a stopped torrent

- **The owner's report (on 1.2.69):** a storage was removed, and instead of "missing files" its torrents downloaded again. Not fixed by 1.2.70-1.2.74: the resume check took every file that was not found as lost (1.2.46's per-file rule, meant for a virus scanner removing one file), so a pulled drive cleared every piece and the empty set was saved.
- **D79 (`disk::storage_gone`, `engine/storage_hold.rs`):** a resume check that finds none of the files its pieces are in, or not its save path, stops the torrent with "files missing (the drive, share or folder is not available)" before anything is made on disk; the stored pieces stay, across restarts too. Every 30 s a probe thread (`DiskIo::async_probe_paths`, so a dead share never blocks the engine or a disk queue) looks at the held paths, and a torrent whose path is back starts and finds its files by the resume data, without hashing. A write that fails with "not found" (the drive pulled while downloading) is held the same way. One file of several gone keeps 1.2.46's rule. Start ends a hold (held again if still missing); Recheck takes what is there.
- **Recheck (the owner's second point):** on a stopped torrent it only forgot the pieces; the check waited for a Start. Now the GUIs and the Web UI start a stopped torrent with the recheck (`frontend::actions::recheck`; the Pause mark goes as with Start), the engine's Recheck clears a torrent's error (libtorrent `force_recheck`'s `clear_error`), and a requested check runs without a queue slot, so a queued torrent is checked at once and queued again after. The qBittorrent shim's `torrents/recheck` keeps qBittorrent's behaviour.
- **Tests:** disk (all files gone, save path gone, nothing to lose, one of two gone, the probe), engine (hold, probe, back, Start and Remove end it), swarm (a seed held over two restarts, then seeding without hashing; Recheck of a held one downloads), queueing (a queued download's recheck), frontend (Recheck clears the Pause mark). Not checked by hand with a real drive yet.

## 2026-11-22_005433_1795305273: 1.2.74, the macOS swarm flake's cause

- **1.2.73's CI** failed once on macOS in `a_download_stopped_midway_resumes_with_its_names_and_pieces` (passed on a rerun); its message (one block fewer done after a passing check) led an agent to the cause: a piece whose last block was being written at a stop was hashed after it, set as had, and kept as partial too, so its blocks counted twice until the check dropped them. A late hash now takes the piece out of the partial pieces, and loading drops partial pieces that are also had (PORTING row corrected; it still said partial pieces were not saved).
- **Evidence:** with a 200 ms delay on the second block's write the swarm test failed 19 of 20 times with CI's exact message, 0 of 20 with the fix; three deterministic engine tests (`stop_hash_tests.rs`) fail without it. Core suite, clippy and fmt pass.

## 2026-11-21_235809_1795301889: 1.2.73, the bug hunt's follow-ups

- **The owner decided the follow-ups** (BUGHUNT-2026-11-21.md "Follow-ups decided"): A-H fixed here by four parallel agents, I-N accepted as known behaviour.
- **Engine:** refused stored v2 layers make a v2-only torrent check in full once refetched (A); a finished torrent fetching layers keeps and dials seeds and announces as a non-seed (B); `announcing_to_dht` follows the DHT switch (E); incoming peers respect the peer-list cap (F). Hybrids are not rechecked: their pieces always needed the v1 hash.
- **nox:** no request runs PBKDF2 under the State lock (C); the subnet and API-key sessions' ban tests (D).
- **GUIs:** the Mac app's 21 Auto Layout warnings at start are gone (sidebar cells give way while the outline is 0 wide; checked on the VM) (G); GTK's Ctrl+L locks from every dialog (checked on the Xubuntu desktop) (H).
- **Problem:** VMware powered the Mac VM off at about 21:59; the agent started it again headless (it is left running).

## 2026-11-21_114756_1795258076: 1.2.72's GTK and macOS fixes on the VMs

- **Xubuntu and the macOS VM**, by two parallel agents, all passed (details: BUGHUNT-2026-11-21.md "VM run"): the GTK and Mac crates' tests on their platforms; a vanished tray brings up the PIN prompt or the window (27); Create's failure box waits for the PIN (25); the Search window keeps its source (26); Manage Sources' last-row removal on macOS. A control build showed macOS 15.8 ignores an out-of-range row, so 28 is defensive there; 34 has no trigger.
- **Found on the way** (BACKLOG): about 21 Auto Layout constraint warnings at the Mac app's start (older than 1.2.72); GTK's Ctrl+L does nothing while Create New Torrent is open; a window shown while the Xfce panel was gone sat under it.
- **Problem:** `tools/mac-vm/push.sh` copied untracked but unignored folders, so the agents' `.claude/` worktrees and `graphify-out/` reached the VM (removed there); it now leaves them out.

## 2026-11-21_111517_1795256117: 1.2.72, the bug hunt's low fixes

- **All 22 low findings (12, 14-34) fixed** by six parallel Opus agents in worktrees, reviewed and merged here; list and test names in BUGHUNT-2026-11-21.md "Fixes in 1.2.72".
- **Decisions:** parole treats pad and restored blocks as neutral and UPnP frees slots that disabled routers held, both deliberate divergences from libtorrent (PORTING). Win32 posted results go by id (`posted.rs`). Run Program's guard looks through wrappers and knows interpreters' eval options; a first draft that refused a code after any program's `-c`/`-e` was sent back (it refused `notify-send -e %N`, `grep -e %N`, `rsync -c %F`), and a stored line refused at load is now reported once in the Logger instead of going off silently. The Web UI's `tick` carries `removed` hashes for the selection.
- **Validation:** core (with and without `mock`), frontend, nox and Win32 (one thread) tests, clippy for the four, GTK for Linux and macOS cross-lints, fmt. The Web UI session-end fix was checked in the built-in browser against a mock nox restarted under an open Preferences dialog. GTK (25-27) and macOS (28, 34) not run; a VM pass is open.

## 2026-11-21_093211_1795249931: 1.2.71, the bug hunt's high and medium fixes

- **Done by five parallel Opus agents in worktrees**, reviewed and merged here; each fix has tests that fail without it (details and test names: BUGHUNT-2026-11-21.md "Fixes in 1.2.71").
- **Security:** v2 piece layers (outside the info dict) are checked against the file roots for a `.torrent` and the store, as libtorrent's `load_torrent_file` (1); `/pa/remote` refuses resume-data adds and file names leaving the save folder, one check shared with the Web UI (2); a feed's item links reach the LAN only when the server that sent the feed is local, and the search proxy rule never resolves a name the proxy resolves (6); a wrong current password counts towards the ban, hashed outside the lock (11).
- **Engine:** the DHT switch applies live (3); announces answered after a stop are dropped, a stale `started` gets a `stopped` (4); bans cover the IP and survive a full peer list, which now follows libtorrent's erase-candidate rule (5, 13); strict end-game, one busy block for an idle peer once every wanted piece is started (7; the 3,000-tiny-files swarm test 28 s to 3-5 s); a late DNS answer cannot steer another tracker request (9).
- **Data:** a failed `resume.log` write is cut back and rewritten at its intact length (10); an RSS smart filter's failed episode is tried again (8).
- **Problem:** worktree isolation kept an agent from SECURITY-AUDIT.md (locally excluded); its SEC-26 update was added here. nox `the_kept_block_list_is_in_force_at_start` failed now and then: its "nothing fetched" check looked for `127.0.0.1:9`, which a random listen port such as `127.0.0.1:9862` also contains; it now looks for `127.0.0.1:9/`.
- **Validation:** after the merge, core (986 lib tests and every integration binary), frontend, nox and Win32 (one thread) all pass; `cargo check` of the four with tests clean. GTK and macOS were not built (no call site of theirs changed).

## 2026-11-21_002110_1795216870: bug hunt of 1.2.70

- Read-only review of `13d405d0` by eighteen parallel reviewers, one per area; the high and medium findings and about half the low ones re-read at the cited lines, the high and medium ones searched for in SECURITY-REVIEW, SECURITY-AUDIT, PORTING and DECISIONS (none listed); nothing built, run or fixed. 34 defects in [BUGHUNT-2026-11-21.md](docs/BUGHUNT-2026-11-21.md): 2 high, 9 medium, 23 low, plus inherited and minor notes. uTP, the DHT node, the disk write paths and Web UI escaping came out clean. BACKLOG's live DHT toggle check fails (item 3).
