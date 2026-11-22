*Dies ist eine Übersetzung der englischen Seite [Web UI](Web-UI). Maßgeblich ist die englische Seite.*

# Weboberfläche

Die Web-UI ist das Hauptfenster von Rookparcel in einem Webbrowser: die Torrent-Liste, die Kategorien, die Detail-Reiter, das Kontextmenü, „Hinzufügen“, die Einstellungen und der RSS-Downloader. Diese Seite beschreibt, wie Sie sie im Windows-Programm und in `rookparcel-nox` einschalten, sich anmelden, HTTPS, was die Seite kann und wie Sie sie auf einem Smartphone verwenden.

Es gibt zwei Wege dorthin:

- **Vom Windows-Programm.** `rookparcel.exe` stellt die Seite selbst bereit, über die Torrents des Fensters, solange es läuft (**Optionen > Einstellungen > Web-UI**).
- **Von `rookparcel-nox`**, dem Rookparcel ohne Fenster, gestartet mit `--webui` ([rookparcel-nox betreiben](rookparcel-nox-betreiben)). Das ist der Weg auf Linux-, BSD- und macOS-Servern und auf einem Windows-PC, der ohne Fenster weiter herunterladen soll.

Das GTK-Programm (Linux, BSD) und die macOS-App stellen keine eigene Web-UI bereit. Beide können stattdessen ein `rookparcel-nox` per [Fernsteuerung](Fernsteuerung) steuern.

## Im Windows-Programm einschalten

![Einstellungen > Web-UI](images/prefs-webui.png)

1. Öffnen Sie **Optionen > Einstellungen** und wählen Sie **Web-UI**.
2. Haken Sie **Web-UI aktivieren** an.
3. Geben Sie unter **Authentifizierung** einen **Benutzername** (`admin` wird vorgeschlagen) und ein **Passwort** ein. Beides ist nötig; ohne sie sagt die Seite „Bitte geben Sie einen Benutzernamen und ein Passwort für die Web-UI ein.“ Das Passwort wird nur als Hash gespeichert. Lassen Sie das Feld später leer, um das gespeicherte zu behalten.
4. Prüfen Sie unter **Erreichbarkeit** den **Port für eingehende Verbindungen**. 8080 wird vorgeschlagen, oder der nächste freie Port, wenn ihn ein anderes Programm oder ein anderes [Profil](Profile) belegt.
5. Klicken Sie auf **Übernehmen** oder **OK**. Die Zeile unten sagt „Die Web-UI läuft: ...“ mit ihrer Adresse.
6. Klicken Sie auf **Web-UI öffnen**, um sie in Ihrem Browser zu öffnen.

Kann die Web-UI nicht starten (zum Beispiel weil der Port belegt ist), sagt die Statuszeile der Seite, warum. Die Einstellungen auf den anderen Seiten werden trotzdem angewendet.

Die Web-UI läuft nur, solange Rookparcel läuft. Schließen Sie das Fenster in den Infobereich, damit sie erreichbar bleibt ([Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart)). Sie startet neu, wenn Sie Änderungen auf dieser Seite übernehmen.

Die übrigen Einstellungen auf der Seite:

| Einstellung | Was sie bewirkt |
|---|---|
| **Verbindungen von anderen Computern zulassen** | Lauscht auf jeder Netzwerkadresse, sodass auch andere Computer und Smartphones die Seite erreichen, nicht nur dieser. Sie brauchen dazu auch eine Windows-Firewall-Regel für `rookparcel.exe` ([Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall)). |
| **HTTPS verwenden (selbstsigniertes Zertifikat)** | Stellt die Seite über HTTPS bereit, mit einem Zertifikat, das Rookparcel selbst erstellt und im Einstellungsordner aufbewahrt. Browser warnen vor einem solchen Zertifikat, bis Sie es akzeptieren. |
| **Server-Domains (mehrere Einträge durch Semikolon trennen)** | Hostnamen, auf die die Seite außer IP-Adressen und `localhost` antwortet, etwa der Name dieses Computers. Andere Namen werden abgelehnt. Siehe [Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr#hostnamen-und-docker). |
| **API-Schlüssel akzeptieren (Authorization: Bearer)** und **API-Schlüssel (mindestens 16 Zeichen)** | Lässt Skripte und Apps wie Sonarr oder Radarr die Web-UI mit diesem Schlüssel statt mit einer Anmeldung verwenden. Standardmäßig aus. Der Schlüssel wird nur als Hash gespeichert und nie wieder angezeigt; lassen Sie das Feld leer, um ihn zu behalten. |
| **Authentifizierung für Clients in diesen Subnetzen umgehen (...)** | Clients in diesen Netzwerken (etwa `192.168.1.0/24`) verwenden die Web-UI ohne Anmeldung, mit voller Kontrolle. Tragen Sie nur Netzwerke ein, denen Sie so sehr vertrauen wie dem Passwort. |

Was anders ist als bei der Seite von nox, wenn das Fenster sie bereitstellt:

- Die Torrents, Labels, Tags, Einstellungen und RSS-Feeds sind die des Fensters. Eine Änderung im Browser zeigt sich im Fenster und umgekehrt. Der RSS-Downloader der Seite bearbeitet die Feeds und Filter des Fensters.
- Der Zeitplaner, das Seeding-Ziel und der Ordner für abgeschlossene Downloads des Fensters erledigen die Arbeit, wie ohne Web-UI.
- Es gibt keine alternativen Ratenlimits.
- Ein Fenster, das ein entferntes nox steuert ([Fernsteuerung](Fernsteuerung)), kann die Web-UI nicht bereitstellen: „Ein entferntes Fenster kann die Web-UI nicht bereitstellen; verwenden Sie die Einstellungen des lokalen Fensters.“

## In rookparcel-nox einschalten

Starten Sie nox mit `--webui` und einem Port:

```sh
rookparcel-nox run --save ~/Downloads --webui 8080
```

| `--webui`-Wert | Wer die Seite erreichen kann |
|---|---|
| `8080` | Nur dieser Computer (`127.0.0.1:8080`). |
| `0.0.0.0:8080` | Jeder Computer, der diesen erreichen kann. |
| `192.168.1.10:8080` | Computer, die diesen unter dieser Adresse erreichen. |

Der Anmeldename ist `admin`, außer Sie geben `--webui-user` an. Der erste Start, ohne gespeichertes Passwort, gibt ein vorläufiges aus:

```text
* Web UI: no password set; temporary password for this run: ... (set one with --webui-pass)
```

Melden Sie sich damit an und legen Sie dann Ihr eigenes in den **Einstellungen > Web-UI** der Seite fest (das Feld **Passwort**, mit dem vorläufigen als **Aktuelles Passwort**). `--webui-pass` setzt es ebenfalls, aber ein Passwort auf der Kommandozeile können andere Benutzer des Computers sehen.

Ein nox, das als Dienst läuft, schreibt das vorläufige Passwort in sein Protokoll oder in das Journal: siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

## Anmelden

Öffnen Sie die Adresse in einem Browser: `http://127.0.0.1:8080/` auf demselben Computer oder `http://<die Adresse des Computers>:8080/` von einem anderen aus (`https://`, wenn HTTPS an ist). Geben Sie **Benutzername** und **Passwort** ein und klicken Sie auf **Anmelden**. **Abmelden** steht am rechten Ende der Symbolleiste.

- Nach fünf falschen Anmeldungen in Folge von einer Adresse wird diese Adresse eine Stunde lang gesperrt. Beide Zahlen stehen in den **Einstellungen > Web-UI**.
- Eine Anmeldung endet nach einer Stunde ohne Anfrage von diesem Browser (**Sitzungs-Timeout (Sekunden)**).
- Ein neues Passwort meldet jeden anderen Browser und jedes entfernte Fenster ab; wer es geändert hat, bleibt angemeldet.
- Wenn die Sitzung endet (nach dem Timeout oder mit **Abmelden**), schließt die Seite ihre offenen Dialoge und zeigt das Anmeldeformular. Eine `.torrent`-Datei, die zum Hinzufügen wartete, bleibt in der Warteschlange und öffnet ihren Hinzufügen-Dialog, nachdem Sie sich erneut angemeldet haben.
- Bricht die Verbindung zu Rookparcel ab, zeigt die Statusleiste „Getrennt, neuer Versuch in ... s“, und die Seite verbindet sich von selbst erneut.

## HTTPS

Ohne HTTPS laufen das Passwort und alles, was die Seite zeigt, unverschlüsselt durch das Netzwerk. Auf Ihrem eigenen Computer spielt das keine Rolle; über ein Netzwerk schalten Sie HTTPS ein.

- **Windows-Programm:** Haken Sie **HTTPS verwenden (selbstsigniertes Zertifikat)** in den **Einstellungen > Web-UI** an.
- **nox:** Fügen Sie `--webui-https` hinzu. nox erstellt bei der ersten Verwendung ein Zertifikat, bewahrt es in seinem Einstellungsordner auf (`webui-cert.pem`, `webui-key.pem`) und erneuert es einen Monat vor Ablauf. Bei jedem Start gibt es eine Zeile wie `* Web UI: certificate SHA-256 ...` aus.
- **nox mit Ihrem eigenen Zertifikat:** `--webui-cert FILE --webui-key FILE` (PEM; der Schlüssel nicht verschlüsselt).

Der Browser warnt beim ersten Mal vor einem selbstsignierten Zertifikat. Vergleichen Sie den Fingerabdruck, den der Browser zeigt, mit der Zeile `certificate SHA-256`, die nox ausgegeben hat, und akzeptieren Sie es dann. Ist HTTPS an, wird eine einfache `http://`-Adresse zu demselben Port auf `https://` weitergeleitet.

## Was die Seite kann

Die Seite folgt dem Desktop-Fenster: dieselben Spalten, Menüs, Wörter und dieselbe Reihenfolge.

- **Symbolleiste:** **Torrent hinzufügen**, **URL hinzufügen**, **Entfernen**, **Starten**, **Anhalten**, **Stoppen**, **Hoch** und **Runter** (Reihenfolge der Warteschlange), **RSS**, **Einstellungen** (<kbd>Ctrl</kbd>+<kbd>P</kbd>), das Suchfeld, das die Liste filtert, **Suchen** und **Abmelden**.
- **Kategorien** links mit ihren Zahlen, einschließlich der Labels, der Tags und der RSS-Feeds. Klicken Sie mit der rechten Maustaste auf den Zweig **Tags** (auf einem Smartphone langes Drücken) für **Neues Tag**, oder auf ein Tag für **Tag löschen** (*Tags sind neu in 1.2.63*).
- **Die Torrent-Liste:** Klicken Sie auf eine Spaltenüberschrift zum Sortieren, ziehen Sie Überschriften, um sie umzuordnen oder ihre Größe zu ändern, klicken Sie mit der rechten Maustaste auf eine Überschrift, um Spalten zu wählen (**Spalten zurücksetzen** stellt sie wieder her).
- **Die Detail-Reiter** unter der Liste: Allgemein, Tracker, Peers, Teile, Dateien, Geschwindigkeit und Protokoll ([Torrent-Details und Dateien](Torrent-Details-und-Dateien)).
- **Das Kontextmenü** enthält, was das Menü des Fensters enthält: **Start erzwingen**, **Starten**, **Anhalten**, **Stoppen**, die Warteschlangen-Verschiebungen, **Entfernen** und **Entfernen und > Daten löschen**, **Erneut prüfen** (es startet einen gestoppten Torrent, wie im Desktop-Programm; *neu in 1.2.76*), **Tracker aktualisieren**, **Magnet-Link kopieren**, **.torrent exportieren...**, **Erweitert > Speicherort festlegen...**, **Initial-Seeding**, **Sequenziell herunterladen** und **Erste und letzte Teile zuerst herunterladen**, **Bandbreite**, **Label**, **Tags** und **Eigenschaften** ([Torrents verwalten](Torrents-verwalten)). **Ordner öffnen** ist grau: Ein Browser kann keinen Ordner auf dem Server öffnen. *Magnet-Link kopieren, .torrent exportieren und die beiden Einträge zur Download-Reihenfolge sind neu in 1.2.63.* Der Export lädt die `.torrent`-Datei über den Browser herunter. Wo der Browser der Seite keine Zwischenablage gibt (einfaches `http://` von einem anderen Computer), zeigt „Magnet-Link kopieren“ die Links an, damit Sie sie von Hand kopieren.
- **Torrents hinzufügen:** **Torrent hinzufügen** lädt `.torrent`-Dateien von dem Computer hoch, auf dem der Browser läuft. **URL hinzufügen** nimmt einen Magnet-Link, einen Info-Hash oder einen `http://`-Link zu einem Torrent. Sie können auch `.torrent`-Dateien oder einen Magnet-Link auf die Seite ziehen oder einen Magnet-Link irgendwo außerhalb eines Textfelds einfügen. Ist „Ein Fenster mit den Dateien im Torrent anzeigen“ an (**Einstellungen > Oberfläche**, standardmäßig an), fragt ein Hinzufügen-Dialog dasselbe wie der des Desktops: Speicherordner, Name, Label, welche Dateien, starten oder nicht. Bei einem Magnet-Link öffnet er sich, sobald die Dateiliste des Torrents eingetroffen ist. Dateien, auf die **Dateien mit diesen Namen nicht herunterladen** (**Einstellungen > Oberfläche**) zutrifft, sind darin zunächst nicht angehakt, und ohne ihn hinzugefügte Torrents lassen sie aus (*neu in 1.2.63*). Siehe [Torrents hinzufügen](Torrents-hinzufügen#dateien-nach-namen-auslassen).
- **Torrent-Eigenschaften:** Tracker, Geschwindigkeitsbegrenzungen, Upload-Slots, das Seeding-Ziel und die Download-Reihenfolge der ausgewählten Torrents.
- **Einstellungen:** die Seiten Allgemein, Oberfläche, Ordner, Verbindung, Bandbreite, BitTorrent, Warteschlange, Zeitplaner, Web-UI, Erweitert, Festplatten-Cache und Festplatten-E/A. Fahren Sie mit der Maus über eine Einstellung für ihre Hilfe oder drücken Sie <kbd>F1</kbd> (*die Hilfetexte sind neu in 1.1.57*). Ordner sind Ordner auf dem Server und werden als Text eingegeben. Die Web-UI, die das Windows-Programm bereitstellt, hat auf ihrer BitTorrent-Seite keine IP-Filter-Adresse, kein **Jetzt aktualisieren** und keine Statuszeile: Die Einstellungen des Fensters selbst enthalten sie. Siehe [Einstellungen im Überblick](Einstellungen-im-Überblick).
- **RSS-Downloader:** Feeds, Filter und Verlauf, wie im Desktop-Programm ([Downloads per RSS](Downloads-per-RSS)).
- **Suchen** öffnet „Torrents suchen“, das Ihr Prowlarr oder Jackett durchsucht; **Quellen verwalten** dort fügt sie hinzu ([Torrents suchen](Torrents-suchen)). Das Hinzufügen einer Quelle oder das Ändern ihrer Adresse oder ihres Schlüssels verlangt das aktuelle Passwort der Web-UI (ebenso das Installieren des Suchhelfers); ein falsches meldet diesen Browser ab und zählt als fehlgeschlagene Anmeldung.

Tasten in der Liste: Pfeiltasten, <kbd>Home</kbd>, <kbd>End</kbd>, <kbd>Page Up</kbd>, <kbd>Page Down</kbd> bewegen; <kbd>Shift</kbd> und <kbd>Ctrl</kbd> mit einem Klick oder einer Taste erweitern die Auswahl; <kbd>Ctrl</kbd>+<kbd>A</kbd> wählt jeden Torrent aus, den der Filter zeigt; <kbd>Space</kbd> hält an; <kbd>Delete</kbd> entfernt; <kbd>Shift</kbd>+<kbd>Delete</kbd> entfernt und löscht die Daten; die Taste <kbd>Menu</kbd> oder <kbd>Shift</kbd>+<kbd>F10</kbd> öffnet das Kontextmenü.

Die Seite verwendet die Sprache, die unter **Einstellungen > Allgemein**, „Oberfläche“, eingestellt ist. Standardmäßig wird sie dunkel, wenn Ihr Browser oder System auf ein dunkles Design eingestellt ist. **Einstellungen > Oberfläche > Design** wählt ein Design für alle, die diese Web-UI öffnen: **System (hell oder dunkel)**, **Hell**, **Dunkel**, **Mitternacht**, **Schiefer**, **Solarized Dunkel**, **Solarized Hell**, **Hoher Kontrast** oder **Post-Saatkrähe**. **Design in diesem Browser** auf derselben Seite (auf einem Smartphone **Design** im Menü „Mehr“) wählt eines nur für diesen Browser, auf dem Gerät gespeichert; **Serverstandard** folgt wieder der Wahl des Servers. Ihr eigenes Design ist eine CSS-Datei, die Sie in den Ordner `webui/themes` unter dem Einstellungsordner legen (`--config-dir` bei `rookparcel-nox`; der Einstellungsordner des Windows-Programms, wenn es die Web-UI bereitstellt): Kopieren Sie eine der eingebauten Dateien aus dem Quellbaum ([crates/nox/src/webui/themes](https://github.com/Balls0fSteel/Rookparcel/tree/main/crates/nox/src/webui/themes)), ändern Sie die Farben und die Zeile `/* name: ... */`, benennen Sie die Datei mit Kleinbuchstaben, Ziffern und Bindestrichen, und sie wird unter diesem Namen aufgeführt.

## Auf dem Smartphone

Dieselbe Adresse funktioniert auf einem Smartphone; die Seite ändert ihr Layout auf einem schmalen Bildschirm (unter 768 Pixeln Breite).

- Die Liste zeigt den Namen jedes Torrents über einem Fortschrittsbalken mit seinem Status, den Geschwindigkeiten und der Restzeit.
- Die Kategorien liegen in einer Schublade hinter der Menütaste am Anfang der Symbolleiste.
- Tippen Sie auf einen Torrent, um seine Details in einem Blatt vom unteren Rand zu sehen.
- Die Taste in jeder Zeile öffnet ihr Menü. Langes Drücken beginnt die Auswahl mehrerer Torrents; tippen Sie auf weitere, um sie hinzuzufügen, und **Fertig** beendet die Auswahl.
- Die Symbolleiste behält **Hinzufügen**, **Starten**, **Anhalten** und **Entfernen**; der Rest und das Sortieren stehen unter **Mehr**.
- Unter Android schließt die Zurück-Taste die oberste Schublade, das oberste Blatt oder den obersten Dialog.

Damit das Smartphone die Seite erreicht, muss die Web-UI Verbindungen von anderen Computern annehmen (**Verbindungen von anderen Computern zulassen** oder `--webui 0.0.0.0:PORT` bei nox), und das Smartphone muss in einem Netzwerk sein, das den Computer erreicht.

## Sicherheit

Die Web-UI steuert Ihre Torrents und kann Dateien überall dort schreiben, wo das Konto von Rookparcel schreiben kann. Bedenken Sie Folgendes, bevor Sie sie für ein Netzwerk öffnen:

- Verwenden Sie ein langes Passwort und schalten Sie HTTPS für alles jenseits Ihres eigenen Computers ein.
- **Prüfung des Host-Headers aktivieren** (standardmäßig an) lehnt Anfragen ab, die an einen anderen Namen als eine IP-Adresse, `localhost` oder eine aufgeführte Server-Domain gerichtet sind. **Schutz vor Cross-Site Request Forgery (CSRF) aktivieren** (standardmäßig an) lehnt Änderungen ab, die von der Seite einer anderen Website gesendet werden. Lassen Sie beides an.
- **Authentifizierung für Clients auf localhost umgehen** (die Seite von nox; standardmäßig aus) lässt Programme auf demselben Computer ohne Anmeldung hinein. Sie und die Subnetz-Liste gelten nie für Anfragen, die über einen Reverse-Proxy kommen.
- Das Ändern der Anmeldung, des API-Schlüssels, der Subnetze oder der anderen Sicherheitseinstellungen auf der Seite erfordert das **Aktuelle Passwort**. Ein falsches meldet diesen Browser ab und zählt als fehlgeschlagene Anmeldung, sodass wiederholtes Raten die Adresse wie fehlgeschlagene Anmeldungen sperrt. Dasselbe gilt für Adressen, von denen der Server herunterlädt, etwa die Adresse der Tracker-Liste und die Adresse der Sperrliste des IP-Filters (Einstellungen > BitTorrent, *neu in 1.2.63*; siehe [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall#eine-liste-von-einer-webadresse)).
- Bei nox begrenzt `--webui-save-root DIR` die Ordner, in die die Web-UI speichern, verschieben und aus denen sie lesen darf ([rookparcel-nox betreiben](rookparcel-nox-betreiben#web-ui)).

Hinter einem Reverse-Proxy auf demselben Computer lassen Sie den Proxy den Header `X-Forwarded-For` setzen, damit fehlgeschlagene Anmeldungen den richtigen Client sperren und nicht alle. Ist **API-Schlüssel akzeptieren (Authorization: Bearer)** an, darf der Proxy keinen eigenen `Authorization`-Header hinzufügen: Bevor sich ein Browser angemeldet hat, zählt ein solcher Header als falscher Schlüssel.

## Siehe auch

- [Fernsteuerung](Fernsteuerung): das Fenster des Desktop-Programms, das ein nox steuert, als Alternative zur Seite.
- [Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr): die qBittorrent-kompatible API der Web-UI verwenden.
- Das Konzept hinter der Seite: [WEBUI.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/WEBUI.md).
