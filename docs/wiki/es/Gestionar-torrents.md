*Esta es una traducción de la página inglesa [Managing torrents](https://github.com/Balls0fSteel/Rookparcel/wiki/Managing-torrents). Si hay diferencias, prevalece la página inglesa.*

# Gestionar torrents

Esta página explica qué se puede hacer con los torrents una vez que están en la lista: iniciarlos y detenerlos, la cola, quitarlos con sus archivos o sin ellos, comprobarlos de nuevo, mover sus archivos, etiquetas y marcas, límites de velocidad por torrent y el menú contextual que reúne todo eso.

Cada acción se aplica a los torrents seleccionados. Seleccione varios con <kbd>Ctrl</kbd>+clic o <kbd>Shift</kbd>+clic, o todos con <kbd>Ctrl</kbd>+<kbd>A</kbd>, y actúe sobre ellos de una vez.

## El menú contextual

![El menú contextual de la lista de torrents](images/torrent-menu.png)

Haga clic con el botón derecho en un torrent (o pulse <kbd>Shift</kbd>+<kbd>F10</kbd>) para abrir este menú:

| Elemento | Qué hace | Tecla |
|---|---|---|
| **Abrir carpeta contenedora** | abre la carpeta del torrent en el Explorador, con sus datos seleccionados | |
| **Copiar enlace magnet** | copia el enlace magnet del torrent al portapapeles | |
| **Exportar .torrent...** | guarda el archivo `.torrent` del torrent | |
| **Forzar inicio** | lo inicia ya, fuera de la cola | |
| **Iniciar** | lo entrega a la cola | <kbd>Ctrl</kbd>+<kbd>S</kbd> |
| **Pausa** | lo detiene y lo saca de la cola; muestra **En pausa** | <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd> |
| **Detener** | lo detiene y lo saca de la cola; muestra **Detenido** o **Terminado** | <kbd>Ctrl</kbd>+<kbd>T</kbd> |
| **Subir en la cola**, **Bajar en la cola** | un puesto arriba o abajo | |
| **Mover al principio de la cola**, **Mover al final de la cola** | al principio o al final, conservando su orden | |
| **Quitar** | lo quita de la lista; los archivos se conservan | <kbd>Del</kbd> |
| **Quitar y > Borrar datos** | lo quita y borra sus archivos | <kbd>Shift</kbd>+<kbd>Del</kbd> |
| **Forzar comprobación** | vuelve a comprobar los archivos en el disco | |
| **Actualizar tracker** | contacta con los trackers ahora | |
| **Avanzado > Establecer ubicación de descarga...** | mueve sus archivos a otra carpeta | |
| **Avanzado > Siembra inicial** | activa o desactiva la siembra inicial | |
| **Avanzado > Descargar en orden secuencial** | descarga las piezas en orden, del principio al final | |
| **Avanzado > Descargar primero las piezas primera y última** | descarga primero los dos extremos de cada archivo deseado | |
| **Asignación de ancho de banda > Establecer límite de bajada / Establecer límite de subida** | sus propios límites de velocidad | |
| **Etiqueta** | le asigna una etiqueta, una nueva o ninguna | |
| **Marcas** | le asigna marcas o se las quita | |
| **Propiedades** | trackers, límites, objetivo de siembra y más | <kbd>Enter</kbd> |

Las secciones siguientes explican cada grupo. La mayoría de estos elementos también están en la barra de herramientas; consulte [La ventana principal](La-ventana-principal#la-barra-de-herramientas).

## Iniciar, Pausa, Detener y Forzar inicio

Rookparcel mantiene una cola, como µTorrent. El número de torrents que pueden funcionar a la vez se define en **Opciones > Preferencias > Cola**; consulte [Cola y siembra](Cola-y-siembra).

- **Iniciar** entrega el torrent a la cola. Si hay sitio, se ejecuta (**Descargando** o **Sembrando**). Si no, espera como **En cola** o **Semilla en cola** hasta que quede un puesto libre.
- **Forzar inicio** ejecuta el torrent de inmediato, aunque la cola esté llena. Su estado empieza por **[F]**, por ejemplo **[F] Sembrando**. Un torrent forzado nunca alcanza un objetivo de siembra, así que siembra hasta que usted lo detenga.
- **Pausa** y **Detener** detienen el torrent y lo sacan de la cola. La única diferencia es la palabra de la columna Estado: **En pausa**, o **Detenido** (**Terminado** si el torrent está completo). Ambos estados se recuerdan tras reiniciar. Use Iniciar o Forzar inicio para continuar.

El menú del icono de la bandeja tiene **Iniciar todos los torrents** y **Detener todos los torrents**, que actúan sobre todos los torrents de la lista.

Para añadir torrents nuevos en estado detenido, consulte [Añadir torrents](Añadir-torrents#añadir-torrents-detenidos).

## La cola

La columna **#** muestra el puesto de cada torrent en la cola. Los números más bajos tienen el turno primero. Use **Subir en la cola** y **Bajar en la cola** (también en la barra de herramientas), o **Mover al principio de la cola** y **Mover al final de la cola**, para cambiar el orden. Al mover varios torrents, se conserva su orden relativo.

En la ventana Añadir nuevo torrent, **Añadir al principio de la cola** coloca un torrent nuevo el primero.

## Quitar torrents

- **Quitar** (<kbd>Del</kbd>) saca los torrents de la lista. Los datos descargados se conservan en el disco.
- **Quitar y > Borrar datos** (<kbd>Shift</kbd>+<kbd>Del</kbd>) los saca y borra sus archivos descargados. En Windows los archivos no pasan por la Papelera de reciclaje y no se pueden recuperar desde ella. Se conserva un archivo parcial terminado en `.!ut` o `.!qB` del que Rookparcel no pueda demostrar que contiene los datos de este torrent; consulte [Disco y almacenamiento](Disco-y-almacenamiento#archivos-incompletos-y-ut).

Con **Confirmar al borrar torrents** activado (Preferencias > Interfaz, activado de forma predeterminada), Rookparcel pregunta antes e indica si también se borran los archivos. Si se desactiva, Quitar actúa de inmediato.

## Forzar comprobación

**Forzar comprobación** lee los archivos del torrent en el disco pieza por pieza y cuenta solo las piezas que están completas y correctas. El estado muestra **En cola para comprobar** mientras la comprobación espera a su unidad (las comprobaciones se hacen de un torrent por unidad; *nuevo en 1.2.76*), y después **Comprobando** con un porcentaje. Úsela después de mover o restaurar archivos a mano, o si sospecha que los datos están dañados.

*Nuevo en 1.2.76:* un torrent detenido se inicia con la comprobación; vuelva a detenerlo después si lo desea detenido. Un torrent en cola se comprueba de inmediato, sin esperar un puesto en la cola, y después vuelve a la cola. Un torrent retenido con **archivos ausentes** (consulte [Disco y almacenamiento](Disco-y-almacenamiento#cuando-falta-una-unidad-o-un-recurso-compartido)) se comprueba con lo que hay ahora, y lo que falte se descarga de nuevo.

Lo suave que es el uso del disco en las comprobaciones se define en **Preferencias > Juegos** (desactivado al principio); consulte [Disco y almacenamiento](Disco-y-almacenamiento).

## Actualizar tracker

**Actualizar tracker** contacta con los trackers del torrent ahora, sin esperar al siguiente anuncio, para informar del progreso y obtener más pares. La [pestaña Trackers](Detalles-y-archivos-del-torrent#trackers) muestra la respuesta.

## Mover los archivos: Establecer ubicación de descarga

1. Seleccione los torrents.
2. Elija **Avanzado > Establecer ubicación de descarga...**.
3. Seleccione la nueva carpeta.

Rookparcel mueve los archivos allí y sigue sembrando o descargando desde el nuevo lugar. Mientras tanto, el estado muestra **Moviendo**, y el elemento aparece atenuado para un torrent que ya se está moviendo. Los archivos del mismo nombre que ya estén en la nueva carpeta se reemplazan.

Si sale mientras se mueven archivos, Rookparcel espera a que terminen los movimientos y muestra su progreso. **Salir ahora** detiene un movimiento: sus archivos se quedan en la carpeta antigua, y lo que ya se había copiado se borra. Vuelva a moverlo más tarde.

Para mover automáticamente las descargas terminadas, consulte **Mover las descargas completadas a:** en [Disco y almacenamiento](Disco-y-almacenamiento).

## Abrir la carpeta

**Abrir carpeta contenedora** abre la carpeta del torrent en el Explorador, con su archivo o carpeta seleccionado una vez que existe. Con varios torrents seleccionados, abre la del primero. Hacer doble clic en un torrent terminado hace lo mismo.

Para abrir un solo archivo, use la [pestaña Archivos](Detalles-y-archivos-del-torrent#archivos).

## Copiar enlace magnet y Exportar .torrent

*Nuevo en 1.2.63.*

**Copiar enlace magnet** copia al portapapeles los enlaces magnet de los torrents seleccionados, uno por línea. Cada enlace contiene el info-hash del torrent, su nombre y sus trackers. Péguelo en otro programa de torrents, o envíeselo a alguien, para compartir el torrent sin un archivo.

Como el enlace incluye los trackers, no comparta un enlace copiado de un torrent de un tracker privado. La dirección de un tracker privado suele contener su clave personal (un passkey), y cualquiera que tenga el enlace podría anunciarse como usted.

**Exportar .torrent...** guarda los archivos `.torrent` de los torrents seleccionados. El archivo contiene el torrent tal como lo recibió Rookparcel, con los trackers y las semillas web que usa ahora. Puede añadirlo en otro equipo o conservarlo como copia.

- Con un torrent seleccionado, Rookparcel pide un nombre de archivo.
- Con varios, pide una carpeta y guarda cada uno como `<nombre>.torrent`. Si el nombre ya existe, añade un número, como ` (2)`, en lugar de reemplazar el archivo.
- Un torrent añadido desde un enlace magnet no tiene archivo `.torrent` hasta que llega su lista de archivos («Descargando metadatos»). El elemento aparece atenuado si ningún torrent seleccionado lo tiene.

Si no se puede escribir el archivo que indicó, Rookparcel muestra «No se pudo exportar» con el nombre del torrent y el motivo. Los archivos que fallan en una exportación a una carpeta se escriben en su lugar en la [pestaña Registro](Detalles-y-archivos-del-torrent#registro).

## Etiquetas

Una etiqueta es un nombre que se da a los torrents para agruparlos, como «Películas» o «Linux». Cada etiqueta en uso aparece en **Etiquetas** en la lista de categorías, con un contador, de modo que un clic muestra solo esos torrents.

- **Etiqueta > Nueva etiqueta...** pide un nombre y se lo asigna a los torrents seleccionados.
- **Etiqueta >** *una etiqueta* asigna una etiqueta existente.
- **Etiqueta > Quitar etiqueta** quita la etiqueta.

El menú marca la etiqueta que comparten los torrents seleccionados. También puede asignar una etiqueta en la ventana Añadir nuevo torrent, y los filtros RSS pueden asignar una.

Con **Añadir la etiqueta del torrent** activado junto a **Mover las descargas completadas a:** (Preferencias > Carpetas), un torrent terminado y con etiqueta se mueve a una carpeta con el nombre de su etiqueta. Consulte [Disco y almacenamiento](Disco-y-almacenamiento).

## Marcas

*Nuevo en 1.2.63.*

Un torrent tiene una sola etiqueta, pero además puede tener cualquier número de marcas (tags), como «4K», «conservar» y «ISO de Linux». Las marcas funcionan como las de qBittorrent. Cada marca aparece en **Marcas** en la lista de categorías, debajo de las etiquetas, con un contador; un torrent con varias marcas se cuenta en cada una. **Sin marca** muestra los torrents que no tienen ninguna marca.

- **Marcas > Nueva marca...** pide un nombre y asigna la nueva marca a los torrents seleccionados.
- **Marcas >** *una marca* asigna esa marca a los torrents seleccionados. Una marca que tienen todos los torrents seleccionados aparece marcada con una señal; al elegirla se quita de todos. Una marca que solo tienen algunos de ellos no aparece marcada (la aplicación de macOS muestra una señal mixta), y al elegirla se asigna a todos.
- **Marcas > Quitar todas las marcas** quita todas las marcas de los torrents seleccionados. Las marcas en sí se conservan.

Una marca permanece en la lista de categorías cuando ya no la tiene ningún torrent, con un contador de 0. Para eliminarla, haga clic con el botón derecho en ella en la lista de categorías y elija **Borrar marca**: Rookparcel pregunta «¿Borrar la marca "*nombre*"? Todos los torrents que la tienen la perderán.» Los torrents en sí se conservan. **Nueva marca...** en ese menú crea una marca sin asignarla a ningún torrent.

Un nombre no puede estar vacío ni contener una coma, un tabulador o un salto de línea. Se eliminan los espacios en ambos extremos, y un nombre de más de 128 bytes se recorta. Los nombres distinguen mayúsculas y minúsculas: `tv` y `TV` son dos marcas. Puede haber como máximo 1024 marcas.

La columna **Marcas** muestra las marcas de un torrent; al principio está oculta (consulte [La ventana principal](La-ventana-principal#columnas)). El cuadro de búsqueda solo busca en los nombres de los torrents, no en las marcas; en su lugar, haga clic en una marca de la lista de categorías. La ventana Añadir nuevo torrent no tiene marcas: asígnelas desde el menú contextual una vez que el torrent esté en la lista. Las aplicaciones que usan la API de qBittorrent, como Sonarr, también pueden asignar marcas ([Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr#categorías-etiquetas-y-marcas)).

## Límites de velocidad para un torrent

**Asignación de ancho de banda > Establecer límite de bajada** y **Establecer límite de subida** ofrecen **Ilimitado**, 10, 25, 50, 100, 250, 500, 1000, 2500 y 5000 kB/s. La opción actual aparece marcada. Para cualquier otro número, use [Propiedades](Detalles-y-archivos-del-torrent#propiedades-del-torrent). Los límites globales de Preferencias se siguen aplicando además. Consulte [Límites de velocidad y programador](Límites-de-velocidad-y-programador).

## Siembra inicial

**Avanzado > Siembra inicial** es para un torrent que usted es el primero en compartir. Cada par recibe piezas que nadie más tiene todavía, de modo que un torrent nuevo se difunde más rápido. Solo se puede activar cuando todos los torrents seleccionados están sembrando. El estado muestra **[S] Sembrando** mientras está activada. Vuelva a elegirla para desactivarla.

## Orden de descarga

*Nuevo en 1.2.63.*

De forma predeterminada, Rookparcel descarga primero las piezas más raras entre los pares. Así todas las piezas siguen disponibles en el enjambre, y es lo mejor para todos los que comparten el torrent. Dos elementos de **Avanzado** cambian el orden, sobre todo para que pueda ver un vídeo mientras se descarga. Cada uno aparece marcado mientras está activado; vuelva a elegirlo para desactivarlo.

- **Descargar en orden secuencial** descarga las piezas en orden, desde el principio de los datos hasta el final, para que un reproductor pueda reproducir el vídeo a medida que llega. Los archivos con prioridad más alta siguen yendo primero. Perjudica al enjambre: las piezas raras que habría compartido llegan tarde. Úselo con un torrent que quiera previsualizar, y déjelo desactivado en los demás casos.
- **Descargar primero las piezas primera y última** descarga las piezas primera y última de cada archivo deseado antes que el resto: el 1 % del archivo en cada extremo, como mínimo una pieza. Muchos formatos de vídeo guardan ahí su cabecera y su índice, de modo que un reproductor puede abrir el archivo sin terminar. Los archivos omitidos no se tocan.

Ambos aparecen atenuados cuando todos los torrents seleccionados están terminados. Cada torrent conserva su elección tras reiniciar. También puede definirlos al añadir un torrent, en la [ventana Añadir nuevo torrent](Añadir-torrents#opciones-del-torrent), y en [Propiedades](Detalles-y-archivos-del-torrent#propiedades-del-torrent).

## Propiedades

**Propiedades** (<kbd>Enter</kbd>, o doble clic en un torrent que no está terminado) abre la ventana Propiedades del torrent: trackers, límites de velocidad, ranuras de subida, el objetivo de siembra, DHT e Intercambio de pares, y el orden de descarga de este torrent. Consulte [Detalles y archivos del torrent](Detalles-y-archivos-del-torrent#propiedades-del-torrent).

## En la versión GTK (Linux, BSD)

- El mismo menú contextual, en el mismo orden, salvo que **Copiar enlace magnet** y **Exportar .torrent...** tienen su propio grupo justo antes de **Avanzado**. La aplicación de macOS también los coloca ahí.
- Atajos: <kbd>Del</kbd> quita, <kbd>Shift</kbd>+<kbd>Del</kbd> quita y borra los archivos, <kbd>Ctrl</kbd>+<kbd>A</kbd> selecciona todo. Iniciar, Pausa y Detener no tienen atajo.
- **Abrir carpeta contenedora** abre la carpeta en su administrador de archivos.
- En Linux y BSD, los archivos borrados se eliminan del disco, no se mueven a la papelera.

## En la interfaz web

- El mismo menú contextual, salvo que **Abrir carpeta contenedora** aparece atenuado («no desde un navegador»), y **Copiar enlace magnet** y **Exportar .torrent...** están justo antes de **Avanzado**. **Marcas** muestra primero **Nueva marca...** y **Quitar todas las marcas**, y después las marcas.
- **Exportar .torrent...** descarga el archivo a través del navegador, en su carpeta de descargas. Con varios torrents, el navegador descarga cada archivo por turno, como máximo 50 por clic.
- **Copiar enlace magnet** necesita el acceso al portapapeles del navegador. Una página abierta mediante `http://` simple desde otro equipo no lo tiene, así que la página muestra los enlaces, bajo «Copie el enlace siguiente:», para que los copie a mano.
- **Establecer ubicación de descarga** pide una carpeta en el servidor, escrita como una ruta.
- Los archivos borrados se eliminan del disco, no se mueven a la papelera.
- En un teléfono, el menú omite **Abrir carpeta contenedora** y empieza por **Detalles**.

## En una ventana remota

En una ventana conectada con **Archivo > Conectar en remoto...**, los archivos están en el otro equipo. **Abrir carpeta contenedora** aparece atenuado, y **Establecer ubicación de descarga...** pide una carpeta en el servidor. Consulte [Control remoto](Control-remoto).

## Páginas relacionadas

- [La ventana principal](La-ventana-principal)
- [Detalles y archivos del torrent](Detalles-y-archivos-del-torrent)
- [Cola y siembra](Cola-y-siembra)
- [Atajos de teclado](Atajos-de-teclado)
