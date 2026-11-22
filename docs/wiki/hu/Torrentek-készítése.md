*Ez a [Creating torrents](Creating-torrents) című angol oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Torrentek készítése

Ez az oldal azt mutatja be, hogyan készíthet új `.torrent` fájlt a számítógépén lévő fájlból vagy mappából az Új torrent létrehozása ablakkal, mit jelent az egyes beállítás, és hogyan indíthatja el azonnal a megosztását.

![Az Új torrent létrehozása ablak](images/create-torrent.png)

## Torrent készítése öt lépésben

1. Válassza a **Fájl > Új torrent létrehozása...** parancsot (<kbd>Ctrl</kbd>+<kbd>N</kbd>), vagy az eszköztár **Új torrent létrehozása** gombját.
2. A **Forrás kiválasztása** alatt kattintson a **Fájl hozzáadása** gombra egyetlen fájlhoz, vagy a **Mappa hozzáadása** gombra egy mappához. Az útvonalat be is írhatja vagy beillesztheti.
3. A **Torrent jellemzői** alatt adja meg a kívánt trackereket, soronként egyet. Tracker nélkül a peerek csak a DHT-n és a peercserén keresztül találhatják meg egymást.
4. Hagyja bejelölve a **Megosztás indítása** négyzetet, hogy azonnal megoszthassa.
5. Kattintson a **Létrehozás és mentés másként...** gombra, válassza ki, hová kerüljön a `.torrent` fájl, és várja meg az előrehaladásjelzőt.

A javasolt fájlnév a forrás neve plusz `.torrent`. Amikor a hash-elés kész, az ablak bezárul. Ossza meg a `.torrent` fájlt azokkal, akiknek le kell tölteniük.

## A beállítások

### Forrás kiválasztása és Fájlok kihagyása

A **Forrás kiválasztása** az a fájl vagy mappa, amelyből a torrent készül. A mappa neve lesz a torrent neve.

A **Fájlok kihagyása** név szerint kihagy fájlokat és mappákat a torrentből. Írjon helyettesítő mintákat: a `*` tetszőleges karaktersort, a `?` egyetlen karaktert jelent; a mintákat `|` vagy `;` választja el. A kis- és nagybetű nem számít, és a mintának a fájl vagy mappa teljes nevére kell illeszkednie, nem az útvonalára. Például a `*.nfo|Thumbs.db|*.tmp` kihagyja az NFO-fájlokat, a Windows miniatűr-gyorsítótárait és az ideiglenes fájlokat.

### Torrent jellemzői

| Beállítás | Jelentése |
|---|---|
| **Trackerek:** | soronként egy trackercím. Az üres sor új szintet kezd: a későbbi szinteket akkor használja, ha az előttük lévő trackerek elbuknak. Az ismételt címeket elhagyja. |
| **Webes seedek:** | olyan webkiszolgálók címei, amelyeken ugyanezek a fájlok megvannak, soronként egy. A peerek ezekről úgy tölthetnek le, mint egy seedről. |
| **Megjegyzés:** | a torrentben tárolt szabad szöveg; más programok megjelenítik, például az Új torrent hozzáadása ablakban. |
| **Részméret:** | az egyes részek mérete, 16 kB-tól 16 MB-ig. Az **(automatikus)** az összméret alapján választ, és a legtöbb torrenthez ez a megfelelő választás. |
| **Verzió:** | a BitTorrent verziója, lásd lent. |

### Verzió: v1, v2 vagy hibrid

| Választás | Mit kap |
|---|---|
| **Hibrid (v1 + v2)** | az alapértelmezés. Betölthető a régebbi, csak v1-et ismerő programokban és az újabbakban is, amelyek értik a v2-t. |
| **v1** | a klasszikus formátum, ahogy a µTorrent 2.2.1 készítette. Minden BitTorrent-program olvassa. |
| **csak v2** | csak az újabb formátum. A csak v1-et értő programok nem tudják betölteni. |

Ha bizonytalan, tartsa meg a **Hibrid (v1 + v2)** beállítást.

### Egyéb

| Beállítás | Jelentése |
|---|---|
| **Megosztás indítása** | az új torrentet a létrehozása után azonnal hozzáadja a listához, letöltési mappának a forrást tartalmazó mappát megadva. A Rookparcel ellenőrzi a fájlokat, majd megosztja őket. A Tulajdonságokban a **Trackerek az új letöltésekhez** alatt megadott trackereket nem adja hozzá; a begépeltek maradnak. |
| **Privát torrent** | privátnak jelöli a torrentet: a peereket csak a trackerein keresztül találják meg, soha a DHT-n, a peercserén vagy a Helyi peerkeresésen át. A privát trackerek ezt kérik. |
| **Fájlsorrend megtartása** | egy mappa fájljait abban a sorrendben tartja, ahogyan a Windows listázza őket, ahelyett hogy név szerint rendezné. Csak a **v1** esetén érhető el: a v2 és a hibrid torrentek mindig rendezik a fájljaikat. |

## Előrehaladás és megszakítás

Amíg a Rookparcel olvassa és hash-eli a fájlokat, az előrehaladásjelző és a százalék mozog, a többi vezérlő pedig szürke. A **Bezárás** gomb **Mégse** gombbá válik: kattintson rá, nyomja meg az <kbd>Esc</kbd> billentyűt, vagy zárja be az ablakot a leállításhoz. Megszakításkor semmi sem mentődik.

Ha valami elromlik, egy üzenet megmondja, mi. Gyakoriak:

| Üzenet | Jelentése |
|---|---|
| „Válasszon fájlt vagy mappát, amelyből a torrent készül.” | A **Forrás kiválasztása** üres. |
| „...: nincs fájl a torrentbe (üres, vagy mind kihagyva).” | a mappa üres, vagy a **Fájlok kihagyása** semmit sem hagyott meg. |

## Mit jegyez meg a Rookparcel

Amikor legközelebb megnyitja az ablakot, benne lesznek a legutóbb használt trackerek, részméret, verzió, **Megosztás indítása**, **Privát torrent** és **Fájlsorrend megtartása** beállítások. A forrás, a kihagyási minták, a webes seedek és a megjegyzés üresen indul.

## Korábban készített torrent megosztása

Ha a torrentet a **Megosztás indítása** nélkül készítette, adja hozzá a `.torrent` fájlt, mint bármelyik másikat ([Torrentek hozzáadása](Torrentek-hozzáadása)), és a **Mentés ide** mezőt állítsa arra a mappára, amely a forrásfájlt vagy -mappát tartalmazza. A Rookparcel ellenőrzi a fájlokat, és megosztja őket.

Hogy egy vadonatúj torrent terjedését segítse, bekapcsolhatja rajta a **Kezdeti megosztás** funkciót; lásd a [Torrentek kezelése](Torrentek-kezelése#kezdeti-megosztás) oldalt.

## A GTK-s változatban (Linux, BSD)

Ugyanaz az ablak ugyanazokkal a beállításokkal, a **Fájl > Új torrent létrehozása...** alatt (<kbd>Ctrl</kbd>+<kbd>N</kbd>).

## A macOS-alkalmazásban

Ugyanaz az ablak ugyanazokkal a beállításokkal, a **Fájl > Új torrent létrehozása...** alatt (<kbd>⌘</kbd>+<kbd>N</kbd>).

## A webes felületen és a rookparcel-noxban

A webes felület nem tud torrentet készíteni: ehhez a fájloknak a kiszolgálón kellene lenniük. Magán a kiszolgálón a `rookparcel-nox create` parancssorból készít torrentet:

```
rookparcel-nox create "D:\Share\My folder" -o my-folder.torrent -t udp://tracker.example.org:6969/announce
```

A `--v1`, a `--v2` és a `--hybrid` választja ki a verziót (alapértelmezés a hibrid), a `--private` privátnak jelöli, a `--piece-size` a részméretet adja meg KiB-ban, a `-w` webes seedet ad hozzá, a `--magnet` pedig a magnet hivatkozást is kiírja. A `rookparcel-nox --help` az összes kapcsolót felsorolja. Lásd [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

A **Fájl > Csatlakozás távoli géphez...** paranccsal kapcsolódott ablakban a **Megosztás indítása** nincs bejelölve, és szürke: a torrent ezen a számítógépen lévő fájlokból készül, amelyek a kiszolgálón nincsenek meg. Lásd a [Távvezérlés](Távvezérlés) oldalt.

## Kapcsolódó oldalak

- [Torrentek hozzáadása](Torrentek-hozzáadása)
- [Torrentek kezelése](Torrentek-kezelése)
- [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal)
