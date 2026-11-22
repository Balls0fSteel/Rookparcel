*Dies ist eine Übersetzung der englischen Seite [RSS Downloader](https://github.com/Balls0fSteel/Rookparcel/wiki/RSS-Downloader). Maßgeblich ist die englische Seite.*

# RSS-Downloader

Diese Seite erklärt, wie Sie in Rookparcel RSS-Feeds verfolgen und daraus herunterladen: Feeds hinzufügen, Einträge von Hand herunterladen, Filter, die passende Einträge automatisch herunterladen, Episoden- und Qualitätsabgleich und den Verlauf. Sie behandelt zuerst das Windows-Fenster, dann das GTK-Fenster (Linux, BSD) und die Web-UI.

Der RSS-Downloader funktioniert wie der von µTorrent 2.2.1. Viele Torrent-Seiten und Tracker veröffentlichen einen Feed ihrer neuen Releases; Rookparcel prüft Ihre Feeds alle 15 Minuten und fügt hinzu, was Ihre Filter verlangen.

## Den RSS-Downloader öffnen

- **Optionen > RSS-Downloader...**
- Die Schaltfläche **RSS-Downloader** in der Symbolleiste.
- Klicken Sie in der Kategorienliste links mit der rechten Maustaste auf **Feeds** (oder auf einen Feed) und wählen Sie **RSS-Downloader...**.

![Das Fenster „RSS-Downloader“](images/rss-downloader.png)

Das Fenster hat links die Liste **Feeds**, darunter **Hinzufügen...**, **Bearbeiten...**, **Entfernen** und **Aktualisieren**, und rechts zwei Reiter: **Filter** und **Verlauf**. Änderungen wirken sofort; es gibt kein OK oder Abbrechen, nur **Schließen**. Wenn Sie das Fenster schließen, laufen die Filter über die aktuellen Einträge der Feeds, sodass ein neuer Filter sofort herunterladen kann.

## Einen Feed hinzufügen

1. Klicken Sie im RSS-Downloader unter **Feeds** auf **Hinzufügen...**. (Oder klicken Sie in der Kategorienliste des Hauptfensters mit der rechten Maustaste auf **Feeds** und wählen **RSS-Feed hinzufügen...**.)
2. **Feed-URL:** die Adresse des Feeds. Liegt eine Feed-Adresse in der Zwischenablage, wird sie für Sie eingetragen.
3. **Eigener Alias:** ein Name für den Feed. Bleibt er leer, wird der eigene Titel des Feeds angezeigt.
4. Wählen Sie unter **Abonnement** eines davon:
   - **Nicht automatisch alle Einträge herunterladen** (die übliche Wahl): Nur Ihre Filter oder Sie laden aus diesem Feed herunter.
   - **Alle im Feed veröffentlichten Einträge automatisch herunterladen**: Jeder neue Eintrag wird heruntergeladen. **Intelligenten Episodenfilter verwenden** darunter lädt jede Episode nur einmal herunter (siehe [Intelligenter Episodenfilter](#intelligenter-episodenfilter)).
5. Klicken Sie auf **OK**. Der Feed wird sofort abgerufen.

Ist **Alle im Feed veröffentlichten Einträge automatisch herunterladen** gewählt, werden Ihre Filter für diesen Feed nicht verwendet: Alles Neue wird genommen.

**Feeds, die einen Login brauchen.** Die Feeds mancher privater Tracker brauchen die Cookies Ihres Logins. Hängen Sie sie wie µTorrent hinter `:COOKIE:` an die URL an:

```
https://tracker.example/rss.php:COOKIE:uid=12345;pass=abcdef
```

Die Cookies werden nur an die eigene Seite des Feeds geschickt (denselben Host oder eine seiner Subdomains) und nie von einem `https`-Feed an einen einfachen `http`-Link.

## Feeds verwalten

Die Liste **Feeds** beginnt mit **Alle Feeds**, dann folgt jeder Feed. Unter der Liste zeigt eine Zeile den Zustand des ausgewählten Feeds: „Aktualisiert“ mit der Uhrzeit und der Anzahl der Einträge, „Noch nicht aktualisiert“, „Deaktiviert“ oder „Fehler:“ und der Grund. Bei ausgewähltem **Alle Feeds** zeigt sie, wie viele Feeds Sie haben.

| Schaltfläche oder Menüeintrag | Was sie tut |
|---|---|
| **Bearbeiten...** / **Feed bearbeiten...** | Ändert URL, Alias oder Abonnement. Eine neue URL ist ein neuer Feed: Seine alten Einträge werden verworfen und er wird erneut abgerufen. |
| **Entfernen** / **Feed löschen** | Entfernt den Feed, nach Rückfrage. Filter, die auf diesen Feed eingestellt sind, bleiben und treffen auf nichts zu, bis Sie sie auf einen anderen Feed richten. |
| **Aktualisieren** / **Feed aktualisieren** | Ruft den ausgewählten Feed jetzt ab (bei ausgewähltem **Alle Feeds** jeden Feed). |
| **Feed deaktivieren** / **Feed aktivieren** | Stoppt oder setzt die Prüfung des Feeds fort, ohne ihn zu entfernen. Nur im Kontextmenü. |

Klicken Sie mit der rechten Maustaste auf einen Feed in der Feed-Liste des RSS-Downloaders oder in der Kategorienliste des Hauptfensters, um dieselben Einträge zu bekommen.

Feeds werden alle 15 Minuten geprüft. Feed-Downloads und die `.torrent`-Dateien, auf die sie zeigen, gehen durch den Proxy, der in den Einstellungen gesetzt ist, falls Sie einen gesetzt haben. Die Links in den Einträgen eines Feeds dürfen nur auf öffentliche Adressen führen, es sei denn, der Feed selbst stammt von einem Server auf diesem Computer oder in Ihrem lokalen Netzwerk.

## Einträge von Hand herunterladen

Die Kategorienliste des Hauptfensters hat einen Zweig **Feeds** mit jedem Feed darunter. Klicken Sie auf einen Feed (oder auf **Feeds** für alle), und die Torrent-Liste zeigt statt Ihrer Torrents die Einträge des Feeds, mit den Spalten **Name**, **Episode**, **Format**, **Datum**, **Feed** und **URL**.

- **Episode** zeigt, was Rookparcel aus dem Titel gelesen hat: `1x02`, einen Bereich wie `1x02-03`, eine absolute Nummer wie `12` (Anime) oder ein Ausstrahlungsdatum wie `2026-10-19`.
- **Format** zeigt die im Titel genannten Qualitäten, etwa `720p` oder `HDTV`.

Um einen Eintrag herunterzuladen, doppelklicken Sie darauf oder klicken Sie mit der rechten Maustaste darauf und wählen **Herunterladen**. Sie können vorher mehrere Einträge auswählen. Ein Eintrag, den Sie von Hand herunterladen, bekommt den Namen des Feeds als Label und kommt in den Verlauf, wie der Download eines Filters.

## Filter

Ein Filter lädt die Einträge herunter, deren Titel zu ihm passen, aus einem Feed oder aus allen. Öffnen Sie den Reiter **Filter** und klicken Sie auf **Hinzufügen**. Ein Filter namens „Neuer Filter“ erscheint in der Liste, angehakt. Füllen Sie dann seine Felder aus:

| Feld | Was es tut |
|---|---|
| **Name:** | Der Name des Filters in der Liste. Das Kontrollkästchen der Liste schaltet den Filter ein oder aus. |
| **Filter:** | Wozu der Titel passen muss. Siehe [Einen Filter schreiben](#einen-filter-schreiben). Leer passt auf nichts; `*` passt auf alles. |
| **Nicht:** | Titel, die hierzu passen, werden übersprungen, auch wenn **Filter** passt. Dieselbe Syntax. Leer überspringt nichts. |
| **Speichern in:** | Der Ordner für die Downloads. Leer wird der Standard-Download-Ordner verwendet. Die Schaltfläche **...** wählt einen Ordner. |
| **Feed:** | **Alle Feeds** oder ein Feed. |
| **Qualität:** | Klicken Sie auf die Schaltfläche und haken Sie die Qualitäten an, die Sie akzeptieren. **Alle** akzeptiert jeden Titel, auch Titel, die keine Qualität nennen. |
| **Episodennummer:** | Welche Episoden genommen werden, zum Beispiel `1x12-14`. Leer nimmt jede. Siehe [Episodennummern](#episodennummern). |
| **Mindestabstand:** | Nachdem der Filter etwas heruntergeladen hat, lädt er nichts mehr herunter, bis so viel Zeit vergangen ist. **0 (ignorieren)** heißt keine Wartezeit. Auswahl von 2 Stunden bis 1 Monat. |
| **Label:** | Das Label, das die Downloads bekommen. Leer wird der Name des Feeds verwendet. Zu Labels siehe [Torrents verwalten](Torrents-verwalten). |
| **Intelligenter Episodenfilter** | Lädt jede Episode nur einmal herunter, ganz gleich welche Qualität oder Release-Gruppe. Siehe unten. |
| **Download die höchste Priorität geben** | Stellt den Download an den Anfang der Warteschlange. |
| **Torrent angehalten hinzufügen** | Fügt den Download hinzu, ohne ihn zu starten. |

Unter den Feldern listet **Passende Einträge in den Feeds:** die Einträge der letzten Prüfung der Feeds auf, die der Filter nach Filter, Nicht, Feed, Qualität und Episodennummer trifft. Damit können Sie einen Filter testen, bevor Sie das Fenster schließen. (Mindestabstand und der intelligente Episodenfilter hängen davon ab, was zuvor heruntergeladen wurde, daher wendet die Liste sie nicht an.)

**Entfernen** löscht den ausgewählten Filter. **Nach oben** und **Nach unten** verschieben ihn in der Liste. Die Reihenfolge zählt: Ein Eintrag geht an den ersten Filter, der ihn nimmt, setzen Sie die spezielleren Filter also über die allgemeinen.

Filter laufen, wenn ein Feed geprüft wird, und wenn Sie den RSS-Downloader schließen. In jedem Feed werden die ältesten Einträge zuerst angesehen, sodass beim intelligenten Episodenfilter oder einem Mindestabstand das erste Release einer Episode gewinnt. Ein Eintrag wird aus demselben Feed nie zweimal heruntergeladen.

### Einen Filter schreiben

Die Felder **Filter** und **Nicht** verwenden einfache Platzhalter:

| Sie tippen | Bedeutung |
|---|---|
| `*` | Eine beliebige Zeichenfolge, auch keine. |
| `?` | Genau ein Zeichen. |
| `\|` | Trennt Alternativen: `show one\|show two` passt auf eine von beiden. |

- Groß- und Kleinschreibung spielt keine Rolle.
- Ein `.` oder `_` in einem Titel zählt als Leerzeichen, sodass `the show` auf `The.Show.S01E02.720p` passt.
- Eine Alternative **ohne** Platzhalter passt irgendwo im Titel: `show` passt auf `The.Show.S01E02`.
- Eine Alternative **mit** Platzhaltern muss auf den ganzen Titel passen: `show*` passt nicht auf `The.Show.S01E02`, aber `*show*` schon.

Beispiele:

| Filter | Nicht | Nimmt |
|---|---|---|
| `the show` | | jeden Titel, der „the show“ enthält |
| `*the show*1080p*` | | diese Serie nur in 1080p |
| `the show` | `*german*\|*sample*` | diese Serie, aber keine deutschen Releases und keine Samples |

### Episodennummern

Das Feld **Episodennummer** nimmt einen oder mehrere Bereiche, getrennt durch `,` oder `;`:

| Sie tippen | Nimmt |
|---|---|
| `1x12` oder `S01E12` | Genau diese Episode. |
| `1x12-14` oder `S01E12-S01E14` | Die Episoden 12 bis 14 der Staffel 1. |
| `S01E01-10` | Die Episoden 1 bis 10 der Staffel 1 (die Staffel kann nach dem Bindestrich weggelassen werden). |
| `S02` | Die ganze Staffel 2. |
| `S01E05-` | Staffel 1 Episode 5 und alles danach. |
| `12` | Episode 12 in jeder Staffel, für durchnummerierte Serien wie Anime. |
| `1-12`, `100-` | Die Episoden 1 bis 12; Episode 100 und später. |
| `2026.10.19` | Die Episode, die an diesem Datum ausgestrahlt wurde (für tägliche Sendungen). |

Rookparcel liest Episoden aus Titeln, die als `S01E02` (auch `S01E02E03`, `S01E02-E03`), `1x02`, ein Ausstrahlungsdatum (`2026.10.19`, `2026-10-19`) oder eine absolute Nummer (`Show - 12`, `Episode 12`) geschrieben sind. Eine Versionsnummer wie `Ubuntu - 22.04` oder ein Rückblick wie `Show - 12.5` wird nicht als Episode gelesen. Ein Staffelpaket wie `Show.S01.Complete` nennt keine Episode, daher nimmt ein Filter mit einer Episodennummer es nicht. Ein Feld, das Rookparcel nicht lesen kann, passt auf nichts.

### Qualitäten

Die Liste **Qualität**: HDTV, TVRip, DVDRip, SVCD, DSRip, DVBRip, PDTV, HR.HDTV, HR.PDTV, DVDR, DVDScr, 720p, 1080i, 1080p, WebRip, SatRip, WEB-DL, BluRay, 480p, 2160p. Ein Titel passt, wenn er eine der Qualitäten nennt, die Sie angehakt haben.

### Intelligenter Episodenfilter

Ist **Intelligenter Episodenfilter** angehakt, merkt sich der Filter die Episoden, die er heruntergeladen hat, und überspringt spätere Releases derselben Episode, ganz gleich welche Qualität oder Release-Gruppe. Das ist nützlich, wenn ein Feed dieselbe Episode mehrfach enthält.

- Ein Titel ohne Episode darin wird immer durchgelassen.
- Ein Release, das als REPACK, PROPER, RERIP oder REAL einer bereits heruntergeladenen Episode gekennzeichnet ist, wird einmal durchgelassen, da es das frühere Release korrigiert.

Das **Intelligenten Episodenfilter verwenden** des Feed-Abonnements funktioniert genauso für einen Feed, der alles herunterladen soll.

## Verlauf

Der Reiter **Verlauf** listet auf, was der RSS-Downloader hinzugefügt hat, die neuesten zuerst, mit **Name**, **Datum** und **Feed**. Er behält die letzten 1.000 Einträge. **Verlauf leeren** leert die Liste; dadurch werden diese Einträge nicht erneut heruntergeladen.

Der Reiter „Protokoll“ des Hauptfensters zeigt außerdem eine Zeile für jeden Download („RSS:“ mit dem Namen des Feeds und dem Titel) und für jeden Feed-Fehler. Ein Download, der nicht abgerufen werden konnte, wird bei der nächsten Prüfung seines Feeds erneut versucht.

## GTK

Das GTK-Fenster hat denselben RSS-Downloader, geöffnet über **Optionen > RSS-Downloader...**, die Schaltfläche **RSS-Downloader** der Symbolleiste oder das Kontextmenü von **Feeds** in der Seitenleiste. Die Felder, Schaltflächen und Reiter sind dieselben wie oben. Der Zweig **Feeds** der Seitenleiste zeigt die Einträge eines Feeds in der Liste, und **Herunterladen** im Kontextmenü eines Eintrags (oder ein Doppelklick) lädt ihn herunter. Das Fenster „RSS-Feed hinzufügen“ bietet ebenfalls eine Feed-Adresse aus der Zwischenablage an.

## macOS

Die macOS-App hat denselben RSS-Downloader, geöffnet über **Optionen > RSS-Downloader...**, die Schaltfläche **RSS-Downloader** der Symbolleiste oder das Kontextmenü von **Feeds** in der Seitenleiste. Die Felder, Schaltflächen und Reiter sind dieselben wie oben. Die Einträge eines Feeds erscheinen in der Liste, wenn Sie den Feed in der Seitenleiste auswählen; doppelklicken Sie auf einen Eintrag, drücken Sie <kbd>Return</kbd> oder wählen Sie **Herunterladen** in seinem Kontextmenü, um ihn herunterzuladen.

## Web-UI

Auch die Web-UI hat den RSS-Downloader. Öffnen Sie ihn mit der Schaltfläche **RSS** der Symbolleiste (auf einem Smartphone im Menü **Mehr** der Symbolleiste, **RSS-Downloader...**), oder klicken Sie in der Kategorienliste unter **Feeds** auf einen Feed, was ihn auf den Einträgen dieses Feeds öffnet.

Der RSS-Downloader der Seite hat drei Reiter statt zwei:

- **Einträge**: die Einträge des ausgewählten Feeds (oder aller Feeds), mit **Herunterladen** für die ausgewählten.
- **Filter** und **Verlauf**: wie oben.

Unterschiede:

- **Speichern in** ist ein Ordner auf dem Server, nicht auf dem Computer mit dem Browser. Die Schaltfläche **...** fragt nach einem Pfad; leer bedeutet den Standard-Download-Ordner des Servers.
- Welche Web-UI Sie verwenden, entscheidet, wessen Feeds Sie bearbeiten:
  - **`rookparcel-nox`** hat seine eigenen Feeds und Filter (in `rss.dat` neben `webui.json`) und führt sie auf dem Server aus, ohne dass ein Fenster geöffnet sein muss. Er führt den RSS-Downloader nur aus, solange seine Web-UI an ist. Siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).
  - **Die Web-UI, die ein Windows-Fenster bereitstellt,** bearbeitet die eigenen Feeds und Filter dieses Fensters: Es gibt einen RSS-Downloader, der an beiden Orten angezeigt wird. Siehe [Web-UI](Weboberfläche).
- Die Web-UI beantwortet auch den RSS-Teil der Web-API von qBittorrent, sodass eine qBittorrent-Fern-App Feeds auflisten und Regeln setzen kann. Ihre Regeln verwenden die oben beschriebene Platzhaltersyntax; die Option für reguläre Ausdrücke wird ignoriert, und Feed-Ordner werden nicht unterstützt.

## Fernfenster

Ein Fenster, das mit einem anderen Rookparcel verbunden ist (Datei > Mit Fernsitzung verbinden, siehe [Fernsteuerung](Fernsteuerung)), hat seine eigenen Feeds und Filter, die auf diesem Computer gespeichert sind. Sie werden von diesem Computer aus geprüft, und was sie herunterladen, wird dem fernen Server hinzugefügt.

## Wo Feeds gespeichert sind

Die Feeds, Filter und der Verlauf liegen in `rss.dat` in Ihrem Einstellungsordner, neben `rookparcel.ini` (bei einem Fernfenster dort in seinem eigenen Ordner `remote\<host>_<port>`). Siehe [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt).

Für einmalige Suchen statt Feeds siehe [Torrents suchen](Torrents-suchen). Für Sonarr und Radarr, die ihre Feeds selbst prüfen, siehe [Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr).
