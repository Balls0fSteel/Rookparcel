*Esta página es una traducción de la página en inglés [Importing from µTorrent and qBittorrent](Importing-from-uTorrent-and-qBittorrent). Si hay diferencias, la página en inglés es la válida.*

# Importar desde µTorrent y qBittorrent

Esta página explica cómo traer sus torrents de µTorrent o qBittorrent en el mismo equipo, para que sigan descargando y sembrando en Rookparcel desde donde están, sin volver a descargar nada.

## Qué se trae

De cada torrent, Rookparcel toma:

- los datos, que se quedan donde están: no se copia ni se mueve nada;
- las estadísticas, las prioridades de los archivos (los archivos omitidos siguen omitidos) y los trackers;
- la etiqueta: la categoría de qBittorrent, o la etiqueta de µTorrent;
- las etiquetas (tags) de qBittorrent, como marcas (*nuevo en la 1.2.63*; antes, la primera etiqueta pasaba a ser la etiqueta de un torrent sin categoría). Un nombre de etiqueta con una coma o un carácter de control se omite;
- un estado detenido o en pausa;
- de qBittorrent, los límites de compartición propios del torrent (proporción y tiempo de siembra), que pasan a ser su propio [objetivo de siembra](Detalles-y-archivos-del-torrent#propiedades-del-torrent).

Rookparcel no se fía del registro del otro programa sobre qué piezas están completas: comprueba los archivos antes de apoyarse en ellos. Por eso los torrents grandes muestran **Comprobando** durante un rato después de la importación.

Los archivos del otro programa solo se leen, nunca se modifican. Los archivos de datos parciales que µTorrent dejó como `.!ut` o qBittorrent como `.!qB` se recogen y se renombran cuando el torrent se ejecuta: consulte [Archivos incompletos y .!ut](Disco-y-almacenamiento#archivos-incompletos-y-ut) (*nuevo en la 1.2.63*). Los torrents que ya están en la lista de Rookparcel se omiten, así que puede ejecutar la importación de nuevo sin riesgo.

## Antes de empezar

1. **Cierre el otro programa**, incluido su icono de la bandeja. Ambos programas reescriben sus archivos al salir, así que Rookparcel se niega a importar mientras ve µTorrent o qBittorrent en ejecución: «... se está ejecutando. Ciérrelo y vuelva a importar: al salir reescribe sus archivos.» En macOS y BSD, Rookparcel no puede saber si se está ejecutando, así que compruébelo usted.
2. Asegúrese de que las unidades con los datos descargados están conectadas, con las mismas letras o rutas que antes.

Después de la importación, no ejecute los dos programas sobre los mismos torrents: ambos escribirían en los mismos archivos.

## Importar

1. Elija **Archivo > Importar de qBittorrent...** o **Archivo > Importar de µTorrent...**.
2. Rookparcel busca en la carpeta habitual del programa (más abajo). Si no está allí, se abre un selector de carpeta: «Seleccione la carpeta de qBittorrent» o «Seleccione la carpeta de µTorrent». Elija la carpeta descrita más abajo.
3. Rookparcel pregunta: «¿Importar 47 torrents de qBittorrent? Sus datos se quedan donde están y los archivos de qBittorrent no se modifican. Los torrents que ya están en la lista se omiten.» La pregunta enumera las carpetas donde están los datos. Revíselas y haga clic en **Sí**.
4. Un mensaje dice cuántos se añadieron, por ejemplo «Se añadieron 45 torrents de qBittorrent. Se omitieron 2; la pestaña Registro indica el motivo.»

Los torrents aparecen en la lista y empiezan a comprobarse. Sus etiquetas aparecen en **Etiquetas** en la lista de categorías, y las etiquetas (tags) de qBittorrent en **Marcas**.

## Dónde busca Rookparcel

| Programa | Carpeta habitual | Qué elegir si se le pregunta |
|---|---|---|
| qBittorrent en Windows | `%LOCALAPPDATA%\qBittorrent\BT_backup` | la carpeta `BT_backup`, o la carpeta `qBittorrent` que la contiene |
| qBittorrent en Linux y BSD | `~/.local/share/qBittorrent/BT_backup` (o bajo `$XDG_DATA_HOME`) | lo mismo |
| qBittorrent en macOS | `~/Library/Application Support/qBittorrent/BT_backup` | lo mismo |
| µTorrent en Windows | `%APPDATA%\uTorrent` | la carpeta que contiene `resume.dat` |

qBittorrent guarda un archivo `.fastresume` y un archivo `.torrent` por torrent en `BT_backup`; µTorrent guarda `resume.dat` junto a los archivos `.torrent`. Si su copia los guarda en otro lugar, elija esa carpeta. Los datos se quedan en las carpetas que nombran los registros, así que esas carpetas deben existir en este equipo.

## Torrents que se omitieron

Cada torrent importado u omitido recibe una línea en la [pestaña Registro](Detalles-y-archivos-del-torrent#registro): «Importado de qBittorrent: ...» o «No importado de qBittorrent: ...: » seguido del motivo.

| Motivo | Qué hacer |
|---|---|
| ya está en la lista | nada; ya está en Rookparcel |
| falta su archivo .torrent | añada usted el torrent, desde su archivo `.torrent` o enlace magnet, con **Guardar en** apuntando a su carpeta de datos |
| no se puede leer su registro de reanudación | lo mismo |
| no se puede leer su archivo .torrent: ... | lo mismo, con una copia buena del archivo `.torrent` |

Cuando Rookparcel no encuentra ningún torrent, dice «No se encontró ningún torrent de qBittorrent en ...» con la carpeta que leyó.

## En la versión GTK (Linux, BSD)

Los mismos dos elementos del menú **Archivo**, con los mismos pasos. µTorrent no tiene carpeta habitual fuera de Windows, así que Rookparcel pide la carpeta con `resume.dat`, por ejemplo una copiada de un equipo con Windows.

## Con rookparcel-nox

El demonio sin ventana importa al iniciarse:

```
rookparcel-nox run --import qbittorrent
rookparcel-nox run --import utorrent --import-dir "D:\Backup\uTorrent"
```

`--import` acepta `qbittorrent` o `utorrent`; `--import-dir` nombra la carpeta cuando no es la habitual (`BT_backup` de qBittorrent o su carpeta de datos, la carpeta de µTorrent con `resume.dat`). El otro programa no debe estar en ejecución, y los torrents que ya están en la lista de nox se dejan como están. Consulte [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

La interfaz web no tiene importación, y en una ventana conectada con **Archivo > Conectar en remoto...** los elementos de importación aparecen atenuados: el servidor no podría leer las carpetas de este equipo. Consulte [Control remoto](Control-remoto).

## Páginas relacionadas

- [Añadir torrents](Añadir-torrents)
- [Gestionar torrents](Gestionar-torrents)
- [Cola y siembra](Cola-y-siembra)
