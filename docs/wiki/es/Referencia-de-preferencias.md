*Esta página es una traducción de [Preferences reference](Preferences-reference). La página en inglés es la versión de referencia.*

# Referencia de preferencias

Esta página enumera todas las páginas de la ventana Preferencias y explica qué hace cada ajuste, en el orden en que la ventana los muestra. Cada sección remite a la página que explica el tema por completo.

## Abrir Preferencias

Abra **Opciones > Preferencias**, pulse <kbd>Ctrl</kbd>+<kbd>P</kbd> o haga clic en **Preferencias** en la barra de herramientas. Las páginas aparecen a la izquierda; **Caché de disco**, **Trackers**, **Ejecutar programa** y **Bloqueo de la interfaz** están dentro de **Avanzado**. **Aceptar** guarda y cierra, **Aplicar** guarda y deja la ventana abierta, **Cancelar** descarta los cambios.

La ayuda está integrada:

- Apunte a un ajuste y un cuadro emergente lo explica. El panel que hay bajo las páginas muestra el mismo texto para el ajuste que está bajo el puntero o, si no, para el que tiene el foco del teclado.
- Pulse <kbd>F1</kbd> y la ayuda del ajuste enfocado aparece a su lado.

*Los textos de ayuda, los cuadros emergentes y F1 son novedad de la 1.1.57.*

Los textos de abajo son versiones breves de esa ayuda integrada. Las velocidades de las páginas están en kB/s; 0 significa sin límite, salvo que la etiqueta diga otra cosa.

## General

![Preferencias, página General](images/prefs-general.png)

| Ajuste | Qué hace |
|---|---|
| **Interfaz:** (idioma) | El idioma de los menús, los diálogos y la interfaz web. La primera opción sigue el idioma del sistema. Las ventanas de escritorio cambian en el siguiente inicio. |
| **Asociar con archivos .torrent** | Hace que esta copia de Rookparcel sea el programa que abre los archivos `.torrent`, solo para su cuenta de usuario. Un programa elegido en los ajustes de aplicaciones predeterminadas del propio sistema tiene prioridad sobre esto. |
| **Asociar con enlaces magnet:** | Hace que esta copia de Rookparcel sea el programa que abre los enlaces `magnet:` en los que se hace clic en un navegador, solo para su cuenta de usuario. Con perfiles, solo un perfil puede tenerlos. |
| **Comprobar asociación al iniciar** | Al iniciar, pregunta si debe asumir los archivos `.torrent` y los enlaces `magnet:` cuando Rookparcel no es el programa que los abre. Desactivado de forma predeterminada. |
| **Iniciar Rookparcel al arrancar el sistema** | Inicia Rookparcel cuando usted inicia sesión. Cada perfil tiene su propia entrada. |
| **Iniciar minimizado** | Con el inicio automático, Rookparcel arranca sin mostrar su ventana, en la bandeja donde la haya. |
| **Aplicaciones predeterminadas...** (solo Windows) | Abre la Configuración de Windows en Aplicaciones predeterminadas. Un programa elegido allí tiene prioridad sobre las casillas anteriores, y solo usted puede cambiar esa elección. |
| **Buscar actualizaciones automáticamente** (Privacidad) | Unos diez segundos después del inicio y luego cada 12 horas, pregunta al sitio web del proyecto si hay una versión más reciente. No se descarga ni se instala nada sin preguntar. **Ayuda > Buscar actualizaciones** funciona en ambos casos. |
| **Actualizar a versiones beta** (Privacidad) | Ofrece también versiones preliminares (beta) al buscar actualizaciones. |
| **Añadir .!ut a los archivos incompletos** (Al descargar) | Un archivo que aún se está descargando se guarda con `.!ut` tras su nombre (`film.mkv.!ut`) y recibe su nombre normal en cuanto se ha descargado entero y ha pasado su comprobación. Activar o desactivar esto cambia al instante el nombre de los archivos de los torrents en marcha, y el de los torrents detenidos cuando se inician. Desactivado de forma predeterminada. |

En Windows las casillas de integración están en **Integración con Windows**; en la versión GTK el grupo se llama **Integración con el escritorio**.

*Añadir .!ut a los archivos incompletos es novedad de la 1.2.63.* Los archivos que µTorrent deja como `.!ut` o qBittorrent como `.!qB` se reconocen tanto si la casilla está marcada como si no. Un archivo que otro programa tiene abierto conserva su nombre hasta que se pueda cambiar. Un archivo cuyo nombre con `.!ut` es otro archivo del mismo torrent conserva su nombre simple, y Quitar y > Borrar datos solo borra un archivo `.!ut` o `.!qB` cuando contiene datos de este torrent.

Más información: [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio), [Añadir torrents](Añadir-torrents), [Actualizar Rookparcel](Actualizar-Rookparcel), [Perfiles](Perfiles), [Disco y almacenamiento](Disco-y-almacenamiento#archivos-incompletos-y-ut).

## Interfaz

| Ajuste | Qué hace |
|---|---|
| **Confirmar al borrar torrents** | Pregunta antes de quitar torrents de la lista e indica si también se borran sus archivos. Desactivado, **Quitar** actúa de inmediato. |
| **Tema** (Windows) | Los colores de la ventana: **Sistema (claro u oscuro)** sigue el *modo* que se elige en Windows para las aplicaciones, **Claro** y **Oscuro** los fijan. El Contraste alto de Windows conserva sus propios colores. El modo oscuro necesita Windows 10 versión 1809 o posterior; un Windows más antiguo lo indica bajo el cuadro y se queda en claro ([Modo oscuro](Modo-oscuro)). |
| **Tema** (solo interfaz web) | Los colores de la página de la interfaz web para todos los que la abran: Sistema (claro u oscuro), Claro, Oscuro, Medianoche, Pizarra, Solarized oscuro, Solarized claro, Contraste alto, Graja postal, y sus propios archivos CSS de la carpeta `webui/themes` dentro de la carpeta de ajustes ([Interfaz web](Interfaz-web)). |
| **Tema en este navegador** (solo interfaz web) | Un tema solo para este navegador, que se guarda en este dispositivo y se usa en lugar del que elige el servidor. **Predeterminado del servidor** vuelve a seguir el tema del servidor. |
| **Alternar el color de fondo de la lista** | Sombrea una de cada dos filas de las listas. |
| **Mostrar la velocidad actual en la barra de título** | Pone las velocidades actuales de bajada y subida al principio del título de la ventana principal, que se ve también en la barra de tareas. |
| **Mostrar notificaciones en la bandeja** | Muestra una notificación, con el nombre del torrent, cuando termina una descarga. |
| **Cerrar a la bandeja** (Bandeja del sistema) | El botón de cierre de la ventana oculta Rookparcel en la bandeja en lugar de salir, de modo que las transferencias continúan. Salga desde el menú o desde el menú del icono de la bandeja. |
| **Minimizar a la bandeja** (Bandeja del sistema) | Al minimizar, la ventana se oculta en la bandeja en lugar de quedar en la barra de tareas. Haga clic en el icono de la bandeja para recuperarla. |
| **No iniciar la descarga automáticamente** (Al añadir torrents) | Los torrents nuevos se añaden detenidos y fuera de la cola, de modo que no se descarga nada hasta que usted los inicie. |
| **Activar la ventana del programa** | Trae la ventana principal al frente cuando otro programa, como un navegador, entrega un torrent a Rookparcel. Desactivado, solo aparece la ventana Añadir nuevo torrent. |
| **Mostrar una ventana con los archivos del torrent** | Abre la ventana Añadir nuevo torrent por cada torrent que añade, para elegir antes la carpeta y los archivos; en un enlace magnet, cuando ha llegado su lista de archivos. Desactivado, los torrents se añaden de inmediato con los valores predeterminados. |
| **No descargar los archivos llamados:** (Al añadir torrents) | Los archivos cuyos nombres coinciden con uno de los patrones del cuadro de abajo empiezan sin marcar, de modo que no se descargan, en los torrents que añada a partir de ahora. Separe los patrones con punto y coma; `*` representa cualquier cantidad de caracteres y `?` uno solo, y no se distingue entre mayúsculas y minúsculas, como en `*.lnk; Thumbs.db; *sample*`. Solo cuenta el nombre del archivo, no sus carpetas. Desactivado de forma predeterminada. |

*No descargar los archivos llamados es novedad de la 1.2.63.* Aún puede marcar uno de esos archivos en la ventana Añadir nuevo torrent, o más tarde en la pestaña Archivos. Un torrent cuyos archivos coinciden todos se deja como está. La lista se aplica también a los torrents añadidos sin la ventana Añadir nuevo torrent, por ejemplo desde la carpeta vigilada, RSS o Sonarr y Radarr, pero no a los torrents que ya están en la lista. Vea [Añadir torrents](Añadir-torrents#archivos-excluidos-por-nombre).

Más información: [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio), [Añadir torrents](Añadir-torrents), [Gestionar torrents](Gestionar-torrents), [La ventana principal](La-ventana-principal).

## Carpetas

| Ajuste | Qué hace |
|---|---|
| **Poner las nuevas descargas en:** | La carpeta donde se guardan los torrents nuevos, salvo que elija otra al añadirlos. |
| **Mover las descargas completadas a:** | Cuando un torrent termina de descargarse, sus archivos se mueven a esta carpeta y sigue sembrando desde allí. Se aplica a los torrents que terminen a partir de ahora. |
| **Añadir la etiqueta del torrent** | Mueve un torrent terminado que tiene etiqueta a una carpeta con el nombre de la etiqueta, dentro de la carpeta de descargas completadas. |
| **Mover solo desde la carpeta de descarga predeterminada** | Mueve solo los torrents guardados en la carpeta de nuevas descargas; los torrents que guardó en otro lugar se quedan donde están. |
| **Guardar los .torrent en:** | Guarda en esta carpeta una copia de cada archivo `.torrent` añadido, con el nombre del torrent. La copia de un enlace magnet se guarda cuando han llegado sus metadatos. |
| **Mover los .torrent de tareas terminadas a:** | Cuando un torrent termina de descargarse, su copia `.torrent` guardada se mueve a esta carpeta. |
| **Cargar automáticamente los .torrent de:** | Vigila esta carpeta y añade cada archivo `.torrent` nuevo que se ponga en ella, sin la ventana Añadir nuevo torrent. Un archivo cargado pasa a llamarse `.torrent.loaded`. |
| **Borrar los .torrent cargados** | Borra un archivo `.torrent` cargado automáticamente cuando su torrent ya está añadido, en lugar de cambiarle el nombre. |

Más información: [Disco y almacenamiento](Disco-y-almacenamiento), [Añadir torrents](Añadir-torrents), [Cola y siembra](Cola-y-siembra).

## Conexión

![Preferencias, página Conexión](images/prefs-connection.png)

| Ajuste | Qué hace |
|---|---|
| **Puerto para las conexiones entrantes:** | El puerto al que se conectan los otros pares. Permítalo en el cortafuegos y redirijalo en el router, o deje que UPnP o NAT-PMP lo hagan. Un cambio se aplica de inmediato. |
| **Puerto aleatorio** (botón) | Rellena un puerto aleatorio de 10000 a 60000. Se usa cuando usted lo aplica. |
| **Puerto aleatorio en cada inicio** | Elige un puerto aleatorio nuevo en cada inicio. Un puerto redirigido a mano en el router deja entonces de coincidir. |
| **Activar la asignación de puertos UPnP** | Pide a un router UPnP que redirija el puerto de escucha a este equipo. Desactívela cuando redirija el puerto usted mismo o el router funcione mal. |
| **Activar la asignación de puertos NAT-PMP** | Pide al router que redirija el puerto mediante NAT-PMP o PCP, que muchos routers ofrecen además de UPnP o en su lugar. |
| **Tipo:** (Servidor proxy) | El tipo de proxy: **Socks4**, **Socks5** o **HTTP**. **(ninguno)** se conecta directamente. |
| **Proxy:** y **Puerto:** | El nombre de host o la dirección IP del servidor proxy, y el puerto en el que escucha (como 1080 en muchos servidores SOCKS). |
| **Autenticación**, **Usuario:**, **Contraseña:** | Inicia sesión en el proxy. A un proxy SOCKS4 solo se le envía el nombre de usuario. |
| **Resolver nombres de servidor a través del proxy** | Hace que el proxy resuelva los nombres de los trackers y de las semillas web, para que su propio servidor DNS no los vea. |
| **Usar el proxy para las conexiones entre pares** | Envía las conexiones con pares y semillas web a través del proxy. Las conexiones entrantes se rechazan entonces, y la asignación de puertos y el descubrimiento de pares locales siguen desactivados; uTP y DHT necesitan un proxy SOCKS5. |
| **Usar el proxy para las conexiones con trackers** | Envía los anuncios a los trackers a través del proxy, de modo que los trackers ven la dirección del proxy en lugar de la suya. |
| **Red:** (Usar solo esta red) | Mantiene todo el tráfico de torrents en un adaptador de red: **Cualquier adaptador VPN**, un adaptador por su nombre o **Rango de direcciones (abajo)**. **Cualquier red (desactivado)** lo desactiva. La interfaz web y las conexiones remotas no quedan ligadas. |
| **Rango de direcciones:** | Un rango como `10.2.0.0/16` o `fd00::/8`, para la opción **Rango de direcciones (abajo)**: el adaptador que tenga una dirección dentro del rango lleva el tráfico de torrents. |
| **Pausar los torrents mientras falte esta red** | Mientras ningún adaptador coincide (la VPN está caída), los torrents en marcha esperan y muestran "Esperando la red", y después continúan por sí solos. |

*El grupo Usar solo esta red es novedad de la 1.1.58.*

Más información: [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos).

## Ancho de banda

![Preferencias, página Ancho de banda](images/prefs-bandwidth.png)

| Ajuste | Qué hace |
|---|---|
| **Velocidad máxima de subida (kB/s)** | Lo máximo que suben por segundo todos los torrents juntos. 0 significa sin límite. Las horas Limitado del programador y Mantener el ping bajo (página Juegos) pueden reducirla durante un tiempo. |
| **Velocidad máxima de bajada (kB/s)** | Lo mismo para las bajadas. |
| **Aplicar el límite de velocidad a la sobrecarga del transporte** | Cuenta las cabeceras TCP/IP del tráfico con pares dentro de los límites, de modo que coincidan con lo que lleva la línea. Desactivado de forma predeterminada. |
| **Aplicar el límite de velocidad a las conexiones uTP** | Aplica los límites también a los pares uTP. Desactivado, a los pares uTP solo los frena el control de retardo propio de uTP. Activado de forma predeterminada. |
| **Número máximo global de conexiones** | Lo máximo de conexiones con pares abiertas a la vez entre todos los torrents. Redúzcalo si el router tiene dificultades. |
| **Número máximo de pares conectados por torrent** | Lo máximo de pares a los que se conecta un torrent; 0 significa sin límite. Se aplica a los torrents añadidos a partir de ahora. |
| **Número de ranuras de subida por torrent** | A cuántos pares sube a la vez un torrent; 0 significa sin límite. Se aplica a los torrents añadidos a partir de ahora. |
| **Número global de ranuras de subida** | A cuántos pares suben a la vez todos los torrents juntos; -1 significa sin límite. Menos ranuras dan más velocidad a cada par. |

Más información: [Límites de velocidad y programador](Límites-de-velocidad-y-programador).

## BitTorrent

![Preferencias, página BitTorrent](images/prefs-bittorrent.png)

| Ajuste | Qué hace |
|---|---|
| **Activar la red DHT** | Encuentra pares mediante DHT, sin tracker; la mayoría de los enlaces magnet lo necesitan. Los torrents privados nunca lo usan. |
| **Activar el descubrimiento de pares locales** | Encuentra pares de los mismos torrents en su red local. Los torrents privados nunca lo usan. |
| **Activar el intercambio de pares** | Permite que los pares conectados se informen entre sí de más pares. Los torrents privados nunca lo usan. |
| **Activar uTP (transporte UDP con gestión del ancho de banda)** | Se conecta con pares y los acepta mediante uTP además de TCP. uTP reduce la velocidad cuando la red está ocupada. |
| **Saliente:** (Cifrado del protocolo) | **Desactivado**: no cifrar nunca. **Activado**: los intentos sin cifrar y cifrados se alternan por cada par, conservando lo que funcionó. **Forzado**: solo cifrado, con lo que se pierden los pares que no pueden cifrar. |
| **Permitir conexiones entrantes sin cifrar** | Acepta conexiones entrantes que no están cifradas. Desactivado, solo se aceptan las cifradas. |
| **Activar el filtro IP** | Bloquea los pares cuyas direcciones están en la lista de bloqueo, en formato `ipfilter.dat` de eMule o P2P de PeerGuardian, o en líneas CIDR como `10.0.0.0/8`. Una ruta vacía significa `ipfilter.dat` en la carpeta de ajustes. |
| **Recargar** | Carga ahora la lista de bloqueo y muestra cuántas reglas contiene. El filtro IP debe estar activado. |
| **Obtener la lista de bloqueo de esta dirección (https):** | Toma la lista de bloqueo de la dirección `https://` de abajo en lugar del archivo de arriba, a través del proxy cuando hay uno. La copia descargada por última vez de esa dirección se usa al instante al iniciar, y la lista se vuelve a descargar una vez al día. Mientras la casilla está marcada, el archivo y **Recargar** aparecen en gris. |
| **Actualizar ahora** | Descarga ahora la lista de bloqueo de la dirección. La línea de abajo muestra cuántas reglas contiene y cuándo se descargó, o por qué falló la última descarga. |

*La lista de bloqueo obtenida de una dirección es novedad de la 1.2.63.* La lista puede estar en formato `ipfilter.dat`, P2P de PeerGuardian o CIDR (`10.0.0.0/8`), sin comprimir o comprimida con gzip; los archivos zip no son compatibles. Cuando una descarga falla o no es una lista de ese tipo, las reglas en uso se conservan. Una dirección nueva conserva la copia de la dirección anterior hasta que se ha descargado una lista de la nueva, de modo que una primera descarga fallida no hace perder nada; en el programa de Windows, **Actualizar ahora** con una dirección que aún no se ha aplicado solo la prueba hasta que se pulse **Aceptar** o **Aplicar**.

Más información: [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos#filtro-ip).

## Cola

![Preferencias, página Cola](images/prefs-queueing.png)

| Ajuste | Qué hace |
|---|---|
| **Número máximo de torrents activos (subida o bajada)** | Lo máximo de torrents en cola que se ejecutan a la vez; los demás esperan. -1 significa sin límite. Los torrents forzados se ejecutan fuera de la cola. |
| **Número máximo de descargas activas** | Lo máximo de torrents en cola que descargan a la vez. -1 significa sin límite. |
| **Número máximo de semillas activas** | Lo máximo de torrents en cola ya terminados que siembran a la vez. -1 significa sin límite. |
| **No contar los torrents lentos en estos límites** | Un torrent que mueve menos de unos 2 kB/s durante un minuto deja de ocupar una plaza, de modo que otro puede empezar. |
| **Proporción mínima (%)** (Objetivo de siembra) | Suba este porcentaje de lo que se descargó; 150 significa una vez y media. 0 significa sin objetivo de proporción. No hace nada hasta que se marque la casilla de abajo. |
| **Tiempo mínimo de siembra (minutos)** | Siembre durante estos minutos. 0 significa sin objetivo de tiempo. Con los dos objetivos definidos, deben alcanzarse ambos. No hace nada hasta que se marque la casilla de abajo. |
| **Limitar la velocidad de subida a (kB/s): [0: detener]** | Pone en vigor el objetivo de siembra. Marcada, un torrent que alcanzó el objetivo se detiene (0) o sigue sembrando con su velocidad de subida limitada a ese número. Desmarcada, como al principio, los torrents siguen sembrando más allá del objetivo. |

Más información: [Cola y siembra](Cola-y-siembra).

## Programador

![Preferencias, página Programador](images/prefs-scheduler.png)

| Ajuste | Qué hace |
|---|---|
| **Activar el programador** | Sigue la tabla semanal hora por hora, en hora local. Desactivado, Rookparcel funciona siempre a toda velocidad. |
| **Tabla del programador** | Una celda por cada hora de la semana; haga clic o arrastre para alternar entre **Velocidad completa**, **Limitado**, **Apagado** y **Solo sembrar**. |
| **Velocidad de subida limitada (kB/s)** y **Velocidad de bajada limitada (kB/s)** | Los límites durante las horas Limitado, en lugar de los normales. 0 significa sin límite. |
| **Desactivar DHT al apagar** | Desactiva también DHT durante las horas Apagado. Activado de forma predeterminada. |

Más información: [Límites de velocidad y programador](Límites-de-velocidad-y-programador).

## Interfaz web

![Preferencias, página Interfaz web](images/prefs-webui.png)

Esta página existe solo en la versión de Windows. Permite que un navegador controle los torrents de esta ventana.

| Ajuste | Qué hace |
|---|---|
| **Activar la interfaz web** | Sirve la interfaz web desde esta ventana mientras Rookparcel se ejecuta. Necesita un nombre de usuario y una contraseña. |
| **Usuario:** y **Contraseña:** | El inicio de sesión. La contraseña se guarda solo como hash; déjela en blanco para conservar la guardada. Una contraseña nueva cierra la sesión de todos los demás navegadores. |
| **Aceptar una clave de API (Authorization: Bearer)** | Permite que aplicaciones como Sonarr o Radarr usen la interfaz web con la clave de abajo, sin iniciar sesión. Desactivado de forma predeterminada. |
| **Clave de API (al menos 16 caracteres):** | La clave que envían las aplicaciones. Se guarda solo como hash y no se vuelve a mostrar. |
| **Omitir la autenticación para los clientes de estas subredes** | Los clientes de estas redes usan la interfaz web sin iniciar sesión. Indique solo redes en las que confíe tanto como en la contraseña. |
| **Puerto de escucha:** | El puerto en el que escucha la interfaz web; se propone 8080, o el siguiente puerto libre. |
| **Permitir conexiones desde otros equipos** | Escucha en todas las direcciones de red, de modo que otros equipos y teléfonos pueden acceder. También necesitan una regla del Firewall de Windows para `rookparcel.exe`. |
| **Usar HTTPS (certificado autofirmado)** | Sirve la interfaz web por HTTPS con un certificado que crea el propio Rookparcel. Los navegadores avisan de ello hasta que lo acepte. |
| **Dominios del servidor** | Nombres de host a los que responde la interfaz web además de las direcciones IP y localhost, como el nombre de este equipo para una aplicación en Docker. |
| **Abrir la interfaz web** | Abre en su navegador la interfaz web en ejecución. |

Más información: [Interfaz web](Interfaz-web), [Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr).

## Juegos

*Novedad de la 1.2.79.* Ajustes que mantienen el equipo y la línea con buena respuesta mientras Rookparcel trabaja. **Mantener el ping bajo** pasó aquí desde Ancho de banda, y la antigua página E/S de disco, desde Avanzado.

**Ping**

| Ajuste | Qué hace |
|---|---|
| **Mantener el ping bajo (limitar las velocidades automáticamente)** | Mide el retardo de su línea y limita automáticamente las subidas y las bajadas mientras crece, para juegos y llamadas. Nunca sube sus propios límites; la barra de estado muestra el tope como "auto". |

**Comprobación**

| Ajuste | Qué hace |
|---|---|
| **Comprobar los torrents con suavidad (prioridad baja, ceder el paso a otros programas)** | Comprueba los torrents con la prioridad de disco en segundo plano del sistema y hace una pausa mientras las lecturas son más lentas de lo habitual, para que el equipo siga respondiendo. Las comprobaciones tardan más mientras el disco está ocupado. |
| **Limitar la comprobación a (MB/s):** | Limita la velocidad de lectura de cada comprobación, para un disco que aún se ralentiza durante las comprobaciones suaves. Desactivado de forma predeterminada. |
| **Unidades comprobadas a la vez (un torrent por unidad):** | Cuántas unidades se comprueban al mismo tiempo; los torrents de la misma unidad esperan su turno. -1 significa sin límite. |

**Unidades**

| Ajuste | Qué hace |
|---|---|
| **Ajustar según el tipo de unidad (SSD, disco duro, red)** | Detecta si cada carpeta de destino está en un SSD, un disco duro, una unidad extraíble o un recurso compartido de red, y ajusta cuántas tareas de disco se ejecutan a la vez. |

**Cuando el disco está ocupado**

| Ajuste | Qué hace |
|---|---|
| **Ralentizar las bajadas (escrituras en disco)** | Mientras una unidad responde a las escrituras con claridad más despacio de lo habitual, reduce a la mitad la velocidad de bajada paso a paso, sin bajar nunca de 50 kB/s, y la deja crecer de nuevo cuando el disco se calma. |
| **Ralentizar las subidas (lecturas de disco)** | Lo mismo para las subidas mientras una unidad responde despacio a las lecturas. |

Un tipo de unidad detectado mal se corrige con `drive_overrides` en la página Avanzado. Al principio solo está activada la casilla **Ajustar según el tipo de unidad (SSD, disco duro, red)**; las demás están desactivadas.

Más información: [Mantener el ping bajo](Mantener-el-ping-bajo), [Disco y almacenamiento](Disco-y-almacenamiento).

## Avanzado

![Preferencias, página Avanzado](images/prefs-advanced.png)

La página Avanzado es una tabla de los ajustes propios del motor, con sus nombres de libtorrent. Escriba parte de un nombre en **Filtro:** para acotar la lista. Seleccione una fila, escriba un valor nuevo en **Valor:** (o elija **Verdadero** o **Falso** para un interruptor) y haga clic en **Establecer**. **Restablecer** devuelve el ajuste seleccionado a su valor predeterminado. Un `*` marca un valor que difiere del predeterminado. Como en todo Preferencias, el cambio surte efecto con **Aceptar** o **Aplicar**. Aquí las velocidades están en bytes por segundo, no en kB/s.

La mayoría de las filas son los mismos ajustes que muestran las demás páginas con nombres más amigables. El resto se reparte en estos grupos:

| Grupo | Ejemplos |
|---|---|
| Identidad | `user_agent`, `peer_fingerprint` (rara vez hace falta cambiarlos) |
| Escucha y asignación de puertos | `listen_interfaces`, `max_retry_port_bind`, `upnp_lease_duration`, `natpmp_gateway` |
| Cola | `active_dht_limit`, `active_tracker_limit`, `inactive_down_rate`, `auto_manage_interval` |
| Conexiones y pares | `connection_speed`, `max_peerlist_size`, `peer_timeout`, `peer_turnover` |
| DHT | `dht_announce_interval`, `dht_upload_rate_limit`, `dht_bootstrap_nodes` y sus interruptores de protección |
| uTP y TCP | `enable_incoming_tcp`, `utp_target_delay`, `mixed_mode_algorithm`, `bt.transp_disposition` |
| Cifrado | `out_enc_policy`, `in_enc_policy`, `allowed_enc_level`, `prefer_rc4` |
| Trackers | `announce_to_all_tiers`, `announce_to_all_trackers`, `num_want`, `stop_tracker_timeout` |
| Disco | `cache_size`, `max_queued_disk_bytes`, `file_pool_size`, `drive_overrides` |
| Semillas web | `max_web_seed_connections`, `urlseed_timeout`, `ssrf_mitigation` |
| Proxy | `proxy_type`, `proxy_send_host_in_connect`, `socks5_udp_send_local_ep` |
| Mantener el ping bajo | `auto_limit_delay_ms`, `auto_limit_probe_hosts` |

Filas que a menudo conviene conocer:

| Ajuste | Qué hace |
|---|---|
| `listen_interfaces` | Direcciones y puertos, separados por comas, en los que aceptar pares. `0.0.0.0` y `[::]` significan todas las interfaces. |
| `connection_speed` | Intentos de conexión saliente nuevos por segundo, compartidos por todos los torrents. Un valor más alto encuentra pares más deprisa, pero puede sobrecargar routers débiles. Valor predeterminado: 30. |
| `announce_to_all_trackers` | Activado: se contacta con todos los trackers de un nivel, no solo con el primero que funciona. Puede encontrar más pares a costa de más tráfico con los trackers. Desactivado de forma predeterminada. |
| `upnp_lease_duration` | Segundos por los que se pide una asignación de puerto UPnP. 0 pide una asignación permanente, que algunos routers necesitan. Valor predeterminado: 3600. |
| `stop_tracker_timeout` | Segundos que Rookparcel espera al salir a los trackers y a los routers. Valor predeterminado: 5. Redúzcalo para salir antes. |
| `auto_limit_delay_ms` | Milisegundos de ping adicional que Mantener el ping bajo permite por encima del ping de la línea en reposo. Un valor más bajo mantiene los juegos más fluidos; uno más alto permite más velocidad. Valor predeterminado: 15. |
| `drive_overrides` | Corrige las unidades cuyo tipo se detecta mal, como pares ruta=tipo, por ejemplo `D:/=hdd, /mnt/nas=network`. |
| `bt.transp_disposition` | Tipos de conexión permitidos como bits que se suman: 1 TCP saliente, 2 uTP saliente, 4 TCP entrante, 8 uTP entrante. El valor predeterminado 15 los permite todos; 5 es solo TCP. |

La ayuda completa de cada fila aparece en el panel bajo la tabla cuando la selecciona.

## Caché de disco

| Ajuste | Qué hace |
|---|---|
| **Activar la caché de lecturas del disco** | Conserva en memoria las piezas leídas hace poco, lo que ahorra búsquedas en el disco al sembrar. Las escrituras se dejan a la caché del sistema. |
| **Reemplazar el tamaño automático de la caché (MB):** | Usa este tamaño en lugar de los 32 MB automáticos para la caché de lectura. Una caché mayor puede ayudar en siembras con mucha actividad, a costa de memoria. |
| **Quitar los bloques sin usar durante más de (segundos):** | Descarta los bloques de la caché que ningún par ha pedido en ese tiempo. Desactivado, permanecen hasta que los desplazan bloques más nuevos. |
| **Cola de escritura en disco (kB):** | Cuántos datos descargados pueden esperar para escribirse; por encima de eso, Rookparcel deja de leer de los pares hasta que el disco se pone al día. Los valores inferiores a 4 MB actúan como 4 MB. |
| **Número máximo de archivos abiertos:** | Lo máximo de archivos que se mantienen abiertos a la vez. Auméntelo cuando muchos torrents o archivos estén activos al mismo tiempo. |
| **Hilos de E/S de disco (desde el próximo inicio):** | Los hilos que leen y escriben los datos de los torrents. |
| **Hilos de hash (no se usa):** | Se conserva por compatibilidad; esta versión no lo usa. Las comprobaciones se hacen una por unidad, como se establece en la página Juegos. |

Más información: [Disco y almacenamiento](Disco-y-almacenamiento).

## Trackers

*Novedad de la 1.2.40.* Trackers que reciben, además de los suyos, todos los torrents que añada a partir de ahora. Las dos casillas están desactivadas al principio.

| Ajuste | Qué hace |
|---|---|
| **Añadir estos trackers a las nuevas descargas:** | Añade los trackers del cuadro, una dirección por línea (`http://`, `https://` o `udp://`); una línea en blanco inicia un nivel nuevo. Van después de los trackers propios del torrent. |
| **Añadir los trackers de esta lista de la web (https):** | Lee un archivo de texto con direcciones de trackers de la dirección `https://` de abajo, al iniciar y luego una vez al día, y añade sus trackers después de los del cuadro. |
| **Actualizar ahora** | Lee ahora la lista de la dirección. La línea bajo el botón indica cuántos trackers tiene la lista y cuándo se leyó, o por qué falló el último intento. |

Conviene saber:

- **Los torrents privados nunca reciben estos trackers**, ni tampoco los torrents que ya están en la lista, los torrents que usted crea o los importados de otro cliente. Un enlace magnet los recibe cuando han llegado sus detalles y muestran que no es privado.
- **La lista de la web solo se lee como lista.** La dirección debe empezar por `https://`. Solo se usan las líneas que son direcciones de trackers, como máximo 200, y un archivo que es otra cosa (una página de error, un programa) se ignora: sigue en uso la última lista válida. Nada del archivo se ejecuta jamás.
- **Quien administra esa dirección elige sus trackers**, y esos trackers ven entonces su dirección y sus descargas públicas. Use una lista en la que confíe.
- En la interfaz web, los ajustes de la página solo cambian con la contraseña actual, y la API compatible con qBittorrent no puede cambiarlos.
- `rookparcel-nox` guarda estos ajustes junto con los de la interfaz web, así que necesita `--webui` para usarlos.

## Ejecutar programa

*Novedad de la 1.2.41.* Inicia un programa suyo cuando se añade un torrent o termina. Las dos casillas están desactivadas al principio.

| Ajuste | Qué hace |
|---|---|
| **Ejecutar este programa cuando se añade un torrent:** | Inicia el programa de la línea de abajo por cada torrent nuevo. No para uno que ya estaba en la lista, ni para los torrents cargados al iniciar. |
| **Ejecutar este programa cuando un torrent termina de descargarse:** | Inicia el programa por cada torrent que termina. No después de una nueva comprobación ni de un reinicio, ni para un torrent cuyos datos estaban completos cuando lo añadió. |

Escriba primero la ruta del programa, entre comillas dobles si tiene espacios, y después sus argumentos, por ejemplo `"C:\Tools\notify.exe" --name "%N" --path "%F"`. En los argumentos, estos códigos representan los datos del torrent:

| Código | Representa |
|---|---|
| `%N` | el nombre del torrent |
| `%L` | su etiqueta |
| `%F` | dónde están sus datos: el archivo, o la carpeta de un torrent con varios archivos |
| `%D` | la carpeta en la que se guardó |
| `%Z` | su tamaño en bytes |
| `%T` | su tracker actual |
| `%I`, `%J` | su hash de información, v1 y v2 (`-` cuando no tiene) |
| `%K` | su id: el hash v1, o el hash v2 abreviado |
| `%%` | un signo de porcentaje |

Conviene saber:

- **El programa se inicia directamente, sin un intérprete de comandos.** Las canalizaciones, `>` y `&&` no hacen nada; para usarlos, ejecute un script y póngalos en el script. Para ejecutar un script, indique primero su intérprete donde el sistema lo necesite (`powershell -File ...`, `/bin/sh /path/script.sh`).
- **El nombre de un torrent nunca puede convertirse en un comando.** Cada código se coloca dentro de su propio argumento, con comillas o sin ellas, y la ruta del propio programa no admite códigos. Un nombre puede empezar por un guion, que algunos programas leen como una opción: escriba `--name=%N` o ponga `--` antes donde importe.
- **No se admiten códigos para un intérprete de comandos o de lenguaje al que se le diga que ejecute una cadena.** Un programa así leería el nombre como comandos, de modo que una línea como `cmd /c ... %N`, `powershell -Command ...`, `sh -c "... %N"`, `python -c`, `perl -e`, `node -e`, `osascript -e`, `ssh host ... %N` o `wsl bash -c` se rechaza si lleva un código, también detrás de `env`, `nohup`, `nice`, `timeout`, `sudo` y envoltorios similares (cuyo programa tampoco admite códigos). Lo mismo ocurre con un código en el texto de un programa de `awk` o `sed`, en una orden `find -exec` que ejecuta un intérprete, y después de un intérprete que inicia otro programa (`gnome-terminal -- bash -c "... %N"`). Los programas que ejecutan como código una cadena de su línea se rechazan también con un código en cualquier parte de ella: `script`, `expect`, `forfiles`, `wmic`, editores como `vim` y `emacs`, y depuradores como `gdb`. Indique en su lugar el script y pásele los códigos: `powershell -File C:\Tools\done.ps1 "%N"`, `/bin/sh /path/done.sh "%N"`, `python /path/done.py "%N"`, `wsl -e /path/done.sh "%N"`. Las opciones propias de otros programas son cosa suya: `notify-send -e "%N"`, `rsync -c "%F" /backup` y `/path/done.sh -c "%L"` son válidos. Una línea guardada por una versión anterior que ahora se rechaza se desactiva, y el Registro lo indica (la línea y el motivo) al iniciar.
- El programa no recibe ventana ni entrada, se ejecutan como máximo cuatro a la vez, y cada inicio y cada fin se anotan en el Registro.
- El programa se ejecuta en el equipo que tiene los torrents: en una ventana conectada a un `rookparcel-nox` remoto, las líneas de esta página no hacen nada; defínalas en el servidor.
- **Quién puede cambiar las líneas.** Quien pueda cambiarlas puede ejecutar programas con su cuenta, por eso: los programas de escritorio, en su propio equipo; la interfaz web, solo desde un navegador del mismo equipo y con la contraseña actual; nunca la API compatible con qBittorrent, y nunca una conexión remota. Un servidor al que solo se puede llegar desde otro lugar (Docker, por ejemplo) necesita `--webui-remote-run-program` en su línea de comandos: vea [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox). Piénselo antes de activarlo: con él, la contraseña de la interfaz web es lo único que separa a un visitante de la ejecución de programas en el servidor.
- **Detrás de un proxy inverso.** Si se llega a la interfaz web a través de un proxy o un túnel en el propio servidor (nginx, HAProxy, stunnel, un túnel SSH), el servidor ve a todos los visitantes como un navegador del mismo equipo, salvo que el proxy diga otra cosa. Haga que el proxy añada la dirección del visitante (`X-Forwarded-For`; para nginx: `proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;`). Sin esa cabecera, cualquiera que llegue al proxy y conozca la contraseña puede cambiar estas líneas, exactamente como con `--webui-remote-run-program`.

## Bloqueo de la interfaz

El PIN que bloquea las ventanas de Rookparcel y el interruptor que desactiva el bloqueo. A diferencia del resto de Preferencias, todo en esta página surte efecto al instante: **Aceptar**, **Aplicar** y **Cancelar** no lo cambian. *Novedad de la 1.2.63; antes, el PIN se establecía en **Opciones > Lock PIN**.*

| Ajuste | Qué hace |
|---|---|
| **Permitir bloquear Rookparcel con un PIN** | Activado de forma predeterminada. Desactivado, no hay **Archivo > Bloquear Rookparcel**, ni <kbd>Ctrl</kbd>+<kbd>L</kbd>, ni **Bloquear Rookparcel** en el menú de la bandeja. Al desactivarlo se pide el PIN, si hay uno establecido, y se quita. |
| **Establecer PIN...** | Pide un PIN nuevo de 4 a 6 dígitos, dos veces. Solo mientras no haya ningún PIN establecido. |
| **Cambiar PIN...** | Pide el PIN actual y después uno nuevo dos veces. |
| **Quitar PIN...** | Pide el PIN actual y luego lo quita. **Bloquear Rookparcel** pedirá entonces un PIN nuevo antes de bloquear. |

La línea bajo los botones indica si hay un PIN establecido. Más información: [Bloquear Rookparcel](Bloquear-Rookparcel).

## Diferencias en la versión GTK

La versión GTK (Linux, BSD) tiene las mismas páginas y ajustes, con estas diferencias:

- No hay página **Interfaz web**.
- **Interfaz** no tiene el ajuste **Tema**: GTK sigue el tema del escritorio ([Modo oscuro](Modo-oscuro)). Las casillas de la bandeja y las notificaciones (**Mostrar notificaciones en la bandeja**, **Cerrar a la bandeja**, **Minimizar a la bandeja**) solo se muestran mientras hay un icono de bandeja en pantalla.
- **General** tiene **Integración con el escritorio** en lugar de Integración con Windows, y no tiene el botón **Aplicaciones predeterminadas...**. En un Flatpak las casillas de asociación aparecen en gris bajo una nota: elija el programa predeterminado en los ajustes de su escritorio. El grupo **Privacidad** no se muestra en un Flatpak, que Flathub actualiza.
- <kbd>Escape</kbd> cierra la ventana como **Cancelar**.

## Diferencias en macOS

La aplicación de macOS abre Preferencias desde **Rookparcel > Preferencias** (<kbd>⌘</kbd>+<kbd>,</kbd>). Tiene las páginas de la versión GTK, enumeradas en una barra lateral, y estas diferencias:

- Cada cambio se aplica al instante, como en otras aplicaciones del Mac: no hay **Aceptar**, **Cancelar** ni **Aplicar**. Una casilla o una lista se aplica cuando la cambia; un campo de texto o de número, cuando pulsa <kbd>Return</kbd>, sale del campo o cierra la ventana.
- Un valor que no se permite, como un puerto fuera de rango, se muestra en rojo bajo su campo, y se sigue usando el último valor válido. Al cerrar la ventana se restablece en el campo el valor válido.
- Las casillas de asociación solo funcionan en `Rookparcel.app` y solo en el perfil predeterminado.

*La aplicación nativa de macOS es novedad de la 1.2.19.*

## Diferencias en la interfaz web

La interfaz web (de `rookparcel-nox`, o la que sirve la ventana de Windows) tiene una ventana **Preferencias** con las mismas páginas. Los valores cambiados de su página Avanzado se muestran en negrita en lugar de con un `*`. Otras diferencias:

- **General**: las casillas de asociación y de inicio automático aparecen en gris con la nota "Las asociaciones de archivos y el inicio automático son cosa del cliente de escritorio". `rookparcel-nox` tiene su propio grupo **Privacidad** para su búsqueda de actualizaciones; la interfaz web que sirve una ventana de Windows no lo tiene.
- **Interfaz** solo tiene **Confirmar al borrar torrents**, los dos ajustes de tema, **No iniciar la descarga automáticamente**, **Mostrar una ventana con los archivos del torrent** y **No descargar los archivos llamados:**.
- **BitTorrent**: cambiar la dirección del filtro IP, o la casilla que hay sobre ella, necesita la contraseña actual, que se escribe en **Contraseña actual:** en la página Interfaz web. La interfaz web que sirve la ventana de Windows no tiene dirección, **Actualizar ahora** ni línea de estado: las Preferencias de la propia ventana los contienen.
- **Ancho de banda** en `rookparcel-nox` tiene **Límites alternativos**: **Velocidad de subida alternativa**, **Velocidad de bajada alternativa** y **Usar ahora los límites alternativos**, que también cambian las aplicaciones que alternan el modo de límites de velocidad de qBittorrent.
- **Cola** tiene la casilla **Detener el torrent o limitar su velocidad de subida** sobre **Limitar la velocidad de subida a (kB/s): [0: detener]**; la casilla pone en vigor el objetivo de siembra.
- **Interfaz web** añade **Contraseña actual:** (necesaria para cambiar el inicio de sesión y los ajustes de seguridad), **Omitir la autenticación para clientes en localhost**, **Dirección de escucha:** (se muestra a título informativo), **Activar la validación de la cabecera Host**, **Activar la protección contra falsificación de peticiones entre sitios (CSRF)**, y un grupo **Seguridad** con **Bloquear al cliente tras fallos consecutivos:**, **Bloquear durante (segundos):** y **Caducidad de la sesión (segundos):**.
- <kbd>Ctrl</kbd>+<kbd>Enter</kbd> funciona como **Aceptar**; <kbd>Enter</kbd> en el campo de valor de Avanzado establece el valor.

Vea [Interfaz web](Interfaz-web) y [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).
