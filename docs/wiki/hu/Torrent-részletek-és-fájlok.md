*Ez a [Torrent details and files](Torrent-details-and-files) című angol oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Torrent-részletek és fájlok

Ez az oldal a torrentlista alatti részletlapokat ismerteti (Általános, Trackerek, Peerek, Részek, Fájlok, Sebesség és Napló), azt, hogyan választhatja ki, mely fájlokat és milyen sorrendben töltse le, valamint a Torrent jellemzői ablakot, amelyben a torrent trackerei, korlátai és megosztási célja található.

Jelöljön ki egy torrentet a listában, és a lapok kitöltődnek. A **Beállítások > Részletek megjelenítése** megjeleníti vagy elrejti őket; a fölöttük lévő határ húzásával a magasságukat módosíthatja. A Rookparcel megjegyzi a megnyitott lapot.

## Általános

![Az Általános lap](images/tab-general.png)

Egy torrent áttekintése.

- **Letöltve:** egy sáv, amelyen minden meglévő részhez egy csík tartozik, és a kész százalék.
- **Elérhetőség:** egy sáv, amely azt mutatja, hogy a csatlakozott peerek mely részekkel rendelkeznek, és hogy együtt hány teljes másolatot birtokolnak. 1 alatt a csatlakozott peerek között nincs senkinél minden rész.

**Átvitel**

| Mező | Jelentése |
|---|---|
| **Eltelt idő:** | mióta fut a torrent |
| **Hátralévő:** | a becsült hátralévő idő |
| **Elvetett:** | eldobott adat: az ellenőrzésen megbukott részek és a duplikátumok |
| **Letöltve:**, **Feltöltve:** | a torrent összesített adatai |
| **Seedek:**, **Peerek:** | „3 / 120 csatlakozva”: a csatlakozottak száma, és a tracker által jelentett összes |
| **Letöltési sebesség:**, **Feltöltési sebesség:** | az aktuális sebességek |
| **Letöltési korlát:**, **Feltöltési korlát:** | a torrent saját korlátai; ∞, ha nincs |
| **Megosztási arány:** | a feltöltött osztva a letöltöttel |
| **Állapot:** | ugyanaz a szöveg, mint az Állapot oszlopban |

**Általános**

| Mező | Jelentése |
|---|---|
| **Mentés mint:** | a mappa, amelyben a fájlok vannak |
| **Hátralévő:** | mennyi van még hátra a letöltésből |
| **Teljes méret:** | a teljes torrent, a kihagyott fájlokkal együtt |
| **Hozzáadva:**, **Befejezve:** | dátumok |
| **Részek:** | „1200 x 1,00 MB (megvan 800)”: a részek száma, a méretük, és hogy hány van meg |
| **Hash:** | a torrent info-hash-e |
| **A címed a trackerek szerint:** | az Ön nyilvános címe, így megállapíthatja, használatban van-e a VPN-je. *Új az 1.1.58-ban.* |

## Trackerek

![A Trackerek lap](images/tab-trackers.png)

A torrent trackerei, soronként egy.

| Oszlop | Jelentése |
|---|---|
| **Név** | a tracker címe |
| **Állapot** | **Működik**, **Működik:** a tracker üzenetével, **Frissítés...**, vagy **Hiba** / a tracker hibaüzenete; üres, ha a trackert még nem érte el |
| **Frissítés** | a következő bejelentésig hátralévő idő |
| **Seedek**, **Peerek**, **Letöltve** | a swarm számai, ahogy a tracker jelenti őket |

A trackerek azonnali elérésére használja a jobb gombos menü **Tracker frissítése** parancsát. Trackerek hozzáadásához, eltávolításához vagy átrendezéséhez használja a [Jellemzők](#torrent-jellemzői) ablakot.

Kattintson egy oszlopfejlécre, hogy eszerint rendezze a trackereket, ahogy a [fájllistánál](#a-fájllista-rendezése): a **Seedek**, a **Peerek** és a **Letöltve** a legtöbbtől indul, a többi oszlop a legkisebbtől; a második kattintás megfordítja a sorrendet, a harmadik visszatér a torrent saját sorrendjéhez (a szintjeihez). Az a tracker, amely egy számot még nem jelentett, mindkét esetben alul marad. A Rookparcel megjegyzi a rendezést. *Új az 1.2.69-ben.*

## Peerek

![A Peerek lap](images/tab-peers.png)

A peerek, amelyekhez csatlakozik. A webes seed a címét az **IP** oszlopban mutatja.

| Oszlop | Jelentése |
|---|---|
| **IP** | a peer címe |
| **Kliens** | a peer BitTorrent-programja |
| **Jelzők** | lásd lent |
| **%** | a torrent mekkora részével rendelkezik a peer |
| **Letöltés**, **Feltöltés** | a sebesség ettől a peertől és ennek a peernek |
| **Kér.** | a várakozó kérések, tőlünk és hozzánk |
| **Letöltve**, **Feltöltve** | ezzel a peerrel az összesített adatok |

Kattintson egy oszlopfejlécre, hogy eszerint rendezze a peereket: a **%**, a sebességek, a **Kér.** és az összesített adatok a legtöbbtől indulnak, az **IP**, a **Kliens** és a **Jelzők** a legkisebbtől (a címek szám szerint); a második kattintás megfordítja a sorrendet, a harmadik visszatér ahhoz a sorrendhez, amelyben a peerek érkeztek. A lista a peerek érkezése és távozása, valamint a számaik változása közben is megtartja a rendezést, és a kijelölt peerek kijelölve maradnak. A Rookparcel megjegyzi a rendezést. *Új az 1.2.69-ben.*

A **Jelzők** a µTorrent betűi:

| Jelző | Jelentése |
|---|---|
| D / d | Ön tölt le a peertől / szeretne, de a peer most nem küld |
| U / u | Ön tölt fel a peernek / a peer adatot kér, de Ön most nem küld |
| O | optimista unchoke: esélyt ad a peernek |
| S | snubbed: a peer abbahagyta a küldést |
| I | a peer csatlakozott Önhöz |
| E / e | a kapcsolat titkosított / csak a kézfogása az |
| P | a kapcsolat uTP-t használ |
| H | a peert a DHT-n keresztül találta meg |
| X | a peert a peercserén keresztül találta meg |
| L | a peert a helyi hálózaton találta meg |

## Részek

Egy térkép, részenként egy négyzettel. A kék négyzetek a meglévő részek. A narancssárgák letöltés alatt állnak. A többi a zöld árnyalatai: minél sötétebb, annál több csatlakozott peernél megvan az a rész; a fehér azt jelenti, hogy egynél sincs meg. A térkép feletti sor megszámolja a részeket. Ha több van, mint amennyi elfér, a térkép „... még” felirattal és a számmal ér véget.

A térkép alatti lista a most letöltés alatt álló részeket mutatja: a számukat (**#**), a **Méret**et, a **Blokkok száma** oszlopot, egy **Blokkok** sávot, és hogy hány blokk **Kész**. A sávban a kék blokkok a lemezen vannak, a zöldek írás alatt állnak, a narancssárgákat egy peertől kérték, a fehérek még szabadok.

## Fájlok

![A Fájlok lap](images/tab-files.png)

A torrent fájljai a mappáikban, ahogy az [Új torrent hozzáadása ablak](Torrentek-hozzáadása#a-torrent-tartalma) mutatja.

| Oszlop | Jelentése |
|---|---|
| **Név** | a fájl vagy a mappa neve; a mappák a mellettük lévő nyíllal nyílnak és záródnak |
| **Méret** | a mérete |
| **Kész** | mennyi töltődött le belőle |
| **%** | ugyanez százalékban |
| **Részek száma** | hány részt fed le |
| **Első rész** | az első részének száma |
| **Prioritás** | **Magas**, **Normál**, **Alacsony** vagy **Ne töltse le**; **Vegyes** az olyan mappánál, amelynek fájljai eltérnek |

A mappasor összeadja a fájljainak **Méret** és **Kész** értékét; részoszlopai nincsenek.

### A fájllista rendezése

*Új az 1.2.65-ben a Windows-ablakban; a GTK-ablakban és a macOS-alkalmazásban új az 1.2.67-ben.*

Kattintson egy oszlopfejlécre, hogy eszerint rendezze a fájlokat; a fejlécben egy nyíl mutatja az oszlopot és az irányt.

- Az első kattintás a neveket, méreteket és részeket a legkisebbtől rendezi, a **Kész**, **%** és **Prioritás** oszlopokat pedig a legmagasabbtól. A második kattintás megfordítja a sorrendet, a harmadik visszatér a torrent saját sorrendjéhez.
- A fában minden mappán belül a fájlok és mappák rendeződnek, a mappák elöl.
- A **Kész** vagy a **%** szerint rendezett lista a letöltés haladtával mozog, a **Prioritás** szerint rendezett pedig akkor, amikor a prioritásokat módosítja. A kijelölt fájlok kijelölve maradnak.
- A Rookparcel megjegyzi a rendezést. A **Fontosság a fájlok sorrendjében** a rendezéstől függetlenül továbbra is a torrentbeli sorrendet követi.

### Fanézet vagy egyetlen lista

*Új az 1.2.21-ben.*

A szűrőmező melletti **Fanézet** gomb, illetve a jobb gombos menü **Fanézet** parancsa a mappák és a µTorrent 2.2.1 egyetlen útvonallistája között vált (a torrent saját mappájával együtt). A Rookparcel megjegyzi a választást. A fában:

- Kattintson a nyílra, kattintson duplán egy mappára, vagy nyomja meg a <kbd>Right</kbd> / <kbd>Left</kbd> billentyűt a megnyitásához vagy bezárásához. A <kbd>Left</kbd> egy fájlon a mappájára ugrik.
- A mappák nyitva vagy zárva maradnak a lista frissülése közben és a prioritások módosítása után. Egy másik torrent kijelölése újra megnyit minden mappát.
- A mappán kiválasztott prioritás, a **Ne töltse le** vagy a **Fontosság a fájlok sorrendjében** a benne lévő minden fájlra vonatkozik (szűrő bekapcsolt állapotában csak a láthatókra). Egy mappán a **Megnyitás** és a **Mappa megnyitása** magát a mappát nyitja meg; az **Áthelyezés...** egyedi fájlokra való.

### Fájlok és sorrendjük kiválasztása

Jelöljön ki egy vagy több fájlt, és kattintson a jobb gombbal:

| Menüpont | Mit tesz |
|---|---|
| **Megnyitás** | megnyitja a fájlt az alapértelmezett programjában, mintha duplán kattintana rá a Fájlkezelőben; nem történik semmi, amíg a fájl még nincs a lemezen |
| **Mappa megnyitása** | megnyitja a mappáját a Fájlkezelőben, kijelölve a fájlt, ha az már létezik; addig a torrent legközelebbi létező mappáját |
| **Fontosság a fájlok sorrendjében** | a kijelölt fájloknak a torrentbeli sorrendjük szerint csökkenő prioritást ad, így egymás után fejeződnek be; a korábban kihagyott fájlokat újra letölti |
| **Prioritás > Magas prioritás / Normál prioritás / Alacsony prioritás** | beállítja a prioritást; a magasabb prioritású fájlok kapják előbb a részeiket |
| **Ne töltse le** | kihagyja a fájlokat |
| **Áthelyezés...** | új helyet választ egy fájlnak, és oda helyezi át; a lemezen nem lévő fájlnál megmondja a torrentnek, hogy ott keresse |

A fájlra kattintás duplán, vagy az <kbd>Enter</kbd> megnyomása a **Megnyitás** parancsként nyitja meg.

A **Ne töltse le** az a mód, ahogyan a torrent hozzáadása után fájlokat hagyhat ki. A prioritás újbóli kiválasztása újra letölti a kihagyott fájlt. Ha a letöltés megkezdése előtt szeretne fájlokat kihagyni, szüntesse meg a jelölésüket az [Új torrent hozzáadása ablakban](Torrentek-hozzáadása#a-torrent-tartalma). Ha minden hozzáadott torrentben bizonyos nevű fájlokat szeretne kihagyni, használja a **Ne töltse le az ilyen nevű fájlokat:** beállítást a Tulajdonságok > Felület oldalon (a [Torrentek hozzáadása](Torrentek-hozzáadása#név-szerint-kihagyott-fájlok) oldalon; *új az 1.2.63-ban*).

Ha az **Áthelyezés...** paranccsal olyan fájlhoz adott meg helyet, amely nem volt a lemezen, futtasson **Kényszerített ellenőrzést**, hogy a Rookparcel beszámítsa, amit ott talált. Az Áthelyezés soha nem cserél le olyan fájlt, amely már az új helyen van.

### A fájllista szűrése

*Új az 1.1.56-ban.*

Az ezres nagyságrendű fájlt tartalmazó torrentet könnyebb kezelni a Fájlok lap jobb felső részén lévő **Fájlok szűrése** mezővel.

1. Kattintson a mezőbe, vagy nyomja meg a <kbd>Ctrl</kbd>+<kbd>F</kbd> kombinációt, amíg a Fájlok lapé a fókusz.
2. Írjon be egy vagy több szót. A lista csak azokat a fájlokat mutatja, amelyek útvonala, a mappákat is beleértve, minden szót tartalmaz, a kis- és nagybetűk megkülönböztetése nélkül; a fában a mappáik nyitva jelennek meg. A mező melletti „12 / 3400 fájl” megmondja, hány egyezik.
3. Jelölje ki a kívánt fájlokat, és használja a jobb gombos menüt. A <kbd>Ctrl</kbd>+<kbd>A</kbd> csak a látható fájlokat jelöli ki.
4. Az <kbd>Esc</kbd> kiüríti a mezőt; az <kbd>Enter</kbd> a listára ugrik.

A szöveg megmarad, amikor másik torrentet jelöl ki, így ugyanazt a szűrőt több torrentre is alkalmazhatja. A Rookparcel bezárásakor nem mentődik el. A **Fontosság a fájlok sorrendjében** a torrent minden fájlját sorba rendezi, a szűrő bekapcsolt állapotában is.

Például írja be, hogy `sample`, és állítsa a találatokat **Ne töltse le** értékre, vagy írja be, hogy `s01 mkv`, hogy megtalálja az első évad videófájljait.

## Sebesség

Az összes torrent letöltési (zöld) és feltöltési (piros) sebességének grafikonja az utolsó tíz percből, felül az aktuális értékekkel.

## Napló

A Rookparcel naplója erre a munkamenetre, minden sor az idővel. Mutatja a hozzáadott és befejezett torrenteket, az áthelyezéseket, az ütemező és az Automatikus leállítás változásait, a figyelt mappák és az IP-szűrő problémáit, az RSS-letöltéseket, az importálásokat és egyebeket. Az utolsó 1000 sort őrzi meg.

A tracker, amely nem válaszol, a [Trackerek lapon](#trackerek) látszik, nem itt: nyilvános torrenteknél ez gyakori és ártalmatlan, mert a peerek a DHT-ból és a peercseréből is jönnek. A kivétel a privát torrent, amelynek csak a trackerei vannak: ha egyik sem válaszol, a Napló egyszer közli ezt, az utolsó hibával.

Ha valami nem a várt módon működik, először itt nézzen szét. Lásd még a [Hibaelhárítás és GYIK](Hibaelhárítás-és-GYIK) oldalt.

## Torrent jellemzői

![A Torrent jellemzői ablak](images/properties.png)

Nyissa meg a jobb gombos menü **Jellemzők** parancsával, a torrentlistában az <kbd>Enter</kbd> megnyomásával, vagy duplán kattintva egy nem kész torrentre. A címsor a torrent nevét mutatja, vagy „Jellemzők - 3 torrent” szöveget, ha többet jelölt ki.

**Trackerek (üres sor a szintek között)**

Soronként egy trackercím. Az üres sor új szintet kezd: a Rookparcel először az első szint trackereit próbálja, és csak akkor lép a következő szintre, ha mind elbukik. Minden sornak `http://`, `https://` vagy `udp://` előtaggal kell kezdődnie; az ismételt címet elhagyja.

**Sávszélesség-beállítások**

| Beállítás | Jelentése |
|---|---|
| **Legnagyobb feltöltési sebesség (kB/s): [0: korlátlan]** | a torrent feltöltési korlátja |
| **Legnagyobb letöltési sebesség (kB/s): [0: korlátlan]** | a torrent letöltési korlátja |
| **A feltöltési helyek alapértelmezett számának felülbírálása** és **Feltöltési helyek száma: [0: korlátlan]** | hány peernek tölt fel egyszerre ez a torrent, a Tulajdonságok > BitTorrent oldal alapértelmezése helyett |

**Megosztási cél**

Jelölje be az **Az alapértelmezett beállítások felülbírálása** négyzetet, hogy a torrent a Tulajdonságok > Sorba állítás oldalon megadott cél helyett saját célt kapjon:

| Beállítás | Jelentése |
|---|---|
| **Legkisebb arány (%): [0: nincs arány cél]** | a letöltöttnek ennyi százalékát tölti fel; a 150 másfélszeresét jelenti |
| **Legrövidebb megosztási idő (perc): [0: nincs idő cél]** | legalább ennyi ideig oszt |

Ha mindkettő meg van adva, mindkettőt el kell érni. Hogy ezután mi történjen, azt a Tulajdonságok > Sorba állítás oldalon állíthatja be; lásd a [Várólista és seedelés](Várólista-és-seedelés) oldalt. A kényszerítetten indított torrent soha nem éri el a célját.

**Egyéb beállítások**

| Beállítás | Jelentése |
|---|---|
| **Kezdeti megosztás** | mint a jobb gombos menüben; csak a megosztás alatt álló torrenteknél |
| **DHT bekapcsolása** | lehetővé teszi, hogy ez a torrent a DHT-n keresztül találjon peereket |
| **Peercsere** | lehetővé teszi, hogy ez a torrent más peerektől tudjon meg peereket |
| **Letöltés sorrendben** | a részeket sorrendben tölti le, ahogy a jobb gombos menü **Haladó > Letöltés sorrendben** parancsa; szürke, ha minden torrent kész |
| **Első és utolsó részek letöltése előbb** | előbb tölti le minden kért fájl mindkét végét, ahogy a **Haladó > Első és utolsó részek letöltése előbb** parancs |

*A két letöltésisorrend-négyzet új az 1.2.63-ban.* Hogy mikor hasznosak, azt lásd a [Torrentek kezelése](Torrentek-kezelése#letöltési-sorrend) oldalon.

A privát torrentek soha nem használják a DHT-t és a peercserét.

Az **OK** alkalmazza a módosításokat.

### Több torrent egyszerre

Jelöljön ki több torrentet, és nyissa meg a Jellemzőket, hogy együtt módosítsa őket. Az a mező, amelynek értéke eltér közöttük, üresen töltődik be, a jelölőnégyzet pedig egy harmadik, vegyes állapotot mutat. Az a mező, amelyet úgy hagy, ahogy betöltődött, minden torrenten érintetlen marad; csak az érvényesül mindegyiken, amit módosít.

## A GTK-s változatban (Linux, BSD)

- Ugyanaz a hét lap ugyanazokkal az oszlopokkal, ugyanaz a Fájlok lap menüje és szűrőmezője, és ugyanaz a Jellemzők ablak.
- A **Megnyitás** és a **Mappa megnyitása** az asztali környezet alapértelmezett programjait és fájlkezelőjét használja.

## A macOS-alkalmazásban

- Ugyanaz a hét lap és oszlop, ugyanaz a Fájlok lap menüje (a **Megnyitás** és a **Mappa megnyitása** a Finderben), a **Fanézet** és a **Fájlok szűrése** mező, valamint ugyanaz a Jellemzők ablak.

## A webes felületen

- Ugyanaz a hét lap. Az Általános lap a **Létrehozva:**, a **Létrehozta:**, a **Trackerek:** és a **Megjegyzés:** mezőt is mutatja.
- Kattintson egy lap táblázatának oszlopfejlécére, hogy eszerint rendezzen.
- A Fájlok lapon is van **Fájlok szűrése** mező. A jobb gombos menüjében a négy prioritás található: **Magas prioritás**, **Normál prioritás**, **Alacsony prioritás** és **Ne töltse le**. A fájlok megnyitása és az Áthelyezés böngészőből nem érhető el.
- A torrentmenü **Jellemzők** parancsa ugyanazt a Torrent jellemzői ablakot nyitja meg.

A **Fájl > Csatlakozás távoli géphez...** paranccsal kapcsolódott ablakban a Fájlok lap menüjében nincs **Megnyitás**, **Mappa megnyitása** és **Áthelyezés...**, mert a fájlok a másik számítógépen vannak. Lásd a [Távvezérlés](Távvezérlés) oldalt.

## Kapcsolódó oldalak

- [Torrentek kezelése](Torrentek-kezelése)
- [A főablak](A-főablak)
- [Várólista és seedelés](Várólista-és-seedelés)
- [Sebességkorlátok és ütemező](Sebességkorlátok-és-ütemező)
