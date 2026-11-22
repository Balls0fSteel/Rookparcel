*Dies ist eine Übersetzung der englischen Seite [Connection, ports and firewall](https://github.com/Balls0fSteel/Rookparcel/wiki/Connection-ports-and-firewall). Maßgeblich ist die englische Seite.*

# Verbindung, Ports und Firewall

Diese Seite behandelt, wie andere Peers Rookparcel erreichen: den Port für eingehende Verbindungen, die Portweiterleitung mit UPnP und NAT-PMP, die Windows-Firewall, wie Sie erkennen, ob eingehende Verbindungen funktionieren, wie Sie mit „Nur dieses Netzwerk verwenden“ den Torrent-Verkehr in einem VPN halten, Proxys, Verschlüsselung, den IP-Filter und seine Sperrliste von einer Webadresse sowie das DHT, den Peer-Austausch und die Lokale Peer-Erkennung.

Die meisten dieser Einstellungen finden Sie unter **Optionen > Einstellungen > Verbindung** und **Optionen > Einstellungen > BitTorrent**.

![Einstellungen, Seite „Verbindung“](images/prefs-connection.png)

## Warum eingehende Verbindungen wichtig sind

Rookparcel verbindet sich nach außen mit Peers, und Peers verbinden sich nach innen mit Rookparcel. Downloads funktionieren allein mit ausgehenden Verbindungen, aber ein Rookparcel, das andere erreichen können, findet mehr Peers, und zwei Peers, die beide nicht erreichbar sind, können überhaupt nicht miteinander sprechen. Für gute Geschwindigkeiten und fürs Seeding lassen Sie Peers herein.

## Der Port für eingehende Verbindungen

Unter **Port für eingehende Verbindungen** auf der Seite „Verbindung“:

| Einstellung | Bedeutung |
|---|---|
| **Port für eingehende Verbindungen:** | Der Port, mit dem sich andere Peers verbinden. Eine Änderung gilt sofort. |
| **Zufälliger Port** | Trägt einen zufälligen Port von 10000 bis 60000 ein. Er wird verwendet, sobald Sie übernehmen. |
| **Port bei jedem Start zufällig wählen** | Wählt bei jedem Start einen neuen zufälligen Port von 10000 bis 60000. Ein im Router von Hand weitergeleiteter Port passt dann nicht mehr. Standardmäßig aus. |
| **UPnP-Portweiterleitung aktivieren** | Bittet einen UPnP-Router, den Port an diesen Computer weiterzuleiten. Standardmäßig an. |
| **NAT-PMP-Portweiterleitung aktivieren** | Bittet den Router, den Port über NAT-PMP oder PCP weiterzuleiten, was viele Router neben UPnP oder statt dessen anbieten. Standardmäßig an. |

Das Standardprofil lauscht auf Port 6881. Ein neues Profil oder eine portable Kopie wählt beim ersten Start einen zufälligen Port von 10000 bis 60000, damit es nicht mit einem anderen Rookparcel kollidiert. Siehe [Profile](Profile).

Rookparcel lauscht auf TCP und UDP am selben Port: UDP trägt uTP-Peers und das DHT.

### Den Port im Router weiterleiten

Sind UPnP oder NAT-PMP an und unterstützt Ihr Router sie, ist nichts zu tun: Rookparcel bittet den Router, den Port weiterzuleiten, und erneuert die Anfrage. Hat Ihr Router sie ausgeschaltet oder spielt er verrückt, schalten Sie beide Kästchen aus und leiten den Port von Hand in den eigenen Einstellungen des Routers weiter: TCP und UDP, an die Adresse dieses Computers. Lassen Sie dann **Port bei jedem Start zufällig wählen** aus, damit der Port der bleibt, den Sie weitergeleitet haben.

## Die Windows-Firewall

Rookparcel fügt von selbst keine Regel der Windows-Firewall hinzu. Windows fragt normalerweise beim ersten Mal, wenn ein Programm auf eingehende Verbindungen lauscht; erlauben Sie dort Rookparcel.

Haben Sie diese Frage weggeklickt oder hat nichts gefragt, fügen Sie über eine als Administrator ausgeführte Eingabeaufforderung eine Regel für das Programm hinzu. Für eine Installation für einen Benutzer:

```bat
netsh advfirewall firewall add rule name="Rookparcel" dir=in action=allow program="%LOCALAPPDATA%\Programs\Rookparcel\rookparcel.exe"
```

Bei einer Installation für alle Benutzer lautet der Pfad `C:\Program Files\Rookparcel\rookparcel.exe`; beim Zip dort, wo Sie `rookparcel.exe` entpackt haben. Der Port der Web-UI braucht dieselbe Regel, wenn andere Computer sie erreichen sollen: siehe [Web-UI](Weboberfläche). Für [rookparcel-nox](rookparcel-nox-betreiben), das als Dienst läuft, siehe dessen Seite.

Erlauben Sie unter Linux und den BSDs den Port in Ihrer Firewall, falls Sie eine betreiben.

## Prüfen, ob Peers sich verbinden können

Der linke Teil der Statusleiste sagt es Ihnen:

| Statusleiste | Bedeutung |
|---|---|
| **Netzwerk OK** | Mindestens ein Peer hat sich von außen verbunden. Der Port funktioniert. |
| **Keine eingehenden Verbindungen** | Noch kein Peer hat sich von außen verbunden. In den ersten Minuten nach einem Start normal; bleibt es so, ist der Port nicht erreichbar. |
| **Port 6881 belegt** (mit Ihrer Portnummer) | Ein anderes Programm, oft ein anderes Rookparcel-Profil, belegt den Port, und Rookparcel lauscht auf einem späteren Port, an dem nichts weitergeleitet wird. Klicken Sie darauf, um die Einstellungen bei „Verbindung“ zu öffnen und einen anderen Port zu wählen. |

Vor diesen Worten können zwei Hinweise stehen. **Übertragungen angehalten: Einstellungen bestätigen** bedeutet, dass Rookparcel nach einem Versionswechsel alle Übertragungen anhält, bis Sie die Einstellungen bestätigen: siehe [Rookparcel aktualisieren](Rookparcel-aktualisieren). **Warten auf das Netzwerk** bedeutet, dass „Nur dieses Netzwerk verwenden“ keinen passenden Adapter findet: siehe den Abschnitt unten.

*„Port ... belegt“ ist neu in 1.1.45.*

Halten Sie den Zeiger über diesen Teil, um **Ihre Adresse laut Trackern:** zu sehen, die öffentliche Adresse, die Tracker zurückmelden. Der Reiter **Allgemein** zeigt dieselbe Zeile. *Neu in 1.1.58.*

Rookparcel hat keinen eingebauten Porttest. Bleibt der Status bei aktiven Torrents auf „Keine eingehenden Verbindungen“:

1. Prüfen Sie, ob die Firewall Rookparcel erlaubt (oben).
2. Prüfen Sie, ob UPnP oder NAT-PMP in Rookparcel und im Router an ist, oder leiten Sie den Port von Hand weiter.
3. Ist „Nur dieses Netzwerk verwenden“ auf ein VPN gesetzt, können Peers Sie nur über die eigene Portweiterleitung des VPN erreichen, falls es eine hat: Rookparcel verlangt von einem VPN-Adapter keine Portweiterleitung.

Siehe auch [Fehlerbehebung und FAQ](Fehlerbehebung-und-FAQ).

## Nur dieses Netzwerk verwenden (VPNs)

*Neu in 1.1.58.*

„Nur dieses Netzwerk verwenden“ hält den gesamten Torrent-Verkehr auf einem Netzwerkadapter, typischerweise einem VPN. Peers, das DHT, Tracker und Webseeds gehen alle darüber und über nichts sonst. Fällt das VPN aus, warten die Torrents, statt über Ihre normale Verbindung weiterzumachen.

1. Verbinden Sie Ihr VPN.
2. Öffnen Sie **Optionen > Einstellungen > Verbindung**.
3. Wählen Sie unter **Nur dieses Netzwerk verwenden** bei **Netzwerk:**
   - **Beliebiger VPN-Adapter**: jeder Adapter, der wie ein VPN-Tunnel aussieht (WireGuard, OpenVPN, der eigene einer VPN-App). Das wollen die meisten.
   - Einen Adapter nach seinem Namen, aus der Liste der Adapter dieses Computers.
   - **Adressbereich (unten)**, und tippen Sie dann den Bereich bei **Adressbereich:** ein, etwa `10.2.0.0/16` oder `fd00::/8`.
   - **Beliebiges Netzwerk (aus)** schaltet es aus; das ist die Voreinstellung.
4. Lassen Sie **Torrents anhalten, solange dieses Netzwerk fehlt** angehakt.
5. Klicken Sie auf **OK** oder **Übernehmen**.

Was Sie wählen sollten:

- **Beliebiger VPN-Adapter** übersteht es, wenn die VPN-App ihren Adapter umbenennt oder neu anlegt, was viele bei jedem Verbinden oder Update tun.
- **Adressbereich** passt zu einem VPN, das immer Adressen aus demselben Bereich vergibt: Der Adapter, der eine Adresse darin hält, trägt den Verkehr, wie er auch heißt.
- **Ein Adapter nach Name** ist die engste Wahl. Ein Adapter, der gerade nicht da ist, wird in der Liste als „(nicht verbunden)“ angezeigt.

Die Regel wird bei jeder Netzwerkänderung neu geprüft, sodass ein VPN, das sich unter einer neuen Adresse wieder verbindet, nachverfolgt wird.

**Solange das Netzwerk fehlt:** Ist **Torrents anhalten, solange dieses Netzwerk fehlt** an, warten laufende Torrents. Ihr Status zeigt **Warten auf das Netzwerk**, die Statusleiste beginnt mit **Warten auf das Netzwerk**, und das Protokoll sagt, welche Regel nichts gefunden hat. Ist das VPN zurück, machen sie von selbst weiter. Stoppen und Starten funktionieren währenddessen weiter. Bei ausgeschaltetem Kästchen wird nichts angehalten, aber auch kein Torrent-Verkehr kann hinaus, da jede Verbindung den passenden Adapter braucht.

**Prüfen, ob es funktioniert:** Halten Sie den Zeiger über den Netzwerkteil der Statusleiste oder sehen Sie in den Reiter **Allgemein**: **Ihre Adresse laut Trackern:** sollte die Adresse des VPN zeigen, nicht Ihre eigene. Es steht „noch nicht bekannt“, bis ein Tracker geantwortet hat.

Gut zu wissen:

- Trägt das VPN nur IPv4, verwendet Rookparcel für Torrents kein IPv6, solange es daran gebunden ist, sodass IPv6 nicht um den Tunnel herum auslaufen kann.
- Die Web-UI und Fernverbindungen zu Rookparcel sind nicht gebunden, Sie erreichen sie also weiterhin über Ihr normales Netzwerk.
- Proxy-Verbindungen sind ebenfalls nicht gebunden. Der Suchhelfer ist ein eigenes Programm und nicht gebunden; siehe [Torrents suchen unter Windows](Torrents-suchen-unter-Windows#gut-zu-wissen).
- In der Web-UI befindet sich dieselbe Gruppe unter **Einstellungen > Verbindung**. Für rookparcel-nox in der Befehlszeile: `--network vpn`, `--network adapter:ID`, `--network range:10.2.0.0/16` oder `off`; und `--no-pause-when-missing`, um Torrents ungebunden weiterlaufen zu lassen.

## Proxyserver

Unter **Proxyserver** auf der Seite „Verbindung“. Die Proxy-Felder sind grau, solange **Typ:** auf **(keiner)** steht, der Voreinstellung.

| Einstellung | Bedeutung |
|---|---|
| **Typ:** | **Socks4**, **Socks5** oder **HTTP** (das alles außer einfachen http-Anfragen per CONNECT tunnelt). **(keiner)** verbindet direkt. |
| **Proxy:** und **Port:** | Der Hostname oder die IP-Adresse des Proxyservers und sein Port, etwa 1080 bei vielen SOCKS-Servern. |
| **Authentifizierung** mit **Benutzername:** und **Passwort:** | Meldet sich beim Proxy an. Ein SOCKS4-Proxy bekommt nur den Benutzernamen. |
| **Hostnamen über den Proxy auflösen** | Lässt den Proxy die Namen von Trackern und Webseeds nachschlagen, sodass Ihr eigener DNS-Server sie nicht sieht. |
| **Proxy für Peer-Verbindungen verwenden** | Schickt Verbindungen zu Peers und Webseeds durch den Proxy. Eingehende Verbindungen werden dann abgelehnt, und Portweiterleitung und Lokale Peer-Erkennung bleiben aus. uTP und das DHT brauchen einen SOCKS5-Proxy. |
| **Proxy für Tracker-Verbindungen verwenden** | Schickt Tracker-Ankündigungen durch den Proxy, sodass Tracker die Adresse des Proxys sehen. |

Die letzten drei sind standardmäßig angehakt und zählen erst, wenn ein Proxy-Typ gewählt ist. Ein SOCKS5-Proxy eines VPN-Anbieters ist eine Möglichkeit, ein VPN ohne Adapter zu nutzen; „Nur dieses Netzwerk verwenden“ ist die andere.

## Protokollverschlüsselung

Unter **Optionen > Einstellungen > BitTorrent**, bei **Protokollverschlüsselung**:

| Einstellung | Bedeutung |
|---|---|
| **Ausgehend:** | Wie die Verbindungen verschlüsselt werden, die Rookparcel öffnet. **Deaktiviert**: nie. **Aktiviert** (die Voreinstellung): einfache und verschlüsselte Versuche wechseln sich pro Peer ab, und es bleibt bei dem, was funktioniert hat. **Erzwungen**: nur verschlüsselt, wobei Peers verloren gehen, die nicht verschlüsseln können. |
| **Eingehende unverschlüsselte Verbindungen erlauben** | Akzeptiert eingehende Verbindungen, die nicht verschlüsselt sind. Aus werden nur verschlüsselte akzeptiert, wobei Peers verloren gehen, die nicht verschlüsseln können. Standardmäßig an. |

Ändern Sie diese, wenn Ihr Provider den BitTorrent-Verkehr ausbremst. Die Seite „Erweitert“ hat die feineren Einstellungen (`out_enc_policy`, `in_enc_policy`, `allowed_enc_level`, `prefer_rc4`).

![Einstellungen, Seite „BitTorrent“](images/prefs-bittorrent.png)

## DHT, Peer-Austausch, Lokale Peer-Erkennung und uTP

Unter **Grundlegende BitTorrent-Funktionen** auf der Seite „BitTorrent“, alle standardmäßig an:

| Einstellung | Bedeutung |
|---|---|
| **DHT-Netzwerk aktivieren** | Findet Peers über das DHT, ein Netzwerk von BitTorrent-Clients, ohne Tracker. Die meisten Magnet-Links brauchen es. |
| **Lokale Peer-Erkennung aktivieren** | Findet Peers für dieselben Torrents in Ihrem lokalen Netzwerk, sodass sie Daten direkt mit Netzwerkgeschwindigkeit austauschen. |
| **Peer-Austausch aktivieren** | Lässt verbundene Peers einander von weiteren Peers erzählen. Eine Änderung gilt für jeden Torrent. |
| **uTP aktivieren (UDP-Transport mit Bandbreitensteuerung)** | Verbindet sich mit Peers und akzeptiert sie über uTP, einen UDP-Transport, der langsamer wird, wenn das Netzwerk ausgelastet ist, ebenso wie über TCP. |

Private Torrents verwenden nie das DHT, den Peer-Austausch oder die Lokale Peer-Erkennung, was diese Kästchen auch sagen. Die Eigenschaften eines Torrents können das DHT und den Peer-Austausch für diesen Torrent allein ausschalten (**DHT aktivieren** und **Peer-Austausch** unter **Sonstige Einstellungen**).

Die Statusleiste zeigt **DHT: N Knoten**, wie viele DHT-Knoten Rookparcel kennt.

Um nur einige Verbindungsarten zuzulassen, etwa nur TCP, verwenden Sie `bt.transp_disposition` auf der Seite „Erweitert“: 1 ausgehend TCP, 2 ausgehend uTP, 4 eingehend TCP, 8 eingehend uTP, zusammengezählt; 15 (der Standard) erlaubt alle, 5 ist nur TCP.

## IP-Filter

Unter **IP-Filter** auf der Seite „BitTorrent“ kann Rookparcel Peers nach Adresse sperren, anhand einer Sperrliste im Format eMule `ipfilter.dat`, PeerGuardian P2P oder CIDR. Die Liste kann eine Datei auf Ihrem Computer sein oder eine Datei im Web, die Rookparcel herunterlädt und aktuell hält.

Jede Zeile einer Liste ist eine Regel:

| Format | Beispielzeile |
|---|---|
| eMule `ipfilter.dat` | `001.002.004.000 - 001.002.004.255 , 000 , Some name` |
| PeerGuardian P2P | `Some name:1.2.4.0-1.2.4.255` |
| CIDR, IPv4 oder IPv6 | `10.0.0.0/8` oder `fd00::/8` |

*CIDR-Zeilen sind neu in 1.2.63.* Sie funktionieren auch in einer Datei auf Ihrem Computer.

### Eine Listendatei auf Ihrem Computer

1. Haken Sie **IP-Filter aktivieren** an.
2. Wählen Sie die Sperrlisten-Datei im Feld darunter. Bleibt es leer, verwendet Rookparcel `ipfilter.dat` in seinem Einstellungsordner.
3. Klicken Sie auf **Neu laden**, um sie jetzt zu laden und zu sehen, wie viele Regeln sie enthält („N Regeln geladen.“), oder auf **OK**, um sie mit den anderen Einstellungen zu laden.

Die Liste wird beim Start und bei „Übernehmen“ geladen. Das Protokoll schreibt „IP-Filter: N Regeln aus ...“ oder den Fehler. Lässt sich eine neue Liste nicht lesen, bleiben die zuvor geladenen Regeln in Kraft.

In der Web-UI gibt es keine Schaltfläche „Neu laden“: Die Liste wird beim Übernehmen geladen.

### Eine Liste von einer Webadresse

*Neu in 1.2.63.* Rookparcel kann die Sperrliste aus dem Web herunterladen und aktuell halten:

1. Haken Sie **IP-Filter aktivieren** an.
2. Haken Sie **Sperrliste von dieser Adresse holen (https):** an und tippen Sie die Adresse der Liste in das Feld darunter. Sie muss mit `https://` beginnen.
3. Klicken Sie auf **Jetzt aktualisieren**, um sie sofort herunterzuladen, oder auf **OK**. Die Zeile unter der Schaltfläche zeigt das Ergebnis.

Solange die Adresse verwendet wird, ersetzt sie die Datei: Das Dateifeld und **Neu laden** sind ausgegraut. Was Sie wissen sollten:

- **Nur `https://`.** Eine Adresse, die Rookparcel auf eine einfache `http://`-Adresse weiterleitet, wird abgelehnt. Der Download geht durch Ihren Proxy, wenn Einstellungen > Verbindung einen setzt.
- **Formate.** Die drei oben genannten Formate, unkomprimiert oder gzip-komprimiert. Zip-Archive werden nicht unterstützt. Ein Download darf bis zu 32 MiB groß sein.
- **Wie oft.** Einmal am Tag und eine Stunde nach einem fehlgeschlagenen Download. **Jetzt aktualisieren** lädt sie jederzeit herunter.
- **Beim Start.** Rookparcel bewahrt den letzten guten Download in seinem Einstellungsordner auf (`ipfilter-url.dat`) und setzt ihn beim nächsten Start sofort in Kraft, bevor es etwas herunterlädt.
- **Eine neue Adresse.** Die Kopie der Liste der alten Adresse bleibt im Einstellungsordner, bis eine Liste von der neuen Adresse heruntergeladen und geprüft wurde, sodass ein Tippfehler oder ein fehlgeschlagener Download nichts verliert, und beim Zurückgehen zur alten Adresse wird deren Kopie wieder verwendet. Eine Kopie wird nur für die Adresse verwendet, von der sie stammt: Bei einem Start mit einer neuen Adresse hat Rookparcel keine Sperrliste in Kraft, bis die neue eintrifft, und das Protokoll sagt es. Im Windows-Programm probiert **Jetzt aktualisieren** mit einer Adresse, die Sie noch nicht übernommen haben, sie nur aus: Die Zeile zeigt das Ergebnis, und nichts ändert sich bis **OK** oder **Übernehmen**.
- **Wenn ein Download fehlschlägt** oder etwas bringt, das keine Sperrliste ist (zum Beispiel eine Fehlerseite), bleiben die bereits geltenden Regeln, und die Statuszeile sagt „Der letzte Download ist fehlgeschlagen: ...“. Auch das Protokoll schreibt den Fehler.
- **Statuszeile.** „N Regeln, heruntergeladen“ mit Datum und Uhrzeit oder „Die Liste wurde noch nicht heruntergeladen.“, bis der erste Download gelingt. Bis dahin bleiben die bereits geltenden Regeln.
- **Die Prüfung der Einstellungen.** Solange Rookparcel nach einem Versionswechsel die Übertragungen anhält, bis Sie die Einstellungen bestätigen ([Rookparcel aktualisieren](Rookparcel-aktualisieren)), wartet auch der Download.

Verwenden Sie eine Adresse, der Sie vertrauen: Wer die Liste kontrolliert, entscheidet, mit welchen Peers Rookparcel nicht spricht.

In der Web-UI braucht das Ändern des Felds oder der Adresse das aktuelle Passwort (**Aktuelles Passwort:** auf der Seite der Web-UI), weil der Server jede Adresse herunterlädt, die gesetzt ist. Die Seite zeigt die Adresse ohne Benutzername, Passwort oder Abfrage (die Zugangsdaten eines Abonnements stehen dort oft); bleibt sie wie sie ist, wird die gespeicherte Adresse beibehalten. Um die Zugangsdaten zu entfernen, leeren Sie das Feld, übernehmen Sie und tippen Sie die Adresse dann erneut ein. Die Web-UI, die das Windows-Programm bereitstellt, lässt die Adresse weg: Sie gehört zu den eigenen Einstellungen des Fensters. Die zu qBittorrent kompatible API kann sie nicht ändern.

### Tracker und das DHT

Standardmäßig gilt der Filter auch für das DHT und für Tracker an gesperrten Adressen (`apply_filter_to_dht` und `apply_ip_filter_to_trackers` auf der Seite „Erweitert“). Schalten Sie die für Tracker aus, wenn ein Tracker, den Sie brauchen, in einem gesperrten Bereich liegt.

## Im GTK-Fenster und in der Web-UI

- **GTK-Fenster**: dieselben Seiten und Einstellungen. Der Netzwerkteil der Statusleiste sagt dasselbe, und ein Klick auf „Port ... belegt“ öffnet die Einstellungen bei „Verbindung“.
- **macOS-App**: dieselben Seiten, die sofort gelten, ohne **OK** ([Einstellungen im Überblick](Einstellungen-im-Überblick#unterschiede-unter-macos)).
- **Web-UI**: dieselben Seiten unter **Einstellungen**; ihre Statusleiste zeigt den Netzwerkteil mit demselben Tooltip.
- **rookparcel-nox**: `--port N` setzt den Port für eingehende Verbindungen (Standard 6881), `--listen` setzt die Listen-Schnittstellen vollständig, `--proxy TYPE://[USER:PASS@]HOST:PORT` setzt einen Proxy, `--ip-filter FILE` lädt eine Sperrlisten-Datei (die Adresse wird in den Einstellungen der Web-UI gesetzt), und `--network` setzt „Nur dieses Netzwerk verwenden“. Ein Dienst unter Windows braucht eine Firewall-Regel für `rookparcel-nox.exe`: siehe [rookparcel-nox betreiben](rookparcel-nox-betreiben).

## Verwandte Seiten

- [Geschwindigkeitsbegrenzung und Zeitplaner](Geschwindigkeitsbegrenzung-und-Zeitplan)
- [Profile](Profile) (Ports für mehrere Rookparcel)
- [Fehlerbehebung und FAQ](Fehlerbehebung-und-FAQ)
- [Einstellungen im Überblick](Einstellungen-im-Überblick)
