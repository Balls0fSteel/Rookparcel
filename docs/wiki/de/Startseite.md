*Dies ist eine Übersetzung der englischen Seite [Home](Home). Maßgeblich ist die englische Seite.*

# Rookparcel-Wiki

Rookparcel ist ein schlanker BitTorrent-Client im Geist von µTorrent 2.2.1: klein, schnell und nativ. Dieses Wiki erklärt, wie Sie ihn benutzen: Installation, Torrents hinzufügen und verwalten, die Einstellungen, die Weboberfläche und der Daemon ohne Fenster.

Rookparcel besteht aus vier Programmen:

| Programm | Wo | Was es ist |
|---|---|---|
| `rookparcel.exe` | Windows 10 und 11 | das Windows-Fenster (GUI) |
| `rookparcel-gtk` | Linux, BSD | das GTK-4-Fenster (GUI): dasselbe Hauptfenster und dieselben Dialoge, mit einigen Unterschieden, die auf den jeweiligen Seiten stehen |
| `Rookparcel.app` | macOS 11 oder neuer, Apple Silicon und Intel | das native macOS-Fenster (GUI): dasselbe Hauptfenster und dieselben Dialoge, mit der Menüleiste des Macs und den Tastenkürzeln mit <kbd>⌘</kbd> |
| `rookparcel-nox` | alle | läuft ohne Fenster und wird über seine Weboberfläche in einem Browser gesteuert |

Die meisten Seiten beschreiben das Windows-Fenster und sagen dann, was im GTK-Fenster und in der Weboberfläche anders ist. Die macOS-App funktioniert wie das GTK-Fenster; wo sie abweicht, hat die Seite einen macOS-Abschnitt: [Rookparcel installieren](Rookparcel-installieren#macos), [Tastenkürzel](Tastenkürzel#macos), [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart#macos), [Einstellungen im Überblick](Einstellungen-im-Überblick#unterschiede-unter-macos) und [Automatisches Herunterfahren](Automatisches-Herunterfahren). *Die native macOS-App ist neu in 1.2.19; Versionen davor haben das GTK-Fenster für Apple Silicon unter macOS.*

**Version:** Das Wiki folgt dem aktuellen Code (1.2.81), der auch die neueste veröffentlichte Version ist: [1.2.81](https://github.com/Balls0fSteel/Rookparcel/releases/latest). Die Seiten sagen, in welcher Version eine neuere Funktion hinzugekommen ist.

**Andere Sprachen:** [English](Home) · [Français](Accueil) · [Español](Inicio) · Deutsch · [Magyar](Kezdőlap). Die Übersetzungen folgen diesem englischen Wiki; wo sie abweichen, gilt die englische Seite.

## Erste Schritte

- [Rookparcel installieren](Rookparcel-installieren)
- [Erster Start und der Einrichtungsassistent](Erster-Start)
- [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt) (und der portable Modus)
- [Profile](Profile): mehrere unabhängige Rookparcel nebeneinander
- [Rookparcel aktualisieren](Rookparcel-aktualisieren)

## Täglicher Gebrauch

- [Torrents hinzufügen](Torrents-hinzufügen): Dateien, Magnet-Links, URLs, das Fenster „Neuen Torrent hinzufügen“
- [Das Hauptfenster](Das-Hauptfenster)
- [Torrents verwalten](Torrents-verwalten): starten, stoppen, entfernen, Labels und Tags, Reihenfolge in der Warteschlange
- [Torrent-Details und Dateien](Torrent-Details-und-Dateien): die Detail-Reiter, Dateien auswählen, Torrent-Eigenschaften
- [Torrents erstellen](Torrents-erstellen)
- [Import aus µTorrent und qBittorrent](Import-aus-uTorrent-und-qBittorrent)
- [Torrents suchen](Torrents-suchen) und die Schritt-für-Schritt-Anleitung [Torrents suchen unter Windows](Torrents-suchen-unter-Windows)
- [Downloads per RSS](Downloads-per-RSS)

## Geschwindigkeit, Netzwerk und Datenträger

- [Geschwindigkeitsbegrenzung und Zeitplan](Geschwindigkeitsbegrenzung-und-Zeitplan)
- [Niedrigen Ping halten](Niedrigen-Ping-halten) (Gamer-Modus)
- [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall), VPNs und Proxys
- [Warteschlange und Seeding](Warteschlange-und-Seeding)
- [Datenträger und Speicher](Datenträger-und-Speicher)
- [Automatisches Herunterfahren](Automatisches-Herunterfahren)

## Fernzugriff und Betrieb ohne Fenster

- [Weboberfläche](Weboberfläche)
- [Fernsteuerung](Fernsteuerung): ein Desktop-Fenster steuert ein Rookparcel auf einem anderen Computer
- [rookparcel-nox betreiben](rookparcel-nox-betreiben): der Daemon ohne Fenster und Dienste
- [Docker](Rookparcel-mit-Docker): rookparcel-nox als Container-Image
- [Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr)

## Referenz

- [Einstellungen im Überblick](Einstellungen-im-Überblick)
- [Dunkler Modus](Dunkler-Modus)
- [Tastenkürzel](Tastenkürzel)
- [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart)
- [Rookparcel sperren](Rookparcel-sperren): die Fenster hinter einer PIN verbergen und eine vergessene PIN entfernen
- [Fehlerbehebung und FAQ](Fehlerbehebung-und-FAQ)

## Mehr

- [Website](https://balls0fsteel.github.io/Rookparcel/): Downloads, Neuigkeiten, FAQ, Benchmarks
- [Releases](https://github.com/Balls0fSteel/Rookparcel/releases)
- [Fehlerberichte und Fragen](https://github.com/Balls0fSteel/Rookparcel/issues)
- Entwicklerdokumentation: der Ordner [docs](https://github.com/Balls0fSteel/Rookparcel/tree/main/docs) im Repository
