*Ez az angol nyelvű [Adding torrents](Adding-torrents) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Torrentek hozzáadása

Ez az oldal bemutatja a torrentek Rookparcelnek való átadásának minden módját: `.torrent` fájl, magnet hivatkozás, webcím, húzás és ejtés, a böngésző és a fájlkezelő, valamint a figyelt mappa. Az Új torrent hozzáadása ablakot is leírja, amelyben a letöltés indítása előtt kiválaszthatja a mappát és a fájlokat.

## A gyors módok

| Amije van | Mit tegyen |
|---|---|
| Egy `.torrent` fájl a számítógépén | **Fájl > Torrent hozzáadása...** (<kbd>Ctrl</kbd>+<kbd>O</kbd>), vagy az eszköztár első gombja. Egyszerre több fájlt is kiválaszthat. |
| Egy magnet hivatkozás | **Fájl > Torrent hozzáadása URL-ről...** (<kbd>Ctrl</kbd>+<kbd>U</kbd>), és illessze be. |
| Egy `.torrent` fájl webcíme (`http://` vagy `https://`) | **Fájl > Torrent hozzáadása URL-ről...**, és illessze be. A Rookparcel letölti a fájlt, és hozzáadja. |
| Egy `.torrent` fájl az Intézőben | Húzza a Rookparcel ablakára, vagy kattintson rá duplán, ha a Rookparcel nyitja meg a `.torrent` fájlokat. |
| Egy magnet hivatkozás a böngészőben | Kattintson rá, ha a Rookparcel nyitja meg a magnet hivatkozásokat. |
| Sok, idővel érkező `.torrent` fájl | Hagyja, hogy a Rookparcel figyeljen egy mappát. Lásd: [Figyelt mappa](#figyelt-mappa). |
| Egy keresési találat | Lásd: [Torrentek keresése](Torrentek-keresése). |

A tálcaikon jobb gombos menüjében is van **Torrent hozzáadása...** és **Torrent hozzáadása URL-ről...**.

## Torrent hozzáadása URL-ről

![A Torrent hozzáadása URL-ről ablak](images/add-url.png)

1. Másolja ki a magnet hivatkozást vagy a `.torrent` fájl címét.
2. Válassza a **Fájl > Torrent hozzáadása URL-ről...** parancsot. Ha a vágólapon `magnet:`, `http://` vagy `https://` kezdetű hivatkozás van, a mező már ki van töltve.
3. Kattintson az **OK** gombra.

A címet a háttérben tölti le a program, a proxyján át, ha a Tulajdonságok > Kapcsolat a trackerkapcsolatokat proxyn át küldi. Egyes webhelyek a címre fájl helyett magnet hivatkozással válaszolnak; a Rookparcel ezt is követi. Ha a letöltés meghiúsul, egy üzenet közli, hogy nem tölthető le, a címmel és az okkal.

## Húzás és ejtés

Ejtsen egy vagy több `.torrent` fájlt az Intézőből a főablakra. Mindegyik úgy kerül hozzáadásra, mintha a **Torrent hozzáadása...** paranccsal választotta volna ki.

## A böngészőből és a fájlkezelőből

A Rookparcel lehet az a program, amelyet a Windows a `.torrent` fájlokhoz és a `magnet:` hivatkozásokhoz használ. Ezt a **Beállítások > Tulajdonságok > Általános** oldalon a **Társítás a .torrent fájlokkal** és a **Társítás a magnet: hivatkozásokkal** jelölőnégyzettel kapcsolhatja be, vagy a [beállítási útmutatóban](Első-indítás). A részleteket, köztük azt is, mit tegyen, ha a Windows egy másik programot tart meg, lásd: [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás).

Ha a Rookparcel már fut, és a böngészőjéből vagy az Intézőből megnyit egy torrentet, a futó Rookparcel veszi át. Ha az **A program ablakának aktiválása** be van kapcsolva (Tulajdonságok > Felület, alapból be van kapcsolva), a főablak is előre kerül; kikapcsolva csak az Új torrent hozzáadása ablak jelenik meg.

A Rookparcelt torrentekkel is elindíthatja a parancssorból: `rookparcel.exe file.torrent "magnet:?xt=..."`. Minden `.torrent` útvonal, `magnet:` hivatkozás és `http(s)://` cím hozzáadásra kerül. Ha a Rookparcel már fut, a futó ablakhoz kerülnek.

Ha a futó Rookparcelt rendszergazdai jogokkal indították, a Windows nem engedi, hogy egy második példány átadjon neki egy torrentet. A Rookparcel ezt közli; adja hozzá a torrentet a futó ablakból, vagy indítsa el a Rookparcelt rendszergazdai jogok nélkül.

## Az Új torrent hozzáadása ablak

![Az Új torrent hozzáadása ablak](images/add-new-torrent.png)

Ha az **Ablak megjelenítése a torrentben lévő fájlokkal** be van kapcsolva (Tulajdonságok > Felület, alapból be van kapcsolva), ez az ablak minden hozzáadott torrentnél megnyílik. Kikapcsolva a torrentek azonnal, az alapértelmezett mappával és az összes fájllal kerülnek hozzáadásra.

Magnet hivatkozásnál az ablak akkor nyílik meg, amikor a torrent fájllistája megérkezett a többi peertől. Addig a torrent a listában „Metaadatok letöltése” állapotban vár. Várakozás közben semmit sem ír a lemezre. Ha a lista megérkezése előtt kilép a Rookparcelből, az ablak a következő indításkor visszatér.

### Mentés ide

A mappa, amelybe a torrent fájljai kerülnek. A lista tartalmazza az alapértelmezett letöltési mappát (a **Tulajdonságok > Mappák > Új letöltések helye:** beállításból) és az utolsó tíz használt mappát. Írjon be egy útvonalat, vagy kattintson a **...** gombra egy kiválasztásához. A még nem létező mappa létrejön. Az adott meghajtó szabad helye az alábbi méret után látható.

**Név:** a torrent mappájának, vagy egyetlen fájljának a neve. A módosítása átnevezi a mappát vagy a fájlt a lemezen.

### A torrent beállításai

| Beállítás | Mit csinál |
|---|---|
| **Hash-ellenőrzés kihagyása** | A mappában már meglévő fájlokat késznek veszi, ellenőrzés nélkül. Csak akkor használja, ha biztosan tudja, hogy az adatok teljesek és helyesek, például miután egy másik számítógépről másolta őket. Magnet hivatkozásnál szürke: a fájljait már ellenőrizték, amikor a fájllista megérkezett. |
| **Torrent indítása** | Bekapcsolva: a torrent bekerül a sorba, és a sor engedélyének megfelelően indul. Kikapcsolva: leállítva kerül hozzáadásra. Alapból be van jelölve, kivéve ha a Tulajdonságok > Felület oldalon a **Ne indítsa el automatikusan a letöltést** be van kapcsolva. |
| **A sor elejére** | A torrentet a sor első helyére teszi, így a többi előtt indul. |
| **Címke:** | Címkét ad a torrentnek. Válasszon egy használatban lévőt, vagy írjon be újat. Lásd: [Torrentek kezelése](Torrentek-kezelése#címkék). |
| **Letöltés sorrendben** | A részeket sorrendben tölti le, az adatok elejétől a végéig, így egy videó lejátszható közben. Csak arra használja, amit előnézni szeretne: a szokásos legritkábbat először sorrend jobb a többi peernek. |
| **Első és utolsó részek letöltése előbb** | Az egyes kívánt fájlok első és utolsó részeit a többi előtt tölti le, hogy a lejátszó korán olvashassa a videó fejlécét és indexét. |

*A két letöltési sorrend jelölőnégyzet az 1.2.63-ban jelent meg.* Mindkettő minden torrentnél bejelöletlenül indul; a Rookparcel nem jegyzi meg őket. A hozzáadás utáni módosításhoz használja a jobb gombos menü **Haladó** elemét vagy a Jellemzőket. Lásd: [Torrentek kezelése](Torrentek-kezelése#letöltési-sorrend).

### A torrent tartalma

A felső sorok a torrent **Név:**, **Megjegyzés:**, **Méret:** és **Dátum:** adatát mutatják. A méret megmondja, mennyit választott ki a torrentből, például „1,20 GB / 3,40 GB kijelölve”, utána a szabad lemezterülettel.

A fájllista a torrent mappáit és fájljait fában mutatja, mindegyiket jelölőnégyzettel, mérettel és prioritással.

- Egy fájl kihagyásához vegye ki a jelölést. Az a mappa, amelynek fájljai részben vannak bejelölve, „Vegyes” feliratot mutat.
- Az **Összes kijelölése** és a **Kijelölés megszüntetése** mindent bejelöl vagy kivesz.
- Egy mappa ikonjára kattintva, vagy rajta duplán kattintva kinyithatja vagy bezárhatja. A <kbd>Right</kbd> és a <kbd>Left</kbd> (vagy a <kbd>+</kbd> és a <kbd>-</kbd> a számbillentyűzeten) ugyanezt teszi; a <kbd>Space</kbd> a kijelölt sorokat bejelöli vagy a jelölést kiveszi.
- A kijelölt sorokon jobb gombbal kattintva megjelenik a **Magas prioritás**, **Normál prioritás**, **Alacsony prioritás**, **Ne töltse le**, **Bejelölés** és **Jelölés törlése**. A prioritás kiválasztása bejelöli a fájlokat; a **Ne töltse le** kiveszi a jelölést.

Azok a fájlok, amelyek neve megegyezik a **Ne töltse le az ilyen nevű fájlokat:** beállítással (Tulajdonságok > Felület), itt bejelöletlenül indulnak. Jelölje be őket, ha mégis kellenek: amit ebben az ablakban kiválaszt, az mindig előnyt élvez. Lásd: [Név szerint kihagyott fájlok](#név-szerint-kihagyott-fájlok).

A fájlok kiválasztását és prioritásait később a [Fájlok lapon](Torrent-részletek-és-fájlok#fájlok) módosíthatja.

### OK, Mégse és Ne mutassa újra

Az **OK** a választásaival együtt hozzáadja a torrentet. A **Mégse** semmit sem ad hozzá; magnet hivatkozásnál eltávolítja a várakozó torrentet. A **Ne mutassa újra** bejelölése kikapcsolja az **Ablak megjelenítése a torrentben lévő fájlokkal** beállítást, így a későbbi torrentek ez az ablak nélkül kerülnek hozzáadásra. A Tulajdonságok > Felület oldalon kapcsolhatja vissza.

A Rookparcel megjegyzi az ablak méretét.

## Torrentek hozzáadása leállítva

Ha úgy szeretne torrenteket hozzáadni, hogy semmi se töltődjön le, amíg Ön nem mondja, kapcsolja be a **Ne indítsa el automatikusan a letöltést** beállítást a **Beállítások > Tulajdonságok > Felület** oldalon. Az új torrentek leállítva és a soron kívül kerülnek hozzáadásra. Indítsa el őket az **Indítás** paranccsal, amikor készen áll. Az Új torrent hozzáadása ablakban ez úgy látszik, hogy a **Torrent indítása** nincs bejelölve.

## Név szerint kihagyott fájlok

*Új az 1.2.63-ban.* Egyes torrentek olyan fájlokat is tartalmaznak, amelyekre soha nincs szüksége, például parancsikonokat, bélyegkép-gyorsítótárakat vagy minta-videókat. A Rookparcel név szerint kihagyhatja őket:

1. Nyissa meg a **Beállítások > Tulajdonságok > Felület** oldalt.
2. A **Torrentek hozzáadásakor** alatt jelölje be a **Ne töltse le az ilyen nevű fájlokat:** beállítást.
3. Az alatta lévő mezőbe írja be a neveket, pontosvesszővel elválasztva. A `*` tetszőleges karaktereket, a `?` egyet jelent, és a kis- és nagybetű nem számít. Például: `*.lnk; Thumbs.db; *sample*`.
4. Kattintson az **OK** gombra.

Csak a fájl saját nevét hasonlítja össze, nem azokat a mappákat, amelyekben van. A lista a mostantól hozzáadott torrentekre vonatkozik:

- Az Új torrent hozzáadása ablakban az egyező fájlok bejelöletlenül indulnak, és visszajelölheti őket.
- Az ablak nélkül hozzáadott torrenteknél az egyező fájlok **Ne töltse le** állapotot kapnak: kikapcsolt ablakkal, a [figyelt mappából](#figyelt-mappa), az RSS-ből, hivatkozásból, a webes felületről, és olyan alkalmazásokból, mint a Sonarr és a Radarr. Magnet hivatkozásnál ez akkor történik meg, amikor a fájllista megérkezett.
- Az a torrent, amelynek minden fájlja egyezik, az összes fájljával kerül hozzáadásra, mintha a lista üres lenne.
- A már a listában lévő torrenteket, az induláskor visszaállított torrenteket és az importált torrenteket nem módosítja.

Az az ablak nélkül hozzáadott magnet hivatkozás, amelynek fájllistája csak a Rookparcel újraindítása után érkezik meg, minden fájlt letölt, és így tesz az a torrent is, amelyet a `rookparcel-nox`-nak a parancssorában adtak át (`add`, vagy a `run` fájllal vagy hivatkozással). Egy kihagyott fájl későbbi megszerzéséhez adjon neki prioritást a [Fájlok lapon](Torrent-részletek-és-fájlok#fájlok).

## A listában már szereplő torrent

Ha olyan torrentet ad hozzá, amely már a listában van, a Rookparcel ezt mondja: „... már a torrentlistában van.”, és nem ad hozzá semmit.

## Figyelt mappa

A Rookparcel figyelhet egy mappát, és hozzáadhat minden oda tett új `.torrent` fájlt:

1. Nyissa meg a **Beállítások > Tulajdonságok > Mappák** oldalt.
2. Jelölje be a **.torrent fájlok automatikus betöltése innen:** beállítást, és válassza ki a mappát.
3. Ha kívánja, jelölje be a **Betöltött .torrent fájlok törlése** beállítást.
4. Kattintson az **OK** gombra.

A Rookparcel az új fájlt néhány másodperccel azután adja hozzá, hogy az már nem változik, az Új torrent hozzáadása ablak nélkül. A betöltött fájl `.torrent.loaded` nevet kap, vagy törlődik, ha a **Betöltött .torrent fájlok törlése** be van kapcsolva. A problémák a [Napló lapra](Torrent-részletek-és-fájlok#napló) íródnak.

Ugyanezen az oldalon másolatot is tarthat minden hozzáadott `.torrent` fájlról (**.torrent fájlok tárolása itt:**). Lásd: [Lemez és tárolás](Lemez-és-tárolás) és [A beállítások leírása](A-beállítások-leírása).

## A GTK-s változatban (Linux, BSD)

- Ugyanazok a menüelemek, billentyűparancsok és ugyanaz az Új torrent hozzáadása ablak.
- A **Torrent hozzáadása URL-ről** ugyanúgy kitölti a vágólapról a hivatkozást. Az a szöveg, amely nem `magnet:` hivatkozás vagy `http(s)://` cím, ezt adja: „Nem olyan hivatkozás, amelyet a Rookparcel hozzá tud adni”.
- A GTK-s ablak nem fogad ejtett fájlokat. Használja a **Fájl > Torrent hozzáadása...** parancsot, vagy nyissa meg a fájlt a Rookparcellel a fájlkezelőből.
- Linuxon és a BSD-ken a fájltársítások a Tulajdonságok > Általános oldalon az **Asztali integráció** alatt vannak. Flatpakban ehelyett az asztali környezet beállításaiban válassza ki az alapértelmezett programot. Lásd: [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás).

## A webes felületen

- A **Torrent hozzáadása** feltölt egy vagy több `.torrent` fájlt arról a számítógépről, amelyen a böngésző fut; az **URL hozzáadása** magnet hivatkozást vagy címet kér. A puszta infohash-t is magnet hivatkozásnak veszi.
- Ejthet `.torrent` fájlokat vagy magnet hivatkozást az oldalra, vagy beillesztheti a magnet hivatkozást bárhol a szövegmezőkön kívül.
- Ugyanez az Új torrent hozzáadása ablak nyílik meg az oldalon. A **Mentés ide** a kiszolgáló egy mappája, útvonalként beírva.
- Telefonon az **URL hozzáadása** a **Továbbiak** menüben (⋯) van.

Lásd: [Webes felület](Webes-felület). Egy a **Fájl > Csatlakozás távoli géphez...** paranccsal csatlakoztatott ablakban az Új torrent hozzáadása **...** gombja szürke: írjon be egy mappát a kiszolgálón. Lásd: [Távvezérlés](Távvezérlés).

## Kapcsolódó oldalak

- [A főablak](A-főablak)
- [Torrentek kezelése](Torrentek-kezelése)
- [Importálás µTorrentből és qBittorrentből](Importálás-uTorrentből-és-qBittorrentből)
- [RSS letöltő](RSS-letöltő)
