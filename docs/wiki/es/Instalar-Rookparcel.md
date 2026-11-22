*Esta página es una traducción de la página en inglés [Installing Rookparcel](Installing-Rookparcel). Si hay diferencias, la página en inglés es la válida.*

# Instalar Rookparcel

Esta página enumera las descargas de cada plataforma, cómo instalar e iniciar cada una la primera vez y cómo quitar Rookparcel de nuevo. Después del primer inicio, consulte [Primer inicio y la Guía de configuración](Primer-inicio).

## Dónde descargar

Todas las versiones están en la [página de versiones publicadas](https://github.com/Balls0fSteel/Rookparcel/releases). La última versión publicada es la 1.2.76. Cada versión tiene notas con los pasos del primer inicio por plataforma y un archivo `SHA256SUMS` con la suma de comprobación de cada descarga.

| Plataforma | Archivo | Qué contiene |
|---|---|---|
| Windows 10 / 11, x64, instalador | `rookparcel-<version>-windows-x64-setup.exe` | `rookparcel.exe` (la ventana) y `rookparcel-nox.exe` (el demonio sin ventana), instalados con una entrada en el menú Inicio y un desinstalador |
| Windows 10 / 11, x64, portátil | `rookparcel-<version>-windows-x64.zip` | los mismos dos programas; no hay nada que instalar |
| Escritorio Linux, x86_64 o aarch64 | `Rookparcel-<version>-<arch>.AppImage` | la ventana GTK 4 con sus bibliotecas incluidas (necesita glibc 2.39 o posterior: Ubuntu 24.04, Fedora 40, Debian 13 o posteriores) |
| Escritorio Linux, Flatpak | `Rookparcel-<version>-<arch>.flatpak` | la ventana GTK 4 sobre el runtime GNOME 51 de Flathub |
| Linux, binarios sueltos | `rookparcel-<version>-linux-<arch>.tar.gz` | `rookparcel-gtk`, `rookparcel-nox`, la entrada de escritorio y las unidades de systemd; usan el GTK 4 (4.12 o posterior) y el OpenSSL 3 del sistema |
| Servidor Linux, sin ventana | `rookparcel-nox-<version>-<arch>.AppImage` | `rookparcel-nox` con OpenSSL incluido |
| macOS, Apple silicon o Intel | `Rookparcel-<version>-macos.dmg` | `Rookparcel.app`, la ventana nativa de macOS, una sola aplicación para los dos tipos de Mac (macOS 11 Big Sur o posterior) |
| macOS, sin ventana | `rookparcel-nox-<version>-macos-<arch>.tar.gz` | `rookparcel-nox` para Apple silicon (`arm64`, macOS 11 o posterior) e Intel (`x86_64`, macOS 10.14 o posterior) |

La ventana en Linux y los BSD es la versión GTK; en macOS es una aplicación nativa del Mac. Ambas funcionan como la versión de Windows; donde se diferencian, las páginas del wiki lo indican. Para ejecutar el demonio sin ventana, consulte [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

*La aplicación nativa de macOS es nueva en la 1.2.19. Las versiones anteriores tienen en su lugar `Rookparcel-<version>-arm64.dmg`: la ventana GTK incluida para Apple silicon, macOS 15 o posterior.*

## Windows

### Instalador o zip

- **El instalador** pone Rookparcel en el menú Inicio, puede asociarlo con los archivos `.torrent` y los enlaces `magnet:`, añade una entrada en Configuración > Aplicaciones para desinstalarlo y permite que Rookparcel se actualice solo (consulte [Actualizar Rookparcel](Actualizar-Rookparcel)).
- **El zip** no necesita instalación: descomprímalo donde quiera e inicie `rookparcel.exe`. Puede funcionar en modo portátil, con sus ajustes y torrents en su propia carpeta (consulte [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos#modo-portátil)). No se actualiza solo; una versión nueva es un zip nuevo.

Ambos son los mismos programas. Rookparcel necesita Windows 10 de 64 bits o posterior. Windows 11 en Arm ejecuta la compilación x64.

### Ejecutar el instalador

1. Ejecute `rookparcel-<version>-windows-x64-setup.exe`.
2. Windows SmartScreen puede mostrar «Windows protegió su PC», porque el instalador no tiene firma de código. Haga clic en **Más información** y después en **Ejecutar de todas formas**.
3. Elija el idioma del instalador, lea la página de bienvenida y acepte la licencia (la PolyForm Noncommercial License 1.0.0).
4. **Elegir usuarios:**
   - **Instalar solo para mí** (la opción predeterminada) no necesita derechos de administrador. Rookparcel se instala en `%LOCALAPPDATA%\Programs\Rookparcel`.
   - **Instalar para cualquier usuario de este equipo** instala en `C:\Program Files\Rookparcel`. Windows pide derechos de administrador y el aviso dice «Editor desconocido». Si lo rechaza, vuelve a esta página.
5. **Componentes:** Rookparcel en sí, **Acceso directo en el escritorio**, **Asociar con archivos .torrent** y **Asociar con enlaces magnet:**. Las tres últimas están marcadas en una instalación nueva.
6. Elija la carpeta, o deje la que se muestra, y deje que se instale.
7. En la última página, deje marcado «Ejecutar Rookparcel» para iniciarlo.

Las asociaciones se escriben para el usuario que ejecuta el instalador. Los demás usuarios de una instalación para todos pueden activarlas en **Opciones > Preferencias > General**.

El instalador rechaza una carpeta que contenga un archivo `rookparcel.ini`: esa carpeta es un Rookparcel portátil. Elija otra carpeta.

### Instalar sobre una versión anterior

Ejecute el instalador más reciente. Encuentra el Rookparcel instalado, omite las páginas de usuarios y de carpeta e instala encima, en la misma carpeta. Sus ajustes y torrents se quedan donde están. En una actualización, las casillas de asociación empiezan sin marcar, lo que deja las asociaciones como están.

Si Rookparcel se está ejecutando, el instalador le pide que lo cierre (**Archivo > Salir**) y que haga clic en **Reintentar**. Si `rookparcel-nox.exe` de la misma carpeta se está ejecutando, por ejemplo como el servicio Rookparcel nox, le pide que lo detenga primero. Desde un símbolo del sistema de administrador:

```
sc stop RookparcelNox
```

Inicie el servicio de nuevo después de la actualización. Una copia instalada también puede hacer todo esto por sí sola: consulte [Actualizar Rookparcel](Actualizar-Rookparcel).

### Instalación silenciosa

El instalador acepta estos parámetros, para scripts y administradores:

| Parámetro | Efecto |
|---|---|
| `/S` | silenciosa: sin páginas y sin iniciar nada al final |
| `/D=<carpeta>` | la carpeta de instalación; debe ser el último argumento, sin comillas |
| `/CURRENTUSER`, `/ALLUSERS` | para mí o para todos, sin la página de usuarios. `/ALLUSERS` sin derechos de administrador los pide mediante el aviso de Windows, también en modo silencioso |
| `/DESKTOPSHORTCUT` | crea el acceso directo del escritorio (una instalación silenciosa no crea ninguno en caso contrario) |
| `/ASSOCIATE=torrent,magnet` | las asociaciones: `torrent`, `magnet` o ambas |

Por ejemplo, desde un símbolo del sistema de administrador:

```
rookparcel-1.2.76-windows-x64-setup.exe /S /ALLUSERS /ASSOCIATE=torrent,magnet
```

### El zip

1. Descomprima `rookparcel-<version>-windows-x64.zip` en cualquier carpeta en la que pueda escribir.
2. Inicie `rookparcel.exe`. SmartScreen avisa la primera vez, como arriba: **Más información** y después **Ejecutar de todas formas**.

Los ajustes y los torrents van a `%APPDATA%\Rookparcel`, el mismo lugar que usa una copia instalada. Para guardarlos en cambio en la carpeta del zip, ponga un archivo vacío llamado `rookparcel.ini` junto a `rookparcel.exe` antes del primer inicio. Consulte [Modo portátil](Dónde-guarda-Rookparcel-sus-archivos#modo-portátil).

Para abrir archivos `.torrent` y enlaces `magnet:` con la copia del zip, marque las asociaciones en **Opciones > Preferencias > General**. Entonces nombran el `rookparcel.exe` de esta copia, así que dejan de funcionar si mueve o borra su carpeta.

### Comprobar una descarga

El `SHA256SUMS` de cada versión enumera el SHA-256 de cada archivo. En PowerShell:

```
Get-FileHash .\rookparcel-1.2.76-windows-x64-setup.exe -Algorithm SHA256
```

El hash debe coincidir con la línea del archivo en `SHA256SUMS`.

## Linux

### AppImage

1. Descargue `Rookparcel-<version>-x86_64.AppImage` (o `-aarch64` en ARM).
2. Hágalo ejecutable e inícielo:

   ```bash
   chmod +x Rookparcel-1.2.76-x86_64.AppImage
   ./Rookparcel-1.2.76-x86_64.AppImage
   ```

3. Si el sistema no tiene FUSE 2, inícielo añadiendo `--appimage-extract-and-run`.

El AppImage necesita glibc 2.39 o posterior (Ubuntu 24.04, Fedora 40, Debian 13 o posteriores). Los ajustes van a `~/.config/rookparcel/gtk` y los torrents a `~/.local/share/rookparcel/gtk`. Un archivo `rookparcel.ini` junto al `.AppImage` lo hace portátil (*nuevo en la 1.1.49*).

### Flatpak

El paquete contiene la aplicación; el runtime viene de Flathub. Añada Flathub una vez y después instale el paquete:

```bash
flatpak remote-add --if-not-exists --user flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install --user Rookparcel-1.2.76-x86_64.flatpak
flatpak run io.github.balls0fsteel.Rookparcel
```

Después aparece en el menú de aplicaciones. El Flatpak puede escribir en su carpeta personal; para una carpeta de descargas en otro disco, dele acceso una vez, por ejemplo `flatpak override --user --filesystem=/mnt/data io.github.balls0fsteel.Rookparcel` (con su carpeta). El Flatpak no tiene comprobación de actualizaciones propia ni modo portátil. Sus asociaciones de `.torrent` y `magnet:` las decide el escritorio: elija Rookparcel como programa predeterminado en la configuración de su escritorio.

### Binarios sueltos (tar.gz)

`rookparcel-<version>-linux-<arch>.tar.gz` contiene `rookparcel-gtk`, `rookparcel-nox`, la entrada de escritorio y las unidades de systemd (`systemd/system`, `systemd/user`). Los binarios usan el GTK 4 (4.12 o posterior) y el OpenSSL 3 del sistema, tal como los tiene Ubuntu 24.04, así que funcionan en distribuciones de esa edad o más recientes.

Descomprímalo e inicie `rookparcel-gtk`. Un archivo `rookparcel.ini` junto al binario lo hace portátil (*nuevo en la 1.1.49*). Para las unidades de systemd, consulte [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

### Servidor sin ventana

`rookparcel-nox-<version>-<arch>.AppImage` es solo el demonio, con OpenSSL incluido; no aparece en los menús. Consulte [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

## macOS

1. Abra `Rookparcel-<version>-macos.dmg` y arrastre `Rookparcel.app` a Aplicaciones.
2. La aplicación tiene firma ad hoc y no está notarizada, así que Gatekeeper la rechaza la primera vez. Permítala una vez:
   - **macOS 15 Sequoia y posteriores:** haga doble clic en la aplicación y haga clic en **Listo** en el aviso. Abra **Ajustes del Sistema > Privacidad y seguridad**, desplácese hasta **Seguridad** y haga clic en **Abrir de todas formas** (el botón está ahí durante aproximadamente una hora). Confirme con **Abrir de todas formas** y su contraseña o Touch ID.
   - **macOS 11 a 14:** haga clic con Control pulsada (o clic derecho) en la aplicación, elija **Abrir** y haga clic en **Abrir**.
3. O bien, también cuando macOS dice que la aplicación «está dañada», quite la marca de cuarentena en Terminal:

   ```bash
   xattr -dr com.apple.quarantine /Applications/Rookparcel.app
   ```

La imagen de disco tiene estos pasos en `How to open Rookparcel.html`, y la [guía de instalación](https://balls0fsteel.github.io/Rookparcel/install.html#macos) del sitio web los muestra con imágenes.

La aplicación funciona en macOS 11 Big Sur o posterior, tanto en Mac con Apple silicon como con Intel: el único `Rookparcel.app` contiene ambas versiones. Los ajustes y los torrents van a `~/Library/Application Support/Rookparcel/mac`.

### Actualizar desde la versión GTK

Las versiones anteriores tenían una versión GTK de `Rookparcel.app` para Apple silicon. Reemplácela por el nuevo `Rookparcel.app` en Aplicaciones. En su primer inicio, la nueva aplicación renombra la carpeta `gtk` de la aplicación anterior en `~/Library/Application Support/Rookparcel` a `mac`, y la de cada perfil también, de modo que sus ajustes, torrents y perfiles se conservan. Esto ocurre una sola vez, y solo mientras todavía no exista una carpeta `mac`. Cierre primero la aplicación anterior: mientras se ejecuta, la nueva dice «Otro Rookparcel ya está usando esta carpeta» y se cierra. Consulte [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos#macos).

Una entrada «Iniciar Rookparcel al arrancar el sistema» creada por la aplicación anterior sigue funcionando. Una carpeta portátil se usa tal como está.

Para el demonio sin ventana, descomprima `rookparcel-nox-<version>-macos-<arch>.tar.gz` y ejecute `rookparcel-nox` desde Terminal ([Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox)).

## Desinstalación

### Instalador de Windows

1. Cierre Rookparcel (**Archivo > Salir**).
2. Abra **Configuración > Aplicaciones**, busque **Rookparcel** y elija **Desinstalar**.
3. El desinstalador tiene una opción, **Configuración y lista de torrents**, sin marcar de forma predeterminada. Márquela para quitar también lo que Rookparcel guarda en `%APPDATA%\Rookparcel`: los ajustes, la lista de torrents y su progreso, los canales RSS, las conexiones remotas, el filtro de IP predeterminado, los archivos de nox y los perfiles.

El desinstalador quita las asociaciones, la entrada de Aplicaciones predeterminadas y la entrada «Iniciar Rookparcel al arrancar el sistema» que nombran el `rookparcel.exe` de esta instalación. Las de otra copia, por ejemplo una portátil, se quedan. Nunca toca los archivos descargados.

Si el servicio Rookparcel nox se ejecuta desde la carpeta de instalación, el desinstalador le pide que lo quite primero. Desde un símbolo del sistema de administrador:

```
"%LOCALAPPDATA%\Programs\Rookparcel\rookparcel-nox.exe" service uninstall --all
```

(use su propia carpeta de instalación). Una desinstalación silenciosa: `"%LOCALAPPDATA%\Programs\Rookparcel\uninstall.exe" /S`.

Los ajustes pertenecen a cada usuario de Windows. Desinstalar una instalación para todos los limpia solo para el usuario que ejecuta el desinstalador.

### Zip de Windows

1. Si marcó asociaciones o «Iniciar Rookparcel al arrancar el sistema», desmárquelas antes en **Opciones > Preferencias > General**.
2. Cierre Rookparcel y borre su carpeta.
3. Borre también `%APPDATA%\Rookparcel`, a menos que la copia fuera portátil (entonces todo estaba en su carpeta).

### Linux y macOS

- **AppImage o tar.gz:** borre el archivo o la carpeta. Para quitar los ajustes y la lista de torrents, borre `~/.config/rookparcel` y `~/.local/share/rookparcel`.
- **Flatpak:** `flatpak uninstall --user io.github.balls0fsteel.Rookparcel`.
- **macOS:** arrastre `Rookparcel.app` a la Papelera. Los ajustes y la lista de torrents están en `~/Library/Application Support/Rookparcel`.

Los archivos descargados se quedan donde están en todos los casos. Para la lista completa de carpetas, consulte [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos).
