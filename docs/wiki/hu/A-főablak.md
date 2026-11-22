*Ez az angol nyelvű [The main window](The-main-window) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# A főablak

Ez az oldal a Rookparcel főablakát mutatja be: a menüket, az eszköztárat és a keresőmezőjét, a kategórialistát, a torrentlistát és oszlopait, a részletlapokat és az állapotsort. Előbb a Windows-os változatot írja le, majd azt, miben tér el a GTK-s változat és a webes felület.

![A főablak torrentekkel és az Általános lappal](images/main-window.png)

## A részek áttekintése

| Rész | Hol | Mire való |
|---|---|---|
| Menüsor | felül | **Fájl**, **Beállítások** és **Súgó** |
| Eszköztár | a menük alatt | gombok a gyakori műveletekhez, jobbra a keresőmezővel |
| Kategórialista | balra | a torrenteket állapot, címke vagy jelölő szerint mutatja, valamint az RSS-csatornákat |
| Torrentlista | középen | torrentenként egy sor |
| Részletlapok | a lista alatt | a kijelölt torrent részletei: Általános, Trackerek, Peerek, Részek, Fájlok, Sebesség, Napló |
| Állapotsor | alul | a hálózat állapota, a DHT és az összesített sebességek |

A menüsor és a torrentlista kivételével minden rész elrejthető a **Beállítások** menüből: **Eszköztár megjelenítése**, **Részletek megjelenítése**, **Állapotsor megjelenítése** és **Kategórialista megjelenítése**. A kategórialista, a torrentlista és a részletlapok közötti határokat húzva átméretezheti őket. A Rookparcel megjegyzi ezeket a választásokat, a méreteket, az oszlopokat, a rendezési sorrendet és a megnyitott lapot.

## A menük

- **Fájl:** **Torrent hozzáadása...**, **Torrent hozzáadása URL-ről...**, **Új torrent létrehozása...**, **Importálás a qBittorrentből...**, **Importálás a µTorrentből...**, **Profilok**, **Csatlakozás távoli géphez...**, **Rookparcel zárolása** (<kbd>Ctrl</kbd>+<kbd>L</kbd>; lásd: [A Rookparcel zárolása](A-Rookparcel-zárolása)) és **Kilépés**.
- **Beállítások:** **Tulajdonságok**, **Beállítási útmutató**, **RSS letöltő...**, a fenti négy **megjelenítés** elem, **Automatikus leállítás**, **Bezáráskor a tálcára** és **Kis méretnél a tálcára**.
- **Súgó:** **A Rookparcel webhelye**, **Frissítések keresése**, **Statisztika**, **Adományozás**, **A Rookparcel névjegye** és **Harmadik felek közleményei**. A **Statisztika** (új az 1.2.39-ben) megmutatja, mennyit töltött fel és le a Rookparcel az összes munkamenetében és ebben, a megosztási arányt, az eldobott adatokat, a csatlakozott peereket, valamint a lemezsort és az olvasási gyorsítótárat; az összesített számok a torrentek eltávolításakor megmaradnak. Az **Adományozás** (új az 1.2.34-ben) egy rövid megjegyzést és egy gombot mutat, amely a webhely adományozási oldalát nyitja meg a böngészőben.

*A Fájl > Profilok az 1.1.48-ban jelent meg, a Beállítások > Beállítási útmutató az 1.1.62-ben.*

Oldalak: [A Rookparcel zárolása](A-Rookparcel-zárolása), [Torrentek hozzáadása](Torrentek-hozzáadása), [Torrentek készítése](Torrentek-készítése), [Importálás µTorrentből és qBittorrentből](Importálás-uTorrentből-és-qBittorrentből), [Profilok](Profilok), [Távvezérlés](Távvezérlés), [A beállítások leírása](A-beállítások-leírása), [Első indítás és a beállítási útmutató](Első-indítás), [RSS letöltő](RSS-letöltő), [Automatikus leállítás](Automatikus-leállítás), [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás), [A Rookparcel frissítése](A-Rookparcel-frissítése).

## Az eszköztár

Balról jobbra:

| Gomb | Mit csinál |
|---|---|
| **Torrent hozzáadása** | `.torrent` fájlokat ad hozzá (<kbd>Ctrl</kbd>+<kbd>O</kbd>) |
| **Torrent hozzáadása URL-ről** | magnet hivatkozást vagy webcímet ad hozzá (<kbd>Ctrl</kbd>+<kbd>U</kbd>) |
| **Új torrent létrehozása** | új `.torrent` fájlt készít (<kbd>Ctrl</kbd>+<kbd>N</kbd>) |
| **Eltávolítás** | eltávolítja a kijelölt torrenteket a listából (<kbd>Del</kbd>) |
| **Indítás**, **Szünet**, **Leállítás** | elindítja, szünetelteti vagy leállítja a kijelölt torrenteket |
| **Feljebb a sorban**, **Lejjebb a sorban** | a kijelölt torrenteket egy hellyel mozgatja a sorban |
| **RSS letöltő** | megnyitja az RSS letöltőt |
| **Tulajdonságok** | megnyitja a Tulajdonságok ablakot (<kbd>Ctrl</kbd>+<kbd>P</kbd>) |

Ha az egeret egy gomb fölé viszi, megjelenik a neve. Az 1.1.57-től az állapotsor azt is egy sorban kiírja, mit csinál a mutató alatti gomb vagy a kiemelt menüelem. Azt, hogy az Indítás, a Szünet és a Leállítás mit jelent, a [Torrentek kezelése](Torrentek-kezelése#indítás-szünet-leállítás-és-kényszerített-indítás) oldal írja le.

### A keresőmező

Az eszköztár jobb végén lévő mező két feladatot lát el, amelyet a mellette lévő kis nyíllal választhat ki:

- **Torrentek szűrése** (a szokásos mód): gépelés közben a lista csak azokat a torrenteket mutatja, amelyek neve tartalmazza a szöveget, a kis- és nagybetűt figyelmen kívül hagyva. A szűrő a bal oldalon kiválasztott kategória tetején működik. Az <kbd>Esc</kbd> kiüríti a mezőt, és visszatér a listához. A <kbd>Ctrl</kbd>+<kbd>F</kbd> a kurzort a mezőbe teszi.
- **Egy keresési forrás**: az <kbd>Enter</kbd> a weben keres torrenteket a Prowlarron, a Jacketten vagy a keresősegéden át. Lásd: [Torrentek keresése](Torrentek-keresése).

A választás az indítások között megmarad.

## A kategórialista

A bal oldali lista szűkíti a torrentlistát. Minden bejegyzés mutatja, hány torrentet tartalmaz.

| Bejegyzés | Mit mutat |
|---|---|
| **Torrentek** | minden torrent |
| **Letöltés alatt** | a még nem befejezett torrentek |
| **Kész** | a befejezett torrentek |
| **Aktív** | a most adatot letöltő vagy feltöltő torrentek |
| **Inaktív** | a most adatot nem mozgató torrentek |
| **Címkék** > **Címke nélkül** | a címke nélküli torrentek |
| **Címkék** > *egy címke* | az adott címkéjű torrentek; használt címkénként egy bejegyzés |
| **Jelölők** > **Jelölő nélkül** | a jelölő nélküli torrentek |
| **Jelölők** > *egy jelölő* | az adott jelölővel rendelkező torrentek; jelölőnként egy bejegyzés, olyan jelölőnél is, amellyel egy torrent sem rendelkezik (0) |
| **Csatornák** | az RSS-csatornái; egyet kiválasztva az elemei jelennek meg a listában |

A címkéket a torrentlista jobb gombos menüjéből hozhatja létre és adhatja meg. Lásd: [Torrentek kezelése](Torrentek-kezelése#címkék). A jelölők ugyanígy adhatók meg; a **Jelölők** ágon jobb gombbal az **Új jelölő...**, egy jelölőn pedig a **Jelölő törlése** érhető el. Lásd: [Torrentek kezelése](Torrentek-kezelése#jelölők) (*a jelölők az 1.2.63-ban jelentek meg*). A Csatornák ágnak saját jobb gombos menüje van; lásd: [RSS letöltő](RSS-letöltő).

## A torrentlista

Torrentenként egy sor. Kattintson egy sorra a kijelöléséhez; a <kbd>Ctrl</kbd>+kattintás és a <kbd>Shift</kbd>+kattintás többet jelöl ki; a <kbd>Ctrl</kbd>+<kbd>A</kbd> mindent kijelöl. Jobb gombbal kattintva megjelenik a torrent menü, amelyet a [Torrentek kezelése](Torrentek-kezelése#a-jobb-gombos-menü) oldal ír le.

Egy befejezett torrentre duplán kattintva megnyílik a mappája az Intézőben. Egy nem befejezett torrentre duplán kattintva, vagy az <kbd>Enter</kbd> megnyomásával megnyílik a [Jellemzői](Torrent-részletek-és-fájlok#torrent-jellemzői) ablak.

### Oszlopok

Az oszlopfejlécen jobb gombbal kattintva oszlopokat jeleníthet meg vagy rejthet el. A **Név** mindig látható. Egy fejlécet húzva az oszlop áthelyezhető; a szélét húzva a szélessége módosítható.

| Oszlop | Mit mutat | Kezdetben látható |
|---|---|---|
| **Név** | a torrent neve | igen |
| **#** | a helye a sorban; `*`, ha nincs | igen |
| **Méret** | a letöltésre kiválasztott fájlok mérete | igen |
| **Kész** | mennyi töltődött le, százalékban | igen |
| **Állapot** | mit csinál a torrent (lásd alább) | igen |
| **Seedek** | a csatlakozott seedek, és zárójelben a seedek az egész rajban, ha a tracker jelenti | igen |
| **Peerek** | a csatlakozott peerek, és zárójelben a peerek a rajban | igen |
| **Letöltés**, **Feltöltés** | az aktuális sebességek | igen |
| **Hátralévő idő** | a becsült hátralévő idő | igen |
| **Feltöltve** | az összes feltöltött | igen |
| **Letöltve** | az összes letöltött | nem |
| **Arány** | a feltöltött osztva a letöltöttel | igen |
| **Elérh.** | hány teljes példányt tartanak együtt a csatlakozott peerek | igen |
| **Címke** | a címkéje | igen |
| **Hozzáadva**, **Befejezve** | dátumok | nem |
| **Tracker** | a használt tracker gépneve | nem |
| **Letöltési korlát**, **Feltöltési korlát** | a saját sebességkorlátai | nem |
| **Hátralévő** | ami még letöltésre vár | nem |
| **Seedek/Peerek** | seedek peerenként a rajban | nem |
| **Mentés mint** | a mappa, amelyben a fájljai vannak | nem |
| **Elvetett** | az eldobott adatok (sikertelen ellenőrzések, duplikátumok) | nem |
| **Jelölők** | a jelölői, vesszővel elválasztva (*új az 1.2.63-ban*) | nem |

### Rendezés

Egy oszlopfejlécre kattintva e szerint rendezhet. Az első kattintás úgy rendez, ahogy általában a leghasznosabb: a sebességek, a seedek, a peerek, az arány és a kész százalék nagytól kicsi felé, a többi kicsitől nagy felé. Ismét kattintva megfordítja. A neveket az Intéző módján rendezi, így az „Episode 2” az „Episode 10” előtt áll.

### Mit mond az Állapot oszlop

| Állapot | Jelentése |
|---|---|
| **Letöltés**, **Megosztás** | fut |
| **Sorban áll**, **Megosztás sorban** | elindítva, de szabad helyre vár a sorban |
| **[F] Letöltés**, **[F] Megosztás** | **Kényszerített indítással** elindítva, a soron kívül |
| **[S] Megosztás** | megosztás bekapcsolt **Kezdeti megosztással** |
| **Szüneteltetve** | Ön szüneteltette |
| **Leállítva** | Ön leállította, mielőtt befejeződött volna |
| **Kész** | a befejezése után leállítva |
| **Metaadatok letöltése** | egy magnet hivatkozás, amely a torrent fájllistáját tölti le |
| **Ellenőrzés 45,3 %**, **Ellenőrzésre vár** | a lemezen lévő fájlokat ellenőrzi. Az **Ellenőrzésre vár** azt jelenti, hogy az ellenőrzés a meghajtójára vár, amely egyszerre egy torrentet ellenőriz. *Az Ellenőrzésre vár az 1.2.76-ban jelent meg.* |
| **Áthelyezés** | a fájljait egy másik mappába helyezi át |
| **Várakozás a hálózatra** | a **Csak ennek a hálózatnak a használata** nem talál megfelelő hálózati adaptert (a VPN nem működik); magától folytatódik, amikor az visszatér. *Új az 1.1.58-ban.* |
| **Hiba: ...** | valami elromlott; az üzenet megmondja, mi. Ha a torrent fájljait tartalmazó meghajtó, megosztás vagy mappa eltűnt, az üzenet „files missing” („fájlok hiányoznak”) szöveget tartalmaz, és a torrent vár, amíg a fájlok vissza nem kerülnek (*új az 1.2.76-ban*; lásd: [Lemez és tárolás](Lemez-és-tárolás#ha-egy-meghajtó-vagy-megosztás-hiányzik)). |

Az állapotok mögötti műveletekről lásd: [Torrentek kezelése](Torrentek-kezelése).

## A részletlapok

A lista alatti lapok a kijelölt torrentet mutatják: **Általános**, **Trackerek**, **Peerek**, **Részek** és **Fájlok**. A **Sebesség** és a **Napló** az egész programra vonatkozik. Az egyes lapokat a [Torrent részletek és fájlok](Torrent-részletek-és-fájlok) oldal írja le.

## Az állapotsor

Balról jobbra:

| Rész | Mit mutat |
|---|---|
| Frissítési értesítés | „Elérhető a Rookparcel ... - kattintson ide a letöltéshez”, csak ha megjelent egy újabb verzió. Lásd: [A Rookparcel frissítése](A-Rookparcel-frissítése). |
| Hálózat | **Hálózat rendben**, ha más peerek tudnak csatlakozni Önhöz, **Nincs bejövő kapcsolat**, ha még egy sem tudott. Ha a felirat **A(z) ... port foglalt**, az azt jelenti, hogy egy másik program, gyakran egy másik Rookparcel-profil tartja a figyelő portot; kattintson rá a Tulajdonságok > Kapcsolat megnyitásához. Ha az ütemező korlátozza a sebességet, utána megjelenik az állapota, például „Ütemező: korlátozva”. A **Torrentek visszaállítása...** az induláskor látszik, amíg a mentett torrentek vissza nem kerülnek a listába. Az **Átvitelek visszatartva: erősítse meg a beállításokat** azt jelenti, hogy a Rookparcel nem bízhatott a beolvasott beállításokban (egy frissítés után egy proxy-, hálózati szabály- vagy figyelő interfész-beállításban, vagy egy olvashatatlan beállításfájlban), ezért semmi sem csatlakozik; ellenőrizze a **Beállítások > Tulajdonságok** oldalt, és kattintson az **OK** gombra. Lásd: [A Rookparcel frissítése](A-Rookparcel-frissítése). |
| **DHT: ... csomópont** | hány DHT-csomópontot ismer a Rookparcel |
| **L: ... Ö: ...** | a letöltési sebesség, és az ebben a munkamenetben letöltött összes mennyiség |
| **F: ... Ö: ...** | a feltöltési sebesség, és az ebben a munkamenetben feltöltött összes mennyiség |

Amikor az **Alacsony ping megtartása** lent tartja a sebességeket, a sebességrészek hozzáadják az automatikus korlátot, például „L: 400 kB/s (auto 450 kB/s) Ö: 2,10 GB”.

*Új az 1.1.58-ban:* amíg a **Csak ennek a hálózatnak a használata** nem talál megfelelő adaptert, a hálózati rész „Várakozás a hálózatra” szöveget mutat, és fölé víve megjelenik a címe, ahogy a trackerek látják. Az „A(z) ... port foglalt” szöveg a profilokkal együtt jelent meg az 1.1.45-ben.

Lásd: [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal), [Sebességkorlátok és ütemező](Sebességkorlátok-és-ütemező) és [Alacsony ping](Alacsony-ping).

## A címsor és a tálca

Ha az **Aktuális sebesség megjelenítése a címsorban** be van kapcsolva (Tulajdonságok > Felület), az ablak címe a letöltési és feltöltési sebességekkel kezdődik, amelyek a tálcán is látszanak.

A Rookparcel ikont tesz az értesítési területre. Fölé víve megjelenik a verzió és a sebességek. Egy kattintás megmutatja vagy elrejti az ablakot; a jobb gombos kattintás menüt nyit, amelyben **Rookparcel elrejtése** vagy **Rookparcel megjelenítése**, **Torrent hozzáadása...**, **Torrent hozzáadása URL-ről...**, **Minden torrent indítása**, **Minden torrent leállítása**, **Rookparcel zárolása** (csak PIN-kód beállítása után) és **Kilépés** van. Alapból a bezárás gomb a tálcára rejti az ablakot, és a torrentek tovább futnak; a **Kilépés** kilép. Lásd: [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás).

## A GTK-s változatban (Linux, BSD)

- Ugyanaz az elrendezés: eszköztár, kategórialista, torrentlista, részletlapok és állapotsor. Az eszköztáron nincs **Tulajdonságok** gomb; használja a **Beállítások > Tulajdonságok** parancsot vagy a <kbd>Ctrl</kbd>+<kbd>P</kbd> billentyűkombinációt.
- A **Beállítások** menüben **Tulajdonságok**, **Beállítási útmutató**, **RSS letöltő...** és **Automatikus leállítás** van. Nincsenek benne **megjelenítés** elemek és tálcaelemek. A **Fájl** menüben a **Rookparcel zárolása** ugyanúgy megvan, mint Windowson.
- A **Fájl > Kilépés** a <kbd>Ctrl</kbd>+<kbd>Q</kbd> is.
- Az oszlopfejlécen jobb gombbal kattintva oszlopokat jeleníthet meg vagy rejthet el; a fejlécek húzással átrendezhetők.
- A dupla kattintás úgy működik, mint Windowson: egy befejezett torrent megnyitja a mappáját, egy másik a Jellemzőket.

## A webes felületen

- Az oldalon van eszköztár (**Torrent hozzáadása**, **URL hozzáadása**, **Eltávolítás**, **Indítás**, **Szünet**, **Leállítás**, **Fel**, **Le**, **RSS**, **Tulajdonságok** és **Keresés**), a kategórialista, a torrentlista és ugyanaz a hét részletlap, valamint egy keresőmező, amely név szerint szűri a listát.
- Egy fejlécre kattintva rendezhet, húzással átrendezheti, a szélét húzva átméretezheti. A fejléceken jobb gombbal kattintva oszlopokat jeleníthet meg vagy rejthet el; az **Oszlopok visszaállítása** visszatér az alapértelmezéshez.
- Telefonon a kategórialista a ☰ gombról nyílik meg, a ⋯ gombon pedig azok a műveletek vannak, amelyek nem férnek el, a **Rendezés** és a **Csökkenő rendezés**.

Lásd: [Webes felület](Webes-felület).

## Kapcsolódó oldalak

- [Billentyűparancsok](Billentyűparancsok)
- [Torrentek kezelése](Torrentek-kezelése)
- [Torrent részletek és fájlok](Torrent-részletek-és-fájlok)
