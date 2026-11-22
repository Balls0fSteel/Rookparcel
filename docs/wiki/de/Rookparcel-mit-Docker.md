*Dies ist eine Übersetzung der englischen Seite [Docker](Docker). Maßgeblich ist die englische Seite.*

# Rookparcel mit Docker

`rookparcel-nox`, der Daemon ohne Fenster, gibt es auch als Container-Image für Docker und andere Container-Engines. Diese Seite beschreibt den Start mit `docker run` oder Docker Compose, seine Ordner und Ports, das Passwort der Web-UI, den Betrieb als anderer Benutzer, die Verwendung mit Sonarr, Radarr und Prowlarr in Containern sowie das Stoppen und Aktualisieren.

*Neu in 1.2.21.*

Das Image ist `ghcr.io/balls0fsteel/rookparcel-nox`, für x86-64 (`amd64`) und 64-Bit-ARM (`arm64`) unter Linux. Jedes Release hat ein eigenes Tag, etwa `:1.2.21`, und `:latest` ist das neueste Release (keine Vorabversion). Darin läuft nox wie auf jedem Linux-Computer: Alles auf [rookparcel-nox betreiben](rookparcel-nox-betreiben) und [Weboberfläche](Weboberfläche) gilt.

## Ein erster Start

```sh
docker run -d --name rookparcel --restart unless-stopped --stop-timeout 60 \
  -p 8080:8080 -p 6881:6881 -p 6881:6881/udp \
  -v rookparcel-config:/config -v rookparcel-downloads:/downloads \
  ghcr.io/balls0fsteel/rookparcel-nox
```

1. Lesen Sie das vorläufige Passwort aus dem Protokoll:

   ```sh
   docker logs rookparcel
   ```

   ```text
   * Web UI: no password set; temporary password for this run: ... (set one with --webui-pass)
   ```

2. Öffnen Sie `http://127.0.0.1:8080/` (oder `http://<die Adresse des Computers>:8080/` von einem anderen Computer) und melden Sie sich damit als `admin` an.
3. Öffnen Sie auf der Seite **Einstellungen > Web-UI** und legen Sie Ihr eigenes **Passwort** fest, mit dem vorläufigen als **Aktuelles Passwort**.

Das vorläufige Passwort wird nur beim ersten Start ausgegeben und bleibt gültig, bis Sie Ihr eigenes festlegen. Ist das Protokoll weg, bevor Sie es gelesen haben (der Container wurde entfernt), legen Sie mit `ROOKPARCEL_WEBUI_PASS` ein Passwort fest ([unten](#das-web-ui-passwort)).

Ohne Befehl führt das Image Folgendes aus:

```sh
rookparcel-nox run --state-dir /config --save /downloads --webui 0.0.0.0:8080 --port 6881
```

Optionen nach dem Image-Namen werden diesem Befehl hinzugefügt, und eine erneut angegebene Option gewinnt gegenüber dem Standard: `... ghcr.io/balls0fsteel/rookparcel-nox --save /srv/media/downloads --verbose`. Ein Befehl nach dem Image-Namen ersetzt ihn dagegen: `... ghcr.io/balls0fsteel/rookparcel-nox run --state-dir /config --save /srv/media/downloads --webui 0.0.0.0:8080 --port 6881`. Jede Option ist auf [rookparcel-nox betreiben](rookparcel-nox-betreiben#optionen-von-run-und-add) aufgeführt.

## Ordner

| Ordner | Was er enthält |
|---|---|
| `/config` | Alles, was nox aufbewahrt: die Torrent-Liste und den Fortschritt jedes Torrents, den DHT-Zustand, die Anmeldung und die Einstellungen der Web-UI (`webui.json`), Labels, Tags und den RSS-Downloader. Halten Sie ihn auf einem Volume, sonst beginnt ein Neustart mit einer leeren Liste. |
| `/downloads` | Der Standard-Download-Ordner. |

Beide sind Volumes. Für Sonarr, Radarr und Prowlarr binden Sie stattdessen Ihren Medienordner unter demselben Pfad ein, den die Apps verwenden, und speichern darin ([unten](#sonarr-radarr-und-prowlarr-in-containern)).

Zum Sichern stoppen Sie den Container und kopieren das Volume `/config`. Um von vorn zu beginnen, entfernen Sie den Container und das Volume.

## Ports

| Port | Wofür |
|---|---|
| `8080/tcp` | Die Web-UI und die qBittorrent-kompatible API. |
| `6881/tcp`, `6881/udp` | Der Listen-Port für Peers (UDP trägt uTP und das DHT). |

Ordnen Sie den Listen-Port außen und innen derselben Nummer zu (`-p 6881:6881`): nox teilt Trackern und Peers den Port mit, auf dem es lauscht, sodass `-p 6882:6881` ihnen den falschen nennen würde. Um einen anderen Port zu verwenden, ändern Sie beide, zum Beispiel `-p 51413:51413 -p 51413:51413/udp` und `--port 51413`. Ein später in den **Einstellungen > Verbindung** der Web-UI geänderter Port braucht dieselbe Änderung in der Zuordnung.

**UPnP und NAT-PMP funktionieren vom üblichen (Bridge-)Netzwerk von Docker aus nicht:** Der Router ist von dort nicht erreichbar, sodass der Port nicht für Sie weitergeleitet wird. Leiten Sie ihn von Hand im Router weiter, TCP und UDP, an den Computer, auf dem Docker läuft ([Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall#den-port-im-router-weiterleiten)), oder lassen Sie den Container unter Linux im Netzwerk des Hosts laufen (`--network host`), wo nox den Router und das lokale Netzwerk sieht wie ein Programm außerhalb eines Containers und `-p` nicht nötig ist. Auch Local Peer Discovery findet Peers in Ihrem Netzwerk nur mit Host-Netzwerk.

## Das Web-UI-Passwort

| Variable | Bedeutung |
|---|---|
| `ROOKPARCEL_WEBUI_PASS` | Legt bei jedem Start das Passwort der Web-UI fest. Es wird als Hash in `/config` gespeichert und bleibt daher bestehen, nachdem Sie die Variable entfernt haben. |
| `ROOKPARCEL_WEBUI_USER` | Der dazugehörige Anmeldename (Standard `admin`). Wird nur zusammen mit `ROOKPARCEL_WEBUI_PASS` verwendet. |

```sh
docker run -d --name rookparcel ... -e ROOKPARCEL_WEBUI_PASS='a long password' ghcr.io/balls0fsteel/rookparcel-nox
```

Ohne `ROOKPARCEL_WEBUI_PASS` behält nox das gespeicherte Passwort oder gibt ein vorläufiges aus, wenn noch keines gespeichert ist.

Zwei Dinge sind zu wissen:

- Solange die Variable gesetzt ist, gewinnt sie bei jedem Start: Ein Passwort, das Sie auf der Seite geändert haben, wird beim nächsten Start wieder zu dem der Variable. Entfernen Sie die Variable, um das auf der Seite festgelegte zu behalten.
- `docker inspect` zeigt die Variable, und nox bekommt das Passwort auf seiner Kommandozeile, die `docker top` und die Prozessliste des Hosts zeigen. Verwenden Sie auf einem Computer, an dem sich andere anmelden können, stattdessen das vorläufige Passwort und legen Sie Ihr eigenes auf der Seite fest, oder entfernen Sie die Variable nach dem ersten Start.

Die Web-UI ist einfaches HTTP. In einem Netzwerk, dem Sie nicht vertrauen, stellen Sie sie über HTTPS bereit (`--webui-https`) oder setzen Sie sie hinter einen Reverse-Proxy, der das tut: siehe [HTTPS](Weboberfläche#https) und [Sicherheit der Web-UI](Weboberfläche#sicherheit).

## Benutzer und Berechtigungen

nox läuft als Benutzer `rookparcel`, Benutzer- und Gruppen-ID 1000, nie als root. Neue Dateien sind für alle lesbar und für ihre Gruppe beschreibbar (umask `0002`), sodass die Apps in anderen Containern sie importieren und entfernen können.

| Um | Tun Sie Folgendes |
|---|---|
| In einen Host-Ordner zu schreiben, der einem anderen Benutzer gehört | Starten Sie den Container als dieser Benutzer: `--user 1001:1001` (`user: "1001:1001"` in Compose). Die IDs finden Sie mit `id -u` und `id -g`. |
| Eine andere umask zu verwenden | `-e UMASK=0022` (Dateien für alle lesbar, nur für ihren Besitzer beschreibbar). |

Bei einem anderen Benutzer muss auch `/config` für ihn beschreibbar sein. Ein neues benanntes Volume gehört dem Benutzer 1000; verwenden Sie also stattdessen einen Host-Ordner, den Sie diesem Benutzer geben:

```sh
sudo mkdir -p /srv/rookparcel && sudo chown 1001:1001 /srv/rookparcel
docker run -d --name rookparcel --user 1001:1001 -v /srv/rookparcel:/config ...
```

## Docker Compose

Das Repository hat eine fertige [compose.yaml](https://github.com/Balls0fSteel/Rookparcel/blob/main/contrib/docker/compose.yaml). Sie hält `/config` in einem Volume, bindet `/srv/media` unter demselben Pfad ein, speichert nach `/srv/media/downloads` und gibt nox 60 Sekunden zum Stoppen:

```yaml
services:
  rookparcel:
    image: ghcr.io/balls0fsteel/rookparcel-nox:latest
    container_name: rookparcel
    restart: unless-stopped
    user: "1000:1000"
    environment:
      ROOKPARCEL_WEBUI_PASS: ${ROOKPARCEL_WEBUI_PASS:-}
    command: [run, --state-dir, /config, --save, /srv/media/downloads, --webui, "0.0.0.0:8080", --port, "6881"]
    volumes:
      - rookparcel-config:/config
      - /srv/media:/srv/media
    ports:
      - "8080:8080"
      - "6881:6881/tcp"
      - "6881:6881/udp"
    stop_grace_period: 60s

volumes:
  rookparcel-config:
```

1. Legen Sie das Passwort in eine Datei `.env` neben `compose.yaml`: `ROOKPARCEL_WEBUI_PASS=a long password`. Ohne sie lesen Sie das vorläufige mit `docker compose logs rookparcel`.
2. Ändern Sie `/srv/media` und `user:` auf Ihren Medienordner und dessen Besitzer.
3. Starten Sie es mit `docker compose up -d`.

## Sonarr, Radarr und Prowlarr in Containern

Richten Sie Rookparcel in den Apps als **qBittorrent**-Client ein, wie es [Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr) beschreibt. In Containern unterscheiden sich drei Dinge:

1. **Derselbe Pfad in jedem Container.** Binden Sie den Medienordner unter demselben Pfad im Container von nox und in denen der Apps ein (jeweils `/srv/media`, wie in der Compose-Datei oben) und speichern Sie die Downloads darin. Die Apps finden die Dateien dann dort, wo nox sie meldet, und Importe können Hardlinks verwenden.
2. **Der Hostname.** Im selben Compose- oder benutzerdefinierten Netzwerk erreichen die Apps nox als `http://rookparcel:8080` (der Dienst- oder Containername). nox antwortet nur auf IP-Adressen und `localhost`, bis Sie den Namen erlauben: Fügen Sie `rookparcel` zu **Server-Domains (mehrere Einträge durch Semikolon trennen)** in den **Einstellungen > Web-UI** der Web-UI hinzu, unter **Erreichbarkeit**. Bis dahin schlägt der **Test** der Apps mit „Bad Request: unexpected Host header“ fehl. Siehe [Hostnamen und Docker](Sonarr-Radarr-und-Prowlarr#hostnamen-und-docker).
3. **Der Download-Ordner, nach dem die Apps fragen,** muss ein Pfad innerhalb des Containers von nox sein, etwa `/srv/media/downloads`.

`--webui-save-root /srv/media` (dem `run`-Befehl hinzugefügt) begrenzt die Ordner, in die die Web-UI und die Apps speichern können.

## Protokolle, Stoppen und Neuladen

nox schreibt sein Protokoll in die Ausgabe des Containers: `docker logs rookparcel` (`-f`, um mitzuverfolgen). `--verbose` fügt Tracker- und Peer-Ereignisse hinzu.

nox speichert beim Stoppen den Zustand jedes Torrents. `docker stop` sendet `SIGTERM` und wartet standardmäßig 10 Sekunden, bevor es den Container beendet, was bei einer großen Liste zu kurz sein kann; geben Sie ihm also eine Minute:

```sh
docker stop -t 60 rookparcel
```

`--stop-timeout 60` bei `docker run` oder `stop_grace_period: 60s` in Compose macht das zum Standard für den Container. Ein sauberer Stopp endet mit diesen Zeilen im Protokoll, und der Container beendet sich mit Code 0:

```text
* SIGTERM received (again to quit at once, without saving)
* shutting down
* stopped
```

`docker kill -s HUP rookparcel` lädt den IP-Filter neu, wie es `SIGHUP` außerhalb eines Containers tut ([Beenden und neu laden](rookparcel-nox-betreiben#beenden-und-neu-laden)).

## Aktualisieren

Das Image sucht nicht nach Updates, und seine Web-UI zeigt keinen Update-Link: Ein Container wird aktualisiert, indem Sie das neue Image holen und einen neuen Container auf demselben `/config` starten.

```sh
docker pull ghcr.io/balls0fsteel/rookparcel-nox
docker stop -t 60 rookparcel && docker rm rookparcel
docker run -d --name rookparcel ...    # the same command as before
```

Mit Compose: `docker compose pull`, dann `docker compose up -d`.

Aus demselben Grund wird der [Suchhelfer](Torrents-suchen#der-suchhelfer) im Image nicht angeboten. Ihr eigenes Prowlarr oder Jackett lässt sich weiterhin als Suchquelle hinzufügen ([Ihr eigenes Prowlarr oder Jackett hinzufügen](Torrents-suchen#ihr-eigenes-prowlarr-oder-jackett-hinzufügen)).

## Das Image selbst bauen

Aus einer Kopie des Repositorys:

```sh
docker build -t rookparcel-nox .
```

Der Build kompiliert nox in einem Rust-Container und kopiert es in ein kleines Debian-Image. Er dauert auf allen Kernen des Computers einige Minuten; `--build-arg CARGO_BUILD_JOBS=1` verwendet einen. Verwenden Sie danach `rookparcel-nox` anstelle von `ghcr.io/balls0fsteel/rookparcel-nox` in den obigen Befehlen.

## Siehe auch

- [rookparcel-nox betreiben](rookparcel-nox-betreiben): jeder Befehl und jede Option.
- [Weboberfläche](Weboberfläche): die Seite, HTTPS und Sicherheit.
- [Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr): die Einstellungen der Apps, Kategorien und Remove Completed.
- [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall): prüfen, ob Peers sich verbinden können.
- Die Dateien des Images: [Dockerfile](https://github.com/Balls0fSteel/Rookparcel/blob/main/Dockerfile) und [contrib/docker](https://github.com/Balls0fSteel/Rookparcel/tree/main/contrib/docker).
