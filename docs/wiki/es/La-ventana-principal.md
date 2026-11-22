*Esta página es una traducción de la página en inglés [The main window](The-main-window). Si hay diferencias, la página en inglés es la válida.*

# La ventana principal

Esta página recorre la ventana principal de Rookparcel: los menús, la barra de herramientas y su cuadro de búsqueda, la lista de categorías, la lista de torrents y sus columnas, las pestañas de detalles y la barra de estado. Describe primero la versión de Windows y después qué cambia en la versión GTK y en la interfaz web.

![La ventana principal con torrents y la pestaña General](images/main-window.png)

## Las partes de un vistazo

| Parte | Dónde | Para qué sirve |
|---|---|---|
| Barra de menús | arriba | **Archivo**, **Opciones** y **Ayuda** |
| Barra de herramientas | bajo los menús | botones para las acciones habituales, y el cuadro de búsqueda a la derecha |
| Lista de categorías | izquierda | muestra los torrents por estado, etiqueta o marca, y sus canales RSS |
| Lista de torrents | centro | una fila por torrent |
| Pestañas de detalles | bajo la lista | los detalles del torrent seleccionado: General, Trackers, Pares, Piezas, Archivos, Velocidad, Registro |
| Barra de estado | abajo | el estado de la red, DHT y las velocidades totales |

Cada parte, salvo los menús y la lista de torrents, puede ocultarse desde el menú **Opciones**: **Mostrar barra de herramientas**, **Mostrar detalles**, **Mostrar barra de estado** y **Mostrar lista de categorías**. Arrastre los bordes entre la lista de categorías, la lista de torrents y las pestañas de detalles para cambiar su tamaño. Rookparcel recuerda estas elecciones, los tamaños, las columnas, el orden y la pestaña abierta.

## Los menús

- **Archivo:** **Añadir torrent...**, **Añadir torrent desde URL...**, **Crear nuevo torrent...**, **Importar de qBittorrent...**, **Importar de µTorrent...**, **Perfiles**, **Conectar en remoto...**, **Bloquear Rookparcel** (<kbd>Ctrl</kbd>+<kbd>L</kbd>; consulte [Bloquear Rookparcel](Bloquear-Rookparcel)) y **Salir**.
- **Opciones:** **Preferencias**, **Guía de configuración**, **Descargador RSS...**, los cuatro elementos **Mostrar** de arriba, **Apagado automático**, **Cerrar a la bandeja** y **Minimizar a la bandeja**.
- **Ayuda:** **Sitio web de Rookparcel**, **Buscar actualizaciones**, **Estadísticas**, **Donar**, **Acerca de Rookparcel** y **Avisos de terceros**. **Estadísticas** (nuevo en la 1.2.39) muestra lo que Rookparcel subió y descargó en todas sus sesiones y en esta, la proporción de compartición, los datos descartados, los pares conectados y la cola de disco y la caché de lectura; los totales de todas las sesiones se conservan cuando se quitan torrents. **Donar** (nuevo en la 1.2.34) muestra una nota breve y un botón que abre en su navegador la página de donaciones del sitio web.

*Archivo > Perfiles es nuevo en la 1.1.48 y Opciones > Guía de configuración en la 1.1.62.*

Páginas: [Bloquear Rookparcel](Bloquear-Rookparcel), [Añadir torrents](Añadir-torrents), [Crear torrents](Crear-torrents), [Importar desde µTorrent y qBittorrent](Importar-desde-uTorrent-y-qBittorrent), [Perfiles](Perfiles), [Control remoto](Control-remoto), [Referencia de preferencias](Referencia-de-preferencias), [Primer inicio y la Guía de configuración](Primer-inicio), [Descargador RSS](Descargador-RSS), [Apagado automático](Apagado-automático), [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio), [Actualizar Rookparcel](Actualizar-Rookparcel).

## La barra de herramientas

De izquierda a derecha:

| Botón | Qué hace |
|---|---|
| **Añadir torrent** | añade archivos `.torrent` (<kbd>Ctrl</kbd>+<kbd>O</kbd>) |
| **Añadir torrent desde URL** | añade un enlace magnet o una dirección web (<kbd>Ctrl</kbd>+<kbd>U</kbd>) |
| **Crear nuevo torrent** | crea un `.torrent` nuevo (<kbd>Ctrl</kbd>+<kbd>N</kbd>) |
| **Quitar** | quita de la lista los torrents seleccionados (<kbd>Del</kbd>) |
| **Iniciar**, **Pausa**, **Detener** | inician, pausan o detienen los torrents seleccionados |
| **Subir en la cola**, **Bajar en la cola** | mueven los torrents seleccionados un lugar en la cola |
| **Descargador RSS** | abre el Descargador RSS |
| **Preferencias** | abre Preferencias (<kbd>Ctrl</kbd>+<kbd>P</kbd>) |

Deje el puntero sobre un botón para ver su nombre. Desde la 1.1.57, la barra de estado también muestra una línea sobre lo que hace el botón bajo el puntero o el elemento de menú resaltado. Lo que significan Iniciar, Pausa y Detener está en [Gestionar torrents](Gestionar-torrents#iniciar-pausa-detener-y-forzar-inicio).

### El cuadro de búsqueda

El cuadro del extremo derecho de la barra de herramientas hace dos trabajos, que se eligen con la pequeña flecha que tiene al lado:

- **Filtrar torrents** (el modo habitual): mientras escribe, la lista muestra solo los torrents cuyo nombre contiene el texto, sin distinguir mayúsculas de minúsculas. El filtro actúa sobre la categoría que haya elegido a la izquierda. <kbd>Esc</kbd> vacía el cuadro y vuelve a la lista. <kbd>Ctrl</kbd>+<kbd>F</kbd> pone el cursor en el cuadro.
- **Una fuente de búsqueda**: <kbd>Enter</kbd> busca torrents en la web mediante Prowlarr, Jackett o el asistente de búsqueda. Consulte [Buscar torrents](Buscar-torrents).

La elección se recuerda entre inicios.

## La lista de categorías

La lista de la izquierda acota la lista de torrents. Cada entrada muestra cuántos torrents contiene.

| Entrada | Muestra |
|---|---|
| **Torrents** | todos los torrents |
| **Descargando** | torrents que todavía no han terminado |
| **Completados** | torrents terminados |
| **Activos** | torrents que ahora mismo descargan o suben datos |
| **Inactivos** | torrents que ahora mismo no mueven datos |
| **Etiquetas** > **Sin etiqueta** | torrents sin etiqueta |
| **Etiquetas** > *una etiqueta* | torrents con esa etiqueta; una entrada por cada etiqueta en uso |
| **Marcas** > **Sin marca** | torrents sin ninguna marca |
| **Marcas** > *una marca* | torrents con esa marca; una entrada por cada marca, también para una marca que ningún torrent tiene (0) |
| **Canales** | sus canales RSS; al elegir uno se muestran sus elementos en la lista |

Las etiquetas se crean y se asignan desde el menú contextual de la lista de torrents. Consulte [Gestionar torrents](Gestionar-torrents#etiquetas). Las marcas se asignan de la misma manera; haga clic derecho en la rama **Marcas** para **Nueva marca...**, o en una marca para **Borrar marca**. Consulte [Gestionar torrents](Gestionar-torrents#marcas) (*las marcas son nuevas en la 1.2.63*). La rama Canales tiene su propio menú contextual; consulte [Descargador RSS](Descargador-RSS).

## La lista de torrents

Una fila por torrent. Haga clic en una fila para seleccionarla; <kbd>Ctrl</kbd>+clic y <kbd>Shift</kbd>+clic seleccionan varias; <kbd>Ctrl</kbd>+<kbd>A</kbd> selecciona todas. Haga clic derecho para el menú del torrent, descrito en [Gestionar torrents](Gestionar-torrents#el-menú-contextual).

Haga doble clic en un torrent terminado para abrir su carpeta en el Explorador. Haga doble clic en un torrent que no ha terminado, o pulse <kbd>Enter</kbd>, para abrir sus [Propiedades](Detalles-y-archivos-del-torrent#propiedades-del-torrent).

### Columnas

Haga clic derecho en los encabezados de columna para mostrar u ocultar columnas. **Nombre** se muestra siempre. Arrastre un encabezado para mover la columna; arrastre su borde para cambiar el ancho.

| Columna | Muestra | Visible al principio |
|---|---|---|
| **Nombre** | el nombre del torrent | sí |
| **#** | su lugar en la cola; `*` cuando no tiene | sí |
| **Tamaño** | el tamaño de los archivos que eligió descargar | sí |
| **Hecho** | cuánto está descargado, en porcentaje | sí |
| **Estado** | qué está haciendo el torrent (consulte más abajo) | sí |
| **Semillas** | semillas conectadas y, entre paréntesis, las semillas de todo el enjambre cuando el tracker lo informa | sí |
| **Pares** | pares conectados y, entre paréntesis, los pares del enjambre | sí |
| **Vel. bajada**, **Vel. subida** | velocidades actuales | sí |
| **Tiempo restante** | tiempo restante estimado | sí |
| **Subido** | total subido | sí |
| **Descargado** | total descargado | no |
| **Proporción** | lo subido dividido por lo descargado | sí |
| **Disp.** | cuántas copias completas tienen juntos los pares conectados | sí |
| **Etiqueta** | su etiqueta | sí |
| **Añadido el**, **Completado el** | fechas | no |
| **Tracker** | el nombre de host del tracker en uso | no |
| **Límite bajada**, **Límite subida** | sus propios límites de velocidad | no |
| **Restante** | lo que queda por descargar | no |
| **Semillas/Pares** | semillas por par en el enjambre | no |
| **Guardar como** | la carpeta donde están sus archivos | no |
| **Desperdiciado** | datos descartados (comprobaciones fallidas, duplicados) | no |
| **Marcas** | sus marcas, separadas por comas (*nuevo en la 1.2.63*) | no |

### Ordenar

Haga clic en el encabezado de una columna para ordenar por ella. El primer clic ordena de la forma que suele ser más útil: las velocidades, semillas, pares, proporción y hecho van de mayor a menor, las demás de menor a mayor. Haga clic de nuevo para invertir. Los nombres se ordenan como lo hace el Explorador, así que «Episode 2» va antes que «Episode 10».

### Qué dice la columna Estado

| Estado | Significado |
|---|---|
| **Descargando**, **Sembrando** | en marcha |
| **En cola**, **Semilla en cola** | iniciado, pero esperando un lugar libre en la cola |
| **[F] Descargando**, **[F] Sembrando** | iniciado con **Forzar inicio**, fuera de la cola |
| **[S] Sembrando** | sembrando con **Siembra inicial** activada |
| **En pausa** | en pausa por usted |
| **Detenido** | detenido por usted antes de terminar |
| **Terminado** | detenido después de terminar |
| **Descargando metadatos** | un enlace magnet que obtiene la lista de archivos del torrent |
| **Comprobando 45.3 %**, **En cola para comprobar** | comprobando los archivos en el disco. **En cola para comprobar** significa que la comprobación espera a su unidad, que comprueba un torrent a la vez. *En cola para comprobar es nuevo en la 1.2.76.* |
| **Moviendo** | moviendo sus archivos a otra carpeta |
| **Esperando la red** | **Usar solo esta red** no encuentra ningún adaptador de red que coincida (la VPN está caída); sigue por sí solo cuando vuelve. *Nuevo en la 1.1.58.* |
| **Error: ...** | algo salió mal; el mensaje dice qué. Cuando falta una unidad, un recurso compartido o una carpeta con los archivos del torrent, el mensaje dice «files missing» y el torrent espera hasta que los archivos vuelven (*nuevo en la 1.2.76*; consulte [Disco y almacenamiento](Disco-y-almacenamiento#cuando-falta-una-unidad-o-un-recurso-compartido)). |

Consulte [Gestionar torrents](Gestionar-torrents) para las acciones detrás de estos estados.

## Las pestañas de detalles

Las pestañas bajo la lista muestran el torrent seleccionado: **General**, **Trackers**, **Pares**, **Piezas** y **Archivos**. **Velocidad** y **Registro** son para todo el programa. Cada pestaña se describe en [Detalles y archivos del torrent](Detalles-y-archivos-del-torrent).

## La barra de estado

De izquierda a derecha:

| Parte | Muestra |
|---|---|
| Aviso de actualización | «Rookparcel ... está disponible - haga clic aquí para descargarlo», solo cuando hay una versión más reciente. Consulte [Actualizar Rookparcel](Actualizar-Rookparcel). |
| Red | **Red OK** cuando otros pares pueden conectarse con usted, **Sin conexiones entrantes** cuando todavía ninguno lo ha hecho. **Puerto ... en uso** significa que otro programa, a menudo otro perfil de Rookparcel, tiene el puerto de escucha; haga clic en él para abrir Preferencias > Conexión. El estado del programador sigue cuando limita la velocidad, por ejemplo «Programador: limitado». **Restaurando los torrents...** aparece al principio hasta que los torrents guardados vuelven a la lista. **Transferencias retenidas: confirme las preferencias** significa que Rookparcel no pudo fiarse de los ajustes que leyó (un ajuste de proxy, de regla de red o de interfaz de escucha después de una actualización, o un archivo de ajustes que no pudo leer), así que no se conecta nada; revise **Opciones > Preferencias** y haga clic en **Aceptar**. Consulte [Actualizar Rookparcel](Actualizar-Rookparcel). |
| **DHT: ... nodos** | cuántos nodos DHT conoce Rookparcel |
| **B: ... T: ...** | velocidad de bajada y total descargado en esta sesión |
| **S: ... T: ...** | velocidad de subida y total subido en esta sesión |

Con **Mantener el ping bajo** reteniendo las velocidades, las partes de velocidad añaden el límite automático, por ejemplo «B: 400 kB/s (auto 450 kB/s) T: 2.10 GB».

*Nuevo en la 1.1.58:* mientras **Usar solo esta red** no encuentra ningún adaptador que coincida, la parte de red dice «Esperando la red», y al dejar el puntero sobre ella se muestra su dirección tal como la ven los trackers. El texto **Puerto ... en uso** llegó con los perfiles en la 1.1.45.

Consulte [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos), [Límites de velocidad y programador](Límites-de-velocidad-y-programador) y [Mantener el ping bajo](Mantener-el-ping-bajo).

## La barra de título y la bandeja

Con **Mostrar la velocidad actual en la barra de título** activada (Preferencias > Interfaz), el título de la ventana empieza con las velocidades de bajada y de subida, que también se muestran en la barra de tareas.

Rookparcel pone un icono en el área de notificación. Al dejar el puntero sobre él se muestran la versión y las velocidades. Un clic muestra u oculta la ventana; un clic derecho abre un menú con **Ocultar Rookparcel** o **Mostrar Rookparcel**, **Añadir torrent...**, **Añadir torrent desde URL...**, **Iniciar todos los torrents**, **Detener todos los torrents**, **Bloquear Rookparcel** (solo cuando hay un PIN establecido) y **Salir**. De forma predeterminada, el botón de cierre oculta la ventana en la bandeja y los torrents siguen en marcha; **Salir** cierra el programa. Consulte [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio).

## En la versión GTK (Linux, BSD)

- La misma disposición: barra de herramientas, lista de categorías, lista de torrents, pestañas de detalles y barra de estado. La barra de herramientas no tiene el botón **Preferencias**; use **Opciones > Preferencias** o <kbd>Ctrl</kbd>+<kbd>P</kbd>.
- El menú **Opciones** tiene **Preferencias**, **Guía de configuración**, **Descargador RSS...** y **Apagado automático**. No tiene elementos **Mostrar** ni elementos de la bandeja. **Archivo** tiene **Bloquear Rookparcel** como en Windows.
- **Archivo > Salir** es también <kbd>Ctrl</kbd>+<kbd>Q</kbd>.
- Al hacer clic derecho en el encabezado de una columna se muestran u ocultan columnas; los encabezados se pueden arrastrar para reordenarlos.
- El doble clic funciona como en Windows: un torrent terminado abre su carpeta, otro abre Propiedades.

## En la interfaz web

- La página tiene una barra de herramientas (**Añadir torrent**, **Añadir URL**, **Quitar**, **Iniciar**, **Pausa**, **Detener**, **Subir**, **Bajar**, **RSS**, **Preferencias** y **Buscar**), la lista de categorías, la lista de torrents y las mismas siete pestañas de detalles, además de un cuadro de búsqueda que filtra la lista por nombre.
- Haga clic en un encabezado para ordenar, arrástrelo para reordenar, arrastre su borde para cambiar el tamaño. Haga clic derecho en los encabezados para mostrar u ocultar columnas; **Restablecer columnas** vuelve a las predeterminadas.
- En un teléfono, la lista de categorías se abre desde el botón ☰, y el botón ⋯ tiene las acciones que no caben, **Ordenar por** y **Orden descendente**.

Consulte [Interfaz web](Interfaz-web).

## Páginas relacionadas

- [Atajos de teclado](Atajos-de-teclado)
- [Gestionar torrents](Gestionar-torrents)
- [Detalles y archivos del torrent](Detalles-y-archivos-del-torrent)
