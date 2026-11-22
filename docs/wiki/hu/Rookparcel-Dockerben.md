# Rookparcel Dockerben

*Ez a [Docker](Docker) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

A `rookparcel-nox`, az ablak nélküli démon, konténerképként is elérhető a Dockerhez és más konténermotorokhoz. Ez az oldal azt mutatja be, hogyan indítható a `docker run` paranccsal vagy a Docker Compose segítségével, milyen mappái és portjai vannak, hogyan működik a webes felület jelszava, hogyan futtatható másik felhasználóként, hogyan használható a konténerekben futó Sonarr, Radarr és Prowlarr mellett, és hogyan állítható le és frissíthető.

*Új az 1.2.21-ben.*

A kép neve `ghcr.io/balls0fsteel/rookparcel-nox`, x86-64 (`amd64`) és 64 bites ARM (`arm64`) Linuxra. Minden kiadásnak saját címkéje van, például `:1.2.21`, a `:latest` pedig a legújabb kiadás (nem előzetes kiadás). Belül a nox ugyanúgy fut, mint bármely Linux-számítógépen: minden érvényes, ami [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása) és a [Webes felület](Webes-felület) oldalon áll.

## Az első indítás

```sh
docker run -d --name rookparcel --restart unless-stopped --stop-timeout 60 \
  -p 8080:8080 -p 6881:6881 -p 6881:6881/udp \
  -v rookparcel-config:/config -v rookparcel-downloads:/downloads \
  ghcr.io/balls0fsteel/rookparcel-nox
```

1. Olvassa ki az ideiglenes jelszót a naplóból:

   ```sh
   docker logs rookparcel
   ```

   ```text
   * Web UI: no password set; temporary password for this run: ... (set one with --webui-pass)
   ```

2. Nyissa meg a `http://127.0.0.1:8080/` címet (vagy másik számítógépről a `http://<a számítógép címe>:8080/` címet), és jelentkezzen be vele `admin` néven.
3. Az oldalon nyissa meg a **Tulajdonságok > Webes felület** lapot, és állítsa be a saját **Jelszó:** értékét, az ideiglenessel mint **Jelenlegi jelszó:**.

Az ideiglenes jelszót csak az első indításkor írja ki, és addig érvényes, amíg be nem állítja a sajátját. Ha a napló eltűnik, mielőtt elolvasta volna (a konténert eltávolították), állítson be jelszót a `ROOKPARCEL_WEBUI_PASS` változóval ([lent](#a-webes-felület-jelszava)).

Parancs nélkül a kép ezt futtatja:

```sh
rookparcel-nox run --state-dir /config --save /downloads --webui 0.0.0.0:8080 --port 6881
```

A kép neve után megadott kapcsolók hozzáadódnak ehhez a parancshoz, és a másodszor megadott kapcsoló győz az alapértékkel szemben: `... ghcr.io/balls0fsteel/rookparcel-nox --save /srv/media/downloads --verbose`. A kép neve után megadott parancs ehelyett lecseréli: `... ghcr.io/balls0fsteel/rookparcel-nox run --state-dir /config --save /srv/media/downloads --webui 0.0.0.0:8080 --port 6881`. Minden kapcsoló fel van sorolva [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása#a-run-és-az-add-kapcsolói) oldalon.

## Mappák

| Mappa | Mit tartalmaz |
|---|---|
| `/config` | Minden, amit a nox tárol: a torrentlista és az egyes torrentek haladása, a DHT állapota, a webes felület bejelentkezése és beállításai (`webui.json`), a címkék, a jelölők és az RSS letöltő. Tartsa kötetben, különben az újraindítás üres listával kezdődik. |
| `/downloads` | Az alapértelmezett letöltési mappa. |

Mindkettő kötet. A Sonarr, a Radarr és a Prowlarr számára inkább a médiamappáját csatolja ugyanarra az útvonalra, amelyet az alkalmazások használnak, és abba mentsen ([lent](#sonarr-radarr-és-prowlarr-konténerekben)).

A mentéshez állítsa le a konténert, és másolja le a `/config` kötetet. Az újrakezdéshez távolítsa el a konténert és a kötetet.

## Portok

| Port | Mire való |
|---|---|
| `8080/tcp` | A webes felület és a qBittorrent-kompatibilis API. |
| `6881/tcp`, `6881/udp` | A peerek számára figyelő port (az UDP az uTP-t és a DHT-t szállítja). |

A figyelő portot kívül és belül ugyanarra a számra képezze le (`-p 6881:6881`): a nox azt a portot közli a trackerekkel és a peerekkel, amelyen figyel, így a `-p 6882:6881` rosszat adna meg nekik. Másik port használatához mindkettőt módosítsa, például `-p 51413:51413 -p 51413:51413/udp` és `--port 51413`. A webes felület **Tulajdonságok > Kapcsolat** lapján később módosított portnál ugyanezt a módosítást el kell végezni a leképezésben is.

**Az UPnP és a NAT-PMP nem működik a Docker szokásos (bridge) hálózatából:** az útválasztó belülről nem érhető el, ezért a portot nem irányítják át Önnek. Irányítsa át kézzel az útválasztón, TCP-n és UDP-n, a Dockert futtató számítógépre ([A port átirányítása az útválasztón](Kapcsolat-portok-és-tűzfal#a-port-továbbítása-a-routerben)), vagy Linuxon futtassa a konténert a gép hálózatán (`--network host`), ahol a nox úgy látja az útválasztót és a helyi hálózatot, mint egy konténeren kívüli program, és a `-p` nem kell. A Helyi peerfelderítés (Local Peer Discovery) is csak gazdahálózattal talál peereket a hálózatán.

## A webes felület jelszava

| Változó | Jelentés |
|---|---|
| `ROOKPARCEL_WEBUI_PASS` | Minden indításkor beállítja a webes felület jelszavát. Hash formájában tárolódik a `/config` mappában, ezért a változó eltávolítása után is megmarad. |
| `ROOKPARCEL_WEBUI_USER` | A hozzá tartozó bejelentkezési név (alapérték: `admin`). Csak a `ROOKPARCEL_WEBUI_PASS` változóval együtt használatos. |

```sh
docker run -d --name rookparcel ... -e ROOKPARCEL_WEBUI_PASS='a long password' ghcr.io/balls0fsteel/rookparcel-nox
```

A `ROOKPARCEL_WEBUI_PASS` nélkül a nox megtartja a tárolt jelszót, vagy ideiglenest ír ki, ha még nincs tárolt.

Két tudnivaló:

- Amíg a változó be van állítva, minden indításkor ez győz: az oldalon módosított jelszó a következő indításkor visszaáll a változóéra. Távolítsa el a változót, hogy az oldalon beállított jelszó megmaradjon.
- A `docker inspect` megmutatja a változót, és a nox a jelszót a parancssorában kapja, amelyet a `docker top` és a gazdagép folyamatlistája mutat. Olyan számítógépen, amelyre mások is bejelentkezhetnek, inkább az ideiglenes jelszót használja, és az oldalon állítsa be a sajátját, vagy az első indítás után távolítsa el a változót.

A webes felület sima HTTP. Olyan hálózaton, amelyben nem bízik, szolgálja ki HTTPS-en (`--webui-https`), vagy tegye egy ezt megtévő fordított proxy mögé: lásd [HTTPS](Webes-felület#https) és [A webes felület biztonsága](Webes-felület#biztonság).

## Felhasználó és jogosultságok

A nox a `rookparcel` felhasználóként fut, felhasználó- és csoportazonosítója 1000, soha nem rootként. Az új fájlokat mindenki olvashatja, és a csoportjuk írhatja (umask `0002`), így a többi konténerben lévő alkalmazások importálhatják és eltávolíthatják őket.

| Cél | Teendő |
|---|---|
| Másik felhasználó tulajdonában lévő gazdamappába írás | Indítsa a konténert ezzel a felhasználóval: `--user 1001:1001` (`user: "1001:1001"` a Compose-ban). Az azonosítókat az `id -u` és az `id -g` paranccsal találja meg. |
| Másik umask használata | `-e UMASK=0022` (a fájlokat mindenki olvashatja, csak a tulajdonosuk írhatja). |

Másik felhasználóval a `/config` mappának is írhatónak kell lennie számára. Az új névvel ellátott kötet az 1000-es felhasználóé, ezért használjon helyette egy gazdamappát, amelyet annak a felhasználónak ad:

```sh
sudo mkdir -p /srv/rookparcel && sudo chown 1001:1001 /srv/rookparcel
docker run -d --name rookparcel --user 1001:1001 -v /srv/rookparcel:/config ...
```

## Docker Compose

A tárolóban van egy kész [compose.yaml](https://github.com/Balls0fSteel/Rookparcel/blob/main/contrib/docker/compose.yaml). A `/config` mappát kötetben tartja, a `/srv/media` mappát ugyanazon az útvonalon csatolja, a `/srv/media/downloads` mappába ment, és 60 másodpercet ad a noxnak a leállásra:

```yaml
services:
  rookparcel:
    image: ghcr.io/balls0fsteel/rookparcel-nox:latest
    container_name: rookparcel
    restart: unless-stopped
    user: "1000:1000"
    environment:
      ROOKPARCEL_WEBUI_PASS: ${ROOKPARCEL_WEBUI_PASS:-}
    command: [run, --state-dir, /config, --save, /srv/media/downloads, --webui, "0.0.0.0:8080", --port, "6881"]
    volumes:
      - rookparcel-config:/config
      - /srv/media:/srv/media
    ports:
      - "8080:8080"
      - "6881:6881/tcp"
      - "6881:6881/udp"
    stop_grace_period: 60s

volumes:
  rookparcel-config:
```

1. Tegye a jelszót egy `.env` fájlba a `compose.yaml` mellett: `ROOKPARCEL_WEBUI_PASS=a long password`. Nélküle az ideiglenest a `docker compose logs rookparcel` paranccsal olvashatja ki.
2. Módosítsa a `/srv/media` és a `user:` értékét a médiamappájára és annak tulajdonosára.
3. Indítsa el a `docker compose up -d` paranccsal.

## Sonarr, Radarr és Prowlarr konténerekben

Állítsa be a Rookparcelt az alkalmazásokban **qBittorrent** kliensként, ahogy a [Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr) oldal leírja. Konténerekben három dolog különbözik:

1. **Ugyanaz az útvonal minden konténerben.** Csatolja a médiamappát ugyanarra az útvonalra a nox konténerében és az alkalmazásokéban (mindegyikben `/srv/media`, mint a fenti Compose-fájlban), és a letöltéseket abba mentse. Az alkalmazások ekkor ott találják a fájlokat, ahol a nox jelzi őket, és az importálások használhatnak keménylinkeket.
2. **A gazdanév.** Ugyanazon a Compose- vagy felhasználó által megadott hálózaton az alkalmazások `http://rookparcel:8080` címen érik el a noxot (a szolgáltatás vagy a konténer neve). A nox csak IP-címekre és a `localhost` névre válaszol, amíg a nevet nem engedélyezi: adja hozzá a `rookparcel` nevet a webes felület **Tulajdonságok > Webes felület** lapján, a **Kapcsolódás** alatt a **Kiszolgálótartományok (több bejegyzést pontosvesszővel válasszon el):** mezőhöz. Addig az alkalmazások **Test** próbája ezzel hibázik: „Bad Request: unexpected Host header”. Lásd: [Gazdanevek és Docker](Sonarr-Radarr-és-Prowlarr#gazdanevek-és-docker).
3. **A letöltési mappa, amelyet az alkalmazások kérnek,** a nox konténerében lévő útvonal kell legyen, például `/srv/media/downloads`.

A `--webui-save-root /srv/media` (a `run` parancshoz adva) korlátozza azokat a mappákat, ahová a webes felület és az alkalmazások menthetnek.

## Naplók, leállítás és újratöltés

A nox a naplóját a konténer kimenetére írja: `docker logs rookparcel` (`-f` a követéshez). A `--verbose` hozzáadja a tracker- és peeresemények adatait.

A nox leálláskor elmenti minden torrent állapotát. A `docker stop` `SIGTERM` jelet küld, és alapértelmezés szerint 10 másodpercet vár, mielőtt kilövi a konténert, ami egy nagy listához kevés lehet, ezért adjon neki egy percet:

```sh
docker stop -t 60 rookparcel
```

A `docker run` parancs `--stop-timeout 60` kapcsolója, vagy a Compose `stop_grace_period: 60s` beállítása ezt teszi a konténer alapértelmezésévé. A tiszta leállás ezekkel a sorokkal végződik a naplóban, és a konténer 0-s kóddal lép ki:

```text
* SIGTERM received (again to quit at once, without saving)
* shutting down
* stopped
```

A `docker kill -s HUP rookparcel` újratölti az IP-szűrőt, ahogy a `SIGHUP` teszi konténeren kívül ([Leállítás és újratöltés](A-rookparcel-nox-futtatása#leállítás-és-újratöltés)).

## Frissítés

A kép nem keres frissítéseket, és a webes felülete nem mutat frissítési hivatkozást: a konténert úgy frissíti, hogy lehúzza az új képet, és új konténert indít ugyanazon a `/config` mappán.

```sh
docker pull ghcr.io/balls0fsteel/rookparcel-nox
docker stop -t 60 rookparcel && docker rm rookparcel
docker run -d --name rookparcel ...    # the same command as before
```

Compose-zal: `docker compose pull`, majd `docker compose up -d`.

Ugyanezért a [keresősegédet](Torrentek-keresése#a-keresősegéd) a kép nem kínálja fel. A saját Prowlarr vagy Jackett továbbra is hozzáadható keresési forrásként ([Saját Prowlarr vagy Jackett hozzáadása](Torrentek-keresése#saját-prowlarr-vagy-jackett-hozzáadása)).

## A kép saját építése

A tároló egy másolatából:

```sh
docker build -t rookparcel-nox .
```

Az építés egy Rust-konténerben fordítja le a noxot, és egy kis Debian-képbe másolja. A számítógép minden magján néhány percig tart; a `--build-arg CARGO_BUILD_JOBS=1` egyet használ. Ezután a fenti parancsokban a `ghcr.io/balls0fsteel/rookparcel-nox` helyett a `rookparcel-nox` használandó.

## Lásd még

- [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása): minden parancs és kapcsoló.
- [Webes felület](Webes-felület): az oldal, a HTTPS és a biztonság.
- [Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr): az alkalmazások beállításai, a kategóriák és a Remove Completed.
- [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal): annak ellenőrzése, hogy a peerek tudnak-e csatlakozni.
- A kép fájljai: [Dockerfile](https://github.com/Balls0fSteel/Rookparcel/blob/main/Dockerfile) és [contrib/docker](https://github.com/Balls0fSteel/Rookparcel/tree/main/contrib/docker).
