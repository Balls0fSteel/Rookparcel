*Esta página es una traducción de la página en inglés [Where Rookparcel keeps its files](Where-Rookparcel-keeps-its-files). Si hay diferencias, la página en inglés es la válida.*

# Dónde guarda Rookparcel sus archivos

Esta página indica dónde guarda Rookparcel sus ajustes y su lista de torrents en cada plataforma, cómo el modo portátil los guarda junto al programa en su lugar, y cómo hacer una copia de seguridad, moverlos a otro equipo o empezar de cero. Sus descargas no forman parte de esto: están donde las guardó.

## Encontrar la carpeta

**Ayuda > Acerca de Rookparcel** (en macOS **Rookparcel > Acerca de Rookparcel**) muestra la ruta del archivo de ajustes al final. **Archivo > Perfiles > Abrir carpeta del perfil** abre la carpeta (*nuevo en la 1.1.48*).

## Windows

La ventana de Windows guarda todo en una carpeta: `%APPDATA%\Rookparcel`, que normalmente es `C:\Users\<usted>\AppData\Roaming\Rookparcel`. Las copias instaladas y las del zip usan la misma carpeta, salvo que la copia del zip sea portátil.

| Archivo o carpeta | Qué contiene |
|---|---|
| `rookparcel.ini` | los ajustes |
| `state\` | la lista de torrents y el progreso de cada torrent |
| `rss.dat` | canales y filtros RSS |
| `remote.ini`, `remote\` | las conexiones remotas y los ajustes de cada ventana remota |
| `ipfilter.dat` | el archivo del filtro de IP predeterminado, si usa uno |
| `ipfilter-url.dat`, `ipfilter-url.txt` | la lista de bloqueo del filtro de IP tal como se descargó por última vez de su dirección web, y una nota de dónde y cuándo, si usa uno (*nuevo en la 1.2.63*) |
| `nox\` | los archivos de `rookparcel-nox` cuando se ejecutó desde una consola |
| `search-helper\` | el asistente de búsqueda, si lo instaló ([Buscar torrents en Windows](Buscar-torrents-en-Windows)) |
| `profiles\` | los demás [perfiles](Perfiles), cada uno en una carpeta con la misma estructura que esta |
| `rookparcel.lock` | marca la carpeta como en uso mientras Rookparcel se ejecuta |

El servicio de Windows de `rookparcel-nox` guarda sus archivos en `%ProgramData%\Rookparcel\nox`. Consulte [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

## Linux y los BSD

La versión GTK sigue las carpetas XDG:

| Qué | Carpeta |
|---|---|
| ajustes: `rookparcel.ini`, `rss.dat`, `remote.ini`, `remote/`, `ipfilter.dat` y la lista descargada del filtro de IP (`ipfilter-url.dat`, `ipfilter-url.txt`) si los usa | `$XDG_CONFIG_HOME/rookparcel/gtk`, o si no `~/.config/rookparcel/gtk` |
| lista de torrents: `resume.db`, `resume.log`, `metadata.pack`, `dht_state.dat`, `stats.dat` (los totales de todas las sesiones de Ayuda > Estadísticas) | `$XDG_DATA_HOME/rookparcel/gtk`, o si no `~/.local/share/rookparcel/gtk` |

`rookparcel.lock` en la carpeta de la lista de torrents la marca como en uso. `rookparcel-nox` usa `nox` en lugar de `gtk` en las dos rutas.

El Flatpak guarda sus carpetas en `~/.var/app/io.github.balls0fsteel.Rookparcel/`. Con systemd, sus carpetas son las del propio servicio; consulte [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

## macOS

Tanto los ajustes como la lista de torrents están en `~/Library/Application Support/Rookparcel/mac`, con la misma estructura que la carpeta de Windows de arriba; `rookparcel-nox` usa `~/Library/Application Support/Rookparcel/nox`. Un [perfil](Perfiles) `work` está en `~/Library/Application Support/Rookparcel/profiles/work/mac`.

La versión GTK de `Rookparcel.app` de las versiones anteriores usaba una carpeta `gtk` en los mismos lugares. El primer inicio de la aplicación nativa renombra cada carpeta `gtk` a `mac` (la del perfil predeterminado y la de cada perfil) y corrige en sus ajustes las rutas que apuntan a ella, así que todo se conserva. Lo hace una sola vez: solo donde todavía no hay una carpeta `mac`, y no mientras la aplicación anterior esté usando la carpeta. Las carpetas portátiles se dejan como están.

*La aplicación nativa de macOS y su carpeta `mac` son nuevas en la 1.2.19; las versiones anteriores usan `gtk`.*

## Modo portátil

En el modo portátil, Rookparcel guarda sus ajustes y su lista de torrents en su propia carpeta, como hacía µTorrent con `settings.dat` junto a `uTorrent.exe`. La carpeta puede estar en una memoria USB o en una carpeta sincronizada.

**Para activarlo**, ponga un archivo llamado `rookparcel.ini` en el lugar correcto antes del primer inicio. Puede estar vacío. Debe ser un archivo: una carpeta con ese nombre no cuenta.

| Copia | Dónde va `rookparcel.ini` |
|---|---|
| Zip de Windows | junto a `rookparcel.exe` |
| tar.gz de Linux o BSD | junto a `rookparcel-gtk` (o `rookparcel-nox`) |
| AppImage de Linux | junto al archivo `.AppImage` |
| macOS | junto a `Rookparcel.app`, nunca dentro de ella; junto a `rookparcel-nox` para el demonio |
| Flatpak, instalador de Windows | sin modo portátil |

*El modo portátil en Windows está en todas las versiones. En Linux y los BSD es nuevo en la 1.1.49, y en macOS en la 1.1.50.*

La estructura dentro de una carpeta portátil es la misma que la de `%APPDATA%\Rookparcel` de arriba: los ajustes y `state` junto al programa, `nox` para `rookparcel-nox` y `profiles` para los demás perfiles.

### Qué cambia en el modo portátil

- **El primer inicio descarga en la carpeta.** Las descargas nuevas van a la carpeta `Downloads` que hay dentro de ella, y Rookparcel elige un puerto de escucha aleatorio, para no chocar con otro Rookparcel en el 6881. *Nuevo en la 1.1.45 y la 1.1.49.*
- **Las rutas dentro de la carpeta se guardan relativas a ella.** Las carpetas de guardado, las carpetas de Preferencias, el archivo del filtro de IP y las carpetas de guardado de RSS que están dentro de la carpeta portátil se escriben como `./Downloads/...`. Así la carpeta puede pasar a otra letra de unidad, o a otro equipo, y los torrents siguen sin necesidad de una nueva comprobación. Las rutas fuera de la carpeta se dejan como están. *Nuevo en la 1.1.49.* Las rutas escritas por una versión anterior se convierten la próxima vez que se guardan.
- **No se escribe nada en el sistema a menos que lo pida.** Las asociaciones, el inicio con el sistema y los servicios solo se establecen cuando los marca en Preferencias. Entonces nombran el programa de esta copia y dejan de funcionar si la carpeta se mueve.
- **No se actualiza sola.** El aviso de actualización de una copia portátil abre la página de descargas. Consulte [Actualizar Rookparcel](Actualizar-Rookparcel).

Una carpeta portátil escrita por la 1.1.49 o posterior necesita la 1.1.49 o posterior: una versión anterior leería mal las rutas relativas.

Si no se puede escribir en la carpeta (una memoria de solo lectura, `C:\Program Files`), Rookparcel se detiene con «Este Rookparcel portátil no puede escribir en su carpeta» en lugar de guardar sus ajustes en otro sitio. Mueva la carpeta, o quite `rookparcel.ini` para usar las carpetas normales.

En macOS, un `Rookparcel.app` iniciado directamente desde la carpeta Descargas después de descargarlo se ejecuta desde una copia temporal, donde no se ve el `rookparcel.ini` que hay junto a él. Rookparcel lo avisa. Mueva `Rookparcel.app` con el Finder a la carpeta que quiera e inícielo de nuevo.

## Una carpeta, un Rookparcel

Dos Rookparcel nunca usan una carpeta al mismo tiempo. Al iniciar Rookparcel una segunda vez, la ventana que ya se ejecuta pasa al primer plano y recibe lo que usted abrió. Una copia que encuentra la carpeta en uso desde otra sesión, o desde otro equipo a través de un recurso compartido de red, muestra «Otro Rookparcel ya está usando esta carpeta» y se cierra. Cierre primero el otro. (*La comprobación entre sesiones y equipos es nueva en la 1.1.43.*)

Para ejecutar dos Rookparcel a la vez a propósito, use [perfiles](Perfiles).

## Copia de seguridad

1. Cierre Rookparcel (**Archivo > Salir**; en macOS **Rookparcel > Salir de Rookparcel**), para que todo quede guardado.
2. Copie la carpeta entera: `%APPDATA%\Rookparcel` en Windows, tanto `~/.config/rookparcel` como `~/.local/share/rookparcel` en Linux, `~/Library/Application Support/Rookparcel` en macOS, o la carpeta portátil.

La copia contiene los ajustes, la lista de torrents con el progreso de cada torrent, los canales RSS y todos los perfiles. No contiene los archivos descargados.

## Mover a otro equipo

1. Cierre Rookparcel en el equipo antiguo y copie la carpeta, como para una copia de seguridad.
2. Instale Rookparcel en el equipo nuevo, pero no lo inicie todavía.
3. Ponga la carpeta en el mismo lugar allí.
4. Copie o mueva sus descargas.
5. Inicie Rookparcel.

Fuera del modo portátil, la lista de torrents guarda rutas completas. Ponga las descargas en las mismas rutas del equipo nuevo (la misma letra de unidad y las mismas carpetas en Windows), o los torrents no encontrarán sus archivos.

Una carpeta portátil es más fácil: copie la carpeta entera, descargas incluidas, e inicie el programa que hay dentro. Sus rutas son relativas, así que da igual otra letra de unidad u otro nombre de usuario.

## Empezar de cero

Para restablecer Rookparcel por completo:

1. Cierre Rookparcel.
2. Renombre la carpeta (por ejemplo a `Rookparcel.old`), para poder volver atrás.
3. Inicie Rookparcel. Empieza como una instalación nueva, con la [Guía de configuración](Primer-inicio) y una lista de torrents vacía.

Sus archivos descargados se quedan donde están; añada de nuevo sus torrents para sembrarlos. El desinstalador de Windows también puede borrar la carpeta: su opción **Configuración y lista de torrents** ([Instalar Rookparcel](Instalar-Rookparcel#desinstalación)).
