*Dies ist eine Übersetzung der englischen Seite [Queueing and seeding](Queueing-and-seeding). Maßgeblich ist die englische Seite.*

# Warteschlange und Seeding

Diese Seite beschreibt, wie Rookparcel entscheidet, welche Torrents gleichzeitig laufen (die Warteschlange), was passiert, wenn ein fertiger Torrent genug geseedet hat (das Seeding-Ziel), das Initial-Seeding für neue Torrents und das Verschieben fertiger Downloads in einen anderen Ordner.

Die Einstellungen finden Sie unter **Optionen > Einstellungen > Warteschlange**.

![Einstellungen, Seite „Warteschlange“](images/prefs-queueing.png)

## Wie die Warteschlange funktioniert

Ein Torrent, den Sie starten, wird an die Warteschlange übergeben. Die Warteschlange lässt so viele Torrents laufen, wie ihre Grenzen erlauben, und lässt die übrigen warten:

| Status | Bedeutung |
|---|---|
| **Lädt herunter**, **Seeding** | Läuft, in einem Platz der Warteschlange. |
| **Eingereiht** | Wartet auf einen Download-Platz. |
| **Seed eingereiht** | Fertig, wartet auf einen Seeding-Platz. |
| **[F] Lädt herunter**, **[F] Seeding** | Mit **Start erzwingen** gestartet, außerhalb der Warteschlange. |
| **Zur Prüfung eingereiht**, **Prüfe** | Wartet, bis das Laufwerk mit der Prüfung an der Reihe ist, und wird dann geprüft. Siehe [Datenträger und Speicher](Datenträger-und-Speicher#prüfung). |
| **Gestoppt**, **Angehalten**, **Fertig** | Von Ihnen (oder, bei „Fertig“, vom Seeding-Ziel) aus der Warteschlange genommen. |

- **Starten** übergibt die ausgewählten Torrents an die Warteschlange.
- **Start erzwingen** lässt sie sofort laufen, außerhalb der Warteschlange, auch wenn deren Grenzen erreicht sind. Erzwungene Torrents erreichen nie ein Seeding-Ziel: Sie seeden, bis Sie sie stoppen.
- **Stoppen** und **Anhalten** nehmen Torrents aus der Warteschlange, bis Sie sie wieder starten.
- **Erneut prüfen** läuft ebenfalls außerhalb der Warteschlange: *neu in 1.2.76.* Ein Torrent, den Sie erneut prüfen, wird sofort geprüft, auch wenn die Warteschlange voll ist, und wartet danach wieder, bis er an der Reihe ist. Bei einem gestoppten oder angehaltenen Torrent startet es den Torrent auch, den Sie dann wieder stoppen können.
- **In der Warteschlange nach oben**, **In der Warteschlange nach unten**, **An den Anfang der Warteschlange** und **An das Ende der Warteschlange** im Kontextmenü ändern die Reihenfolge. Die Spalte **#** zeigt die Position jedes Torrents. Torrents weiter oben bekommen zuerst einen Platz.

Diese Befehle und ihre Tasten finden Sie unter [Torrents verwalten](Torrents-verwalten). Ein neuer Torrent kann mit **An den Anfang der Warteschlange** im Fenster „Neuen Torrent hinzufügen“ ganz oben in die Warteschlange eingefügt werden ([Torrents hinzufügen](Torrents-hinzufügen)).

## Warteschlangen-Einstellungen

Unter **Warteschlangen-Einstellungen**:

| Einstellung | Bedeutung | Standard |
|---|---|---|
| **Höchstzahl aktiver Torrents (Upload oder Download): [-1: keine Grenze]** | Die höchste Zahl eingereihter Torrents, die gleichzeitig laufen, Downloads und Seeds zusammen. | 500 |
| **Höchstzahl aktiver Downloads: [-1: keine Grenze]** | Die höchste Zahl eingereihter Torrents, die gleichzeitig herunterladen. Der nächste startet, wenn einer fertig wird oder stoppt. | 3 |
| **Höchstzahl aktiver Seeds: [-1: keine Grenze]** | Die höchste Zahl fertiger eingereihter Torrents, die gleichzeitig seeden. | 5 |
| **Langsame Torrents nicht zu diesen Grenzen zählen** | Ein Torrent, der eine Minute lang weniger als etwa 2 kB/s bewegt, belegt keinen Download- oder Seeding-Platz mehr, sodass ein anderer starten kann. Er zählt weiterhin zur Gesamtzahl. | An |

Damit alles gleichzeitig läuft, setzen Sie die Grenzen auf -1. Um jeweils einen Torrent herunterzuladen, setzen Sie **Höchstzahl aktiver Downloads** auf 1.

„Langsam“ wird auf der Seite „Erweitert“ festgelegt: `inactive_down_rate` und `inactive_up_rate` (Bytes pro Sekunde, Standard 2048) und `auto_manage_startup` (Sekunden, die ein Torrent darunter bleiben muss, Standard 60).

## Seeding-Ziel

Das Seeding-Ziel legt fest, wann ein fertiger Torrent genug geseedet hat. Unter **Seeding-Ziel**:

| Einstellung | Bedeutung | Standard |
|---|---|---|
| **Mindestverhältnis (%): [0: kein Verhältnisziel]** | Laden Sie diesen Prozentsatz dessen hoch, was der Torrent heruntergeladen hat. 150 heißt das Eineinhalbfache. | 150 |
| **Mindest-Seedingzeit (Minuten): [0: kein Zeitziel]** | Seeden Sie mindestens so viele Minuten. | 0 |

Sind beide gesetzt, müssen beide erreicht werden. Bei beiden auf 0 gibt es kein Ziel.

Diese beiden Zahlen bewirken für sich allein nichts. Zunächst seeden Torrents, bis Sie sie stoppen, egal wie ihr Verhältnis ist.

**Wenn ein Torrent das Seeding-Ziel erreicht (erzwungene Torrents nie):**

- **Upload-Rate begrenzen auf (kB/s): [0: stoppen]** setzt das Ziel in Kraft. Zunächst ist es aus.
- Angehakt mit 0 wird ein Torrent, der das Ziel erreicht hat, gestoppt. Er verlässt die Warteschlange und wird als **Fertig** angezeigt.
- Angehakt mit einer Rate seedet der Torrent langsam weiter: Sein eigenes Upload-Limit wird auf diese Rate gesetzt. Ein niedrigeres Limit, das der Torrent schon hat, bleibt bestehen.

Der Reiter **Protokoll** schreibt den Namen des Torrents und das Erreichte, zum Beispiel „seeding goal reached, stopped (ratio 1.523, goal 1.500)“ oder „seeding goal reached, upload limited to ...“.

Geprüft werden nur eingereihte Torrents: Ein erzwungen gestarteter, gestoppter, geprüfter oder noch herunterladender Torrent bleibt unberührt.

Ein Ziel wirkt nur einmal auf einen Torrent. Wenn Sie einen Torrent erneut starten, nachdem ihn sein Ziel gestoppt hat, seedet er weiter, bis Sie ihn stoppen. Er wird erst wieder gestoppt, nachdem Sie das Ziel geändert haben und der Torrent das neue erreicht.

Bis Version 1.2.44 war das Ziel immer in Kraft und stoppte jeden Torrent bei 150 %. Ein Update lässt ein Ziel, das Sie nie geändert haben, ausgeschaltet. Ein Ziel, das Sie geändert hatten, stoppt weiterhin Torrents: Das Kästchen wird für Sie angehakt, mit 0.

### Ein Ziel für einen einzelnen Torrent

1. Wählen Sie den Torrent aus und wählen Sie **Eigenschaften** (<kbd>Enter</kbd>).
2. Haken Sie unter **Seeding-Ziel** **Standardeinstellungen überschreiben** an.
3. Legen Sie **Mindestverhältnis (%): [0: kein Verhältnisziel]** und **Mindest-Seedingzeit (Minuten): [0: kein Zeitziel]** fest. Beide auf 0 lassen diesen Torrent für immer seeden.
4. Klicken Sie auf **OK**.

Das eigene Ziel eines Torrents ist immer in Kraft, auch wenn das Kästchen in den Einstellungen aus ist: Der Torrent wird gestoppt, wenn er es erreicht (oder sein Upload wird begrenzt, wenn das Kästchen mit einer Rate angehakt ist).

![Der Dialog „Torrent-Eigenschaften“](images/properties.png)

## Initial-Seeding

Das Initial-Seeding (Super-Seeding) hilft Ihnen, einen soeben erstellten Torrent zu verbreiten, wenn Sie sein einziger Seed sind: Jeder Peer bekommt Teile, die noch niemand sonst hat, sodass der Schwarm schneller eine vollständige Kopie erhält.

- Klicken Sie mit der rechten Maustaste auf einen seedenden Torrent und wählen Sie **Erweitert > Initial-Seeding**. Der Eintrag ist grau, außer wenn jeder ausgewählte Torrent seedet; wählen Sie ihn erneut, um es auszuschalten.
- Oder haken Sie in den Eigenschaften des Torrents unter **Sonstige Einstellungen** **Initial-Seeding** an.

Solange es an ist, zeigt der Status **[S] Seeding**. Wie Sie einen Torrent zum Seeden erstellen, steht unter [Torrents erstellen](Torrents-erstellen). Bei rookparcel-nox macht `--super-seed` dasselbe für vollständige Torrents.

## Abgeschlossene Downloads verschieben

Rookparcel kann die Dateien eines Torrents in einen anderen Ordner verschieben, wenn er fertig heruntergeladen ist, zum Beispiel von einer schnellen Festplatte auf eine große. Es seedet vom neuen Ort weiter. Stellen Sie es unter **Optionen > Einstellungen > Ordner** ein, unter **Ort der heruntergeladenen Dateien**:

1. Haken Sie **Abgeschlossene Downloads verschieben nach** an und wählen Sie darunter den Ordner.
2. Haken Sie optional **Label des Torrents anhängen** an: Ein Torrent mit einem Label kommt in einen nach dem Label benannten Ordner innerhalb dieses Ordners. Es zählt das Label, das er beim Fertigwerden hat.
3. Lassen Sie **Nur aus dem Standard-Downloadordner verschieben** angehakt, um nur Torrents zu verschieben, die im Ordner für neue Downloads (**Neue Downloads speichern in**) selbst gespeichert sind; Torrents, die Sie anderswo gespeichert haben, auch in einem Ordner darin, bleiben dann, wo sie sind. Nehmen Sie den Haken weg, um jeden fertigen Torrent zu verschieben.
4. Klicken Sie auf **OK**.

Es gilt für Torrents, die ab jetzt fertig werden. Mehr zu Ordnern und zum Verschieben von Dateien: [Datenträger und Speicher](Datenträger-und-Speicher#abgeschlossene-downloads-verschieben).

## Automatisches Herunterfahren und Seeding

**Optionen > Automatisch herunterfahren > ... wenn alles abgeschlossen ist** wartet, bis jeder Seed gestoppt ist. Ein Seed ohne Seeding-Ziel oder ein erzwungener seedet weiter und hält es zurück. Siehe [Automatisches Herunterfahren](Automatisches-Herunterfahren).

## Im GTK-Fenster und in der Web-UI

- **GTK-Fenster**: dieselbe Seite „Warteschlange“, dieselben Eigenschaften und Menüs.
- **macOS-App**: dieselbe Seite „Warteschlange“, dieselben Eigenschaften und Menüs; Einstellungen gelten sofort, ohne **OK**.
- **Web-UI**: dieselbe Seite „Warteschlange“. Ihr Kästchen für das Seeding-Ziel heißt **Torrent stoppen oder seine Upload-Rate begrenzen**, mit der Rate daneben; 0 stoppt den Torrent. Das Torrent-Menü hat dieselben Einträge für Warteschlange und Initial-Seeding.
- **rookparcel-nox**: Die Warteschlangengrenzen und das Seeding-Ziel werden in den Einstellungen seiner Web-UI festgelegt. `--move-on-complete DIR` verschiebt jeden Torrent, der fertig wird, in einen Ordner.

## Verwandte Seiten

- [Torrents verwalten](Torrents-verwalten)
- [Geschwindigkeitsbegrenzung und Zeitplan](Geschwindigkeitsbegrenzung-und-Zeitplan)
- [Datenträger und Speicher](Datenträger-und-Speicher)
- [Einstellungen im Überblick](Einstellungen-im-Überblick)
