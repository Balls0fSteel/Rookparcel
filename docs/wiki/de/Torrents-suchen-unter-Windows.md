*Dies ist eine Übersetzung der englischen Seite [Searching for torrents on Windows](https://github.com/Balls0fSteel/Rookparcel/wiki/Searching-for-torrents-on-Windows). Maßgeblich ist die englische Seite.*

# Torrents suchen unter Windows

Eine kurze Anleitung zum Suchfeld in der Windows-Version von Rookparcel: wie Sie den Suchhelfer holen, die Seiten auswählen, die er durchsucht, und hinzufügen, was er findet. Zehn Minuten und ein Download von 55 MB, einmalig.

**Version:** Der Suchhelfer ist in Rookparcel 1.1.59 und neuer enthalten. Das Release 1.1.40 hat das Suchfeld und „Quellen verwalten“, aber nicht die Schaltfläche „Suchhelfer holen“; dort gilt der letzte Abschnitt (Ihr eigenes Prowlarr oder Jackett).

## Was Sie bekommen

Rookparcel hat keine eigene Suchmaschine. Es fragt eine *Suchquelle*: ein kleines Programm, das Torrent-Seiten durchsuchen kann. Die einfachste Quelle ist der **Suchhelfer**, das ist [Jackett](https://github.com/Jackett/Jackett), ein eigenständiges kostenloses Programm seiner eigenen Autoren. Rookparcel lädt es auf Ihren Klick für Sie herunter, bewahrt es in einem eigenen Ordner auf und führt es nur aus, während Sie suchen. Sie wählen, welche öffentlichen Seiten er durchsucht. Bis Sie auf die Schaltfläche klicken, wird nichts heruntergeladen oder ausgeführt.

Wenn Sie Prowlarr oder Jackett schon selbst betreiben, können Sie stattdessen das verwenden: siehe den letzten Abschnitt.

## 1. „Quellen verwalten“ öffnen

Das Suchfeld befindet sich oben rechts im Hauptfenster. Der kleine Pfeil daneben öffnet ein Menü. Wählen Sie **Quellen verwalten...**.

![Das Aufklappmenü des Suchfelds mit „Quellen verwalten“](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-dropdown.png)

Das Fenster „Suchquellen“ öffnet sich. Zuerst ist es leer.

![Das Fenster „Suchquellen“ mit der Schaltfläche „Suchhelfer holen“](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-sources.png)

## 2. Den Suchhelfer holen

Klicken Sie auf **Suchhelfer holen (Jackett)...**. Rookparcel sucht das neueste Jackett-Release und fragt nach, bevor es etwas herunterlädt:

> Der Suchhelfer ist Jackett, ein eigenständiges Programm seiner eigenen Autoren unter der GPL-2.0-Lizenz, heruntergeladen von github.com/Jackett/Jackett (v0.24.3009, 54.5 MB).
>
> Er wird in C:\Users\you\AppData\Roaming\Rookparcel\search-helper abgelegt und läuft nur auf diesem Computer und nur, während Sie suchen. Danach wählen Sie die öffentlichen Seiten aus, die er durchsucht.
>
> Jetzt herunterladen?

Klicken Sie auf **Ja**. Der Download dauert je nach Verbindung einige Sekunden bis eine Minute.

![Das Fortschrittsfenster des Downloads](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-download.png)

Rookparcel prüft die Datei anhand der Prüfsumme des Releases, entpackt sie und startet sie einmal, um ihre Seitenliste zu lesen. Es erscheint keine Windows-Abfrage: Nichts braucht Administratorrechte, und der Helfer lauscht nur auf diesem Computer, die Firewall hat also nichts zu fragen.

## 3. Die Seiten zum Durchsuchen anhaken

Die Seitenliste öffnet sich von selbst (später über die Schaltfläche **Seiten auswählen...**). Nichts ist für Sie angehakt. Haken Sie die öffentlichen Seiten an, die Sie durchsuchen möchten, und klicken Sie dann auf **OK**.

![Das Fenster „Seiten des Suchhelfers“ mit angehaktem The Pirate Bay](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-sites.png)

- Die ausgegrauten Zeilen sagen „Benötigt FlareSolverr, das der Helfer nicht enthält“. Diese Seiten stehen hinter einer Bot-Prüfung, die der Helfer allein nicht bestehen kann; lassen Sie sie aus.
- Zwei oder drei Seiten genügen für den Anfang. Jede angehakte Seite wird bei jeder Suche gefragt, mehr Seiten bedeuten also langsamere Suchen und mehr Duplikate.
- Private Seiten brauchen ihre Logins. Die eigene Seite des Helfers kümmert sich darum: siehe „Private Seiten“ unten.

Zurück in „Suchquellen“ ist der Helfer jetzt als **Suchhelfer (alle)** aufgeführt, mit seinen Schaltflächen darunter, und die Zeile unten sagt, wie viele Seiten angehakt sind. Klicken Sie auf **Schließen**.

![„Suchquellen“ mit installiertem Helfer und zwei angehakten Seiten](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-sources-installed.png)

## 4. Suchen

Öffnen Sie das Menü des Pfeils erneut und wählen Sie **Suchhelfer (alle)**. Der Hinweis des Suchfelds ändert sich entsprechend. (Der erste Eintrag, **Torrents filtern**, ist die andere Aufgabe des Felds: Ihre eigene Torrent-Liste nach Namen eingrenzen. **Alle Quellen** fragt alle Ihre Quellen auf einmal.)

![Das Aufklappmenü mit gewähltem Suchhelfer (alle)](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-dropdown-helper.png)

Geben Sie ein, was Sie suchen, und drücken Sie die **Eingabetaste**. Das Fenster „Torrents suchen“ öffnet sich, und die Ergebnisse treffen innerhalb weniger Sekunden ein.

![Das Fenster „Torrents suchen“ mit Ergebnissen für „ubuntu“](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-results.png)

- Klicken Sie auf eine Spaltenüberschrift, um zu sortieren: Seeds von hoch nach niedrig findet die gesunden.
- Das Kategoriefeld grenzt die Suche auf Filme, TV, Audio, PC, Bücher oder Sonstiges ein. Das Quellenfeld und die Schaltfläche „Suchen“ wiederholen die Suche aus diesem Fenster.
- Wählen Sie ein Ergebnis aus und klicken Sie auf **Hinzufügen**, oder doppelklicken Sie darauf. Der Torrent wird so hinzugefügt wie ein Magnet-Link; ist in den Einstellungen „Ein Fenster mit den Dateien im Torrent anzeigen“ eingeschaltet, öffnet sich zuerst das Fenster „Torrent hinzufügen“.
- Damit das Feld wieder Ihre Torrent-Liste filtert, wählen Sie im Menü des Pfeils **Torrents filtern**. Die Wahl bleibt zwischen Starts erhalten.

Das ist die ganze Einrichtung. Die folgenden Abschnitte sind für später.

## Gut zu wissen

**Er läuft nur, während Sie suchen.** Der Helfer startet bei Ihrer ersten Suche und stoppt sich zehn Minuten nach der letzten selbst. Er schließt mit Rookparcel, egal wie Rookparcel endet. Er startet nie von selbst oder beim Hochfahren, und er lauscht nur auf diesem Computer (`127.0.0.1`), nie in Ihrem Netzwerk.

**Wo er liegt.** Der Ordner `search-helper` in Ihrem Einstellungsordner von Rookparcel: normalerweise `%APPDATA%\Rookparcel\search-helper`, im portablen Modus der Ordner mit `rookparcel.exe`, und der Ordner des Profils, wenn Sie Profile verwenden. Die eigenen Einstellungen und das Protokoll von Jackett liegen in seinem Unterordner `data`.

**Proxy und VPN.** Wenn Sie unter Einstellungen > Verbindung einen Proxy setzen, verwendet der Helfer ihn auch für die Seiten. Der Helfer ist ein eigenes Programm, daher bindet „Nur dieses Netzwerk verwenden“ ihn nicht: Seine Anfragen an die Seiten nehmen den normalen Weg von Windows, und das ist Ihr VPN, wenn das VPN Ihren gesamten Verkehr trägt.

**Aktualisieren.** Nichts aktualisiert sich von selbst. **Suchhelfer aktualisieren** in „Suchquellen“ sucht ein neueres Jackett und fragt vor dem Herunterladen nach; Ihre Seiten und Einstellungen bleiben.

**Entfernen.** **Suchhelfer entfernen** stoppt ihn, löscht seinen Ordner und nimmt seine Quelle aus der Liste. Die Schaltfläche **Holen** kehrt zurück. Wird der Einstellungsordner von Rookparcel gelöscht, verschwindet er ebenfalls.

**Private Seiten.** **Seite des Helfers öffnen** öffnet die eigene Seite von Jackett in Ihrem Browser und kopiert deren Passwort in die Zwischenablage (die Seite fragt danach). Dort können Sie eine private Seite mit Ihrem Login hinzufügen, wie in jedem Jackett. Dort hinzugefügte Seiten werden zusammen mit den angehakten durchsucht.

**Virenscanner.** Der Download ist das eigene Release von Jackett, geprüft anhand der Prüfsumme, die GitHub dafür veröffentlicht. Verschiebt ein Virenscanner Dateien aus dem Ordner `search-helper` in die Quarantäne, schlagen Suchen mit „Der Suchhelfer wurde nicht gestartet“ fehl; stellen Sie die Dateien wieder her oder entfernen Sie ihn und holen Sie ihn erneut.

## Wenn etwas schiefgeht

| Was Sie sehen | Was es bedeutet |
|---|---|
| „Der Suchhelfer konnte nicht geladen werden: ...“ | GitHub war nicht erreichbar (keine Verbindung, ein Proxy, der es blockiert) oder der Download wurde unterbrochen. Versuchen Sie es erneut. |
| „Der Suchhelfer wurde nicht gestartet: ...“ | Das Programm im Ordner `search-helper` ließ sich nicht ausführen: ein beschädigter Download oder ein Virenscanner. Entfernen Sie ihn und holen Sie ihn erneut. |
| Eine Suche findet nichts | Es sind keine Seiten angehakt, oder die angehakten Seiten sind nicht erreichbar oder haben sich geändert. „Seiten auswählen...“ zeigt, was angehakt ist; probieren Sie eine andere Seite. |
| „Suchhelfer (alle): ...“ mit einem Fehler | Eine Seite ist fehlgeschlagen; die anderen antworten weiter. Die Meldung nennt die Seite. |
| Das Ergebnisfenster öffnet sich nie | Das Feld ist im Modus **Torrents filtern**. Wählen Sie zuerst im Menü des Pfeils **Suchhelfer (alle)** oder eine Quelle. |

## Stattdessen Ihr eigenes Prowlarr oder Jackett verwenden

Wenn Sie Prowlarr oder Jackett schon betreiben (auf diesem PC, einem NAS, einem Server), fügen Sie es als Quelle hinzu und lassen Sie den Helfer aus:

1. Klicken Sie in „Suchquellen“ auf **Hinzufügen...**.
2. **Name:** beliebig. **Torznab-URL:** für Jackett `http://host:9117/api/v2.0/indexers/all/results/torznab/`; für Prowlarr die Torznab-Feed-URL eines Indexers, `http://host:9696/1/api`, eine Quelle pro Indexer. **API-Schlüssel:** von der Seite von Jackett oder aus Settings > General von Prowlarr.
3. Klicken Sie auf **Testen**: Es sollte „OK“ und eine Anzahl Kategorien melden. Dann auf **OK**.

Der API-Schlüssel wird verschlüsselt gespeichert und nie wieder angezeigt. Eine Quelle auf Ihrem eigenen Computer oder in Ihrem Netzwerk wird direkt erreicht; eine öffentliche geht durch Ihren Proxy, falls Sie einen gesetzt haben. (Löst der Proxy Hostnamen selbst auf, geht auch eine Quelle, die mit einem anderen Hostnamen als `localhost` angegeben ist, durch ihn; geben Sie die IP-Adresse der Quelle an, damit sie direkt bleibt.) Der Helfer und Ihre eigenen Quellen lassen sich nebeneinander verwenden: **Alle Quellen** im Menü des Pfeils fragt sie alle.

Wie der Helfer aufgebaut ist und was er im Netz tut, steht in [SEARCH_HELPER.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/SEARCH_HELPER.md); die Torznab-Suche selbst behandelt Teil 2 von [INTEGRATIONS.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/INTEGRATIONS.md).
