*Dies ist eine Übersetzung der englischen Seite [Updating Rookparcel](Updating-Rookparcel). Maßgeblich ist die englische Seite.*

# Rookparcel aktualisieren

Rookparcel sucht von selbst nach neuen Versionen und sagt Ihnen, wenn eine erschienen ist. Diese Seite behandelt die Prüfung, wie sich eine installierte Windows-Kopie selbst aktualisiert, das Aktualisieren der ZIP-, Linux- und macOS-Kopien von Hand, Beta-Versionen, das Ausschalten der Prüfung und `rookparcel-nox`.

## Die Update-Prüfung

Etwa zehn Sekunden nach dem Start von Rookparcel und danach alle 12 Stunden, solange es läuft, fragt es die Website des Projekts, welche Version die neueste ist. Das ist eine kleine HTTPS-Anfrage nach einer kurzen Liste von Versionsnummern. Nichts über Ihren Computer, Ihre Torrents oder Ihre Einstellungen wird gesendet, und ohne Ihr Ja wird nichts heruntergeladen oder installiert. Verwendet Rookparcel einen Proxy (**Optionen > Einstellungen > Verbindung**), läuft auch die Prüfung darüber.

Ist eine neuere Version erschienen:

- Ein Fragefenster erscheint, einmal für jede neue Version.
- Danach bleibt ein Hinweis in der Statusleiste, bis Sie darauf klicken: „Rookparcel 1.2.80 ist verfügbar - zum Herunterladen hier klicken“.
- Der Reiter Protokoll hat ebenfalls eine Zeile mit dem Link.

Um sofort zu prüfen, wählen Sie **Hilfe > Nach Updates suchen** (unter macOS **Rookparcel > Nach Updates suchen**). Es antwortet in beiden Fällen, mit der neueren Version oder mit „Sie haben die neueste Version“.

## Eine installierte Kopie unter Windows aktualisiert sich selbst

Ein Rookparcel, das mit dem Windows-Setup installiert wurde, kann die neue Version selbst installieren. *Neu in 1.1.40.*

1. Die Frage lautet „Rookparcel 1.2.80 ist verfügbar. Sie haben 1.2.79. Jetzt herunterladen und installieren?“. Klicken Sie auf **Herunterladen und installieren**. (**Downloadseite öffnen** öffnet stattdessen die Release-Seite in Ihrem Browser; **Abbrechen** tut nichts.)
2. Ein kleines Fenster zeigt den Fortschritt des Downloads. Rookparcel prüft das heruntergeladene Setup gegen die `SHA256SUMS` der Version, bevor es es verwendet.
3. Rookparcel fragt: „Zum Installieren wird Rookparcel jetzt beendet und nach dem Setup neu gestartet. Jetzt beenden und installieren?“. Klicken Sie auf **Ja**.
4. Rookparcel speichert alles und schließt sich. Das Setup installiert die neue Version still in denselben Ordner und startet Rookparcel erneut.

Ihre Einstellungen, Torrents und Verknüpfungen bleiben, wie sie waren. Schlägt die Installation fehl, bleibt die alte Version und startet wieder.

Bei einer Installation für alle (in `C:\Program Files`) verlangt Windows Administratorrechte, während das Setup läuft.

**Aktualisieren von 1.1.17 oder älter:** Diese Versionen können keine Updates installieren; ihre Frage öffnet die Download-Seite. Laden Sie das neue Setup herunter und führen Sie es einmal von Hand aus, wie unter [Rookparcel installieren](Rookparcel-installieren#über-eine-ältere-version-installieren). Von da an aktualisiert sich Rookparcel selbst.

## Von Hand aktualisieren

### Windows-Installer

Laden Sie das neue `rookparcel-<version>-windows-x64-setup.exe` herunter und führen Sie es aus. Es findet das installierte Rookparcel und installiert darüber, behält den Ordner, den Modus (für Sie oder für alle), die Einstellungen und die Torrents. Beenden Sie Rookparcel zuerst, sonst bittet das Setup Sie darum. Siehe [Rookparcel installieren](Rookparcel-installieren#über-eine-ältere-version-installieren).

### Windows-ZIP

Eine ZIP-Kopie aktualisiert sich nicht selbst. Ihre Frage bietet an, die Download-Seite zu öffnen.

1. Laden Sie die neue ZIP-Datei herunter.
2. Beenden Sie Rookparcel (**Datei > Beenden**).
3. Entpacken Sie die neuen `rookparcel.exe` und `rookparcel-nox.exe` über die alten.
4. Starten Sie Rookparcel.

Lassen Sie `rookparcel.ini` im Ordner, wenn die Kopie portabel ist. Die Einstellungen und Torrents sind nicht in der ZIP-Datei, es wird also nichts von Ihnen überschrieben.

### Linux und macOS

In der GTK-Version und der macOS-App öffnet die Frage die Release-Seite.

- **AppImage:** Laden Sie das neue `.AppImage` herunter, machen Sie es ausführbar (`chmod +x`) und starten Sie es anstelle des alten. Löschen Sie die alte Datei. Einstellungen und Torrents sind nicht im AppImage, sie bleiben also erhalten. Für ein portables AppImage legen Sie die neue Datei neben `rookparcel.ini`.
- **tar.gz:** Beenden Sie Rookparcel und entpacken Sie die neuen Programme über die alten.
- **Flatpak:** Das Flatpak hat keine Update-Prüfung. Laden Sie das neuere Bundle von der [Releases-Seite](https://github.com/Balls0fSteel/Rookparcel/releases) herunter und installieren Sie es wie unter [Rookparcel installieren](Rookparcel-installieren#flatpak).
- **macOS:** Beenden Sie Rookparcel (**Rookparcel > Rookparcel beenden**), ziehen Sie dann das neue `Rookparcel.app` aus `Rookparcel-<version>-macos.dmg` nach „Programme“ und ersetzen Sie das alte. Weist Gatekeeper die neue Kopie ab, öffnen Sie sie wie unter [Rookparcel installieren](Rookparcel-installieren#macos). Dasselbe `Rookparcel.app` läuft auf Macs mit Apple Silicon und mit Intel.
- **macOS, von der GTK-Version:** Frühere Versionen hatten ein GTK-`Rookparcel.app` für Apple Silicon. Ersetzen Sie es auf dieselbe Weise; beim ersten Start übernimmt die neue App die Einstellungen und Torrents der alten. Siehe [Rookparcel installieren](Rookparcel-installieren#von-der-gtk-version-umsteigen).

## Beta-Versionen

Rookparcel veröffentlicht vor einer Version manchmal eine Vorabversion (einen Release Candidate). Damit Ihnen auch diese angeboten werden, haken Sie **Auf Beta-Versionen aktualisieren** unter **Optionen > Einstellungen > Allgemein** an, unter **Privatsphäre**. Es ist standardmäßig aus.

Eine Vorabversion erfährt immer von der nächsten Vorabversion, was auch immer das Kästchen sagt.

## Die Prüfung ausschalten

Nehmen Sie den Haken bei **Automatisch nach Updates suchen** unter **Optionen > Einstellungen > Allgemein**, unter **Privatsphäre**, heraus. Rookparcel fragt die Website dann nie von selbst. **Hilfe > Nach Updates suchen** funktioniert weiterhin, wenn Sie es wählen.

Einige Builds haben gar keine Update-Prüfung: das Flatpak und Pakete, die für eine Paketverwaltung gebaut sind, die das Aktualisieren übernimmt.

## rookparcel-nox

`rookparcel-nox` prüft auf dieselbe Weise, zehn Sekunden nachdem `run` gestartet ist, und alle 12 Stunden. *Neu in 1.1.17.* Es lädt nie etwas herunter oder installiert etwas. Ist eine neuere Version erschienen:

- gibt es eine Zeile aus, zum Beispiel `* Rookparcel 1.2.0 is available: https://github.com/Balls0fSteel/Rookparcel/releases/tag/v1.2.0`;
- zeigt seine Weboberfläche die Zeile in ihrem Protokoll und einen Link in der Statusleiste der Seite.

Zum Ausschalten starten Sie es mit `--no-update-check` oder nehmen den Haken bei **Automatisch nach Updates suchen** in **Allgemein > Privatsphäre** der Weboberfläche heraus. **Auf Beta-Versionen aktualisieren** ist dort ebenfalls.

Um den Daemon zu aktualisieren, beenden Sie ihn, ersetzen `rookparcel-nox` durch das neue und starten ihn wieder. Für den Windows-Dienst beenden Sie ihn zuerst in einer Eingabeaufforderung als Administrator (`sc stop RookparcelNox`) und starten ihn danach wieder; der Windows-Installer bittet darum, wenn er den Dienst laufend findet. Siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

## Nach dem Aktualisieren

- **Fernsteuerung:** Ändert eine Version das Fernprotokoll, müssen ein Fenster und das von ihm gesteuerte `rookparcel-nox` beide auf dieser Version sein. Aktualisieren Sie beide. Die Hinweise zur Version sagen, wann das gilt ([Fernsteuerung](Fernsteuerung)).
- **Zurück zu einer älteren Version:** Einstellungen und Torrents bleiben zwischen den 1.1.x-Versionen erhalten. 1.1.40 speichert Fortsetzungsdaten in kleinerer Form, daher werden beim Zurückgehen von dieser Version auf eine ältere alle Torrents einmal neu geprüft. Ein [portabler Ordner](Wo-Rookparcel-seine-Dateien-ablegt#portabler-modus), der von 1.1.49 oder später verwendet wurde, braucht 1.1.49 oder später.
- Was sich in jeder Version geändert hat: die Hinweise zur Version auf der [Releases-Seite](https://github.com/Balls0fSteel/Rookparcel/releases) oder die Seite [News](https://balls0fsteel.github.io/Rookparcel/news.html) der Website.

## Wenn eine Version Ihre Einstellungen anders liest

*Neu in 1.2.57.*

Jedes Mal, wenn Rookparcel seine Einstellungen speichert, vermerkt es auch seine Version und wie es die Einstellungen gelesen hat, die bestimmen, wohin Ihr Datenverkehr geht: den Proxy, **Nur dieses Netzwerk verwenden**, die Listen-Schnittstellen, die Verschlüsselung und den IP-Filter. Startet eine andere Version, vergleicht sie diesen Vermerk mit ihrer eigenen Lesart derselben Datei:

- Zuerst legt sie daneben eine Kopie der Datei an, wie sie sie vorgefunden hat: `rookparcel.ini.<alte Version>.bak` (`webui.json.<alte Version>.bak` für `rookparcel-nox`). Die letzten drei bleiben erhalten.
- Jede Einstellung, die anders gelesen wird, kommt in den Reiter **Protokoll**, zum Beispiel `proxy: socks5 10.8.0.1:1080 peers trackers names → none`.
- Würden der Proxy, die Netzwerkregel oder die Listen-Schnittstellen verloren gehen, verbindet sich nichts: Die Statusleiste sagt **Übertragungen angehalten: Einstellungen bestätigen**, und die Torrents warten. Prüfen Sie **Optionen > Einstellungen > Verbindung** und klicken Sie dann auf **OK**, und die Übertragungen starten. In der Weboberfläche speichern Sie deren Einstellungsseite. RSS-Feeds, `.torrent`-Links und die Update-Prüfung warten ebenfalls. Um stattdessen zurückzugehen, beenden Sie Rookparcel und kopieren die `.bak`-Datei über die Einstellungsdatei.

Die Übertragungen werden auf dieselbe Weise angehalten, wenn die Einstellungsdatei gar nicht gelesen werden kann (sie wird dann nie überschrieben), wenn sie fehlt oder leer ist, obwohl es gespeicherte Torrents gibt, oder wenn diese Version den Proxy-Typ nicht lesen kann, den eine neuere Version geschrieben hat; diese Zeile bleibt für die neuere Version in der Datei. Eine Einstellungsdatei, die kein gültiger Text ist (von einem Editor in einer anderen Kodierung gespeichert), wird so gut wie möglich gelesen, und das Original bleibt als `rookparcel.ini.damaged-<Nummer>` erhalten. `rookparcel-nox` startet nicht mit einer `webui.json`, die es nicht lesen kann; es nennt die Datei, damit Sie sie reparieren oder wegschieben können.
