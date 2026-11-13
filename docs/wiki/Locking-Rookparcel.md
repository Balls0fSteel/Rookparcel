# Locking Rookparcel

This page explains how to lock Rookparcel's windows with a PIN, so that someone else at your computer cannot see your torrent list, and how to remove a PIN you have forgotten. *New in 1.2.50, not in a published release yet.*

The lock is for privacy, not security. It hides the windows; it does not encrypt anything. Anyone who can edit Rookparcel's settings file, or open your download folder, can get past it. Do not use your bank card's PIN for it.

## Set a PIN and lock

1. Choose **Options > Lock PIN > Set PIN...**.
2. Type a PIN of 4 to 6 digits in **New PIN (4 to 6 digits)**, and again in **Repeat the new PIN**.
3. Click **OK**.

To lock, choose **File > Lock Rookparcel** or press <kbd>Ctrl</kbd>+<kbd>L</kbd>. Once a PIN is set, the tray menu has **Lock Rookparcel** too. If no PIN is set yet, **Lock Rookparcel** asks for one first, then locks.

Locking hides every Rookparcel window in the tray: the main window and any dialog that was open. Your torrents keep downloading and seeding.

To change the PIN, choose **Options > Lock PIN > Change PIN...**, type the current PIN, then the new one twice. To stop using the lock, choose **Remove PIN...** and type the current PIN.

## Open a locked Rookparcel

Click the tray icon, choose **Show Rookparcel** in the tray menu, or start Rookparcel again. A window titled **Rookparcel is locked** asks for the PIN. With the right PIN, the windows that were open come back as they were. **Cancel** leaves Rookparcel locked in the tray.

The fifth wrong PIN in a row makes Rookparcel wait 30 seconds before it takes another one, and after that each wrong PIN does the same. The right PIN ends this.

Rookparcel stays locked when you exit it. The next time you start it, it asks for the PIN at once, without showing its main window. Started at login with **Start minimized**, it waits in the tray without asking.

## While Rookparcel is locked

- The tray menu offers only **Show Rookparcel** and **Exit**. **Exit** works without the PIN.
- The "Download Complete" notification does not appear, because it names the torrent.
- Messages that would open by themselves wait until you unlock: error messages, an update offer, and the **Add New Torrent** window of a magnet link whose files have arrived.
- A `.torrent` file or magnet link you open while it is locked asks for the PIN first. If you click **Cancel**, it is not added.
- The **Auto Shutdown** countdown still appears, so you can stop it.
- The [Web UI](Web-UI) is not locked. It has its own user name and password. If **Bypass authentication for clients on localhost** is on, a browser on the same computer can still see your torrents.

## Forgot the PIN

You can remove a forgotten PIN when Rookparcel starts. Rookparcel asks **Remove the lock PIN? Rookparcel will open without asking for it.** Answer **Yes** (**OK** on Linux, **Remove** on macOS) and it opens without the PIN. Your torrents and your other settings stay as they are.

**Windows:** hold down <kbd>Ctrl</kbd> while you start Rookparcel, for example while you click its Start menu item or double-click its desktop icon. If Rookparcel is already running, the running copy asks.

**macOS:** quit Rookparcel (**Quit Rookparcel** in the menu-bar icon's menu), then hold down <kbd>⌥</kbd> (Option) while you open it. While Rookparcel runs, holding <kbd>⌥</kbd> while you click its Dock icon asks too.

**Linux and the BSDs:** start it from a terminal with `--reset-lock`. If Rookparcel is already running, the running copy asks.

| Copy | Command |
|---|---|
| Plain binaries (tar.gz) | `rookparcel-gtk --reset-lock` |
| AppImage | `./Rookparcel-<version>-x86_64.AppImage --reset-lock` |
| Flatpak | `flatpak run io.github.balls0fsteel.Rookparcel --reset-lock` |

`--reset-lock` works on Windows and macOS too: `rookparcel.exe --reset-lock`, or on a Mac `/Applications/Rookparcel.app/Contents/MacOS/Rookparcel --reset-lock`. If Rookparcel already runs on the Mac, its question can open behind the window in front: look for it.

With a [profile](Profiles), add its switch: `rookparcel.exe --profile NAME --reset-lock`.

**By hand:** exit Rookparcel, open its `rookparcel.ini` in a text editor, delete the lines that start with `ui.lock_pin=` and `ui.locked=`, and save. [Where Rookparcel keeps its files](Where-Rookparcel-keeps-its-files) says where the file is.

## Profiles and remote windows

Each [profile](Profiles) has its own PIN, or none. So does each window connected to a remote Rookparcel ([Remote control](Remote-control)), since it keeps its own settings.

## GTK version (Linux, BSD)

The lock works as on Windows. On desktops without a tray (plain GNOME without the AppIndicator extension), there is nowhere to hide the windows, so the **Rookparcel is locked** window takes their place. It has no **Cancel** button, and closing it quits Rookparcel, as closing the main window does there.

## macOS

**File > Lock Rookparcel** is <kbd>⌘</kbd>+<kbd>L</kbd>. While Rookparcel is locked:

- the menu bar has only **Show Rookparcel**, **Hide Rookparcel** and **Quit Rookparcel** in the **Rookparcel** menu, and the **Edit** menu;
- the menu-bar icon's menu offers **Show Rookparcel** and **Quit Rookparcel**, and the Dock icon's menu **Show Rookparcel**;
- while the PIN window is open, **Quit Rookparcel** is greyed: click **Cancel** first.
