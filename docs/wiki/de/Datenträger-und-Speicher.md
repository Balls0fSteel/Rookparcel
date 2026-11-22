*Dies ist eine Übersetzung der englischen Seite [Disk and storage](Disk-and-storage). Maßgeblich ist die englische Seite.*

# Datenträger und Speicher

Diese Seite beschreibt, wohin Rookparcel Downloads legt und wie es sie verschiebt, und wie es mit Ihren Festplatten umgeht: intelligente Festplatten-E/A (schonende Prüfungen, Abstimmung je Laufwerkstyp, Abbremsen, solange eine Festplatte ausgelastet ist) und der Lesecache der Festplatte.

## Wohin Downloads gehen

Unter **Optionen > Einstellungen > Ordner**, unter **Ort der heruntergeladenen Dateien**:

| Einstellung | Bedeutung |
|---|---|
| **Neue Downloads speichern in** | Der Ordner, in dem neue Torrents gespeichert werden, sofern Sie im Fenster „Neuen Torrent hinzufügen“ keinen anderen wählen. |
| **Abgeschlossene Downloads verschieben nach** | Wenn ein Torrent fertig heruntergeladen ist, werden seine Dateien in den darunter stehenden Ordner verschoben, und er seedet von dort weiter. |
| **Label des Torrents anhängen** | Neben dem Kästchen darüber: Ein fertiger Torrent mit einem Label wird in einen nach dem Label benannten Ordner verschoben, innerhalb des Ordners für abgeschlossene Downloads. |
| **Nur aus dem Standard-Downloadordner verschieben** | Verschiebt nur Torrents, die im Ordner für neue Downloads gespeichert sind; Torrents, die Sie anderswo gespeichert haben, bleiben, wo sie sind. Standardmäßig an. |

Der Einrichtungsassistent fragt beim ersten Start nach dem Download-Ordner: siehe [Erster Start](Erster-Start).

Dieselbe Seite verwaltet auch Kopien von .torrent-Dateien und lädt sie aus einem überwachten Ordner (**Ort der .torrent-Dateien**): siehe [Torrents hinzufügen](Torrents-hinzufügen).

### Abgeschlossene Downloads verschieben

1. Haken Sie **Abgeschlossene Downloads verschieben nach** an und wählen Sie den Ordner.
2. Haken Sie **Label des Torrents anhängen** an, wenn Sie nach Labels sortieren, zum Beispiel `D:\Done\Movies` für Torrents mit dem Label „Movies“. Ein Torrent ohne Label kommt direkt in den Ordner.
3. Entscheiden Sie über **Nur aus dem Standard-Downloadordner verschieben** (oben).
4. Klicken Sie auf **OK**.

Es gilt für Torrents, die ab jetzt fertig werden; früher fertig gewordene Torrents bleiben, wo sie sind. Solange die Dateien eines Torrents verschoben werden, zeigt sein Status **Wird verschoben**, und das **Protokoll** meldet „moving the completed download to ...“. Es zählt das Label, das ein Torrent beim Fertigwerden hat, also legen Sie Labels fest, bevor der Download abgeschlossen ist.

### Einen Torrent selbst verschieben

Klicken Sie mit der rechten Maustaste auf Torrents und wählen Sie **Erweitert > Speicherort festlegen...**, um einen Ordner zu wählen und ihre Dateien dorthin zu verschieben. Gleichnamige Dateien, die schon in diesem Ordner liegen, werden ersetzt. Der Eintrag ist grau, solange der Torrent bereits verschoben wird. Eine einzelne Datei eines Torrents lässt sich mit **Verschieben...** im Reiter „Dateien“ verschieben: siehe [Torrent-Details und Dateien](Torrent-Details-und-Dateien).

### Schließen, während Dateien verschoben werden

Wenn Sie beenden, während Dateien verschoben werden, wartet das Schließen-Fenster mit Fortschrittsanzeige auf die Verschiebungen, denn ein Abbruch mittendrin könnte einen Torrent halb in dem einen und halb in dem anderen Ordner zurücklassen. **Jetzt beenden** fragt zuerst nach und sagt, was es rückgängig machen würde: Die Dateien des Torrents bleiben im alten Ordner, was in den neuen kopiert wurde, wird gelöscht, und Sie können ihn später erneut verschieben.

### Unvollständige Dateien und .!ut

*Neu in 1.2.63.* Ist **Optionen > Einstellungen > Allgemein > An unvollständige Dateien .!ut anhängen** angehakt (unter **Beim Herunterladen**, standardmäßig aus), wird eine Datei, die noch heruntergeladen wird, mit `.!ut` nach ihrem Namen gespeichert, zum Beispiel `film.mkv.!ut`. Sobald die ganze Datei heruntergeladen ist und ihre Prüfung bestanden hat, erhält sie ihren normalen Namen, während der Rest des Torrents weiterläuft.

- **Warum:** Mediaplayer, Dateiüberwachungen und [Sonarr und Radarr](Sonarr-Radarr-und-Prowlarr) sehen eine Datei erst unter ihrem richtigen Namen, wenn sie vollständig ist, sodass nichts eine halbe Datei öffnet oder importiert. So benennt µTorrent unfertige Dateien.
- **Ein- oder Ausschalten** benennt die Dateien laufender Torrents sofort um und die gestoppter Torrents, wenn sie das nächste Mal starten.
- **Was es nie bekommt:** leere Dateien, die Füll-Dateien (Padding), die manche Torrents enthalten, eine Datei, deren Name mit dem Suffix zu lang würde, und eine Datei, deren Name mit dem Suffix eine andere Datei oder ein anderer Ordner desselben Torrents ist (ein Torrent, der sowohl `x` als auch `x.!ut` enthält): Sie behält ihren einfachen Namen. Eine Datei, die Sie im Reiter „Dateien“ umbenannt haben, erhält `.!ut` nach ihrem neuen Namen.
- **Reiter „Dateien“:** **Öffnen** und **Ordner öffnen** finden die Datei unter ihrem `.!ut`-Namen. Die Web-UI und ihre API zeigen die einfachen Namen.
- **Eine Datei, die ein anderes Programm geöffnet hat** (unter Windows zum Beispiel ein Player, der sie nicht freigibt), behält ihren jetzigen Namen, bis sie umbenannt werden kann. Das Protokoll sagt das einmal, und Rookparcel versucht es etwa alle 30 Sekunden erneut sowie beim nächsten Start. Der Torrent arbeitet währenddessen weiter.
- **Eine vorhandene Datei wird nie überschrieben.** Liegen sowohl `film.mkv` als auch `film.mkv.!ut` im Ordner, bleibt die fertige Datei als `film.mkv.!ut`. Das Protokoll sagt das einmal. Entfernen Sie die andere Datei, halten Sie den Torrent dann an und setzen Sie ihn fort, damit sie umbenannt wird.
- **Verschieben...** im Reiter „Dateien“ übernimmt für eine noch nicht heruntergeladene Datei die Datei, auf die gezeigt wird, unter dem Namen, den sie dort hat: `film.mkv`, `film.mkv.!ut` oder `film.mkv.!qB`. Führen Sie **Erneut prüfen** aus, damit ihre Teile gezählt werden.
- **Speicherort festlegen** ersetzt eine gleichnamige Datei im neuen Ordner, auch eine dortige `film.mkv`, während die verschobene Datei noch `film.mkv.!ut` heißt. Eine noch nicht heruntergeladene Datei wird aus dem neuen Ordner unter demjenigen dieser Namen übernommen, den sie dort hat.

**Wenn Sie von µTorrent oder qBittorrent kommen.** Ob das Kästchen angehakt ist oder nicht: Eine Datei, die unter dem erwarteten Namen fehlt, wird mit `.!ut` (dem Suffix von µTorrent) und mit `.!qB` (qBittorrents „Append .!qB extension to incomplete files“) gesucht. Ein [importierter](Import-aus-uTorrent-und-qBittorrent) oder erneut hinzugefügter Torrent, bei dem **Speichern in** auf seinen alten Ordner gesetzt ist, macht deshalb mit seinen Teildateien weiter, statt von vorn anzufangen. Sie werden dann in den Namen umbenannt, den Rookparcel ihnen gibt: `.!ut`, solange sie unfertig sind und das Kästchen angehakt ist, sonst der einfache Name. qBittorrents separater Ordner für unvollständige Dateien und seine `.unwanted`-Ordner werden nicht durchsucht.

**Entfernen und > Daten löschen** löscht eine `.!ut`- oder `.!qB`-Datei nur, wenn sie Daten dieses Torrents enthält: Rookparcel hat in sie geschrieben, oder Teile darin haben ihre Prüfung bestanden. Eine Teildatei dieses Namens, die der Torrent nie berührt hat (die eines anderen Programms oder eine aufgenommene, aber noch nicht geprüfte), bleibt erhalten, und das Protokoll nennt sie.

## Wenn ein Laufwerk oder eine Freigabe fehlt

*Neu in 1.2.76.* Findet ein Torrent, der bereits heruntergeladene Daten hatte, beim Start keine seiner Dateien, oder ist sein Speicherordner weg (ein USB-Laufwerk abgezogen, eine Netzwerkfreigabe oder ein Mount nicht da, ein Laufwerksbuchstabe noch nicht bereit), lädt Rookparcel die Daten nicht erneut herunter. Der Torrent stoppt mit einem Fehler, der mit dem fehlenden Pfad beginnt und **files missing (the drive, share or folder is not available)** sagt, und behält die Teile, die er hatte. Alle 30 Sekunden schaut Rookparcel erneut nach, und sobald der Ordner wieder da ist, startet der Torrent von selbst und macht weiter, ohne die Dateien erneut zu prüfen. Dasselbe geschieht, wenn das Laufwerk verschwindet, während ein Torrent herunterlädt. Rookparcel in der Zwischenzeit zu schließen ändert nichts: Der nächste Start findet den Torrent wieder fehlend oder wieder da. **Starten** bei einem angehaltenen Torrent versucht es sofort, und er wird wieder gehalten, wenn die Dateien noch fehlen. Ein gehaltener Torrent hat einen Fehler, sodass [Automatisches Herunterfahren](Automatisches-Herunterfahren) nicht auf ihn wartet.

Fehlen nur einige Dateien eines Torrents (von Hand oder von einem Virenscanner gelöscht), werden nur deren Teile erneut heruntergeladen. Löschen Sie den ganzen Ordner eines Torrents von Hand, wird der Torrent ebenfalls gehalten. Um die alten Daten eines gehaltenen Torrents aufzugeben und sie neu herunterzuladen oder um zu übernehmen, was jetzt auf der Festplatte liegt, verwenden Sie **Erneut prüfen**: Es prüft die vorhandenen Dateien und lädt den Rest herunter.

## Intelligente Festplatten-E/A

Die intelligente Festplatten-E/A hält den Computer reaktionsschnell, während Rookparcel liest und schreibt. Ihre Einstellungen finden Sie unter **Optionen > Einstellungen > Erweitert > Festplatten-E/A** (eine Seite unter „Erweitert“ in der Seitenliste).

Das Kästchen **Intelligente Festplatten-E/A verwenden** im Einrichtungsassistenten schaltet alle vier Teile gemeinsam ein oder aus. Schonende Prüfungen und die Abstimmung je Laufwerk sind von Anfang an an; das Kästchen fügt das Abbremsen hinzu, solange eine Festplatte ausgelastet ist. *Der Einrichtungsassistent ist neu in 1.1.62. Die Seite „Festplatten-E/A“ selbst gibt es seit 1.1.40.*

### Prüfung

Eine Prüfung (zum Beispiel nach **Erneut prüfen**) liest jedes Teil von der Festplatte und vergleicht es mit dem Torrent. Unter **Prüfung**:

| Einstellung | Bedeutung | Standard |
|---|---|---|
| **Torrents schonend prüfen (niedrige Priorität, anderen Programmen den Vortritt lassen)** | Prüft mit der Hintergrund-Festplattenpriorität des Systems, lässt die zwischengespeicherten Dateien anderer Programme in Ruhe und pausiert, solange Lesezugriffe langsamer als gewöhnlich sind, damit der Computer reaktionsschnell bleibt. Prüfungen dauern länger, solange die Festplatte ausgelastet ist. | An |
| **Prüfung begrenzen auf (MB/s):** | Begrenzt die Lesegeschwindigkeit jeder Prüfung, für eine Festplatte, die selbst bei schonenden Prüfungen noch hinterherhinkt. Hier sind 1 MB 1.000.000 Bytes. | Aus, 100 |
| **Gleichzeitig geprüfte Laufwerke (ein Torrent je Laufwerk):** | Wie viele Laufwerke gleichzeitig geprüft werden, ein Torrent je Laufwerk; Torrents auf demselben Laufwerk warten, bis sie an der Reihe sind. Von 1 bis 8. | 4 |

Prüfungen von Torrents auf verschiedenen Laufwerken laufen nebeneinander; zwei Torrents auf einem Laufwerk werden nacheinander geprüft, damit das Laufwerk nicht zwischen ihnen hin- und herspringt. Der Status zeigt **Zur Prüfung eingereiht** für einen Torrent, der wartet, und dann **Prüfe** mit dem Prozentwert. *Der Status „eingereiht“ ist neu in 1.2.76; zuvor zeigte eine wartende Prüfung „Checking 0.0 %“.* Eine Prüfung, die Sie mit **Erneut prüfen** anfordern, wartet nicht auf einen freien Platz in der Torrent-Warteschlange ([Warteschlange und Seeding](Warteschlange-und-Seeding)).

*Neu in 1.2.76.* Eine Datei, die existiert, aber nicht gelesen werden kann (ein anderes Programm hält sie, oder Sie haben keine Berechtigung), stoppt die Prüfung mit einem Dateifehler, statt ihre Teile als fehlend zu zählen und erneut herunterzuladen. Eine Datei, die gar nicht da ist, verliert weiterhin nur ihre eigenen Teile.

### Laufwerke

| Einstellung | Bedeutung | Standard |
|---|---|---|
| **An den Laufwerkstyp anpassen (SSD, Festplatte, Netzwerk)** | Erkennt, ob sich jeder Speicherordner auf einer SSD, einer Festplatte, einem Wechsellaufwerk oder einer Netzwerkfreigabe befindet. Eine Festplatte oder ein Wechsellaufwerk bekommt dann höchstens zwei Festplattenaufträge gleichzeitig, ein Netzwerkserver vier, damit sie nicht überlastet werden. | An |

Das Protokoll listet die beim Start gefundenen Laufwerke auf, etwa „Disk I/O: drive D: (HDD, SATA)“. USB-Adapter, RAID-Controller und virtuelle Maschinen melden oft den falschen Typ.

**Eine falsche Vermutung korrigieren.** Verwenden Sie `drive_overrides` unter **Optionen > Einstellungen > Erweitert**: durch Kommas getrennte `Pfad=Typ`-Paare, wobei der Typ einer von `ssd`, `hdd`, `network` oder `removable` ist, zum Beispiel `D:/=hdd, E:/=removable`. Der am längsten passende Pfad gewinnt.

1. Öffnen Sie **Erweitert** und geben Sie `drive_overrides` bei **Filter** ein.
2. Wählen Sie die Zeile aus, geben Sie die Paare bei **Wert** ein und klicken Sie auf **Setzen**.
3. Klicken Sie auf **OK** oder **Übernehmen**.

![Einstellungen, Seite „Erweitert“](images/prefs-advanced.png)

### Wenn die Festplatte ausgelastet ist

Unter **Wenn die Festplatte ausgelastet ist**, beide standardmäßig aus:

| Einstellung | Bedeutung |
|---|---|
| **Downloads verlangsamen (Schreibzugriffe)** | Solange ein Laufwerk Schreibzugriffe deutlich langsamer als gewöhnlich beantwortet, wird die Download-Rate schrittweise halbiert, nie unter 50 kB/s, und darf wieder wachsen, sobald die Festplatte ruhig ist. |
| **Uploads verlangsamen (Lesezugriffe)** | Dasselbe für Uploads, solange ein Laufwerk Lesezugriffe langsam beantwortet. Das Seeding lädt dann weniger hoch, was bei Trackern wichtig ist, die Ihr Verhältnis zählen. |

Sie heben Ihre eigenen Limits nie an. Das Abbremsen ist eine Obergrenze je Richtung, die mit [Niedrigen Ping halten](Niedrigen-Ping-halten) geteilt wird: Die niedrigere Obergrenze gilt. Solange sie eine Richtung zurückhält, zeigt die Statusleiste sie als „(auto ...)“ an, und das Protokoll hat eine Zeile dazu.

## Festplatten-Cache und Dateien

Unter **Optionen > Einstellungen > Erweitert > Festplatten-Cache** finden Sie den Lesecache und die Festplatten-Warteschlange. Die meisten brauchen sie nie zu ändern.

Unter **Lesecache**:

| Einstellung | Bedeutung | Standard |
|---|---|---|
| **Lesezugriffe auf die Festplatte zwischenspeichern** | Hält zuletzt gelesene Teile im Speicher und liest einen Block, den Peers anfordern, zusammen mit seinen Nachbarn, was beim Seeding Festplattenzugriffe spart. | An |
| **Automatische Cachegröße überschreiben (MB):** | Verwendet die daneben stehende Größe statt der automatischen 32 MB. Ein größerer Cache kann bei viel Seeding helfen, auf Kosten von Speicher. | Aus |
| **Blöcke entfernen, die länger ungenutzt sind als (Sekunden):** | Verwirft zwischengespeicherte Blöcke, die in dieser Zeit kein Peer angefordert hat. Aus bleiben sie, bis neuere Blöcke sie verdrängen. | An, 300 |

Schreibzugriffe überlässt Rookparcel dem eigenen Cache des Systems.

Unter **Festplatten-Warteschlange und Dateien**:

| Einstellung | Bedeutung | Standard |
|---|---|---|
| **Schreibwarteschlange (kB):** | Wie viele heruntergeladene Daten auf das Schreiben auf die Festplatte warten dürfen; darüber hört Rookparcel auf, von Peers zu lesen, bis die Festplatte aufgeholt hat. Werte unter 4 MB gelten als 4 MB. | 1024 |
| **Höchstzahl offener Dateien:** | Die höchste Zahl gleichzeitig offen gehaltener Dateien; andere werden geschlossen und bei Bedarf wieder geöffnet. Erhöhen Sie sie, wenn viele Torrents oder Dateien gleichzeitig beschäftigt sind. | 40 |
| **Festplatten-E/A-Threads (ab dem nächsten Start):** | Die Threads, die Torrent-Daten lesen und schreiben. | 10 |
| **Hash-Threads (nicht verwendet):** | Aus Kompatibilitätsgründen beibehalten; diese Version nutzt es nicht. Prüfungen laufen eine je Laufwerk, eingestellt auf der Seite „Festplatten-E/A“. | |

Rookparcel hat keine Einstellung, um Dateien vorab zu reservieren. Eine Datei wird angelegt, wenn die ersten Daten für sie eintreffen, und unter Windows wird sie als Sparse-Datei (dünn besetzt) markiert, sodass ein Schreiben nahe ihrem Ende NTFS nicht dazu bringt, den Teil davor mit Nullen zu füllen.

## Im GTK-Fenster und in der Web-UI

- **GTK-Fenster** (Linux, BSD), **macOS** und die **Web-UI**: **An unvollständige Dateien .!ut anhängen** steht auch auf der Seite „Allgemein“ unter **Beim Herunterladen**.
- **macOS-App**: dieselben Seiten und Einstellungen; Einstellungen gelten sofort, ohne **OK** ([Einstellungen im Überblick](Einstellungen-im-Überblick#unterschiede-unter-macos)).
- **GTK-Fenster** (Linux, BSD): dieselben Seiten „Ordner“, „Festplatten-Cache“ und „Festplatten-E/A“. Laufwerkstypen werden auch unter Linux, macOS und FreeBSD erkannt, jeweils mit den Methoden des Systems.
- **Web-UI**: dieselben Seiten unter **Einstellungen**, mit **Festplatten-Cache** und **Festplatten-E/A** unter **Erweitert**. Ordner sind dort Ordner auf dem Computer, auf dem Rookparcel läuft, nicht die des Browsers.
- **rookparcel-nox**: `--save DIR` legt den Download-Ordner fest und `--move-on-complete DIR` verschiebt fertige Torrents; die übrigen Einstellungen finden Sie in den Einstellungen der Web-UI.

## Verwandte Seiten

- [Warteschlange und Seeding](Warteschlange-und-Seeding)
- [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt) (Einstellungen und Torrent-Zustand, nicht Downloads)
- [Torrents verwalten](Torrents-verwalten)
- [Einstellungen im Überblick](Einstellungen-im-Überblick)
