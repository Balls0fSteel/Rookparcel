*Esta página es una traducción de la página en inglés [First start and the Setup Guide](First-start-and-the-Setup-Guide). Si hay diferencias, la página en inglés es la válida.*

# Primer inicio y la Guía de configuración

Esta página explica qué ocurre la primera vez que inicia Rookparcel: la Guía de configuración, una ventana breve con cuatro preguntas, qué ajusta cada respuesta y cómo abrir la guía de nuevo más adelante.

*Nuevo en la 1.1.62.* Hasta la 1.1.40 no hay guía: Rookparcel se abre con sus ajustes predeterminados, y todo lo que sigue se ajusta en cambio en **Opciones > Preferencias**.

## Cuándo se abre la guía

La Guía de configuración se abre sola en el primer inicio de Rookparcel: cuando su archivo de ajustes todavía no tiene ningún ajuste. Eso ocurre en una instalación nueva, en una carpeta portátil nueva y en cada [perfil](Perfiles) nuevo. Se abre antes de la ventana principal, así que nada empieza a descargarse antes de que haya respondido. Un archivo de ajustes que existe pero no se puede leer no cuenta como primer inicio.

No se abre sola para quien ya usó una versión anterior: los ajustes ya existen. Ábrala desde el menú (más abajo).

![La primera página de la Guía de configuración](images/setup-guide.png)

## Moverse por la guía

Cada página tiene un título, «Paso 1 de 4» encima, un texto breve y sus ajustes. Los botones:

| Botón | Qué hace |
|---|---|
| **< Atrás** | la página anterior |
| **Siguiente >** | la página siguiente |
| **Finalizar** | guarda sus respuestas y cierra la guía |
| **Omitir** | cierra la guía y deja los ajustes como están. <kbd>Esc</kbd> y el botón de cierre de la ventana hacen lo mismo |

No se guarda nada hasta **Finalizar**, y **Finalizar** cambia solo las respuestas que usted cambió. Deje el ratón sobre un ajuste para ver qué hace; la ayuda también aparece debajo de los ajustes.

Después de **Finalizar** u **Omitir**, la guía no se vuelve a abrir sola.

## Página 1: Dónde van las descargas

Elija la carpeta donde se guardan las descargas nuevas.

| Ajuste | Significado |
|---|---|
| **Poner las nuevas descargas en:** | la carpeta de las descargas nuevas. La predeterminada es su carpeta Descargas; en una carpeta portátil es la carpeta `Downloads` que hay dentro de ella |
| **Mover las descargas completadas a:** | si está marcada, las descargas terminadas se mueven solas a la carpeta que hay junto a ella, de un disco rápido a uno grande, por ejemplo |

**Opciones > Preferencias > Carpetas** tiene más ajustes de carpetas. Consulte [Disco y almacenamiento](Disco-y-almacenamiento).

## Página 2: Su router

Cada conexión con un par ocupa un lugar en la tabla de conexiones de su router. Los routers antiguos y los equipos básicos de los proveedores de internet tienen tablas pequeñas; cuando una se llena, el router corta conexiones, ralentiza toda la casa o se reinicia.

**¿Qué router tiene?**

- **Moderno (comprado después de 2015 aproximadamente), o no estoy seguro:** los números normales de conexiones de Rookparcel.
- **Más antiguo, o el equipo básico del proveedor de internet:** menos conexiones: 100 en total, 40 por torrent y 8 nuevas por segundo.

La opción de un router más antiguo es un punto de partida. Puede cambiar estos números más adelante en **Opciones > Preferencias > Ancho de banda** y **Avanzado**. Si más adelante responde «moderno», se restablecen los números normales.

La misma página tiene **Mantener el ping bajo (limitar las velocidades automáticamente)**. Márquela si juega o hace llamadas mientras Rookparcel se ejecuta: mantiene las velocidades bajas mientras su línea está ocupada. Consulte [Mantener el ping bajo](Mantener-el-ping-bajo).

## Página 3: Sus discos

**Usar E/S de disco inteligente** mantiene el equipo ágil mientras Rookparcel lee y escribe:

- las comprobaciones se ejecutan con la prioridad de disco en segundo plano del sistema;
- cada unidad se ajusta a su tipo (SSD, disco duro, recurso de red);
- las transferencias se ralentizan mientras un disco responde despacio.

Las comprobaciones suaves y el ajuste por unidad están activos desde el principio. La casilla añade la ralentización mientras un disco está ocupado. Las comprobaciones tardan más mientras otra cosa usa el disco. **Opciones > Preferencias > Avanzado > E/S de disco** configura cada parte por separado. Consulte [Disco y almacenamiento](Disco-y-almacenamiento).

## Página 4: Este equipo

Rookparcel puede iniciarse cuando usted inicia sesión y abrir los archivos `.torrent` y los enlaces `magnet:` desde su navegador y su administrador de archivos.

| Ajuste | Significado |
|---|---|
| **Iniciar Rookparcel al arrancar el sistema** | inicia Rookparcel cuando usted inicia sesión |
| **Asociar con archivos .torrent** | al hacer doble clic en un archivo `.torrent`, se abre en Rookparcel |
| **Asociar con enlaces magnet:** | al hacer clic en un enlace `magnet:` en el navegador, se abre en Rookparcel |

**Opciones > Preferencias > General** cambia esto más adelante. Consulte [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio).

Esta página se omite donde no procede:

- **El instalador de Windows** ya preguntó por las asociaciones, así que una copia instalada muestra tres páginas en su primer inicio. La página aparece cuando abre la guía desde el menú.
- **Una ventana conectada a un `rookparcel-nox` remoto** muestra tres páginas cuando abre la guía desde el menú.
- **El Flatpak** muestra solo la casilla de inicio: el escritorio decide qué programa abre los archivos `.torrent` y los enlaces `magnet:`. Lo mismo ocurre en macOS donde Rookparcel no puede establecer las asociaciones por sí mismo (un perfil con nombre, o un binario suelto fuera de `Rookparcel.app`).

## Abrir la guía de nuevo

Elija **Opciones > Guía de configuración**, o pulse <kbd>Ctrl</kbd>+<kbd>G</kbd>. La guía muestra sus ajustes actuales, y **Finalizar** aplica las respuestas que cambió, igual que **Aceptar** en Preferencias.

## GTK y la interfaz web

- La versión GTK (Linux, los BSD) tiene la misma guía, con las mismas páginas, en el primer inicio de cada perfil y en **Opciones > Guía de configuración** (<kbd>Ctrl</kbd>+<kbd>G</kbd>).
- La aplicación de macOS también la tiene, en **Rookparcel > Guía de configuración** (<kbd>⌘</kbd>+<kbd>G</kbd>).
- `rookparcel-nox` y la interfaz web no tienen guía. Configúrelos en las Preferencias de la interfaz web: consulte [Interfaz web](Interfaz-web) y [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

## Después de la guía

Se abre la ventana principal. Próximos pasos:

- [Añadir torrents](Añadir-torrents)
- [La ventana principal](La-ventana-principal)
- [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos), si las descargas siguen lentas
- [Referencia de preferencias](Referencia-de-preferencias), para todos los demás ajustes
