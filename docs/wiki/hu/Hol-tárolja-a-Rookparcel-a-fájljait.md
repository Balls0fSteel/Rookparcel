*Ez az angol nyelvű [Where Rookparcel keeps its files](Where-Rookparcel-keeps-its-files) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Hol tárolja a Rookparcel a fájljait

Ez az oldal megmondja, hol tárolja a Rookparcel a beállításait és a torrentlistáját az egyes platformokon, hogyan tartja a hordozható mód ezeket a program mellett, és hogyan készíthet róluk biztonsági másolatot, vihet át őket másik számítógépre, vagy kezdheti elölről. A letöltései nem tartoznak ide: azok ott vannak, ahová elmentette őket.

## A mappa megkeresése

A **Súgó > A Rookparcel névjegye** (macOS-en **Rookparcel > A Rookparcel névjegye**) alul megmutatja a beállításfájl útvonalát. A **Fájl > Profilok > Profil mappájának megnyitása** megnyitja a mappát (*új az 1.1.48-ban*).

## Windows

A Windows-os ablak mindent egyetlen mappában tárol: `%APPDATA%\Rookparcel`, ami általában `C:\Users\<Ön>\AppData\Roaming\Rookparcel`. A telepített és a zipből futó példányok ugyanezt a mappát használják, kivéve ha a zipből futó példány hordozható.

| Fájl vagy mappa | Mit tartalmaz |
|---|---|
| `rookparcel.ini` | a beállítások |
| `state\` | a torrentlista és az egyes torrentek haladása |
| `rss.dat` | RSS-csatornák és szűrők |
| `remote.ini`, `remote\` | távoli kapcsolatok, és az egyes távoli ablakok beállításai |
| `ipfilter.dat` | az alapértelmezett IP-szűrő fájl, ha használ ilyet |
| `ipfilter-url.dat`, `ipfilter-url.txt` | az IP-szűrő tiltólistája, ahogy legutóbb letöltődött a webcíméről, és egy jegyzet arról, honnan és mikor, ha használ ilyet (*új az 1.2.63-ban*) |
| `nox\` | a `rookparcel-nox` fájljai, ha konzolból futott |
| `search-helper\` | a keresősegéd, ha beszerezte ([Torrentek keresése Windowson](Torrentek-keresése-Windowson)) |
| `profiles\` | a többi [profil](Profilok), mindegyik ehhez hasonlóan felépített mappa |
| `rookparcel.lock` | a Rookparcel futása közben a mappát használatban lévőként jelöli |

A `rookparcel-nox` Windows-szolgáltatása a fájljait ehelyett a `%ProgramData%\Rookparcel\nox` mappában tartja. Lásd: [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

## Linux és a BSD-k

A GTK-s változat az XDG-mappákat követi:

| Mi | Mappa |
|---|---|
| beállítások: `rookparcel.ini`, `rss.dat`, `remote.ini`, `remote/`, `ipfilter.dat` és az IP-szűrő letöltött listája (`ipfilter-url.dat`, `ipfilter-url.txt`), ha használja őket | `$XDG_CONFIG_HOME/rookparcel/gtk`, egyébként `~/.config/rookparcel/gtk` |
| torrentlista: `resume.db`, `resume.log`, `metadata.pack`, `dht_state.dat`, `stats.dat` (a Súgó > Statisztika összesített számai) | `$XDG_DATA_HOME/rookparcel/gtk`, egyébként `~/.local/share/rookparcel/gtk` |

A torrentlista mappájában lévő `rookparcel.lock` használatban lévőként jelöli azt. A `rookparcel-nox` mindkét útvonalban a `gtk` helyett `nox`-ot használ.

A Flatpak a mappáit a `~/.var/app/io.github.balls0fsteel.Rookparcel/` alatt tartja. A systemd alatt a mappák a szolgáltatás saját mappái; lásd: [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

## macOS

A beállítások és a torrentlista egyaránt a `~/Library/Application Support/Rookparcel/mac` mappában vannak, a fenti Windows-mappához hasonló felépítéssel; a `rookparcel-nox` a `~/Library/Application Support/Rookparcel/nox` mappát használja. A `work` nevű [profil](Profilok) a `~/Library/Application Support/Rookparcel/profiles/work/mac` mappában van.

A `Rookparcel.app` GTK-s változata a korábbi kiadásokban ugyanezeken a helyeken `gtk` mappát használt. A natív alkalmazás első indítása átnevezi a `gtk` mappákat `mac`-re (az alapértelmezett profilét és minden profilét), és a beállításaiban kijavítja a beléjük mutató útvonalakat, így minden átkerül. Ezt egyszer teszi meg: csak ott, ahol még nincs `mac` mappa, és nem akkor, amikor a régi alkalmazás használja a mappát. A hordozható mappákat békén hagyja.

*A natív macOS-es alkalmazás és a `mac` mappa az 1.2.19-es verzióban jelent meg; az előtte kiadott verziók a `gtk` mappát használják.*

## Hordozható mód

Hordozható módban a Rookparcel a beállításait és a torrentlistáját a saját mappájában tárolja, ahogy a µTorrent tette a `uTorrent.exe` mellett lévő `settings.dat` fájllal. A mappa lehet USB-meghajtón vagy szinkronizált mappában.

**A bekapcsolásához** tegyen egy `rookparcel.ini` nevű fájlt a megfelelő helyre az első indítás előtt. Üres is lehet. Fájlnak kell lennie: az ilyen nevű mappa nem számít.

| Példány | Hová kerül a `rookparcel.ini` |
|---|---|
| Windows zip | a `rookparcel.exe` mellé |
| Linux vagy BSD tar.gz | a `rookparcel-gtk` (vagy a `rookparcel-nox`) mellé |
| Linux AppImage | az `.AppImage` fájl mellé |
| macOS | a `Rookparcel.app` mellé, soha nem bele; a démonhoz a `rookparcel-nox` mellé |
| Flatpak, Windows-telepítő | nincs hordozható mód |

*A hordozható mód Windowson minden kiadásban megvan. Linuxon és a BSD-ken az 1.1.49-ben jelent meg, macOS-en az 1.1.50-ben.*

A hordozható mappa belső felépítése megegyezik a fenti `%APPDATA%\Rookparcel` mappáéval: a beállítások és a `state` a program mellett, a `nox` a `rookparcel-nox` számára, a `profiles` a többi profilnak.

### Mi változik hordozható módban

- **Az első indítás a mappába tölt le.** Az új letöltések a benne lévő `Downloads` mappába kerülnek, és a Rookparcel véletlen figyelő portot választ, hogy ne ütközzön egy másik, a 6881-es porton lévő Rookparcellel. *Új az 1.1.45-ben és az 1.1.49-ben.*
- **A mappán belüli útvonalak a mappához képest relatívan tárolódnak.** A mentési mappák, a Tulajdonságok mappái, az IP-szűrő fájl és az RSS mentési mappái, amelyek a hordozható mappán belül vannak, `./Downloads/...` alakban íródnak. A mappa így átkerülhet másik meghajtóbetűre vagy másik számítógépre, és a torrentek újraellenőrzés nélkül folytatódnak. A mappán kívüli útvonalak változatlanok maradnak. *Új az 1.1.49-ben.* A régebbi verzió által írt útvonalakat a program a következő mentéskor átalakítja.
- **A rendszerbe semmi sem íródik, ha Ön nem kéri.** A társítások, a rendszerrel való indítás és a szolgáltatások csak akkor állítódnak be, ha a Tulajdonságok ablakban bejelöli őket. Ezek ekkor ennek a példánynak a programját nevezik meg, és megszűnnek működni, ha a mappa elmozdul.
- **Nem frissíti magát.** Egy hordozható példány frissítési értesítése a letöltési oldalt nyitja meg. Lásd: [A Rookparcel frissítése](A-Rookparcel-frissítése).

Az 1.1.49 vagy újabb verzió által írt hordozható mappához 1.1.49 vagy újabb kell: egy régebbi verzió rosszul olvasná a relatív útvonalakat.

Ha a mappába nem lehet írni (csak olvasható meghajtó, `C:\Program Files`), a Rookparcel a „Ez a hordozható Rookparcel nem tud írni a mappájába” üzenettel leáll, ahelyett hogy a beállításait máshová tenné. Helyezze át a mappát, vagy távolítsa el a `rookparcel.ini` fájlt a szokásos mappák használatához.

macOS-en egy letöltés után közvetlenül a Letöltések mappából indított `Rookparcel.app` egy ideiglenes másolatból fut, ahol a mellette lévő `rookparcel.ini` nem látszik. A Rookparcel ezt közli. Helyezze át a `Rookparcel.app`-ot a Finderrel a kívánt mappába, és indítsa el újra.

## Egy mappa, egy Rookparcel

Két Rookparcel soha nem használ egy mappát egyszerre. Ha a Rookparcelt másodszor indítja el, a futó ablakot előre hozza, és átadja neki, amit megnyitott. Az a példány, amely a mappát egy másik munkamenetből vagy egy másik számítógépről, hálózati megosztáson át használatban találja, az „Egy másik Rookparcel már használja ezt a mappát” üzenetet mutatja, és bezárul. Előbb zárja be a másikat. (*A munkamenetek és számítógépek közötti ellenőrzés az 1.1.43-ban jelent meg.*)

Ha szándékosan két Rookparcelt szeretne egymás mellett futtatni, használjon [profilokat](Profilok).

## Biztonsági mentés

1. Lépjen ki a Rookparcelből (**Fájl > Kilépés**; macOS-en **Rookparcel > Kilépés a Rookparcel alkalmazásból**), hogy minden elmentődjön.
2. Másolja le a teljes mappát: Windowson a `%APPDATA%\Rookparcel` mappát, Linuxon a `~/.config/rookparcel` és a `~/.local/share/rookparcel` mappát is, macOS-en a `~/Library/Application Support/Rookparcel` mappát, vagy a hordozható mappát.

A mentés tartalmazza a beállításokat, a torrentlistát az egyes torrentek haladásával, az RSS-csatornákat és az összes profilt. A letöltött fájlokat nem tartalmazza.

## Átvitel másik számítógépre

1. Lépjen ki a Rookparcelből a régi számítógépen, és másolja le a mappát, ahogy a biztonsági mentésnél.
2. Telepítse a Rookparcelt az új számítógépre, de még ne indítsa el.
3. Tegye a mappát ugyanarra a helyre az új gépen.
4. Másolja vagy helyezze át a letöltéseit.
5. Indítsa el a Rookparcelt.

Hordozható módon kívül a torrentlista teljes útvonalakat tárol. Tegye a letöltéseket az új számítógépen ugyanazokra az útvonalakra (Windowson ugyanarra a meghajtóbetűre és ugyanazokba a mappákba), különben a torrentek nem találják a fájljaikat.

Egy hordozható mappával egyszerűbb: másolja le az egész mappát a letöltésekkel együtt, és indítsa el a benne lévő programot. Az útvonalai relatívak, ezért mindegy, hogy más a meghajtóbetű vagy a felhasználónév.

## Elölről kezdés

A Rookparcel teljes visszaállításához:

1. Lépjen ki a Rookparcelből.
2. Nevezze át a mappát (például `Rookparcel.old` névre), hogy vissza tudjon térni.
3. Indítsa el a Rookparcelt. Új telepítésként indul, a [beállítási útmutatóval](Első-indítás) és üres torrentlistával.

A letöltött fájlok a helyükön maradnak; a megosztásukhoz adja hozzá újra a torrentjeiket. A Windows-eltávolító is törölheti a mappát: ennek **Beállítások és torrentlista** beállítása ([A Rookparcel telepítése](A-Rookparcel-telepítése#eltávolítás)).
