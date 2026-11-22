*Ez a [Troubleshooting and FAQ](Troubleshooting-and-FAQ) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Hibaelhárítás és GYIK

Ez az oldal akkor segít, ha valami nem működik: lassú vagy elakadt letöltések, elérhetetlen port, a Windows vagy a macOS figyelmeztetései, privát trackerek. Azt is megmondja, hol vannak a Rookparcel üzenetei, hogyan állíthatja vissza a beállításait, és hogyan jelezhet hibát.

## A letöltések lassúak, vagy nem indulnak el

Nézze meg a torrent **Állapot** oszlopát és az ablak alján lévő állapotsort. Ezek általában megmondják az okot.

| Amit lát | Mit jelent, és mit tegyen |
|---|---|
| **Sorban áll** | A sor megtelt: elérte az aktív torrentek vagy letöltések **Tulajdonságok > Sorba állítás** oldalon beállított számát. A torrent akkor indul, amikor egy másik befejeződik vagy leáll. A jobb gombos menü **Kényszerített indítás** parancsa a soron kívül futtatja. Lásd [Várólista és seedelés](Várólista-és-seedelés). |
| **Leállítva** vagy **Szüneteltetve** | Indítsa el az **Indítás** paranccsal (<kbd>Ctrl</kbd>+<kbd>S</kbd>). Ha a Tulajdonságok > Felület oldalon be van kapcsolva a **Ne indítsa el automatikusan a letöltést**, az új torrentek leállítva kerülnek hozzáadásra. |
| **Ellenőrzésre vár**, **Ellenőrzés** | A Rookparcel a lemezen már meglévő fájlokat ellenőrzi. Az ugyanazon a meghajtón lévő torrenteket egymás után ellenőrzi: a sorukra várók állapota **Ellenőrzésre vár**. Ha be van kapcsolva a **Kíméletes torrentellenőrzés (alacsony prioritás, elsőbbség a többi programnak)**, az ellenőrzés elsőbbséget ad más programoknak, és tovább tart, amíg a lemez foglalt. Lásd [Lemez és tárolás](Lemez-és-tárolás). *A várakozó ellenőrzések az 1.2.76 óta ezt írják; korábban **Ellenőrzés** állt náluk, 0 %-kal.* |
| **Várakozás a hálózatra** | A Tulajdonságok > Kapcsolat oldalon be van kapcsolva a **Csak ennek a hálózatnak a használata**, és a hálózata (általában a VPN-je) nincs jelen. A torrent magától folytatódik, amikor visszatér. Lásd [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal). |
| **Hiba: ...** | A „Hiba:” utáni üzenet megmondja, mi romlott el. A „files missing (the drive, share or folder is not available)” esetén lásd a következő szakaszt. Ha egy ellenőrzés egy fájlt megnevező hibával áll le, egy másik program tartja nyitva a fájlt, vagy nem olvashatja: zárja be a programot, vagy javítsa az engedélyt, majd **Indítás** a torrenten. *Az 1.2.76 óta az ilyen ellenőrzés hibával áll le; korábban a fájl részeit hiányzónak számolta, és újra letöltötte őket.* |
| „Ütemező: korlátozva”, „Ütemező: kikapcsolva” vagy „Ütemező: csak megosztás” az állapotsorban | Az ütemező ebben az órában visszatartja a sebességet. Lásd [Sebességkorlátok és ütemező](Sebességkorlátok-és-ütemező). |
| „(auto ...)” a sebességek mellett az állapotsorban | Az **Alacsony ping megtartása** korlátozza a sebességet, mert megnőtt a késleltetés a vonalán. Lásd [Alacsony ping](Alacsony-ping). |

Egyéb ellenőrizendők:

- **Seedek és peerek.** A seed nélküli torrent nem fejeződhet be. A **Trackerek** lap megmutatja, válaszolnak-e a trackerek; a **Peerek** lap azt, kikhez csatlakozott. Lásd [Torrent részletek és fájlok](Torrent-részletek-és-fájlok).
- **Magnet hivatkozások.** Ezeknek előbb a többi peertől kell megkapniuk a torrent fájllistáját, és a legtöbbhöz a DHT is kell: tartsa bekapcsolva a **DHT-hálózat bekapcsolása** beállítást a Tulajdonságok > BitTorrent oldalon.
- **Sebességkorlátok.** Nézze meg a **Tulajdonságok > Sávszélesség** oldalt és a torrent saját korlátait (a jobb gombos menü **Sávszélesség** pontja, vagy a **Jellemzők**). A 0 korlátlant jelent.
- **Foglalt lemez.** Ha be van kapcsolva a **Letöltések lassítása (lemezírások)** (Tulajdonságok > Haladó > Lemez I/O), a Rookparcel csökkenti a letöltési sebességet, amíg a lemez lassan válaszol.
- **Proxy a peerkapcsolatokhoz.** Ha be van kapcsolva a **Proxy használata a peerkapcsolatokhoz**, senki sem tud csatlakozni Önhöz, így kevesebb a peer.
- **A bejövő kapcsolatok hiánya** (lent) szintén sebességbe kerül: csak olyan peerekhez érhet el, amelyek maguk is fogadnak kapcsolatot.

## „Files missing (the drive, share or folder is not available)”

Ha egy torrent mentési mappája nincs ott (egy kihúzott USB- vagy hálózati meghajtó, egy nem csatlakoztatott megosztás vagy csatolás), a Rookparcel ezzel a hibával leállítja a torrentet ahelyett, hogy újra letöltené. A már meglévő részek rögzítve maradnak, és újraindítások után is így maradnak.

1. Csatlakoztassa újra a meghajtót vagy a megosztást. A Rookparcel 30 másodpercenként megkeresi a mappát; amikor visszatért, a torrent magától elindul, és úgy találja a fájljait, ahogy voltak.
2. Ha azonnal próbálkozni szeretne, válassza az **Indítás** parancsot. Ha a mappa még mindig hiányzik, a torrent újra leáll a hibával.
3. Ha a fájlokat véglegesen áthelyezte vagy törölte, a **Kényszerített ellenőrzés** azt veszi alapul, ami most a mappában van: a hiányzó részeket újra letölti.

A letöltés közben kihúzott meghajtót ugyanígy tartja vissza. Ha egy torrentnek csak néhány fájlja tűnt el, azokat inkább újra letölti. Lásd [Torrentek kezelése](Torrentek-kezelése). *Az 1.2.76-ban jelent meg; korábban a torrent mindent újra letöltött.*

A leállított torrenten végzett **Kényszerített ellenőrzés** most elindítja a torrentet, ahogy az **Indítás** is, és az ellenőrzés azonnal fut akkor is, ha a torrent sorban állt; ha leállítva szeretné tartani, állítsa le újra. *Az 1.2.76-ban jelent meg; korábban az ellenőrzés egy Indításra várt.*

## „Átvitelek visszatartva: erősítse meg a beállításokat”

Frissítés után, vagy ha a beállításfájl nem olvasható, a Rookparcel minden átvitelt visszatart, amíg meg nem erősíti, hogy a proxy-, hálózati és figyelőbeállításai olyanok, amilyeneket szeretne. Nyissa meg a **Beállítások > Tulajdonságok > Kapcsolat** oldalt, ellenőrizze őket, és kattintson az **OK** gombra. A Napló lap felsorolja, mi változott. Lásd [A Rookparcel frissítése](A-Rookparcel-frissítése#ha-egy-verzió-másként-olvassa-a-beállításait).

## „Nincs bejövő kapcsolat” vagy „A(z) ... port foglalt”

Az állapotsor bal oldala a hálózat állapotát mutatja:

| Szöveg | Jelentés |
|---|---|
| **Hálózat rendben** | Más peerek csatlakoztak Önhöz, tehát a figyelő portja elérhető. |
| **Nincs bejövő kapcsolat** | Még egy peer sem csatlakozott Önhöz. Közvetlenül az indítás után ez normális; ha így marad, a portot valószínűleg a router vagy a tűzfal blokkolja. |
| **A(z) 6881 port foglalt** (az Ön portszámával) | Egy másik program, gyakran egy másik Rookparcel-profil, foglalja a portot, ezért a Rookparcel egy másikon figyel, ahová semmi sincs továbbítva. Kattintson a szövegre, hogy megnyissa a **Tulajdonságok > Kapcsolat** oldalt, és válasszon másik portot. *Az 1.1.45-ben jelent meg.* |

A port elérhetővé tételéhez:

1. Nyissa meg a **Beállítások > Tulajdonságok > Kapcsolat** oldalt, és jegyezze meg a **Bejövő kapcsolatok portja** értékét.
2. Jelölje be az **UPnP porttovábbítás bekapcsolása** és a **NAT-PMP porttovábbítás bekapcsolása** négyzetet, hogy a router kapjon kérést a port továbbítására. Vagy továbbítsa a portot kézzel a routerben (TCP és UDP) erre a számítógépre, és hagyja kikapcsolva a **Véletlen port minden indításkor** beállítást.
3. Engedélyezze a `rookparcel.exe` programot a tűzfalon (Linuxon a `rookparcel-gtk` programot), vagy nyissa meg benne a portot.
4. VPN mögött a bejövő kapcsolatok csak akkor működnek, ha a VPN továbbít Önnek egy portot.

Az **Általános** lap megmutatja az „A címed a trackerek szerint:” sort, amiből látszik, hogy a forgalma VPN-en megy-e át. *Az 1.1.58-ban jelent meg.* További információ: [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal).

## A Windows SmartScreen vagy a macOS Gatekeeper figyelmeztet a Rookparcelre

A programok nincsenek kódaláírással ellátva, ezért a rendszerek az első alkalommal figyelmeztetnek:

- **Windows:** „A Windows megvédte a számítógépet”. Kattintson a **További információ**, majd a **Futtatás mindenképp** gombra.
- **macOS:** az alkalmazást elutasítja, amíg egyszer meg nem engedi. A macOS 15 és újabb verzióiban próbálja megnyitni, kattintson a **Kész** gombra, majd a **Rendszerbeállítások > Adatvédelem és biztonság** helyen a **Megnyitás mindenképp** gombra. A macOS 11–14 verzióiban Control-kattintson az alkalmazásra, és válassza a **Megnyitás** parancsot. Lásd [A Rookparcel telepítése](A-Rookparcel-telepítése#macos).

Annak ellenőrzéséhez, hogy egy fájl az, amit közzétettek, vesse össze a checksumját a kiadás `SHA256SUMS` fájljával. Az egyes platformok részletei [A Rookparcel telepítése](A-Rookparcel-telepítése) oldalon vannak.

## Használhatom a Rookparcelt privát trackereken?

Még nem. A privát trackerek csak az általuk engedélyezett klienseket fogadják el, a Rookparcel kliensazonosítója pedig (a peer ID-je `-PA`-val kezdődik, és a trackereknek `Rookparcel/<version>` adja meg magát) sehol sincs még engedélyezve. A Rookparcel mindig őszintén Rookparcelként mutatkozik be, ezért egy privát trackeren való használata bajba sodorhatja a fiókját. Előbb nézze meg a trackere engedélyezett kliensek listáját.

A privátként megjelölt torrentek soha nem használják a DHT-t, a helyi peerkeresést és a peercserét, akármit mondanak a Tulajdonságok.

## A .torrent fájlra vagy magnet hivatkozásra duplán kattintva nem a Rookparcel nyílik meg

Jelölje be a **Társítás a .torrent fájlokkal** és a **Társítás a magnet: hivatkozásokkal** négyzetet a **Tulajdonságok > Általános** oldalon. Ha a Windows még mindig másik programmal nyitja meg őket, akkor magában a Windowsban választottak programot, és ezt csak Ön változtathatja meg: kattintson az **Alapértelmezett alkalmazások...** gombra, és ott válassza a Rookparcelt. Lásd [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás#torrent-fájlok-és-magnet-hivatkozások-megnyitása-a-rookparcellel).

## A webes felület nem érhető el másik eszközről

- Jelölje be a **Kapcsolatok engedélyezése más gépekről** négyzetet a **Tulajdonságok > Webes felület** oldalon; különben csak ezen a számítógépen figyel.
- Engedélyezze a `rookparcel.exe` programot a Windows tűzfalán a webes felület portjához.
- „port ... is in use, perhaps by another Rookparcel profile”: válasszon másik **Figyelő portot**.
- Az az alkalmazás, amely gépnéven éri el a Rookparcelt (például Dockerben), „unexpected Host header” hibával elbukik. Adja hozzá a nevet a **Kiszolgálótartományok** mezőben, vagy használja az IP-címet.

Lásd [Webes felület](Webes-felület) és [Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr).

## Egy távoli ablak azt írja: „the server refused the connection (HTTP 426)”

A **Fájl > Csatlakozás távoli géphez...** paranccsal csatlakoztatott ablaknak és a `rookparcel-nox` programnak, amelyhez csatlakozik, ugyanannak a verziónak kell lennie: az üzenet így végződik: „this server speaks remote protocol ...”. Egyes kiadások megváltoztatják a távoli protokollt (utoljára az 1.2.70-ben), ezért ha az egyik oldalt 1.2.76-ra frissíti, frissítse a másikat is: az asztali ablakot, valamint a `rookparcel-nox` programot vagy a Docker-képfájlját. A webes felületet használó böngészőre nem vonatkozik ez a korlát. Lásd [Távvezérlés](Távvezérlés) és [A Rookparcel frissítése](A-Rookparcel-frissítése).

## A Rookparcel bezárása sokáig tart

A Rookparcel elmenti a torrentlistát és a haladást, szól a trackereknek, hogy leállt, és megvárja a még költöztetett fájlokat. Ha ez egy pillanatnál tovább tart, **A Rookparcel bezárul** ablak megmutatja, mire vár. A **Kilépés most** abbahagyja a várakozást; előbb rákérdez, és felsorolja, mit ad fel. A torrentlista és a haladás mindenképpen előbb mentésre kerül.

Linuxon a GTK-s ablak ugyanígy záródik be, ha a terminálban <kbd>Ctrl</kbd>+<kbd>C</kbd>-t vagy `SIGTERM` jelzést kap, például a `kill` parancstól. A jelzés ismételt elküldése nem rövidíti le a bezárást. Az azonnali leálláshoz használja a **Kilépés most** gombot, vagy a `kill -KILL` parancsot a folyamatazonosítóval. *Az 1.2.54 óta; korábban egy második jelzés mentés nélkül azonnal kilépett.*

## A hordozható példány nem tud írni a mappájába

Az „Ez a hordozható Rookparcel nem tud írni a mappájába” azt jelenti, hogy a `rookparcel.exe` és a `rookparcel.ini` mappája csak olvasható Önnek, például a `C:\Program Files` alatt. Helyezze át a mappát írható helyre, vagy törölje a program melletti `rookparcel.ini` fájlt, hogy a beállítások a felhasználói mappájába kerüljenek. Lásd [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait).

## A keresősegéd nem működik

Lásd a [Torrentek keresése Windowson](Torrentek-keresése-Windowson) oldal végén lévő táblázatot és a [Torrentek keresése](Torrentek-keresése) oldalt.

## Linuxos tudnivalók

- **Nincs tálcaikon GNOME-on.** A sima GNOME-nak nincs tálcája; telepítse az AppIndicator bővítményt, vagy használja a Rookparcelt anélkül (az ablak bezárása ilyenkor kilépteti). Lásd [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás#gtk-s-változat-linux-bsd).
- **A Kis méretnél a tálcára nem csinál semmit** néhány Wayland-es asztalon, amelyek nem jelzik, hogy egy ablakot kis méretre állítottak.
- **Az Xfce-ből kijelentkezés** nem várja meg, hogy a Rookparcel mentsen; előbb lépjen ki belőle.

## macOS-es tudnivalók

- **Az ablak bezárása nem lép ki.** Ha a **Bezáráskor a tálcára** be van kapcsolva (alapértelmezés), az ablak elrejtőzik, és a Rookparcel tovább fut a menüsorban. A kilépéshez használja a <kbd>⌘</kbd>+<kbd>Q</kbd> billentyűket. Lásd [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás#macos).
- **Nincsenek értesítések.** Engedélyezze a Rookparcelt itt: **Rendszerbeállítások > Értesítések**.
- **Az Automatikus leállítás nem altatja el és nem állítja le a Mac-et.** Engedélyezze a Rookparcelnek a „loginwindow” vezérlését itt: **Rendszerbeállítások > Adatvédelem és biztonság > Automatizálás**. Lásd [Automatikus leállítás](Automatikus-leállítás).
- **Eltűnt beállítások a GTK-s változat cseréje után.** Az új alkalmazás csak addig veszi át a régi `gtk` mappát, amíg nincs `mac` mappa. Ha előbb készült `mac` mappa, lépjen ki a Rookparcelből, és lásd [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait#macos).

## Hol vannak a naplók

**A Windowsos és a GTK-s változat** nem ír naplófájlt. Az üzeneteik (hibák, az IP-szűrő, a frissítésellenőrzés, az ütemező, a befejezett letöltések) a főablak alján lévő **Napló** lapra kerülnek, amely a Rookparcel futása alatt az utolsó 1000 sort őrzi. Közvetlenül a probléma után nézze meg, még kilépés előtt, és jegyezze fel a rá vonatkozó sorokat.

A **rookparcel-nox** az üzeneteit a terminálra írja. Részletesebb kimenetért indítsa a `--verbose` kapcsolóval, amely hozzáadja a tracker- és peeresemények (minden megszakadt kapcsolat az okával) sorait. A `--log FILE` mindent egy fájlhoz fűz. Ha szolgáltatásként fut:

| Hol | Napló |
|---|---|
| Windows-szolgáltatás | a `nox.log` az állapotmappában, ami a `%ProgramData%\Rookparcel\nox`, hacsak a `--state-dir` mást nem mond (emelt jogú parancssorból olvassa el) |
| systemd | a napló: `journalctl -u rookparcel-nox` |
| FreeBSD rc.d | `/var/log/rookparcel-nox.log` |

Lásd [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

**A keresősegéd** (Jackett) saját beállításait és naplóját a Rookparcel beállítási mappájában lévő `search-helper` mappa `data` mappájában tartja.

## Beállítások visszaállítása

- **Egy beállítás:** a **Tulajdonságok > Haladó** oldalon jelölje ki, kattintson a **Visszaállítás** gombra, majd az **OK** gombra. A `*` jelölte sorok eltérnek az alapértelmezéstől.
- **Elfelejtett zárolási PIN-kód:** tartsa lenyomva a <kbd>Ctrl</kbd> (Windows) vagy az <kbd>⌥</kbd> (macOS) billentyűt a Rookparcel indítása közben, vagy indítsa a `--reset-lock` kapcsolóval, és erősítse meg. Lásd [A Rookparcel zárolása](A-Rookparcel-zárolása#elfelejtett-pin-kód).
- **Az első indítás kérdései:** a **Beállítások > Beállítási útmutató** (<kbd>Ctrl</kbd>+<kbd>G</kbd>) újra felteszi őket. *Az 1.1.62-ben jelent meg.*
- **Minden:** lépjen ki a Rookparcelből (**Fájl > Kilépés**), majd nevezze át a mappát, ahol a beállításait és a torrentjeit tartja (Linuxon és BSD-n a beállítási mappát és az adatmappát); a Rookparcel ezután újként indul, üres torrentlistával. Windowson a **Súgó > A Rookparcel névjegye** megmutatja a mappát. Ha visszanevezi, minden visszaáll. Lásd [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait).

## Hiba jelzése

Nyisson egy hibajegyet a követőn: <https://github.com/Balls0fSteel/Rookparcel/issues>. Kérjük, adja meg:

1. A rendszerét (Windows 11, Ubuntu 24.04, macOS 15...) és a letöltött fájlt.
2. A Rookparcel verzióját, a **Súgó > A Rookparcel névjegye** helyről.
3. Mit tett, mit várt, és mi történt helyette, lépésről lépésre.
4. Összeomlás vagy elakadt torrent esetén a **Napló** lap sorait, vagy a `rookparcel-nox run --verbose` kimenetét.

Hagyja ki a jelszavakat, az API-kulcsokat és minden más személyeset; a hibakövető nyilvános.

## Biztonsági probléma jelzése

Ne nyisson nyilvános hibajegyet. Használja a GitHub privát sérülékenységjelentését: <https://github.com/Balls0fSteel/Rookparcel/security/advisories/new>. Lásd [SECURITY.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/SECURITY.md).

## GYIK

**Ingyenes a Rookparcel?** Nem kereskedelmi célra ingyenes a PolyForm Noncommercial License 1.0.0 alapján: személyes használatra, tanulmányokhoz, hobbiprojektekhez, jótékonysági szervezeteknek, iskoláknak és közintézményeknek, díjmentesen. A kereskedelmi használathoz külön engedély kell. Lásd [LICENSE.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/LICENSE.md).

**Kapcsolatban áll a µTorrenttel vagy a qBittorrenttel?** Nem. A Rookparcel a µTorrent 2.2.1 külsejét és viselkedését alkotja újra, de egyetlen sort sem használ a kódjából. A motorja a libtorrent-rasterbar 2.0.11 független Rust-átirata. A Web API-ja a qBittorrent közzétett dokumentációját követi, hogy olyan eszközök, mint a Sonarr, tudjanak beszélni vele.

**Milyen nyelveken beszél?** Angolul és további 34 nyelven, mind beépítve. A rendszer nyelvét követi; másikat a **Tulajdonságok > Általános** oldalon választhat. A fordítások gépiek; a javításokat hibajegyként szívesen fogadjuk.

**Mit küld a Rookparcel, és kinek?** A frissítésellenőrzés egyetlen kis fájlt tölt le a projekt webhelyéről, semmi olyat nem küldve, ami Önre vagy a torrentjeire vonatkozna; a **Tulajdonságok > Általános > Adatvédelem** helyen kikapcsolhatja. A keresések csak az Ön által hozzáadott keresési forrásokhoz kerülnek. Mint minden BitTorrent-kliens, a Rookparcel is megmutatja az IP-címét a trackereknek, a DHT-nak és a torrentjei peerjeinek. Lásd [A Rookparcel frissítése](A-Rookparcel-frissítése).

**Hol vannak a beállításaim és a torrentjeim?** Lásd [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait).

**Futtathatok több példányt?** Igen, profilként. Lásd [Profilok](Profilok).

**Hol kaphatok segítséget?** Olvassa el a [GYIK-ot a webhelyen](https://balls0fsteel.github.io/Rookparcel/faq.html) és a wiki oldalait, vagy kérdezzen a [hibakövetőn](https://github.com/Balls0fSteel/Rookparcel/issues).
