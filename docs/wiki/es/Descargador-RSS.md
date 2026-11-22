*Esta es una traducción de la página inglesa [RSS Downloader](https://github.com/Balls0fSteel/Rookparcel/wiki/RSS-Downloader). Si hay diferencias, prevalece la página inglesa.*

# Descargador RSS

Esta página explica cómo seguir canales RSS en Rookparcel y descargar desde ellos: añadir canales, descargar elementos a mano, filtros que descargan automáticamente los elementos coincidentes, coincidencia de episodios y de calidad, y el historial. Abarca primero la ventana de Windows, y después la ventana GTK (Linux, BSD) y la interfaz web.

El Descargador RSS funciona como el de µTorrent 2.2.1. Muchos sitios de torrents y trackers publican un canal con sus novedades; Rookparcel revisa sus canales cada 15 minutos y añade lo que piden sus filtros.

## Abrir el Descargador RSS

- **Opciones > Descargador RSS...**
- El botón **Descargador RSS** de la barra de herramientas.
- Haga clic con el botón derecho en **Canales** (o en un canal) en la lista de categorías de la izquierda, y después en **Descargador RSS...**.

![La ventana del Descargador RSS](images/rss-downloader.png)

La ventana tiene la lista **Canales** a la izquierda, con **Añadir...**, **Editar...**, **Quitar** y **Actualizar** debajo, y dos pestañas a la derecha: **Filtros** e **Historial**. Los cambios se aplican de inmediato; no hay Aceptar ni Cancelar, solo **Cerrar**. Al cerrar la ventana, los filtros se ejecutan sobre los elementos actuales de los canales, de modo que un filtro nuevo puede descargar enseguida.

## Añadir un canal

1. En el Descargador RSS, haga clic en **Añadir...** bajo **Canales**. (O haga clic con el botón derecho en **Canales** en la lista de categorías de la ventana principal y elija **Añadir canal RSS...**.)
2. **URL del canal:** la dirección del canal. Si hay una dirección de canal en el portapapeles, se rellena por usted.
3. **Alias personalizado:** un nombre para el canal. Si se deja vacío, se muestra el título propio del canal.
4. En **Suscripción**, elija una opción:
   - **No descargar automáticamente todos los elementos** (la opción habitual): solo sus filtros, o usted, descargan de este canal.
   - **Descargar automáticamente todos los elementos publicados en el canal**: se descarga cada elemento nuevo. **Usar el filtro inteligente de episodios**, debajo, descarga cada episodio solo una vez (consulte [Filtro de episodios inteligente](#filtro-de-episodios-inteligente)).
5. Haga clic en **Aceptar**. El canal se obtiene de inmediato.

Con **Descargar automáticamente todos los elementos publicados en el canal** elegido, sus filtros no se usan para ese canal: se toma todo lo nuevo.

**Canales que necesitan inicio de sesión.** Los canales de algunos trackers privados necesitan las cookies de su inicio de sesión. Añádalas a la URL como hacía µTorrent, después de `:COOKIE:`:

```
https://tracker.example/rss.php:COOKIE:uid=12345;pass=abcdef
```

Las cookies se envían solo al sitio propio del canal (el mismo servidor o uno de sus subdominios), y nunca desde un canal `https` hacia un enlace `http` simple.

## Administrar canales

La lista **Canales** empieza con **Todos los canales**, y después cada canal. Bajo la lista, una línea muestra el estado del canal seleccionado: «Actualizado» con la hora y el número de elementos, «Aún sin actualizar», «Desactivado», o «Error:» y el motivo. Con **Todos los canales** seleccionado muestra cuántos canales tiene.

| Botón o entrada de menú | Qué hace |
|---|---|
| **Editar...** / **Editar canal...** | Cambia la URL, el alias o la suscripción. Una URL nueva es un canal nuevo: sus elementos antiguos se descartan y se vuelve a obtener. |
| **Quitar** / **Borrar canal** | Quita el canal, después de preguntar. Los filtros definidos para ese canal se conservan, y no coinciden con nada hasta que los dirija a otro canal. |
| **Actualizar** / **Actualizar canal** | Obtiene ahora el canal seleccionado (con **Todos los canales** seleccionado, todos los canales). |
| **Desactivar canal** / **Activar canal** | Detiene o reanuda la revisión del canal, sin quitarlo. Solo en el menú contextual. |

Haga clic con el botón derecho en un canal de la lista Canales del Descargador RSS, o de la lista de categorías de la ventana principal, para ver las mismas entradas.

Los canales se revisan cada 15 minutos. Las descargas de canales y de los archivos `.torrent` a los que apuntan pasan por el proxy definido en Preferencias, si definió uno. Los enlaces de los elementos de un canal solo pueden llevar a direcciones públicas, a menos que el propio canal provenga de un servidor de este equipo o de su red local.

## Descargar elementos a mano

La lista de categorías de la ventana principal tiene una rama **Canales** con cada canal debajo. Haga clic en un canal (o en **Canales** para todos) y la lista de torrents muestra los elementos del canal en lugar de sus torrents, con las columnas **Nombre**, **Episodio**, **Formato**, **Fecha**, **Canal** y **URL**.

- **Episodio** muestra lo que Rookparcel leyó del título: `1x02`, un intervalo como `1x02-03`, un número absoluto como `12` (anime), o una fecha de emisión como `2026-10-19`.
- **Formato** muestra las calidades que nombra el título, como `720p` o `HDTV`.

Para descargar un elemento, haga doble clic en él, o haga clic con el botón derecho y elija **Descargar**. Puede seleccionar varios elementos antes. Un elemento que descargue a mano recibe el nombre del canal como etiqueta y pasa al Historial, como lo haría una descarga de un filtro.

## Filtros

Un filtro descarga los elementos cuyos títulos coinciden con él, de un canal o de todos. Abra la pestaña **Filtros** y haga clic en **Añadir**. Aparece en la lista un filtro llamado «Nuevo filtro», marcado. Después rellene sus campos:

| Campo | Qué hace |
|---|---|
| **Nombre:** | El nombre del filtro en la lista. La casilla de la lista activa o desactiva el filtro. |
| **Filtro:** | Con qué debe coincidir el título. Consulte [Escribir un filtro](#escribir-un-filtro). Vacío no coincide con nada; `*` coincide con todo. |
| **Excepto:** | Los títulos que coinciden con esto se omiten, aunque **Filtro** coincida. Misma sintaxis. Vacío no omite nada. |
| **Guardar en:** | La carpeta de las descargas. Vacío usa la carpeta de descarga predeterminada. El botón **...** elige una carpeta. |
| **Canal:** | **Todos los canales**, o un canal. |
| **Calidad:** | Haga clic en el botón y marque las calidades que acepta. **Todas** acepta cualquier título, incluidos los títulos que no nombran ninguna calidad. |
| **Número de episodio:** | Qué episodios tomar, por ejemplo `1x12-14`. Vacío toma cualquiera. Consulte [Números de episodio](#números-de-episodio). |
| **Intervalo mínimo:** | Después de que el filtro descargue algo, no descarga nada más hasta que haya pasado este tiempo. **0 (ignorar)** significa sin espera. Opciones de 2 horas a 1 mes. |
| **Etiqueta:** | La etiqueta que reciben las descargas. Vacío usa el nombre del canal. Para las etiquetas, consulte [Gestionar torrents](Gestionar-torrents). |
| **Filtro de episodios inteligente** | Descarga cada episodio solo una vez, sea cual sea su calidad o su grupo de publicación. Véase más abajo. |
| **Dar a la descarga la máxima prioridad** | Coloca la descarga al principio de la cola. |
| **Añadir el torrent en pausa** | Añade la descarga sin iniciarla. |

Bajo los campos, **Elementos coincidentes en los canales:** muestra los elementos de la última revisión de los canales que el filtro acepta según Filtro, Excepto, Canal, Calidad y Número de episodio. Úselo para probar un filtro antes de cerrar la ventana. (El intervalo mínimo y el filtro de episodios inteligente dependen de lo que se descargó antes, así que la lista no los aplica.)

**Quitar** elimina el filtro seleccionado. **Subir** y **Bajar** lo mueven en la lista. El orden importa: un elemento va al primer filtro que lo acepta, así que ponga los filtros más específicos por encima de los generales.

Los filtros se ejecutan cuando se revisa un canal, y cuando cierra el Descargador RSS. En cada canal, se examinan primero los elementos más antiguos, así que con el filtro de episodios inteligente o un intervalo mínimo gana la primera publicación de un episodio. Un elemento nunca se descarga dos veces del mismo canal.

### Escribir un filtro

Los campos **Filtro** y **Excepto** usan comodines sencillos:

| Lo que escribe | Significado |
|---|---|
| `*` | Cualquier serie de caracteres, incluso ninguno. |
| `?` | Exactamente un carácter. |
| `\|` | Separa alternativas: `show one\|show two` coincide con cualquiera de las dos. |

- No se distinguen mayúsculas y minúsculas.
- Un `.` o un `_` en un título cuenta como un espacio, así que `the show` coincide con `The.Show.S01E02.720p`.
- Una alternativa **sin** comodines coincide en cualquier parte del título: `show` coincide con `The.Show.S01E02`.
- Una alternativa **con** comodines debe coincidir con el título completo: `show*` no coincide con `The.Show.S01E02`, pero `*show*` sí.

Ejemplos:

| Filtro | Excepto | Toma |
|---|---|---|
| `the show` | | todos los títulos que contienen «the show» |
| `*the show*1080p*` | | esa serie solo en 1080p |
| `the show` | `*german*\|*sample*` | esa serie, pero sin publicaciones en alemán y sin muestras (samples) |

### Números de episodio

El campo **Número de episodio** acepta uno o más intervalos, separados por `,` o `;`:

| Lo que escribe | Toma |
|---|---|
| `1x12` o `S01E12` | Ese único episodio. |
| `1x12-14` o `S01E12-S01E14` | Los episodios 12 a 14 de la temporada 1. |
| `S01E01-10` | Los episodios 1 a 10 de la temporada 1 (la temporada puede omitirse después del guion). |
| `S02` | Toda la temporada 2. |
| `S01E05-` | El episodio 5 de la temporada 1 y todo lo posterior. |
| `12` | El episodio 12 de cualquier temporada, para series numeradas de corrido, como el anime. |
| `1-12`, `100-` | Los episodios 1 a 12; el episodio 100 y posteriores. |
| `2026.10.19` | El episodio emitido en esa fecha (para series diarias). |

Rookparcel lee los episodios de títulos escritos como `S01E02` (también `S01E02E03`, `S01E02-E03`), `1x02`, una fecha de emisión (`2026.10.19`, `2026-10-19`), o un número absoluto (`Show - 12`, `Episode 12`). Un número de versión como `Ubuntu - 22.04` o un resumen como `Show - 12.5` no se lee como un episodio. Un paquete de temporada como `Show.S01.Complete` no nombra ningún episodio, así que un filtro con un número de episodio no lo toma. Un campo que Rookparcel no pueda leer no coincide con nada.

### Calidades

La lista **Calidad**: HDTV, TVRip, DVDRip, SVCD, DSRip, DVBRip, PDTV, HR.HDTV, HR.PDTV, DVDR, DVDScr, 720p, 1080i, 1080p, WebRip, SatRip, WEB-DL, BluRay, 480p, 2160p. Un título coincide cuando nombra una de las calidades que marcó.

### Filtro de episodios inteligente

Con **Filtro de episodios inteligente** marcado, el filtro recuerda los episodios que ha descargado y omite las publicaciones posteriores del mismo episodio, sea cual sea su calidad o su grupo de publicación. Es útil cuando un canal incluye el mismo episodio varias veces.

- Un título sin episodio se deja pasar siempre.
- Una publicación marcada REPACK, PROPER, RERIP o REAL de un episodio ya descargado se deja pasar una vez, ya que corrige la publicación anterior.

La opción **Usar el filtro inteligente de episodios** de la suscripción del canal funciona del mismo modo para un canal configurado para descargarlo todo.

## Historial

La pestaña **Historial** muestra lo que añadió el Descargador RSS, lo más reciente primero, con **Nombre**, **Fecha** y **Canal**. Conserva las últimas 1000 entradas. **Borrar historial** vacía la lista; no hace que esos elementos se vuelvan a descargar.

La pestaña Registro de la ventana principal también muestra una línea por cada descarga («RSS:» con el nombre del canal y el título) y por cada error de canal. Una descarga que no se pudo obtener se vuelve a intentar en la siguiente revisión de su canal.

## GTK

La ventana GTK tiene el mismo Descargador RSS, que se abre desde **Opciones > Descargador RSS...**, el botón **Descargador RSS** de la barra de herramientas, o el menú contextual de **Canales** en la barra lateral. Los campos, botones y pestañas son los mismos que arriba. La rama **Canales** de la barra lateral muestra los elementos de un canal en la lista, y **Descargar** en el menú contextual de un elemento (o un doble clic) lo descarga. La ventana Añadir canal RSS también ofrece una dirección de canal tomada del portapapeles.

## macOS

La aplicación de macOS tiene el mismo Descargador RSS, que se abre desde **Opciones > Descargador RSS...**, el botón **Descargador RSS** de la barra de herramientas, o el menú contextual de **Canales** en la barra lateral. Los campos, botones y pestañas son los mismos que arriba. Los elementos de un canal aparecen en la lista cuando elige el canal en la barra lateral; haga doble clic en un elemento, pulse <kbd>Return</kbd> o elija **Descargar** en su menú contextual para descargarlo.

## Interfaz web

La interfaz web también tiene el Descargador RSS. Ábralo con el botón **RSS** de la barra de herramientas (en un teléfono, el menú **Más** de la barra de herramientas, **Descargador RSS...**), o haga clic en un canal bajo **Canales** en la lista de categorías, lo que lo abre en los elementos de ese canal.

El Descargador RSS de la página tiene tres pestañas en lugar de dos:

- **Elementos**: los elementos del canal seleccionado (o de todos los canales), con **Descargar** para los seleccionados.
- **Filtros** e **Historial**: como arriba.

Diferencias:

- **Guardar en** es una carpeta del servidor, no del equipo del navegador. El botón **...** pide una ruta; vacío significa la carpeta de descarga predeterminada del servidor.
- La interfaz web que use decide de quién son los canales que edita:
  - **`rookparcel-nox`** guarda sus propios canales y filtros (en `rss.dat`, junto a `webui.json`) y los ejecuta en el servidor, sin ninguna ventana abierta. Ejecuta el Descargador RSS solo mientras su interfaz web está activada. Consulte [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).
  - **La interfaz web que aloja una ventana de Windows** edita los canales y filtros propios de esa ventana: hay un solo Descargador RSS, mostrado en ambos sitios. Consulte [Interfaz web](Interfaz-web).
- La interfaz web también responde a la parte RSS de la API web de qBittorrent, de modo que una aplicación remota de qBittorrent puede listar canales y definir reglas. Sus reglas usan la sintaxis de comodines de arriba; la opción de expresión regular se ignora, y las carpetas de canales no se admiten.

## Ventanas remotas

Una ventana conectada a otro Rookparcel (Archivo > Conectar en remoto, consulte [Control remoto](Control-remoto)) tiene sus propios canales y filtros, guardados en este equipo. Se revisan desde este equipo, y lo que descargan se añade al servidor remoto.

## Dónde se guardan los canales

Los canales, los filtros y el historial están en `rss.dat` en su carpeta de ajustes, junto a `rookparcel.ini` (para una ventana remota, en su propia carpeta `remote\<host>_<port>` allí). Consulte [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos).

Para búsquedas puntuales en lugar de canales, consulte [Buscar torrents](Buscar-torrents). Para Sonarr y Radarr, que hacen su propia revisión de canales, consulte [Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr).
