*Esta página es una traducción de [Troubleshooting and FAQ](Troubleshooting-and-FAQ). La página en inglés es la versión de referencia.*

# Solución de problemas y preguntas frecuentes

Esta página ayuda cuando algo no funciona: descargas lentas o detenidas, un puerto al que no se puede llegar, avisos de Windows o macOS, trackers privados. También dice dónde están los mensajes de Rookparcel, cómo restablecer sus ajustes y cómo informar de un problema.

## Las descargas son lentas o no empiezan

Mire la columna **Estado** del torrent y la barra de estado de la parte inferior de la ventana. Suelen decir el motivo.

| Lo que ve | Qué significa y qué hacer |
|---|---|
| **En cola** | La cola está llena: se ha alcanzado el número de torrents o descargas activos definido en **Preferencias > Cola**. El torrent empieza cuando otro termina o se detiene. **Forzar inicio** en el menú contextual lo ejecuta fuera de la cola. Vea [Cola y siembra](Cola-y-siembra). |
| **Detenido** o **En pausa** | Inícielo con **Iniciar** (<kbd>Ctrl</kbd>+<kbd>S</kbd>). Con **No iniciar la descarga automáticamente** activado en Preferencias > Interfaz, los torrents nuevos se añaden detenidos. |
| **En cola para comprobar**, **Comprobando** | Rookparcel está comprobando los archivos que ya hay en el disco. Los torrents de la misma unidad se comprueban uno tras otro: los que esperan su turno dicen **En cola para comprobar**. Con **Comprobar los torrents con suavidad** activado (Preferencias > Juegos), una comprobación cede el paso a otros programas y tarda más mientras el disco está ocupado. Vea [Disco y almacenamiento](Disco-y-almacenamiento). *Las comprobaciones en espera lo indican desde la 1.2.76; antes, decían **Comprobando** al 0 %.* |
| **Esperando la red** | **Usar solo esta red** está activado en Preferencias > Conexión y su red (normalmente su VPN) no está. El torrent continúa por sí solo cuando vuelve. Vea [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos). |
| **Error: ...** | El mensaje que sigue a "Error:" dice qué fue mal. Para "files missing (the drive, share or folder is not available)", vea la sección siguiente. Si una comprobación se detiene con un error que nombra un archivo, otro programa tiene ese archivo abierto o usted no puede leerlo: cierre el programa o corrija el permiso, y después inicie el torrent con **Iniciar**. *Desde la 1.2.76 una comprobación así se detiene con el error; antes, contaba las piezas del archivo como ausentes y las descargaba de nuevo.* |
| "Programador: limitado", "Programador: apagado" o "Programador: solo sembrar" en la barra de estado | El programador está frenando las velocidades en esta hora. Vea [Límites de velocidad y programador](Límites-de-velocidad-y-programador). |
| "(auto ...)" junto a las velocidades en la barra de estado | **Mantener el ping bajo** está limitando la velocidad porque el retardo de su línea ha aumentado. Vea [Mantener el ping bajo](Mantener-el-ping-bajo). |

Otras cosas que conviene comprobar:

- **Semillas y pares.** Un torrent sin semillas no puede terminar. La pestaña **Trackers** muestra si los trackers responden; la pestaña **Pares** muestra con quién está conectado. Vea [Detalles y archivos del torrent](Detalles-y-archivos-del-torrent).
- **Los enlaces magnet** necesitan primero la lista de archivos del torrent de otros pares, y la mayoría necesitan DHT: mantenga **Activar la red DHT** activado en Preferencias > BitTorrent.
- **Los límites de velocidad.** Compruebe **Preferencias > Ancho de banda** y los límites propios del torrent (**Asignación de ancho de banda** en el menú contextual, o **Propiedades**). 0 significa sin límite.
- **Un disco ocupado.** Con **Ralentizar las bajadas (escrituras en disco)** activado (Preferencias > Juegos), Rookparcel reduce la velocidad de bajada mientras el disco responde despacio.
- **Un proxy para las conexiones con pares.** Con **Usar el proxy para las conexiones entre pares** activado, nadie puede conectarse con usted, lo que deja menos pares.
- **La falta de conexiones entrantes** (más abajo) también cuesta velocidad: solo puede llegar a los pares que aceptan conexiones por sí mismos.

## "Files missing (the drive, share or folder is not available)"

Cuando la carpeta de destino de un torrent no está (una unidad USB o de red desconectada, un recurso compartido o un montaje que no está conectado), Rookparcel detiene el torrent con este error en lugar de descargarlo otra vez. Las piezas que tenía siguen registradas, y lo siguen estando tras reiniciar.

1. Vuelva a conectar la unidad o el recurso compartido. Cada 30 segundos Rookparcel busca la carpeta; cuando vuelve, el torrent se inicia solo y encuentra sus archivos tal como estaban.
2. Para probar de inmediato, elija **Iniciar**. Si la carpeta sigue sin estar, el torrent se detiene de nuevo con el error.
3. Si movió o borró los archivos definitivamente, **Forzar comprobación** toma lo que hay ahora en la carpeta: las piezas que no están se descargan de nuevo.

Una unidad retirada mientras el torrent se descarga se trata del mismo modo. Cuando solo faltan algunos archivos de un torrent, este los descarga de nuevo en su lugar. Vea [Gestionar torrents](Gestionar-torrents). *Novedad de la 1.2.76; antes, el torrent descargaba todo de nuevo.*

**Forzar comprobación** en un torrent detenido ahora lo inicia, como hace **Iniciar**, y la comprobación se ejecuta de inmediato aunque el torrent estuviera en cola; deténgalo de nuevo si quiere que siga detenido. *Novedad de la 1.2.76; antes, la comprobación esperaba a un Inicio.*

## "Transferencias retenidas: confirme las preferencias"

Tras una actualización, o cuando no se puede leer el archivo de ajustes, Rookparcel retiene todas las transferencias hasta que usted confirme que sus ajustes de proxy, red y escucha son los que quiere. Abra **Opciones > Preferencias > Conexión**, compruébelos y haga clic en **Aceptar**. La pestaña Registro enumera lo que cambió. Vea [Actualizar Rookparcel](Actualizar-Rookparcel#cuando-una-versión-lee-sus-ajustes-de-forma-distinta).

## "Sin conexiones entrantes" o "Puerto ... en uso"

La parte izquierda de la barra de estado muestra el estado de la red:

| Texto | Significado |
|---|---|
| **Red OK** | Otros pares se han conectado con usted, de modo que su puerto de escucha es accesible. |
| **Sin conexiones entrantes** | Ningún par se ha conectado aún con usted. Justo después del inicio es normal; si sigue así, es probable que su puerto esté bloqueado por el router o el cortafuegos. |
| **Puerto 6881 en uso** (con su número de puerto) | Otro programa, a menudo otro perfil de Rookparcel, tiene el puerto, así que Rookparcel escucha en otro en el que no hay nada redirigido. Haga clic en el texto para abrir **Preferencias > Conexión** y elegir otro puerto. *Novedad de la 1.1.45.* |

Para hacer accesible el puerto:

1. Abra **Opciones > Preferencias > Conexión** y anote el **Puerto para las conexiones entrantes**.
2. Marque **Activar la asignación de puertos UPnP** y **Activar la asignación de puertos NAT-PMP**, para que se pida al router que redirija el puerto. O redirija el puerto a mano en el router (TCP y UDP) a este equipo, y deje desactivado **Puerto aleatorio en cada inicio**.
3. Permita `rookparcel.exe` en el cortafuegos (en Linux, `rookparcel-gtk`), o abra el puerto en él.
4. Detrás de una VPN, las conexiones entrantes solo funcionan si la VPN le redirige un puerto.

La pestaña **General** muestra "Su dirección según los trackers:", que le indica si su tráfico pasa por una VPN. *Novedad de la 1.1.58.* Más información en [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos).

## Windows SmartScreen o macOS Gatekeeper avisan sobre Rookparcel

Los programas no están firmados con código, así que los sistemas avisan la primera vez:

- **Windows:** "Windows protegió su PC". Haga clic en **Más información** y después en **Ejecutar de todas formas**.
- **macOS:** la aplicación se rechaza hasta que la permita una vez. En macOS 15 y posteriores, intente abrirla, haga clic en **Hecho** y después en **Abrir igualmente** en **Ajustes del Sistema > Privacidad y seguridad**. En macOS 11 a 14, haga clic con Control en la aplicación y elija **Abrir**. Vea [Instalar Rookparcel](Instalar-Rookparcel#macos).

Para asegurarse de que un archivo es el que se publicó, compare su suma de comprobación con el archivo `SHA256SUMS` de la versión. Los detalles de cada plataforma están en [Instalar Rookparcel](Instalar-Rookparcel).

## ¿Puedo usar Rookparcel en trackers privados?

Todavía no. Los trackers privados solo aceptan los clientes que tienen en su lista blanca, y el ID de cliente de Rookparcel (su ID de par empieza por `-PA`, y envía `Rookparcel/<version>` a los trackers) todavía no está en la lista blanca de ninguno. Rookparcel siempre se presenta con sinceridad como Rookparcel, así que usarlo en un tracker privado podría poner su cuenta en apuros. Compruebe primero la lista de clientes permitidos de su tracker.

Los torrents marcados como privados nunca usan DHT, el descubrimiento de pares locales ni el intercambio de pares, digan lo que digan las Preferencias.

## Al hacer doble clic en un archivo .torrent o en un enlace magnet no se abre Rookparcel

Marque **Asociar con archivos .torrent** y **Asociar con enlaces magnet:** en **Preferencias > General**. Si Windows sigue abriéndolos con otro programa, se eligió un programa en el propio Windows, y solo usted puede cambiarlo: haga clic en **Aplicaciones predeterminadas...** y elija allí Rookparcel. Vea [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio#abrir-archivos-torrent-y-enlaces-magnet-con-rookparcel).

## No se puede acceder a la interfaz web desde otro dispositivo

- Marque **Permitir conexiones desde otros equipos** en **Preferencias > Interfaz web**; de lo contrario, solo escucha en este equipo.
- Permita `rookparcel.exe` en el Firewall de Windows para el puerto de la interfaz web.
- "port ... is in use, perhaps by another Rookparcel profile": elija otro **Puerto de escucha**.
- Una aplicación que llega a Rookparcel por un nombre de host (como en Docker) falla con "unexpected Host header". Añada el nombre en **Dominios del servidor**, o use la dirección IP.

Vea [Interfaz web](Interfaz-web) y [Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr).

## Una ventana remota dice "the server refused the connection (HTTP 426)"

Una ventana conectada con **Archivo > Conectar en remoto...** y el `rookparcel-nox` al que se conecta deben ser de la misma versión: el mensaje termina con "this server speaks remote protocol ...". Algunas versiones cambian el protocolo remoto (la última vez en la 1.2.70), así que después de actualizar un lado a la 1.2.76, actualice también el otro: la ventana de escritorio y el programa `rookparcel-nox` o su imagen de Docker. Un navegador que usa la interfaz web no tiene este límite. Vea [Control remoto](Control-remoto) y [Actualizar Rookparcel](Actualizar-Rookparcel).

## Cerrar Rookparcel tarda mucho

Rookparcel guarda la lista de torrents y el progreso, avisa a los trackers de que se detuvo y espera a los archivos que todavía se están moviendo. Cuando esto tarda más de un momento, la ventana "Rookparcel se está cerrando" muestra qué está esperando. **Salir ahora** deja de esperar; pregunta antes y enumera a qué renuncia. La lista de torrents y el progreso se guardan primero en cualquier caso.

En Linux, la ventana GTK se cierra del mismo modo cuando recibe <kbd>Ctrl</kbd>+<kbd>C</kbd> en su terminal o un `SIGTERM`, por ejemplo desde `kill`. Enviar la señal otra vez no acorta el cierre. Para detenerlo de inmediato, use **Salir ahora**, o `kill -KILL` con el ID del proceso. *Desde la 1.2.54; antes, una segunda señal cerraba de inmediato sin guardar.*

## Una copia portátil no puede escribir en su carpeta

"Este Rookparcel portátil no puede escribir en su carpeta" significa que la carpeta con `rookparcel.exe` y `rookparcel.ini` es de solo lectura para usted, por ejemplo bajo `C:\Program Files`. Mueva la carpeta a un lugar donde pueda escribir, o quite `rookparcel.ini` junto al programa para guardar los ajustes en su carpeta de usuario. Vea [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos).

## El asistente de búsqueda no funciona

Vea la tabla del final de [Buscar torrents en Windows](Buscar-torrents-en-Windows) y [Buscar torrents](Buscar-torrents).

## Notas sobre Linux

- **No hay icono de bandeja en GNOME.** El GNOME simple no tiene bandeja; instale la extensión AppIndicator, o use Rookparcel sin ella (cerrar la ventana lo cierra entonces). Vea [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio#versión-gtk-linux-bsd).
- **Minimizar a la bandeja no hace nada** en algunos escritorios Wayland, que no indican que una ventana se minimizó.
- **Cerrar sesión en Xfce** no espera a que Rookparcel guarde; ciérrelo primero.

## Notas sobre macOS

- **Cerrar la ventana no cierra el programa.** Con **Cerrar a la bandeja** activado (el valor predeterminado), la ventana se oculta y Rookparcel sigue ejecutándose en la barra de menús. Salga con <kbd>⌘</kbd>+<kbd>Q</kbd>. Vea [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio#macos).
- **No hay notificaciones.** Permita Rookparcel en **Ajustes del Sistema > Notificaciones**.
- **Apagado automático no suspende ni apaga el Mac.** Permita que Rookparcel controle "loginwindow" en **Ajustes del Sistema > Privacidad y seguridad > Automatización**. Vea [Apagado automático](Apagado-automático).
- **Los ajustes han desaparecido tras sustituir la versión GTK.** La aplicación nueva se hace cargo de la antigua carpeta `gtk` solo mientras no exista una carpeta `mac`. Si se creó antes una carpeta `mac`, cierre Rookparcel y vea [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos#macos).

## Dónde están los registros

**Las versiones de Windows y GTK** no escriben ningún archivo de registro. Sus mensajes (errores, el filtro IP, la búsqueda de actualizaciones, el programador, las descargas completadas) van a la pestaña **Registro** de la parte inferior de la ventana principal, que conserva las últimas 1000 líneas mientras Rookparcel se ejecuta. Mírela justo después de que ocurra el problema, antes de salir, y anote las líneas relacionadas.

**rookparcel-nox** imprime sus mensajes en el terminal. Para más detalle, inícielo con `--verbose`, que añade eventos de trackers y pares (cada desconexión con su motivo). `--log FILE` añade todo al final de un archivo. Cuando se ejecuta como servicio:

| Dónde | Registro |
|---|---|
| Servicio de Windows | `nox.log` en la carpeta de estado, `%ProgramData%\Rookparcel\nox` salvo que `--state-dir` diga otra cosa (léalo desde un símbolo del sistema con privilegios elevados) |
| systemd | el diario: `journalctl -u rookparcel-nox` |
| FreeBSD rc.d | `/var/log/rookparcel-nox.log` |

Vea [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

**El asistente de búsqueda** (Jackett) guarda sus propios ajustes y su registro en la carpeta `data` dentro de `search-helper` en su carpeta de ajustes de Rookparcel.

## Restablecer los ajustes

- **Un ajuste:** en **Preferencias > Avanzado**, selecciónelo y haga clic en **Restablecer**, y después en **Aceptar**. Las filas marcadas con `*` difieren del valor predeterminado.
- **Un PIN de bloqueo olvidado:** mantenga pulsada <kbd>Ctrl</kbd> (Windows) o <kbd>⌥</kbd> (macOS) mientras inicia Rookparcel, o inícielo con `--reset-lock`, y confirme. Vea [Bloquear Rookparcel](Bloquear-Rookparcel#si-olvidó-el-pin).
- **Las preguntas del primer inicio:** **Opciones > Guía de configuración** (<kbd>Ctrl</kbd>+<kbd>G</kbd>) las vuelve a hacer. *Novedad de la 1.1.62.*
- **Todo:** cierre Rookparcel (**Archivo > Salir**) y después cambie el nombre de la carpeta donde guarda sus ajustes y torrents (en Linux y BSD, la carpeta de ajustes y la carpeta de datos); Rookparcel se inicia entonces como nuevo, con una lista de torrents vacía. En Windows, **Ayuda > Acerca de Rookparcel** muestra la carpeta. Si vuelve a ponerle su nombre, se restaura todo. Vea [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos).

## Informar de un error

Abra una incidencia en el seguimiento de incidencias: <https://github.com/Balls0fSteel/Rookparcel/issues>. Incluya, por favor:

1. Su sistema (Windows 11, Ubuntu 24.04, macOS 15...) y el archivo que descargó.
2. La versión de Rookparcel, desde **Ayuda > Acerca de Rookparcel**.
3. Qué hizo, qué esperaba y qué ocurrió en cambio, paso a paso.
4. Para un bloqueo o un torrent atascado, las líneas de la pestaña **Registro**, o la salida de `rookparcel-nox run --verbose`.

No incluya contraseñas, claves de API ni nada más que sea privado; el seguimiento de incidencias es público.

## Informar de un problema de seguridad

No abra una incidencia pública. Use la notificación privada de vulnerabilidades de GitHub: <https://github.com/Balls0fSteel/Rookparcel/security/advisories/new>. Vea [SECURITY.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/SECURITY.md).

## Preguntas frecuentes

**¿Es gratis Rookparcel?** Es gratis para uso no comercial bajo la PolyForm Noncommercial License 1.0.0: uso personal, estudio, proyectos de aficionados, organizaciones benéficas, escuelas e instituciones públicas, sin coste. El uso comercial necesita un permiso aparte. Vea [LICENSE.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/LICENSE.md).

**¿Está relacionado con µTorrent o qBittorrent?** No. Rookparcel recrea el aspecto y el comportamiento de µTorrent 2.2.1, pero no usa nada de su código. Su motor es una adaptación independiente a Rust de libtorrent-rasterbar 2.0.11. Su API web sigue la documentación publicada de qBittorrent para que herramientas como Sonarr puedan comunicarse con él.

**¿Qué idiomas habla?** Inglés y otros 34, todos integrados. Sigue el idioma de su sistema; elija otro en **Preferencias > General**. Las traducciones son automáticas; las correcciones son bienvenidas como incidencias.

**¿Qué envía Rookparcel, y a quién?** La búsqueda de actualizaciones descarga un archivo pequeño del sitio web del proyecto, sin nada sobre usted ni sus torrents; desactívela en **Preferencias > General > Privacidad**. Las búsquedas van solo a las fuentes de búsqueda que usted añadió. Como todo cliente BitTorrent, Rookparcel muestra su dirección IP a los trackers, a DHT y a los pares de sus torrents. Vea [Actualizar Rookparcel](Actualizar-Rookparcel).

**¿Dónde están mis ajustes y mis torrents?** Vea [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos).

**¿Puedo ejecutar varias copias?** Sí, como perfiles. Vea [Perfiles](Perfiles).

**¿Dónde consigo ayuda?** Lea las [preguntas frecuentes del sitio web](https://balls0fsteel.github.io/Rookparcel/faq.html) y las páginas de este wiki, o pregunte en el [seguimiento de incidencias](https://github.com/Balls0fSteel/Rookparcel/issues).
