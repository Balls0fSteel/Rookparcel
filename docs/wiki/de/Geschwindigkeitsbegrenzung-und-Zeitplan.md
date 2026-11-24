*Dies ist eine Übersetzung der englischen Seite [Speed limits and scheduler](https://github.com/Balls0fSteel/Rookparcel/wiki/Speed-limits-and-scheduler). Maßgeblich ist die englische Seite.*

# Geschwindigkeitsbegrenzung und Zeitplaner

Diese Seite behandelt, wie Sie die Download- und Upload-Geschwindigkeit von Rookparcel begrenzen: für alle Torrents zusammen, für einzelne Torrents und stundenweise mit dem wöchentlichen Zeitplaner. Für Begrenzungen, die dem Ping Ihrer Leitung von selbst folgen, siehe [Niedrigen Ping halten](Niedrigen-Ping-halten).

Alle Raten sind in kB/s angegeben, wobei 1 kB 1024 Byte sind. In jedem Ratenfeld bedeutet 0 unbegrenzt.

## Globale Begrenzungen

Die globalen Begrenzungen deckeln alle Torrents zusammen.

1. Öffnen Sie **Optionen > Einstellungen** (<kbd>Ctrl</kbd>+<kbd>P</kbd>) und wählen Sie **Bandbreite**.
2. Stellen Sie unter **Globale Begrenzung der Upload-Rate** **Maximale Upload-Rate (kB/s): [0: unbegrenzt]** ein.
3. Stellen Sie unter **Globale Begrenzung der Download-Rate** **Maximale Download-Rate (kB/s): [0: unbegrenzt]** ein.
4. Klicken Sie auf **OK** oder **Übernehmen**. Die neuen Begrenzungen gelten sofort.

![Einstellungen, Seite „Bandbreite“](images/prefs-bandwidth.png)

Beide sind standardmäßig unbegrenzt. Ein begrenzter Upload ist der übliche Grund, eine zu setzen: Wenn Rookparcel so schnell hochlädt, wie die Leitung erlaubt, werden Webseiten, Spiele und Anrufe für alle im Haus langsamer.

### Optionen für die globale Ratenbegrenzung

Dieselbe Seite hat zwei Kästchen, die ändern, was die Begrenzungen zählen:

| Option | Was sie tut | Standard |
|---|---|---|
| **Ratenbegrenzung auf Transport-Overhead anwenden** | Rechnet die TCP/IP-Header des Peer-Verkehrs, pro Paket geschätzt, auf die globalen Begrenzungen an, sodass die Begrenzungen dem entsprechen, was die Leitung trägt. | Aus |
| **Ratenbegrenzung auf uTP-Verbindungen anwenden** | Wendet die globalen Begrenzungen auch auf uTP-Peers an. Aus werden uTP-Peers nur von der eigenen Verzögerungssteuerung von uTP gebremst, die anderem Verkehr den Vortritt lässt. | An |

### Verbindungen und Upload-Slots

Die Gruppe **Anzahl der Verbindungen** auf derselben Seite ist keine Geschwindigkeitsbegrenzung, beeinflusst aber die Geschwindigkeit:

| Einstellung | Bedeutung | Standard |
|---|---|---|
| **Globale Höchstzahl an Verbindungen:** | Die meisten Peer-Verbindungen, die über alle Torrents hinweg gleichzeitig offen sind. Senken Sie sie, wenn der Router Mühe hat. | 200 |
| **Höchstzahl verbundener Peers pro Torrent: [0: keine Grenze]** | Die meisten Peers, mit denen sich ein Torrent verbindet. | 0 |
| **Anzahl der Upload-Slots pro Torrent: [0: keine Grenze]** | An wie viele Peers ein Torrent gleichzeitig hochlädt. | 0 |
| **Globale Anzahl der Upload-Slots: [-1: keine Grenze]** | An wie viele Peers alle Torrents zusammen gleichzeitig hochladen, wobei die schnellsten gewählt werden. Weniger Slots geben jedem Peer mehr Geschwindigkeit. | 8 |

Die beiden Werte pro Torrent gelten für Torrents, die ab jetzt hinzugefügt werden; die Eigenschaften eines Torrents können seine eigenen Upload-Slots ändern. Die Router-Frage des Einrichtungsassistenten kann die Verbindungszahlen für einen älteren Router senken: siehe [Erster Start](Erster-Start).

## Begrenzungen für einzelne Torrents

Ein Torrent kann eigene Begrenzungen unterhalb der globalen haben. Die globalen Begrenzungen gelten zusätzlich.

**Über das Kontextmenü:**

1. Wählen Sie einen oder mehrere Torrents in der Liste aus.
2. Klicken Sie mit der rechten Maustaste und wählen Sie **Bandbreite > Download-Limit festlegen** oder **Bandbreite > Upload-Limit festlegen**.
3. Wählen Sie eine Rate: **Unbegrenzt** oder eine von 10, 25, 50, 100, 250, 500, 1000, 2500 und 5000 kB/s.

Die aktuelle Rate des ersten ausgewählten Torrents ist abgehakt.

![Das Kontextmenü der Torrent-Liste](images/torrent-menu.png)

**Über die Torrent-Eigenschaften**, für jede beliebige Rate:

1. Wählen Sie die Torrents aus und wählen Sie **Eigenschaften** (<kbd>Enter</kbd>).
2. Stellen Sie unter **Bandbreiteneinstellungen** **Maximale Upload-Rate (kB/s): [0: unbegrenzt]** und **Maximale Download-Rate (kB/s): [0: unbegrenzt]** ein.
3. Um dem Torrent eine eigene Anzahl von Upload-Slots zu geben, haken Sie **Standardanzahl der Upload-Slots überschreiben** an und füllen **Anzahl der Upload-Slots: [0: unbegrenzt]** aus.
4. Klicken Sie auf **OK**.

![Das Fenster „Torrent-Eigenschaften“](images/properties.png)

Die eigenen Begrenzungen eines Torrents erscheinen in den Spalten **Download-Limit** und **Upload-Limit** der Torrent-Liste und im Reiter **Allgemein**. Den Rest des Fensters „Eigenschaften“ finden Sie unter [Torrent-Details und Dateien](Torrent-Details-und-Dateien).

Ein Torrent, der sein Seeding-Ziel erreicht, kann auch von selbst ein Upload-Limit bekommen: siehe [Warteschlange und Seeding](Warteschlange-und-Seeding#seeding-ziel).

## Die Statusleiste

Die Statusleiste unten im Hauptfenster zeigt die aktuellen Raten und die Summen dieser Sitzung, als **D: 1.2 MB/s G: 3.4 GB** für den Download und **U: ... G: ...** für den Upload. Solange Ping niedrig halten oder die Verlangsamung bei ausgelastetem Datenträger eine Richtung zurückhält, zeigt ihr Teil auch den automatischen Deckel, als **U: 412.0 kB/s (auto 450.0 kB/s) G: ...**.

Die Statusleiste hat kein Geschwindigkeitsmenü. Setzen Sie die globalen Begrenzungen in den Einstellungen oder die eigenen Begrenzungen der Torrents über deren Menü.

Ist **Aktuelle Geschwindigkeit in der Titelleiste anzeigen** eingeschaltet (**Optionen > Einstellungen > Oberfläche**), beginnen auch der Fenstertitel und die Schaltfläche in der Taskleiste mit den Geschwindigkeiten.

## Der Zeitplaner

Der Zeitplaner folgt einer Wochentabelle, Stunde für Stunde, und schaltet Rookparcel zwischen vier Modi um. Er ist standardmäßig aus.

1. Öffnen Sie **Optionen > Einstellungen > Zeitplaner**.
2. Haken Sie **Zeitplaner aktivieren** an. Die Tabelle ist grau, bis Sie das tun.
3. Klicken Sie auf eine Zelle in der **Zeitplantabelle**, um diese Stunde zum nächsten Modus weiterzuschalten. Halten Sie die Taste gedrückt und ziehen Sie, um die Zellen, über die Sie fahren, mit demselben Modus zu füllen.
4. Stellen Sie unter **Zeitplaner-Einstellungen** die Raten für die **Begrenzt**-Stunden ein.
5. Klicken Sie auf **OK** oder **Übernehmen**.

![Einstellungen, Seite „Zeitplaner“](images/prefs-scheduler.png)

Die Tabelle hat eine Zeile pro Tag, Montag zuerst, und eine Spalte pro Stunde, in der Ortszeit Ihres Computers. Ein Klick schaltet durch **Volle Geschwindigkeit**, **Begrenzt**, **Aus** und **Nur Seeding**. Die Zeile unter der Tabelle nennt die Stunde unter dem Zeiger und ihren Modus, etwa „Mo 09:00 - 10:00: Volle Geschwindigkeit“.

| Modus | Farbe | Was Rookparcel in dieser Stunde tut |
|---|---|---|
| **Volle Geschwindigkeit** | dunkelgrün | Läuft mit Ihren normalen Einstellungen. |
| **Begrenzt** | hellgrün | Verwendet die beiden Begrenzt-Raten unten anstelle der globalen Begrenzungen. |
| **Aus** | weiß | Hält jeden Torrent an. Mit **DHT beim Ausschalten deaktivieren** wird auch das DHT ausgeschaltet, sodass Rookparcel im Netzwerk still bleibt. |
| **Nur Seeding** | hellrot | Lädt hoch, lädt aber nichts herunter. Torrents, die in diesen Stunden hinzugefügt werden, warten ebenfalls. |

| Zeitplaner-Einstellung | Bedeutung | Standard |
|---|---|---|
| **Begrenzte Upload-Rate (kB/s): [0: unbegrenzt]** | Das globale Upload-Limit während der Begrenzt-Stunden. | 0 |
| **Begrenzte Download-Rate (kB/s): [0: unbegrenzt]** | Das globale Download-Limit während der Begrenzt-Stunden. | 0 |
| **DHT beim Ausschalten deaktivieren** | Schaltet das DHT während der Aus-Stunden ebenfalls aus. | An |

Was Sie erwarten können:

- Wechselt die Stunde den Modus, ergänzt der Netzwerkteil der Statusleiste **Zeitplaner: begrenzt**, **Zeitplaner: ausgeschaltet** oder **Zeitplaner: nur Seeding**, und der Reiter **Protokoll** schreibt eine Zeile wie „Zeitplaner: Begrenzt“. Volle Geschwindigkeit zeigt nichts.
- „Aus“ ändert den eigenen Zustand der Torrents nicht. Kehrt die volle Geschwindigkeit zurück, machen sie wie zuvor weiter, und ein Torrent, den Sie gestoppt haben, bleibt gestoppt.
- Nehmen Sie das Häkchen bei **Zeitplaner aktivieren** weg, um sofort mit Ihren normalen Einstellungen zur vollen Geschwindigkeit zurückzukehren.
- Ping niedrig halten und die Verlangsamung bei ausgelastetem Datenträger können die Geschwindigkeiten in jeder Stunde weiter senken; sie erhöhen sie nie.

## Alternative Geschwindigkeitsbegrenzungen (Web-UI von rookparcel-nox)

Die Fenster für Windows und GTK haben keine alternativen Geschwindigkeitsbegrenzungen; verwenden Sie stattdessen die Begrenzt-Stunden des Zeitplaners.

Die Web-UI von [rookparcel-nox](rookparcel-nox-betreiben) hat sie, unter **Einstellungen > Bandbreite > Alternative Ratenbegrenzungen**:

| Einstellung | Bedeutung | Standard |
|---|---|---|
| **Alternative Upload-Rate (kB/s): [0: unbegrenzt]** | Das globale Upload-Limit, solange die alternativen Begrenzungen an sind. | 10 |
| **Alternative Download-Rate (kB/s): [0: unbegrenzt]** | Das globale Download-Limit, solange sie an sind. | 10 |
| **Die alternativen Ratenbegrenzungen jetzt verwenden** | Schaltet auf die alternativen Begrenzungen um, bis Sie zurückschalten. | Aus |

Apps, die den Modus für die Geschwindigkeitsbegrenzungen von qBittorrent über die Web-UI umschalten (siehe [Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr)), schalten dasselbe um. Die Begrenzt-Stunden des Zeitplaners gelten weiterhin vor beiden Sätzen, solange sie dauern.

Wenn ein Desktop-Fenster die Web-UI bereitstellt (Einstellungen > Web-UI im Fenster), wird diese Gruppe nicht angezeigt, und die alternativen Begrenzungen lassen sich nicht einschalten: Es gelten die eigenen Begrenzungen und der Zeitplaner des Fensters.

## Im GTK-Fenster und in der Web-UI

- **GTK-Fenster** (Linux, BSD): dieselben Einstellungsseiten, dasselbe Untermenü **Bandbreite** im Torrent-Menü, dieselbe Zeitplantabelle, auf dieselbe Weise bearbeitet.
- **macOS-App**: dieselben Seiten, dasselbe Untermenü **Bandbreite** und eine Zeitplantabelle, die Sie durch Ziehen füllen. Einstellungen gelten sofort, ohne **OK**: siehe [Einstellungen im Überblick](Einstellungen-im-Überblick#unterschiede-unter-macos).
- **Web-UI**: dieselben Seiten unter ihren **Einstellungen** und dasselbe Untermenü **Bandbreite**. Ihre Statusleiste zeigt eine globale Begrenzung in Klammern hinter der Rate, solange eine gesetzt ist, und den automatischen Deckel auf dieselbe Weise wie das Fenster.
- **rookparcel-nox** ohne Web-UI: `--dl-limit KIB` und `--ul-limit KIB` setzen die globalen Begrenzungen beim Start (0 = unbegrenzt). Der Zeitplaner, das Seeding-Ziel und die alternativen Begrenzungen werden in den **Einstellungen** der Web-UI gesetzt.

## Verwandte Seiten

- [Niedrigen Ping halten](Niedrigen-Ping-halten)
- [Warteschlange und Seeding](Warteschlange-und-Seeding)
- [Datenträger und Speicher](Datenträger-und-Speicher#wenn-die-festplatte-ausgelastet-ist)
- [Einstellungen im Überblick](Einstellungen-im-Überblick)
