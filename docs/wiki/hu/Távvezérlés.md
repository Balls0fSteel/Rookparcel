# Távvezérlés

*Ez a [Remote control](Remote-control) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

Az asztali program egy másik számítógépen futó `rookparcel-nox` példányt is vezérelhet: NAS-t, otthoni kiszolgálót, egy másik szoba gépét. Az ablak a megszokott módon néz ki és működik, de a torrentek, a letöltések és a motor a kiszolgálón vannak. Ez az oldal azt mutatja be, hogyan csatlakozhat a **Fájl > Csatlakozás távoli géphez...** paranccsal vagy a `--remote` kapcsolóval, mi működik egy távoli ablakban, és hogyan védett a kapcsolat.

Az alternatíva a böngészőben futó [webes felület](Webes-felület), amelyhez semmit sem kell telepíteni. A távoli ablak ehelyett a teljes asztali programot adja: a párbeszédablakait, a billentyűparancsait és a részletező lapjait.

## Mire van szüksége

- Egy `rookparcel-nox`, amelynek be van kapcsolva a webes felülete, például `rookparcel-nox run --webui 0.0.0.0:8443 --webui-https` ([A rookparcel-nox futtatása](A-rookparcel-nox-futtatása)). A távoli ablak a webes felület portját és bejelentkezését használja.
- A webes felület felhasználóneve és jelszava.
- Mindkét oldalon ugyanaz a Rookparcel-verzió. A más verziót beszélő kiszolgáló elutasítja a kapcsolatot ezzel: „the server refused the connection (HTTP 426): this server speaks remote protocol ...”. A távoli protokoll időnként változik (utoljára az 1.2.70-ben), az 1.2.76 pedig nem csatlakozik az 1.2.63-tól az 1.2.69-ig terjedő verziókhoz, ezért a kiszolgálót és a távoli ablakokat futtató számítógépeket együtt frissítse. A webes felületet és a qBittorrent-kompatibilis API-t ez nem érinti.

A távoli ablak a `rookparcel-nox` programhoz csatlakozik. A Windows-program saját webes felülete (**Tulajdonságok > Webes felület**) böngészőknek szól.

## Csatlakozás

1. A Windows-programban válassza a **Fájl > Csatlakozás távoli géphez...** parancsot. Egy második Rookparcel-ablak indul a saját mellett, és megmutatja a **Csatlakozás távoli géphez** párbeszédablakot. (Vagy indítsa a `rookparcel.exe --remote` parancsot, amely csak a távoli ablakot nyitja meg.)
2. Töltse ki:

   | Mező | Mit adjon meg |
   |---|---|
   | **Gép:** | A kiszolgáló neve vagy IP-címe. |
   | **Port:** | A webes felület portja (a 8080 van felkínálva). |
   | **Felhasználónév:** | A webes felület felhasználóneve (az `admin` van felkínálva). |
   | **Jelszó:** | A webes felület jelszava. |
   | **HTTPS használata** | Alapértelmezés szerint bejelölve. Hagyja bejelölve, ha a kiszolgáló `--webui-https` kapcsolóval vagy saját tanúsítvánnyal fut. |

3. Kattintson a **Csatlakozás** gombra.
4. Az első alkalommal az önaláírt tanúsítvánnyal rendelkező kiszolgálót (amilyet a `--webui-https` készít) az ujjlenyomatával mutatja:

   > ... olyan tanúsítványt mutat, amelyben ez a számítógép nem bízik (önaláírtat, amilyet a rookparcel-nox --webui-https készít).

   Hasonlítsa össze a hosszú számot a nox által indításkor kiírt `certificate SHA-256` sorral (a terminálján, a naplójában vagy a journalban). Ha egyeznek, kattintson az **Igen** gombra. A Rookparcel megjegyzi a tanúsítványt az adott kiszolgálóhoz, és nem kérdez újra.
5. Az ablak megnyílik a kiszolgáló torrentjeivel. A címsora a kiszolgáló `host:port` értékét mutatja.

A párbeszédablak megjegyzi a gépet, a portot, a felhasználónevet és a HTTPS-választást a következő alkalomra. A jelszót soha nem tárolja; minden csatlakozáskor be kell írnia.

A GTK-programban (Linux, BSD) ugyanez van: **Fájl > Csatlakozás távoli géphez...**, vagy `rookparcel-gtk --remote`. A macOS-alkalmazásban is van **Fájl > Csatlakozás távoli géphez...**. Minden távoli ablak önálló program, így egyszerre nyitva lehet a helyi ablaka és több kiszolgáló is.

## HTTPS nélkül

Ha törli a **HTTPS használata** jelölését, és a gép egy másik számítógép, a Rookparcel előbb rákérdez:

> ... egy másik számítógép, és HTTPS nélkül a felhasználónév, a jelszó és minden parancs titkosítatlanul halad át a hálózaton: bárki a hálózaton elolvashatja vagy megváltoztathatja őket.

Válaszoljon **Nem**mel, és inkább a kiszolgálón kapcsolja be a HTTPS-t (`--webui-https`), hacsak a kettő közötti hálózat nem teljes egészében az Öné. Az ugyanarra a számítógépre mutató kapcsolatnál (`localhost`, `127.0.0.1`) nem kérdez.

## Mi működik egy távoli ablakban

Szinte minden úgy működik, mint a helyi ablakban, a kiszolgálón végrehajtva:

- Torrentek hozzáadása: a `.torrent` fájlokat erről a számítógépről küldi el a kiszolgálónak; a magnet-hivatkozásokat is.
- Indítás, leállítás, szüneteltetés, kényszerített indítás, eltávolítás (az adatokkal együtt is, a kiszolgálón törölve), újraellenőrzés, várólista-sorrend, címkék, jelölők, sebességkorlátok, trackerek, fájlprioritások, átnevezés, Torrent jellemzői.
- A részletező lapok a kiszolgáló peerjeivel, részeivel, fájljaival és sebességgrafikonjával.
- A **Magnet hivatkozás másolása** és a **.torrent exportálása...** (*új az 1.2.63-ban*). Az exportált `.torrent` fájl ezen a számítógépen kerül mentésre.
- **Tulajdonságok:** a motor beállításai (kapcsolat, sávszélesség, BitTorrent, sorba állítás és hasonlók) a kiszolgálóéi. Amit ott módosít, a kiszolgálón lép érvénybe és ott marad meg.

Ami különbözik:

| | Távoli ablakban |
|---|---|
| Mappák | A mappák a kiszolgálón vannak. A torrentek mappaválasztói ki vannak kapcsolva; a **Letöltési hely megadása...** egy kiszolgálómappát kér szövegként. Az új letöltések a kiszolgáló alapértelmezett mappájába kerülnek. |
| **Mappa megnyitása** | Ki van kapcsolva: a mappa a kiszolgálón van. |
| Torrentek készítése | Az itt készített torrentet a kiszolgáló nem tudja megosztani, mivel a fájlok ezen a számítógépen vannak. |
| IP-szűrő | A kiszolgáló saját IP-szűrője érvényes; az ablaké nem kerül elküldésre. |
| Importálás | A **Fájl > Importálás** nem csinál semmit (a Windows- és macOS-programban szürke): a kiszolgáló nem vesz át torrenteket egy másik kliens mentett állapotából. |
| Mappakorlátok | A `--webui-save-root` kapcsolóval indított kiszolgáló elutasítja azt a hozzáadást vagy áthelyezést, amely ezeken a gyökereken kívüli mappába történne, ahogy a webes felületnél is. Elutasítja azt az átnevezést is, amely kivinné a torrentet a mappájából. |
| **Tulajdonságok > Webes felület** | A távoli ablak nem szolgálhatja ki a webes felületet: „Távoli ablak nem szolgálhatja ki a webes felületet; használja a helyi ablak Tulajdonságok ablakát.” |
| Címkék, jelölők, RSS-csatornák, ablakbeállítások | Ezen a számítógépen tárolódnak, kiszolgálónként külön (lásd alább). |
| Ütemező, RSS letöltő, megosztási cél, `.torrent` mappák | Az ablak saját példányai futnak, amíg nyitva van, és a kiszolgáló torrentjein hatnak, a nox saját műveletein felül. |

A kiszolgáló az ablak bezárása után is tovább tölt.

## A kapcsolat megszakadása

Amikor a kapcsolat megszakad, a Rookparcel egyszer jelzi:

> A kapcsolat megszakadt ezzel: ...: ... A lista a torrenteket a korábbi állapotukban mutatja. Zárja be ezt az ablakot, és csatlakozzon újra a Fájl > Csatlakozás távoli géphez... menüponttal.

A címsor ezután „(bontva)” szóra végződik. A parancsok ettől kezdve nem jutnak el sehová. Zárja be az ablakot, és csatlakozzon újra.

## Ha a csatlakozás nem sikerül

| Üzenet | Mit ellenőrizzen |
|---|---|
| „Adja meg a kiszolgáló nevét vagy címét és portját (1 és 65535 között).” | A gép üres, vagy a port nem szám a megengedett tartományban. |
| „Nem lehet csatlakozni ehhez: ...: wrong user name or password” | A webes felület bejelentkezése. Túl sok hibás próba egy időre kizárja a címet ([Webes felület](Webes-felület#bejelentkezés)). |
| „... Ez egy rookparcel-nox, amelynek webes felülete ezen a porton fut (--webui)?” | Valami más válaszolt azon a porton. |
| „... Ha a kiszolgáló webes felülete HTTPS-t használ (--webui-https), jelölje be a HTTPS használata lehetőséget.” | A kiszolgáló HTTPS-t beszél, de a **HTTPS használata** ki volt kapcsolva. |
| „... nem érhető el HTTPS-en: ...” | A **HTTPS használata** be van kapcsolva, de a kiszolgáló nem beszél HTTPS-t, vagy egyáltalán nem érhető el. |
| „A(z) ... tanúsítványa nem az, amelyet korábban elfogadott: ...” | A kiszolgáló tanúsítványa megváltozott. Ha Ön cserélte le, vagy a nox megújította a sajátját, csatlakozzon újra az új ellenőrzéséhez. Különben valaki közbeékelődhetett: ne folytassa. |
| „... (HTTP 400): ...”, amely a Host fejlécet nevezi meg | A kiszolgáló elutasítja a beírt gépnevet. Használja az IP-címét, vagy adja hozzá a nevet a kiszolgáló **Tulajdonságok > Webes felület > Kiszolgálótartományok** beállításához ([Webes felület](Webes-felület#biztonság)). |
| „... (HTTP 426): this server speaks remote protocol ...” | A két program különböző verziójú. Frissítse a régebbit ([A Rookparcel frissítése](A-Rookparcel-frissítése)). |

A kiszolgáló tűzfalának be kell engednie a webes felület portját.

## Hol tárolja a távoli ablak a fájljait

| | Windows | GTK (Linux, BSD) |
|---|---|---|
| A kapcsolat adatai és az elfogadott tanúsítványok | `remote.ini` a `rookparcel.ini` mellett a beállítások mappájában | `remote.ini` itt: `~/.config/rookparcel/gtk` |
| Az ablak beállításai, címkéi, jelölői és RSS-csatornái kiszolgálónként | `remote\<host>_<port>` a beállítások mappájában | `~/.config/rookparcel/gtk/remote/<host>_<port>` |

Ezek soha nem keverednek a helyi ablak beállításaival. Lásd: [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait).

## Biztonság

- A távoli ablak a webes felület bejelentkezésével jelentkezik be, ezért minden érvényes, ami a [webes felület biztonságáról](Webes-felület#biztonság) szól: hosszú jelszó, HTTPS hálózaton át.
- A hibás jelszavak ugyanabba a kizárásba számítanak bele, mint a webes felület bejelentkezési űrlapja.
- Új webes felületi jelszó beállítása minden távoli ablakot lecsatlakoztat.
- A megváltozott tanúsítványt, miután elfogadta, figyelmeztetéssel elutasítja; a következő próbálkozáskor a Rookparcel újra kérdez.
- A jelszó soha nem kerül lemezre ezen a számítógépen.

## Lásd még

- [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása): a kiszolgáló indítása, a HTTPS-beállítások és a szolgáltatások.
- [Webes felület](Webes-felület): ugyanaz a kiszolgáló egy böngészőben.
- A műszaki leírás: [WEBUI.md, Remote mode](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/WEBUI.md#remote-mode).
