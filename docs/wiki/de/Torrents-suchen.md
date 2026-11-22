*Dies ist eine Übersetzung der englischen Seite [Searching for torrents](https://github.com/Balls0fSteel/Rookparcel/wiki/Searching-for-torrents). Maßgeblich ist die englische Seite.*

# Torrents suchen

Diese Seite erklärt, wie Sie neue Torrents direkt in Rookparcel finden: den Unterschied zwischen dem Filtern Ihrer eigenen Liste und dem Suchen, wie Sie Suchquellen hinzufügen (den Suchhelfer oder Ihr eigenes Prowlarr oder Jackett) und wie Sie hinzufügen, was eine Suche findet. Sie behandelt das Windows-Fenster, das GTK-Fenster (Linux, BSD) und die Web-UI.

Eine Schritt-für-Schritt-Anleitung für die Windows-Version mit Screenshots finden Sie unter [Torrents suchen unter Windows](Torrents-suchen-unter-Windows).

## So funktioniert die Suche

Rookparcel hat keine eigene Suchmaschine und durchsucht Websites nicht selbst. Es fragt eine *Suchquelle*: ein Programm, das Torrent-Seiten durchsuchen kann und in einem Standardformat namens Torznab antwortet. Zwei kostenlose Programme leisten das:

- **Jackett** und **Prowlarr**, die Sie vielleicht schon für Sonarr oder Radarr betreiben, auf diesem Computer, einem NAS oder einem Server.
- Der **Suchhelfer**, das ist Jackett, das Rookparcel für Sie herunterlädt und ausführt. Das ist der einfache Weg, wenn Sie keins von beiden haben.

Eine Suche geht gleichzeitig an alle Quellen, die Sie auswählen. Die Antworten werden zusammengeführt, Duplikate (derselbe Torrent aus zwei Quellen) werden nur einmal angezeigt, und die Liste ist nach Seeds sortiert.

*Das Suchfeld und „Quellen verwalten“ sind im Release 1.1.40 enthalten. Der Suchhelfer ist neu in 1.1.59.*

## Ihre Liste filtern oder suchen

Das Suchfeld oben rechts im Hauptfenster hat zwei Aufgaben:

| Modus | Was die Eingabetaste tut |
|---|---|
| Filter (die Voreinstellung) | Die Eingabe grenzt Ihre eigene Torrent-Liste auf die Namen ein, die den Text enthalten. Online wird nichts gesucht. |
| Eine Quelle oder **Alle Quellen** | Die Eingabe filtert die Liste nicht. Die Eingabetaste schickt den Text an die gewählte Quelle (oder jede Quelle) und öffnet das Fenster **Torrents suchen** mit den Ergebnissen. |

<kbd>Ctrl</kbd>+<kbd>F</kbd> setzt in beiden Oberflächen den Cursor in das Feld. Der graue Hinweis im leeren Feld sagt, in welchem Modus es ist.

### Windows

Der kleine Pfeil neben dem Feld öffnet ein Menü (<kbd>Alt</kbd>+<kbd>Down</kbd> im Feld öffnet es ebenfalls):

- **Torrents filtern**: der Listenfilter. Der Hinweis lautet „Torrent-Namen durchsuchen“.
- Jede eingeschaltete Suchquelle, mit ihrem Namen. Der Hinweis lautet „In *Name der Quelle* suchen“.
- **Alle Quellen**. Der Hinweis lautet „In allen Quellen suchen“.
- **Quellen verwalten...**, das das Fenster „Suchquellen“ öffnet.

Die Wahl bleibt zwischen Starts erhalten. Wird die gewählte Quelle entfernt oder ausgeschaltet, springt das Feld zurück auf **Torrents filtern**.

### GTK

Die Menüschaltfläche neben dem Feld hat dieselben Einträge, nur dass der Filtereintrag **Torrent-Namen durchsuchen** heißt. Ist eine Quelle gewählt, wechselt der Hinweis zu „Mit Prowlarr oder Jackett suchen“.

### macOS

Das Menü des Suchfelds hat dieselben Einträge wie das von GTK: **Torrent-Namen durchsuchen**, jede eingeschaltete Quelle, **Alle Quellen** und **Quellen verwalten...**.

### Web-UI

Das Suchfeld der Web-UI filtert nur die Liste. Das Suchen hat eine eigene Schaltfläche, **Suchen**, daneben (auf einem Smartphone im Menü **Mehr** der Symbolleiste, als **Torrents suchen**). „Suchen“ öffnet das Fenster „Torrents suchen“ mit dem Text des Filterfelds bereits darin.

## Quellen verwalten

Quellen werden im Fenster **Suchquellen** verwaltet:

- Windows: der Pfeil neben dem Suchfeld, dann **Quellen verwalten...**.
- GTK: die Menüschaltfläche neben dem Suchfeld, dann **Quellen verwalten...**, oder die Schaltfläche **Quellen verwalten...** im Fenster „Torrents suchen“.
- Web-UI: **Suchen**, dann **Quellen verwalten...** im Fenster „Torrents suchen“.

![Das Fenster „Suchquellen“](images/search-sources.png)

Das Fenster listet Ihre Quellen mit je einem Kontrollkästchen auf. Eine angehakte Quelle wird durchsucht, wenn Sie **Alle Quellen** durchsuchen; eine nicht angehakte bleibt erhalten, wird aber ausgelassen. Die Schaltflächen:

| Schaltfläche | Was sie tut |
|---|---|
| **Hinzufügen...** | Fügt eine eigene Quelle hinzu (siehe unten). |
| **Bearbeiten...** | Ändert Name, URL, API-Schlüssel oder Kontrollkästchen der ausgewählten Quelle. |
| **Entfernen** | Entfernt die ausgewählte Quelle, nach Rückfrage. |
| **Testen** | Fragt die Quelle nach ihrer Kategorienliste. „OK“ und eine Anzahl Kategorien heißt, dass URL und API-Schlüssel funktionieren; andernfalls zeigt die Zeile unter der Liste den Fehler. |

In den Fenstern für Windows und GTK sind hier auch die Schaltflächen des Suchhelfers; siehe [Der Suchhelfer](#der-suchhelfer).

Solange Sie keine Quellen haben, zeigt eine Suche „Noch keine Suchquellen: Fügen Sie Ihr Prowlarr oder Jackett unter „Quellen verwalten“ hinzu.“

## Ihr eigenes Prowlarr oder Jackett hinzufügen

Wenn Sie Prowlarr oder Jackett bereits betreiben, fügen Sie es als Quelle hinzu:

1. Öffnen Sie **Suchquellen** und klicken Sie auf **Hinzufügen...**. Das Fenster **Suchquelle hinzufügen** öffnet sich.
2. **Name:** beliebig. Bleibt es leer, wird die Quelle mit ihrem Hostnamen angezeigt.
3. **Torznab-URL:** die Adresse des Torznab-Feeds der Quelle:
   - Jackett: `http://host:9117/api/v2.0/indexers/all/results/torznab/`. Der Teil `all` durchsucht jeden Indexer, den Sie in Jackett eingerichtet haben, eine Quelle genügt also.
   - Prowlarr: `http://host:9696/1/api`, wobei `1` die Nummer des Indexers ist. Prowlarr hat keinen Feed „alle Indexer“, fügen Sie also eine Quelle pro Indexer hinzu.

   Ersetzen Sie `host` durch `localhost`, wenn es auf demselben Computer läuft, oder durch die Adresse des Computers in Ihrem Netzwerk.
4. **API-Schlüssel:** der Schlüssel von der Seite von Jackett oder aus **Settings > General** von Prowlarr.
5. Lassen Sie **Diese Quelle durchsuchen** angehakt und klicken Sie auf **OK**.
6. Wählen Sie die neue Quelle aus und klicken Sie auf **Testen**. Es sollte „OK“ und eine Anzahl Kategorien melden.

Die URL muss mit `http://` oder `https://` beginnen.

**Der API-Schlüssel** wird verschlüsselt gespeichert (unter Windows) und nie wieder angezeigt. Wenn Sie eine Quelle bearbeiten, beginnt das Schlüsselfeld leer mit dem grauen Hinweis „Unverändert“; lassen Sie es leer, um den gespeicherten Schlüssel zu behalten, oder tippen Sie einen neuen ein. Der Schlüssel wird aus jeder Adresse entfernt, die in einem Fehler oder einer Statuszeile angezeigt wird.

**Proxy.** Eine Quelle auf diesem Computer oder in Ihrem lokalen Netzwerk wird direkt erreicht: eine IP-Adresse oder `localhost`, die nicht öffentlich ist, oder ein Hostname, dessen Adressen alle es nicht sind. Eine Quelle an einer öffentlichen Adresse geht durch den Proxy, der in den Einstellungen gesetzt ist, falls Sie einen gesetzt haben. Löst der Proxy Hostnamen selbst auf (ein SOCKS-Proxy, der so eingestellt ist, dass er Hostnamen sendet, oder ein HTTP-Proxy für eine einfache `http://`-Adresse), schlägt Rookparcel einen Namen nicht zuerst auf diesem Computer nach, sodass auch eine Quelle, die mit einem anderen Hostnamen als `localhost` angegeben ist, durch den Proxy geht; geben Sie die IP-Adresse der Quelle an, damit sie direkt bleibt. Siehe [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall).

### In der Web-UI

Die Fenster „Hinzufügen“ und „Bearbeiten“ der Web-UI haben ein Feld mehr, **Aktuelles Passwort der Web-UI:**. Zum Hinzufügen einer Quelle oder zum Ändern der URL oder des API-Schlüssels einer Quelle brauchen Sie das aktuelle Passwort der Web-UI. Das Umbenennen einer Quelle, das An- oder Ausschalten, das Entfernen, das Testen und das Suchen brauchen es nicht. Ein falsches Passwort meldet Sie ab, wie an anderer Stelle in der Web-UI.

Die Web-UI zeigt nie einen gespeicherten API-Schlüssel und schickt nie die Ergebnislinks (die den Schlüssel des Indexers tragen) an den Browser.

## Der Suchhelfer

Der Suchhelfer ist [Jackett](https://github.com/Jackett/Jackett), ein eigenständiges kostenloses Programm seiner eigenen Autoren unter der GPL-2.0-Lizenz. Rookparcel lädt das eigene Release von Jackett von GitHub herunter, wenn Sie auf die Schaltfläche klicken, prüft es anhand der Prüfsumme, die GitHub veröffentlicht, bewahrt es in einem Ordner `search-helper` in Ihrem Profil auf und führt es nur aus, während Sie suchen. Vor Ihrem Klick wird nichts heruntergeladen oder ausgeführt.

*Neu in 1.1.59.*

### Wo er angeboten wird

| Wo | Suchhelfer |
|---|---|
| Windows-Fenster (x64 und ARM64) | Ja |
| GTK-Fenster unter Linux (x86-64 und ARM64) | Ja |
| macOS-App (Intel und Apple Silicon) | Ja |
| Flatpak | Nein |
| FreeBSD | Nein (Jackett hat dafür keinen Build) |
| Von einem Paketmanager erstellte Builds ohne die Update-Prüfung | Nein |
| Web-UI und `rookparcel-nox` | Ja, wenn der Server Windows, Linux oder macOS ist (nicht FreeBSD, kein Flatpak). *Neu in 1.1.67.* |

Wo er nicht angeboten wird, hat das Fenster „Suchquellen“ keine Helfer-Schaltflächen, und **Hinzufügen...** für Ihr eigenes Prowlarr oder Jackett funktioniert weiterhin.

### Ihn holen

1. Öffnen Sie **Suchquellen** und klicken Sie auf **Suchhelfer holen (Jackett)...**.
2. Rookparcel sucht das neueste Jackett-Release und fragt vor dem Herunterladen nach. Die Frage nennt die Version, die Größe (etwa 50 MB) und den Ordner, in den er kommt. Klicken Sie auf **Ja**.
3. Ein Fortschrittsfenster zeigt den Download. Rookparcel prüft dann die Datei, entpackt sie und startet sie einmal, um die Liste ihrer Seiten zu lesen.
4. Das Fenster **Seiten des Suchhelfers** öffnet sich. Nichts ist für Sie angehakt. Haken Sie die öffentlichen Seiten an, die Sie durchsuchen möchten, und klicken Sie auf **OK**. Seiten mit der Angabe „Benötigt FlareSolverr, das der Helfer nicht enthält“ sind ausgegraut und lassen sich nicht anhaken.
5. Zurück in „Suchquellen“ ist der Helfer als **Suchhelfer (alle)** aufgeführt. Die Zeile unten sagt, wie viele Seiten angehakt sind.

Wählen Sie dann **Suchhelfer (alle)** (oder **Alle Quellen**) im Menü des Suchfelds und suchen Sie.

Zwei oder drei Seiten genügen für den Anfang. Jede angehakte Seite wird bei jeder Suche gefragt, mehr Seiten machen Suchen also langsamer und bringen mehr Duplikate.

Ist der Helfer installiert, wird die Schaltfläche **Holen** durch vier andere ersetzt:

| Schaltfläche | Was sie tut |
|---|---|
| **Seiten auswählen...** | Öffnet das Fenster „Seiten des Suchhelfers“ erneut, um Seiten an- oder abzuhaken. |
| **Seite des Helfers öffnen** | Öffnet die eigene Seite von Jackett in Ihrem Browser und kopiert deren Passwort in die Zwischenablage (die Seite fragt danach). Verwenden Sie sie, um private Seiten mit Ihrem Login hinzuzufügen; sie werden dann zusammen mit den angehakten durchsucht. |
| **Suchhelfer aktualisieren** | Sucht ein neueres Jackett und fragt vor dem Herunterladen nach. Ihre Seiten und Einstellungen bleiben. Nichts aktualisiert sich von selbst. |
| **Suchhelfer entfernen** | Fragt nach, stoppt dann den Helfer, löscht seinen Ordner und nimmt seine Quelle aus der Liste. |

### Wie er läuft

- Er startet bei Ihrer ersten Suche in seiner Quelle und stoppt sich zehn Minuten nach der letzten selbst. Er stoppt auch, wenn Rookparcel geschlossen wird. Er startet nie von selbst oder beim Hochfahren.
- Er lauscht nur auf diesem Computer (`127.0.0.1`), nie in Ihrem Netzwerk.
- Er braucht keine Administratorrechte.
- Wenn Sie in den Einstellungen einen Proxy setzen, verwendet der Helfer ihn für die Seiten. „Nur dieses Netzwerk verwenden“ bindet den Helfer nicht, weil er ein eigenes Programm ist: Seine Anfragen nehmen den normalen Weg des Computers.

### In der Web-UI und `rookparcel-nox`

**Quellen verwalten** in der Web-UI hat dieselben Schaltflächen und dieselbe Seitenliste. Der Helfer wird auf dem Server heruntergeladen und läuft dort, nicht auf dem Computer mit dem Browser, und mit `rookparcel-nox --no-state` wird er nicht angeboten. Jeder Schritt, der Zeit braucht, zeigt seinen Fortschritt, und der Download hat ein **Abbrechen**. Zum Herunterladen brauchen Sie das aktuelle Passwort der Web-UI, wie beim Hinzufügen einer Quelle. Die eigene Seite des Helfers lauscht auf dem `127.0.0.1` des Servers, daher wird **Seite des Helfers öffnen** nur einem Browser auf dem Computer gezeigt, auf dem Rookparcel läuft; anderswo sagt das Fenster das. Ein Fenster, das mit **Datei > Mit Fernsitzung verbinden...** verbunden ist, verwendet seinen eigenen Helfer auf diesem Computer.

### Wo er liegt

Der Ordner `search-helper` im Ordner Ihres Profils:

| System | Ordner |
|---|---|
| Windows | `%APPDATA%\Rookparcel\search-helper`; im portablen Modus der Ordner mit `rookparcel.exe`; mit einem Profil der Ordner dieses Profils |
| Linux (GTK) | `~/.local/share/rookparcel/gtk/search-helper` (mit einem Profil unter `profiles/<name>/`) |
| macOS | `~/Library/Application Support/Rookparcel/mac/search-helper` (mit einem Profil unter `profiles/<name>/`) |
| `rookparcel-nox` | `search-helper` in seinem Zustandsordner: `%APPDATA%\Rookparcel\nox\search-helper`, `~/.local/share/rookparcel/nox/search-helper` oder `/var/lib/rookparcel-nox/search-helper` beim mitgelieferten Systemdienst |

Jedes Profil hat seinen eigenen Helfer. Siehe [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt) und [Profile](Profile).

## Das Fenster „Torrents suchen“

Das Fenster öffnet sich, wenn Sie bei gewählter Quelle im Suchfeld die Eingabetaste drücken (Windows, GTK) oder auf **Suchen** klicken (Web-UI). Es hat:

- **Die Suchanfrage** und **Suchen**, um aus dem Fenster erneut zu suchen.
- **Eine Kategorie**: **Alle Kategorien**, **Filme**, **Audio**, **PC**, **TV**, **Bücher** oder **Sonstiges**.
- **Eine Quelle**: **Alle Quellen** oder eine einzelne Quelle.
- **Stoppen**, um die Quellen zu stoppen, die noch suchen. Die bisherigen Ergebnisse bleiben.
- **Eine Statuszeile pro Quelle**: „Suche läuft...“, die Anzahl der Ergebnisse oder der Fehler. Dass eine Quelle fehlschlägt, hält die anderen nicht auf.
- **Die Ergebnisse**, mit den Spalten **Name**, **Größe**, **Seeds**, **Peers**, **Kategorie**, **Alter** und **Quelle**. Klicken Sie auf eine Spaltenüberschrift, um zu sortieren; die Liste beginnt nach Seeds sortiert. Finden zwei Quellen denselben Torrent, wird er einmal aufgeführt, und **Quelle** nennt beide.
- Die Anzahl der Ergebnisse, **Hinzufügen** und **Schließen**.

Auf einem Smartphone zeigen die Ergebnisse der Web-UI nur Name, Größe und Seeds.

Das Schließen des Fensters beendet die Suche (Windows: <kbd>Esc</kbd> schließt es ebenfalls).

## Ein Ergebnis hinzufügen

Wählen Sie ein Ergebnis aus und klicken Sie auf **Hinzufügen**, doppelklicken Sie darauf oder drücken Sie <kbd>Enter</kbd> in der Liste.

Rookparcel holt das Ergebnis von seiner Quelle (eine `.torrent`-Datei oder einen Magnet-Link) und fügt es so hinzu, wie eine geöffnete `.torrent`-Datei oder ein angeklickter Magnet-Link hinzugefügt wird. Ist **Ein Fenster mit den Dateien im Torrent anzeigen** unter **Optionen > Einstellungen > Oberfläche** eingeschaltet, öffnet sich zuerst das Fenster „Torrent hinzufügen“. Siehe [Torrents hinzufügen](Torrents-hinzufügen).

Die Zeile unten sagt „Hinzugefügt:“ und den Namen des Torrents. Das GTK-Fenster und die Web-UI weisen ein Ergebnis zurück, das schon in Ihrer Torrent-Liste ist.

In der Web-UI wird der Torrent auf dem Server hinzugefügt.

## Aus einem Fernfenster oder der Web-UI suchen

- **Ein Fernfenster** (Datei > Mit Fernsitzung verbinden, siehe [Fernsteuerung](Fernsteuerung)) sucht von diesem Computer aus, mit seiner eigenen Quellenliste, und fügt, was Sie auswählen, dem fernen Server hinzu.
- **Die Web-UI von `rookparcel-nox`** hat ihre eigenen Quellen auf dem Server. Suchen und Downloads laufen auf dem Server; jede geöffnete Seite sieht dieselbe Suche.
- **Die Web-UI, die ein Windows-Fenster bereitstellt,** verwendet die Quellen dieses Fensters: Eine im Fenster hinzugefügte Quelle erscheint auf der Seite und umgekehrt. Siehe [Web-UI](Weboberfläche).

Die Web-UI beantwortet auch den Suchteil der Web-API von qBittorrent, sodass auch der Such-Reiter einer qBittorrent-Fern-App Ihre Quellen durchsucht.

## Wenn etwas schiefgeht

| Was Sie sehen | Was es bedeutet |
|---|---|
| Das Ergebnisfenster öffnet sich nie | Das Feld ist im Filtermodus. Wählen Sie zuerst in seinem Menü eine Quelle oder **Alle Quellen**. |
| „Noch keine Suchquellen: ...“ | Fügen Sie unter „Quellen verwalten“ eine Quelle hinzu oder holen Sie den Suchhelfer. |
| Die Zeile einer Quelle zeigt einen Fehler | Diese Quelle ist fehlgeschlagen; die anderen antworten weiter. Klicken Sie in „Suchquellen“ auf **Testen**, um URL und API-Schlüssel zu prüfen. |
| Der Test zeigt `HTTP 401` oder einen Fehler zum API-Schlüssel | Der API-Schlüssel ist falsch. Bearbeiten Sie die Quelle und tippen Sie den Schlüssel erneut ein. |
| Der Suchhelfer findet nichts | Es sind keine Seiten angehakt, oder die angehakten Seiten sind nicht erreichbar oder haben sich geändert. **Seiten auswählen...** zeigt, was angehakt ist; probieren Sie eine andere Seite. |
| „Der Suchhelfer konnte nicht geladen werden: ...“ | GitHub war nicht erreichbar, oder der Download wurde unterbrochen. Versuchen Sie es erneut. |
| „Der Suchhelfer wurde nicht gestartet: ...“ | Das Programm im Ordner `search-helper` ließ sich nicht ausführen: ein beschädigter Download oder ein Virenscanner, der Dateien entfernt hat. Entfernen Sie ihn und holen Sie ihn erneut. |

Weitere Hilfe: [Fehlerbehebung und FAQ](Fehlerbehebung-und-FAQ). Für automatische Downloads neuer Episoden aus Feeds siehe [Downloads per RSS](Downloads-per-RSS). Um Rookparcel als Download-Client von Prowlarr zu verwenden, siehe [Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr).
