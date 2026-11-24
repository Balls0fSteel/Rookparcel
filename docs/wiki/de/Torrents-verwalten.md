*Dies ist eine Übersetzung der englischen Seite [Managing torrents](https://github.com/Balls0fSteel/Rookparcel/wiki/Managing-torrents). Maßgeblich ist die englische Seite.*

# Torrents verwalten

Diese Seite behandelt, was Sie mit Torrents tun, sobald sie in der Liste stehen: starten und stoppen, die Warteschlange, Entfernen mit oder ohne Dateien, erneut prüfen, Dateien verschieben, Labels und Tags, Geschwindigkeitsbegrenzungen für einzelne Torrents und das Kontextmenü, das all das enthält.

Jede Aktion wirkt auf die ausgewählten Torrents. Wählen Sie mehrere mit <kbd>Ctrl</kbd>+Klick oder <kbd>Shift</kbd>+Klick aus, oder alle mit <kbd>Ctrl</kbd>+<kbd>A</kbd>, und behandeln Sie sie in einem Zug.

## Das Kontextmenü

![Das Kontextmenü der Torrent-Liste](images/torrent-menu.png)

Klicken Sie mit der rechten Maustaste auf einen Torrent (oder drücken Sie <kbd>Shift</kbd>+<kbd>F10</kbd>), um dieses Menü zu öffnen:

| Eintrag | Funktion | Taste |
|---|---|---|
| **Ordner öffnen** | öffnet den Ordner des Torrents im Explorer, mit seinen Daten ausgewählt | |
| **Magnet-Link kopieren** | legt den Magnet-Link des Torrents in die Zwischenablage | |
| **.torrent exportieren...** | speichert die `.torrent`-Datei des Torrents | |
| **Start erzwingen** | startet sofort, außerhalb der Warteschlange | |
| **Starten** | übergibt den Torrent an die Warteschlange | <kbd>Ctrl</kbd>+<kbd>S</kbd> |
| **Anhalten** | hält ihn an und nimmt ihn aus der Warteschlange; zeigt **Angehalten** | <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd> |
| **Stoppen** | stoppt ihn und nimmt ihn aus der Warteschlange; zeigt **Gestoppt** oder **Fertig** | <kbd>Ctrl</kbd>+<kbd>T</kbd> |
| **In der Warteschlange nach oben**, **In der Warteschlange nach unten** | einen Platz nach oben oder unten | |
| **An den Anfang der Warteschlange**, **An das Ende der Warteschlange** | an den Anfang oder das Ende, unter Beibehaltung ihrer Reihenfolge | |
| **Entfernen** | entfernt ihn aus der Liste; die Dateien bleiben | <kbd>Del</kbd> |
| **Entfernen und > Daten löschen** | entfernt ihn und löscht seine Dateien | <kbd>Shift</kbd>+<kbd>Del</kbd> |
| **Erneut prüfen** | prüft die Dateien auf dem Datenträger noch einmal | |
| **Tracker aktualisieren** | kontaktiert die Tracker jetzt | |
| **Erweitert > Speicherort festlegen...** | verschiebt seine Dateien in einen anderen Ordner | |
| **Erweitert > Initial-Seeding** | schaltet das Initial-Seeding ein oder aus | |
| **Erweitert > Sequenziell herunterladen** | lädt die Teile der Reihe nach herunter, vom Anfang bis zum Ende | |
| **Erweitert > Erste und letzte Teile zuerst herunterladen** | lädt beide Enden jeder gewünschten Datei zuerst herunter | |
| **Bandbreite > Download-Limit festlegen / Upload-Limit festlegen** | eigene Geschwindigkeitsbegrenzungen | |
| **Label** | gibt ihm ein Label, ein neues oder keines | |
| **Tags** | gibt ihm Tags oder nimmt sie weg | |
| **Eigenschaften** | Tracker, Begrenzungen, Seeding-Ziel und mehr | <kbd>Enter</kbd> |

Die folgenden Abschnitte erklären jede Gruppe. Die meisten Einträge gibt es auch in der Symbolleiste; siehe [Das Hauptfenster](Das-Hauptfenster#die-symbolleiste).

## Starten, Anhalten, Stoppen und Start erzwingen

Rookparcel führt eine Warteschlange, wie µTorrent. Wie viele Torrents gleichzeitig laufen dürfen, stellen Sie unter **Optionen > Einstellungen > Warteschlange** ein; siehe [Warteschlange und Seeding](Warteschlange-und-Seeding).

- **Starten** übergibt den Torrent an die Warteschlange. Ist Platz, läuft er (**Lädt herunter** oder **Seeding**). Wenn nicht, wartet er als **Eingereiht** oder **Seed eingereiht**, bis ein Platz frei wird.
- **Start erzwingen** lässt den Torrent sofort laufen, auch wenn die Warteschlange voll ist. Sein Status beginnt mit **[F]**, zum Beispiel **[F] Seeding**. Ein erzwungener Torrent erreicht nie ein Seeding-Ziel, er seedet also, bis Sie ihn stoppen.
- **Anhalten** und **Stoppen** halten den Torrent beide an und nehmen ihn aus der Warteschlange. Der einzige Unterschied ist das Wort in der Spalte „Status“: **Angehalten**, oder **Gestoppt** (**Fertig** bei einem vollständigen Torrent). Beides bleibt nach einem Neustart erhalten. Mit „Starten“ oder „Start erzwingen“ geht es weiter.

Das Menü des Tray-Symbols enthält **Alle Torrents starten** und **Alle Torrents stoppen**; sie wirken auf jeden Torrent in der Liste.

Wie Sie neue Torrents gestoppt hinzufügen, steht unter [Torrents hinzufügen](Torrents-hinzufügen#torrents-angehalten-hinzufügen).

## Die Warteschlange

Die Spalte **#** zeigt den Platz jedes Torrents in der Warteschlange. Niedrigere Nummern sind zuerst dran. Mit **In der Warteschlange nach oben** und **In der Warteschlange nach unten** (auch in der Symbolleiste) oder **An den Anfang der Warteschlange** und **An das Ende der Warteschlange** ändern Sie die Reihenfolge. Werden mehrere Torrents verschoben, behalten sie ihre Reihenfolge untereinander.

Im Fenster „Torrent hinzufügen“ stellt **An den Anfang der Warteschlange** einen neuen Torrent an die erste Stelle.

## Torrents entfernen

- **Entfernen** (<kbd>Del</kbd>) nimmt die Torrents aus der Liste. Die heruntergeladenen Daten bleiben auf dem Datenträger.
- **Entfernen und > Daten löschen** (<kbd>Shift</kbd>+<kbd>Del</kbd>) nimmt sie heraus und löscht ihre heruntergeladenen Dateien. Unter Windows umgehen die Dateien den Papierkorb und lassen sich von dort nicht wiederherstellen. Eine Teildatei, die auf `.!ut` oder `.!qB` endet und von der Rookparcel nicht belegen kann, dass sie die Daten dieses Torrents enthält, bleibt erhalten; siehe [Datenträger und Speicher](Datenträger-und-Speicher#unvollständige-dateien-und-ut).

Ist **Löschen von Torrents bestätigen** eingeschaltet (Einstellungen > Oberfläche, standardmäßig an), fragt Rookparcel zuerst nach und sagt, ob auch die Dateien gelöscht werden. Ist es ausgeschaltet, handelt „Entfernen“ sofort.

## Erneut prüfen

**Erneut prüfen** liest die Dateien des Torrents auf dem Datenträger Teil für Teil und zählt nur die Teile, die vollständig und korrekt sind. Der Status zeigt **Zur Prüfung eingereiht**, solange die Prüfung auf ihr Laufwerk wartet (Prüfungen laufen ein Torrent je Laufwerk; *neu in 1.2.76*), dann **Prüfe** mit einer Prozentangabe. Nutzen Sie es, nachdem Sie Dateien von Hand verschoben oder wiederhergestellt haben, oder wenn Sie beschädigte Daten vermuten.

*Neu in 1.2.76:* Ein gestoppter Torrent startet mit der Prüfung; stoppen Sie ihn danach erneut, wenn er gestoppt sein soll. Ein eingereihter Torrent wird sofort geprüft, ohne auf einen Platz in der Warteschlange zu warten, und danach wieder eingereiht. Ein Torrent, der mit **fehlenden Dateien** angehalten ist (siehe [Datenträger und Speicher](Datenträger-und-Speicher#wenn-ein-laufwerk-oder-eine-freigabe-fehlt)), wird gegen das geprüft, was jetzt vorhanden ist, und was fehlt, wird erneut heruntergeladen.

Wie schonend Prüfungen den Datenträger nutzen, stellen Sie unter **Einstellungen > Gaming** (zunächst aus) ein; siehe [Datenträger und Speicher](Datenträger-und-Speicher).

## Tracker aktualisieren

**Tracker aktualisieren** kontaktiert die Tracker des Torrents sofort, ohne auf die nächste Ankündigung zu warten, um den Fortschritt zu melden und weitere Peers zu bekommen. Der [Reiter „Tracker“](Torrent-Details-und-Dateien#tracker) zeigt die Antwort.

## Dateien verschieben: Speicherort festlegen

1. Wählen Sie die Torrents aus.
2. Wählen Sie **Erweitert > Speicherort festlegen...**.
3. Wählen Sie den neuen Ordner.

Rookparcel verschiebt die Dateien dorthin und lädt oder seedet vom neuen Ort aus weiter. Der Status zeigt währenddessen **Wird verschoben**, und der Eintrag ist bei einem Torrent, der bereits verschoben wird, ausgegraut. Gleichnamige Dateien, die schon im neuen Ordner liegen, werden ersetzt.

Wenn Sie beenden, während Dateien verschoben werden, wartet Rookparcel, bis die Verschiebungen fertig sind, und zeigt deren Fortschritt. **Jetzt beenden** bricht eine Verschiebung ab: Ihre Dateien bleiben im alten Ordner, und was schon kopiert war, wird gelöscht. Verschieben Sie sie später erneut.

Wie Sie fertige Downloads automatisch verschieben, steht unter **Abgeschlossene Downloads verschieben nach:** auf [Datenträger und Speicher](Datenträger-und-Speicher).

## Den Ordner öffnen

**Ordner öffnen** öffnet den Ordner des Torrents im Explorer, mit seiner Datei oder seinem Ordner ausgewählt, sobald er existiert. Sind mehrere Torrents ausgewählt, wird der des ersten geöffnet. Ein Doppelklick auf einen fertigen Torrent tut dasselbe.

Um eine einzelne Datei zu öffnen, verwenden Sie den [Reiter „Dateien“](Torrent-Details-und-Dateien#dateien).

## Magnet-Link kopieren und .torrent exportieren

*Neu in 1.2.63.*

**Magnet-Link kopieren** legt die Magnet-Links der ausgewählten Torrents in die Zwischenablage, einen pro Zeile. Jeder Link enthält den Info-Hash des Torrents, seinen Namen und seine Tracker. Fügen Sie ihn in ein anderes Torrent-Programm ein oder schicken Sie ihn jemandem, um den Torrent ohne Datei weiterzugeben.

Weil der Link die Tracker enthält, geben Sie keinen Link weiter, der von einem Torrent eines privaten Trackers kopiert wurde. Die Adresse eines privaten Trackers enthält meist Ihren persönlichen Schlüssel (einen Passkey), und jeder mit dem Link könnte sich als Sie ausgeben.

**.torrent exportieren...** speichert die `.torrent`-Dateien der ausgewählten Torrents. Die Datei enthält den Torrent so, wie Rookparcel ihn erhalten hat, mit den Trackern und Webseeds, die er jetzt verwendet. Sie können sie auf einem anderen Computer hinzufügen oder als Kopie aufbewahren.

- Ist ein Torrent ausgewählt, fragt Rookparcel nach einem Dateinamen.
- Bei mehreren fragt es nach einem Ordner und speichert jeden als `<Name>.torrent`. Ist ein Name vergeben, hängt es eine Zahl an, etwa ` (2)`, statt die Datei zu ersetzen.
- Ein Torrent, der aus einem Magnet-Link hinzugefügt wurde, hat keine `.torrent`-Datei, bis seine Dateiliste eingetroffen ist („Lade Metadaten“). Der Eintrag ist ausgegraut, wenn kein ausgewählter Torrent eine hat.

Kann die benannte Datei nicht geschrieben werden, meldet Rookparcel „… konnte nicht exportiert werden“ mit dem Namen des Torrents und dem Grund. Dateien, bei denen ein Export in einen Ordner fehlschlägt, werden stattdessen in den [Reiter „Protokoll“](Torrent-Details-und-Dateien#protokoll) geschrieben.

## Labels

Ein Label ist ein Name, mit dem Sie Torrents gruppieren, etwa „Filme“ oder „Linux“. Jedes verwendete Label erscheint unter **Labels** in der Kategorienliste, mit einer Anzahl, sodass ein Klick nur diese Torrents zeigt.

- **Label > Neues Label...** fragt nach einem Namen und gibt ihn den ausgewählten Torrents.
- **Label >** *ein Label* vergibt ein vorhandenes Label.
- **Label > Label entfernen** nimmt das Label weg.

Das Menü hakt das Label ab, das die ausgewählten Torrents gemeinsam haben. Sie können ein Label auch im Fenster „Torrent hinzufügen“ vergeben, und RSS-Filter können eines vergeben.

Ist neben **Abgeschlossene Downloads verschieben nach:** (Einstellungen > Ordner) **Label des Torrents anhängen** eingeschaltet, wandert ein fertiger Torrent mit Label in einen Ordner, der nach seinem Label benannt ist. Siehe [Datenträger und Speicher](Datenträger-und-Speicher).

## Tags

*Neu in 1.2.63.*

Ein Torrent hat ein Label, kann daneben aber beliebig viele Tags haben, etwa „4K“, „behalten“ und „Linux-ISOs“. Tags funktionieren wie die von qBittorrent. Jedes Tag erscheint unter **Tags** in der Kategorienliste, unter den Labels, mit einer Anzahl; ein Torrent mit mehreren Tags wird unter jedem gezählt. **Ohne Tag** zeigt die Torrents ohne jedes Tag.

- **Tags > Neues Tag...** fragt nach einem Namen und gibt das neue Tag den ausgewählten Torrents.
- **Tags >** *ein Tag* gibt dieses Tag den ausgewählten Torrents. Ein Tag, das alle ausgewählten Torrents haben, ist abgehakt; wählen Sie es, nimmt es allen ab. Ein Tag, das nur einige von ihnen haben, ist nicht abgehakt (die macOS-App zeigt eine gemischte Markierung); wählen Sie es, bekommen es alle.
- **Tags > Alle Tags entfernen** nimmt den ausgewählten Torrents jedes Tag ab. Die Tags selbst bleiben.

Ein Tag bleibt in der Kategorienliste, wenn kein Torrent es mehr hat, mit der Anzahl 0. Um es loszuwerden, klicken Sie in der Kategorienliste mit der rechten Maustaste darauf und wählen **Tag löschen**: Rookparcel fragt „Das Tag „*Name*“ löschen? Alle Torrents, die es haben, verlieren es.“ Die Torrents selbst bleiben. **Neues Tag...** in diesem Menü legt ein Tag an, ohne es einem Torrent zu geben.

Ein Name darf nicht leer sein und kein Komma, keinen Tabulator und keinen Zeilenumbruch enthalten. Leerzeichen an den Enden werden entfernt, und ein Name, der länger als 128 Byte ist, wird gekürzt. Groß- und Kleinschreibung zählt: `tv` und `TV` sind zwei Tags. Es kann höchstens 1024 Tags geben.

Die Spalte **Tags** zeigt die Tags eines Torrents; sie ist zunächst ausgeblendet (siehe [Das Hauptfenster](Das-Hauptfenster#spalten)). Das Suchfeld findet nur Torrent-Namen, keine Tags; klicken Sie stattdessen in der Kategorienliste auf ein Tag. Das Fenster „Torrent hinzufügen“ hat keine Tags: Vergeben Sie sie über das Kontextmenü, sobald der Torrent in der Liste steht. Apps, die die qBittorrent-API verwenden, etwa Sonarr, können ebenfalls Tags vergeben ([Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr#kategorien-labels-und-tags)).

## Geschwindigkeitsbegrenzung für einen Torrent

**Bandbreite > Download-Limit festlegen** und **Upload-Limit festlegen** bieten **Unbegrenzt**, 10, 25, 50, 100, 250, 500, 1000, 2500 und 5000 kB/s. Die aktuelle Wahl ist abgehakt. Für jede andere Zahl verwenden Sie die [Eigenschaften](Torrent-Details-und-Dateien#torrent-eigenschaften). Die globalen Begrenzungen in den Einstellungen gelten zusätzlich. Siehe [Geschwindigkeitsbegrenzung und Zeitplan](Geschwindigkeitsbegrenzung-und-Zeitplan).

## Initial-Seeding

**Erweitert > Initial-Seeding** ist für einen Torrent, den Sie als Erster teilen. Jeder Peer bekommt Teile, die noch niemand sonst hat, sodass sich ein neuer Torrent schneller verbreitet. Es lässt sich nur einschalten, wenn jeder ausgewählte Torrent seedet. Der Status zeigt **[S] Seeding**, solange es an ist. Wählen Sie den Eintrag erneut, um es auszuschalten.

## Download-Reihenfolge

*Neu in 1.2.63.*

Standardmäßig lädt Rookparcel zuerst die Teile herunter, die unter den Peers am seltensten sind. Das hält jedes Teil im Schwarm verfügbar und ist für alle, die den Torrent teilen, am besten. Zwei Einträge unter **Erweitert** ändern die Reihenfolge, vor allem, damit Sie ein Video ansehen können, während es heruntergeladen wird. Jeder ist abgehakt, solange er an ist; wählen Sie ihn erneut, um ihn auszuschalten.

- **Sequenziell herunterladen** lädt die Teile der Reihe nach herunter, vom Anfang der Daten bis zum Ende, sodass ein Player das Video abspielen kann, während es eintrifft. Dateien mit höherer Priorität kommen weiterhin zuerst. Das geht zulasten des Schwarms: Die seltenen Teile, die Sie sonst weitergegeben hätten, kommen spät. Nutzen Sie es für einen Torrent, den Sie vorab ansehen wollen, und lassen Sie es sonst aus.
- **Erste und letzte Teile zuerst herunterladen** lädt die ersten und letzten Teile jeder gewünschten Datei vor dem Rest herunter: 1 % der Datei an jedem Ende, mindestens ein Teil. Viele Videoformate haben dort ihren Header und Index, sodass ein Player die unfertige Datei öffnen kann. Übersprungene Dateien bleiben unberührt.

Beide sind ausgegraut, wenn jeder ausgewählte Torrent fertig ist. Jeder Torrent behält seine Wahl nach einem Neustart. Sie können sie auch beim Hinzufügen eines Torrents setzen, im [Fenster „Torrent hinzufügen“](Torrents-hinzufügen#torrent-optionen), und in den [Eigenschaften](Torrent-Details-und-Dateien#torrent-eigenschaften).

## Eigenschaften

**Eigenschaften** (<kbd>Enter</kbd>, oder Doppelklick auf einen Torrent, der nicht fertig ist) öffnet das Fenster „Torrent-Eigenschaften“: Tracker, Geschwindigkeitsbegrenzungen, Upload-Slots, das Seeding-Ziel, DHT und Peer-Austausch sowie die Download-Reihenfolge für diesen Torrent. Siehe [Torrent-Details und Dateien](Torrent-Details-und-Dateien#torrent-eigenschaften).

## In der GTK-Version (Linux, BSD)

- Dasselbe Kontextmenü in derselben Reihenfolge, nur dass **Magnet-Link kopieren** und **.torrent exportieren...** eine eigene Gruppe unmittelbar vor **Erweitert** haben. Die macOS-App legt sie ebenfalls dorthin.
- Tastenkürzel: <kbd>Delete</kbd> entfernt, <kbd>Shift</kbd>+<kbd>Delete</kbd> entfernt und löscht die Dateien, <kbd>Ctrl</kbd>+<kbd>A</kbd> wählt alles aus. Starten, Anhalten und Stoppen haben kein Kürzel.
- **Ordner öffnen** öffnet den Ordner in Ihrem Dateimanager.
- Unter Linux und BSD werden gelöschte Dateien vom Datenträger entfernt, nicht in den Papierkorb verschoben.

## In der Web-UI

- Dasselbe Kontextmenü, nur dass **Ordner öffnen** ausgegraut ist („nicht aus einem Browser“) und **Magnet-Link kopieren** und **.torrent exportieren...** unmittelbar vor **Erweitert** stehen. **Tags** führt zuerst **Neues Tag...** und **Alle Tags entfernen** auf, dann die Tags.
- **.torrent exportieren...** lädt die Datei über den Browser in dessen Download-Ordner herunter. Bei mehreren Torrents lädt der Browser jede Datei nacheinander herunter, höchstens 50 pro Klick.
- **Magnet-Link kopieren** braucht den Zugriff des Browsers auf die Zwischenablage. Eine Seite, die über einfaches `http://` von einem anderen Computer geöffnet wurde, hat ihn nicht; dann zeigt die Seite die Links stattdessen unter „Kopieren Sie den folgenden Link:“, damit Sie sie von Hand kopieren.
- **Speicherort festlegen** fragt nach einem Ordner auf dem Server, als Pfad eingetippt.
- Gelöschte Dateien werden vom Datenträger entfernt, nicht in den Papierkorb verschoben.
- Auf einem Smartphone lässt das Menü **Ordner öffnen** weg und beginnt mit **Details**.

## In einem Fernfenster

In einem Fenster, das mit **Datei > Mit Fernsitzung verbinden...** verbunden ist, liegen die Dateien auf dem anderen Computer. **Ordner öffnen** ist ausgegraut, und **Speicherort festlegen...** fragt nach einem Ordner auf dem Server. Siehe [Fernsteuerung](Fernsteuerung).

## Verwandte Seiten

- [Das Hauptfenster](Das-Hauptfenster)
- [Torrent-Details und Dateien](Torrent-Details-und-Dateien)
- [Warteschlange und Seeding](Warteschlange-und-Seeding)
- [Tastenkürzel](Tastenkürzel)
