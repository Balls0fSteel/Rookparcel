*Esta es una traducción de la página inglesa [Keep ping low](https://github.com/Balls0fSteel/Rookparcel/wiki/Keep-ping-low). Si hay diferencias, prevalece la página inglesa.*

# Mantener el ping bajo

Mantener el ping bajo es el «modo jugador» de Rookparcel: vigila el retardo de su línea de internet y mantiene bajas las velocidades de Rookparcel mientras el retardo crece, de modo que los juegos, las videollamadas y la navegación web sigan respondiendo bien mientras funcionan los torrents. Esta página explica qué hace, cómo activarlo y qué se ve mientras trabaja.

## Por qué los torrents aumentan su ping

La subida suele ser la parte más estrecha de su línea. Cuando Rookparcel envía más rápido de lo que la línea puede transportar, los paquetes esperan en una cola del módem o del router, y todos los demás paquetes esperan detrás de ellos: los de un juego, los de una llamada, los de una página web. Esa cola es lo que se nota como lag. Enviar un poco más despacio que la línea mantiene la cola vacía y el ping en su valor en reposo. La dirección de bajada tiene el mismo problema en el extremo de su proveedor de la línea.

Mantener el ping bajo no necesita saber qué juego se está ejecutando. Mide la propia cola, así que funciona con cualquier juego, llamada u otro programa.

## Activarlo

1. Abra **Opciones > Preferencias** (<kbd>Ctrl</kbd>+<kbd>P</kbd>) y elija **Juegos**.
2. En **Ping**, marque **Mantener el ping bajo (limitar las velocidades automáticamente)**.
3. Haga clic en **Aceptar** o **Aplicar**.

![Preferencias, página Juegos](images/prefs-gaming.png)

Está desactivado de forma predeterminada. Para desactivarlo de nuevo, desmarque la casilla: sus propios límites vuelven de inmediato.

Cuando lo activa por primera vez, Rookparcel aprende el ping en reposo de su línea. Eso tarda hasta unos 20 segundos, durante los cuales las velocidades pueden empezar más bajas.

## Qué hace

- **Mide el retardo.** Rookparcel hace ping al primer router más allá de su línea doméstica unas cuatro veces por segundo (unos cientos de bytes por segundo). Si ningún router más allá de la línea responde, hace ping a los servidores indicados en el ajuste avanzado `auto_limit_probe_hosts`, que de forma predeterminada es `1.1.1.1,8.8.8.8,9.9.9.9`. Los pares uTP ocupados dan una segunda medición sin tráfico adicional.
- **Limita ambas direcciones.** Mientras el retardo se mantenga por encima de su valor en reposo en más de lo permitido, se reduce el límite de subida o de bajada; cuando la línea vuelve a estar tranquila, el límite crece paso a paso.
- **Nunca aumenta sus propios límites.** Se usa el menor entre el límite automático y su propio límite global. Sus ajustes no se modifican.
- **Tiene mínimos.** El límite no baja de unos 10 kB/s, ni de una pequeña parte de la velocidad más alta observada, de modo que un Wi-Fi saturado o la subida de otro programa no puedan dejar a Rookparcel sin recursos por completo.
- **uTP cede antes.** Mientras está activado, las conexiones uTP apuntan a un retardo de cola de 25 ms como máximo, en lugar de los 100 ms habituales.

La nota bajo la casilla también lo dice: esos pings van al primer router más allá de su línea, o si no a los servidores de reserva, cuyos operadores los ven.

## Qué se ve

- **La barra de estado.** Mientras una dirección está limitada por debajo de su propio límite, su parte muestra el límite entre paréntesis, por ejemplo **S: 412.0 kB/s (auto 450.0 kB/s) T: 2.1 GB**. Sin límite, tiene el aspecto habitual.
- **La pestaña Registro.** Las líneas que empiezan por «Keep ping low:» dicen a qué router o servidores se hace ping, y cuándo una dirección queda limitada o deja de estarlo.
- **La interfaz web** muestra el mismo «(auto ...)» en su barra de estado.

Es normal que las velocidades sean más bajas que el máximo de su línea mientras está activado: ese margen es lo que mantiene bajo el ping.

## Ajuste fino

Dos ajustes de **Opciones > Preferencias > Avanzado** pertenecen a esta función. Seleccione la fila, introduzca el valor, haga clic en **Establecer**, y después en **Aceptar** o **Aplicar**.

| Ajuste | Significado | Valor predeterminado |
|---|---|---|
| `auto_limit_delay_ms` | Milisegundos de ping adicional permitidos por encima del ping en reposo de la línea antes de reducir los límites. Un valor menor mantiene los juegos y las llamadas más fluidos; uno mayor permite más velocidad. | 15 |
| `auto_limit_probe_hosts` | Direcciones IP separadas por comas a las que hacer ping cuando ningún router más allá de la línea responde. Se lee cuando se activa la función. Rara vez hace falta cambiarlo. | `1.1.1.1,8.8.8.8,9.9.9.9` |

`utp_target_delay` (100 de forma predeterminada) es el objetivo de retardo de uTP que la función reduce a 25 mientras está activada.

## Plataformas

| Sistema | Cómo mide |
|---|---|
| Windows | Pings, sin necesidad de derechos de administrador, más los pares uTP. |
| Linux | Pings mediante los sockets de ping sin privilegios del sistema, más los pares uTP. Si el sistema no los permite, el Registro dice «this system does not let Rookparcel ping (its group must be in net.ipv4.ping_group_range)» y solo se usan los pares uTP. |
| macOS | Pings, más los pares uTP. |
| FreeBSD y otros sistemas sin pings sin privilegios | Solo los pares uTP. El Registro dice que los pings no están disponibles. |

Con solo pares uTP, necesita al menos tres pares uTP ocupados para medir algo; sembrar solo a pares TCP no le da nada en que basarse.

## Cuando algo distinto es mejor

Un router con SQM (como CAKE) o un QoS para juegos que funcione bien hace esto mejor, para todos los dispositivos de la casa, no solo para Rookparcel. Si tiene uno, puede dejar Mantener el ping bajo desactivado.

## Funciones relacionadas

- La ralentización por disco ocupado de la misma página **Juegos** usa el mismo tipo de límite automático, para un disco lento en lugar de una línea lenta; gana el menor de los dos límites. Consulte [Disco y almacenamiento](Disco-y-almacenamiento#cuando-el-disco-está-ocupado).
- La página del router de la Guía de configuración menciona Mantener el ping bajo para juegos y llamadas: consulte [Primer inicio](Primer-inicio).
- Límites fijos y el programador semanal: [Límites de velocidad y programador](Límites-de-velocidad-y-programador).

## En la ventana GTK y en la interfaz web

La ventana GTK (Linux, BSD) y la aplicación de macOS tienen la misma casilla en **Preferencias > Juegos** (en el Mac se aplica en cuanto la marca), y la interfaz web la tiene en su página **Preferencias > Juegos**, con la misma nota. En [rookparcel-nox](Ejecutar-rookparcel-nox) es el mismo ajuste, `auto_rate_limit`, que se define en las Preferencias de la interfaz web.
