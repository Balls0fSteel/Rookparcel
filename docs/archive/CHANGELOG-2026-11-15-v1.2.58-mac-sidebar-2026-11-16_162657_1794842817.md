# Changelog archive: 2026-11-15 and 2026-11-16, version 1.2.58 and the Mac sidebar symbols

2 entries moved verbatim from the root changelog on 2026-11-16_162657_1794842817 to keep it under budget.

## 2026-11-16_001242_1794784362: The Mac's dark Preferences sidebar symbols: the VM's drawing, not a bug (documentation only)

- **Asked for:** a fix for the Preferences sidebar's SF Symbols drawn nearly black on the dark sidebar on the macOS VM (BACKLOG "UI lock", older than D71).
- **Found:** a source-list row (`NSTableViewStyle::SourceList`) tints its template symbols with an effect that VMware's software renderer draws black. On the VM they are black in light mode too.
  - An `NSTableCellView` subclass that set `contentTintColor` (`secondaryLabelColor`, by background style) changed nothing. At run time the image was a template with the tint set under the dark appearance, and still black.
  - A small Swift app with the same outline in a sidebar split item showed the same, even with a red tint. Finder's own sidebar icons on the VM are black instead of the accent colour.
  - The same symbols outside a source list draw right in light and dark: plain and sidebar image views in that app, the browse buttons on Directories.
- **Kept:** no code change; the subclass was dropped, so no version bump. BACKLOG, `tools/mac-vm/README.md` ("Not seen right") and MACOS_GUI.md L (a check for the owner's real Mac) say so.

## 2026-11-15_232343_1794781423: Version 1.2.58: Preferences > Advanced > UI Lock, the lock's off switch, a narrower PIN dialog (D71)

- **Asked for:** a place in Advanced for the UI lock, a way to change the PIN and to turn the lock off, and a fix for the oddly sized Set PIN window on Windows. The owner chose an off switch that hides every Lock command and dropping Options > Lock PIN.
- **Shared:**
  - `ui_lock` has `ui.lock_enabled` (`enabled`, `set_enabled`; off removes the PIN), `Form::Off` and `State` (the page's status line and buttons).
  - `pref_pages` has a fifth Advanced page, `ui_lock`, with `Item::LockSwitch`, `Item::LockStatus` and three PIN actions. `UiIcon::Lock` is the bold set's lock.
  - The menu strings became `PrefLockSetPin`/`ChangePin`/`RemovePin`, renamed in every table so the translations stayed; `MenuLockPin` went. Eight new strings were translated into the 34 tables by a subagent, reusing each table's own words for PIN, File and Lock Rookparcel.
- **Win32:** the page's controls act through `Hooks::ui_lock`, with the PIN dialog modal on Preferences. `show_lock_item` deletes File > Lock while off and puts it back above Exit. `lock_command` ignores Ctrl+L and posted commands while off, and the tray offers Lock only when the lock is on and a PIN is set.
- **The PIN dialog:** it is 170 dialog units wide instead of 230, and the message line is one line, growing with the dialog when a message needs more (`fit_pin_message`). Set PIN went from 419×234 to 314×219 px at 100 %.
- **GTK and Mac:** subagents built them, and both cross-lint clean. GTK hides File > Lock with `hidden-when=action-disabled`. The Mac hides it in validation, refuses ⌘L while off, and drops `Command::SetPin/ChangePin/RemovePin` (kinds 81-83).
- **Checked:**
  - frontend 401 tests and Win32 151 tests (one thread), including new ones for the page, the switch and the dialog's growth.
  - Clippy with `-D warnings` for Win32, frontend and nox; GTK (Linux target) and the Mac (both targets) cross-linted.
  - The hidden desktop, light and dark: Set → Change with a wrong PIN → lock → wrong and right PIN in the prompt → switch off with the PIN → File > Lock gone and a posted Lock ignored → on again.
- **VM runs (same evening):**
  - **GTK on the Xubuntu VM** (Xvfb, private bus, no tray): the page, Set PIN, the switch with Cancel and with the PIN, File > Lock hidden and Ctrl+L ignored while off, both back once ticked, and Ctrl+L without a PIN asking for one before it locks.
  - **The Mac on the macOS VM** (a subagent, light and dark), the same plus Change PIN with a wrong PIN, no Lock in the status item's or the Dock's menu while off, and a lock and unlock.
  - No D71 bug. Found and left in BACKLOG "UI lock": "tray" in the Mac's status line, and the Preferences sidebar's symbols nearly invisible in dark mode (older than D71).
