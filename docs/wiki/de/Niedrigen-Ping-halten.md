*Dies ist eine Übersetzung der englischen Seite [Keep ping low](https://github.com/Balls0fSteel/Rookparcel/wiki/Keep-ping-low). Maßgeblich ist die englische Seite.*

# Niedrigen Ping halten

„Ping niedrig halten“ ist der „Gamer-Modus“ von Rookparcel: Er beobachtet die Verzögerung auf Ihrer Internetleitung und hält die Geschwindigkeiten von Rookparcel unten, solange die Verzögerung wächst, damit Spiele, Videoanrufe und das Surfen im Web reaktionsschnell bleiben, während Torrents laufen. Diese Seite erklärt, was die Funktion tut, wie Sie sie einschalten und was Sie sehen, während sie arbeitet.

## Warum Torrents Ihren Ping erhöhen

Der Upload ist meist der engste Teil Ihrer Leitung. Sendet Rookparcel schneller, als die Leitung tragen kann, warten die Pakete in einer Warteschlange im Modem oder Router, und jedes andere Paket wartet hinter ihnen: das eines Spiels, eines Anrufs, einer Webseite. Diese Warteschlange bemerken Sie als Lag. Etwas langsamer als die Leitung zu senden hält die Warteschlange leer und den Ping auf seinem Ruhewert. Beim Download gibt es dasselbe Problem am Ende der Leitung bei Ihrem Provider.

„Ping niedrig halten“ muss nicht wissen, welches Spiel läuft. Es misst die Warteschlange selbst und funktioniert daher für jedes Spiel, jeden Anruf und jedes andere Programm.

## Einschalten

1. Öffnen Sie **Optionen > Einstellungen** (<kbd>Ctrl</kbd>+<kbd>P</kbd>) und wählen Sie **Bandbreite**.
2. Haken Sie unter **Optionen für die globale Ratenbegrenzung** **Ping niedrig halten (Geschwindigkeiten automatisch begrenzen)** an.
3. Klicken Sie auf **OK** oder **Übernehmen**.

![Einstellungen, Seite „Bandbreite“](images/prefs-bandwidth.png)

Standardmäßig ist es aus. Um es wieder auszuschalten, nehmen Sie das Häkchen weg: Ihre eigenen Begrenzungen gelten sofort wieder.

Wenn Sie es zum ersten Mal einschalten, lernt Rookparcel den Ruhe-Ping Ihrer Leitung kennen. Das dauert bis zu etwa 20 Sekunden, in denen die Geschwindigkeiten zunächst niedriger sein können.

## Was es tut

- **Es misst die Verzögerung.** Rookparcel pingt den ersten Router hinter Ihrer Heimleitung etwa viermal pro Sekunde an (einige hundert Byte pro Sekunde). Antwortet kein Router hinter der Leitung, pingt es stattdessen die Hosts in der erweiterten Einstellung `auto_limit_probe_hosts` an, standardmäßig `1.1.1.1,8.8.8.8,9.9.9.9`. Ausgelastete uTP-Peers liefern eine zweite Messung ohne zusätzlichen Verkehr.
- **Es deckelt beide Richtungen.** Solange die Verzögerung um mehr als die Toleranz über ihrem Ruhewert bleibt, wird der Upload- oder Download-Deckel gesenkt; ist die Leitung wieder ruhig, wächst der Deckel Schritt für Schritt.
- **Es erhöht nie Ihre eigenen Begrenzungen.** Es gilt der niedrigere Wert aus dem automatischen Deckel und Ihrer eigenen globalen Begrenzung. Ihre Einstellungen werden nicht geändert.
- **Es hat Untergrenzen.** Der Deckel sinkt nicht unter etwa 10 kB/s oder unter einen kleinen Anteil der höchsten gesehenen Rate, damit ein ausgelastetes WLAN oder der Upload eines anderen Programms Rookparcel nicht vollständig aushungern kann.
- **uTP gibt früher nach.** Solange es an ist, zielen uTP-Verbindungen auf höchstens 25 ms Warteschlangenverzögerung statt der üblichen 100 ms.

Der Hinweis unter dem Kästchen sagt es ebenfalls: Diese Pings gehen an den ersten Router hinter Ihrer Leitung oder sonst an die Ausweich-Hosts, deren Betreiber sie sehen.

## Was Sie sehen

- **Die Statusleiste.** Solange eine Richtung unter Ihre eigene Begrenzung gedeckelt ist, zeigt ihr Teil den Deckel in Klammern, zum Beispiel **U: 412.0 kB/s (auto 450.0 kB/s) G: 2.1 GB**. Ohne Deckel sieht sie wie gewohnt aus.
- **Der Reiter „Protokoll“.** Zeilen, die mit „Keep ping low:“ beginnen, sagen, welchen Router oder welche Hosts es anpingt und wann eine Richtung begrenzt oder nicht mehr begrenzt ist.
- **Die Web-UI** zeigt dasselbe „(auto ...)“ in ihrer Statusleiste.

Geschwindigkeiten, die niedriger sind als das Maximum Ihrer Leitung, sind bei eingeschalteter Funktion zu erwarten: Dieser Spielraum hält den Ping niedrig.

## Feinabstimmung

Zwei Einstellungen unter **Optionen > Einstellungen > Erweitert** gehören dazu. Wählen Sie die Zeile aus, geben Sie den Wert ein, klicken Sie auf **Setzen** und dann auf **OK** oder **Übernehmen**.

| Einstellung | Bedeutung | Standard |
|---|---|---|
| `auto_limit_delay_ms` | Millisekunden zusätzlicher Ping über dem Ruhe-Ping der Leitung, die erlaubt sind, bevor die Deckel gesenkt werden. Niedriger hält Spiele und Anrufe flüssiger; höher erlaubt mehr Geschwindigkeit. | 15 |
| `auto_limit_probe_hosts` | Kommagetrennte IP-Adressen, die gepingt werden, wenn kein Router hinter der Leitung antwortet. Wird gelesen, wenn die Funktion eingeschaltet wird. Muss selten geändert werden. | `1.1.1.1,8.8.8.8,9.9.9.9` |

`utp_target_delay` (Standard 100) ist das uTP-Verzögerungsziel, das die Funktion auf 25 senkt, solange sie an ist.

## Plattformen

| System | Wie gemessen wird |
|---|---|
| Windows | Pings, keine Administratorrechte nötig, plus die uTP-Peers. |
| Linux | Pings über die unprivilegierten Ping-Sockets des Systems, plus die uTP-Peers. Erlaubt das System sie nicht, sagt das Protokoll „this system does not let Rookparcel ping (its group must be in net.ipv4.ping_group_range)“, und nur die uTP-Peers werden verwendet. |
| macOS | Pings, plus die uTP-Peers. |
| FreeBSD und andere Systeme ohne unprivilegierte Pings | Nur die uTP-Peers. Das Protokoll sagt, dass Pings nicht verfügbar sind. |

Mit nur uTP-Peers braucht es mindestens drei ausgelastete uTP-Peers, um überhaupt etwas zu messen; wer nur zu TCP-Peers seedet, gibt ihm nichts, woran es sich orientieren kann.

## Wann etwas anderes besser ist

Ein Router mit SQM (etwa CAKE) oder einem funktionierenden Gaming-QoS leistet das besser, für jedes Gerät im Haus, nicht nur für Rookparcel. Haben Sie einen, können Sie „Ping niedrig halten“ ausgeschaltet lassen.

## Verwandte Funktionen

- Die Verlangsamung bei ausgelastetem Datenträger unter **Erweitert > Festplatten-E/A** verwendet dieselbe Art automatischen Deckel, für einen langsamen Datenträger statt einer langsamen Leitung; der niedrigere der beiden Deckel gilt. Siehe [Datenträger und Speicher](Datenträger-und-Speicher#wenn-die-festplatte-ausgelastet-ist).
- Die Router-Seite des Einrichtungsassistenten erwähnt „Ping niedrig halten“ für Spiele und Anrufe: siehe [Erster Start](Erster-Start).
- Feste Begrenzungen und der wöchentliche Zeitplaner: [Geschwindigkeitsbegrenzung und Zeitplaner](Geschwindigkeitsbegrenzung-und-Zeitplan).

## Im GTK-Fenster und in der Web-UI

Das GTK-Fenster (Linux, BSD) und die macOS-App haben dasselbe Kästchen unter **Einstellungen > Bandbreite** (auf dem Mac gilt es, sobald Sie es anhaken), und die Web-UI hat es auf ihrer Seite **Einstellungen > Bandbreite**, mit demselben Hinweis. In [rookparcel-nox](rookparcel-nox-betreiben) ist es dieselbe Einstellung, `auto_rate_limit`, gesetzt über die Einstellungen der Web-UI.
