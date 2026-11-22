*Ez a [Searching for torrents](Searching-for-torrents) című angol oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Torrentek keresése

Ez az oldal azt mutatja be, hogyan találhat új torrenteket a Rookparcelen belül: mi a különbség a saját lista szűrése és a keresés között, hogyan adhat hozzá keresési forrásokat (a keresősegédet, vagy a saját Prowlarr- vagy Jackett-példányát), és hogyan adhatja hozzá, amit a keresés talált. A Windows-ablakot, a GTK-ablakot (Linux, BSD) és a webes felületet tárgyalja.

A Windows-verzió lépésről lépésre, képernyőképekkel követhető bemutatásához lásd a [Torrentek keresése Windowson](Torrentek-keresése-Windowson) oldalt.

## Hogyan működik a keresés

A Rookparcelnek nincs saját keresőmotorja, és maga nem olvassa ki a webhelyeket. Egy *keresési forrást* kérdez meg: olyan programot, amely tud torrentoldalakon keresni, és a Torznab nevű szabványos formátumban válaszol. Két ingyenes program teszi ezt:

- A **Jackett** és a **Prowlarr**, amelyet esetleg már futtat a Sonarrhoz vagy a Radarrhoz, ezen a számítógépen, egy NAS-on vagy egy kiszolgálón.
- A **keresősegéd**, amely a Rookparcel által Önnek letöltött és futtatott Jackett. Ez a könnyű út, ha egyikük sincs meg Önnek.

A keresés egyszerre megy el minden kiválasztott forráshoz. A válaszokat egyesíti a program, az ismétlődéseket (ugyanazt a torrentet két forrásból) egyszer mutatja, a listát pedig a seedek szerint rendezi.

*A keresőmező és a Források kezelése az 1.1.40-es kiadásban van meg. A keresősegéd új az 1.1.59-ben.*

## A lista szűrése és a keresés

A főablak jobb felső részén lévő keresőmezőnek két feladata van:

| Mód | Mit tesz az Enter |
|---|---|
| Szűrés (alapértelmezés) | A gépelés a saját torrentlistáját szűkíti azokra a nevekre, amelyek tartalmazzák a szöveget. Semmit sem keres az interneten. |
| Egy forrás, vagy a **Minden forrás** | A gépelés nem szűri a listát. Az Enter elküldi a szöveget a kiválasztott forrásnak (vagy minden forrásnak), és megnyitja a **Torrentek keresése** ablakot az eredményekkel. |

A <kbd>Ctrl</kbd>+<kbd>F</kbd> mindkét felületen a mezőbe teszi a kurzort. Az üres mezőben lévő szürke súgószöveg megmondja, melyik módban van.

### Windows

A mező melletti kis nyíl menüt nyit (a mezőben az <kbd>Alt</kbd>+<kbd>Down</kbd> is megnyitja):

- **Torrentek szűrése**: a listaszűrő. A súgószöveg: „Keresés a torrentek nevében”.
- Minden bekapcsolt keresési forrás, név szerint. A súgószöveg: „Keresés itt:” és a forrás neve.
- **Minden forrás**. A súgószöveg: „Keresés minden forrásban”.
- **Források kezelése...**, amely megnyitja a Keresési források ablakot.

A választást a program megjegyzi az indítások között. Ha a kiválasztott forrást eltávolítják vagy kikapcsolják, a mező visszaáll a **Torrentek szűrése** módra.

### GTK

A mező melletti menügombnak ugyanezek a pontjai vannak, azzal a különbséggel, hogy a szűrő pontja **Keresés a torrentek nevében** nevű. Kiválasztott forrásnál a súgószöveg „Keresés a Prowlarr vagy a Jackett segítségével” lesz.

### macOS

A keresőmező menüjében ugyanazok a pontok vannak, mint a GTK-énál: **Keresés a torrentek nevében**, minden bekapcsolt forrás, **Minden forrás** és **Források kezelése...**.

### Webes felület

A webes felület keresőmezője csak a listát szűri. A kereséshez külön gomb tartozik mellette, a **Keresés** (telefonon az eszköztár **Továbbiak** menüjében, **Torrentek keresése** néven). A Keresés megnyitja a Torrentek keresése ablakot, amelyben már benne van a szűrőmező szövege.

## Források kezelése

A forrásokat a **Keresési források** ablak tartja nyilván:

- Windows: a keresőmező melletti nyíl, majd a **Források kezelése...**.
- GTK: a keresőmező melletti menügomb, majd a **Források kezelése...**, vagy a Torrentek keresése ablak **Források kezelése...** gombja.
- Webes felület: a **Keresés**, majd a Torrentek keresése ablak **Források kezelése...** gombja.

![A Keresési források ablak](images/search-sources.png)

Az ablak jelölőnégyzettel sorolja fel a forrásokat. A bejelölt forrásban keres a program, amikor a **Minden forrás** módban keres; a be nem jelölt megmarad, de kimarad. A gombok:

| Gomb | Mit tesz |
|---|---|
| **Hozzáadás...** | Saját forrást ad hozzá (lásd lent). |
| **Szerkesztés...** | Módosítja a kijelölt forrás nevét, URL-jét, API-kulcsát vagy jelölőnégyzetét. |
| **Eltávolítás** | Eltávolítja a kijelölt forrást, rákérdezés után. |
| **Teszt** | Elkéri a forrástól a kategóriáinak listáját. Az „OK” és a kategóriák száma azt jelenti, hogy az URL és az API-kulcs működik; különben a lista alatti sor mutatja a hibát. |

A Windows- és a GTK-ablakban itt vannak a keresősegéd gombjai is; lásd [A keresősegéd](#a-keresősegéd).

Amíg nincs forrása, a keresés ezt mutatja: „Még nincs keresési forrás: a Források kezelése alatt adja hozzá Prowlarr vagy Jackett szerverét.”

## Saját Prowlarr vagy Jackett hozzáadása

Ha már fut Prowlarr vagy Jackett, adja hozzá forrásként:

1. Nyissa meg a **Keresési források** ablakot, és kattintson a **Hozzáadás...** gombra. Megnyílik a **Keresési forrás hozzáadása** ablak.
2. **Név:** bármi, amit szeretne. Üresen hagyva a forrást a gépneve azonosítja.
3. **Torznab URL:** a forrás Torznab-csatornájának címe:
   - Jackett: `http://host:9117/api/v2.0/indexers/all/results/torznab/`. Az `all` rész a Jackettben beállított minden indexelőben keres, így egy forrás elég.
   - Prowlarr: `http://host:9696/1/api`, ahol az `1` az indexelő száma. A Prowlarrnak nincs „minden indexelő” csatornája, ezért indexelőnként egy forrást adjon hozzá.

   A `host` helyére írja be, hogy `localhost`, ha ugyanazon a számítógépen fut, vagy a számítógép hálózati címét.
4. **API-kulcs:** a Jackett oldaláról, vagy a Prowlarr **Settings > General** oldaláról való kulcs.
5. Hagyja bejelölve a **Keresés ebben a forrásban** négyzetet, és kattintson az **OK** gombra.
6. Jelölje ki az új forrást, és kattintson a **Teszt** gombra. Azt kell mondania, hogy „OK”, és meg kell adnia a kategóriák számát.

Az URL-nek `http://` vagy `https://` előtaggal kell kezdődnie.

**Az API-kulcsot** a program titkosítva tárolja (Windowson), és soha többé nem mutatja. Forrás szerkesztésekor a kulcs mezője üresen indul, a szürke „Változatlan” súgószöveggel; hagyja üresen a tárolt kulcs megtartásához, vagy írjon be újat. A kulcsot a program eltávolítja minden hibában vagy állapotsorban megjelenő címből.

**Proxy.** Az ezen a számítógépen vagy a helyi hálózaton lévő forrást közvetlenül éri el a program: nem nyilvános IP-címet vagy `localhost`-ot, vagy olyan gépnevet, amelynek minden címe ilyen. A nyilvános címen lévő forrás a Tulajdonságokban beállított proxyn keresztül megy, ha állított be ilyet. Ha a proxy maga oldja fel a gépneveket (a gépnevek küldésére beállított SOCKS-proxy, vagy sima `http://` címhez HTTP-proxy), a Rookparcel nem old fel előbb nevet ezen a számítógépen, így a `localhost`-tól eltérő gépnévvel megadott forrás is a proxyn megy keresztül; a közvetlen eléréshez adja meg a forrás IP-címét. Lásd a [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal) oldalt.

### A webes felületen

A webes felület Hozzáadás és Szerkesztés ablakában még egy mező van: **A webes felület jelenlegi jelszava:**. Forrás hozzáadásához, illetve a forrás URL-jének vagy API-kulcsának módosításához a webes felület jelenlegi jelszava kell. Forrás átnevezéséhez, ki- és bekapcsolásához, eltávolításához, teszteléséhez és a kereséshez nem. A hibás jelszó kijelentkezteti, ahogy a webes felület más részein is.

A webes felület soha nem mutatja a tárolt API-kulcsot, és soha nem küldi el a böngészőnek az eredménylinkeket (amelyek az indexelő kulcsát hordozzák).

## A keresősegéd

A keresősegéd a [Jackett](https://github.com/Jackett/Jackett), egy különálló ingyenes program saját szerzőitől, GPL-2.0 licenc alatt. A Rookparcel a gombra kattintáskor letölti a Jackett saját kiadását a GitHubról, ellenőrzi a GitHub által közzétett ellenőrzőösszeggel, a profiljában lévő `search-helper` mappában tartja, és csak a keresés idejére futtatja. A gombra kattintás előtt semmit sem tölt le és nem futtat.

*Új az 1.1.59-ben.*

### Hol érhető el

| Hol | Keresősegéd |
|---|---|
| Windows-ablak (x64 és ARM64) | Igen |
| GTK-ablak Linuxon (x86-64 és ARM64) | Igen |
| macOS-alkalmazás (Intel és Apple silicon) | Igen |
| Flatpak | Nem |
| FreeBSD | Nem (a Jackettnek nincs hozzá build-je) |
| Csomagkezelő által készített build, a frissítésellenőrzés nélkül | Nem |
| Webes felület és `rookparcel-nox` | Igen, ha a kiszolgáló Windows, Linux vagy macOS (FreeBSD nem, Flatpak nem). *Új az 1.1.67-ben.* |

Ahol nem érhető el, a Keresési források ablakban nincsenek segédgombok, és a **Hozzáadás...** a saját Prowlarr vagy Jackett számára továbbra is működik.

### Beszerzése

1. Nyissa meg a **Keresési források** ablakot, és kattintson a **Keresősegéd letöltése (Jackett)...** gombra.
2. A Rookparcel megkeresi a legújabb Jackett-kiadást, és letöltés előtt rákérdez. A kérdés megnevezi a verziót, a méretet (kb. 50 MB) és a mappát, ahová kerül. Kattintson az **Igen** gombra.
3. Egy folyamatjelző ablak mutatja a letöltést. A Rookparcel ezután ellenőrzi a fájlt, kicsomagolja, és egyszer elindítja, hogy beolvassa az oldalainak listáját.
4. Megnyílik **A keresősegéd oldalai** ablak. Semmi sincs előre bejelölve. Jelölje be a kívánt nyilvános oldalakat, és kattintson az **OK** gombra. A „FlareSolverr szükséges hozzá, amelyet a segéd nem tartalmaz” megjegyzésű oldalak szürkék, és nem jelölhetők be.
5. Visszatérve a Keresési forrásokhoz, a segéd **Keresősegéd (összes)** néven szerepel a listában. Az alsó sor megmondja, hány oldal van bejelölve.

Ezután válassza a **Keresősegéd (összes)** (vagy a **Minden forrás**) lehetőséget a keresőmező menüjében, és keressen.

Két-három oldal elég az induláshoz. Minden bejelölt oldalt megkérdez minden keresésnél, így több oldal lassabbá teszi a keresést, és több ismétlődést okoz.

Amint a segéd telepítve van, a letöltés gombját négy másik váltja fel:

| Gomb | Mit tesz |
|---|---|
| **Oldalak kiválasztása...** | Újra megnyitja A keresősegéd oldalai ablakot, hogy oldalakat jelöljön be vagy vegyen ki. |
| **A segéd oldalának megnyitása** | Megnyitja a Jackett saját oldalát a böngészőben, és a jelszavát a vágólapra másolja (az oldal kéri). Ezzel privát oldalakat adhat hozzá a bejelentkezésével; ezekben ezután a bejelöltekkel együtt keres a program. |
| **Keresősegéd frissítése** | Újabb Jackettet keres, és letöltés előtt rákérdez. Az oldalai és beállításai megmaradnak. Semmi sem frissül magától. |
| **Keresősegéd eltávolítása** | Rákérdez, majd leállítja a segédet, törli a mappáját, és kiveszi a forrását a listából. |

### Hogyan fut

- A forrásának első keresésekor indul, és az utolsó keresés után tíz perccel magától leáll. A Rookparcel bezárásakor is leáll. Magától vagy rendszerindításkor soha nem indul.
- Csak ezen a számítógépen figyel (`127.0.0.1`), a hálózaton soha.
- Nem igényel rendszergazdai jogot.
- Ha a Tulajdonságokban proxyt állított be, a segéd az oldalakhoz azt használja. A „Csak ennek a hálózatnak a használata” nem köti meg a segédet, mert különálló program: a kérései a számítógép szokásos útvonalán mennek.

### A webes felületen és a `rookparcel-noxban`

A webes felület **Források kezelése** ablakában ugyanazok a gombok és ugyanaz az oldallista van. A segédet a kiszolgáló tölti le és futtatja, nem a böngészős számítógép, és a `rookparcel-nox --no-state` esetén nem érhető el. Minden időigényes lépés mutatja az előrehaladását, és a letöltésnek van **Mégse** gombja. A letöltéséhez a webes felület jelenlegi jelszava kell, ahogy a forrás hozzáadásához is. A segéd saját oldala a kiszolgáló `127.0.0.1` címén figyel, ezért a segéd oldalának megnyitása gomb csak azon a számítógépen futó böngészőnek látszik, amelyen a Rookparcel fut; máshol az ablak ezt közli. A **Fájl > Csatlakozás távoli géphez...** paranccsal kapcsolódott ablak a saját segédjét használja, ezen a számítógépen.

### Hol van

A `search-helper` mappa a profilja mappájában:

| Rendszer | Mappa |
|---|---|
| Windows | `%APPDATA%\Rookparcel\search-helper`; hordozható módban a `rookparcel.exe` mappája; profil esetén annak a profilnak a mappája |
| Linux (GTK) | `~/.local/share/rookparcel/gtk/search-helper` (profil esetén a `profiles/<név>/` alatt) |
| macOS | `~/Library/Application Support/Rookparcel/mac/search-helper` (profil esetén a `profiles/<név>/` alatt) |
| `rookparcel-nox` | `search-helper` az állapotmappájában: `%APPDATA%\Rookparcel\nox\search-helper`, `~/.local/share/rookparcel/nox/search-helper`, vagy `/var/lib/rookparcel-nox/search-helper` a mellékelt rendszerszolgáltatás esetén |

Minden profilnak saját segédje van. Lásd [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait) és [Profilok](Profilok).

## A Torrentek keresése ablak

Az ablak akkor nyílik meg, amikor kiválasztott forrásnál megnyomja az Entert a keresőmezőben (Windows, GTK), vagy rákattint a **Keresés** gombra (webes felület). Ezek vannak benne:

- **A lekérdezés** és a **Keresés**, hogy az ablakból újra kereshessen.
- **Egy kategória**: **Minden kategória**, **Filmek**, **Audio**, **PC**, **TV**, **Könyvek** vagy **Egyéb**.
- **Egy forrás**: **Minden forrás**, vagy egy forrás.
- **Leállítás**, hogy leállítsa a még kereső forrásokat. Az eddigi eredmények megmaradnak.
- **Forrásonként egy állapotsor**: „keresés...”, az eredmények száma, vagy a hiba. Az egyik forrás hibája nem állítja meg a többit.
- **Az eredmények**, a **Név**, **Méret**, **Seedek**, **Peerek**, **Kategória**, **Kor** és **Forrás** oszlopokkal. Kattintson egy oszlopfejlécre a rendezéshez; a lista seedek szerint rendezve indul. Ha két forrás ugyanazt a torrentet találja, egyszer szerepel, és a **Forrás** mindkettőt megnevezi.
- Az eredmények száma, a **Hozzáadás** és a **Bezárás**.

Telefonon a webes felület eredményei csak a nevet, a méretet és a seedeket mutatják.

Az ablak bezárása leállítja a keresést (Windowson az <kbd>Esc</kbd> is bezárja).

## Eredmény hozzáadása

Jelöljön ki egy eredményt, és kattintson a **Hozzáadás** gombra, kattintson rá duplán, vagy nyomja meg az <kbd>Enter</kbd> billentyűt a listában.

A Rookparcel lekéri az eredményt a forrásától (egy `.torrent` fájlt vagy egy magnet hivatkozást), és úgy adja hozzá, ahogy a megnyitott `.torrent` fájlt vagy a rákattintott magnet hivatkozást. Ha a **Beállítások > Tulajdonságok > Felület** oldalon be van kapcsolva az **Ablak megjelenítése a torrentben lévő fájlokkal**, előbb az Új torrent hozzáadása ablak nyílik meg. Lásd a [Torrentek hozzáadása](Torrentek-hozzáadása) oldalt.

Az alsó sor azt mondja: „Hozzáadva:” és a torrent neve. A GTK-ablak és a webes felület elutasítja azt az eredményt, amely már a torrentlistájában van.

A webes felületen a torrent a kiszolgálón kerül hozzáadásra.

## Keresés távoli ablakból vagy a webes felületről

- **A távoli ablak** (Fájl > Csatlakozás távoli géphez, lásd [Távvezérlés](Távvezérlés)) erről a számítógépről keres, a saját forráslistájával, és amit kiválaszt, azt a távoli kiszolgálóhoz adja hozzá.
- **A `rookparcel-nox` webes felülete** a saját forrásait a kiszolgálón tartja. A keresések és a letöltések a kiszolgálón futnak; minden megnyitott oldal ugyanazt a keresést látja.
- **A Windows-ablak által üzemeltetett webes felület** az ablak forrásait használja: az ablakban hozzáadott forrás megjelenik az oldalon, és fordítva. Lásd a [Webes felület](Webes-felület) oldalt.

A webes felület a qBittorrent webes API-jának keresési részére is válaszol, így a qBittorrent távoli alkalmazásainak keresőlapja is az Ön forrásaiban keres.

## Ha valami nem működik

| Amit lát | Mit jelent |
|---|---|
| Az eredményablak soha nem nyílik meg | A mező szűrő módban van. Előbb válasszon forrást vagy a **Minden forrás** lehetőséget a menüjében. |
| „Még nincs keresési forrás: ...” | Adjon hozzá forrást a Források kezelésében, vagy szerezze be a keresősegédet. |
| Egy forrás sora hibát mutat | Az a forrás elbukott; a többiek továbbra is válaszolnak. Kattintson a **Teszt** gombra a Keresési forrásokban az URL-je és az API-kulcsa ellenőrzéséhez. |
| A teszt `HTTP 401`-et vagy az API-kulcsra vonatkozó hibát mutat | Az API-kulcs hibás. Szerkessze a forrást, és írja be újra a kulcsot. |
| A keresősegéd nem talál semmit | Nincs oldal bejelölve, vagy a bejelölt oldalak nem működnek vagy megváltoztak. Az **Oldalak kiválasztása...** megmutatja, mi van bejelölve; próbáljon egy másik oldalt. |
| „Nem sikerült beszerezni a keresősegédet: ...” | A GitHub nem volt elérhető, vagy a letöltés megszakadt. Próbálja újra. |
| „A keresősegéd nem indult el: ...” | A `search-helper` mappában lévő program nem tudott futni: sérült letöltés, vagy egy vírusirtó eltávolított fájlokat. Távolítsa el, és szerezze be újra. |

További segítség: [Hibaelhárítás és GYIK](Hibaelhárítás-és-GYIK). Az új epizódok csatornákból való automatikus letöltéséhez lásd az [RSS letöltő](RSS-letöltő) oldalt. A Rookparcel Prowlarr letöltési kliensként való használatához lásd a [Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr) oldalt.
