*Dies ist eine Übersetzung der englischen Seite [Remote control](Remote-control). Maßgeblich ist die englische Seite.*

# Fernsteuerung

Das Desktop-Programm kann ein `rookparcel-nox` auf einem anderen Computer steuern: einem NAS, einem Heimserver, einem PC in einem anderen Zimmer. Das Fenster sieht aus und funktioniert wie gewohnt, aber die Torrents, Downloads und die Engine liegen auf dem Server. Diese Seite beschreibt das Verbinden mit **Datei > Mit Fernsitzung verbinden...** oder `--remote`, was in einem entfernten Fenster funktioniert und wie die Verbindung geschützt wird.

Die Alternative ist die [Web-UI](Weboberfläche) in einem Browser, die nichts Installiertes braucht. Ein entferntes Fenster gibt Ihnen stattdessen das vollständige Desktop-Programm: seine Dialoge, Tastenkürzel und Detail-Reiter.

## Was Sie brauchen

- Ein `rookparcel-nox`, das mit eingeschalteter Web-UI läuft, zum Beispiel `rookparcel-nox run --webui 0.0.0.0:8443 --webui-https` ([rookparcel-nox betreiben](rookparcel-nox-betreiben)). Das entfernte Fenster verwendet den Port und die Anmeldung der Web-UI.
- Den Benutzernamen und das Passwort der Web-UI.
- Dieselbe Rookparcel-Version auf beiden Seiten. Ein Server, der eine andere Version spricht, lehnt die Verbindung ab mit „the server refused the connection (HTTP 426): this server speaks remote protocol ...“. Das Fernsteuerungsprotokoll ändert sich von Zeit zu Zeit (zuletzt in 1.2.70), und 1.2.76 verbindet sich nicht mit 1.2.63 bis 1.2.69; aktualisieren Sie also den Server und die Computer mit entfernten Fenstern gemeinsam. Die Web-UI und die qBittorrent-kompatible API sind davon nicht betroffen.

Ein entferntes Fenster verbindet sich mit `rookparcel-nox`. Die eigene Web-UI des Windows-Programms (**Einstellungen > Web-UI**) ist für Browser gedacht.

## Verbinden

1. Wählen Sie im Windows-Programm **Datei > Mit Fernsitzung verbinden...**. Ein zweites Rookparcel-Fenster startet neben Ihrem eigenen und zeigt den Dialog **Mit Fernsitzung verbinden**. (Oder starten Sie `rookparcel.exe --remote`, das nur das entfernte Fenster öffnet.)
2. Füllen Sie aus:

   | Feld | Was Sie eingeben |
   |---|---|
   | **Host** | Der Name oder die IP-Adresse des Servers. |
   | **Port** | Der Port der Web-UI (8080 wird vorgeschlagen). |
   | **Benutzername** | Der Benutzername der Web-UI (`admin` wird vorgeschlagen). |
   | **Passwort** | Das Passwort der Web-UI. |
   | **HTTPS verwenden** | Standardmäßig angehakt. Lassen Sie es angehakt, wenn der Server mit `--webui-https` oder mit einem eigenen Zertifikat läuft. |

3. Klicken Sie auf **Verbinden**.
4. Beim ersten Mal wird ein Server mit einem selbstsignierten Zertifikat (dem, das `--webui-https` erstellt) mit seinem Fingerabdruck angezeigt:

   > ... legt ein Zertifikat vor, dem dieser Computer nicht vertraut (ein selbstsigniertes, wie es rookparcel-nox --webui-https erstellt).

   Vergleichen Sie die lange Zahl mit der Zeile `certificate SHA-256`, die nox beim Start ausgegeben hat (in seinem Terminal, Protokoll oder Journal). Stimmen sie überein, klicken Sie auf **Ja**. Rookparcel merkt sich das Zertifikat für diesen Server und fragt nicht wieder.
5. Das Fenster öffnet sich mit den Torrents des Servers. Seine Titelleiste zeigt `Host:Port` des Servers.

Der Dialog merkt sich Host, Port, Benutzername und die HTTPS-Wahl für das nächste Mal. Das Passwort wird nie gespeichert; Sie geben es bei jeder Verbindung ein.

Im GTK-Programm (Linux, BSD) ist es genauso: **Datei > Mit Fernsitzung verbinden...** oder `rookparcel-gtk --remote`. Die macOS-App hat ebenfalls **Datei > Mit Fernsitzung verbinden...**. Jedes entfernte Fenster ist ein eigenes Programm, sodass Sie Ihr lokales Fenster und mehrere Server gleichzeitig offen haben können.

## Ohne HTTPS

Wenn Sie **HTTPS verwenden** abwählen und der Host ein anderer Computer ist, fragt Rookparcel zuerst:

> ... ist ein anderer Computer, und ohne HTTPS gehen Benutzername, Passwort und jeder Befehl unverschlüsselt über das Netzwerk: Jeder darin kann sie lesen oder ändern.

Antworten Sie **Nein** und schalten Sie stattdessen HTTPS am Server ein (`--webui-https`), außer das Netzwerk zwischen beiden gehört ganz Ihnen. Bei einer Verbindung zum selben Computer (`localhost`, `127.0.0.1`) wird nicht gefragt.

## Was in einem entfernten Fenster funktioniert

Fast alles funktioniert wie in einem lokalen Fenster und wirkt auf den Server:

- Torrents hinzufügen: `.torrent`-Dateien von diesem Computer werden an den Server gesendet; Magnet-Links ebenso.
- Starten, Stoppen, Anhalten, Start erzwingen, Entfernen (auch mit den Daten, die auf dem Server gelöscht werden), erneut prüfen, Reihenfolge der Warteschlange, Labels, Tags, Geschwindigkeitsbegrenzungen, Tracker, Dateipriorität, Umbenennen, Torrent-Eigenschaften.
- Die Detail-Reiter, mit den Peers, Teilen, Dateien und dem Geschwindigkeitsdiagramm des Servers.
- **Magnet-Link kopieren** und **.torrent exportieren...** (*neu in 1.2.63*). Eine exportierte `.torrent`-Datei wird auf diesem Computer gespeichert.
- **Einstellungen:** Die Einstellungen der Engine (Verbindung, Bandbreite, BitTorrent, Warteschlange und Ähnliches) sind die des Servers. Was Sie dort ändern, wird auf dem Server angewendet und dort gespeichert.

Was anders ist:

| | In einem entfernten Fenster |
|---|---|
| Ordner | Ordner liegen auf dem Server. Ordnerauswahl für Torrents ist aus; **Speicherort festlegen...** fragt nach einem Serverordner als Text. Neue Downloads gehen in den Standardordner des Servers. |
| **Ordner öffnen** | Aus: Der Ordner liegt auf dem Server. |
| Torrents erstellen | Ein Torrent, den Sie hier erstellen, kann vom Server nicht geseedet werden, da die Dateien auf diesem Computer liegen. |
| IP-Filter | Der eigene IP-Filter des Servers gilt; der des Fensters wird nicht gesendet. |
| Importieren | **Datei > Import** tut nichts (in den Programmen für Windows und macOS ausgegraut): Der Server übernimmt keine Torrents aus dem gespeicherten Zustand eines anderen Clients. |
| Ordnergrenzen | Ein Server, der mit `--webui-save-root` gestartet wurde, lehnt ein Hinzufügen oder Verschieben in einen Ordner außerhalb dieser Wurzeln ab, wie bei der Web-UI. Eine Umbenennung, die den Ordner des Torrents verlassen würde, wird ebenfalls abgelehnt. |
| **Einstellungen > Web-UI** | Ein entferntes Fenster kann die Web-UI nicht bereitstellen: „Ein entferntes Fenster kann die Web-UI nicht bereitstellen; verwenden Sie die Einstellungen des lokalen Fensters.“ |
| Labels, Tags, RSS-Feeds, Fenstereinstellungen | Werden auf diesem Computer gehalten, für jeden Server getrennt (siehe unten). |
| Zeitplaner, RSS-Downloader, Seeding-Ziel, `.torrent`-Ordner | Die des Fensters laufen, solange es offen ist, und wirken auf die Torrents des Servers, zusätzlich zu allem, was nox selbst tut. |

Der Server lädt weiter herunter, nachdem Sie das Fenster geschlossen haben.

## Die Verbindung verlieren

Bricht die Verbindung ab, sagt Rookparcel das einmal:

> Die Verbindung zu ... wurde unterbrochen: ... Die Liste zeigt die Torrents, wie sie waren. Schließen Sie dieses Fenster und verbinden Sie sich mit Datei > Mit Fernsitzung verbinden... erneut.

Die Titelleiste endet dann auf „(getrennt)“. Befehle gehen von da an nirgendwohin. Schließen Sie das Fenster und verbinden Sie sich erneut.

## Wenn das Verbinden fehlschlägt

| Meldung | Was zu prüfen ist |
|---|---|
| „Geben Sie den Hostnamen oder die Adresse des Servers und seinen Port (1 bis 65535) ein.“ | Der Host ist leer, oder der Port ist keine Zahl im gültigen Bereich. |
| „Keine Verbindung zu ...: wrong user name or password“ | Die Anmeldung der Web-UI. Zu viele falsche Versuche sperren die Adresse für eine Weile ([Weboberfläche](Weboberfläche#anmelden)). |
| „... Ist es ein rookparcel-nox mit seiner Web-UI auf diesem Port (--webui)?“ | Auf diesem Port hat etwas anderes geantwortet. |
| „... Wenn die Web-UI des Servers HTTPS verwendet (--webui-https), aktivieren Sie HTTPS verwenden.“ | Der Server spricht HTTPS, aber **HTTPS verwenden** war aus. |
| „... ist über HTTPS nicht erreichbar: ...“ | **HTTPS verwenden** ist an, aber der Server spricht kein HTTPS oder ist gar nicht erreichbar. |
| „Das Zertifikat von ... ist nicht das, das Sie zuvor akzeptiert haben: ...“ | Das Zertifikat des Servers hat sich geändert. Wenn Sie es ersetzt haben oder nox sein eigenes erneuert hat, verbinden Sie sich erneut, um das neue zu prüfen. Sonst könnte sich jemand dazwischengeschaltet haben: Machen Sie nicht weiter. |
| „... (HTTP 400): ...“ mit dem Host-Header genannt | Der Server lehnt den eingegebenen Hostnamen ab. Verwenden Sie seine IP-Adresse oder fügen Sie den Namen den **Server-Domains** in den **Einstellungen > Web-UI** des Servers hinzu ([Weboberfläche](Weboberfläche#sicherheit)). |
| „... (HTTP 426): this server speaks remote protocol ...“ | Die beiden Programme haben verschiedene Versionen. Aktualisieren Sie das ältere ([Rookparcel aktualisieren](Rookparcel-aktualisieren)). |

Die Firewall auf dem Server muss den Port der Web-UI hereinlassen.

## Wo das entfernte Fenster seine Dateien ablegt

| | Windows | GTK (Linux, BSD) |
|---|---|---|
| Verbindungsdaten und akzeptierte Zertifikate | `remote.ini` neben `rookparcel.ini` im Einstellungsordner | `remote.ini` in `~/.config/rookparcel/gtk` |
| Die Einstellungen, Labels, Tags und RSS-Feeds des Fensters je Server | `remote\<host>_<port>` im Einstellungsordner | `~/.config/rookparcel/gtk/remote/<host>_<port>` |

Diese vermischen sich nie mit den Einstellungen Ihres lokalen Fensters. Siehe [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt).

## Sicherheit

- Das entfernte Fenster meldet sich mit der Anmeldung der Web-UI an, sodass alles unter [Sicherheit der Web-UI](Weboberfläche#sicherheit) gilt: ein langes Passwort, HTTPS über ein Netzwerk.
- Falsche Passwörter zählen zur selben Sperre wie das Anmeldeformular der Web-UI.
- Ein neues Passwort der Web-UI trennt jedes entfernte Fenster.
- Ein Zertifikat, das sich ändert, nachdem Sie es akzeptiert haben, wird mit einer Warnung abgelehnt; Rookparcel fragt beim nächsten Versuch erneut.
- Das Passwort wird auf diesem Computer nie auf die Festplatte geschrieben.

## Siehe auch

- [rookparcel-nox betreiben](rookparcel-nox-betreiben): den Server starten, HTTPS-Optionen und Dienste.
- [Weboberfläche](Weboberfläche): derselbe Server in einem Browser.
- Die technische Beschreibung: [WEBUI.md, Remote mode](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/WEBUI.md#remote-mode).
