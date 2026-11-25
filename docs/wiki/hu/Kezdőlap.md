*Ez az angol nyelvű [Home](Home) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

[Vissza az angol kezdőlapra (Home)](Home)

# Rookparcel wiki

A Rookparcel egy karcsú BitTorrent-kliens a µTorrent 2.2.1 szellemében: kicsi, gyors és natív. Ez a wiki azt mutatja be, hogyan használhatja: a telepítést, a torrentek hozzáadását és kezelését, a beállításokat, a webes felületet és a fej nélküli démont.

A Rookparcel négy programból áll:

| Program | Hol | Mi ez |
|---|---|---|
| `rookparcel.exe` | Windows 10 és 11 | a Windows-os ablak (grafikus felület) |
| `rookparcel-gtk` | Linux, BSD | a GTK 4-es ablak (grafikus felület): ugyanaz a főablak és ugyanazok a párbeszédablakok, néhány eltéréssel, amelyeket az egyes oldalak megemlítenek |
| `Rookparcel.app` | macOS 11 vagy újabb, Apple silicon és Intel | a natív macOS-es ablak (grafikus felület): ugyanaz a főablak és ugyanazok a párbeszédablakok, a Mac menüsorával és <kbd>⌘</kbd> billentyűparancsaival |
| `rookparcel-nox` | mindegyik | ablak nélkül fut, böngészőből, a webes felületén át vezérelhető |

A legtöbb oldal a Windows-os ablakot írja le, majd megmondja, miben tér el a GTK-s ablak és a webes felület. A macOS-es alkalmazás a GTK-s ablakhoz hasonlóan működik; ahol eltér, az oldalon macOS-szakasz van: [A Rookparcel telepítése](A-Rookparcel-telepítése#macos), [Billentyűparancsok](Billentyűparancsok#macos), [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás#macos), [A beállítások leírása](A-beállítások-leírása#különbségek-macos-en) és [Automatikus leállítás](Automatikus-leállítás). *A natív macOS-es alkalmazás az 1.2.19-es verzióban jelent meg; az előtte kiadott verziók macOS-en az Apple silicon számára készült GTK-s ablakot tartalmazzák.*

**Verzió:** a wiki a jelenlegi kódot (1.2.80) követi, amely egyben a legújabb kiadott verzió is: [1.2.80](https://github.com/Balls0fSteel/Rookparcel/releases/latest). Az oldalak megmondják, melyik verzióban jelent meg egy újabb funkció.

**Más nyelvek:** [Français](Accueil) · [Español](Inicio) · [Deutsch](Startseite) · [Magyar](Kezdőlap). A fordítások ezt az angol wikit követik; ha eltérnek, az angol oldal a mérvadó.

## Első lépések

- [A Rookparcel telepítése](A-Rookparcel-telepítése)
- [Első indítás és a beállítási útmutató](Első-indítás)
- [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait) (és a hordozható mód)
- [Profilok](Profilok): több független Rookparcel egymás mellett
- [A Rookparcel frissítése](A-Rookparcel-frissítése)

## Mindennapi használat

- [Torrentek hozzáadása](Torrentek-hozzáadása): fájlok, magnet hivatkozások, URL-ek, az Új torrent hozzáadása ablak
- [A főablak](A-főablak)
- [Torrentek kezelése](Torrentek-kezelése): indítás, leállítás, eltávolítás, címkék és jelölők, a sor sorrendje
- [Torrent részletek és fájlok](Torrent-részletek-és-fájlok): a részletlapok, fájlok kiválasztása, a torrent jellemzői
- [Torrentek készítése](Torrentek-készítése)
- [Importálás µTorrentből és qBittorrentből](Importálás-uTorrentből-és-qBittorrentből)
- [Torrentek keresése](Torrentek-keresése) és a lépésenkénti [Torrentek keresése Windowson](Torrentek-keresése-Windowson)
- [RSS letöltő](RSS-letöltő)

## Sebesség, hálózat és lemez

- [Sebességkorlátok és ütemező](Sebességkorlátok-és-ütemező)
- [Alacsony ping](Alacsony-ping) (játékos mód)
- [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal), VPN-ek és proxyk
- [Várólista és seedelés](Várólista-és-seedelés)
- [Lemez és tárolás](Lemez-és-tárolás)
- [Automatikus leállítás](Automatikus-leállítás)

## Távoli és fej nélküli használat

- [Webes felület](Webes-felület)
- [Távvezérlés](Távvezérlés): egy asztali ablak, amely egy máshol futó Rookparcelt vezérel
- [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása): a fej nélküli démon és a szolgáltatások
- [Rookparcel Dockerben](Rookparcel-Dockerben): a rookparcel-nox tárolórendszerképként
- [Sonarr, Radarr és Prowlarr](Sonarr-Radarr-és-Prowlarr)

## Referencia

- [A beállítások leírása](A-beállítások-leírása)
- [Sötét mód](Sötét-mód)
- [Billentyűparancsok](Billentyűparancsok)
- [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás)
- [A Rookparcel zárolása](A-Rookparcel-zárolása): az ablakok elrejtése PIN-kód mögé, és az elfelejtett PIN-kód eltávolítása
- [Hibaelhárítás és GYIK](Hibaelhárítás-és-GYIK)

## Továbbiak

- [Webhely](https://balls0fsteel.github.io/Rookparcel/): letöltések, hírek, GYIK, teljesítménymérések
- [Kiadások](https://github.com/Balls0fSteel/Rookparcel/releases)
- [Hibajelentések és kérdések](https://github.com/Balls0fSteel/Rookparcel/issues)
- Fejlesztői dokumentáció: a tárhely [docs](https://github.com/Balls0fSteel/Rookparcel/tree/main/docs) mappája
