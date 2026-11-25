*Dies ist eine Übersetzung der englischen Seite [Installing Rookparcel](Installing-Rookparcel). Maßgeblich ist die englische Seite.*

# Rookparcel installieren

Diese Seite listet die Downloads für jede Plattform auf, erklärt, wie Sie jeden beim ersten Mal installieren und starten, und wie Sie Rookparcel wieder entfernen. Nach dem ersten Start lesen Sie [Erster Start und der Einrichtungsassistent](Erster-Start).

## Wo Sie es herunterladen

Jede Version steht auf der [Releases-Seite](https://github.com/Balls0fSteel/Rookparcel/releases). Die neueste veröffentlichte Version ist 1.2.79. Jede Version hat Hinweise mit den Schritten für den ersten Start je Plattform und eine Datei `SHA256SUMS` mit der Prüfsumme jedes Downloads.

| Plattform | Datei | Was darin ist |
|---|---|---|
| Windows 10 / 11, x64, Installer | `rookparcel-<version>-windows-x64-setup.exe` | `rookparcel.exe` (das Fenster) und `rookparcel-nox.exe` (der Daemon ohne Fenster), installiert mit einem Startmenü-Eintrag und einem Deinstallationsprogramm |
| Windows 10 / 11, x64, portabel | `rookparcel-<version>-windows-x64.zip` | dieselben beiden Programme; nichts zu installieren |
| Linux-Desktop, x86_64 oder aarch64 | `Rookparcel-<version>-<arch>.AppImage` | das GTK-4-Fenster mit mitgelieferten Bibliotheken (benötigt glibc 2.39 oder neuer: Ubuntu 24.04, Fedora 40, Debian 13 oder später) |
| Linux-Desktop, Flatpak | `Rookparcel-<version>-<arch>.flatpak` | das GTK-4-Fenster auf der GNOME-51-Runtime von Flathub |
| Linux, einfache Programmdateien | `rookparcel-<version>-linux-<arch>.tar.gz` | `rookparcel-gtk`, `rookparcel-nox`, der Desktop-Eintrag und die systemd-Units; sie verwenden GTK 4 (4.12 oder neuer) und OpenSSL 3 des Systems |
| Linux-Server, ohne Fenster | `rookparcel-nox-<version>-<arch>.AppImage` | `rookparcel-nox` mit mitgeliefertem OpenSSL |
| macOS, Apple Silicon oder Intel | `Rookparcel-<version>-macos.dmg` | `Rookparcel.app`, das native macOS-Fenster, eine App für beide Mac-Arten (macOS 11 Big Sur oder neuer) |
| macOS, ohne Fenster | `rookparcel-nox-<version>-macos-<arch>.tar.gz` | `rookparcel-nox` für Apple Silicon (`arm64`, macOS 11 oder neuer) und Intel (`x86_64`, macOS 10.14 oder neuer) |

Das Fenster unter Linux und den BSDs ist die GTK-Version; unter macOS ist es eine native Mac-App. Beide funktionieren wie die Windows-Version; wo sie sich unterscheiden, sagen es die Wiki-Seiten. Zum Betrieb des Daemons ohne Fenster siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

*Die native macOS-App ist neu in 1.2.19. Versionen davor haben stattdessen `Rookparcel-<version>-arm64.dmg`: das GTK-Fenster für Apple Silicon, macOS 15 oder neuer.*

## Windows

### Installer oder ZIP

- **Der Installer** legt Rookparcel in Ihr Startmenü, kann es mit `.torrent`-Dateien und `magnet:`-Links verknüpfen, fügt unter Einstellungen > Apps einen Eintrag zum Deinstallieren hinzu und lässt Rookparcel sich selbst aktualisieren (siehe [Rookparcel aktualisieren](Rookparcel-aktualisieren)).
- **Die ZIP-Datei** braucht keine Installation: Entpacken Sie sie irgendwo und starten Sie `rookparcel.exe`. Sie kann im portablen Modus laufen, mit Einstellungen und Torrents im eigenen Ordner (siehe [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt#portabler-modus)). Sie aktualisiert sich nicht selbst; eine neuere Version ist eine neue ZIP-Datei.

Beide sind dieselben Programme. Rookparcel braucht 64-Bit-Windows 10 oder neuer. Windows 11 auf Arm führt die x64-Version aus.

### Den Installer ausführen

1. Führen Sie `rookparcel-<version>-windows-x64-setup.exe` aus.
2. Windows SmartScreen zeigt möglicherweise „Der Computer wurde durch Windows geschützt“, weil das Setup nicht codesigniert ist. Klicken Sie auf **Weitere Informationen** und dann auf **Trotzdem ausführen**.
3. Wählen Sie die Sprache des Setups, lesen Sie die Willkommensseite und akzeptieren Sie die Lizenz (die PolyForm Noncommercial License 1.0.0).
4. **Benutzer auswählen:**
   - **Nur für mich installieren** (die Voreinstellung) braucht keine Administratorrechte. Rookparcel kommt nach `%LOCALAPPDATA%\Programs\Rookparcel`.
   - **Für alle Benutzer dieses Computers installieren** installiert nach `C:\Program Files\Rookparcel`. Windows verlangt Administratorrechte, und die Abfrage nennt „Unbekannter Herausgeber“. Lehnen Sie sie ab, sind Sie wieder auf dieser Seite.
5. **Komponenten:** Rookparcel selbst, **Verknüpfung auf dem Desktop**, **Mit .torrent-Dateien verknüpfen** und **Mit magnet:-Links verknüpfen**. Die letzten drei sind bei einer neuen Installation angehakt.
6. Wählen Sie den Ordner oder behalten Sie den angezeigten und lassen Sie installieren.
7. Lassen Sie auf der letzten Seite „Rookparcel ausführen“ angehakt, um es zu starten.

Die Verknüpfungen werden für den Benutzer geschrieben, der das Setup ausführt. Andere Benutzer einer Installation für alle können sie unter **Optionen > Einstellungen > Allgemein** einschalten.

Das Setup lehnt einen Ordner ab, der eine Datei `rookparcel.ini` enthält: Dieser Ordner ist ein portables Rookparcel. Wählen Sie einen anderen Ordner.

### Über eine ältere Version installieren

Führen Sie das neuere Setup aus. Es findet das installierte Rookparcel, überspringt die Seiten für Benutzer und Ordner und installiert darüber, im selben Ordner. Ihre Einstellungen und Torrents bleiben, wo sie sind. Bei einem Upgrade sind die Kästchen für die Verknüpfungen nicht angehakt; die Verknüpfungen bleiben dann, wie sie sind.

Läuft Rookparcel, bittet Sie das Setup, es zu beenden (**Datei > Beenden**) und auf **Wiederholen** zu klicken. Läuft `rookparcel-nox.exe` aus demselben Ordner, zum Beispiel als nox-Dienst von Rookparcel, bittet es Sie, ihn zuerst zu beenden. In einer Eingabeaufforderung als Administrator:

```
sc stop RookparcelNox
```

Starten Sie den Dienst nach dem Upgrade wieder. Eine installierte Kopie kann all das auch selbst erledigen: siehe [Rookparcel aktualisieren](Rookparcel-aktualisieren).

### Stille Installation

Das Setup versteht diese Schalter, für Skripte und Administratoren:

| Schalter | Wirkung |
|---|---|
| `/S` | still: keine Seiten, am Ende wird nichts gestartet |
| `/D=<Ordner>` | der Installationsordner; muss das letzte Argument sein, ohne Anführungszeichen |
| `/CURRENTUSER`, `/ALLUSERS` | für mich oder für alle, ohne die Benutzerseite. `/ALLUSERS` ohne Administratorrechte fragt über die Windows-Abfrage, auch im stillen Modus |
| `/DESKTOPSHORTCUT` | die Verknüpfung auf dem Desktop anlegen (eine stille Installation legt sonst keine an) |
| `/ASSOCIATE=torrent,magnet` | die Verknüpfungen: `torrent`, `magnet` oder beide |

Zum Beispiel in einer Eingabeaufforderung als Administrator:

```
rookparcel-1.2.79-windows-x64-setup.exe /S /ALLUSERS /ASSOCIATE=torrent,magnet
```

### Die ZIP-Datei

1. Entpacken Sie `rookparcel-<version>-windows-x64.zip` in einen beliebigen Ordner, in den Sie schreiben dürfen.
2. Starten Sie `rookparcel.exe`. SmartScreen warnt beim ersten Mal, wie oben: **Weitere Informationen**, dann **Trotzdem ausführen**.

Einstellungen und Torrents kommen nach `%APPDATA%\Rookparcel`, an denselben Ort wie bei einer installierten Kopie. Um sie stattdessen im Ordner der ZIP-Datei zu behalten, legen Sie vor dem ersten Start eine leere Datei namens `rookparcel.ini` neben `rookparcel.exe`. Siehe [Portabler Modus](Wo-Rookparcel-seine-Dateien-ablegt#portabler-modus).

Um `.torrent`-Dateien und `magnet:`-Links mit der ZIP-Kopie zu öffnen, haken Sie die Verknüpfungen unter **Optionen > Einstellungen > Allgemein** an. Sie nennen dann die `rookparcel.exe` dieser Kopie und funktionieren nicht mehr, wenn Sie ihren Ordner verschieben oder löschen.

### Einen Download prüfen

Die `SHA256SUMS` jeder Version listet den SHA-256 jeder Datei. In PowerShell:

```
Get-FileHash .\rookparcel-1.2.79-windows-x64-setup.exe -Algorithm SHA256
```

Der Hash sollte mit der Zeile der Datei in `SHA256SUMS` übereinstimmen.

## Linux

### AppImage

1. Laden Sie `Rookparcel-<version>-x86_64.AppImage` herunter (oder `-aarch64` auf ARM).
2. Machen Sie es ausführbar und starten Sie es:

   ```bash
   chmod +x Rookparcel-1.2.79-x86_64.AppImage
   ./Rookparcel-1.2.79-x86_64.AppImage
   ```

3. Ist FUSE 2 nicht auf dem System, starten Sie es mit zusätzlichem `--appimage-extract-and-run`.

Das AppImage braucht glibc 2.39 oder neuer (Ubuntu 24.04, Fedora 40, Debian 13 oder später). Die Einstellungen kommen nach `~/.config/rookparcel/gtk`, die Torrents nach `~/.local/share/rookparcel/gtk`. Eine Datei `rookparcel.ini` neben der `.AppImage`-Datei macht es stattdessen portabel (*neu in 1.1.49*).

### Flatpak

Das Bundle enthält die Anwendung; die Runtime kommt von Flathub. Fügen Sie Flathub einmal hinzu und installieren Sie dann das Bundle:

```bash
flatpak remote-add --if-not-exists --user flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install --user Rookparcel-1.2.79-x86_64.flatpak
flatpak run io.github.balls0fsteel.Rookparcel
```

Danach steht es in Ihrem Anwendungsmenü. Das Flatpak darf in Ihren Home-Ordner schreiben; für einen Download-Ordner auf einem anderen Laufwerk geben Sie ihm einmal Zugriff, zum Beispiel mit `flatpak override --user --filesystem=/mnt/data io.github.balls0fsteel.Rookparcel` (mit Ihrem Ordner). Das Flatpak hat keine eigene Update-Prüfung und keinen portablen Modus. Seine Verknüpfungen für `.torrent` und `magnet:` sind Sache des Desktops: Wählen Sie Rookparcel in den Einstellungen Ihres Desktops als Standardprogramm.

### Einfache Programmdateien (tar.gz)

`rookparcel-<version>-linux-<arch>.tar.gz` enthält `rookparcel-gtk`, `rookparcel-nox`, den Desktop-Eintrag und die systemd-Units (`systemd/system`, `systemd/user`). Die Programme verwenden GTK 4 (4.12 oder neuer) und OpenSSL 3 des Systems, wie Ubuntu 24.04 sie hat; sie laufen also auf Distributionen von etwa diesem Alter oder neuer.

Entpacken Sie es und starten Sie `rookparcel-gtk`. Eine Datei `rookparcel.ini` neben der Programmdatei macht es portabel (*neu in 1.1.49*). Zu den systemd-Units siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

### Server ohne Fenster

`rookparcel-nox-<version>-<arch>.AppImage` ist der Daemon allein, mit mitgeliefertem OpenSSL; er bleibt aus den Menüs heraus. Siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

## macOS

1. Öffnen Sie `Rookparcel-<version>-macos.dmg` und ziehen Sie `Rookparcel.app` nach „Programme“.
2. Die App ist ad hoc signiert und nicht notarisiert, deshalb weist Gatekeeper sie beim ersten Mal ab. Erlauben Sie sie einmal:
   - **macOS 15 Sequoia und neuer:** Doppelklicken Sie die App und klicken Sie in der Warnung auf **Fertig**. Öffnen Sie **Systemeinstellungen > Datenschutz & Sicherheit**, scrollen Sie zu **Sicherheit** und klicken Sie auf **Trotzdem öffnen** (die Schaltfläche ist etwa eine Stunde lang da). Bestätigen Sie mit **Trotzdem öffnen** und Ihrem Passwort oder Touch ID.
   - **macOS 11 bis 14:** Klicken Sie die App mit gedrückter Control-Taste (oder mit der rechten Maustaste) an, wählen Sie **Öffnen** und klicken Sie dann auf **Öffnen**.
3. Oder, auch wenn macOS sagt, die App „ist beschädigt“, entfernen Sie das Quarantäne-Flag im Terminal:

   ```bash
   xattr -dr com.apple.quarantine /Applications/Rookparcel.app
   ```

Das Disk-Image enthält diese Schritte in `How to open Rookparcel.html`, und die [Installationsanleitung](https://balls0fsteel.github.io/Rookparcel/install.html#macos) der Website zeigt sie mit Bildern.

Die App läuft unter macOS 11 Big Sur oder neuer, auf Macs mit Apple Silicon und Intel gleichermaßen: Das eine `Rookparcel.app` enthält beides. Einstellungen und Torrents kommen nach `~/Library/Application Support/Rookparcel/mac`.

### Von der GTK-Version umsteigen

Frühere Versionen hatten eine GTK-Version von `Rookparcel.app` für Apple Silicon. Ersetzen Sie sie durch das neue `Rookparcel.app` in „Programme“. Beim ersten Start benennt die neue App den Ordner `gtk` der alten App in `~/Library/Application Support/Rookparcel` in `mac` um, ebenso den jedes Profils, sodass Ihre Einstellungen, Torrents und Profile erhalten bleiben. Das geschieht einmal und nur, solange noch kein Ordner `mac` da ist. Beenden Sie zuerst die alte App: Solange sie läuft, sagt die neue „Ein anderes Rookparcel verwendet diesen Ordner bereits“ und schließt sich. Siehe [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt#macos).

Ein Eintrag „Rookparcel beim Systemstart starten“, den die alte App angelegt hat, funktioniert weiter. Ein portabler Ordner wird so verwendet, wie er ist.

Für den Daemon ohne Fenster entpacken Sie `rookparcel-nox-<version>-macos-<arch>.tar.gz` und führen `rookparcel-nox` im Terminal aus ([rookparcel-nox betreiben](rookparcel-nox-betreiben)).

## Deinstallation

### Windows-Installer

1. Beenden Sie Rookparcel (**Datei > Beenden**).
2. Öffnen Sie **Einstellungen > Apps**, suchen Sie **Rookparcel** und wählen Sie **Deinstallieren**.
3. Das Deinstallationsprogramm hat eine Option, **Einstellungen und Torrent-Liste**, standardmäßig nicht angehakt. Haken Sie sie an, um auch zu entfernen, was Rookparcel in `%APPDATA%\Rookparcel` ablegt: die Einstellungen, die Torrent-Liste mit ihrem Fortschritt, RSS-Feeds, Fernverbindungen, den Standard-IP-Filter, die Dateien von nox und die Profile.

Das Deinstallationsprogramm entfernt die Verknüpfungen, den Eintrag unter „Standard-Apps“ und den Eintrag „Rookparcel beim Systemstart starten“, die die `rookparcel.exe` dieser Installation nennen. Die einer anderen Kopie, zum Beispiel einer portablen, bleiben. Es rührt Ihre heruntergeladenen Dateien nie an.

Läuft der nox-Dienst von Rookparcel aus dem Installationsordner, bittet Sie das Deinstallationsprogramm, ihn zuerst zu entfernen. In einer Eingabeaufforderung als Administrator:

```
"%LOCALAPPDATA%\Programs\Rookparcel\rookparcel-nox.exe" service uninstall --all
```

(mit Ihrem eigenen Installationsordner). Eine stille Deinstallation: `"%LOCALAPPDATA%\Programs\Rookparcel\uninstall.exe" /S`.

Die Einstellungen gehören jedem Windows-Benutzer einzeln. Beim Deinstallieren einer Installation für alle werden sie nur für den Benutzer aufgeräumt, der das Deinstallationsprogramm ausführt.

### Windows-ZIP

1. Wenn Sie Verknüpfungen oder „Rookparcel beim Systemstart starten“ angehakt haben, nehmen Sie die Haken zuerst unter **Optionen > Einstellungen > Allgemein** wieder heraus.
2. Beenden Sie Rookparcel und löschen Sie seinen Ordner.
3. Löschen Sie auch `%APPDATA%\Rookparcel`, außer die Kopie war portabel (dann war alles in ihrem Ordner).

### Linux und macOS

- **AppImage oder tar.gz:** Löschen Sie die Datei oder den Ordner. Um die Einstellungen und die Torrent-Liste zu entfernen, löschen Sie `~/.config/rookparcel` und `~/.local/share/rookparcel`.
- **Flatpak:** `flatpak uninstall --user io.github.balls0fsteel.Rookparcel`.
- **macOS:** Ziehen Sie `Rookparcel.app` in den Papierkorb. Die Einstellungen und die Torrent-Liste liegen in `~/Library/Application Support/Rookparcel`.

Heruntergeladene Dateien bleiben in jedem Fall, wo sie sind. Die vollständige Liste der Ordner steht unter [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt).
