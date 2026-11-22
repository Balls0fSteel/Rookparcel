*Esta página es una traducción de la página en inglés [Adding torrents](Adding-torrents). Si hay diferencias, la página en inglés es la válida.*

# Añadir torrents

Esta página muestra todas las formas de dar un torrent a Rookparcel: un archivo `.torrent`, un enlace magnet, una dirección web, arrastrar y soltar, su navegador y su administrador de archivos, y una carpeta vigilada. También explica la ventana Añadir nuevo torrent, donde elige la carpeta y los archivos antes de que empiece la descarga.

## Las formas rápidas

| Qué tiene | Qué hacer |
|---|---|
| Un archivo `.torrent` en su equipo | **Archivo > Añadir torrent...** (<kbd>Ctrl</kbd>+<kbd>O</kbd>), o el primer botón de la barra de herramientas. Puede elegir varios archivos a la vez. |
| Un enlace magnet | **Archivo > Añadir torrent desde URL...** (<kbd>Ctrl</kbd>+<kbd>U</kbd>) y péguelo. |
| La dirección web de un archivo `.torrent` (`http://` o `https://`) | **Archivo > Añadir torrent desde URL...** y péguela. Rookparcel descarga el archivo y lo añade. |
| Un archivo `.torrent` en el Explorador | Arrástrelo a la ventana de Rookparcel, o haga doble clic en él si Rookparcel abre los archivos `.torrent`. |
| Un enlace magnet en su navegador | Haga clic en él, si Rookparcel abre los enlaces magnet. |
| Muchos archivos `.torrent` que llegan con el tiempo | Deje que Rookparcel vigile una carpeta. Consulte [Una carpeta vigilada](#una-carpeta-vigilada). |
| Un resultado de búsqueda | Consulte [Buscar torrents](Buscar-torrents). |

El menú contextual del icono de la bandeja también tiene **Añadir torrent...** y **Añadir torrent desde URL...**.

## Añadir torrent desde URL

![La ventana Añadir torrent desde URL](images/add-url.png)

1. Copie el enlace magnet o la dirección del archivo `.torrent`.
2. Elija **Archivo > Añadir torrent desde URL...**. Si el portapapeles contiene un enlace que empieza por `magnet:`, `http://` o `https://`, el cuadro ya aparece rellenado.
3. Haga clic en **Aceptar**.

Una dirección se descarga en segundo plano, a través de su proxy cuando Preferencias > Conexión envía las conexiones con trackers por uno. Algunos sitios responden a la dirección con un enlace magnet en lugar de un archivo; Rookparcel también lo sigue. Si la descarga falla, un mensaje dice «No se pudo descargar» con la dirección y el motivo.

## Arrastrar y soltar

Suelte uno o más archivos `.torrent` del Explorador sobre la ventana principal. Cada uno se añade como si lo hubiera elegido con **Añadir torrent...**.

## Desde su navegador y su administrador de archivos

Rookparcel puede ser el programa que Windows usa para los archivos `.torrent` y los enlaces `magnet:`. Actívelo en **Opciones > Preferencias > General** con **Asociar con archivos .torrent** y **Asociar con enlaces magnet:**, o en la [Guía de configuración](Primer-inicio). Consulte [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio) para los detalles, incluido qué hacer cuando Windows conserva otro programa.

Cuando Rookparcel ya se está ejecutando y abre un torrent desde su navegador o el Explorador, el Rookparcel en ejecución lo recibe. Con **Activar la ventana del programa** activada (Preferencias > Interfaz, activada de forma predeterminada), la ventana principal también pasa al primer plano; con ella desactivada, solo aparece la ventana Añadir nuevo torrent.

También puede iniciar Rookparcel con torrents en la línea de comandos: `rookparcel.exe file.torrent "magnet:?xt=..."`. Se añade cada ruta de `.torrent`, enlace `magnet:` y dirección `http(s)://`. Si Rookparcel ya se está ejecutando, van a la ventana en ejecución.

Si el Rookparcel en ejecución se inició con derechos de administrador, Windows no permite que una segunda copia le entregue un torrent. Rookparcel se lo dice entonces; añada el torrent desde la ventana en ejecución, o inicie Rookparcel sin derechos de administrador.

## La ventana Añadir nuevo torrent

![La ventana Añadir nuevo torrent](images/add-new-torrent.png)

Con **Mostrar una ventana con los archivos del torrent** activada (Preferencias > Interfaz, activada de forma predeterminada), esta ventana se abre con cada torrent que añade. Si está desactivada, los torrents se añaden de inmediato con la carpeta predeterminada y todos los archivos.

Para un enlace magnet, la ventana se abre cuando la lista de archivos del torrent ha llegado de otros pares. Hasta entonces, el torrent espera en la lista como «Descargando metadatos». No escribe nada en el disco mientras espera. Si cierra Rookparcel antes de que llegue la lista, la ventana vuelve en el siguiente inicio.

### Guardar en

La carpeta donde van los archivos del torrent. La lista contiene la carpeta de descargas predeterminada (de **Preferencias > Carpetas > Poner las nuevas descargas en:**) y las diez últimas carpetas que usó. Escriba una ruta o haga clic en **...** para elegir una. Una carpeta que todavía no existe se crea. El espacio libre de esa unidad se muestra después del tamaño, más abajo.

**Nombre:** el nombre de la carpeta del torrent, o de su único archivo. Al cambiarlo se renombra la carpeta o el archivo en el disco.

### Opciones del torrent

| Opción | Qué hace |
|---|---|
| **Omitir comprobación de hash** | Trata los archivos que ya están en la carpeta como completos, sin comprobarlos. Úsela solo cuando sepa que los datos están completos y correctos, por ejemplo después de copiarlos desde otro equipo. Aparece atenuada para un enlace magnet: sus archivos ya se comprobaron cuando llegó la lista de archivos. |
| **Iniciar el torrent** | Activada: el torrent entra en la cola y empieza según lo permita la cola. Desactivada: se añade detenido. Empieza marcada, salvo que **No iniciar la descarga automáticamente** esté activada en Preferencias > Interfaz. |
| **Añadir al principio de la cola** | Pone el torrent el primero de la cola, para que empiece antes que los demás. |
| **Etiqueta:** | Da una etiqueta al torrent. Elija una en uso o escriba una nueva. Consulte [Gestionar torrents](Gestionar-torrents#etiquetas). |
| **Descargar en orden secuencial** | Descarga las piezas en orden, desde el principio de los datos hasta el final, para que un vídeo pueda reproducirse mientras se descarga. Úsela solo para lo que quiera previsualizar: el orden habitual de lo más raro primero es mejor para los demás pares. |
| **Descargar primero las piezas primera y última** | Descarga las piezas primera y última de cada archivo deseado antes que el resto, para que un reproductor pueda leer pronto la cabecera y el índice de un vídeo. |

*Las dos casillas de orden de descarga son nuevas en la 1.2.63.* Ambas empiezan sin marcar para cada torrent; Rookparcel no las recuerda. Para cambiarlas después de añadir, use **Avanzado** en el menú contextual o en Propiedades. Consulte [Gestionar torrents](Gestionar-torrents#orden-de-descarga).

### Contenido del torrent

Las líneas de arriba muestran el **Nombre:**, el **Comentario:**, el **Tamaño:** y la **Fecha:** del torrent. El tamaño indica cuánto del torrent tiene seleccionado, por ejemplo «1.20 GB de 3.40 GB seleccionados», seguido del espacio libre en disco.

La lista de archivos muestra las carpetas y los archivos del torrent como un árbol, cada uno con una casilla, su tamaño y su prioridad.

- Desmarque un archivo para omitirlo. Una carpeta cuyos archivos están marcados solo en parte muestra «Mixta».
- **Seleccionar todo** y **Seleccionar nada** marcan o desmarcan todo.
- Haga clic en el icono de una carpeta, o haga doble clic en ella, para abrirla o cerrarla. <kbd>Right</kbd> y <kbd>Left</kbd> (o <kbd>+</kbd> y <kbd>-</kbd> en el teclado numérico) hacen lo mismo; <kbd>Space</kbd> marca o desmarca las filas seleccionadas.
- Haga clic derecho en las filas seleccionadas para **Prioridad alta**, **Prioridad normal**, **Prioridad baja**, **No descargar**, **Marcar** y **Desmarcar**. Al elegir una prioridad se marcan los archivos; **No descargar** los desmarca.

Los archivos cuyos nombres coinciden con **No descargar los archivos llamados:** (Preferencias > Interfaz) empiezan sin marcar aquí. Márquelos si los quiere después de todo: lo que elija en esta ventana siempre prevalece. Consulte [Archivos excluidos por nombre](#archivos-excluidos-por-nombre).

Puede cambiar la selección de archivos y las prioridades más adelante en la [pestaña Archivos](Detalles-y-archivos-del-torrent#archivos).

### Aceptar, Cancelar y No volver a mostrar esto

**Aceptar** añade el torrent con sus elecciones. **Cancelar** no añade nada; para un enlace magnet, quita el torrent que espera. Al marcar **No volver a mostrar esto** se desactiva **Mostrar una ventana con los archivos del torrent**, de modo que los torrents posteriores se añaden sin esta ventana. Vuelva a activarla en Preferencias > Interfaz.

Rookparcel recuerda el tamaño de la ventana.

## Añadir torrents detenidos

Para añadir torrents sin descargar nada hasta que usted lo diga, active **No iniciar la descarga automáticamente** en **Opciones > Preferencias > Interfaz**. Los torrents nuevos se añaden detenidos y fuera de la cola. Inícielos con **Iniciar** cuando esté listo. En la ventana Añadir nuevo torrent esto se ve como **Iniciar el torrent** sin marcar.

## Archivos excluidos por nombre

*Nuevo en la 1.2.63.* Algunos torrents llevan archivos que nunca quiere, como accesos directos, cachés de miniaturas o vídeos de muestra. Rookparcel puede excluirlos por nombre:

1. Abra **Opciones > Preferencias > Interfaz**.
2. En **Al añadir torrents**, marque **No descargar los archivos llamados:**.
3. En el cuadro de debajo, escriba los nombres, separados por punto y coma. `*` representa cualquier carácter y `?` uno solo, y no se distingue entre mayúsculas y minúsculas. Por ejemplo: `*.lnk; Thumbs.db; *sample*`.
4. Haga clic en **Aceptar**.

Solo se compara el nombre propio del archivo, no las carpetas en que está. La lista se aplica a los torrents que añada a partir de ahora:

- En la ventana Añadir nuevo torrent, los archivos coincidentes empiezan sin marcar, y puede volver a marcarlos.
- A los torrents añadidos sin la ventana se les pone **No descargar** en los archivos coincidentes: con la ventana desactivada, desde la [carpeta vigilada](#una-carpeta-vigilada), desde RSS, desde un enlace, desde la interfaz web y desde aplicaciones como Sonarr y Radarr. Para un enlace magnet, esto ocurre cuando ha llegado su lista de archivos.
- Un torrent cuyos archivos coinciden todos se añade con todos sus archivos, como si la lista estuviera vacía.
- Los torrents que ya están en la lista, los torrents recuperados al iniciar y los torrents importados no se modifican.

Un enlace magnet añadido sin la ventana cuya lista de archivos llega solo después de reiniciar Rookparcel descarga todos los archivos, y lo mismo ocurre con un torrent entregado a `rookparcel-nox` en su línea de comandos (`add`, o `run` con un archivo o enlace). Para obtener más adelante un archivo omitido, dele una prioridad en la [pestaña Archivos](Detalles-y-archivos-del-torrent#archivos).

## Un torrent que ya está en la lista

Si añade un torrent que ya está en la lista, Rookparcel dice «... ya está en la lista de torrents.» y no añade nada.

## Una carpeta vigilada

Rookparcel puede vigilar una carpeta y añadir cada archivo `.torrent` nuevo que se ponga en ella:

1. Abra **Opciones > Preferencias > Carpetas**.
2. Marque **Cargar automáticamente los .torrent de:** y elija la carpeta.
3. Opcionalmente, marque **Borrar los .torrent cargados**.
4. Haga clic en **Aceptar**.

Rookparcel añade un archivo nuevo unos segundos después de que deje de cambiar, sin la ventana Añadir nuevo torrent. Un archivo cargado se renombra a `.torrent.loaded`, o se borra cuando **Borrar los .torrent cargados** está activada. Los problemas se escriben en la [pestaña Registro](Detalles-y-archivos-del-torrent#registro).

La misma página puede guardar una copia de cada archivo `.torrent` añadido (**Guardar los .torrent en:**). Consulte [Disco y almacenamiento](Disco-y-almacenamiento) y la [Referencia de preferencias](Referencia-de-preferencias).

## En la versión GTK (Linux, BSD)

- Los mismos elementos de menú, atajos y ventana Añadir nuevo torrent.
- **Añadir torrent desde URL** rellena un enlace del portapapeles de la misma manera. Un texto que no es un enlace `magnet:` ni una dirección `http(s)://` da «No es un enlace que Rookparcel pueda añadir».
- La ventana GTK no acepta archivos soltados. Use **Archivo > Añadir torrent...**, o abra el archivo con Rookparcel desde su administrador de archivos.
- En Linux y BSD, las asociaciones de archivos están en **Integración con el escritorio** de Preferencias > General. En un Flatpak, elija en cambio el programa predeterminado en la configuración de su escritorio. Consulte [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio).

## En la interfaz web

- **Añadir torrent** sube uno o más archivos `.torrent` desde el equipo en el que se ejecuta el navegador; **Añadir URL** pide un enlace magnet o una dirección. Un hash de información suelto también se toma como un enlace magnet.
- Puede soltar archivos `.torrent` o un enlace magnet sobre la página, o pegar un enlace magnet en cualquier lugar fuera de un cuadro de texto.
- La misma ventana Añadir nuevo torrent se abre en la página. **Guardar en** es una carpeta del servidor, escrita como una ruta.
- En un teléfono, **Añadir URL** está en el menú **Más** (⋯).

Consulte [Interfaz web](Interfaz-web). En una ventana conectada con **Archivo > Conectar en remoto...**, el botón **...** de Añadir nuevo torrent aparece atenuado: escriba una carpeta del servidor. Consulte [Control remoto](Control-remoto).

## Páginas relacionadas

- [La ventana principal](La-ventana-principal)
- [Gestionar torrents](Gestionar-torrents)
- [Importar desde µTorrent y qBittorrent](Importar-desde-uTorrent-y-qBittorrent)
- [Descargador RSS](Descargador-RSS)
