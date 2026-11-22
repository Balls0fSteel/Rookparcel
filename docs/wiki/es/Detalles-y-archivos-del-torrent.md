*Esta es una traducción de la página inglesa [Torrent details and files](https://github.com/Balls0fSteel/Rookparcel/wiki/Torrent-details-and-files). Si hay diferencias, prevalece la página inglesa.*

# Detalles y archivos del torrent

Esta página explica las pestañas de detalles bajo la lista de torrents (General, Trackers, Pares, Piezas, Archivos, Velocidad y Registro), cómo elegir qué archivos descargar y en qué orden, y la ventana Propiedades del torrent, con los trackers, los límites y el objetivo de siembra de un torrent.

Seleccione un torrent de la lista para rellenar las pestañas. **Opciones > Mostrar detalles** las muestra u oculta; arrastre el borde que hay sobre ellas para cambiar su altura. Rookparcel recuerda la pestaña abierta.

## General

![La pestaña General](images/tab-general.png)

La vista general de un torrent.

- **Descargado:** una barra con una franja por cada pieza que tiene, y el porcentaje completado.
- **Disponibilidad:** una barra que muestra qué piezas tienen los pares conectados, y cuántas copias completas tienen entre todos. Por debajo de 1, ninguno de los conectados tiene todas las piezas.

**Transferencia**

| Campo | Significado |
|---|---|
| **Tiempo transcurrido:** | cuánto tiempo lleva funcionando el torrent |
| **Restante:** | tiempo restante estimado |
| **Desperdiciado:** | datos descartados: piezas que no superaron su comprobación, y duplicados |
| **Descargado:**, **Subido:** | totales de este torrent |
| **Semillas:**, **Pares:** | «3 de 120 conectados»: los conectados, y el total que indica el tracker |
| **Velocidad de bajada:**, **Velocidad de subida:** | velocidades actuales |
| **Límite de bajada:**, **Límite de subida:** | los límites propios del torrent; ∞ si no tiene |
| **Proporción:** | lo subido dividido entre lo descargado |
| **Estado:** | el mismo texto que la columna Estado |

**General**

| Campo | Significado |
|---|---|
| **Guardar como:** | la carpeta donde están los archivos |
| **Restante:** | cuánto queda por descargar |
| **Tamaño total:** | el torrent completo, con los archivos omitidos incluidos |
| **Añadido el:**, **Completado el:** | fechas |
| **Piezas:** | «1200 x 1.00 MB (tiene 800)»: el número de piezas, su tamaño y cuántas tiene |
| **Hash:** | el info-hash del torrent |
| **Su dirección según los trackers:** | su dirección pública, para que pueda saber si su VPN está en uso. *Nuevo en 1.1.58.* |

## Trackers

![La pestaña Trackers](images/tab-trackers.png)

Los trackers del torrent, uno por fila.

| Columna | Significado |
|---|---|
| **Nombre** | la dirección del tracker |
| **Estado** | **Funciona**, **Funciona:** con el mensaje del tracker, **Actualizando...**, o **Error** / el mensaje de error del tracker; vacío si aún no se contactó con él |
| **Actualizar en** | tiempo hasta el siguiente anuncio |
| **Semillas**, **Pares**, **Descargado** | las cifras del enjambre según las indica el tracker |

Para contactar con los trackers ahora, use **Actualizar tracker** en el menú contextual. Para añadir, quitar o reordenar trackers, use [Propiedades](#propiedades-del-torrent).

Haga clic en el encabezado de una columna para ordenar los trackers por ella, como en la [lista de archivos](#ordenar-la-lista-de-archivos): **Semillas**, **Pares** y **Descargado** empiezan por los de más, las demás columnas por los más pequeños; un segundo clic invierte el orden, y un tercero vuelve al orden propio del torrent (sus niveles). Un tracker que no ha indicado una cifra se queda al final en ambos casos. Rookparcel recuerda la ordenación. *Nuevo en 1.2.69.*

## Pares

![La pestaña Pares](images/tab-peers.png)

Los pares a los que está conectado. Una semilla web muestra su dirección en la columna **IP**.

| Columna | Significado |
|---|---|
| **IP** | la dirección del par |
| **Cliente** | el programa BitTorrent del par |
| **Indicadores** | véase más abajo |
| **%** | qué parte del torrent tiene el par |
| **Vel. bajada**, **Vel. subida** | velocidad desde y hacia este par |
| **Pet** | solicitudes en espera, nuestras y hacia nosotros |
| **Descargado**, **Subido** | totales con este par |

Haga clic en el encabezado de una columna para ordenar los pares por ella: **%**, las velocidades, **Pet** y los totales empiezan por los de más; **IP**, **Cliente** e **Indicadores**, por los más pequeños (las direcciones, por su número); un segundo clic invierte el orden, y un tercero vuelve al orden en que llegaron los pares. La lista mantiene su ordenación mientras los pares van y vienen y cambian sus cifras, y los pares seleccionados siguen seleccionados. Rookparcel recuerda la ordenación. *Nuevo en 1.2.69.*

Los **Indicadores** son las letras de µTorrent:

| Indicador | Significado |
|---|---|
| D / d | usted descarga del par / quiere hacerlo, pero ahora no envía |
| U / u | usted sube al par / este quiere datos, pero usted no envía ahora |
| O | desbloqueo optimista: usted le da una oportunidad al par |
| S | ignorado (snubbed): el par ha dejado de enviar |
| I | el par se conectó con usted |
| E / e | la conexión está cifrada / solo lo está su protocolo de enlace (handshake) |
| P | la conexión usa uTP |
| H | el par se encontró a través de DHT |
| X | el par se encontró a través del intercambio de pares |
| L | el par se encontró en su red local |

## Piezas

Un mapa con un cuadrado por pieza. Los cuadrados azules son piezas que usted tiene. Los naranjas se están descargando. Los demás son tonos de verde: cuanto más oscuros, más pares conectados tienen esa pieza; el blanco significa que nadie la tiene. La línea sobre el mapa cuenta las piezas. Cuando hay más de las que caben, el mapa termina con «... más».

La lista bajo el mapa muestra las piezas que se están descargando ahora: su número (**#**), **Tamaño**, **N.º de bloques**, una barra de **Bloques**, y cuántos bloques están **Completados**. En la barra, los bloques azules están en el disco, los verdes se están escribiendo, los naranjas se han solicitado a un par y los blancos siguen libres.

## Archivos

![La pestaña Archivos](images/tab-files.png)

Los archivos del torrent en sus carpetas, tal como los muestra la [ventana Añadir nuevo torrent](Añadir-torrents#contenido-del-torrent).

| Columna | Significado |
|---|---|
| **Nombre** | el nombre del archivo o de la carpeta; las carpetas se abren y se cierran con la flecha que hay junto a ellas |
| **Tamaño** | su tamaño |
| **Hecho** | cuánto de él está descargado |
| **%** | lo mismo en porcentaje |
| **N.º de piezas** | cuántas piezas abarca |
| **Primera pieza** | el número de su primera pieza |
| **Prioridad** | **Alta**, **Normal**, **Baja** o **No descargar**; **Mixta** para una carpeta cuyos archivos difieren |

Una fila de carpeta suma el **Tamaño** y lo **Hecho** de sus archivos; no tiene columnas de piezas.

### Ordenar la lista de archivos

*Nuevo en 1.2.65 en la ventana de Windows; en la ventana GTK y en la aplicación de macOS, nuevo en 1.2.67.*

Haga clic en el encabezado de una columna para ordenar los archivos por ella; una flecha en el encabezado indica la columna y la dirección.

- El primer clic ordena los nombres, los tamaños y las piezas empezando por los más pequeños, y **Hecho**, **%** y **Prioridad** empezando por los más altos. Un segundo clic invierte el orden, y un tercero vuelve al orden propio del torrent.
- En el árbol, se ordenan los archivos y las carpetas de cada carpeta, con las carpetas primero.
- Una lista ordenada por **Hecho** o **%** se reordena a medida que avanza la descarga, y una ordenada por **Prioridad**, cuando cambia las prioridades. Los archivos seleccionados siguen seleccionados.
- Rookparcel recuerda la ordenación. **Priorizar por orden de archivos** sigue usando el orden del torrent, sea cual sea la ordenación.

### Vista de árbol o una sola lista

*Nuevo en 1.2.21.*

El botón **Vista de árbol** junto al cuadro de filtro, o **Vista de árbol** en el menú contextual, alterna entre las carpetas y la lista única de rutas de µTorrent 2.2.1 (con la carpeta propia del torrent incluida). Rookparcel recuerda la elección. En el árbol:

- Haga clic en la flecha, haga doble clic en una carpeta o pulse <kbd>Right</kbd> / <kbd>Left</kbd> para abrirla o cerrarla. <kbd>Left</kbd> sobre un archivo va a su carpeta.
- Las carpetas siguen abiertas o cerradas mientras se actualiza la lista y después de cambiar las prioridades. Al seleccionar otro torrent se vuelven a abrir todas las carpetas.
- Una prioridad, **No descargar** o **Priorizar por orden de archivos** elegidos en una carpeta se aplican a todos los archivos que contiene (solo a los mostrados mientras hay un filtro activo). **Abrir** y **Abrir carpeta contenedora** en una carpeta abren esa carpeta; **Reubicar...** es para archivos sueltos.

### Elegir archivos y su orden

Seleccione uno o varios archivos y haga clic con el botón derecho:

| Elemento | Qué hace |
|---|---|
| **Abrir** | abre el archivo con su programa predeterminado, como lo haría un doble clic en el Explorador; no ocurre nada mientras aún no está en el disco |
| **Abrir carpeta contenedora** | abre su carpeta en el Explorador, con el archivo seleccionado una vez que existe; hasta entonces, la carpeta más cercana del torrent que exista |
| **Priorizar por orden de archivos** | da a los archivos seleccionados prioridades decrecientes según su orden en el torrent, de modo que terminan uno tras otro; los archivos que había omitido se vuelven a descargar |
| **Prioridad > Prioridad alta / Prioridad normal / Prioridad baja** | define la prioridad; los archivos de prioridad más alta reciben sus piezas primero |
| **No descargar** | omite los archivos |
| **Reubicar...** | elige un nuevo lugar para un archivo y lo mueve allí; si el archivo no está en el disco, le indica al torrent que lo busque allí |

Hacer doble clic en un archivo, o pulsar <kbd>Enter</kbd> sobre él, lo abre como **Abrir**.

**No descargar** es la forma de omitir archivos después de añadir un torrent. Si vuelve a elegir una prioridad, se descarga un archivo omitido. Para omitir archivos antes de que empiece la descarga, desmárquelos en la [ventana Añadir nuevo torrent](Añadir-torrents#contenido-del-torrent). Para omitir los archivos con determinados nombres en todos los torrents que añada, use **No descargar los archivos llamados:** en Preferencias > Interfaz ([Archivos omitidos por nombre](Añadir-torrents#archivos-excluidos-por-nombre); *nuevo en 1.2.63*).

Después de **Reubicar...** hacia un archivo que no estaba en el disco, ejecute **Forzar comprobación** para que Rookparcel cuente lo que encontró allí. Reubicar nunca reemplaza un archivo que ya esté en el nuevo lugar.

### Filtrar la lista de archivos

*Nuevo en 1.1.56.*

Un torrent con miles de archivos resulta más fácil de manejar con el cuadro **Filtrar archivos**, arriba a la derecha en la pestaña Archivos.

1. Haga clic en el cuadro, o pulse <kbd>Ctrl</kbd>+<kbd>F</kbd> mientras la pestaña Archivos tiene el foco.
2. Escriba una o más palabras. La lista muestra solo los archivos cuya ruta, carpetas incluidas, contiene todas las palabras, sin distinguir mayúsculas y minúsculas; en el árbol, sus carpetas se muestran abiertas. «12 de 3,400 archivos» junto al cuadro indica cuántos coinciden.
3. Seleccione los archivos que quiere y use el menú contextual. <kbd>Ctrl</kbd>+<kbd>A</kbd> selecciona solo los archivos mostrados.
4. <kbd>Esc</kbd> vacía el cuadro; <kbd>Enter</kbd> pasa a la lista.

El texto se mantiene al seleccionar otro torrent, de modo que puede aplicar el mismo filtro a varios. No se guarda al cerrar Rookparcel. **Priorizar por orden de archivos** ordena todos los archivos del torrent, incluso con un filtro activo.

Por ejemplo, escriba `sample` y defina los archivos que coincidan como **No descargar**, o escriba `s01 mkv` para encontrar los archivos de vídeo de la primera temporada.

## Velocidad

Un gráfico de la velocidad de bajada (verde) y de subida (rojo) de todos los torrents juntos durante los últimos diez minutos, con los valores actuales arriba.

## Registro

El registro de Rookparcel de esta sesión, con la hora en cada línea. Muestra los torrents añadidos y terminados, los movimientos, los cambios del programador y de Apagado automático, los problemas con las carpetas vigiladas y el filtro IP, las descargas RSS, las importaciones y más. Conserva las últimas 1000 líneas.

Un tracker que no responde se muestra en la [pestaña Trackers](#trackers), no aquí: con torrents públicos eso es habitual e inofensivo, porque los pares también proceden de DHT y del intercambio de pares. La excepción es un torrent privado, que solo tiene sus trackers: cuando ninguno responde, el Registro lo indica una vez, con el último error.

Cuando algo no funciona como se espera, mire aquí primero. Consulte también [Solución de problemas y preguntas frecuentes](Solución-de-problemas-y-preguntas-frecuentes).

## Propiedades del torrent

![La ventana Propiedades del torrent](images/properties.png)

Ábrala con **Propiedades** en el menú contextual, <kbd>Enter</kbd> en la lista de torrents, o un doble clic en un torrent que no está terminado. El título muestra el nombre del torrent, o «Propiedades - 3 torrents» si hay varios seleccionados.

**Trackers (una línea en blanco entre niveles)**

Una dirección de tracker por línea. Una línea en blanco inicia un nuevo nivel: Rookparcel prueba primero los trackers del primer nivel y pasa al siguiente nivel solo cuando todos fallan. Cada línea debe empezar por `http://`, `https://` o `udp://`; una dirección repetida se descarta.

**Ajustes de ancho de banda**

| Ajuste | Significado |
|---|---|
| **Velocidad máxima de subida (kB/s): [0: ilimitada]** | el límite de subida de este torrent |
| **Velocidad máxima de bajada (kB/s): [0: ilimitada]** | el límite de bajada de este torrent |
| **Reemplazar el número predeterminado de ranuras de subida** y **Número de ranuras de subida: [0: ilimitado]** | a cuántos pares sube este torrent a la vez, en lugar del valor predeterminado de Preferencias > BitTorrent |

**Objetivo de siembra**

Marque **Reemplazar los ajustes predeterminados** para dar a este torrent su propio objetivo en lugar del de Preferencias > Cola:

| Ajuste | Significado |
|---|---|
| **Proporción mínima (%): [0: sin objetivo de proporción]** | subir este porcentaje de lo descargado; 150 significa una vez y media |
| **Tiempo mínimo de siembra (minutos): [0: sin objetivo de tiempo]** | sembrar al menos este tiempo |

Si se definen ambos, deben alcanzarse los dos. Lo que ocurre entonces se define en Preferencias > Cola; consulte [Cola y siembra](Cola-y-siembra). Un torrent iniciado con Forzar inicio nunca alcanza su objetivo.

**Otros ajustes**

| Ajuste | Significado |
|---|---|
| **Siembra inicial** | como en el menú contextual; solo para torrents que están sembrando |
| **Activar DHT** | permite que este torrent encuentre pares a través de DHT |
| **Intercambio de pares** | permite que este torrent conozca pares a través de otros pares |
| **Descargar en orden secuencial** | descarga las piezas en orden, como lo hace **Avanzado > Descargar en orden secuencial** en el menú contextual; atenuado cuando todos los torrents están terminados |
| **Descargar primero las piezas primera y última** | descarga primero los dos extremos de cada archivo deseado, como lo hace **Avanzado > Descargar primero las piezas primera y última** |

*Las dos casillas de orden de descarga son nuevas en 1.2.63.* Consulte [Gestionar torrents](Gestionar-torrents#orden-de-descarga) para saber cuándo ayudan.

Los torrents privados nunca usan DHT ni el intercambio de pares.

**Aceptar** aplica lo que haya cambiado.

### Varios torrents a la vez

Seleccione varios torrents y abra Propiedades para cambiarlos juntos. Un campo cuyo valor difiere entre ellos se carga vacío, y una casilla muestra un tercer estado, mixto. Un campo que deje tal como se cargó no se toca en ningún torrent; solo lo que cambie se aplica a todos.

## En la versión GTK (Linux, BSD)

- Las mismas siete pestañas con las mismas columnas, el mismo menú y cuadro de filtro de la pestaña Archivos, y la misma ventana Propiedades.
- **Abrir** y **Abrir carpeta contenedora** usan los programas predeterminados y el administrador de archivos de su escritorio.

## En la aplicación de macOS

- Las mismas siete pestañas y columnas, el mismo menú de la pestaña Archivos (con **Abrir** y **Abrir carpeta contenedora** en el Finder), **Vista de árbol** y el cuadro **Filtrar archivos**, y la misma ventana Propiedades.

## En la interfaz web

- Las mismas siete pestañas. La pestaña General muestra además **Creado el:**, **Creado con:**, **Trackers:** y **Comentario:**.
- Haga clic en el encabezado de una columna de la tabla de una pestaña para ordenar por ella.
- La pestaña Archivos también tiene el cuadro **Filtrar archivos**. Su menú contextual tiene las cuatro prioridades: **Prioridad alta**, **Prioridad normal**, **Prioridad baja** y **No descargar**. Abrir archivos y Reubicar no están disponibles desde un navegador.
- **Propiedades** en el menú del torrent abre la misma ventana Propiedades del torrent.

En una ventana conectada con **Archivo > Conectar en remoto...**, el menú de la pestaña Archivos no tiene **Abrir**, **Abrir carpeta contenedora** ni **Reubicar...**, ya que los archivos están en el otro equipo. Consulte [Control remoto](Control-remoto).

## Páginas relacionadas

- [Gestionar torrents](Gestionar-torrents)
- [La ventana principal](La-ventana-principal)
- [Cola y siembra](Cola-y-siembra)
- [Límites de velocidad y programador](Límites-de-velocidad-y-programador)
