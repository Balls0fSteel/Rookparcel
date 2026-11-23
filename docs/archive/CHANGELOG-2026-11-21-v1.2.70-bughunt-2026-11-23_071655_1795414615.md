# Changelog archive: bug hunt of 1.2.70 (2026-11-21)

Moved verbatim from the root `CHANGELOG.md` on 2026-11-23_071655_1795414615. Paths and links keep their repository-root context.

## 2026-11-21_002110_1795216870: bug hunt of 1.2.70

- Read-only review of `13d405d0` by eighteen parallel reviewers, one per area; the high and medium findings and about half the low ones re-read at the cited lines, the high and medium ones searched for in SECURITY-REVIEW, SECURITY-AUDIT, PORTING and DECISIONS (none listed); nothing built, run or fixed. 34 defects in [BUGHUNT-2026-11-21.md](docs/BUGHUNT-2026-11-21.md): 2 high, 9 medium, 23 low, plus inherited and minor notes. uTP, the DHT node, the disk write paths and Web UI escaping came out clean. BACKLOG's live DHT toggle check fails (item 3).
