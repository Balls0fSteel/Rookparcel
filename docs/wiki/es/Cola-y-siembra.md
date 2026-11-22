*Esta página es una traducción de la página inglesa [Queueing and seeding](Queueing-and-seeding). Si hay diferencias, prevalece la página en inglés.*

# Cola y siembra

Esta página explica cómo decide Rookparcel qué torrents se ejecutan a la vez (la cola), qué ocurre cuando un torrent terminado ha sembrado lo suficiente (el objetivo de siembra), la siembra inicial de los torrents nuevos y cómo mover las descargas terminadas a otra carpeta.

Los ajustes están en **Opciones > Preferencias > Cola**.

![Preferencias, página Cola](images/prefs-queueing.png)

## Cómo funciona la cola

Un torrent que usted inicia pasa a la cola. La cola ejecuta tantos torrents como permiten sus límites y deja esperar al resto:

| Estado | Significado |
|---|---|
| **Descargando**, **Sembrando** | En ejecución, en una plaza de la cola. |
| **En cola** | Esperando una plaza de descarga. |
| **Semilla en cola** | Terminado, esperando una plaza de siembra. |
| **[F] Descargando**, **[F] Sembrando** | Iniciado con **Forzar inicio**, fuera de la cola. |
| **En cola para comprobar**, **Comprobando** | Esperando el turno de su unidad para ser comprobado y, después, comprobándose. Vea [Disco y almacenamiento](Disco-y-almacenamiento#comprobación). |
| **Detenido**, **En pausa**, **Terminado** | Sacado de la cola por usted (o, en el caso de Terminado, por el objetivo de siembra). |

- **Iniciar** pasa a la cola los torrents seleccionados.
- **Forzar inicio** los ejecuta ya, fuera de la cola, aunque se hayan alcanzado sus límites. Los torrents forzados nunca alcanzan un objetivo de siembra: siembran hasta que usted los detiene.
- **Detener** y **Pausa** sacan los torrents de la cola hasta que usted los inicia de nuevo.
- **Forzar comprobación** también se ejecuta fuera de la cola: *nuevo en 1.2.76.* Un torrent que usted vuelve a comprobar se comprueba de inmediato, aunque la cola esté llena, y después vuelve a esperar su turno. En un torrent detenido o en pausa, además lo inicia, y usted puede detenerlo de nuevo.
- **Subir en la cola**, **Bajar en la cola**, **Mover al principio de la cola** y **Mover al final de la cola**, en el menú del botón derecho, cambian el orden. La columna **#** muestra el lugar de cada torrent. Los que están más arriba obtienen plaza antes.

Vea [Gestionar torrents](Gestionar-torrents) para estas órdenes y sus teclas. Un torrent nuevo puede añadirse al principio de la cola con **Añadir al principio de la cola** en la ventana Añadir nuevo torrent ([Añadir torrents](Añadir-torrents)).

## Ajustes de la cola

En **Ajustes de la cola**:

| Ajuste | Significado | Valor predeterminado |
|---|---|---|
| **Número máximo de torrents activos (subida o bajada): [-1: sin límite]** | La mayor cantidad de torrents en cola que se ejecutan a la vez, descargas y siembras juntas. | 500 |
| **Número máximo de descargas activas: [-1: sin límite]** | La mayor cantidad de torrents en cola que descargan a la vez. El siguiente se inicia cuando uno termina o se detiene. | 3 |
| **Número máximo de semillas activas: [-1: sin límite]** | La mayor cantidad de torrents terminados en cola que siembran a la vez. | 5 |
| **No contar los torrents lentos en estos límites** | Un torrent que mueve menos de unos 2 kB/s durante un minuto deja de ocupar una plaza de descarga o de siembra, de modo que otro puede iniciarse. Sigue contando para el total. | Activado |

Para que todo se ejecute a la vez, ponga los límites en -1. Para descargar un torrent cada vez, ponga **Número máximo de descargas activas** en 1.

Lo que es «lento» se define en la página Avanzado: `inactive_down_rate` e `inactive_up_rate` (bytes por segundo, 2048 de forma predeterminada) y `auto_manage_startup` (segundos que un torrent debe mantenerse por debajo de ellos, 60 de forma predeterminada).

## Objetivo de siembra

El objetivo de siembra decide cuándo ha sembrado lo suficiente un torrent terminado. En **Objetivo de siembra**:

| Ajuste | Significado | Valor predeterminado |
|---|---|---|
| **Proporción mínima (%): [0: sin objetivo de proporción]** | Suba este porcentaje de lo que el torrent ha descargado. 150 significa una vez y media. | 150 |
| **Tiempo mínimo de siembra (minutos): [0: sin objetivo de tiempo]** | Siembre al menos estos minutos. | 0 |

Si se definen ambos, deben alcanzarse los dos. Si ambos están en 0, no hay objetivo.

Estos dos números no hacen nada por sí solos. Al principio, los torrents siembran hasta que usted los detiene, sea cual sea su proporción.

**Cuando un torrent alcanza el objetivo de siembra (los forzados nunca):**

- **Limitar la velocidad de subida a (kB/s): [0: detener]** pone el objetivo en vigor. Al principio está desactivado.
- Marcada con 0, un torrent que alcanzó el objetivo se detiene. Sale de la cola y aparece como **Terminado**.
- Marcada con una velocidad, el torrent sigue sembrando despacio: su propio límite de subida se fija en esa velocidad. Un límite más bajo que el torrent ya tenga se mantiene.

La pestaña **Registro** anota el nombre del torrent y lo que se alcanzó, por ejemplo «objetivo de siembra alcanzado, detenido (proporción 1.523, objetivo 1.500)» o «objetivo de siembra alcanzado, subida limitada a ...».

Solo se comprueban los torrents en cola: un torrent iniciado con Forzar inicio, detenido, en comprobación o que aún descarga se deja en paz.

Un objetivo actúa sobre un torrent una sola vez. Si usted vuelve a iniciar un torrent después de que su objetivo lo detuviera, sigue sembrando hasta que usted lo detenga. Solo se detiene de nuevo después de que usted cambie el objetivo y el torrent alcance el nuevo.

Hasta la versión 1.2.44 el objetivo estaba siempre en vigor y detenía todos los torrents al 150 %. Una actualización deja desactivado un objetivo que usted nunca cambió. Un objetivo que sí había cambiado sigue deteniendo torrents: la casilla se marca por usted, con 0.

### Un objetivo para un solo torrent

1. Seleccione el torrent y elija **Propiedades** (<kbd>Enter</kbd>).
2. En **Objetivo de siembra**, marque **Reemplazar los ajustes predeterminados**.
3. Defina **Proporción mínima (%): [0: sin objetivo de proporción]** y **Tiempo mínimo de siembra (minutos): [0: sin objetivo de tiempo]**. Con ambos en 0, este torrent siembra para siempre.
4. Haga clic en **Aceptar**.

El objetivo propio de un torrent está siempre en vigor, también mientras la casilla de Preferencias está desactivada: el torrent se detiene cuando lo alcanza (o se limita su subida, si la casilla está marcada con una velocidad).

![El cuadro de diálogo Propiedades del torrent](images/properties.png)

## Siembra inicial

La siembra inicial (super-siembra) le ayuda a repartir un torrent que acaba de crear, cuando usted es su única semilla: cada par recibe piezas que nadie más tiene aún, de modo que el enjambre obtiene una copia completa antes.

- Haga clic con el botón derecho en un torrent que está sembrando y elija **Avanzado > Siembra inicial**. El elemento está en gris salvo que todos los torrents seleccionados estén sembrando; elíjalo de nuevo para desactivarla.
- O marque **Siembra inicial** en **Otros ajustes**, en las Propiedades del torrent.

Mientras está activada, el estado muestra **[S] Sembrando**. Vea [Crear torrents](Crear-torrents) para hacer un torrent que sembrar. En rookparcel-nox, `--super-seed` hace lo mismo con los torrents completos.

## Mover las descargas completadas

Rookparcel puede mover los archivos de un torrent a otra carpeta cuando termina de descargarse, por ejemplo de un disco rápido a uno grande. Sigue sembrando desde el nuevo lugar. Se define en **Opciones > Preferencias > Carpetas**, en **Ubicación de los archivos descargados**:

1. Marque **Mover las descargas completadas a:** y elija la carpeta que hay debajo.
2. Si lo desea, marque **Añadir la etiqueta del torrent**: un torrent con etiqueta va a una carpeta con el nombre de la etiqueta, dentro de esa carpeta. Cuenta la etiqueta que tiene cuando termina.
3. Deje marcada **Mover solo desde la carpeta de descarga predeterminada** para mover solo los torrents guardados en la propia carpeta de las descargas nuevas (**Poner las nuevas descargas en:**); los torrents que guardó en otro sitio, incluso en una carpeta dentro de ella, se quedan donde están. Desmárquela para mover todos los torrents terminados.
4. Haga clic en **Aceptar**.

Se aplica a los torrents que terminen a partir de ahora. Más sobre las carpetas y el movimiento de archivos: [Disco y almacenamiento](Disco-y-almacenamiento#mover-las-descargas-completadas).

## Apagado automático y la siembra

**Opciones > Apagado automático > ... Al terminar todo** espera a que se detenga cada semilla. Una semilla sin objetivo de siembra, o una forzada, sigue sembrando y lo retrasa. Vea [Apagado automático](Apagado-automático).

## En la ventana GTK y en la interfaz web

- **Ventana GTK**: la misma página Cola, las mismas Propiedades y los mismos menús.
- **Aplicación de macOS**: la misma página Cola, las mismas Propiedades y los mismos menús; las Preferencias se aplican de inmediato, sin **Aceptar**.
- **Interfaz web**: la misma página Cola. Su casilla del objetivo de siembra dice **Detener el torrent o limitar su velocidad de subida**, con la velocidad al lado; 0 detiene el torrent. El menú del torrent tiene los mismos elementos de cola y de Siembra inicial.
- **rookparcel-nox**: los límites de la cola y el objetivo de siembra se definen en las Preferencias de su interfaz web. `--move-on-complete DIR` mueve a una carpeta cada torrent que termina.

## Páginas relacionadas

- [Gestionar torrents](Gestionar-torrents)
- [Límites de velocidad y programador](Límites-de-velocidad-y-programador)
- [Disco y almacenamiento](Disco-y-almacenamiento)
- [Referencia de preferencias](Referencia-de-preferencias)
