*Dies ist eine Übersetzung der englischen Seite [First start and the Setup Guide](First-start-and-the-Setup-Guide). Maßgeblich ist die englische Seite.*

# Erster Start und der Einrichtungsassistent

Diese Seite behandelt, was beim ersten Start von Rookparcel geschieht: der Einrichtungsassistent, ein kurzes Fenster mit vier Fragen, was jede Antwort einstellt und wie Sie den Assistenten später wieder öffnen.

*Neu in 1.1.62.* Bis 1.1.40 gibt es keinen Assistenten: Rookparcel öffnet sich mit seinen Standardeinstellungen, und alles unten Genannte stellen Sie stattdessen unter **Optionen > Einstellungen** ein.

## Wann sich der Assistent öffnet

Der Einrichtungsassistent öffnet sich von selbst beim ersten Start von Rookparcel: wenn in seiner Einstellungsdatei noch keine Einstellungen stehen. Das ist bei einer neuen Installation so, bei einem neuen portablen Ordner und bei jedem neuen [Profil](Profile). Er öffnet sich vor dem Hauptfenster, sodass nichts zu laden beginnt, bevor Sie geantwortet haben. Eine Einstellungsdatei, die vorhanden, aber nicht lesbar ist, gilt nicht als erster Start.

Für jemanden, der schon eine ältere Version benutzt hat, öffnet er sich nicht von selbst: Die Einstellungen sind ja da. Öffnen Sie ihn stattdessen über das Menü (unten).

![Die erste Seite des Einrichtungsassistenten](images/setup-guide.png)

## So geht es weiter

Jede Seite hat einen Titel, darüber „Schritt 1 von 4“, einen kurzen Text und ihre Einstellungen. Die Schaltflächen:

| Schaltfläche | Was sie tut |
|---|---|
| **< Zurück** | die Seite davor |
| **Weiter >** | die nächste Seite |
| **Fertig stellen** | speichert Ihre Antworten und schließt den Assistenten |
| **Überspringen** | schließt den Assistenten und lässt die Einstellungen, wie sie sind. <kbd>Esc</kbd> und das Schließen-Kästchen des Fensters tun dasselbe |

Nichts wird gespeichert, bevor Sie **Fertig stellen** wählen, und **Fertig stellen** ändert nur die Antworten, die Sie geändert haben. Zeigen Sie mit der Maus auf eine Einstellung, um zu sehen, was sie tut; die Hilfe erscheint auch unter den Einstellungen.

Nach **Fertig stellen** oder **Überspringen** öffnet sich der Assistent nicht mehr von selbst.

## Seite 1: Wohin Downloads gehen

Wählen Sie den Ordner, in dem neue Downloads gespeichert werden.

| Einstellung | Bedeutung |
|---|---|
| **Neue Downloads speichern in:** | der Ordner für neue Downloads. Standard ist Ihr Ordner „Downloads“; in einem portablen Ordner ist es der Ordner `Downloads` darin |
| **Abgeschlossene Downloads verschieben nach:** | wenn angehakt, werden fertige Downloads von selbst in den Ordner daneben verschoben, zum Beispiel von einer schnellen Festplatte auf eine große |

Unter **Optionen > Einstellungen > Ordner** gibt es weitere Ordnereinstellungen. Siehe [Datenträger und Speicher](Datenträger-und-Speicher).

## Seite 2: Ihr Router

Jede Verbindung zu einem Peer belegt einen Platz in der Verbindungstabelle Ihres Routers. Ältere Router und die einfachen Geräte der Internetanbieter haben kleine Tabellen; ist eine voll, trennt der Router Verbindungen, bremst das ganze Haus aus oder startet neu.

**Welchen Router haben Sie?**

- **Modern (etwa nach 2015 gekauft) oder nicht sicher:** die normalen Verbindungszahlen von Rookparcel.
- **Älter oder das einfache Gerät des Internetanbieters:** weniger Verbindungen: insgesamt 100, 40 pro Torrent und 8 neue pro Sekunde.

Die ältere Antwort ist ein Ausgangspunkt. Sie können diese Zahlen später unter **Optionen > Einstellungen > Bandbreite** und **Erweitert** ändern. Wählen Sie später „modern“, werden die normalen Zahlen wiederhergestellt.

Auf derselben Seite steht **Ping niedrig halten (Geschwindigkeiten automatisch begrenzen)**. Haken Sie es an, wenn Sie spielen oder telefonieren, während Rookparcel läuft: Es hält die Geschwindigkeiten niedrig, solange Ihre Leitung ausgelastet ist. Siehe [Niedrigen Ping halten](Niedrigen-Ping-halten). Sie steht auch unter **Optionen > Einstellungen > Gaming**.

## Seite 3: Ihre Festplatten

**Intelligente Festplatten-E/A verwenden** hält den Computer reaktionsfähig, während Rookparcel liest und schreibt:

- Prüfungen laufen mit der Hintergrund-Festplattenpriorität des Systems;
- jedes Laufwerk wird auf seinen Typ abgestimmt (SSD, Festplatte, Netzwerkfreigabe);
- Übertragungen werden langsamer, solange eine Festplatte langsam antwortet.

Die Abstimmung pro Laufwerk ist von Anfang an aktiv. Das Kästchen fügt schonende Prüfungen und das Abbremsen hinzu, solange eine Festplatte ausgelastet ist. Prüfungen dauern länger, solange etwas anderes die Festplatte benutzt. Unter **Optionen > Einstellungen > Gaming** stellen Sie jeden Teil einzeln ein. Siehe [Datenträger und Speicher](Datenträger-und-Speicher).

## Seite 4: Dieser Computer

Rookparcel kann starten, wenn Sie sich anmelden, und `.torrent`-Dateien und `magnet:`-Links aus Ihrem Browser und Dateimanager öffnen.

| Einstellung | Bedeutung |
|---|---|
| **Rookparcel beim Systemstart starten** | startet Rookparcel, wenn Sie sich anmelden |
| **Mit .torrent-Dateien verknüpfen** | ein Doppelklick auf eine `.torrent`-Datei öffnet sie in Rookparcel |
| **Mit magnet:-Links verknüpfen** | ein Klick auf einen `magnet:`-Link im Browser öffnet ihn in Rookparcel |

Unter **Optionen > Einstellungen > Allgemein** ändern Sie das später. Siehe [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart).

Diese Seite entfällt, wo sie nicht zutrifft:

- **Der Windows-Installer** hat schon nach den Verknüpfungen gefragt, deshalb zeigt eine installierte Kopie beim ersten Start drei Seiten. Die Seite ist da, wenn Sie den Assistenten über das Menü öffnen.
- **Ein Fenster, das mit einem entfernten `rookparcel-nox` verbunden ist,** zeigt drei Seiten, wenn Sie den Assistenten über das Menü öffnen.
- **Das Flatpak** zeigt nur das Kästchen für den Start: Der Desktop entscheidet, welches Programm `.torrent`-Dateien und `magnet:`-Links öffnet. Ebenso macOS, wo Rookparcel die Verknüpfungen nicht selbst setzen kann (ein benanntes Profil oder eine bloße Programmdatei außerhalb von `Rookparcel.app`).

## Den Assistenten erneut öffnen

Wählen Sie **Optionen > Einrichtungsassistent** oder drücken Sie <kbd>Ctrl</kbd>+<kbd>G</kbd>. Der Assistent zeigt Ihre aktuellen Einstellungen, und **Fertig stellen** übernimmt die Antworten, die Sie geändert haben, so wie **OK** in den Einstellungen.

## GTK und die Weboberfläche

- Die GTK-Version (Linux, die BSDs) hat denselben Assistenten mit denselben Seiten, beim ersten Start jedes Profils und unter **Optionen > Einrichtungsassistent** (<kbd>Ctrl</kbd>+<kbd>G</kbd>).
- Die macOS-App hat ihn auch, unter **Rookparcel > Einrichtungsassistent** (<kbd>⌘</kbd>+<kbd>G</kbd>).
- `rookparcel-nox` und die Weboberfläche haben keinen Assistenten. Richten Sie sie in den Einstellungen der Weboberfläche ein: siehe [Weboberfläche](Weboberfläche) und [rookparcel-nox betreiben](rookparcel-nox-betreiben).

## Nach dem Assistenten

Das Hauptfenster öffnet sich. Die nächsten Schritte:

- [Torrents hinzufügen](Torrents-hinzufügen)
- [Das Hauptfenster](Das-Hauptfenster)
- [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall), wenn Downloads langsam bleiben
- [Einstellungen im Überblick](Einstellungen-im-Überblick), für alle anderen Einstellungen
