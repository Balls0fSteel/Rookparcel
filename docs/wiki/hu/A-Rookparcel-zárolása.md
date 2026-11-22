*Ez a [Locking Rookparcel](Locking-Rookparcel) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# A Rookparcel zárolása

Ez az oldal azt mutatja be, hogyan zárolhatja a Rookparcel ablakait PIN-kóddal, hogy más a számítógépénél ülve ne lássa a torrentlistáját, és hogyan távolíthat el egy elfelejtett PIN-kódot. *Az 1.2.54-ben jelent meg.*

A zárolás az adatvédelmet szolgálja, nem a biztonságot. Elrejti az ablakokat, de semmit sem titkosít. Aki szerkesztheti a Rookparcel beállításfájlját, vagy megnyithatja a letöltési mappáját, megkerülheti. Ne használja hozzá a bankkártyája PIN-kódját.

## PIN-kód beállítása és zárolás

1. Nyissa meg a **Beállítások > Tulajdonságok** ablakot, és válassza a **Haladó > Felület zárolása** oldalt (Macen: **Rookparcel > Tulajdonságok**, majd **Felület zárolása**).
2. Kattintson a **PIN-kód beállítása...** gombra.
3. Írjon be egy 4–6 számjegyű PIN-kódot az **Új PIN-kód (4–6 számjegy)** mezőbe, és még egyszer az **Az új PIN-kód még egyszer** mezőbe.
4. Kattintson az **OK** gombra. A PIN-kód azonnal mentésre kerül; a Tulajdonságok ablakban nem kell az **OK** gombra kattintania.

*Az 1.2.63 előtt a PIN-kódot a **Beállítások > Lock PIN** menüpontban lehetett beállítani.*

A zároláshoz válassza a **Fájl > Rookparcel zárolása** menüpontot, vagy nyomja meg a <kbd>Ctrl</kbd>+<kbd>L</kbd> billentyűket. Ha be van állítva PIN-kód, a tálcamenüben is van **Rookparcel zárolása** menüpont. Ha még nincs PIN-kód, a **Rookparcel zárolása** előbb azt kéri be, és csak utána zárol.

A zárolás a tálcára rejti a Rookparcel minden ablakát: a főablakot és minden nyitott párbeszédablakot. A torrentjei tovább töltődnek és megosztódnak.

A PIN-kód módosításához kattintson ugyanezen az oldalon a **PIN-kód módosítása...** gombra, írja be a jelenlegi PIN-kódot, majd az újat kétszer. A **PIN-kód eltávolítása...** a jelenlegi PIN-kódot kéri, és eltávolítja; ezután a **Rookparcel zárolása** új PIN-kódot kér, mielőtt zárolna.

## A zárolás kikapcsolása

Ha soha nem szeretne zárolást, a **Felület zárolása** oldalon törölje a jelölést ebből: **A Rookparcel zárolásának engedélyezése PIN-kóddal**. Ha be van állítva PIN-kód, a Rookparcel előbb azt kéri, aztán eltávolítja. Amíg a zárolás ki van kapcsolva, nincs **Fájl > Rookparcel zárolása** menüpont, sem billentyűparancs, és a tálcamenü **Rookparcel zárolása** pontja is hiányzik. A jelölőnégyzetet újra bejelölve visszatérnek.

## Zárolt Rookparcel megnyitása

Kattintson a tálcaikonra, válassza a tálcamenü **Rookparcel megjelenítése** pontját, vagy indítsa újra a Rookparcelt. Egy ablak kéri a PIN-kódot; a címe **A Rookparcel zárolva van**. A helyes PIN-kóddal a korábban nyitott ablakok úgy jönnek vissza, ahogy voltak. A **Mégse** a Rookparcelt zárolva hagyja a tálcán.

Az ötödik egymás utáni hibás PIN-kód után a Rookparcel 30 másodpercet vár, mielőtt újabbat elfogad, és utána minden hibás PIN-kód ugyanezt váltja ki. A helyes PIN-kód véget vet ennek.

A Rookparcel zárolva marad, amikor kilép belőle. A következő indításkor azonnal a PIN-kódot kéri, a főablakot meg sem mutatva. Ha bejelentkezéskor indul az **Indítás kis méretben** beállítással, kérdezés nélkül a tálcán vár.

## Amíg a Rookparcel zárolva van

- A tálcamenü csak a **Rookparcel megjelenítése** és a **Kilépés** pontot kínálja. A **Kilépés** PIN-kód nélkül is működik.
- Az „A letöltés kész” értesítés nem jelenik meg, mert megnevezi a torrentet.
- Azok az üzenetek, amelyek maguktól nyílnának meg, a feloldásig várnak: a hibaüzenetek, a frissítési ajánlat és annak a magnet hivatkozásnak az **Új torrent hozzáadása** ablaka, amelynek a fájljai megérkeztek.
- Egy zárolás alatt megnyitott `.torrent` fájl vagy magnet hivatkozás előbb a PIN-kódot kéri. Ha a **Mégse** gombra kattint, nem kerül hozzáadásra.
- Az **Automatikus leállítás** visszaszámlálása továbbra is megjelenik, így megállíthatja.
- A [webes felület](Webes-felület) nincs zárolva. Saját felhasználóneve és jelszava van. Ha a **Nincs hitelesítés a localhost klienseinek** be van kapcsolva, ugyanazon a számítógépen egy böngésző továbbra is láthatja a torrentjeit.

## Elfelejtett PIN-kód

Az elfelejtett PIN-kódot a Rookparcel indításakor távolíthatja el. A Rookparcel azt kérdezi: **Eltávolítja a zárolási PIN-kódot? A Rookparcel ezután kérés nélkül megnyílik.** Válaszoljon **Igen**nel (Linuxon **OK**, macOS-en **Eltávolítás**), és a Rookparcel PIN-kód nélkül megnyílik. A torrentjei és a többi beállítása változatlan marad.

**Windows:** tartsa lenyomva a <kbd>Ctrl</kbd> billentyűt a Rookparcel indítása közben, például amikor a Start menü elemére vagy az asztali ikonjára kattint. Ha a Rookparcel már fut, a futó példány kérdez.

**macOS:** lépjen ki a Rookparcelből (a menüsori ikon menüjében **Kilépés a Rookparcel alkalmazásból**), majd tartsa lenyomva az <kbd>⌥</kbd> (Option) billentyűt a megnyitása közben. Amíg a Rookparcel fut, az <kbd>⌥</kbd> lenyomva tartása a Dock-ikonjára kattintáskor szintén kérdez.

**Linux és a BSD-k:** indítsa terminálból a `--reset-lock` kapcsolóval. Ha a Rookparcel már fut, a futó példány kérdez.

| Példány | Parancs |
|---|---|
| Sima bináris fájlok (tar.gz) | `rookparcel-gtk --reset-lock` |
| AppImage | `./Rookparcel-<version>-x86_64.AppImage --reset-lock` |
| Flatpak | `flatpak run io.github.balls0fsteel.Rookparcel --reset-lock` |

A `--reset-lock` Windowson és macOS-en is működik: `rookparcel.exe --reset-lock`, Macen pedig `/Applications/Rookparcel.app/Contents/MacOS/Rookparcel --reset-lock`. Ha a Rookparcel már fut a Macen, a kérdése az előtérben lévő ablak mögött is megnyílhat: keresse meg.

[Profil](Profilok) esetén adja hozzá a kapcsolóját is: `rookparcel.exe --profile NAME --reset-lock`.

**Kézzel:** lépjen ki a Rookparcelből, nyissa meg a `rookparcel.ini` fájlját egy szövegszerkesztőben, törölje a `ui.lock_pin=` és a `ui.locked=` kezdetű sorokat, és mentse. A [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait) oldal megmondja, hol van a fájl.

## Profilok és távoli ablakok

Minden [profilnak](Profilok) saját PIN-kódja van, vagy nincs egy sem. Ugyanez igaz minden olyan ablakra, amely egy távoli Rookparcelhez csatlakozik ([Távvezérlés](Távvezérlés)), mert annak saját beállításai vannak.

## GTK-s változat (Linux, BSD)

A zárolás úgy működik, mint Windowson. A <kbd>Ctrl</kbd>+<kbd>L</kbd> a Rookparcel bármelyik ablakából zárol, nem csak a főablakból: a Tulajdonságok, a Jellemzők, az Új torrent létrehozása, az Új torrent hozzáadása, az RSS letöltő, a Keresési ablakok és a többi párbeszédablak (a PIN-ablakok kivételével) közül. *Az 1.2.76-ban jelent meg; korábban nem csinált semmit, amíg valamelyik ilyen párbeszédablak nyitva volt.* Ha a tálca eltűnik, miközben a Rookparcel zárolva van (például újraindul a panel), megjelenik a zárolási ablak (címe: **A Rookparcel zárolva van**), így az ablakok nem maradnak elrejtve visszaút nélkül. *Az 1.2.76-ban jelent meg.*

Azokon az asztali környezeteken, amelyeknek nincs tálcájuk (az AppIndicator bővítmény nélküli sima GNOME), nincs hová elrejteni az ablakokat, ezért a zárolási ablak (címe: **A Rookparcel zárolva van**) lép a helyükre. Nincs **Mégse** gombja, és a bezárása kilépteti a Rookparcelt, ahogy a főablak bezárása is ott.

## macOS

A **Fájl > Rookparcel zárolása** a <kbd>⌘</kbd>+<kbd>L</kbd>. Amíg a Rookparcel zárolva van:

- a menüsorban a **Rookparcel** menü csak a **Rookparcel megjelenítése**, a **Rookparcel elrejtése** és a **Kilépés a Rookparcel alkalmazásból** pontot tartalmazza, valamint ott van a **Szerkesztés** menü;
- a menüsori ikon menüje a **Rookparcel megjelenítése** és a **Kilépés a Rookparcel alkalmazásból** pontot kínálja, a Dock-ikon menüje pedig a **Rookparcel megjelenítése** pontot;
- amíg a PIN-ablak nyitva van, a **Kilépés a Rookparcel alkalmazásból** szürke: előbb kattintson a **Mégse** gombra.
