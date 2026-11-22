*Ez az [RSS Downloader](RSS-Downloader) című angol oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# RSS letöltő

Ez az oldal azt mutatja be, hogyan követhet RSS-csatornákat a Rookparcelben, és hogyan tölthet le belőlük: csatornák hozzáadása, elemek kézi letöltése, az egyező elemeket automatikusan letöltő szűrők, epizód- és minőségszűrés, valamint az előzmények. Először a Windows-ablakot tárgyalja, majd a GTK-ablakot (Linux, BSD) és a webes felületet.

Az RSS letöltő úgy működik, mint a µTorrent 2.2.1-é. Sok torrentoldal és tracker csatornát tesz közzé az új kiadásairól; a Rookparcel 15 percenként ellenőrzi a csatornáit, és hozzáadja, amit a szűrői kérnek.

## Az RSS letöltő megnyitása

- **Beállítások > RSS letöltő...**
- Az eszköztár **RSS letöltő** gombja.
- Kattintson a jobb gombbal a **Csatornák** elemre (vagy egy csatornára) a bal oldali kategórialistában, majd az **RSS letöltő...** parancsra.

![Az RSS letöltő ablak](images/rss-downloader.png)

Az ablak bal oldalán a **Csatornák** lista áll, alatta a **Hozzáadás...**, **Szerkesztés...**, **Eltávolítás** és **Frissítés** gombbal, jobb oldalán pedig két lap: **Szűrők** és **Előzmények**. A módosítások azonnal érvénybe lépnek; nincs OK és Mégse, csak **Bezárás**. Az ablak bezárásakor a szűrők lefutnak a csatornák aktuális elemein, így egy új szűrő azonnal tölthet le.

## Csatorna hozzáadása

1. Az RSS letöltőben kattintson a **Hozzáadás...** gombra a **Csatornák** alatt. (Vagy kattintson a jobb gombbal a **Csatornák** elemre a főablak kategórialistájában, és válassza az **RSS-csatorna hozzáadása...** parancsot.)
2. **Csatorna URL-je:** a csatorna címe. Ha a vágólapon van csatornacím, a program kitölti Önnek.
3. **Saját álnév:** a csatorna neve. Üresen hagyva a csatorna saját címe jelenik meg.
4. A **Feliratkozás** alatt válasszon egyet:
   - **Ne töltse le automatikusan az összes elemet** (a szokásos választás): ebből a csatornából csak a szűrői vagy Ön tölt le.
   - **A csatornán megjelenő összes elem automatikus letöltése**: minden új elemet letölt. Az alatta lévő **Okos epizódszűrő használata** minden epizódot csak egyszer tölt le (lásd [Okos epizódszűrő](#okos-epizódszűrő)).
5. Kattintson az **OK** gombra. A csatorna azonnal letöltődik.

Ha az **A csatornán megjelenő összes elem automatikus letöltése** van kiválasztva, a szűrői nem érvényesülnek arra a csatornára: minden újat átvesz.

**Bejelentkezést igénylő csatornák.** Egyes privát trackerek csatornáihoz a bejelentkezése sütijei kellenek. Adja hozzá őket az URL-hez úgy, ahogy a µTorrent tette, a `:COOKIE:` után:

```
https://tracker.example/rss.php:COOKIE:uid=12345;pass=abcdef
```

A sütiket a program csak a csatorna saját webhelyének küldi (ugyanannak a gépnek vagy egy aldomainjének), és `https` csatornáról soha nem küldi sima `http` hivatkozásra.

## Csatornák kezelése

A **Csatornák** lista a **Minden csatorna** elemmel kezdődik, majd az egyes csatornák következnek. A lista alatt egy sor mutatja a kijelölt csatorna állapotát: „Frissítve:” az idővel és az elemek számával, „Még nem frissült”, „Kikapcsolva”, vagy „Hiba:” és az ok. A **Minden csatorna** kijelölésekor azt mutatja, hány csatornája van.

| Gomb vagy menüpont | Mit tesz |
|---|---|
| **Szerkesztés...** / **Csatorna szerkesztése...** | Módosítja az URL-t, az álnevet vagy a feliratkozást. Az új URL új csatorna: a régi elemeit elveti, és újra letölti. |
| **Eltávolítás** / **Csatorna törlése** | Eltávolítja a csatornát, rákérdezés után. Az arra a csatornára beállított szűrők megmaradnak, és semmire sem illeszkednek, amíg át nem állítja őket egy másik csatornára. |
| **Frissítés** / **Csatorna frissítése** | Most letölti a kijelölt csatornát (a **Minden csatorna** kijelölésekor minden csatornát). |
| **Csatorna kikapcsolása** / **Csatorna bekapcsolása** | Leállítja vagy folytatja a csatorna ellenőrzését, eltávolítás nélkül. Csak a jobb gombos menüben. |

Ugyanezekért a pontokért kattintson a jobb gombbal egy csatornára az RSS letöltő Csatornák listájában vagy a főablak kategórialistájában.

A csatornákat 15 percenként ellenőrzi a program. A csatornaletöltések és a `.torrent` fájlok, amelyekre mutatnak, a Tulajdonságokban beállított proxyn mennek keresztül, ha állított be ilyet. A csatorna elemeinek hivatkozásai csak nyilvános címekre vezethetnek, kivéve, ha maga a csatorna ezen a számítógépen vagy a helyi hálózaton lévő kiszolgálóról érkezett.

## Elemek kézi letöltése

A főablak kategórialistájában van egy **Csatornák** ág, alatta az egyes csatornákkal. Kattintson egy csatornára (vagy a **Csatornák** elemre az összesért), és a torrentlista a torrentjei helyett a csatorna elemeit mutatja, a **Név**, **Epizód**, **Formátum**, **Dátum**, **Csatorna** és **URL** oszlopokkal.

- Az **Epizód** azt mutatja, amit a Rookparcel a címből kiolvasott: `1x02`, egy tartomány, például `1x02-03`, egy abszolút szám, például `12` (animék), vagy egy adásnap, például `2026-10-19`.
- A **Formátum** a címben megnevezett minőségeket mutatja, például `720p` vagy `HDTV`.

Egy elem letöltéséhez kattintson rá duplán, vagy kattintson rá a jobb gombbal, és válassza a **Letöltés** parancsot. Előbb több elemet is kijelölhet. A kézzel letöltött elem címkéje a csatorna neve lesz, és az Előzmények közé kerül, ahogy egy szűrő letöltése is.

## Szűrők

A szűrő letölti azokat az elemeket, amelyek címe illeszkedik rá, egy csatornából vagy mindegyikből. Nyissa meg a **Szűrők** lapot, és kattintson a **Hozzáadás** gombra. „Új szűrő” nevű szűrő jelenik meg a listában, bejelölve. Ezután töltse ki a mezőit:

| Mező | Mit tesz |
|---|---|
| **Név:** | A szűrő neve a listában. A lista jelölőnégyzete be- vagy kikapcsolja a szűrőt. |
| **Szűrő:** | Mire kell illeszkednie a címnek. Lásd [A szűrő megadása](#a-szűrő-megadása). Üresen semmire sem illeszkedik; a `*` mindenre illeszkedik. |
| **Kivéve:** | Az erre illeszkedő címeket kihagyja, akkor is, ha a **Szűrő** illeszkedik. Ugyanaz a szintaxis. Üresen semmit sem hagy ki. |
| **Mentés ide:** | A letöltések mappája. Üresen az alapértelmezett letöltési mappát használja. A **...** gomb mappát választ. |
| **Csatorna:** | **Minden csatorna**, vagy egy csatorna. |
| **Minőség:** | Kattintson a gombra, és jelölje be az elfogadott minőségeket. A **Mind** minden címet elfogad, azokat is, amelyek nem neveznek meg minőséget. |
| **Epizódszám:** | Mely epizódokat vegye át, például `1x12-14`. Üresen bármelyiket. Lásd [Epizódszámok](#epizódszámok). |
| **Legkisebb időköz:** | Miután a szűrő letöltött valamit, addig nem tölt le többet, amíg ennyi idő el nem telt. A **0 (mellőzés)** azt jelenti, hogy nincs várakozás. Választható értékek 2 órától 1 hónapig. |
| **Címke:** | A címke, amelyet a letöltések kapnak. Üresen a csatorna nevét használja. A címkékről lásd a [Torrentek kezelése](Torrentek-kezelése) oldalt. |
| **Okos epizódszűrő** | Minden epizódot csak egyszer tölt le, a minőségétől vagy a kiadó csoporttól függetlenül. Lásd lent. |
| **A letöltés legmagasabb prioritást kapjon** | A letöltést a várólista elejére teszi. |
| **Torrent hozzáadása szüneteltetve** | A letöltést elindítás nélkül adja hozzá. |

A mezők alatt az **Egyező elemek a csatornákban:** felsorolja a csatornák utolsó ellenőrzésének azokat az elemeit, amelyekre a szűrő a Szűrő, a Kivéve, a Csatorna, a Minőség és az Epizódszám alapján illeszkedik. Használja a szűrő kipróbálására az ablak bezárása előtt. (A Legkisebb időköz és az okos epizódszűrő attól függ, mi töltődött le korábban, ezért a lista ezeket nem alkalmazza.)

Az **Eltávolítás** törli a kijelölt szűrőt. A **Fel** és a **Le** mozgatja a listában. A sorrend számít: egy elem az első szűrőhöz kerül, amelyik átveszi, ezért a konkrétabb szűrőket tegye az általánosabbak fölé.

A szűrők a csatorna ellenőrzésekor és az RSS letöltő bezárásakor futnak. Minden csatornában a legrégebbi elemeket nézi meg először, így okos epizódszűrő vagy legkisebb időköz esetén egy epizód első kiadása nyer. Egy elemet ugyanabból a csatornából soha nem tölt le kétszer.

### A szűrő megadása

A **Szűrő** és a **Kivéve** mező egyszerű helyettesítő karaktereket használ:

| Amit beír | Jelentése |
|---|---|
| `*` | Tetszőleges karaktersor, akár üres is. |
| `?` | Pontosan egy karakter. |
| `\|` | Alternatívákat választ el: a `show one\|show two` bármelyikre illeszkedik. |

- A kis- és nagybetű nem számít.
- A címben a `.` vagy `_` szóköznek számít, így a `the show` illeszkedik a `The.Show.S01E02.720p` címre.
- A helyettesítő karakter **nélküli** alternatíva a cím bármely részén illeszkedik: a `show` illeszkedik a `The.Show.S01E02` címre.
- A helyettesítő karakterrel **rendelkező** alternatívának a teljes címre kell illeszkednie: a `show*` nem illeszkedik a `The.Show.S01E02` címre, de a `*show*` igen.

Példák:

| Szűrő | Kivéve | Mit vesz át |
|---|---|---|
| `the show` | | minden címet, amely tartalmazza: „the show” |
| `*the show*1080p*` | | azt a sorozatot, csak 1080p-ben |
| `the show` | `*german*\|*sample*` | azt a sorozatot, de német kiadásokat és mintákat (sample) nem |

### Epizódszámok

Az **Epizódszám** mező egy vagy több tartományt fogad, `,` vagy `;` választja el őket:

| Amit beír | Mit vesz át |
|---|---|
| `1x12` vagy `S01E12` | Azt az egy epizódot. |
| `1x12-14` vagy `S01E12-S01E14` | Az 1. évad 12–14. epizódját. |
| `S01E01-10` | Az 1. évad 1–10. epizódját (a kötőjel után az évad elhagyható). |
| `S02` | A 2. évad egészét. |
| `S01E05-` | Az 1. évad 5. epizódját és minden későbbit. |
| `12` | A 12. epizódot bármely évadban, a végig folyamatosan számozott sorozatoknál, például az animéknél. |
| `1-12`, `100-` | Az 1–12. epizódot; a 100. és későbbi epizódokat. |
| `2026.10.19` | Azt az epizódot, amelyet azon a napon adtak le (napi műsoroknál). |

A Rookparcel az epizódokat az `S01E02` (valamint `S01E02E03`, `S01E02-E03`), `1x02`, adásnap (`2026.10.19`, `2026-10-19`) vagy abszolút szám (`Show - 12`, `Episode 12`) alakban írt címekből olvassa ki. A verziószámot, például `Ubuntu - 22.04`, vagy az összefoglalót, például `Show - 12.5`, nem olvassa epizódnak. Az évadcsomag, például `Show.S01.Complete`, nem nevez meg epizódot, ezért az epizódszámos szűrő nem veszi át. Az a mező, amelyet a Rookparcel nem tud kiolvasni, semmire sem illeszkedik.

### Minőségek

A **Minőség** lista: HDTV, TVRip, DVDRip, SVCD, DSRip, DVBRip, PDTV, HR.HDTV, HR.PDTV, DVDR, DVDScr, 720p, 1080i, 1080p, WebRip, SatRip, WEB-DL, BluRay, 480p, 2160p. A cím akkor illeszkedik, ha a bejelölt minőségek valamelyikét megnevezi.

### Okos epizódszűrő

Ha az **Okos epizódszűrő** be van jelölve, a szűrő megjegyzi a letöltött epizódokat, és kihagyja ugyanannak az epizódnak a későbbi kiadásait, a minőségüktől vagy a kiadó csoporttól függetlenül. Ez akkor hasznos, ha egy csatorna ugyanazt az epizódot többször is közli.

- Az epizód nélküli címet mindig átengedi.
- A már letöltött epizód REPACK, PROPER, RERIP vagy REAL jelölésű kiadását egyszer átengedi, mivel az a korábbi kiadást javítja.

A csatorna feliratkozásának **Okos epizódszűrő használata** beállítása ugyanígy működik az olyan csatornánál, amely mindent letölt.

## Előzmények

Az **Előzmények** lap felsorolja, mit adott hozzá az RSS letöltő, a legújabbal kezdve, a **Név**, **Dátum** és **Csatorna** oszlopokkal. Az utolsó 1000 bejegyzést őrzi meg. Az **Előzmények törlése** kiüríti a listát; ez nem okozza ezeknek az elemeknek az újbóli letöltését.

A főablak Napló lapja is mutat egy sort minden letöltésről („RSS:” a csatorna nevével és a címmel) és minden csatornahibáról. A letöltést, amelyet nem sikerült lekérni, a program a csatorna következő ellenőrzésekor újra megpróbálja.

## GTK

A GTK-ablakban ugyanez az RSS letöltő van, a **Beállítások > RSS letöltő...** parancsból, az eszköztár **RSS letöltő** gombjából vagy az oldalsáv **Csatornák** elemének jobb gombos menüjéből nyitva. A mezők, gombok és lapok ugyanazok, mint fent. Az oldalsáv **Csatornák** ága a csatorna elemeit mutatja a listában, és egy elem jobb gombos menüjének **Letöltés** parancsa (vagy a dupla kattintás) letölti. Az RSS-csatorna hozzáadása ablak a vágólapról is felkínál csatornacímet.

## macOS

A macOS-alkalmazásban ugyanez az RSS letöltő van, a **Beállítások > RSS letöltő...** parancsból, az eszköztár **RSS letöltő** gombjából vagy az oldalsáv **Csatornák** elemének jobb gombos menüjéből nyitva. A mezők, gombok és lapok ugyanazok, mint fent. A csatorna elemei akkor jelennek meg a listában, amikor kiválasztja a csatornát az oldalsávon; egy elem letöltéséhez kattintson rá duplán, nyomja meg a <kbd>Return</kbd> billentyűt, vagy válassza a jobb gombos menü **Letöltés** parancsát.

## Webes felület

A webes felületen is van RSS letöltő. Nyissa meg az eszköztár **RSS** gombjával (telefonon az eszköztár **Továbbiak** menüjében, az **RSS letöltő...** paranccsal), vagy kattintson egy csatornára a kategórialista **Csatornák** alatt, amely az adott csatorna elemein nyitja meg.

A weboldal RSS letöltőjének két helyett három lapja van:

- **Elemek**: a kijelölt csatorna (vagy az összes csatorna) elemei, a kijelöltekhez **Letöltés** paranccsal.
- **Szűrők** és **Előzmények**: mint fent.

Eltérések:

- A **Mentés ide** a kiszolgáló egy mappája, nem a böngészős számítógépé. A **...** gomb útvonalat kér; az üres az alapértelmezett kiszolgálói letöltési mappát jelenti.
- Hogy melyik webes felületet használja, az dönti el, kinek a csatornáit szerkeszti:
  - A **`rookparcel-nox`** saját csatornákat és szűrőket tart (az `rss.dat` fájlban, a `webui.json` mellett), és a kiszolgálón futtatja őket, nyitott ablak nélkül. Az RSS letöltőt csak akkor futtatja, amikor a webes felülete be van kapcsolva. Lásd [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).
  - A **Windows-ablak által üzemeltetett webes felület** az ablak saját csatornáit és szűrőit szerkeszti: egyetlen RSS letöltő van, mindkét helyen látszik. Lásd a [Webes felület](Webes-felület) oldalt.
- A webes felület a qBittorrent webes API-jának RSS-részére is válaszol, így egy qBittorrent távoli alkalmazás felsorolhatja a csatornákat és beállíthat szabályokat. A szabályai a fenti helyettesítő karakteres szintaxist használják; a reguláris kifejezés beállítást figyelmen kívül hagyja, és a csatornamappák nem támogatottak.

## Távoli ablakok

Egy másik Rookparcelhez kapcsolódó ablaknak (Fájl > Csatlakozás távoli géphez, lásd [Távvezérlés](Távvezérlés)) saját csatornái és szűrői vannak, ezen a számítógépen tárolva. Ezen a számítógépen ellenőrzi őket, és amit letöltenek, azt a távoli kiszolgálóhoz adja hozzá.

## Hol tárolódnak a csatornák

A csatornák, a szűrők és az előzmények az `rss.dat` fájlban vannak a beállításmappában, a `rookparcel.ini` mellett (távoli ablaknál az ottani saját `remote\<gép>_<port>` mappában). Lásd [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait).

Az egyszeri keresésekhez csatornák helyett lásd a [Torrentek keresése](Torrentek-keresése) oldalt. A Sonarrhoz és a Radarrhoz, amelyek maguk ellenőrzik a csatornákat, lásd a [Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr) oldalt.
