*Esta página es una traducción de la página en inglés [Updating Rookparcel](Updating-Rookparcel). Si hay diferencias, la página en inglés es la válida.*

# Actualizar Rookparcel

Rookparcel busca versiones nuevas por sí mismo y le avisa cuando hay una. Esta página explica la comprobación, cómo se actualiza solo una copia instalada en Windows, cómo actualizar a mano las copias del zip, de Linux y de macOS, las versiones beta, cómo desactivar la comprobación y `rookparcel-nox`.

## La comprobación de actualizaciones

Unos diez segundos después de iniciarse, y después cada 12 horas mientras se ejecuta, Rookparcel pregunta al sitio web del proyecto cuál es la versión más reciente. Es una pequeña petición HTTPS que obtiene una lista corta de números de versión. No se envía nada sobre su equipo, sus torrents ni sus ajustes, y no se descarga ni se instala nada sin su confirmación. Si Rookparcel usa un proxy (**Opciones > Preferencias > Conexión**), la comprobación también pasa por él.

Cuando hay una versión más reciente:

- Aparece un cuadro de pregunta, una vez por cada versión nueva.
- Después, un aviso permanece en la barra de estado hasta que haga clic en él: «Rookparcel 1.2.79 está disponible - haga clic aquí para descargarlo».
- La pestaña Registro tiene también una línea con el enlace.

Para comprobar al instante, elija **Ayuda > Buscar actualizaciones** (en macOS **Rookparcel > Buscar actualizaciones**). Responde en cualquier caso, con la versión más reciente o con «Tiene la versión más reciente».

## Una copia instalada en Windows se actualiza sola

Un Rookparcel instalado con el instalador de Windows puede instalar la versión nueva por sí mismo. *Nuevo en la 1.1.40.*

1. La pregunta dice «Rookparcel 1.2.79 está disponible. Tiene la versión 1.2.77. ¿Descargarlo e instalarlo ahora?». Haga clic en **Descargar e instalar**. (**Abrir la página de descarga** abre en cambio la página de la versión en su navegador; **Cancelar** no hace nada.)
2. Una ventana pequeña muestra el progreso de la descarga. Rookparcel comprueba el instalador descargado con el `SHA256SUMS` de la versión antes de usarlo.
3. Rookparcel pregunta: «Para instalarlo, Rookparcel se cierra ahora y se inicia de nuevo cuando termine el instalador. ¿Cerrar e instalar ahora?». Haga clic en **Sí**.
4. Rookparcel guarda todo y se cierra. El instalador instala la versión nueva en silencio en la misma carpeta y vuelve a iniciar Rookparcel.

Sus ajustes, torrents y asociaciones se quedan como estaban. Si la instalación falla, la versión anterior se conserva y se inicia de nuevo.

En una instalación para todos los usuarios (en `C:\Program Files`), Windows pide derechos de administrador mientras se ejecuta el instalador.

**Actualizar desde la 1.1.17 o anteriores:** esas versiones no pueden instalar actualizaciones; su pregunta abre la página de descarga. Descargue el instalador nuevo y ejecútelo una vez a mano, como en [Instalar Rookparcel](Instalar-Rookparcel#instalar-sobre-una-versión-anterior). A partir de ahí, Rookparcel se actualiza solo.

## Actualizar a mano

### Instalador de Windows

Descargue el nuevo `rookparcel-<version>-windows-x64-setup.exe` y ejecútelo. Encuentra el Rookparcel instalado e instala encima, conservando la carpeta, el modo (para usted o para todos), los ajustes y los torrents. Cierre Rookparcel antes, o el instalador se lo pedirá. Consulte [Instalar Rookparcel](Instalar-Rookparcel#instalar-sobre-una-versión-anterior).

### Zip de Windows

Una copia del zip no se actualiza sola. Su pregunta ofrece abrir la página de descarga.

1. Descargue el zip nuevo.
2. Cierre Rookparcel (**Archivo > Salir**).
3. Descomprima los nuevos `rookparcel.exe` y `rookparcel-nox.exe` sobre los antiguos.
4. Inicie Rookparcel.

Conserve `rookparcel.ini` en la carpeta si la copia es portátil. Los ajustes y los torrents no están en el zip, así que no se sobrescribe nada suyo.

### Linux y macOS

En la versión GTK y en la aplicación de macOS, la pregunta abre la página de la versión.

- **AppImage:** descargue el nuevo `.AppImage`, hágalo ejecutable (`chmod +x`) e inícielo en lugar del antiguo. Borre el archivo antiguo. Los ajustes y los torrents no están dentro del AppImage, así que se conservan. Para un AppImage portátil, ponga el archivo nuevo junto a `rookparcel.ini`.
- **tar.gz:** cierre Rookparcel y descomprima los nuevos binarios sobre los antiguos.
- **Flatpak:** el Flatpak no tiene comprobación de actualizaciones. Descargue el paquete más reciente de la [página de versiones publicadas](https://github.com/Balls0fSteel/Rookparcel/releases) e instálelo como en [Instalar Rookparcel](Instalar-Rookparcel#flatpak).
- **macOS:** cierre Rookparcel (**Rookparcel > Salir de Rookparcel**), arrastre el nuevo `Rookparcel.app` desde `Rookparcel-<version>-macos.dmg` a Aplicaciones y reemplace el antiguo. Si Gatekeeper rechaza la copia nueva, ábrala como en [Instalar Rookparcel](Instalar-Rookparcel#macos). El mismo `Rookparcel.app` funciona en Mac con Apple silicon y con Intel.
- **macOS, desde la versión GTK:** las versiones anteriores tenían un `Rookparcel.app` GTK para Apple silicon. Reemplácelo de la misma manera; en su primer inicio, la aplicación nueva toma los ajustes y torrents de la antigua. Consulte [Instalar Rookparcel](Instalar-Rookparcel#actualizar-desde-la-versión-gtk).

## Versiones beta

Rookparcel publica a veces una versión preliminar (una versión candidata) antes de una versión. Para que también se le ofrezcan, marque **Actualizar a versiones beta** en **Opciones > Preferencias > General**, en **Privacidad**. Está desactivado de forma predeterminada.

Una versión preliminar siempre se entera de la siguiente versión preliminar, diga lo que diga la casilla.

## Desactivar la comprobación

Desmarque **Buscar actualizaciones automáticamente** en **Opciones > Preferencias > General**, en **Privacidad**. Rookparcel deja entonces de preguntar al sitio web por sí mismo. **Ayuda > Buscar actualizaciones** sigue funcionando cuando usted la elige.

Algunas compilaciones no tienen ninguna comprobación de actualizaciones: el Flatpak, y los paquetes creados para un gestor de paquetes que se encarga de actualizar.

## rookparcel-nox

`rookparcel-nox` comprueba de la misma manera, diez segundos después de que empiece `run` y cada 12 horas. *Nuevo en la 1.1.17.* Nunca descarga ni instala nada. Cuando hay una versión más reciente:

- imprime una línea, por ejemplo `* Rookparcel 1.2.0 is available: https://github.com/Balls0fSteel/Rookparcel/releases/tag/v1.2.0`;
- su interfaz web muestra la línea en su Registro y un enlace en la barra de estado de la página.

Para desactivarlo, inícielo con `--no-update-check`, o desmarque **Buscar actualizaciones automáticamente** en **General > Privacidad** de la interfaz web. **Actualizar a versiones beta** está ahí también.

Para actualizar el demonio, deténgalo, reemplace `rookparcel-nox` por el nuevo e inícielo de nuevo. Para el servicio de Windows, deténgalo primero desde un símbolo del sistema de administrador (`sc stop RookparcelNox`) e inícielo de nuevo después; el instalador de Windows lo pide cuando encuentra el servicio en ejecución. Consulte [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

## Después de actualizar

- **Control remoto:** cuando una versión cambia el protocolo remoto, una ventana y el `rookparcel-nox` que controla deben estar ambos en esa versión. Actualice los dos. Las notas de la versión indican cuándo se aplica ([Control remoto](Control-remoto)).
- **Volver a una versión anterior:** los ajustes y los torrents se conservan entre versiones 1.1.x. La 1.1.40 guarda los datos de reanudación de una forma más compacta, así que al volver desde ella a una versión anterior, cada torrent se comprueba de nuevo una vez. Una [carpeta portátil](Dónde-guarda-Rookparcel-sus-archivos#modo-portátil) usada por la 1.1.49 o posterior necesita la 1.1.49 o posterior.
- Qué cambió en cada versión: las notas de la versión en la [página de versiones publicadas](https://github.com/Balls0fSteel/Rookparcel/releases), o la página de [Noticias](https://balls0fsteel.github.io/Rookparcel/news.html) del sitio web.

## Cuando una versión lee sus ajustes de forma distinta

*Nuevo en la 1.2.57.*

Cada vez que Rookparcel guarda sus ajustes, anota también su versión y cómo leyó los ajustes que deciden hacia dónde va su tráfico: el proxy, **Usar solo esta red**, las interfaces de escucha, el cifrado y el filtro de IP. Cuando se inicia otra versión, compara esa nota con su propia lectura del mismo archivo:

- Primero guarda una copia del archivo tal como lo encontró, junto a él: `rookparcel.ini.<versión anterior>.bak` (`webui.json.<versión anterior>.bak` para `rookparcel-nox`). Se conservan las tres últimas.
- Cada ajuste que se lee de forma distinta va a la pestaña **Registro**, por ejemplo `proxy: socks5 10.8.0.1:1080 peers trackers names → none`.
- Si se perdieran el proxy, la regla de red o las interfaces de escucha, no se conecta nada: la barra de estado dice **Transferencias retenidas: confirme las preferencias**, y los torrents esperan. Revise **Opciones > Preferencias > Conexión** y pulse **Aceptar**, y las transferencias empiezan. En la interfaz web, guarde su página de Preferencias. Los canales RSS, los enlaces `.torrent` y la comprobación de actualizaciones también esperan. Para volver atrás en su lugar, cierre Rookparcel y copie el archivo `.bak` sobre el archivo de ajustes.

Las transferencias se retienen de la misma manera cuando el archivo de ajustes no se puede leer en absoluto (entonces nunca se sobrescribe), cuando falta o está vacío habiendo torrents guardados, o cuando esta versión no puede leer el tipo de proxy que escribió una versión más reciente; esa línea se queda en el archivo para la versión más reciente. Un archivo de ajustes que no es texto válido (guardado por un editor con otra codificación) se lee lo mejor posible, y el original se conserva como `rookparcel.ini.damaged-<número>`. `rookparcel-nox` no se inicia con un `webui.json` que no puede leer; nombra el archivo, para que pueda corregirlo o apartarlo.
