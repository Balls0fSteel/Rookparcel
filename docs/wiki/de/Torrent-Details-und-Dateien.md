*Dies ist eine Übersetzung der englischen Seite [Torrent details and files](https://github.com/Balls0fSteel/Rookparcel/wiki/Torrent-details-and-files). Maßgeblich ist die englische Seite.*

# Torrent-Details und Dateien

Diese Seite erklärt die Detail-Reiter unter der Torrent-Liste (Allgemein, Tracker, Peers, Teile, Dateien, Geschwindigkeit und Protokoll), wie Sie auswählen, welche Dateien in welcher Reihenfolge heruntergeladen werden, und das Fenster „Torrent-Eigenschaften“ mit den Trackern, Begrenzungen und dem Seeding-Ziel eines Torrents.

Wählen Sie einen Torrent in der Liste aus, um die Reiter zu füllen. **Optionen > Details anzeigen** blendet sie ein oder aus; ziehen Sie den Rand über ihnen, um ihre Höhe zu ändern. Rookparcel merkt sich den geöffneten Reiter.

## Allgemein

![Der Reiter „Allgemein“](images/tab-general.png)

Die Übersicht über einen Torrent.

- **Heruntergeladen:** ein Balken mit einem Streifen für jedes Teil, das Sie haben, und dem erledigten Anteil in Prozent.
- **Verfügbarkeit:** ein Balken, der zeigt, welche Teile die verbundenen Peers haben und wie viele vollständige Kopien sie zusammen besitzen. Unter 1 hat keiner der Verbundenen jedes Teil.

**Übertragung**

| Feld | Bedeutung |
|---|---|
| **Verstrichene Zeit:** | wie lange der Torrent schon läuft |
| **Verbleibend:** | geschätzte Restzeit |
| **Verworfen:** | weggeworfene Daten: Teile, die ihre Prüfung nicht bestanden haben, und Duplikate |
| **Heruntergeladen:**, **Hochgeladen:** | Summen für diesen Torrent |
| **Seeds:**, **Peers:** | „3 von 120 verbunden“: verbunden, und die Gesamtzahl, die der Tracker meldet |
| **Download-Geschwindigkeit:**, **Upload-Geschwindigkeit:** | aktuelle Geschwindigkeiten |
| **Download-Limit:**, **Upload-Limit:** | die eigenen Begrenzungen des Torrents; ∞ für keine |
| **Verhältnis:** | hochgeladen geteilt durch heruntergeladen |
| **Status:** | derselbe Text wie in der Spalte „Status“ |

**Allgemein**

| Feld | Bedeutung |
|---|---|
| **Speichern unter:** | der Ordner, in dem die Dateien liegen |
| **Verbleibend:** | wie viel noch herunterzuladen ist |
| **Gesamtgröße:** | der ganze Torrent, übersprungene Dateien eingeschlossen |
| **Hinzugefügt am:**, **Abgeschlossen am:** | Datumsangaben |
| **Teile:** | „1200 x 1.00 MB (800 vorhanden)“: die Anzahl der Teile, ihre Größe und wie viele Sie haben |
| **Hash:** | der Info-Hash des Torrents |
| **Ihre Adresse laut Trackern:** | Ihre öffentliche Adresse, damit Sie erkennen, ob Ihr VPN verwendet wird. *Neu in 1.1.58.* |

## Tracker

![Der Reiter „Tracker“](images/tab-trackers.png)

Die Tracker des Torrents, je eine Zeile.

| Spalte | Bedeutung |
|---|---|
| **Name** | die Adresse des Trackers |
| **Status** | **Funktioniert**, **Funktioniert:** mit der Meldung des Trackers, **Aktualisiere...**, oder **Fehler** / die Fehlermeldung des Trackers; leer, wenn er noch nicht kontaktiert wurde |
| **Aktualisierung in** | Zeit bis zur nächsten Ankündigung |
| **Seeds**, **Peers**, **Heruntergeladen** | die Zahlen des Schwarms, wie der Tracker sie meldet |

Um die Tracker sofort zu kontaktieren, verwenden Sie **Tracker aktualisieren** im Kontextmenü. Um Tracker hinzuzufügen, zu entfernen oder umzusortieren, verwenden Sie die [Eigenschaften](#torrent-eigenschaften).

Klicken Sie auf eine Spaltenüberschrift, um die Tracker danach zu sortieren, wie in der [Dateiliste](#dateiliste-sortieren): **Seeds**, **Peers** und **Heruntergeladen** beginnen mit dem größten Wert, die anderen Spalten mit dem kleinsten; ein zweiter Klick kehrt die Reihenfolge um, ein dritter geht zur eigenen Reihenfolge des Torrents zurück (seine Stufen). Ein Tracker, der keine Zahl gemeldet hat, bleibt in beiden Fällen unten. Rookparcel merkt sich die Sortierung. *Neu in 1.2.69.*

## Peers

![Der Reiter „Peers“](images/tab-peers.png)

Die Peers, mit denen Sie verbunden sind. Ein Webseed zeigt seine Adresse in der Spalte **IP**.

| Spalte | Bedeutung |
|---|---|
| **IP** | die Adresse des Peers |
| **Client** | das BitTorrent-Programm des Peers |
| **Flags** | siehe unten |
| **%** | wie viel vom Torrent der Peer hat |
| **Download**, **Upload** | Geschwindigkeit von und zu diesem Peer |
| **Anfr.** | wartende Anfragen, von uns und an uns |
| **Heruntergeladen**, **Hochgeladen** | Summen mit diesem Peer |

Klicken Sie auf eine Spaltenüberschrift, um die Peers danach zu sortieren: **%**, die Geschwindigkeiten, **Anfr.** und die Summen beginnen mit dem größten Wert, **IP**, **Client** und **Flags** mit dem kleinsten (Adressen nach Zahlenwert); ein zweiter Klick kehrt die Reihenfolge um, ein dritter geht zur Reihenfolge zurück, in der die Peers eingetroffen sind. Die Liste behält ihre Sortierung, während Peers kommen und gehen und sich ihre Zahlen ändern, und die ausgewählten Peers bleiben ausgewählt. Rookparcel merkt sich die Sortierung. *Neu in 1.2.69.*

Die **Flags** sind die Buchstaben von µTorrent:

| Flag | Bedeutung |
|---|---|
| D / d | Sie laden vom Peer herunter / Sie möchten es, aber er sendet gerade nicht |
| U / u | Sie laden zum Peer hoch / er möchte Daten, aber Sie senden gerade nicht |
| O | optimistisches Unchoke: Sie geben dem Peer eine Chance |
| S | snubbed: der Peer hat aufgehört zu senden |
| I | der Peer hat sich mit Ihnen verbunden |
| E / e | die Verbindung ist verschlüsselt / nur ihr Handshake ist es |
| P | die Verbindung verwendet uTP |
| H | der Peer wurde über das DHT gefunden |
| X | der Peer wurde über den Peer-Austausch gefunden |
| L | der Peer wurde in Ihrem lokalen Netzwerk gefunden |

## Teile

Eine Karte mit einem Quadrat je Teil. Blaue Quadrate sind Teile, die Sie haben. Orange werden gerade heruntergeladen. Die übrigen sind Grüntöne: je dunkler, desto mehr verbundene Peers haben dieses Teil; weiß bedeutet, dass keiner es hat. Die Zeile über der Karte zählt die Teile. Sind es mehr, als hineinpassen, endet die Karte mit „... weitere“.

Die Liste unter der Karte zeigt die Teile, die gerade heruntergeladen werden: ihre Nummer (**#**), **Größe**, **Anzahl Blöcke**, einen Balken **Blöcke** und wie viele Blöcke **Abgeschlossen** sind. Im Balken sind blaue Blöcke auf dem Datenträger, grüne werden geschrieben, orange sind bei einem Peer angefordert, weiße sind noch frei.

## Dateien

![Der Reiter „Dateien“](images/tab-files.png)

Die Dateien des Torrents in ihren Ordnern, so wie das [Fenster „Torrent hinzufügen“](Torrents-hinzufügen#torrent-inhalt) sie zeigt.

| Spalte | Bedeutung |
|---|---|
| **Name** | der Name der Datei oder des Ordners; Ordner öffnen und schließen sich mit dem Pfeil daneben |
| **Größe** | ihre Größe |
| **Fertig** | wie viel davon heruntergeladen ist |
| **%** | dasselbe in Prozent |
| **Anzahl Teile** | wie viele Teile sie umfasst |
| **Erstes Teil** | die Nummer ihres ersten Teils |
| **Priorität** | **Hoch**, **Normal**, **Niedrig** oder **Nicht herunterladen**; **Gemischt** bei einem Ordner, dessen Dateien sich unterscheiden |

Eine Ordnerzeile addiert **Größe** und **Fertig** ihrer Dateien; sie hat keine Teile-Spalten.

### Dateiliste sortieren

*Neu in 1.2.65 im Windows-Fenster; im GTK-Fenster und in der macOS-App neu in 1.2.67.*

Klicken Sie auf eine Spaltenüberschrift, um die Dateien danach zu sortieren; ein Pfeil in der Überschrift zeigt Spalte und Richtung.

- Der erste Klick sortiert Namen, Größen und Teile vom kleinsten Wert an, **Fertig**, **%** und **Priorität** von oben. Ein zweiter Klick kehrt die Reihenfolge um, und ein dritter geht zur eigenen Reihenfolge des Torrents zurück.
- In der Baumansicht werden die Dateien und Ordner innerhalb jedes Ordners sortiert, Ordner zuerst.
- Eine nach **Fertig** oder **%** sortierte Liste bewegt sich, während der Download läuft, eine nach **Priorität** sortierte, wenn Sie Prioritäten ändern. Die ausgewählten Dateien bleiben ausgewählt.
- Rookparcel merkt sich die Sortierung. **Nach Dateireihenfolge priorisieren** folgt weiterhin der Reihenfolge im Torrent, ganz gleich, wie sortiert ist.

### Baumansicht oder eine Liste

*Neu in 1.2.21.*

Die Schaltfläche **Baumansicht** neben dem Filterfeld oder **Baumansicht** im Kontextmenü wechselt zwischen den Ordnern und der einzelnen Liste von Pfaden aus µTorrent 2.2.1 (der eigene Ordner des Torrents eingeschlossen). Rookparcel merkt sich die Wahl. In der Baumansicht:

- Klicken Sie auf den Pfeil, doppelklicken Sie auf einen Ordner oder drücken Sie <kbd>Right</kbd> / <kbd>Left</kbd>, um ihn zu öffnen oder zu schließen. <kbd>Left</kbd> auf einer Datei springt zu ihrem Ordner.
- Ordner bleiben offen oder geschlossen, während die Liste aktualisiert wird und nachdem Sie Prioritäten geändert haben. Wenn Sie einen anderen Torrent auswählen, öffnen sich wieder alle Ordner.
- Eine Priorität, **Nicht herunterladen** oder **Nach Dateireihenfolge priorisieren**, die Sie auf einem Ordner wählen, gilt für alle Dateien darin (bei eingeschaltetem Filter nur für die angezeigten). **Öffnen** und **Ordner öffnen** auf einem Ordner öffnen diesen Ordner; **Verschieben...** gilt nur für einzelne Dateien.

### Dateien auswählen und ihre Reihenfolge

Wählen Sie eine oder mehrere Dateien aus und klicken Sie mit der rechten Maustaste:

| Eintrag | Funktion |
|---|---|
| **Öffnen** | öffnet die Datei im Standardprogramm, wie ein Doppelklick im Explorer; nichts geschieht, solange sie noch nicht auf dem Datenträger ist |
| **Ordner öffnen** | öffnet ihren Ordner im Explorer, mit der Datei ausgewählt, sobald sie existiert; bis dahin den nächstgelegenen vorhandenen Ordner des Torrents |
| **Nach Dateireihenfolge priorisieren** | gibt den ausgewählten Dateien in ihrer Reihenfolge im Torrent fallende Prioritäten, sodass sie nacheinander fertig werden; Dateien, die Sie übersprungen hatten, werden wieder heruntergeladen |
| **Priorität > Hohe Priorität / Normale Priorität / Niedrige Priorität** | setzt die Priorität; Dateien mit höherer Priorität bekommen ihre Teile zuerst |
| **Nicht herunterladen** | überspringt die Dateien |
| **Verschieben...** | wählt einen neuen Ort für eine Datei und verschiebt sie dorthin; bei einer Datei, die nicht auf dem Datenträger ist, sagt es dem Torrent, dass er sie dort suchen soll |

Ein Doppelklick auf eine Datei oder <kbd>Enter</kbd> darauf öffnet sie wie **Öffnen**.

**Nicht herunterladen** ist der Weg, Dateien nach dem Hinzufügen eines Torrents zu überspringen. Wählen Sie erneut eine Priorität, wird eine übersprungene Datei heruntergeladen. Um Dateien zu überspringen, bevor der Download beginnt, entfernen Sie im [Fenster „Torrent hinzufügen“](Torrents-hinzufügen#torrent-inhalt) ihr Häkchen. Um in jedem Torrent, den Sie hinzufügen, Dateien mit bestimmten Namen zu überspringen, verwenden Sie **Dateien mit diesen Namen nicht herunterladen:** unter Einstellungen > Oberfläche ([Torrents hinzufügen](Torrents-hinzufügen#dateien-nach-namen-auslassen); *neu in 1.2.63*).

Führen Sie nach **Verschieben...** zu einer Datei, die nicht auf dem Datenträger war, **Erneut prüfen** aus, damit Rookparcel zählt, was es dort gefunden hat. „Verschieben...“ ersetzt nie eine Datei, die am neuen Ort schon vorhanden ist.

### Dateiliste filtern

*Neu in 1.1.56.*

Ein Torrent mit Tausenden von Dateien lässt sich mit dem Feld **Dateien filtern** oben rechts im Reiter „Dateien“ leichter handhaben.

1. Klicken Sie in das Feld, oder drücken Sie <kbd>Ctrl</kbd>+<kbd>F</kbd>, während der Reiter „Dateien“ den Fokus hat.
2. Geben Sie ein oder mehrere Wörter ein. Die Liste zeigt nur die Dateien, deren Pfad, Ordner eingeschlossen, jedes Wort enthält, ohne Beachtung der Groß- und Kleinschreibung; in der Baumansicht werden ihre Ordner geöffnet angezeigt. „12 von 3,400 Dateien“ neben dem Feld sagt, wie viele passen.
3. Wählen Sie die gewünschten Dateien aus und verwenden Sie das Kontextmenü. <kbd>Ctrl</kbd>+<kbd>A</kbd> wählt nur die angezeigten Dateien aus.
4. <kbd>Esc</kbd> leert das Feld; <kbd>Enter</kbd> springt in die Liste.

Der Text bleibt erhalten, wenn Sie einen anderen Torrent auswählen, sodass Sie denselben Filter auf mehrere anwenden können. Er wird nicht gespeichert, wenn Rookparcel geschlossen wird. **Nach Dateireihenfolge priorisieren** ordnet alle Dateien des Torrents, auch bei eingeschaltetem Filter.

Geben Sie zum Beispiel `sample` ein und setzen Sie die passenden Dateien auf **Nicht herunterladen**, oder geben Sie `s01 mkv` ein, um die Videodateien der ersten Staffel zu finden.

## Geschwindigkeit

Ein Diagramm der Download- (grün) und Upload-Geschwindigkeit (rot) aller Torrents zusammen über die letzten zehn Minuten, mit den aktuellen Werten oben.

## Protokoll

Das Protokoll von Rookparcel für diese Sitzung, jede Zeile mit der Uhrzeit. Es zeigt hinzugefügte und fertige Torrents, Verschiebungen, Änderungen von Zeitplaner und „Automatisch herunterfahren“, Probleme mit überwachten Ordnern und dem IP-Filter, RSS-Downloads, Importe und mehr. Es behält die letzten 1000 Zeilen.

Ein Tracker, der nicht antwortet, wird im [Reiter „Tracker“](#tracker) angezeigt, nicht hier: Bei öffentlichen Torrents ist das häufig und harmlos, weil Peers auch aus dem DHT und dem Peer-Austausch kommen. Die Ausnahme ist ein privater Torrent, der nur seine Tracker hat: Antwortet keiner von ihnen, sagt das Protokoll es einmal, mit dem letzten Fehler.

Wenn etwas nicht wie erwartet funktioniert, schauen Sie zuerst hier nach. Siehe auch [Fehlerbehebung und FAQ](Fehlerbehebung-und-FAQ).

## Torrent-Eigenschaften

![Das Fenster „Torrent-Eigenschaften“](images/properties.png)

Öffnen Sie es mit **Eigenschaften** im Kontextmenü, mit <kbd>Enter</kbd> in der Torrent-Liste oder per Doppelklick auf einen Torrent, der nicht fertig ist. Der Titel zeigt den Namen des Torrents oder „Eigenschaften - 3 Torrents“, wenn mehrere ausgewählt sind.

**Tracker (eine Leerzeile zwischen den Stufen)**

Eine Tracker-Adresse pro Zeile. Eine Leerzeile beginnt eine neue Stufe: Rookparcel probiert zuerst die Tracker der ersten Stufe und geht erst zur nächsten Stufe über, wenn alle versagen. Jede Zeile muss mit `http://`, `https://` oder `udp://` beginnen; eine wiederholte Adresse wird verworfen.

**Bandbreiteneinstellungen**

| Einstellung | Bedeutung |
|---|---|
| **Maximale Upload-Rate (kB/s): [0: unbegrenzt]** | das Upload-Limit dieses Torrents |
| **Maximale Download-Rate (kB/s): [0: unbegrenzt]** | das Download-Limit dieses Torrents |
| **Standardanzahl der Upload-Slots überschreiben** und **Anzahl der Upload-Slots: [0: unbegrenzt]** | an wie viele Peers dieser Torrent gleichzeitig hochlädt, statt des Standardwerts aus Einstellungen > BitTorrent |

**Seeding-Ziel**

Setzen Sie ein Häkchen bei **Standardeinstellungen überschreiben**, um diesem Torrent ein eigenes Ziel zu geben statt des Ziels unter Einstellungen > Warteschlange:

| Einstellung | Bedeutung |
|---|---|
| **Mindestverhältnis (%): [0: kein Verhältnisziel]** | laden Sie diesen Prozentsatz dessen hoch, was heruntergeladen wurde; 150 heißt anderthalbfach |
| **Mindest-Seedingzeit (Minuten): [0: kein Zeitziel]** | seeden Sie mindestens so lange |

Sind beide gesetzt, müssen beide erreicht werden. Was dann geschieht, stellen Sie unter Einstellungen > Warteschlange ein; siehe [Warteschlange und Seeding](Warteschlange-und-Seeding). Ein Torrent, der mit „Start erzwingen“ gestartet wurde, erreicht sein Ziel nie.

**Sonstige Einstellungen**

| Einstellung | Bedeutung |
|---|---|
| **Initial-Seeding** | wie im Kontextmenü; nur für Torrents, die seeden |
| **DHT aktivieren** | lässt diesen Torrent Peers über das DHT finden |
| **Peer-Austausch** | lässt diesen Torrent von anderen Peers etwas über Peers erfahren |
| **Sequenziell herunterladen** | lädt die Teile der Reihe nach herunter, wie es **Erweitert > Sequenziell herunterladen** im Kontextmenü tut; ausgegraut, wenn jeder Torrent fertig ist |
| **Erste und letzte Teile zuerst herunterladen** | lädt beide Enden jeder gewünschten Datei zuerst herunter, wie es **Erweitert > Erste und letzte Teile zuerst herunterladen** tut |

*Die beiden Kontrollkästchen zur Download-Reihenfolge sind neu in 1.2.63.* Wann sie helfen, steht unter [Torrents verwalten](Torrents-verwalten#download-reihenfolge).

Private Torrents verwenden nie das DHT oder den Peer-Austausch.

**OK** übernimmt Ihre Änderungen.

### Mehrere Torrents gleichzeitig

Wählen Sie mehrere Torrents aus und öffnen Sie die Eigenschaften, um sie gemeinsam zu ändern. Ein Feld, dessen Wert sich zwischen ihnen unterscheidet, wird leer geladen, und ein Kontrollkästchen zeigt einen dritten, gemischten Zustand. Ein Feld, das Sie so lassen, wie es geladen wurde, bleibt bei jedem Torrent unangetastet; nur was Sie ändern, wird bei allen übernommen.

## In der GTK-Version (Linux, BSD)

- Dieselben sieben Reiter mit denselben Spalten, dasselbe Menü und Filterfeld im Reiter „Dateien“ und dasselbe Fenster „Eigenschaften“.
- **Öffnen** und **Ordner öffnen** verwenden die Standardprogramme und den Dateimanager Ihres Desktops.

## In der macOS-App

- Dieselben sieben Reiter und Spalten, dasselbe Menü im Reiter „Dateien“ (mit **Öffnen** und **Ordner öffnen** im Finder), **Baumansicht** und Feld **Dateien filtern** und dasselbe Fenster „Eigenschaften“.

## In der Web-UI

- Dieselben sieben Reiter. Der Reiter „Allgemein“ zeigt zusätzlich **Erstellt am:**, **Erstellt mit:**, **Tracker:** und **Kommentar:**.
- Klicken Sie auf eine Spaltenüberschrift in der Tabelle eines Reiters, um danach zu sortieren.
- Auch der Reiter „Dateien“ hat das Feld **Dateien filtern**. Sein Kontextmenü enthält die vier Prioritäten: **Hohe Priorität**, **Normale Priorität**, **Niedrige Priorität** und **Nicht herunterladen**. Dateien öffnen und „Verschieben...“ sind aus einem Browser nicht möglich.
- **Eigenschaften** im Torrent-Menü öffnet dasselbe Fenster „Torrent-Eigenschaften“.

In einem Fenster, das mit **Datei > Mit Fernsitzung verbinden...** verbunden ist, hat das Menü des Reiters „Dateien“ kein **Öffnen**, **Ordner öffnen** oder **Verschieben...**, weil die Dateien auf dem anderen Computer liegen. Siehe [Fernsteuerung](Fernsteuerung).

## Verwandte Seiten

- [Torrents verwalten](Torrents-verwalten)
- [Das Hauptfenster](Das-Hauptfenster)
- [Warteschlange und Seeding](Warteschlange-und-Seeding)
- [Geschwindigkeitsbegrenzung und Zeitplan](Geschwindigkeitsbegrenzung-und-Zeitplan)
