*Esta página es una traducción de la página inglesa [Disk and storage](Disk-and-storage). Si hay diferencias, prevalece la página en inglés.*

# Disco y almacenamiento

Esta página explica dónde pone Rookparcel las descargas y cómo las mueve, y cómo trata sus discos: la E/S de disco inteligente (comprobaciones suaves, ajuste según el tipo de unidad y ralentización mientras un disco está ocupado) y la caché de lectura del disco.

## Dónde van las descargas

En **Opciones > Preferencias > Carpetas**, en **Ubicación de los archivos descargados**:

| Ajuste | Significado |
|---|---|
| **Poner las nuevas descargas en:** | La carpeta donde se guardan los torrents nuevos, salvo que elija otra en la ventana Añadir nuevo torrent. |
| **Mover las descargas completadas a:** | Cuando un torrent termina de descargarse, sus archivos se mueven a la carpeta de debajo y sigue sembrando desde allí. |
| **Añadir la etiqueta del torrent** | Junto a la casilla anterior: un torrent terminado con etiqueta se mueve a una carpeta con el nombre de la etiqueta, dentro de la carpeta de las descargas completadas. |
| **Mover solo desde la carpeta de descarga predeterminada** | Mueve solo los torrents guardados en la carpeta de las descargas nuevas; los torrents que guardó en otro sitio se quedan donde están. Activado de forma predeterminada. |

La Guía de configuración pregunta por la carpeta de descargas en el primer inicio: vea [Primer inicio y la Guía de configuración](Primer-inicio).

La misma página también guarda copias de los archivos .torrent y los carga desde una carpeta vigilada (**Ubicación de los .torrent**): vea [Añadir torrents](Añadir-torrents).

### Mover las descargas completadas

1. Marque **Mover las descargas completadas a:** y elija la carpeta.
2. Marque **Añadir la etiqueta del torrent** si ordena por etiquetas, por ejemplo `D:\Done\Movies` para los torrents con la etiqueta «Movies». Un torrent sin etiqueta va directamente a la carpeta.
3. Decida sobre **Mover solo desde la carpeta de descarga predeterminada** (arriba).
4. Haga clic en **Aceptar**.

Se aplica a los torrents que terminen a partir de ahora; los que terminaron antes se quedan donde están. Mientras se mueven los archivos de un torrent, su estado muestra **Moviendo** y el **Registro** dice «moviendo la descarga completada a ...». Cuenta la etiqueta que tiene el torrent cuando termina, así que defina las etiquetas antes de que se complete la descarga.

### Mover un torrent usted mismo

Haga clic con el botón derecho en los torrents y elija **Avanzado > Establecer ubicación de descarga...** para elegir una carpeta y mover sus archivos allí. Los archivos con el mismo nombre que ya estén en esa carpeta se reemplazan. El elemento está en gris mientras el torrent ya se está moviendo. Un solo archivo de un torrent puede moverse con **Reubicar...** de la pestaña Archivos: vea [Detalles y archivos del torrent](Detalles-y-archivos-del-torrent).

### Cerrar mientras se mueven archivos

Si sale mientras se mueven archivos, la ventana de cierre espera a que terminen los movimientos, con su progreso, ya que detenerse a mitad podría dejar un torrent mitad en una carpeta y mitad en la otra. **Salir ahora** pregunta primero y dice qué deshará: los archivos del torrent se quedan en la carpeta antigua, lo copiado a la nueva se borra y usted puede moverlo de nuevo más tarde.

### Archivos incompletos y .!ut

*Nuevo en 1.2.63.* Con **Opciones > Preferencias > General > Añadir .!ut a los archivos incompletos** marcada (en **Al descargar**, desactivada de forma predeterminada), un archivo que aún se está descargando se guarda con `.!ut` después de su nombre, por ejemplo `film.mkv.!ut`. En cuanto todo el archivo se ha descargado y ha superado su comprobación, recibe su nombre normal, mientras el resto del torrent continúa.

- **Por qué:** los reproductores multimedia, los vigilantes de archivos y [Sonarr y Radarr](Sonarr-Radarr-y-Prowlarr) solo ven un archivo con su nombre real cuando está completo, de modo que nada abre ni importa medio archivo. Así nombra µTorrent los archivos sin terminar.
- **Activarla o desactivarla** renombra de inmediato los archivos de los torrents en ejecución, y los de los torrents detenidos cuando se inicien de nuevo.
- **Lo que nunca lo recibe:** los archivos vacíos, los archivos de relleno que llevan algunos torrents, un archivo cuyo nombre sería demasiado largo con el sufijo, y un archivo cuyo nombre con el sufijo es otro archivo o carpeta del mismo torrent (un torrent que contiene a la vez `x` y `x.!ut`): conserva su nombre simple. Un archivo que usted renombró en la pestaña Archivos recibe `.!ut` después de su nuevo nombre.
- **Pestaña Archivos:** **Abrir** y **Abrir carpeta contenedora** encuentran el archivo con su nombre `.!ut`. La interfaz web y su API muestran los nombres simples.
- **Un archivo que otro programa tiene abierto** (en Windows, por ejemplo un reproductor que no lo comparte) conserva su nombre actual hasta que pueda renombrarse. El Registro lo dice una vez, y Rookparcel lo intenta de nuevo cada 30 segundos aproximadamente y en el siguiente inicio. Mientras tanto el torrent sigue funcionando.
- **Un archivo existente nunca se sobrescribe.** Si `film.mkv` y `film.mkv.!ut` están en la carpeta, el archivo terminado se queda como `film.mkv.!ut`. El Registro lo dice una vez. Quite el otro archivo y luego pause y reanude el torrent para que se renombre.
- **Reubicar...** en la pestaña Archivos, para un archivo que aún no se ha descargado, toma el archivo al que apunta con el nombre que tenga allí: `film.mkv`, `film.mkv.!ut` o `film.mkv.!qB`. Ejecute **Forzar comprobación** para que se cuenten sus piezas.
- **Establecer ubicación de descarga** reemplaza un archivo del mismo nombre en la carpeta nueva, también un `film.mkv` que haya allí mientras el archivo que se mueve sigue siendo `film.mkv.!ut`. Un archivo que aún no se había descargado se toma de la carpeta nueva con cualquiera de esos nombres que tenga allí.

**Si viene de µTorrent o qBittorrent.** Esté la casilla marcada o no, un archivo que falta con su nombre esperado se busca con `.!ut` (el sufijo de µTorrent) y con `.!qB` (la opción «Añadir la extensión .!qB a los archivos incompletos» de qBittorrent). Un torrent [importado](Importar-desde-uTorrent-y-qBittorrent) o añadido de nuevo con **Guardar en** apuntando a su carpeta antigua continúa por tanto desde sus archivos parciales en lugar de empezar de cero. Después se renombran al nombre que les da Rookparcel: `.!ut` mientras están sin terminar con la casilla marcada, el nombre simple en caso contrario. La carpeta separada de qBittorrent para los archivos incompletos y sus carpetas `.unwanted` no se examinan.

**Quitar y > Borrar datos** borra un archivo `.!ut` o `.!qB` solo cuando contiene datos de este torrent: Rookparcel escribió en él, o las piezas que contiene superaron su comprobación. Un archivo parcial con ese nombre que el torrent nunca tocó (de otro programa, o recogido pero aún sin comprobar) se conserva, y el Registro lo nombra.

## Cuando falta una unidad o un recurso compartido

*Nuevo en 1.2.76.* Si un torrent que tenía datos descargados no encuentra ninguno de sus archivos al iniciarse, o su carpeta de destino ha desaparecido (un disco USB desconectado, un recurso compartido de red o un montaje que no está, una letra de unidad que aún no está lista), Rookparcel no vuelve a descargar los datos. El torrent se detiene con un error que empieza por la ruta que falta y dice **files missing (the drive, share or folder is not available)**, y conserva las piezas que tenía. Cada 30 segundos Rookparcel vuelve a mirar y, cuando la carpeta ha vuelto, el torrent se inicia solo y continúa sin comprobar de nuevo los archivos. Lo mismo ocurre cuando la unidad desaparece mientras un torrent está descargando. Cerrar Rookparcel mientras tanto no cambia nada: el siguiente inicio encuentra el torrent de nuevo ausente, o de vuelta. **Iniciar** en un torrent retenido lo intenta de inmediato, y vuelve a quedar retenido si los archivos siguen faltando. Un torrent retenido tiene un error, así que el [Apagado automático](Apagado-automático) no lo espera.

Si solo faltan algunos archivos de un torrent (borrados a mano o por un antivirus), solo se vuelven a descargar sus piezas. Si borra a mano toda la carpeta de un torrent, el torrent también queda retenido. Para renunciar a los datos antiguos de un torrent retenido y descargarlo de nuevo, o para aceptar lo que hay ahora en el disco, use **Forzar comprobación**: comprueba los archivos que están y descarga el resto.

## E/S de disco inteligente

La E/S de disco inteligente mantiene el equipo con buena respuesta mientras Rookparcel lee y escribe. Sus ajustes están en **Opciones > Preferencias > Juegos** (una página justo encima de Avanzado en la lista de páginas).

La casilla **Usar E/S de disco inteligente** de la Guía de configuración activa o desactiva las cuatro partes a la vez. El ajuste por unidad está activado desde el principio; la casilla añade las comprobaciones suaves y la ralentización mientras un disco está ocupado. *La Guía de configuración es nueva en 1.1.62. Los ajustes existen desde la 1.1.40; desde la 1.2.78 están en la página Juegos, y las comprobaciones suaves están desactivadas al principio.*

### Comprobación

Una comprobación (por ejemplo, tras **Forzar comprobación**) lee cada pieza del disco y la compara con el torrent. En **Comprobación**:

| Ajuste | Significado | Valor predeterminado |
|---|---|---|
| **Comprobar los torrents con suavidad (prioridad baja, ceder el paso a otros programas)** | Comprueba con la prioridad de disco en segundo plano del sistema, deja en paz los archivos en caché de otros programas y hace pausas mientras las lecturas son más lentas de lo normal, para que el equipo siga respondiendo. Las comprobaciones tardan más mientras el disco está ocupado. | Desactivado |
| **Limitar la comprobación a (MB/s):** | Limita la velocidad de lectura de cada comprobación, para un disco que aún se retrasa durante las comprobaciones suaves. Aquí 1 MB son 1.000.000 de bytes. | Desactivado, 100 |
| **Unidades comprobadas a la vez (un torrent por unidad):** | Cuántas unidades se comprueban al mismo tiempo, un torrent por unidad; los torrents de la misma unidad esperan su turno. De 1 a 8. | 4 |

Las comprobaciones de torrents en unidades distintas se ejecutan en paralelo; dos torrents en una misma unidad se comprueban uno después de otro, para que la unidad no salte de uno a otro. El estado muestra **En cola para comprobar** para un torrent que espera su turno y después **Comprobando** con el porcentaje. *El estado en cola es nuevo en 1.2.76; antes, una comprobación en espera mostraba «Checking 0.0 %».* Una comprobación que usted pide con **Forzar comprobación** no espera una plaza libre en la cola de torrents ([Cola y siembra](Cola-y-siembra)).

*Nuevo en 1.2.76.* Un archivo que existe pero no se puede leer (otro programa lo tiene bloqueado o usted no tiene permiso) detiene la comprobación con un error de archivo, en lugar de contar sus piezas como ausentes y descargarlas de nuevo. Un archivo que no está en absoluto solo pierde sus propias piezas.

### Unidades

| Ajuste | Significado | Valor predeterminado |
|---|---|---|
| **Ajustar según el tipo de unidad (SSD, disco duro, red)** | Detecta si cada carpeta de destino está en un SSD, un disco duro, una unidad extraíble o un recurso compartido de red. Un disco duro o una unidad extraíble recibe entonces como máximo dos trabajos de disco a la vez, un servidor de red cuatro, para que no se saturen. | Activado |

El Registro enumera las unidades que encontró al iniciarse, como «Disk I/O: drive D: (HDD, SATA)». Los puentes USB, los controladores RAID y las máquinas virtuales suelen informar de un tipo equivocado.

**Corregir una suposición errónea.** Use `drive_overrides` en **Opciones > Preferencias > Avanzado**: pares `ruta=tipo` separados por comas, con el tipo `ssd`, `hdd`, `network` o `removable`, por ejemplo `D:/=hdd, E:/=removable`. Gana la ruta que coincide más larga.

1. Abra **Avanzado** y escriba `drive_overrides` en **Filtro:**.
2. Seleccione la fila, escriba los pares en **Valor:** y haga clic en **Establecer**.
3. Haga clic en **Aceptar** o en **Aplicar**.

![Preferencias, página Avanzado](images/prefs-advanced.png)

### Cuando el disco está ocupado

En **Cuando el disco está ocupado**, ambos desactivados de forma predeterminada:

| Ajuste | Significado |
|---|---|
| **Ralentizar las bajadas (escrituras en disco)** | Mientras una unidad responde a las escrituras claramente más despacio de lo normal, reduce a la mitad la velocidad de descarga paso a paso, nunca por debajo de 50 kB/s, y la deja crecer de nuevo cuando el disco está tranquilo. |
| **Ralentizar las subidas (lecturas de disco)** | Lo mismo para las subidas, mientras una unidad responde despacio a las lecturas. La siembra sube entonces menos, lo que importa en los trackers que cuentan su proporción. |

Nunca aumentan sus propios límites. La ralentización es un solo tope por sentido, compartido con [Mantener el ping bajo](Mantener-el-ping-bajo): gana el tope más bajo. Mientras frena un sentido, la barra de estado lo muestra como «(auto ...)», y el Registro tiene una línea para ello.

## Caché de disco y archivos

**Opciones > Preferencias > Avanzado > Caché de disco** tiene la caché de lectura y la cola de disco. La mayoría de las personas nunca necesita cambiarlas.

En **Caché de lectura del disco**:

| Ajuste | Significado | Valor predeterminado |
|---|---|---|
| **Activar la caché de lecturas del disco** | Guarda en memoria las piezas leídas recientemente, leyendo un bloque que piden los pares junto con sus vecinos, lo que ahorra búsquedas de disco al sembrar. | Activado |
| **Reemplazar el tamaño automático de la caché (MB):** | Usa el tamaño indicado al lado en lugar de los 32 MB automáticos. Una caché mayor puede ayudar en una siembra intensa, a costa de memoria. | Desactivado |
| **Quitar los bloques sin usar durante más de (segundos):** | Descarta los bloques en caché que ningún par ha pedido en ese tiempo. Desactivado, permanecen hasta que bloques más nuevos los desplazan. | Activado, 300 |

Las escrituras se dejan a la caché propia del sistema.

En **Cola de disco y archivos**:

| Ajuste | Significado | Valor predeterminado |
|---|---|---|
| **Cola de escritura en disco (kB):** | Cuántos datos descargados pueden esperar a escribirse en el disco; por encima de ese valor, Rookparcel deja de leer de los pares hasta que el disco se pone al día. Los valores por debajo de 4 MB actúan como 4 MB. | 1024 |
| **Número máximo de archivos abiertos:** | La mayor cantidad de archivos abiertos a la vez; los demás se cierran y se vuelven a abrir cuando hace falta. Auméntelo cuando muchos torrents o archivos están activos a la vez. | 40 |
| **Hilos de E/S de disco (desde el próximo inicio):** | Los hilos que leen y escriben los datos de los torrents. | 10 |
| **Hilos de hash (no se usa):** | Se conserva por compatibilidad; esta versión no lo usa. Las comprobaciones se ejecutan una por unidad, como se define en la página Juegos. | |

Rookparcel no tiene ningún ajuste para preasignar archivos. Un archivo se crea cuando llegan los primeros datos para él y, en Windows, se marca como disperso, de modo que escribir cerca de su final no hace que NTFS rellene de ceros la parte anterior.

## En la ventana GTK y en la interfaz web

- **Ventana GTK** (Linux, BSD), **macOS** y la **interfaz web**: **Añadir .!ut a los archivos incompletos** también está en la página General, en **Al descargar**.
- **Aplicación de macOS**: las mismas páginas y ajustes; las Preferencias se aplican de inmediato, sin **Aceptar** ([Referencia de preferencias](Referencia-de-preferencias#diferencias-en-macos)).
- **Ventana GTK** (Linux, BSD): las mismas páginas Carpetas, Caché de disco y Juegos. Los tipos de unidad también se detectan en Linux, macOS y FreeBSD, cada uno con los métodos propios del sistema.
- **Interfaz web**: las mismas páginas en **Preferencias**, con **Caché de disco** dentro de **Avanzado** y los ajustes de disco en **Juegos**. Las carpetas allí son carpetas del equipo donde se ejecuta Rookparcel, no del navegador.
- **rookparcel-nox**: `--save DIR` define la carpeta de descargas y `--move-on-complete DIR` mueve los torrents terminados; los demás ajustes están en las Preferencias de la interfaz web.

## Páginas relacionadas

- [Cola y siembra](Cola-y-siembra)
- [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos) (ajustes y estado de los torrents, no las descargas)
- [Gestionar torrents](Gestionar-torrents)
- [Referencia de preferencias](Referencia-de-preferencias)
