*Ez a [Keep ping low](Keep-ping-low) című angol oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Alacsony ping

Az Alacsony ping megtartása a Rookparcel „játékos módja”: figyeli az internetvonal késleltetését, és visszafogja a Rookparcel sebességét, amíg a késleltetés nő, hogy a játékok, a videóhívások és a webböngészés a torrentek futása közben is reagálók maradjanak. Ez az oldal azt ismerteti, mit csinál, hogyan kapcsolható be, és mit lát működés közben.

## Miért emelik a torrentek a pinget

A vonal feltöltési iránya általában a legszűkebb része. Amikor a Rookparcel gyorsabban küld, mint amennyit a vonal elbír, a csomagok sorban várakoznak a modemben vagy a routerben, és minden más csomag mögöttük vár: egy játéké, egy hívásé, egy weboldalé. Ezt a sort veszi észre lagként. Ha a vonalnál kicsit lassabban küld, a sor üres marad, a ping pedig az üresjárati értékén marad. A letöltési iránynak ugyanez a gondja a szolgáltató oldalán, a vonal másik végén.

Az Alacsony ping megtartásának nem kell tudnia, melyik játék fut. Magát a sort méri, így bármely játékkal, hívással vagy más programmal működik.

## Bekapcsolása

1. Nyissa meg a **Beállítások > Tulajdonságok** ablakot (<kbd>Ctrl</kbd>+<kbd>P</kbd>), és válassza a **Játék** oldalt.
2. A **Ping** alatt jelölje be az **Alacsony ping megtartása (a sebességek automatikus korlátozása)** négyzetet.
3. Kattintson az **OK** vagy az **Alkalmaz** gombra.

![A Tulajdonságok Játék oldala](images/prefs-gaming.png)

Alapértelmezés szerint ki van kapcsolva. A kikapcsolásához szüntesse meg a négyzet jelölését: a saját korlátai azonnal visszatérnek.

Amikor először bekapcsolja, a Rookparcel megtanulja a vonal üresjárati pingjét. Ez legfeljebb kb. 20 másodpercig tart, és közben a sebességek alacsonyabban indulhatnak.

## Mit csinál

- **Méri a késleltetést.** A Rookparcel másodpercenként körülbelül négyszer pingeli az otthoni vonal utáni első routert (másodpercenként néhány száz bájt). Ha a vonal utáni egyik router sem válaszol, helyette a Haladó beállítás, az `auto_limit_probe_hosts` gépeit pingeli, alapértelmezés szerint: `1.1.1.1,8.8.8.8,9.9.9.9`. A forgalmas uTP-peerek további forgalom nélkül második mérést adnak.
- **Mindkét irányt korlátozza.** Amíg a késleltetés a megengedett értéknél jobban meghaladja az üresjárati értéket, a feltöltési vagy letöltési korlát csökken; amint a vonal újra csendes, a korlát lépésről lépésre nő.
- **Soha nem emeli a saját korlátait.** Az automatikus korlát és az Ön saját általános korlátja közül a kisebbet használja. A beállításai nem változnak.
- **Vannak alsó határai.** A korlát nem süllyed kb. 10 kB/s alá, sem a látott legnagyobb sebesség egy kis hányada alá, így egy zsúfolt Wi-Fi vagy egy másik program feltöltése nem éheztetheti ki teljesen a Rookparcelt.
- **Az uTP korábban enged.** Amíg be van kapcsolva, az uTP-kapcsolatok legfeljebb 25 ms sorban állási késleltetésre törekednek a szokásos 100 ms helyett.

A négyzet alatti megjegyzés is ezt mondja: ezek a pingek a vonal utáni első routerhez mennek, vagy ha az nem válaszol, a tartalék gépekhez, amelyek üzemeltetői látják őket.

## Mit lát

- **Az állapotsor.** Amíg egy irányt az Ön saját korlátja alá korlátoz, annak a része zárójelben mutatja a korlátot, például **U: 412.0 kB/s (auto 450.0 kB/s) T: 2.1 GB**. Korlát nélkül a szokásos módon néz ki.
- **A Napló lap.** A „Keep ping low:” kezdetű sorok megmondják, melyik routert vagy gépeket pingeli, és mikor lesz korlátozott vagy mikor nem korlátozott egy irány.
- **A webes felület** az állapotsorában ugyanezt az „(auto ...)” jelzést mutatja.

Hogy a sebességek alacsonyabbak a vonal maximumánál, amíg ez be van kapcsolva, az várható: ez a tartalék tartja alacsonyan a pinget.

## Finomhangolás

A **Beállítások > Tulajdonságok > Haladó** oldal két beállítása tartozik hozzá. Jelölje ki a sort, adja meg az értéket, kattintson a **Beállítás** gombra, majd az **OK** vagy az **Alkalmaz** gombra.

| Beállítás | Jelentése | Alapértelmezés |
|---|---|---|
| `auto_limit_delay_ms` | Ennyi ezredmásodperc többletping engedélyezett a vonal üresjárati pingje fölött, mielőtt a korlátok csökkennek. Az alacsonyabb simábbá teszi a játékokat és hívásokat; a magasabb több sebességet enged. | 15 |
| `auto_limit_probe_hosts` | Vesszővel elválasztott IP-címek, amelyeket pingel, ha a vonal utáni egyik router sem válaszol. A funkció bekapcsolásakor olvassa be. Ritkán kell módosítani. | `1.1.1.1,8.8.8.8,9.9.9.9` |

Az `utp_target_delay` (alapértelmezés 100) az uTP késleltetési célja, amelyet a funkció 25-re csökkent, amíg be van kapcsolva.

## Platformok

| Rendszer | Hogyan mér |
|---|---|
| Windows | Pingekkel, rendszergazdai jog nélkül, valamint az uTP-peerekkel. |
| Linux | A rendszer jogosultság nélküli ping-socketjein át pingel, valamint az uTP-peerekkel. Ha a rendszer nem engedi, a Napló ezt mondja: „this system does not let Rookparcel ping (its group must be in net.ipv4.ping_group_range)”, és csak az uTP-peereket használja. |
| macOS | Pingekkel, valamint az uTP-peerekkel. |
| FreeBSD és más rendszerek jogosultság nélküli ping nélkül | Csak az uTP-peerekkel. A Napló közli, hogy a pingek nem érhetők el. |

Csak uTP-peerekkel legalább három forgalmas uTP-peer kell ahhoz, hogy bármit mérjen; ha csak TCP-peereknek oszt, nincs mire támaszkodnia.

## Amikor valami más a jobb

Az SQM-et (például CAKE) vagy működő játékos QoS-t tudó router ezt jobban végzi, a ház minden eszközére, nem csak a Rookparcelre. Ha van ilyenje, kikapcsolva hagyhatja az Alacsony ping megtartását.

## Kapcsolódó funkciók

- A foglalt lemez miatti lassítás ugyanazon a **Játék** oldalon ugyanilyen automatikus korlátot használ, lassú vonal helyett lassú lemezre; a két korlát közül a kisebb érvényesül. Lásd a [Lemez és tárolás](Lemez-és-tárolás#ha-a-lemez-foglalt) oldalt.
- A Beállítási útmutató router-oldala megemlíti az Alacsony ping megtartását játékokhoz és hívásokhoz: lásd az [Első indítás](Első-indítás) oldalt.
- Rögzített korlátok és a heti ütemező: [Sebességkorlátok és az ütemező](Sebességkorlátok-és-ütemező).

## A GTK-ablakban és a webes felületen

A GTK-ablakban (Linux, BSD) és a macOS-alkalmazásban ugyanez a négyzet a **Tulajdonságok > Játék** oldalon van (Macen a bejelölésekor azonnal érvénybe lép), a webes felületen pedig a **Tulajdonságok > Játék** oldalán, ugyanazzal a megjegyzéssel. A [rookparcel-noxban](A-rookparcel-nox-futtatása) ez ugyanaz a beállítás, az `auto_rate_limit`, amelyet a webes felület Tulajdonságai közt állíthat.
