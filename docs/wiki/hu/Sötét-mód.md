*Ez a [Dark mode](Dark-mode) oldal magyar fordítása. Eltérés esetén az angol oldal az irányadó.*

# Sötét mód

A Rookparcel minden platformon követi a rendszer világos vagy sötét beállítását, a Windowsos program és a webes felület pedig választást is kínál.

## Windows

Alapértelmezés szerint az ablak az alkalmazásokra vonatkozó Windows-beállítást követi: **Beállítások > Személyre szabás > Színek > Mód kiválasztása**. A **Beállítások > Tulajdonságok > Felület > Téma** három lehetőséget kínál:

- **Rendszer (világos vagy sötét)**: a Windowst követi, és a program futása közben is vele együtt változik.
- **Világos** és **Sötét**: rögzíti a színeket, bármit mond is a Windows.

Minden ehhez igazodik: az ablak és a menüi, az eszköztár, a torrentlista, a kategóriák, a részletek lapjai, az állapotsor és minden párbeszédablak, a Tulajdonságok ablakot is beleértve. Az ikonokból van világos és sötét készlet. A Windows **Nagy kontraszt** módja mindig erősebb: ha be van kapcsolva, a Rookparcel megtartja a rendszer színeit.

A sötét módhoz **Windows 10 1809-es vagy újabb verziója** kell. Régebbi Windows, vagy olyan, amelyen hiányoznak a sötét módhoz használt részek, megjegyzést mutat a Téma mező alatt, és világos marad. Három dolog a rendszer saját megjelenését viseli: az üzenetablakok, a fájlválasztó (ez a Windows saját módját követi, nem a Téma beállítást), valamint a frissítés letöltésének folyamatjelzője, amíg még nincs ismert mérete.

## Linux (GTK)

A GTK-s program a GTK-témát követi: például egy sötét témát, mint az Adwaita-dark vagy a Breeze-Dark, a `gtk-application-prefer-dark-theme` beállítást vagy a `GTK_THEME=Adwaita:dark` változót. Az asztali környezet önmagában vett sötét beállítása (a GNOME vagy a KDE *sötét* stílusa) nem sötétíti el a sima GTK 4-es ablakokat, ezért a Rookparcel világos marad, amíg a GTK-téma is sötét nem lesz. A saját rajzolt részei (a folyamatjelzők, a részek térképe, a sebességgrafikon, az ütemező) és az ikonok a témával együtt váltanak.

## macOS

A Mac-es alkalmazás a rendszer megjelenését követi.

## Webes felület

Az oldal alapértelmezés szerint a böngésző vagy a rendszer beállítását követi, és saját témákat is kínál, köztük saját CSS-fájlokat is: lásd [Webes felület](Webes-felület).
