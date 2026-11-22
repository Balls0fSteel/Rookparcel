*Esta página es una traducción de la página en inglés [Profiles](Profiles). Si hay diferencias, la página en inglés es la válida.*

# Perfiles

Un perfil es un Rookparcel completo y propio: con sus propios ajustes, lista de torrents, canales RSS, puertos y ventana. Esta página explica cómo crear e iniciar perfiles, qué mantiene separado cada uno y el parámetro `--profile` para accesos directos y para la línea de comandos.

*Nuevo en la 1.2.0.* Hasta la 1.1.40, la única forma de ejecutar dos Rookparcel es con dos copias portátiles en carpetas separadas ([modo portátil](Dónde-guarda-Rookparcel-sus-archivos#modo-portátil)).

## Para qué sirven los perfiles

- Un Rookparcel que usa solo su VPN, junto a uno normal: cada perfil tiene su propio **Usar solo esta red**, su proxy y sus puertos ([Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos)).
- Trackers privados separados de los torrents públicos, cada uno con sus propios objetivos de siembra y fuentes de búsqueda.
- Probar un ajuste sin tocar su configuración real: `--profile test`.

Varios perfiles pueden ejecutarse al mismo tiempo, cada uno en su propia ventana. No comparten nada: un torrent de un perfil no se ve en otro.

## El perfil predeterminado

Sin un parámetro de perfil, Rookparcel ejecuta el **perfil predeterminado**: las carpetas descritas en [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos), exactamente como antes de que existieran los perfiles. No se mueve nada cuando empieza a usar perfiles.

Un perfil con nombre vive en una carpeta `profiles` dentro de la predeterminada:

| | perfil predeterminado | perfil `work` |
|---|---|---|
| Windows | `%APPDATA%\Rookparcel` | `%APPDATA%\Rookparcel\profiles\work` |
| Linux, BSD | `~/.config/rookparcel/gtk`, `~/.local/share/rookparcel/gtk` | `~/.config/rookparcel/profiles/work/gtk`, `~/.local/share/rookparcel/profiles/work/gtk` |
| macOS | `~/Library/Application Support/Rookparcel/mac` | `~/Library/Application Support/Rookparcel/profiles/work/mac` |
| carpeta portátil | la carpeta | `profiles\work` dentro de ella |

## Nombres de perfil

Un nombre tiene de 1 a 32 caracteres: letras a-z, dígitos y guiones (-), y empieza por una letra. Las mayúsculas se toman como minúsculas, así que `Work` y `work` son el mismo perfil. `default` significa el perfil predeterminado. Cualquier otro nombre se rechaza con un mensaje que lo indica.

## Crear e iniciar perfiles desde el menú

**Archivo > Perfiles** muestra el perfil predeterminado (como **Predeterminado**) y todos los perfiles con nombre. El que ejecuta esta ventana aparece marcado.

| Elemento | Qué hace |
|---|---|
| el nombre de un perfil | inicia ese perfil en una ventana propia. Si ya se está ejecutando, su ventana pasa al primer plano |
| **Nuevo perfil...** | pide un nombre («Nombre (letras a-z, dígitos y guiones):»), crea el perfil y lo inicia |
| **Crear acceso directo...** | Windows: pone en su escritorio un acceso directo que inicia este perfil, con su nombre, por ejemplo `Rookparcel (work).lnk`. Linux: pone el perfil en su menú de aplicaciones |
| **Abrir carpeta del perfil** | abre la carpeta de este perfil |

Un perfil nuevo empieza vacío, como una instalación nueva, y se abre con la [Guía de configuración](Primer-inicio).

## Distinguir las ventanas

La ventana de un perfil con nombre muestra su nombre en la barra de título y en la información emergente del icono de la bandeja: `Rookparcel 1.2.63 [work]`. El perfil predeterminado no muestra ningún nombre. En Windows, cada perfil tiene su propio grupo de botones en la barra de tareas, así que puede anclarlos por separado.

## Qué mantiene separado cada perfil

**Puertos.** Un perfil nuevo elige un puerto de escucha aleatorio entre 10000 y 60000 en su primer inicio, para no pelear con el perfil predeterminado por el 6881. El perfil predeterminado conserva su puerto. Si dos perfiles tienen el mismo puerto de todos modos, la barra de estado muestra «Puerto N en uso»; haga clic en ella para abrir **Opciones > Preferencias > Conexión**. Consulte [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos).

Cada perfil asigna su propio puerto en el router (UPnP, NAT-PMP). La asignación lleva el nombre del perfil, por ejemplo `Rookparcel (work)`, para poder distinguirlas en la lista del router.

**La interfaz web.** La interfaz web de cada perfil necesita su propio puerto. En la ventana de Windows, cuando activa la interfaz web en un perfil, **Opciones > Preferencias > Interfaz web** propone el primer puerto libre a partir del 8080. Si el puerto está ocupado de todos modos, dice «el puerto N está en uso, quizá por otro perfil de Rookparcel». Consulte [Interfaz web](Interfaz-web).

**Archivos `.torrent` y enlaces `magnet:`.** Solo un programa puede abrirlos, así que pertenecen a un perfil cada vez. Al marcar **Asociar con archivos .torrent** o **Asociar con enlaces magnet:** en **Opciones > Preferencias > General** de un perfil, se los da a ese perfil. En Windows, cuando otro perfil los tiene, la página lo indica: «Ahora los abre el perfil de Rookparcel "work"; marcar una casilla hace que los abra este perfil.» Al hacer doble clic en un archivo `.torrent`, se abre entonces en el perfil que posee la asociación, y se inicia ese perfil si no se está ejecutando.

Las casillas de asociación del instalador de Windows se las dan al perfil predeterminado.

**Inicio con el sistema.** **Iniciar Rookparcel al arrancar el sistema** se establece por perfil, así que cada perfil puede iniciarse por su cuenta. En Windows aparecen como `Rookparcel` y `Rookparcel (work)`.

**Ventanas remotas.** Una ventana que controla un `rookparcel-nox` remoto ([Control remoto](Control-remoto)) guarda sus ajustes dentro del perfil desde el que se abrió.

## Iniciar un perfil desde la línea de comandos

Añada `--profile NOMBRE` (o `--profile=NOMBRE`) al comando:

```
rookparcel.exe --profile work
rookparcel-gtk --profile work
```

- Puede ir en cualquier lugar entre los argumentos, y el nombre nunca se toma como un torrent: `rookparcel.exe --profile work file.torrent` añade `file.torrent` al perfil `work`.
- `--profile default`, o ningún parámetro, es el perfil predeterminado.
- Cuando el perfil todavía no existe, la ventana pregunta primero: «El perfil "work" todavía no existe. ¿Crearlo?», y muestra la carpeta que creará. **No** cierra sin crear nada, así que un nombre mal escrito no inicia un Rookparcel vacío.
- Se combina con los demás parámetros, por ejemplo `--minimized` para iniciar en la bandeja, o `--remote` para conectar con un `rookparcel-nox` remoto.

Para crear un acceso directo de Windows a mano, añada `--profile work` después de `rookparcel.exe` en el cuadro **Destino** del acceso directo. **Archivo > Perfiles > Crear acceso directo...** hace lo mismo por usted.

## Perfiles y modo portátil

Un Rookparcel portátil también tiene perfiles, en la carpeta `profiles` dentro de la carpeta portátil. Viajan con ella. Consulte [Dónde guarda Rookparcel sus archivos](Dónde-guarda-Rookparcel-sus-archivos#modo-portátil).

## Diferencias en GTK

- **Archivo > Perfiles** funciona igual. El perfil actual se muestra como el elemento seleccionado.
- **Crear acceso directo...** pone el perfil en su menú de aplicaciones («Rookparcel (work)»). El perfil tiene una entrada de escritorio propia, así que el dock y la barra de tareas muestran su ventana con su propio nombre e icono.
- En el **Flatpak**, los perfiles funcionan, pero solo el perfil predeterminado puede iniciarse con el sistema.

## Diferencias en macOS

- **Archivo > Perfiles** muestra los perfiles, **Nuevo perfil...** y **Abrir carpeta del perfil**; no hay **Crear acceso directo...**.
- Abrir un archivo `.torrent` o un enlace `magnet:` llega siempre al perfil predeterminado: macOS elige la aplicación, no un perfil. El inicio con el sistema funciona por perfil.

## Perfiles de rookparcel-nox

`rookparcel-nox` también acepta `--profile NOMBRE`, para ejecutar varios demonios en un equipo, cada uno con sus propios torrents, interfaz web y servicio. Un perfil de demonio no pregunta antes de crearse. Su puerto de escucha se deriva de su nombre, así que es el mismo en cada inicio. Un perfil de GUI y un perfil de `rookparcel-nox` con el mismo nombre son programas distintos que solo comparten la carpeta `profiles/<nombre>`. Consulte [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

## Quitar un perfil

1. Cierre la ventana del perfil (**Archivo > Salir**).
2. Si posee las asociaciones o se inicia con el sistema, desmárquelas antes en sus Preferencias, o dé las asociaciones a otro perfil.
3. Borre su carpeta, `profiles\<nombre>`. **Archivo > Perfiles > Abrir carpeta del perfil** muestra dónde está.

Las descargas del perfil se quedan donde están.
