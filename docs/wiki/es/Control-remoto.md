*Esta página es una traducción de la página inglesa [Remote control](Remote-control). Si hay diferencias, prevalece la página en inglés.*

# Control remoto

El programa de escritorio puede controlar un `rookparcel-nox` que se ejecuta en otro equipo: un NAS, un servidor doméstico, un PC de otra habitación. La ventana tiene el aspecto y el funcionamiento habituales, pero los torrents, las descargas y el motor están en el servidor. Esta página explica cómo conectarse con **Archivo > Conectar en remoto...** o con `--remote`, qué funciona en una ventana remota y cómo se protege la conexión.

La alternativa es la [Interfaz web](Interfaz-web) en un navegador, que no necesita instalar nada. Una ventana remota le da en cambio el programa de escritorio completo: sus cuadros de diálogo, sus atajos de teclado y sus pestañas de detalles.

## Qué necesita

- Un `rookparcel-nox` en ejecución con su interfaz web activada, por ejemplo `rookparcel-nox run --webui 0.0.0.0:8443 --webui-https` ([Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox)). La ventana remota usa el puerto y el inicio de sesión de la interfaz web.
- El nombre de usuario y la contraseña de la interfaz web.
- La misma versión de Rookparcel en ambos lados. Un servidor que habla otra versión rechaza la conexión con «the server refused the connection (HTTP 426): this server speaks remote protocol ...». El protocolo remoto cambia de vez en cuando (por última vez en 1.2.70), y 1.2.76 no se conecta con 1.2.63 a 1.2.69, así que actualice juntos el servidor y los equipos con ventanas remotas. La interfaz web y la API compatible con qBittorrent no se ven afectadas.

Una ventana remota se conecta a `rookparcel-nox`. La interfaz web propia del programa de Windows (**Preferencias > Interfaz web**) es para navegadores.

## Conectar

1. En el programa de Windows elija **Archivo > Conectar en remoto...**. Se inicia una segunda ventana de Rookparcel junto a la suya y muestra el cuadro de diálogo **Conectar en remoto**. (O inicie `rookparcel.exe --remote`, que abre solo la ventana remota.)
2. Rellene:

   | Campo | Qué escribir |
   |---|---|
   | **Servidor:** | El nombre o la dirección IP del servidor. |
   | **Puerto:** | El puerto de la interfaz web (se propone 8080). |
   | **Usuario:** | El nombre de usuario de la interfaz web (se propone `admin`). |
   | **Contraseña:** | La contraseña de la interfaz web. |
   | **Usar HTTPS** | Marcada de forma predeterminada. Déjela marcada cuando el servidor se ejecute con `--webui-https` o con su propio certificado. |

3. Haga clic en **Conectar**.
4. La primera vez, un servidor con un certificado autofirmado (el que crea `--webui-https`) se muestra con su huella:

   > ... presenta un certificado en el que este equipo no confía (uno autofirmado, como el que crea rookparcel-nox --webui-https).

   Compare el número largo con la línea `certificate SHA-256` que nox mostró al iniciarse (en su terminal, registro o diario). Si coinciden, haga clic en **Sí**. Rookparcel recuerda el certificado de ese servidor y no vuelve a preguntar.
5. La ventana se abre con los torrents del servidor. Su barra de título muestra el `host:port` del servidor.

El cuadro de diálogo recuerda para la próxima vez el servidor, el puerto, el nombre de usuario y la elección de HTTPS. La contraseña nunca se guarda; la escribe en cada conexión.

En el programa GTK (Linux, BSD) es igual: **Archivo > Conectar en remoto...**, o `rookparcel-gtk --remote`. La aplicación de macOS también tiene **Archivo > Conectar en remoto...**. Cada ventana remota es un programa independiente, de modo que puede tener abiertos a la vez su ventana local y varios servidores.

## Sin HTTPS

Si desmarca **Usar HTTPS** y el servidor es otro equipo, Rookparcel pregunta primero:

> ... es otro equipo y, sin HTTPS, el nombre de usuario, la contraseña y cada orden viajan por la red sin cifrar: cualquiera en ella puede leerlos o cambiarlos.

Responda **No** y active HTTPS en el servidor (`--webui-https`), salvo que la red entre ambos sea enteramente suya. Una conexión al mismo equipo (`localhost`, `127.0.0.1`) no genera esta pregunta.

## Qué funciona en una ventana remota

Casi todo funciona como en una ventana local, actuando sobre el servidor:

- Añadir torrents: los archivos `.torrent` de este equipo se envían al servidor; los enlaces magnet también.
- Iniciar, detener, pausar, forzar inicio, quitar (también con los datos, que se borran en el servidor), comprobar de nuevo, orden de la cola, etiquetas, marcas, límites de velocidad, trackers, prioridades de archivos, renombrar, Propiedades del torrent.
- Las pestañas de detalles, con los pares, las piezas, los archivos y el gráfico de velocidad del servidor.
- **Copiar enlace magnet** y **Exportar .torrent...** (*nuevo en 1.2.63*). Un archivo `.torrent` exportado se guarda en este equipo.
- **Preferencias:** los ajustes del motor (conexión, ancho de banda, BitTorrent, cola y similares) son los del servidor. Lo que cambia allí se aplica en el servidor y se conserva allí.

Qué es distinto:

| | En una ventana remota |
|---|---|
| Carpetas | Las carpetas están en el servidor. Los selectores de carpeta para torrents están desactivados; **Establecer ubicación de descarga...** pide una carpeta del servidor como texto. Las descargas nuevas van a la carpeta predeterminada del servidor. |
| **Abrir carpeta contenedora** | Desactivado: la carpeta está en el servidor. |
| Crear torrents | Un torrent que cree aquí no puede ser sembrado por el servidor, ya que los archivos están en este equipo. |
| Filtro IP | Se aplica el filtro IP propio del servidor; el de la ventana no se envía. |
| Importar | **Archivo > Importar** no hace nada (en gris en los programas de Windows y macOS): el servidor no toma torrents del estado guardado de otro cliente. |
| Límites de carpetas | Un servidor iniciado con `--webui-save-root` rechaza añadir o mover a una carpeta fuera de esas raíces, igual que para la interfaz web. También se rechaza un renombrado que sacaría al torrent de su carpeta. |
| **Preferencias > Interfaz web** | Una ventana remota no puede servir la interfaz web: «Una ventana remota no puede servir la interfaz web; use las Preferencias de la ventana local.» |
| Etiquetas, marcas, fuentes RSS, ajustes de la ventana | Se conservan en este equipo, por separado para cada servidor (vea más abajo). |
| Programador, Descargador RSS, objetivo de siembra, carpetas de `.torrent` | Los de la propia ventana se ejecutan mientras está abierta y actúan sobre los torrents del servidor, además de lo que haga el propio nox. |

El servidor sigue descargando después de que usted cierre la ventana.

## Perder la conexión

Cuando se pierde la conexión, Rookparcel lo dice una vez:

> Se perdió la conexión con ...: ... La lista muestra los torrents tal como estaban. Cierre esta ventana y use Archivo > Conectar en remoto... para volver a conectar.

La barra de título termina entonces en «(desconectado)». A partir de ahí las órdenes no van a ninguna parte. Cierre la ventana y vuelva a conectar.

## Cuando falla la conexión

| Mensaje | Qué comprobar |
|---|---|
| «Escriba el nombre o la dirección del servidor y su puerto (de 1 a 65535).» | El servidor está vacío o el puerto no es un número dentro del intervalo. |
| «No se puede conectar con ...: wrong user name or password» | El inicio de sesión de la interfaz web. Demasiados intentos erróneos bloquean la dirección durante un tiempo ([Interfaz web](Interfaz-web#iniciar-sesión)). |
| «... ¿Es un rookparcel-nox con su interfaz web en este puerto (--webui)?» | Otra cosa respondió en ese puerto. |
| «... Si la interfaz web del servidor usa HTTPS (--webui-https), marque Usar HTTPS.» | El servidor habla HTTPS pero **Usar HTTPS** estaba desactivada. |
| «No se puede llegar a ... por HTTPS: ...» | **Usar HTTPS** está activada, pero el servidor no habla HTTPS, o no se puede alcanzar en absoluto. |
| «El certificado de ... no es el que aceptó antes: ...» | Cambió el certificado del servidor. Si lo reemplazó, o nox renovó el suyo, vuelva a conectar para comprobar el nuevo. Si no, puede haber alguien en medio: no continúe. |
| «... (HTTP 400): ...» con el nombre de la cabecera Host | El servidor rechaza el nombre de host que escribió. Use su dirección IP, o añada el nombre a **Preferencias > Interfaz web > Dominios del servidor** del servidor ([Interfaz web](Interfaz-web#seguridad)). |
| «... (HTTP 426): this server speaks remote protocol ...» | Los dos programas son de versiones distintas. Actualice el más antiguo ([Actualizar Rookparcel](Actualizar-Rookparcel)). |

El cortafuegos del servidor debe dejar entrar el puerto de la interfaz web.

## Dónde guarda sus archivos la ventana remota

| | Windows | GTK (Linux, BSD) |
|---|---|---|
| Datos de conexión y certificados aceptados | `remote.ini` junto a `rookparcel.ini` en la carpeta de ajustes | `remote.ini` en `~/.config/rookparcel/gtk` |
| Los ajustes, etiquetas, marcas y fuentes RSS de la ventana, por servidor | `remote\<host>_<port>` en la carpeta de ajustes | `~/.config/rookparcel/gtk/remote/<host>_<port>` |

Estos nunca se mezclan con los ajustes de su ventana local. Vea [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos).

## Seguridad

- La ventana remota inicia sesión con el inicio de sesión de la interfaz web, de modo que todo lo que dice [Seguridad de la interfaz web](Interfaz-web#seguridad) se aplica: una contraseña larga, HTTPS a través de una red.
- Las contraseñas equivocadas cuentan para el mismo bloqueo que el formulario de inicio de sesión de la interfaz web.
- Definir una contraseña nueva de la interfaz web desconecta todas las ventanas remotas.
- Un certificado que cambia después de que usted lo aceptó se rechaza con un aviso; Rookparcel vuelve a preguntar en el siguiente intento.
- La contraseña nunca se escribe en el disco de este equipo.

## Véase también

- [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox): iniciar el servidor, opciones de HTTPS y servicios.
- [Interfaz web](Interfaz-web): el mismo servidor en un navegador.
- La descripción técnica: [WEBUI.md, Remote mode](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/WEBUI.md#remote-mode).
