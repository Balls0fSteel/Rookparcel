*Dies ist eine Übersetzung der englischen Seite [Tray, notifications and startup](Tray-notifications-and-startup). Maßgeblich ist die englische Seite.*

# Infobereich, Benachrichtigungen und Autostart

Diese Seite behandelt das Symbol von Rookparcel im Infobereich, das Schließen und Minimieren in den Infobereich, die Benachrichtigung, wenn ein Download abgeschlossen ist, den Start von Rookparcel bei der Anmeldung und wie Rookparcel zum Programm wird, das `.torrent`-Dateien und `magnet:`-Links öffnet.

## Das Symbol im Infobereich (Windows)

Solange Rookparcel läuft, befindet sich sein Symbol im Infobereich der Taskleiste. Zeigen Sie darauf, sehen Sie die Version und die aktuellen Download- und Upload-Geschwindigkeiten.

- Ein Linksklick (oder ein Doppelklick) blendet das Hauptfenster aus oder wieder ein.
- Ein Rechtsklick öffnet das Menü des Symbols:

| Eintrag | Was er bewirkt |
|---|---|
| **Rookparcel ausblenden** / **Rookparcel anzeigen** | Blendet das Hauptfenster aus oder wieder ein |
| **Torrent hinzufügen...** | Öffnet eine `.torrent`-Datei, wie es **Datei > Torrent hinzufügen...** tut |
| **Torrent von URL hinzufügen...** | Fügt einen Magnet-Link oder eine Webadresse hinzu |
| **Alle Torrents starten** | Startet jeden Torrent in der Liste |
| **Alle Torrents stoppen** | Stoppt jeden Torrent in der Liste |
| **Rookparcel sperren** | Blendet alle Rookparcel-Fenster aus, bis die Sperr-PIN eingegeben wird; erscheint, sobald eine PIN festgelegt ist |
| **Beenden** | Beendet Rookparcel |

Ist Rookparcel gesperrt, hat das Menü nur **Rookparcel anzeigen**, das nach der PIN fragt, und **Beenden**. Siehe [Rookparcel sperren](Rookparcel-sperren).

## In den Infobereich schließen und minimieren

Zwei Einstellungen bestimmen, was mit dem Fenster geschieht:

| Einstellung | Vorgabe | Was sie bewirkt |
|---|---|---|
| **Beim Schließen in den Infobereich** | Ein | Die Schließen-Schaltfläche des Fensters (und <kbd>Alt</kbd>+<kbd>F4</kbd>) blendet Rookparcel im Infobereich aus, statt es zu beenden; die Übertragungen laufen weiter. |
| **Beim Minimieren in den Infobereich** | Aus | Das Minimieren blendet das Fenster im Infobereich aus, statt es in der Taskleiste zu belassen. |

Sie stellen sie unter **Optionen > Einstellungen > Oberfläche** bei **Infobereich** ein, oder Sie setzen die Häkchen direkt im Menü **Optionen** (**Beim Schließen in den Infobereich** und **Beim Minimieren in den Infobereich**).

Um Rookparcel zu beenden, während **Beim Schließen in den Infobereich** eingeschaltet ist, verwenden Sie **Datei > Beenden** oder **Beenden** im Menü des Infobereich-Symbols. Rookparcel stoppt dann seine Torrents und speichert deren Fortschritt. Dauert das länger als einen Moment (zum Beispiel, solange Dateien noch auf ein anderes Laufwerk verschoben werden), zeigt ein Fenster, worauf es wartet, mit einer Schaltfläche **Jetzt beenden**.

Die Weboberfläche, die das Windows-Fenster bereitstellt, läuft nur, solange Rookparcel läuft; das Schließen in den Infobereich hält sie also verfügbar. Siehe [Weboberfläche](Weboberfläche).

## Benachrichtigung, wenn ein Download abgeschlossen ist

Ist **Sprechblasen im Infobereich anzeigen** eingeschaltet (die Vorgabe), zeigt Rookparcel jedes Mal, wenn ein Download fertig ist, eine Benachrichtigung „Download abgeschlossen“ mit dem Namen des Torrents. Sie schalten sie unter **Optionen > Einstellungen > Oberfläche** aus. Solange Rookparcel [gesperrt](Rookparcel-sperren) ist, erscheint die Benachrichtigung nicht.

Ist ein Update verfügbar, sagt Rookparcel das stattdessen in der Statusleiste; siehe [Rookparcel aktualisieren](Rookparcel-aktualisieren).

## Die Geschwindigkeit in der Titelleiste anzeigen

**Aktuelle Geschwindigkeit in der Titelleiste anzeigen** (Einstellungen > Oberfläche, standardmäßig aus) setzt die Download- und Upload-Geschwindigkeit an den Anfang des Fenstertitels, sodass Sie sie auf der Schaltfläche in der Taskleiste lesen können.

## Rookparcel bei der Anmeldung starten

1. Öffnen Sie **Optionen > Einstellungen > Allgemein**.
2. Setzen Sie das Häkchen bei **Rookparcel beim Systemstart starten**.
3. Damit es im Infobereich startet, ohne sein Fenster zu zeigen, setzen Sie zusätzlich das Häkchen bei **Minimiert starten**.
4. Klicken Sie auf **OK**.

Unter Windows ist das ein Eintrag in den Autostart-Programmen Ihres Benutzerkontos. Administratorrechte sind nicht nötig, und auch eine portable Kopie kann ihn verwenden. Verschieben Sie eine portable Kopie in einen anderen Ordner, setzen Sie das Häkchen erneut: Es ist nur gesetzt, solange der Eintrag genau diese Kopie startet.

Mit [Profilen](Profile) hat jedes Profil seinen eigenen Autostart-Eintrag, sodass Sie bei der Anmeldung mehrere starten können. *Einträge pro Profil sind neu in 1.1.46.*

Der [Einrichtungsassistent](Erster-Start) bietet dieselbe Wahl auf seiner letzten Seite an.

## .torrent-Dateien und Magnet-Links mit Rookparcel öffnen

1. Öffnen Sie **Optionen > Einstellungen > Allgemein**.
2. Setzen Sie die Häkchen bei **Mit .torrent-Dateien verknüpfen** und **Mit magnet:-Links verknüpfen**.
3. Klicken Sie auf **OK**.

Ein Doppelklick auf eine `.torrent`-Datei oder ein Klick auf einen Magnet-Link in Ihrem Browser übergibt sie dann an Rookparcel. Läuft Rookparcel bereits, übernimmt die laufende Kopie sie. Ist **Ein Fenster mit den Dateien im Torrent anzeigen** eingeschaltet, öffnet sich das Fenster „Neuen Torrent hinzufügen“; ist **Programmfenster aktivieren** eingeschaltet, kommt außerdem das Hauptfenster in den Vordergrund. Siehe [Torrents hinzufügen](Torrents-hinzufügen).

Gut zu wissen unter Windows:

- Die Einstellungen gelten nur für Ihr Benutzerkonto und brauchen keine Administratorrechte. Auch das Installationsprogramm kann sie setzen.
- Haben Sie in Windows selbst ein anderes Programm gewählt („Immer diese App verwenden“ oder Einstellungen > Standard-Apps), behält Windows diese Wahl, und nur Sie können sie dort ändern. Rookparcel sagt dann, welches Programm Windows noch verwendet, zum Beispiel „Windows öffnet .torrent-Dateien noch mit ...“. Klicken Sie auf **Standard-Apps...**, um die passende Stelle in den Windows-Einstellungen zu öffnen, und wählen Sie Rookparcel.
- Mit Profilen kann nur ein Profil `.torrent`-Dateien und Magnet-Links öffnen. Setzen Sie die Häkchen in einem Profil, übernimmt es sie vom anderen; die Seite nennt das Profil, das sie jetzt hat.
- **Verknüpfung beim Start prüfen** (standardmäßig aus) lässt Rookparcel bei jedem Start fragen, ob es die Verknüpfungen zurückholen soll, wenn ein anderes Programm sie hat. Es fragt nicht, wenn Rookparcel bei der Anmeldung minimiert startet.

## GTK-Version (Linux, BSD)

**Der Infobereich.** Rookparcel zeigt ein Symbol im Infobereich auf Desktops, die einen haben: KDE Plasma, Xfce, LXQt, Cinnamon, MATE, waybar und GNOME mit der AppIndicator-Erweiterung. Sein Menü hat dieselben Einträge wie unter Windows. Verschwindet der Infobereich (etwa weil das Panel neu startet), während Rookparcel darin ausgeblendet ist, zeigt Rookparcel das Hauptfenster oder, wenn es gesperrt ist, das PIN-Fenster, damit nichts ohne Weg zurück bleibt. *Neu in 1.2.76.* Auf einfachem GNOME ohne diese Erweiterung gibt es keinen Infobereich: Die Infobereich-Felder erscheinen nicht in den Einstellungen, und das Schließen des Fensters beendet Rookparcel. <kbd>Ctrl</kbd>+<kbd>Q</kbd> beendet es aus dem Fenster heraus.

**Beim Minimieren in den Infobereich** setzt voraus, dass der Desktop meldet, dass das Fenster minimiert wurde; wo er das nicht tut (manche Wayland-Desktops), bleibt das Minimieren ein gewöhnliches Minimieren.

**Benachrichtigungen** laufen über den eigenen Benachrichtigungsdienst des Desktops, mit demselben Text „Download abgeschlossen“.

**Start bei der Anmeldung.** **Rookparcel beim Systemstart starten** schreibt einen Autostart-Eintrag in den Ordner `~/.config/autostart` Ihres Benutzers. In einem Flatpak fragt es stattdessen den Desktop über dessen Hintergrund-Berechtigung, und der Desktop bittet Sie womöglich um Erlaubnis; dort kann nur das Standardprofil bei der Anmeldung starten.

**Verknüpfungen.** Die Felder machen Rookparcel in den Einstellungen Ihres Benutzers zum Standardprogramm für `.torrent`-Dateien und `magnet:`-Links. Nehmen Sie ein Häkchen weg, geht der Typ an den Standard des Systems zurück; ein früher verwendetes Programm wird dadurch nicht wiederhergestellt. Verschieben Sie ein AppImage, setzen Sie die Häkchen erneut. In einem Flatpak sind die Felder ausgegraut: Der Desktop bietet Rookparcel für beide Typen bereits an, und das Standardprogramm wählen Sie in den Einstellungen Ihres Desktops.

## macOS

**Die Menüleiste.** Rookparcel setzt sein Symbol in die Menüleiste, wo Windows den Infobereich hat. Zeigen Sie darauf, sehen Sie die Geschwindigkeiten. Ein Klick blendet das Fenster aus oder wieder ein; ein Rechtsklick oder Control-Klick öffnet das Menü: **Rookparcel ausblenden** / **Rookparcel anzeigen**, **Rookparcel sperren** (sobald eine Sperr-PIN festgelegt ist), **Torrent hinzufügen...**, **Torrent von URL hinzufügen...**, **Alle Torrents starten**, **Alle Torrents stoppen** und **Rookparcel beenden**. Das Menü des Dock-Symbols hat dieselben Einträge, ohne **Rookparcel beenden**, das das Dock selbst hinzufügt. Solange Rookparcel [gesperrt](Rookparcel-sperren) ist, bieten die Menüs nur **Rookparcel anzeigen** an (und **Rookparcel beenden** im Menü des Menüleisten-Symbols).

**Das Fenster schließen.** Ist **Beim Schließen in den Infobereich** eingeschaltet (die Vorgabe), blendet die Schließen-Schaltfläche des Fensters (<kbd>⌘</kbd>+<kbd>W</kbd>) das Fenster aus, und Rookparcel läuft weiter. Klicken Sie auf das Dock-Symbol oder das Menüleisten-Symbol, um es zurückzuholen. Zum Beenden wählen Sie **Rookparcel > Rookparcel beenden** (<kbd>⌘</kbd>+<kbd>Q</kbd>) oder **Rookparcel beenden** im Menü des Menüleisten-Symbols. **Beim Minimieren in den Infobereich** blendet ein minimiertes Fenster auf dieselbe Weise aus.

**Benachrichtigungen** für abgeschlossene Downloads sind macOS-Mitteilungen, mit demselben Text „Download abgeschlossen“. macOS fragt womöglich einmal, ob Rookparcel Mitteilungen zeigen darf; erscheinen keine, erlauben Sie Rookparcel unter **Systemeinstellungen > Mitteilungen**.

**Start bei der Anmeldung.** **Rookparcel beim Systemstart starten** fügt für dieses Profil ein Anmeldeobjekt hinzu (einen Launch Agent in `~/Library/LaunchAgents`); **Minimiert starten** startet es ausgeblendet. Ab macOS 13 listet es **Systemeinstellungen > Allgemein > Anmeldeobjekte** auf. Ein Anmeldeobjekt, das die ältere GTK-Version der App angelegt hat, funktioniert weiter.

**Verknüpfungen.** Die Felder funktionieren nur, wenn Rookparcel als `Rookparcel.app` läuft, und nur im Standardprofil. Das Ausschalten geschieht im Finder unter **Informationen > Öffnen mit**.

Die Einstellungen unter macOS haben keine Schaltfläche **OK**: Jede Änderung wird sofort übernommen.

*Die native macOS-App ist neu in 1.2.19.*

## Weboberfläche und rookparcel-nox

Die Weboberfläche hat keinen Infobereich, keine Benachrichtigungen und keine Autostart-Einstellungen: Ihre Einstellungen zeigen die Felder für Verknüpfungen und Autostart ausgegraut, mit dem Hinweis, dass sie zum Desktop-Client gehören. Um `rookparcel-nox` mit dem Computer zu starten, betreiben Sie es als Dienst; siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).
