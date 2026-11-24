# Changelog, 2026-11-21 (1.2.72 and 1.2.73)

Moved verbatim from the root changelog on 2026-11-24_235917_1795561157.

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
