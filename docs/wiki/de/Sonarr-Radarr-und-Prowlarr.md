*Dies ist eine Übersetzung der englischen Seite [Sonarr, Radarr and Prowlarr](Sonarr-Radarr-and-Prowlarr). Maßgeblich ist die englische Seite.*

# Sonarr, Radarr und Prowlarr

Sonarr, Radarr und Prowlarr können Rookparcel als Download-Client verwenden. Die Web-UI von Rookparcel beantwortet dieselbe API wie die von qBittorrent, sodass Sie Rookparcel in diesen Apps als **qBittorrent**-Client hinzufügen. Diese Seite beschreibt die Einrichtung mit `rookparcel-nox` oder mit der eigenen Web-UI des Windows-Programms, Kategorien, Download-Ordner und das Entfernen fertiger Downloads.

Um Ihr Prowlarr oder Jackett von Rookparcel aus zu durchsuchen (die andere Richtung), siehe [Torrents suchen](Torrents-suchen).

## Was funktioniert

Diese Punkte wurden gegen die echten Apps geprüft (Sonarr 4, Radarr und Prowlarr in Docker), sowohl mit `rookparcel-nox` unter Linux als auch mit der Web-UI des Windows-Programms:

- Der **Test** des Download-Clients in jeder App.
- Grabs von Sonarr, Radarr und Prowlarr, jeweils in eine eigene Kategorie.
- Die Seed-Zeit eines Indexers, die mit dem Torrent eintrifft und ihn rechtzeitig stoppt.
- Sonarr und Radarr, die die fertige Datei importieren (per Hardlink oder über eine Remote-Pfad-Zuordnung) und den Torrent dann mit **Remove Completed** entfernen.
- Suche und Grab von Prowlarr.

Nicht geprüft: Lidarr und Readarr (Lidarr verwendet denselben Client-Code wie Radarr) und Grabs von Magnet-Links.

## 1. Die Web-UI von Rookparcel einschalten

**Mit rookparcel-nox:**

```sh
rookparcel-nox run --save /srv/media/downloads --webui 0.0.0.0:8080
```

Melden Sie sich einmal in einem Browser mit dem vorläufigen Passwort an, das nox ausgibt, und legen Sie Ihr eigenes in den **Einstellungen > Web-UI** fest. Einzelheiten: [Weboberfläche](Weboberfläche#in-rookparcel-nox-einschalten) und [rookparcel-nox betreiben](rookparcel-nox-betreiben).

`0.0.0.0` lässt andere Computer und Container es erreichen. Laufen die Apps auf demselben Computer, nicht in Containern, genügt `--webui 8080` (nur dieser Computer).

**Mit dem Windows-Programm:** **Optionen > Einstellungen > Web-UI**, haken Sie **Web-UI aktivieren** an, legen Sie einen **Benutzername** und ein **Passwort** fest und haken Sie **Verbindungen von anderen Computern zulassen** an, wenn die Apps auf einem anderen Computer laufen. Die Windows-Firewall braucht dann eine Regel für `rookparcel.exe`. Einzelheiten: [Weboberfläche](Weboberfläche#im-windows-programm-einschalten). Rookparcel muss laufen, damit die Apps es erreichen.

## 2. Rookparcel in der App hinzufügen

Fügen Sie in Sonarr, Radarr oder Prowlarr einen neuen Download-Client vom Typ **qBittorrent** hinzu und füllen Sie aus:

| Feld | Wert |
|---|---|
| Host | Die IP-Adresse des Computers, auf dem Rookparcel läuft (oder ein Hostname, den Sie erlaubt haben, siehe [unten](#hostnamen-und-docker)). |
| Port | Der Port der Web-UI, etwa `8080`. |
| SSL | Aus, außer die Web-UI verwendet HTTPS mit einem Zertifikat, dem die App vertraut. Die oben genannten Prüfungen wurden über einfaches HTTP durchgeführt. |
| Username, Password | Die Anmeldung der Web-UI. |
| Category | Eine Kategorie Ihrer Wahl, etwa `tv-sonarr`, `radarr` oder `prowlarr`. |

Klicken Sie auf **Test** und speichern Sie dann. Der Test legt die Kategorie in Rookparcel an, wenn sie noch nicht existiert.

Statt Benutzername und Passwort können die Apps einen API-Schlüssel senden. Schalten Sie **API-Schlüssel akzeptieren (Authorization: Bearer)** in den **Einstellungen > Web-UI** von Rookparcel ein, geben Sie dort einen Schlüssel von mindestens 16 Zeichen ein und geben Sie der App denselben Schlüssel. Ein dritter Weg ist **Authentifizierung für Clients in diesen Subnetzen umgehen (...)** mit dem Netzwerk der App, was jeden Client darin ohne Anmeldung hineinlässt: Tragen Sie nur Netzwerke ein, denen Sie so sehr vertrauen wie dem Passwort ([Sicherheit der Web-UI](Weboberfläche#sicherheit)).

## Hostnamen und Docker

Rookparcel antwortet nur auf IP-Adressen und `localhost`, sofern Sie nicht weitere Namen erlauben. Eine App, die es unter einem Namen erreicht, etwa `http://rookparcel:8080` aus einem Docker-Container oder dem Netzwerknamen eines PCs, bekommt „Bad Request: unexpected Host header“, und ihr Test meldet einen Verbindungsfehler.

Verwenden Sie entweder die IP-Adresse in der App oder fügen Sie den Namen zu **Server-Domains (mehrere Einträge durch Semikolon trennen)** hinzu:

- In der Web-UI von nox: **Einstellungen > Web-UI**, unter **Erreichbarkeit**. Das Ändern dort verlangt das **Aktuelle Passwort**.
- Im Windows-Programm: **Optionen > Einstellungen > Web-UI**.

Mehrere Namen werden mit `;` getrennt; `*` passt auf alles. Nennen Sie lieber die Hosts, statt `*` zu verwenden: Die Prüfung schützt vor Webseiten, die die Web-UI unter einem eigenen Namen erreichen.

Um nox selbst in einem Container neben den Apps zu betreiben, siehe [Rookparcel mit Docker](Rookparcel-mit-Docker): sein Image, eine Compose-Datei mit dem gemeinsamen Medienordner und die Benutzer-IDs.

## Kategorien, Labels und Tags

Eine qBittorrent-Kategorie ist in Rookparcel ein **Label**. Die Torrents, die eine App hinzufügt, bekommen deren Kategorie als Label, sodass Sie sie unter diesem Label in der Kategorienliste sehen ([Torrents verwalten](Torrents-verwalten)). Ändern Sie das Label eines Torrents in Rookparcel, ändert sich auch seine Kategorie für die App; eine App verfolgt nur Torrents in ihrer eigenen Kategorie.

Gibt die App keinen eigenen Download-Ordner an, kommt ein Torrent in den Speicherordner seiner Kategorie, wenn die Kategorie einen hat, und sonst in den Standard-Download-Ordner von Rookparcel.

Ein qBittorrent-Tag ist in Rookparcel ein **Tag** (*neu in 1.2.63*; zuvor wurden Tags angenommen und verworfen). Ein Torrent kann beliebig viele davon neben seinem Label haben; sie erscheinen unter **Tags** in der Kategorienliste und in der Spalte **Tags** ([Torrents verwalten](Torrents-verwalten#tags)). Ist **Add Series Tags** in Sonarr an, bekommt ein Grab die Tags der Serie. Radarr sendet keine Tags. Ein Name mit einem Komma oder einem Steuerzeichen wird ohne Fehler übersprungen, und bei Namen wird zwischen Groß- und Kleinschreibung unterschieden, wie in qBittorrent.

## Download-Ordner und Pfade

Die App importiert fertige Dateien, indem sie sie von dem Pfad liest, den Rookparcel meldet. Dieser Pfad muss für beide dieselbe Datei bedeuten:

- **Derselbe Computer, keine Container:** Nichts zu tun.
- **Container:** Binden Sie den Download-Ordner im Container der App unter demselben Pfad ein, den Rookparcel verwendet, zum Beispiel in beiden `/srv/media`. Dann funktionieren auch Hardlinks.
- **Verschiedene Computer** (zum Beispiel Rookparcel unter Windows, das auf eine Freigabe auf dem Server speichert, auf dem Sonarr läuft): Fügen Sie in Sonarr und Radarr eine **Remote Path Mapping** vom Pfad von Rookparcel zum Pfad der App hinzu, etwa `\\server\media\downloads\` zu `/srv/media/downloads/`. Das war die geprüfte Konfiguration mit dem Windows-Programm.

Bei nox begrenzt `--webui-save-root DIR` die Ordner, in die die Web-UI und damit die Apps speichern dürfen ([rookparcel-nox betreiben](rookparcel-nox-betreiben#web-ui)). Nehmen Sie die Ordner auf, die die Apps verwenden.

## Unerwünschte Dateien auslassen

*Neu in 1.2.63.* **Dateien mit diesen Namen nicht herunterladen** in den **Einstellungen > Oberfläche** von Rookparcel gilt auch für die Grabs der Apps. Ist es an, werden Dateien, deren Namen zu den Mustern passen, etwa `*.lnk; Thumbs.db; *sample*`, so gesetzt, dass sie nicht heruntergeladen werden, wenn die App den Torrent hinzufügt; bei einem Magnet-Link, sobald seine Dateiliste eingetroffen ist. Ein Torrent, bei dem jede Datei passt, wird ganz heruntergeladen. Siehe [Torrents hinzufügen](Torrents-hinzufügen#dateien-nach-namen-auslassen).

Apps, die qBittorrents „Excluded file names“ über seine API setzen, erreichen dieselbe Einstellung: `excluded_file_names_enabled` und `excluded_file_names` (ein Muster pro Zeile).

## Seeding-Grenzen und Remove Completed

**Remove Completed** in Sonarr und Radarr entfernt einen Torrent, sobald er gestoppt ist und seine Seeding-Grenze erreicht ist. Rookparcel meldet beides:

- **Ein am Indexer festgelegtes Seed-Verhältnis oder eine Seed-Zeit** (in Sonarr, Radarr oder Prowlarr) trifft mit jedem Torrent ein und wird das eigene Seeding-Ziel dieses Torrents. Ein Wert, den der Indexer leer lässt, wird aus den Einstellungen von Rookparcel genommen.
- **Das eigene Seeding-Ziel von Rookparcel** steht in den **Einstellungen > Warteschlange**, unter **Seeding-Ziel**: „Mindestverhältnis (%): [0: kein Verhältnisziel]“ (standardmäßig 150 %) und „Mindest-Seedingzeit (Minuten): [0: kein Zeitziel]“. Erreicht ein Torrent es, stoppt Rookparcel ihn, und die App kann ihn entfernen. Siehe [Warteschlange und Seeding](Warteschlange-und-Seeding).

Zwei Dinge sind zu wissen:

- Ist **Upload-Rate begrenzen auf (kB/s): [0: stoppen]** an, seedet ein Torrent nach seinem Ziel weiter, sodass die App ihn nie als fertig sieht und nicht entfernt.
- Das Ziel von Rookparcel ist erreicht, wenn jedes gesetzte Ziel erreicht ist (Verhältnis und Zeit), wie bei µTorrent; qBittorrent stoppt beim ersten. Ein Torrent kann also etwas länger seeden, als die App erwartet, wird aber nie entfernt, bevor beide erfüllt sind.

Ein Torrent, den Sie mit **Start erzwingen** starten, erreicht sein Seeding-Ziel nie.

## Prowlarr

Prowlarr fügt nur Torrents hinzu; es verfolgt oder importiert sie nicht. Fügen Sie Rookparcel in Prowlarr auf dieselbe Weise hinzu (ein qBittorrent-Client mit eigener Kategorie, etwa `prowlarr`), um Grabs aus der eigenen Suche von Prowlarr an Rookparcel zu senden.

## Bekannte Grenzen

- Rookparcel meldet sich als Version 2.11.2 der API von qBittorrent.
- Ignoriert, wenn eine App sie sendet: die Wahl des Inhaltslayouts und das Limit der inaktiven Seeding-Zeit von Sonarr. „Erste und letzte Teile zuerst“ wird seit 1.2.63 beachtet; davor wurde es ignoriert.
- Die Anforderung einer erneuten Prüfung durch eine App behält das Verhalten von qBittorrent: Ein gestoppter Torrent bleibt gestoppt und wird geprüft, wenn er startet. **Erneut prüfen** in der Web-UI und den Fenstern startet einen gestoppten Torrent seit 1.2.76.
- Das Hinzufügen eines Torrents, der schon in der Liste ist, wird als Erfolg beantwortet; die App verfolgt dann den vorhandenen.
- Die API zeigt `ip_filter_enabled` und `ip_filter_path` des IP-Filters an, ändert sie aber nicht, und sie hat keine Einstellung für die Web-Adresse der Sperrliste. Legen Sie diese in den eigenen Einstellungen von Rookparcel fest.

## Wenn etwas schiefgeht

| Was Sie sehen | Was es bedeutet |
|---|---|
| Der Test schlägt fehl, und die Adresse von Rookparcel ist ein Name | Die Host-Prüfung hat den Namen abgelehnt. Verwenden Sie die IP-Adresse oder fügen Sie den Namen zu **Server-Domains** hinzu ([oben](#hostnamen-und-docker)). |
| Der Test schlägt mit einer IP-Adresse fehl | Die Web-UI ist nicht an, lauscht nur auf `127.0.0.1` (`--webui 8080` von nox oder **Verbindungen von anderen Computern zulassen** aus), eine Firewall blockiert den Port, oder die Anmeldung ist falsch. Zu viele falsche Anmeldungen sperren die Adresse der App für eine Stunde. |
| Der Import schlägt fehl: Die Datei wird nicht gefunden | Die App sieht den Download unter einem anderen Pfad. Verwenden Sie denselben Mount-Pfad oder eine Remote Path Mapping ([oben](#download-ordner-und-pfade)). |
| Fertige Torrents bleiben in der Warteschlange der App | Rookparcel seedet sie weiter: Es ist kein Seeding-Ziel gesetzt, das Ziel ist noch nicht erreicht, oder **Upload-Rate begrenzen auf (kB/s): [0: stoppen]** ist an. |

## Siehe auch

- [Weboberfläche](Weboberfläche): einschalten, HTTPS und Sicherheit.
- [rookparcel-nox betreiben](rookparcel-nox-betreiben): nox als Dienst, der mit dem Computer startet.
- [Rookparcel mit Docker](Rookparcel-mit-Docker): nox in einem Container.
- Die technischen Hinweise dazu, was die Apps aufrufen: [INTEGRATIONS.md, part 1](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/INTEGRATIONS.md#part-1-the-shim-as-an-arr-download-client).
