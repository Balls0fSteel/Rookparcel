*Dies ist eine Übersetzung der englischen Seite [Preferences reference](Preferences-reference). Maßgeblich ist die englische Seite.*

# Einstellungen im Überblick

Diese Seite listet jede Seite des Einstellungsfensters auf und sagt, was jede Einstellung darauf bewirkt, in der Reihenfolge, in der das Fenster sie zeigt. Jeder Abschnitt verweist auf die Seite, die das Thema ausführlich erklärt.

## Die Einstellungen öffnen

Öffnen Sie **Optionen > Einstellungen**, drücken Sie <kbd>Ctrl</kbd>+<kbd>P</kbd> oder klicken Sie in der Symbolleiste auf **Einstellungen**. Die Seiten stehen links; **Festplatten-Cache**, **Tracker**, **Programm ausführen** und **UI-Sperre** liegen unter **Erweitert**. **OK** speichert und schließt, **Übernehmen** speichert und lässt das Fenster offen, **Abbrechen** verwirft die Änderungen.

Eine Hilfe ist eingebaut:

- Zeigen Sie auf eine Einstellung, und ein Tooltip erklärt sie. Der Bereich unter den Seiten zeigt denselben Text für die Einstellung unter dem Mauszeiger oder sonst für die mit dem Tastaturfokus.
- Drücken Sie <kbd>F1</kbd>, und die Hilfe zur Einstellung mit dem Fokus erscheint neben ihr.

*Die Hilfetexte, Tooltips und F1 sind neu in 1.1.57.*

Die Texte unten sind Kurzfassungen dieser eingebauten Hilfe. Raten auf den Seiten sind in kB/s angegeben; 0 bedeutet unbegrenzt, sofern die Beschriftung nichts anderes sagt.

## Allgemein

![Einstellungen, Seite „Allgemein“](images/prefs-general.png)

| Einstellung | Was sie bewirkt |
|---|---|
| **Oberfläche:** (Sprache) | Die Sprache der Menüs, Dialoge und der Weboberfläche. Die erste Wahl folgt der Sprache des Systems. Die Desktop-Fenster wechseln beim nächsten Start. |
| **Mit .torrent-Dateien verknüpfen** | Macht diese Kopie von Rookparcel für Ihr Benutzerkonto zum Programm, das `.torrent`-Dateien öffnet. Ein Programm, das in den eigenen Standard-App-Einstellungen des Systems gewählt wurde, hat Vorrang. |
| **Mit magnet:-Links verknüpfen** | Macht diese Kopie von Rookparcel für Ihr Benutzerkonto zum Programm, das `magnet:`-Links öffnet, auf die in einem Browser geklickt wird. Mit Profilen kann nur ein Profil sie halten. |
| **Verknüpfung beim Start prüfen** | Fragt beim Start, ob `.torrent`-Dateien und `magnet:`-Links übernommen werden sollen, wenn Rookparcel nicht das Programm ist, das sie öffnet. Standardmäßig aus. |
| **Rookparcel beim Systemstart starten** | Startet Rookparcel, wenn Sie sich anmelden. Jedes Profil hat seinen eigenen Eintrag. |
| **Minimiert starten** | Beim automatischen Start startet Rookparcel, ohne sein Fenster zu zeigen, im Infobereich, wo es einen gibt. |
| **Standard-Apps...** (nur Windows) | Öffnet die Windows-Einstellungen bei „Standard-Apps“. Ein dort gewähltes Programm hat Vorrang vor den Feldern oben, und nur Sie können diese Wahl ändern. |
| **Automatisch nach Updates suchen** (Privatsphäre) | Fragt etwa zehn Sekunden nach dem Start und dann alle 12 Stunden die Website des Projekts, ob ein neueres Release erschienen ist. Ohne Nachfrage wird nichts heruntergeladen oder installiert. **Hilfe > Nach Updates suchen** funktioniert in beiden Fällen. |
| **Auf Beta-Versionen aktualisieren** (Privatsphäre) | Bietet bei der Update-Prüfung auch Vorabversionen (Beta) an. |
| **An unvollständige Dateien .!ut anhängen** (Beim Herunterladen) | Eine Datei, die noch heruntergeladen wird, wird mit `.!ut` hinter ihrem Namen gespeichert (`film.mkv.!ut`) und erhält ihren normalen Namen, sobald sie vollständig heruntergeladen ist und ihre Prüfung bestanden hat. Das Ein- oder Ausschalten benennt die Dateien laufender Torrents sofort um, die gestoppter Torrents, wenn sie starten. Standardmäßig aus. |

Unter Windows stehen die Felder für die Integration unter **Windows-Integration**; in der GTK-Version heißt die Gruppe **Desktop-Integration**.

*An unvollständige Dateien .!ut anhängen ist neu in 1.2.63.* Dateien, die µTorrent als `.!ut` oder qBittorrent als `.!qB` hinterlassen hat, werden übernommen, ob das Feld angekreuzt ist oder nicht. Eine Datei, die ein anderes Programm offen hält, behält ihren Namen, bis sie umbenannt werden kann. Eine Datei, deren Name mit `.!ut` eine andere Datei desselben Torrents ist, behält ihren einfachen Namen, und Entfernen und > Daten löschen löscht eine `.!ut`- oder `.!qB`-Datei nur, wenn sie die Daten dieses Torrents enthält.

Mehr: [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart), [Torrents hinzufügen](Torrents-hinzufügen), [Rookparcel aktualisieren](Rookparcel-aktualisieren), [Profile](Profile), [Datenträger und Speicher](Datenträger-und-Speicher#unvollständige-dateien-und-ut).

## Oberfläche

| Einstellung | Was sie bewirkt |
|---|---|
| **Löschen von Torrents bestätigen** | Fragt vor dem Entfernen von Torrents aus der Liste und sagt, ob auch ihre Dateien gelöscht werden. Ist es aus, handelt **Entfernen** sofort. |
| **Design** (Windows) | Die Farben des Fensters: **System (hell oder dunkel)** folgt dem Windows-Modus *Modus auswählen* für Apps, **Hell** und **Dunkel** legen sie fest. Der Hohe Kontrast von Windows behält seine eigenen Farben. Der dunkle Modus braucht Windows 10 Version 1809 oder neuer; ein älteres Windows sagt das unter dem Feld und bleibt hell ([Dunkler Modus](Dunkler-Modus)). |
| **Design** (nur Weboberfläche) | Die Farben der Seite der Weboberfläche für alle, die sie öffnen: System (hell oder dunkel), Hell, Dunkel, Mitternacht, Schiefer, Solarized Dunkel, Solarized Hell, Hoher Kontrast, Post-Saatkrähe und Ihre eigenen CSS-Dateien aus dem Ordner `webui/themes` im Einstellungsordner ([Weboberfläche](Weboberfläche)). |
| **Design in diesem Browser** (nur Weboberfläche) | Ein Design nur für diesen Browser, auf diesem Gerät gespeichert und anstelle der Wahl des Servers verwendet. **Serverstandard** folgt wieder dem Design des Servers. |
| **Abwechselnde Listenhintergrundfarbe** | Schattiert jede zweite Zeile der Listen. |
| **Aktuelle Geschwindigkeit in der Titelleiste anzeigen** | Setzt die aktuelle Download- und Upload-Geschwindigkeit an den Anfang des Titels des Hauptfensters, was auch in der Taskleiste erscheint. |
| **Sprechblasen im Infobereich anzeigen** | Zeigt eine Benachrichtigung mit dem Namen des Torrents, wenn ein Download abgeschlossen ist. |
| **Beim Schließen in den Infobereich** (Infobereich) | Die Schließen-Schaltfläche des Fensters blendet Rookparcel im Infobereich aus, statt es zu beenden, sodass die Übertragungen weiterlaufen. Beenden Sie es über das Menü oder das Menü des Infobereich-Symbols. |
| **Beim Minimieren in den Infobereich** (Infobereich) | Das Minimieren blendet das Fenster im Infobereich aus, statt es in der Taskleiste zu belassen. Klicken Sie auf das Infobereich-Symbol, um es zurückzuholen. |
| **Download nicht automatisch starten** (Beim Hinzufügen von Torrents) | Neue Torrents werden gestoppt und außerhalb der Warteschlange hinzugefügt, sodass nichts heruntergeladen wird, bis Sie sie starten. |
| **Programmfenster aktivieren** | Bringt das Hauptfenster in den Vordergrund, wenn ein anderes Programm, etwa ein Browser, Rookparcel einen Torrent übergibt. Ist es aus, kommt nur das Fenster „Neuen Torrent hinzufügen“ hoch. |
| **Ein Fenster mit den Dateien im Torrent anzeigen** | Öffnet für jeden hinzugefügten Torrent das Fenster „Neuen Torrent hinzufügen“, um zuerst den Ordner und die Dateien zu wählen; bei einem Magnet-Link, sobald seine Dateiliste eingetroffen ist. Ist es aus, werden Torrents sofort mit den Vorgaben hinzugefügt. |
| **Dateien mit diesen Namen nicht herunterladen:** (Beim Hinzufügen von Torrents) | Dateien, deren Namen zu einem der Muster im Feld darunter passen, beginnen ohne Häkchen und werden daher nicht heruntergeladen, in Torrents, die Sie ab jetzt hinzufügen. Trennen Sie die Muster mit Semikola; `*` steht für beliebige Zeichen und `?` für eines, und Groß- und Kleinschreibung spielt keine Rolle, wie in `*.lnk; Thumbs.db; *sample*`. Es zählt nur der eigene Name einer Datei, nicht ihre Ordner. Standardmäßig aus. |

*Dateien mit diesen Namen nicht herunterladen ist neu in 1.2.63.* Sie können eine solche Datei im Fenster „Neuen Torrent hinzufügen“ oder später in der Registerkarte „Dateien“ trotzdem ankreuzen. Ein Torrent, bei dem jede Datei passt, bleibt unberührt. Die Liste gilt auch für Torrents, die ohne das Fenster „Neuen Torrent hinzufügen“ hinzugefügt werden, etwa aus dem überwachten Ordner, von RSS oder von Sonarr und Radarr, aber nicht für Torrents, die schon in der Liste sind. Siehe [Torrents hinzufügen](Torrents-hinzufügen#dateien-nach-namen-auslassen).

Mehr: [Infobereich, Benachrichtigungen und Autostart](Infobereich-Benachrichtigungen-und-Autostart), [Torrents hinzufügen](Torrents-hinzufügen), [Torrents verwalten](Torrents-verwalten), [Das Hauptfenster](Das-Hauptfenster).

## Ordner

| Einstellung | Was sie bewirkt |
|---|---|
| **Neue Downloads speichern in:** | Der Ordner, in dem neue Torrents gespeichert werden, sofern Sie beim Hinzufügen keinen anderen wählen. |
| **Abgeschlossene Downloads verschieben nach:** | Wenn ein Torrent fertig heruntergeladen ist, werden seine Dateien in diesen Ordner verschoben, und er seedet von dort weiter. Gilt für Torrents, die ab jetzt fertig werden. |
| **Label des Torrents anhängen** | Verschiebt einen fertigen Torrent, der ein Label hat, in einen nach dem Label benannten Ordner innerhalb des Ordners für abgeschlossene Downloads. |
| **Nur aus dem Standard-Downloadordner verschieben** | Verschiebt nur Torrents, die im Ordner für neue Downloads gespeichert sind; Torrents, die Sie woanders gespeichert haben, bleiben, wo sie sind. |
| **.torrent-Dateien speichern in:** | Legt eine Kopie jeder hinzugefügten `.torrent`-Datei in diesem Ordner ab, nach dem Torrent benannt. Die Kopie eines Magnet-Links wird gespeichert, sobald seine Metadaten eingetroffen sind. |
| **.torrent-Dateien abgeschlossener Aufgaben verschieben nach:** | Wenn ein Torrent fertig heruntergeladen ist, wandert seine gespeicherte `.torrent`-Kopie in diesen Ordner. |
| **.torrent-Dateien automatisch laden aus:** | Überwacht diesen Ordner und fügt jede neue dort abgelegte `.torrent`-Datei hinzu, ohne das Fenster „Neuen Torrent hinzufügen“. Eine geladene Datei wird in `.torrent.loaded` umbenannt. |
| **Geladene .torrent-Dateien löschen** | Löscht eine automatisch geladene `.torrent`-Datei, sobald ihr Torrent hinzugefügt ist, statt sie umzubenennen. |

Mehr: [Datenträger und Speicher](Datenträger-und-Speicher), [Torrents hinzufügen](Torrents-hinzufügen), [Warteschlange und Seeding](Warteschlange-und-Seeding).

## Verbindung

![Einstellungen, Seite „Verbindung“](images/prefs-connection.png)

| Einstellung | Was sie bewirkt |
|---|---|
| **Port für eingehende Verbindungen:** | Der Port, mit dem sich andere Peers verbinden. Erlauben Sie ihn in der Firewall und leiten Sie ihn im Router weiter, oder lassen Sie UPnP oder NAT-PMP das tun. Eine Änderung wirkt sofort. |
| **Zufälliger Port** (Schaltfläche) | Trägt einen zufälligen Port von 10000 bis 60000 ein. Er wird verwendet, sobald Sie ihn übernehmen. |
| **Port bei jedem Start zufällig wählen** | Wählt bei jedem Start einen neuen zufälligen Port. Ein im Router von Hand weitergeleiteter Port passt dann nicht mehr. |
| **UPnP-Portweiterleitung aktivieren** | Bittet einen UPnP-Router, den Listening-Port an diesen Computer weiterzuleiten. Schalten Sie es aus, wenn Sie den Port selbst weiterleiten oder der Router sich falsch verhält. |
| **NAT-PMP-Portweiterleitung aktivieren** | Bittet den Router, den Port über NAT-PMP oder PCP weiterzuleiten, was viele Router neben UPnP oder statt dessen anbieten. |
| **Typ:** (Proxyserver) | Die Art des Proxys: **Socks4**, **Socks5** oder **HTTP**. **(keiner)** verbindet direkt. |
| **Proxy:** und **Port:** | Der Hostname oder die IP-Adresse des Proxyservers und der Port, auf dem er lauscht (etwa 1080 bei vielen SOCKS-Servern). |
| **Authentifizierung**, **Benutzername:**, **Passwort:** | Meldet sich am Proxy an. Ein SOCKS4-Proxy erhält nur den Benutzernamen. |
| **Hostnamen über den Proxy auflösen** | Lässt den Proxy die Namen von Trackern und Web-Seeds nachschlagen, sodass Ihr eigener DNS-Server sie nicht sieht. |
| **Proxy für Peer-Verbindungen verwenden** | Leitet Verbindungen zu Peers und Web-Seeds über den Proxy. Eingehende Verbindungen werden dann abgelehnt, und Portweiterleitung und die lokale Peer-Erkennung bleiben aus; uTP und das DHT brauchen einen SOCKS5-Proxy. |
| **Proxy für Tracker-Verbindungen verwenden** | Leitet Tracker-Announces über den Proxy, sodass Tracker die Adresse des Proxys statt Ihrer sehen. |
| **Netzwerk:** (Nur dieses Netzwerk verwenden) | Hält den gesamten Torrent-Verkehr auf einem Netzwerkadapter: **Beliebiger VPN-Adapter**, ein Adapter nach Namen oder **Adressbereich (unten)**. **Beliebiges Netzwerk (aus)** schaltet das ab. Die Weboberfläche und Fernverbindungen sind nicht gebunden. |
| **Adressbereich:** | Ein Bereich wie `10.2.0.0/16` oder `fd00::/8` für die Wahl **Adressbereich**: Der Adapter, der eine Adresse darin hat, trägt den Torrent-Verkehr. |
| **Torrents anhalten, solange dieses Netzwerk fehlt** | Solange kein Adapter passt (das VPN ist unten), warten laufende Torrents und zeigen „Warten auf das Netzwerk“, und machen dann von selbst weiter. |

*Die Gruppe Nur dieses Netzwerk verwenden ist neu in 1.1.58.*

Mehr: [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall).

## Bandbreite

![Einstellungen, Seite „Bandbreite“](images/prefs-bandwidth.png)

| Einstellung | Was sie bewirkt |
|---|---|
| **Maximale Upload-Rate (kB/s)** | Das meiste, was alle Torrents zusammen pro Sekunde hochladen. 0 bedeutet unbegrenzt. Die begrenzten Stunden des Zeitplaners und Ping niedrig halten (Seite „Gaming“) können sie eine Weile senken. |
| **Maximale Download-Rate (kB/s)** | Dasselbe für Downloads. |
| **Ratenbegrenzung auf Transport-Overhead anwenden** | Rechnet die TCP/IP-Header des Peer-Verkehrs auf die Grenzen an, sodass sie dem entsprechen, was die Leitung trägt. Standardmäßig aus. |
| **Ratenbegrenzung auf uTP-Verbindungen anwenden** | Wendet die Grenzen auch auf uTP-Peers an. Ist es aus, werden uTP-Peers nur von der eigenen Verzögerungssteuerung von uTP gebremst. Standardmäßig ein. |
| **Globale Höchstzahl an Verbindungen:** | Die meisten Peer-Verbindungen, die über alle Torrents hinweg gleichzeitig offen sind. Senken Sie sie, wenn der Router Mühe hat. |
| **Höchstzahl verbundener Peers pro Torrent** | Die meisten Peers, mit denen sich ein Torrent verbindet; 0 bedeutet keine Grenze. Gilt für Torrents, die ab jetzt hinzugefügt werden. |
| **Anzahl der Upload-Slots pro Torrent** | An wie viele Peers ein Torrent gleichzeitig hochlädt; 0 bedeutet keine Grenze. Gilt für Torrents, die ab jetzt hinzugefügt werden. |
| **Globale Anzahl der Upload-Slots** | An wie viele Peers alle Torrents zusammen gleichzeitig hochladen; -1 bedeutet keine Grenze. Weniger Slots geben jedem Peer mehr Geschwindigkeit. |

Mehr: [Geschwindigkeitsbegrenzung und Zeitplan](Geschwindigkeitsbegrenzung-und-Zeitplan).

## BitTorrent

![Einstellungen, Seite „BitTorrent“](images/prefs-bittorrent.png)

| Einstellung | Was sie bewirkt |
|---|---|
| **DHT-Netzwerk aktivieren** | Findet Peers über das DHT, ohne Tracker; die meisten Magnet-Links brauchen es. Private Torrents verwenden es nie. |
| **Lokale Peer-Erkennung aktivieren** | Findet Peers für dieselben Torrents in Ihrem lokalen Netzwerk. Private Torrents verwenden sie nie. |
| **Peer-Austausch aktivieren** | Lässt verbundene Peers einander von weiteren Peers erzählen. Private Torrents verwenden ihn nie. |
| **uTP aktivieren (UDP-Transport mit Bandbreitensteuerung)** | Verbindet sich mit Peers und nimmt Peers an, neben TCP auch über uTP. uTP bremst, wenn das Netzwerk ausgelastet ist. |
| **Ausgehend:** (Protokollverschlüsselung) | **Deaktiviert**: nie verschlüsseln. **Aktiviert**: unverschlüsselte und verschlüsselte Versuche wechseln sich pro Peer ab, und was funktioniert hat, bleibt. **Erzwungen**: nur verschlüsselt, wobei Peers, die nicht verschlüsseln können, verloren gehen. |
| **Eingehende unverschlüsselte Verbindungen erlauben** | Nimmt eingehende Verbindungen an, die nicht verschlüsselt sind. Ist es aus, werden nur verschlüsselte angenommen. |
| **IP-Filter aktivieren** | Blockiert Peers, deren Adressen in der Sperrliste stehen, im Format `ipfilter.dat` von eMule oder PeerGuardian P2P oder in CIDR-Zeilen wie `10.0.0.0/8`. Ein leerer Pfad bedeutet `ipfilter.dat` im Einstellungsordner. |
| **Neu laden** | Lädt die Sperrliste jetzt und zeigt, wie viele Regeln sie enthält. Der IP-Filter muss aktiviert sein. |
| **Sperrliste von dieser Adresse holen (https):** | Nimmt die Sperrliste von der `https://`-Adresse darunter statt aus der Datei oben, über den Proxy, wenn einer eingestellt ist. Die Kopie, die zuletzt von dieser Adresse heruntergeladen wurde, wird beim Start sofort verwendet, und die Liste wird einmal täglich neu heruntergeladen. Solange das Häkchen gesetzt ist, sind die Datei und **Neu laden** ausgegraut. |
| **Jetzt aktualisieren** | Lädt die Sperrliste jetzt von der Adresse herunter. Die Zeile darunter zeigt, wie viele Regeln sie enthält und wann sie heruntergeladen wurde, oder warum der letzte Download scheiterte. |

*Die Sperrliste von einer Adresse ist neu in 1.2.63.* Die Liste kann in der Form `ipfilter.dat`, PeerGuardian P2P oder CIDR (`10.0.0.0/8`) vorliegen, unkomprimiert oder gzip-komprimiert; ZIP-Archive werden nicht unterstützt. Scheitert ein Download oder ist es keine solche Liste, bleiben die verwendeten Regeln. Eine neue Adresse behält die Kopie der alten Adresse, bis eine Liste von der neuen heruntergeladen wurde, sodass ein gescheiterter erster Download nichts verliert; im Windows-Programm versucht **Jetzt aktualisieren** mit einer noch nicht übernommenen Adresse sie nur bis **OK** oder **Übernehmen**.

Mehr: [Verbindung, Ports und Firewall](Verbindung-Ports-und-Firewall#ip-filter).

## Warteschlange

![Einstellungen, Seite „Warteschlange“](images/prefs-queueing.png)

| Einstellung | Was sie bewirkt |
|---|---|
| **Höchstzahl aktiver Torrents (Upload oder Download)** | Die meisten Torrents in der Warteschlange, die gleichzeitig laufen; die übrigen warten. -1 bedeutet keine Grenze. Erzwungene Torrents laufen außerhalb der Warteschlange. |
| **Höchstzahl aktiver Downloads** | Die meisten Torrents in der Warteschlange, die gleichzeitig herunterladen. -1 bedeutet keine Grenze. |
| **Höchstzahl aktiver Seeds** | Die meisten fertigen Torrents in der Warteschlange, die gleichzeitig seeden. -1 bedeutet keine Grenze. |
| **Langsame Torrents nicht zu diesen Grenzen zählen** | Ein Torrent, der eine Minute lang weniger als etwa 2 kB/s bewegt, belegt keinen Platz mehr, sodass ein anderer starten kann. |
| **Mindestverhältnis (%)** (Seeding-Ziel) | Laden Sie diesen Prozentsatz dessen hoch, was heruntergeladen wurde; 150 bedeutet das Eineinhalbfache. 0 bedeutet kein Verhältnisziel. Bewirkt nichts, bis das Feld darunter angekreuzt ist. |
| **Mindest-Seedingzeit (Minuten)** | Seeden Sie so viele Minuten lang. 0 bedeutet kein Zeitziel. Sind beide Ziele gesetzt, müssen beide erreicht werden. Bewirkt nichts, bis das Feld darunter angekreuzt ist. |
| **Upload-Rate begrenzen auf (kB/s): [0: stoppen]** | Setzt das Seeding-Ziel in Kraft. Ist es angekreuzt, wird ein Torrent, der das Ziel erreicht hat, gestoppt (0) oder seedet weiter, mit auf die Zahl begrenzter Upload-Rate. Ist es aus, wie zu Beginn, seeden Torrents über das Ziel hinaus weiter. |

Mehr: [Warteschlange und Seeding](Warteschlange-und-Seeding).

## Zeitplaner

![Einstellungen, Seite „Zeitplaner“](images/prefs-scheduler.png)

| Einstellung | Was sie bewirkt |
|---|---|
| **Zeitplaner aktivieren** | Folgt der Wochentabelle Stunde für Stunde, in Ortszeit. Ist er aus, läuft Rookparcel immer mit voller Geschwindigkeit. |
| **Zeitplantabelle** | Eine Zelle pro Stunde der Woche; klicken oder ziehen Sie, um zwischen **Volle Geschwindigkeit**, **Begrenzt**, **Aus** und **Nur Seeding** zu wechseln. |
| **Begrenzte Upload-Rate (kB/s)** und **Begrenzte Download-Rate (kB/s)** | Die Grenzen während der begrenzten Stunden, anstelle der normalen. 0 bedeutet unbegrenzt. |
| **DHT beim Ausschalten deaktivieren** | Schaltet das DHT in den Stunden „Aus“ ebenfalls aus. Standardmäßig ein. |

Mehr: [Geschwindigkeitsbegrenzung und Zeitplan](Geschwindigkeitsbegrenzung-und-Zeitplan).

## Web-UI

![Einstellungen, Seite „Web-UI“](images/prefs-webui.png)

Diese Seite gibt es nur in der Windows-Version. Sie lässt einen Browser die Torrents dieses Fensters steuern.

| Einstellung | Was sie bewirkt |
|---|---|
| **Web-UI aktivieren** | Stellt die Weboberfläche aus diesem Fenster bereit, solange Rookparcel läuft. Sie braucht einen Benutzernamen und ein Passwort. |
| **Benutzername:** und **Passwort:** | Die Anmeldedaten. Das Passwort wird nur als Hash gespeichert; lassen Sie es leer, um das gespeicherte zu behalten. Ein neues Passwort meldet jeden anderen Browser ab. |
| **API-Schlüssel akzeptieren (Authorization: Bearer)** | Lässt Apps wie Sonarr oder Radarr die Weboberfläche mit dem Schlüssel darunter ohne Anmeldung verwenden. Standardmäßig aus. |
| **API-Schlüssel (mindestens 16 Zeichen):** | Der Schlüssel, den Apps senden. Wird nur als Hash gespeichert und nie wieder angezeigt. |
| **Authentifizierung für Clients in diesen Subnetzen umgehen** | Clients in diesen Netzwerken verwenden die Weboberfläche ohne Anmeldung. Führen Sie nur Netzwerke auf, denen Sie ebenso vertrauen wie dem Passwort. |
| **Port für eingehende Verbindungen:** | Der Port, auf dem die Weboberfläche lauscht; 8080 wird vorgeschlagen, oder der nächste freie Port. |
| **Verbindungen von anderen Computern zulassen** | Lauscht auf jeder Netzwerkadresse, sodass andere Computer und Telefone sie erreichen können. Sie brauchen außerdem eine Windows-Firewall-Regel für `rookparcel.exe`. |
| **HTTPS verwenden (selbstsigniertes Zertifikat)** | Stellt die Weboberfläche über HTTPS bereit, mit einem Zertifikat, das Rookparcel selbst erstellt. Browser warnen davor, bis Sie es akzeptieren. |
| **Server-Domains** | Hostnamen, auf die die Weboberfläche außer IP-Adressen und localhost antwortet, etwa der Name dieses Computers für eine App in Docker. |
| **Web-UI öffnen** | Öffnet die laufende Weboberfläche in Ihrem Browser. |

Mehr: [Weboberfläche](Weboberfläche), [Sonarr, Radarr und Prowlarr](Sonarr-Radarr-und-Prowlarr).

## Gaming

*Neu in 1.2.79.* Einstellungen, die den Computer und die Leitung reaktionsschnell halten, solange Rookparcel arbeitet. **Ping niedrig halten** ist von der Seite „Bandbreite“ hierher gezogen, die frühere Seite „Festplatten-E/A“ von „Erweitert“.

**Ping**

| Einstellung | Was sie bewirkt |
|---|---|
| **Ping niedrig halten (Geschwindigkeiten automatisch begrenzen)** | Misst die Verzögerung auf Ihrer Leitung und begrenzt Uploads und Downloads automatisch, solange sie wächst, für Spiele und Anrufe. Es hebt nie Ihre eigenen Grenzen an; die Statusleiste zeigt die Begrenzung als „auto“. |

**Prüfung**

| Einstellung | Was sie bewirkt |
|---|---|
| **Torrents schonend prüfen (niedrige Priorität, anderen Programmen den Vortritt lassen)** | Prüft Torrents mit der Hintergrund-Datenträgerpriorität des Systems und pausiert, solange Lesezugriffe langsamer als üblich sind, sodass der Computer reaktionsfähig bleibt. Prüfungen dauern länger, solange der Datenträger beschäftigt ist. |
| **Prüfung begrenzen auf (MB/s):** | Begrenzt die Lesegeschwindigkeit jeder Prüfung, für einen Datenträger, der auch bei schonenden Prüfungen noch hinterherhinkt. Standardmäßig aus. |
| **Gleichzeitig geprüfte Laufwerke (ein Torrent je Laufwerk):** | Wie viele Laufwerke gleichzeitig geprüft werden; Torrents auf demselben Laufwerk warten auf ihre Reihe. -1 bedeutet keine Grenze. |

**Laufwerke**

| Einstellung | Was sie bewirkt |
|---|---|
| **An den Laufwerkstyp anpassen (SSD, Festplatte, Netzwerk)** | Erkennt, ob jeder Speicherordner auf einer SSD, einer Festplatte, einem Wechsellaufwerk oder einer Netzwerkfreigabe liegt, und passt an, wie viele Datenträgeraufgaben gleichzeitig laufen. |

**Wenn die Festplatte ausgelastet ist**

| Einstellung | Was sie bewirkt |
|---|---|
| **Downloads verlangsamen (Schreibzugriffe)** | Solange ein Laufwerk Schreibzugriffe deutlich langsamer als üblich beantwortet, halbiert es die Download-Rate Schritt für Schritt, nie unter 50 kB/s, und lässt sie wieder wachsen, sobald der Datenträger ruhig ist. |
| **Uploads verlangsamen (Lesezugriffe)** | Dasselbe für Uploads, solange ein Laufwerk Lesezugriffe langsam beantwortet. |

Ein falsch erkannter Laufwerkstyp lässt sich mit `drive_overrides` auf der Seite „Erweitert“ korrigieren. Zunächst ist nur **An den Laufwerkstyp anpassen (SSD, Festplatte, Netzwerk)** an; die anderen Kästchen sind aus.

Mehr: [Niedrigen Ping halten](Niedrigen-Ping-halten), [Datenträger und Speicher](Datenträger-und-Speicher).

## Erweitert

![Einstellungen, Seite „Erweitert“](images/prefs-advanced.png)

Die Seite „Erweitert“ ist eine Tabelle der eigenen Einstellungen der Engine, unter ihren libtorrent-Namen. Geben Sie einen Teil eines Namens in **Filter:** ein, um die Liste einzugrenzen. Wählen Sie eine Zeile aus, geben Sie in **Wert:** einen neuen Wert ein (oder wählen Sie bei einem Schalter **Wahr** oder **Falsch**) und klicken Sie auf **Setzen**. **Zurücksetzen** stellt die ausgewählte Einstellung auf ihre Vorgabe zurück. Ein `*` markiert einen Wert, der von der Vorgabe abweicht. Wie überall in den Einstellungen wirkt die Änderung mit **OK** oder **Übernehmen**. Raten sind hier in Byte pro Sekunde angegeben, nicht in kB/s.

Die meisten Zeilen sind dieselben Einstellungen, die die anderen Seiten unter freundlicheren Namen zeigen. Der Rest gliedert sich in diese Gruppen:

| Gruppe | Beispiele |
|---|---|
| Identität | `user_agent`, `peer_fingerprint` (müssen selten geändert werden) |
| Listening und Portweiterleitung | `listen_interfaces`, `max_retry_port_bind`, `upnp_lease_duration`, `natpmp_gateway` |
| Warteschlange | `active_dht_limit`, `active_tracker_limit`, `inactive_down_rate`, `auto_manage_interval` |
| Verbindungen und Peers | `connection_speed`, `max_peerlist_size`, `peer_timeout`, `peer_turnover` |
| DHT | `dht_announce_interval`, `dht_upload_rate_limit`, `dht_bootstrap_nodes` und seine Schutzschalter |
| uTP und TCP | `enable_incoming_tcp`, `utp_target_delay`, `mixed_mode_algorithm`, `bt.transp_disposition` |
| Verschlüsselung | `out_enc_policy`, `in_enc_policy`, `allowed_enc_level`, `prefer_rc4` |
| Tracker | `announce_to_all_tiers`, `announce_to_all_trackers`, `num_want`, `stop_tracker_timeout` |
| Datenträger | `cache_size`, `max_queued_disk_bytes`, `file_pool_size`, `drive_overrides` |
| Web-Seeds | `max_web_seed_connections`, `urlseed_timeout`, `ssrf_mitigation` |
| Proxy | `proxy_type`, `proxy_send_host_in_connect`, `socks5_udp_send_local_ep` |
| Ping niedrig halten | `auto_limit_delay_ms`, `auto_limit_probe_hosts` |

Zeilen, die oft zu kennen lohnen:

| Einstellung | Was sie bewirkt |
|---|---|
| `listen_interfaces` | Durch Kommas getrennte Adressen und Ports, auf denen Peers angenommen werden. `0.0.0.0` und `[::]` bedeuten jede Schnittstelle. |
| `connection_speed` | Neue ausgehende Verbindungsversuche pro Sekunde, von allen Torrents geteilt. Höher findet Peers schneller, kann aber schwache Router überlasten. Vorgabe 30. |
| `announce_to_all_trackers` | Ein: Jeder Tracker einer Stufe wird kontaktiert, nicht nur der erste funktionierende. Kann mehr Peers finden, auf Kosten von mehr Tracker-Verkehr. Standardmäßig aus. |
| `upnp_lease_duration` | Sekunden, für die eine UPnP-Portweiterleitung angefordert wird. 0 fordert eine dauerhafte an, die manche Router brauchen. Vorgabe 3600. |
| `stop_tracker_timeout` | Sekunden, die Rookparcel beim Beenden auf Tracker und Router wartet. Vorgabe 5. Senken Sie den Wert, um schneller zu beenden. |
| `auto_limit_delay_ms` | Millisekunden zusätzlicher Ping, die Ping niedrig halten über dem Leerlauf-Ping der Leitung erlaubt. Niedriger hält Spiele flüssiger; höher erlaubt mehr Geschwindigkeit. Vorgabe 15. |
| `drive_overrides` | Korrigiert Laufwerke, deren Typ falsch erkannt wird, als Paare Pfad=Typ wie `D:/=hdd, /mnt/nas=network`. |
| `bt.transp_disposition` | Erlaubte Verbindungsarten als addierte Bits: 1 ausgehendes TCP, 2 ausgehendes uTP, 4 eingehendes TCP, 8 eingehendes uTP. Die Vorgabe 15 erlaubt alle; 5 ist nur TCP. |

Die vollständige Hilfe jeder Zeile erscheint im Bereich unter der Tabelle, wenn Sie sie auswählen.

## Festplatten-Cache

| Einstellung | Was sie bewirkt |
|---|---|
| **Lesezugriffe auf die Festplatte zwischenspeichern** | Hält kürzlich gelesene Teile im Speicher, was beim Seeden Festplattenzugriffe spart. Schreibzugriffe überlässt es dem Cache des Systems. |
| **Automatische Cachegröße überschreiben (MB):** | Verwendet diese Größe statt der automatischen 32 MB für den Lese-Cache. Ein größerer Cache kann bei regem Seeding helfen, auf Kosten von Arbeitsspeicher. |
| **Blöcke entfernen, die länger ungenutzt sind als (Sekunden):** | Verwirft Blöcke im Cache, die in dieser Zeit kein Peer angefordert hat. Ist es aus, bleiben sie, bis neuere Blöcke sie verdrängen. |
| **Schreibwarteschlange (kB):** | Wie viele heruntergeladene Daten auf das Schreiben warten dürfen; darüber hört Rookparcel auf, von Peers zu lesen, bis der Datenträger aufgeholt hat. Werte unter 4 MB wirken wie 4 MB. |
| **Höchstzahl offener Dateien:** | Die meisten Dateien, die gleichzeitig offen gehalten werden. Erhöhen Sie sie, wenn viele Torrents oder Dateien gleichzeitig beschäftigt sind. |
| **Festplatten-E/A-Threads (ab dem nächsten Start):** | Die Threads, die Torrent-Daten lesen und schreiben. |
| **Hash-Threads (nicht verwendet):** | Aus Kompatibilitätsgründen beibehalten; diese Version verwendet sie nicht. Prüfungen laufen eine je Laufwerk, eingestellt auf der Seite „Gaming“. |

Mehr: [Datenträger und Speicher](Datenträger-und-Speicher).

## Tracker

*Neu in 1.2.40.* Tracker, die jeder Torrent, den Sie ab jetzt hinzufügen, zusätzlich zu seinen eigenen erhält. Beide Felder sind zu Beginn aus.

| Einstellung | Was sie bewirkt |
|---|---|
| **Diese Tracker zu neuen Downloads hinzufügen:** | Fügt die Tracker im Feld hinzu, eine Adresse pro Zeile (`http://`, `https://` oder `udp://`); eine Leerzeile beginnt eine neue Stufe. Sie kommen nach den eigenen Trackern des Torrents. |
| **Die Tracker dieser Liste im Web hinzufügen (https):** | Liest eine Textdatei mit Tracker-Adressen von der `https://`-Adresse darunter, beim Start und dann einmal täglich, und fügt ihre Tracker nach denen des Felds hinzu. |
| **Jetzt aktualisieren** | Liest die Liste jetzt von der Adresse. Die Zeile unter der Schaltfläche sagt, wie viele Tracker die Liste hat und wann sie gelesen wurde, oder warum der letzte Versuch scheiterte. |

Gut zu wissen:

- **Private Torrents erhalten diese Tracker nie**, ebenso wenig Torrents, die schon in der Liste sind, von Ihnen erstellte Torrents oder aus einem anderen Client importierte. Ein Magnet-Link erhält sie, sobald seine Details eingetroffen sind und zeigen, dass er nicht privat ist.
- **Die Liste im Web wird immer nur als Liste gelesen.** Die Adresse muss mit `https://` beginnen. Es werden nur Zeilen verwendet, die Tracker-Adressen sind, höchstens 200, und eine Datei, die etwas anderes ist (eine Fehlerseite, ein Programm), wird ignoriert: Die zuletzt gute Liste bleibt in Gebrauch. Nichts in der Datei wird je ausgeführt.
- **Wer diese Adresse betreibt, wählt ihre Tracker**, und diese Tracker sehen dann Ihre Adresse und Ihre öffentlichen Downloads. Verwenden Sie eine Liste, der Sie vertrauen.
- In der Weboberfläche ändern sich die Einstellungen der Seite nur mit dem aktuellen Passwort, und die qBittorrent-kompatible API kann sie nicht ändern.
- `rookparcel-nox` führt diese Einstellungen zusammen mit denen der Weboberfläche, braucht also `--webui`, um sie zu verwenden.

## Programm ausführen

*Neu in 1.2.41.* Startet ein Programm von Ihnen, wenn ein Torrent hinzugefügt wird oder fertig ist. Beide Felder sind zu Beginn aus.

| Einstellung | Was sie bewirkt |
|---|---|
| **Dieses Programm ausführen, wenn ein Torrent hinzugefügt wird:** | Startet das Programm in der Zeile darunter für jeden neuen Torrent. Nicht für einen, der schon in der Liste war, und nicht für die beim Start geladenen Torrents. |
| **Dieses Programm ausführen, wenn der Download eines Torrents abgeschlossen ist:** | Startet das Programm für jeden Torrent, der fertig wird. Nicht nach einer erneuten Prüfung oder einem Neustart und nicht für einen Torrent, dessen Daten beim Hinzufügen schon vollständig waren. |

Schreiben Sie zuerst den Pfad des Programms, in doppelten Anführungszeichen, wenn er Leerzeichen enthält, dann seine Argumente, zum Beispiel `"C:\Tools\notify.exe" --name "%N" --path "%F"`. In den Argumenten stehen diese Codes für die Angaben des Torrents:

| Code | Steht für |
|---|---|
| `%N` | den Namen des Torrents |
| `%L` | sein Label |
| `%F` | wo seine Daten liegen: die Datei oder der Ordner eines Torrents mit mehreren Dateien |
| `%D` | den Ordner, in dem er gespeichert wurde |
| `%Z` | seine Größe in Byte |
| `%T` | seinen aktuellen Tracker |
| `%I`, `%J` | seinen Info-Hash, v1 und v2 (`-`, wenn er keinen hat) |
| `%K` | seine ID: den v1-Hash oder den gekürzten v2-Hash |
| `%%` | ein Prozentzeichen |

Gut zu wissen:

- **Das Programm wird direkt gestartet, ohne Befehls-Shell.** Pipes, `>` und `&&` bewirken nichts; um sie zu verwenden, führen Sie ein Skript aus und setzen sie in das Skript. Um ein Skript auszuführen, nennen Sie zuerst seinen Interpreter, wo das System einen braucht (`powershell -File ...`, `/bin/sh /path/script.sh`).
- **Der Name eines Torrents kann nie zu einem Befehl werden.** Jeder Code wird in sein eigenes Argument gesetzt, mit oder ohne Anführungszeichen, und der eigene Pfad des Programms nimmt keinen Code an. Ein Name kann aber mit einem Bindestrich beginnen, den manche Programme als Option lesen: Schreiben Sie `--name=%N` oder setzen Sie `--` davor, wo das wichtig ist.
- **Keine Codes für eine Shell oder einen Interpreter, der angewiesen ist, eine Zeichenkette auszuführen.** Ein solches Programm würde den Namen als Befehle lesen, daher wird eine Zeile wie `cmd /c ... %N`, `powershell -Command ...`, `sh -c "... %N"`, `python -c`, `perl -e`, `node -e`, `osascript -e`, `ssh host ... %N` oder `wsl bash -c` mit einem Code darin abgelehnt, auch hinter `env`, `nohup`, `nice`, `timeout`, `sudo` und ähnlichen Wrappern (deren Programm ebenfalls keinen Code annimmt). Ebenso ein Code im Programmtext von `awk` oder `sed`, in einem `find -exec`-Befehl, der eine Shell ausführt, und nach einer Shell oder einem Interpreter, den ein anderes Programm startet (`gnome-terminal -- bash -c "... %N"`). Programme, die eine Zeichenkette ihrer Zeile als Code ausführen, werden mit einem Code irgendwo darin ebenfalls abgelehnt: `script`, `expect`, `forfiles`, `wmic`, Editoren wie `vim` und `emacs` und Debugger wie `gdb`. Nennen Sie stattdessen das Skript und reichen Sie die Codes an es weiter: `powershell -File C:\Tools\done.ps1 "%N"`, `/bin/sh /path/done.sh "%N"`, `python /path/done.py "%N"`, `wsl -e /path/done.sh "%N"`. Die eigenen Optionen anderer Programme sind deren Sache: `notify-send -e "%N"`, `rsync -c "%F" /backup` und `/path/done.sh -c "%L"` sind in Ordnung. Eine Zeile, die eine ältere Version gespeichert hat und die jetzt abgelehnt wird, wird ausgeschaltet, und das Protokoll sagt das (die Zeile und warum) beim Start.
- Das Programm erhält kein Fenster und keine Eingabe, höchstens vier laufen gleichzeitig, und jeder Start und jedes Ende wird ins Protokoll geschrieben.
- Das Programm läuft auf dem Computer, der die Torrents hat: In einem Fenster, das mit einem entfernten `rookparcel-nox` verbunden ist, bewirken die Zeilen dieser Seite nichts; stellen Sie sie auf dem Server ein.
- **Wer die Zeilen ändern kann.** Wer sie ändern kann, kann Programme unter Ihrem Konto ausführen, daher: die Desktop-Programme auf ihrem eigenen Computer; die Weboberfläche nur aus einem Browser auf demselben Computer, mit dem aktuellen Passwort; nie die qBittorrent-kompatible API und nie eine Fernverbindung. Ein Server, den Sie nur von anderswo erreichen (Docker zum Beispiel), braucht `--webui-remote-run-program` in seiner Befehlszeile: siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben). Überlegen Sie, bevor Sie es geben: Mit ihm ist das Passwort der Weboberfläche alles, was einen Besucher davon trennt, Programme auf dem Server auszuführen.
- **Hinter einem Reverse-Proxy.** Wird die Weboberfläche über einen Proxy oder Tunnel auf dem Server selbst erreicht (nginx, HAProxy, stunnel, ein SSH-Tunnel), sieht der Server jeden Besucher als Browser auf demselben Computer, sofern der Proxy nichts anderes sagt. Lassen Sie den Proxy die Adresse des Besuchers hinzufügen (`X-Forwarded-For`; für nginx: `proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;`). Ohne diesen Header kann jeder, der den Proxy erreicht und das Passwort kennt, diese Zeilen ändern, genau wie bei `--webui-remote-run-program`.

## UI-Sperre

Die PIN, die die Fenster von Rookparcel sperrt, und der Schalter, der die Sperre ausschaltet. Anders als der Rest der Einstellungen wirkt hier alles sofort: **OK**, **Übernehmen** und **Abbrechen** ändern daran nichts. *Neu in 1.2.63; vorher wurde die PIN unter **Optionen > Sperr-PIN** festgelegt.*

| Einstellung | Was sie bewirkt |
|---|---|
| **Sperren von Rookparcel mit einer PIN erlauben** | Standardmäßig ein. Ist es aus, gibt es kein **Datei > Rookparcel sperren**, kein <kbd>Ctrl</kbd>+<kbd>L</kbd> und kein **Rookparcel sperren** im Menü des Infobereich-Symbols. Das Ausschalten fragt nach der PIN, falls eine festgelegt ist, und entfernt sie. |
| **PIN festlegen...** | Fragt zweimal nach einer neuen PIN aus 4 bis 6 Ziffern. Nur, solange keine PIN festgelegt ist. |
| **PIN ändern...** | Fragt nach der aktuellen PIN, dann zweimal nach einer neuen. |
| **PIN entfernen...** | Fragt nach der aktuellen PIN und entfernt sie dann. **Rookparcel sperren** fragt dann vor dem Sperren nach einer neuen PIN. |

Die Zeile unter den Schaltflächen sagt, ob eine PIN festgelegt ist. Mehr: [Rookparcel sperren](Rookparcel-sperren).

## Unterschiede in der GTK-Version

Die GTK-Version (Linux, BSD) hat dieselben Seiten und Einstellungen, mit diesen Unterschieden:

- Es gibt keine Seite **Web-UI**.
- **Oberfläche** hat keine Einstellung **Design**: GTK folgt dem Design des Desktops ([Dunkler Modus](Dunkler-Modus)). Die Felder für Infobereich und Benachrichtigung (**Sprechblasen im Infobereich anzeigen**, **Beim Schließen in den Infobereich**, **Beim Minimieren in den Infobereich**) erscheinen nur, solange ein Infobereich-Symbol auf dem Bildschirm ist.
- **Allgemein** hat **Desktop-Integration** statt Windows-Integration und keine Schaltfläche **Standard-Apps...**. In einem Flatpak sind die Verknüpfungsfelder unter einem Hinweis ausgegraut: Wählen Sie das Standardprogramm in den Einstellungen Ihres Desktops. Die Gruppe **Privatsphäre** wird in einem Flatpak nicht angezeigt, das Flathub aktualisiert.
- <kbd>Escape</kbd> schließt das Fenster wie **Abbrechen**.

## Unterschiede unter macOS

Die macOS-App öffnet die Einstellungen über **Rookparcel > Einstellungen** (<kbd>⌘</kbd>+<kbd>,</kbd>). Sie hat die Seiten der GTK-Version, in einer Seitenleiste aufgelistet, und diese Unterschiede:

- Jede Änderung wirkt sofort, wie in anderen Mac-Apps: Es gibt kein **OK**, **Abbrechen** oder **Übernehmen**. Ein Feld zum Ankreuzen oder eine Liste wirkt, wenn Sie es ändern; ein Text- oder Zahlenfeld, wenn Sie <kbd>Return</kbd> drücken, das Feld verlassen oder das Fenster schließen.
- Ein Wert, der nicht erlaubt ist, etwa ein Port außerhalb des Bereichs, wird rot unter seinem Feld angezeigt, und der letzte gute Wert bleibt in Gebrauch. Das Schließen des Fensters stellt den guten Wert im Feld wieder her.
- Die Verknüpfungsfelder funktionieren nur in `Rookparcel.app` und nur im Standardprofil.

*Die native macOS-App ist neu in 1.2.19.*

## Unterschiede in der Weboberfläche

Die Weboberfläche (von `rookparcel-nox` oder die, die das Windows-Fenster bereitstellt) hat ein Fenster **Einstellungen** mit denselben Seiten. Geänderte Werte auf ihrer Seite „Erweitert“ werden fett statt mit einem `*` angezeigt. Weitere Unterschiede:

- **Allgemein**: Die Felder für Verknüpfungen und Autostart sind ausgegraut, mit dem Hinweis „Dateiverknüpfungen und Autostart gehören zum Desktop-Client.“ `rookparcel-nox` hat für seine Update-Prüfung eine eigene Gruppe **Privatsphäre**; die Weboberfläche, die ein Windows-Fenster bereitstellt, nicht.
- **Oberfläche** hat nur **Löschen von Torrents bestätigen**, die beiden Design-Einstellungen, **Download nicht automatisch starten**, **Ein Fenster mit den Dateien im Torrent anzeigen** und **Dateien mit diesen Namen nicht herunterladen:**.
- **BitTorrent**: Das Ändern der Adresse des IP-Filters oder des Felds darüber braucht das aktuelle Passwort, eingegeben in **Aktuelles Passwort:** auf der Seite „Web-UI“. Die Weboberfläche, die das Windows-Fenster bereitstellt, hat keine Adresse, kein **Jetzt aktualisieren** und keine Statuszeile: Die Einstellungen des Fensters selbst enthalten sie.
- **Bandbreite** hat in `rookparcel-nox` **Alternative Ratenbegrenzungen**: **Alternative Upload-Rate**, **Alternative Download-Rate** und **Die alternativen Ratenbegrenzungen jetzt verwenden**, was auch Apps umschalten, die den Modus der Geschwindigkeitsgrenzen von qBittorrent umschalten.
- **Warteschlange** hat das Feld **Torrent stoppen oder seine Upload-Rate begrenzen** über **Upload-Rate begrenzen auf (kB/s): [0: stoppen]**; das Feld setzt das Seeding-Ziel in Kraft.
- **Web-UI** fügt **Aktuelles Passwort:** hinzu (nötig, um die Anmeldedaten und die Sicherheitseinstellungen zu ändern), **Authentifizierung für Clients auf localhost umgehen**, **Lauschadresse:** (zur Ansicht angezeigt), **Prüfung des Host-Headers aktivieren**, **Schutz vor Cross-Site Request Forgery (CSRF) aktivieren** und eine Gruppe **Sicherheit** mit **Client nach aufeinanderfolgenden Fehlversuchen sperren:**, **Sperrdauer (Sekunden):** und **Sitzungs-Timeout (Sekunden):**.
- <kbd>Ctrl</kbd>+<kbd>Enter</kbd> funktioniert als **OK**; <kbd>Enter</kbd> im Wertfeld von „Erweitert“ setzt den Wert.

Siehe [Weboberfläche](Weboberfläche) und [rookparcel-nox betreiben](rookparcel-nox-betreiben).
