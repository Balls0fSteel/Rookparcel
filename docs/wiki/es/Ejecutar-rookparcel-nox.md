*Esta página es una traducción de la página inglesa [Running rookparcel-nox](Running-rookparcel-nox). Si hay diferencias, prevalece la página en inglés.*

# Ejecutar rookparcel-nox

`rookparcel-nox` es Rookparcel sin ventana: el mismo motor, ejecutado desde un terminal o como servicio, y controlado mediante su [Interfaz web](Interfaz-web), desde un Rookparcel de escritorio con [Control remoto](Control-remoto), o por Sonarr, Radarr y Prowlarr ([Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr)). Esta página explica sus órdenes y opciones, cómo ejecutarlo como servicio de Windows, con systemd y en FreeBSD, dónde guarda sus archivos y cómo detenerlo.

## Dónde obtenerlo

`rookparcel-nox` viene con cada versión ([Instalar Rookparcel](Instalar-Rookparcel)):

| Plataforma | Archivo |
|---|---|
| Windows | `rookparcel-nox.exe`, junto a `rookparcel.exe` en el zip y en la carpeta instalada |
| Linux | en el archivo `rookparcel-<version>-linux-<arch>.tar.gz` (con las unidades de systemd), o por separado como `rookparcel-nox-<version>-<arch>.AppImage` |
| macOS | `rookparcel-nox-<version>-macos-<arch>.tar.gz` |

`rookparcel-nox --help` enumera todas las órdenes y opciones, `rookparcel-nox --version` muestra la versión y `rookparcel-nox --license` muestra la licencia y los avisos de terceros.

## Una primera ejecución

1. Abra un terminal (en Windows, un símbolo del sistema) en la carpeta que contiene `rookparcel-nox`.
2. Inícielo con una carpeta de descargas y la interfaz web:

   ```sh
   rookparcel-nox run --save ~/Downloads --webui 8080
   ```

3. Muestra unas líneas que empiezan por `*`. Una de ellas es la dirección de la interfaz web y, la primera vez, una línea como esta:

   ```text
   * Web UI: no password set; temporary password for this run: ... (set one with --webui-pass)
   ```

4. Abra `http://127.0.0.1:8080/` en un navegador e inicie sesión como `admin` con esa contraseña temporal.
5. En la página, abra **Preferencias > Interfaz web** y defina su propia contraseña. A partir de entonces queda guardada (como un hash), y nox la pide en cada inicio.

Para detenerlo, escriba `q` y pulse <kbd>Enter</kbd>, o pulse <kbd>Ctrl</kbd>+<kbd>C</kbd>. Guarda el estado de cada torrent al salir, de modo que el siguiente `run` los reanuda sin volver a comprobar los archivos.

`--webui 8080` escucha solo en este equipo (`127.0.0.1`). Para llegar a la página desde otros equipos o desde un teléfono, indique también una dirección, por ejemplo `--webui 0.0.0.0:8080`, y lea antes [Seguridad de la interfaz web](Interfaz-web#seguridad).

## Órdenes

| Orden | Qué hace |
|---|---|
| `rookparcel-nox run [OPTIONS]` | Reanuda cada torrent guardado en la carpeta de estado y sigue ejecutándose hasta que se detiene. Es el demonio. |
| `rookparcel-nox add <file.torrent>... [--save DIR] [OPTIONS]` | Añade los torrents (archivos o enlaces `magnet:`), los descarga y se cierra cuando terminan. Con `--seed` sigue sembrando en cambio. |
| `rookparcel-nox create <file or folder> [-o OUT] [CREATE OPTIONS]` | Crea un archivo `.torrent` ([más abajo](#hacer-un-archivo-torrent)). |
| `rookparcel-nox service ...` | El servicio de Windows (solo Windows, [más abajo](#el-servicio-de-windows)). |
| `rookparcel-nox demo [--torrents N] [--active FRACTION] [--seconds N]` | Carga torrents simulados y muestra totales; solo para pruebas. |

`run` también acepta archivos `.torrent` y enlaces magnet después de las opciones; se añaden al inicio.

## Opciones de run y add

### Carpetas y estado

| Opción | Significado |
|---|---|
| `--save DIR` | La carpeta de descargas. Valor predeterminado: la carpeta actual. |
| `--state-dir DIR` | Dónde se guardan la lista de torrents y los datos de reanudación ([archivos](#dónde-guarda-sus-archivos)); también la carpeta de ajustes salvo que se indique `--config-dir`. `--state DIR` es lo mismo. |
| `--config-dir DIR` | Dónde se guardan el inicio de sesión y los ajustes de la interfaz web (`webui.json`). |
| `--profile NAME` | Ejecuta un nox independiente con sus propias carpetas, inicio de sesión y puerto de escucha ([perfiles](#perfiles)). |
| `--no-state` | No carga ni guarda nada. |
| `--move-on-complete DIR` | Mueve a DIR cada torrent que termina de descargarse. |

### Siembra y detención

| Opción | Significado |
|---|---|
| `--seed` | `add`: sigue sembrando tras la descarga en lugar de cerrarse. |
| `--super-seed` | Siembra inicial para los torrents que están completos; implica `--seed`. |
| `--seconds N` | Se detiene tras N segundos. |
| `--exit-when-done downloads` o `all` | Se cierra, con el estado guardado, cuando ya no se descarga nada (`downloads`), o no se descarga ni se siembra nada (`all`). Es lo mismo que **Opciones > Apagado automático** en el programa de escritorio ([Apagado automático](Apagado-automático)). |

### Red

| Opción | Significado |
|---|---|
| `--port N` | El puerto de escucha para los pares. Valor predeterminado: 6881. |
| `--listen LIST` | Interfaces de escucha en lugar de `--port`: `IP:PORT`, `[IPv6]:PORT` o `DEVICE:PORT`, separadas por comas. |
| `--dl-limit KIB`, `--ul-limit KIB` | Límites de bajada y de subida en KiB/s; 0 es sin límite. |
| `--ip-filter FILE` | Bloquea los intervalos de direcciones de FILE (`ipfilter.dat`, formato P2P o CIDR; CIDR es nuevo en 1.2.63). |
| `--proxy TYPE://[USER:PASS@]HOST:PORT` | Se conecta a través de un proxy; TYPE es `socks4`, `socks5` o `http`. No se acepta nada entrante mientras los pares pasan por él. |
| `--no-proxy-peers` | Se conecta directamente a los pares y a las semillas web; solo los trackers usan el proxy. |
| `--no-proxy-trackers` | Contacta directamente con los trackers. |
| `--no-proxy-hostnames` | Resuelve los nombres de host aquí en lugar de en el proxy. |
| `--socks5-udp-local-ep` | Para un proxy SOCKS5 sin NAT por en medio; desactivado de forma predeterminada. |
| `--network RULE` | Usar solo esta red: el tráfico de los torrents se vincula al adaptador que coincide. RULE es `vpn` (cualquier adaptador VPN), `adapter:ID` (un nombre de dispositivo; en Windows el GUID del adaptador), `range:CIDR` (como `10.2.0.0/16`) u `off`. |
| `--no-pause-when-missing` | Mientras la regla de `--network` no coincide con nada, deja que los torrents se ejecuten sin vincular en lugar de esperar a la red. |

*`--network` es nueva en 1.1.58.* Las opciones de proxy y de red se guardan en los ajustes de la interfaz web como las casillas correspondientes de su página **Preferencias > Conexión**. Una vez que cambia algo en la interfaz web, el valor guardado prevalece sobre la línea de órdenes, así que a partir de entonces cámbielas en la página. Una contraseña de proxy en la línea de órdenes puede ser vista por otros usuarios del equipo en la lista de procesos; defínala en **Preferencias > Conexión**. Más sobre estos ajustes: [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos).

### Interfaz web

| Opción | Significado |
|---|---|
| `--webui [ADDR:]PORT` | Sirve la interfaz web y la API compatible con qBittorrent. Sin ADDR escucha solo en `127.0.0.1`. |
| `--webui-user NAME` | El nombre de usuario. Valor predeterminado: `admin`. |
| `--webui-pass PASS` | La contraseña, guardada como un hash. Sin una (y sin ninguna guardada), se muestra una contraseña temporal al inicio. |
| `--webui-save-root DIR` | Puede indicarse varias veces. La interfaz web solo podrá entonces guardar, mover y leer en estas carpetas y por debajo de ellas. Sin esta opción, cualquiera con el inicio de sesión puede usar cualquier carpeta a la que llegue la cuenta de nox. |
| `--webui-https` | Sirve HTTPS con un certificado autofirmado creado en el primer uso. |
| `--webui-remote-run-program` | Permite que una sesión de la interfaz web desde otro equipo cambie [Ejecutar programa](Referencia-de-preferencias#ejecutar-programa), siempre con la contraseña actual. Sin ella, esas líneas solo se cambian desde un navegador del mismo equipo. Para servidores sin navegador local, como un contenedor; nunca se guarda, así que debe estar en la línea de órdenes en cada inicio. Un proxy inverso en el propio servidor debe añadir `X-Forwarded-For`, o cada visitante cuenta como un navegador del mismo equipo ([Ejecutar programa](Referencia-de-preferencias#ejecutar-programa)). |
| `--webui-cert FILE [--webui-key FILE]` | Sirve HTTPS con su propio certificado y clave (PEM; la clave sin cifrar). |

Una contraseña en la línea de órdenes es visible para otros usuarios locales en la lista de procesos. Es preferible definirla en **Preferencias > Interfaz web** de la página. Detalles sobre HTTPS y los demás ajustes de seguridad: [Interfaz web](Interfaz-web#https).

### Otras

| Opción | Significado |
|---|---|
| `--import qbittorrent` o `utorrent` | Solo `run`: añade al inicio los torrents de qBittorrent o µTorrent, con sus datos donde están (comprobados primero), estadísticas, prioridades, trackers, etiquetas y las marcas de qBittorrent. El otro cliente no debe estar en ejecución; sus archivos solo se leen. Vea [Importar desde uTorrent y qBittorrent](Importar-desde-uTorrent-y-qBittorrent). |
| `--import-dir DIR` | La carpeta del otro cliente cuando no está en el lugar habitual. |
| `--verbose` (`-v`) | Muestra los eventos de trackers y pares. |
| `--no-update-check` | Solo `run`: no pregunta al sitio web del proyecto por versiones más recientes. |
| `--daemon` | Unix: se desacopla y pasa a segundo plano ([otros sistemas de inicio](#otras-formas-de-ejecutarlo-en-segundo-plano)). |
| `--pid-file FILE` | Escribe el id del proceso en FILE y lo bloquea; un segundo nox con el mismo FILE se niega a iniciarse. |
| `--log FILE` | Añade a FILE todo lo que muestra nox. |

### La búsqueda de actualizaciones

`run` pregunta al sitio web del proyecto qué versiones existen, 10 segundos después del inicio y cada 12 horas. No se envía nada sobre su instalación y no se descarga nada. Una versión más reciente es una línea en la salida y un enlace en la barra de estado de la interfaz web. Desactívela con `--no-update-check`, o en la interfaz web en **Preferencias > General > Privacidad**, «Buscar actualizaciones automáticamente». «Actualizar a versiones beta», allí mismo, ofrece también las versiones preliminares. Vea [Actualizar Rookparcel](Actualizar-Rookparcel).

## Hacer un archivo torrent

```sh
rookparcel-nox create ~/Videos/holiday -t udp://tracker.example:1337/announce --magnet
```

| Opción | Significado |
|---|---|
| `-o`, `--output FILE` | Dónde escribir el torrent. Valor predeterminado: `<name>.torrent`. |
| `-t`, `--tracker URL` | Un tracker; repítalo para más niveles, o ponga varios en un nivel separados por comas. |
| `-w`, `--web-seed URL` | Una semilla web; repetible. |
| `--node HOST:PORT` | Un nodo de arranque de DHT; repetible. |
| `--private` | Activa el indicador de privado (sin DHT, intercambio de pares ni detección local). |
| `--piece-size KIB` | Tamaño de pieza en KiB. Valor predeterminado: automático. |
| `--comment TEXT`, `--source TEXT`, `--creator TEXT` | El comentario, el campo de origen (usado por los trackers privados) y «creado por». |
| `--no-date` | Omite la fecha de creación. |
| `--hybrid`, `--v1`, `--v2` | Híbrido (el valor predeterminado), solo v1 o solo v2. |
| `--pad` | Alinea los archivos con las piezas mediante archivos de relleno (siempre activado en híbrido y v2). |
| `--mtime`, `--symlinks`, `--no-attributes`, `--skip-hidden` | Guarda las fechas de modificación; guarda los enlaces como enlaces (Unix); omite los atributos de oculto y ejecutable; omite los nombres que empiezan por punto. |
| `--magnet` | También muestra el enlace magnet. |

La ventana del programa de escritorio para esto se describe en [Crear torrents](Crear-torrents).

## Dónde guarda sus archivos

| | Linux y BSD | Windows | macOS |
|---|---|---|---|
| Lista de torrents y datos de reanudación (`resume.db`, `metadata.pack`, `dht_state.dat`) | `~/.local/share/rookparcel/nox` | `%APPDATA%\Rookparcel\nox` | `~/Library/Application Support/Rookparcel/nox` |
| Ajustes (`webui.json`, `ipfilter.dat`, el certificado HTTPS) | `~/.config/rookparcel/nox` | la misma carpeta | la misma carpeta |
| El servicio de Windows | | `%ProgramData%\Rookparcel\nox` | |

- En Linux se usan `$XDG_DATA_HOME` y `$XDG_CONFIG_HOME` cuando están definidas.
- El nox en ejecución reescribe `webui.json` siempre que algo cambia en la interfaz web. Edítelo solo mientras nox está detenido.
- Solo un nox puede usar una carpeta a la vez. Un segundo se detiene con «... is in use by another rookparcel-nox».
- **Modo portátil:** con un `rookparcel.ini` junto al programa, ambos van a una carpeta `nox` junto a él, y las carpetas de descargas dentro de ella se guardan relativas a ella, de modo que toda la carpeta puede moverse. *Nuevo en 1.1.49.* Vea [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos).

## Perfiles

`--profile NAME` ejecuta un nox independiente con sus propios torrents, ajustes, inicio de sesión de la interfaz web y servicio. Un nombre tiene de 1 a 32 caracteres: letras a-z, dígitos y guiones, empezando por una letra. Salvo que `--port` o `--listen` indiquen otra cosa, cada perfil recibe un puerto de escucha propio, calculado a partir de su nombre y el mismo en cada inicio.

| | perfil predeterminado | `--profile work` |
|---|---|---|
| Linux y BSD | `~/.local/share/rookparcel/nox` | `~/.local/share/rookparcel/profiles/work/nox` (y lo mismo bajo `~/.config`) |
| Windows | `%APPDATA%\Rookparcel\nox` | `%APPDATA%\Rookparcel\profiles\work\nox` |
| macOS | `~/Library/Application Support/Rookparcel/nox` | `.../Rookparcel/profiles/work/nox` |
| Servicio de Windows | `RookparcelNox` | `RookparcelNox_work` |
| systemd | `rookparcel-nox.service` | `rookparcel-nox@work.service` |

La interfaz web de cada perfil necesita un puerto propio, por ejemplo `--webui 127.0.0.1:8081`. Un puerto que ya está en uso detiene nox con «port N is in use, perhaps by another Rookparcel profile». *Los perfiles son nuevos en 1.1.44.* Vea [Perfiles](Perfiles).

## El servicio de Windows

`rookparcel-nox service install` registra nox como un servicio de Windows llamado **RookparcelNox** («Rookparcel nox»). Se inicia con Windows y se ejecuta sin que nadie haya iniciado sesión.

Esto no es lo mismo que **Preferencias > Interfaz web** en `rookparcel.exe`: eso sirve la interfaz web desde la ventana, sobre los torrents de la propia ventana, solo mientras la ventana está en ejecución ([Interfaz web](Interfaz-web)).

`install`, `uninstall`, `start` y `stop` necesitan un símbolo del sistema de administrador.

1. Ponga `rookparcel-nox.exe` donde la cuenta del servicio pueda leerlo, como `C:\Program Files\Rookparcel\`. Una copia dentro de su carpeta de usuario (Descargas, Documentos) no puede ser abierta por la cuenta del servicio; `install` avisa de ello.
2. Instálelo con las opciones de `run` con las que deba iniciarse:

   ```bat
   "C:\Program Files\Rookparcel\rookparcel-nox.exe" service install --webui 127.0.0.1:8080
   ```

3. Inícielo con `rookparcel-nox service start`, o añada `--now` a la instalación. A partir de entonces se inicia con Windows.
4. Lea la contraseña temporal de la interfaz web en `%ProgramData%\Rookparcel\nox\nox.log` (desde un símbolo de administrador), abra la página y defina su propia contraseña. O indique `--webui-user` y `--webui-pass` a `install`: la contraseña se guarda como un hash en el `webui.json` del servicio, nunca en la línea de órdenes del servicio.

Opciones de `install` en sí:

| Opción | Significado |
|---|---|
| `--account A` | `LocalService` (el valor predeterminado), `NetworkService` o `LocalSystem`. |
| `--start TYPE` | `delayed` (el valor predeterminado: automático, cuando el arranque se ha asentado), `auto` o `manual`. |
| `--replace` | Detiene y quita primero un RookparcelNox instalado; su estado se conserva. |
| `--now` | Inicia el servicio una vez instalado. |
| `--profile NAME` | Instala el servicio propio del perfil, `RookparcelNox_NAME`. |

Qué configura:

- **Estado y ajustes** en `%ProgramData%\Rookparcel\nox` (o `--state-dir`), una carpeta que solo pueden abrir SYSTEM, los administradores y la cuenta del servicio, ya que contiene el inicio de sesión de la interfaz web.
- **Descargas** en su subcarpeta `Downloads` salvo que indique `--save`.
- **Registro** en `nox.log` allí salvo que indique `--log`.
- **Reinicios** tras un fallo: a los 5, 30 y 60 segundos.
- Al apagar el sistema se le da tiempo para guardar el estado de los torrents.

Las carpetas en las que escribe el servicio y que `install` no creó (una carpeta de `--save`, una carpeta de `--move-on-complete`) necesitan un permiso para la cuenta, desde un símbolo de administrador:

```bat
icacls D:\Torrents /grant "NT AUTHORITY\LocalService:(OI)(CI)M"
```

Las conexiones entrantes de pares necesitan una regla del Firewall de Windows para el programa; un servicio no puede pedirla:

```bat
netsh advfirewall firewall add rule name="Rookparcel nox" dir=in action=allow program="C:\Program Files\Rookparcel\rookparcel-nox.exe"
```

Las demás órdenes del servicio:

| Orden | Qué hace |
|---|---|
| `rookparcel-nox service start`, `service stop` | Lo inicia o lo detiene y espera hasta que lo ha hecho. `sc start RookparcelNox`, `sc stop RookparcelNox` y la consola Servicios hacen lo mismo. |
| `rookparcel-nox service status` | Muestra el estado y la línea de órdenes del servicio (no necesita administrador). `--all` enumera el servicio de cada perfil. |
| `rookparcel-nox service uninstall` | Lo detiene y lo quita. La carpeta de estado y el registro se conservan. `--all` quita el servicio de cada perfil que ejecuta este programa. |
| `rookparcel-nox service debug [OPTIONS]` | Ejecuta el trabajo del servicio en esta consola, para probar las opciones antes de instalar. |
| `sc control RookparcelNox paramchange` | Recarga el filtro IP y vuelve a abrir el registro. |

Para cambiar las opciones, instale de nuevo con `--replace`. Para borrar también el estado de los torrents y las descargas, borre `%ProgramData%\Rookparcel` después de desinstalar.

`start`, `stop`, `status` y `uninstall` aceptan `--profile NAME` para el servicio de un perfil. *Los servicios por perfil son nuevos en 1.1.47.*

**Sin el servicio:** `rookparcel-nox run` en un símbolo del sistema también funciona y se detiene limpiamente con <kbd>Ctrl</kbd>+<kbd>C</kbd> o al cerrar la ventana. El Programador de tareas es otra vía: una tarea «Al iniciar» que ejecute `rookparcel-nox.exe run --save D:\Torrents --webui 8080 --log C:\Rookparcel\nox.log`. `--daemon` no funciona en Windows.

## systemd (Linux)

El archivo de Linux trae unidades ya preparadas, también en la carpeta [`contrib/systemd`](https://github.com/Balls0fSteel/Rookparcel/tree/main/contrib/systemd) del repositorio.

**Como servicio del sistema**, ejecutado con una cuenta `rookparcel`, con su estado en `/var/lib/rookparcel-nox`:

```sh
sudo install -m 755 rookparcel-nox /usr/local/bin/
sudo install -D -m 644 rookparcel-nox.sysusers /etc/sysusers.d/rookparcel-nox.conf
sudo systemd-sysusers
sudo install -m 644 rookparcel-nox.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now rookparcel-nox
journalctl -u rookparcel-nox
```

El diario muestra la contraseña temporal de la interfaz web en el primer inicio. La unidad sirve la interfaz web en `127.0.0.1:8080` y descarga en `/var/lib/rookparcel-nox/downloads`. Para cambiarlo, y para permitirle escribir en otra carpeta:

```ini
# sudo systemctl edit rookparcel-nox
[Service]
ExecStart=
ExecStart=/usr/local/bin/rookparcel-nox run --save /srv/torrents --webui 0.0.0.0:8080
ReadWritePaths=/srv/torrents
```

El servicio está muy restringido (no puede escribir fuera de su propia carpeta sin `ReadWritePaths=`), de modo que una carpeta de descargas en otro lugar siempre necesita esa línea.

**Como servicio de su propio usuario**, con sus archivos en las carpetas enumeradas [más arriba](#dónde-guarda-sus-archivos):

```sh
install -m 755 rookparcel-nox ~/.local/bin/
install -D -m 644 rookparcel-nox.service ~/.config/systemd/user/rookparcel-nox.service
systemctl --user daemon-reload
systemctl --user enable --now rookparcel-nox
loginctl enable-linger "$USER"
```

`loginctl enable-linger` lo mantiene en ejecución después de que usted cierre la sesión. Esta unidad descarga en `~/Downloads`.

Los **perfiles** usan las unidades de plantilla `rookparcel-nox@.service`: `systemctl enable --now rookparcel-nox@work` inicia el perfil `work`. No tienen interfaz web de forma predeterminada, porque dos no pueden compartir un puerto; dé a cada uno la suya en `systemctl edit rookparcel-nox@work`.

`systemctl reload rookparcel-nox` recarga el filtro IP. La salida va al diario.

## FreeBSD

El script rc.d está en la carpeta [`contrib/freebsd`](https://github.com/Balls0fSteel/Rookparcel/tree/main/contrib/freebsd) del repositorio:

```sh
pw useradd rookparcel -d /var/db/rookparcel-nox -s /usr/sbin/nologin -c "Rookparcel daemon"
install -m 755 rookparcel-nox /usr/local/bin/
install -m 555 rookparcel_nox /usr/local/etc/rc.d/
sysrc rookparcel_nox_enable=YES
sysrc rookparcel_nox_args="--webui 0.0.0.0:8080"
service rookparcel_nox start
```

Se ejecuta como `rookparcel`, con su estado en `/var/db/rookparcel-nox` y el registro en `/var/log/rookparcel-nox.log`, donde el primer inicio escribe la contraseña temporal de la interfaz web. Ajustes en `/etc/rc.conf`: `rookparcel_nox_args` (más opciones de `run`; valor predeterminado `--webui 127.0.0.1:8080`), `rookparcel_nox_user`, `rookparcel_nox_dir`, `rookparcel_nox_save`, `rookparcel_nox_log`. Use `rookparcel_nox_args`, no `rookparcel_nox_flags`. `service rookparcel_nox reload` recarga el filtro IP y vuelve a abrir el registro.

Perfiles: `sysrc rookparcel_nox_profiles="work"` y `sysrc rookparcel_nox_work_args="--webui 127.0.0.1:8081"`; `service rookparcel_nox start work` inicia uno, `service rookparcel_nox start` todos.

## Otras formas de ejecutarlo en segundo plano

En otros sistemas Unix:

- `--daemon` se desacopla y pasa a segundo plano, y vuelve cuando nox está en marcha (estado 0) o ha fallado al iniciarse (estado 1). Sin `--log`, la salida posterior se pierde, así que lea antes la contraseña temporal, o use `--log`.
- `--pid-file FILE` escribe el id del proceso; un segundo nox con el mismo archivo se niega a iniciarse.
- `--log FILE` añade toda la salida a FILE, legible solo por su propietario, ya que puede contener la contraseña temporal. Una señal `SIGHUP` lo vuelve a abrir, así que la rotación de registros funciona.

## Detener y recargar

| Qué | Efecto |
|---|---|
| `q` e <kbd>Enter</kbd>, <kbd>Ctrl</kbd>+<kbd>C</kbd>, `SIGTERM` | Una parada limpia: estado de los torrents guardado, asignaciones de puertos del router quitadas. |
| Un segundo <kbd>Ctrl</kbd>+<kbd>C</kbd> o `SIGTERM` durante esa parada | Sale de inmediato, sin guardar. |
| `SIGHUP` (servicio de Windows: `sc control RookparcelNox paramchange`) | Recarga la lista del filtro IP y vuelve a abrir el archivo de `--log`. `webui.json` no se vuelve a leer. Cuando la lista procede de una dirección, se recarga la copia descargada más recientemente; no se descarga nada. |
| Windows: cerrar la ventana de la consola, cerrar sesión, apagar | Una parada limpia. |

## Véase también

- [Interfaz web](Interfaz-web): la página que sirve nox.
- [Control remoto](Control-remoto): manejar nox desde la ventana del programa de escritorio.
- [Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr): nox como su cliente de descargas.
- La descripción técnica completa: [DAEMON.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/DAEMON.md).
