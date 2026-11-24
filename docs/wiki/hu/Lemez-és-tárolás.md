# Lemez és tárolás

*Ez a [Disk and storage](Disk-and-storage) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

Ez az oldal azt mutatja be, hová teszi a Rookparcel a letöltéseket és hogyan helyezi át őket, valamint hogyan bánik a lemezekkel: az okos lemez I/O-val (kíméletes ellenőrzések, hangolás a meghajtó típusa szerint, lassítás, amíg a lemez foglalt) és a lemezolvasási gyorsítótárral.

## Hová kerülnek a letöltések

A **Beállítások > Tulajdonságok > Mappák** oldalon, **A letöltött fájlok helye** alatt:

| Beállítás | Jelentés |
|---|---|
| **Új letöltések helye:** | A mappa, ahová az új torrentek kerülnek, hacsak az Új torrent hozzáadása ablakban nem választ másikat. |
| **Kész letöltések áthelyezése ide:** | Amikor egy torrent letöltése befejeződik, a fájljai az alatta megadott mappába kerülnek, és a torrent onnan folytatja a megosztást. |
| **A torrent címkéjének hozzáfűzése** | Az előző négyzet mellett: a címkével rendelkező kész torrent a címkéről elnevezett mappába kerül, a kész letöltések mappáján belül. |
| **Csak az alapértelmezett letöltési mappából helyezze át** | Csak az új letöltések mappájába mentett torrenteket helyezi át; a máshová mentett torrentek a helyükön maradnak. Alapértelmezés szerint be van kapcsolva. |

A Beállítási útmutató az első indításkor kéri a letöltési mappát: lásd [Első indítás](Első-indítás).

Ugyanezen az oldalon a .torrent fájlok másolatainak tárolása és a figyelt mappából való betöltés is megadható (**A .torrent fájlok helye**): lásd [Torrentek hozzáadása](Torrentek-hozzáadása).

### Kész letöltések áthelyezése

1. Jelölje be a **Kész letöltések áthelyezése ide:** négyzetet, és válassza ki a mappát.
2. Jelölje be **A torrent címkéjének hozzáfűzése** négyzetet, ha címkék szerint rendez, például a „Filmek” címkéjű torrentek a `D:\Done\Movies` mappába kerüljenek. A címke nélküli torrent közvetlenül a mappába kerül.
3. Döntsön a **Csak az alapértelmezett letöltési mappából helyezze át** négyzetről (lásd fent).
4. Kattintson az **OK** gombra.

A mostantól befejeződő torrentekre vonatkozik; a korábban befejezettek a helyükön maradnak. Amíg egy torrent fájljait áthelyezik, az állapota **Áthelyezés**, a **Napló** pedig azt írja: „a kész letöltés áthelyezése ide: ...”. A befejezéskor érvényes címke számít, ezért a címkéket a letöltés befejeződése előtt állítsa be.

### Torrent áthelyezése kézzel

Kattintson jobb gombbal a torrentekre, és válassza a **Haladó > Letöltési hely megadása...** parancsot, hogy kiválasszon egy mappát, és a fájljaikat oda helyezze. Az ugyanolyan nevű, már ott lévő fájlokat felülírja. A tétel szürke, amíg a torrent áthelyezése már folyamatban van. A torrent egyetlen fájlja a Fájlok lap **Áthelyezés...** parancsával helyezhető át: lásd [Torrent-részletek és fájlok](Torrent-részletek-és-fájlok).

### Kilépés fájlok áthelyezése közben

Ha kilép, miközben fájlokat helyeznek át, a záródó ablak megvárja az áthelyezéseket, a haladásukkal együtt, mert félúton megállva egy torrent fél részben az egyik, fél részben a másik mappában maradhatna. A **Kilépés most** előbb rákérdez, és megmondja, mit vonna vissza: a torrent fájljai a régi mappában maradnak, az új helyre már átmásolt részek törlődnek, és később újra áthelyezheti.

### Befejezetlen fájlok és a .!ut

*Új az 1.2.63-ban.* Ha a **Beállítások > Tulajdonságok > Általános > .!ut hozzáfűzése a befejezetlen fájlokhoz** be van jelölve (a **Letöltéskor** alatt, alapértelmezés szerint ki van kapcsolva), a még letöltés alatt álló fájl a neve után `.!ut` kiterjesztéssel tárolódik, például `film.mkv.!ut`. Amint a fájl teljesen letöltődött és átment az ellenőrzésen, megkapja a szokásos nevét, miközben a torrent többi része tovább tölt.

- **Miért:** a médialejátszók, a fájlfigyelők és a [Sonarr és Radarr](Sonarr-Radarr-és-Prowlarr) csak akkor látnak egy fájlt a valódi nevén, ha az kész, így semmi sem nyit meg vagy importál félig kész fájlt. A µTorrent is így nevezi el a befejezetlen fájlokat.
- **Be- vagy kikapcsolás:** a futó torrentek fájljait azonnal átnevezi, a leállított torrentekét pedig a következő indításukkor.
- **Mi nem kapja soha:** az üres fájlok, egyes torrentek kitöltő fájljai, az a fájl, amelynek a neve a toldalékkal túl hosszú lenne, és az a fájl, amelynek a toldalékos neve ugyanannak a torrentnek egy másik fájlja vagy mappája (olyan torrent, amelyben egyaránt van `x` és `x.!ut`): ez megtartja a sima nevét. A Fájlok lapon átnevezett fájl az új neve után kapja a `.!ut` toldalékot.
- **Fájlok lap:** a **Megnyitás** és a **Mappa megnyitása** a fájlt a `.!ut` nevén találja meg. A webes felület és az API a sima neveket mutatja.
- **Az a fájl, amelyet egy másik program megnyitva tart** (Windowson például egy lejátszó, amely nem osztja meg), addig megtartja a mostani nevét, amíg át nem nevezhető. A Napló ezt egyszer kiírja, a Rookparcel pedig körülbelül 30 másodpercenként, valamint a következő indításkor újra próbálkozik. A torrent eközben tovább működik.
- **Meglévő fájlt soha nem ír felül.** Ha a mappában `film.mkv` és `film.mkv.!ut` is van, a kész fájl `film.mkv.!ut` marad. A Napló ezt egyszer kiírja. Távolítsa el a másik fájlt, majd szüneteltesse és folytassa a torrentet, hogy átnevezze.
- **Áthelyezés...** a Fájlok lapon egy még le nem töltött fájlnál azt a fájlt veszi át, amelyre mutat, azon a néven, amelyen ott van: `film.mkv`, `film.mkv.!ut` vagy `film.mkv.!qB`. Futtasson **Kényszerített ellenőrzés** parancsot, hogy a darabjait megszámolja.
- **Letöltési hely megadása** felülírja az új mappában az ugyanolyan nevű fájlt, az ott lévő `film.mkv`-t is, ha az áthelyezett fájl még `film.mkv.!ut`. A még le nem töltött fájlt az új mappából veszi át, bármelyik néven is van ott.

**µTorrentből vagy qBittorrentből érkezve.** Akár be van jelölve a négyzet, akár nem, a várt néven hiányzó fájlt a `.!ut` (a µTorrent toldaléka) és a `.!qB` (a qBittorrent „Append .!qB extension to incomplete files” beállítása) névvel is megkeresi. Az [importált](Importálás-uTorrentből-és-qBittorrentből) vagy a **Mentés ide** mezőben a régi mappájával újra hozzáadott torrent ezért a részleges fájljaiból folytatódik, ahelyett hogy elölről kezdené. Ezeket utána átnevezi arra a névre, amelyet a Rookparcel ad nekik: a `.!ut` névre, amíg befejezetlenek és a négyzet be van jelölve, egyébként a sima névre. A qBittorrent külön mappájában a befejezetlen fájlok, valamint a `.unwanted` mappák között nem keres.

Az **Eltávolítás és > Adatok törlése** a `.!ut` vagy `.!qB` fájlt csak akkor törli, ha az ennek a torrentnek az adatait tartalmazza: a Rookparcel írt bele, vagy a benne lévő darabok átmentek az ellenőrzésen. Az ilyen nevű részleges fájlt, amelyhez a torrent soha nem nyúlt (másik programé, vagy felvett, de még nem ellenőrzött), megtartja, és a Napló megnevezi.

## Ha egy meghajtó vagy megosztás hiányzik

*Új az 1.2.76-ban.* Ha egy olyan torrent, amelynek már le volt töltve adata, indításkor egyetlen fájlját sem találja, vagy a mentési mappája eltűnt (kihúzott USB-lemez, nem elérhető hálózati megosztás vagy csatolás, még nem kész meghajtóbetűjel), a Rookparcel nem tölti le újra az adatokat. A torrent hibával leáll, amely a hiányzó útvonallal kezdődik, és ezt mondja: **files missing (the drive, share or folder is not available)**, a meglévő darabjait pedig megtartja. A Rookparcel 30 másodpercenként újra megnézi, és amint a mappa visszatér, a torrent magától elindul, és a fájlok újraellenőrzése nélkül folytatja. Ugyanez történik, ha a meghajtó letöltés közben tűnik el. A Rookparcel közbeni bezárása nem változtat semmit: a következő indításkor a torrentet ismét hiányzónak vagy visszatértnek találja. A visszatartott torrenten a **Start** azonnal újrapróbálja, és ha a fájlok még mindig hiányoznak, újra visszatartják. A visszatartott torrent hibás, ezért az [Automatikus leállítás](Automatikus-leállítás) nem vár rá.

Ha egy torrentnek csak néhány fájlja tűnt el (kézzel vagy vírusirtó törölte), csak a darabjaik töltődnek le újra. Ha egy torrent teljes mappáját kézzel törli, a torrentet ugyanúgy visszatartja. Ha le akar mondani a visszatartott torrent régi adatairól és újra le akarja tölteni, vagy el akarja fogadni, ami most a lemezen van, használja a **Kényszerített ellenőrzés** parancsot: ez ellenőrzi a meglévő fájlokat, és letölti a többit.

## Okos lemez I/O

Az okos lemez I/O reagálóképesen tartja a számítógépet, amíg a Rookparcel olvas és ír. A beállításai a **Beállítások > Tulajdonságok > Játék** oldalon vannak (a Haladó fölötti oldal az oldallistában).

A Beállítási útmutató **Okos lemez I/O használata** négyzete mind a négy részt együtt kapcsolja be vagy ki. A meghajtónkénti hangolás kezdettől be van kapcsolva; a négyzet hozzáadja a kíméletes ellenőrzéseket és a lassítást, amíg a lemez foglalt. *A Beállítási útmutató az 1.1.62-ben új. A beállítások az 1.1.40 óta megvannak; az 1.2.79 óta a Játék oldalon vannak, és a kíméletes ellenőrzés kezdetben ki van kapcsolva.*

### Ellenőrzés

Az ellenőrzés (például a **Kényszerített ellenőrzés** után) minden darabot beolvas a lemezről, és összeveti a torrenttel. Az **Ellenőrzés** alatt:

| Beállítás | Jelentés | Alapérték |
|---|---|---|
| **Kíméletes torrentellenőrzés (alacsony prioritás, elsőbbség a többi programnak)** | A rendszer háttérlemez-prioritásán ellenőriz, békén hagyja a többi program gyorsítótárazott fájljait, és szünetel, amíg az olvasás a szokásosnál lassabb, így a számítógép reagálóképes marad. Az ellenőrzések tovább tartanak, amíg a lemez foglalt. | Ki |
| **Ellenőrzés korlátozása (MB/s):** | Korlátozza az egyes ellenőrzések olvasási sebességét, olyan lemezhez, amely a kíméletes ellenőrzés mellett is lemarad. Itt 1 MB 1 000 000 bájt. | Ki, 100 |
| **Egyszerre ellenőrzött meghajtók (meghajtónként egy torrent):** | Hány meghajtót ellenőriz egyszerre, meghajtónként egy torrentet; az ugyanazon a meghajtón lévő torrentek a sorukra várnak. 1-től 8-ig. | 4 |

A különböző meghajtókon lévő torrentek ellenőrzése egymás mellett fut; az egy meghajtón lévő két torrentet egymás után ellenőrzi, így a meghajtó nem ugrál közöttük. Az állapot **Ellenőrzésre vár** a sorára váró torrentnél, majd **Ellenőrzés** a százalékkal. *A várakozó állapot az 1.2.76-ban új; korábban a várakozó ellenőrzés „Ellenőrzés 0,0 %” állapotot mutatott.* A **Kényszerített ellenőrzés** paranccsal kért ellenőrzés nem vár szabad helyre a torrentek várólistájában ([Várólista és seedelés](Várólista-és-seedelés)).

*Új az 1.2.76-ban.* Az a fájl, amely létezik, de nem olvasható (másik program tartja, vagy nincs hozzá jogosultsága), fájlhibával állítja meg az ellenőrzést, ahelyett hogy a darabjait hiányzónak számítaná és újra letöltené. Az a fájl, amely egyáltalán nincs ott, továbbra is csak a saját darabjait veszíti el.

### Meghajtók

| Beállítás | Jelentés | Alapérték |
|---|---|---|
| **Hangolás a meghajtó típusához (SSD, merevlemez, hálózat)** | Felismeri, hogy az egyes mentési mappák SSD-n, merevlemezen, cserélhető meghajtón vagy hálózati megosztáson vannak-e. A merevlemez vagy cserélhető meghajtó ezután egyszerre legfeljebb két lemezmunkát kap, a hálózati kiszolgáló négyet, hogy ne kalapáljanak. | Be |

A Napló kiírja az indításkor talált meghajtókat, például „Disk I/O: drive D: (HDD, SATA)” formában. Az USB-hidak, a RAID-vezérlők és a virtuális gépek gyakran rossz típust jelentenek.

**Téves találgatás kijavítása.** Használja a `drive_overrides` beállítást a **Beállítások > Tulajdonságok > Haladó** oldalon: vesszővel elválasztott `útvonal=típus` párok, a típus `ssd`, `hdd`, `network` vagy `removable`, például `D:/=hdd, E:/=removable`. A leghosszabb egyező útvonal nyer.

1. Nyissa meg a **Haladó** oldalt, és írja be a `drive_overrides` szót a **Szűrő:** mezőbe.
2. Jelölje ki a sort, írja be a párokat az **Érték:** mezőbe, és kattintson a **Beállítás** gombra.
3. Kattintson az **OK** vagy az **Alkalmaz** gombra.

![Tulajdonságok, Haladó oldal](images/prefs-advanced.png)

### Ha a lemez foglalt

A **Ha a lemez foglalt** alatt, mindkettő alapértelmezés szerint ki van kapcsolva:

| Beállítás | Jelentés |
|---|---|
| **Letöltések lassítása (lemezírások)** | Amíg egy meghajtó az írásokra észrevehetően lassabban válaszol a szokásosnál, lépésenként felezi a letöltési sebességet, de 50 kB/s alá soha, és újra növekedni hagyja, amint a lemez megnyugszik. |
| **Feltöltések lassítása (lemezolvasások)** | Ugyanez a feltöltésekre, amíg egy meghajtó lassan válaszol az olvasásokra. A megosztás ilyenkor kevesebbet tölt fel, ami az arányt számon tartó trackereknél számít. |

Soha nem emelik meg az Ön saját korlátait. A lassítás irányonként egyetlen korlát, amelyen az [Alacsony ping](Alacsony-ping) is osztozik: az alacsonyabb korlát nyer. Amíg egy irányt visszatart, az állapotsor „(auto ...)” formában mutatja, a Naplóban pedig van róla egy sor.

## Lemezgyorsítótár és fájlok

A **Beállítások > Tulajdonságok > Haladó > Lemezgyorsítótár** oldalon van az olvasási gyorsítótár és a lemezsor. A legtöbb felhasználónak soha nem kell módosítania őket.

A **Lemezolvasási gyorsítótár** alatt:

| Beállítás | Jelentés | Alapérték |
|---|---|---|
| **Lemezolvasások gyorsítótárazása** | A nemrég olvasott darabokat a memóriában tartja, a peerek által kért blokkot a szomszédaival együtt olvassa, ami megosztáskor lemezkereséseket takarít meg. | Be |
| **Automatikus gyorsítótárméret felülbírálása (MB):** | A mellette megadott méretet használja az automatikus 32 MB helyett. Nagyobb gyorsítótár segíthet a forgalmas megosztásnál, a memória terhére. | Ki |
| **Ennél régebben nem használt blokkok eltávolítása (másodperc):** | Eldobja azokat a gyorsítótárazott blokkokat, amelyeket ennyi ideje egy peer sem kért. Kikapcsolva, addig maradnak, amíg újabb blokkok ki nem szorítják őket. | Be, 300 |

Az írásokat a rendszer saját gyorsítótárára hagyja.

A **Lemezsor és fájlok** alatt:

| Beállítás | Jelentés | Alapérték |
|---|---|---|
| **Lemezírási sor (kB):** | Mennyi letöltött adat várakozhat a lemezre íráshoz; efölött a Rookparcel nem olvas a peerektől, amíg a lemez fel nem zárkózik. A 4 MB alatti értékek 4 MB-nak számítanak. | 1024 |
| **Megnyitott fájlok legnagyobb száma:** | Legfeljebb ennyi fájlt tart egyszerre nyitva; a többit bezárja, és szükség esetén újra megnyitja. Emelje meg, ha sok torrent vagy fájl dolgozik egyszerre. | 40 |
| **Lemez I/O szálak (a következő indítástól):** | A torrentadatokat író és olvasó szálak. | 10 |
| **Hash-szálak (nincs használatban):** | Kompatibilitás miatt marad meg; ez a verzió nem használja. Az ellenőrzések meghajtónként egyesével futnak, a Játék oldalon beállítva. | |

A Rookparcelben nincs beállítás a fájlok előre lefoglalására. A fájl akkor jön létre, amikor az első adat megérkezik hozzá, Windowson pedig ritka (sparse) fájlként van megjelölve, így a vége közelébe írás nem készteti az NTFS-t arra, hogy az előtte lévő részt nullákkal töltse ki.

## A GTK ablakban és a webes felületen

- **GTK ablak** (Linux, BSD), **macOS** és a **webes felület**: a **.!ut hozzáfűzése a befejezetlen fájlokhoz** az Általános oldalon is megtalálható, a **Letöltéskor** alatt.
- **macOS-alkalmazás**: ugyanazok az oldalak és beállítások; a Tulajdonságok azonnal érvénybe lépnek, **OK** nélkül ([A beállítások leírása](A-beállítások-leírása#különbségek-macos-en)).
- **GTK ablak** (Linux, BSD): ugyanaz a Mappák, Lemezgyorsítótár és Játék oldal. A meghajtótípusokat Linuxon, macOS-en és FreeBSD-n is felismeri, mindegyiken a rendszer saját módszereivel.
- **Webes felület**: ugyanezek az oldalak a **Tulajdonságok** alatt, a **Lemezgyorsítótár** a **Haladó** alatt, a lemezbeállítások a **Játék** alatt. Az ottani mappák a Rookparcelt futtató számítógép mappái, nem a böngészőé.
- **rookparcel-nox**: a `--save DIR` a letöltési mappát adja meg, a `--move-on-complete DIR` pedig áthelyezi a kész torrenteket; a többi beállítás a webes felület Tulajdonságok ablakában van.

## Kapcsolódó oldalak

- [Várólista és seedelés](Várólista-és-seedelés)
- [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait) (a beállítások és a torrentek állapota, nem a letöltések)
- [Torrentek kezelése](Torrentek-kezelése)
- [A beállítások leírása](A-beállítások-leírása)
