*Dies ist eine Übersetzung der englischen Seite [Adding torrents](Adding-torrents). Maßgeblich ist die englische Seite.*

# Torrents hinzufügen

Diese Seite zeigt jeden Weg, Rookparcel einen Torrent zu geben: eine `.torrent`-Datei, einen Magnet-Link, eine Webadresse, Ziehen und Ablegen, Ihren Browser und Dateimanager und einen überwachten Ordner. Sie erklärt auch das Fenster „Neuen Torrent hinzufügen“, in dem Sie den Ordner und die Dateien wählen, bevor der Download beginnt.

## Die schnellen Wege

| Was Sie haben | Was Sie tun |
|---|---|
| Eine `.torrent`-Datei auf Ihrem Computer | **Datei > Torrent hinzufügen...** (<kbd>Ctrl</kbd>+<kbd>O</kbd>) oder die erste Schaltfläche der Symbolleiste. Sie können mehrere Dateien auf einmal wählen. |
| Einen Magnet-Link | **Datei > Torrent von URL hinzufügen...** (<kbd>Ctrl</kbd>+<kbd>U</kbd>) und fügen Sie ihn ein. |
| Eine Webadresse einer `.torrent`-Datei (`http://` oder `https://`) | **Datei > Torrent von URL hinzufügen...** und fügen Sie sie ein. Rookparcel lädt die Datei herunter und fügt sie hinzu. |
| Eine `.torrent`-Datei im Explorer | Ziehen Sie sie auf das Rookparcel-Fenster oder doppelklicken Sie sie, wenn Rookparcel `.torrent`-Dateien öffnet. |
| Einen Magnet-Link in Ihrem Browser | Klicken Sie darauf, wenn Rookparcel Magnet-Links öffnet. |
| Viele `.torrent`-Dateien, die nach und nach eintreffen | Lassen Sie Rookparcel einen Ordner überwachen. Siehe [Ein überwachter Ordner](#ein-überwachter-ordner). |
| Ein Suchergebnis | Siehe [Torrents suchen](Torrents-suchen). |

Das Rechtsklickmenü des Infobereich-Symbols hat ebenfalls **Torrent hinzufügen...** und **Torrent von URL hinzufügen...**.

## Torrent von URL hinzufügen

![Das Fenster „Torrent von URL hinzufügen“](images/add-url.png)

1. Kopieren Sie den Magnet-Link oder die Adresse der `.torrent`-Datei.
2. Wählen Sie **Datei > Torrent von URL hinzufügen...**. Enthält die Zwischenablage einen Link, der mit `magnet:`, `http://` oder `https://` beginnt, ist das Feld schon ausgefüllt.
3. Klicken Sie auf **OK**.

Eine Adresse wird im Hintergrund heruntergeladen, über Ihren Proxy, wenn Einstellungen > Verbindung Tracker-Verbindungen über einen schickt. Manche Seiten antworten auf die Adresse mit einem Magnet-Link statt mit einer Datei; Rookparcel folgt auch dem. Schlägt der Download fehl, nennt eine Meldung „... konnte nicht heruntergeladen werden“ mit der Adresse und dem Grund.

## Ziehen und Ablegen

Ziehen Sie eine oder mehrere `.torrent`-Dateien aus dem Explorer auf das Hauptfenster. Jede wird hinzugefügt, als hätten Sie sie mit **Torrent hinzufügen...** gewählt.

## Aus Ihrem Browser und Dateimanager

Rookparcel kann das Programm sein, das Windows für `.torrent`-Dateien und `magnet:`-Links verwendet. Schalten Sie das unter **Optionen > Einstellungen > Allgemein** mit **Mit .torrent-Dateien verknüpfen** und **Mit magnet:-Links verknüpfen** ein oder im [Einrichtungsassistenten](Erster-Start). Einzelheiten stehen unter [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart), auch was zu tun ist, wenn Windows ein anderes Programm behält.

Läuft Rookparcel schon und Sie öffnen einen Torrent aus Ihrem Browser oder dem Explorer, übernimmt ihn das laufende Rookparcel. Ist **Programmfenster aktivieren** an (Einstellungen > Oberfläche, standardmäßig an), kommt auch das Hauptfenster nach vorn; ist es aus, erscheint nur das Fenster „Neuen Torrent hinzufügen“.

Sie können Rookparcel auch mit Torrents auf der Kommandozeile starten: `rookparcel.exe file.torrent "magnet:?xt=..."`. Jeder `.torrent`-Pfad, `magnet:`-Link und jede `http(s)://`-Adresse wird hinzugefügt. Läuft Rookparcel schon, gehen sie an das laufende Fenster.

Wurde das laufende Rookparcel mit Administratorrechten gestartet, lässt Windows nicht zu, dass eine zweite Kopie ihm einen Torrent übergibt. Rookparcel sagt Ihnen das dann; fügen Sie den Torrent im laufenden Fenster hinzu oder starten Sie Rookparcel ohne Administratorrechte.

## Das Fenster „Neuen Torrent hinzufügen“

![Das Fenster „Neuen Torrent hinzufügen“](images/add-new-torrent.png)

Ist **Ein Fenster mit den Dateien im Torrent anzeigen** an (Einstellungen > Oberfläche, standardmäßig an), öffnet sich dieses Fenster für jeden Torrent, den Sie hinzufügen. Ist es aus, werden Torrents sofort mit dem Standardordner und allen Dateien hinzugefügt.

Bei einem Magnet-Link öffnet sich das Fenster, sobald die Dateiliste des Torrents von anderen Peers eingetroffen ist. Bis dahin wartet der Torrent in der Liste als „Lade Metadaten“. Währenddessen schreibt er nichts auf den Datenträger. Beenden Sie Rookparcel, bevor die Liste eintrifft, kommt das Fenster beim nächsten Start wieder.

### Speichern in

Der Ordner, in den die Dateien des Torrents kommen. Die Liste enthält den Standard-Downloadordner (aus **Einstellungen > Ordner > Neue Downloads speichern in:**) und die letzten zehn Ordner, die Sie verwendet haben. Geben Sie einen Pfad ein oder klicken Sie auf **...**, um einen zu wählen. Ein Ordner, den es noch nicht gibt, wird angelegt. Der freie Platz auf diesem Laufwerk wird nach der Größe darunter angezeigt.

**Name:** der Name des Ordners des Torrents oder seiner einzigen Datei. Ändern Sie ihn, wird der Ordner oder die Datei auf dem Datenträger umbenannt.

### Torrent-Optionen

| Option | Was sie tut |
|---|---|
| **Hash-Prüfung überspringen** | Behandelt die Dateien, die schon im Ordner sind, als vollständig, ohne sie zu prüfen. Verwenden Sie das nur, wenn Sie wissen, dass die Daten vollständig und korrekt sind, zum Beispiel nach dem Kopieren von einem anderen Computer. Bei einem Magnet-Link ausgegraut: Seine Dateien wurden schon geprüft, als die Dateiliste eintraf. |
| **Torrent starten** | An: Der Torrent kommt in die Warteschlange und startet, wie die Warteschlange es zulässt. Aus: Er wird angehalten hinzugefügt. Es ist angehakt, außer **Download nicht automatisch starten** ist in Einstellungen > Oberfläche an. |
| **An den Anfang der Warteschlange** | Setzt den Torrent an die erste Stelle der Warteschlange, sodass er vor den anderen startet. |
| **Label:** | Gibt dem Torrent ein Label. Wählen Sie ein verwendetes oder geben Sie ein neues ein. Siehe [Torrents verwalten](Torrents-verwalten#labels). |
| **Sequenziell herunterladen** | Lädt die Teile der Reihe nach herunter, vom Anfang der Daten bis zu ihrem Ende, sodass ein Video schon beim Herunterladen abgespielt werden kann. Verwenden Sie es nur für das, was Sie ansehen wollen: Die übliche Reihenfolge „seltenste zuerst“ ist besser für die anderen Peers. |
| **Erste und letzte Teile zuerst herunterladen** | Lädt die ersten und letzten Teile jeder gewünschten Datei vor dem Rest herunter, damit ein Player den Header und den Index eines Videos früh lesen kann. |

*Die beiden Kästchen für die Download-Reihenfolge sind neu in 1.2.63.* Beide sind bei jedem Torrent zunächst nicht angehakt; Rookparcel merkt sie sich nicht. Um sie nach dem Hinzufügen zu ändern, verwenden Sie **Erweitert** im Rechtsklickmenü oder die Eigenschaften. Siehe [Torrents verwalten](Torrents-verwalten#download-reihenfolge).

### Torrent-Inhalt

Die obersten Zeilen zeigen **Name:**, **Kommentar:**, **Größe:** und **Datum:** des Torrents. Die Größe sagt, wie viel vom Torrent Sie ausgewählt haben, zum Beispiel „1.20 GB von 3.40 GB ausgewählt“, gefolgt vom freien Platz auf dem Datenträger.

Die Dateiliste zeigt die Ordner und Dateien des Torrents als Baum, jeweils mit einem Kästchen, ihrer Größe und ihrer Priorität.

- Nehmen Sie bei einer Datei den Haken heraus, um sie zu überspringen. Ein Ordner, dessen Dateien teilweise angehakt sind, zeigt „Gemischt“.
- **Alle auswählen** und **Keine auswählen** haken alles an oder nehmen alle Haken heraus.
- Klicken Sie auf das Symbol eines Ordners oder doppelklicken Sie ihn, um ihn zu öffnen oder zu schließen. <kbd>Right</kbd> und <kbd>Left</kbd> (oder <kbd>+</kbd> und <kbd>-</kbd> auf dem Ziffernblock) tun dasselbe; <kbd>Space</kbd> hakt die ausgewählten Zeilen an oder nimmt den Haken heraus.
- Klicken Sie ausgewählte Zeilen mit der rechten Maustaste an für **Hohe Priorität**, **Normale Priorität**, **Niedrige Priorität**, **Nicht herunterladen**, **Auswählen** und **Abwählen**. Eine Priorität zu wählen hakt die Dateien an; **Nicht herunterladen** nimmt die Haken heraus.

Dateien, deren Namen zu **Dateien mit diesen Namen nicht herunterladen:** passen (Einstellungen > Oberfläche), sind hier zunächst nicht angehakt. Haken Sie sie an, wenn Sie sie doch wollen: Was Sie in diesem Fenster wählen, gilt immer. Siehe [Dateien nach Namen auslassen](#dateien-nach-namen-auslassen).

Die Auswahl der Dateien und die Prioritäten können Sie später im [Reiter Dateien](Torrent-Details-und-Dateien#dateien) ändern.

### OK, Abbrechen und Nicht mehr anzeigen

**OK** fügt den Torrent mit Ihrer Auswahl hinzu. **Abbrechen** fügt nichts hinzu; bei einem Magnet-Link entfernt es den wartenden Torrent. Wenn Sie **Nicht mehr anzeigen** anhaken, wird **Ein Fenster mit den Dateien im Torrent anzeigen** ausgeschaltet, sodass spätere Torrents ohne dieses Fenster hinzugefügt werden. Schalten Sie es in Einstellungen > Oberfläche wieder ein.

Rookparcel merkt sich die Größe des Fensters.

## Torrents angehalten hinzufügen

Um Torrents hinzuzufügen, ohne dass etwas heruntergeladen wird, bis Sie es sagen, schalten Sie **Download nicht automatisch starten** unter **Optionen > Einstellungen > Oberfläche** ein. Neue Torrents werden angehalten und außerhalb der Warteschlange hinzugefügt. Starten Sie sie mit **Starten**, wenn Sie bereit sind. Im Fenster „Neuen Torrent hinzufügen“ zeigt sich das so, dass **Torrent starten** nicht angehakt ist.

## Dateien nach Namen auslassen

*Neu in 1.2.63.* Manche Torrents enthalten Dateien, die Sie nie wollen, etwa Verknüpfungen, Vorschaubild-Caches oder Beispielvideos. Rookparcel kann sie nach Namen auslassen:

1. Öffnen Sie **Optionen > Einstellungen > Oberfläche**.
2. Haken Sie unter **Beim Hinzufügen von Torrents** **Dateien mit diesen Namen nicht herunterladen:** an.
3. Geben Sie im Feld darunter die Namen ein, durch Semikolons getrennt. `*` steht für beliebige Zeichen und `?` für eines, Groß- und Kleinschreibung spielt keine Rolle. Zum Beispiel: `*.lnk; Thumbs.db; *sample*`.
4. Klicken Sie auf **OK**.

Verglichen wird nur der eigene Name einer Datei, nicht die Ordner, in denen sie liegt. Die Liste gilt für Torrents, die Sie ab jetzt hinzufügen:

- Im Fenster „Neuen Torrent hinzufügen“ sind die passenden Dateien zunächst nicht angehakt, und Sie können sie wieder anhaken.
- Torrents, die ohne das Fenster hinzugefügt werden, bekommen die passenden Dateien auf **Nicht herunterladen** gesetzt: bei ausgeschaltetem Fenster, aus dem [überwachten Ordner](#ein-überwachter-ordner), aus RSS, aus einem Link, aus der Weboberfläche und von Apps wie Sonarr und Radarr. Bei einem Magnet-Link geschieht das, sobald seine Dateiliste eingetroffen ist.
- Ein Torrent, dessen sämtliche Dateien passen, wird mit allen seinen Dateien hinzugefügt, als wäre die Liste leer.
- Torrents, die schon in der Liste sind, beim Start wiederhergestellte Torrents und importierte Torrents werden nicht geändert.

Ein Magnet-Link, der ohne das Fenster hinzugefügt wird und dessen Dateiliste erst nach einem Neustart von Rookparcel eintrifft, lädt jede Datei herunter, ebenso ein Torrent, der `rookparcel-nox` auf der Kommandozeile übergeben wird (`add`, oder `run` mit einer Datei oder einem Link). Um später eine ausgelassene Datei zu bekommen, geben Sie ihr im [Reiter Dateien](Torrent-Details-und-Dateien#dateien) eine Priorität.

## Ein Torrent, der schon in der Liste ist

Fügen Sie einen Torrent hinzu, der schon in der Liste ist, sagt Rookparcel „... ist bereits in der Torrent-Liste.“ und fügt nichts hinzu.

## Ein überwachter Ordner

Rookparcel kann einen Ordner überwachen und jede neue `.torrent`-Datei hinzufügen, die dort abgelegt wird:

1. Öffnen Sie **Optionen > Einstellungen > Ordner**.
2. Haken Sie **.torrent-Dateien automatisch laden aus:** an und wählen Sie den Ordner.
3. Haken Sie optional **Geladene .torrent-Dateien löschen** an.
4. Klicken Sie auf **OK**.

Rookparcel fügt eine neue Datei einige Sekunden hinzu, nachdem sie sich nicht mehr ändert, ohne das Fenster „Neuen Torrent hinzufügen“. Eine geladene Datei wird in `.torrent.loaded` umbenannt oder gelöscht, wenn **Geladene .torrent-Dateien löschen** an ist. Probleme werden in den [Reiter Protokoll](Torrent-Details-und-Dateien#protokoll) geschrieben.

Dieselbe Seite kann eine Kopie jeder hinzugefügten `.torrent`-Datei aufbewahren (**.torrent-Dateien speichern in:**). Siehe [Datenträger und Speicher](Datenträger-und-Speicher) und [Einstellungen im Überblick](Einstellungen-im-Überblick).

## In der GTK-Version (Linux, BSD)

- Dieselben Menüeinträge, Tastenkürzel und dasselbe Fenster „Neuen Torrent hinzufügen“.
- **Torrent von URL hinzufügen** füllt einen Link aus der Zwischenablage auf dieselbe Weise ein. Text, der kein `magnet:`-Link und keine `http(s)://`-Adresse ist, ergibt „Kein Link, den Rookparcel hinzufügen kann“.
- Das GTK-Fenster nimmt keine abgelegten Dateien an. Verwenden Sie **Datei > Torrent hinzufügen...** oder öffnen Sie die Datei aus Ihrem Dateimanager mit Rookparcel.
- Unter Linux und BSD stehen die Dateiverknüpfungen unter **Desktop-Integration** auf Einstellungen > Allgemein. In einem Flatpak wählen Sie das Standardprogramm stattdessen in den Einstellungen Ihres Desktops. Siehe [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart).

## In der Weboberfläche

- **Torrent hinzufügen** lädt eine oder mehrere `.torrent`-Dateien von dem Computer hoch, auf dem der Browser läuft; **URL hinzufügen** fragt nach einem Magnet-Link oder einer Adresse. Ein bloßer Info-Hash wird ebenfalls als Magnet-Link genommen.
- Sie können `.torrent`-Dateien oder einen Magnet-Link auf die Seite ablegen oder einen Magnet-Link an beliebiger Stelle außerhalb eines Textfelds einfügen.
- Dasselbe Fenster „Neuen Torrent hinzufügen“ öffnet sich in der Seite. **Speichern in** ist ein Ordner auf dem Server, als Pfad eingegeben.
- Auf einem Smartphone steht **URL hinzufügen** im Menü **Mehr** (⋯).

Siehe [Weboberfläche](Weboberfläche). In einem Fenster, das mit **Datei > Mit Fernsitzung verbinden...** verbunden ist, ist die Schaltfläche **...** in „Neuen Torrent hinzufügen“ ausgegraut: Geben Sie einen Ordner auf dem Server ein. Siehe [Fernsteuerung](Fernsteuerung).

## Verwandte Seiten

- [Das Hauptfenster](Das-Hauptfenster)
- [Torrents verwalten](Torrents-verwalten)
- [Import aus µTorrent und qBittorrent](Import-aus-uTorrent-und-qBittorrent)
- [Downloads per RSS](Downloads-per-RSS)
