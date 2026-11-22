*Ez a [Connection, ports and firewall](Connection-ports-and-firewall) című angol oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Kapcsolat, portok és tűzfal

Ez az oldal azt tárgyalja, hogyan érik el a többi peerek a Rookparcelt: a figyelő port, a porttovábbítás UPnP-vel és NAT-PMP-vel, a Windows tűzfal, hogyan állapíthatja meg, működnek-e a bejövő kapcsolatok, hogyan tarthatja a torrentforgalmat VPN-en a „Csak ennek a hálózatnak a használata” beállítással, a proxyk, a titkosítás, az IP-szűrő és webcímről letöltött tiltólistája, valamint a DHT, a peercsere és a Helyi peerkeresés.

E beállítások többsége a **Beállítások > Tulajdonságok > Kapcsolat** és a **Beállítások > Tulajdonságok > BitTorrent** oldalon található.

![A Tulajdonságok Kapcsolat oldala](images/prefs-connection.png)

## Miért fontosak a bejövő kapcsolatok

A Rookparcel kifelé csatlakozik a peerekhez, a peerek pedig befelé a Rookparcelhez. A letöltés pusztán kimenő kapcsolatokkal is működik, de az a Rookparcel, amelyet mások elérnek, több peert talál, és két peer, amelyek egyike sem érhető el, egyáltalán nem tud beszélni egymással. A jó sebességhez és a megosztáshoz engedje be a peereket.

## A figyelő port

A Kapcsolat oldalon a **Figyelő port** alatt:

| Beállítás | Jelentése |
|---|---|
| **Bejövő kapcsolatok portja:** | A port, amelyhez a többi peer csatlakozik. A módosítás azonnal érvénybe lép. |
| **Véletlen port** | Véletlen portot tölt ki 10000 és 60000 között. Az alkalmazása után használatos. |
| **Véletlen port minden indításkor** | Minden indításkor új véletlen portot választ 10000 és 60000 között. A routerben kézzel továbbított port ezután már nem egyezik. Alapértelmezés szerint ki van kapcsolva. |
| **UPnP porttovábbítás bekapcsolása** | Megkéri az UPnP-t tudó routert, hogy továbbítsa a portot erre a számítógépre. Alapértelmezés szerint be van kapcsolva. |
| **NAT-PMP porttovábbítás bekapcsolása** | Megkéri a routert, hogy a NAT-PMP-n vagy PCP-n át továbbítsa a portot, amelyet sok router az UPnP mellett vagy helyett kínál. Alapértelmezés szerint be van kapcsolva. |

Az alapértelmezett profil a 6881-es porton figyel. Az új profil vagy a hordozható másolat az első indításakor véletlen portot választ 10000 és 60000 között, így nem ütközik egy másik Rookparcellel. Lásd a [Profilok](Profilok) oldalt.

A Rookparcel ugyanazon a porton figyel TCP-n és UDP-n: az UDP az uTP-peereket és a DHT-t szállítja.

### A port továbbítása a routerben

Ha az UPnP vagy a NAT-PMP be van kapcsolva, és a router támogatja őket, nincs mit tennie: a Rookparcel megkéri a routert a port továbbítására, és megújítja a kérést. Ha a routerén ki vannak kapcsolva, vagy hibásan működnek, kapcsolja ki mindkét négyzetet, és továbbítsa a portot kézzel a router saját beállításaiban: TCP-re és UDP-re, ennek a számítógépnek a címére. Ezután hagyja kikapcsolva a **Véletlen port minden indításkor** négyzetet, hogy a port az maradjon, amelyet továbbított.

## A Windows tűzfal

A Rookparcel magától nem ad hozzá Windows tűzfalszabályt. A Windows rendszerint rákérdez, amikor egy program először figyel bejövő kapcsolatokra; engedélyezze ott a Rookparcelt.

Ha elvetette a kérdést, vagy semmi sem kérdezett, adjon hozzá szabályt a programhoz egy rendszergazdaként futtatott Parancssorból. Egyfelhasználós telepítéshez:

```bat
netsh advfirewall firewall add rule name="Rookparcel" dir=in action=allow program="%LOCALAPPDATA%\Programs\Rookparcel\rookparcel.exe"
```

A minden felhasználóra szóló telepítésnél az útvonal `C:\Program Files\Rookparcel\rookparcel.exe`; a zip esetén bárhol, ahová a `rookparcel.exe` fájlt kicsomagolta. A webes felület portjához ugyanez a szabály kell, ha más számítógépeknek is el kell érniük: lásd a [Webes felület](Webes-felület) oldalt. A szolgáltatásként futó [rookparcel-noxhoz](A-rookparcel-nox-futtatása) lásd annak oldalát.

Linuxon és a BSD-ken engedélyezze a portot a tűzfalában, ha futtat ilyet.

## Annak ellenőrzése, hogy a peerek csatlakozhatnak

Az állapotsor bal oldali része megmondja:

| Állapotsor | Jelentése |
|---|---|
| **Hálózat rendben** | Legalább egy peer csatlakozott befelé. A port működik. |
| **Nincs bejövő kapcsolat** | Még egy peer sem csatlakozott befelé. Az indítás utáni első percekben ez normális; ha így marad, a port nem érhető el. |
| **A(z) 6881 port foglalt** (az Ön portszámával) | Egy másik program, gyakran egy másik Rookparcel-profil foglalja a portot, és a Rookparcel egy későbbi porton figyel, ahová semmi sincs továbbítva. Kattintson rá, hogy a Tulajdonságok a Kapcsolat oldalon nyíljon meg, és válasszon másik portot. |

E szavak előtt két értesítés állhat. Az **Átvitelek visszatartva: erősítse meg a beállításokat** azt jelenti, hogy a Rookparcel minden átvitelt visszatart, amíg egy verzióváltás után meg nem erősíti a beállításokat: lásd [A Rookparcel frissítése](A-Rookparcel-frissítése). A **Várakozás a hálózatra** azt jelenti, hogy a „Csak ennek a hálózatnak a használata” nem talál megfelelő adaptert: lásd az alábbi szakaszt.

*A „Port ... in use” (A(z) ... port foglalt) új az 1.1.45-ben.*

Tartsa az egérmutatót e rész fölé, hogy lássa az **A címed a trackerek szerint:** sort, azt a nyilvános címet, amelyet a trackerek visszajeleznek. Az **Általános** lap ugyanezt a sort mutatja. *Új az 1.1.58-ban.*

A Rookparcelnek nincs beépített porttesztje. Ha az állapot aktív torrentek mellett is „Nincs bejövő kapcsolat” marad:

1. Ellenőrizze, hogy a tűzfal engedélyezi-e a Rookparcelt (lásd fent).
2. Ellenőrizze, hogy az UPnP vagy a NAT-PMP be van-e kapcsolva a Rookparcelben és a routerben, vagy továbbítsa a portot kézzel.
3. Ha a „Csak ennek a hálózatnak a használata” VPN-re van állítva, a peerek csak a VPN saját porttovábbításán át érhetik el, ha van ilyen: a Rookparcel nem kér porttérképezést a VPN-adapterre.

Lásd még a [Hibaelhárítás és GYIK](Hibaelhárítás-és-GYIK) oldalt.

## Csak ennek a hálózatnak a használata (VPN-ek)

*Új az 1.1.58-ban.*

A „Csak ennek a hálózatnak a használata” az egész torrentforgalmat egyetlen hálózati adapteren tartja, jellemzően egy VPN-en. A peerek, a DHT, a trackerek és a webes seedek mind azon mennek át, és semmi más. Amikor a VPN megszakad, a torrentek várnak, ahelyett hogy a szokásos kapcsolaton folytatnák.

1. Csatlakoztassa a VPN-t.
2. Nyissa meg a **Beállítások > Tulajdonságok > Kapcsolat** oldalt.
3. A **Csak ennek a hálózatnak a használata** alatt válasszon a **Hálózat:** mezőben:
   - **Bármely VPN-adapter**: bármelyik adapter, amely VPN-alagútnak látszik (WireGuard, OpenVPN, egy VPN-alkalmazás saját adaptere). A legtöbben ezt szeretnék.
   - Egy adapter a nevével, a számítógép adaptereinek listájából.
   - **Címtartomány (lent)**, majd írja be a tartományt a **Címtartomány:** mezőbe, például `10.2.0.0/16` vagy `fd00::/8`.
   - **Bármely hálózat (ki)** kikapcsolja; ez az alapértelmezés.
4. Hagyja bejelölve a **Torrentek szüneteltetése, amíg ez a hálózat hiányzik** négyzetet.
5. Kattintson az **OK** vagy az **Alkalmaz** gombra.

Melyiket válassza:

- A **Bármely VPN-adapter** túléli, ha a VPN-alkalmazás átnevezi vagy újra létrehozza az adapterét, amit sok alkalmazás minden csatlakozáskor vagy frissítéskor megtesz.
- A **Címtartomány** olyan VPN-hez illik, amely mindig ugyanabból a tartományból oszt címeket: a tartományban címet tartó adapter viszi a forgalmat, bárhogy hívják.
- Az **egy adapter név szerint** választás a legszűkebb. Az az adapter, amely most nincs jelen, „(nincs csatlakoztatva)” felirattal látszik a listában.

A szabályt minden hálózatváltozásnál újra ellenőrzi a program, így követi azt a VPN-t is, amely új címen csatlakozik újra.

**Amíg a hálózat hiányzik:** ha a **Torrentek szüneteltetése, amíg ez a hálózat hiányzik** be van kapcsolva, a futó torrentek várnak. Az állapotuk **Várakozás a hálózatra**, az állapotsor **Várakozás a hálózatra** felirattal kezdődik, és a Napló megmondja, melyik szabály nem talált semmit. Amikor a VPN visszatér, maguktól folytatják. A Leállítás és az Indítás közben is működik rajtuk. Kikapcsolt négyzetnél semmi sem szünetel, de torrentforgalom sem hagyhatja el a gépet, mert minden kapcsolathoz a megfelelő adapter kell.

**Annak ellenőrzése, hogy működik:** tartsa az egérmutatót az állapotsor hálózati része fölé, vagy nézze meg az **Általános** lapot: az **A címed a trackerek szerint:** sornak a VPN címét kell mutatnia, nem a sajátját. „még nem ismert” feliratot mutat, amíg egy tracker nem válaszolt.

Jó tudni:

- Ha a VPN csak IPv4-et visz, a Rookparcel nem használ IPv6-ot a torrentekhez, amíg hozzá van kötve, így az IPv6 nem szivároghat az alagút mellett.
- A webes felület és a Rookparcelhez érkező távoli kapcsolatok nincsenek hozzákötve, így továbbra is elérheti őket a szokásos hálózaton.
- A proxykapcsolatok sincsenek hozzákötve. A keresősegéd különálló program, és nincs hozzákötve; lásd [Torrentek keresése Windowson](Torrentek-keresése-Windowson#jó-tudni).
- A webes felületen ugyanez a csoport a **Tulajdonságok > Kapcsolat** oldalon van. A rookparcel-noxhoz parancssorban: `--network vpn`, `--network adapter:ID`, `--network range:10.2.0.0/16` vagy `off`; és `--no-pause-when-missing`, hogy a torrentek hozzákötés nélkül fussanak.

## Proxykiszolgáló

A Kapcsolat oldalon a **Proxykiszolgáló** alatt. A proxymezők szürkék, amíg a **Típus:** **(nincs)**, ami az alapértelmezés.

| Beállítás | Jelentése |
|---|---|
| **Típus:** | **Socks4**, **Socks5** vagy **HTTP** (amely a sima http-kérések kivételével mindent CONNECT-en át alagutaz). A **(nincs)** közvetlenül csatlakozik. |
| **Proxy:** és **Port:** | A proxykiszolgáló gépneve vagy IP-címe, és a portja, például 1080 sok SOCKS-kiszolgálónál. |
| **Hitelesítés** a **Felhasználónév:** és a **Jelszó** mezővel | Bejelentkezik a proxyra. A SOCKS4 proxy csak a felhasználónevet kapja. |
| **Gépnevek feloldása a proxyn keresztül** | A trackerek és a webes seedek neveit a proxy oldatja fel, így a saját DNS-kiszolgálója nem látja őket. |
| **Proxy használata a peerkapcsolatokhoz** | A peerekhez és a webes seedekhez menő kapcsolatokat a proxyn keresztül küldi. A bejövő kapcsolatokat ekkor elutasítja, a porttérképezés és a Helyi peerkeresés pedig kikapcsolva marad. Az uTP-hez és a DHT-hoz SOCKS5 proxy kell. |
| **Proxy használata a trackerkapcsolatokhoz** | A tracker-bejelentéseket a proxyn keresztül küldi, így a trackerek a proxy címét látják. |

Az utolsó három alapértelmezés szerint be van jelölve, és csak akkor számít, ha proxytípust választott. A VPN-szolgáltatótól kapott SOCKS5 proxy az egyik mód a VPN használatára adapter nélkül; a „Csak ennek a hálózatnak a használata” a másik.

## Protokolltitkosítás

A **Beállítások > Tulajdonságok > BitTorrent** oldalon, a **Protokolltitkosítás** alatt:

| Beállítás | Jelentése |
|---|---|
| **Kimenő:** | Hogyan titkosítja a Rookparcel által nyitott kapcsolatokat. **Kikapcsolva**: soha. **Bekapcsolva** (az alapértelmezés): a sima és a titkosított próbálkozás peerenként váltakozik, megtartva, ami bevált. **Kényszerítve**: csak titkosított, elveszítve azokat a peereket, amelyek nem tudnak titkosítani. |
| **Titkosítatlan bejövő kapcsolatok engedélyezése** | Elfogadja a nem titkosított bejövő kapcsolatokat. Kikapcsolva csak a titkosítottakat fogadja el, ami elveszíti azokat a peereket, amelyek nem tudnak titkosítani. Alapértelmezés szerint be van kapcsolva. |

Módosítsa ezeket, ha a szolgáltatója lassítja a BitTorrent-forgalmat. A Haladó oldalon finomabb beállítások vannak (`out_enc_policy`, `in_enc_policy`, `allowed_enc_level`, `prefer_rc4`).

![A Tulajdonságok BitTorrent oldala](images/prefs-bittorrent.png)

## DHT, peercsere, Helyi peerkeresés és uTP

A BitTorrent oldalon az **Alapvető BitTorrent-funkciók** alatt, alapértelmezés szerint mind be van kapcsolva:

| Beállítás | Jelentése |
|---|---|
| **DHT-hálózat bekapcsolása** | A DHT-n, a BitTorrent-kliensek hálózatán keresztül talál peereket, tracker nélkül. A legtöbb magnet hivatkozáshoz szükséges. |
| **Helyi peerkeresés bekapcsolása** | Ugyanezekhez a torrentekhez keres peereket a helyi hálózaton, így azok közvetlenül, hálózati sebességgel cserélnek adatot. |
| **Peercsere bekapcsolása** | Lehetővé teszi, hogy a csatlakozott peerek további peerekről értesítsék egymást. A módosítás minden torrentre érvényes. |
| **uTP bekapcsolása (UDP-átvitel sávszélesség-kezeléssel)** | A TCP mellett uTP-n, egy UDP-átvitelen át is csatlakozik peerekhez és fogad peereket; az uTP lassít, amikor a hálózat foglalt. |

A privát torrentek soha nem használják a DHT-t, a peercserét vagy a Helyi peerkeresést, bármit mondanak is ezek a négyzetek. Egy torrent Jellemzői egyedül arra a torrentre kapcsolhatják ki a DHT-t és a peercserét (**DHT bekapcsolása** és **Peercsere** az **Egyéb beállítások** alatt).

Az állapotsor a **DHT: N csomópont** feliratot mutatja, vagyis hogy a Rookparcel hány DHT-csomópontot ismer.

Hogy csak bizonyos kapcsolattípusokat engedélyezzen, például csak TCP-t, használja a Haladó oldal `bt.transp_disposition` beállítását: 1 kimenő TCP, 2 kimenő uTP, 4 bejövő TCP, 8 bejövő uTP, összeadva; a 15 (az alapértelmezés) mindet engedi, az 5 csak TCP.

## IP-szűrő

A BitTorrent oldalon az **IP-szűrő** alatt a Rookparcel cím szerint blokkolhat peereket, eMule `ipfilter.dat`, PeerGuardian P2P vagy CIDR formátumú tiltólistából. A lista lehet egy fájl a számítógépén, vagy egy fájl a weben, amelyet a Rookparcel letölt és naprakészen tart.

A lista minden sora egy szabály:

| Formátum | Példasor |
|---|---|
| eMule `ipfilter.dat` | `001.002.004.000 - 001.002.004.255 , 000 , Some name` |
| PeerGuardian P2P | `Some name:1.2.4.0-1.2.4.255` |
| CIDR, IPv4 vagy IPv6 | `10.0.0.0/8` vagy `fd00::/8` |

*A CIDR-sorok újak az 1.2.63-ban.* A számítógépén lévő fájlban is működnek.

### Listafájl a számítógépén

1. Jelölje be az **IP-szűrő bekapcsolása** négyzetet.
2. Válassza ki a tiltólista fájlját az alatta lévő mezőben. Üresen hagyva a Rookparcel a beállításmappájában lévő `ipfilter.dat` fájlt használja.
3. Kattintson az **Újratöltés** gombra, hogy most betöltse, és lássa, hány szabályt tartalmaz („N szabály betöltve.”), vagy kattintson az **OK** gombra, hogy a többi beállítással együtt töltse be.

A listát indításkor és az Alkalmazáskor tölti be. A Napló ezt írja: „IP filter: N rules from ...” vagy a hibát. Ha az új lista nem olvasható, a korábban betöltött szabályok érvényben maradnak.

A webes felületen nincs Újratöltés gomb: a lista az alkalmazáskor töltődik be.

### Lista webcímről

*Új az 1.2.63-ban.* A Rookparcel letöltheti a tiltólistát a webről, és naprakészen tarthatja:

1. Jelölje be az **IP-szűrő bekapcsolása** négyzetet.
2. Jelölje be a **Tiltólista letöltése erről a címről (https):** négyzetet, és írja be a lista címét az alatta lévő mezőbe. `https://` előtaggal kell kezdődnie.
3. Kattintson a **Frissítés most** gombra, hogy azonnal letöltse, vagy az **OK** gombra. A gomb alatti sor mutatja az eredményt.

Amíg a cím használatban van, az felváltja a fájlt: a fájlmező és az **Újratöltés** szürke. Tudnivalók:

- **Csak `https://`.** Az a cím, amely a Rookparcelt sima `http://` címre irányítja tovább, el van utasítva. A letöltés a proxyn megy keresztül, ha a Tulajdonságok > Kapcsolat oldalon van beállítva ilyen.
- **Formátumok.** A fenti három formátum, sima vagy gzip-tömörített. A zip-archívumok nem támogatottak. A letöltés legfeljebb 32 MiB lehet.
- **Milyen gyakran.** Naponta egyszer, és egy óra múlva egy sikertelen letöltés után. A **Frissítés most** bármikor letölti.
- **Indításkor.** A Rookparcel az utolsó jó letöltést a beállításmappájában őrzi (`ipfilter-url.dat`), és a következő indításkor azonnal érvénybe helyezi, még mielőtt bármit letöltene.
- **Új cím.** A régi cím listájának másolata a beállításmappában marad, amíg az új címről le nem töltöttek és nem ellenőriztek egy listát, így egy elírás vagy egy sikertelen letöltés semmit sem veszít, és a régi címre visszatérve annak másolatát használja újra. A másolatot csak arra a címre használja, ahonnan származik: új címmel indítva a Rookparcelnek nincs érvényben lévő tiltólistája, amíg az új meg nem érkezik, és a Napló ezt közli. A Windows-programban a **Frissítés most** egy még nem alkalmazott címmel csak kipróbálja azt: a sor mutatja az eredményt, és semmi sem változik az **OK** vagy az **Alkalmaz** megnyomásáig.
- **Ha a letöltés sikertelen**, vagy olyat hoz, ami nem tiltólista (például egy hibaoldalt), az érvényben lévő szabályok megmaradnak, és az állapotsor ezt mondja: „The last download failed: ...”. A Napló is kiírja a hibát.
- **Állapotsor.** „N szabály, letöltve” a dátummal és az idővel, vagy „A lista még nem töltődött le.”, amíg az első letöltés nem sikerül. Addig az érvényben lévő szabályok megmaradnak.
- **A beállítások ellenőrzése.** Amíg a Rookparcel egy verzióváltás után visszatartja az átvitelt, amíg meg nem erősíti a beállításokat ([A Rookparcel frissítése](A-Rookparcel-frissítése)), a letöltés is vár.

Olyan címet használjon, amelyben megbízik: aki a listát ellenőrzi, az dönti el, mely peerekkel nem hajlandó a Rookparcel beszélni.

A webes felületen a mező vagy a cím módosításához a jelenlegi jelszó kell (**Jelenlegi jelszó:** a webes felület oldalán), mert a kiszolgáló bármit letölt, ami a címnél be van állítva. Az oldal a címet felhasználónév, jelszó és lekérdezés nélkül mutatja (egy előfizetés hitelesítő adatai gyakran ott vannak); ha úgy hagyja, ahogy van, a tárolt cím megmarad. A hitelesítő adatok eltávolításához ürítse ki a mezőt, alkalmazza, majd írja be újra a címet. A Windows-program által kiszolgált webes felület kihagyja a címet: az az ablak saját Tulajdonságaihoz tartozik. A qBittorrent-kompatibilis API nem tudja módosítani.

### Trackerek és a DHT

Alapértelmezés szerint a szűrő a DHT-ra és a tiltott címeken lévő trackerekre is érvényes (`apply_filter_to_dht` és `apply_ip_filter_to_trackers` a Haladó oldalon). Kapcsolja ki a trackeresét, ha egy Önnek szükséges tracker tiltott tartományban van.

## A GTK-ablakban és a webes felületen

- **GTK-ablak**: ugyanazok az oldalak és beállítások. Az állapotsor hálózati része ugyanazt mondja, és az „A(z) ... port foglalt” szövegre kattintva a Tulajdonságok a Kapcsolat oldalon nyílik meg.
- **macOS-alkalmazás**: ugyanazok az oldalak, azonnal érvénybe lépve **OK** nélkül ([A beállítások leírása](A-beállítások-leírása#különbségek-macos-en)).
- **Webes felület**: ugyanazok az oldalak a **Tulajdonságok** alatt; az állapotsora a hálózati részt ugyanazzal a súgószöveggel mutatja.
- **rookparcel-nox**: a `--port N` a figyelő portot állítja be (alapértelmezés 6881), a `--listen` a figyelő felületeket adja meg teljesen, a `--proxy TYPE://[USER:PASS@]HOST:PORT` proxyt állít be, az `--ip-filter FILE` tiltólista-fájlt tölt be (a címet a webes felület Tulajdonságaiban kell megadni), a `--network` pedig a „Csak ennek a hálózatnak a használata” beállítást adja meg. A Windowson futó szolgáltatáshoz tűzfalszabály kell a `rookparcel-nox.exe` számára: lásd [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

## Kapcsolódó oldalak

- [Sebességkorlátok és az ütemező](Sebességkorlátok-és-ütemező)
- [Profilok](Profilok) (portok több Rookparcelhez)
- [Hibaelhárítás és GYIK](Hibaelhárítás-és-GYIK)
- [A beállítások leírása](A-beállítások-leírása)
