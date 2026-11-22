*Ez a [Tray, notifications and startup](Tray-notifications-and-startup) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Tálca, értesítések és indítás

Ez az oldal a Rookparcel tálcaikonjáról, a tálcára zárásról és kis méretre állításról, a letöltés befejezésekor megjelenő értesítésről, a Rookparcel bejelentkezéskori indításáról, valamint arról szól, hogyan lehet a Rookparcel a `.torrent` fájlokat és a `magnet:` hivatkozásokat megnyitó program.

## A tálcaikon (Windows)

Amíg a Rookparcel fut, az ikonja a tálca értesítési területén van. Ha az egeret rávisszük, megjelenik a verzió, valamint az aktuális letöltési és feltöltési sebesség.

- A bal gombbal egyszer (vagy duplán) kattintva elrejthető a főablak, illetve újra előhozható.
- A jobb gombos kattintás megnyitja a tálcamenüt:

| Menüpont | Mit csinál |
|---|---|
| **Rookparcel elrejtése** / **Rookparcel megjelenítése** | Elrejti a főablakot, vagy újra megjeleníti |
| **Torrent hozzáadása...** | Megnyit egy `.torrent` fájlt, mint a **Fájl > Torrent hozzáadása...** |
| **Torrent hozzáadása URL-ről...** | Hozzáad egy magnet hivatkozást vagy webcímet |
| **Minden torrent indítása** | Elindítja a lista minden torrentjét |
| **Minden torrent leállítása** | Leállítja a lista minden torrentjét |
| **Rookparcel zárolása** | Elrejti a Rookparcel minden ablakát, amíg be nem írják a zárolási PIN-kódot; akkor jelenik meg, ha be van állítva PIN-kód |
| **Kilépés** | Kilép a Rookparcelből |

Amíg a Rookparcel zárolva van, a menüben csak a **Rookparcel megjelenítése** (ez a PIN-kódot kéri) és a **Kilépés** van. Lásd [A Rookparcel zárolása](A-Rookparcel-zárolása).

## Bezárás és kis méretre állítás a tálcára

Két beállítás dönti el, mi történjen az ablakkal:

| Beállítás | Alapértelmezés | Mit csinál |
|---|---|---|
| **Bezáráskor a tálcára** | Be | Az ablak bezárógombja (és az <kbd>Alt</kbd>+<kbd>F4</kbd>) kilépés helyett a tálcára rejti a Rookparcelt, így az átvitelek folytatódnak. |
| **Kis méretnél a tálcára** | Ki | A kis méretre állítás a tálcára rejti az ablakot a tálca gombja helyett. |

Állítsa be őket a **Beállítások > Tulajdonságok > Felület** oldalon az **Értesítési terület** csoportban, vagy jelölje be közvetlenül a **Beállítások** menüben (**Bezáráskor a tálcára** és **Kis méretnél a tálcára**).

Ha ki szeretne lépni a Rookparcelből, miközben a **Bezáráskor a tálcára** be van kapcsolva, használja a **Fájl > Kilépés** menüpontot vagy a tálcamenü **Kilépés** pontját. A Rookparcel ekkor leállítja a torrentjeit, és elmenti a haladásukat. Ha ez egy pillanatnál tovább tart (például mert a fájlok még egy másik meghajtóra költöznek), egy ablak megmutatja, mire vár, és van benne egy **Kilépés most** gomb.

A webes felület, amelyet a Windowsos ablak szolgáltat, csak a Rookparcel futása alatt működik, ezért a tálcára zárás elérhetőn tartja. Lásd [Webes felület](Webes-felület).

## Értesítés a letöltés befejezéséről

Ha a **Buborékértesítések a tálcán** be van kapcsolva (alapértelmezés), a Rookparcel minden letöltés befejezésekor megjelenít egy „A letöltés kész” értesítést a torrent nevével. A **Beállítások > Tulajdonságok > Felület** oldalon kapcsolhatja ki. Amíg a Rookparcel [zárolva](A-Rookparcel-zárolása) van, az értesítés nem jelenik meg.

Ha frissítés érhető el, azt a Rookparcel ehelyett az állapotsorban jelzi; lásd [A Rookparcel frissítése](A-Rookparcel-frissítése).

## A sebesség a címsorban

Az **Aktuális sebesség megjelenítése a címsorban** (Tulajdonságok > Felület, alapból ki van kapcsolva) az ablak címének elejére írja a letöltési és feltöltési sebességet, így a tálca gombján is olvasható.

## A Rookparcel indítása bejelentkezéskor

1. Nyissa meg a **Beállítások > Tulajdonságok > Általános** oldalt.
2. Jelölje be a **Rookparcel indítása a rendszerrel** négyzetet.
3. Ha azt szeretné, hogy az ablak megjelenítése nélkül, a tálcán induljon, jelölje be az **Indítás kis méretben** négyzetet is.
4. Kattintson az **OK** gombra.

Windowson ez egy bejegyzés a felhasználói fiókja indítási programjai között. Nem kell hozzá rendszergazdai jog, és hordozható példány is használhatja. Ha a hordozható példányt másik mappába helyezi át, jelölje be újra a négyzetet: addig van bejelölve, amíg a bejegyzés pontosan ezt a példányt indítja.

[Profilok](Profilok) esetén minden profilnak saját indítási bejegyzése van, így bejelentkezéskor többet is elindíthat. *A profilonkénti bejegyzések az 1.1.46-ban jelentek meg.*

A [Beállítási útmutató](Első-indítás) az utolsó oldalán ugyanezt a lehetőséget kínálja.

## .torrent fájlok és magnet hivatkozások megnyitása a Rookparcellel

1. Nyissa meg a **Beállítások > Tulajdonságok > Általános** oldalt.
2. Jelölje be a **Társítás a .torrent fájlokkal** és a **Társítás a magnet: hivatkozásokkal** négyzetet.
3. Kattintson az **OK** gombra.

Ezután egy `.torrent` fájlra duplán kattintva, vagy egy magnet hivatkozásra a böngészőben kattintva a Rookparcel kapja meg. Ha a Rookparcel már fut, a futó példány veszi át. Ha az **Ablak megjelenítése a torrentben lévő fájlokkal** be van kapcsolva, megnyílik az Új torrent hozzáadása ablak; ha **A program ablakának aktiválása** be van kapcsolva, a főablak is előtérbe kerül. Lásd [Torrentek hozzáadása](Torrentek-hozzáadása).

Tudnivalók Windowson:

- A beállítások csak a saját felhasználói fiókjára vonatkoznak, és nem kell hozzájuk rendszergazdai jog. A telepítő is beállíthatja őket.
- Ha magában a Windowsban választott másik programot („Mindig ezt az alkalmazást használja”, vagy Beállítások > Alapértelmezett alkalmazások), a Windows megtartja ezt a választást, és csak Ön változtathatja meg ott. A Rookparcel ilyenkor megmondja, melyik programot használja még a Windows, például „A Windows még mindig ezzel nyitja meg a .torrent fájlokat: ...”. Kattintson az **Alapértelmezett alkalmazások...** gombra, hogy megnyissa a megfelelő helyet a Windows beállításaiban, és válassza ki a Rookparcelt.
- Profilok esetén csak egy profil nyithatja meg a `.torrent` fájlokat és a magnet hivatkozásokat. Ha egy profilban bejelöli a négyzeteket, átveszi őket a másiktól; az oldal megnevezi azt a profilt, amelyiknél most vannak.
- A **Társítás ellenőrzése indításkor** (alapból ki van kapcsolva) hatására a Rookparcel minden indításkor megkérdezi, visszavegye-e a társításokat, ha egy másik program birtokolja őket. Nem kérdez, ha a Rookparcel bejelentkezéskor kis méretben indul.

## GTK-s változat (Linux, BSD)

**A tálca.** A Rookparcel tálcaikont mutat azokon az asztali környezeteken, amelyek tartanak ilyet: KDE Plasma, Xfce, LXQt, Cinnamon, MATE, waybar és az AppIndicator bővítménnyel ellátott GNOME. A menüje ugyanazokat a pontokat tartalmazza, mint Windowson. Ha a tálca eltűnik (például újraindul a panel), miközben a Rookparcel el van rejtve benne, a Rookparcel megmutatja a főablakot, zárolt állapotban pedig a PIN-ablakot, hogy semmi ne maradjon visszaút nélkül. *Az 1.2.76-ban jelent meg.* A bővítmény nélküli sima GNOME-on nincs tálca: a tálcával kapcsolatos négyzetek nem jelennek meg a Tulajdonságok ablakban, és az ablak bezárása kilépteti a Rookparcelt. A <kbd>Ctrl</kbd>+<kbd>Q</kbd> az ablakból kilép.

A **Kis méretnél a tálcára** működéséhez az asztali környezetnek jeleznie kell, hogy az ablakot kis méretre állították; ahol ezt nem teszi (néhány Wayland-es asztalon), a kis méretre állítás sima kis méretre állítás marad.

Az **értesítések** az asztali környezet saját értesítési szolgáltatásán át érkeznek, ugyanazzal az „A letöltés kész” szöveggel.

**Indítás bejelentkezéskor.** A **Rookparcel indítása a rendszerrel** egy automatikus indítási bejegyzést ír a felhasználó `~/.config/autostart` mappájába. Flatpakban ehelyett a háttérengedélyén át az asztali környezetet kéri meg, és az asztali környezet megkérdezheti, engedélyezi-e; ott csak az alapértelmezett profil indulhat bejelentkezéskor.

**Társítások.** A négyzetek a Rookparcelt teszik meg a `.torrent` fájlok és a `magnet:` hivatkozások alapértelmezett programjává a felhasználó beállításaiban. Egy négyzet jelölésének törlése visszaadja a típust a rendszer alapértelmezettjének; nem állítja vissza a korábban használt programot. Ha áthelyez egy AppImage-et, jelölje be újra a négyzeteket. Flatpakban a négyzetek szürkék: az asztali környezet már felajánlja a Rookparcelt mindkét típushoz, az alapértelmezett programot pedig az asztali környezet beállításaiban választja ki.

## macOS

**A menüsor.** A Rookparcel az ikonját a menüsorba teszi, ahol a Windowson a tálca van. Ha az egeret rávisszük, megjelennek a sebességek. Egy kattintás elrejti az ablakot vagy előhozza; a jobb gombos vagy Control-kattintás megnyitja a menüt: **Rookparcel elrejtése** / **Rookparcel megjelenítése**, **Rookparcel zárolása** (ha be van állítva zárolási PIN-kód), **Torrent hozzáadása...**, **Torrent hozzáadása URL-ről...**, **Minden torrent indítása**, **Minden torrent leállítása** és **Kilépés a Rookparcel alkalmazásból**. A Dock-ikon menüjében ugyanezek a pontok vannak, a **Kilépés a Rookparcel alkalmazásból** nélkül, amelyet a Dock maga ad hozzá. Amíg a Rookparcel [zárolva](A-Rookparcel-zárolása) van, a menük csak a **Rookparcel megjelenítése** pontot kínálják (a menüsori ikonén a **Kilépés a Rookparcel alkalmazásból** is ott van).

**Az ablak bezárása.** Ha a **Bezáráskor a tálcára** be van kapcsolva (alapértelmezés), az ablak bezárógombja (<kbd>⌘</kbd>+<kbd>W</kbd>) elrejti az ablakot, és a Rookparcel tovább fut. A Dock-ikonra vagy a menüsori ikonra kattintva hozhatja vissza. A kilépéshez válassza a **Rookparcel > Kilépés a Rookparcel alkalmazásból** menüpontot (<kbd>⌘</kbd>+<kbd>Q</kbd>), vagy a menüsori ikon menüjének **Kilépés a Rookparcel alkalmazásból** pontját. A **Kis méretnél a tálcára** ugyanígy elrejti a kis méretre állított ablakot.

A letöltés befejezéséről szóló **értesítések** macOS-értesítések, ugyanazzal az „A letöltés kész” szöveggel. A macOS egyszer megkérdezheti, mutathat-e a Rookparcel értesítéseket; ha egy sem jelenik meg, engedélyezze a Rookparcelt itt: **Rendszerbeállítások > Értesítések**.

**Indítás bejelentkezéskor.** A **Rookparcel indítása a rendszerrel** egy bejelentkezési elemet vesz fel ehhez a profilhoz (egy indítási ügynököt a `~/Library/LaunchAgents` mappában); az **Indítás kis méretben** rejtve indítja. A macOS 13 és újabb verzióiban a **Rendszerbeállítások > Általános > Bejelentkezési elemek** felsorolja. Az alkalmazás régebbi, GTK-s verziója által létrehozott bejelentkezési elem továbbra is működik.

**Társítások.** A négyzetek csak akkor működnek, ha a Rookparcel `Rookparcel.app`-ként fut, és csak az alapértelmezett profilban. Egy társítás kikapcsolása a Finder **Get Info > Open with** helyén történik.

A macOS-es Tulajdonságok ablaknak nincs **OK** gombja: minden módosítás azonnal érvénybe lép.

*A natív macOS-es alkalmazás az 1.2.19-ben jelent meg.*

## Webes felület és rookparcel-nox

A webes felületnek nincs tálcája, értesítései vagy indítási beállításai: a Tulajdonságok ablakában a társítás és az automatikus indítás négyzetei szürkék, azzal a megjegyzéssel, hogy az asztali klienshez tartoznak. A `rookparcel-nox` számítógéppel együtt indításához futtassa szolgáltatásként; lásd [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).
