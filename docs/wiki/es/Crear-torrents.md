*Esta es una traducción de la página inglesa [Creating torrents](https://github.com/Balls0fSteel/Rookparcel/wiki/Creating-torrents). Si hay diferencias, prevalece la página inglesa.*

# Crear torrents

Esta página muestra cómo crear un archivo `.torrent` nuevo a partir de un archivo o una carpeta de su equipo con la ventana Crear nuevo torrent, qué significa cada opción y cómo empezar a sembrarlo de inmediato.

![La ventana Crear nuevo torrent](images/create-torrent.png)

## Crear un torrent en cinco pasos

1. Elija **Archivo > Crear nuevo torrent...** (<kbd>Ctrl</kbd>+<kbd>N</kbd>), o el botón **Crear nuevo torrent** de la barra de herramientas.
2. En **Seleccionar origen**, haga clic en **Añadir archivo** para un solo archivo o en **Añadir carpeta** para una carpeta. También puede escribir o pegar la ruta.
3. En **Propiedades del torrent**, añada los trackers que quiera, uno por línea. Sin trackers, los pares solo pueden encontrarse entre sí mediante DHT y el intercambio de pares.
4. Deje marcada **Empezar a sembrar** para compartirlo de inmediato.
5. Haga clic en **Crear y guardar como...**, elija dónde guardar el archivo `.torrent` y espere a que termine la barra de progreso.

El nombre de archivo sugerido es el nombre del origen más `.torrent`. Cuando termina el cálculo de los hashes, la ventana se cierra. Comparta el archivo `.torrent` con las personas que deban descargarlo.

## Las opciones

### Seleccionar origen y Omitir archivos

**Seleccionar origen** es el archivo o la carpeta a partir de los cuales se crea el torrent. El nombre de una carpeta se convierte en el nombre del torrent.

**Omitir archivos** deja fuera del torrent archivos y carpetas según su nombre. Escriba patrones con comodines, `*` para cualquier serie de caracteres y `?` para uno, separados por `|` o `;`. No se distinguen mayúsculas y minúsculas, y un patrón debe coincidir con el nombre completo de un archivo o carpeta, no con su ruta. Por ejemplo, `*.nfo|Thumbs.db|*.tmp` omite los archivos NFO, las cachés de miniaturas de Windows y los archivos temporales.

### Propiedades del torrent

| Opción | Significado |
|---|---|
| **Trackers:** | una dirección de tracker por línea. Una línea en blanco inicia un nuevo nivel: los niveles posteriores se usan cuando fallan los trackers anteriores. Las direcciones repetidas se descartan. |
| **Semillas web:** | direcciones de servidores web que tienen los mismos archivos, una por línea. Los pares pueden descargar de ellos como de una semilla. |
| **Comentario:** | un texto libre guardado en el torrent; otros programas lo muestran, por ejemplo en la ventana Añadir nuevo torrent. |
| **Tamaño de pieza:** | el tamaño de cada pieza, de 16kB a 16MB. **(detección automática)** elige uno a partir del tamaño total y es la opción adecuada para la mayoría de los torrents. |
| **Versión:** | la versión de BitTorrent, véase más abajo. |

### Versión: v1, v2 o híbrido

| Opción | Qué se obtiene |
|---|---|
| **Híbrido (v1 + v2)** | el valor predeterminado. Se carga en programas antiguos que solo admiten v1 y en los más nuevos que entienden v2. |
| **v1** | el formato clásico, como lo creaba µTorrent 2.2.1. Todos los programas BitTorrent lo leen. |
| **solo v2** | solo el formato más nuevo. Los programas que entienden únicamente v1 no pueden cargarlo. |

En caso de duda, deje **Híbrido (v1 + v2)**.

### Otros

| Opción | Significado |
|---|---|
| **Empezar a sembrar** | añade el torrent nuevo a su lista en cuanto se crea, con la carpeta que contiene el origen como carpeta de descarga. Rookparcel comprueba los archivos y después siembra. Los trackers de **Trackers para las nuevas descargas** en Preferencias no se le añaden; conserva los que usted escribió. |
| **Torrent privado** | marca el torrent como privado: los pares se encuentran solo a través de sus trackers, nunca mediante DHT, el intercambio de pares ni el descubrimiento de pares locales. Los trackers privados lo exigen. |
| **Conservar el orden de los archivos** | mantiene los archivos de una carpeta en el orden en que los lista Windows, en lugar de ordenarlos por nombre. Disponible solo para **v1**: los torrents v2 e híbridos siempre ordenan sus archivos. |

## Progreso y cancelación

Mientras Rookparcel lee los archivos y calcula sus hashes, la barra de progreso y el porcentaje avanzan y los demás controles aparecen atenuados. El botón **Cerrar** pasa a ser **Cancelar**: haga clic en él, pulse <kbd>Esc</kbd> o cierre la ventana para detener el proceso. Al cancelar no se guarda nada.

Si algo sale mal, un mensaje indica qué. Los más habituales:

| Mensaje | Significado |
|---|---|
| «Seleccione un archivo o una carpeta de la que crear el torrent.» | **Seleccionar origen** está vacío. |
| «...: no hay archivos para el torrent (vacío, o todos omitidos).» | la carpeta está vacía, o **Omitir archivos** no dejó nada. |

## Qué recuerda Rookparcel

La próxima vez que abra la ventana, tendrá los trackers, el tamaño de pieza, la versión, **Empezar a sembrar**, **Torrent privado** y **Conservar el orden de los archivos** que usó la última vez. El origen, los patrones de omisión, las semillas web y el comentario empiezan vacíos.

## Sembrar un torrent que creó antes

Si creó un torrent sin **Empezar a sembrar**, añada el archivo `.torrent` como cualquier otro ([Añadir torrents](Añadir-torrents)) y defina **Guardar en** con la carpeta que contiene el archivo o la carpeta de origen. Rookparcel comprueba los archivos y los siembra.

Para ayudar a que un torrent totalmente nuevo se difunda, puede activar **Siembra inicial** para él; consulte [Gestionar torrents](Gestionar-torrents#siembra-inicial).

## En la versión GTK (Linux, BSD)

La misma ventana con las mismas opciones, en **Archivo > Crear nuevo torrent...** (<kbd>Ctrl</kbd>+<kbd>N</kbd>).

## En la aplicación de macOS

La misma ventana con las mismas opciones, en **Archivo > Crear nuevo torrent...** (<kbd>⌘</kbd>+<kbd>N</kbd>).

## En la interfaz web y rookparcel-nox

La interfaz web no puede crear torrents: los archivos tendrían que estar en el servidor. En el propio servidor, `rookparcel-nox create` crea un torrent desde la línea de comandos:

```
rookparcel-nox create "D:\Share\My folder" -o my-folder.torrent -t udp://tracker.example.org:6969/announce
```

`--v1`, `--v2` y `--hybrid` eligen la versión (híbrido de forma predeterminada), `--private` lo marca como privado, `--piece-size` define el tamaño de pieza en KiB, `-w` añade una semilla web, y `--magnet` imprime además el enlace magnet. `rookparcel-nox --help` muestra todas las opciones. Consulte [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

En una ventana conectada con **Archivo > Conectar en remoto...**, **Empezar a sembrar** está desmarcada y atenuada: el torrent se crea a partir de archivos de este equipo, que el servidor no tiene. Consulte [Control remoto](Control-remoto).

## Páginas relacionadas

- [Añadir torrents](Añadir-torrents)
- [Gestionar torrents](Gestionar-torrents)
- [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos)
