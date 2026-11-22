*Dies ist eine Übersetzung der englischen Seite [Importing from µTorrent and qBittorrent](Importing-from-uTorrent-and-qBittorrent). Maßgeblich ist die englische Seite.*

# Import aus µTorrent und qBittorrent

Diese Seite erklärt, wie Sie Ihre Torrents von µTorrent oder qBittorrent auf demselben Computer übernehmen, sodass sie in Rookparcel dort weiterladen und seeden, wo sie stehen, ohne dass etwas erneut heruntergeladen wird.

## Was übernommen wird

Für jeden Torrent übernimmt Rookparcel:

- die Daten, die bleiben, wo sie sind: Nichts wird kopiert oder verschoben;
- die Statistik, die Dateiprioritäten (übersprungene Dateien bleiben übersprungen) und die Tracker;
- das Label: die Kategorie von qBittorrent oder das Label von µTorrent;
- die Tags von qBittorrent als Tags (*neu in 1.2.63*; vorher wurde das erste Tag zum Label eines Torrents ohne Kategorie). Ein Tag-Name mit Komma oder Steuerzeichen wird ausgelassen;
- einen gestoppten oder angehaltenen Zustand;
- von qBittorrent die eigenen Grenzen des Torrents für das Teilen (Verhältnis und Seeding-Zeit), die zu seinem eigenen [Seeding-Ziel](Torrent-Details-und-Dateien#torrent-eigenschaften) werden.

Rookparcel vertraut dem Eintrag des anderen Programms darüber, welche Teile vollständig sind, nicht: Es prüft die Dateien, bevor es sich auf sie verlässt. Große Torrents zeigen deshalb nach dem Import eine Weile **Prüfe**.

Die Dateien des anderen Programms werden nur gelesen, nie verändert. Teildateien, die µTorrent als `.!ut` oder qBittorrent als `.!qB` hinterlassen hat, werden übernommen und umbenannt, wenn der Torrent läuft: siehe [Unvollständige Dateien und .!ut](Datenträger-und-Speicher#unvollständige-dateien-und-ut) (*neu in 1.2.63*). Torrents, die schon in der Liste von Rookparcel sind, werden ausgelassen, Sie können den Import also gefahrlos erneut ausführen.

## Bevor Sie beginnen

1. **Schließen Sie das andere Programm**, auch sein Symbol im Infobereich. Beide Programme schreiben ihre Dateien beim Beenden neu, deshalb verweigert Rookparcel den Import, solange es µTorrent oder qBittorrent laufen sieht: „... läuft. Beenden Sie es und importieren Sie dann erneut: Beim Beenden schreibt es seine Dateien neu.“ Unter macOS und BSD kann Rookparcel nicht erkennen, ob es läuft; vergewissern Sie sich selbst.
2. Stellen Sie sicher, dass die Laufwerke mit den heruntergeladenen Daten verbunden sind, unter denselben Buchstaben oder Pfaden wie vorher.

Führen Sie nach dem Import nicht beide Programme mit denselben Torrents aus: Sie würden in dieselben Dateien schreiben.

## Importieren

1. Wählen Sie **Datei > Aus qBittorrent importieren...** oder **Datei > Aus µTorrent importieren...**.
2. Rookparcel sucht im üblichen Ordner des Programms (unten). Ist er dort nicht, öffnet sich eine Ordnerauswahl: „Ordner von qBittorrent auswählen“ oder „Ordner von µTorrent auswählen“. Wählen Sie den unten beschriebenen Ordner.
3. Rookparcel fragt: „47 Torrents aus qBittorrent importieren? Ihre Daten bleiben, wo sie sind, und die Dateien von qBittorrent werden nicht verändert. Torrents, die bereits in der Liste sind, werden ausgelassen.“ Die Frage listet die Ordner auf, in denen die Daten liegen. Prüfen Sie sie und klicken Sie dann auf **Ja**.
4. Eine Meldung sagt, wie viele hinzugefügt wurden, zum Beispiel „45 Torrents aus qBittorrent hinzugefügt. 2 wurden ausgelassen; der Reiter Protokoll nennt die Gründe.“

Die Torrents erscheinen in der Liste und beginnen mit der Prüfung. Ihre Labels erscheinen in der Kategorieliste unter **Labels** und die Tags von qBittorrent unter **Tags**.

## Wo Rookparcel sucht

| Programm | Üblicher Ordner | Was zu wählen ist, wenn danach gefragt wird |
|---|---|---|
| qBittorrent unter Windows | `%LOCALAPPDATA%\qBittorrent\BT_backup` | der Ordner `BT_backup` oder der Ordner `qBittorrent` darüber |
| qBittorrent unter Linux und BSD | `~/.local/share/qBittorrent/BT_backup` (oder unter `$XDG_DATA_HOME`) | derselbe |
| qBittorrent unter macOS | `~/Library/Application Support/qBittorrent/BT_backup` | derselbe |
| µTorrent unter Windows | `%APPDATA%\uTorrent` | der Ordner, der `resume.dat` enthält |

qBittorrent legt in `BT_backup` pro Torrent eine `.fastresume`- und eine `.torrent`-Datei ab; µTorrent legt `resume.dat` neben den `.torrent`-Dateien ab. Liegen sie bei Ihrer Kopie woanders, wählen Sie diesen Ordner. Die Daten bleiben in den Ordnern, die die Einträge nennen, diese Ordner müssen also auf diesem Computer existieren.

## Torrents, die ausgelassen wurden

Jeder importierte oder ausgelassene Torrent bekommt eine Zeile im [Reiter Protokoll](Torrent-Details-und-Dateien#protokoll): „Aus qBittorrent importiert: ...“ oder „Nicht aus qBittorrent importiert: ...: “, gefolgt vom Grund.

| Grund | Was zu tun ist |
|---|---|
| bereits in der Liste | nichts; er ist schon in Rookparcel |
| die .torrent-Datei fehlt | fügen Sie den Torrent selbst hinzu, aus seiner `.torrent`-Datei oder seinem Magnet-Link, mit **Speichern in** auf seinen Datenordner gesetzt |
| der Fortsetzungseintrag kann nicht gelesen werden | dasselbe |
| die .torrent-Datei kann nicht gelesen werden: ... | dasselbe, mit einer guten Kopie der `.torrent`-Datei |

Findet Rookparcel gar keine Torrents, sagt es „In ... wurden keine Torrents von qBittorrent gefunden.“ mit dem Ordner, den es gelesen hat.

## In der GTK-Version (Linux, BSD)

Dieselben zwei Einträge im Menü **Datei**, mit denselben Schritten. µTorrent hat außerhalb von Windows keinen üblichen Ordner, deshalb fragt Rookparcel nach dem Ordner mit `resume.dat`, zum Beispiel einem von einem Windows-Computer kopierten.

## Mit rookparcel-nox

Der Daemon ohne Fenster importiert beim Start:

```
rookparcel-nox run --import qbittorrent
rookparcel-nox run --import utorrent --import-dir "D:\Backup\uTorrent"
```

`--import` nimmt `qbittorrent` oder `utorrent`; `--import-dir` nennt den Ordner, wenn es nicht der übliche ist (`BT_backup` oder der Datenordner von qBittorrent, der Ordner von µTorrent mit `resume.dat`). Das andere Programm darf nicht laufen, und Torrents, die schon in der Liste von nox sind, bleiben, wie sie sind. Siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

Die Weboberfläche hat keinen Import, und in einem Fenster, das mit **Datei > Mit Fernsitzung verbinden...** verbunden ist, sind die Importeinträge ausgegraut: Der Server könnte die Ordner dieses Computers nicht lesen. Siehe [Fernsteuerung](Fernsteuerung).

## Verwandte Seiten

- [Torrents hinzufügen](Torrents-hinzufügen)
- [Torrents verwalten](Torrents-verwalten)
- [Warteschlange und Seeding](Warteschlange-und-Seeding)
