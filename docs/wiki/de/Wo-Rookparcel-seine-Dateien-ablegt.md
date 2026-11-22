*Dies ist eine Übersetzung der englischen Seite [Where Rookparcel keeps its files](Where-Rookparcel-keeps-its-files). Maßgeblich ist die englische Seite.*

# Wo Rookparcel seine Dateien ablegt

Diese Seite sagt, wo Rookparcel auf jeder Plattform seine Einstellungen und seine Torrent-Liste ablegt, wie der portable Modus sie stattdessen neben dem Programm hält und wie Sie sie sichern, auf einen anderen Computer übertragen oder von vorn beginnen. Ihre Downloads gehören nicht dazu: Sie liegen dort, wo Sie sie gespeichert haben.

## Den Ordner finden

**Hilfe > Über Rookparcel** (unter macOS **Rookparcel > Über Rookparcel**) zeigt unten den Pfad der Einstellungsdatei. **Datei > Profile > Profilordner öffnen** öffnet den Ordner (*neu in 1.1.48*).

## Windows

Das Windows-Fenster legt alles in einem Ordner ab: `%APPDATA%\Rookparcel`, meist `C:\Users\<Sie>\AppData\Roaming\Rookparcel`. Installierte und ZIP-Kopien verwenden denselben Ordner, außer die ZIP-Kopie ist portabel.

| Datei oder Ordner | Was darin ist |
|---|---|
| `rookparcel.ini` | die Einstellungen |
| `state\` | die Torrent-Liste und der Fortschritt jedes Torrents |
| `rss.dat` | RSS-Feeds und Filter |
| `remote.ini`, `remote\` | Fernverbindungen und die Einstellungen jedes Fernfensters |
| `ipfilter.dat` | die Standard-IP-Filterdatei, falls Sie eine verwenden |
| `ipfilter-url.dat`, `ipfilter-url.txt` | die Sperrliste des IP-Filters, wie zuletzt von ihrer Webadresse geladen, und eine Notiz, woher und wann, falls Sie einen verwenden (*neu in 1.2.63*) |
| `nox\` | die Dateien von `rookparcel-nox`, wenn es aus einer Konsole lief |
| `search-helper\` | der Suchhelfer, falls Sie ihn geholt haben ([Torrents suchen unter Windows](Torrents-suchen-unter-Windows)) |
| `profiles\` | die anderen [Profile](Profile), jedes ein Ordner, aufgebaut wie dieser |
| `rookparcel.lock` | markiert den Ordner als in Benutzung, solange Rookparcel läuft |

Der Windows-Dienst von `rookparcel-nox` legt seine Dateien stattdessen in `%ProgramData%\Rookparcel\nox` ab. Siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

## Linux und die BSDs

Die GTK-Version folgt den XDG-Ordnern:

| Was | Ordner |
|---|---|
| Einstellungen: `rookparcel.ini`, `rss.dat`, `remote.ini`, `remote/`, `ipfilter.dat` und die geladene Liste des IP-Filters (`ipfilter-url.dat`, `ipfilter-url.txt`), falls Sie sie verwenden | `$XDG_CONFIG_HOME/rookparcel/gtk`, sonst `~/.config/rookparcel/gtk` |
| Torrent-Liste: `resume.db`, `resume.log`, `metadata.pack`, `dht_state.dat`, `stats.dat` (die Gesamtsummen von Hilfe > Statistik) | `$XDG_DATA_HOME/rookparcel/gtk`, sonst `~/.local/share/rookparcel/gtk` |

`rookparcel.lock` im Ordner der Torrent-Liste markiert ihn als in Benutzung. `rookparcel-nox` verwendet in beiden Pfaden `nox` statt `gtk`.

Das Flatpak legt seine Ordner unter `~/.var/app/io.github.balls0fsteel.Rookparcel/` ab. Unter systemd sind die Ordner die des Dienstes selbst; siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

## macOS

Sowohl die Einstellungen als auch die Torrent-Liste liegen in `~/Library/Application Support/Rookparcel/mac`, aufgebaut wie der Windows-Ordner oben; `rookparcel-nox` verwendet `~/Library/Application Support/Rookparcel/nox`. Ein [Profil](Profile) `work` liegt in `~/Library/Application Support/Rookparcel/profiles/work/mac`.

Die GTK-Version von `Rookparcel.app` in früheren Versionen verwendete an denselben Stellen einen Ordner `gtk`. Der erste Start der nativen App benennt jeden Ordner `gtk` in `mac` um (den des Standardprofils und den jedes Profils) und korrigiert die Pfade dorthin in ihren Einstellungen, sodass alles erhalten bleibt. Das geschieht einmal: nur dort, wo noch kein Ordner `mac` da ist, und nicht, solange die alte App den Ordner benutzt. Portable Ordner bleiben, wie sie sind.

*Die native macOS-App und ihr Ordner `mac` sind neu in 1.2.19; Versionen davor verwenden `gtk`.*

## Portabler Modus

Im portablen Modus legt Rookparcel seine Einstellungen und seine Torrent-Liste im eigenen Ordner ab, wie µTorrent es mit `settings.dat` neben `uTorrent.exe` tat. Der Ordner kann auf einem USB-Stick oder in einem synchronisierten Ordner liegen.

**Zum Einschalten** legen Sie vor dem ersten Start eine Datei namens `rookparcel.ini` an die richtige Stelle. Sie darf leer sein. Es muss eine Datei sein: Ein Ordner mit diesem Namen zählt nicht.

| Kopie | Wohin `rookparcel.ini` kommt |
|---|---|
| Windows-ZIP | neben `rookparcel.exe` |
| Linux- oder BSD-tar.gz | neben `rookparcel-gtk` (oder `rookparcel-nox`) |
| Linux-AppImage | neben die `.AppImage`-Datei |
| macOS | neben `Rookparcel.app`, nie hinein; neben `rookparcel-nox` für den Daemon |
| Flatpak, Windows-Installer | kein portabler Modus |

*Der portable Modus unter Windows ist in jeder Version. Unter Linux und den BSDs ist er neu in 1.1.49, unter macOS in 1.1.50.*

Der Aufbau in einem portablen Ordner ist derselbe wie bei `%APPDATA%\Rookparcel` oben: die Einstellungen und `state` neben dem Programm, `nox` für `rookparcel-nox`, `profiles` für andere Profile.

### Was sich im portablen Modus ändert

- **Der erste Start lädt in den Ordner herunter.** Neue Downloads kommen in den Ordner `Downloads` darin, und Rookparcel wählt einen zufälligen Listen-Port, damit es nicht mit einem anderen Rookparcel auf 6881 kollidiert. *Neu in 1.1.45 und 1.1.49.*
- **Pfade im Ordner werden relativ dazu gespeichert.** Speicherordner, die Ordner der Einstellungen, die IP-Filterdatei und die RSS-Speicherordner, die im portablen Ordner liegen, werden als `./Downloads/...` geschrieben. Der Ordner kann dann auf einen anderen Laufwerksbuchstaben oder auf einen anderen Computer wandern, und die Torrents laufen ohne erneute Prüfung weiter. Pfade außerhalb des Ordners bleiben, wie sie sind. *Neu in 1.1.49.* Von einer älteren Version geschriebene Pfade werden beim nächsten Speichern umgewandelt.
- **Nichts wird ins System geschrieben, wenn Sie es nicht verlangen.** Verknüpfungen, Start mit dem System und Dienste werden nur gesetzt, wenn Sie sie in den Einstellungen anhaken. Sie nennen dann das Programm dieser Kopie und brechen, wenn der Ordner verschoben wird.
- **Es aktualisiert sich nicht selbst.** Der Update-Hinweis einer portablen Kopie öffnet die Download-Seite. Siehe [Rookparcel aktualisieren](Rookparcel-aktualisieren).

Ein portabler Ordner, der von 1.1.49 oder später geschrieben wurde, braucht 1.1.49 oder später: Eine ältere Version würde die relativen Pfade falsch lesen.

Kann in den Ordner nicht geschrieben werden (ein schreibgeschützter Stick, `C:\Program Files`), hält Rookparcel mit „Dieses portable Rookparcel kann nicht in seinen Ordner schreiben“ an, statt Ihre Einstellungen woanders abzulegen. Verschieben Sie den Ordner oder entfernen Sie `rookparcel.ini`, um die normalen Ordner zu verwenden.

Unter macOS läuft ein `Rookparcel.app`, das nach dem Herunterladen direkt aus dem Ordner „Downloads“ gestartet wird, aus einer temporären Kopie, in der eine `rookparcel.ini` daneben nicht gesehen wird. Rookparcel sagt das. Verschieben Sie `Rookparcel.app` mit dem Finder in den gewünschten Ordner und starten Sie es erneut.

## Ein Ordner, ein Rookparcel

Zwei Rookparcel verwenden nie gleichzeitig einen Ordner. Starten Sie Rookparcel ein zweites Mal, holt es das laufende Fenster nach vorn und übergibt ihm, was Sie geöffnet haben. Eine Kopie, die den Ordner aus einer anderen Sitzung oder von einem anderen Computer über eine Netzwerkfreigabe in Benutzung findet, zeigt „Ein anderes Rookparcel verwendet diesen Ordner bereits“ und schließt sich. Beenden Sie zuerst das andere. (*Die Prüfung über Sitzungen und Computer hinweg ist neu in 1.1.43.*)

Um absichtlich zwei Rookparcel nebeneinander laufen zu lassen, verwenden Sie [Profile](Profile).

## Sichern

1. Beenden Sie Rookparcel (**Datei > Beenden**; unter macOS **Rookparcel > Rookparcel beenden**), damit alles gespeichert ist.
2. Kopieren Sie den ganzen Ordner: `%APPDATA%\Rookparcel` unter Windows, sowohl `~/.config/rookparcel` als auch `~/.local/share/rookparcel` unter Linux, `~/Library/Application Support/Rookparcel` unter macOS oder den portablen Ordner.

Die Sicherung enthält die Einstellungen, die Torrent-Liste mit dem Fortschritt jedes Torrents, die RSS-Feeds und jedes Profil. Die heruntergeladenen Dateien enthält sie nicht.

## Auf einen anderen Computer übertragen

1. Beenden Sie Rookparcel auf dem alten Computer und kopieren Sie den Ordner, wie bei einer Sicherung.
2. Installieren Sie Rookparcel auf dem neuen Computer, starten Sie es aber noch nicht.
3. Legen Sie den Ordner dort an dieselbe Stelle.
4. Kopieren oder verschieben Sie Ihre Downloads.
5. Starten Sie Rookparcel.

Außerhalb des portablen Modus speichert die Torrent-Liste vollständige Pfade. Legen Sie die Downloads auf dem neuen Computer unter denselben Pfaden ab (unter Windows derselbe Laufwerksbuchstabe und dieselben Ordner), sonst finden die Torrents ihre Dateien nicht.

Ein portabler Ordner ist einfacher: Kopieren Sie den ganzen Ordner, Downloads eingeschlossen, und starten Sie das Programm darin. Seine Pfade sind relativ, ein anderer Laufwerksbuchstabe oder Benutzername spielt also keine Rolle.

## Von vorn beginnen

So setzen Sie Rookparcel vollständig zurück:

1. Beenden Sie Rookparcel.
2. Benennen Sie den Ordner um (zum Beispiel in `Rookparcel.old`), damit Sie zurückgehen können.
3. Starten Sie Rookparcel. Es startet wie eine neue Installation, mit dem [Einrichtungsassistenten](Erster-Start) und einer leeren Torrent-Liste.

Ihre heruntergeladenen Dateien bleiben, wo sie sind; fügen Sie ihre Torrents erneut hinzu, um sie zu seeden. Auch das Windows-Deinstallationsprogramm kann den Ordner löschen: seine Option **Einstellungen und Torrent-Liste** ([Rookparcel installieren](Rookparcel-installieren#deinstallation)).
