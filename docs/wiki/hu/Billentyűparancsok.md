*Ez a [Keyboard shortcuts](Keyboard-shortcuts) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Billentyűparancsok

Ez az oldal a Rookparcel Windowsos változatának billentyűparancsait sorolja fel, majd a GTK-s változatét, a macOS-es alkalmazásét és a webes felületét. A legtöbbjük a menüpontok mellett is látható.

## A főablak (Windows)

| Billentyűk | Mit csinálnak |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>O</kbd> | **Fájl > Torrent hozzáadása...** |
| <kbd>Ctrl</kbd>+<kbd>U</kbd> | **Fájl > Torrent hozzáadása URL-ről...** (a vágólapon lévő magnet hivatkozás vagy webcím előre ki van töltve) |
| <kbd>Ctrl</kbd>+<kbd>N</kbd> | **Fájl > Új torrent létrehozása...** |
| <kbd>Ctrl</kbd>+<kbd>P</kbd> | **Beállítások > Tulajdonságok** |
| <kbd>Ctrl</kbd>+<kbd>G</kbd> | **Beállítások > Beállítási útmutató** |
| <kbd>Ctrl</kbd>+<kbd>L</kbd> | **Fájl > Rookparcel zárolása**: elrejti a Rookparcel minden ablakát, amíg be nem írják a zárolási PIN-kódot (lásd [A Rookparcel zárolása](A-Rookparcel-zárolása)) |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> | Az eszköztár keresőmezőjére lép (ha az eszköztár látszik). A Fájlok lapon ehelyett a lap szűrőmezőjére lép. |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Kijelöli a lista összes torrentjét. A Fájlok lapon azokat a fájlokat jelöli ki, amelyeket a szűrő mutat. |
| <kbd>Ctrl</kbd>+<kbd>S</kbd> | A kijelölt torrentek **Indítás**a |
| <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd> | A kijelölt torrentek **Szünet**e |
| <kbd>Ctrl</kbd>+<kbd>T</kbd> | A kijelölt torrentek **Leállítás**a |
| <kbd>Del</kbd> | A kijelölt torrentek **Eltávolítás**a |
| <kbd>Shift</kbd>+<kbd>Del</kbd> | **Eltávolítás és > Adatok törlése**: eltávolítja a torrenteket, és törli a fájljaikat |
| <kbd>Enter</kbd> | A kijelölt torrentek **Jellemzők** ablaka (a torrentlistában) |
| <kbd>Shift</kbd>+<kbd>F10</kbd> vagy a menübillentyű | Megnyitja a jobb gombos menüt a fókuszban lévő sor alatt |
| <kbd>Alt</kbd>+<kbd>F4</kbd> | Bezárja az ablakot. Ha a **Bezáráskor a tálcára** be van kapcsolva (alapértelmezés), a Rookparcel tovább fut a tálcán. |

Ha a Tulajdonságok ablakban be van kapcsolva a **Torrentek eltávolításának megerősítése**, a <kbd>Del</kbd> és a <kbd>Shift</kbd>+<kbd>Del</kbd> előbb rákérdez. A **Fájl > Kilépés** menüponthoz nincs billentyűparancs; lásd [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás).

*A <kbd>Ctrl</kbd>+<kbd>G</kbd> és a Beállítási útmutató az 1.1.62-ben, a Fájlok lap szűrőmezője az 1.1.56-ban, a <kbd>Ctrl</kbd>+<kbd>L</kbd> pedig az 1.2.54-ben jelent meg.*

Hozzájuk tartozó egérparancsok:

- Ha egy befejezett torrentre duplán kattint, megnyílik a mappája; ha egy még be nem fejezettre, megnyílik a **Jellemzők** ablaka.
- A Fájlok lapon egy fájlra duplán kattintva, vagy rajta az <kbd>Enter</kbd> billentyűt megnyomva megnyílik a fájl.

A Fájlok lap mappafájában a <kbd>Right</kbd> vagy a <kbd>+</kbd> kinyitja a fókuszban lévő mappát, a <kbd>Left</kbd> vagy a <kbd>-</kbd> becsukja; fájlon vagy zárt mappán a <kbd>Left</kbd> a tartalmazó mappára ugrik. A torrentlistában és a részletek lapjain egy név első betűit begépelve a következő olyan sor lesz kijelölve, amelynek neve ezekkel kezdődik, mint a Windows Intézőben; egy másodperc szünet után újra kezdődik a keresés. *A mappabillentyűk és a begépelés az 1.2.21-ben jelentek meg.*

## A keresőmező és a Fájlok lap szűrője

| Billentyűk | Az eszköztár keresőmezőjében | A Fájlok lap szűrőmezőjében |
|---|---|---|
| <kbd>Escape</kbd> | Kiüríti a mezőt, és a torrentlistára lép | Kiüríti a mezőt, és a fájllistára lép |
| <kbd>Enter</kbd> | Ha a **Torrentek szűrése** van kiválasztva, a listára lép. Ha egy keresési forrás van kiválasztva, abban keres. | A fájllistára lép |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Kijelöli a mező szövegét | Kijelöli a mező szövegét (ezt teszi a <kbd>Ctrl</kbd>+<kbd>F</kbd> is) |
| <kbd>Alt</kbd>+<kbd>Down</kbd> | Megnyitja a mező legördülő menüjét (források, **Források kezelése...**) | |

Amíg valamelyik mezőbe gépel, a <kbd>Del</kbd> és a <kbd>Ctrl</kbd>+<kbd>A</kbd> a szöveget szerkeszti, nem a listát. Lásd [Torrentek keresése](Torrentek-keresése) és [Torrent részletek és fájlok](Torrent-részletek-és-fájlok).

## Az Új torrent hozzáadása ablak

Az Új torrent hozzáadása ablak fájllistájában:

| Billentyűk | Mit csinálnak |
|---|---|
| <kbd>Space</kbd> | Bejelöli vagy törli a kijelölt fájlok és mappák jelölését |
| <kbd>Right</kbd> vagy <kbd>+</kbd> | Kinyitja a fókuszban lévő mappát |
| <kbd>Left</kbd> vagy <kbd>-</kbd> | Becsukja a fókuszban lévő mappát; fájlon vagy zárt mappán a <kbd>Left</kbd> a szülőmappára ugrik |
| <kbd>Enter</kbd> | Kinyitja vagy becsukja a fókuszban lévő mappát |
| Betűk | Kijelölik a következő olyan sort, amelynek neve ezekkel kezdődik |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> | A lista fölötti **Fájlok szűrése** mezőre lép, a szövegét kijelölve |

A szűrőmezőben az <kbd>Enter</kbd> a listára lép, az <kbd>Escape</kbd> pedig kiüríti a mezőt; üres mező esetén az <kbd>Escape</kbd> bezárja az ablakot, mint a **Mégse**. *A szűrőmező és a begépelés az 1.2.21-ben jelent meg.*

Lásd [Torrentek hozzáadása](Torrentek-hozzáadása).

## A Tulajdonságok és a többi ablak

| Billentyűk | Mit csinálnak |
|---|---|
| <kbd>F1</kbd> a Tulajdonságok ablakban | Megmutatja a fókuszban lévő beállítás súgóját mellette, és az oldalak alatti súgópanelen |
| <kbd>Escape</kbd> | Bezár egy párbeszédablakot, mint a **Mégse**. A Beállítási útmutatóban ez a **Kihagyás**. |
| <kbd>Tab</kbd>, <kbd>Enter</kbd>, <kbd>Escape</kbd> a Torrentek keresése ablakban | A vezérlők között lépnek, a fókuszban lévőt működtetik, bezárják az ablakot |

A Tulajdonságok ablak súgója szövegbuborékként is megjelenik, ha egy beállításra mutat. A főablakban a menüpontot vagy eszköztárgombot, amelyre mutat, az állapotsor írja le. *Ezek a súgószövegek az 1.1.57-ben jelentek meg.*

Lásd [A beállítások leírása](A-beállítások-leírása).

## Menübillentyűk

Minden menü elérhető az <kbd>Alt</kbd> és az aláhúzott betű együttes megnyomásával, mint más Windowsos programokban: <kbd>Alt</kbd>+<kbd>F</kbd> a **Fájl**, <kbd>Alt</kbd>+<kbd>O</kbd> a **Beállítások**, <kbd>Alt</kbd>+<kbd>H</kbd> a **Súgó** menühöz, utána a menüpont aláhúzott betűje.

## GTK-s változat (Linux, BSD)

A GTK-s változat billentyűparancsai:

| Billentyűk | Mit csinálnak |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>O</kbd> | **Torrent hozzáadása...** |
| <kbd>Ctrl</kbd>+<kbd>U</kbd> | **Torrent hozzáadása URL-ről...** |
| <kbd>Ctrl</kbd>+<kbd>N</kbd> | **Új torrent létrehozása...** |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> | A keresőmezőre lép |
| <kbd>Ctrl</kbd>+<kbd>P</kbd> | **Tulajdonságok** |
| <kbd>Ctrl</kbd>+<kbd>G</kbd> | **Beállítási útmutató** |
| <kbd>Ctrl</kbd>+<kbd>L</kbd> | **Rookparcel zárolása**. A Rookparcel minden ablakában működik (Tulajdonságok, Jellemzők, Új torrent hozzáadása és a többi), nem csak a főablakban. *Az 1.2.76-ban jelent meg; korábban nem csinált semmit, amíg egy párbeszédablak nyitva volt.* |
| <kbd>Ctrl</kbd>+<kbd>Q</kbd> | **Kilépés**: kilép a Rookparcelből akkor is, ha az ablak bezáráskor a tálcára menne |
| <kbd>Ctrl</kbd>+<kbd>S</kbd> | A kijelölt torrentek **Indítás**a |
| <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd> | A kijelölt torrentek **Szünet**e |
| <kbd>Ctrl</kbd>+<kbd>T</kbd> | A kijelölt torrentek **Leállítás**a |
| <kbd>Delete</kbd> | **Eltávolítás** |
| <kbd>Shift</kbd>+<kbd>Delete</kbd> | Eltávolítás és az adatok törlése |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Kijelöli az összes torrentet; a Fájlok lapon azokat a fájlokat, amelyeket a szűrője mutat |
| <kbd>Enter</kbd> a torrentlistában | **Jellemzők** |
| <kbd>F1</kbd> a Tulajdonságok ablakban | Megmutatja a fókuszban lévő beállítás súgóját |
| <kbd>Escape</kbd> | Kiüríti a keresőmezőt vagy a Fájlok lap szűrőjét; bezárja a párbeszédablakokat |

Ami eltér a Windowstól:

- Az Indítás, a Szünet, a Leállítás, a Jellemzők és az Eltávolítás ugyanazokkal a billentyűkkel működik, mint Windowson.
- A <kbd>Ctrl</kbd>+<kbd>F</kbd> a Fájlok lap szűrőmezőjére lép, ha a fókusz a Fájlok lapon van, mint Windowson.
- A Fájlok lap mappafájában a <kbd>Right</kbd> és a <kbd>Left</kbd> úgy nyitja és csukja a mappákat, mint Windowson.
- A <kbd>Ctrl</kbd>+<kbd>Q</kbd> kilép; Windowson nincs kilépési billentyűparancs.
- Az Új torrent hozzáadása ablak billentyűi ugyanazok, mint Windowson.

## macOS

A macOS-es alkalmazás ott használja a <kbd>⌘</kbd> (Command) billentyűt, ahol a Windows a <kbd>Ctrl</kbd>-t, és a Mac saját billentyűit, ahol azok eltérnek. A billentyűparancsok a menüsor menüpontjai mellett vannak feltüntetve.

| Billentyűk | Mit csinálnak |
|---|---|
| <kbd>⌘</kbd>+<kbd>,</kbd> | **Rookparcel > Tulajdonságok** |
| <kbd>⌘</kbd>+<kbd>G</kbd> | **Rookparcel > Beállítási útmutató** |
| <kbd>⌘</kbd>+<kbd>H</kbd> | Elrejti a Rookparcelt (az <kbd>⌥</kbd>+<kbd>⌘</kbd>+<kbd>H</kbd> a többi alkalmazást rejti el) |
| <kbd>⌘</kbd>+<kbd>Q</kbd> | **Kilépés a Rookparcel alkalmazásból**, akkor is, ha az ablak bezáráskor a menüsorba menne |
| <kbd>⌘</kbd>+<kbd>O</kbd> | **Fájl > Torrent hozzáadása...** |
| <kbd>⌘</kbd>+<kbd>U</kbd> | **Fájl > Torrent hozzáadása URL-ről...** |
| <kbd>⌘</kbd>+<kbd>N</kbd> | **Fájl > Új torrent létrehozása...** |
| <kbd>⌘</kbd>+<kbd>L</kbd> | **Fájl > Rookparcel zárolása** |
| <kbd>⌘</kbd>+<kbd>W</kbd> | **Fájl > Ablak bezárása**: ha a **Bezáráskor a tálcára** be van kapcsolva, a Rookparcel tovább fut a menüsorban |
| <kbd>⌘</kbd>+<kbd>A</kbd> | **Szerkesztés > Összes kijelölése**: a lista összes torrentje, vagy egy mező szövege |
| <kbd>⌘</kbd>+<kbd>F</kbd> | **Szerkesztés > Keresés...**: az eszköztár keresőmezőjére lép; a Fájlok lapon annak szűrőmezőjére |
| <kbd>⌥</kbd>+<kbd>⌘</kbd>+<kbd>T</kbd> | **Nézet > Eszköztár megjelenítése** |
| <kbd>⌃</kbd>+<kbd>⌘</kbd>+<kbd>S</kbd> | **Nézet > Oldalsáv megjelenítése** (a kategórialista) |
| <kbd>⌃</kbd>+<kbd>⌘</kbd>+<kbd>F</kbd> | **Nézet > Belépés teljes képernyős módba** |
| <kbd>⌘</kbd>+<kbd>S</kbd> | **Torrent > Indítás** |
| <kbd>⇧</kbd>+<kbd>⌘</kbd>+<kbd>P</kbd> | **Torrent > Szünet** |
| <kbd>⌘</kbd>+<kbd>T</kbd> | **Torrent > Leállítás** |
| <kbd>⌘</kbd>+<kbd>⌫</kbd>, vagy a <kbd>⌫</kbd> a listában | **Torrent > Eltávolítás** |
| <kbd>⌥</kbd>+<kbd>⌘</kbd>+<kbd>⌫</kbd> | **Torrent > Eltávolítás és az adatok törlése**: eltávolítja a torrenteket, és törli a fájljaikat |
| <kbd>⌘</kbd>+<kbd>I</kbd>, vagy a <kbd>Return</kbd> a listában | **Torrent > Jellemzők** |
| <kbd>⌘</kbd>+<kbd>M</kbd> | **Ablak > Kis méret** |

Ami eltér a Windowstól:

- A Tulajdonságok a <kbd>⌘</kbd>+<kbd>,</kbd>, mint más Mac-alkalmazásokban, nem a <kbd>Ctrl</kbd>+<kbd>P</kbd>.
- Az Eltávolítás a <kbd>⌘</kbd>+<kbd>⌫</kbd> (Delete), a Jellemzők pedig a <kbd>⌘</kbd>+<kbd>I</kbd>.
- A torrentlista jobb gombos menüjének ugyanazok a pontjai, mint a **Torrent** menüének; a Control-kattintás is megnyitja.
- A **Szerkesztés** menüben ott van a szövegmezők szokásos Visszavonás, Kivágás, Másolás és Beillesztés parancsa.

*A natív macOS-es alkalmazás az 1.2.19-ben jelent meg.*

## Webes felület

A webes felület torrentlistájában (előbb kattintson a listára, hogy övé legyen a billentyűzet):

| Billentyűk | Mit csinálnak |
|---|---|
| <kbd>Up</kbd>, <kbd>Down</kbd>, <kbd>Page Up</kbd>, <kbd>Page Down</kbd>, <kbd>Home</kbd>, <kbd>End</kbd> | Mozgatják a kijelölést |
| <kbd>Shift</kbd> ezekkel a billentyűkkel | Kiterjeszti a kijelölést |
| <kbd>Ctrl</kbd> ezekkel a billentyűkkel | A fókuszt mozgatja a kijelölés megváltoztatása nélkül |
| <kbd>Ctrl</kbd>+<kbd>Space</kbd> | Hozzáadja a fókuszban lévő sort a kijelöléshez, vagy kiveszi belőle |
| <kbd>Space</kbd> | A kijelölt torrentek **Szünet**e |
| <kbd>Delete</kbd> | **Eltávolítás** |
| <kbd>Shift</kbd>+<kbd>Delete</kbd> | Eltávolítás és az adatok törlése |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Kijelöli az összes torrentet |
| <kbd>Shift</kbd>+<kbd>F10</kbd> vagy a menübillentyű | Megnyitja a jobb gombos menüt |
| <kbd>Escape</kbd> | Bezár egy nyitott menüt |

A webes felület más részein:

| Billentyűk | Mit csinálnak |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>P</kbd> | Megnyitja a **Tulajdonságok** ablakot (nem, ha a Hozzáadás vagy a Jellemzők ablak nyitva van) |
| <kbd>Escape</kbd> a szűrőmezőben | Kiüríti, és a listára lép |
| <kbd>Enter</kbd> a szűrőmezőben | A Hozzáadás ablakban: a listára lép |
| <kbd>F1</kbd> a Tulajdonságok ablakban | Megmutatja a fókuszban lévő beállítás súgóját |
| <kbd>Ctrl</kbd>+<kbd>Enter</kbd> a Tulajdonságok ablakban | **OK** |
| <kbd>Enter</kbd> a Haladó oldal értékmezőjében | Beállítja az értéket |
| <kbd>Escape</kbd> a Tulajdonságok ablakban | Bezárja, mint a **Mégse** |
| <kbd>Up</kbd>, <kbd>Down</kbd>, <kbd>Home</kbd>, <kbd>End</kbd> a Tulajdonságok oldalainak listáján | Az oldalak között lépnek |
| <kbd>Ctrl</kbd>+<kbd>Enter</kbd> a Torrent jellemzői ablakban | **OK** (az <kbd>Enter</kbd> önmagában új sort kezd a trackerek mezőjében) |

A Fájlok lapon és a Hozzáadás ablak fájllistájában:

| Billentyűk | Mit csinálnak |
|---|---|
| <kbd>Up</kbd>, <kbd>Down</kbd>, <kbd>Home</kbd>, <kbd>End</kbd> | A fókuszt mozgatják (a <kbd>Shift</kbd>-tel kiterjesztik a kijelölést) |
| <kbd>Right</kbd>, <kbd>Left</kbd> | A mappafában: kinyitnak vagy becsuknak egy mappát; fájlon vagy zárt mappán a <kbd>Left</kbd> a szülőmappára ugrik, nyitott mappán a <kbd>Right</kbd> az első bejegyzésére |
| <kbd>Space</kbd> | A Hozzáadás ablakban: bejelöli vagy törli a kijelölt fájlok és mappák jelölését |
| Betűk | Kijelölik a következő olyan sort, amelynek neve ezekkel kezdődik |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> | A szűrőmezőre lép |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | A Fájlok lapon: kijelöli a látható fájlokat |
| <kbd>Escape</kbd> a szűrőmezőben | Kiüríti (a Hozzáadás ablakban bezárja az ablakot, ha már üres) |
| <kbd>Enter</kbd> a szűrőmezőben | A listára lép |

Macen a <kbd>Command</kbd> működik mindenütt, ahol a webes felület <kbd>Ctrl</kbd>-t említ. A böngésző saját billentyűparancsai egyes billentyűket előbb elvihetnek. Lásd [Webes felület](Webes-felület).
