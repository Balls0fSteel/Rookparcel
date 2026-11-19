# Changelog archive: 1.2.66, 1.2.67 and the Flatpak runtime fix (2026-11-17)

Moved verbatim from the root `CHANGELOG.md` on 2026-11-19_161259_1795101179. Paths and links keep their repository-root context.

## 2026-11-17_194447_1794941087: Release workflow: the Flatpak runtimes copied into the container's own layer

- **Problem:** the v1.2.67 Release run built ten of eleven jobs; `flatpak (x86_64)` failed three times in a row (two re-runs), each on runner image `ubuntu24/20261111.127`: every runtime update in the builder container failed with "Invalid cross-device link", the GNOME SDK's too, and flatpak-builder exited. 1.2.65's run that morning (image `.370`) and the aarch64 leg passed with the same container digest and cache.
- **Found with a probe branch** (`ubuntu-24.04`, the same container, deleted afterwards): the runtimes are installed in the image's layers at `/var/lib/flatpak` (3.9 GB) and the container's overlay has no `redirect_dir`, so flatpak cannot rename their directories; `flatpak --system update` failed with EXDEV, and after copying the installation into the container's own layer (25 s) the same update succeeded.
- **Fix:** the `flatpak` job copies the installation before flatpak-builder (both legs, skipped if it is a mount point); RELEASING.md "Flatpak" says why. No program changed, so the version stays 1.2.67; the unpublished tag `v1.2.67` moves to this commit (no draft had been made).

## 2026-11-17_184916_1794937756: Version 1.2.67: the security review of 1.2.66 and its fix; release 1.2.67 prepared

- **Before the release (owner: "check CI, release"):** CI, FreeBSD and Fuzz ran on `68bb29c6` (1.2.66). The per-release review over `v1.2.65..HEAD -- crates` by one read-only reviewer: nothing critical, high or medium. Recorded in SECURITY-REVIEW.md "Release 1.2.67".
- **Fixed (low):** sorted by Done or %, `set_file_done` counted every progress alert with changed bytes as a new order, so Win32, GTK and the Mac reloaded every row and reselected each second even when nothing moved. Now `file_order` changes only when the order does (`FileTree::node_order` or the plain list's rows compared). **Left open:** a header click dropped while the app state is borrowed keeps the toolkit's arrow until the next click (cosmetic; BACKLOG).
- **Release:** `docs/releases/1.2.67.md` (covers 1.2.66, which is not tagged); the Flatpak metainfo's first `<release>` is 1.2.67; the wiki's sort note says 1.2.67.
- **Checked after the fix:** fmt clean; clippy `-D warnings` clean for frontend and Win32; frontend 451 tests and Win32 172 (one thread) passing.

## 2026-11-17_183218_1794936738: Version 1.2.66: the GTK and macOS Files tabs sort by their headers (D78)

- **On the owner's word** ("do the Linux/Mac sort job"): both GUIs now call the shared model Win32 uses. `Core::set_file_sort` and `Core::toggle_file_sort` are new (they flag the detail event) for the Mac, which holds `Core`, not a `Detail`.
- **GTK:** the `ColumnView`'s columns get sorters that never reorder (the model sorts); a header change becomes the next step of the cycle (`files_header_changed`, tested), so GTK's own two-state toggle cannot disagree with the model, and the header shows the model's sort (cleared on the third click). Columns no longer move, so header indices stay the column ids. The selection is taken as `file_marks` before a sort, a new file list or a progress update of a sorted Done/% column, and put back by `file_rows_of` after.
- **macOS:** the outline had no `outlineView:sortDescriptorsDidChange:`, so its header clicks went nowhere; it now has one, and a new `sort_click` hook lets the Files tab choose the sort and set the descriptors itself (a `sorting` flag keeps that from counting as a click). The tree keeps its selection by node and reloads when `Detail::file_order` changes; the plain list keeps its selection's marks from each selection change and reselects after a re-sort, without scrolling. The stored sort is read before the tab is built.
- **Validation:** fmt clean; clippy `-D warnings` clean for core, frontend, nox and Win32, GTK (cross, stand-in pkg-config) and the Mac (x86_64 and aarch64 Darwin); frontend 451 tests. On the VMs: GTK built, its 28 tests and the frontend's passed, and the Xvfb run checked the cycle, the selection and the restart; the Mac app built and signed, 24 tests, and an Accessibility-driven run checked the same. Not seen: a real download moving rows (BACKLOG).
