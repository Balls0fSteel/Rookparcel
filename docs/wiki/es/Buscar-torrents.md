*Esta es una traducción de la página inglesa [Searching for torrents](https://github.com/Balls0fSteel/Rookparcel/wiki/Searching-for-torrents). Si hay diferencias, prevalece la página inglesa.*

# Buscar torrents

Esta página explica cómo encontrar torrents nuevos desde Rookparcel: la diferencia entre filtrar su propia lista y buscar, cómo añadir fuentes de búsqueda (el asistente de búsqueda, o su propio Prowlarr o Jackett) y cómo añadir lo que encuentra una búsqueda. Abarca la ventana de Windows, la ventana GTK (Linux, BSD) y la interfaz web.

Para un recorrido paso a paso de la versión de Windows con capturas de pantalla, consulte [Buscar torrents en Windows](Buscar-torrents-en-Windows).

## Cómo funciona la búsqueda

Rookparcel no tiene un motor de búsqueda propio ni extrae datos de los sitios por sí mismo. Consulta una *fuente de búsqueda*: un programa que sabe buscar en sitios de torrents y responde en un formato estándar llamado Torznab. Dos programas gratuitos hacen esto:

- **Jackett** y **Prowlarr**, que quizá ya use para Sonarr o Radarr, en este equipo, en un NAS o en un servidor.
- El **asistente de búsqueda**, que es Jackett descargado y ejecutado por Rookparcel por usted. Es la vía fácil si no tiene ninguno de los dos.

Una búsqueda va a todas las fuentes que elija al mismo tiempo. Las respuestas se combinan, los duplicados (el mismo torrent de dos fuentes) se muestran una sola vez, y la lista se ordena por semillas.

*El cuadro de búsqueda y Administrar fuentes están en la versión 1.1.40. El asistente de búsqueda es nuevo en 1.1.59.*

## Filtrar la lista frente a buscar

El cuadro de búsqueda, arriba a la derecha en la ventana principal, tiene dos funciones:

| Modo | Qué hace Intro |
|---|---|
| Filtro (el predeterminado) | Al escribir, su propia lista de torrents se reduce a los nombres que contienen el texto. No se busca nada en línea. |
| Una fuente, o **Todas las fuentes** | Al escribir no se filtra la lista. Intro envía el texto a la fuente elegida (o a todas) y abre la ventana **Buscar torrents** con los resultados. |

<kbd>Ctrl</kbd>+<kbd>F</kbd> coloca el cursor en el cuadro en ambas interfaces. La indicación gris del cuadro vacío dice en qué modo está.

### Windows

La pequeña flecha junto al cuadro abre un menú (<kbd>Alt</kbd>+<kbd>Down</kbd> en el cuadro también lo abre):

- **Filtrar torrents**: el filtro de la lista. La indicación dice «Buscar nombres de torrents».
- Cada fuente de búsqueda activada, por su nombre. La indicación dice «Buscar en» y el nombre de la fuente.
- **Todas las fuentes**. La indicación dice «Buscar en todas las fuentes».
- **Administrar fuentes...**, que abre la ventana Fuentes de búsqueda.

La elección se recuerda entre inicios. Si se quita o se desactiva la fuente elegida, el cuadro vuelve a **Filtrar torrents**.

### GTK

El botón de menú junto al cuadro tiene las mismas entradas, salvo que la entrada del filtro se llama **Buscar nombres de torrents**. Con una fuente elegida, la indicación cambia a «Buscar con Prowlarr o Jackett».

### macOS

El menú del campo de búsqueda tiene las mismas entradas que el de GTK: **Buscar nombres de torrents**, cada fuente activada, **Todas las fuentes** y **Administrar fuentes...**.

### Interfaz web

El cuadro de búsqueda de la interfaz web solo filtra la lista. La búsqueda tiene su propio botón, **Buscar**, junto a él (en un teléfono, en el menú **Más** de la barra de herramientas, como **Buscar torrents**). Buscar abre la ventana Buscar torrents con el texto del cuadro de filtro ya escrito.

## Administrar fuentes

Las fuentes se guardan en la ventana **Fuentes de búsqueda**:

- Windows: la flecha junto al cuadro de búsqueda, y después **Administrar fuentes...**.
- GTK: el botón de menú junto al cuadro de búsqueda, y después **Administrar fuentes...**, o el botón **Administrar fuentes...** de la ventana Buscar torrents.
- Interfaz web: **Buscar**, y después **Administrar fuentes...** en la ventana Buscar torrents.

![La ventana Fuentes de búsqueda](images/search-sources.png)

La ventana muestra sus fuentes con una casilla cada una. Una fuente marcada se consulta cuando busca en **Todas las fuentes**; una sin marcar se conserva pero se deja fuera. Los botones:

| Botón | Qué hace |
|---|---|
| **Añadir...** | Añade una fuente propia (véase más abajo). |
| **Editar...** | Cambia el nombre, la URL, la clave de API o la casilla de la fuente seleccionada. |
| **Quitar** | Quita la fuente seleccionada, después de preguntar. |
| **Probar** | Pide a la fuente su lista de categorías. «OK» y un número de categorías significa que la URL y la clave de API funcionan; si no, la línea bajo la lista muestra el error. |

En las ventanas de Windows y GTK, aquí también están los botones del asistente de búsqueda; consulte [El asistente de búsqueda](#el-asistente-de-búsqueda).

Mientras no tenga fuentes, una búsqueda muestra «Aún no hay fuentes de búsqueda: añada su Prowlarr o Jackett en Administrar fuentes.»

## Añadir su propio Prowlarr o Jackett

Si ya usa Prowlarr o Jackett, añádalo como fuente:

1. Abra **Fuentes de búsqueda** y haga clic en **Añadir...**. Se abre la ventana **Añadir fuente de búsqueda**.
2. **Nombre:** lo que quiera. Si se deja vacío, la fuente se muestra con el nombre de su servidor.
3. **URL de Torznab:** la dirección del canal Torznab de la fuente:
   - Jackett: `http://host:9117/api/v2.0/indexers/all/results/torznab/`. La parte `all` busca en todos los indexadores que haya configurado en Jackett, así que una sola fuente basta.
   - Prowlarr: `http://host:9696/1/api`, donde `1` es el número del indexador. Prowlarr no tiene un canal de «todos los indexadores», así que añada una fuente por cada indexador.

   Sustituya `host` por `localhost` si se ejecuta en el mismo equipo, o por la dirección del equipo en su red.
4. **Clave de API:** la clave de la página de Jackett, o de **Settings > General** de Prowlarr.
5. Deje marcada **Buscar en esta fuente** y haga clic en **Aceptar**.
6. Seleccione la nueva fuente y haga clic en **Probar**. Debe decir «OK» y un número de categorías.

La URL debe empezar por `http://` o `https://`.

**La clave de API** se guarda cifrada (en Windows) y no se vuelve a mostrar. Al editar una fuente, el cuadro de la clave empieza vacío con la indicación gris «Sin cambios»; déjelo vacío para conservar la clave guardada, o escriba una nueva. La clave se elimina de cualquier dirección que aparezca en un error o en una línea de estado.

**Proxy.** A una fuente de este equipo o de su red local se accede directamente: una dirección IP o `localhost` que no sea pública, o un nombre de servidor cuyas direcciones lo sean todas. Una fuente en una dirección pública pasa por el proxy definido en Preferencias, si definió uno. Si el proxy resuelve por sí mismo los nombres de servidor (un proxy SOCKS configurado para enviar nombres de servidor, o un proxy HTTP para una dirección `http://` simple), Rookparcel no resuelve antes un nombre en este equipo, así que una fuente dada por un nombre de servidor distinto de `localhost` también pasa por el proxy; indique la dirección IP de la fuente para mantenerla directa. Consulte [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos).

### En la interfaz web

Las ventanas Añadir y Editar de la interfaz web tienen un campo más, **Contraseña actual de la interfaz web:**. Añadir una fuente, o cambiar la URL o la clave de API de una fuente, requiere la contraseña actual de la interfaz web. Cambiar el nombre de una fuente, activarla o desactivarla, quitarla, probarla y buscar no la requieren. Una contraseña incorrecta cierra su sesión, como en el resto de la interfaz web.

La interfaz web nunca muestra una clave de API guardada, y nunca envía al navegador los enlaces de los resultados (que llevan la clave del indexador).

## El asistente de búsqueda

El asistente de búsqueda es [Jackett](https://github.com/Jackett/Jackett), un programa gratuito independiente de sus propios autores, con licencia GPL-2.0. Rookparcel descarga la versión propia de Jackett desde GitHub cuando usted hace clic en el botón, la comprueba con la suma de verificación que publica GitHub, la guarda en una carpeta `search-helper` de su perfil y la ejecuta solo mientras usted busca. No se descarga ni se ejecuta nada antes de que haga clic.

*Nuevo en 1.1.59.*

### Dónde se ofrece

| Dónde | Asistente de búsqueda |
|---|---|
| Ventana de Windows (x64 y ARM64) | Sí |
| Ventana GTK en Linux (x86-64 y ARM64) | Sí |
| Aplicación de macOS (Intel y Apple silicon) | Sí |
| Flatpak | No |
| FreeBSD | No (Jackett no tiene compilación para él) |
| Compilaciones hechas por un administrador de paquetes sin la comprobación de actualizaciones | No |
| Interfaz web y `rookparcel-nox` | Sí, cuando el servidor es Windows, Linux o macOS (no FreeBSD, no un Flatpak). *Nuevo en 1.1.67.* |

Donde no se ofrece, la ventana Fuentes de búsqueda no tiene los botones del asistente, y **Añadir...** para su propio Prowlarr o Jackett sigue funcionando.

### Obtenerlo

1. Abra **Fuentes de búsqueda** y haga clic en **Obtener el asistente de búsqueda (Jackett)...**.
2. Rookparcel busca la versión más reciente de Jackett y pregunta antes de descargar. La pregunta indica la versión, el tamaño (unos 50 MB) y la carpeta donde se guardará. Haga clic en **Sí**.
3. Una ventana de progreso muestra la descarga. Después Rookparcel comprueba el archivo, lo descomprime y lo inicia una vez para leer su lista de sitios.
4. Se abre la ventana **Sitios del asistente de búsqueda**. No se marca nada por usted. Marque los sitios públicos en los que quiere buscar y haga clic en **Aceptar**. Los sitios marcados «Necesita FlareSolverr, que el asistente no incluye» aparecen atenuados y no se pueden marcar.
5. De vuelta en Fuentes de búsqueda, el asistente aparece como **Asistente de búsqueda (todos)**. La línea inferior indica cuántos sitios están marcados.

Después elija **Asistente de búsqueda (todos)** (o **Todas las fuentes**) en el menú del cuadro de búsqueda y busque.

Dos o tres sitios bastan para empezar. Cada sitio marcado se consulta en cada búsqueda, así que más sitios hacen las búsquedas más lentas y añaden duplicados.

Una vez instalado el asistente, el botón **Obtener** se sustituye por otros cuatro:

| Botón | Qué hace |
|---|---|
| **Elegir sitios...** | Vuelve a abrir la ventana Sitios del asistente de búsqueda, para marcar o desmarcar sitios. |
| **Abrir la página del asistente** | Abre la página propia de Jackett en su navegador y copia su contraseña al portapapeles (la página la pide). Úsela para añadir sitios privados con su inicio de sesión; después se buscan junto con los marcados. |
| **Actualizar el asistente de búsqueda** | Busca un Jackett más reciente y pregunta antes de descargarlo. Sus sitios y ajustes se conservan. Nada se actualiza por sí solo. |
| **Quitar el asistente de búsqueda** | Pregunta, y después detiene el asistente, borra su carpeta y quita su fuente de la lista. |

### Cómo se ejecuta

- Se inicia en su primera búsqueda en su fuente y se detiene solo diez minutos después de la última. También se detiene cuando se cierra Rookparcel. Nunca se inicia por sí solo ni al arrancar el equipo.
- Solo escucha en este equipo (`127.0.0.1`), nunca en su red.
- No necesita derechos de administrador.
- Si define un proxy en Preferencias, el asistente lo usa para los sitios. «Usar solo esta red» no se aplica al asistente, porque es un programa independiente: sus solicitudes toman la ruta normal del equipo.

### En la interfaz web y `rookparcel-nox`

**Administrar fuentes** en la interfaz web tiene los mismos botones y la misma lista de sitios. El asistente se descarga y se ejecuta en el servidor, no en el equipo del navegador, y con `rookparcel-nox --no-state` no se ofrece. Cada paso que tarda muestra su progreso, y la descarga tiene un botón **Cancelar**. Descargarlo requiere la contraseña actual de la interfaz web, igual que añadir una fuente. La página propia del asistente escucha en el `127.0.0.1` del servidor, así que **Abrir la página del asistente** se muestra solo a un navegador del equipo donde se ejecuta Rookparcel; en otros casos, la ventana lo indica. Una ventana conectada con **Archivo > Conectar en remoto...** usa su propio asistente, en este equipo.

### Dónde se guarda

La carpeta `search-helper` dentro de la carpeta de su perfil:

| Sistema | Carpeta |
|---|---|
| Windows | `%APPDATA%\Rookparcel\search-helper`; en modo portátil, la carpeta con `rookparcel.exe`; con un perfil, la carpeta de ese perfil |
| Linux (GTK) | `~/.local/share/rookparcel/gtk/search-helper` (con un perfil, bajo `profiles/<nombre>/`) |
| macOS | `~/Library/Application Support/Rookparcel/mac/search-helper` (con un perfil, bajo `profiles/<nombre>/`) |
| `rookparcel-nox` | `search-helper` en su carpeta de estado: `%APPDATA%\Rookparcel\nox\search-helper`, `~/.local/share/rookparcel/nox/search-helper`, o `/var/lib/rookparcel-nox/search-helper` con el servicio del sistema que se entrega |

Cada perfil tiene su propio asistente. Consulte [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos) y [Perfiles](Perfiles).

## La ventana Buscar torrents

La ventana se abre al pulsar Intro en el cuadro de búsqueda con una fuente elegida (Windows, GTK), o al hacer clic en **Buscar** (interfaz web). Tiene:

- **La consulta** y **Buscar**, para volver a buscar desde la ventana.
- **Una categoría**: **Todas las categorías**, **Películas**, **Audio**, **PC**, **TV**, **Libros** u **Otros**.
- **Una fuente**: **Todas las fuentes**, o una fuente.
- **Detener**, para detener las fuentes que aún están buscando. Los resultados obtenidos hasta entonces se conservan.
- **Una línea de estado por fuente**: «buscando...», el número de resultados, o el error. Que una fuente falle no detiene a las demás.
- **Los resultados**, con las columnas **Nombre**, **Tamaño**, **Semillas**, **Pares**, **Categoría**, **Antigüedad** y **Fuente**. Haga clic en el encabezado de una columna para ordenar; la lista empieza ordenada por semillas. Cuando dos fuentes encuentran el mismo torrent, aparece una sola vez, y **Fuente** nombra a ambas.
- El número de resultados, **Añadir** y **Cerrar**.

En un teléfono, los resultados de la interfaz web muestran solo Nombre, Tamaño y Semillas.

Cerrar la ventana detiene la búsqueda (en Windows, <kbd>Esc</kbd> también la cierra).

## Añadir un resultado

Seleccione un resultado y haga clic en **Añadir**, haga doble clic en él o pulse <kbd>Enter</kbd> en la lista.

Rookparcel obtiene el resultado de su fuente (un archivo `.torrent` o un enlace magnet) y lo añade del mismo modo que se añade un archivo `.torrent` abierto o un enlace magnet pulsado. Si **Mostrar una ventana con los archivos del torrent** está activado en **Opciones > Preferencias > Interfaz**, primero se abre la ventana Añadir nuevo torrent. Consulte [Añadir torrents](Añadir-torrents).

La línea inferior dice «Añadido:» y el nombre del torrent. La ventana GTK y la interfaz web rechazan un resultado que ya está en su lista de torrents.

En la interfaz web, el torrent se añade en el servidor.

## Buscar desde una ventana remota o la interfaz web

- **Una ventana remota** (Archivo > Conectar en remoto, consulte [Control remoto](Control-remoto)) busca desde este equipo, con su propia lista de fuentes, y añade lo que elija al servidor remoto.
- **La interfaz web de `rookparcel-nox`** guarda sus propias fuentes en el servidor. Las búsquedas y las descargas se ejecutan en el servidor; todas las páginas abiertas ven la misma búsqueda.
- **La interfaz web que aloja una ventana de Windows** usa las fuentes de esa ventana: una fuente añadida en la ventana aparece en la página y a la inversa. Consulte [Interfaz web](Interfaz-web).

La interfaz web también responde a la parte de búsqueda de la API web de qBittorrent, de modo que la pestaña de búsqueda de una aplicación remota de qBittorrent también busca en sus fuentes.

## Si algo sale mal

| Lo que ve | Qué significa |
|---|---|
| La ventana de resultados no se abre nunca | El cuadro está en modo de filtro. Elija antes una fuente o **Todas las fuentes** en su menú. |
| «Aún no hay fuentes de búsqueda: ...» | Añada una fuente en Administrar fuentes, u obtenga el asistente de búsqueda. |
| La línea de una fuente muestra un error | Esa fuente falló; las demás siguen respondiendo. Haga clic en **Probar** en Fuentes de búsqueda para comprobar su URL y su clave de API. |
| Probar muestra `HTTP 401` o un error sobre la clave de API | La clave de API es incorrecta. Edite la fuente y escriba la clave de nuevo. |
| El asistente de búsqueda no encuentra nada | No hay sitios marcados, o los sitios marcados están caídos o han cambiado. **Elegir sitios...** muestra lo que está marcado; pruebe otro sitio. |
| «No se pudo obtener el asistente de búsqueda: ...» | No se pudo acceder a GitHub o se interrumpió la descarga. Inténtelo de nuevo. |
| «El asistente de búsqueda no se inició: ...» | No se pudo ejecutar el programa de la carpeta `search-helper`: una descarga dañada, o un antivirus que eliminó archivos. Quite el asistente y vuelva a obtenerlo. |

Más ayuda: [Solución de problemas y preguntas frecuentes](Solución-de-problemas-y-preguntas-frecuentes). Para descargas automáticas de episodios nuevos desde canales, consulte [Descargador RSS](Descargador-RSS). Para usar Rookparcel como cliente de descarga de Prowlarr, consulte [Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr).
