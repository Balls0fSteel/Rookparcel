*Dies ist eine Übersetzung der englischen Seite [Troubleshooting and FAQ](Troubleshooting-and-FAQ). Maßgeblich ist die englische Seite.*

# Fehlerbehebung und FAQ

Diese Seite hilft, wenn etwas nicht funktioniert: langsame oder stockende Downloads, ein Port, der nicht erreichbar ist, Warnungen von Windows oder macOS, private Tracker. Sie sagt auch, wo die Meldungen von Rookparcel stehen, wie Sie seine Einstellungen zurücksetzen und wie Sie ein Problem melden.

## Downloads sind langsam oder starten nicht

Sehen Sie in die Spalte **Status** des Torrents und in die Statusleiste am unteren Fensterrand. Meist sagen sie, warum.

| Was Sie sehen | Was es bedeutet und was zu tun ist |
|---|---|
| **Eingereiht** | Die Warteschlange ist voll: Die unter **Einstellungen > Warteschlange** festgelegte Anzahl aktiver Torrents oder Downloads ist erreicht. Der Torrent startet, wenn ein anderer fertig ist oder stoppt. **Start erzwingen** im Kontextmenü führt ihn außerhalb der Warteschlange aus. Siehe [Warteschlange und Seeding](Warteschlange-und-Seeding). |
| **Gestoppt** oder **Angehalten** | Starten Sie ihn mit **Starten** (<kbd>Ctrl</kbd>+<kbd>S</kbd>). Ist in Einstellungen > Oberfläche **Download nicht automatisch starten** eingeschaltet, werden neue Torrents gestoppt hinzugefügt. |
| **Zur Prüfung eingereiht**, **Prüfe** | Rookparcel prüft die Dateien, die schon auf dem Datenträger liegen. Torrents auf demselben Laufwerk werden nacheinander geprüft: Die, die auf ihre Reihe warten, zeigen **Zur Prüfung eingereiht**. Ist **Torrents schonend prüfen** eingeschaltet (Einstellungen > Gaming), gibt eine Prüfung anderen Programmen den Vortritt und dauert länger, solange der Datenträger beschäftigt ist. Siehe [Datenträger und Speicher](Datenträger-und-Speicher). *Wartende Prüfungen sagen das seit 1.2.76; vorher zeigten sie **Prüfe** bei 0 %.* |
| **Warten auf das Netzwerk** | **Nur dieses Netzwerk verwenden** ist in Einstellungen > Verbindung eingeschaltet, und sein Netzwerk (meist Ihr VPN) ist nicht da. Der Torrent macht von selbst weiter, wenn es zurückkommt. Siehe [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall). |
| **Fehler: ...** | Die Meldung nach „Fehler:“ sagt, was schiefging. Für „files missing (the drive, share or folder is not available)“ siehe den nächsten Abschnitt. Bricht eine Prüfung mit einem Fehler ab, der eine Datei nennt, hält ein anderes Programm diese Datei offen, oder Sie dürfen sie nicht lesen: Schließen Sie das Programm oder beheben Sie die Berechtigung und **Starten** Sie dann den Torrent. *Seit 1.2.76 bricht eine solche Prüfung mit dem Fehler ab; vorher zählte sie die Teile der Datei als fehlend und lud sie erneut herunter.* |
| „Zeitplaner: begrenzt“, „Zeitplaner: ausgeschaltet“ oder „Zeitplaner: nur Seeding“ in der Statusleiste | Der Zeitplaner hält in dieser Stunde die Geschwindigkeiten zurück. Siehe [Geschwindigkeitsbegrenzung und Zeitplan](Geschwindigkeitsbegrenzung-und-Zeitplan). |
| „(auto ...)“ neben den Geschwindigkeiten in der Statusleiste | **Ping niedrig halten** begrenzt die Geschwindigkeit, weil die Verzögerung auf Ihrer Leitung gewachsen ist. Siehe [Niedrigen Ping halten](Niedrigen-Ping-halten). |

Weitere Dinge zum Prüfen:

- **Seeds und Peers.** Ein Torrent ohne Seeds kann nicht fertig werden. Die Registerkarte **Tracker** zeigt, ob die Tracker antworten; die Registerkarte **Peers** zeigt, mit wem Sie verbunden sind. Siehe [Torrent-Details und Dateien](Torrent-Details-und-Dateien).
- **Magnet-Links** brauchen zuerst die Dateiliste des Torrents von anderen Peers, und die meisten brauchen das DHT: Lassen Sie **DHT-Netzwerk aktivieren** in Einstellungen > BitTorrent eingeschaltet.
- **Geschwindigkeitsbegrenzungen.** Prüfen Sie **Einstellungen > Bandbreite** und die eigenen Begrenzungen des Torrents (**Bandbreite** im Kontextmenü, oder **Eigenschaften**). 0 bedeutet unbegrenzt.
- **Ein ausgelasteter Datenträger.** Ist **Downloads verlangsamen (Schreibzugriffe)** eingeschaltet (Einstellungen > Gaming), senkt Rookparcel die Download-Geschwindigkeit, solange der Datenträger langsam antwortet.
- **Ein Proxy für Peer-Verbindungen.** Ist **Proxy für Peer-Verbindungen verwenden** eingeschaltet, kann sich niemand mit Ihnen verbinden, was weniger Peers bedeutet.
- **Keine eingehenden Verbindungen** (siehe unten) kosten ebenfalls Geschwindigkeit: Sie erreichen nur Peers, die selbst Verbindungen annehmen.

## „Files missing (the drive, share or folder is not available)“

Fehlt der Speicherordner eines Torrents (ein USB- oder Netzlaufwerk, das abgezogen ist, eine Freigabe oder ein Mount, der nicht verbunden ist), stoppt Rookparcel den Torrent mit diesem Fehler, statt ihn erneut herunterzuladen. Die Teile, die es hatte, bleiben aufgezeichnet, auch über Neustarts hinweg.

1. Verbinden Sie das Laufwerk oder die Freigabe wieder. Alle 30 Sekunden sucht Rookparcel den Ordner; ist er zurück, startet der Torrent von selbst und findet seine Dateien, wie sie waren.
2. Um es sofort zu versuchen, wählen Sie **Starten**. Fehlt der Ordner noch, stoppt der Torrent erneut mit dem Fehler.
3. Haben Sie die Dateien endgültig verschoben oder gelöscht, übernimmt **Erneut prüfen** das, was jetzt im Ordner liegt: Teile, die nicht da sind, werden erneut heruntergeladen.

Ein Laufwerk, das während des Downloads abgezogen wird, wird genauso festgehalten. Fehlen nur einige Dateien eines Torrents, lädt er stattdessen diese erneut herunter. Siehe [Torrents verwalten](Torrents-verwalten). *Neu in 1.2.76; vorher lud der Torrent alles erneut herunter.*

**Erneut prüfen** bei einem gestoppten Torrent startet ihn jetzt, wie **Starten**, und die Prüfung läuft sofort, auch wenn der Torrent in der Warteschlange stand; stoppen Sie ihn wieder, wenn er gestoppt bleiben soll. *Neu in 1.2.76; vorher wartete die Prüfung auf ein Starten.*

## „Übertragungen angehalten: Einstellungen bestätigen“

Nach einem Update, oder wenn die Einstellungsdatei nicht gelesen werden kann, hält Rookparcel alle Übertragungen an, bis Sie bestätigen, dass seine Proxy-, Netzwerk- und Listening-Einstellungen so sind, wie Sie sie wollen. Öffnen Sie **Optionen > Einstellungen > Verbindung**, prüfen Sie sie und klicken Sie auf **OK**. Die Registerkarte „Protokoll“ listet auf, was sich geändert hat. Siehe [Rookparcel aktualisieren](Rookparcel-aktualisieren#wenn-eine-version-ihre-einstellungen-anders-liest).

## „Keine eingehenden Verbindungen“ oder „Port ... belegt“

Der linke Teil der Statusleiste zeigt den Zustand des Netzwerks:

| Text | Bedeutung |
|---|---|
| **Netzwerk OK** | Andere Peers haben sich mit Ihnen verbunden, Ihr Listening-Port ist also erreichbar. |
| **Keine eingehenden Verbindungen** | Noch kein Peer hat sich mit Ihnen verbunden. Direkt nach dem Start ist das normal; bleibt es so, ist Ihr Port wahrscheinlich vom Router oder der Firewall blockiert. |
| **Port 6881 belegt** (mit Ihrer Portnummer) | Ein anderes Programm, oft ein anderes Rookparcel-Profil, belegt den Port, daher lauscht Rookparcel auf einem anderen, auf den nichts weitergeleitet wird. Klicken Sie auf den Text, um **Einstellungen > Verbindung** zu öffnen und einen anderen Port zu wählen. *Neu in 1.1.45.* |

So machen Sie den Port erreichbar:

1. Öffnen Sie **Optionen > Einstellungen > Verbindung** und notieren Sie den **Port für eingehende Verbindungen**.
2. Setzen Sie die Häkchen bei **UPnP-Portweiterleitung aktivieren** und **NAT-PMP-Portweiterleitung aktivieren**, damit der Router gebeten wird, den Port weiterzuleiten. Oder leiten Sie den Port im Router von Hand (TCP und UDP) an diesen Computer weiter und lassen **Port bei jedem Start zufällig wählen** ausgeschaltet.
3. Erlauben Sie `rookparcel.exe` in der Firewall (unter Linux `rookparcel-gtk`) oder öffnen Sie den Port darin.
4. Hinter einem VPN funktionieren eingehende Verbindungen nur, wenn das VPN einen Port an Sie weiterleitet.

Die Registerkarte **Allgemein** zeigt „Ihre Adresse laut Trackern:“, woran Sie erkennen, ob Ihr Datenverkehr über ein VPN läuft. *Neu in 1.1.58.* Mehr in [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall).

## Windows SmartScreen oder macOS Gatekeeper warnt vor Rookparcel

Die Programme sind nicht code-signiert, daher warnen die Systeme beim ersten Mal:

- **Windows:** „Der Computer wurde durch Windows geschützt“. Klicken Sie auf **Weitere Informationen** und dann auf **Trotzdem ausführen**.
- **macOS:** Die App wird abgelehnt, bis Sie sie einmal erlauben. Versuchen Sie ab macOS 15, sie zu öffnen, klicken Sie auf **Fertig** und dann in **Systemeinstellungen > Datenschutz & Sicherheit** auf **Trotzdem öffnen**. Unter macOS 11 bis 14 klicken Sie bei gedrückter Control-Taste auf die App und wählen **Öffnen**. Siehe [Rookparcel installieren](Rookparcel-installieren#macos).

Um sicherzugehen, dass eine Datei die veröffentlichte ist, vergleichen Sie ihre Prüfsumme mit der Datei `SHA256SUMS` des Releases. Einzelheiten für jede Plattform stehen in [Rookparcel installieren](Rookparcel-installieren).

## Kann ich Rookparcel auf privaten Trackern verwenden?

Noch nicht. Private Tracker akzeptieren nur Clients, die sie auf eine Whitelist gesetzt haben, und die Client-Kennung von Rookparcel (seine Peer-ID beginnt mit `-PA`, und es sendet `Rookparcel/<version>` an Tracker) steht noch nirgends auf einer Whitelist. Rookparcel gibt sich immer ehrlich als Rookparcel zu erkennen; die Verwendung auf einem privaten Tracker könnte Ihr Konto in Schwierigkeiten bringen. Prüfen Sie zuerst die Liste der erlaubten Clients Ihres Trackers.

Als privat markierte Torrents verwenden nie das DHT, die lokale Peer-Erkennung (Local Peer Discovery) oder den Peer-Austausch (Peer Exchange), was die Einstellungen auch sagen.

## Ein Doppelklick auf eine .torrent-Datei oder einen Magnet-Link öffnet Rookparcel nicht

Setzen Sie die Häkchen bei **Mit .torrent-Dateien verknüpfen** und **Mit magnet:-Links verknüpfen** unter **Einstellungen > Allgemein**. Öffnet Windows sie weiterhin mit einem anderen Programm, wurde ein Programm in Windows selbst gewählt, und nur Sie können das ändern: Klicken Sie auf **Standard-Apps...** und wählen Sie dort Rookparcel. Siehe [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart#torrent-dateien-und-magnet-links-mit-rookparcel-öffnen).

## Die Weboberfläche ist von einem anderen Gerät nicht erreichbar

- Setzen Sie das Häkchen bei **Verbindungen von anderen Computern zulassen** unter **Einstellungen > Web-UI**; sonst lauscht sie nur auf diesem Computer.
- Erlauben Sie `rookparcel.exe` in der Windows-Firewall für den Port der Weboberfläche.
- „port ... is in use, perhaps by another Rookparcel profile“: Wählen Sie einen anderen **Port für eingehende Verbindungen**.
- Eine App, die Rookparcel über einen Hostnamen erreicht (wie in Docker), scheitert mit „unexpected Host header“. Tragen Sie den Namen unter **Server-Domains** ein oder verwenden Sie die IP-Adresse.

Siehe [Weboberfläche](Weboberfläche) und [Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr).

## Ein Fernfenster meldet „the server refused the connection (HTTP 426)“

Ein Fenster, das mit **Datei > Mit Fernsitzung verbinden...** verbunden wurde, und das `rookparcel-nox`, mit dem es sich verbindet, müssen dieselbe Version haben: Die Meldung endet mit „this server speaks remote protocol ...“. Manche Releases ändern das Fernprotokoll (zuletzt in 1.2.70); nachdem Sie eine Seite auf 1.2.76 aktualisiert haben, aktualisieren Sie also auch die andere: das Desktop-Fenster und das Programm `rookparcel-nox` oder sein Docker-Image. Ein Browser, der die Weboberfläche verwendet, unterliegt dieser Grenze nicht. Siehe [Fernsteuerung](Fernsteuerung) und [Rookparcel aktualisieren](Rookparcel-aktualisieren).

## Das Beenden von Rookparcel dauert lange

Rookparcel speichert die Torrent-Liste und den Fortschritt, meldet den Trackern, dass es gestoppt hat, und wartet auf Dateien, die noch verschoben werden. Dauert das länger als einen Moment, zeigt das Fenster „Rookparcel wird beendet“, worauf es wartet. **Jetzt beenden** hört auf zu warten; es fragt zuerst und listet auf, was es aufgibt. Die Torrent-Liste und der Fortschritt werden in jedem Fall zuerst gespeichert.

Unter Linux schließt sich das GTK-Fenster genauso, wenn es <kbd>Ctrl</kbd>+<kbd>C</kbd> in seinem Terminal oder ein `SIGTERM` erhält, zum Beispiel von `kill`. Das Signal erneut zu senden, verkürzt das Beenden nicht. Um sofort zu stoppen, verwenden Sie **Jetzt beenden** oder `kill -KILL` mit der Prozess-ID. *Seit 1.2.54; vorher beendete ein zweites Signal sofort ohne Speichern.*

## Eine portable Kopie kann nicht in ihren Ordner schreiben

„Dieses portable Rookparcel kann nicht in seinen Ordner schreiben“ bedeutet, dass der Ordner mit `rookparcel.exe` und `rookparcel.ini` für Sie schreibgeschützt ist, zum Beispiel unter `C:\Program Files`. Verschieben Sie den Ordner an einen Ort, an dem Sie schreiben können, oder entfernen Sie `rookparcel.ini` neben dem Programm, um die Einstellungen in Ihrem Benutzerordner zu behalten. Siehe [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt).

## Der Such-Helfer funktioniert nicht

Siehe die Tabelle am Ende von [Torrents suchen unter Windows](Torrents-suchen-unter-Windows) und [Torrents suchen](Torrents-suchen).

## Hinweise zu Linux

- **Kein Infobereich-Symbol unter GNOME.** Einfaches GNOME hat keinen Infobereich; installieren Sie die AppIndicator-Erweiterung oder verwenden Sie Rookparcel ohne ihn (das Schließen des Fensters beendet es dann). Siehe [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart#gtk-version-linux-bsd).
- **Beim Minimieren in den Infobereich passiert nichts** auf manchen Wayland-Desktops, die nicht melden, dass ein Fenster minimiert wurde.
- **Das Abmelden unter Xfce** wartet nicht darauf, dass Rookparcel speichert; beenden Sie es vorher.

## Hinweise zu macOS

- **Das Schließen des Fensters beendet nicht.** Ist **Beim Schließen in den Infobereich** eingeschaltet (die Vorgabe), blendet sich das Fenster aus, und Rookparcel läuft in der Menüleiste weiter. Beenden Sie es mit <kbd>⌘</kbd>+<kbd>Q</kbd>. Siehe [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart#macos).
- **Keine Mitteilungen.** Erlauben Sie Rookparcel unter **Systemeinstellungen > Mitteilungen**.
- **Automatisch herunterfahren versetzt den Mac nicht in den Ruhezustand und fährt ihn nicht herunter.** Erlauben Sie Rookparcel unter **Systemeinstellungen > Datenschutz & Sicherheit > Automation**, „loginwindow“ zu steuern. Siehe [Automatisches Herunterfahren](Automatisches-Herunterfahren).
- **Einstellungen weg nach dem Ersetzen der GTK-Version.** Die neue App übernimmt den alten Ordner `gtk` nur, solange kein Ordner `mac` existiert. Wurde zuerst ein Ordner `mac` angelegt, beenden Sie Rookparcel und siehe [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt#macos).

## Wo die Protokolle sind

**Die Windows- und die GTK-Version** schreiben keine Protokolldatei. Ihre Meldungen (Fehler, der IP-Filter, die Update-Prüfung, der Zeitplaner, abgeschlossene Downloads) gehen in die Registerkarte **Protokoll** unten im Hauptfenster, die die letzten 1000 Zeilen behält, solange Rookparcel läuft. Sehen Sie dort gleich nach dem Auftreten des Problems nach, bevor Sie Rookparcel beenden, und notieren Sie die Zeilen dazu.

**rookparcel-nox** gibt seine Meldungen im Terminal aus. Für mehr Einzelheiten starten Sie es mit `--verbose`, was Tracker- und Peer-Ereignisse hinzufügt (jede Trennung mit ihrem Grund). `--log FILE` hängt alles an eine Datei an. Läuft es als Dienst:

| Wo | Protokoll |
|---|---|
| Windows-Dienst | `nox.log` im Statusordner, `%ProgramData%\Rookparcel\nox`, wenn nicht `--state-dir` etwas anderes sagt (lesen Sie sie in einer Eingabeaufforderung mit Administratorrechten) |
| systemd | das Journal: `journalctl -u rookparcel-nox` |
| FreeBSD rc.d | `/var/log/rookparcel-nox.log` |

Siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

**Der Such-Helfer** (Jackett) führt seine eigenen Einstellungen und sein Protokoll im Ordner `data` innerhalb von `search-helper` in Ihrem Rookparcel-Einstellungsordner.

## Einstellungen zurücksetzen

- **Eine Einstellung:** Wählen Sie sie in **Einstellungen > Erweitert** aus, klicken Sie auf **Zurücksetzen** und dann auf **OK**. Mit `*` markierte Zeilen weichen von der Vorgabe ab.
- **Eine vergessene Sperr-PIN:** Halten Sie <kbd>Ctrl</kbd> (Windows) oder <kbd>⌥</kbd> (macOS) gedrückt, während Sie Rookparcel starten, oder starten Sie es mit `--reset-lock`, und bestätigen Sie. Siehe [Rookparcel sperren](Rookparcel-sperren#pin-vergessen).
- **Die Fragen beim ersten Start:** **Optionen > Einrichtungsassistent** (<kbd>Ctrl</kbd>+<kbd>G</kbd>) stellt sie erneut. *Neu in 1.1.62.*
- **Alles:** Beenden Sie Rookparcel (**Datei > Beenden**) und benennen Sie dann den Ordner um, in dem es seine Einstellungen und Torrents ablegt (unter Linux und BSD den Einstellungsordner und den Datenordner); Rookparcel startet dann wie neu, mit leerer Torrent-Liste. Unter Windows zeigt **Hilfe > Über Rookparcel** den Ordner. Benennen Sie ihn zurück, ist alles wiederhergestellt. Siehe [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt).

## Einen Fehler melden

Öffnen Sie ein Issue im Tracker: <https://github.com/Balls0fSteel/Rookparcel/issues>. Bitte geben Sie an:

1. Ihr System (Windows 11, Ubuntu 24.04, macOS 15...) und die Datei, die Sie heruntergeladen haben.
2. Die Version von Rookparcel, aus **Hilfe > Über Rookparcel**.
3. Was Sie getan haben, was Sie erwartet haben und was stattdessen geschah, Schritt für Schritt.
4. Bei einem Absturz oder einem hängenden Torrent die Zeilen aus der Registerkarte **Protokoll** oder die Ausgabe von `rookparcel-nox run --verbose`.

Lassen Sie Passwörter, API-Schlüssel und alles andere Private weg; der Issue-Tracker ist öffentlich.

## Ein Sicherheitsproblem melden

Öffnen Sie kein öffentliches Issue. Verwenden Sie die private Meldung von Sicherheitslücken auf GitHub: <https://github.com/Balls0fSteel/Rookparcel/security/advisories/new>. Siehe [SECURITY.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/SECURITY.md).

## FAQ

**Ist Rookparcel kostenlos?** Kostenlos für nichtkommerzielle Nutzung unter der PolyForm Noncommercial License 1.0.0: private Nutzung, Studium, Hobbyprojekte, gemeinnützige Organisationen, Schulen und öffentliche Einrichtungen, ohne Kosten. Kommerzielle Nutzung braucht eine gesonderte Erlaubnis. Siehe [LICENSE.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/LICENSE.md).

**Hat es mit µTorrent oder qBittorrent zu tun?** Nein. Rookparcel bildet das Aussehen und Verhalten von µTorrent 2.2.1 nach, verwendet aber nichts von dessen Code. Seine Engine ist eine unabhängige Rust-Portierung von libtorrent-rasterbar 2.0.11. Seine Web-API folgt der veröffentlichten Dokumentation von qBittorrent, damit Werkzeuge wie Sonarr mit ihm sprechen können.

**Welche Sprachen spricht es?** Englisch und 34 weitere, alle eingebaut. Es folgt der Sprache Ihres Systems; eine andere wählen Sie in **Einstellungen > Allgemein**. Die Übersetzungen sind maschinell erstellt; Korrekturen sind als Issues willkommen.

**Was sendet Rookparcel, und an wen?** Die Update-Prüfung lädt eine kleine Datei von der Website des Projekts herunter, ohne etwas über Sie oder Ihre Torrents; schalten Sie sie unter **Einstellungen > Allgemein > Privatsphäre** aus. Suchanfragen gehen nur an die Suchquellen, die Sie hinzugefügt haben. Wie jeder BitTorrent-Client zeigt Rookparcel den Trackern, dem DHT und den Peers Ihrer Torrents Ihre IP-Adresse. Siehe [Rookparcel aktualisieren](Rookparcel-aktualisieren).

**Wo sind meine Einstellungen und Torrents?** Siehe [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt).

**Kann ich mehrere Kopien ausführen?** Ja, als Profile. Siehe [Profile](Profile).

**Wo bekomme ich Hilfe?** Lesen Sie die [FAQ auf der Website](https://balls0fsteel.github.io/Rookparcel/faq.html) und die Seiten dieses Wikis, oder fragen Sie im [Issue-Tracker](https://github.com/Balls0fSteel/Rookparcel/issues).
