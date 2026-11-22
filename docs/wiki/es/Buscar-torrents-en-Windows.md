*Esta es una traducción de la página inglesa [Searching for torrents on Windows](https://github.com/Balls0fSteel/Rookparcel/wiki/Searching-for-torrents-on-Windows). Si hay diferencias, prevalece la página inglesa.*

# Buscar torrents en Windows

Una guía breve del cuadro de búsqueda de la versión de Windows de Rookparcel: cómo obtener el asistente de búsqueda, elegir los sitios en los que busca y añadir lo que encuentra. Diez minutos y una descarga de 55 MB, que se hace una sola vez.

**Versión:** el asistente de búsqueda está en Rookparcel 1.1.59 y posteriores. La versión 1.1.40 tiene el cuadro de búsqueda y Administrar fuentes, pero no el botón «Obtener el asistente de búsqueda (Jackett)»; allí se aplica la última sección (su propio Prowlarr o Jackett).

## Qué obtiene

Rookparcel no tiene un motor de búsqueda propio. Consulta una *fuente de búsqueda*: un pequeño programa que sabe buscar en sitios de torrents. La fuente más fácil es el **asistente de búsqueda**, que es [Jackett](https://github.com/Jackett/Jackett), un programa gratuito independiente de sus propios autores. Rookparcel lo descarga por usted con un clic, lo guarda en su propia carpeta y lo ejecuta solo mientras usted busca. Usted elige en qué sitios públicos busca. No se descarga ni se ejecuta nada hasta que hace clic en el botón.

Si ya usa Prowlarr o Jackett por su cuenta, puede usarlo en su lugar: consulte la última sección.

## 1. Abrir Administrar fuentes

El cuadro de búsqueda está arriba a la derecha en la ventana principal. La pequeña flecha que hay junto a él abre un menú. Elija **Administrar fuentes...**.

![El menú desplegable del cuadro de búsqueda con Administrar fuentes](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-dropdown.png)

Se abre la ventana Fuentes de búsqueda. Al principio está vacía.

![La ventana Fuentes de búsqueda con el botón Obtener el asistente de búsqueda](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-sources.png)

## 2. Obtener el asistente de búsqueda

Haga clic en **Obtener el asistente de búsqueda (Jackett)...**. Rookparcel busca la versión más reciente de Jackett y pregunta antes de descargar nada:

> El asistente de búsqueda es Jackett, un programa independiente de sus propios autores con licencia GPL-2.0, descargado de github.com/Jackett/Jackett (v0.24.3009, 54.5 MB).
>
> Se guarda en C:\Users\you\AppData\Roaming\Rookparcel\search-helper y solo se ejecuta en este equipo, y solo mientras busca. Después elige los sitios públicos en los que busca.
>
> ¿Descargarlo ahora?

Haga clic en **Sí**. La descarga tarda desde unos segundos hasta un minuto, según su conexión.

![La ventana de progreso de la descarga](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-download.png)

Rookparcel comprueba el archivo con la suma de verificación de la versión, lo descomprime y lo inicia una vez para leer su lista de sitios. No aparece ningún aviso de Windows: nada necesita derechos de administrador, y el asistente escucha solo en este equipo, de modo que el cortafuegos no tiene nada que preguntar.

## 3. Marcar los sitios en los que buscar

La lista de sitios se abre sola (más adelante, es el botón **Elegir sitios...**). No se marca nada por usted. Marque los sitios públicos en los que quiere buscar y haga clic en **Aceptar**.

![La ventana Sitios del asistente de búsqueda con The Pirate Bay marcado](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-sites.png)

- Las filas atenuadas dicen «Necesita FlareSolverr, que el asistente no incluye». Esos sitios están tras una comprobación antibots que el asistente no puede superar por sí solo; déjelos.
- Dos o tres sitios bastan para empezar. Cada sitio marcado se consulta en cada búsqueda, así que más sitios significan búsquedas más lentas y más duplicados.
- Los sitios privados necesitan sus inicios de sesión. La página propia del asistente se ocupa de ellos: consulte «Sitios privados» más abajo.

De vuelta en Fuentes de búsqueda, el asistente aparece ahora como **Asistente de búsqueda (todos)** con sus botones debajo, y la línea inferior indica cuántos sitios están marcados. Haga clic en **Cerrar**.

![Fuentes de búsqueda con el asistente instalado y dos sitios marcados](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-sources-installed.png)

## 4. Buscar

Abra de nuevo el menú de la flecha y elija **Asistente de búsqueda (todos)**. La indicación del cuadro de búsqueda cambia en consecuencia. (La primera entrada, **Filtrar torrents**, es la otra función del cuadro: reducir su propia lista de torrents por nombre. **Todas las fuentes** consulta a la vez todas las fuentes que tenga.)

![El menú desplegable con Asistente de búsqueda (todos) elegido](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-dropdown-helper.png)

Escriba lo que busca y pulse **Intro**. Se abre la ventana Buscar torrents y los resultados llegan en pocos segundos.

![La ventana Buscar torrents con resultados para «ubuntu»](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-results.png)

- Haga clic en el encabezado de una columna para ordenar: Semillas de mayor a menor encuentra los más sanos.
- El cuadro de categoría reduce la búsqueda a Películas, TV, Audio, PC, Libros u Otros. El cuadro de fuente y el botón Buscar repiten la búsqueda desde esta ventana.
- Seleccione un resultado y haga clic en **Añadir**, o haga doble clic en él. El torrent se añade como se añade un enlace magnet; con «Mostrar una ventana con los archivos del torrent» activado en Preferencias, primero se abre la ventana Añadir nuevo torrent.
- Para que el cuadro vuelva a filtrar su lista de torrents, elija **Filtrar torrents** en el menú de la flecha. La elección se recuerda entre inicios.

Esa es toda la configuración. Las secciones siguientes son para más adelante.

## Conviene saber

**Solo se ejecuta mientras busca.** El asistente se inicia en su primera búsqueda y se detiene solo diez minutos después de la última. Se cierra con Rookparcel, termine este como termine. Nunca se inicia por sí solo ni al arrancar el equipo, y solo escucha en este equipo (`127.0.0.1`), nunca en su red.

**Dónde se guarda.** La carpeta `search-helper` dentro de la carpeta de ajustes de Rookparcel: normalmente `%APPDATA%\Rookparcel\search-helper`, la carpeta con `rookparcel.exe` en modo portátil, y la carpeta del perfil cuando usa perfiles. Los ajustes y el registro propios de Jackett están en su subcarpeta `data`.

**Proxy y VPN.** Si define un proxy en Preferencias > Conexión, el asistente también lo usa para los sitios. El asistente es un programa independiente, así que «Usar solo esta red» no se aplica a él: sus solicitudes a los sitios toman la ruta normal de Windows, que es su VPN cuando la VPN lleva todo su tráfico.

**Actualizar.** Nada se actualiza por sí solo. **Actualizar el asistente de búsqueda** en Fuentes de búsqueda busca un Jackett más reciente y pregunta antes de descargarlo; sus sitios y ajustes se conservan.

**Quitar.** **Quitar el asistente de búsqueda** lo detiene, borra su carpeta y quita su fuente de la lista. El botón **Obtener** vuelve. Borrar la carpeta de ajustes de Rookparcel también lo elimina.

**Sitios privados.** **Abrir la página del asistente** abre la página propia de Jackett en su navegador y copia su contraseña al portapapeles (la página la pide). Allí puede añadir un sitio privado con su inicio de sesión, como en cualquier Jackett. Los sitios añadidos allí se buscan junto con los marcados.

**Antivirus.** La descarga es la versión propia de Jackett, comprobada con la suma de verificación que GitHub publica para ella. Si un antivirus pone en cuarentena archivos de la carpeta `search-helper`, las búsquedas fallan con «El asistente de búsqueda no se inició»; restaure los archivos, o Quite el asistente y vuelva a Obtenerlo.

## Si algo sale mal

| Lo que ve | Qué significa |
|---|---|
| «No se pudo obtener el asistente de búsqueda: ...» | No se pudo acceder a GitHub (sin conexión, o un proxy que lo bloquea) o se interrumpió la descarga. Inténtelo de nuevo. |
| «El asistente de búsqueda no se inició: ...» | No se pudo ejecutar el programa de la carpeta `search-helper`: una descarga dañada o un antivirus. Quite el asistente y vuelva a obtenerlo. |
| Una búsqueda no encuentra nada | No hay sitios marcados, o los sitios marcados están caídos o han cambiado. Elegir sitios... muestra lo que está marcado; pruebe otro sitio. |
| «Asistente de búsqueda (todos): ...» con un error | Un sitio falló; los demás siguen respondiendo. El mensaje nombra el sitio. |
| La ventana de resultados no se abre nunca | El cuadro está en modo **Filtrar torrents**. Elija antes **Asistente de búsqueda (todos)** o una fuente en el menú de la flecha. |

## Usar su propio Prowlarr o Jackett en su lugar

Si ya usa Prowlarr o Jackett (en este PC, un NAS, un servidor), añádalo como fuente y omita el asistente:

1. En Fuentes de búsqueda, haga clic en **Añadir...**.
2. **Nombre:** cualquiera. **URL de Torznab:** para Jackett, `http://host:9117/api/v2.0/indexers/all/results/torznab/`; para Prowlarr, la URL del canal Torznab de un indexador, `http://host:9696/1/api`, una fuente por indexador. **Clave de API:** de la página de Jackett, o de Settings > General de Prowlarr.
3. Haga clic en **Probar**: debe decir «OK» y un número de categorías. Después, **Aceptar**.

La clave de API se guarda cifrada y no se vuelve a mostrar. A una fuente de su propio equipo o red se accede directamente; una pública pasa por su proxy, si definió uno. (Si el proxy resuelve por sí mismo los nombres de servidor, una fuente dada por un nombre de servidor distinto de `localhost` también pasa por él; indique la dirección IP de la fuente para mantenerla directa.) El asistente y sus propias fuentes se pueden usar a la vez: **Todas las fuentes** en el menú de la flecha las consulta todas.

Para saber cómo está construido el asistente y qué hace en la red, consulte [SEARCH_HELPER.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/SEARCH_HELPER.md); para la propia búsqueda Torznab, la parte 2 de [INTEGRATIONS.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/INTEGRATIONS.md).
