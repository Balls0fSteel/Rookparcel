*Esta página es una traducción de la página inglesa [Web UI](Web-UI). Si hay diferencias, prevalece la página en inglés.*

# Interfaz web

La interfaz web es la ventana principal de Rookparcel en un navegador web: la lista de torrents, las categorías, las pestañas de detalles, el menú del botón derecho, Añadir, Preferencias y el Descargador RSS. Esta página explica cómo activarla en el programa de Windows y en `rookparcel-nox`, cómo iniciar sesión, HTTPS, qué puede hacer la página y cómo usarla en un teléfono.

Hay dos maneras de obtenerla:

- **Desde el programa de Windows.** `rookparcel.exe` sirve la página por sí mismo, sobre los torrents de la propia ventana, mientras se ejecuta (**Opciones > Preferencias > Interfaz web**).
- **Desde `rookparcel-nox`**, el Rookparcel sin ventana, iniciado con `--webui` ([Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox)). Es el camino en servidores Linux, BSD y macOS, y en un PC con Windows que deba seguir descargando sin una ventana.

El programa GTK (Linux, BSD) y la aplicación de macOS no sirven una interfaz web propia. Ambos pueden controlar un `rookparcel-nox` mediante el [Control remoto](Control-remoto).

## Activarla en el programa de Windows

![Preferencias > Interfaz web](images/prefs-webui.png)

1. Abra **Opciones > Preferencias** y elija **Interfaz web**.
2. Marque **Activar la interfaz web**.
3. En **Autenticación**, escriba un **Usuario:** (se propone `admin`) y una **Contraseña:**. Se necesitan ambos; sin ellos la página dice «Escriba un nombre de usuario y una contraseña para la interfaz web.» La contraseña se guarda solo como un hash. Más adelante, deje el campo vacío para conservar la guardada.
4. En **Conectividad**, compruebe el **Puerto de escucha:**. Se propone 8080, o el siguiente puerto libre cuando otro programa o [perfil](Perfiles) lo ocupa.
5. Haga clic en **Aplicar** o en **Aceptar**. La línea de la parte inferior dice «La interfaz web se está ejecutando: ...» con su dirección.
6. Haga clic en **Abrir la interfaz web** para abrirla en su navegador.

Si la interfaz web no puede iniciarse (porque el puerto está en uso, por ejemplo), la línea de estado de la página dice por qué. Los ajustes de las otras páginas se aplican igualmente.

La interfaz web funciona solo mientras Rookparcel está en ejecución. Cierre la ventana a la bandeja para mantenerla activa ([Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio)). Se reinicia cuando usted aplica cambios en esta página.

Los demás ajustes de la página:

| Ajuste | Qué hace |
|---|---|
| **Permitir conexiones desde otros equipos** | Escucha en todas las direcciones de red, de modo que otros equipos y teléfonos pueden llegar a la página, no solo este. También necesitan una regla del Firewall de Windows para `rookparcel.exe` ([Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos)). |
| **Usar HTTPS (certificado autofirmado)** | Sirve la página por HTTPS con un certificado que crea el propio Rookparcel y guarda en la carpeta de ajustes. Los navegadores avisan sobre un certificado así hasta que usted lo acepta. |
| **Dominios del servidor (separe varias entradas con punto y coma):** | Nombres de host a los que responde la página además de las direcciones IP y `localhost`, como el nombre de este equipo. Los demás nombres se rechazan. Vea [Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr#nombres-de-host-y-docker). |
| **Aceptar una clave de API (Authorization: Bearer)** y **Clave de API (al menos 16 caracteres):** | Permite que scripts y aplicaciones como Sonarr o Radarr usen la interfaz web con esta clave en lugar de iniciar sesión. Desactivado de forma predeterminada. La clave se guarda solo como un hash y no vuelve a mostrarse; deje el campo vacío para conservarla. |
| **Omitir la autenticación para los clientes de estas subredes (...)** | Los clientes de estas redes (como `192.168.1.0/24`) usan la interfaz web sin iniciar sesión, con control total. Indique solo redes en las que confíe tanto como en la contraseña. |

Qué es distinto respecto a la página de nox cuando la sirve la ventana:

- Los torrents, etiquetas, marcas, ajustes y fuentes RSS son los de la propia ventana. Un cambio hecho en el navegador se ve en la ventana, y al revés. El Descargador RSS de la página edita las fuentes y los filtros de la ventana.
- El programador, el objetivo de siembra y la carpeta de las descargas completadas de la ventana hacen el trabajo, como sin la interfaz web.
- No hay límites de velocidad alternativos.
- Una ventana que controla un nox remoto ([Control remoto](Control-remoto)) no puede servir la interfaz web: «Una ventana remota no puede servir la interfaz web; use las Preferencias de la ventana local.»

## Activarla en rookparcel-nox

Inicie nox con `--webui` y un puerto:

```sh
rookparcel-nox run --save ~/Downloads --webui 8080
```

| Valor de `--webui` | Quién puede llegar a la página |
|---|---|
| `8080` | Solo este equipo (`127.0.0.1:8080`). |
| `0.0.0.0:8080` | Todos los equipos que puedan llegar a este. |
| `192.168.1.10:8080` | Los equipos que llegan a este en esa dirección. |

El nombre de usuario es `admin` salvo que indique `--webui-user`. El primer inicio, sin contraseña guardada, muestra una temporal:

```text
* Web UI: no password set; temporary password for this run: ... (set one with --webui-pass)
```

Inicie sesión con ella y luego defina la suya en **Preferencias > Interfaz web** de la página (el campo **Contraseña:**, con la temporal como **Contraseña actual:**). `--webui-pass` también la define, pero una contraseña en la línea de órdenes puede ser vista por otros usuarios del equipo.

Un nox que se ejecuta como servicio escribe la contraseña temporal en su registro o en el diario: vea [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

## Iniciar sesión

Abra la dirección en un navegador: `http://127.0.0.1:8080/` en el mismo equipo, o `http://<la dirección del equipo>:8080/` desde otro (`https://` cuando HTTPS está activado). Escriba el **Usuario** y la **Contraseña** y haga clic en **Entrar**. **Salir** está en el extremo derecho de la barra de herramientas.

- Tras cinco inicios de sesión fallidos seguidos desde una misma dirección, esa dirección queda bloqueada durante una hora. Ambos números están en **Preferencias > Interfaz web**.
- Una sesión termina tras una hora sin ninguna petición de ese navegador (**Caducidad de la sesión (segundos):**).
- Definir una contraseña nueva cierra la sesión de todos los demás navegadores y ventanas remotas; el que la cambió sigue con la sesión iniciada.
- Cuando la sesión termina (por la caducidad, o con **Salir**), la página cierra sus cuadros de diálogo abiertos y muestra el formulario de inicio de sesión. Un archivo `.torrent` que esperaba para añadirse sigue en espera y abre su cuadro Añadir cuando usted inicia sesión de nuevo.
- Cuando se pierde la conexión con Rookparcel, la barra de estado muestra «Desconectado, reconectando en ... s» y la página vuelve a conectarse sola.

## HTTPS

Sin HTTPS, la contraseña y todo lo que muestra la página cruzan la red sin cifrar. En su propio equipo eso no importa; a través de una red, active HTTPS.

- **Programa de Windows:** marque **Usar HTTPS (certificado autofirmado)** en **Preferencias > Interfaz web**.
- **nox:** añada `--webui-https`. nox crea un certificado la primera vez que se usa, lo guarda en su carpeta de ajustes (`webui-cert.pem`, `webui-key.pem`) y lo renueva un mes antes de que caduque. En cada inicio muestra una línea como `* Web UI: certificate SHA-256 ...`.
- **nox con su propio certificado:** `--webui-cert FILE --webui-key FILE` (PEM; la clave sin cifrar).

El navegador avisa sobre un certificado autofirmado la primera vez. Compare la huella que muestra el navegador con la línea `certificate SHA-256` que mostró nox y después acéptelo. Con HTTPS activado, una dirección `http://` simple al mismo puerto se redirige a `https://`.

## Qué puede hacer la página

La página sigue la ventana de escritorio: las mismas columnas, menús, palabras y orden.

- **Barra de herramientas:** **Añadir torrent**, **Añadir URL**, **Quitar**, **Iniciar**, **Pausa**, **Detener**, **Subir** y **Bajar** (orden de la cola), **RSS**, **Preferencias** (<kbd>Ctrl</kbd>+<kbd>P</kbd>), el cuadro de búsqueda que filtra la lista, **Buscar** y **Salir**.
- **Categorías** a la izquierda con sus recuentos, incluidas las etiquetas, las marcas y las fuentes RSS. Haga clic con el botón derecho en la rama **Marcas** (una pulsación larga en un teléfono) para **Nueva marca...**, o en una marca para **Borrar marca** (*las marcas son nuevas en 1.2.63*).
- **La lista de torrents:** haga clic en un encabezado para ordenar, arrastre los encabezados para reordenarlos o cambiar su tamaño, haga clic con el botón derecho en un encabezado para elegir columnas (**Restablecer columnas** las devuelve a su estado inicial).
- **Las pestañas de detalles** bajo la lista: General, Trackers, Pares, Piezas, Archivos, Velocidad y Registro ([Detalles y archivos del torrent](Detalles-y-archivos-del-torrent)).
- **El menú del botón derecho** tiene lo que tiene el menú de la ventana: **Forzar inicio**, **Iniciar**, **Pausa**, **Detener**, los movimientos en la cola, **Quitar** y **Quitar y > Borrar datos**, **Forzar comprobación** (inicia un torrent detenido, como en el programa de escritorio; *nuevo en 1.2.76*), **Actualizar tracker**, **Copiar enlace magnet**, **Exportar .torrent...**, **Avanzado > Establecer ubicación de descarga...**, **Siembra inicial**, **Descargar en orden secuencial** y **Descargar primero las piezas primera y última**, **Asignación de ancho de banda**, **Etiqueta**, **Marcas** y **Propiedades** ([Gestionar torrents](Gestionar-torrents)). **Abrir carpeta contenedora** está en gris: un navegador no puede abrir una carpeta del servidor. *Copiar enlace magnet, Exportar .torrent y los dos elementos de orden de descarga son nuevos en 1.2.63.* Exportar descarga el archivo `.torrent` a través del navegador. Donde el navegador no da a la página acceso al portapapeles (`http://` simple desde otro equipo), Copiar enlace magnet muestra los enlaces para que los copie a mano.
- **Añadir torrents:** **Añadir torrent** sube archivos `.torrent` desde el equipo donde se ejecuta el navegador. **Añadir URL** acepta un enlace magnet, un info-hash o un enlace `http://` a un torrent. También puede soltar archivos `.torrent` o un enlace magnet sobre la página, o pegar un enlace magnet en cualquier punto fuera de un cuadro de texto. Con «Mostrar una ventana con los archivos del torrent» activado (**Preferencias > Interfaz**, activado de forma predeterminada), un cuadro Añadir pregunta lo mismo que el de escritorio: carpeta de destino, nombre, etiqueta, qué archivos, iniciar o no. Para un enlace magnet se abre cuando ha llegado la lista de archivos del torrent. Los archivos que coinciden con **No descargar los archivos llamados:** (**Preferencias > Interfaz**) empiezan sin marcar en él, y los torrents añadidos sin él los dejan fuera (*nuevo en 1.2.63*). Vea [Añadir torrents](Añadir-torrents#archivos-excluidos-por-nombre).
- **Propiedades del torrent:** trackers, límites de velocidad, plazas de subida, el objetivo de siembra y el orden de descarga de los torrents seleccionados.
- **Preferencias:** las páginas General, Interfaz, Carpetas, Conexión, Ancho de banda, BitTorrent, Cola, Programador, Interfaz web, Juegos, Avanzado y Caché de disco. Pase el puntero sobre un ajuste para ver su ayuda, o pulse <kbd>F1</kbd> (*los textos de ayuda son nuevos en 1.1.57*). Las carpetas son carpetas del servidor, escritas como texto. La interfaz web que sirve el programa de Windows no tiene dirección del filtro IP, **Actualizar ahora** ni línea de estado en su página BitTorrent: las Preferencias de la propia ventana los contienen. Vea [Referencia de preferencias](Referencia-de-preferencias).
- **Descargador RSS:** fuentes, filtros e historial, como en el programa de escritorio ([Descargador RSS](Descargador-RSS)).
- **Buscar** abre Buscar torrents, que busca en su Prowlarr o Jackett; **Administrar fuentes...** allí los añade ([Buscar torrents](Buscar-torrents)). Añadir una fuente, o cambiar su dirección o su clave, pide la contraseña actual de la interfaz web (como también instalar el ayudante de búsqueda); una contraseña equivocada cierra la sesión de ese navegador y cuenta como un inicio de sesión fallido.

Teclas en la lista: las flechas, <kbd>Home</kbd>, <kbd>End</kbd>, <kbd>Page Up</kbd> y <kbd>Page Down</kbd> mueven; <kbd>Shift</kbd> y <kbd>Ctrl</kbd> con un clic o una tecla amplían la selección; <kbd>Ctrl</kbd>+<kbd>A</kbd> selecciona todos los torrents que muestra el filtro; <kbd>Space</kbd> pausa; <kbd>Del</kbd> quita; <kbd>Shift</kbd>+<kbd>Del</kbd> quita y borra los datos; la tecla <kbd>Menu</kbd> o <kbd>Shift</kbd>+<kbd>F10</kbd> abre el menú del botón derecho.

La página usa el idioma definido en **Preferencias > General**, «Interfaz». De forma predeterminada se vuelve oscura cuando su navegador o su sistema usa un tema oscuro. **Preferencias > Interfaz > Tema** elige un tema para todos los que abran esta interfaz web: **Sistema (claro u oscuro)**, **Claro**, **Oscuro**, **Medianoche**, **Pizarra**, **Solarized oscuro**, **Solarized claro**, **Contraste alto** o **Graja postal**. **Tema en este navegador** en la misma página (en un teléfono, el **Tema** del menú Más) elige uno solo para este navegador, guardado en el dispositivo; **Predeterminado del servidor** vuelve a seguir la elección del servidor. Su propio tema es un archivo CSS que se deja en la carpeta `webui/themes` de la carpeta de ajustes (`--config-dir` para `rookparcel-nox`; la carpeta de ajustes del programa de Windows cuando sirve la interfaz web): copie uno de los archivos incluidos del árbol de código fuente ([crates/nox/src/webui/themes](https://github.com/Balls0fSteel/Rookparcel/tree/main/crates/nox/src/webui/themes)), cambie los colores y la línea `/* name: ... */`, ponga al archivo un nombre en minúsculas, dígitos y guiones, y aparecerá en la lista con ese nombre.

## En un teléfono

La misma dirección funciona en un teléfono; la página cambia su disposición en una pantalla estrecha (de menos de 768 píxeles de ancho).

- La lista muestra el nombre de cada torrent sobre una barra de progreso con su estado, velocidades y tiempo restante.
- Las categorías están en un panel lateral tras el botón de menú al principio de la barra de herramientas.
- Toque un torrent para ver sus detalles en una hoja que sube desde abajo.
- El botón de cada fila abre su menú. Una pulsación larga empieza a seleccionar varios torrents; toque más para añadirlos, y **Hecho** termina la selección.
- La barra de herramientas conserva **Añadir**, **Iniciar**, **Pausa** y **Quitar**; el resto, y la ordenación, están en **Más**.
- En Android el botón Atrás cierra el panel, la hoja o el cuadro de diálogo que esté más arriba.

Para que el teléfono llegue a la página, la interfaz web debe aceptar conexiones desde otros equipos (**Permitir conexiones desde otros equipos**, o `--webui 0.0.0.0:PORT` de nox), y el teléfono debe estar en una red que llegue al equipo.

## Seguridad

La interfaz web controla sus torrents y puede escribir archivos en cualquier lugar donde pueda hacerlo la cuenta de Rookparcel. Tenga esto presente antes de abrirla a una red:

- Use una contraseña larga y active HTTPS para cualquier uso fuera de su propio equipo.
- **Activar la validación de la cabecera Host** (activada de forma predeterminada) rechaza las peticiones dirigidas a cualquier nombre que no sea una dirección IP, `localhost` o un dominio de servidor indicado. **Activar la protección contra falsificación de peticiones entre sitios (CSRF)** (activada de forma predeterminada) rechaza los cambios enviados desde la página de otro sitio web. Deje ambas activadas.
- **Omitir la autenticación para clientes en localhost** (página de nox; desactivada de forma predeterminada) deja entrar sin iniciar sesión a los programas del mismo equipo. Ella y la lista de subredes nunca se aplican a las peticiones que llegan a través de un proxy inverso.
- Cambiar el inicio de sesión, la clave de API, las subredes u otros ajustes de seguridad en la página requiere la **Contraseña actual:**. Una equivocada cierra la sesión de ese navegador y cuenta como un inicio de sesión fallido, de modo que los intentos repetidos bloquean la dirección igual que los inicios de sesión fallidos. Lo mismo vale para las direcciones desde las que el servidor descarga, como la dirección de la lista de trackers y la de la lista de bloqueo del filtro IP (Preferencias > BitTorrent, *nuevo en 1.2.63*; vea [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos#una-lista-desde-una-dirección-web)).
- Con nox, `--webui-save-root DIR` limita las carpetas en las que la interfaz web puede guardar, a las que puede mover y de las que puede leer ([Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox#interfaz-web)).

Detrás de un proxy inverso en el mismo equipo, haga que el proxy establezca la cabecera `X-Forwarded-For`, para que los inicios de sesión fallidos bloqueen al cliente correcto y no a todos. Con **Aceptar una clave de API (Authorization: Bearer)** activada, el proxy no debe añadir una cabecera `Authorization` propia: antes de que un navegador haya iniciado sesión, esa cabecera cuenta como una clave equivocada.

## Véase también

- [Control remoto](Control-remoto): la ventana del programa de escritorio que maneja un nox, como alternativa a la página.
- [Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr): uso de la API compatible con qBittorrent de la interfaz web.
- El diseño que hay detrás de la página: [WEBUI.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/WEBUI.md).
