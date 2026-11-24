# Managing torrents

This page covers what you do with torrents once they are in the list: starting and stopping them, the queue, removing them with or without their files, checking them again, moving their files, labels and tags, per-torrent speed limits, and the right-click menu that holds all of it.

Every action works on the selected torrents. Select several with <kbd>Ctrl</kbd>+click or <kbd>Shift</kbd>+click, or all of them with <kbd>Ctrl</kbd>+<kbd>A</kbd>, and act on them in one go.

## The right-click menu

![The torrent list's right-click menu](images/torrent-menu.png)

Right-click a torrent (or press <kbd>Shift</kbd>+<kbd>F10</kbd>) for this menu:

| Item | Does | Key |
|---|---|---|
| **Open Containing Folder** | opens the torrent's folder in Explorer, with its data selected | |
| **Copy Magnet Link** | puts the torrent's magnet link on the clipboard | |
| **Export .torrent...** | saves the torrent's `.torrent` file | |
| **Force Start** | starts now, outside the queue | |
| **Start** | hands the torrent to the queue | <kbd>Ctrl</kbd>+<kbd>S</kbd> |
| **Pause** | halts it and takes it out of the queue; shows **Paused** | <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd> |
| **Stop** | stops it and takes it out of the queue; shows **Stopped** or **Finished** | <kbd>Ctrl</kbd>+<kbd>T</kbd> |
| **Move Up Queue**, **Move Down Queue** | one place up or down | |
| **Move to Top of Queue**, **Move to Bottom of Queue** | to the top or bottom, keeping their order | |
| **Remove** | removes it from the list; the files stay | <kbd>Del</kbd> |
| **Remove And > Delete Data** | removes it and deletes its files | <kbd>Shift</kbd>+<kbd>Del</kbd> |
| **Force Re-Check** | checks the files on disk again | |
| **Update Tracker** | contacts the trackers now | |
| **Advanced > Set Download Location...** | moves its files to another folder | |
| **Advanced > Initial Seeding** | turns initial seeding on or off | |
| **Advanced > Download in Sequential Order** | downloads the pieces in order, from start to end | |
| **Advanced > Download First and Last Pieces First** | downloads both ends of each wanted file first | |
| **Bandwidth Allocation > Set Download Limit / Set Upload Limit** | its own speed limits | |
| **Label** | gives it a label, a new one, or none | |
| **Tags** | gives it tags or takes them off | |
| **Properties** | trackers, limits, seeding goal and more | <kbd>Enter</kbd> |

The sections below explain each group. Most of these are also on the toolbar; see [The main window](The-main-window#the-toolbar).

## Start, Pause, Stop and Force Start

Rookparcel keeps a queue, as µTorrent does. How many torrents may run at once is set in **Options > Preferences > Queueing**; see [Queueing and seeding](Queueing-and-seeding).

- **Start** hands the torrent to the queue. If there is room, it runs (**Downloading** or **Seeding**). If not, it waits as **Queued** or **Queued Seed** until a place frees up.
- **Force Start** runs the torrent at once, even when the queue is full. Its status starts with **[F]**, for example **[F] Seeding**. A forced torrent never reaches a seeding goal, so it seeds until you stop it.
- **Pause** and **Stop** both halt the torrent and take it out of the queue. The only difference is the word in the Status column: **Paused**, or **Stopped** (**Finished** for a complete torrent). Both are remembered after a restart. Start or Force Start it to carry on.

The tray icon's menu has **Start All Torrents** and **Stop All Torrents**, which act on every torrent in the list.

To add new torrents stopped, see [Adding torrents](Adding-torrents#adding-torrents-stopped).

## The queue

The **#** column shows each torrent's place in the queue. Lower numbers get a turn first. Use **Move Up Queue** and **Move Down Queue** (also on the toolbar), or **Move to Top of Queue** and **Move to Bottom of Queue**, to change the order. Moving several torrents keeps their order among themselves.

In the Add New Torrent window, **Add to top of queue** puts a new torrent first.

## Removing torrents

- **Remove** (<kbd>Del</kbd>) takes the torrents out of the list. The downloaded data stays on disk.
- **Remove And > Delete Data** (<kbd>Shift</kbd>+<kbd>Del</kbd>) takes them out and deletes their downloaded files. On Windows the files skip the Recycle Bin and cannot be recovered from it. A partial file ending in `.!ut` or `.!qB` that Rookparcel cannot show to hold this torrent's data is kept; see [Disk and storage](Disk-and-storage#incomplete-files-and-ut).

With **Confirm when deleting torrents** on (Preferences > UI Settings, on by default), Rookparcel asks first and says whether the files are deleted too. Turned off, Remove acts at once.

## Force Re-Check

**Force Re-Check** reads the torrent's files on disk piece by piece and counts only the pieces that are complete and correct. The status shows **Queued for checking** while the check waits for its drive (checks run one torrent per drive; *new in 1.2.76*), then **Checking** with a percentage. Use it after you moved or restored files by hand, or when you suspect damaged data.

*New in 1.2.76:* a stopped torrent starts with the check; stop it again afterwards if you want it stopped. A queued torrent is checked at once, without waiting for a queue slot, and is queued again after. A torrent held with **files missing** (see [Disk and storage](Disk-and-storage#when-a-drive-or-share-is-missing)) is checked against what is there now, and whatever is missing is downloaded again.

How gently checks use the disk is set in **Preferences > Gaming** (off at first); see [Disk and storage](Disk-and-storage).

## Update Tracker

**Update Tracker** contacts the torrent's trackers now, without waiting for the next announce, to report progress and get more peers. The [Trackers tab](Torrent-details-and-files#trackers) shows the answer.

## Moving the files: Set Download Location

1. Select the torrents.
2. Choose **Advanced > Set Download Location...**.
3. Pick the new folder.

Rookparcel moves the files there and keeps seeding or downloading from the new place. The status shows **Moving** meanwhile, and the item is greyed out for a torrent that is already moving. Files of the same name already in the new folder are replaced.

If you quit while files are moving, Rookparcel waits for the moves to finish and shows their progress. **Quit Now** stops a move: its files stay in the old folder, and what was already copied is deleted. Move it again later.

To move finished downloads automatically, see **Move completed downloads to:** on [Disk and storage](Disk-and-storage).

## Opening the folder

**Open Containing Folder** opens the torrent's folder in Explorer, with its file or folder selected once it exists. With several torrents selected, it opens the first one's. Double-clicking a finished torrent does the same.

To open a single file, use the [Files tab](Torrent-details-and-files#files).

## Copy Magnet Link and Export .torrent

*New in 1.2.63.*

**Copy Magnet Link** puts the selected torrents' magnet links on the clipboard, one per line. Each link holds the torrent's info-hash, its name and its trackers. Paste it into another torrent program, or send it to someone, to share the torrent without a file.

Because the link includes the trackers, do not share a link copied from a private tracker's torrent. A private tracker's address usually holds your personal key (a passkey), and anyone with the link could announce as you.

**Export .torrent...** saves the selected torrents' `.torrent` files. The file holds the torrent as Rookparcel received it, with the trackers and web seeds it uses now. You can add it on another computer, or keep it as a copy.

- With one torrent selected, Rookparcel asks for a file name.
- With several, it asks for a folder and saves each as `<name>.torrent`. When a name is taken, it adds a number, such as ` (2)`, instead of replacing the file.
- A torrent added from a magnet link has no `.torrent` file until its file list has arrived ("Downloading metadata"). The item is greyed out when no selected torrent has one.

If the file you named cannot be written, Rookparcel says "Could not export" with the torrent's name and the reason. Files that fail in a folder export are written to the [Logger tab](Torrent-details-and-files#logger) instead.

## Labels

A label is a name you give torrents to group them, such as "Movies" or "Linux". Each label in use appears under **Labels** in the category list, with a count, so a click shows only those torrents.

- **Label > New Label...** asks for a name and gives it to the selected torrents.
- **Label >** *a label* gives an existing label.
- **Label > Remove Label** takes the label off.

The menu ticks the label the selected torrents share. You can also give a label in the Add New Torrent window, and RSS filters can give one.

With **Append the torrent's label** on next to **Move completed downloads to:** (Preferences > Directories), a finished labelled torrent moves into a folder named after its label. See [Disk and storage](Disk-and-storage).

## Tags

*New in 1.2.63.*

A torrent has one label, but it can have any number of tags besides it, such as "4K", "keep" and "Linux ISOs". Tags work as qBittorrent's do. Each tag appears under **Tags** in the category list, below the labels, with a count; a torrent with several tags is counted under each. **No Tag** shows the torrents without any tag.

- **Tags > New Tag...** asks for a name and gives the new tag to the selected torrents.
- **Tags >** *a tag* gives that tag to the selected torrents. A tag every selected torrent has is ticked; choosing it takes it off them all. A tag only some of them have is not ticked (the macOS app shows a mixed mark), and choosing it gives it to all.
- **Tags > Remove All Tags** takes every tag off the selected torrents. The tags themselves stay.

A tag stays in the category list when no torrent has it any more, with a count of 0. To get rid of it, right-click it in the category list and choose **Delete Tag**: Rookparcel asks "Delete the tag "*name*"? Every torrent that has it loses it." The torrents themselves stay. **New Tag...** in that menu makes a tag without giving it to any torrent.

A name cannot be empty or contain a comma, a tab or a line break. Spaces at either end are dropped, and a name longer than 128 bytes is cut. Names are case-sensitive: `tv` and `TV` are two tags. There can be at most 1024 tags.

The **Tags** column shows a torrent's tags; it is hidden at first (see [The main window](The-main-window#columns)). The search box matches torrent names only, not tags; click a tag in the category list instead. The Add New Torrent window has no tags: give them from the right-click menu once the torrent is in the list. Apps that use the qBittorrent API, such as Sonarr, can give tags too ([Sonarr, Radarr and Prowlarr](Sonarr-Radarr-and-Prowlarr#categories-labels-and-tags)).

## Speed limits for one torrent

**Bandwidth Allocation > Set Download Limit** and **Set Upload Limit** offer **Unlimited**, 10, 25, 50, 100, 250, 500, 1000, 2500 and 5000 kB/s. The current choice is ticked. For any other number, use [Properties](Torrent-details-and-files#torrent-properties). The global limits in Preferences still apply on top. See [Speed limits and scheduler](Speed-limits-and-scheduler).

## Initial Seeding

**Advanced > Initial Seeding** is for a torrent you are the first to share. Each peer gets pieces no one else has yet, so a new torrent spreads faster. It can be turned on only when every selected torrent is seeding. The status shows **[S] Seeding** while it is on. Choose it again to turn it off.

## Download order

*New in 1.2.63.*

By default Rookparcel downloads the pieces that are rarest among the peers first. That keeps every piece available in the swarm and is best for everyone sharing the torrent. Two items under **Advanced** change the order, mainly so you can watch a video while it downloads. Each is ticked while it is on; choose it again to turn it off.

- **Download in Sequential Order** downloads the pieces in order, from the start of the data to its end, so a player can play the video as it arrives. Files with a higher priority still come first. It costs the swarm: the rare pieces you would have passed on arrive late. Use it for a torrent you want to preview, and leave it off otherwise.
- **Download First and Last Pieces First** downloads the first and last pieces of each wanted file before the rest: 1 % of the file at each end, at least one piece. Many video formats keep their header and index there, so a player can open the unfinished file. Skipped files are left alone.

Both are greyed out when every selected torrent is finished. Each torrent keeps its choice after a restart. You can also set them when you add a torrent, in the [Add New Torrent window](Adding-torrents#torrent-options), and in [Properties](Torrent-details-and-files#torrent-properties).

## Properties

**Properties** (<kbd>Enter</kbd>, or double-click a torrent that is not finished) opens the Torrent Properties window: trackers, speed limits, upload slots, the seeding goal, DHT and Peer Exchange, and the download order for this torrent. See [Torrent details and files](Torrent-details-and-files#torrent-properties).

## In the GTK version (Linux, BSD)

- The same right-click menu, in the same order, except that **Copy Magnet Link** and **Export .torrent...** have their own group just before **Advanced**. The macOS app puts them there too.
- Shortcuts: <kbd>Delete</kbd> removes, <kbd>Shift</kbd>+<kbd>Delete</kbd> removes and deletes the files, <kbd>Ctrl</kbd>+<kbd>A</kbd> selects all. Start, Pause and Stop have no shortcut.
- **Open Containing Folder** opens the folder in your file manager.
- On Linux and BSD, deleted files are removed from the disk, not moved to the trash.

## In the Web UI

- The same right-click menu, except that **Open Containing Folder** is greyed out ("not from a browser"), and **Copy Magnet Link** and **Export .torrent...** sit just before **Advanced**. **Tags** lists **New Tag...** and **Remove All Tags** first, then the tags.
- **Export .torrent...** downloads the file through the browser, into its download folder. With several torrents, the browser downloads each file in turn, at most 50 per click.
- **Copy Magnet Link** needs the browser's clipboard access. A page opened over plain `http://` from another computer has none, so the page shows the links instead, under "Copy the link below:", for you to copy by hand.
- **Set Download Location** asks for a folder on the server, typed as a path.
- Deleted files are removed from the disk, not moved to the trash.
- On a phone, the menu leaves out **Open Containing Folder** and starts with **Details**.

## In a remote window

In a window connected with **File > Connect to Remote...**, the files are on the other computer. **Open Containing Folder** is greyed out, and **Set Download Location...** asks for a folder on the server. See [Remote control](Remote-control).

## Related pages

- [The main window](The-main-window)
- [Torrent details and files](Torrent-details-and-files)
- [Queueing and seeding](Queueing-and-seeding)
- [Keyboard shortcuts](Keyboard-shortcuts)
