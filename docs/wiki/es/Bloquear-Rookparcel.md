*Esta página es una traducción de [Locking Rookparcel](Locking-Rookparcel). La página en inglés es la versión de referencia.*

# Bloquear Rookparcel

Esta página explica cómo bloquear las ventanas de Rookparcel con un PIN, para que otra persona que esté en su equipo no pueda ver su lista de torrents, y cómo quitar un PIN que ha olvidado. *Novedad de la 1.2.54.*

El bloqueo es para la privacidad, no para la seguridad. Oculta las ventanas; no cifra nada. Cualquiera que pueda editar el archivo de ajustes de Rookparcel, o abrir su carpeta de descargas, puede saltárselo. No use para esto el PIN de su tarjeta bancaria.

## Establecer un PIN y bloquear

1. Abra **Opciones > Preferencias** y elija **Avanzado > Bloqueo de la interfaz** (en un Mac: **Rookparcel > Preferencias**, y después **Bloqueo de la interfaz**).
2. Haga clic en **Establecer PIN...**.
3. Escriba un PIN de 4 a 6 dígitos en **PIN nuevo (de 4 a 6 dígitos)**, y otra vez en **Repita el PIN nuevo**.
4. Haga clic en **Aceptar**. El PIN se guarda al instante; no hace falta hacer clic en **Aceptar** en Preferencias.

*Antes de la 1.2.63, el PIN se establecía en **Opciones > Lock PIN**.*

Para bloquear, elija **Archivo > Bloquear Rookparcel** o pulse <kbd>Ctrl</kbd>+<kbd>L</kbd>. Cuando hay un PIN establecido, el menú de la bandeja también tiene **Bloquear Rookparcel**. Si aún no hay ningún PIN establecido, **Bloquear Rookparcel** pide uno primero y después bloquea.

Al bloquear se ocultan en la bandeja todas las ventanas de Rookparcel: la ventana principal y cualquier diálogo que estuviera abierto. Sus torrents siguen descargando y sembrando.

Para cambiar el PIN, haga clic en **Cambiar PIN...** en la misma página, escriba el PIN actual y después el nuevo dos veces. **Quitar PIN...** pide el PIN actual y lo quita; **Bloquear Rookparcel** pedirá entonces uno nuevo antes de bloquear.

## Desactivar el bloqueo

Si nunca quiere el bloqueo, desmarque **Permitir bloquear Rookparcel con un PIN** en la página **Bloqueo de la interfaz**. Si hay un PIN establecido, Rookparcel lo pide primero y después lo quita. Mientras el bloqueo está desactivado, **Archivo > Bloquear Rookparcel**, su atajo y el **Bloquear Rookparcel** del menú de la bandeja desaparecen. Vuelva a marcar la casilla para recuperarlos.

## Abrir un Rookparcel bloqueado

Haga clic en el icono de la bandeja, elija **Mostrar Rookparcel** en el menú de la bandeja o vuelva a iniciar Rookparcel. Una ventana titulada **Rookparcel está bloqueado** pide el PIN. Con el PIN correcto, las ventanas que estaban abiertas vuelven tal como estaban. **Cancelar** deja Rookparcel bloqueado en la bandeja.

Con el quinto PIN erróneo seguido, Rookparcel espera 30 segundos antes de aceptar otro, y a partir de ahí cada PIN erróneo hace lo mismo. El PIN correcto pone fin a esto.

Rookparcel sigue bloqueado cuando sale. La próxima vez que lo inicie, pide el PIN de inmediato, sin mostrar su ventana principal. Si se inicia al iniciar sesión con **Iniciar minimizado**, espera en la bandeja sin preguntar.

## Mientras Rookparcel está bloqueado

- El menú de la bandeja ofrece solo **Mostrar Rookparcel** y **Salir**. **Salir** funciona sin el PIN.
- La notificación "Descarga completada" no aparece, porque nombra el torrent.
- Los mensajes que se abrirían por sí solos esperan hasta que desbloquee: los mensajes de error, una oferta de actualización y la ventana **Añadir nuevo torrent** de un enlace magnet cuyos archivos han llegado.
- Un archivo `.torrent` o un enlace magnet que abra mientras está bloqueado pide primero el PIN. Si hace clic en **Cancelar**, no se añade.
- La cuenta atrás de **Apagado automático** sigue apareciendo, para que pueda detenerla.
- La [interfaz web](Interfaz-web) no está bloqueada. Tiene su propio nombre de usuario y contraseña. Si **Omitir la autenticación para clientes en localhost** está activado, un navegador del mismo equipo aún puede ver sus torrents.

## Si olvidó el PIN

Puede quitar un PIN olvidado cuando Rookparcel se inicia. Rookparcel pregunta **¿Quitar el PIN de bloqueo? Rookparcel se abrirá sin pedirlo.** Responda **Sí** (**Aceptar** en Linux, **Quitar** en macOS) y se abrirá sin el PIN. Sus torrents y sus demás ajustes se quedan como están.

**Windows:** mantenga pulsada la tecla <kbd>Ctrl</kbd> mientras inicia Rookparcel, por ejemplo mientras hace clic en su elemento del menú Inicio o hace doble clic en su icono del escritorio. Si Rookparcel ya se está ejecutando, pregunta la copia en ejecución.

**macOS:** cierre Rookparcel (**Salir de Rookparcel** en el menú del icono de la barra de menús) y mantenga pulsada <kbd>⌥</kbd> (Opción) mientras lo abre. Mientras Rookparcel se ejecuta, mantener pulsada <kbd>⌥</kbd> al hacer clic en su icono del Dock también pregunta.

**Linux y los BSD:** inícielo desde un terminal con `--reset-lock`. Si Rookparcel ya se está ejecutando, pregunta la copia en ejecución.

| Copia | Comando |
|---|---|
| Binarios simples (tar.gz) | `rookparcel-gtk --reset-lock` |
| AppImage | `./Rookparcel-<version>-x86_64.AppImage --reset-lock` |
| Flatpak | `flatpak run io.github.balls0fsteel.Rookparcel --reset-lock` |

`--reset-lock` funciona también en Windows y macOS: `rookparcel.exe --reset-lock`, o en un Mac `/Applications/Rookparcel.app/Contents/MacOS/Rookparcel --reset-lock`. Si Rookparcel ya se ejecuta en el Mac, su pregunta puede abrirse detrás de la ventana que está al frente: búsquela.

Con un [perfil](Perfiles), añada su opción: `rookparcel.exe --profile NAME --reset-lock`.

**A mano:** cierre Rookparcel, abra su `rookparcel.ini` en un editor de texto, borre las líneas que empiezan por `ui.lock_pin=` y `ui.locked=`, y guarde. [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos) dice dónde está el archivo.

## Perfiles y ventanas remotas

Cada [perfil](Perfiles) tiene su propio PIN, o ninguno. Lo mismo ocurre con cada ventana conectada a un Rookparcel remoto ([Control remoto](Control-remoto)), ya que conserva sus propios ajustes.

## Versión GTK (Linux, BSD)

El bloqueo funciona como en Windows. <kbd>Ctrl</kbd>+<kbd>L</kbd> bloquea desde cualquier ventana de Rookparcel, no solo desde la principal: Preferencias, Propiedades, Crear nuevo torrent, Añadir nuevo torrent, el Descargador RSS, las ventanas de búsqueda y los demás diálogos (no las ventanas del PIN). *Novedad de la 1.2.76; antes, no hacía nada mientras había abierto uno de esos diálogos.* Si la bandeja desaparece mientras Rookparcel está bloqueado (por ejemplo, porque se reinicia el panel), aparece la ventana **Rookparcel está bloqueado**, de modo que las ventanas no quedan ocultas sin forma de volver. *Novedad de la 1.2.76.*

En los escritorios sin bandeja (GNOME simple sin la extensión AppIndicator), no hay dónde ocultar las ventanas, así que la ventana **Rookparcel está bloqueado** ocupa su lugar. No tiene botón **Cancelar**, y al cerrarla se cierra Rookparcel, como ocurre allí al cerrar la ventana principal.

## macOS

**Archivo > Bloquear Rookparcel** es <kbd>⌘</kbd>+<kbd>L</kbd>. Mientras Rookparcel está bloqueado:

- la barra de menús solo tiene **Mostrar Rookparcel**, **Ocultar Rookparcel** y **Salir de Rookparcel** en el menú **Rookparcel**, y el menú **Edición**;
- el menú del icono de la barra de menús ofrece **Mostrar Rookparcel** y **Salir de Rookparcel**, y el menú del icono del Dock **Mostrar Rookparcel**;
- mientras la ventana del PIN está abierta, **Salir de Rookparcel** aparece en gris: haga clic primero en **Cancelar**.
