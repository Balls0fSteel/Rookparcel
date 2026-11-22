*Ez az angol nyelvű [Importing from µTorrent and qBittorrent](Importing-from-uTorrent-and-qBittorrent) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Importálás µTorrentből és qBittorrentből

Ez az oldal azt írja le, hogyan hozhatja át a torrentjeit a µTorrentből vagy a qBittorrentből ugyanarról a számítógépről, hogy a Rookparcelben onnan folytassák a letöltést és a megosztást, ahol tartanak, anélkül hogy bármit újra le kellene tölteni.

## Mi kerül át

A Rookparcel minden torrentnél átveszi:

- az adatokat, amelyek a helyükön maradnak: semmi sem másolódik vagy mozog;
- a statisztikákat, a fájlprioritásokat (a kihagyott fájlok kihagyva maradnak) és a trackereket;
- a címkét: a qBittorrent kategóriáját, vagy a µTorrent címkéjét;
- a qBittorrent jelölőit jelölőként (*új az 1.2.63-ban*; korábban a kategória nélküli torrent címkéje az első jelölő lett). A vesszőt vagy vezérlőkaraktert tartalmazó jelölőnevet kihagyja;
- a leállított vagy szüneteltetett állapotot;
- a qBittorrentből a torrent saját megosztási korlátait (arány és megosztási idő), amelyekből a torrent saját [megosztási célja](Torrent-részletek-és-fájlok#torrent-jellemzői) lesz.

A Rookparcel nem bízik a másik program arra vonatkozó feljegyzésében, hogy mely részek készek: a fájlokat ellenőrzi, mielőtt támaszkodna rájuk. A nagy torrenteknél ezért az importálás után egy ideig **Ellenőrzés** látszik.

A másik program fájljait csak olvassa, soha nem módosítja. A részleges adatfájlokat, amelyeket a µTorrent `.!ut`, a qBittorrent `.!qB` néven hagyott, felveszi, és átnevezi, amikor a torrent fut: lásd: [Lemez és tárolás](Lemez-és-tárolás#befejezetlen-fájlok-és-a-ut) (*új az 1.2.63-ban*). A Rookparcel listájában már szereplő torrenteket kihagyja, így az importálást nyugodtan futtathatja újra.

## Mielőtt elkezdi

1. **Zárja be a másik programot**, a tálcaikonját is. Mindkét program újraírja a fájljait kilépéskor, ezért a Rookparcel nem importál, amíg látja, hogy a µTorrent vagy a qBittorrent fut: „... fut. Zárja be, majd importáljon újra: kilépéskor újraírja a fájljait.” macOS-en és BSD-n a Rookparcel nem tudja megállapítani, hogy fut-e, ezért győződjön meg róla Ön.
2. Győződjön meg róla, hogy a letöltött adatokat tartalmazó meghajtók csatlakoztatva vannak, ugyanazokkal a betűjelekkel vagy útvonalakkal, mint korábban.

Az importálás után ne futtassa mindkét programot ugyanazokon a torrenteken: mindkettő ugyanazokba a fájlokba írna.

## Importálás

1. Válassza a **Fájl > Importálás a qBittorrentből...** vagy a **Fájl > Importálás a µTorrentből...** parancsot.
2. A Rookparcel megnézi a program szokásos mappáját (lásd alább). Ha nincs ott, megnyílik egy mappaválasztó: „Válassza ki a(z) qBittorrent mappáját” vagy „Válassza ki a(z) µTorrent mappáját”. Válassza ki az alább leírt mappát.
3. A Rookparcel megkérdezi: „Importál 47 torrentet a(z) qBittorrent programból? Adataik a helyükön maradnak, a(z) qBittorrent fájljai nem változnak. A már a listában lévő torrentek kimaradnak.” A kérdés felsorolja azokat a mappákat, amelyekben az adatok vannak. Ellenőrizze őket, majd kattintson az **Igen** gombra.
4. Egy üzenet közli, hány torrent került hozzáadásra, például „45 torrent hozzáadva a(z) qBittorrent programból. 2 kimaradt; az indokokat a Napló lap sorolja fel.”

A torrentek megjelennek a listában, és megkezdődik az ellenőrzésük. A címkéik a kategórialistában a **Címkék** alatt, a qBittorrent jelölői a **Jelölők** alatt látszanak.

## Hol keres a Rookparcel

| Program | Szokásos mappa | Mit válasszon, ha kéri |
|---|---|---|
| qBittorrent Windowson | `%LOCALAPPDATA%\qBittorrent\BT_backup` | a `BT_backup` mappát, vagy a fölötte lévő `qBittorrent` mappát |
| qBittorrent Linuxon és BSD-n | `~/.local/share/qBittorrent/BT_backup` (vagy az `$XDG_DATA_HOME` alatt) | ugyanezt |
| qBittorrent macOS-en | `~/Library/Application Support/qBittorrent/BT_backup` | ugyanezt |
| µTorrent Windowson | `%APPDATA%\uTorrent` | azt a mappát, amelyben a `resume.dat` van |

A qBittorrent a `BT_backup` mappában torrentenként egy `.fastresume` és egy `.torrent` fájlt tart; a µTorrent a `resume.dat` fájlt a `.torrent` fájlok mellett. Ha az Ön példánya máshol tartja őket, azt a mappát válassza ki. Az adatok a feljegyzések által megnevezett mappákban maradnak, ezért ezeknek a mappáknak létezniük kell ezen a számítógépen.

## A kihagyott torrentek

Minden importált vagy kihagyott torrentről kap egy sort a [Napló lap](Torrent-részletek-és-fájlok#napló): „Importálva a(z) qBittorrent programból: ...” vagy „Nem importálva a(z) qBittorrent programból: ...: ”, utána az okkal.

| Ok | Mit tegyen |
|---|---|
| már a listában van | semmit; a Rookparcelben már megvan |
| hiányzik a .torrent fájlja | adja hozzá Ön a torrentet a `.torrent` fájljából vagy a magnet hivatkozásából, a **Mentés ide** mezőt az adatai mappájára állítva |
| a folytatási rekordja nem olvasható | ugyanígy |
| a .torrent fájlja nem olvasható: ... | ugyanígy, a `.torrent` fájl egy jó másolatával |

Ha a Rookparcel egyáltalán nem talál torrenteket, ezt mondja: „A(z) ... mappában nem található qBittorrent-torrent.”, a beolvasott mappával.

## A GTK-s változatban (Linux, BSD)

Ugyanaz a két elem a **Fájl** menüben, ugyanazokkal a lépésekkel. A µTorrentnek Windowson kívül nincs szokásos mappája, ezért a Rookparcel a `resume.dat` fájlt tartalmazó mappát kéri, például egy Windows-os számítógépről átmásoltat.

## A rookparcel-noxszal

A fej nélküli démon induláskor importál:

```
rookparcel-nox run --import qbittorrent
rookparcel-nox run --import utorrent --import-dir "D:\Backup\uTorrent"
```

Az `--import` a `qbittorrent` vagy az `utorrent` értéket fogadja; az `--import-dir` megnevezi a mappát, ha nem a szokásos (a qBittorrent `BT_backup` vagy adatmappája, a µTorrent `resume.dat` fájlt tartalmazó mappája). A másik program nem futhat, és a nox listájában már szereplő torrenteket úgy hagyja, ahogy vannak. Lásd: [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

A webes felületen nincs importálás, és a **Fájl > Csatlakozás távoli géphez...** paranccsal csatlakoztatott ablakban az importálás elemei szürkék: a kiszolgáló nem olvashatná ennek a számítógépnek a mappáit. Lásd: [Távvezérlés](Távvezérlés).

## Kapcsolódó oldalak

- [Torrentek hozzáadása](Torrentek-hozzáadása)
- [Torrentek kezelése](Torrentek-kezelése)
- [Várólista és seedelés](Várólista-és-seedelés)
