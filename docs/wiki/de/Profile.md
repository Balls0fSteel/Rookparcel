*Dies ist eine Übersetzung der englischen Seite [Profiles](Profiles). Maßgeblich ist die englische Seite.*

# Profile

Ein Profil ist ein ganzes eigenes Rookparcel: mit eigenen Einstellungen, eigener Torrent-Liste, eigenen RSS-Feeds, Ports und einem eigenen Fenster. Diese Seite behandelt, wie Sie Profile anlegen und starten, was jedes getrennt hält und den Schalter `--profile` für Verknüpfungen und die Kommandozeile.

*Neu in 1.2.0.* Bis 1.1.40 war der einzige Weg, zwei Rookparcel laufen zu lassen, zwei portable Kopien in getrennten Ordnern ([portabler Modus](Wo-Rookparcel-seine-Dateien-ablegt#portabler-modus)).

## Wofür Profile gut sind

- Ein Rookparcel, das nur Ihr VPN verwendet, neben einem normalen: Jedes Profil hat sein eigenes **Nur dieses Netzwerk verwenden**, seinen eigenen Proxy und eigene Ports ([Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall)).
- Private Tracker getrennt von öffentlichen Torrents, jeweils mit eigenen Seeding-Zielen und Suchquellen.
- Eine Einstellung ausprobieren, ohne Ihre echte Einrichtung anzufassen: `--profile test`.

Mehrere Profile können gleichzeitig laufen, jedes in einem eigenen Fenster. Sie teilen nichts: Ein Torrent in einem Profil ist in einem anderen nicht zu sehen.

## Das Standardprofil

Ohne Profilschalter führt Rookparcel das **Standardprofil** aus: die Ordner, die unter [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt) beschrieben sind, genau wie vor den Profilen. Nichts wird verschoben, wenn Sie Profile zu benutzen beginnen.

Ein benanntes Profil liegt in einem Ordner `profiles` im Standardprofil:

| | Standardprofil | Profil `work` |
|---|---|---|
| Windows | `%APPDATA%\Rookparcel` | `%APPDATA%\Rookparcel\profiles\work` |
| Linux, BSD | `~/.config/rookparcel/gtk`, `~/.local/share/rookparcel/gtk` | `~/.config/rookparcel/profiles/work/gtk`, `~/.local/share/rookparcel/profiles/work/gtk` |
| macOS | `~/Library/Application Support/Rookparcel/mac` | `~/Library/Application Support/Rookparcel/profiles/work/mac` |
| portabler Ordner | der Ordner | `profiles\work` darin |

## Profilnamen

Ein Name hat 1 bis 32 Zeichen: Buchstaben a-z, Ziffern und Bindestriche (-), beginnend mit einem Buchstaben. Großbuchstaben werden als Kleinbuchstaben genommen, `Work` und `work` sind also dasselbe Profil. `default` bedeutet das Standardprofil. Jeder andere Name wird mit einer entsprechenden Meldung abgelehnt.

## Profile über das Menü anlegen und starten

**Datei > Profile** listet das Standardprofil (als **Standard**) und jedes benannte Profil auf. Das Profil, das dieses Fenster ausführt, ist angehakt.

| Eintrag | Was er tut |
|---|---|
| der Name eines Profils | startet dieses Profil in einem eigenen Fenster. Läuft es schon, kommt sein Fenster nach vorn |
| **Neues Profil...** | fragt nach einem Namen („Name (Buchstaben a-z, Ziffern und Bindestriche):“), legt das Profil an und startet es |
| **Verknüpfung erstellen...** | Windows: legt eine Verknüpfung auf Ihren Desktop, die dieses Profil startet und nach ihm benannt ist, zum Beispiel `Rookparcel (work).lnk`. Linux: nimmt das Profil in Ihr Anwendungsmenü auf |
| **Profilordner öffnen** | öffnet den Ordner dieses Profils |

Ein neues Profil beginnt leer, wie eine neue Installation, und öffnet sich mit dem [Einrichtungsassistenten](Erster-Start).

## Die Fenster auseinanderhalten

Das Fenster eines benannten Profils zeigt seinen Namen in der Titelleiste und im Tooltip des Infobereich-Symbols: `Rookparcel 1.2.63 [work]`. Das Standardprofil zeigt keinen Namen. Unter Windows hat jedes Profil seine eigene Gruppe von Schaltflächen in der Taskleiste, Sie können sie also getrennt anheften.

## Was jedes Profil getrennt hält

**Ports.** Ein neues Profil wählt bei seinem ersten Start einen zufälligen Listen-Port zwischen 10000 und 60000, damit es nicht mit dem Standardprofil um 6881 streitet. Das Standardprofil behält seinen Port. Sind zwei Profile trotzdem auf denselben Port eingestellt, zeigt die Statusleiste „Port N belegt“; klicken Sie darauf, um **Optionen > Einstellungen > Verbindung** zu öffnen. Siehe [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall).

Jedes Profil bildet seinen eigenen Port am Router ab (UPnP, NAT-PMP). Die Zuordnung ist nach dem Profil benannt, zum Beispiel `Rookparcel (work)`, sodass Sie sie in der Liste des Routers unterscheiden können.

**Die Weboberfläche.** Die Weboberfläche jedes Profils braucht einen eigenen Port. Schalten Sie im Windows-Fenster die Weboberfläche in einem Profil ein, schlägt **Optionen > Einstellungen > Web-UI** den ersten freien Port ab 8080 vor. Ist der Port trotzdem belegt, heißt es „Port N ist belegt, vielleicht durch ein anderes Rookparcel-Profil“. Siehe [Weboberfläche](Weboberfläche).

**`.torrent`-Dateien und `magnet:`-Links.** Nur ein Programm kann sie öffnen, sie gehören also immer einem Profil. Wenn Sie in **Optionen > Einstellungen > Allgemein** eines Profils **Mit .torrent-Dateien verknüpfen** oder **Mit magnet:-Links verknüpfen** anhaken, bekommt dieses Profil sie. Unter Windows sagt die Seite es, wenn ein anderes Profil sie hat: „Das Rookparcel-Profil "work" öffnet sie jetzt; ein Häkchen lässt stattdessen dieses Profil sie öffnen.“ Ein Doppelklick auf eine `.torrent`-Datei öffnet sie dann in dem Profil, dem die Verknüpfung gehört, und startet dieses Profil, falls es nicht läuft.

Die Kästchen für die Verknüpfungen im Windows-Installer geben sie dem Standardprofil.

**Start mit dem System.** **Rookparcel beim Systemstart starten** wird pro Profil gesetzt, jedes Profil kann also für sich starten. Unter Windows werden sie als `Rookparcel` und `Rookparcel (work)` aufgeführt.

**Fernfenster.** Ein Fenster, das ein entferntes `rookparcel-nox` steuert ([Fernsteuerung](Fernsteuerung)), behält seine Einstellungen in dem Profil, von dem aus es geöffnet wurde.

## Ein Profil von der Kommandozeile starten

Fügen Sie `--profile NAME` (oder `--profile=NAME`) zum Befehl hinzu:

```
rookparcel.exe --profile work
rookparcel-gtk --profile work
```

- Es darf an beliebiger Stelle unter den Argumenten stehen, und der Name wird nie als Torrent genommen: `rookparcel.exe --profile work file.torrent` fügt `file.torrent` zum Profil `work` hinzu.
- `--profile default` oder kein Schalter bedeutet das Standardprofil.
- Existiert das Profil noch nicht, fragt das Fenster zuerst: „Das Profil "work" existiert noch nicht. Erstellen?“, und zeigt den Ordner, den es anlegen wird. **Nein** schließt, ohne etwas anzulegen, damit ein vertippter Name kein leeres Rookparcel startet.
- Es lässt sich mit den anderen Schaltern kombinieren, zum Beispiel `--minimized`, um im Infobereich zu starten, oder `--remote`, um sich mit einem entfernten `rookparcel-nox` zu verbinden.

Um eine Windows-Verknüpfung von Hand anzulegen, fügen Sie `--profile work` nach `rookparcel.exe` im Feld **Ziel** der Verknüpfung hinzu. **Datei > Profile > Verknüpfung erstellen...** erledigt das für Sie.

## Profile und portabler Modus

Auch ein portables Rookparcel hat Profile, im Ordner `profiles` im portablen Ordner. Sie reisen mit ihm. Siehe [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt#portabler-modus).

## Unterschiede in GTK

- **Datei > Profile** funktioniert genauso. Das aktuelle Profil wird als ausgewählter Eintrag gezeigt.
- **Verknüpfung erstellen...** nimmt das Profil in Ihr Anwendungsmenü auf („Rookparcel (work)“). Das Profil hat einen eigenen Desktop-Eintrag, sodass das Dock und die Taskleiste sein Fenster unter seinem eigenen Namen und Symbol zeigen.
- Im **Flatpak** funktionieren Profile, aber nur das Standardprofil kann mit dem System starten.

## Unterschiede unter macOS

- **Datei > Profile** listet die Profile, **Neues Profil...** und **Profilordner öffnen** auf; es gibt kein **Verknüpfung erstellen...**.
- Das Öffnen einer `.torrent`-Datei oder eines `magnet:`-Links erreicht immer das Standardprofil: macOS wählt die App, nicht ein Profil. Der Start mit dem System funktioniert pro Profil.

## Profile von rookparcel-nox

`rookparcel-nox` nimmt ebenfalls `--profile NAME`, um mehrere Daemons auf einem Computer laufen zu lassen, jeden mit eigenen Torrents, eigener Weboberfläche und eigenem Dienst. Ein Daemon-Profil fragt nicht, bevor es angelegt wird. Sein Listen-Port ergibt sich aus seinem Namen und bleibt so bei jedem Start gleich. Ein GUI-Profil und ein `rookparcel-nox`-Profil mit demselben Namen sind getrennte Programme, die sich nur den Ordner `profiles/<name>` teilen. Siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

## Ein Profil entfernen

1. Schließen Sie das Fenster des Profils (**Datei > Beenden**).
2. Wenn es die Verknüpfungen besitzt oder mit dem System startet, nehmen Sie diese Haken zuerst in seinen Einstellungen heraus oder geben Sie sie einem anderen Profil.
3. Löschen Sie seinen Ordner, `profiles\<name>`. **Datei > Profile > Profilordner öffnen** zeigt, wo er liegt.

Die Downloads des Profils bleiben, wo sie sind.
