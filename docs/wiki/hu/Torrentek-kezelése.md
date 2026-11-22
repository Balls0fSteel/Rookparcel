*Ez a [Managing torrents](Managing-torrents) című angol oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Torrentek kezelése

Ez az oldal azt tárgyalja, mit tehet a listában szereplő torrentekkel: indítás és leállítás, a várólista, eltávolítás a fájlokkal vagy azok nélkül, újraellenőrzés, a fájlok áthelyezése, címkék és jelölők, torrentenkénti sebességkorlátok, valamint a jobb gombbal előhívható menü, amely mindezt egybefogja.

Minden művelet a kijelölt torrenteken hat. Többet jelölhet ki a <kbd>Ctrl</kbd>+kattintással vagy a <kbd>Shift</kbd>+kattintással, az összeset pedig a <kbd>Ctrl</kbd>+<kbd>A</kbd> billentyűkombinációval, és egyszerre végezhet rajtuk műveletet.

## A jobb gombos menü

![A torrentlista jobb gombos menüje](images/torrent-menu.png)

Kattintson a jobb gombbal egy torrentre (vagy nyomja meg a <kbd>Shift</kbd>+<kbd>F10</kbd> kombinációt), és megjelenik ez a menü:

| Menüpont | Mit tesz | Billentyű |
|---|---|---|
| **Mappa megnyitása** | megnyitja a torrent mappáját a Fájlkezelőben, kijelölve az adatait | |
| **Magnet hivatkozás másolása** | a vágólapra teszi a torrent magnet hivatkozását | |
| **.torrent exportálása...** | elmenti a torrent `.torrent` fájlját | |
| **Kényszerített indítás** | azonnal elindítja, a várólistát megkerülve | |
| **Indítás** | átadja a torrentet a várólistának | <kbd>Ctrl</kbd>+<kbd>S</kbd> |
| **Szünet** | megállítja, és kiveszi a várólistából; az állapota **Szüneteltetve** | <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd> |
| **Leállítás** | leállítja, és kiveszi a várólistából; az állapota **Leállítva** vagy **Kész** | <kbd>Ctrl</kbd>+<kbd>T</kbd> |
| **Feljebb a sorban**, **Lejjebb a sorban** | egy hellyel feljebb vagy lejjebb | |
| **A sor elejére**, **A sor végére** | a sor elejére vagy végére, a sorrendjük megtartásával | |
| **Eltávolítás** | kiveszi a listából; a fájlok megmaradnak | <kbd>Del</kbd> |
| **Eltávolítás és > Adatok törlése** | kiveszi, és törli a fájljait | <kbd>Shift</kbd>+<kbd>Del</kbd> |
| **Kényszerített ellenőrzés** | újra ellenőrzi a lemezen lévő fájlokat | |
| **Tracker frissítése** | most felveszi a kapcsolatot a trackerekkel | |
| **Haladó > Letöltési hely megadása...** | áthelyezi a fájljait egy másik mappába | |
| **Haladó > Kezdeti megosztás** | be- vagy kikapcsolja a kezdeti megosztást | |
| **Haladó > Letöltés sorrendben** | a részeket sorrendben tölti le, az elejétől a végéig | |
| **Haladó > Első és utolsó részek letöltése előbb** | előbb tölti le minden kért fájl mindkét végét | |
| **Sávszélesség > Letöltési korlát / Feltöltési korlát** | saját sebességkorlátok | |
| **Címke** | címkét ad neki, újat, vagy egyet sem | |
| **Jelölők** | jelölőket ad neki, vagy elvesz tőle | |
| **Jellemzők** | trackerek, korlátok, megosztási cél és egyéb | <kbd>Enter</kbd> |

Az alábbi szakaszok csoportonként magyarázzák a menüpontokat. Ezek többsége az eszköztáron is megtalálható; lásd [A főablak](A-főablak#az-eszköztár) oldalt.

## Indítás, szünet, leállítás és kényszerített indítás

A Rookparcel várólistát vezet, ahogy a µTorrent is. Hogy egyszerre hány torrent futhat, azt a **Beállítások > Tulajdonságok > Sorba állítás** oldalon adhatja meg; lásd [Várólista és seedelés](Várólista-és-seedelés).

- Az **Indítás** átadja a torrentet a várólistának. Ha van hely, a torrent fut (**Letöltés** vagy **Megosztás**). Ha nincs, **Sorban áll** vagy **Megosztás sorban** állapotban vár, amíg hely nem szabadul.
- A **Kényszerített indítás** azonnal elindítja a torrentet, akkor is, ha a várólista tele van. Az állapota **[F]** jellel kezdődik, például **[F] Megosztás**. A kényszerítetten indított torrent soha nem éri el a megosztási célt, ezért addig oszt, amíg le nem állítja.
- A **Szünet** és a **Leállítás** egyaránt megállítja a torrentet, és kiveszi a várólistából. Az egyetlen különbség az Állapot oszlopban látható szó: **Szüneteltetve**, illetve **Leállítva** (kész torrentnél **Kész**). Mindkettőt megjegyzi a program az újraindítás után. A folytatáshoz indítsa el, vagy indítsa kényszerítetten.

A tálcaikon menüjében ott a **Minden torrent indítása** és a **Minden torrent leállítása**, amelyek a lista minden torrentjére hatnak.

Ha új torrenteket leállítva szeretne hozzáadni, lásd a [Torrentek hozzáadása](Torrentek-hozzáadása#torrentek-hozzáadása-leállítva) oldalt.

## A várólista

A **#** oszlop mutatja minden torrent helyét a várólistában. Az alacsonyabb szám előbb kerül sorra. A sorrend módosításához használja a **Feljebb a sorban** és a **Lejjebb a sorban** parancsot (az eszköztáron is megvan), vagy az **A sor elejére** és az **A sor végére** parancsot. Több torrent áthelyezésekor a köztük lévő sorrend megmarad.

Az Új torrent hozzáadása ablakban az **A sor elejére** jelölőnégyzet az új torrentet az első helyre teszi.

## Torrentek eltávolítása

- Az **Eltávolítás** (<kbd>Del</kbd>) kiveszi a torrenteket a listából. A letöltött adatok a lemezen maradnak.
- Az **Eltávolítás és > Adatok törlése** (<kbd>Shift</kbd>+<kbd>Del</kbd>) kiveszi őket, és törli a letöltött fájlokat is. Windowson a fájlok kikerülik a Lomtárat, onnan nem állíthatók vissza. A `.!ut` vagy `.!qB` végződésű részleges fájlt, amelyről a Rookparcel nem tudja megállapítani, hogy ennek a torrentnek az adatait tartalmazza, megtartja; lásd a [Lemez és tárolás](Lemez-és-tárolás#befejezetlen-fájlok-és-a-ut) oldalt.

Ha a **Torrentek eltávolításának megerősítése** be van kapcsolva (Tulajdonságok > Felület, alapértelmezés szerint be van kapcsolva), a Rookparcel előbb rákérdez, és közli, hogy a fájlok is törlődnek-e. Kikapcsolva az Eltávolítás azonnal végrehajtódik.

## Kényszerített ellenőrzés

A **Kényszerített ellenőrzés** darabonként beolvassa a torrent fájljait a lemezről, és csak a teljes és hibátlan részeket számolja be. Az állapot **Ellenőrzésre vár**, amíg az ellenőrzés a meghajtójára vár (az ellenőrzések meghajtónként egy torrenttel futnak; *új az 1.2.76-ban*), majd **Ellenőrzés** százalékos értékkel. Akkor használja, ha a fájlokat kézzel helyezte át vagy állította vissza, vagy ha sérült adatokra gyanakszik.

*Új az 1.2.76-ban:* a leállított torrent az ellenőrzéssel együtt elindul; ha utána is leállítva szeretné tartani, állítsa le újra. A sorban álló torrentet azonnal ellenőrzi, a várólistán lévő helyre várás nélkül, utána pedig újra sorba állítja. A **hiányzó fájlok** miatt visszatartott torrentet (lásd a [Lemez és tárolás](Lemez-és-tárolás#ha-egy-meghajtó-vagy-megosztás-hiányzik) oldalt) azzal vetíti össze, ami most megvan, és ami hiányzik, azt újra letölti.

Hogy az ellenőrzések mennyire kímélik a lemezt, azt a **Tulajdonságok > Haladó > Lemez I/O** részben állíthatja be; lásd a [Lemez és tárolás](Lemez-és-tárolás) oldalt.

## Tracker frissítése

A **Tracker frissítése** most felveszi a kapcsolatot a torrent trackereivel, a következő bejelentésre várás nélkül, hogy jelentse az előrehaladást, és további peereket kapjon. A választ a [Trackerek lap](Torrent-részletek-és-fájlok#trackerek) mutatja.

## A fájlok áthelyezése: Letöltési hely megadása

1. Jelölje ki a torrenteket.
2. Válassza a **Haladó > Letöltési hely megadása...** parancsot.
3. Válassza ki az új mappát.

A Rookparcel áthelyezi oda a fájlokat, és az új helyről folytatja a megosztást vagy a letöltést. Az állapot közben **Áthelyezés**, és a menüpont szürke azon a torrenten, amelynek az áthelyezése már folyamatban van. Az azonos nevű fájlokat az új mappában lecseréli.

Ha kilép, miközben fájlok mozognak, a Rookparcel megvárja az áthelyezések végét, és mutatja az előrehaladásukat. A **Kilépés most** megszakít egy áthelyezést: a fájljai a régi mappában maradnak, a már átmásolt részek pedig törlődnek. Helyezze át újra később.

A kész letöltések automatikus áthelyezéséhez lásd a **Kész letöltések áthelyezése ide:** beállítást a [Lemez és tárolás](Lemez-és-tárolás) oldalon.

## A mappa megnyitása

A **Mappa megnyitása** megnyitja a torrent mappáját a Fájlkezelőben, kijelölve a fájlját vagy mappáját, ha az már létezik. Ha több torrent van kijelölve, az elsőét nyitja meg. Ugyanezt teszi a kész torrenten végzett dupla kattintás.

Egyetlen fájl megnyitásához használja a [Fájlok lapot](Torrent-részletek-és-fájlok#fájlok).

## Magnet hivatkozás másolása és .torrent exportálása

*Új az 1.2.63-ban.*

A **Magnet hivatkozás másolása** a kijelölt torrentek magnet hivatkozását a vágólapra teszi, soronként egyet. Minden hivatkozás tartalmazza a torrent info-hash-ét, a nevét és a trackereit. Illessze be egy másik torrentprogramba, vagy küldje el valakinek, hogy fájl nélkül ossza meg a torrentet.

Mivel a hivatkozás a trackereket is tartalmazza, ne ossza meg olyan hivatkozást, amelyet privát tracker torrentjéből másolt. A privát trackerek címe általában tartalmazza az Ön személyes kulcsát (passkey), és a hivatkozás birtokában bárki az Ön nevében jelentkezhetne be.

A **.torrent exportálása...** a kijelölt torrentek `.torrent` fájljait menti el. A fájl a torrentet úgy tartalmazza, ahogy a Rookparcel megkapta, a most használt trackerekkel és webes seedekkel. Hozzáadhatja egy másik számítógépen, vagy megtarthatja másolatként.

- Egy kijelölt torrentnél a Rookparcel fájlnevet kér.
- Többnél mappát kér, és mindegyiket `<név>.torrent` néven menti. Ha a név foglalt, számot fűz hozzá, például ` (2)`, ahelyett, hogy lecserélné a fájlt.
- A magnet hivatkozásból hozzáadott torrentnek nincs `.torrent` fájlja, amíg a fájllistája meg nem érkezik („Metaadatok letöltése”). A menüpont szürke, ha egyik kijelölt torrentnek sincs ilyen.

Ha a megadott fájl nem írható, a Rookparcel azt közli, hogy a torrent nem exportálható, megadva a torrent nevét és az okot. A mappába exportálásnál sikertelen fájlokat a rendszer a [Napló lapra](Torrent-részletek-és-fájlok#napló) írja.

## Címkék

A címke egy név, amellyel a torrenteket csoportosíthatja, például „Filmek” vagy „Linux”. A használatban lévő címkék a kategórialistában a **Címkék** alatt jelennek meg, darabszámmal, így egy kattintással csak az ilyen torrentek látszanak.

- A **Címke > Új címke...** nevet kér, és a kijelölt torrenteknek adja.
- A **Címke >** *egy címke* egy már meglévő címkét ad.
- A **Címke > Címke eltávolítása** leveszi a címkét.

A menü megjelöli azt a címkét, amelyet a kijelölt torrentek közösen viselnek. Címkét az Új torrent hozzáadása ablakban is adhat, és az RSS-szűrők is adhatnak.

Ha a **Kész letöltések áthelyezése ide:** mellett be van kapcsolva **A torrent címkéjének hozzáfűzése** (Tulajdonságok > Mappák), a kész, címkézett torrent a címkéjéről elnevezett mappába kerül. Lásd a [Lemez és tárolás](Lemez-és-tárolás) oldalt.

## Jelölők

*Új az 1.2.63-ban.*

Egy torrentnek egy címkéje van, de mellette tetszőleges számú jelölője lehet, például „4K”, „megtart” és „Linux ISO-k”. A jelölők úgy működnek, mint a qBittorrentben. Minden jelölő a kategórialistában a **Jelölők** alatt jelenik meg, a címkék alatt, darabszámmal; a több jelölővel rendelkező torrent mindegyik alatt beleszámít. A **Jelölő nélkül** a jelölő nélküli torrenteket mutatja.

- A **Jelölők > Új jelölő...** nevet kér, és az új jelölőt a kijelölt torrenteknek adja.
- A **Jelölők >** *egy jelölő* az adott jelölőt adja a kijelölt torrenteknek. Az a jelölő, amelyik minden kijelölt torrenten rajta van, be van jelölve; kiválasztása mindegyikről leveszi. Az a jelölő, amelyik csak néhányon van rajta, nincs bejelölve (a macOS-alkalmazás vegyes jelet mutat), és kiválasztása mindegyiknek odaadja.
- A **Jelölők > Minden jelölő eltávolítása** minden jelölőt leveszi a kijelölt torrentekről. Maguk a jelölők megmaradnak.

Ha egyetlen torrenten sincs már rajta, a jelölő a kategórialistában marad, 0 darabszámmal. Az eltávolításához kattintson rá a jobb gombbal a kategórialistában, és válassza a **Jelölő törlése** parancsot: a Rookparcel megkérdezi: „Törli a(z) „*név*” jelölőt? Minden torrent, amelyen rajta van, elveszíti.” Maguk a torrentek megmaradnak. Az ottani menü **Új jelölő...** parancsa úgy hoz létre jelölőt, hogy egyetlen torrentnek sem adja oda.

A név nem lehet üres, és nem tartalmazhat vesszőt, tabulátort vagy sortörést. A két végén lévő szóközöket a program elhagyja, a 128 bájtnál hosszabb nevet pedig levágja. A nevek kis- és nagybetűre érzékenyek: a `tv` és a `TV` két külön jelölő. Legfeljebb 1024 jelölő lehet.

A **Jelölők** oszlop egy torrent jelölőit mutatja; kezdetben rejtett (lásd [A főablak](A-főablak#oszlopok)). A keresőmező csak a torrentek nevében keres, a jelölők között nem; kattintson inkább egy jelölőre a kategórialistában. Az Új torrent hozzáadása ablakban nincsenek jelölők: azokat a jobb gombos menüből adhatja, amint a torrent a listában van. A qBittorrent API-ját használó alkalmazások, például a Sonarr, szintén adhatnak jelölőket ([Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr#kategóriák-címkék-és-jelölők)).

## Egy torrent sebességkorlátja

A **Sávszélesség > Letöltési korlát** és a **Feltöltési korlát** a következőket kínálja: **Korlátlan**, 10, 25, 50, 100, 250, 500, 1000, 2500 és 5000 kB/s. Az aktuális választás be van jelölve. Más szám megadásához használja a [Jellemzők](Torrent-részletek-és-fájlok#torrent-jellemzői) ablakot. A Tulajdonságokban beállított általános korlátok emellett továbbra is érvényesek. Lásd a [Sebességkorlátok és ütemező](Sebességkorlátok-és-ütemező) oldalt.

## Kezdeti megosztás

A **Haladó > Kezdeti megosztás** olyan torrentre való, amelyet Ön oszt meg elsőként. Minden peer olyan részeket kap, amelyek még senki másnál nincsenek meg, így az új torrent gyorsabban terjed. Csak akkor kapcsolható be, ha minden kijelölt torrent megosztás alatt áll. Az állapot bekapcsolva **[S] Megosztás**. A kikapcsoláshoz válassza ki újra.

## Letöltési sorrend

*Új az 1.2.63-ban.*

Alapértelmezés szerint a Rookparcel azokat a részeket tölti le előbb, amelyek a peerek között a legritkábbak. Ez minden részt elérhetően tart a swarmban, és mindenkinek a legjobb, aki a torrentet osztja. Két menüpont a **Haladó** alatt megváltoztatja a sorrendet, főleg azért, hogy egy videót nézhessen letöltés közben. Mindegyik be van jelölve, amíg be van kapcsolva; a kikapcsoláshoz válassza ki újra.

- A **Letöltés sorrendben** a részeket sorrendben tölti le, az adatok elejétől a végéig, így a lejátszó játszani tudja a videót, ahogy megérkezik. A magasabb prioritású fájlok továbbra is előbb jönnek. Ennek ára van a swarm számára: a ritka részek, amelyeket egyébként továbbadott volna, későn érkeznek meg. Olyan torrentnél használja, amelyet előnézni szeretne, egyébként hagyja kikapcsolva.
- Az **Első és utolsó részek letöltése előbb** minden kért fájl első és utolsó részeit tölti le a többi előtt: a fájl mindkét végén az 1 %-át, legalább egy részt. Sok videóformátum a fejlécét és az indexét itt tartja, így a lejátszó meg tudja nyitni a befejezetlen fájlt. A kihagyott fájlokat békén hagyja.

Mindkettő szürke, ha minden kijelölt torrent kész. Minden torrent megtartja a választását az újraindítás után. Torrent hozzáadásakor is beállíthatja őket, az [Új torrent hozzáadása ablakban](Torrentek-hozzáadása#a-torrent-beállításai), valamint a [Jellemzőkben](Torrent-részletek-és-fájlok#torrent-jellemzői).

## Jellemzők

A **Jellemzők** (<kbd>Enter</kbd>, vagy dupla kattintás egy nem kész torrenten) megnyitja a Torrent jellemzői ablakot: trackerek, sebességkorlátok, feltöltési helyek, a megosztási cél, DHT és Peercsere, valamint a letöltési sorrend ehhez a torrenthez. Lásd [Torrent-részletek és fájlok](Torrent-részletek-és-fájlok#torrent-jellemzői).

## A GTK-s változatban (Linux, BSD)

- Ugyanaz a jobb gombos menü, ugyanabban a sorrendben, azzal a különbséggel, hogy a **Magnet hivatkozás másolása** és a **.torrent exportálása...** külön csoportban áll közvetlenül a **Haladó** előtt. A macOS-alkalmazás is ide teszi őket.
- Billentyűparancsok: a <kbd>Delete</kbd> eltávolít, a <kbd>Shift</kbd>+<kbd>Delete</kbd> eltávolít és töröl a fájlokat, a <kbd>Ctrl</kbd>+<kbd>A</kbd> mindent kijelöl. Az Indításnak, a Szünetnek és a Leállításnak nincs billentyűparancsa.
- A **Mappa megnyitása** a mappát a fájlkezelőjében nyitja meg.
- Linuxon és BSD-n a törölt fájlok a lemezről törlődnek, nem a kukába kerülnek.

## A webes felületen

- Ugyanaz a jobb gombos menü, azzal a különbséggel, hogy a **Mappa megnyitása** szürke („böngészőből nem”), a **Magnet hivatkozás másolása** és a **.torrent exportálása...** pedig közvetlenül a **Haladó** előtt áll. A **Jelölők** először az **Új jelölő...** és a **Minden jelölő eltávolítása** parancsot sorolja fel, majd a jelölőket.
- A **.torrent exportálása...** a fájlt a böngészőn keresztül, a böngésző letöltési mappájába tölti le. Több torrentnél a böngésző egymás után tölti le az egyes fájlokat, kattintásonként legfeljebb 50-et.
- A **Magnet hivatkozás másolása** a böngésző vágólap-hozzáférését igényli. Egy másik számítógépről sima `http://` címen megnyitott oldalnak nincs ilyen, ezért az oldal ilyenkor magát a hivatkozást mutatja „Másolja ki az alábbi hivatkozást:” felirat alatt, hogy kézzel másolhassa.
- A **Letöltési hely megadása** a kiszolgáló egy mappáját kéri, útvonalként beírva.
- A törölt fájlok a lemezről törlődnek, nem a kukába kerülnek.
- Telefonon a menüből hiányzik a **Mappa megnyitása**, és a **Részletek** paranccsal kezdődik.

## Távoli ablakban

A **Fájl > Csatlakozás távoli géphez...** paranccsal kapcsolódott ablakban a fájlok a másik számítógépen vannak. A **Mappa megnyitása** szürke, a **Letöltési hely megadása...** pedig a kiszolgáló egy mappáját kéri. Lásd a [Távvezérlés](Távvezérlés) oldalt.

## Kapcsolódó oldalak

- [A főablak](A-főablak)
- [Torrent-részletek és fájlok](Torrent-részletek-és-fájlok)
- [Várólista és seedelés](Várólista-és-seedelés)
- [Billentyűparancsok](Billentyűparancsok)
