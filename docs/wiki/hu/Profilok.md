*Ez az angol nyelvű [Profiles](Profiles) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Profilok

A profil egy teljes, különálló Rookparcel: saját beállításokkal, torrentlistával, RSS-csatornákkal, portokkal és ablakkal. Ez az oldal a profilok létrehozását és indítását, az egyes profilok által különválasztott dolgokat, valamint a parancsikonokhoz és a parancssorhoz való `--profile` kapcsolót írja le.

*Új az 1.2.0-ban.* Az 1.1.40-ig két Rookparcel futtatására csak két, külön mappában lévő hordozható példány szolgált ([hordozható mód](Hol-tárolja-a-Rookparcel-a-fájljait#hordozható-mód)).

## Mire valók a profilok

- Egy csak a VPN-t használó Rookparcel egy normális mellett: minden profilnak saját **Csak ennek a hálózatnak a használata**, proxy- és port-beállítása van ([Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal)).
- A privát trackerek elkülönítve a nyilvános torrentektől, mindegyik saját megosztási célokkal és keresési forrásokkal.
- Egy beállítás kipróbálása a valódi beállításai érintése nélkül: `--profile test`.

Több profil is futhat egyszerre, mindegyik a saját ablakában. Semmit sem osztanak meg: az egyik profil torrentjét a másik nem látja.

## Az alapértelmezett profil

Profilkapcsoló nélkül a Rookparcel az **alapértelmezett profilt** futtatja: a [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait) oldalon leírt mappákat, pontosan úgy, ahogy a profilok megjelenése előtt. A profilok használatának megkezdésekor semmi sem mozdul el.

A nevesített profil az alapértelmezett mappájában lévő `profiles` mappában él:

| | alapértelmezett profil | `work` profil |
|---|---|---|
| Windows | `%APPDATA%\Rookparcel` | `%APPDATA%\Rookparcel\profiles\work` |
| Linux, BSD | `~/.config/rookparcel/gtk`, `~/.local/share/rookparcel/gtk` | `~/.config/rookparcel/profiles/work/gtk`, `~/.local/share/rookparcel/profiles/work/gtk` |
| macOS | `~/Library/Application Support/Rookparcel/mac` | `~/Library/Application Support/Rookparcel/profiles/work/mac` |
| hordozható mappa | a mappa | benne a `profiles\work` |

## Profilnevek

A név 1–32 karakterből áll: a-z betűk, számjegyek és kötőjelek (-), és betűvel kezdődik. A nagybetűket a program kisbetűnek veszi, így a `Work` és a `work` ugyanaz a profil. A `default` az alapértelmezett profilt jelenti. Bármely más nevet egy erről szóló üzenettel elutasít.

## Profilok létrehozása és indítása a menüből

A **Fájl > Profilok** felsorolja az alapértelmezett profilt (**Alapértelmezett** néven) és minden nevesített profilt. Amelyiket ez az ablak futtatja, az be van jelölve.

| Elem | Mit csinál |
|---|---|
| egy profil neve | elindítja azt a profilt a saját ablakában. Ha már fut, az ablaka előre kerül |
| **Új profil...** | nevet kér („Név (a-z betűk, számjegyek és kötőjelek):”), létrehozza a profilt, és elindítja |
| **Parancsikon létrehozása...** | Windows: az asztalra tesz egy parancsikont, amely ezt a profilt indítja, és a profilról kapja a nevét, például `Rookparcel (work).lnk`. Linux: a profilt felveszi az alkalmazások menüjébe |
| **Profil mappájának megnyitása** | megnyitja ennek a profilnak a mappáját |

Az új profil üresen indul, mint egy új telepítés, és megnyílik vele a [beállítási útmutató](Első-indítás).

## Az ablakok megkülönböztetése

A nevesített profil ablaka a nevét mutatja a címsorban és a tálcaikon súgószövegében: `Rookparcel 1.2.63 [work]`. Az alapértelmezett profil nem mutat nevet. Windowson minden profilnak saját tálcagomb-csoportja van, így külön is kitűzheti őket.

## Mit különít el az egyes profil

**Portok.** Az új profil az első indításakor véletlen figyelő portot választ 10000 és 60000 között, így nem verekszik az alapértelmezett profillal a 6881-ért. Az alapértelmezett profil megtartja a portját. Ha két profil mégis ugyanarra a portra van állítva, az állapotsor a „A(z) N port foglalt” szöveget mutatja; kattintson rá a **Beállítások > Tulajdonságok > Kapcsolat** megnyitásához. Lásd: [Kapcsolat, portok és tűzfal](Kapcsolat-portok-és-tűzfal).

Minden profil a saját portját képezi le a routeren (UPnP, NAT-PMP). A leképezés a profilról kapja a nevét, például `Rookparcel (work)`, így megkülönböztetheti őket a router listájában.

**A webes felület.** Minden profil webes felületének saját portra van szüksége. A Windows-os ablakban, amikor egy profilban bekapcsolja a webes felületet, a **Beállítások > Tulajdonságok > Webes felület** a 8080-tól felfelé az első szabad portot javasolja. Ha a port mégis foglalt, ezt írja: „a(z) N port foglalt, talán egy másik Rookparcel-profil használja”. Lásd: [Webes felület](Webes-felület).

**`.torrent` fájlok és `magnet:` hivatkozások.** Csak egy program nyithatja meg őket, ezért egyszerre egy profilhoz tartoznak. A **Társítás a .torrent fájlokkal** vagy a **Társítás a magnet: hivatkozásokkal** bejelölése egy profil **Beállítások > Tulajdonságok > Általános** oldalán azt a profilt teszi meg a tulajdonosukká. Windowson, ha egy másik profilnál vannak, az oldal ezt közli: „Most a(z) "work" Rookparcel-profil nyitja meg őket; egy jelölőnégyzet bejelölésével ez a profil fogja megnyitni őket.” Egy `.torrent` fájlra duplán kattintva az a társítást birtokló profilban nyílik meg, és ha az nem fut, elindítja.

A Windows-telepítő társítás-jelölőnégyzetei az alapértelmezett profilnak adják őket.

**Indítás a rendszerrel.** A **Rookparcel indítása a rendszerrel** profilonként állítható, így minden profil külön is elindulhat. Windowson `Rookparcel` és `Rookparcel (work)` néven vannak felsorolva.

**Távoli ablakok.** Az az ablak, amely egy távoli `rookparcel-nox`-ot vezérel ([Távvezérlés](Távvezérlés)), a beállításait annak a profilnak a belsejében tárolja, amelyből megnyitották.

## Profil indítása a parancssorból

Adja a parancshoz a `--profile NAME` (vagy a `--profile=NAME`) kapcsolót:

```
rookparcel.exe --profile work
rookparcel-gtk --profile work
```

- Bárhol állhat az argumentumok között, és a nevet soha nem veszi torrentnek: a `rookparcel.exe --profile work file.torrent` a `file.torrent` fájlt a `work` profilhoz adja.
- A `--profile default`, vagy a kapcsoló hiánya, az alapértelmezett profil.
- Ha a profil még nem létezik, az ablak előbb rákérdez: „A(z) "work" profil még nem létezik. Létrehozza?”, és megmutatja a mappát, amelyet létrehozna. A **Nem** semmit sem hozva létre bezárja, így egy elgépelt név nem indít egy üres Rookparcelt.
- Kombinálható a többi kapcsolóval, például a `--minimized` a tálcára indításhoz, vagy a `--remote` egy távoli `rookparcel-nox`-hoz való csatlakozáshoz.

Windows-parancsikon kézi létrehozásához adja hozzá a `--profile work` kapcsolót a `rookparcel.exe` után a parancsikon **Cél** mezőjében. A **Fájl > Profilok > Parancsikon létrehozása...** ugyanezt elvégzi Ön helyett.

## Profilok és hordozható mód

A hordozható Rookparcelnek is vannak profiljai, a hordozható mappán belüli `profiles` mappában. Azzal együtt vándorolnak. Lásd: [Hol tárolja a Rookparcel a fájljait](Hol-tárolja-a-Rookparcel-a-fájljait#hordozható-mód).

## GTK-s eltérések

- A **Fájl > Profilok** ugyanúgy működik. A jelenlegi profil kijelölt elemként látszik.
- A **Parancsikon létrehozása...** a profilt felveszi az alkalmazások menüjébe („Rookparcel (work)”). A profilnak saját asztali bejegyzése van, így a dokk és a tálca az ablakát a saját nevén és ikonjával mutatja.
- A **Flatpakban** a profilok működnek, de csak az alapértelmezett profil indulhat a rendszerrel.

## macOS-es eltérések

- A **Fájl > Profilok** felsorolja a profilokat, az **Új profil...** és a **Profil mappájának megnyitása** elemet; **Parancsikon létrehozása...** nincs.
- Egy `.torrent` fájl vagy `magnet:` hivatkozás megnyitása mindig az alapértelmezett profilhoz jut el: a macOS az alkalmazást választja, nem a profilt. A rendszerrel való indítás profilonként működik.

## A rookparcel-nox profiljai

A `rookparcel-nox` is fogadja a `--profile NAME` kapcsolót, hogy egy számítógépen több démont futtathasson, mindegyiket saját torrentekkel, webes felülettel és szolgáltatással. A démonprofil létrehozás előtt nem kérdez. A figyelő portja a nevéből adódik, így minden indításnál ugyanaz marad. Egy grafikus profil és egy azonos nevű `rookparcel-nox` profil különálló programok, amelyek csak a `profiles/<name>` mappát osztják meg. Lásd: [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

## Profil eltávolítása

1. Zárja be a profil ablakát (**Fájl > Kilépés**).
2. Ha a profil birtokolja a társításokat, vagy indul a rendszerrel, előbb vegye ki a jelölésüket a Tulajdonságok ablakában, vagy adja őket másik profilnak.
3. Törölje a mappáját, a `profiles\<name>` mappát. A **Fájl > Profilok > Profil mappájának megnyitása** megmutatja, hol van.

A profil letöltései a helyükön maradnak.
