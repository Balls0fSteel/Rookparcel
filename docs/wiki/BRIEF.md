# Wiki writing brief (for the agents writing docs/wiki)

`docs/wiki/` is the source of the GitHub wiki (https://github.com/Balls0fSteel/Rookparcel/wiki).
Each `Page-Name.md` here becomes the wiki page `Page-Name`. Images go in `docs/wiki/images/`.
This file is not copied to the wiki.

## Audience and goal

End users of Rookparcel, not developers. Task-oriented: "how do I...". Cover the Windows GUI
(`rookparcel.exe`) first, then say what differs in the GTK GUI (Linux, BSD), the macOS app
(`Rookparcel.app`) and the Web UI (`rookparcel-nox`'s page, or the one a desktop window hosts). Do not describe code.

## Accuracy rules (most important)

- Every claim must be read in the code or the docs. Menu, button, dialog and setting names
  are copied exactly from the English strings in `crates/frontend/src/lang/en.rs`
  (and `help_en.rs` / `docs/HELP_TEXTS.md` for what a setting does). Do not guess a name.
- Where a behaviour is only in the code, read the code (`crates/win32/src`, `crates/gtk/src`,
  `crates/cocoa/src`, `crates/frontend/src`, `crates/nox/src`). If you cannot confirm something, leave it out.
- The wiki describes the current code, version 1.2.80; the latest published release is
  1.2.80. When a feature is newer than 1.1.40, say so in one line, e.g. "*New in 1.1.59.*";
  when it is newer than the latest published release, add "not in a published release yet"
  (and take that out at the next release). No release was published between 1.1.40 and
  1.2.0, so 1.1.41-1.1.63 features first shipped in 1.2.0. Find the version in CHANGELOG.md or
  `docs/archive/` (search by topic), or in `docs/DECISIONS.md`.
- Do not mention internal decision IDs (D42 etc.), gates, agents, or the development plan.
- Leave out unfinished or proposal-only features (e.g. the Microsoft Store flavour).

## Style

- Model: `docs/SEARCH_ON_WINDOWS.md` (read it). Plain, calm, short sentences, second person.
  No marketing words, no emoji, no exclamation marks.
- Start each page with one or two sentences on what it covers. Then `##` sections, numbered
  steps where the user follows a sequence, tables for option lists.
- UI names in **bold**, menu paths as **Options > Preferences > Connection**, keys as
  <kbd>Ctrl</kbd>+<kbd>N</kbd>, file paths and commands in `code`.
- Links to other wiki pages: `[Adding torrents](Adding-torrents)` (no `.md`, no folder).
  Link to anchors as `[text](Page-Name#section-name)`. Link generously between pages, but only
  to page names in the list below.
- Links to repository files use full URLs: `https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/DAEMON.md`.
- Images: `![Alt text](images/name.png)`, only names from the image list below. Not every
  image must be used; do not invent new image names.
- Length: as long as the task needs; most pages 80-250 lines. No filler.

## Page list and owners

| Page file | Owner | Covers |
|---|---|---|
| Home.md, _Sidebar.md, _Footer.md | lead | index |
| Installing-Rookparcel.md | A | downloads per platform, installer vs zip, SmartScreen/Gatekeeper, AppImage/Flatpak/tar, uninstalling |
| First-start-and-the-Setup-Guide.md | A | the Setup Guide window and what each step sets; skipping; running it again |
| Where-Rookparcel-keeps-its-files.md | A | settings/torrent folders per platform, portable mode, moving to another PC, backing up, reset |
| Profiles.md | A | several independent Rookparcels (profiles), portable profiles, command-line switches |
| Updating-Rookparcel.md | A | the update check, self-update from the installer, beta channel, turning it off, nox |
| Adding-torrents.md | B | .torrent files, magnet links, URLs, drag and drop, browser/file associations, the Add New Torrent window, adding stopped, watched/auto-load folder if any |
| The-main-window.md | B | layout: categories/labels list, torrent list, columns, toolbar, status bar, detail tabs overview, list search box (filter), sorting, tray |
| Managing-torrents.md | B | start/stop/pause/force start, remove (and with data), recheck, move storage, labels, tags, queue order, copy magnet, open folder, multi-select, the right-click menu |
| Torrent-details-and-files.md | B | each detail tab (General, Trackers, Peers, Pieces, Files, Speed, Logger as they exist), file priorities, skipping files, the Files tab filter and menu, Torrent Properties dialog (trackers, limits, ratio) |
| Creating-torrents.md | B | Create New Torrent window, v1/v2/hybrid, trackers, private, piece size, seeding it |
| Importing-from-uTorrent-and-qBittorrent.md | B | the importers |
| Speed-limits-and-scheduler.md | C | global limits, per-torrent limits, the status-bar menus, scheduler grid, anything alternative-speed-like that exists |
| Keep-ping-low.md | C | the keep-ping-low / gamer mode feature: what it does, turning it on, settings |
| Connection-ports-and-firewall.md | C | listening port, UPnP/NAT-PMP, firewall exception, checking the port, uTP/TCP, "Use only this network" (VPN binding), proxies, encryption, IP filter, DHT/PEX/LSD |
| Queueing-and-seeding.md | C | active downloads/seeds limits, seed ratio/time goals, what happens when reached, super-seeding, move completed downloads |
| Disk-and-storage.md | C | smart disk I/O, cache, pre-allocation, move completed, label folders, incomplete-file options that exist |
| Auto-shutdown.md | C | the Auto Shutdown dialog and actions |
| Web-UI.md | D | turning on the Web UI in the Windows/GTK window (Preferences > Web UI) and in nox; signing in; HTTPS; what the page can do; phones |
| Remote-control.md | D | File > Connect to Remote, `--remote`, what works remotely, security |
| Running-rookparcel-nox.md | D | the headless daemon: commands, flags, Windows service, systemd, FreeBSD, files, stopping |
| Docker.md | D | the rookparcel-nox container image: docker run, Compose, volumes, ports, the password variables, user IDs, *arr containers, stopping, updating |
| Sonarr-Radarr-and-Prowlarr.md | D | using nox or the hosted Web UI as a qBittorrent-type download client, categories, paths |
| Searching-for-torrents.md | E | search box filter vs source search, Manage Sources, the search helper on all platforms, Prowlarr/Jackett, adding results; links to the existing page Searching-for-torrents-on-Windows |
| RSS-Downloader.md | E | feeds, filters, episode matching, downloading from feeds, in the Web UI too |
| Preferences-reference.md | F | every Preferences page with a table of its settings and one-line meanings (from HELP_TEXTS.md), plus the Advanced table summary; GTK/Web UI differences |
| Keyboard-shortcuts.md | F | every shortcut in the Windows GUI (and GTK where different), plus F1 help |
| Tray-notifications-and-startup.md | F | tray icon and its menu, close/minimize to tray, notifications, start with the computer, start minimized, file associations |
| Locking-Rookparcel.md | F | the windows' PIN lock: setting a PIN, locking, opening a locked Rookparcel, what still works, removing a forgotten PIN on each platform |
| Troubleshooting-and-FAQ.md | F | slow or no downloads, port not reachable, firewall, SmartScreen, private trackers (client ID not whitelisted), where logs are, resetting settings, reporting a bug, getting help |

Existing page, do not edit: `Searching-for-torrents-on-Windows` (source `docs/SEARCH_ON_WINDOWS.md`).

## Images (made by agent S from the Windows GUI in demo mode, simulated torrents)

| File | Shows |
|---|---|
| main-window.png | the main window with torrents, a detail tab open |
| add-new-torrent.png | the Add New Torrent window |
| add-url.png | the Add Torrent from URL window |
| torrent-menu.png | the right-click menu of the torrent list |
| tab-general.png | the General detail tab |
| tab-files.png | the Files detail tab |
| tab-peers.png | the Peers detail tab |
| tab-trackers.png | the Trackers detail tab |
| properties.png | the Torrent Properties dialog |
| create-torrent.png | the Create New Torrent window |
| setup-guide.png | the Setup Guide's first page |
| prefs-general.png | Preferences > General |
| prefs-connection.png | Preferences > Connection |
| prefs-bandwidth.png | Preferences > Bandwidth |
| prefs-gaming.png | Preferences > Gaming |
| prefs-bittorrent.png | Preferences > BitTorrent |
| prefs-queueing.png | Preferences > Queueing |
| prefs-scheduler.png | Preferences > Scheduler |
| prefs-webui.png | Preferences > Web UI |
| prefs-advanced.png | Preferences > Advanced |
| rss-downloader.png | the RSS Downloader window |
| auto-shutdown.png | the Auto Shutdown window |
| search-sources.png | the Search Sources window |

## Translations (French, Spanish, German, Hungarian)

*Added 2026-11-22_205639_1795377399.* Each language has a folder: `docs/wiki/fr/`, `es/`, `de/`,
`hu/`. GitHub flattens folders, so every page name must be unique across the whole wiki; use
exactly the names in the table below (file = name + `.md`). Each folder has its own
`_Sidebar.md` and `_Footer.md` (GitHub uses the nearest one), translated from the English ones.

- Translate the current English page in full: same sections, steps, tables, notes and
  "*New in ...*" lines. Do not add, drop or "improve" facts; if the English looks wrong, report it.
- UI names in **bold** use the app's own translation: look up the English string's key in
  `crates/frontend/src/lang/en.rs` (or `en.lng`) and copy that key's text from
  `crates/frontend/lang/<code>.lng`, dropping the `&` accelerator marks and the trailing `...`
  only where the English page drops them. If a key is missing there, keep the English name.
  Help texts (`help.<Key>=` lines) are in the same `.lng` files.
- Keep code, commands, flags, paths, file names, keys (<kbd>Ctrl</kbd>), URLs, image links and
  `#anchors` into English pages unchanged. Links between translated pages use the translated
  page names; an anchor into a translated page uses that page's translated heading
  (GitHub: lower case, spaces to `-`, punctuation dropped, accents kept).
- Each translated page starts with one line in italics, in the language, saying it is a
  translation of the English page (linked) and that the English page is authoritative.
- Natural, plain, formal-polite register: French *vous*, Spanish *usted*-free neutral
  imperative (*haga clic* style is fine, be consistent), German *Sie*, Hungarian magázó/neutral
  ("kattintson"). Same calm style as the English.
- The English `Home.md` and `_Sidebar.md` link each language's home page.

| English page | fr | es | de | hu |
|---|---|---|---|---|
| Home | Accueil | Inicio | Startseite | Kezdőlap |
| Installing-Rookparcel | Installer-Rookparcel | Instalar-Rookparcel | Rookparcel-installieren | A-Rookparcel-telepítése |
| First-start-and-the-Setup-Guide | Premier-démarrage | Primer-inicio | Erster-Start | Első-indítás |
| Where-Rookparcel-keeps-its-files | Emplacement-des-fichiers-de-Rookparcel | Dónde-guarda-Rookparcel-sus-archivos | Wo-Rookparcel-seine-Dateien-ablegt | Hol-tárolja-a-Rookparcel-a-fájljait |
| Profiles | Profils | Perfiles | Profile | Profilok |
| Updating-Rookparcel | Mettre-à-jour-Rookparcel | Actualizar-Rookparcel | Rookparcel-aktualisieren | A-Rookparcel-frissítése |
| Adding-torrents | Ajouter-des-torrents | Añadir-torrents | Torrents-hinzufügen | Torrentek-hozzáadása |
| The-main-window | La-fenêtre-principale | La-ventana-principal | Das-Hauptfenster | A-főablak |
| Managing-torrents | Gérer-les-torrents | Gestionar-torrents | Torrents-verwalten | Torrentek-kezelése |
| Torrent-details-and-files | Détails-et-fichiers-des-torrents | Detalles-y-archivos-del-torrent | Torrent-Details-und-Dateien | Torrent-részletek-és-fájlok |
| Creating-torrents | Créer-des-torrents | Crear-torrents | Torrents-erstellen | Torrentek-készítése |
| Importing-from-uTorrent-and-qBittorrent | Importer-depuis-uTorrent-et-qBittorrent | Importar-desde-uTorrent-y-qBittorrent | Import-aus-uTorrent-und-qBittorrent | Importálás-uTorrentből-és-qBittorrentből |
| Searching-for-torrents | Rechercher-des-torrents | Buscar-torrents | Torrents-suchen | Torrentek-keresése |
| Searching-for-torrents-on-Windows | Rechercher-des-torrents-sous-Windows | Buscar-torrents-en-Windows | Torrents-suchen-unter-Windows | Torrentek-keresése-Windowson |
| RSS-Downloader | Téléchargeur-RSS | Descargador-RSS | Downloads-per-RSS | RSS-letöltő |
| Speed-limits-and-scheduler | Limites-de-vitesse-et-planificateur | Límites-de-velocidad-y-programador | Geschwindigkeitsbegrenzung-und-Zeitplan | Sebességkorlátok-és-ütemező |
| Keep-ping-low | Garder-un-ping-bas | Mantener-el-ping-bajo | Niedrigen-Ping-halten | Alacsony-ping |
| Connection-ports-and-firewall | Connexion-ports-et-pare-feu | Conexión-puertos-y-cortafuegos | Verbindung-Ports-und-Firewall | Kapcsolat-portok-és-tűzfal |
| Queueing-and-seeding | Mise-en-file-et-partage | Cola-y-siembra | Warteschlange-und-Seeding | Várólista-és-seedelés |
| Disk-and-storage | Disque-et-stockage | Disco-y-almacenamiento | Datenträger-und-Speicher | Lemez-és-tárolás |
| Auto-shutdown | Arrêt-automatique | Apagado-automático | Automatisches-Herunterfahren | Automatikus-leállítás |
| Web-UI | Interface-Web | Interfaz-web | Weboberfläche | Webes-felület |
| Remote-control | Contrôle-à-distance | Control-remoto | Fernsteuerung | Távvezérlés |
| Running-rookparcel-nox | Utiliser-rookparcel-nox | Ejecutar-rookparcel-nox | rookparcel-nox-betreiben | A-rookparcel-nox-futtatása |
| Docker | Rookparcel-avec-Docker | Rookparcel-en-Docker | Rookparcel-mit-Docker | Rookparcel-Dockerben |
| Sonarr-Radarr-and-Prowlarr | Sonarr-Radarr-et-Prowlarr | Sonarr-Radarr-y-Prowlarr | Sonarr-Radarr-und-Prowlarr | Sonarr-Radarr-és-Prowlarr |
| Preferences-reference | Référence-des-préférences | Referencia-de-preferencias | Einstellungen-im-Überblick | A-beállítások-leírása |
| Dark-mode | Mode-sombre | Modo-oscuro | Dunkler-Modus | Sötét-mód |
| Keyboard-shortcuts | Raccourcis-clavier | Atajos-de-teclado | Tastenkürzel | Billentyűparancsok |
| Tray-notifications-and-startup | Zone-de-notification-et-démarrage | Bandeja-notificaciones-e-inicio | Infobereich-Benachrichtigungen-und-Autostart | Tálca-értesítések-és-indítás |
| Locking-Rookparcel | Verrouiller-Rookparcel | Bloquear-Rookparcel | Rookparcel-sperren | A-Rookparcel-zárolása |
| Troubleshooting-and-FAQ | Dépannage-et-FAQ | Solución-de-problemas-y-preguntas-frecuentes | Fehlerbehebung-und-FAQ | Hibaelhárítás-és-GYIK |

The source of Searching-for-torrents-on-Windows is `docs/SEARCH_ON_WINDOWS.md`; its
translations live in the language folders like the rest. Its images are in `docs/screenshots/`,
not the wiki: write them as full URLs,
`https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/<name>.png`.
