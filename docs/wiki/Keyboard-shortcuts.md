# Keyboard shortcuts

This page lists the keyboard shortcuts of the Windows version of Rookparcel, then those of the GTK version, the macOS app and the Web UI. Most of them are also written beside their menu items.

## Main window (Windows)

| Keys | What they do |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>O</kbd> | **File > Add Torrent...** |
| <kbd>Ctrl</kbd>+<kbd>U</kbd> | **File > Add Torrent from URL...** (a magnet link or web address on the clipboard is filled in) |
| <kbd>Ctrl</kbd>+<kbd>N</kbd> | **File > Create New Torrent...** |
| <kbd>Ctrl</kbd>+<kbd>P</kbd> | **Options > Preferences** |
| <kbd>Ctrl</kbd>+<kbd>G</kbd> | **Options > Setup Guide** |
| <kbd>Ctrl</kbd>+<kbd>L</kbd> | **File > Lock Rookparcel**: hides every Rookparcel window until the lock PIN is typed (see [Locking Rookparcel](Locking-Rookparcel)) |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> | Moves to the search box on the toolbar (when the toolbar is shown). In the Files tab, moves to the Files tab's filter box instead. |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Selects every torrent in the list. In the Files tab, selects the files the filter shows. |
| <kbd>Ctrl</kbd>+<kbd>S</kbd> | **Start** the selected torrents |
| <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd> | **Pause** the selected torrents |
| <kbd>Ctrl</kbd>+<kbd>T</kbd> | **Stop** the selected torrents |
| <kbd>Del</kbd> | **Remove** the selected torrents |
| <kbd>Shift</kbd>+<kbd>Del</kbd> | **Remove And > Delete Data**: removes the torrents and deletes their files |
| <kbd>Enter</kbd> | **Properties** of the selected torrents (in the torrent list) |
| <kbd>Shift</kbd>+<kbd>F10</kbd> or the menu key | Opens the right-click menu under the focused row |
| <kbd>Alt</kbd>+<kbd>F4</kbd> | Closes the window. With **Close to tray** on (the default), Rookparcel keeps running in the tray. |

With **Confirm when deleting torrents** on in Preferences, <kbd>Del</kbd> and <kbd>Shift</kbd>+<kbd>Del</kbd> ask first. **File > Exit** has no shortcut; see [Tray, notifications and startup](Tray-notifications-and-startup).

*<kbd>Ctrl</kbd>+<kbd>G</kbd> and the Setup Guide are new in 1.1.62, the Files tab's filter box in 1.1.56, and <kbd>Ctrl</kbd>+<kbd>L</kbd> in 1.2.54.*

Mouse shortcuts that go with them:

- Double-clicking a torrent that has finished opens its folder; double-clicking one that has not opens **Properties**.
- In the Files tab, double-clicking a file or pressing <kbd>Enter</kbd> on it opens it.

In the Files tab's folder tree, <kbd>Right</kbd> or <kbd>+</kbd> opens the focused folder, and <kbd>Left</kbd> or <kbd>-</kbd> closes it; on a file or a closed folder, <kbd>Left</kbd> goes to the folder that holds it. In the torrent list and in the detail tabs, typing the first letters of a name selects the next row whose name starts with them, as in Explorer; pause for a second to start again. *The folder keys and the typing are new in 1.2.21.*

## The search box and the Files tab filter

| Keys | In the toolbar's search box | In the Files tab's filter box |
|---|---|---|
| <kbd>Escape</kbd> | Empties the box and moves to the torrent list | Empties the box and moves to the file list |
| <kbd>Enter</kbd> | With **Filter torrents** chosen, moves to the list. With a search source chosen, searches it. | Moves to the file list |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Selects the box's text | Selects the box's text (so does <kbd>Ctrl</kbd>+<kbd>F</kbd>) |
| <kbd>Alt</kbd>+<kbd>Down</kbd> | Opens the box's drop-down menu (sources, **Manage Sources...**) | |

While you type in either box, <kbd>Del</kbd> and <kbd>Ctrl</kbd>+<kbd>A</kbd> edit the text, not the list. See [Searching for torrents](Searching-for-torrents) and [Torrent details and files](Torrent-details-and-files).

## Add New Torrent window

In the file list of the Add New Torrent window:

| Keys | What they do |
|---|---|
| <kbd>Space</kbd> | Ticks or unticks the selected files and folders |
| <kbd>Right</kbd> or <kbd>+</kbd> | Opens the focused folder |
| <kbd>Left</kbd> or <kbd>-</kbd> | Closes the focused folder; on a file or a closed folder, <kbd>Left</kbd> goes to its parent folder |
| <kbd>Enter</kbd> | Opens or closes the focused folder |
| Letters | Select the next row whose name starts with them |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> | Moves to the **Filter files** box above the list, with its text selected |

In the filter box, <kbd>Enter</kbd> moves to the list, and <kbd>Escape</kbd> empties the box; with the box empty, <kbd>Escape</kbd> closes the window like **Cancel**. *The filter box and the typing are new in 1.2.21.*

See [Adding torrents](Adding-torrents).

## Preferences and other windows

| Keys | What they do |
|---|---|
| <kbd>F1</kbd> in Preferences | Shows the help of the focused setting beside it, and in the help pane under the pages |
| <kbd>Escape</kbd> | Closes a dialog like **Cancel**. In the Setup Guide it is **Skip**. |
| <kbd>Tab</kbd>, <kbd>Enter</kbd>, <kbd>Escape</kbd> in the Search Torrents window | Move between its controls, act on the focused one, close it |

The help in Preferences also appears as a tooltip when you point at a setting. In the main window, a menu item or toolbar button you point at is described in the status bar. *These help texts are new in 1.1.57.*

See [Preferences reference](Preferences-reference).

## Menu keys

Every menu can be reached with <kbd>Alt</kbd> and the underlined letter, as in other Windows programs: <kbd>Alt</kbd>+<kbd>F</kbd> for **File**, <kbd>Alt</kbd>+<kbd>O</kbd> for **Options**, <kbd>Alt</kbd>+<kbd>H</kbd> for **Help**, then the underlined letter of an item.

## GTK version (Linux, BSD)

The GTK version has these shortcuts:

| Keys | What they do |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>O</kbd> | **Add Torrent...** |
| <kbd>Ctrl</kbd>+<kbd>U</kbd> | **Add Torrent from URL...** |
| <kbd>Ctrl</kbd>+<kbd>N</kbd> | **Create New Torrent...** |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> | Moves to the search box |
| <kbd>Ctrl</kbd>+<kbd>P</kbd> | **Preferences** |
| <kbd>Ctrl</kbd>+<kbd>G</kbd> | **Setup Guide** |
| <kbd>Ctrl</kbd>+<kbd>L</kbd> | **Lock Rookparcel**. It works in every Rookparcel window (Preferences, Properties, Add New Torrent and the others), not only the main window. *New in 1.2.76; before, it did nothing while a dialog was open.* |
| <kbd>Ctrl</kbd>+<kbd>Q</kbd> | **Exit**: quits Rookparcel, also when the window would close to the tray |
| <kbd>Ctrl</kbd>+<kbd>S</kbd> | **Start** the selected torrents |
| <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd> | **Pause** the selected torrents |
| <kbd>Ctrl</kbd>+<kbd>T</kbd> | **Stop** the selected torrents |
| <kbd>Delete</kbd> | **Remove** |
| <kbd>Shift</kbd>+<kbd>Delete</kbd> | Remove and delete the data |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Selects every torrent; in the Files tab, the files its filter shows |
| <kbd>Enter</kbd> in the torrent list | **Properties** |
| <kbd>F1</kbd> in Preferences | Shows the focused setting's help |
| <kbd>Escape</kbd> | Empties the search box or the Files tab's filter; closes dialogs |

What differs from Windows:

- Start, Pause, Stop, Properties and Remove have the same keys as on Windows.
- <kbd>Ctrl</kbd>+<kbd>F</kbd> moves to the Files tab's filter box when the focus is in the Files tab, as on Windows.
- In the Files tab's folder tree, <kbd>Right</kbd> and <kbd>Left</kbd> open and close folders as on Windows.
- <kbd>Ctrl</kbd>+<kbd>Q</kbd> quits; on Windows there is no quit shortcut.
- The Add New Torrent window's keys are the same as on Windows.

## macOS

The macOS app uses <kbd>⌘</kbd> (Command) where Windows uses <kbd>Ctrl</kbd>, and the Mac's own keys where they differ. The shortcuts are written beside the menu items in the menu bar.

| Keys | What they do |
|---|---|
| <kbd>⌘</kbd>+<kbd>,</kbd> | **Rookparcel > Preferences** |
| <kbd>⌘</kbd>+<kbd>G</kbd> | **Rookparcel > Setup Guide** |
| <kbd>⌘</kbd>+<kbd>H</kbd> | Hides Rookparcel (<kbd>⌥</kbd>+<kbd>⌘</kbd>+<kbd>H</kbd> hides the other apps) |
| <kbd>⌘</kbd>+<kbd>Q</kbd> | **Quit Rookparcel**, also when the window would close to the menu bar |
| <kbd>⌘</kbd>+<kbd>O</kbd> | **File > Add Torrent...** |
| <kbd>⌘</kbd>+<kbd>U</kbd> | **File > Add Torrent from URL...** |
| <kbd>⌘</kbd>+<kbd>N</kbd> | **File > Create New Torrent...** |
| <kbd>⌘</kbd>+<kbd>L</kbd> | **File > Lock Rookparcel** |
| <kbd>⌘</kbd>+<kbd>W</kbd> | **File > Close Window**: with **Close to tray** on, Rookparcel keeps running in the menu bar |
| <kbd>⌘</kbd>+<kbd>A</kbd> | **Edit > Select All**: every torrent in the list, or the text in a box |
| <kbd>⌘</kbd>+<kbd>F</kbd> | **Edit > Find...**: moves to the search box on the toolbar; in the Files tab, to its filter box |
| <kbd>⌥</kbd>+<kbd>⌘</kbd>+<kbd>T</kbd> | **View > Show Toolbar** |
| <kbd>⌃</kbd>+<kbd>⌘</kbd>+<kbd>S</kbd> | **View > Show Sidebar** (the category list) |
| <kbd>⌃</kbd>+<kbd>⌘</kbd>+<kbd>F</kbd> | **View > Enter Full Screen** |
| <kbd>⌘</kbd>+<kbd>S</kbd> | **Torrent > Start** |
| <kbd>⇧</kbd>+<kbd>⌘</kbd>+<kbd>P</kbd> | **Torrent > Pause** |
| <kbd>⌘</kbd>+<kbd>T</kbd> | **Torrent > Stop** |
| <kbd>⌘</kbd>+<kbd>⌫</kbd>, or <kbd>⌫</kbd> in the list | **Torrent > Remove** |
| <kbd>⌥</kbd>+<kbd>⌘</kbd>+<kbd>⌫</kbd> | **Torrent > Remove And Delete Data**: removes the torrents and deletes their files |
| <kbd>⌘</kbd>+<kbd>I</kbd>, or <kbd>Return</kbd> in the list | **Torrent > Properties** |
| <kbd>⌘</kbd>+<kbd>M</kbd> | **Window > Minimize** |

What differs from Windows:

- Preferences is <kbd>⌘</kbd>+<kbd>,</kbd>, as in other Mac apps, not <kbd>Ctrl</kbd>+<kbd>P</kbd>.
- Remove is <kbd>⌘</kbd>+<kbd>⌫</kbd> (Delete), and Properties <kbd>⌘</kbd>+<kbd>I</kbd>.
- The right-click menu of the torrent list has the same items as the **Torrent** menu; Control-click opens it too.
- **Edit** has the usual Undo, Cut, Copy and Paste for text boxes.

*The native macOS app is new in 1.2.19.*

## Web UI

In the Web UI's torrent list (click the list first so it has the keyboard):

| Keys | What they do |
|---|---|
| <kbd>Up</kbd>, <kbd>Down</kbd>, <kbd>Page Up</kbd>, <kbd>Page Down</kbd>, <kbd>Home</kbd>, <kbd>End</kbd> | Move the selection |
| <kbd>Shift</kbd> with those keys | Extends the selection |
| <kbd>Ctrl</kbd> with those keys | Moves the focus without changing the selection |
| <kbd>Ctrl</kbd>+<kbd>Space</kbd> | Adds the focused row to the selection, or takes it out |
| <kbd>Space</kbd> | **Pause** the selected torrents |
| <kbd>Delete</kbd> | **Remove** |
| <kbd>Shift</kbd>+<kbd>Delete</kbd> | Remove and delete the data |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Selects every torrent |
| <kbd>Shift</kbd>+<kbd>F10</kbd> or the menu key | Opens the right-click menu |
| <kbd>Escape</kbd> | Closes an open menu |

Elsewhere in the Web UI:

| Keys | What they do |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>P</kbd> | Opens **Preferences** (not while the Add or Properties window is open) |
| <kbd>Escape</kbd> in the filter box | Empties it and moves to the list |
| <kbd>Enter</kbd> in the filter box | In the Add window: moves to the list |
| <kbd>F1</kbd> in Preferences | Shows the focused setting's help |
| <kbd>Ctrl</kbd>+<kbd>Enter</kbd> in Preferences | **OK** |
| <kbd>Enter</kbd> in the Advanced page's value field | Sets the value |
| <kbd>Escape</kbd> in Preferences | Closes it like **Cancel** |
| <kbd>Up</kbd>, <kbd>Down</kbd>, <kbd>Home</kbd>, <kbd>End</kbd> on the list of Preferences pages | Move between the pages |
| <kbd>Ctrl</kbd>+<kbd>Enter</kbd> in Torrent Properties | **OK** (<kbd>Enter</kbd> alone adds a line in the trackers box) |

In the Files tab and in the Add window's file list:

| Keys | What they do |
|---|---|
| <kbd>Up</kbd>, <kbd>Down</kbd>, <kbd>Home</kbd>, <kbd>End</kbd> | Move the focus (with <kbd>Shift</kbd>, extend the selection) |
| <kbd>Right</kbd>, <kbd>Left</kbd> | In the folder tree: open or close a folder; on a file or a closed folder, <kbd>Left</kbd> goes to the parent folder, and <kbd>Right</kbd> on an open folder goes to its first entry |
| <kbd>Space</kbd> | In the Add window: ticks or unticks the selected files and folders |
| Letters | Select the next row whose name starts with them |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> | Moves to the filter box |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | In the Files tab: selects the files in sight |
| <kbd>Escape</kbd> in the filter box | Empties it (in the Add window, closes the window when it is already empty) |
| <kbd>Enter</kbd> in the filter box | Moves to the list |

On a Mac, <kbd>Command</kbd> works wherever the Web UI says <kbd>Ctrl</kbd>. Your browser's own shortcuts may take some keys first. See [Web UI](Web-UI).
