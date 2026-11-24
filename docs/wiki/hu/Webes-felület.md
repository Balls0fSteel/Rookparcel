# Webes felület

*Ez a [Web UI](Web-UI) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

A webes felület a Rookparcel főablaka egy böngészőben: a torrentlista, a kategóriák, a részletező lapok, a jobb gombos menü, a Hozzáadás, a Tulajdonságok és az RSS letöltő. Ez az oldal azt mutatja be, hogyan kapcsolható be a Windows-programban és a `rookparcel-nox`ban, hogyan jelentkezhet be, hogyan működik a HTTPS, mit tud az oldal, és hogyan használhatja telefonon.

Kétféleképpen érheti el:

- **A Windows-programból.** A `rookparcel.exe` futás közben maga szolgálja ki az oldalt, az ablak saját torrentjei fölött (**Beállítások > Tulajdonságok > Webes felület**).
- **A `rookparcel-nox` programból**, a fej nélküli Rookparcelből, amelyet a `--webui` kapcsolóval indít ([A rookparcel-nox futtatása](A-rookparcel-nox-futtatása)). Linux-, BSD- és macOS-kiszolgálókon, valamint olyan Windows-gépen ez a módja, amelynek ablak nélkül is tovább kell töltenie.

A GTK-program (Linux, BSD) és a macOS-alkalmazás nem szolgál ki saját webes felületet. Mindkettő a [Távvezérlés](Távvezérlés) segítségével vezérelhet egy `rookparcel-nox` példányt.

## Bekapcsolás a Windows-programban

![Tulajdonságok > Webes felület](images/prefs-webui.png)

1. Nyissa meg a **Beállítások > Tulajdonságok** ablakot, és válassza a **Webes felület** lapot.
2. Jelölje be a **Webes felület bekapcsolása** négyzetet.
3. A **Hitelesítés** alatt adjon meg egy **Felhasználónév:** és egy **Jelszó:** értéket. Mindkettő kell; nélkülük az oldal ezt mondja: „Adjon meg egy felhasználónevet és egy jelszót a webes felülethez.” (A felhasználónévként az `admin` van felkínálva.) A jelszót csak hash formájában tárolja. Később hagyja üresen a mezőt, hogy a tárolt jelszó megmaradjon.
4. A **Kapcsolódás** alatt ellenőrizze a **Figyelő port:** értékét. Felkínált érték a 8080, vagy a következő szabad port, ha egy másik program vagy [profil](Profilok) foglalja.
5. Kattintson az **Alkalmaz** vagy az **OK** gombra. Az alsó sor ezt írja: „A webes felület fut: ...” a címével.
6. Kattintson a **Webes felület megnyitása** gombra, hogy megnyissa a böngészőben.

Ha a webes felület nem tud elindulni (például foglalt a port), az oldal állapotsora megmondja az okot. A többi oldal beállításai ettől függetlenül érvénybe lépnek.

A webes felület csak addig fut, amíg a Rookparcel fut. Zárja be az ablakot a tálcára, hogy fent maradjon ([Tálca, értesítések és indítás](Tálca-értesítések-és-indítás)). Újraindul, amikor módosításokat alkalmaz ezen az oldalon.

Az oldal többi beállítása:

| Beállítás | Mit csinál |
|---|---|
| **Kapcsolatok engedélyezése más gépekről** | Minden hálózati címen figyel, így nemcsak ez a számítógép, hanem más számítógépek és telefonok is elérik az oldalt. Ehhez a `rookparcel.exe`-hez Windows tűzfalszabály is kell ([Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal)). |
| **HTTPS használata (önaláírt tanúsítvány)** | HTTPS-en szolgálja ki az oldalt, olyan tanúsítvánnyal, amelyet a Rookparcel maga készít, és a beállítások mappájában tart. A böngészők figyelmeztetnek egy ilyen tanúsítványra, amíg el nem fogadja. |
| **Kiszolgálótartományok (több bejegyzést pontosvesszővel válasszon el):** | Azok a gazdanevek, amelyekre az oldal az IP-címeken és a `localhost` néven kívül válaszol, például e számítógép neve. Más neveket elutasít. Lásd: [Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr#gazdanevek-és-docker). |
| **API-kulcs elfogadása (Authorization: Bearer)** és **API-kulcs (legalább 16 karakter):** | Lehetővé teszi, hogy szkriptek és alkalmazások, például a Sonarr vagy a Radarr ezzel a kulccsal használják a webes felületet bejelentkezés helyett. Alapértelmezés szerint ki van kapcsolva. A kulcsot csak hash formájában tárolja, és többé nem mutatja; hagyja üresen a mezőt, hogy megmaradjon. |
| **Nincs hitelesítés ezen alhálózatok klienseinek (...)** | Az ezekben a hálózatokban (például `192.168.1.0/24`) lévő kliensek bejelentkezés nélkül, teljes vezérléssel használják a webes felületet. Csak olyan hálózatokat soroljon fel, amelyekben ugyanannyira megbízik, mint a jelszóban. |

Miben különbözik a noxéhoz képest az oldal, ha az ablak szolgálja ki:

- A torrentek, címkék, jelölők, beállítások és RSS-csatornák az ablak sajátjai. A böngészőben végzett módosítás látszik az ablakban, és fordítva. Az oldal RSS letöltője az ablak csatornáit és szűrőit szerkeszti.
- Az ablak ütemezője, megosztási célja és a kész letöltések áthelyezési mappája végzi a munkát, ahogy a webes felület nélkül is.
- Nincsenek alternatív sebességkorlátok.
- Az az ablak, amely egy távoli noxot vezérel ([Távvezérlés](Távvezérlés)), nem szolgálhatja ki a webes felületet: „Távoli ablak nem szolgálhatja ki a webes felületet; használja a helyi ablak Tulajdonságok ablakát.”

## Bekapcsolás a rookparcel-noxban

Indítsa a noxot a `--webui` kapcsolóval és egy porttal:

```sh
rookparcel-nox run --save ~/Downloads --webui 8080
```

| `--webui` értéke | Ki éri el az oldalt |
|---|---|
| `8080` | Csak ez a számítógép (`127.0.0.1:8080`). |
| `0.0.0.0:8080` | Minden számítógép, amely eléri ezt. |
| `192.168.1.10:8080` | Azok a számítógépek, amelyek ezen a címen érik el ezt. |

A bejelentkezési név `admin`, hacsak nem ad meg `--webui-user` kapcsolót. Az első indításkor, amikor még nincs tárolt jelszó, kiír egy ideiglenest:

```text
* Web UI: no password set; temporary password for this run: ... (set one with --webui-pass)
```

Jelentkezzen be vele, majd állítsa be a sajátját az oldal **Tulajdonságok > Webes felület** lapján (a **Jelszó:** mező, az ideiglenes jelszóval mint **Jelenlegi jelszó:**). A `--webui-pass` is beállítja, de a parancssorban megadott jelszót a számítógép más felhasználói láthatják.

A szolgáltatásként futó nox az ideiglenes jelszót a naplójába vagy a journalba írja: lásd [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

## Bejelentkezés

Nyissa meg a címet egy böngészőben: `http://127.0.0.1:8080/` ugyanazon a számítógépen, vagy `http://<a számítógép címe>:8080/` egy másikról (`https://`, ha a HTTPS be van kapcsolva). Adja meg a **Felhasználónév** és a **Jelszó** mezőt, és kattintson a **Bejelentkezés** gombra. A **Kijelentkezés** az eszköztár jobb végén van.

- Egy címről öt egymás utáni hibás bejelentkezés után az a cím egy órára ki van zárva. Mindkét szám a **Tulajdonságok > Webes felület** lapon található.
- A bejelentkezés egy órával azután ér véget, hogy az a böngésző utoljára kérést küldött (**Munkamenet időkorlátja (másodperc):**).
- Új jelszó beállítása minden más böngészőt és távoli ablakot kijelentkeztet; amelyik megváltoztatta, bejelentkezve marad.
- Amikor a munkamenet véget ér (az időkorlát után vagy a **Kijelentkezés** gombbal), az oldal bezárja a nyitott párbeszédablakait, és megmutatja a bejelentkezési űrlapot. A hozzáadásra váró `.torrent` fájl a sorban marad, és a Hozzáadás ablaka a következő bejelentkezés után megnyílik.
- Ha a Rookparcellel megszakad a kapcsolat, az állapotsor ezt mutatja: „Bontva, újracsatlakozás ... mp múlva”, és az oldal magától újra csatlakozik.

## HTTPS

HTTPS nélkül a jelszó és minden, amit az oldal mutat, titkosítatlanul halad át a hálózaton. A saját számítógépén ez nem számít; hálózaton keresztül kapcsolja be a HTTPS-t.

- **Windows-program:** jelölje be a **HTTPS használata (önaláírt tanúsítvány)** négyzetet a **Tulajdonságok > Webes felület** lapon.
- **nox:** adja hozzá a `--webui-https` kapcsolót. A nox az első használatkor tanúsítványt készít, a beállítások mappájában tartja (`webui-cert.pem`, `webui-key.pem`), és egy hónappal a lejárat előtt megújítja. Minden indításkor kiír egy ilyen sort: `* Web UI: certificate SHA-256 ...`.
- **nox saját tanúsítvánnyal:** `--webui-cert FILE --webui-key FILE` (PEM; a kulcs nem lehet titkosítva).

A böngésző az első alkalommal figyelmeztet az önaláírt tanúsítványra. Hasonlítsa össze a böngésző által mutatott ujjlenyomatot a nox által kiírt `certificate SHA-256` sorral, majd fogadja el. Bekapcsolt HTTPS mellett ugyanarra a portra érkező sima `http://` cím át lesz irányítva a `https://` címre.

## Mit tud az oldal

Az oldal az asztali ablakot követi: ugyanazok az oszlopok, menük, szavak és sorrend.

- **Eszköztár:** **Torrent hozzáadása**, **URL hozzáadása**, **Eltávolítás**, **Indítás**, **Szünet**, **Leállítás**, **Fel** és **Le** (várólista-sorrend), **RSS**, **Tulajdonságok** (<kbd>Ctrl</kbd>+<kbd>P</kbd>), a listát szűrő keresőmező, a **Keresés**, és a **Kijelentkezés**.
- **Kategóriák** a bal oldalon a számlálóikkal, köztük a címkék, a jelölők és az RSS-csatornák. Kattintson jobb gombbal a **Jelölők** ágra (telefonon hosszan nyomva) az **Új jelölő...** parancsért, vagy egy jelölőre a **Jelölő törlése** parancsért (*a jelölők az 1.2.63-ban újak*).
- **A torrentlista:** kattintson egy fejlécre a rendezéshez, húzza a fejléceket az átrendezéshez vagy átméretezéshez, kattintson jobb gombbal egy fejlécre az oszlopok kiválasztásához (az **Oszlopok visszaállítása** visszaállítja őket).
- **A részletező lapok** a lista alatt: Általános, Trackerek, Peerek, Részek, Fájlok, Sebesség és Napló ([Torrent-részletek és fájlok](Torrent-részletek-és-fájlok)).
- **A jobb gombos menü** azt tartalmazza, amit az ablak menüje: **Kényszerített indítás**, **Indítás**, **Szünet**, **Leállítás**, a várólista-mozgatások, **Eltávolítás** és **Eltávolítás és > Adatok törlése**, **Kényszerített ellenőrzés** (a leállított torrentet elindítja, mint az asztali programban; *új az 1.2.76-ban*), **Tracker frissítése**, **Magnet hivatkozás másolása**, **.torrent exportálása...**, **Haladó > Letöltési hely megadása...**, **Kezdeti megosztás**, **Letöltés sorrendben** és **Első és utolsó részek letöltése előbb**, **Sávszélesség**, **Címke**, **Jelölők** és **Jellemzők** ([Torrentek kezelése](Torrentek-kezelése)). A **Mappa megnyitása** szürke: a böngésző nem tud megnyitni egy mappát a kiszolgálón. *A Magnet hivatkozás másolása, a .torrent exportálása és a két letöltési sorrendi tétel az 1.2.63-ban új.* Az exportálás a `.torrent` fájlt a böngészőn keresztül tölti le. Ahol a böngésző nem ad vágólapot az oldalnak (sima `http://` egy másik számítógépről), a Magnet hivatkozás másolása megmutatja a hivatkozásokat, hogy kézzel másolja őket.
- **Torrentek hozzáadása:** a **Torrent hozzáadása** a böngészőt futtató számítógépről tölt fel `.torrent` fájlokat. Az **URL hozzáadása** magnet-hivatkozást, info-hasht vagy egy torrentre mutató `http://` hivatkozást fogad. `.torrent` fájlokat vagy magnet-hivatkozást az oldalra is ejthet, vagy beillesztheti a magnet-hivatkozást szövegmezőn kívül bárhol. Ha az „Ablak megjelenítése a torrentben lévő fájlokkal” be van kapcsolva (**Tulajdonságok > Felület**, alapértelmezés szerint be van), egy Hozzáadás párbeszédablak ugyanazt kérdezi, mint az asztali: mentési mappa, név, címke, mely fájlok, indítás vagy nem. Magnet-hivatkozásnál akkor nyílik meg, amikor a torrent fájllistája megérkezett. Azok a fájlok, amelyekre a **Ne töltse le az ilyen nevű fájlokat:** (**Tulajdonságok > Felület**) illeszkedik, kijelöletlenül indulnak benne, és az ablak nélkül hozzáadott torrentek kihagyják őket (*új az 1.2.63-ban*). Lásd: [Torrentek hozzáadása](Torrentek-hozzáadása#név-szerint-kihagyott-fájlok).
- **Torrent jellemzői:** a kijelölt torrentek trackerei, sebességkorlátai, feltöltési helyei, megosztási célja és letöltési sorrendje.
- **Tulajdonságok:** az Általános, Felület, Mappák, Kapcsolat, Sávszélesség, BitTorrent, Sorba állítás, Ütemező, Webes felület, Játék, Haladó és Lemezgyorsítótár lap. Tartsa az egeret egy beállítás fölött a súgójáért, vagy nyomja meg az <kbd>F1</kbd> billentyűt (*a súgószövegek az 1.1.57-ben újak*). A mappák a kiszolgálón lévő mappák, szövegként megadva. A Windows-program által kiszolgált webes felület BitTorrent lapján nincs IP-szűrő cím, **Frissítés most** gomb vagy állapotsor: ezeket az ablak saját Tulajdonságok ablaka tartalmazza. Lásd: [A beállítások leírása](A-beállítások-leírása).
- **RSS letöltő:** csatornák, szűrők és előzmények, mint az asztali programban ([RSS letöltő](RSS-letöltő)).
- A **Keresés** megnyitja a Torrentek keresése ablakot, amely a Prowlarrban vagy Jackettben keres; ott a **Források kezelése...** adja hozzá őket ([Torrentek keresése](Torrentek-keresése)). Forrás hozzáadása, illetve címének vagy kulcsának módosítása a webes felület jelenlegi jelszavát kéri (a keresősegéd telepítése is): a hibás jelszó kijelentkezteti azt a böngészőt, és sikertelen bejelentkezésnek számít.

Billentyűk a listában: a nyilak, a <kbd>Home</kbd>, az <kbd>End</kbd>, a <kbd>Page Up</kbd> és a <kbd>Page Down</kbd> mozgat; a <kbd>Shift</kbd> és a <kbd>Ctrl</kbd> kattintással vagy billentyűvel kiterjeszti a kijelölést; a <kbd>Ctrl</kbd>+<kbd>A</kbd> kijelöli az összes, a szűrő által mutatott torrentet; a <kbd>Space</kbd> szüneteltet; a <kbd>Delete</kbd> eltávolít; a <kbd>Shift</kbd>+<kbd>Delete</kbd> eltávolít és törli az adatokat; a <kbd>Menu</kbd> billentyű vagy a <kbd>Shift</kbd>+<kbd>F10</kbd> megnyitja a jobb gombos menüt.

Az oldal a **Tulajdonságok > Általános** lapon, a „Felület:” mezőben beállított nyelvet használja. Alapértelmezés szerint sötétre vált, ha a böngészője vagy a rendszere sötét témára van állítva. A **Tulajdonságok > Felület > Téma:** mindenkinek témát választ, aki megnyitja ezt a webes felületet: **Rendszer (világos vagy sötét)**, **Világos**, **Sötét**, **Éjfél**, **Pala**, **Solarized sötét**, **Solarized világos**, **Nagy kontraszt** vagy **Postás varjú**. A **Téma ebben a böngészőben:** ugyanazon az oldalon (telefonon a Továbbiak menü **Téma** tétele) csak ennek a böngészőnek választ, az eszközön tárolva; a **A kiszolgáló alapértelmezése** választás ismét a kiszolgáló választását követi. Saját téma egy CSS-fájl, amelyet a beállítások mappája alatti `webui/themes` mappába kell tenni (a `rookparcel-nox` esetében `--config-dir`; a Windows-program beállítási mappája, amikor az szolgálja ki a webes felületet): másolja át a beépített fájlok egyikét a forrásfából ([crates/nox/src/webui/themes](https://github.com/Balls0fSteel/Rookparcel/tree/main/crates/nox/src/webui/themes)), módosítsa a színeket és a `/* name: ... */` sort, a fájlt kisbetűkből, számjegyekből és kötőjelekből álló névvel nevezze el, és ezen a néven jelenik meg a listában.

## Telefonon

Ugyanaz a cím működik telefonon is; keskeny képernyőn (768 képpontnál keskenyebben) az oldal megváltoztatja az elrendezését.

- A lista minden torrent nevét egy folyamatjelző sáv fölött mutatja az állapotával, a sebességekkel és a hátralévő idővel.
- A kategóriák egy fiókban vannak, az eszköztár elején lévő menügomb mögött.
- Koppintson egy torrentre, hogy a részletei egy alulról érkező lapon megjelenjenek.
- Az egyes sorokon lévő gomb megnyitja a menüjét. A hosszú nyomás több torrent kijelölését indítja; koppintson továbbiakra a hozzáadásukhoz, és a **Kész** befejezi a kijelölést.
- Az eszköztár a **Hozzáadás**, az **Indítás**, a **Szünet** és az **Eltávolítás** gombot tartja meg; a többi, és a rendezés, a **Továbbiak** alatt van.
- Androidon a Vissza gomb bezárja a legfelső fiókot, lapot vagy párbeszédablakot.

Ahhoz, hogy a telefon elérje az oldalt, a webes felületnek el kell fogadnia más számítógépek kapcsolatait (**Kapcsolatok engedélyezése más gépekről**, vagy a nox `--webui 0.0.0.0:PORT` kapcsolója), és a telefonnak olyan hálózaton kell lennie, amely eléri a számítógépet.

## Biztonság

A webes felület a torrentjeit vezérli, és bárhová írhat fájlokat, ahová a Rookparcel fiókja. Mielőtt hálózatnak megnyitja, tartsa szem előtt:

- Használjon hosszú jelszót, és a saját számítógépén túlmenően kapcsolja be a HTTPS-t.
- Az alapértelmezés szerint bekapcsolt **A Host fejléc ellenőrzésének bekapcsolása** elutasítja azokat a kéréseket, amelyek bármi másra címzettek, mint egy IP-cím, a `localhost` vagy egy felsorolt kiszolgálótartomány. A **Védelem a webhelyek közötti kéréshamisítás (CSRF) ellen** (szintén alapértelmezés szerint bekapcsolt) elutasítja a másik webhely oldaláról érkező módosításokat. Hagyja mindkettőt bekapcsolva.
- A A **Nincs hitelesítés a localhost klienseinek** (a nox oldalán; alapértelmezés szerint ki van kapcsolva) bejelentkezés nélkül beengedi az ugyanazon a számítógépen futó programokat. Ez és az alhálózati lista soha nem vonatkozik a fordított proxyn át érkező kérésekre.
- A bejelentkezés, az API-kulcs, az alhálózatok vagy az oldal többi biztonsági beállításának módosításához a **Jelenlegi jelszó:** szükséges. A hibás jelszó kijelentkezteti azt a böngészőt, és sikertelen bejelentkezésnek számít, így az ismételt találgatás ugyanúgy kizárja a címet, mint a sikertelen bejelentkezések. Ugyanez igaz azokra a címekre, amelyekről a kiszolgáló letölt, például a trackerlista címére és az IP-szűrő tiltólistájának címére (Tulajdonságok > BitTorrent, *új az 1.2.63-ban*; lásd: [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal#lista-webcímről)).
- A noxnál a `--webui-save-root DIR` korlátozza azokat a mappákat, ahová a webes felület menthet, áthelyezhet, és ahonnan olvashat ([A rookparcel-nox futtatása](A-rookparcel-nox-futtatása#webes-felület)).

Ha ugyanazon a számítógépen fordított proxy mögött fut, a proxy állítsa be az `X-Forwarded-For` fejlécet, hogy a sikertelen bejelentkezések a megfelelő klienst zárják ki, és ne mindenkit. Ha az **API-kulcs elfogadása (Authorization: Bearer)** be van kapcsolva, a proxy nem adhat hozzá saját `Authorization` fejlécet: mielőtt egy böngésző bejelentkezett, az ilyen fejléc hibás kulcsnak számít.

## Lásd még

- [Távvezérlés](Távvezérlés): az asztali program ablaka vezérel egy noxot, az oldal alternatívájaként.
- [Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr): a webes felület qBittorrent-kompatibilis API-jának használata.
- Az oldal mögötti tervezés: [WEBUI.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/WEBUI.md).
