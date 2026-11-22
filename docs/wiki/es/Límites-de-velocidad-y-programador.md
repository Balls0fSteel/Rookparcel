*Esta es una traducción de la página inglesa [Speed limits and the scheduler](https://github.com/Balls0fSteel/Rookparcel/wiki/Speed-limits-and-scheduler). Si hay diferencias, prevalece la página inglesa.*

# Límites de velocidad y programador

Esta página explica cómo limitar las velocidades de bajada y de subida de Rookparcel: para todos los torrents juntos, para torrents individuales y por horas con el programador semanal. Para límites que siguen por sí solos el ping de su línea, consulte [Mantener el ping bajo](Mantener-el-ping-bajo).

Todas las velocidades están en kB/s, donde 1 kB son 1024 bytes. En cada cuadro de velocidad, 0 significa ilimitada.

## Límites globales

Los límites globales se aplican a todos los torrents juntos.

1. Abra **Opciones > Preferencias** (<kbd>Ctrl</kbd>+<kbd>P</kbd>) y elija **Ancho de banda**.
2. En **Límite global de velocidad de subida**, defina **Velocidad máxima de subida (kB/s): [0: ilimitada]**.
3. En **Límite global de velocidad de bajada**, defina **Velocidad máxima de bajada (kB/s): [0: ilimitada]**.
4. Haga clic en **Aceptar** o **Aplicar**. Los nuevos límites se aplican de inmediato.

![Preferencias, página Ancho de banda](images/prefs-bandwidth.png)

Ambos son ilimitados de forma predeterminada. Un límite de subida es el motivo habitual para definir uno: cuando Rookparcel sube tan rápido como permite la línea, las páginas web, los juegos y las llamadas van más lentos para todos en casa.

### Opciones del límite global de velocidad

La misma página tiene tres casillas que cambian lo que cuentan los límites:

| Opción | Qué hace | Valor predeterminado |
|---|---|---|
| **Aplicar el límite de velocidad a la sobrecarga del transporte** | Cuenta los encabezados TCP/IP del tráfico de pares, estimados por paquete, dentro de los límites globales, de modo que los límites coincidan con lo que transporta la línea. | Desactivada |
| **Aplicar el límite de velocidad a las conexiones uTP** | Aplica los límites globales también a los pares uTP. Si está desactivada, los pares uTP solo se ralentizan por el control de retardo propio de uTP, que cede ante otro tráfico. | Activada |
| **Mantener el ping bajo (limitar las velocidades automáticamente)** | Limita automáticamente las subidas y las bajadas mientras crece el retardo de su línea. Consulte [Mantener el ping bajo](Mantener-el-ping-bajo). | Desactivada |

### Conexiones y ranuras de subida

El grupo **Número de conexiones** de la misma página no es un límite de velocidad, pero afecta a la velocidad:

| Ajuste | Significado | Valor predeterminado |
|---|---|---|
| **Número máximo global de conexiones:** | El máximo de conexiones con pares abiertas a la vez entre todos los torrents. Redúzcalo si el router tiene dificultades. | 200 |
| **Número máximo de pares conectados por torrent: [0: sin límite]** | El máximo de pares con los que se conecta un torrent. | 0 |
| **Número de ranuras de subida por torrent: [0: sin límite]** | A cuántos pares sube un torrent a la vez. | 0 |
| **Número global de ranuras de subida: [-1: sin límite]** | A cuántos pares suben todos los torrents juntos a la vez, eligiendo los más rápidos. Menos ranuras dan más velocidad a cada par. | 8 |

Los dos valores por torrent se aplican a los torrents que se añadan a partir de ahora; las Propiedades de un torrent pueden cambiar sus propias ranuras de subida. La pregunta sobre el router de la Guía de configuración puede reducir los números de conexiones para un router antiguo: consulte [Primer inicio](Primer-inicio).

## Límites para torrents individuales

Un torrent puede tener límites propios, por debajo de los globales. Los límites globales se siguen aplicando además.

**Desde el menú contextual:**

1. Seleccione uno o más torrents de la lista.
2. Haga clic con el botón derecho y elija **Asignación de ancho de banda > Establecer límite de bajada** o **Asignación de ancho de banda > Establecer límite de subida**.
3. Elija una velocidad: **Ilimitado**, o una de 10, 25, 50, 100, 250, 500, 1000, 2500 y 5000 kB/s.

La velocidad actual del primer torrent seleccionado aparece marcada.

![El menú contextual de la lista de torrents](images/torrent-menu.png)

**Desde Propiedades del torrent**, para cualquier velocidad:

1. Seleccione los torrents y elija **Propiedades** (<kbd>Enter</kbd>).
2. En **Ajustes de ancho de banda**, defina **Velocidad máxima de subida (kB/s): [0: ilimitada]** y **Velocidad máxima de bajada (kB/s): [0: ilimitada]**.
3. Para dar al torrent su propio número de ranuras de subida, marque **Reemplazar el número predeterminado de ranuras de subida** y rellene **Número de ranuras de subida: [0: ilimitado]**.
4. Haga clic en **Aceptar**.

![El cuadro de diálogo Propiedades del torrent](images/properties.png)

Los límites propios de un torrent aparecen en las columnas **Límite bajada** y **Límite subida** de la lista de torrents y en la pestaña **General**. Consulte [Detalles y archivos del torrent](Detalles-y-archivos-del-torrent) para el resto del cuadro de diálogo Propiedades.

Un torrent que alcanza su objetivo de siembra también puede recibir por sí solo un límite de subida: consulte [Cola y siembra](Cola-y-siembra#objetivo-de-siembra).

## La barra de estado

La barra de estado, en la parte inferior de la ventana principal, muestra las velocidades actuales y los totales de esta sesión, como **B: 1.2 MB/s T: 3.4 GB** para la bajada y **S: ... T: ...** para la subida. Mientras Mantener el ping bajo o la ralentización por disco ocupado frenan una dirección, su parte muestra también el límite automático, como **S: 412.0 kB/s (auto 450.0 kB/s) T: ...**.

La barra de estado no tiene menú de velocidad. Defina los límites globales en Preferencias, o los límites propios de los torrents desde su menú.

Con **Mostrar la velocidad actual en la barra de título** activado (**Opciones > Preferencias > Interfaz**), el título de la ventana y el botón de la barra de tareas también empiezan con las velocidades.

## El programador

El programador sigue una tabla semanal, hora por hora, y cambia Rookparcel entre cuatro modos. Está desactivado de forma predeterminada.

1. Abra **Opciones > Preferencias > Programador**.
2. Marque **Activar el programador**. La tabla está en gris hasta que lo haga.
3. Haga clic en una celda de la **Tabla del programador** para pasar esa hora al siguiente modo. Mantenga pulsado el botón y arrastre para pintar con el mismo modo las celdas por las que pase.
4. En **Ajustes del programador**, defina las velocidades de las horas **Limitado**.
5. Haga clic en **Aceptar** o **Aplicar**.

![Preferencias, página Programador](images/prefs-scheduler.png)

La tabla tiene una fila por día, empezando por el lunes, y una columna por hora, en la hora local de su equipo. Un clic recorre cíclicamente **Velocidad completa**, **Limitado**, **Apagado** y **Solo sembrar**. La línea bajo la tabla nombra la hora que hay bajo el puntero y su modo, como «Lun 09:00 - 10:00: Velocidad completa».

| Modo | Color | Qué hace Rookparcel en esa hora |
|---|---|---|
| **Velocidad completa** | verde oscuro | Funciona con sus ajustes normales. |
| **Limitado** | verde claro | Usa las dos velocidades Limitado de abajo en lugar de los límites globales. |
| **Apagado** | blanco | Pone en pausa todos los torrents. Con **Desactivar DHT al apagar**, también se desactiva DHT, de modo que Rookparcel permanece en silencio en la red. |
| **Solo sembrar** | rojo claro | Sube, pero no descarga nada. Los torrents añadidos durante estas horas también esperan. |

| Ajuste del programador | Significado | Valor predeterminado |
|---|---|---|
| **Velocidad de subida limitada (kB/s): [0: ilimitada]** | El límite global de subida durante las horas Limitado. | 0 |
| **Velocidad de bajada limitada (kB/s): [0: ilimitada]** | El límite global de bajada durante las horas Limitado. | 0 |
| **Desactivar DHT al apagar** | También desactiva DHT durante las horas Apagado. | Activado |

Qué esperar:

- Cuando la hora cambia de modo, la parte de red de la barra de estado añade **Programador: limitado**, **Programador: apagado** o **Programador: solo sembrar**, y la pestaña **Registro** escribe una línea como «Programador: Limitado». Velocidad completa no muestra nada.
- Apagado no cambia el estado propio de los torrents. Cuando vuelve Velocidad completa, continúan como antes, y un torrent que usted detuvo sigue detenido.
- Desmarque **Activar el programador** para volver de inmediato a la velocidad completa con sus ajustes normales.
- Mantener el ping bajo y la ralentización por disco ocupado todavía pueden reducir las velocidades durante cualquier hora; nunca las aumentan.

## Límites de velocidad alternativos (interfaz web de rookparcel-nox)

Las ventanas de Windows y GTK no tienen límites de velocidad alternativos; use en su lugar las horas Limitado del programador.

La interfaz web de [rookparcel-nox](Ejecutar-rookparcel-nox) sí los tiene, en **Preferencias > Ancho de banda > Límites alternativos**:

| Ajuste | Significado | Valor predeterminado |
|---|---|---|
| **Velocidad de subida alternativa (kB/s): [0: ilimitada]** | El límite global de subida mientras los límites alternativos están activados. | 10 |
| **Velocidad de bajada alternativa (kB/s): [0: ilimitada]** | El límite global de bajada mientras están activados. | 10 |
| **Usar ahora los límites alternativos** | Cambia a los límites alternativos hasta que vuelva a cambiar. | Desactivado |

Las aplicaciones que cambian el modo de límites de velocidad de qBittorrent a través de la interfaz web (consulte [Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr)) cambian lo mismo. Las horas Limitado del programador siguen imponiéndose a ambos conjuntos mientras duran.

Cuando una ventana de escritorio sirve la interfaz web (Preferencias > Interfaz web en la ventana), este grupo no se muestra y los límites alternativos no se pueden activar: se aplican los límites propios de la ventana y el programador.

## En la ventana GTK y en la interfaz web

- **Ventana GTK** (Linux, BSD): las mismas páginas de Preferencias, el mismo submenú **Asignación de ancho de banda** en el menú del torrent, la misma tabla del programador, que se edita del mismo modo.
- **Aplicación de macOS**: las mismas páginas, el mismo submenú **Asignación de ancho de banda** y una tabla del programador que se pinta arrastrando. Las Preferencias se aplican de inmediato, sin **Aceptar**: consulte [Referencia de preferencias](Referencia-de-preferencias#diferencias-en-macos).
- **Interfaz web**: las mismas páginas bajo sus **Preferencias**, y el mismo submenú **Asignación de ancho de banda**. Su barra de estado muestra un límite global entre corchetes tras la velocidad mientras hay uno definido, y el límite automático del mismo modo que la ventana.
- **rookparcel-nox** sin la interfaz web: `--dl-limit KIB` y `--ul-limit KIB` definen los límites globales al iniciar (0 = ilimitado). El programador, el objetivo de siembra y los límites alternativos se definen en las **Preferencias** de la interfaz web.

## Páginas relacionadas

- [Mantener el ping bajo](Mantener-el-ping-bajo)
- [Cola y siembra](Cola-y-siembra)
- [Disco y almacenamiento](Disco-y-almacenamiento#cuando-el-disco-está-ocupado)
- [Referencia de preferencias](Referencia-de-preferencias)
