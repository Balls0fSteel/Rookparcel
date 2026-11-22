*Dies ist eine Übersetzung der englischen Seite [Auto shutdown](Auto-shutdown). Maßgeblich ist die englische Seite.*

# Automatisch herunterfahren

„Automatisch herunterfahren“ beendet Rookparcel oder versetzt den Computer in den Standby, in den Ruhezustand oder fährt ihn herunter, sobald Ihre Torrents fertig sind. Diese Seite beschreibt die Auswahlmöglichkeiten, was als „fertig“ zählt, und den Countdown, bevor es handelt.

## Auswählen, was passiert

Öffnen Sie **Optionen > Automatisch herunterfahren** und wählen Sie einen Eintrag:

| Eintrag | Was passiert, wenn die Arbeit erledigt ist |
|---|---|
| **Deaktiviert** | Nichts. Das ist der Standard. |
| **Beenden, wenn die Downloads abgeschlossen sind** | Rookparcel beendet sich, sobald kein gestarteter Torrent mehr herunterlädt. |
| **Beenden, wenn alles abgeschlossen ist** | Rookparcel beendet sich, sobald kein gestarteter Torrent mehr herunterlädt oder seedet. |
| **Standby, wenn die Downloads abgeschlossen sind** | Der Computer geht in den Standby (Energiesparmodus), sobald die Downloads erledigt sind. |
| **Standby, wenn alles abgeschlossen ist** | Dasselbe, sobald nichts mehr herunterlädt oder seedet. |
| **Ruhezustand, wenn die Downloads abgeschlossen sind** | Der Computer geht in den Ruhezustand, sobald die Downloads erledigt sind. |
| **Ruhezustand, wenn alles abgeschlossen ist** | Dasselbe, sobald nichts mehr herunterlädt oder seedet. |
| **Herunterfahren, wenn die Downloads abgeschlossen sind** | Der Computer fährt herunter, sobald die Downloads erledigt sind. |
| **Herunterfahren, wenn alles abgeschlossen ist** | Dasselbe, sobald nichts mehr herunterlädt oder seedet. |

![Das Menü „Optionen > Automatisch herunterfahren“](images/auto-shutdown.png)

Der gewählte Eintrag hat ein Häkchen daneben. Standby oder Ruhezustand sind grau, wenn Windows sie auf diesem Computer nicht anbietet (zum Beispiel bei ausgeschaltetem Ruhezustand).

Die Wahl gilt nur für diesen Lauf: Jeder Start von Rookparcel beginnt bei **Deaktiviert**, und nachdem es gehandelt hat, steht es wieder auf **Deaktiviert**.

## Was als fertig zählt

**Downloads abgeschlossen** wartet auf jeden gestarteten Torrent, der noch nicht fertig heruntergeladen ist: laufende, erzwungene oder eingereihte, auch einen, der gerade geprüft wird, die Dateiliste eines Magnet-Links holt oder vom Zeitplaner zurückgehalten wird. Ein Torrent, dessen gewünschte Dateien alle fertig sind, zählt als fertig, auch wenn übersprungene Dateien übrig sind.

**Alles abgeschlossen** wartet zusätzlich auf jeden gestarteten Torrent, der seedet oder zum Seeden eingereiht ist. Ein Seed ist abgeschlossen, wenn ihn sein [Seeding-Ziel](Warteschlange-und-Seeding#seeding-ziel) stoppt. Ein Seed ohne wirksames Ziel (das Kästchen des Ziels unter Einstellungen > Warteschlange ist zunächst aus) oder ein erzwungen gestarteter seedet weiter und hält das Herunterfahren zurück; setzen Sie also ein Ziel in Kraft, bevor Sie einen „Alles“-Eintrag wählen.

In beiden Fällen gilt:

- Ein gestoppter oder angehaltener Torrent oder einer mit einem Fehler zählt nicht: Er wartet auf Sie. Ein Torrent, der gehalten wird, weil sein Laufwerk oder Ordner fehlt, hat ebenfalls einen Fehler ([Datenträger und Speicher](Datenträger-und-Speicher#wenn-ein-laufwerk-oder-eine-freigabe-fehlt)).
- Ein Torrent, dessen Dateien verschoben werden, zählt als Arbeit, bis die Verschiebung beendet ist.
- „Automatisch herunterfahren“ handelt erst, nachdem es Arbeit gesehen hat, während der Eintrag gewählt war. Wählen Sie ihn, wenn nichts mehr zu tun ist, wartet es auf den nächsten Download, statt sofort zu handeln.

## Der Countdown

Ist die Arbeit erledigt, erscheint über allen Fenstern ein kleines Fenster **Automatisch herunterfahren**, auch wenn Rookparcel im Infobereich versteckt ist. Es sagt, was abgeschlossen ist, und zählt von 30 Sekunden herunter, zum Beispiel:

> Alle Downloads sind abgeschlossen.
> Der Computer wird in 0:30 heruntergefahren.

- Klicken Sie auf **Abbrechen** (oder drücken Sie <kbd>Esc</kbd>), um es abzusagen. „Automatisch herunterfahren“ steht dann auf **Deaktiviert**.
- Sind die Fenster durch eine PIN gesperrt ([Rookparcel sperren](Rookparcel-sperren)), wird der Countdown trotzdem angezeigt, sodass Sie ihn abbrechen können.
- Trifft während des Countdowns neue Arbeit ein, etwa ein RSS-Download, schließt sich der Countdown und wartet, bis auch diese Arbeit erledigt ist; dann beginnt wieder ein voller Countdown.
- Ist die Zeit abgelaufen, handelt es.

Der Reiter **Protokoll** hält jeden Schritt fest: „Automatisch herunterfahren: erledigt, ... in 30 s“, „Automatisch herunterfahren: abgebrochen, jetzt deaktiviert“, „Automatisch herunterfahren: neue Aufgaben, der Countdown wartet, bis sie erledigt sind“.

## Wie jede Aktion ausgeführt wird

- **Beenden**: Rookparcel beendet sich wie mit **Datei > Beenden** und speichert dabei die Torrent-Liste und den Fortschritt.
- **Herunterfahren**: Rookparcel beendet sich zuerst auf dieselbe Weise, bittet dann Windows, herunterzufahren und auszuschalten. Windows wird nicht gezwungen: Ein Programm mit ungespeicherter Arbeit kann das Herunterfahren trotzdem aufhalten.
- **Standby** und **Ruhezustand**: Rookparcel speichert zuerst die Torrent-Liste und den Fortschritt und versetzt dann den Computer in den Schlaf. Es läuft weiter und macht weiter, wenn der Computer aufwacht. „Automatisch herunterfahren“ steht dann auf **Deaktiviert**.

Lehnt Windows ab, sagt eine Meldung das, etwa „Der Computer konnte nicht heruntergefahren werden:“ mit dem Grund. Ein Windows-Konto, das den Computer nicht herunterfahren darf, bekommt diese Meldung.

## Im GTK-Fenster, unter macOS und in rookparcel-nox

- **GTK-Fenster** (Linux, BSD): dasselbe Menü **Optionen > Automatisch herunterfahren** und derselbe Countdown. Standby, Ruhezustand und Herunterfahren laufen über den Login-Manager des Systems (systemd-logind), der nach Ihrem Passwort fragen kann. Wo er fehlt (die BSDs, ein System ohne systemd) oder eine Aktion nicht erlaubt, sind diese Einträge grau, und nur die Einträge zum Beenden bleiben übrig.
- **macOS**: dasselbe Menü **Optionen > Automatisch herunterfahren** und derselbe Countdown. Standby und Herunterfahren bitten macOS um den Ruhezustand oder das Herunterfahren wie das Apple-Menü, mit Apple Events an das „loginwindow“ des Systems. Beim ersten Mal fragt macOS möglicherweise, ob Rookparcel „loginwindow“ steuern darf; klicken Sie auf **OK**. Haben Sie abgelehnt, erlauben Sie es unter **Systemeinstellungen > Datenschutz & Sicherheit > Automation**. Die Einträge für den Ruhezustand sind grau: macOS hat keinen eigenen Ruhezustand, um ihn anzufordern. *Die native macOS-App ist neu in 1.2.19.*
- **Web-UI**: hat kein „Automatisch herunterfahren“.
- **rookparcel-nox**: Starten Sie es mit `--exit-when-done downloads` oder `--exit-when-done all`, um sich, mit gespeichertem Zustand, zu beenden, sobald nichts mehr herunterlädt, oder sobald nichts mehr herunterlädt oder seedet. Es wartet, bis zuvor etwas gelaufen ist, wie das Menü, und beendet sich einige Sekunden nach Abschluss der Arbeit. Es versetzt den Computer nie in den Schlaf und fährt ihn nie herunter. Siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

## Verwandte Seiten

- [Warteschlange und Seeding](Warteschlange-und-Seeding)
- [Geschwindigkeitsbegrenzung und Zeitplan](Geschwindigkeitsbegrenzung-und-Zeitplan)
- [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart)
