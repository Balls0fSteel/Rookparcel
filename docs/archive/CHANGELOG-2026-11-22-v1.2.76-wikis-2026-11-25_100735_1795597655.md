# Changelog, 2026-11-22 (1.2.76 and the translated wikis)

Moved verbatim from the root changelog on 2026-11-25_100735_1795597655.

## 2026-11-22_205639_1795377399: the wiki checked against 1.2.76; French, Spanish, German and Hungarian wikis

- **English pages:** seven agents checked every page against the code and the changelogs since each page's last edit. Fixes include: the Web UI does offer the search helper (since 1.1.67; the page said no buttons), GTK has Start/Pause/Stop/Properties keys and Ctrl+A/Ctrl+F in the Files tab, **Hashing threads (not used):**, GTK has the alternate row colour, a Web UI setting and a Sonarr/Radarr setting name that did not exist, the files-missing hold and Recheck changes (marked 1.2.76, since 1.2.75 was not published), remote protocol 21, macOS sections on several pages.
- **Translations:** `docs/wiki/fr`, `es`, `de`, `hu`, 34 pages each with its own sidebar and footer; page names are a table in `docs/wiki/BRIEF.md` (unique across the flat GitHub wiki). UI names come from the app's `.lng` tables by key; engine messages that the app shows in English stay English. Sixteen agents translated four page groups per language; a script restored 193 cross-page anchors by heading position (every page kept the English heading structure) and a link check found no broken page, anchor or image link. Key names in `<kbd>` stay English.
- **Found in the app's tables, not fixed here:** `de.lng` gives the Web UI port (`PrefWebUiPort`) the same text as the connection port; `hu.lng` uses *Tulajdonságok* for Preferences and has a few informal (tegező) strings such as `GenExternal`.

## 2026-11-22_184404_1795369444: 1.2.76, the release review's fixes; release notes for 1.2.76

- **Release review** of 1.2.71-1.2.75 (SECURITY-REVIEW.md "Release 1.2.76"; one fresh reviewer, findings checked here), nothing critical or high. **Medium, fixed:** 1.2.71 kept banned peers in the list for good, so peers banned from enough addresses (one IPv6 /64) could fill it and keep every new peer out; an entry banned for bad data is now the first a full list gives up, its IP staying banned. **Low, fixed:** Run Program's list knows `script`, `expect`, `forfiles`, `wmic`, editors, debuggers and `dc`; a Stop ends a requested check's place outside the queue, and a magnet without metadata takes none. Open: Run Program stays a list (environment variables would end the guessing).
- **Release 1.2.76, published 2026-11-22** after CI, FreeBSD and Fuzz passed on `c12e3fe5`; the draft's digests matched SHA256SUMS, the zip held the four files and both programs said 1.2.76; nothing started on a clean machine or a Linux desktop (1.2.75 was not published, so its notes, metainfo and news name 1.2.76 and cover 1.2.70-1.2.75); remote protocol 21 since 1.2.70, so a window and `rookparcel-nox` both need 1.2.76.

