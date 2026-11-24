*Ez a [Preferences reference](Preferences-reference) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# A beállítások leírása

Ez az oldal felsorolja a Tulajdonságok ablak minden oldalát, és azt, hogy az egyes beállítások mit csinálnak, abban a sorrendben, ahogy az ablak mutatja őket. Minden szakasz arra az oldalra hivatkozik, amely a témát részletesen elmagyarázza.

## A Tulajdonságok megnyitása

Nyissa meg a **Beállítások > Tulajdonságok** ablakot, nyomja meg a <kbd>Ctrl</kbd>+<kbd>P</kbd> billentyűket, vagy kattintson az eszköztár **Tulajdonságok** gombjára. Az oldalak a bal oldalon vannak felsorolva; a **Lemezgyorsítótár**, a **Trackerek**, a **Program futtatása** és a **Felület zárolása** a **Haladó** alatt található. Az **OK** ment és bezár, az **Alkalmaz** ment, és nyitva hagyja az ablakot, a **Mégse** elveti a módosításokat.

A súgó be van építve:

- Ha egy beállításra mutat, szövegbuborék magyarázza el. Az oldalak alatti panel ugyanezt a szöveget mutatja az egérmutató alatti beállításhoz, vagy ha nincs ilyen, annak, amelyiké a billentyűzetfókusz.
- Az <kbd>F1</kbd> megnyomására a fókuszban lévő beállítás súgója jelenik meg mellette.

*A súgószövegek, a szövegbuborékok és az F1 az 1.1.57-ben jelentek meg.*

Az alábbi szövegek a beépített súgó rövid változatai. Az oldalakon a sebességek kB/s-ban vannak megadva; a 0 korlátlant jelent, kivéve, ha a címke mást mond.

## Általános

![Tulajdonságok, Általános oldal](images/prefs-general.png)

| Beállítás | Mit csinál |
|---|---|
| **Felület:** (nyelv) | A menük, a párbeszédablakok és a webes felület nyelve. Az első lehetőség a rendszer nyelvét követi. Az asztali ablakok a következő indításkor váltanak. |
| **Társítás a .torrent fájlokkal** | Ezt a Rookparcel-példányt teszi azzá a programmá, amely a `.torrent` fájlokat megnyitja, csak a felhasználói fiókjára vonatkozóan. A rendszer saját alapértelmezett alkalmazások beállításában választott program erősebb ennél. |
| **Társítás a magnet: hivatkozásokkal** | Ezt a Rookparcel-példányt teszi azzá a programmá, amely a böngészőben kattintott `magnet:` hivatkozásokat megnyitja, csak a felhasználói fiókjára vonatkozóan. Profilok esetén csak egy profil birtokolhatja őket. |
| **Társítás ellenőrzése indításkor** | Indításkor megkérdezi, átvegye-e a `.torrent` fájlokat és a `magnet:` hivatkozásokat, ha nem a Rookparcel nyitja meg őket. Alapból ki van kapcsolva. |
| **Rookparcel indítása a rendszerrel** | Bejelentkezéskor elindítja a Rookparcelt. Minden profilnak saját bejegyzése van. |
| **Indítás kis méretben** | Az automatikus indításkor a Rookparcel az ablaka megjelenítése nélkül indul, a tálcán, ahol van ilyen. |
| **Alapértelmezett alkalmazások...** (csak Windows) | Megnyitja a Windows Beállítások Alapértelmezett alkalmazások oldalát. Az ott választott program erősebb a fenti négyzeteknél, és ezt a választást csak Ön változtathatja meg. |
| **Frissítések automatikus keresése** (Adatvédelem) | Az indítás után nagyjából tíz másodperccel, majd 12 óránként megkérdezi a projekt webhelyét, megjelent-e újabb kiadás. Kérdezés nélkül semmi sem töltődik le és semmi sem települ. A **Súgó > Frissítések keresése** így is, úgy is működik. |
| **Frissítés bétaverziókra** (Adatvédelem) | A frissítések keresésekor a kiadás előtti (béta) verziókat is felajánlja. |
| **.!ut hozzáfűzése a befejezetlen fájlokhoz** (Letöltéskor) | A még letöltés alatt álló fájl a neve után `.!ut` kiterjesztéssel tárolódik (`film.mkv.!ut`), és abban a pillanatban kapja vissza a rendes nevét, amikor teljesen letöltődött, és átment az ellenőrzésen. A be- vagy kikapcsolás azonnal átnevezi a futó torrentek fájljait, a leállított torrentekét pedig az indulásukkor. Alapból ki van kapcsolva. |

Windowson az integrációs négyzetek a **Windows-integráció** alatt vannak; a GTK-s változatban a csoport neve **Asztali integráció**.

*A .!ut hozzáfűzése a befejezetlen fájlokhoz az 1.2.63-ban jelent meg.* A µTorrent által `.!ut`, illetve a qBittorrent által `.!qB` kiterjesztéssel hagyott fájlokat a program felveszi, akár be van jelölve a négyzet, akár nem. Az a fájl, amelyet egy másik program nyitva tart, megtartja a nevét, amíg át nem nevezhető. Az a fájl, amelynek `.!ut` kiterjesztéssel kiegészített neve ugyanannak a torrentnek egy másik fájlja, megtartja a sima nevét, és az Eltávolítás és > Adatok törlése csak akkor töröl `.!ut` vagy `.!qB` fájlt, ha az ennek a torrentnek az adatait tartalmazza.

További információ: [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás), [Torrentek hozzáadása](Torrentek-hozzáadása), [A Rookparcel frissítése](A-Rookparcel-frissítése), [Profilok](Profilok), [Lemez és tárolás](Lemez-és-tárolás#befejezetlen-fájlok-és-a-ut).

## Felület

| Beállítás | Mit csinál |
|---|---|
| **Torrentek eltávolításának megerősítése** | Rákérdez, mielőtt eltávolít torrenteket a listából, és megmondja, hogy a fájljaikat is törli-e. Kikapcsolva az **Eltávolítás** azonnal végrehajtódik. |
| **Téma** (Windows) | Az ablak színei: a **Rendszer (világos vagy sötét)** a Windows alkalmazásokra vonatkozó *Mód kiválasztása* beállítását követi, a **Világos** és a **Sötét** rögzíti őket. A Windows Nagy kontraszt módja megtartja a saját színeit. A sötét módhoz Windows 10 1809-es vagy újabb verziója kell; régebbi Windows ezt a mező alatt jelzi, és világos marad ([Sötét mód](Sötét-mód)). |
| **Téma** (csak a webes felületen) | A webes felület oldalának színei mindenkinek, aki megnyitja: Rendszer (világos vagy sötét), Világos, Sötét, Éjfél, Pala, Solarized sötét, Solarized világos, Nagy kontraszt, Postás varjú, valamint a saját CSS-fájljai a beállítási mappa `webui/themes` mappájából ([Webes felület](Webes-felület)). |
| **Téma ebben a böngészőben** (csak a webes felületen) | Csak ennek a böngészőnek szóló téma, amely ezen az eszközön tárolódik, és a kiszolgáló választása helyett érvényes. Az **A kiszolgáló alapértelmezése** választás újra a kiszolgáló témáját követi. |
| **Váltakozó háttérszín a listákban** | A lista minden második sorát árnyalja. |
| **Aktuális sebesség megjelenítése a címsorban** | A főablak címének elejére írja az aktuális letöltési és feltöltési sebességet, ami a tálcán is látszik. |
| **Buborékértesítések a tálcán** | Értesítést mutat a torrent nevével, amikor egy letöltés befejeződik. |
| **Bezáráskor a tálcára** (Értesítési terület) | Az ablak bezárógombja kilépés helyett a tálcára rejti a Rookparcelt, így az átvitelek folytatódnak. Kilépni a menüből vagy a tálcaikon menüjéből lehet. |
| **Kis méretnél a tálcára** (Értesítési terület) | A kis méretre állítás a tálcára rejti az ablakot a tálca gombja helyett. A tálcaikonra kattintva hozható vissza. |
| **Ne indítsa el automatikusan a letöltést** (Torrentek hozzáadásakor) | Az új torrentek leállítva és a soron kívül kerülnek hozzáadásra, így semmi sem töltődik le, amíg el nem indítja őket. |
| **A program ablakának aktiválása** | Előtérbe hozza a főablakot, amikor egy másik program, például egy böngésző átad egy torrentet a Rookparcelnek. Kikapcsolva csak az Új torrent hozzáadása ablak jön elő. |
| **Ablak megjelenítése a torrentben lévő fájlokkal** | Minden hozzáadott torrenthez megnyitja az Új torrent hozzáadása ablakot, hogy előbb kiválaszthassa a mappát és a fájlokat; magnet hivatkozásnál akkor, amikor a fájllistája megérkezett. Kikapcsolva a torrentek azonnal, az alapértelmezésekkel kerülnek hozzáadásra. |
| **Ne töltse le az ilyen nevű fájlokat:** (Torrentek hozzáadásakor) | Azok a fájlok, amelyek nevei megfelelnek az alatta lévő mező valamelyik mintájának, jelöletlenül indulnak, vagyis nem töltődnek le, a mostantól hozzáadott torrentekben. A mintákat pontosvesszővel válassza el; a `*` tetszőleges karaktereket, a `?` egyet jelent, a kis- és nagybetű különbsége nem számít, például: `*.lnk; Thumbs.db; *sample*`. Csak a fájl saját neve számít, a mappáié nem. Alapból ki van kapcsolva. |

*A Ne töltse le az ilyen nevű fájlokat az 1.2.63-ban jelent meg.* Egy ilyen fájlt az Új torrent hozzáadása ablakban, vagy később a Fájlok lapon még bejelölhet. Az a torrent, amelynek minden fájlja megfelel, érintetlen marad. A lista az Új torrent hozzáadása ablak nélkül hozzáadott torrentekre is vonatkozik, például a figyelt mappából, az RSS-ből vagy a Sonarrból és a Radarrból érkezőkre, de a már a listában lévő torrentekre nem. Lásd [Torrentek hozzáadása](Torrentek-hozzáadása#név-szerint-kihagyott-fájlok).

További információ: [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás), [Torrentek hozzáadása](Torrentek-hozzáadása), [Torrentek kezelése](Torrentek-kezelése), [A főablak](A-főablak).

## Mappák

| Beállítás | Mit csinál |
|---|---|
| **Új letöltések helye:** | A mappa, ahová az új torrentek kerülnek, hacsak hozzáadáskor nem választ másikat. |
| **Kész letöltések áthelyezése ide:** | Amikor egy torrent letöltése befejeződik, a fájljai ebbe a mappába kerülnek, és onnan folytatja a megosztást. A mostantól befejeződő torrentekre vonatkozik. |
| **A torrent címkéjének hozzáfűzése** | A címkével rendelkező, befejezett torrentet a kész letöltések mappáján belül a címkéről elnevezett mappába helyezi. |
| **Csak az alapértelmezett letöltési mappából helyezze át** | Csak azokat a torrenteket helyezi át, amelyek az új letöltések mappájába lettek mentve; az Ön által máshová mentettek a helyükön maradnak. |
| **.torrent fájlok tárolása itt:** | Minden hozzáadott `.torrent` fájlról másolatot tart ebben a mappában, a torrent nevén. A magnet hivatkozás másolata a metaadatainak megérkezése után tárolódik. |
| **Kész feladatok .torrent fájljainak áthelyezése ide:** | Amikor egy torrent letöltése befejeződik, a tárolt `.torrent` másolata ebbe a mappába kerül. |
| **.torrent fájlok automatikus betöltése innen:** | Figyeli ezt a mappát, és az Új torrent hozzáadása ablak nélkül hozzáadja az ide tett új `.torrent` fájlokat. A betöltött fájl `.torrent.loaded` nevet kap. |
| **Betöltött .torrent fájlok törlése** | Az automatikusan betöltött `.torrent` fájlt törli, amint a torrentje hozzáadódott, átnevezés helyett. |

További információ: [Lemez és tárolás](Lemez-és-tárolás), [Torrentek hozzáadása](Torrentek-hozzáadása), [Várólista és seedelés](Várólista-és-seedelés).

## Kapcsolat

![Tulajdonságok, Kapcsolat oldal](images/prefs-connection.png)

| Beállítás | Mit csinál |
|---|---|
| **Bejövő kapcsolatok portja:** | A port, amelyre a többi peer csatlakozik. Engedélyezze a tűzfalon, és továbbítsa a routerben, vagy bízza az UPnP-re vagy a NAT-PMP-re. A módosítás azonnal érvénybe lép. |
| **Véletlen port** (gomb) | Egy véletlen portot ír be 10000 és 60000 között. Az alkalmazás után lesz használatban. |
| **Véletlen port minden indításkor** | Minden indításkor új véletlen portot választ. A routerben kézzel továbbított port ekkor már nem egyezik. |
| **UPnP porttovábbítás bekapcsolása** | Megkéri az UPnP-s routert, hogy továbbítsa a figyelő portot erre a számítógépre. Kapcsolja ki, ha Ön továbbítja a portot, vagy a router rosszul viselkedik. |
| **NAT-PMP porttovábbítás bekapcsolása** | Megkéri a routert, hogy NAT-PMP-vel vagy PCP-vel továbbítsa a portot; sok router ezt kínálja az UPnP mellett vagy helyett. |
| **Típus:** (Proxykiszolgáló) | A proxy fajtája: **Socks4**, **Socks5** vagy **HTTP**. A **(nincs)** közvetlenül csatlakozik. |
| **Proxy:** és **Port:** | A proxykiszolgáló gépneve vagy IP-címe, és a port, amelyen figyel (például 1080 sok SOCKS-kiszolgálónál). |
| **Hitelesítés**, **Felhasználónév:**, **Jelszó:** | Bejelentkezik a proxyra. A SOCKS4-proxy csak a felhasználónevet kapja meg. |
| **Gépnevek feloldása a proxyn keresztül** | A trackerek és a webes seedek neveit a proxy oldja fel, így a saját DNS-kiszolgálója nem látja őket. |
| **Proxy használata a peerkapcsolatokhoz** | A peerekhez és webes seedekhez menő kapcsolatokat a proxyn át küldi. A bejövő kapcsolatokat ekkor elutasítja, a porttovábbítás és a helyi peerkeresés kikapcsolva marad; az uTP-hez és a DHT-hoz SOCKS5-proxy kell. |
| **Proxy használata a trackerkapcsolatokhoz** | A tracker-bejelentkezéseket a proxyn át küldi, így a trackerek a proxy címét látják az Öné helyett. |
| **Hálózat:** (Csak ennek a hálózatnak a használata) | Az összes torrentforgalmat egy hálózati adapteren tartja: **Bármely VPN-adapter**, egy név szerint megadott adapter, vagy **Címtartomány (lent)**. A **Bármely hálózat (ki)** kikapcsolja ezt. A webes felület és a távoli kapcsolatok nincsenek hozzákötve. |
| **Címtartomány:** | Egy tartomány, például `10.2.0.0/16` vagy `fd00::/8`, a **Címtartomány (lent)** választáshoz: az az adapter viszi a torrentforgalmat, amelyiknek van címe benne. |
| **Torrentek szüneteltetése, amíg ez a hálózat hiányzik** | Amíg egy adapter sem felel meg (leállt a VPN), a futó torrentek várakoznak, és a „Várakozás a hálózatra” állapotot mutatják, majd maguktól folytatódnak. |

*A Csak ennek a hálózatnak a használata csoport az 1.1.58-ban jelent meg.*

További információ: [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal).

## Sávszélesség

![Tulajdonságok, Sávszélesség oldal](images/prefs-bandwidth.png)

| Beállítás | Mit csinál |
|---|---|
| **Legnagyobb feltöltési sebesség (kB/s)** | A legtöbb, amennyit az összes torrent együtt feltölt másodpercenként. A 0 korlátlant jelent. Az ütemező Korlátozva óráiban és az Alacsony ping megtartása (a Játék oldalon) időlegesen csökkentheti. |
| **Legnagyobb letöltési sebesség (kB/s)** | Ugyanez a letöltésekre. |
| **Sebességkorlát alkalmazása az átviteli többletforgalomra** | A peerforgalom TCP/IP-fejléceit is beleszámolja a korlátokba, így azok megegyeznek azzal, amit a vonal szállít. Alapból ki van kapcsolva. |
| **Sebességkorlát alkalmazása az uTP-kapcsolatokra** | A korlátokat az uTP-peerekre is alkalmazza. Kikapcsolva az uTP-peereket csak az uTP saját késleltetésvezérlése lassítja. Alapból be van kapcsolva. |
| **Kapcsolatok legnagyobb száma összesen:** | A legtöbb egyszerre nyitott peerkapcsolat az összes torrentben. Csökkentse, ha a router nem bírja. |
| **Csatlakozott peerek legnagyobb száma torrentenként** | A legtöbb peer, amelyhez egy torrent csatlakozik; a 0 azt jelenti: nincs korlát. A mostantól hozzáadott torrentekre vonatkozik. |
| **Feltöltési helyek száma torrentenként** | Hány peernek tölt fel egy torrent egyszerre; a 0 azt jelenti: nincs korlát. A mostantól hozzáadott torrentekre vonatkozik. |
| **Feltöltési helyek száma összesen** | Hány peernek tölt fel az összes torrent együtt egyszerre; a -1 azt jelenti: nincs korlát. Kevesebb hely nagyobb sebességet ad az egyes peereknek. |

További információ: [Sebességkorlátok és ütemező](Sebességkorlátok-és-ütemező).

## BitTorrent

![Tulajdonságok, BitTorrent oldal](images/prefs-bittorrent.png)

| Beállítás | Mit csinál |
|---|---|
| **DHT-hálózat bekapcsolása** | Tracker nélkül, a DHT-n át talál peereket; a legtöbb magnet hivatkozásnak szüksége van rá. A privát torrentek sosem használják. |
| **Helyi peerkeresés bekapcsolása** | Peereket talál ugyanezekhez a torrentekhez a helyi hálózaton. A privát torrentek sosem használják. |
| **Peercsere bekapcsolása** | Lehetővé teszi, hogy a csatlakozott peerek további peereket mondjanak egymásnak. A privát torrentek sosem használják. |
| **uTP bekapcsolása (UDP-átvitel sávszélesség-kezeléssel)** | A TCP mellett uTP-n is csatlakozik peerekhez és fogadja őket. Az uTP lassít, ha a hálózat foglalt. |
| **Kimenő:** (Protokolltitkosítás) | **Kikapcsolva**: sosem titkosít. **Bekapcsolva**: a sima és a titkosított próbálkozások peerenként váltakoznak, a bevált marad. **Kényszerítve**: csak titkosítva, a nem titkosítani tudó peerek elvesznek. |
| **Titkosítatlan bejövő kapcsolatok engedélyezése** | Fogadja a nem titkosított bejövő kapcsolatokat. Kikapcsolva csak a titkosítottakat fogadja. |
| **IP-szűrő bekapcsolása** | Blokkolja azokat a peereket, amelyeknek a címe a tiltólistán szerepel; a lista lehet eMule `ipfilter.dat`, PeerGuardian P2P formátumú, vagy CIDR-sorok, például `10.0.0.0/8`. Az üres útvonal a beállítási mappa `ipfilter.dat` fájlját jelenti. |
| **Újratöltés** | Azonnal betölti a tiltólistát, és megmutatja, hány szabályt tartalmaz. Az IP-szűrőnek bekapcsolva kell lennie. |
| **Tiltólista letöltése erről a címről (https):** | A tiltólistát az alatta lévő `https://` címről veszi a fenti fájl helyett, a proxyn át, ha van beállítva. Az utoljára erről a címről letöltött másolat azonnal használatba kerül indításkor, a listát pedig naponta egyszer újra letölti. Amíg be van jelölve, a fájl és az **Újratöltés** szürke. |
| **Frissítés most** | Azonnal letölti a tiltólistát a címről. Az alatta lévő sor megmutatja, hány szabályt tartalmaz és mikor töltődött le, vagy hogy miért sikertelen az utolsó letöltés. |

*A címről letöltött tiltólista az 1.2.63-ban jelent meg.* A lista lehet `ipfilter.dat`, PeerGuardian P2P vagy CIDR (`10.0.0.0/8`) alakú, sima vagy gzip-pel tömörített; a zip-archívumokat nem támogatja. Ha egy letöltés sikertelen, vagy nem ilyen lista, a használatban lévő szabályok megmaradnak. Az új cím megtartja a régi cím másolatát, amíg az újról nem töltődik le egy lista, így egy sikertelen első letöltéssel nem vész el semmi; a Windowsos programban a **Frissítés most** egy még nem alkalmazott címmel csak kipróbálja azt, amíg az **OK** vagy az **Alkalmaz** meg nem történik.

További információ: [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal#ip-szűrő).

## Sorba állítás

![Tulajdonságok, Sorba állítás oldal](images/prefs-queueing.png)

| Beállítás | Mit csinál |
|---|---|
| **Aktív torrentek legnagyobb száma (feltöltés vagy letöltés)** | A legtöbb egyszerre futó sorba állított torrent; a többi vár. A -1 azt jelenti: nincs korlát. A kényszerített torrentek a soron kívül futnak. |
| **Aktív letöltések legnagyobb száma** | A legtöbb egyszerre letöltő sorba állított torrent. A -1 azt jelenti: nincs korlát. |
| **Aktív megosztások legnagyobb száma** | A legtöbb egyszerre megosztó, befejezett sorba állított torrent. A -1 azt jelenti: nincs korlát. |
| **A lassú torrentek ne számítsanak bele ezekbe a korlátokba** | Az a torrent, amely egy percig nagyjából 2 kB/s-nál kevesebbet mozgat, nem foglal helyet, így egy másik elindulhat. |
| **Legkisebb arány (%)** (Megosztási cél) | A letöltött mennyiség ennyi százalékát tölti fel; a 150 másfélszeresét jelenti. A 0 azt jelenti: nincs arány cél. Nem csinál semmit, amíg az alatta lévő négyzet nincs bejelölve. |
| **Legrövidebb megosztási idő (perc)** | Ennyi percig oszt. A 0 azt jelenti: nincs idő cél. Ha mindkét cél meg van adva, mindkettőt el kell érni. Nem csinál semmit, amíg az alatta lévő négyzet nincs bejelölve. |
| **Feltöltési sebesség korlátozása (kB/s): [0: leállítás]** | Érvénybe helyezi a megosztási célt. Bejelölve a célt elérő torrent leáll (0), vagy tovább oszt, a feltöltési sebessége a megadott számra korlátozva. Kikapcsolva, ahogy kezdetben, a torrentek a cél után is tovább osztanak. |

További információ: [Várólista és seedelés](Várólista-és-seedelés).

## Ütemező

![Tulajdonságok, Ütemező oldal](images/prefs-scheduler.png)

| Beállítás | Mit csinál |
|---|---|
| **Ütemező bekapcsolása** | A heti táblázatot követi óráról órára, helyi idő szerint. Kikapcsolva a Rookparcel mindig teljes sebességgel fut. |
| **Ütemezési táblázat** | A hét minden órájához egy cella; kattintással vagy húzással lépked a **Teljes sebesség**, a **Korlátozva**, a **Kikapcsolva** és a **Csak megosztás** között. |
| **Korlátozott feltöltési sebesség (kB/s)** és **Korlátozott letöltési sebesség (kB/s)** | A Korlátozva órák alatti korlátok a szokásosak helyett. A 0 korlátlant jelent. |
| **DHT kikapcsolása leállításkor** | A Kikapcsolva órák alatt a DHT-t is kikapcsolja. Alapból be van kapcsolva. |

További információ: [Sebességkorlátok és ütemező](Sebességkorlátok-és-ütemező).

## Webes felület

![Tulajdonságok, Webes felület oldal](images/prefs-webui.png)

Ez az oldal csak a Windowsos változatban van meg. Lehetővé teszi, hogy egy böngésző vezérelje ennek az ablaknak a torrentjeit.

| Beállítás | Mit csinál |
|---|---|
| **Webes felület bekapcsolása** | A Rookparcel futása alatt kiszolgálja a webes felületet ebből az ablakból. Felhasználónév és jelszó kell hozzá. |
| **Felhasználónév:** és **Jelszó:** | A bejelentkezés. A jelszó csak hashként tárolódik; hagyja üresen a tárolt megtartásához. Egy új jelszó minden más böngészőt kijelentkeztet. |
| **API-kulcs elfogadása (Authorization: Bearer)** | Lehetővé teszi, hogy olyan alkalmazások, mint a Sonarr vagy a Radarr, bejelentkezés nélkül használják a webes felületet az alatta lévő kulccsal. Alapból ki van kapcsolva. |
| **API-kulcs (legalább 16 karakter):** | A kulcs, amelyet az alkalmazások küldenek. Csak hashként tárolódik, és soha többé nem jelenik meg. |
| **Nincs hitelesítés ezen alhálózatok klienseinek** | Az ezekben a hálózatokban lévő kliensek bejelentkezés nélkül használják a webes felületet. Csak olyan hálózatokat sorolja fel, amelyekben ugyanannyira megbízik, mint a jelszóban. |
| **Figyelő port:** | A port, amelyen a webes felület figyel; a 8080-at javasolja, vagy a következő szabadot. |
| **Kapcsolatok engedélyezése más gépekről** | Minden hálózati címen figyel, így más számítógépek és telefonok is elérhetik. Nekik a `rookparcel.exe` programhoz egy Windows tűzfalszabály is kell. |
| **HTTPS használata (önaláírt tanúsítvány)** | A webes felületet HTTPS-en szolgálja ki egy olyan tanúsítvánnyal, amelyet a Rookparcel maga készít. A böngészők figyelmeztetnek rá, amíg el nem fogadja. |
| **Kiszolgálótartományok** | Gépnevek, amelyekre a webes felület az IP-címeken és a localhoston kívül válaszol, például ennek a számítógépnek a neve egy Dockerben futó alkalmazáshoz. |
| **Webes felület megnyitása** | Megnyitja a futó webes felületet a böngészőben. |

További információ: [Webes felület](Webes-felület), [Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr).

## Játék

*Az 1.2.78-ban jelent meg.* Olyan beállítások, amelyek reagálóképesen tartják a számítógépet és a vonalat, amíg a Rookparcel dolgozik. Az **Alacsony ping megtartása** a Sávszélesség oldalról, a korábbi Lemez I/O oldal pedig a Haladó alól került ide.

**Ping**

| Beállítás | Mit csinál |
|---|---|
| **Alacsony ping megtartása (a sebességek automatikus korlátozása)** | Méri a vonal késleltetését, és automatikusan korlátozza a feltöltést és a letöltést, amíg az nő, játékokhoz és hívásokhoz. Sosem emeli meg az Ön saját korlátait; az állapotsor „auto”-ként mutatja a korlátot. |

**Ellenőrzés**

| Beállítás | Mit csinál |
|---|---|
| **Kíméletes torrentellenőrzés (alacsony prioritás, elsőbbség a többi programnak)** | A torrenteket a rendszer háttérlemez-prioritásán ellenőrzi, és szünetel, amíg az olvasások a szokásosnál lassabbak, így a számítógép reszponzív marad. Az ellenőrzések tovább tartanak, amíg a lemez foglalt. |
| **Ellenőrzés korlátozása (MB/s):** | Korlátozza az egyes ellenőrzések olvasási sebességét, olyan lemezhez, amely a kíméletes ellenőrzések alatt is lemarad. Alapból ki van kapcsolva. |
| **Egyszerre ellenőrzött meghajtók (meghajtónként egy torrent):** | Hány meghajtót ellenőriz egyszerre; az ugyanazon a meghajtón lévő torrentek a sorukra várnak. A -1 azt jelenti: nincs korlát. |

**Meghajtók**

| Beállítás | Mit csinál |
|---|---|
| **Hangolás a meghajtó típusához (SSD, merevlemez, hálózat)** | Felismeri, hogy az egyes mentési mappák SSD-n, merevlemezen, cserélhető meghajtón vagy hálózati megosztáson vannak-e, és ehhez igazítja, hány lemezművelet fut egyszerre. |

**Ha a lemez foglalt**

| Beállítás | Mit csinál |
|---|---|
| **Letöltések lassítása (lemezírások)** | Amíg egy meghajtó az írásokra érezhetően lassabban válaszol a szokásosnál, lépésenként felezi a letöltési sebességet, de 50 kB/s alá soha, és újra engedi nőni, ha a lemez elcsendesedett. |
| **Feltöltések lassítása (lemezolvasások)** | Ugyanez a feltöltésekre, amíg egy meghajtó lassan válaszol az olvasásokra. |

A rosszul felismert meghajtótípust a Haladó oldalon a `drive_overrides` javítja. Kezdetben csak a **Hangolás a meghajtó típusához (SSD, merevlemez, hálózat)** négyzet van bejelölve; a többi nincs.

További információ: [Alacsony ping](Alacsony-ping), [Lemez és tárolás](Lemez-és-tárolás).

## Haladó

![Tulajdonságok, Haladó oldal](images/prefs-advanced.png)

A Haladó oldal a motor saját beállításainak táblázata, a libtorrent neveivel. Gépeljen be egy névrészletet a **Szűrő:** mezőbe a lista szűkítéséhez. Jelöljön ki egy sort, írjon be új értéket az **Érték:** mezőbe (vagy kapcsolónál válassza az **Igaz** vagy a **Hamis** értéket), és kattintson a **Beállítás** gombra. A **Visszaállítás** a kijelölt beállítást az alapértelmezésre állítja. A `*` az alapértelmezéstől eltérő értéket jelöli. Mint a Tulajdonságok minden oldalán, a módosítás az **OK** vagy az **Alkalmaz** gombbal lép érvénybe. A sebességek itt bájt/másodpercben vannak, nem kB/s-ban.

A legtöbb sor ugyanaz a beállítás, amelyet a többi oldal barátságosabb neveken mutat. A többi ezekbe a csoportokba tartozik:

| Csoport | Példák |
|---|---|
| Azonosító | `user_agent`, `peer_fingerprint` (ritkán kell módosítani) |
| Figyelés és porttovábbítás | `listen_interfaces`, `max_retry_port_bind`, `upnp_lease_duration`, `natpmp_gateway` |
| Sor | `active_dht_limit`, `active_tracker_limit`, `inactive_down_rate`, `auto_manage_interval` |
| Kapcsolatok és peerek | `connection_speed`, `max_peerlist_size`, `peer_timeout`, `peer_turnover` |
| DHT | `dht_announce_interval`, `dht_upload_rate_limit`, `dht_bootstrap_nodes` és a védelmi kapcsolói |
| uTP és TCP | `enable_incoming_tcp`, `utp_target_delay`, `mixed_mode_algorithm`, `bt.transp_disposition` |
| Titkosítás | `out_enc_policy`, `in_enc_policy`, `allowed_enc_level`, `prefer_rc4` |
| Trackerek | `announce_to_all_tiers`, `announce_to_all_trackers`, `num_want`, `stop_tracker_timeout` |
| Lemez | `cache_size`, `max_queued_disk_bytes`, `file_pool_size`, `drive_overrides` |
| Webes seedek | `max_web_seed_connections`, `urlseed_timeout`, `ssrf_mitigation` |
| Proxy | `proxy_type`, `proxy_send_host_in_connect`, `socks5_udp_send_local_ep` |
| Alacsony ping megtartása | `auto_limit_delay_ms`, `auto_limit_probe_hosts` |

Gyakran érdemes ismerni ezeket a sorokat:

| Beállítás | Mit csinál |
|---|---|
| `listen_interfaces` | Vesszővel elválasztott címek és portok, amelyeken peereket fogad. A `0.0.0.0` és a `[::]` minden interfészt jelent. |
| `connection_speed` | Az új kimenő kapcsolatok próbálkozásai másodpercenként, az összes torrent között megosztva. Magasabb értéknél gyorsabban talál peereket, de túlterhelheti a gyenge routereket. Alapérték: 30. |
| `announce_to_all_trackers` | Be: egy szint minden trackerét megkeresi, nem csak az első működőt. Több peert találhat, több trackerforgalom árán. Alapból ki van kapcsolva. |
| `upnp_lease_duration` | Hány másodpercre kér UPnP-porttovábbítást. A 0 állandó továbbítást kér, amire néhány routernek szüksége van. Alapérték: 3600. |
| `stop_tracker_timeout` | Hány másodpercet vár a Rookparcel kilépéskor a trackerekre és a routerekre. Alapérték: 5. Csökkentse a gyorsabb kilépéshez. |
| `auto_limit_delay_ms` | Hány ezredmásodperc többletpinget enged az Alacsony ping megtartása a vonal tétlen pingje fölött. Az alacsonyabb simábban tartja a játékokat; a magasabb nagyobb sebességet enged. Alapérték: 15. |
| `drive_overrides` | Kijavítja azokat a meghajtókat, amelyek típusát rosszul ismeri fel, útvonal=típus párokkal, például `D:/=hdd, /mnt/nas=network`. |
| `bt.transp_disposition` | Az engedélyezett kapcsolattípusok összeadott bitekként: 1 kimenő TCP, 2 kimenő uTP, 4 bejövő TCP, 8 bejövő uTP. Az alapértelmezett 15 mindet engedi; az 5 csak TCP. |

Az egyes sorok teljes súgója a táblázat alatti panelen jelenik meg, ha kijelöli.

## Lemezgyorsítótár

| Beállítás | Mit csinál |
|---|---|
| **Lemezolvasások gyorsítótárazása** | A nemrég olvasott részeket a memóriában tartja, ami megosztáskor lemezkeresést spórol. Az írásokat a rendszer gyorsítótárára hagyja. |
| **Automatikus gyorsítótárméret felülbírálása (MB):** | Az olvasási gyorsítótárhoz az automatikus 32 MB helyett ezt a méretet használja. Egy nagyobb gyorsítótár segíthet a forgalmas megosztásnál, a memória rovására. |
| **Ennél régebben nem használt blokkok eltávolítása (másodperc):** | Eldobja azokat a gyorsítótárazott blokkokat, amelyeket ennyi ideig egy peer sem kért. Kikapcsolva addig maradnak, amíg újabb blokkok ki nem szorítják őket. |
| **Lemezírási sor (kB):** | Mennyi letöltött adat várhat a kiírásra; efölött a Rookparcel nem olvas a peerektől, amíg a lemez utol nem éri magát. A 4 MB alatti értékek 4 MB-ként hatnak. |
| **Megnyitott fájlok legnagyobb száma:** | A legtöbb egyszerre nyitva tartott fájl. Emelje meg, ha sok torrent vagy fájl dolgozik egyszerre. |
| **Lemez I/O szálak (a következő indítástól):** | A torrentadatokat író és olvasó szálak. |
| **Hash-szálak (nincs használatban):** | Kompatibilitási okból megmaradt; ez a verzió nem használja. Az ellenőrzések meghajtónként egyesével futnak, ezt a Játék oldalon lehet beállítani. |

További információ: [Lemez és tárolás](Lemez-és-tárolás).

## Trackerek

*Az 1.2.40-ben jelent meg.* Olyan trackerek, amelyeket a mostantól hozzáadott minden torrent megkap a saját trackerei mellett. Mindkét négyzet kezdetben ki van kapcsolva.

| Beállítás | Mit csinál |
|---|---|
| **Ezen trackerek hozzáadása az új letöltésekhez:** | Hozzáadja a mezőben lévő trackereket, soronként egy címet (`http://`, `https://` vagy `udp://`); az üres sor új szintet kezd. A torrent saját trackerei után következnek. |
| **A webes lista trackereinek hozzáadása (https):** | Az alatta lévő `https://` címről indításkor, majd naponta egyszer beolvas egy trackercímeket tartalmazó szövegfájlt, és a trackereit a mező trackerei után hozzáadja. |
| **Frissítés most** | Azonnal beolvassa a listát a címről. A gomb alatti sor megmondja, hány trackert tartalmaz a lista és mikor olvasták be, vagy hogy miért sikertelen az utolsó próbálkozás. |

Tudnivalók:

- **A privát torrentek sosem kapják meg ezeket a trackereket**, és a már a listában lévő torrentek, az Ön által készített torrentek és a más kliensből importáltak sem. A magnet hivatkozás akkor kapja meg őket, amikor a részletei megérkeztek, és kiderül belőlük, hogy nem privát.
- **A webes listát mindig csak listaként olvassa be.** A címnek `https://` előtaggal kell kezdődnie. Csak a trackercímek sorait használja, legfeljebb 200-at, és az olyan fájlt, amely más (hibaoldal, program), figyelmen kívül hagyja: az utolsó jó lista marad használatban. A fájlból semmi sem kerül végrehajtásra.
- **A lista trackereit az választja ki, aki a címet üzemelteti**, és ezek a trackerek ezután látják az Ön címét és nyilvános letöltéseit. Olyan listát használjon, amelyben megbízik.
- A webes felületen az oldal beállításai csak a jelenlegi jelszóval módosíthatók, a qBittorrent-kompatibilis API pedig nem tudja módosítani őket.
- A `rookparcel-nox` ezeket a beállításokat a webes felületéi mellett tartja, ezért a használatukhoz kell neki a `--webui`.

## Program futtatása

*Az 1.2.41-ben jelent meg.* Elindít egy Ön által megadott programot, amikor egy torrent hozzáadódik vagy befejeződik. Mindkét négyzet kezdetben ki van kapcsolva.

| Beállítás | Mit csinál |
|---|---|
| **A következő program futtatása torrent hozzáadásakor:** | Minden új torrenthez elindítja az alatta lévő sorban megadott programot. Nem indítja azokhoz, amelyek már a listában voltak, és az induláskor betöltött torrentekhez sem. |
| **A következő program futtatása, amikor egy torrent letöltése befejeződik:** | Minden befejezett torrenthez elindítja a programot. Nem indítja újraellenőrzés vagy újraindítás után, és olyan torrenthez sem, amelynek az adatai teljesek voltak, amikor hozzáadta. |

Először a program útvonalát írja be, idézőjelek között, ha szóköz van benne, utána az argumentumait, például: `"C:\Tools\notify.exe" --name "%N" --path "%F"`. Az argumentumokban ezek a kódok a torrent adatait jelentik:

| Kód | Jelentése |
|---|---|
| `%N` | a torrent neve |
| `%L` | a címkéje |
| `%F` | az adatai helye: a fájl, vagy a több fájlból álló torrent mappája |
| `%D` | a mappa, ahová mentve lett |
| `%Z` | a mérete bájtban |
| `%T` | az aktuális trackere |
| `%I`, `%J` | az info-hash-e, v1 és v2 (`-`, ha nincs) |
| `%K` | az azonosítója: a v1 hash, vagy a rövidített v2 hash |
| `%%` | egy százalékjel |

Tudnivalók:

- **A programot közvetlenül indítja, parancsértelmező nélkül.** A csövek, a `>` és a `&&` nem csinálnak semmit; használatukhoz futtasson egy szkriptet, és tegye őket a szkriptbe. Szkript futtatásához, ahol a rendszernek szüksége van rá, előbb nevezze meg az értelmezőjét (`powershell -File ...`, `/bin/sh /path/script.sh`).
- **A torrent neve soha nem válhat paranccsá.** Minden kód a saját argumentumába kerül, idézőjellel vagy anélkül, és a program saját útvonala nem fogad kódot. A név attól még kezdődhet kötőjellel, amelyet egyes programok kapcsolónak olvasnak: írja `--name=%N` alakban, vagy tegyen elé `--` jelet, ahol ez számít.
- **Nincs kód olyan parancsértelmezőhöz vagy értelmezőhöz, amelynek egy karakterláncot kell futtatnia.** Egy ilyen program a nevet parancsként olvasná, ezért az olyan sort, mint a `cmd /c ... %N`, a `powershell -Command ...`, az `sh -c "... %N"`, a `python -c`, a `perl -e`, a `node -e`, az `osascript -e`, az `ssh host ... %N` vagy a `wsl bash -c`, elutasítja, ha kód van benne, az `env`, `nohup`, `nice`, `timeout`, `sudo` és hasonló burkolók mögött is (amelyek programja szintén nem fogad kódot). Ugyanígy elutasítja a kódot az `awk` vagy a `sed` programszövegében, egy parancsértelmezőt futtató `find -exec` parancsban, és egy olyan parancsértelmező vagy értelmező után, amelyet egy másik program indít (`gnome-terminal -- bash -c "... %N"`). Azokat a programokat, amelyek a soruk egy karakterláncát kódként futtatják, akkor is elutasítja, ha a kód bárhol benne van: `script`, `expect`, `forfiles`, `wmic`, szerkesztők, mint a `vim` és az `emacs`, és hibakeresők, mint a `gdb`. Nevezze meg inkább a szkriptet, és adja át neki a kódokat: `powershell -File C:\Tools\done.ps1 "%N"`, `/bin/sh /path/done.sh "%N"`, `python /path/done.py "%N"`, `wsl -e /path/done.sh "%N"`. Más programok saját kapcsolói az övéik: a `notify-send -e "%N"`, az `rsync -c "%F" /backup` és a `/path/done.sh -c "%L"` rendben vannak. Az egy régebbi verzió által mentett, de most elutasított sort kikapcsolja, és a Napló ezt az indításkor megmondja (a sort és az okot).
- A program nem kap ablakot és bemenetet, egyszerre legfeljebb négy fut, és minden indulása és vége bekerül a Naplóba.
- A program azon a számítógépen fut, amelyiken a torrentek vannak: egy távoli `rookparcel-nox` programhoz csatlakozó ablakban ennek az oldalnak a sorai nem csinálnak semmit; a kiszolgálón állítsa be őket.
- **Ki módosíthatja a sorokat.** Aki módosíthatja őket, az az Ön fiókjával futtathat programokat, ezért: az asztali programok a saját számítógépükön; a webes felület csak ugyanazon a számítógépen lévő böngészőből, a jelenlegi jelszóval; a qBittorrent-kompatibilis API soha, és távoli kapcsolat soha. Az a kiszolgáló, amelyet csak máshonnan érhet el (például a Docker), a parancssorában a `--webui-remote-run-program` kapcsolót igényli: lásd [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása). Gondolja meg, mielőtt megadja: vele a webes felület jelszava az egyetlen, ami egy látogató és a kiszolgálón futtatott programok között áll.
- **Fordított proxy mögött.** Ha a webes felületet egy proxyn vagy alagúton át magán a kiszolgálón éri el (nginx, HAProxy, stunnel, SSH-alagút), a kiszolgáló minden látogatót ugyanazon a számítógépen lévő böngészőnek lát, hacsak a proxy mást nem mond. Állítsa be, hogy a proxy hozzáadja a látogató címét (`X-Forwarded-For`; nginx esetén: `proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;`). E fejléc nélkül bárki, aki eléri a proxyt és ismeri a jelszót, módosíthatja ezeket a sorokat, pontosan úgy, mint a `--webui-remote-run-program` esetén.

## Felület zárolása

A PIN-kód, amely a Rookparcel ablakait zárolja, és a kapcsoló, amely kikapcsolja a zárolást. A Tulajdonságok többi részétől eltérően ezen az oldalon minden azonnal érvénybe lép: az **OK**, az **Alkalmaz** és a **Mégse** nem változtat rajta. *Az 1.2.63-ban jelent meg; korábban a PIN-kódot a **Beállítások > Lock PIN** helyen lehetett beállítani.*

| Beállítás | Mit csinál |
|---|---|
| **A Rookparcel zárolásának engedélyezése PIN-kóddal** | Alapból be van kapcsolva. Kikapcsolva nincs **Fájl > Rookparcel zárolása**, nincs <kbd>Ctrl</kbd>+<kbd>L</kbd>, és nincs **Rookparcel zárolása** pont a tálcamenüben. Kikapcsolása PIN-kódot kér, ha van beállítva, és eltávolítja. |
| **PIN-kód beállítása...** | Egy 4–6 számjegyű új PIN-kódot kér, kétszer. Csak amíg nincs PIN-kód beállítva. |
| **PIN-kód módosítása...** | A jelenlegi PIN-kódot kéri, majd egy újat kétszer. |
| **PIN-kód eltávolítása...** | A jelenlegi PIN-kódot kéri, majd eltávolítja. A **Rookparcel zárolása** ezután új PIN-kódot kér, mielőtt zárolna. |

A gombok alatti sor megmondja, be van-e állítva PIN-kód. További információ: [A Rookparcel zárolása](A-Rookparcel-zárolása).

## Különbségek a GTK-s változatban

A GTK-s változatnak (Linux, BSD) ugyanezek az oldalai és beállításai vannak, ezekkel a különbségekkel:

- Nincs **Webes felület** oldal.
- A **Felület** oldalon nincs **Téma** beállítás: a GTK az asztali környezet témáját követi ([Sötét mód](Sötét-mód)). A tálca- és értesítési négyzetek (**Buborékértesítések a tálcán**, **Bezáráskor a tálcára**, **Kis méretnél a tálcára**) csak akkor jelennek meg, ha van tálcaikon a képernyőn.
- Az **Általános** oldalon a Windows-integráció helyett **Asztali integráció** van, és nincs **Alapértelmezett alkalmazások...** gomb. Flatpakban a társítási négyzetek szürkék egy megjegyzés alatt: az alapértelmezett programot az asztali környezet beállításaiban válassza ki. Az **Adatvédelem** csoport Flatpakban nem látszik, mert azt a Flathub frissíti.
- Az <kbd>Escape</kbd> bezárja az ablakot, mint a **Mégse**.

## Különbségek macOS-en

A macOS-es alkalmazás a **Rookparcel > Tulajdonságok** (<kbd>⌘</kbd>+<kbd>,</kbd>) paranccsal nyitja a Tulajdonságokat. Ugyanazok az oldalai, mint a GTK-s változatnak, oldalsávban felsorolva, és ezek a különbségek:

- Minden módosítás azonnal érvénybe lép, mint más Mac-alkalmazásokban: nincs **OK**, **Mégse** vagy **Alkalmaz**. A négyzet vagy a lista a módosításkor lép érvénybe; a szöveg- vagy számmező a <kbd>Return</kbd> megnyomásakor, a mező elhagyásakor vagy az ablak bezárásakor.
- Egy nem megengedett érték, például egy tartományon kívüli port, pirossal látszik a mezője alatt, és az utolsó jó érték marad használatban. Az ablak bezárása visszateszi a jó értéket a mezőbe.
- A társítási négyzetek csak a `Rookparcel.app`-ban működnek, és csak az alapértelmezett profilban.

*A natív macOS-es alkalmazás az 1.2.19-ben jelent meg.*

## Különbségek a webes felületen

A webes felületnek (a `rookparcel-nox` webes felületének, vagy annak, amelyet a Windowsos ablak szolgáltat) **Tulajdonságok** ablaka van ugyanezekkel az oldalakkal. A Haladó oldalon a módosított értékek `*` helyett félkövéren látszanak. További különbségek:

- **Általános**: a társítási és az automatikus indítási négyzetek szürkék, a „A fájltársítás és az automatikus indítás az asztali klienshez tartozik.” megjegyzéssel. A `rookparcel-nox`-nak saját **Adatvédelem** csoportja van a frissítésellenőrzéséhez; a Windowsos ablak által szolgáltatott webes felületnek nincs.
- A **Felület** oldalon csak ezek vannak: **Torrentek eltávolításának megerősítése**, a két témabeállítás, **Ne indítsa el automatikusan a letöltést**, **Ablak megjelenítése a torrentben lévő fájlokkal** és **Ne töltse le az ilyen nevű fájlokat:**.
- **BitTorrent**: az IP-szűrő címének, vagy a fölötte lévő négyzetnek a módosításához a jelenlegi jelszó kell, amelyet a webes felület oldalán a **Jelenlegi jelszó:** mezőbe kell beírni. A Windowsos ablak által szolgáltatott webes felületnek nincs címe, **Frissítés most** gombja vagy állapotsora: azokat az ablak saját Tulajdonságai tartalmazzák.
- A **Sávszélesség** a `rookparcel-nox`-ban tartalmazza az **Alternatív sebességkorlátok** csoportot: **Alternatív feltöltési sebesség**, **Alternatív letöltési sebesség** és **Az alternatív korlátok használata most**; ez utóbbit azok az alkalmazások is átkapcsolják, amelyek a qBittorrent sebességkorlát-módját kapcsolgatják.
- A **Sorba állítás** oldalon a **Feltöltési sebesség korlátozása (kB/s): [0: leállítás]** fölött ott a **A torrent leállítása vagy a feltöltési sebességének korlátozása** négyzet; ez a négyzet helyezi érvénybe a megosztási célt.
- A **Webes felület** oldal kiegészül ezekkel: **Jelenlegi jelszó:** (a bejelentkezés és a biztonsági beállítások módosításához kell), **Nincs hitelesítés a localhost klienseinek**, **Figyelő cím:** (tájékoztatásul látszik), **A Host fejléc ellenőrzésének bekapcsolása**, **Védelem a webhelyek közötti kéréshamisítás (CSRF) ellen**, valamint egy **Biztonság** csoport a **Kliens kitiltása egymást követő hibák után:**, a **Kitiltás időtartama (másodperc):** és a **Munkamenet időkorlátja (másodperc):** beállításokkal.
- A <kbd>Ctrl</kbd>+<kbd>Enter</kbd> **OK**-ként működik; az <kbd>Enter</kbd> a Haladó oldal értékmezőjében beállítja az értéket.

Lásd [Webes felület](Webes-felület) és [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).
