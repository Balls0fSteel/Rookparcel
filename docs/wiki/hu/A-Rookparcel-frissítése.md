*Ez az angol nyelvű [Updating Rookparcel](Updating-Rookparcel) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# A Rookparcel frissítése

A Rookparcel magától ellenőrzi az új verziókat, és szól, ha megjelent egy. Ez az oldal az ellenőrzést, egy telepített Windows-os példány önfrissítését, a zip, a Linux és a macOS példányok kézi frissítését, a béta verziókat, az ellenőrzés kikapcsolását és a `rookparcel-nox`-ot írja le.

## A frissítések keresése

A Rookparcel indítása után körülbelül tíz másodperccel, majd futás közben 12 óránként megkérdezi a projekt webhelyét, melyik a legújabb verzió. Ez egyetlen kis HTTPS-kérés, amely verziószámok rövid listáját kéri le. A számítógépéről, a torrentjeiről vagy a beállításairól semmi sem kerül elküldésre, és az Ön igenje nélkül semmi sem töltődik le vagy települ. Ha a Rookparcel proxyt használ (**Beállítások > Tulajdonságok > Kapcsolat**), az ellenőrzés is azon át megy.

Ha megjelent egy újabb verzió:

- Megjelenik egy kérdésablak, minden új verziónál egyszer.
- Ezután egy értesítés marad az állapotsorban, amíg rá nem kattint: „Elérhető a Rookparcel 1.2.76 - kattintson ide a letöltéshez”.
- A Napló lapon is van egy sor a hivatkozással.

Az azonnali ellenőrzéshez válassza a **Súgó > Frissítések keresése** parancsot (macOS-en **Rookparcel > Frissítések keresése**). Mindkét esetben válaszol: az újabb verzióval, vagy azzal, hogy „Ön a legújabb verziót használja”.

## A telepített Windows-os példány magát frissíti

A Windows-telepítővel telepített Rookparcel maga is telepítheti az új verziót. *Új az 1.1.40-ben.*

1. A kérdés így szól: „Elérhető a Rookparcel 1.2.76. Az Ön verziója: 1.2.69. Letölti és telepíti most?”. Kattintson a **Letöltés és telepítés** gombra. (A **Letöltési oldal megnyitása** a kiadási oldalt nyitja meg a böngészőben; a **Mégse** nem csinál semmit.)
2. Egy kis ablak mutatja a letöltés előrehaladását. A Rookparcel a letöltött telepítőt a használat előtt ellenőrzi a kiadás `SHA256SUMS` fájlja alapján.
3. A Rookparcel megkérdezi: „A telepítéshez a Rookparcel most bezárul, és a telepítő végeztével újraindul. Bezárja és telepíti most?”. Kattintson az **Igen** gombra.
4. A Rookparcel elmenti az összes adatot, és bezárul. A telepítő csendben telepíti az új verziót ugyanabba a mappába, és újra elindítja a Rookparcelt.

A beállításai, a torrentjei és a társítások úgy maradnak, ahogy voltak. Ha a telepítés meghiúsul, a régi verzió marad, és újra elindul.

Egy mindenkinek szóló telepítésnél (a `C:\Program Files` mappában) a Windows rendszergazdai jogot kér, amíg a telepítő fut.

**Frissítés az 1.1.17 vagy régebbi verzióról:** ezek a verziók nem tudnak frissítéseket telepíteni; a kérdésük a letöltési oldalt nyitja meg. Töltse le az új telepítőt, és futtassa egyszer kézzel, ahogy itt: [A Rookparcel telepítése](A-Rookparcel-telepítése#telepítés-régebbi-verzió-fölé). Attól kezdve a Rookparcel magát frissíti.

## Frissítés kézzel

### Windows-telepítő

Töltse le az új `rookparcel-<version>-windows-x64-setup.exe` fájlt, és futtassa. Megtalálja a telepített Rookparcelt, és a régi fölé telepít, megtartva a mappát, a módot (saját részre vagy mindenkinek), a beállításokat és a torrenteket. Előbb lépjen ki a Rookparcelből, különben a telepítő kéri. Lásd: [A Rookparcel telepítése](A-Rookparcel-telepítése#telepítés-régebbi-verzió-fölé).

### Windows zip

A zipből futó példány nem frissíti magát. A kérdése a letöltési oldal megnyitását ajánlja.

1. Töltse le az új zipet.
2. Lépjen ki a Rookparcelből (**Fájl > Kilépés**).
3. Csomagolja ki az új `rookparcel.exe` és `rookparcel-nox.exe` fájlt a régiek fölé.
4. Indítsa el a Rookparcelt.

Hagyja a `rookparcel.ini` fájlt a mappában, ha a példány hordozható. A beállítások és a torrentek nincsenek a zipben, ezért semmi sem íródik felül az Önéiből.

### Linux és macOS

A GTK-s változatban és a macOS-es alkalmazásban a kérdés a kiadási oldalt nyitja meg.

- **AppImage:** töltse le az új `.AppImage` fájlt, tegye futtathatóvá (`chmod +x`), és indítsa el a régi helyett. Törölje a régi fájlt. A beállítások és a torrentek nincsenek az AppImage-ben, ezért átkerülnek. Hordozható AppImage esetén az új fájlt a `rookparcel.ini` mellé tegye.
- **tar.gz:** lépjen ki a Rookparcelből, és csomagolja ki az új bináris fájlokat a régiek fölé.
- **Flatpak:** a Flatpaknak nincs frissítésellenőrzése. Töltse le az újabb csomagot a [Kiadások oldalról](https://github.com/Balls0fSteel/Rookparcel/releases), és telepítse, ahogy itt: [A Rookparcel telepítése](A-Rookparcel-telepítése#flatpak).
- **macOS:** lépjen ki a Rookparcelből (**Rookparcel > Kilépés a Rookparcel alkalmazásból**), majd húzza az új `Rookparcel.app`-ot a `Rookparcel-<version>-macos.dmg` lemezképről az Alkalmazások mappába, és cserélje le a régit. Ha a Gatekeeper elutasítja az új példányt, nyissa meg úgy, ahogy itt: [A Rookparcel telepítése](A-Rookparcel-telepítése#macos). Ugyanaz a `Rookparcel.app` fut Apple silicon és Intel Macen.
- **macOS, a GTK-s változatról:** a korábbi kiadásokban volt egy GTK-s `Rookparcel.app` Apple silicon számára. Cserélje le ugyanígy; az új alkalmazás az első indításkor átveszi a régi beállításait és torrentjeit. Lásd: [A Rookparcel telepítése](A-Rookparcel-telepítése#frissítés-a-gtk-s-változatról).

## Béta verziók

A Rookparcel időnként kiad egy előzetes kiadást (kiadásjelöltet) egy verzió előtt. Ha ezeket is fel szeretné ajánlani magának, jelölje be a **Frissítés bétaverziókra** beállítást a **Beállítások > Tulajdonságok > Általános** oldalon, az **Adatvédelem** alatt. Alapból ki van kapcsolva.

Egy előzetes kiadás mindig értesül a következő előzetes kiadásról, bármit mutat a jelölőnégyzet.

## Az ellenőrzés kikapcsolása

Vegye ki a jelölést a **Frissítések automatikus keresése** beállításból a **Beállítások > Tulajdonságok > Általános** oldalon, az **Adatvédelem** alatt. A Rookparcel ekkor soha nem kérdezi meg magától a webhelyet. A **Súgó > Frissítések keresése** továbbra is működik, ha Ön választja.

Néhány összeállításnak egyáltalán nincs frissítésellenőrzése: a Flatpaknak, és a csomagkezelőhöz készített csomagoknak, amelyek a frissítést a csomagkezelő végzi.

## rookparcel-nox

A `rookparcel-nox` ugyanígy ellenőriz, tíz másodperccel a `run` indítása után és 12 óránként. *Új az 1.1.17-ben.* Soha nem tölt le és nem telepít semmit. Ha megjelent egy újabb verzió:

- kiír egy sort, például `* Rookparcel 1.2.0 is available: https://github.com/Balls0fSteel/Rookparcel/releases/tag/v1.2.0`;
- a webes felülete a sort a Naplójában, a hivatkozást pedig az oldal állapotsorában mutatja.

A kikapcsolásához indítsa a `--no-update-check` kapcsolóval, vagy vegye ki a jelölést a **Frissítések automatikus keresése** beállításból a webes felület **Általános > Adatvédelem** oldalán. A **Frissítés bétaverziókra** is ott van.

A démon frissítéséhez állítsa le, cserélje le a `rookparcel-nox`-ot az újra, és indítsa el újra. A Windows-szolgáltatásnál előbb állítsa le rendszergazdai parancssorból (`sc stop RookparcelNox`), és utána indítsa újra; a Windows-telepítő ezt kéri, ha futó szolgáltatást talál. Lásd: [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

## A frissítés után

- **Távvezérlés:** ha egy kiadás megváltoztatja a távoli protokollt, az ablaknak és az általa vezérelt `rookparcel-nox`-nak egyaránt ezen a kiadáson kell lennie. Frissítse mindkettőt. A kiadási jegyzetek megmondják, mikor érvényes ez ([Távvezérlés](Távvezérlés)).
- **Visszatérés egy régebbi verzióra:** a beállítások és a torrentek átvihetők az 1.1.x verziók között. Az 1.1.40 kisebb formában tárolja a folytatási adatokat, ezért az innen egy régebbi verzióra való visszatérés minden torrentet egyszer újraellenőriz. Az 1.1.49 vagy újabb verzió által használt [hordozható mappához](Hol-tárolja-a-Rookparcel-a-fájljait#hordozható-mód) 1.1.49 vagy újabb kell.
- Mi változott az egyes kiadásokban: a [Kiadások oldalon](https://github.com/Balls0fSteel/Rookparcel/releases) lévő kiadási jegyzetek, vagy a webhely [Hírek](https://balls0fsteel.github.io/Rookparcel/news.html) oldala.

## Ha egy verzió másként olvassa a beállításait

*Új az 1.2.57-ben.*

A Rookparcel minden beállításmentéskor azt is feljegyzi, melyik verzió mentette, és hogyan olvasta azokat a beállításokat, amelyek eldöntik, merre megy a forgalma: a proxyt, a **Csak ennek a hálózatnak a használata** beállítást, a figyelő interfészeket, a titkosítást és az IP-szűrőt. Amikor egy másik verzió elindul, összeveti ezt a feljegyzést a saját olvasatával ugyanarról a fájlról:

- Először másolatot tart a fájlról úgy, ahogy találta, mellette: `rookparcel.ini.<old version>.bak` (a `rookparcel-nox` esetén `webui.json.<old version>.bak`). Az utolsó hármat megtartja.
- Minden másként olvasott beállítás a **Napló** lapra kerül, például `proxy: socks5 10.8.0.1:1080 peers trackers names → none`.
- Ha a proxy, a hálózati szabály vagy a figyelő interfészek elvesznének, semmi sem csatlakozik: az állapotsor ezt mutatja: **Átvitelek visszatartva: erősítse meg a beállításokat**, és a torrentek várakoznak. Ellenőrizze a **Beállítások > Tulajdonságok > Kapcsolat** oldalt, majd nyomja meg az **OK** gombot, és az átvitelek elindulnak. A webes felületen mentse a Tulajdonságok oldalát. Az RSS-csatornák, a `.torrent` hivatkozások és a frissítésellenőrzés is várakozik. A visszatéréshez inkább lépjen ki a Rookparcelből, és másolja a `.bak` fájlt a beállításfájl fölé.

Az átvitelek ugyanígy visszatartásra kerülnek, ha a beállításfájl egyáltalán nem olvasható (ilyenkor soha nem mentődik fölé), ha hiányzik vagy üres, miközben vannak mentett torrentek, vagy ha ez a verzió nem tudja olvasni a proxytípust, amelyet egy újabb verzió írt; az a sor a fájlban marad az újabb verzió számára. Az a beállításfájl, amely nem érvényes szöveg (más kódolású szerkesztő mentette), a lehető legjobban beolvasásra kerül, és az eredeti `rookparcel.ini.damaged-<number>` néven megmarad. A `rookparcel-nox` nem indul el egy általa olvashatatlan `webui.json` fájllal; megnevezi a fájlt, így kijavíthatja vagy elmozgathatja.
