*Esta página es una traducción de [Dark mode](Dark-mode). La página en inglés es la versión de referencia.*

# Modo oscuro

Rookparcel sigue el ajuste claro u oscuro de su sistema en todas las plataformas, y el programa de Windows y la interfaz web le permiten elegir.

## Windows

De forma predeterminada, la ventana sigue lo que se elija para las aplicaciones en **Configuración > Personalización > Colores > Elija su modo** de Windows. **Opciones > Preferencias > Interfaz > Tema** tiene tres opciones:

- **Sistema (claro u oscuro)**: sigue a Windows y cambia con él mientras el programa se ejecuta.
- **Claro** y **Oscuro**: fijan los colores diga lo que diga Windows.

Todo lo sigue: la ventana y sus menús, la barra de herramientas, la lista de torrents, las categorías, las pestañas de detalles, la barra de estado y todos los diálogos, incluido Preferencias. Los iconos vienen en un juego claro y otro oscuro. El **Contraste alto** de Windows siempre tiene prioridad: con él activado, Rookparcel conserva los colores del sistema.

El modo oscuro necesita **Windows 10 versión 1809** o posterior. Un Windows más antiguo, o uno sin los componentes que usa el modo oscuro, muestra una nota bajo el cuadro Tema y se queda en claro. Tres cosas siguen siendo del sistema: los cuadros de mensaje, el selector de archivos (que sigue el modo propio de Windows, no el ajuste Tema) y la barra de progreso de la descarga de la actualización mientras aún no tiene tamaño.

## Linux (GTK)

El programa GTK sigue su tema GTK: un tema oscuro como Adwaita-dark o Breeze-Dark, `gtk-application-prefer-dark-theme` o `GTK_THEME=Adwaita:dark`. La preferencia oscura de un escritorio por sí sola (el estilo *Oscuro* de GNOME o de KDE) no oscurece las ventanas GTK 4 simples, de modo que Rookparcel sigue en claro con ella hasta que el tema GTK también sea oscuro. Sus partes dibujadas por él mismo (las barras de progreso, el mapa de piezas, el gráfico de velocidad, el programador) y los iconos cambian con el tema.

## macOS

La aplicación de Mac sigue la apariencia del sistema.

## Interfaz web

De forma predeterminada, la página sigue el ajuste de su navegador o de su sistema y ofrece temas propios, incluidos sus propios archivos CSS: vea [Interfaz web](Interfaz-web).
