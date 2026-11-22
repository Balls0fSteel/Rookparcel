*Dies ist eine Übersetzung der englischen Seite [Running rookparcel-nox](Running-rookparcel-nox). Maßgeblich ist die englische Seite.*

# rookparcel-nox betreiben

`rookparcel-nox` ist Rookparcel ohne Fenster: dieselbe Engine, aus einem Terminal oder als Dienst gestartet und gesteuert über seine [Web-UI](Weboberfläche), von einem Desktop-Rookparcel mit [Fernsteuerung](Fernsteuerung) oder von Sonarr, Radarr und Prowlarr ([Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr)). Diese Seite beschreibt seine Befehle und Optionen, den Betrieb als Windows-Dienst, unter systemd und unter FreeBSD, wo es seine Dateien ablegt und wie Sie es beenden.

## Wo Sie es bekommen

`rookparcel-nox` gehört zu jedem Release ([Rookparcel installieren](Rookparcel-installieren)):

| Plattform | Datei |
|---|---|
| Windows | `rookparcel-nox.exe`, neben `rookparcel.exe` in der Zip-Datei und im installierten Ordner |
| Linux | im Archiv `rookparcel-<version>-linux-<arch>.tar.gz` (mit den systemd-Units) oder einzeln als `rookparcel-nox-<version>-<arch>.AppImage` |
| macOS | `rookparcel-nox-<version>-macos-<arch>.tar.gz` |

`rookparcel-nox --help` listet jeden Befehl und jede Option auf, `rookparcel-nox --version` gibt die Version aus, und `rookparcel-nox --license` gibt die Lizenz und die Hinweise zu Drittanbieter-Software aus.

## Ein erster Start

1. Öffnen Sie ein Terminal (unter Windows eine Eingabeaufforderung) in dem Ordner mit `rookparcel-nox`.
2. Starten Sie es mit einem Download-Ordner und der Web-UI:

   ```sh
   rookparcel-nox run --save ~/Downloads --webui 8080
   ```

3. Es gibt einige Zeilen aus, die mit `*` beginnen. Eine davon ist die Adresse der Web-UI und, beim ersten Mal, eine Zeile wie diese:

   ```text
   * Web UI: no password set; temporary password for this run: ... (set one with --webui-pass)
   ```

4. Öffnen Sie `http://127.0.0.1:8080/` in einem Browser und melden Sie sich als `admin` mit diesem vorläufigen Passwort an.
5. Öffnen Sie auf der Seite **Einstellungen > Web-UI** und legen Sie Ihr eigenes Passwort fest. Von da an wird es (als Hash) gespeichert, und nox verlangt es bei jedem Start.

Um es zu beenden, geben Sie `q` ein und drücken Sie <kbd>Enter</kbd>, oder drücken Sie <kbd>Ctrl</kbd>+<kbd>C</kbd>. Es speichert beim Beenden den Zustand jedes Torrents, sodass der nächste `run` sie fortsetzt, ohne die Dateien erneut zu prüfen.

`--webui 8080` lauscht nur auf diesem Computer (`127.0.0.1`). Um die Seite von anderen Computern oder einem Smartphone zu erreichen, geben Sie auch eine Adresse an, zum Beispiel `--webui 0.0.0.0:8080`, und lesen Sie zuerst [Sicherheit der Web-UI](Weboberfläche#sicherheit).

## Befehle

| Befehl | Was er tut |
|---|---|
| `rookparcel-nox run [OPTIONS]` | Setzt jeden im Zustandsordner gespeicherten Torrent fort und läuft weiter, bis es gestoppt wird. Das ist der Daemon. |
| `rookparcel-nox add <file.torrent>... [--save DIR] [OPTIONS]` | Fügt die Torrents hinzu (Dateien oder `magnet:`-Links), lädt sie herunter und beendet sich, wenn sie fertig sind. Mit `--seed` seedet es stattdessen weiter. |
| `rookparcel-nox create <file or folder> [-o OUT] [CREATE OPTIONS]` | Erstellt eine `.torrent`-Datei ([unten](#eine-torrent-datei-erstellen)). |
| `rookparcel-nox service ...` | Der Windows-Dienst (nur Windows, [unten](#der-windows-dienst)). |
| `rookparcel-nox demo [--torrents N] [--active FRACTION] [--seconds N]` | Lädt simulierte Torrents und gibt Summen aus; nur zum Testen. |

`run` nimmt nach den Optionen auch `.torrent`-Dateien und Magnet-Links entgegen; sie werden beim Start hinzugefügt.

## Optionen von run und add

### Ordner und Zustand

| Option | Bedeutung |
|---|---|
| `--save DIR` | Der Download-Ordner. Standard: der aktuelle Ordner. |
| `--state-dir DIR` | Wo die Torrent-Liste und die Fortsetzungsdaten liegen ([Dateien](#wo-es-seine-dateien-ablegt)); auch der Einstellungsordner, sofern `--config-dir` nicht angegeben ist. `--state DIR` ist dasselbe. |
| `--config-dir DIR` | Wo die Anmeldung und die Einstellungen der Web-UI (`webui.json`) liegen. |
| `--profile NAME` | Startet ein getrenntes nox mit eigenen Ordnern, eigener Anmeldung und eigenem Listen-Port ([Profile](#profile)). |
| `--no-state` | Lädt und speichert nichts. |
| `--move-on-complete DIR` | Verschiebt jeden Torrent, der fertig heruntergeladen ist, nach DIR. |

### Seeding und Stoppen

| Option | Bedeutung |
|---|---|
| `--seed` | `add`: seedet nach dem Download weiter, statt sich zu beenden. |
| `--super-seed` | Initial-Seeding für Torrents, die vollständig sind; schließt `--seed` ein. |
| `--seconds N` | Stoppt nach N Sekunden. |
| `--exit-when-done downloads` oder `all` | Beendet sich, mit gespeichertem Zustand, sobald nichts mehr herunterlädt (`downloads`) oder nichts mehr herunterlädt oder seedet (`all`). Dasselbe wie **Optionen > Automatisch herunterfahren** im Desktop-Programm ([Automatisches Herunterfahren](Automatisches-Herunterfahren)). |

### Netzwerk

| Option | Bedeutung |
|---|---|
| `--port N` | Der Listen-Port für Peers. Standard 6881. |
| `--listen LIST` | Listen-Schnittstellen statt `--port`: `IP:PORT`, `[IPv6]:PORT` oder `DEVICE:PORT`, durch Kommas getrennt. |
| `--dl-limit KIB`, `--ul-limit KIB` | Download- und Upload-Limits in KiB/s; 0 heißt unbegrenzt. |
| `--ip-filter FILE` | Sperrt die Adressbereiche in FILE (`ipfilter.dat`, P2P- oder CIDR-Format; CIDR ist neu in 1.2.63). |
| `--proxy TYPE://[USER:PASS@]HOST:PORT` | Verbindet über einen Proxy; TYPE ist `socks4`, `socks5` oder `http`. Eingehend wird nichts angenommen, solange Peers über ihn laufen. |
| `--no-proxy-peers` | Verbindet sich direkt mit Peers und Web-Seeds; nur Tracker verwenden den Proxy. |
| `--no-proxy-trackers` | Kontaktiert Tracker direkt. |
| `--no-proxy-hostnames` | Löst Hostnamen hier statt beim Proxy auf. |
| `--socks5-udp-local-ep` | Für einen SOCKS5-Proxy ohne NAT dazwischen; standardmäßig aus. |
| `--network RULE` | Nur dieses Netzwerk verwenden: Der Torrent-Verkehr wird an den passenden Adapter gebunden. RULE ist `vpn` (ein beliebiger VPN-Adapter), `adapter:ID` (ein Gerätename; unter Windows die GUID des Adapters), `range:CIDR` (etwa `10.2.0.0/16`) oder `off`. |
| `--no-pause-when-missing` | Solange die `--network`-Regel auf nichts passt, lässt es Torrents ungebunden laufen, statt auf das Netzwerk zu warten. |

*`--network` ist neu in 1.1.58.* Die Proxy- und Netzwerkoptionen werden in den Einstellungen der Web-UI gehalten wie die entsprechenden Kästchen ihrer Seite **Einstellungen > Verbindung**. Sobald Sie in der Web-UI etwas ändern, gewinnt der gespeicherte Wert gegenüber der Kommandozeile; ändern Sie sie also von da an auf der Seite. Ein Proxy-Passwort auf der Kommandozeile können andere Benutzer des Computers in der Prozessliste sehen; legen Sie es stattdessen unter **Einstellungen > Verbindung** fest. Mehr zu diesen Einstellungen: [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall).

### Web-UI

| Option | Bedeutung |
|---|---|
| `--webui [ADDR:]PORT` | Stellt die Web-UI und die qBittorrent-kompatible API bereit. Ohne ADDR lauscht es nur auf `127.0.0.1`. |
| `--webui-user NAME` | Der Anmeldename. Standard `admin`. |
| `--webui-pass PASS` | Das Passwort, als Hash gespeichert. Ohne eines (und ohne gespeichertes) wird beim Start ein vorläufiges Passwort ausgegeben. |
| `--webui-save-root DIR` | Kann mehrfach angegeben werden. Die Web-UI darf dann nur in diesen Ordnern und darunter speichern, verschieben und lesen. Ohne sie kann jeder mit der Anmeldung jeden Ordner verwenden, den das nox-Konto erreichen kann. |
| `--webui-https` | Stellt HTTPS mit einem selbstsignierten Zertifikat bereit, das bei der ersten Verwendung erstellt wird. |
| `--webui-remote-run-program` | Lässt eine Web-UI-Sitzung von einem anderen Computer [Programm ausführen](Einstellungen-im-Überblick#programm-ausführen) ändern, weiterhin nur mit dem aktuellen Passwort. Ohne sie lassen sich diese Zeilen nur von einem Browser auf demselben Computer ändern. Für Server ohne lokalen Browser, etwa einen Container; sie wird nie gespeichert und muss deshalb bei jedem Start auf der Kommandozeile stehen. Ein Reverse-Proxy auf dem Server selbst muss `X-Forwarded-For` hinzufügen, sonst zählt jeder Besucher als Browser auf demselben Computer ([Programm ausführen](Einstellungen-im-Überblick#programm-ausführen)). |
| `--webui-cert FILE [--webui-key FILE]` | Stellt HTTPS mit Ihrem eigenen Zertifikat und Schlüssel bereit (PEM; der Schlüssel nicht verschlüsselt). |

Ein Passwort auf der Kommandozeile ist für andere lokale Benutzer in der Prozessliste sichtbar. Legen Sie es lieber unter **Einstellungen > Web-UI** der Seite fest. Einzelheiten zu HTTPS und den anderen Sicherheitseinstellungen: [Weboberfläche](Weboberfläche#https).

### Sonstiges

| Option | Bedeutung |
|---|---|
| `--import qbittorrent` oder `utorrent` | Nur `run`: fügt beim Start die Torrents von qBittorrent oder µTorrent hinzu, mit ihren Daten, wo sie liegen (zuerst geprüft), Statistiken, Prioritäten, Trackern, Labels und den Tags von qBittorrent. Der andere Client darf nicht laufen; seine Dateien werden nur gelesen. Siehe [Import aus uTorrent und qBittorrent](Import-aus-uTorrent-und-qBittorrent). |
| `--import-dir DIR` | Der Ordner des anderen Clients, wenn er nicht an der üblichen Stelle liegt. |
| `--verbose` (`-v`) | Gibt Tracker- und Peer-Ereignisse aus. |
| `--no-update-check` | Nur `run`: fragt die Website des Projekts nicht nach neueren Releases. |
| `--daemon` | Unix: löst sich in den Hintergrund ([andere Init-Systeme](#weitere-möglichkeiten-es-im-hintergrund-zu-betreiben)). |
| `--pid-file FILE` | Schreibt die Prozess-ID nach FILE und sperrt sie; ein zweites nox mit demselben FILE weigert sich zu starten. |
| `--log FILE` | Hängt alles, was nox ausgibt, an FILE an. |

### Die Update-Prüfung

`run` fragt die Website des Projekts, welche Releases es gibt, 10 Sekunden nach dem Start und danach alle 12 Stunden. Über Ihre Installation wird nichts gesendet und nichts heruntergeladen. Eine neuere Version ist eine Zeile in der Ausgabe und ein Link in der Statusleiste der Web-UI. Schalten Sie es mit `--no-update-check` aus oder in der Web-UI unter **Einstellungen > Allgemein > Privatsphäre**, „Automatisch nach Updates suchen“. „Auf Beta-Versionen aktualisieren“ dort bietet auch Vorabversionen an. Siehe [Rookparcel aktualisieren](Rookparcel-aktualisieren).

## Eine Torrent-Datei erstellen

```sh
rookparcel-nox create ~/Videos/holiday -t udp://tracker.example:1337/announce --magnet
```

| Option | Bedeutung |
|---|---|
| `-o`, `--output FILE` | Wohin der Torrent geschrieben wird. Standard: `<name>.torrent`. |
| `-t`, `--tracker URL` | Ein Tracker; wiederholen Sie die Option für weitere Ebenen (Tiers) oder setzen Sie mehrere durch Kommas getrennt in eine Ebene. |
| `-w`, `--web-seed URL` | Ein Web-Seed; wiederholbar. |
| `--node HOST:PORT` | Ein DHT-Bootstrap-Knoten; wiederholbar. |
| `--private` | Setzt das Private-Flag (kein DHT, kein Peer-Austausch, keine lokale Erkennung). |
| `--piece-size KIB` | Teilgröße in KiB. Standard: automatisch. |
| `--comment TEXT`, `--source TEXT`, `--creator TEXT` | Der Kommentar, das Source-Feld (von privaten Trackern verwendet) und „erstellt von“. |
| `--no-date` | Lässt das Erstellungsdatum weg. |
| `--hybrid`, `--v1`, `--v2` | Hybrid (der Standard), nur v1 oder nur v2. |
| `--pad` | Richtet Dateien mit Füll-Dateien (Padding) an Teilen aus (bei Hybrid und v2 immer an). |
| `--mtime`, `--symlinks`, `--no-attributes`, `--skip-hidden` | Änderungszeiten speichern; Links als Links speichern (Unix); versteckte und ausführbare Attribute weglassen; Namen weglassen, die mit einem Punkt beginnen. |
| `--magnet` | Gibt auch den Magnet-Link aus. |

Das Fenster des Desktop-Programms dafür ist unter [Torrents erstellen](Torrents-erstellen) beschrieben.

## Wo es seine Dateien ablegt

| | Linux und BSD | Windows | macOS |
|---|---|---|---|
| Torrent-Liste und Fortsetzungsdaten (`resume.db`, `metadata.pack`, `dht_state.dat`) | `~/.local/share/rookparcel/nox` | `%APPDATA%\Rookparcel\nox` | `~/Library/Application Support/Rookparcel/nox` |
| Einstellungen (`webui.json`, `ipfilter.dat`, das HTTPS-Zertifikat) | `~/.config/rookparcel/nox` | derselbe Ordner | derselbe Ordner |
| Der Windows-Dienst | | `%ProgramData%\Rookparcel\nox` | |

- Unter Linux werden `$XDG_DATA_HOME` und `$XDG_CONFIG_HOME` verwendet, wenn sie gesetzt sind.
- `webui.json` wird vom laufenden nox neu geschrieben, sobald sich in der Web-UI etwas ändert. Bearbeiten Sie sie nur, während nox gestoppt ist.
- Nur ein nox kann einen Ordner zur Zeit verwenden. Ein zweites stoppt mit „... is in use by another rookparcel-nox“.
- **Portabler Modus:** Mit einer `rookparcel.ini` neben dem Programm liegen beide in einem Ordner `nox` daneben, und Download-Ordner darin werden relativ dazu gespeichert, sodass sich der ganze Ordner verschieben lässt. *Neu in 1.1.49.* Siehe [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt).

## Profile

`--profile NAME` startet ein getrenntes nox mit eigenen Torrents, Einstellungen, eigener Web-UI-Anmeldung und eigenem Dienst. Ein Name hat 1 bis 32 Zeichen: Buchstaben a-z, Ziffern und Bindestriche, beginnend mit einem Buchstaben. Sofern `--port` oder `--listen` nichts anderes sagt, bekommt jedes Profil einen eigenen Listen-Port, aus seinem Namen berechnet und bei jedem Start derselbe.

| | Standardprofil | `--profile work` |
|---|---|---|
| Linux und BSD | `~/.local/share/rookparcel/nox` | `~/.local/share/rookparcel/profiles/work/nox` (und dasselbe unter `~/.config`) |
| Windows | `%APPDATA%\Rookparcel\nox` | `%APPDATA%\Rookparcel\profiles\work\nox` |
| macOS | `~/Library/Application Support/Rookparcel/nox` | `.../Rookparcel/profiles/work/nox` |
| Windows-Dienst | `RookparcelNox` | `RookparcelNox_work` |
| systemd | `rookparcel-nox.service` | `rookparcel-nox@work.service` |

Die Web-UI jedes Profils braucht einen eigenen Port, zum Beispiel `--webui 127.0.0.1:8081`. Ein bereits verwendeter Port stoppt nox mit „Port N ist belegt, vielleicht durch ein anderes Rookparcel-Profil“. *Profile sind neu in 1.1.44.* Siehe [Profile](Profile).

## Der Windows-Dienst

`rookparcel-nox service install` registriert nox als Windows-Dienst mit dem Namen **RookparcelNox** („Rookparcel nox“). Er startet mit Windows und läuft, ohne dass jemand angemeldet ist.

Das ist nicht dasselbe wie **Einstellungen > Web-UI** in `rookparcel.exe`: Das stellt die Web-UI aus dem Fenster bereit, über die Torrents des Fensters und nur, solange das Fenster läuft ([Weboberfläche](Weboberfläche)).

`install`, `uninstall`, `start` und `stop` brauchen eine Eingabeaufforderung als Administrator.

1. Legen Sie `rookparcel-nox.exe` dorthin, wo das Dienstkonto es lesen kann, etwa `C:\Program Files\Rookparcel\`. Eine Kopie unter Ihrem Benutzerordner (Downloads, Dokumente) kann das Dienstkonto nicht öffnen; `install` warnt davor.
2. Installieren Sie ihn mit den `run`-Optionen, mit denen er starten soll:

   ```bat
   "C:\Program Files\Rookparcel\rookparcel-nox.exe" service install --webui 127.0.0.1:8080
   ```

3. Starten Sie ihn mit `rookparcel-nox service start` oder fügen Sie `--now` zu `install` hinzu. Von da an startet er mit Windows.
4. Lesen Sie das vorläufige Web-UI-Passwort aus `%ProgramData%\Rookparcel\nox\nox.log` (aus einer Administrator-Eingabeaufforderung), öffnen Sie die Seite und legen Sie Ihr eigenes Passwort fest. Oder geben Sie `--webui-user` und `--webui-pass` an `install`: Das Passwort wird als Hash in der `webui.json` des Dienstes gespeichert, nie in der Kommandozeile des Dienstes.

Optionen von `install` selbst:

| Option | Bedeutung |
|---|---|
| `--account A` | `LocalService` (der Standard), `NetworkService` oder `LocalSystem`. |
| `--start TYPE` | `delayed` (der Standard: automatisch, nachdem sich der Systemstart beruhigt hat), `auto` oder `manual`. |
| `--replace` | Stoppt und entfernt zuerst einen installierten RookparcelNox; sein Zustand bleibt. |
| `--now` | Startet den Dienst, sobald er installiert ist. |
| `--profile NAME` | Installiert den eigenen Dienst des Profils, `RookparcelNox_NAME`. |

Was er einrichtet:

- **Zustand und Einstellungen** in `%ProgramData%\Rookparcel\nox` (oder `--state-dir`), einem Ordner, den nur SYSTEM, Administratoren und das Dienstkonto öffnen können, da er die Anmeldung der Web-UI enthält.
- **Downloads** in seinem Unterordner `Downloads`, sofern Sie nicht `--save` angeben.
- **Protokoll** dort in `nox.log`, sofern Sie nicht `--log` angeben.
- **Neustarts** nach einem Fehler: nach 5, 30 und 60 Sekunden.
- Beim Herunterfahren des Systems bekommt er Zeit, den Zustand der Torrents zu speichern.

Ordner, in die der Dienst schreibt und die `install` nicht angelegt hat (ein `--save`-Ordner, ein `--move-on-complete`-Ordner), brauchen eine Berechtigung für das Konto, aus einer Administrator-Eingabeaufforderung:

```bat
icacls D:\Torrents /grant "NT AUTHORITY\LocalService:(OI)(CI)M"
```

Eingehende Peer-Verbindungen brauchen eine Windows-Firewall-Regel für das Programm; ein Dienst kann nicht danach fragen:

```bat
netsh advfirewall firewall add rule name="Rookparcel nox" dir=in action=allow program="C:\Program Files\Rookparcel\rookparcel-nox.exe"
```

Die übrigen Dienstbefehle:

| Befehl | Was er tut |
|---|---|
| `rookparcel-nox service start`, `service stop` | Startet oder stoppt ihn und wartet, bis er es getan hat. `sc start RookparcelNox`, `sc stop RookparcelNox` und die Diensteverwaltung tun dasselbe. |
| `rookparcel-nox service status` | Gibt den Zustand und die Kommandozeile des Dienstes aus (kein Administrator nötig). `--all` listet den Dienst jedes Profils auf. |
| `rookparcel-nox service uninstall` | Stoppt und entfernt ihn. Der Zustandsordner und das Protokoll bleiben. `--all` entfernt den Dienst jedes Profils, der dieses Programm ausführt. |
| `rookparcel-nox service debug [OPTIONS]` | Führt die Arbeit des Dienstes in dieser Konsole aus, um die Optionen vor dem Installieren auszuprobieren. |
| `sc control RookparcelNox paramchange` | Lädt den IP-Filter neu und öffnet das Protokoll erneut. |

Um die Optionen zu ändern, installieren Sie erneut mit `--replace`. Um auch den Zustand der Torrents und die Downloads zu löschen, löschen Sie nach dem Deinstallieren `%ProgramData%\Rookparcel`.

`start`, `stop`, `status` und `uninstall` nehmen `--profile NAME` für den Dienst eines Profils. *Dienste je Profil sind neu in 1.1.47.*

**Ohne den Dienst:** `rookparcel-nox run` in einer Eingabeaufforderung funktioniert auch und stoppt sauber bei <kbd>Ctrl</kbd>+<kbd>C</kbd> oder wenn das Fenster geschlossen wird. Die Aufgabenplanung ist ein weiterer Weg: eine Aufgabe „Beim Start“, die `rookparcel-nox.exe run --save D:\Torrents --webui 8080 --log C:\Rookparcel\nox.log` ausführt. `--daemon` funktioniert unter Windows nicht.

## systemd (Linux)

Das Linux-Archiv enthält fertige Units, auch im Ordner [`contrib/systemd`](https://github.com/Balls0fSteel/Rookparcel/tree/main/contrib/systemd) des Repositorys.

**Als Systemdienst**, ausgeführt als Konto `rookparcel`, mit seinem Zustand in `/var/lib/rookparcel-nox`:

```sh
sudo install -m 755 rookparcel-nox /usr/local/bin/
sudo install -D -m 644 rookparcel-nox.sysusers /etc/sysusers.d/rookparcel-nox.conf
sudo systemd-sysusers
sudo install -m 644 rookparcel-nox.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now rookparcel-nox
journalctl -u rookparcel-nox
```

Das Journal zeigt beim ersten Start das vorläufige Web-UI-Passwort. Die Unit stellt die Web-UI auf `127.0.0.1:8080` bereit und lädt nach `/var/lib/rookparcel-nox/downloads` herunter. Um das zu ändern und ihm das Schreiben in einen anderen Ordner zu erlauben:

```ini
# sudo systemctl edit rookparcel-nox
[Service]
ExecStart=
ExecStart=/usr/local/bin/rookparcel-nox run --save /srv/torrents --webui 0.0.0.0:8080
ReadWritePaths=/srv/torrents
```

Der Dienst ist abgeschottet (er kann ohne `ReadWritePaths=` nicht außerhalb seines eigenen Ordners schreiben), sodass ein Download-Ordner an anderer Stelle immer diese Zeile braucht.

**Als Ihr eigener Benutzerdienst**, mit Ihren Dateien in den [oben](#wo-es-seine-dateien-ablegt) aufgeführten Ordnern:

```sh
install -m 755 rookparcel-nox ~/.local/bin/
install -D -m 644 rookparcel-nox.service ~/.config/systemd/user/rookparcel-nox.service
systemctl --user daemon-reload
systemctl --user enable --now rookparcel-nox
loginctl enable-linger "$USER"
```

`loginctl enable-linger` hält ihn nach dem Abmelden am Laufen. Diese Unit lädt nach `~/Downloads` herunter.

**Profile** verwenden die Vorlagen-Units `rookparcel-nox@.service`: `systemctl enable --now rookparcel-nox@work` startet das Profil `work`. Sie haben standardmäßig keine Web-UI, weil zwei sich keinen Port teilen können; geben Sie jedem in `systemctl edit rookparcel-nox@work` einen eigenen.

`systemctl reload rookparcel-nox` lädt den IP-Filter neu. Die Ausgabe geht ins Journal.

## FreeBSD

Das rc.d-Skript liegt im Ordner [`contrib/freebsd`](https://github.com/Balls0fSteel/Rookparcel/tree/main/contrib/freebsd) des Repositorys:

```sh
pw useradd rookparcel -d /var/db/rookparcel-nox -s /usr/sbin/nologin -c "Rookparcel daemon"
install -m 755 rookparcel-nox /usr/local/bin/
install -m 555 rookparcel_nox /usr/local/etc/rc.d/
sysrc rookparcel_nox_enable=YES
sysrc rookparcel_nox_args="--webui 0.0.0.0:8080"
service rookparcel_nox start
```

Es läuft als `rookparcel`, mit seinem Zustand in `/var/db/rookparcel-nox` und dem Protokoll in `/var/log/rookparcel-nox.log`, wo der erste Start das vorläufige Web-UI-Passwort hinschreibt. Einstellungen in `/etc/rc.conf`: `rookparcel_nox_args` (weitere `run`-Optionen; Standard `--webui 127.0.0.1:8080`), `rookparcel_nox_user`, `rookparcel_nox_dir`, `rookparcel_nox_save`, `rookparcel_nox_log`. Verwenden Sie `rookparcel_nox_args`, nicht `rookparcel_nox_flags`. `service rookparcel_nox reload` lädt den IP-Filter neu und öffnet das Protokoll erneut.

Profile: `sysrc rookparcel_nox_profiles="work"` und `sysrc rookparcel_nox_work_args="--webui 127.0.0.1:8081"`; `service rookparcel_nox start work` startet eines, `service rookparcel_nox start` alle.

## Weitere Möglichkeiten, es im Hintergrund zu betreiben

Auf anderen Unix-Systemen:

- `--daemon` löst sich in den Hintergrund und kehrt zurück, sobald nox läuft (Status 0) oder nicht starten konnte (Status 1). Ohne `--log` geht die Ausgabe ab diesem Punkt verloren; lesen Sie also das vorläufige Passwort vorher oder verwenden Sie `--log`.
- `--pid-file FILE` schreibt die Prozess-ID; ein zweites nox mit derselben Datei weigert sich zu starten.
- `--log FILE` hängt die gesamte Ausgabe an FILE an, nur für den Besitzer lesbar, da sie das vorläufige Passwort enthalten kann. Ein `SIGHUP` öffnet sie erneut, sodass die Protokollrotation funktioniert.

## Beenden und neu laden

| Was | Wirkung |
|---|---|
| `q` und <kbd>Enter</kbd>, <kbd>Ctrl</kbd>+<kbd>C</kbd>, `SIGTERM` | Ein sauberer Stopp: Torrent-Zustand gespeichert, die Port-Zuordnungen am Router entfernt. |
| Ein zweites <kbd>Ctrl</kbd>+<kbd>C</kbd> oder `SIGTERM` während dieses Stopps | Beendet sofort, ohne zu speichern. |
| `SIGHUP` (Windows-Dienst: `sc control RookparcelNox paramchange`) | Lädt die Liste des IP-Filters neu und öffnet die `--log`-Datei erneut. `webui.json` wird nicht neu gelesen. Stammt die Liste von einer Adresse, wird die zuletzt heruntergeladene Kopie neu geladen; es wird nichts heruntergeladen. |
| Windows: das Konsolenfenster schließen, abmelden, herunterfahren | Ein sauberer Stopp. |

## Siehe auch

- [Weboberfläche](Weboberfläche): die Seite, die nox bereitstellt.
- [Fernsteuerung](Fernsteuerung): nox vom Fenster des Desktop-Programms aus steuern.
- [Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr): nox als ihr Download-Client.
- Die vollständige technische Beschreibung: [DAEMON.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/DAEMON.md).
