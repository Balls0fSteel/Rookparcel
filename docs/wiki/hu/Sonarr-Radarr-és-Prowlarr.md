# Sonarr, Radarr és Prowlarr

*Ez a [Sonarr, Radarr and Prowlarr](Sonarr-Radarr-and-Prowlarr) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

A Sonarr, a Radarr és a Prowlarr használhatja a Rookparcelt letöltő kliensként. A Rookparcel webes felülete ugyanazt az API-t beszéli, mint a qBittorrent, ezért ezekben az alkalmazásokban a Rookparcelt **qBittorrent** kliensként adja hozzá. Ez az oldal azt mutatja be, hogyan állítható ez be a `rookparcel-noxszal` vagy a Windows-program saját webes felületével, valamint a kategóriákat, a letöltési mappákat és a kész letöltések eltávolítását.

Ha a Prowlarrban vagy Jackettben a Rookparcelből szeretne keresni (a másik irány), lásd: [Torrentek keresése](Torrentek-keresése).

## Mi működik

Ezeket a valódi alkalmazásokkal ellenőrizték (Sonarr 4, Radarr és Prowlarr Dockerben), egyaránt a Linuxon futó `rookparcel-noxszal` és a Windows-program webes felületével:

- A letöltő kliens **Test** próbája mindegyik alkalmazásban.
- A Sonarr, a Radarr és a Prowlarr letöltései, mindegyik a saját kategóriájába.
- Az indexer seed-ideje, amely a torrenttel együtt érkezik, és időben leállítja azt.
- A Sonarr és a Radarr importálja a kész fájlt (keménylinkkel vagy távoli útvonal-leképezéssel), majd a **Remove Completed** eltávolítja a torrentet.
- A Prowlarr keresése és letöltése.

Nem ellenőrizték: a Lidarrt és a Readarrt (a Lidarr ugyanazt a klienskódot használja, mint a Radarr), valamint a magnet-hivatkozások letöltését.

## 1. Kapcsolja be a Rookparcel webes felületét

**A rookparcel-noxszal:**

```sh
rookparcel-nox run --save /srv/media/downloads --webui 0.0.0.0:8080
```

Jelentkezzen be egyszer egy böngészőben a nox által kiírt ideiglenes jelszóval, és állítsa be a sajátját a **Tulajdonságok > Webes felület** lapon. Részletek: [Webes felület](Webes-felület#bekapcsolás-a-rookparcel-noxban) és [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

A `0.0.0.0` lehetővé teszi, hogy más számítógépek és konténerek elérjék. Ha az alkalmazások ugyanazon a számítógépen futnak, nem konténerekben, elég a `--webui 8080` (csak ez a számítógép).

**A Windows-programmal:** **Beállítások > Tulajdonságok > Webes felület**, jelölje be a **Webes felület bekapcsolása** négyzetet, adjon meg **Felhasználónév:** és **Jelszó:** értéket, és jelölje be a **Kapcsolatok engedélyezése más gépekről** négyzetet, ha az alkalmazások másik számítógépen futnak. Ilyenkor a Windows tűzfalnak szabály kell a `rookparcel.exe` számára. Részletek: [Webes felület](Webes-felület#bekapcsolás-a-windows-programban). A Rookparcelnek futnia kell, hogy az alkalmazások elérjék.

## 2. Adja hozzá a Rookparcelt az alkalmazásban

A Sonarrban, a Radarrban vagy a Prowlarrban adjon hozzá egy új, **qBittorrent** típusú letöltő klienst, és töltse ki:

| Mező | Érték |
|---|---|
| Host | A Rookparcelt futtató számítógép IP-címe (vagy egy Ön által engedélyezett gazdanév, lásd [lent](#gazdanevek-és-docker)). |
| Port | A webes felület portja, például `8080`. |
| SSL | Ki, hacsak a webes felület nem HTTPS-t használ olyan tanúsítvánnyal, amelyben az alkalmazás megbízik. A fenti ellenőrzéseket sima HTTP-n végezték. |
| Username, Password | A webes felület bejelentkezése. |
| Category | Tetszőleges kategória, például `tv-sonarr`, `radarr` vagy `prowlarr`. |

Kattintson a **Test** gombra, majd mentsen. A Test létrehozza a kategóriát a Rookparcelben, ha még nem létezik.

Felhasználónév és jelszó helyett az alkalmazások API-kulcsot is küldhetnek. Kapcsolja be az **API-kulcs elfogadása (Authorization: Bearer)** beállítást a Rookparcel **Tulajdonságok > Webes felület** lapján, adjon meg ott legalább 16 karakteres kulcsot, és ugyanezt a kulcsot adja meg az alkalmazásnak. Egy harmadik mód a **Nincs hitelesítés ezen alhálózatok klienseinek (...)** az alkalmazás hálózatával, amely a hálózat minden kliensét bejelentkezés nélkül beengedi: csak olyan hálózatokat soroljon fel, amelyekben ugyanannyira megbízik, mint a jelszóban ([A webes felület biztonsága](Webes-felület#biztonság)).

## Gazdanevek és Docker

A Rookparcel csak IP-címekre és a `localhost` névre válaszol, hacsak nem enged meg több nevet. Az az alkalmazás, amely névvel éri el, például egy Docker-konténerből a `http://rookparcel:8080` címen vagy egy gép hálózati nevén, ezt kapja: „Bad Request: unexpected Host header”, és a Test kapcsolati hibát jelez.

Vagy használja az IP-címet az alkalmazásban, vagy adja hozzá a nevet a **Kiszolgálótartományok (több bejegyzést pontosvesszővel válasszon el):** mezőhöz:

- A nox webes felületén: **Tulajdonságok > Webes felület**, a **Kapcsolódás** alatt. Módosításához a **Jelenlegi jelszó:** szükséges.
- A Windows-programban: **Beállítások > Tulajdonságok > Webes felület**.

Több nevet `;` választ el; a `*` bármire illeszkedik. Inkább nevezze meg a gazdagépeket, mint a `*` használatát: az ellenőrzés a saját név alatt a webes felületet elérő weboldalak ellen véd.

Ha magát a noxot is konténerben szeretné futtatni az alkalmazások mellett, lásd: [Rookparcel Dockerben](Rookparcel-Dockerben): a képe, a megosztott médiamappás Compose-fájl és a felhasználóazonosítók.

## Kategóriák, címkék és jelölők

A qBittorrent kategóriája a Rookparcelben **címke**. Az alkalmazások által hozzáadott torrentek a kategóriájukat kapják címkéül, így a kategórialistában ez alatt a címke alatt látja őket ([Torrentek kezelése](Torrentek-kezelése)). Ha a Rookparcelben módosítja egy torrent címkéjét, az az alkalmazás számára is módosítja a kategóriáját; az alkalmazás csak a saját kategóriájában lévő torrenteket követi.

Ha az alkalmazás nem ad meg saját letöltési mappát, a torrent a kategóriája mentési mappájába kerül, ha a kategóriának van ilyen, egyébként a Rookparcel alapértelmezett letöltési mappájába.

A qBittorrent jelölője a Rookparcelben **jelölő** (*új az 1.2.63-ban*; korábban a jelölőket elfogadta, de eldobta). Egy torrentnek a címkéje mellett tetszőleges számú jelölője lehet; ezek a kategórialistában a **Jelölők** alatt és a **Jelölők** oszlopban jelennek meg ([Torrentek kezelése](Torrentek-kezelése#jelölők)). A Sonarr **Add Series Tags** beállításának bekapcsolt állapotában a letöltés megkapja a sorozat jelölőit. A Radarr nem küld jelölőket. A vesszőt vagy vezérlőkaraktert tartalmazó nevet hiba nélkül kihagyja, a nevek pedig kis- és nagybetűkre érzékenyek, mint a qBittorrentben.

## Letöltési mappák és útvonalak

Az alkalmazás úgy importálja a kész fájlokat, hogy beolvassa őket arról az útvonalról, amelyet a Rookparcel jelent. Ennek az útvonalnak mindkettőjük számára ugyanazt a fájlt kell jelentenie:

- **Ugyanaz a számítógép, konténerek nélkül:** nincs teendő.
- **Konténerek:** csatolja a letöltési mappát az alkalmazás konténerében ugyanarra az útvonalra, amelyet a Rookparcel használ, például mindkettőben `/srv/media`. Így a keménylinkek is működnek.
- **Különböző számítógépek** (például a Windowson futó Rookparcel egy megosztásra ment azon a kiszolgálón, ahol a Sonarr fut): adjon hozzá egy **Remote Path Mapping** leképezést a Sonarrban és a Radarrban a Rookparcel útvonaláról az alkalmazás útvonalára, például `\\server\media\downloads\` a `/srv/media/downloads/` útvonalra. A Windows-programmal ezt a beállítást ellenőrizték.

A noxnál a `--webui-save-root DIR` korlátozza azokat a mappákat, ahová a webes felület, és így az alkalmazások menthetnek ([A rookparcel-nox futtatása](A-rookparcel-nox-futtatása#webes-felület)). Vegye bele az alkalmazások által használt mappákat.

## A nem kívánt fájlok kihagyása

*Új az 1.2.63-ban.* A Rookparcel **Tulajdonságok > Felület** lapján a **Ne töltse le az ilyen nevű fájlokat:** az alkalmazások letöltéseire is vonatkozik. Bekapcsolva a mintákra illeszkedő nevű fájlok, például `*.lnk; Thumbs.db; *sample*`, nem letöltendőre lesznek állítva, amikor az alkalmazás hozzáadja a torrentet; magnet-hivatkozásnál azután, hogy a fájllistája megérkezett. Az a torrent, amelynek minden fájlja illeszkedik, egészében letöltődik. Lásd: [Torrentek hozzáadása](Torrentek-hozzáadása#név-szerint-kihagyott-fájlok).

Az alkalmazások, amelyek a qBittorrent „Excluded file names” beállítását az API-ján át állítják, ugyanezt a beállítást érik el: `excluded_file_names_enabled` és `excluded_file_names` (soronként egy minta).

## Megosztási korlátok és a Remove Completed

A Sonarr és a Radarr **Remove Completed** funkciója eltávolít egy torrentet, ha leállt és elérte a megosztási korlátját. A Rookparcel mindkettőt jelenti:

- **Az indexeren beállított seed-arány vagy seed-idő** (a Sonarrban, a Radarrban vagy a Prowlarrban) minden torrenttel megérkezik, és a torrent saját megosztási céljává válik. Az indexer által üresen hagyott értéket a Rookparcel beállításaiból veszi.
- **A Rookparcel saját megosztási célja** a **Tulajdonságok > Sorba állítás** lapon van, a **Megosztási cél** alatt: „Legkisebb arány (%): [0: nincs arány cél]” (alapértelmezés szerint 150 %) és „Legrövidebb megosztási idő (perc): [0: nincs idő cél]”. Amikor egy torrent eléri, a Rookparcel leállítja, és az alkalmazás eltávolíthatja. Lásd: [Várólista és seedelés](Várólista-és-seedelés).

Két tudnivaló:

- Ha a **Feltöltési sebesség korlátozása (kB/s): [0: leállítás]** be van jelölve, és a sebesség nagyobb 0-nál, a torrent a cél elérése után is tovább oszt, így az alkalmazás sosem látja késznek, és nem távolítja el.
- A Rookparcel célja akkor teljesül, ha minden megadott célt elértek (az arányt és az időt is), mint a µTorrentben; a qBittorrent az elsőnél megáll. Így egy torrent kicsit tovább oszthat, mint amit az alkalmazás vár, de soha nem távolítják el, mielőtt mindkettő teljesült volna.

A **Kényszerített indítás** paranccsal indított torrent sosem éri el a megosztási célját.

## Prowlarr

A Prowlarr csak hozzáadja a torrenteket; nem követi és nem importálja őket. Adja hozzá a Rookparcelt a Prowlarrban ugyanígy (qBittorrent kliens saját kategóriával, például `prowlarr`), hogy a Prowlarr saját keresésének letöltéseit a Rookparcelnek küldje.

## Ismert korlátok

- A Rookparcel a qBittorrent 2.11.2-es API-verziójaként jelentkezik.
- Figyelmen kívül marad, ha egy alkalmazás elküldi: a tartalom-elrendezési választás és a Sonarr inaktív megosztási időkorlátja. Az „első és utolsó darab előbb” az 1.2.63 óta érvényesül; korábban figyelmen kívül maradt.
- Az alkalmazás újraellenőrzési kérése a qBittorrent viselkedését tartja meg: a leállított torrent leállított marad, és induláskor ellenőrzik. A **Kényszerített ellenőrzés** a webes felületen és az ablakokban az 1.2.76 óta elindítja a leállított torrentet.
- A már a listában lévő torrent hozzáadására sikerrel válaszol; az alkalmazás ekkor a meglévőt követi.
- Az API megmutatja az IP-szűrő `ip_filter_enabled` és `ip_filter_path` értékét, de nem módosítja őket, és nincs beállítása a tiltólista webes címéhez. Ezeket a Rookparcel saját Tulajdonságok ablakában állítsa be.

## Ha valami nem sikerül

| Amit lát | Mit jelent |
|---|---|
| A Test sikertelen, és a Rookparcel címe egy név | A Host-ellenőrzés elutasította a nevet. Használja az IP-címet, vagy adja hozzá a nevet a **Kiszolgálótartományok** mezőhöz ([fent](#gazdanevek-és-docker)). |
| A Test IP-címmel sikertelen | A webes felület nincs bekapcsolva, csak a `127.0.0.1` címen figyel (a nox `--webui 8080` kapcsolója, vagy a **Kapcsolatok engedélyezése más gépekről** ki van kapcsolva), egy tűzfal blokkolja a portot, vagy hibás a bejelentkezés. A túl sok hibás bejelentkezés egy órára kizárja az alkalmazás címét. |
| Az importálás sikertelen: a fájl nem található | Az alkalmazás a letöltést másik útvonalon látja. Használja ugyanazt a csatolási útvonalat, vagy egy Remote Path Mapping leképezést ([fent](#letöltési-mappák-és-útvonalak)). |
| A kész torrentek az alkalmazás várólistájában maradnak | A Rookparcel tovább osztja őket: nincs beállítva megosztási cél, a cél még nincs elérve, vagy a **Feltöltési sebesség korlátozása (kB/s): [0: leállítás]** értéke 0-nál nagyobb. |

## Lásd még

- [Webes felület](Webes-felület): bekapcsolás, HTTPS és biztonság.
- [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása): a nox a számítógéppel együtt induló szolgáltatásként.
- [Rookparcel Dockerben](Rookparcel-Dockerben): a nox konténerben.
- Műszaki jegyzetek arról, mit hívnak az alkalmazások: [INTEGRATIONS.md, part 1](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/INTEGRATIONS.md#part-1-the-shim-as-an-arr-download-client).
