*Ez a [Speed limits and the scheduler](Speed-limits-and-scheduler) című angol oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Sebességkorlátok és az ütemező

Ez az oldal azt tárgyalja, hogyan korlátozhatja a Rookparcel letöltési és feltöltési sebességét: az összes torrentre együtt, egyes torrentekre, valamint óránként a heti ütemezővel. Azokhoz a korlátokhoz, amelyek maguktól követik az internetvonal pingjét, lásd az [Alacsony ping](Alacsony-ping) oldalt.

Minden sebesség kB/s-ban van megadva, ahol 1 kB 1024 bájt. Minden sebességmezőben a 0 korlátlant jelent.

## Általános korlátok

Az általános korlátok az összes torrentet együtt korlátozzák.

1. Nyissa meg a **Beállítások > Tulajdonságok** ablakot (<kbd>Ctrl</kbd>+<kbd>P</kbd>), és válassza a **Sávszélesség** oldalt.
2. Az **A feltöltési sebesség általános korlátja** alatt adja meg a **Legnagyobb feltöltési sebesség (kB/s): [0: korlátlan]** értékét.
3. Az **A letöltési sebesség általános korlátja** alatt adja meg a **Legnagyobb letöltési sebesség (kB/s): [0: korlátlan]** értékét.
4. Kattintson az **OK** vagy az **Alkalmaz** gombra. Az új korlátok azonnal érvénybe lépnek.

![A Tulajdonságok Sávszélesség oldala](images/prefs-bandwidth.png)

Alapértelmezés szerint mindkettő korlátlan. Általában a feltöltés korlátozása az oka, hogy valaki beállít egyet: amikor a Rookparcel olyan gyorsan tölt fel, ahogy a vonal engedi, a weboldalak, a játékok és a hívások mindenkinek lelassulnak a háznál.

### Általános sebességkorlát-beállítások

Ugyanezen az oldalon két négyzet módosítja, mit számolnak a korlátok:

| Beállítás | Mit tesz | Alapértelmezés |
|---|---|---|
| **Sebességkorlát alkalmazása az átviteli többletforgalomra** | A peerforgalom TCP/IP-fejléceit, csomagonként becsülve, beszámítja az általános korlátokba, így a korlátok megfelelnek annak, amit a vonal szállít. | Ki |
| **Sebességkorlát alkalmazása az uTP-kapcsolatokra** | Az általános korlátokat az uTP-peerekre is alkalmazza. Kikapcsolva az uTP-peereket csak az uTP saját késleltetés-szabályozása lassítja, amely ad utat a többi forgalomnak. | Be |

### Kapcsolatok és feltöltési helyek

Ugyanezen az oldalon a **Kapcsolatok száma** csoport nem sebességkorlát, de hatással van a sebességre:

| Beállítás | Jelentése | Alapértelmezés |
|---|---|---|
| **Kapcsolatok legnagyobb száma összesen:** | A legtöbb egyszerre nyitott peerkapcsolat az összes torrenten át. Csökkentse, ha a router nehezen bírja. | 200 |
| **Csatlakozott peerek legnagyobb száma torrentenként: [0: nincs korlát]** | A legtöbb peer, amelyhez egy torrent csatlakozik. | 0 |
| **Feltöltési helyek száma torrentenként: [0: nincs korlát]** | Hány peernek tölt fel egyszerre egy torrent. | 0 |
| **Feltöltési helyek száma összesen: [-1: nincs korlát]** | Hány peernek tölt fel egyszerre az összes torrent együtt, a leggyorsabbakat választva. Kevesebb hely több sebességet ad minden peernek. | 8 |

A két torrentenkénti érték a mostantól hozzáadott torrentekre vonatkozik; egy torrent Jellemzői módosíthatják a saját feltöltési helyeit. A Beállítási útmutató routerkérdése csökkentheti a kapcsolatok számát egy régebbi routerhez: lásd az [Első indítás](Első-indítás) oldalt.

## Egyes torrentek korlátai

Egy torrentnek lehetnek saját korlátai az általánosak alatt. Az általános korlátok emellett továbbra is érvényesek.

**A jobb gombos menüből:**

1. Jelöljön ki egy vagy több torrentet a listában.
2. Kattintson a jobb gombbal, és válassza a **Sávszélesség > Letöltési korlát** vagy a **Sávszélesség > Feltöltési korlát** parancsot.
3. Válasszon sebességet: **Korlátlan**, vagy a 10, 25, 50, 100, 250, 500, 1000, 2500 és 5000 kB/s közül egyet.

Az első kijelölt torrent aktuális sebessége be van jelölve.

![A torrentlista jobb gombos menüje](images/torrent-menu.png)

**A Torrent jellemzőiből**, tetszőleges sebességhez:

1. Jelölje ki a torrenteket, és válassza a **Jellemzők** parancsot (<kbd>Enter</kbd>).
2. A **Sávszélesség-beállítások** alatt adja meg a **Legnagyobb feltöltési sebesség (kB/s): [0: korlátlan]** és a **Legnagyobb letöltési sebesség (kB/s): [0: korlátlan]** értékét.
3. Hogy a torrent saját számú feltöltési helyet kapjon, jelölje be **A feltöltési helyek alapértelmezett számának felülbírálása** négyzetet, és töltse ki a **Feltöltési helyek száma: [0: korlátlan]** mezőt.
4. Kattintson az **OK** gombra.

![A Torrent jellemzői ablak](images/properties.png)

Egy torrent saját korlátai a torrentlista **Letöltési korlát** és **Feltöltési korlát** oszlopában, valamint az **Általános** lapon látszanak. A Jellemzők ablak többi részéhez lásd a [Torrent-részletek és fájlok](Torrent-részletek-és-fájlok) oldalt.

Az a torrent, amely eléri a megosztási célját, magától is kaphat feltöltési korlátot: lásd a [Várólista és seedelés](Várólista-és-seedelés#megosztási-cél) oldalt.

## Az állapotsor

A főablak alján lévő állapotsor mutatja az aktuális sebességeket és a munkamenet összesített adatait, a letöltésnél **D: 1.2 MB/s T: 3.4 GB** alakban, a feltöltésnél **U: ... T: ...** alakban. Amíg az Alacsony ping megtartása vagy a foglalt lemez miatti lassítás visszatart egy irányt, annak a része az automatikus korlátot is mutatja, például **U: 412.0 kB/s (auto 450.0 kB/s) T: ...**.

Az állapotsornak nincs sebességmenüje. Az általános korlátokat a Tulajdonságokban, a torrentek saját korlátait pedig a menüjükből állíthatja be.

Ha be van kapcsolva az **Aktuális sebesség megjelenítése a címsorban** (**Beállítások > Tulajdonságok > Felület**), az ablak címe és a tálcagomb is a sebességekkel kezdődik.

## Az ütemező

Az ütemező egy heti táblázatot követ, óráról órára, és négy mód között kapcsolgatja a Rookparcelt. Alapértelmezés szerint ki van kapcsolva.

1. Nyissa meg a **Beállítások > Tulajdonságok > Ütemező** oldalt.
2. Jelölje be az **Ütemező bekapcsolása** négyzetet. A táblázat szürke, amíg ezt meg nem teszi.
3. Kattintson egy cellára az **Ütemezési táblázatban**, hogy az az óra a következő módra váltson. Tartsa lenyomva a gombot, és húzzon, hogy az útba eső cellákat ugyanazzal a móddal fesse be.
4. Az **Ütemező beállításai** alatt adja meg a **Korlátozva** órák sebességét.
5. Kattintson az **OK** vagy az **Alkalmaz** gombra.

![A Tulajdonságok Ütemező oldala](images/prefs-scheduler.png)

A táblázatban naponta egy sor van, hétfővel kezdve, és óránként egy oszlop, a számítógép helyi idejében. A kattintás a **Teljes sebesség**, **Korlátozva**, **Kikapcsolva** és **Csak megosztás** között lépked. A táblázat alatti sor megnevezi a mutató alatti órát és a módját, például „H 09:00 - 10:00: Teljes sebesség”.

| Mód | Szín | Mit tesz a Rookparcel abban az órában |
|---|---|---|
| **Teljes sebesség** | sötétzöld | A szokásos beállításaival fut. |
| **Korlátozva** | világoszöld | Az általános korlátok helyett az alábbi két Korlátozott sebességet használja. |
| **Kikapcsolva** | fehér | Szünetelteti az összes torrentet. A **DHT kikapcsolása leállításkor** esetén a DHT is ki van kapcsolva, így a Rookparcel csendben marad a hálózaton. |
| **Csak megosztás** | világospiros | Feltölt, de nem tölt le semmit. Az ezekben az órákban hozzáadott torrentek is várnak. |

| Ütemező beállítása | Jelentése | Alapértelmezés |
|---|---|---|
| **Korlátozott feltöltési sebesség (kB/s): [0: korlátlan]** | Az általános feltöltési korlát a Korlátozva órákban. | 0 |
| **Korlátozott letöltési sebesség (kB/s): [0: korlátlan]** | Az általános letöltési korlát a Korlátozva órákban. | 0 |
| **DHT kikapcsolása leállításkor** | A Kikapcsolva órákban a DHT-t is kikapcsolja. | Be |

Mire számíthat:

- Amikor az óra módot vált, az állapotsor hálózati része kiegészül az **Ütemező: korlátozva**, az **Ütemező: kikapcsolva** vagy az **Ütemező: csak megosztás** felirattal, a **Napló** lap pedig egy ilyen sort ír: „Ütemező: Korlátozva”. A Teljes sebesség semmit sem mutat.
- A Kikapcsolva nem változtatja meg a torrentek saját állapotát. Amikor a Teljes sebesség visszatér, úgy folytatják, mint korábban, és az a torrent, amelyet Ön leállított, leállítva marad.
- Az **Ütemező bekapcsolása** jelölésének megszüntetése azonnal visszatér a teljes sebességhez a szokásos beállításokkal.
- Az Alacsony ping megtartása és a foglalt lemez miatti lassítás bármelyik órában csökkentheti a sebességeket; soha nem növelik őket.

## Alternatív sebességkorlátok (a rookparcel-nox webes felülete)

A Windows- és a GTK-ablakban nincsenek alternatív sebességkorlátok; használja helyette az ütemező Korlátozva óráit.

A [rookparcel-nox](A-rookparcel-nox-futtatása) webes felületén vannak, a **Tulajdonságok > Sávszélesség > Alternatív sebességkorlátok** alatt:

| Beállítás | Jelentése | Alapértelmezés |
|---|---|---|
| **Alternatív feltöltési sebesség (kB/s): [0: korlátlan]** | Az általános feltöltési korlát, amíg az alternatív korlátok be vannak kapcsolva. | 10 |
| **Alternatív letöltési sebesség (kB/s): [0: korlátlan]** | Az általános letöltési korlát, amíg be vannak kapcsolva. | 10 |
| **Az alternatív korlátok használata most** | Az alternatív korlátokra vált, amíg vissza nem váltja. | Ki |

Azok az alkalmazások, amelyek a qBittorrent sebességkorlát-módját a webes felületen át kapcsolják (lásd [Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr)), ugyanezt kapcsolják. Az ütemező Korlátozva órái mindkét készlet fölött továbbra is érvényesek, amíg tartanak.

Ha egy asztali ablak szolgálja ki a webes felületet (Tulajdonságok > Webes felület az ablakban), ez a csoport nem látszik, és az alternatív korlátok nem kapcsolhatók be: az ablak saját korlátai és ütemezője érvényes.

## A GTK-ablakban és a webes felületen

- **GTK-ablak** (Linux, BSD): ugyanazok a Tulajdonságok oldalak, ugyanaz a **Sávszélesség** almenü a torrentmenüben, ugyanaz az ütemezési táblázat, ugyanúgy szerkesztve.
- **macOS-alkalmazás**: ugyanazok az oldalak, ugyanaz a **Sávszélesség** almenü, és egy húzással festhető ütemezési táblázat. A beállítások azonnal érvénybe lépnek, **OK** nélkül: lásd [A beállítások leírása](A-beállítások-leírása#különbségek-macos-en).
- **Webes felület**: ugyanazok az oldalak a **Tulajdonságok** alatt, és ugyanaz a **Sávszélesség** almenü. Az állapotsora a sebesség után zárójelben mutatja az általános korlátot, amíg be van állítva, az automatikus korlátot pedig ugyanúgy, mint az ablak.
- **rookparcel-nox** webes felület nélkül: a `--dl-limit KIB` és a `--ul-limit KIB` indításkor beállítja az általános korlátokat (0 = korlátlan). Az ütemezőt, a megosztási célt és az alternatív korlátokat a webes felület **Tulajdonságok** oldalán állíthatja be.

## Kapcsolódó oldalak

- [Alacsony ping](Alacsony-ping)
- [Várólista és seedelés](Várólista-és-seedelés)
- [Lemez és tárolás](Lemez-és-tárolás#ha-a-lemez-foglalt)
- [A beállítások leírása](A-beállítások-leírása)
