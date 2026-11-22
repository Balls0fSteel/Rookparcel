*Dies ist eine Übersetzung der englischen Seite [Creating torrents](https://github.com/Balls0fSteel/Rookparcel/wiki/Creating-torrents). Maßgeblich ist die englische Seite.*

# Torrents erstellen

Diese Seite zeigt, wie Sie mit dem Fenster „Neuen Torrent erstellen“ aus einer Datei oder einem Ordner auf Ihrem Computer eine neue `.torrent`-Datei erstellen, was jede Option bedeutet und wie Sie sofort mit dem Seeden beginnen.

![Das Fenster „Neuen Torrent erstellen“](images/create-torrent.png)

## Einen Torrent in fünf Schritten erstellen

1. Wählen Sie **Datei > Neuen Torrent erstellen...** (<kbd>Ctrl</kbd>+<kbd>N</kbd>) oder die Schaltfläche **Neuen Torrent erstellen** in der Symbolleiste.
2. Klicken Sie unter **Quelle auswählen** auf **Datei hinzufügen** für eine einzelne Datei oder auf **Ordner hinzufügen** für einen Ordner. Sie können den Pfad auch eintippen oder einfügen.
3. Fügen Sie unter **Torrent-Eigenschaften** die gewünschten Tracker hinzu, einen pro Zeile. Ohne Tracker können Peers einander nur über das DHT und den Peer-Austausch finden.
4. Lassen Sie **Seeding starten** angehakt, um ihn sofort zu teilen.
5. Klicken Sie auf **Erstellen und speichern unter...**, wählen Sie, wo die `.torrent`-Datei gespeichert werden soll, und warten Sie auf den Fortschrittsbalken.

Der vorgeschlagene Dateiname ist der Name der Quelle plus `.torrent`. Ist das Hashing fertig, schließt sich das Fenster. Geben Sie die `.torrent`-Datei an die Leute weiter, die ihn herunterladen sollen.

## Die Optionen

### Quelle auswählen und Dateien überspringen

**Quelle auswählen** ist die Datei oder der Ordner, aus der bzw. dem der Torrent erstellt wird. Der Name eines Ordners wird zum Namen des Torrents.

**Dateien überspringen** lässt Dateien und Ordner nach Namen aus dem Torrent heraus. Schreiben Sie Platzhaltermuster, `*` für eine beliebige Zeichenfolge und `?` für ein Zeichen, getrennt durch `|` oder `;`. Die Groß- und Kleinschreibung spielt keine Rolle, und ein Muster muss auf den ganzen Namen einer Datei oder eines Ordners passen, nicht auf ihren Pfad. Zum Beispiel überspringt `*.nfo|Thumbs.db|*.tmp` NFO-Dateien, Windows-Miniaturansichts-Caches und temporäre Dateien.

### Torrent-Eigenschaften

| Option | Bedeutung |
|---|---|
| **Tracker:** | eine Tracker-Adresse pro Zeile. Eine Leerzeile beginnt eine neue Stufe: Spätere Stufen werden verwendet, wenn die Tracker davor versagen. Wiederholte Adressen werden verworfen. |
| **Webseeds:** | Adressen von Webservern, die dieselben Dateien bereithalten, eine pro Zeile. Peers können von ihnen wie von einem Seed herunterladen. |
| **Kommentar:** | ein freier Text, der im Torrent gespeichert wird; andere Programme zeigen ihn an, zum Beispiel im Fenster „Torrent hinzufügen“. |
| **Teilgröße:** | die Größe jedes Teils, von 16kB bis 16MB. **(automatisch)** wählt eine aus der Gesamtgröße und ist für die meisten Torrents die richtige Wahl. |
| **Version:** | die BitTorrent-Version, siehe unten. |

### Version: v1, v2 oder Hybrid

| Auswahl | Was Sie bekommen |
|---|---|
| **Hybrid (v1 + v2)** | die Voreinstellung. Lässt sich in älteren Programmen laden, die nur v1 kennen, und in neueren, die v2 verstehen. |
| **v1** | das klassische Format, wie µTorrent 2.2.1 es erstellte. Jedes BitTorrent-Programm liest es. |
| **nur v2** | nur das neuere Format. Programme, die nur v1 verstehen, können es nicht laden. |

Im Zweifel behalten Sie **Hybrid (v1 + v2)**.

### Sonstiges

| Option | Bedeutung |
|---|---|
| **Seeding starten** | fügt den neuen Torrent Ihrer Liste hinzu, sobald er erstellt ist, mit dem Ordner, der die Quelle enthält, als Download-Ordner. Rookparcel prüft die Dateien und seedet dann. Die Tracker aus **Tracker für neue Downloads** in den Einstellungen werden nicht hinzugefügt; er behält die, die Sie eingegeben haben. |
| **Privater Torrent** | kennzeichnet den Torrent als privat: Peers werden nur über seine Tracker gefunden, nie über das DHT, den Peer-Austausch oder die Lokale Peer-Erkennung. Private Tracker verlangen das. |
| **Dateireihenfolge beibehalten** | behält die Dateien eines Ordners in der Reihenfolge, in der Windows sie auflistet, statt sie nach Namen zu sortieren. Nur für **v1** verfügbar: v2- und Hybrid-Torrents sortieren ihre Dateien immer. |

## Fortschritt und Abbrechen

Während Rookparcel die Dateien liest und hasht, bewegen sich Fortschrittsbalken und Prozentanzeige, und die übrigen Bedienelemente sind ausgegraut. Die Schaltfläche **Schließen** wird zu **Abbrechen**: Klicken Sie darauf, drücken Sie <kbd>Esc</kbd> oder schließen Sie das Fenster, um den Vorgang zu beenden. Beim Abbrechen wird nichts gespeichert.

Geht etwas schief, sagt eine Meldung, was. Häufige:

| Meldung | Bedeutung |
|---|---|
| „Bitte wählen Sie eine Datei oder einen Ordner, aus dem der Torrent erstellt wird.“ | **Quelle auswählen** ist leer. |
| „...: keine Dateien für den Torrent (leer oder alle übersprungen).“ | der Ordner ist leer, oder **Dateien überspringen** hat nichts übrig gelassen. |

## Was Rookparcel sich merkt

Beim nächsten Öffnen des Fensters stehen dort die Tracker, die Teilgröße, die Version sowie **Seeding starten**, **Privater Torrent** und **Dateireihenfolge beibehalten**, die Sie zuletzt verwendet haben. Quelle, Überspringmuster, Webseeds und Kommentar beginnen leer.

## Einen früher erstellten Torrent seeden

Haben Sie einen Torrent ohne **Seeding starten** erstellt, fügen Sie die `.torrent`-Datei wie jede andere hinzu ([Torrents hinzufügen](Torrents-hinzufügen)) und setzen **Speichern in** auf den Ordner, der die Quelldatei oder den Quellordner enthält. Rookparcel prüft die Dateien und seedet sie.

Um einem ganz neuen Torrent bei der Verbreitung zu helfen, können Sie für ihn **Initial-Seeding** einschalten; siehe [Torrents verwalten](Torrents-verwalten#initial-seeding).

## In der GTK-Version (Linux, BSD)

Dasselbe Fenster mit denselben Optionen, unter **Datei > Neuen Torrent erstellen...** (<kbd>Ctrl</kbd>+<kbd>N</kbd>).

## In der macOS-App

Dasselbe Fenster mit denselben Optionen, unter **Datei > Neuen Torrent erstellen...** (<kbd>⌘</kbd>+<kbd>N</kbd>).

## In der Web-UI und rookparcel-nox

Die Web-UI kann keine Torrents erstellen: Die Dateien müssten auf dem Server liegen. Auf dem Server selbst erstellt `rookparcel-nox create` einen Torrent von der Befehlszeile aus:

```
rookparcel-nox create "D:\Share\My folder" -o my-folder.torrent -t udp://tracker.example.org:6969/announce
```

`--v1`, `--v2` und `--hybrid` wählen die Version (standardmäßig Hybrid), `--private` kennzeichnet ihn als privat, `--piece-size` setzt die Teilgröße in KiB, `-w` fügt einen Webseed hinzu, und `--magnet` gibt zusätzlich den Magnet-Link aus. `rookparcel-nox --help` listet alle Optionen auf. Siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

In einem Fenster, das mit **Datei > Mit Fernsitzung verbinden...** verbunden ist, ist **Seeding starten** nicht angehakt und ausgegraut: Der Torrent wird aus Dateien auf diesem Computer erstellt, die der Server nicht hat. Siehe [Fernsteuerung](Fernsteuerung).

## Verwandte Seiten

- [Torrents hinzufügen](Torrents-hinzufügen)
- [Torrents verwalten](Torrents-verwalten)
- [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall)
