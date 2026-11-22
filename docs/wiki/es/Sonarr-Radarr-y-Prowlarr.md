*Esta página es una traducción de la página inglesa [Sonarr, Radarr and Prowlarr](Sonarr-Radarr-and-Prowlarr). Si hay diferencias, prevalece la página en inglés.*

# Sonarr, Radarr y Prowlarr

Sonarr, Radarr y Prowlarr pueden usar Rookparcel como su cliente de descargas. La interfaz web de Rookparcel responde a la misma API que la de qBittorrent, así que en estas aplicaciones usted añade Rookparcel como un cliente **qBittorrent**. Esta página explica cómo configurarlo con `rookparcel-nox` o con la propia interfaz web del programa de Windows, las categorías, las carpetas de descarga y cómo quitar las descargas terminadas.

Para buscar en su Prowlarr o Jackett desde dentro de Rookparcel (la otra dirección), vea [Buscar torrents](Buscar-torrents).

## Qué funciona

Esto se comprobó con las aplicaciones reales (Sonarr 4, Radarr y Prowlarr en Docker), tanto con `rookparcel-nox` en Linux como con la interfaz web del programa de Windows:

- El **Test** del cliente de descargas en cada aplicación.
- Las capturas desde Sonarr, Radarr y Prowlarr, cada una en su propia categoría.
- El tiempo de siembra de un indexador llegando con el torrent y deteniéndolo a tiempo.
- Sonarr y Radarr importando el archivo terminado (mediante enlace duro, o mediante una asignación de ruta remota) y quitando después el torrent con **Remove Completed**.
- La búsqueda y la captura de Prowlarr.

No comprobado: Lidarr y Readarr (Lidarr usa el mismo código de cliente que Radarr) y las capturas de enlaces magnet.

## 1. Active la interfaz web de Rookparcel

**Con rookparcel-nox:**

```sh
rookparcel-nox run --save /srv/media/downloads --webui 0.0.0.0:8080
```

Inicie sesión una vez en un navegador con la contraseña temporal que muestra nox y defina la suya en **Preferencias > Interfaz web**. Detalles: [Interfaz web](Interfaz-web#activarla-en-rookparcel-nox) y [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

`0.0.0.0` permite que otros equipos y contenedores lleguen a ella. Cuando las aplicaciones se ejecutan en el mismo equipo, no en contenedores, basta con `--webui 8080` (solo este equipo).

**Con el programa de Windows:** **Opciones > Preferencias > Interfaz web**, marque **Activar la interfaz web**, defina un **Usuario:** y una **Contraseña:**, y marque **Permitir conexiones desde otros equipos** si las aplicaciones se ejecutan en otro equipo. El Firewall de Windows necesita entonces una regla para `rookparcel.exe`. Detalles: [Interfaz web](Interfaz-web#activarla-en-el-programa-de-windows). Rookparcel debe estar en ejecución para que las aplicaciones lleguen a él.

## 2. Añada Rookparcel en la aplicación

En Sonarr, Radarr o Prowlarr, añada un nuevo cliente de descargas del tipo **qBittorrent** y rellene:

| Campo | Valor |
|---|---|
| Host | La dirección IP del equipo que ejecuta Rookparcel (o un nombre de host que haya permitido, vea [más abajo](#nombres-de-host-y-docker)). |
| Port | El puerto de la interfaz web, como `8080`. |
| SSL | Desactivado, salvo que la interfaz web use HTTPS con un certificado en el que confíe la aplicación. Las comprobaciones anteriores se hicieron con HTTP simple. |
| Username, Password | El inicio de sesión de la interfaz web. |
| Category | Una categoría de su elección, como `tv-sonarr`, `radarr` o `prowlarr`. |

Haga clic en **Test** y luego guarde. El Test crea la categoría en Rookparcel si aún no existe.

En lugar de un nombre de usuario y una contraseña, las aplicaciones pueden enviar una clave de API. Active **Aceptar una clave de API (Authorization: Bearer)** en **Preferencias > Interfaz web** de Rookparcel, escriba allí una clave de al menos 16 caracteres y dé la misma clave a la aplicación. Una tercera vía es **Omitir la autenticación para los clientes de estas subredes (...)** con la red de la aplicación, que deja entrar sin iniciar sesión a todos los clientes de ella: indique solo redes en las que confíe tanto como en la contraseña ([Seguridad de la interfaz web](Interfaz-web#seguridad)).

## Nombres de host y Docker

Rookparcel responde solo a direcciones IP y a `localhost` salvo que usted permita más nombres. Una aplicación que llega a él por un nombre, como `http://rookparcel:8080` desde un contenedor de Docker o el nombre de red de un PC, recibe «Bad Request: unexpected Host header», y su Test informa de un fallo de conexión.

Use la dirección IP en la aplicación, o añada el nombre a **Dominios del servidor (separe varias entradas con punto y coma):**:

- En la interfaz web de nox: **Preferencias > Interfaz web**, en **Conectividad**. Cambiarlo allí pide la **Contraseña actual:**.
- En el programa de Windows: **Opciones > Preferencias > Interfaz web**.

Varios nombres se separan con `;`; `*` coincide con cualquiera. Es preferible nombrar los hosts en lugar de usar `*`: la comprobación protege contra las páginas web que llegan a la interfaz web con un nombre propio.

Para ejecutar el propio nox en un contenedor junto a las aplicaciones, vea [Rookparcel en Docker](Rookparcel-en-Docker): su imagen, un archivo Compose con la carpeta de medios compartida y los ID de usuario.

## Categorías, etiquetas y marcas

Una categoría de qBittorrent es una **etiqueta** en Rookparcel. Los torrents que añade una aplicación reciben su categoría como etiqueta, así que los ve bajo esa etiqueta en la lista de categorías ([Gestionar torrents](Gestionar-torrents)). Cambiar la etiqueta de un torrent en Rookparcel cambia también su categoría para la aplicación; una aplicación solo sigue los torrents de su propia categoría.

Cuando la aplicación no da una carpeta de descarga propia, un torrent va a la carpeta de destino de su categoría si esta tiene una y, si no, a la carpeta de descargas predeterminada de Rookparcel.

Una etiqueta (tag) de qBittorrent es una **marca** en Rookparcel (*nuevo en 1.2.63*; antes, las marcas se aceptaban y se descartaban). Un torrent puede tener cualquier cantidad de ellas además de su etiqueta; se muestran en **Marcas** en la lista de categorías y en la columna **Marcas** ([Gestionar torrents](Gestionar-torrents#marcas)). Con **Add Series Tags** de Sonarr activado, una captura recibe las marcas de la serie. Radarr no envía marcas. Un nombre con una coma o un carácter de control se omite sin error, y los nombres distinguen mayúsculas de minúsculas, como en qBittorrent.

## Carpetas de descarga y rutas

La aplicación importa los archivos terminados leyéndolos de la ruta que indica Rookparcel. Esa ruta debe significar el mismo archivo para ambos:

- **Mismo equipo, sin contenedores:** nada que hacer.
- **Contenedores:** monte la carpeta de descargas en la misma ruta en el contenedor de la aplicación que la que usa Rookparcel, por ejemplo `/srv/media` en ambos. Así los enlaces duros también funcionan.
- **Equipos distintos** (por ejemplo, Rookparcel en Windows guardando en un recurso compartido del servidor donde se ejecuta Sonarr): añada un **Remote Path Mapping** en Sonarr y Radarr desde la ruta de Rookparcel a la ruta de la aplicación, como `\\server\media\downloads\` a `/srv/media/downloads/`. Esta fue la configuración comprobada con el programa de Windows.

Con nox, `--webui-save-root DIR` limita las carpetas en las que pueden guardar la interfaz web, y por tanto las aplicaciones ([Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox#interfaz-web)). Incluya las carpetas que usan las aplicaciones.

## Dejar fuera los archivos no deseados

*Nuevo en 1.2.63.* **No descargar los archivos llamados:** en **Preferencias > Interfaz** de Rookparcel se aplica también a las capturas de las aplicaciones. Con ello activado, los archivos cuyos nombres coinciden con los patrones, como `*.lnk; Thumbs.db; *sample*`, se marcan para no descargarse cuando la aplicación añade el torrent; para un enlace magnet, cuando ha llegado su lista de archivos. Un torrent cuyos archivos coinciden todos se descarga entero. Vea [Añadir torrents](Añadir-torrents#archivos-excluidos-por-nombre).

Las aplicaciones que definen los «Excluded file names» de qBittorrent mediante su API llegan al mismo ajuste: `excluded_file_names_enabled` y `excluded_file_names` (un patrón por línea).

## Límites de siembra y Remove Completed

**Remove Completed** en Sonarr y Radarr quita un torrent cuando está detenido y ha alcanzado su límite de siembra. Rookparcel informa de ambas cosas:

- **Una proporción de siembra o un tiempo de siembra definidos en el indexador** (en Sonarr, Radarr o Prowlarr) llega con cada torrent y se convierte en el objetivo de siembra propio de ese torrent. Un valor que el indexador deja vacío se toma de los ajustes de Rookparcel.
- **El objetivo de siembra propio de Rookparcel** está en **Preferencias > Cola**, en **Objetivo de siembra**: «Proporción mínima (%): [0: sin objetivo de proporción]» (150 % de forma predeterminada) y «Tiempo mínimo de siembra (minutos): [0: sin objetivo de tiempo]». Cuando un torrent lo alcanza, Rookparcel lo detiene y la aplicación puede quitarlo. Vea [Cola y siembra](Cola-y-siembra).

Dos cosas que conviene saber:

- Con **Limitar la velocidad de subida a (kB/s): [0: detener]** activada, un torrent sigue sembrando después de su objetivo, de modo que la aplicación nunca lo ve como terminado y no lo quita.
- El objetivo de Rookparcel se alcanza cuando se alcanza cada meta definida (proporción y tiempo), como en µTorrent; qBittorrent se detiene en la primera. Así que un torrent puede sembrar un poco más de lo que espera la aplicación, pero nunca se quita antes de que se cumplan ambas.

Un torrent que usted inicia con **Forzar inicio** nunca alcanza su objetivo de siembra.

## Prowlarr

Prowlarr solo añade torrents; no los sigue ni los importa. Añada Rookparcel en Prowlarr del mismo modo (un cliente qBittorrent con su propia categoría, como `prowlarr`) para enviar a Rookparcel las capturas de la búsqueda propia de Prowlarr.

## Límites conocidos

- Rookparcel se presenta como la versión 2.11.2 de la API de qBittorrent.
- Se ignoran cuando una aplicación los envía: la elección de disposición del contenido y el límite de tiempo de siembra inactiva de Sonarr. «Primero la primera y la última pieza» se respeta desde 1.2.63; antes se ignoraba.
- La petición de comprobación de una aplicación mantiene el comportamiento de qBittorrent: un torrent detenido sigue detenido y se comprueba cuando se inicia. **Forzar comprobación** en la interfaz web y en las ventanas inicia un torrent detenido desde 1.2.76.
- Añadir un torrent que ya está en la lista se responde como un éxito; la aplicación sigue entonces el existente.
- La API muestra `ip_filter_enabled` e `ip_filter_path` del filtro IP pero no los cambia, y no tiene ajuste para la dirección web de la lista de bloqueo. Defínalos en las propias Preferencias de Rookparcel.

## Si algo sale mal

| Lo que ve | Qué significa |
|---|---|
| El Test falla, y la dirección de Rookparcel es un nombre | La comprobación del Host rechazó el nombre. Use la dirección IP, o añada el nombre a **Dominios del servidor** ([más arriba](#nombres-de-host-y-docker)). |
| El Test falla con una dirección IP | La interfaz web no está activada, escucha solo en `127.0.0.1` (`--webui 8080` de nox, o **Permitir conexiones desde otros equipos** desactivada), un cortafuegos bloquea el puerto, o el inicio de sesión es incorrecto. Demasiados inicios de sesión erróneos bloquean la dirección de la aplicación durante una hora. |
| La importación falla: no se encuentra el archivo | La aplicación ve la descarga con otra ruta. Use la misma ruta de montaje, o un Remote Path Mapping ([más arriba](#carpetas-de-descarga-y-rutas)). |
| Los torrents terminados se quedan en la cola de la aplicación | Rookparcel sigue sembrándolos: no hay objetivo de siembra definido, el objetivo aún no se ha alcanzado, o **Limitar la velocidad de subida a (kB/s): [0: detener]** está activada. |

## Véase también

- [Interfaz web](Interfaz-web): activarla, HTTPS y seguridad.
- [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox): nox como servicio que se inicia con el equipo.
- [Rookparcel en Docker](Rookparcel-en-Docker): nox en un contenedor.
- Las notas técnicas sobre lo que llaman las aplicaciones: [INTEGRATIONS.md, part 1](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/INTEGRATIONS.md#part-1-the-shim-as-an-arr-download-client).
