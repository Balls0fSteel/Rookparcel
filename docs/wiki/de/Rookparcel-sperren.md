*Dies ist eine Übersetzung der englischen Seite [Locking Rookparcel](Locking-Rookparcel). Maßgeblich ist die englische Seite.*

# Rookparcel sperren

Diese Seite erklärt, wie Sie die Fenster von Rookparcel mit einer PIN sperren, damit niemand sonst an Ihrem Computer Ihre Torrent-Liste sehen kann, und wie Sie eine vergessene PIN entfernen. *Neu in 1.2.54.*

Die Sperre dient der Privatsphäre, nicht der Sicherheit. Sie blendet die Fenster aus; sie verschlüsselt nichts. Wer die Einstellungsdatei von Rookparcel bearbeiten oder Ihren Download-Ordner öffnen kann, kommt an ihr vorbei. Verwenden Sie dafür nicht die PIN Ihrer Bankkarte.

## PIN festlegen und sperren

1. Öffnen Sie **Optionen > Einstellungen** und wählen Sie **Erweitert > UI-Sperre** (auf einem Mac: **Rookparcel > Einstellungen**, dann **UI-Sperre**).
2. Klicken Sie auf **PIN festlegen...**.
3. Geben Sie in **Neue PIN (4 bis 6 Ziffern)** eine PIN aus 4 bis 6 Ziffern ein, und noch einmal in **Neue PIN wiederholen**.
4. Klicken Sie auf **OK**. Die PIN wird sofort gespeichert; Sie müssen in den Einstellungen nicht auf **OK** klicken.

*Vor 1.2.63 wurde die PIN unter **Optionen > Sperr-PIN** festgelegt.*

Zum Sperren wählen Sie **Datei > Rookparcel sperren** oder drücken <kbd>Ctrl</kbd>+<kbd>L</kbd>. Sobald eine PIN festgelegt ist, hat auch das Menü des Infobereich-Symbols **Rookparcel sperren**. Ist noch keine PIN festgelegt, fragt **Rookparcel sperren** zuerst nach einer und sperrt dann.

Das Sperren blendet alle Rookparcel-Fenster im Infobereich aus: das Hauptfenster und jeden Dialog, der offen war. Ihre Torrents laden und seeden weiter.

Um die PIN zu ändern, klicken Sie auf derselben Seite auf **PIN ändern...**, geben die aktuelle PIN ein und dann die neue zweimal. **PIN entfernen...** fragt nach der aktuellen PIN und entfernt sie; **Rookparcel sperren** fragt dann vor dem Sperren nach einer neuen.

## Die Sperre ausschalten

Wenn Sie die Sperre nie wollen, nehmen Sie auf der Seite **UI-Sperre** das Häkchen bei **Sperren von Rookparcel mit einer PIN erlauben** weg. Ist eine PIN festgelegt, fragt Rookparcel zuerst danach und entfernt sie dann. Solange die Sperre ausgeschaltet ist, fehlen **Datei > Rookparcel sperren**, das zugehörige Tastenkürzel und **Rookparcel sperren** im Menü des Infobereich-Symbols. Setzen Sie das Häkchen erneut, um sie zurückzubekommen.

## Ein gesperrtes Rookparcel öffnen

Klicken Sie auf das Infobereich-Symbol, wählen Sie **Rookparcel anzeigen** im Menü des Symbols oder starten Sie Rookparcel erneut. Ein Fenster mit dem Titel **Rookparcel ist gesperrt** fragt nach der PIN. Mit der richtigen PIN kommen die Fenster, die offen waren, so zurück, wie sie waren. **Abbrechen** lässt Rookparcel im Infobereich gesperrt.

Nach der fünften falschen PIN in Folge lässt Rookparcel 30 Sekunden verstreichen, bevor es eine weitere annimmt, und danach jede weitere falsche PIN ebenso. Die richtige PIN beendet das.

Rookparcel bleibt gesperrt, wenn Sie es beenden. Beim nächsten Start fragt es sofort nach der PIN, ohne sein Hauptfenster zu zeigen. Bei der Anmeldung mit **Minimiert starten** gestartet, wartet es im Infobereich, ohne zu fragen.

## Solange Rookparcel gesperrt ist

- Das Menü des Infobereich-Symbols bietet nur **Rookparcel anzeigen** und **Beenden**. **Beenden** funktioniert ohne die PIN.
- Die Benachrichtigung „Download abgeschlossen“ erscheint nicht, weil sie den Torrent nennt.
- Meldungen, die sich von selbst öffnen würden, warten, bis Sie entsperren: Fehlermeldungen, ein Update-Angebot und das Fenster **Neuen Torrent hinzufügen** eines Magnet-Links, dessen Dateien eingetroffen sind.
- Eine `.torrent`-Datei oder ein Magnet-Link, die Sie im gesperrten Zustand öffnen, fragt zuerst nach der PIN. Klicken Sie auf **Abbrechen**, wird sie nicht hinzugefügt.
- Der Countdown von **Automatisch herunterfahren** erscheint weiterhin, sodass Sie ihn stoppen können.
- Die [Weboberfläche](Weboberfläche) ist nicht gesperrt. Sie hat ihren eigenen Benutzernamen und ihr eigenes Passwort. Ist **Authentifizierung für Clients auf localhost umgehen** eingeschaltet, kann ein Browser auf demselben Computer Ihre Torrents trotzdem sehen.

## PIN vergessen

Sie können eine vergessene PIN beim Start von Rookparcel entfernen. Rookparcel fragt **Die Sperr-PIN entfernen? Rookparcel öffnet sich dann, ohne danach zu fragen.** Antworten Sie mit **Ja** (**OK** unter Linux, **Entfernen** unter macOS), und es öffnet sich ohne die PIN. Ihre Torrents und Ihre übrigen Einstellungen bleiben, wie sie sind.

**Windows:** Halten Sie <kbd>Ctrl</kbd> gedrückt, während Sie Rookparcel starten, zum Beispiel beim Klick auf seinen Eintrag im Startmenü oder beim Doppelklick auf sein Desktop-Symbol. Läuft Rookparcel bereits, fragt die laufende Kopie.

**macOS:** Beenden Sie Rookparcel (**Rookparcel beenden** im Menü des Menüleisten-Symbols) und halten Sie dann <kbd>⌥</kbd> (Wahltaste) gedrückt, während Sie es öffnen. Läuft Rookparcel, fragt es auch, wenn Sie bei gedrückter <kbd>⌥</kbd>-Taste auf sein Dock-Symbol klicken.

**Linux und die BSDs:** Starten Sie es in einem Terminal mit `--reset-lock`. Läuft Rookparcel bereits, fragt die laufende Kopie.

| Kopie | Befehl |
|---|---|
| Einfache Programmdateien (tar.gz) | `rookparcel-gtk --reset-lock` |
| AppImage | `./Rookparcel-<version>-x86_64.AppImage --reset-lock` |
| Flatpak | `flatpak run io.github.balls0fsteel.Rookparcel --reset-lock` |

`--reset-lock` funktioniert auch unter Windows und macOS: `rookparcel.exe --reset-lock` oder auf einem Mac `/Applications/Rookparcel.app/Contents/MacOS/Rookparcel --reset-lock`. Läuft Rookparcel auf dem Mac schon, kann sich seine Frage hinter dem vordersten Fenster öffnen: Suchen Sie danach.

Mit einem [Profil](Profile) fügen Sie dessen Schalter hinzu: `rookparcel.exe --profile NAME --reset-lock`.

**Von Hand:** Beenden Sie Rookparcel, öffnen Sie seine `rookparcel.ini` in einem Texteditor, löschen Sie die Zeilen, die mit `ui.lock_pin=` und `ui.locked=` beginnen, und speichern Sie. [Wo Rookparcel seine Dateien ablegt](Wo-Rookparcel-seine-Dateien-ablegt) sagt, wo die Datei liegt.

## Profile und Fernfenster

Jedes [Profil](Profile) hat seine eigene PIN oder keine. Ebenso jedes Fenster, das mit einem entfernten Rookparcel verbunden ist ([Fernsteuerung](Fernsteuerung)), da es seine eigenen Einstellungen führt.

## GTK-Version (Linux, BSD)

Die Sperre funktioniert wie unter Windows. <kbd>Ctrl</kbd>+<kbd>L</kbd> sperrt aus jedem Rookparcel-Fenster, nicht nur aus dem Hauptfenster: Einstellungen, Eigenschaften, Neuen Torrent erstellen, Neuen Torrent hinzufügen, dem RSS-Downloader, den Suchfenstern und den anderen Dialogen (nicht den PIN-Fenstern). *Neu in 1.2.76; vorher tat es nichts, solange einer dieser Dialoge offen war.* Verschwindet der Infobereich, während Rookparcel gesperrt ist (etwa weil das Panel neu startet), erscheint das Fenster **Rookparcel ist gesperrt**, damit die Fenster nicht ohne Weg zurück ausgeblendet bleiben. *Neu in 1.2.76.*

Auf Desktops ohne Infobereich (einfaches GNOME ohne die AppIndicator-Erweiterung) gibt es keinen Ort, an dem man die Fenster ausblenden könnte; dort tritt das Fenster **Rookparcel ist gesperrt** an ihre Stelle. Es hat keine Schaltfläche **Abbrechen**, und sein Schließen beendet Rookparcel, wie das Schließen des Hauptfensters dort.

## macOS

**Datei > Rookparcel sperren** ist <kbd>⌘</kbd>+<kbd>L</kbd>. Solange Rookparcel gesperrt ist:

- hat die Menüleiste nur **Rookparcel anzeigen**, **Rookparcel ausblenden** und **Rookparcel beenden** im Menü **Rookparcel** sowie das Menü **Bearbeiten**;
- bietet das Menü des Menüleisten-Symbols **Rookparcel anzeigen** und **Rookparcel beenden** an, das Menü des Dock-Symbols **Rookparcel anzeigen**;
- ist **Rookparcel beenden** ausgegraut, solange das PIN-Fenster offen ist: Klicken Sie zuerst auf **Abbrechen**.
