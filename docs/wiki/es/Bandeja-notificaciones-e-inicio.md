*Esta página es una traducción de [Tray, notifications and startup](Tray-notifications-and-startup). La página en inglés es la versión de referencia.*

# Bandeja, notificaciones e inicio

Esta página trata del icono de la bandeja de Rookparcel, de cerrar y minimizar a la bandeja, de la notificación cuando termina una descarga, de iniciar Rookparcel al iniciar sesión y de hacer de Rookparcel el programa que abre los archivos `.torrent` y los enlaces `magnet:`.

## El icono de la bandeja (Windows)

Mientras Rookparcel se ejecuta, su icono está en el área de notificación de la barra de tareas. Al apuntar a él se muestran la versión y las velocidades actuales de bajada y subida.

- Un clic izquierdo (o un doble clic) oculta la ventana principal o la recupera.
- Un clic derecho abre el menú de la bandeja:

| Elemento | Qué hace |
|---|---|
| **Ocultar Rookparcel** / **Mostrar Rookparcel** | Oculta la ventana principal o la muestra de nuevo |
| **Añadir torrent...** | Abre un archivo `.torrent`, como hace **Archivo > Añadir torrent...** |
| **Añadir torrent desde URL...** | Añade un enlace magnet o una dirección web |
| **Iniciar todos los torrents** | Inicia todos los torrents de la lista |
| **Detener todos los torrents** | Detiene todos los torrents de la lista |
| **Bloquear Rookparcel** | Oculta todas las ventanas de Rookparcel hasta que se escriba el PIN de bloqueo; se muestra cuando hay un PIN establecido |
| **Salir** | Cierra Rookparcel |

Mientras Rookparcel está bloqueado, el menú solo tiene **Mostrar Rookparcel**, que pide el PIN, y **Salir**. Vea [Bloquear Rookparcel](Bloquear-Rookparcel).

## Cerrar y minimizar a la bandeja

Dos ajustes deciden qué pasa con la ventana:

| Ajuste | Valor predeterminado | Qué hace |
|---|---|---|
| **Cerrar a la bandeja** | Activado | El botón de cierre de la ventana (y <kbd>Alt</kbd>+<kbd>F4</kbd>) oculta Rookparcel en la bandeja en lugar de salir, de modo que las transferencias continúan. |
| **Minimizar a la bandeja** | Desactivado | Al minimizar, la ventana se oculta en la bandeja en lugar de quedar en la barra de tareas. |

Defínalos en **Opciones > Preferencias > Interfaz** bajo **Bandeja del sistema**, o márquelos directamente en el menú **Opciones** (**Cerrar a la bandeja** y **Minimizar a la bandeja**).

Para cerrar Rookparcel con **Cerrar a la bandeja** activado, use **Archivo > Salir** o **Salir** en el menú de la bandeja. Rookparcel detiene entonces sus torrents y guarda su progreso. Si esto tarda más de un momento (por ejemplo, mientras aún se están moviendo archivos a otra unidad), una ventana muestra qué está esperando, con un botón **Salir ahora**.

La interfaz web que sirve la ventana de Windows funciona solo mientras Rookparcel se ejecuta, así que cerrar a la bandeja la mantiene disponible. Vea [Interfaz web](Interfaz-web).

## Notificación al completarse una descarga

Con **Mostrar notificaciones en la bandeja** activado (el valor predeterminado), Rookparcel muestra una notificación "Descarga completada" con el nombre del torrent cada vez que termina una descarga. Desactívelo en **Opciones > Preferencias > Interfaz**. Mientras Rookparcel está [bloqueado](Bloquear-Rookparcel), la notificación no aparece.

Cuando hay una actualización disponible, Rookparcel lo indica en cambio en la barra de estado; vea [Actualizar Rookparcel](Actualizar-Rookparcel).

## Mostrar la velocidad en la barra de título

**Mostrar la velocidad actual en la barra de título** (Preferencias > Interfaz, desactivado de forma predeterminada) pone las velocidades de bajada y subida al principio del título de la ventana, para que pueda leerlas en el botón de la barra de tareas.

## Iniciar Rookparcel al iniciar sesión

1. Abra **Opciones > Preferencias > General**.
2. Marque **Iniciar Rookparcel al arrancar el sistema**.
3. Para que arranque en la bandeja sin mostrar su ventana, marque también **Iniciar minimizado**.
4. Haga clic en **Aceptar**.

En Windows esto es una entrada en los programas de inicio de su cuenta de usuario. No se necesitan derechos de administrador, y una copia portátil también puede usarla. Si mueve una copia portátil a otra carpeta, vuelva a marcar la casilla: solo está marcada mientras la entrada inicia esta misma copia.

Con [perfiles](Perfiles), cada perfil tiene su propia entrada de inicio, de modo que puede iniciar varios al iniciar sesión. *Las entradas por perfil son novedad de la 1.1.46.*

La [Guía de configuración](Primer-inicio) ofrece la misma opción en su última página.

## Abrir archivos .torrent y enlaces magnet con Rookparcel

1. Abra **Opciones > Preferencias > General**.
2. Marque **Asociar con archivos .torrent** y **Asociar con enlaces magnet:**.
3. Haga clic en **Aceptar**.

Al hacer doble clic en un archivo `.torrent`, o al hacer clic en un enlace magnet en su navegador, este se entrega entonces a Rookparcel. Si Rookparcel ya se está ejecutando, lo recibe la copia en ejecución. Con **Mostrar una ventana con los archivos del torrent** activado, se abre la ventana Añadir nuevo torrent; con **Activar la ventana del programa** activado, la ventana principal también pasa al frente. Vea [Añadir torrents](Añadir-torrents).

Conviene saber en Windows:

- Los ajustes son solo para su cuenta de usuario y no necesitan derechos de administrador. El instalador también puede definirlos.
- Si eligió otro programa en el propio Windows ("Usar siempre esta aplicación", o Configuración > Aplicaciones predeterminadas), Windows conserva esa elección, y solo usted puede cambiarla allí. Rookparcel indica entonces qué programa sigue usando Windows, por ejemplo "Windows aún abre los archivos .torrent con ...". Haga clic en **Aplicaciones predeterminadas...** para abrir el lugar correcto de la Configuración de Windows y elegir Rookparcel.
- Con perfiles, solo un perfil puede abrir los archivos `.torrent` y los enlaces magnet. Al marcar las casillas en un perfil, se los quita al otro; la página indica el perfil que los tiene ahora.
- **Comprobar asociación al iniciar** (desactivado de forma predeterminada) hace que Rookparcel pregunte en cada inicio si debe recuperar las asociaciones cuando otro programa las tiene. No pregunta cuando Rookparcel arranca minimizado al iniciar sesión.

## Versión GTK (Linux, BSD)

**La bandeja.** Rookparcel muestra un icono de bandeja en los escritorios que tienen una: KDE Plasma, Xfce, LXQt, Cinnamon, MATE, waybar y GNOME con la extensión AppIndicator. Su menú tiene los mismos elementos que en Windows. Si la bandeja desaparece (por ejemplo, porque se reinicia el panel) mientras Rookparcel está oculto en ella, Rookparcel muestra la ventana principal, o la ventana del PIN cuando está bloqueado, de modo que nada se quede sin forma de volver. *Novedad de la 1.2.76.* En GNOME sin esa extensión no hay bandeja: las casillas de la bandeja no aparecen en Preferencias y cerrar la ventana cierra Rookparcel. <kbd>Ctrl</kbd>+<kbd>Q</kbd> cierra desde la ventana.

**Minimizar a la bandeja** necesita que el escritorio indique que la ventana se minimizó; donde no lo hace (algunos escritorios Wayland), minimizar sigue siendo una minimización normal.

**Las notificaciones** pasan por el propio servicio de notificaciones del escritorio, con el mismo texto "Descarga completada".

**Inicio al iniciar sesión.** **Iniciar Rookparcel al arrancar el sistema** escribe una entrada de inicio automático en la carpeta `~/.config/autostart` de su usuario. En un Flatpak, en cambio, se lo pide al escritorio mediante su permiso de segundo plano, y el escritorio puede pedirle que lo permita; allí solo el perfil predeterminado puede iniciarse al iniciar sesión.

**Asociaciones.** Las casillas hacen de Rookparcel el programa predeterminado para los archivos `.torrent` y los enlaces `magnet:` en los ajustes de su usuario. Al desmarcar una casilla, el tipo vuelve al programa predeterminado del sistema; no restablece un programa que usaba antes. Si mueve un AppImage, vuelva a marcar las casillas. En un Flatpak las casillas aparecen en gris: el escritorio ya ofrece Rookparcel para ambos tipos, y usted elige el programa predeterminado en los ajustes de su escritorio.

## macOS

**La barra de menús.** Rookparcel pone su icono en la barra de menús, donde Windows tiene la bandeja. Al apuntar a él se muestran las velocidades. Un clic oculta la ventana o la recupera; un clic derecho o un clic con Control abre el menú: **Ocultar Rookparcel** / **Mostrar Rookparcel**, **Bloquear Rookparcel** (cuando hay un PIN de bloqueo establecido), **Añadir torrent...**, **Añadir torrent desde URL...**, **Iniciar todos los torrents**, **Detener todos los torrents** y **Salir de Rookparcel**. El menú del icono del Dock tiene los mismos elementos, sin **Salir de Rookparcel**, que el Dock añade por sí mismo. Mientras Rookparcel está [bloqueado](Bloquear-Rookparcel), los menús ofrecen solo **Mostrar Rookparcel** (y **Salir de Rookparcel** en el del icono de la barra de menús).

**Cerrar la ventana.** Con **Cerrar a la bandeja** activado (el valor predeterminado), el botón de cierre de la ventana (<kbd>⌘</kbd>+<kbd>W</kbd>) oculta la ventana y Rookparcel sigue ejecutándose. Haga clic en el icono del Dock o en el de la barra de menús para recuperarla. Para salir, elija **Rookparcel > Salir de Rookparcel** (<kbd>⌘</kbd>+<kbd>Q</kbd>) o **Salir de Rookparcel** en el menú del icono de la barra de menús. **Minimizar a la bandeja** oculta una ventana minimizada del mismo modo.

**Las notificaciones** de descargas completadas son notificaciones de macOS, con el mismo texto "Descarga completada". macOS puede preguntar una vez si Rookparcel puede mostrar notificaciones; si no aparece ninguna, permita Rookparcel en **Ajustes del Sistema > Notificaciones**.

**Inicio al iniciar sesión.** **Iniciar Rookparcel al arrancar el sistema** añade un ítem de inicio de sesión para este perfil (un agente de lanzamiento en `~/Library/LaunchAgents`); **Iniciar minimizado** lo inicia oculto. En macOS 13 y posteriores, **Ajustes del Sistema > General > Ítems de inicio** lo enumera. Un ítem de inicio de sesión que creó la versión GTK anterior de la aplicación sigue funcionando.

**Asociaciones.** Las casillas funcionan solo cuando Rookparcel se ejecuta como `Rookparcel.app`, y solo en el perfil predeterminado. Para desactivar una se usa **Obtener información > Abrir con** del Finder.

Las Preferencias en macOS no tienen botón **Aceptar**: cada cambio se aplica al instante.

*La aplicación nativa de macOS es novedad de la 1.2.19.*

## Interfaz web y rookparcel-nox

La interfaz web no tiene bandeja, notificaciones ni ajustes de inicio: sus Preferencias muestran las casillas de asociación y de inicio automático en gris, con la nota de que pertenecen al cliente de escritorio. Para iniciar `rookparcel-nox` con el equipo, ejecútelo como servicio; vea [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).
