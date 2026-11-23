# Changelog archive: 2026-11-21 (1.2.71)

Moved verbatim from the root `CHANGELOG.md` on 2026-11-23_131433_1795436073.

## 2026-11-21_093211_1795249931: 1.2.71, the bug hunt's high and medium fixes

- **Done by five parallel Opus agents in worktrees**, reviewed and merged here; each fix has tests that fail without it (details and test names: BUGHUNT-2026-11-21.md "Fixes in 1.2.71").
- **Security:** v2 piece layers (outside the info dict) are checked against the file roots for a `.torrent` and the store, as libtorrent's `load_torrent_file` (1); `/pa/remote` refuses resume-data adds and file names leaving the save folder, one check shared with the Web UI (2); a feed's item links reach the LAN only when the server that sent the feed is local, and the search proxy rule never resolves a name the proxy resolves (6); a wrong current password counts towards the ban, hashed outside the lock (11).
- **Engine:** the DHT switch applies live (3); announces answered after a stop are dropped, a stale `started` gets a `stopped` (4); bans cover the IP and survive a full peer list, which now follows libtorrent's erase-candidate rule (5, 13); strict end-game, one busy block for an idle peer once every wanted piece is started (7; the 3,000-tiny-files swarm test 28 s to 3-5 s); a late DNS answer cannot steer another tracker request (9).
- **Data:** a failed `resume.log` write is cut back and rewritten at its intact length (10); an RSS smart filter's failed episode is tried again (8).
- **Problem:** worktree isolation kept an agent from SECURITY-AUDIT.md (locally excluded); its SEC-26 update was added here. nox `the_kept_block_list_is_in_force_at_start` failed now and then: its "nothing fetched" check looked for `127.0.0.1:9`, which a random listen port such as `127.0.0.1:9862` also contains; it now looks for `127.0.0.1:9/`.
- **Validation:** after the merge, core (986 lib tests and every integration binary), frontend, nox and Win32 (one thread) all pass; `cargo check` of the four with tests clean. GTK and macOS were not built (no call site of theirs changed).
