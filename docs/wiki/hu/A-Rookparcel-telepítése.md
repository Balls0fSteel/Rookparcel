*Ez az angol nyelvű [Installing Rookparcel](Installing-Rookparcel) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# A Rookparcel telepítése

Ez az oldal felsorolja az egyes platformok letöltéseit, megmutatja, hogyan telepítheti és indíthatja el őket először, és hogyan távolíthatja el újra a Rookparcelt. Az első indítás után lásd: [Első indítás és a beállítási útmutató](Első-indítás).

## Honnan tölthető le

Minden kiadás megtalálható a [Kiadások oldalon](https://github.com/Balls0fSteel/Rookparcel/releases). A legújabb kiadott verzió az 1.2.79. Minden kiadáshoz tartoznak kiadási jegyzetek az első indítás lépéseivel platformonként, valamint egy `SHA256SUMS` fájl, amely minden letöltés ellenőrző összegét tartalmazza.

| Platform | Fájl | Mi van benne |
|---|---|---|
| Windows 10 / 11, x64, telepítő | `rookparcel-<version>-windows-x64-setup.exe` | a `rookparcel.exe` (az ablak) és a `rookparcel-nox.exe` (a fej nélküli démon), Start menü-bejegyzéssel és eltávolítóval telepítve |
| Windows 10 / 11, x64, hordozható | `rookparcel-<version>-windows-x64.zip` | ugyanez a két program; nincs mit telepíteni |
| Linux asztali gép, x86_64 vagy aarch64 | `Rookparcel-<version>-<arch>.AppImage` | a GTK 4-es ablak a könyvtáraival együtt (glibc 2.39 vagy újabb kell hozzá: Ubuntu 24.04, Fedora 40, Debian 13 vagy újabb) |
| Linux asztali gép, Flatpak | `Rookparcel-<version>-<arch>.flatpak` | a GTK 4-es ablak a Flathubról származó GNOME 51 futtatókörnyezeten |
| Linux, sima bináris fájlok | `rookparcel-<version>-linux-<arch>.tar.gz` | a `rookparcel-gtk`, a `rookparcel-nox`, az asztali bejegyzés és a systemd-egységek; a rendszer GTK 4-ét (4.12 vagy újabb) és OpenSSL 3-át használják |
| Linux kiszolgáló, fej nélküli | `rookparcel-nox-<version>-<arch>.AppImage` | a `rookparcel-nox` beépített OpenSSL-lel |
| macOS, Apple silicon vagy Intel | `Rookparcel-<version>-macos.dmg` | a `Rookparcel.app`, a natív macOS-es ablak, egyetlen alkalmazás mindkét Mac-típusra (macOS 11 Big Sur vagy újabb) |
| macOS, fej nélküli | `rookparcel-nox-<version>-macos-<arch>.tar.gz` | a `rookparcel-nox` Apple siliconhoz (`arm64`, macOS 11 vagy újabb) és Intelhez (`x86_64`, macOS 10.14 vagy újabb) |

Linuxon és a BSD-ken az ablak a GTK-s változat, macOS-en natív Mac-alkalmazás. Mindkettő a Windows-os változathoz hasonlóan működik; ahol eltérnek, a wiki oldalai megmondják. A fej nélküli démon futtatásáról lásd: [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

*A natív macOS-es alkalmazás az 1.2.19-es verzióban jelent meg. Az előtte kiadott verziókban helyette `Rookparcel-<version>-arm64.dmg` van: az Apple silicon számára csomagolt GTK-s ablak, macOS 15 vagy újabb rendszerre.*

## Windows

### Telepítő vagy zip

- **A telepítő** felveszi a Rookparcelt a Start menübe, társíthatja a `.torrent` fájlokkal és a `magnet:` hivatkozásokkal, felvesz egy eltávolítási bejegyzést a Beállítások > Alkalmazások alá, és lehetővé teszi, hogy a Rookparcel magát frissítse (lásd: [A Rookparcel frissítése](A-Rookparcel-frissítése)).
- **A zip** nem igényel telepítést: csomagolja ki bárhová, és indítsa el a `rookparcel.exe`-t. Futhat hordozható módban is, ilyenkor a beállításait és a torrentjeit a saját mappájában tárolja (lásd: [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait#hordozható-mód)). Nem frissíti magát; az újabb verzió új zip.

Mindkettő ugyanazokat a programokat tartalmazza. A Rookparcelhez 64 bites Windows 10 vagy újabb szükséges. Az Arm-alapú Windows 11 az x64-es változatot futtatja.

### A telepítő futtatása

1. Futtassa a `rookparcel-<version>-windows-x64-setup.exe` fájlt.
2. A Windows SmartScreen megjelenítheti a „A Windows védte a számítógépét” üzenetet, mert a telepítő nincs kódaláírással ellátva. Kattintson a **További információ**, majd a **Futtatás mindenképp** gombra.
3. Válassza ki a telepítő nyelvét, olvassa el az üdvözlőoldalt, és fogadja el a licencet (PolyForm Noncommercial License 1.0.0).
4. **Felhasználók kiválasztása:**
   - **Telepítés csak a saját részemre** (az alapértelmezett) nem igényel rendszergazdai jogot. A Rookparcel a `%LOCALAPPDATA%\Programs\Rookparcel` mappába kerül.
   - **Telepítés a számítógép minden felhasználója számára** a `C:\Program Files\Rookparcel` mappába telepít. A Windows rendszergazdai jogot kér, és a kérdés „Ismeretlen közzétevő” feliratot mutat. Ha elutasítja, visszakerül erre az oldalra.
5. **Összetevők:** maga a Rookparcel, **Parancsikon az asztalon**, **Társítás a .torrent fájlokkal** és **Társítás a magnet: hivatkozásokkal**. Az utolsó három új telepítésnél be van jelölve.
6. Válassza ki a mappát, vagy hagyja a megjelenítettet, és hagyja, hogy a telepítés lefusson.
7. Az utolsó oldalon hagyja bejelölve a „Rookparcel futtatása” jelölőnégyzetet az elindításhoz.

A társításokat a telepítőt futtató felhasználónak írja be a program. Egy mindenkinek szóló telepítés többi felhasználója a **Beállítások > Tulajdonságok > Általános** oldalon kapcsolhatja be őket.

A telepítő elutasít minden olyan mappát, amelyben `rookparcel.ini` fájl van: az a mappa egy hordozható Rookparcel. Válasszon másik mappát.

### Telepítés régebbi verzió fölé

Futtassa az újabb telepítőt. Megtalálja a telepített Rookparcelt, kihagyja a felhasználók és a mappa oldalát, és ugyanabba a mappába telepít a régi fölé. A beállításai és a torrentjei a helyükön maradnak. Frissítéskor a társítások jelölőnégyzetei bejelöletlenül indulnak, így a társítások úgy maradnak, ahogy voltak.

Ha a Rookparcel fut, a telepítő kéri, hogy lépjen ki belőle (**Fájl > Kilépés**), és kattintson az **Ismét** gombra. Ha ugyanabból a mappából fut a `rookparcel-nox.exe`, például a Rookparcel nox szolgáltatásként, a telepítő kéri, hogy előbb állítsa le. Rendszergazdai parancssorból:

```
sc stop RookparcelNox
```

A frissítés után indítsa újra a szolgáltatást. A telepített példány mindezt magától is el tudja végezni: lásd: [A Rookparcel frissítése](A-Rookparcel-frissítése).

### Csendes telepítés

A telepítő a következő kapcsolókat fogadja, szkriptekhez és rendszergazdáknak:

| Kapcsoló | Hatás |
|---|---|
| `/S` | csendes: nincsenek oldalak, a végén semmi sem indul el |
| `/D=<folder>` | a telepítési mappa; az utolsó argumentumnak kell lennie, idézőjelek nélkül |
| `/CURRENTUSER`, `/ALLUSERS` | saját részre vagy mindenkinek, a felhasználók oldala nélkül. Rendszergazdai jog nélkül az `/ALLUSERS` a Windows kérdésén át kér jogot, csendes módban is |
| `/DESKTOPSHORTCUT` | létrehozza az asztali parancsikont (csendes telepítés egyébként nem készít ilyet) |
| `/ASSOCIATE=torrent,magnet` | a társítások: `torrent`, `magnet` vagy mindkettő |

Például rendszergazdai parancssorból:

```
rookparcel-1.2.79-windows-x64-setup.exe /S /ALLUSERS /ASSOCIATE=torrent,magnet
```

### A zip

1. Csomagolja ki a `rookparcel-<version>-windows-x64.zip` fájlt egy tetszőleges, írható mappába.
2. Indítsa el a `rookparcel.exe`-t. A SmartScreen az első alkalommal figyelmeztet, ahogy fent: **További információ**, majd **Futtatás mindenképp**.

A beállítások és a torrentek a `%APPDATA%\Rookparcel` mappába kerülnek, ugyanoda, ahová a telepített példány is írja őket. Ha ehelyett a zip mappájában szeretné tartani őket, az első indítás előtt tegyen egy `rookparcel.ini` nevű üres fájlt a `rookparcel.exe` mellé. Lásd: [Hordozható mód](Hol-tárolja-a-Rookparcel-a-fájljait#hordozható-mód).

Ha a zipből futó példánnyal szeretné megnyitni a `.torrent` fájlokat és a `magnet:` hivatkozásokat, jelölje be a társításokat a **Beállítások > Tulajdonságok > Általános** oldalon. Ezek ekkor ennek a példánynak a `rookparcel.exe`-jét nevezik meg, ezért megszűnnek működni, ha a mappáját áthelyezi vagy törli.

### Letöltés ellenőrzése

Minden kiadás `SHA256SUMS` fájlja felsorolja az összes fájl SHA-256 értékét. PowerShellben:

```
Get-FileHash .\rookparcel-1.2.79-windows-x64-setup.exe -Algorithm SHA256
```

Az ellenőrző összegnek meg kell egyeznie a fájl sorával a `SHA256SUMS` fájlban.

## Linux

### AppImage

1. Töltse le a `Rookparcel-<version>-x86_64.AppImage` fájlt (ARM-on az `-aarch64` változatot).
2. Tegye futtathatóvá, és indítsa el:

   ```bash
   chmod +x Rookparcel-1.2.79-x86_64.AppImage
   ./Rookparcel-1.2.79-x86_64.AppImage
   ```

3. Ha a rendszeren nincs FUSE 2, indítsa az `--appimage-extract-and-run` kapcsolóval kiegészítve.

Az AppImage glibc 2.39 vagy újabb verziót igényel (Ubuntu 24.04, Fedora 40, Debian 13 vagy újabb). A beállítások a `~/.config/rookparcel/gtk`, a torrentek a `~/.local/share/rookparcel/gtk` mappába kerülnek. Az `.AppImage` fájl mellé tett `rookparcel.ini` fájl ehelyett hordozhatóvá teszi (*új az 1.1.49-ben*).

### Flatpak

A csomag az alkalmazást tartalmazza; a futtatókörnyezet a Flathubról jön. Adja hozzá egyszer a Flathubot, majd telepítse a csomagot:

```bash
flatpak remote-add --if-not-exists --user flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install --user Rookparcel-1.2.79-x86_64.flatpak
flatpak run io.github.balls0fsteel.Rookparcel
```

Ezután ott van az alkalmazások menüjében. A Flatpak írhat a saját mappájába; egy másik lemezen lévő letöltési mappához egyszer hozzáférést kell adni, például így: `flatpak override --user --filesystem=/mnt/data io.github.balls0fsteel.Rookparcel` (a saját mappájával). A Flatpaknak nincs saját frissítésellenőrzése és hordozható módja. A `.torrent` és `magnet:` társításai az asztali környezet dolgai: az asztal beállításaiban válassza a Rookparcelt alapértelmezett programnak.

### Sima bináris fájlok (tar.gz)

A `rookparcel-<version>-linux-<arch>.tar.gz` tartalmazza a `rookparcel-gtk`-t, a `rookparcel-nox`-ot, az asztali bejegyzést és a systemd-egységeket (`systemd/system`, `systemd/user`). A bináris fájlok a rendszer GTK 4-ét (4.12 vagy újabb) és OpenSSL 3-át használják, ahogy az Ubuntu 24.04-ben megvannak, ezért nagyjából ilyen korú vagy újabb disztribúciókon futnak.

Csomagolja ki, és indítsa el a `rookparcel-gtk`-t. A bináris fájl mellé tett `rookparcel.ini` fájl hordozhatóvá teszi (*új az 1.1.49-ben*). A systemd-egységekről lásd: [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

### Fej nélküli kiszolgáló

A `rookparcel-nox-<version>-<arch>.AppImage` csak a démont tartalmazza, beépített OpenSSL-lel; nem jelenik meg a menükben. Lásd: [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

## macOS

1. Nyissa meg a `Rookparcel-<version>-macos.dmg` fájlt, és húzza a `Rookparcel.app`-ot az Alkalmazások mappába.
2. Az alkalmazás ad-hoc aláírású és nincs közjegyzően hitelesítve (notarised), ezért a Gatekeeper először elutasítja. Egyszer engedélyeznie kell:
   - **macOS 15 Sequoia és újabb:** kattintson duplán az alkalmazásra, és a figyelmeztetésben kattintson a **Kész** gombra. Nyissa meg a **Rendszerbeállítások > Adatvédelem és biztonság** oldalt, görgessen a **Biztonság** részhez, és kattintson a **Megnyitás mindenképp** gombra (a gomb nagyjából egy óráig ott van). Erősítse meg a **Megnyitás mindenképp** gombbal és a jelszavával vagy a Touch ID-vel.
   - **macOS 11–14:** Control-kattintással (vagy jobb kattintással) nyissa meg az alkalmazás menüjét, válassza a **Megnyitás** parancsot, majd kattintson a **Megnyitás** gombra.
3. Vagy, akkor is, ha a macOS azt mondja, hogy az alkalmazás „sérült”, távolítsa el a karanténjelzőt a Terminálban:

   ```bash
   xattr -dr com.apple.quarantine /Applications/Rookparcel.app
   ```

A lemezkép ezeket a lépéseket a `How to open Rookparcel.html` fájlban tartalmazza, a webhely [telepítési útmutatója](https://balls0fsteel.github.io/Rookparcel/install.html#macos) pedig képekkel mutatja be.

Az alkalmazás macOS 11 Big Sur vagy újabb rendszeren fut, Apple silicon és Intel Macen egyaránt: az az egy `Rookparcel.app` mindkettőt tartalmazza. A beállítások és a torrentek a `~/Library/Application Support/Rookparcel/mac` mappába kerülnek.

### Frissítés a GTK-s változatról

A korábbi kiadásokban a `Rookparcel.app` GTK-s változat volt Apple silicon számára. Cserélje le az Alkalmazások mappában az új `Rookparcel.app`-ra. Az új alkalmazás az első indításkor átnevezi a régi alkalmazás `gtk` mappáját a `~/Library/Application Support/Rookparcel` alatt `mac`-re, és minden profil mappáját is, így a beállításai, a torrentjei és a profiljai átkerülnek. Ez egyszer történik meg, és csak akkor, ha még nincs `mac` mappa. Előbb lépjen ki a régi alkalmazásból: amíg fut, az új „Egy másik Rookparcel már használja ezt a mappát” üzenetet mutat, és bezárul. Lásd: [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait#macos).

A régi alkalmazás által létrehozott „Rookparcel indítása a rendszerrel” bejegyzés tovább működik. A hordozható mappát úgy használja, ahogy van.

A fej nélküli démonhoz csomagolja ki a `rookparcel-nox-<version>-macos-<arch>.tar.gz` fájlt, és futtassa a `rookparcel-nox`-ot a Terminálból ([A rookparcel-nox futtatása](A-rookparcel-nox-futtatása)).

## Eltávolítás

### Windows-telepítő

1. Lépjen ki a Rookparcelből (**Fájl > Kilépés**).
2. Nyissa meg a **Beállítások > Alkalmazások** oldalt, keresse meg a **Rookparcel** bejegyzést, és válassza az **Eltávolítás** parancsot.
3. Az eltávolítónak egyetlen beállítása van, a **Beállítások és torrentlista**, alapból bejelöletlenül. Jelölje be, ha azt is törölni szeretné, amit a Rookparcel a `%APPDATA%\Rookparcel` mappában tárol: a beállításokat, a torrentlistát és a haladást, az RSS-csatornákat, a távoli kapcsolatokat, az alapértelmezett IP-szűrőt, a nox fájljait és a profilokat.

Az eltávolító törli a társításokat, az Alapértelmezett alkalmazások bejegyzést és a „Rookparcel indítása a rendszerrel” bejegyzést, amelyek ennek a telepítésnek a `rookparcel.exe`-jét nevezik meg. Egy másik példányéi, például egy hordozhatóéi, megmaradnak. A letöltött fájljaihoz soha nem nyúl.

Ha a Rookparcel nox szolgáltatás a telepítési mappából fut, az eltávolító kéri, hogy előbb távolítsa el. Rendszergazdai parancssorból:

```
"%LOCALAPPDATA%\Programs\Rookparcel\rookparcel-nox.exe" service uninstall --all
```

(a saját telepítési mappájával). Csendes eltávolítás: `"%LOCALAPPDATA%\Programs\Rookparcel\uninstall.exe" /S`.

A beállítások minden Windows-felhasználóhoz külön tartoznak. Egy mindenkinek szóló telepítés eltávolítása csak az eltávolítót futtató felhasználó beállításait takarítja el.

### Windows zip

1. Ha bejelölt társításokat vagy a „Rookparcel indítása a rendszerrel” beállítást, előbb vegye ki a jelölést a **Beállítások > Tulajdonságok > Általános** oldalon.
2. Lépjen ki a Rookparcelből, és törölje a mappáját.
3. Törölje a `%APPDATA%\Rookparcel` mappát is, kivéve ha a példány hordozható volt (akkor minden a saját mappájában volt).

### Linux és macOS

- **AppImage vagy tar.gz:** törölje a fájlt vagy a mappát. A beállítások és a torrentlista eltávolításához törölje a `~/.config/rookparcel` és a `~/.local/share/rookparcel` mappát.
- **Flatpak:** `flatpak uninstall --user io.github.balls0fsteel.Rookparcel`.
- **macOS:** húzza a `Rookparcel.app`-ot a Kukába. A beállítások és a torrentlista a `~/Library/Application Support/Rookparcel` mappában vannak.

A letöltött fájlok minden esetben a helyükön maradnak. A mappák teljes listáját lásd: [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait).
