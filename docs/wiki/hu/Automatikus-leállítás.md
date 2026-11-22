# Automatikus leállítás

*Ez az [Auto Shutdown](Auto-shutdown) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

Az automatikus leállítás kilépteti a Rookparcelt, vagy készenléti állapotba helyezi, hibernálja vagy leállítja a számítógépet, amint a torrentek elkészültek. Ez az oldal a választási lehetőségeket, a „kész” fogalmát és a művelet előtti visszaszámlálást mutatja be.

## A művelet kiválasztása

Nyissa meg a **Beállítások > Automatikus leállítás** menüt, és válasszon egy tételt:

| Tétel | Mi történik, ha a munka kész |
|---|---|
| **Kikapcsolva** | Semmi. Ez az alapérték. |
| **Kilépés, ha a letöltések befejeződtek** | A Rookparcel kilép, amint egyetlen elindított torrent sem tölt le többé. |
| **Kilépés, ha minden befejeződött** | A Rookparcel kilép, amint egyetlen elindított torrent sem tölt le és nem oszt többé. |
| **Készenlét, ha a letöltések befejeződtek** | A számítógép készenléti (alvó) állapotba lép, amint a letöltések elkészültek. |
| **Készenlét, ha minden befejeződött** | Ugyanez, amint semmi sem tölt le és nem oszt. |
| **Hibernálás, ha a letöltések befejeződtek** | A számítógép hibernálódik, amint a letöltések elkészültek. |
| **Hibernálás, ha minden befejeződött** | Ugyanez, amint semmi sem tölt le és nem oszt. |
| **Leállítás, ha a letöltések befejeződtek** | A számítógép leáll, amint a letöltések elkészültek. |
| **Leállítás, ha minden befejeződött** | Ugyanez, amint semmi sem tölt le és nem oszt. |

![A Beállítások > Automatikus leállítás menü](images/auto-shutdown.png)

A kiválasztott tétel mellett jelölés látható. A Készenlét és a Hibernálás szürke, ha a Windows nem kínálja fel őket ezen a számítógépen (például mert a hibernálás ki van kapcsolva).

A választás csak erre a futásra érvényes: a Rookparcel minden indításkor a **Kikapcsolva** állapotból indul, és miután végrehajtotta a műveletet, ismét **Kikapcsolva** lesz.

## Mi számít késznek

A **letöltések befejeződése** minden olyan elindított torrentre vár, amely még nem fejezte be a letöltést: futó, kényszerített vagy sorban álló torrentre, beleértve azt is, amelyik éppen ellenőrzés alatt áll, a magnet-hivatkozás fájllistáját kéri le, vagy amelyet az ütemező visszatart. Az a torrent, amelynek minden kért fájlja kész, késznek számít, még ha kihagyott fájlok maradtak is.

A **minden befejeződött** ezen felül minden olyan elindított torrentre is vár, amely oszt vagy megosztásra vár sorban. A megosztás akkor fejeződik be, ha a [megosztási cél](Várólista-és-seedelés#megosztási-cél) leállítja. Az a megosztás, amelynek nincs érvényben lévő célja (a cél jelölőnégyzete a Tulajdonságok > Sorba állítás oldalon kezdetben ki van kapcsolva), vagy amelyet kényszerítetten indítottak, tovább oszt, és visszatartja a leállítást, ezért „minden” tételt csak azután válasszon, hogy beállított egy célt.

Mindkét esetben:

- A leállított vagy szüneteltetett torrent, illetve a hibás torrent nem számít: ő Önre vár. A hibásnak számít az a torrent is, amelyet azért tartottak vissza, mert a meghajtója vagy a mappája hiányzik ([Lemez és tárolás](Lemez-és-tárolás#ha-egy-meghajtó-vagy-megosztás-hiányzik)).
- Az a torrent, amelynek a fájljait éppen áthelyezik, az áthelyezés végéig munkának számít.
- Az automatikus leállítás csak azután lép működésbe, hogy a tétel kiválasztása után munkát látott. Ha akkor választja ki, amikor már nincs semmi teendő, a következő letöltésre vár, és nem hajtja végre azonnal a műveletet.

## A visszaszámlálás

Amikor a munka kész, egy kis **Automatikus leállítás** ablak jelenik meg minden ablak fölött, akkor is, ha a Rookparcel a tálcára van rejtve. Kiírja, mi fejeződött be, és 30 másodpercről visszaszámol, például:

> Minden letöltés befejeződött.
> A számítógép 0:30 múlva leáll.

- Kattintson a **Mégse** gombra (vagy nyomja meg az <kbd>Esc</kbd> billentyűt) a lemondáshoz. Az automatikus leállítás ekkor **Kikapcsolva** lesz.
- Ha az ablakokat PIN-kód zárolja ([A Rookparcel zárolása](A-Rookparcel-zárolása)), a visszaszámlálás akkor is látszik, így lemondhatja.
- Ha a visszaszámlálás alatt új munka érkezik, például egy RSS-letöltés, a visszaszámlálás bezárul, és megvárja, hogy ez a munka is elkészüljön; ekkor újra teljes visszaszámlálás indul.
- Ha az idő lejár, végrehajtja a műveletet.

A **Napló** lap minden lépést rögzít: „Automatikus leállítás: kész, ... 30 mp múlva”, „Automatikus leállítás: megszakítva, most kikapcsolva”, „Automatikus leállítás: új feladat, a visszaszámlálás megvárja a végét”.

## Az egyes műveletek végrehajtása

- **Kilépés**: a Rookparcel úgy lép ki, mint a **Fájl > Kilépés** parancsra, és közben elmenti a torrentlistát és a haladást.
- **Leállítás**: a Rookparcel előbb ugyanígy kilép, majd megkéri a Windowst, hogy álljon le és kapcsolja ki a gépet. A Windows nem kényszerített: egy mentetlen munkával rendelkező program még visszatarthatja a leállítást.
- **Készenlét** és **Hibernálás**: a Rookparcel előbb elmenti a torrentlistát és a haladást, majd altatja a számítógépet. Tovább fut, és a számítógép felébredésekor folytatja. Az automatikus leállítás ekkor **Kikapcsolva** lesz.

Ha a Windows elutasítja, üzenet jelzi, például „A számítógép nem állítható le:” az ok megjelölésével. Ezt az üzenetet kapja az a Windows-fiók, amelynek nincs joga a számítógép leállítására.

## A GTK ablakban, macOS-en és a rookparcel-noxban

- **GTK ablak** (Linux, BSD): ugyanaz a **Beállítások > Automatikus leállítás** menü és visszaszámlálás. A Készenlét, a Hibernálás és a Leállítás a rendszer bejelentkezéskezelőjén (systemd-logind) keresztül megy, amely kérheti a jelszavát. Ha ez hiányzik (a BSD-ken, systemd nélküli rendszeren), vagy egy műveletet nem enged meg, ezek a tételek szürkék, és csak a Kilépés tételek maradnak.
- **macOS**: ugyanaz a **Beállítások > Automatikus leállítás** menü és visszaszámlálás. A Készenlét és a Leállítás úgy kéri a macOS-t az altatásra vagy a leállításra, ahogy az Apple menü teszi, Apple-eseményekkel a rendszer „loginwindow” folyamatának. Első alkalommal a macOS megkérdezheti, hogy a Rookparcel vezérelheti-e a „loginwindow”-t; kattintson az **OK** gombra. Ha elutasította, engedélyezze a **Rendszerbeállítások > Adatvédelem és biztonság > Automatizálás** (Rendszerbeállítások > Adatvédelem és biztonság > Automatizálás) területen. A Hibernálás tételek szürkék: a macOS-nek nincs saját hibernálása, amelyet kérni lehetne. *A natív macOS-alkalmazás az 1.2.19-ben új.*
- **Webes felület**: nincs automatikus leállítása.
- **rookparcel-nox**: indítsa a `--exit-when-done downloads` vagy a `--exit-when-done all` kapcsolóval, hogy elmentett állapottal kilépjen, amint semmi sem tölt le, illetve semmi sem tölt le és nem oszt. Megvárja, hogy előbb fusson valami, ahogy a menü is, és néhány másodperccel a munka befejezése után lép ki. Soha nem altatja és nem állítja le a számítógépet. Lásd: [A rookparcel-nox futtatása](A-rookparcel-nox-futtatása).

## Kapcsolódó oldalak

- [Várólista és seedelés](Várólista-és-seedelés)
- [Sebességkorlátok és ütemező](Sebességkorlátok-és-ütemező)
- [Tálca, értesítések és indítás](Tálca-értesítések-és-indítás)
