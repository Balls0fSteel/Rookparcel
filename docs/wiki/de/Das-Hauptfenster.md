*Dies ist eine Übersetzung der englischen Seite [The main window](The-main-window). Maßgeblich ist die englische Seite.*

# Das Hauptfenster

Diese Seite führt durch das Hauptfenster von Rookparcel: die Menüs, die Symbolleiste mit ihrem Suchfeld, die Kategorieliste, die Torrent-Liste mit ihren Spalten, die Detail-Reiter und die Statusleiste. Sie beschreibt zuerst die Windows-Version, dann, was in der GTK-Version und der Weboberfläche anders ist.

![Das Hauptfenster mit Torrents und dem Reiter Allgemein](images/main-window.png)

## Die Teile im Überblick

| Teil | Wo | Wofür er da ist |
|---|---|---|
| Menüleiste | oben | **Datei**, **Optionen** und **Hilfe** |
| Symbolleiste | unter den Menüs | Schaltflächen für die häufigen Aktionen und rechts das Suchfeld |
| Kategorieliste | links | zeigt die Torrents nach Status, Label oder Tag und Ihre RSS-Feeds |
| Torrent-Liste | Mitte | eine Zeile pro Torrent |
| Detail-Reiter | unter der Liste | die Details des ausgewählten Torrents: Allgemein, Tracker, Peers, Teile, Dateien, Geschwindigkeit, Protokoll |
| Statusleiste | unten | der Netzwerkstatus, DHT und die Gesamtgeschwindigkeiten |

Jeder Teil außer den Menüs und der Torrent-Liste kann im Menü **Optionen** ausgeblendet werden: **Symbolleiste anzeigen**, **Details anzeigen**, **Statusleiste anzeigen** und **Kategorieliste anzeigen**. Ziehen Sie die Ränder zwischen Kategorieliste, Torrent-Liste und Detail-Reitern, um die Größen zu ändern. Rookparcel merkt sich diese Auswahl, die Größen, die Spalten, die Sortierung und den geöffneten Reiter.

## Die Menüs

- **Datei:** **Torrent hinzufügen...**, **Torrent von URL hinzufügen...**, **Neuen Torrent erstellen...**, **Aus qBittorrent importieren...**, **Aus µTorrent importieren...**, **Profile**, **Mit Fernsitzung verbinden...**, **Rookparcel sperren** (<kbd>Ctrl</kbd>+<kbd>L</kbd>; siehe [Rookparcel sperren](Rookparcel-sperren)) und **Beenden**.
- **Optionen:** **Einstellungen**, **Einrichtungsassistent**, **RSS-Downloader...**, die vier oben genannten Einträge **Anzeigen**, **Automatisch herunterfahren**, **Beim Schließen in den Infobereich** und **Beim Minimieren in den Infobereich**.
- **Hilfe:** **Rookparcel-Website**, **Nach Updates suchen**, **Statistik**, **Spenden**, **Über Rookparcel** und **Hinweise zu Drittanbietern**. **Statistik** (neu in 1.2.39) zeigt, was Rookparcel in allen seinen Sitzungen und in dieser hoch- und heruntergeladen hat, das Verhältnis, die verworfenen Daten, die verbundenen Peers sowie die Datenträger-Warteschlange und den Lese-Cache; die Gesamtsummen bleiben erhalten, wenn Torrents entfernt werden. **Spenden** (neu in 1.2.34) zeigt einen kurzen Hinweis und eine Schaltfläche, die die Spendenseite der Website in Ihrem Browser öffnet.

*Datei > Profile ist neu in 1.1.48 und Optionen > Einrichtungsassistent in 1.1.62.*

Seiten: [Rookparcel sperren](Rookparcel-sperren), [Torrents hinzufügen](Torrents-hinzufügen), [Torrents erstellen](Torrents-erstellen), [Import aus µTorrent und qBittorrent](Import-aus-uTorrent-und-qBittorrent), [Profile](Profile), [Fernsteuerung](Fernsteuerung), [Einstellungen im Überblick](Einstellungen-im-Überblick), [Erster Start und der Einrichtungsassistent](Erster-Start), [Downloads per RSS](Downloads-per-RSS), [Automatisches Herunterfahren](Automatisches-Herunterfahren), [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart), [Rookparcel aktualisieren](Rookparcel-aktualisieren).

## Die Symbolleiste

Von links nach rechts:

| Schaltfläche | Funktion |
|---|---|
| **Torrent hinzufügen** | fügt `.torrent`-Dateien hinzu (<kbd>Ctrl</kbd>+<kbd>O</kbd>) |
| **Torrent von URL hinzufügen** | fügt einen Magnet-Link oder eine Webadresse hinzu (<kbd>Ctrl</kbd>+<kbd>U</kbd>) |
| **Neuen Torrent erstellen** | erstellt eine neue `.torrent` (<kbd>Ctrl</kbd>+<kbd>N</kbd>) |
| **Entfernen** | entfernt die ausgewählten Torrents aus der Liste (<kbd>Del</kbd>) |
| **Starten**, **Anhalten**, **Stoppen** | starten, pausieren oder stoppen die ausgewählten Torrents |
| **In der Warteschlange nach oben**, **In der Warteschlange nach unten** | verschieben die ausgewählten Torrents um einen Platz in der Warteschlange |
| **RSS-Downloader** | öffnet den RSS-Downloader |
| **Einstellungen** | öffnet die Einstellungen (<kbd>Ctrl</kbd>+<kbd>P</kbd>) |

Zeigen Sie auf eine Schaltfläche, um ihren Namen zu sehen. Ab 1.1.57 zeigt die Statusleiste außerdem eine Zeile dazu, was die Schaltfläche unter dem Mauszeiger oder der markierte Menüeintrag tut. Was Starten, Anhalten und Stoppen bedeuten, steht unter [Torrents verwalten](Torrents-verwalten#starten-anhalten-stoppen-und-start-erzwingen).

### Das Suchfeld

Das Feld am rechten Ende der Symbolleiste hat zwei Aufgaben, die Sie mit dem kleinen Pfeil daneben wählen:

- **Torrents filtern** (der übliche Modus): Während Sie tippen, zeigt die Liste nur Torrents, deren Name den Text enthält, ohne Rücksicht auf Groß- und Kleinschreibung. Der Filter wirkt zusätzlich zur Kategorie, die Sie links gewählt haben. <kbd>Esc</kbd> leert das Feld und kehrt zur Liste zurück. <kbd>Ctrl</kbd>+<kbd>F</kbd> setzt den Cursor in das Feld.
- **Eine Suchquelle**: <kbd>Enter</kbd> sucht im Web nach Torrents über Prowlarr, Jackett oder den Suchhelfer. Siehe [Torrents suchen](Torrents-suchen).

Die Wahl bleibt zwischen den Starts erhalten.

## Die Kategorieliste

Die Liste links schränkt die Torrent-Liste ein. Jeder Eintrag zeigt, wie viele Torrents er enthält.

| Eintrag | Zeigt |
|---|---|
| **Torrents** | jeden Torrent |
| **Herunterladen** | noch nicht fertige Torrents |
| **Abgeschlossen** | fertige Torrents |
| **Aktiv** | Torrents, die gerade Daten herunter- oder hochladen |
| **Inaktiv** | Torrents, die gerade keine Daten bewegen |
| **Labels** > **Ohne Label** | Torrents ohne Label |
| **Labels** > *ein Label* | Torrents mit diesem Label; ein Eintrag pro verwendetem Label |
| **Tags** > **Ohne Tag** | Torrents ohne jedes Tag |
| **Tags** > *ein Tag* | Torrents mit diesem Tag; ein Eintrag pro Tag, auch für ein Tag, das kein Torrent hat (0) |
| **Feeds** | Ihre RSS-Feeds; wählen Sie einen, zeigt die Liste seine Einträge |

Labels werden im Rechtsklickmenü der Torrent-Liste angelegt und vergeben. Siehe [Torrents verwalten](Torrents-verwalten#labels). Tags werden auf dieselbe Weise vergeben; klicken Sie den Zweig **Tags** mit der rechten Maustaste an für **Neues Tag...** oder ein Tag für **Tag löschen**. Siehe [Torrents verwalten](Torrents-verwalten#tags) (*Tags sind neu in 1.2.63*). Der Zweig Feeds hat sein eigenes Rechtsklickmenü; siehe [Downloads per RSS](Downloads-per-RSS).

## Die Torrent-Liste

Eine Zeile pro Torrent. Klicken Sie eine Zeile an, um sie auszuwählen; <kbd>Ctrl</kbd>+Klick und <kbd>Shift</kbd>+Klick wählen mehrere aus; <kbd>Ctrl</kbd>+<kbd>A</kbd> wählt alle aus. Klicken Sie mit der rechten Maustaste für das Torrent-Menü, das unter [Torrents verwalten](Torrents-verwalten#das-kontextmenü) beschrieben ist.

Doppelklicken Sie einen fertigen Torrent, um seinen Ordner im Explorer zu öffnen. Doppelklicken Sie einen Torrent, der nicht fertig ist, oder drücken Sie <kbd>Enter</kbd>, um seine [Eigenschaften](Torrent-Details-und-Dateien#torrent-eigenschaften) zu öffnen.

### Spalten

Klicken Sie die Spaltenköpfe mit der rechten Maustaste an, um Spalten ein- oder auszublenden. **Name** wird immer angezeigt. Ziehen Sie einen Kopf, um die Spalte zu verschieben; ziehen Sie seinen Rand, um die Breite zu ändern.

| Spalte | Zeigt | Anfangs angezeigt |
|---|---|---|
| **Name** | den Namen des Torrents | ja |
| **#** | seinen Platz in der Warteschlange; `*`, wenn er keinen hat | ja |
| **Größe** | die Größe der Dateien, die Sie herunterladen wollen | ja |
| **Fertig** | wie viel heruntergeladen ist, in Prozent | ja |
| **Status** | was der Torrent gerade tut (siehe unten) | ja |
| **Seeds** | verbundene Seeds und in Klammern die Seeds im ganzen Schwarm, wenn der Tracker sie meldet | ja |
| **Peers** | verbundene Peers und in Klammern die Peers im Schwarm | ja |
| **Download**, **Upload** | aktuelle Geschwindigkeiten | ja |
| **Restzeit** | geschätzte verbleibende Zeit | ja |
| **Hochgeladen** | insgesamt hochgeladen | ja |
| **Heruntergeladen** | insgesamt heruntergeladen | nein |
| **Verhältnis** | hochgeladen geteilt durch heruntergeladen | ja |
| **Verfügb.** | wie viele vollständige Kopien die verbundenen Peers zusammen haben | ja |
| **Label** | sein Label | ja |
| **Hinzugefügt am**, **Abgeschlossen am** | Daten | nein |
| **Tracker** | den Hostnamen des verwendeten Trackers | nein |
| **Download-Limit**, **Upload-Limit** | seine eigenen Geschwindigkeitsgrenzen | nein |
| **Verbleibend** | was noch herunterzuladen ist | nein |
| **Seeds/Peers** | Seeds pro Peer im Schwarm | nein |
| **Speichern unter** | der Ordner, in dem seine Dateien liegen | nein |
| **Verworfen** | weggeworfene Daten (fehlgeschlagene Prüfungen, Duplikate) | nein |
| **Tags** | seine Tags, durch Kommas getrennt (*neu in 1.2.63*) | nein |

### Sortieren

Klicken Sie auf einen Spaltenkopf, um danach zu sortieren. Der erste Klick sortiert so, wie es meist am nützlichsten ist: Geschwindigkeiten, Seeds, Peers, Verhältnis und Fertig gehen von hoch nach niedrig, die anderen von niedrig nach hoch. Klicken Sie erneut, um umzukehren. Namen werden so sortiert wie im Explorer, „Episode 2“ steht also vor „Episode 10“.

### Was die Spalte Status sagt

| Status | Bedeutung |
|---|---|
| **Lädt herunter**, **Seeding** | läuft |
| **Eingereiht**, **Seed eingereiht** | gestartet, wartet aber auf einen freien Platz in der Warteschlange |
| **[F] Lädt herunter**, **[F] Seeding** | mit **Start erzwingen** gestartet, außerhalb der Warteschlange |
| **[S] Seeding** | Seeding mit eingeschaltetem **Initial-Seeding** |
| **Angehalten** | von Ihnen angehalten |
| **Gestoppt** | von Ihnen gestoppt, bevor er fertig war |
| **Fertig** | gestoppt, nachdem er fertig war |
| **Lade Metadaten** | ein Magnet-Link, der die Dateiliste des Torrents holt |
| **Prüfe 45.3 %**, **Zur Prüfung eingereiht** | prüft die Dateien auf dem Datenträger. **Zur Prüfung eingereiht** bedeutet, dass die Prüfung auf ihr Laufwerk wartet, das einen Torrent nach dem anderen prüft. *Zur Prüfung eingereiht ist neu in 1.2.76.* |
| **Wird verschoben** | verschiebt seine Dateien in einen anderen Ordner |
| **Warten auf das Netzwerk** | **Nur dieses Netzwerk verwenden** findet keinen passenden Netzwerkadapter (das VPN ist aus); es läuft von selbst weiter, wenn er zurück ist. *Neu in 1.1.58.* |
| **Fehler: ...** | etwas ist schiefgegangen; die Meldung sagt, was. Fehlt ein Laufwerk, eine Freigabe oder ein Ordner mit den Dateien des Torrents, sagt die Meldung „files missing“, und der Torrent wartet, bis die Dateien wieder da sind (*neu in 1.2.76*; siehe [Datenträger und Speicher](Datenträger-und-Speicher#wenn-ein-laufwerk-oder-eine-freigabe-fehlt)). |

Die Aktionen hinter diesen Zuständen stehen unter [Torrents verwalten](Torrents-verwalten).

## Die Detail-Reiter

Die Reiter unter der Liste zeigen den ausgewählten Torrent: **Allgemein**, **Tracker**, **Peers**, **Teile** und **Dateien**. **Geschwindigkeit** und **Protokoll** gelten für das ganze Programm. Jeder Reiter ist unter [Torrent-Details und Dateien](Torrent-Details-und-Dateien) beschrieben.

## Die Statusleiste

Von links nach rechts:

| Teil | Zeigt |
|---|---|
| Update-Hinweis | „Rookparcel ... ist verfügbar - zum Herunterladen hier klicken“, nur wenn eine neuere Version erschienen ist. Siehe [Rookparcel aktualisieren](Rookparcel-aktualisieren). |
| Netzwerk | **Netzwerk OK**, wenn andere Peers sich mit Ihnen verbinden können, **Keine eingehenden Verbindungen**, wenn es noch keiner getan hat. **Port ... belegt** bedeutet, dass ein anderes Programm, oft ein anderes Rookparcel-Profil, den Listen-Port belegt; klicken Sie darauf, um Einstellungen > Verbindung zu öffnen. Der Zustand des Zeitplaners folgt, wenn er die Geschwindigkeit begrenzt, zum Beispiel „Zeitplaner: begrenzt“. **Torrents werden wiederhergestellt...** erscheint beim Start, bis die gespeicherten Torrents wieder in der Liste sind. **Übertragungen angehalten: Einstellungen bestätigen** bedeutet, dass Rookparcel den gelesenen Einstellungen nicht trauen konnte (eine Einstellung zu Proxy, Netzwerkregel oder Listen-Schnittstelle nach einem Update oder eine Einstellungsdatei, die es nicht lesen konnte), sodass sich nichts verbindet; prüfen Sie **Optionen > Einstellungen** und klicken Sie auf **OK**. Siehe [Rookparcel aktualisieren](Rookparcel-aktualisieren). |
| **DHT: ... Knoten** | wie viele DHT-Knoten Rookparcel kennt |
| **D: ... G: ...** | Download-Geschwindigkeit und insgesamt in dieser Sitzung heruntergeladen |
| **U: ... G: ...** | Upload-Geschwindigkeit und insgesamt in dieser Sitzung hochgeladen |

Hält **Ping niedrig halten** die Geschwindigkeiten niedrig, fügen die Geschwindigkeitsteile die automatische Grenze hinzu, zum Beispiel „D: 400 kB/s (auto 450 kB/s) G: 2.10 GB“.

*Neu in 1.1.58:* Findet **Nur dieses Netzwerk verwenden** keinen passenden Adapter, sagt der Netzwerkteil „Warten auf das Netzwerk“, und wenn Sie darauf zeigen, sehen Sie Ihre Adresse, wie die Tracker sie sehen. Der Text **Port ... belegt** kam mit den Profilen in 1.1.45.

Siehe [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall), [Geschwindigkeitsbegrenzung und Zeitplan](Geschwindigkeitsbegrenzung-und-Zeitplan) und [Niedrigen Ping halten](Niedrigen-Ping-halten).

## Die Titelleiste und der Infobereich

Ist **Aktuelle Geschwindigkeit in der Titelleiste anzeigen** an (Einstellungen > Oberfläche), beginnt der Fenstertitel mit den Download- und Upload-Geschwindigkeiten, die auch in der Taskleiste erscheinen.

Rookparcel legt ein Symbol in den Infobereich. Zeigen Sie darauf, sehen Sie die Version und die Geschwindigkeiten. Ein Klick zeigt oder verbirgt das Fenster; ein Rechtsklick öffnet ein Menü mit **Rookparcel ausblenden** oder **Rookparcel anzeigen**, **Torrent hinzufügen...**, **Torrent von URL hinzufügen...**, **Alle Torrents starten**, **Alle Torrents stoppen**, **Rookparcel sperren** (erst wenn eine PIN gesetzt ist) und **Beenden**. Standardmäßig blendet die Schaltfläche zum Schließen das Fenster in den Infobereich aus, und die Torrents laufen weiter; **Beenden** beendet das Programm. Siehe [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart).

## In der GTK-Version (Linux, BSD)

- Dasselbe Layout: Symbolleiste, Kategorieliste, Torrent-Liste, Detail-Reiter und Statusleiste. Die Symbolleiste hat keine Schaltfläche **Einstellungen**; verwenden Sie **Optionen > Einstellungen** oder <kbd>Ctrl</kbd>+<kbd>P</kbd>.
- Das Menü **Optionen** hat **Einstellungen**, **Einrichtungsassistent**, **RSS-Downloader...** und **Automatisch herunterfahren**. Es gibt darin keine Einträge **Anzeigen** und keine Einträge für den Infobereich. **Datei** hat **Rookparcel sperren** wie unter Windows.
- **Datei > Beenden** ist auch <kbd>Ctrl</kbd>+<kbd>Q</kbd>.
- Ein Rechtsklick auf einen Spaltenkopf blendet Spalten ein oder aus; Köpfe lassen sich ziehen, um sie neu anzuordnen.
- Der Doppelklick funktioniert wie unter Windows: Ein fertiger Torrent öffnet seinen Ordner, ein anderer öffnet die Eigenschaften.

## In der Weboberfläche

- Die Seite hat eine Symbolleiste (**Torrent hinzufügen**, **URL hinzufügen**, **Entfernen**, **Starten**, **Anhalten**, **Stoppen**, **Nach oben**, **Nach unten**, **RSS**, **Einstellungen** und **Suchen**), die Kategorieliste, die Torrent-Liste und dieselben sieben Detail-Reiter sowie ein Suchfeld, das die Liste nach Namen filtert.
- Klicken Sie auf einen Kopf, um zu sortieren, ziehen Sie ihn, um neu anzuordnen, ziehen Sie seinen Rand, um die Größe zu ändern. Klicken Sie die Köpfe mit der rechten Maustaste an, um Spalten ein- oder auszublenden; **Spalten zurücksetzen** geht zurück zum Standard.
- Auf einem Smartphone öffnet sich die Kategorieliste über die Schaltfläche ☰, und die Schaltfläche ⋯ hat die Aktionen, die nicht hineinpassen, **Sortieren nach** und **Absteigend sortieren**.

Siehe [Weboberfläche](Weboberfläche).

## Verwandte Seiten

- [Tastenkürzel](Tastenkürzel)
- [Torrents verwalten](Torrents-verwalten)
- [Torrent-Details und Dateien](Torrent-Details-und-Dateien)
