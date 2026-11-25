*Esta página es una traducción de la [página principal en inglés](Home). Si hay diferencias, la página en inglés es la válida.*

# Wiki de Rookparcel

Rookparcel es un cliente BitTorrent ligero con el espíritu de µTorrent 2.2.1: pequeño, rápido y nativo. Este wiki explica cómo usarlo: instalarlo, añadir y gestionar torrents, los ajustes, la interfaz web y el demonio sin ventana.

Rookparcel se ofrece como cuatro programas:

| Programa | Dónde | Qué es |
|---|---|---|
| `rookparcel.exe` | Windows 10 y 11 | la ventana de Windows (GUI) |
| `rookparcel-gtk` | Linux, BSD | la ventana GTK 4 (GUI): la misma ventana principal y los mismos diálogos, con algunas diferencias indicadas en cada página |
| `Rookparcel.app` | macOS 11 o posterior, Apple silicon e Intel | la ventana nativa de macOS (GUI): la misma ventana principal y los mismos diálogos, con la barra de menús del Mac y los atajos con <kbd>⌘</kbd> |
| `rookparcel-nox` | todos | se ejecuta sin ventana y se controla desde un navegador mediante su interfaz web |

La mayoría de las páginas describen la ventana de Windows y después indican qué cambia en la ventana GTK y en la interfaz web. La aplicación de macOS funciona como la ventana GTK; donde se diferencia, la página tiene una sección sobre macOS: [Instalar Rookparcel](Instalar-Rookparcel#macos), [Atajos de teclado](Atajos-de-teclado#macos), [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio#macos), [Referencia de preferencias](Referencia-de-preferencias#diferencias-en-macos) y [Apagado automático](Apagado-automático). *La aplicación nativa de macOS es nueva en la 1.2.19; las versiones anteriores tienen la ventana GTK para Apple silicon en macOS.*

**Versión:** el wiki sigue el código actual (1.2.80), que también es la versión publicada más reciente: [1.2.80](https://github.com/Balls0fSteel/Rookparcel/releases/latest). Las páginas indican en qué versión llegó una función más reciente.

**Otros idiomas:** [English](Home) · [Français](Accueil) · [Deutsch](Startseite) · [Magyar](Kezdőlap). Las traducciones siguen este wiki en inglés; donde difieren, la página en inglés es la válida.

## Primeros pasos

- [Instalar Rookparcel](Instalar-Rookparcel)
- [Primer inicio y la Guía de configuración](Primer-inicio)
- [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos) (y el modo portátil)
- [Perfiles](Perfiles): varios Rookparcel independientes a la vez
- [Actualizar Rookparcel](Actualizar-Rookparcel)

## Uso diario

- [Añadir torrents](Añadir-torrents): archivos, enlaces magnet, URL, la ventana Añadir nuevo torrent
- [La ventana principal](La-ventana-principal)
- [Gestionar torrents](Gestionar-torrents): iniciar, detener, quitar, etiquetas y marcas, orden de la cola
- [Detalles y archivos del torrent](Detalles-y-archivos-del-torrent): las pestañas de detalles, elegir archivos, Propiedades del torrent
- [Crear torrents](Crear-torrents)
- [Importar desde µTorrent y qBittorrent](Importar-desde-uTorrent-y-qBittorrent)
- [Buscar torrents](Buscar-torrents) y la guía paso a paso [Buscar torrents en Windows](Buscar-torrents-en-Windows)
- [Descargador RSS](Descargador-RSS)

## Velocidad, red y disco

- [Límites de velocidad y programador](Límites-de-velocidad-y-programador)
- [Mantener el ping bajo](Mantener-el-ping-bajo) (modo para jugadores)
- [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos), VPN y proxies
- [Cola y siembra](Cola-y-siembra)
- [Disco y almacenamiento](Disco-y-almacenamiento)
- [Apagado automático](Apagado-automático)

## Remoto y sin ventana

- [Interfaz web](Interfaz-web)
- [Control remoto](Control-remoto): una ventana de escritorio que controla un Rookparcel en otro equipo
- [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox): el demonio sin ventana y los servicios
- [Docker](Rookparcel-en-Docker): rookparcel-nox como imagen de contenedor
- [Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr)

## Referencia

- [Referencia de preferencias](Referencia-de-preferencias)
- [Modo oscuro](Modo-oscuro)
- [Atajos de teclado](Atajos-de-teclado)
- [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio)
- [Bloquear Rookparcel](Bloquear-Rookparcel): ocultar las ventanas tras un PIN y quitar uno olvidado
- [Solución de problemas y preguntas frecuentes](Solución-de-problemas-y-preguntas-frecuentes)

## Más

- [Sitio web](https://balls0fsteel.github.io/Rookparcel/): descargas, noticias, preguntas frecuentes, pruebas de rendimiento
- [Versiones publicadas](https://github.com/Balls0fSteel/Rookparcel/releases)
- [Informes de errores y preguntas](https://github.com/Balls0fSteel/Rookparcel/issues)
- Documentación para desarrolladores: la carpeta [docs](https://github.com/Balls0fSteel/Rookparcel/tree/main/docs) del repositorio
