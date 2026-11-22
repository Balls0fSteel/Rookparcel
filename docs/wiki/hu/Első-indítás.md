*Ez az angol nyelvű [First start and the Setup Guide](First-start-and-the-Setup-Guide) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Első indítás és a beállítási útmutató

Ez az oldal azt írja le, mi történik a Rookparcel első indításakor: a beállítási útmutatót, egy rövid, négy kérdéses ablakot, azt, hogy az egyes válaszok mit állítanak be, és hogyan nyithatja meg újra az útmutatót később.

*Új az 1.1.62-ben.* Az 1.1.40-ig nincs útmutató: a Rookparcel az alapértelmezett beállításaival nyílik meg, és minden, amit alább leírunk, a **Beállítások > Tulajdonságok** oldalon állítható.

## Mikor nyílik meg az útmutató

A beállítási útmutató a Rookparcel első indításakor magától megnyílik: amikor a beállításfájlban még nincs semmilyen beállítás. Ez egy új telepítésnél, egy új hordozható mappánál és minden új [profilnál](Profilok) így van. A főablak előtt nyílik meg, így semmi sem kezd letöltődni, mielőtt válaszolt volna. Egy meglévő, de nem olvasható beállításfájl nem számít első indításnak.

Magától nem nyílik meg annak, aki már használt egy régebbi verziót: a beállítások megvannak. Ilyenkor a menüből nyissa meg (lásd alább).

![A beállítási útmutató első oldala](images/setup-guide.png)

## Mozgás az útmutatóban

Minden oldalon van egy cím, fölötte az „1/4. lépés” felirat, egy rövid szöveg és a hozzá tartozó beállítások. A gombok:

| Gomb | Mit csinál |
|---|---|
| **< Vissza** | az előző oldal |
| **Tovább >** | a következő oldal |
| **Befejezés** | elmenti a válaszait, és bezárja az útmutatót |
| **Kihagyás** | bezárja az útmutatót, és a beállításokat úgy hagyja, ahogy vannak. Az <kbd>Esc</kbd> és az ablak bezárás gombja ugyanezt teszi |

A **Befejezés** gombig semmi sem mentődik el, és a **Befejezés** csak azokat a válaszokat módosítja, amelyeket Ön megváltoztatott. Ha az egeret egy beállítás fölé viszi, megjelenik a súgója; a súgó a beállítások alatt is látszik.

A **Befejezés** vagy a **Kihagyás** után az útmutató magától többé nem nyílik meg.

## 1. oldal: Hová kerülnek a letöltések

Válassza ki azt a mappát, ahová az új letöltések mentődnek.

| Beállítás | Jelentése |
|---|---|
| **Új letöltések helye:** | az új letöltések mappája. Alapértelmezés szerint a Letöltések mappa; hordozható mappában a benne lévő `Downloads` mappa |
| **Kész letöltések áthelyezése ide:** | ha be van jelölve, a befejezett letöltések maguktól átkerülnek a mellette megadott mappába, például egy gyors lemezről egy nagyra |

A **Beállítások > Tulajdonságok > Mappák** oldalon további mappabeállítások vannak. Lásd: [Lemez és tárolás](Lemez-és-tárolás).

## 2. oldal: Az Ön routere

A peerekkel létesített minden kapcsolat egy helyet foglal a router kapcsolati táblájában. A régebbi routerek és az internetszolgáltatók alapdobozai kis táblával rendelkeznek; ha az megtelik, a router eldobja a kapcsolatokat, lelassítja az egész háztartás hálózatát, vagy újraindul.

**Milyen routere van?**

- **Modern (kb. 2015 után vásárolt), vagy nem tudom:** a Rookparcel szokásos kapcsolatszámai.
- **Régebbi, vagy az internetszolgáltató alapdoboza:** kevesebb kapcsolat: összesen 100, torrentenként 40, és másodpercenként 8 új.

A régebbi routerre vonatkozó válasz kiindulópont. Ezeket a számokat később módosíthatja a **Beállítások > Tulajdonságok > Sávszélesség** és a **Haladó** oldalon. Ha később a modern választ adja, a szokásos számok visszakerülnek.

Ugyanezen az oldalon található az **Alacsony ping megtartása (a sebességek automatikus korlátozása)** beállítás. Jelölje be, ha játszik vagy hívásokat bonyolít, miközben a Rookparcel fut: lent tartja a sebességeket, amíg a vonala foglalt. Lásd: [Alacsony ping](Alacsony-ping).

## 3. oldal: Az Ön lemezei

Az **Okos lemez I/O használata** reszponzívan tartja a számítógépet, miközben a Rookparcel olvas és ír:

- az ellenőrzések a rendszer háttérlemez-prioritásával futnak;
- minden meghajtó a típusához van hangolva (SSD, merevlemez, hálózati megosztás);
- az átvitelek lelassulnak, amíg egy lemez lassan válaszol.

A kíméletes ellenőrzések és a meghajtónkénti hangolás kezdettől be van kapcsolva. A jelölőnégyzet a lassítást adja hozzá, amíg egy lemez foglalt. Az ellenőrzések tovább tartanak, amíg valami más is használja a lemezt. A **Beállítások > Tulajdonságok > Haladó > Lemez I/O** oldal mindegyik részt külön állítja. Lásd: [Lemez és tárolás](Lemez-és-tárolás).

## 4. oldal: Ez a számítógép

A Rookparcel elindulhat, amikor bejelentkezik, és megnyithatja a `.torrent` fájlokat és a `magnet:` hivatkozásokat a böngészőből és a fájlkezelőből.

| Beállítás | Jelentése |
|---|---|
| **Rookparcel indítása a rendszerrel** | bejelentkezéskor elindítja a Rookparcelt |
| **Társítás a .torrent fájlokkal** | egy `.torrent` fájlra duplán kattintva az a Rookparcelben nyílik meg |
| **Társítás a magnet: hivatkozásokkal** | a böngészőben egy `magnet:` hivatkozásra kattintva az a Rookparcelben nyílik meg |

A **Beállítások > Tulajdonságok > Általános** oldalon ezeket később módosíthatja. Lásd: [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás).

Ez az oldal kimarad, ahol nem értelmezhető:

- **A Windows-telepítő** már rákérdezett a társításokra, ezért egy telepített példány az első indításkor három oldalt mutat. Az oldal ott van, ha az útmutatót a menüből nyitja meg.
- **Egy távoli `rookparcel-nox`-hoz csatlakozott ablak** három oldalt mutat, ha az útmutatót a menüből nyitja meg.
- **A Flatpak** csak az indítás jelölőnégyzetét mutatja: az asztali környezet dönti el, melyik program nyitja meg a `.torrent` fájlokat és a `magnet:` hivatkozásokat. Ugyanez a helyzet a macOS-en, ahol a Rookparcel nem tudja maga beállítani a társításokat (nevesített profil, vagy a `Rookparcel.app`-on kívüli puszta bináris fájl).

## Az útmutató újbóli megnyitása

Válassza a **Beállítások > Beállítási útmutató** parancsot, vagy nyomja meg a <kbd>Ctrl</kbd>+<kbd>G</kbd> billentyűkombinációt. Az útmutató a jelenlegi beállításait mutatja, és a **Befejezés** azokat a válaszokat alkalmazza, amelyeket módosított, ahogy az **OK** a Tulajdonságok ablakban.

## GTK és a webes felület

- A GTK-s változat (Linux, BSD-k) ugyanezt az útmutatót ugyanezekkel az oldalakkal mutatja minden profil első indításakor, és a **Beállítások > Beállítási útmutató** alatt (<kbd>Ctrl</kbd>+<kbd>G</kbd>).
- A macOS-es alkalmazásban is megvan, a **Rookparcel > Beállítási útmutató** alatt (<kbd>⌘</kbd>+<kbd>G</kbd>).
- A `rookparcel-nox`-nak és a webes felületnek nincs útmutatója. Állítsa be őket a webes felület Tulajdonságok oldalán: lásd: [Webes felület](Webes-felület) és [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

## Az útmutató után

Megnyílik a főablak. A következő lépések:

- [Torrentek hozzáadása](Torrentek-hozzáadása)
- [A főablak](A-főablak)
- [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal), ha a letöltések lassúak maradnak
- [A beállítások leírása](A-beállítások-leírása), az összes többi beállításhoz
