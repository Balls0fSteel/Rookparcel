*Ez a [Searching for torrents on Windows](Searching-for-torrents-on-Windows) című angol oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Torrentek keresése Windowson

Rövid útmutató a Rookparcel Windows-verziójának keresőmezőjéhez: hogyan szerezze be a keresősegédet, hogyan válassza ki azokat az oldalakat, amelyekben keres, és hogyan adja hozzá, amit talált. Tíz perc és egy 55 MB-os letöltés, egyszer kell elvégezni.

**Verzió:** a keresősegéd a Rookparcel 1.1.59-es és újabb verzióiban van meg. Az 1.1.40-es kiadásban megvan a keresőmező és a Források kezelése, de a „Keresősegéd letöltése” gomb nincs; ott az utolsó szakasz (saját Prowlarr vagy Jackett) érvényes.

## Mit kap

A Rookparcelnek nincs saját keresőmotorja. Egy *keresési forrást* kérdez meg: egy kis programot, amely tud torrentoldalakon keresni. A legkönnyebb forrás a **keresősegéd**, vagyis a [Jackett](https://github.com/Jackett/Jackett), egy különálló ingyenes program saját szerzőitől. A Rookparcel egy kattintásra letölti Önnek, a saját mappájában tartja, és csak a keresés idejére futtatja. Ön választja ki, mely nyilvános oldalakon keressen. A gombra kattintásig semmit sem tölt le és nem futtat.

Ha már fut Prowlarr vagy Jackett Önnél, használhatja azt: lásd az utolsó szakaszt.

## 1. Nyissa meg a Források kezelését

A keresőmező a főablak jobb felső részén van. A mellette lévő kis nyíl menüt nyit. Válassza a **Források kezelése...** pontot.

![A keresőmező legördülő menüje a Források kezelésével](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-dropdown.png)

Megnyílik a Keresési források ablak. Eleinte üres.

![A Keresési források ablak a Keresősegéd letöltése gombbal](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-sources.png)

## 2. Szerezze be a keresősegédet

Kattintson a **Keresősegéd letöltése (Jackett)...** gombra. A Rookparcel megkeresi a legújabb Jackett-kiadást, és letöltés előtt rákérdez:

> A keresősegéd a Jackett, egy különálló program saját szerzőitől, GPL-2.0 licenccel, a github.com/Jackett/Jackett címről letöltve (v0.24.3009, 54,5 MB).
>
> A(z) C:\Users\you\AppData\Roaming\Rookparcel\search-helper mappába kerül, és csak ezen a számítógépen fut, csak amíg keres. Ezután kiválasztja azokat a nyilvános oldalakat, amelyekben keres.
>
> Letölti most?

Kattintson az **Igen** gombra. A letöltés néhány másodperctől egy percig tart, a kapcsolattól függően.

![A letöltés folyamatjelző ablaka](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-download.png)

A Rookparcel ellenőrzi a fájlt a kiadás ellenőrzőösszegével, kicsomagolja, és egyszer elindítja, hogy beolvassa az oldallistáját. Windows-kérdés nem jelenik meg: semmihez sem kell rendszergazdai jog, és a segéd csak ezen a számítógépen figyel, így a tűzfalnak nincs mit kérdeznie.

## 3. Jelölje be a keresendő oldalakat

Az oldallista magától megnyílik (később az **Oldalak kiválasztása...** gomb nyitja meg). Semmi sincs előre bejelölve. Jelölje be a kívánt nyilvános oldalakat, majd kattintson az **OK** gombra.

![A Keresősegéd oldalai ablak bejelölt The Pirate Bay-jel](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-sites.png)

- A szürke sorok ezt mondják: „FlareSolverr szükséges hozzá, amelyet a segéd nem tartalmaz”. Ezek az oldalak olyan robotellenőrzés mögött vannak, amelyen a segéd egyedül nem jut át; hagyja őket.
- Két-három oldal elég az induláshoz. Minden bejelölt oldalt megkérdez minden keresésnél, így több oldal lassabb keresést és több ismétlődést jelent.
- A privát oldalakhoz bejelentkezés kell. Ezeket a segéd saját oldala kezeli: lásd lent a „Privát oldalak” részt.

Visszatérve a Keresési forrásokhoz, a segéd most **Keresősegéd (összes)** néven szerepel a listában, a gombjaival alatta, és az alsó sor megmondja, hány oldal van bejelölve. Kattintson a **Bezárás** gombra.

![A Keresési források a telepített segéddel és két bejelölt oldallal](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-sources-installed.png)

## 4. Keresés

Nyissa meg újra a nyíl menüjét, és válassza a **Keresősegéd (összes)** pontot. A keresőmező súgószövege ennek megfelelően megváltozik. (Az első pont, a **Torrentek szűrése** a mező másik feladata: a saját torrentlistáját szűkíti név szerint. A **Minden forrás** egyszerre kérdezi meg az összes meglévő forrását.)

![A legördülő menü a kiválasztott Keresősegéd (összes) ponttal](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-dropdown-helper.png)

Írja be, amit keres, és nyomja meg az **Enter** billentyűt. Megnyílik a Torrentek keresése ablak, és az eredmények néhány másodpercen belül megérkeznek.

![A Torrentek keresése ablak az „ubuntu” eredményeivel](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-results.png)

- Kattintson egy oszlopfejlécre a rendezéshez: a seedek szerint csökkenően rendezve az egészségeseket találja meg.
- A kategóriamező a keresést a Filmekre, TV-re, Audióra, PC-re, Könyvekre vagy az Egyébre szűkíti. A forrásmező és a Keresés gomb ebből az ablakból megismétli a keresést.
- Jelöljön ki egy eredményt, és kattintson a **Hozzáadás** gombra, vagy kattintson rá duplán. A torrent úgy kerül hozzáadásra, ahogy a magnet hivatkozás; ha a Tulajdonságokban be van kapcsolva az „Ablak megjelenítése a torrentben lévő fájlokkal”, előbb az Új torrent hozzáadása ablak nyílik meg.
- Hogy a mező újra a torrentlistáját szűrje, válassza a **Torrentek szűrése** pontot a nyíl menüjében. A választást a program megjegyzi az indítások között.

Ennyi az egész beállítás. Az alábbi szakaszok későbbre valók.

## Jó tudni

**Csak a keresés idejére fut.** A segéd az első keresésekor indul, és az utolsó után tíz perccel magától leáll. A Rookparcellel együtt bezárul, akárhogyan ér is véget a Rookparcel. Magától vagy rendszerindításkor soha nem indul, és csak ezen a számítógépen figyel (`127.0.0.1`), a hálózaton soha.

**Hol van.** A `search-helper` mappa a Rookparcel beállításmappájában: általában `%APPDATA%\Rookparcel\search-helper`, hordozható módban a `rookparcel.exe` mappája, profilok használatakor pedig a profil mappája. A Jackett saját beállításai és naplója a `data` almappájában vannak.

**Proxy és VPN.** Ha a Tulajdonságok > Kapcsolat oldalon proxyt állított be, a segéd az oldalakhoz azt is használja. A segéd különálló program, ezért a „Csak ennek a hálózatnak a használata” nem köti meg: az oldalakhoz küldött kérései a Windows szokásos útvonalán mennek, ami a VPN-je, ha a VPN viszi az Ön teljes forgalmát.

**Frissítés.** Semmi sem frissül magától. A Keresési források **Keresősegéd frissítése** gombja újabb Jackettet keres, és letöltés előtt rákérdez; az oldalai és beállításai megmaradnak.

**Eltávolítás.** A **Keresősegéd eltávolítása** leállítja, törli a mappáját, és kiveszi a forrását a listából. A letöltés gombja visszatér. A Rookparcel beállításmappájának törlése is eltávolítja.

**Privát oldalak.** Az **A segéd oldalának megnyitása** gomb megnyitja a Jackett saját oldalát a böngészőben, és a jelszavát a vágólapra másolja (az oldal kéri). Ott privát oldalt adhat hozzá a bejelentkezésével, mint bármelyik Jackettben. Az ott hozzáadott oldalakban a bejelöltekkel együtt keres a program.

**Vírusirtó.** A letöltés a Jackett saját kiadása, a GitHub által közzétett ellenőrzőösszeggel ellenőrizve. Ha egy vírusirtó karanténba helyez fájlokat a `search-helper` mappából, a keresések ezzel hibáznak: „A keresősegéd nem indult el”; állítsa vissza a fájlokat, vagy távolítsa el és szerezze be újra.

## Ha valami nem működik

| Amit lát | Mit jelent |
|---|---|
| „Nem sikerült beszerezni a keresősegédet: ...” | A GitHub nem volt elérhető (nincs kapcsolat, vagy egy proxy blokkolja), vagy a letöltés megszakadt. Próbálja újra. |
| „A keresősegéd nem indult el: ...” | A `search-helper` mappában lévő program nem tudott futni: sérült letöltés vagy egy vírusirtó. Távolítsa el, és szerezze be újra. |
| A keresés nem talál semmit | Nincs oldal bejelölve, vagy a bejelölt oldalak nem működnek vagy megváltoztak. Az Oldalak kiválasztása... megmutatja, mi van bejelölve; próbáljon egy másik oldalt. |
| „Keresősegéd (összes): ...” hibával | Egy oldal elbukott; a többiek továbbra is válaszolnak. Az üzenet megnevezi az oldalt. |
| Az eredményablak soha nem nyílik meg | A mező **Torrentek szűrése** módban van. Előbb válassza a **Keresősegéd (összes)** pontot vagy egy forrást a nyíl menüjében. |

## Saját Prowlarr vagy Jackett használata helyette

Ha már fut Prowlarr vagy Jackett (ezen a PC-n, egy NAS-on, egy kiszolgálón), adja hozzá forrásként, és hagyja ki a segédet:

1. A Keresési forrásokban kattintson a **Hozzáadás...** gombra.
2. **Név:** bármi. **Torznab URL:** Jackett esetén `http://host:9117/api/v2.0/indexers/all/results/torznab/`; Prowlarr esetén egy indexelő Torznab-csatornájának URL-je, `http://host:9696/1/api`, indexelőnként egy forrás. **API-kulcs:** a Jackett oldaláról, vagy a Prowlarr Settings > General oldaláról.
3. Kattintson a **Teszt** gombra: azt kell mondania, hogy „OK”, és meg kell adnia a kategóriák számát. Majd **OK**.

Az API-kulcsot a program titkosítva tárolja, és soha többé nem mutatja. A saját számítógépén vagy hálózatán lévő forrást közvetlenül éri el; a nyilvánosat a proxyján át, ha beállított ilyet. (Ha a proxy maga oldja fel a gépneveket, a `localhost`-tól eltérő gépnévvel megadott forrás is rajta megy keresztül; a közvetlen eléréshez adja meg a forrás IP-címét.) A segéd és a saját források egymás mellett használhatók: a nyíl menüjében a **Minden forrás** mindet megkérdezi.

A segéd felépítéséről és arról, mit tesz a hálózaton, lásd a [SEARCH_HELPER.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/SEARCH_HELPER.md) fájlt; magáról a Torznab-keresésről az [INTEGRATIONS.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/INTEGRATIONS.md) 2. részét.
