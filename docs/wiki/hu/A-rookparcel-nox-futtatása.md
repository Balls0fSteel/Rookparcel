# A rookparcel-nox futtatása

*Ez a [Running rookparcel-nox](Running-rookparcel-nox) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

A `rookparcel-nox` a Rookparcel ablak nélkül: ugyanaz a motor, terminálból vagy szolgáltatásként futtatva, és a [webes felületén](Webes-felület) keresztül, egy asztali Rookparcelből a [Távvezérléssel](Távvezérlés), vagy a Sonarr, Radarr és Prowlarr ([Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr)) programokkal vezérelhető. Ez az oldal a parancsait és kapcsolóit, a Windows-szolgáltatásként, a systemd alatt és FreeBSD-n való futtatását, a fájljai helyét és a leállítását mutatja be.

## Honnan szerezhető be

A `rookparcel-nox` minden kiadással együtt érkezik ([A Rookparcel telepítése](A-Rookparcel-telepítése)):

| Platform | Fájl |
|---|---|
| Windows | `rookparcel-nox.exe`, a `rookparcel.exe` mellett a zipben és a telepített mappában |
| Linux | a `rookparcel-<version>-linux-<arch>.tar.gz` archívumban (a systemd-egységekkel), vagy önállóan `rookparcel-nox-<version>-<arch>.AppImage` néven |
| macOS | `rookparcel-nox-<version>-macos-<arch>.tar.gz` |

A `rookparcel-nox --help` felsorolja az összes parancsot és kapcsolót, a `rookparcel-nox --version` kiírja a verziót, a `rookparcel-nox --license` pedig a licencet és a harmadik felek közleményeit.

## Az első futtatás

1. Nyisson egy terminált (Windowson parancssort) a `rookparcel-nox` mappájában.
2. Indítsa el egy letöltési mappával és a webes felülettel:

   ```sh
   rookparcel-nox run --save ~/Downloads --webui 8080
   ```

3. Kiír néhány `*`-gal kezdődő sort. Az egyik a webes felület címe, és az első alkalommal egy ilyen sor:

   ```text
   * Web UI: no password set; temporary password for this run: ... (set one with --webui-pass)
   ```

4. Nyissa meg a `http://127.0.0.1:8080/` címet egy böngészőben, és jelentkezzen be `admin` néven ezzel az ideiglenes jelszóval.
5. Az oldalon nyissa meg a **Tulajdonságok > Webes felület** lapot, és állítsa be a saját jelszavát. Ettől kezdve tárolva van (hash formájában), és a nox minden indításkor kéri.

A leállításhoz írja be, hogy `q`, és nyomja meg az <kbd>Enter</kbd> billentyűt, vagy nyomja meg a <kbd>Ctrl</kbd>+<kbd>C</kbd> billentyűkombinációt. Kilépés közben elmenti minden torrent állapotát, így a következő `run` a fájlok újraellenőrzése nélkül folytatja őket.

A `--webui 8080` csak ezen a számítógépen figyel (`127.0.0.1`). Hogy más számítógépekről vagy telefonról elérje az oldalt, adjon meg címet is, például `--webui 0.0.0.0:8080`, és előbb olvassa el a [webes felület biztonságáról](Webes-felület#biztonság) szóló részt.

## Parancsok

| Parancs | Mit csinál |
|---|---|
| `rookparcel-nox run [OPTIONS]` | Folytatja az állapotmappában mentett összes torrentet, és fut, amíg le nem állítják. Ez a démon. |
| `rookparcel-nox add <file.torrent>... [--save DIR] [OPTIONS]` | Hozzáadja a torrenteket (fájlokat vagy `magnet:` hivatkozásokat), letölti őket, és kilép, ha elkészültek. A `--seed` kapcsolóval ehelyett tovább oszt. |
| `rookparcel-nox create <file or folder> [-o OUT] [CREATE OPTIONS]` | `.torrent` fájlt készít ([lent](#torrentfájl-készítése)). |
| `rookparcel-nox service ...` | A Windows-szolgáltatás (csak Windowson, [lent](#a-windows-szolgáltatás)). |
| `rookparcel-nox demo [--torrents N] [--active FRACTION] [--seconds N]` | Szimulált torrenteket tölt be, és kiírja az összesítést; csak tesztelésre. |

A `run` a kapcsolók után `.torrent` fájlokat és magnet-hivatkozásokat is fogad; ezeket az indításkor adja hozzá.

## A run és az add kapcsolói

### Mappák és állapot

| Kapcsoló | Jelentés |
|---|---|
| `--save DIR` | A letöltési mappa. Alapérték: az aktuális mappa. |
| `--state-dir DIR` | Itt tárolja a torrentlistát és a folytatási adatokat ([fájlok](#hol-tartja-a-fájljait)); ez a beállítások mappája is, hacsak nincs megadva a `--config-dir`. A `--state DIR` ugyanez. |
| `--config-dir DIR` | Itt tárolja a webes felület bejelentkezését és beállításait (`webui.json`). |
| `--profile NAME` | Külön noxot futtat saját mappákkal, bejelentkezéssel és figyelő porttal ([profilok](#profilok)). |
| `--no-state` | Semmit sem tölt be és nem ment. |
| `--move-on-complete DIR` | A letöltését befejező minden torrentet a DIR mappába helyezi át. |

### Megosztás és leállítás

| Kapcsoló | Jelentés |
|---|---|
| `--seed` | `add`: a letöltés után tovább oszt, ahelyett hogy kilépne. |
| `--super-seed` | Kezdeti megosztás a kész torrenteknél; maga után vonja a `--seed` kapcsolót. |
| `--seconds N` | N másodperc után leáll. |
| `--exit-when-done downloads` vagy `all` | Elmentett állapottal kilép, amint semmi sem tölt le többé (`downloads`), vagy semmi sem tölt le és nem oszt (`all`). Ugyanaz, mint az asztali program **Beállítások > Automatikus leállítás** menüje ([Automatikus leállítás](Automatikus-leállítás)). |

### Hálózat

| Kapcsoló | Jelentés |
|---|---|
| `--port N` | A peerek számára figyelő port. Alapérték: 6881. |
| `--listen LIST` | A `--port` helyett figyelő felületek: `IP:PORT`, `[IPv6]:PORT` vagy `DEVICE:PORT`, vesszővel elválasztva. |
| `--dl-limit KIB`, `--ul-limit KIB` | Letöltési és feltöltési korlát KiB/s-ban; a 0 korlátlan. |
| `--ip-filter FILE` | Letiltja a FILE (`ipfilter.dat`, P2P vagy CIDR formátum; a CIDR az 1.2.63-ban új) címtartományait. |
| `--proxy TYPE://[USER:PASS@]HOST:PORT` | Proxyn keresztül csatlakozik; a TYPE `socks4`, `socks5` vagy `http`. Amíg a peerek ezen mennek át, semmi bejövőt nem fogad el. |
| `--no-proxy-peers` | A peerekhez és a webes seedekhez közvetlenül csatlakozik; csak a trackerek használják a proxyt. |
| `--no-proxy-trackers` | A trackereket közvetlenül keresi meg. |
| `--no-proxy-hostnames` | A gazdaneveket itt oldja fel, nem a proxynál. |
| `--socks5-udp-local-ep` | Olyan SOCKS5-proxyhoz, amely és a gép között nincs NAT; alapértelmezés szerint ki van kapcsolva. |
| `--network RULE` | Csak ezt a hálózatot használja: a torrentforgalom az illeszkedő adapterhez van kötve. A RULE lehet `vpn` (bármely VPN-adapter), `adapter:ID` (eszköznév; Windowson az adapter GUID-ja), `range:CIDR` (például `10.2.0.0/16`) vagy `off`. |
| `--no-pause-when-missing` | Amíg a `--network` szabály semmire sem illeszkedik, a torrentek kötés nélkül futnak, ahelyett hogy a hálózatra várnának. |

*A `--network` az 1.1.58-ban új.* A proxy- és hálózati kapcsolók a webes felület beállításaiban tárolódnak, mint a **Tulajdonságok > Kapcsolat** oldal megfelelő négyzetei. Ha a webes felületen módosít valamit, a tárolt érték elsőbbséget élvez a parancssorral szemben, ezért attól kezdve az oldalon módosítsa őket. A parancssorban megadott proxyjelszót a számítógép más felhasználói láthatják a folyamatlistában; inkább a **Tulajdonságok > Kapcsolat** oldalon állítsa be. Bővebben ezekről a beállításokról: [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal).

### Webes felület

| Kapcsoló | Jelentés |
|---|---|
| `--webui [ADDR:]PORT` | Kiszolgálja a webes felületet és a qBittorrent-kompatibilis API-t. ADDR nélkül csak a `127.0.0.1` címen figyel. |
| `--webui-user NAME` | A bejelentkezési név. Alapérték: `admin`. |
| `--webui-pass PASS` | A jelszó, hash formájában tárolva. Ha nincs megadva (és nincs tárolt), az indításkor ideiglenes jelszót ír ki. |
| `--webui-save-root DIR` | Többször is megadható. A webes felület ekkor csak ezekbe a mappákba és alattuk menthet, helyezhet át és olvashat. Nélküle a bejelentkezéssel rendelkező bárki a nox fiókja által elérhető bármely mappát használhatja. |
| `--webui-https` | HTTPS-t szolgál ki, első használatkor készített önaláírt tanúsítvánnyal. |
| `--webui-remote-run-program` | Lehetővé teszi, hogy egy másik számítógépről érkező webes munkamenet módosítsa a [Program futtatása](A-beállítások-leírása#program-futtatása) beállításait, továbbra is csak a jelenlegi jelszóval. Nélküle ezek a sorok csak ugyanazon a számítógépen futó böngészőből módosíthatók. Olyan kiszolgálókhoz való, amelyeken nincs helyi böngésző, például egy konténerhez; soha nem tárolódik, ezért minden indításkor szerepelnie kell a parancssorban. A magán a kiszolgálón lévő fordított proxynak `X-Forwarded-For` fejlécet kell hozzáadnia, különben minden látogató ugyanazon a számítógépen lévő böngészőnek számít ([Program futtatása](A-beállítások-leírása#program-futtatása)). |
| `--webui-cert FILE [--webui-key FILE]` | HTTPS-t szolgál ki saját tanúsítvánnyal és kulccsal (PEM; a kulcs nem lehet titkosítva). |

A parancssorban megadott jelszót a többi helyi felhasználó láthatja a folyamatlistában. Inkább az oldal **Tulajdonságok > Webes felület** lapján állítsa be. A HTTPS-ről és a többi biztonsági beállításról részletesen: [Webes felület](Webes-felület#https).

### Egyéb

| Kapcsoló | Jelentés |
|---|---|
| `--import qbittorrent` vagy `utorrent` | Csak a `run`: indításkor hozzáadja a qBittorrent vagy a µTorrent torrentjeit, az adataikkal ott, ahol vannak (előbb ellenőrizve), a statisztikákkal, prioritásokkal, trackerekkel, címkékkel és a qBittorrent jelölőivel. A másik kliens nem futhat; a fájljait csak olvassa. Lásd: [Importálás uTorrentből és qBittorrentből](Importálás-uTorrentből-és-qBittorrentből). |
| `--import-dir DIR` | A másik kliens mappája, ha nem a szokásos helyen van. |
| `--verbose` (`-v`) | Kiírja a tracker- és peeresemények adatait. |
| `--no-update-check` | Csak a `run`: nem kérdezi meg a projekt weboldalát újabb kiadásokról. |
| `--daemon` | Unix: a háttérbe válik le ([más init rendszerek](#a-háttérben-futtatás-más-módjai)). |
| `--pid-file FILE` | A folyamatazonosítót a FILE fájlba írja, és zárolja; az ugyanezzel a FILE-lal indított második nox nem indul el. |
| `--log FILE` | Mindent, amit a nox kiír, a FILE végére fűz. |

### A frissítésellenőrzés

A `run` az indítás után 10 másodperccel, majd 12 óránként megkérdezi a projekt weboldalát, milyen kiadások léteznek. Nem küld semmit a telepítéséről, és nem tölt le semmit. Az újabb verzió egy sor a kimenetben és egy hivatkozás a webes felület állapotsorában. Kapcsolja ki a `--no-update-check` kapcsolóval, vagy a webes felületen a **Tulajdonságok > Általános > Adatvédelem** alatt, a „Frissítések automatikus keresése” négyzettel. Az ottani „Frissítés bétaverziókra” az előzetes kiadásokat is felkínálja. Lásd: [A Rookparcel frissítése](A-Rookparcel-frissítése).

## Torrentfájl készítése

```sh
rookparcel-nox create ~/Videos/holiday -t udp://tracker.example:1337/announce --magnet
```

| Kapcsoló | Jelentés |
|---|---|
| `-o`, `--output FILE` | Hová írja a torrentet. Alapérték: `<name>.torrent`. |
| `-t`, `--tracker URL` | Egy tracker; ismételje több szinthez, vagy tegyen többet egy szintre vesszővel. |
| `-w`, `--web-seed URL` | Egy webes seed; ismételhető. |
| `--node HOST:PORT` | Egy DHT-indítócsomópont; ismételhető. |
| `--private` | Beállítja a privát jelzőt (nincs DHT, peercsere vagy helyi felderítés). |
| `--piece-size KIB` | A darabméret KiB-ban. Alapérték: automatikus. |
| `--comment TEXT`, `--source TEXT`, `--creator TEXT` | A megjegyzés, a forrásmező (a privát trackerek használják) és a „készítette”. |
| `--no-date` | Kihagyja a létrehozás dátumát. |
| `--hybrid`, `--v1`, `--v2` | Hibrid (az alapérték), csak v1 vagy csak v2. |
| `--pad` | A fájlokat kitöltő fájlokkal igazítja a darabokhoz (hibridnél és v2-nél mindig be van kapcsolva). |
| `--mtime`, `--symlinks`, `--no-attributes`, `--skip-hidden` | Módosítási időket tárol; a hivatkozásokat hivatkozásként tárolja (Unix); kihagyja a rejtett és végrehajtható attribútumokat; kihagyja a ponttal kezdődő neveket. |
| `--magnet` | A magnet-hivatkozást is kiírja. |

Az asztali program erre szolgáló ablakát a [Torrentek készítése](Torrentek-készítése) írja le.

## Hol tartja a fájljait

| | Linux és BSD | Windows | macOS |
|---|---|---|---|
| Torrentlista és folytatási adatok (`resume.db`, `metadata.pack`, `dht_state.dat`) | `~/.local/share/rookparcel/nox` | `%APPDATA%\Rookparcel\nox` | `~/Library/Application Support/Rookparcel/nox` |
| Beállítások (`webui.json`, `ipfilter.dat`, a HTTPS-tanúsítvány) | `~/.config/rookparcel/nox` | ugyanaz a mappa | ugyanaz a mappa |
| A Windows-szolgáltatás | | `%ProgramData%\Rookparcel\nox` | |

- Linuxon a `$XDG_DATA_HOME` és a `$XDG_CONFIG_HOME` használatos, ha be vannak állítva.
- A `webui.json` fájlt a futó nox újraírja, valahányszor valami megváltozik a webes felületen. Csak leállított nox mellett szerkessze.
- Egy mappát egyszerre csak egy nox használhat. A második leáll ezzel: „... is in use by another rookparcel-nox”.
- **Hordozható mód:** ha a program mellett van `rookparcel.ini`, mindkettő a mellette lévő `nox` mappába kerül, és az abban lévő letöltési mappák hozzá viszonyítva tárolódnak, így az egész mappa áthelyezhető. *Új az 1.1.49-ben.* Lásd: [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait).

## Profilok

A `--profile NAME` külön noxot futtat saját torrentekkel, beállításokkal, webes felületi bejelentkezéssel és szolgáltatással. A név 1–32 karakter: a-z betűk, számjegyek és kötőjelek, betűvel kezdődően. Ha a `--port` vagy a `--listen` mást nem mond, minden profil saját figyelő portot kap, amelyet a nevéből számol ki, és minden indításkor ugyanaz.

| | alapprofil | `--profile work` |
|---|---|---|
| Linux és BSD | `~/.local/share/rookparcel/nox` | `~/.local/share/rookparcel/profiles/work/nox` (és ugyanez a `~/.config` alatt) |
| Windows | `%APPDATA%\Rookparcel\nox` | `%APPDATA%\Rookparcel\profiles\work\nox` |
| macOS | `~/Library/Application Support/Rookparcel/nox` | `.../Rookparcel/profiles/work/nox` |
| Windows-szolgáltatás | `RookparcelNox` | `RookparcelNox_work` |
| systemd | `rookparcel-nox.service` | `rookparcel-nox@work.service` |

Minden profil webes felületének saját port kell, például `--webui 127.0.0.1:8081`. A már használatban lévő port leállítja a noxot ezzel: „port N is in use, perhaps by another Rookparcel profile”. *A profilok az 1.1.44-ben újak.* Lásd: [Profilok](Profilok).

## A Windows-szolgáltatás

A `rookparcel-nox service install` Windows-szolgáltatásként regisztrálja a noxot **RookparcelNox** („Rookparcel nox”) néven. A Windows-szal együtt indul, és akkor is fut, ha senki sincs bejelentkezve.

Ez nem ugyanaz, mint a `rookparcel.exe` **Tulajdonságok > Webes felület** lapja: az az ablakból szolgálja ki a webes felületet, az ablak saját torrentjei fölött, csak amíg az ablak fut ([Webes felület](Webes-felület)).

Az `install`, az `uninstall`, a `start` és a `stop` rendszergazdai parancssort igényel.

1. Tegye a `rookparcel-nox.exe` fájlt olyan helyre, ahol a szolgáltatás fiókja olvashatja, például `C:\Program Files\Rookparcel\`. A felhasználói mappája alatti másolatot (Letöltések, Dokumentumok) a szolgáltatás fiókja nem tudja megnyitni; az `install` figyelmeztet erre.
2. Telepítse azokkal a `run` kapcsolókkal, amelyekkel indulnia kell:

   ```bat
   "C:\Program Files\Rookparcel\rookparcel-nox.exe" service install --webui 127.0.0.1:8080
   ```

3. Indítsa el a `rookparcel-nox service start` paranccsal, vagy adja az `install` parancshoz a `--now` kapcsolót. Ettől kezdve a Windows-szal együtt indul.
4. Olvassa ki az ideiglenes webes felületi jelszót a `%ProgramData%\Rookparcel\nox\nox.log` fájlból (rendszergazdai parancssorból), nyissa meg az oldalt, és állítsa be a saját jelszavát. Vagy adja meg az `install` parancsnak a `--webui-user` és a `--webui-pass` kapcsolót: a jelszó hash formájában a szolgáltatás `webui.json` fájljában tárolódik, soha nem a szolgáltatás parancssorában.

Maga az `install` kapcsolói:

| Kapcsoló | Jelentés |
|---|---|
| `--account A` | `LocalService` (az alapérték), `NetworkService` vagy `LocalSystem`. |
| `--start TYPE` | `delayed` (az alapérték: automatikus, az indulás elcsendesedése után), `auto` vagy `manual`. |
| `--replace` | Előbb leállítja és eltávolítja a telepített RookparcelNox szolgáltatást; az állapota megmarad. |
| `--now` | A telepítés után elindítja a szolgáltatást. |
| `--profile NAME` | A profil saját szolgáltatását telepíti, `RookparcelNox_NAME` néven. |

Mit állít be:

- **Állapot és beállítások** a `%ProgramData%\Rookparcel\nox` mappában (vagy a `--state-dir` helyen), olyan mappában, amelyet csak a SYSTEM, a rendszergazdák és a szolgáltatás fiókja nyithat meg, mivel ez tartalmazza a webes felület bejelentkezését.
- **Letöltések** a `Downloads` almappájában, hacsak nem ad meg `--save` kapcsolót.
- **Napló** ott a `nox.log` fájlban, hacsak nem ad meg `--log` kapcsolót.
- **Újraindítások** hiba után: 5, 30 és 60 másodperc múlva.
- A rendszer leállításakor időt kap a torrentek állapotának mentésére.

Azok a mappák, amelyekbe a szolgáltatás ír, és az `install` nem készítette őket (egy `--save` mappa, egy `--move-on-complete` mappa), jogosultságot igényelnek a fióknak, rendszergazdai parancssorból:

```bat
icacls D:\Torrents /grant "NT AUTHORITY\LocalService:(OI)(CI)M"
```

A bejövő peerkapcsolatokhoz Windows tűzfalszabály kell a programnak; a szolgáltatás nem tud kérni ilyet:

```bat
netsh advfirewall firewall add rule name="Rookparcel nox" dir=in action=allow program="C:\Program Files\Rookparcel\rookparcel-nox.exe"
```

A többi szolgáltatásparancs:

| Parancs | Mit csinál |
|---|---|
| `rookparcel-nox service start`, `service stop` | Elindítja vagy leállítja, és megvárja, amíg megtörtént. Az `sc start RookparcelNox`, az `sc stop RookparcelNox` és a Szolgáltatások konzol ugyanezt teszi. |
| `rookparcel-nox service status` | Kiírja a szolgáltatás állapotát és parancssorát (nem kell rendszergazda). Az `--all` felsorolja minden profil szolgáltatását. |
| `rookparcel-nox service uninstall` | Leállítja és eltávolítja. Az állapotmappa és a napló megmarad. Az `--all` eltávolítja minden olyan profil szolgáltatását, amely ezt a programot futtatja. |
| `rookparcel-nox service debug [OPTIONS]` | A szolgáltatás munkáját ebben a konzolban futtatja, hogy telepítés előtt kipróbálhassa a kapcsolókat. |
| `sc control RookparcelNox paramchange` | Újratölti az IP-szűrőt, és újranyitja a naplót. |

A kapcsolók módosításához telepítse újra a `--replace` kapcsolóval. A torrentek állapotának és a letöltéseknek a törléséhez is, az eltávolítás után törölje a `%ProgramData%\Rookparcel` mappát.

A `start`, a `stop`, a `status` és az `uninstall` a `--profile NAME` kapcsolót fogadja egy profil szolgáltatásához. *A profilonkénti szolgáltatások az 1.1.47-ben újak.*

**A szolgáltatás nélkül:** a `rookparcel-nox run` parancssorban is működik, és a <kbd>Ctrl</kbd>+<kbd>C</kbd> hatására vagy az ablak bezárásakor tisztán leáll. A Feladatütemező egy másik mód: egy „Indításkor” feladat, amely ezt futtatja: `rookparcel-nox.exe run --save D:\Torrents --webui 8080 --log C:\Rookparcel\nox.log`. A `--daemon` Windowson nem működik.

## systemd (Linux)

A Linux-archívumban kész egységek vannak, a tároló [`contrib/systemd`](https://github.com/Balls0fSteel/Rookparcel/tree/main/contrib/systemd) mappájában is.

**Rendszerszolgáltatásként**, `rookparcel` fiókként futtatva, az állapotával a `/var/lib/rookparcel-nox` mappában:

```sh
sudo install -m 755 rookparcel-nox /usr/local/bin/
sudo install -D -m 644 rookparcel-nox.sysusers /etc/sysusers.d/rookparcel-nox.conf
sudo systemd-sysusers
sudo install -m 644 rookparcel-nox.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now rookparcel-nox
journalctl -u rookparcel-nox
```

A journal az első indításkor megmutatja az ideiglenes webes felületi jelszót. Az egység a webes felületet a `127.0.0.1:8080` címen szolgálja ki, és a `/var/lib/rookparcel-nox/downloads` mappába tölt le. Ennek módosításához, és hogy másik mappába is írhasson:

```ini
# sudo systemctl edit rookparcel-nox
[Service]
ExecStart=
ExecStart=/usr/local/bin/rookparcel-nox run --save /srv/torrents --webui 0.0.0.0:8080
ReadWritePaths=/srv/torrents
```

A szolgáltatás le van zárva (a `ReadWritePaths=` nélkül nem tud a saját mappáján kívülre írni), ezért máshol lévő letöltési mappához mindig kell ez a sor.

**Saját felhasználói szolgáltatásként**, a fájljaival a [fent](#hol-tartja-a-fájljait) felsorolt mappákban:

```sh
install -m 755 rookparcel-nox ~/.local/bin/
install -D -m 644 rookparcel-nox.service ~/.config/systemd/user/rookparcel-nox.service
systemctl --user daemon-reload
systemctl --user enable --now rookparcel-nox
loginctl enable-linger "$USER"
```

A `loginctl enable-linger` a kijelentkezés után is futva tartja. Ez az egység a `~/Downloads` mappába tölt le.

A **profilok** a `rookparcel-nox@.service` sablonegységeket használják: a `systemctl enable --now rookparcel-nox@work` a `work` profilt indítja. Alapértelmezés szerint nincs webes felületük, mert kettő nem osztozhat egy porton; adjon mindegyiknek sajátot a `systemctl edit rookparcel-nox@work` parancsban.

A `systemctl reload rookparcel-nox` újratölti az IP-szűrőt. A kimenet a journalba kerül.

## FreeBSD

Az rc.d szkript a tároló [`contrib/freebsd`](https://github.com/Balls0fSteel/Rookparcel/tree/main/contrib/freebsd) mappájában van:

```sh
pw useradd rookparcel -d /var/db/rookparcel-nox -s /usr/sbin/nologin -c "Rookparcel daemon"
install -m 755 rookparcel-nox /usr/local/bin/
install -m 555 rookparcel_nox /usr/local/etc/rc.d/
sysrc rookparcel_nox_enable=YES
sysrc rookparcel_nox_args="--webui 0.0.0.0:8080"
service rookparcel_nox start
```

`rookparcel` felhasználóként fut, az állapotával a `/var/db/rookparcel-nox` mappában és a naplóval a `/var/log/rookparcel-nox.log` fájlban, ahová az első indítás az ideiglenes webes felületi jelszót írja. Beállítások az `/etc/rc.conf` fájlban: `rookparcel_nox_args` (további `run` kapcsolók; alapérték `--webui 127.0.0.1:8080`), `rookparcel_nox_user`, `rookparcel_nox_dir`, `rookparcel_nox_save`, `rookparcel_nox_log`. A `rookparcel_nox_args` kapcsolót használja, ne a `rookparcel_nox_flags` kapcsolót. A `service rookparcel_nox reload` újratölti az IP-szűrőt, és újranyitja a naplót.

Profilok: `sysrc rookparcel_nox_profiles="work"` és `sysrc rookparcel_nox_work_args="--webui 127.0.0.1:8081"`; a `service rookparcel_nox start work` egyet indít, a `service rookparcel_nox start` mindet.

## A háttérben futtatás más módjai

Más Unix-rendszereken:

- A `--daemon` a háttérbe válik le, és visszatér, amint a nox elindult (0-s állapottal) vagy nem tudott elindulni (1-es állapottal). A `--log` nélkül az ezután keletkező kimenet elvész, ezért előbb olvassa el az ideiglenes jelszót, vagy használja a `--log` kapcsolót.
- A `--pid-file FILE` a folyamatazonosítót írja; az ugyanazzal a fájllal indított második nox nem indul el.
- A `--log FILE` minden kimenetet a FILE végére fűz, amelyet csak a tulajdonosa olvashat, mivel az ideiglenes jelszót tartalmazhatja. A `SIGHUP` újranyitja, így a naplóforgatás működik.

## Leállítás és újratöltés

| Mi | Hatás |
|---|---|
| `q` és <kbd>Enter</kbd>, <kbd>Ctrl</kbd>+<kbd>C</kbd>, `SIGTERM` | Tiszta leállás: a torrentek állapota elmentve, az útválasztó porthozzárendelései eltávolítva. |
| Második <kbd>Ctrl</kbd>+<kbd>C</kbd> vagy `SIGTERM` e leállás közben | Azonnal kilép, mentés nélkül. |
| `SIGHUP` (Windows-szolgáltatás: `sc control RookparcelNox paramchange`) | Újratölti az IP-szűrő listáját, és újranyitja a `--log` fájlt. A `webui.json` fájlt nem olvassa újra. Ha a lista egy címről származik, az utoljára letöltött másolatot tölti újra; semmit sem tölt le. |
| Windows: a konzolablak bezárása, kijelentkezés, leállítás | Tiszta leállás. |

## Lásd még

- [Webes felület](Webes-felület): a nox által kiszolgált oldal.
- [Távvezérlés](Távvezérlés): a nox vezérlése az asztali program ablakából.
- [Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr): a nox mint a letöltő kliensük.
- A teljes műszaki leírás: [DAEMON.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/DAEMON.md).
