# Várólista és seedelés

*Ez a [Queueing and seeding](Queueing-and-seeding) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

Ez az oldal azt mutatja be, hogyan dönti el a Rookparcel, mely torrentek fussanak egyszerre (a várólista), mi történik, ha egy kész torrent eleget osztott (a megosztási cél), hogyan működik az új torrentek kezdeti megosztása, és hogyan helyezhetők át a kész letöltések másik mappába.

A beállítások a **Beállítások > Tulajdonságok > Sorba állítás** oldalon vannak.

![Tulajdonságok, Sorba állítás oldal](images/prefs-queueing.png)

## A várólista működése

Az elindított torrent a várólistára kerül. A várólista annyi torrentet futtat, amennyit a korlátai megengednek, a többi vár:

| Állapot | Jelentés |
|---|---|
| **Letöltés**, **Megosztás** | Fut, a várólista egyik helyén. |
| **Sorban áll** | Letöltési helyre vár. |
| **Megosztás sorban** | Kész, megosztási helyre vár. |
| **[F] Letöltés**, **[F] Megosztás** | A **Kényszerített indítás** paranccsal indítva, a várólistán kívül. |
| **Ellenőrzésre vár**, **Ellenőrzés** | Arra vár, hogy a meghajtójára kerüljön a sor az ellenőrzéshez, majd ellenőrzés alatt áll. Lásd: [Lemez és tárolás](Lemez-és-tárolás#ellenőrzés). |
| **Leállítva**, **Szüneteltetve**, **Kész** | Ön (a Kész állapotnál a megosztási cél) kivette a várólistából. |

- A **Start** a kijelölt torrenteket a várólistára adja.
- A **Kényszerített indítás** azonnal, a várólistán kívül futtatja őket, akkor is, ha a korlátok elérték a határt. A kényszerített torrentek sosem érik el a megosztási célt: addig osztanak, amíg le nem állítja őket.
- A **Leállítás** és a **Szünet** kiveszi a torrenteket a várólistából, amíg újra el nem indítja őket.
- A **Kényszerített ellenőrzés** szintén a várólistán kívül fut: *új az 1.2.76-ban.* Az újraellenőrzött torrent azonnal ellenőrzés alá kerül, akkor is, ha a várólista tele van, utána pedig újra a sorára vár. Leállított vagy szüneteltetett torrentet el is indít, amelyet aztán újra leállíthat.
- A jobb gombos menü **Feljebb a sorban**, **Lejjebb a sorban**, **A sor elejére** és **A sor végére** parancsa megváltoztatja a sorrendet. A **#** oszlop mutatja minden torrent helyét. A feljebb lévő torrentek előbb kapnak helyet.

E parancsokról és billentyűikről a [Torrentek kezelése](Torrentek-kezelése) oldalon olvashat. Új torrent a várólista elejére is felvehető az Új torrent hozzáadása ablak **A sor elejére** jelölőnégyzetével ([Torrentek hozzáadása](Torrentek-hozzáadása)).

## Sorbaállítási beállítások

A **Sorbaállítási beállítások** csoportban:

| Beállítás | Jelentés | Alapérték |
|---|---|---|
| **Aktív torrentek legnagyobb száma (feltöltés vagy letöltés): [-1: nincs korlát]** | Legfeljebb ennyi várólistás torrent fut egyszerre, a letöltések és a megosztások együtt. | 500 |
| **Aktív letöltések legnagyobb száma: [-1: nincs korlát]** | Legfeljebb ennyi várólistás torrent tölt le egyszerre. A következő akkor indul, amikor egy befejeződik vagy leáll. | 3 |
| **Aktív megosztások legnagyobb száma: [-1: nincs korlát]** | Legfeljebb ennyi kész, várólistás torrent oszt egyszerre. | 5 |
| **A lassú torrentek ne számítsanak bele ezekbe a korlátokba** | Az a torrent, amely egy percig körülbelül 2 kB/s-nál lassabban mozog, többé nem foglal letöltési vagy megosztási helyet, így egy másik elindulhat. Az összesített korlátba továbbra is beleszámít. | Be |

Ha mindent egyszerre akar futtatni, állítsa a korlátokat -1-re. Ha egyszerre csak egy torrentet akar letölteni, az **Aktív letöltések legnagyobb száma** értéke legyen 1.

A „lassú” küszöböt a Haladó oldal adja meg: `inactive_down_rate` és `inactive_up_rate` (bájt/másodperc, alapérték 2048), valamint `auto_manage_startup` (hány másodpercig kell a torrentnek ezek alatt maradnia, alapérték 60).

## Megosztási cél

A megosztási cél dönti el, mikor osztott eleget egy kész torrent. A **Megosztási cél** csoportban:

| Beállítás | Jelentés | Alapérték |
|---|---|---|
| **Legkisebb arány (%): [0: nincs arány cél]** | A torrent által letöltött mennyiség ekkora százalékát kell feltölteni. A 150 másfélszeres mennyiséget jelent. | 150 |
| **Legrövidebb megosztási idő (perc): [0: nincs idő cél]** | Legalább ennyi percig kell osztani. | 0 |

Ha mindkettő meg van adva, mindkettőt el kell érni. Ha mindkettő 0, nincs cél.

Ez a két szám önmagában nem csinál semmit. Kezdetben a torrentek az arányuktól függetlenül addig osztanak, amíg le nem állítja őket.

**Amikor egy torrent eléri a megosztási célt (a kényszerítettek soha):**

- A **Feltöltési sebesség korlátozása (kB/s): [0: leállítás]** léptet életbe a célt. Kezdetben ki van kapcsolva.
- Bejelölve, 0-val: a célt elért torrent leáll. Kikerül a várólistából, és **Kész** állapotot mutat.
- Bejelölve, sebességgel: a torrent lassan tovább oszt, a saját feltöltési korlátja erre a sebességre áll be. Ha a torrentnek már van ennél alacsonyabb korlátja, az megmarad.

A **Napló** lap kiírja a torrent nevét és azt, mit ért el, például „megosztási cél teljesült, leállítva (arány 1,523, cél 1,500)” vagy „megosztási cél teljesült, a feltöltés korlátozva: ...”.

Csak a várólistás torrentek kerülnek ellenőrzésre: a kényszerítetten indított, a leállított, az ellenőrzés alatt álló és a még letöltő torrenteket békén hagyja.

A cél egyszer hat egy torrentre. Ha egy torrentet azután indít újra, hogy a cél leállította, tovább oszt, amíg le nem állítja. Csak akkor áll le újra, ha megváltoztatja a célt, és a torrent eléri az újat.

Az 1.2.44-es verzióig a cél mindig életben volt, és minden torrentet 150 %-nál leállított. Frissítéskor a soha nem módosított cél kikapcsolva marad. A módosított cél tovább állítja a torrenteket: a jelölőnégyzet bejelölve marad Önnek, 0-val.

### Cél egyetlen torrentre

1. Jelölje ki a torrentet, és válassza a **Jellemzők** parancsot (<kbd>Enter</kbd>).
2. A **Megosztási cél** alatt jelölje be az **Az alapértelmezett beállítások felülbírálása** négyzetet.
3. Adja meg a **Legkisebb arány (%): [0: nincs arány cél]** és a **Legrövidebb megosztási idő (perc): [0: nincs idő cél]** értékét. Ha mindkettő 0, a torrent örökké oszt.
4. Kattintson az **OK** gombra.

A torrent saját célja mindig életben van, akkor is, ha a Tulajdonságok ablakban a jelölőnégyzet ki van kapcsolva: a torrent leáll, amikor eléri (vagy a feltöltése korlátozva lesz, ha a négyzet sebességgel van bejelölve).

![A Torrent jellemzői ablak](images/properties.png)

## Kezdeti megosztás

A kezdeti megosztás (super-seeding) abban segít, hogy az éppen elkészített torrentet elterjessze, amikor Ön az egyetlen seed: minden peer olyan darabokat kap, amelyek még senkinél nincsenek meg, így a raj hamarabb jut teljes másolathoz.

- Kattintson jobb gombbal egy megosztó torrentre, és válassza a **Haladó > Kezdeti megosztás** parancsot. A tétel szürke, hacsak nem minden kijelölt torrent oszt; a parancs újbóli kiválasztásával kapcsolható ki.
- Vagy jelölje be a **Kezdeti megosztás** négyzetet az **Egyéb beállítások** alatt a torrent Jellemzők ablakában.

Amíg be van kapcsolva, az állapot **[S] Megosztás**. A megosztandó torrent készítéséről lásd: [Torrentek készítése](Torrentek-készítése). A rookparcel-nox esetében a `--super-seed` ugyanezt teszi a kész torrentekkel.

## Kész letöltések áthelyezése

A Rookparcel a torrent fájljait másik mappába helyezheti, amikor a letöltés befejeződik, például egy gyors lemezről egy nagyra. A megosztást az új helyről folytatja. A **Beállítások > Tulajdonságok > Mappák** oldalon, **A letöltött fájlok helye** alatt adható meg:

1. Jelölje be a **Kész letöltések áthelyezése ide:** négyzetet, és válassza ki alatta a mappát.
2. Ha kívánja, jelölje be **A torrent címkéjének hozzáfűzése** négyzetet: a címkével rendelkező torrent ebbe a mappába, a címkéről elnevezett almappába kerül. A befejezéskor érvényes címke számít.
3. Hagyja bejelölve a **Csak az alapértelmezett letöltési mappából helyezze át** négyzetet, ha csak azokat a torrenteket akarja áthelyezni, amelyek magában az új letöltések mappájában (**Új letöltések helye:**) vannak; a máshová mentett torrentek, még az abban lévő almappák is, ilyenkor a helyükön maradnak. Törölje a jelölést, ha minden kész torrentet át akar helyezni.
4. Kattintson az **OK** gombra.

A mostantól befejeződő torrentekre vonatkozik. A mappákról és a fájlok áthelyezéséről bővebben: [Lemez és tárolás](Lemez-és-tárolás#kész-letöltések-áthelyezése).

## Automatikus leállítás és a megosztás

A **Beállítások > Automatikus leállítás > ..., ha minden befejeződött** megvárja, hogy minden megosztás leálljon. A megosztási cél nélküli vagy kényszerített megosztás tovább oszt, és visszatartja a leállítást. Lásd: [Automatikus leállítás](Automatikus-leállítás).

## A GTK ablakban és a webes felületen

- **GTK ablak**: ugyanaz a Sorba állítás oldal, ugyanazok a Jellemzők és menük.
- **macOS-alkalmazás**: ugyanaz a Sorba állítás oldal, ugyanazok a Jellemzők és menük; a Tulajdonságok azonnal érvénybe lépnek, **OK** nélkül.
- **Webes felület**: ugyanaz a Sorba állítás oldal. A megosztási cél négyzetének felirata **A torrent leállítása vagy a feltöltési sebességének korlátozása**, mellette a sebességgel; a 0 leállítja a torrentet. A torrentmenüben ugyanazok a várólista- és Kezdeti megosztás tételek vannak.
- **rookparcel-nox**: a várólista korlátait és a megosztási célt a webes felület Tulajdonságok ablakában kell megadni. A `--move-on-complete DIR` minden befejeződő torrentet egy mappába helyez át.

## Kapcsolódó oldalak

- [Torrentek kezelése](Torrentek-kezelése)
- [Sebességkorlátok és ütemező](Sebességkorlátok-és-ütemező)
- [Lemez és tárolás](Lemez-és-tárolás)
- [A beállítások leírása](A-beállítások-leírása)
