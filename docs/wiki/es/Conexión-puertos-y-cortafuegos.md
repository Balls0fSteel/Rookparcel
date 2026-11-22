*Esta es una traducción de la página inglesa [Connection, ports and firewall](https://github.com/Balls0fSteel/Rookparcel/wiki/Connection-ports-and-firewall). Si hay diferencias, prevalece la página inglesa.*

# Conexión, puertos y cortafuegos

Esta página explica cómo llegan otros pares hasta Rookparcel: el puerto de escucha, el reenvío de puertos con UPnP y NAT-PMP, el cortafuegos de Windows, cómo saber si las conexiones entrantes funcionan, cómo mantener el tráfico de torrents en una VPN con «Usar solo esta red», los proxies, el cifrado, el filtro IP y su lista de bloqueo desde una dirección web, y DHT, el intercambio de pares y el descubrimiento de pares locales.

La mayoría de estos ajustes están en **Opciones > Preferencias > Conexión** y **Opciones > Preferencias > BitTorrent**.

![Preferencias, página Conexión](images/prefs-connection.png)

## Por qué importan las conexiones entrantes

Rookparcel se conecta con los pares, y los pares se conectan con Rookparcel. Las descargas funcionan solo con conexiones salientes, pero un Rookparcel al que otros pueden llegar encuentra más pares, y dos pares a los que ninguno puede llegar no pueden comunicarse en absoluto. Para tener buenas velocidades, y para sembrar, permita la entrada de pares.

## El puerto de escucha

En **Puerto de escucha** en la página Conexión:

| Ajuste | Significado |
|---|---|
| **Puerto para las conexiones entrantes:** | El puerto al que se conectan los demás pares. Un cambio se aplica de inmediato. |
| **Puerto aleatorio** | Rellena un puerto aleatorio de 10000 a 60000. Se usa cuando lo aplica. |
| **Puerto aleatorio en cada inicio** | Elige un nuevo puerto aleatorio de 10000 a 60000 en cada inicio. Un puerto reenviado a mano en el router deja entonces de coincidir. Desactivado de forma predeterminada. |
| **Activar la asignación de puertos UPnP** | Pide a un router UPnP que reenvíe el puerto a este equipo. Activado de forma predeterminada. |
| **Activar la asignación de puertos NAT-PMP** | Pide al router que reenvíe el puerto mediante NAT-PMP o PCP, que muchos routers ofrecen además de UPnP o en su lugar. Activado de forma predeterminada. |

El perfil predeterminado escucha en el puerto 6881. Un perfil nuevo, o una copia portátil, elige un puerto aleatorio de 10000 a 60000 en su primer inicio, para no chocar con otro Rookparcel. Consulte [Perfiles](Perfiles).

Rookparcel escucha en TCP y UDP en el mismo puerto: UDP transporta los pares uTP y DHT.

### Reenviar el puerto en el router

Con UPnP o NAT-PMP activados y un router que los admita, no hay nada que hacer: Rookparcel pide al router que reenvíe el puerto y renueva la solicitud. Si su router los tiene desactivados, o funciona mal, desactive ambas casillas y reenvíe el puerto a mano en los ajustes propios del router: TCP y UDP, a la dirección de este equipo. Después deje desactivado **Puerto aleatorio en cada inicio**, para que el puerto siga siendo el que reenvió.

## El cortafuegos de Windows

Rookparcel no añade por sí mismo una regla al Firewall de Windows. Windows suele preguntar la primera vez que un programa escucha conexiones entrantes; permita ahí Rookparcel.

Si descartó esa pregunta, o no preguntó nada, añada una regla para el programa desde un símbolo del sistema ejecutado como administrador. Para una instalación para un solo usuario:

```bat
netsh advfirewall firewall add rule name="Rookparcel" dir=in action=allow program="%LOCALAPPDATA%\Programs\Rookparcel\rookparcel.exe"
```

Para una instalación para todos los usuarios, la ruta es `C:\Program Files\Rookparcel\rookparcel.exe`; para el zip, donde haya descomprimido `rookparcel.exe`. El puerto de la interfaz web necesita la misma regla cuando otros equipos deban llegar a él: consulte [Interfaz web](Interfaz-web). Para [rookparcel-nox](Ejecutar-rookparcel-nox) ejecutándose como servicio, consulte su página.

En Linux y en los BSD, permita el puerto en su cortafuegos si usa uno.

## Comprobar que los pares pueden conectarse

La parte izquierda de la barra de estado se lo indica:

| Barra de estado | Significado |
|---|---|
| **Red OK** | Al menos un par se ha conectado desde fuera. El puerto funciona. |
| **Sin conexiones entrantes** | Ningún par se ha conectado todavía desde fuera. Es normal en los primeros minutos tras un inicio; si se mantiene, el puerto no es accesible. |
| **Puerto 6881 en uso** (con su número de puerto) | Otro programa, a menudo otro perfil de Rookparcel, ocupa el puerto, y Rookparcel escucha en un puerto posterior donde no hay nada reenviado. Haga clic en él para abrir Preferencias en Conexión y elegir otro puerto. |

Pueden aparecer dos avisos antes de estas palabras. **Transferencias retenidas: confirme las preferencias** significa que Rookparcel retiene todas las transferencias hasta que usted confirme los ajustes tras un cambio de versión: consulte [Actualizar Rookparcel](Actualizar-Rookparcel). **Esperando la red** significa que «Usar solo esta red» no encuentra ningún adaptador que coincida: consulte la sección siguiente.

*«Puerto ... en uso» es nuevo en 1.1.45.*

Mantenga el puntero sobre esa parte para ver **Su dirección según los trackers:**, la dirección pública que los trackers devuelven. La pestaña **General** muestra la misma línea. *Nuevo en 1.1.58.*

Rookparcel no tiene una prueba de puertos integrada. Si el estado sigue en «Sin conexiones entrantes» con torrents activos:

1. Compruebe que el cortafuegos permite Rookparcel (arriba).
2. Compruebe que UPnP o NAT-PMP están activados en Rookparcel y en el router, o reenvíe el puerto a mano.
3. Con «Usar solo esta red» configurado en una VPN, los pares solo pueden llegar a usted mediante el reenvío de puertos propio de la VPN, si lo tiene: Rookparcel no solicita asignación de puertos a un adaptador VPN.

Consulte también [Solución de problemas y preguntas frecuentes](Solución-de-problemas-y-preguntas-frecuentes).

## Usar solo esta red (VPN)

*Nuevo en 1.1.58.*

«Usar solo esta red» mantiene todo el tráfico de torrents en un solo adaptador de red, normalmente una VPN. Los pares, DHT, los trackers y las semillas web pasan todos por él y por nada más. Cuando la VPN se cae, los torrents esperan en lugar de continuar por su conexión normal.

1. Conecte su VPN.
2. Abra **Opciones > Preferencias > Conexión**.
3. En **Usar solo esta red**, elija en **Red:**
   - **Cualquier adaptador VPN**: cualquier adaptador que parezca un túnel VPN (WireGuard, OpenVPN, el propio de una aplicación de VPN). Es lo que quiere la mayoría de la gente.
   - Un adaptador por su nombre, de la lista de adaptadores de este equipo.
   - **Rango de direcciones (abajo)**, y después escriba el rango en **Rango de direcciones:**, como `10.2.0.0/16` o `fd00::/8`.
   - **Cualquier red (desactivado)** lo desactiva; es el valor predeterminado.
4. Deje marcada **Pausar los torrents mientras falte esta red**.
5. Haga clic en **Aceptar** o **Aplicar**.

Cuál elegir:

- **Cualquier adaptador VPN** resiste a que la aplicación de VPN cambie el nombre de su adaptador o lo recree, algo que muchas hacen en cada conexión o actualización.
- **Rango de direcciones** va bien para una VPN que siempre reparte direcciones del mismo rango: el adaptador que tenga una dirección en él transporta el tráfico, sea cual sea su nombre.
- **Un adaptador por su nombre** es la opción más restrictiva. Un adaptador que no está ahora aparece como «(no conectado)» en la lista.

La regla se vuelve a comprobar en cada cambio de red, así que se sigue a una VPN que se reconecta con una dirección nueva.

**Mientras falta la red:** con **Pausar los torrents mientras falte esta red** activado, los torrents en ejecución esperan. Su estado muestra **Esperando la red**, la barra de estado empieza con **Esperando la red**, y el Registro dice qué regla no encontró nada. Cuando vuelve la VPN, continúan por sí solos. Detener e Iniciar siguen funcionando con ellos mientras tanto. Con la casilla desactivada, no se pausa nada, pero tampoco puede salir tráfico de torrents, ya que cada conexión necesita el adaptador que coincide.

**Comprobar que funciona:** mantenga el puntero sobre la parte de red de la barra de estado, o mire la pestaña **General**: **Su dirección según los trackers:** debe mostrar la dirección de la VPN, no la suya. Dice «aún desconocida» hasta que un tracker ha respondido.

Conviene saber:

- Si la VPN lleva solo IPv4, Rookparcel no usa IPv6 para los torrents mientras está vinculado a ella, de modo que IPv6 no puede filtrarse por fuera del túnel.
- La interfaz web y las conexiones remotas a Rookparcel no están vinculadas, así que puede seguir accediendo a ellas por su red normal.
- Las conexiones de proxy tampoco están vinculadas. El asistente de búsqueda es un programa independiente y no está vinculado; consulte [Buscar torrents en Windows](Buscar-torrents-en-Windows#conviene-saber).
- En la interfaz web, el mismo grupo está en **Preferencias > Conexión**. Para rookparcel-nox en la línea de comandos: `--network vpn`, `--network adapter:ID`, `--network range:10.2.0.0/16` u `off`; y `--no-pause-when-missing` para dejar los torrents funcionando sin vincular.

## Servidor proxy

En **Servidor proxy** en la página Conexión. Los campos del proxy aparecen en gris mientras **Tipo:** es **(ninguno)**, el valor predeterminado.

| Ajuste | Significado |
|---|---|
| **Tipo:** | **Socks4**, **Socks5** o **HTTP** (que canaliza todo excepto las solicitudes http simples mediante CONNECT). **(ninguno)** se conecta directamente. |
| **Proxy:** y **Puerto:** | El nombre de servidor o la dirección IP del servidor proxy, y su puerto, como 1080 para muchos servidores SOCKS. |
| **Autenticación** con **Usuario:** y **Contraseña:** | Inicia sesión en el proxy. Un proxy SOCKS4 recibe solo el nombre de usuario. |
| **Resolver nombres de servidor a través del proxy** | Hace que el proxy resuelva los nombres de los trackers y de las semillas web, de modo que su propio servidor DNS no los vea. |
| **Usar el proxy para las conexiones entre pares** | Envía las conexiones con pares y semillas web a través del proxy. Entonces se rechazan las conexiones entrantes, y la asignación de puertos y el descubrimiento de pares locales siguen desactivados. uTP y DHT necesitan un proxy SOCKS5. |
| **Usar el proxy para las conexiones con trackers** | Envía los anuncios a los trackers a través del proxy, de modo que los trackers ven la dirección del proxy. |

Los tres últimos están marcados de forma predeterminada y solo cuentan una vez que se elige un tipo de proxy. Un proxy SOCKS5 de un proveedor de VPN es una forma de usar una VPN sin un adaptador; «Usar solo esta red» es la otra.

## Cifrado del protocolo

En **Opciones > Preferencias > BitTorrent**, en **Cifrado del protocolo**:

| Ajuste | Significado |
|---|---|
| **Saliente:** | Cómo se cifran las conexiones que abre Rookparcel. **Desactivado**: nunca. **Activado** (el valor predeterminado): los intentos sin cifrar y cifrados se alternan por par, conservando lo que funcionó. **Forzado**: solo cifradas, con lo que se pierden los pares que no pueden cifrar. |
| **Permitir conexiones entrantes sin cifrar** | Acepta conexiones entrantes que no están cifradas. Si está desactivado, solo se aceptan las cifradas, con lo que se pierden los pares que no pueden cifrar. Activado de forma predeterminada. |

Cambie esto si su proveedor ralentiza el tráfico de BitTorrent. La página Avanzado tiene los ajustes más finos (`out_enc_policy`, `in_enc_policy`, `allowed_enc_level`, `prefer_rc4`).

![Preferencias, página BitTorrent](images/prefs-bittorrent.png)

## DHT, intercambio de pares, descubrimiento de pares locales y uTP

En **Funciones básicas de BitTorrent** en la página BitTorrent, todas activadas de forma predeterminada:

| Ajuste | Significado |
|---|---|
| **Activar la red DHT** | Encuentra pares a través de DHT, una red de clientes BitTorrent, sin un tracker. La mayoría de los enlaces magnet la necesitan. |
| **Activar el descubrimiento de pares locales** | Encuentra pares de los mismos torrents en su red local, de modo que intercambian datos directamente a la velocidad de la red. |
| **Activar el intercambio de pares** | Permite que los pares conectados se informen entre sí de más pares. Un cambio se aplica a todos los torrents. |
| **Activar uTP (transporte UDP con gestión del ancho de banda)** | Se conecta con pares y los acepta mediante uTP, un transporte UDP que reduce su velocidad cuando la red está ocupada, además de mediante TCP. |

Los torrents privados nunca usan DHT, el intercambio de pares ni el descubrimiento de pares locales, digan lo que digan estas casillas. Las Propiedades de un torrent pueden desactivar DHT y el intercambio de pares solo para ese torrent (**Activar DHT** e **Intercambio de pares** en **Otros ajustes**).

La barra de estado muestra **DHT: N nodos**, cuántos nodos DHT conoce Rookparcel.

Para permitir solo algunos tipos de conexión, como solo TCP, use `bt.transp_disposition` en la página Avanzado: 1 TCP saliente, 2 uTP saliente, 4 TCP entrante, 8 uTP entrante, sumados; 15 (el valor predeterminado) permite todos, 5 es solo TCP.

## Filtro IP

En **Filtro IP** en la página BitTorrent, Rookparcel puede bloquear pares por dirección, a partir de una lista de bloqueo en formato eMule `ipfilter.dat`, PeerGuardian P2P o CIDR. La lista puede ser un archivo de su equipo o un archivo de la web que Rookparcel descarga y mantiene al día.

Cada línea de una lista es una regla:

| Formato | Línea de ejemplo |
|---|---|
| eMule `ipfilter.dat` | `001.002.004.000 - 001.002.004.255 , 000 , Some name` |
| PeerGuardian P2P | `Some name:1.2.4.0-1.2.4.255` |
| CIDR, IPv4 o IPv6 | `10.0.0.0/8` o `fd00::/8` |

*Las líneas CIDR son nuevas en 1.2.63.* También funcionan en un archivo de su equipo.

### Un archivo de lista en su equipo

1. Marque **Activar el filtro IP**.
2. Elija el archivo de la lista de bloqueo en el cuadro de debajo. Si se deja vacío, Rookparcel usa `ipfilter.dat` en su carpeta de ajustes.
3. Haga clic en **Recargar** para cargarlo ahora y ver cuántas reglas contiene («N reglas cargadas.»), o haga clic en **Aceptar** para cargarlo con los demás ajustes.

La lista se carga al iniciar y al aplicar. El Registro escribe «Filtro IP: N reglas de ...» o el error. Si no se puede leer una lista nueva, las reglas cargadas antes siguen en vigor.

En la interfaz web no hay botón Recargar: la lista se carga al aplicar.

### Una lista desde una dirección web

*Nuevo en 1.2.63.* Rookparcel puede descargar la lista de bloqueo de la web y mantenerla al día:

1. Marque **Activar el filtro IP**.
2. Marque **Obtener la lista de bloqueo de esta dirección (https):** y escriba la dirección de la lista en el cuadro de debajo. Debe empezar por `https://`.
3. Haga clic en **Actualizar ahora** para descargarla de inmediato, o en **Aceptar**. La línea bajo el botón muestra el resultado.

Mientras se usa la dirección, esta sustituye al archivo: el cuadro del archivo y **Recargar** aparecen atenuados. Qué conviene saber:

- **Solo `https://`.** Se rechaza una dirección que redirija a Rookparcel a una `http://` simple. La descarga pasa por su proxy cuando Preferencias > Conexión define uno.
- **Formatos.** Los tres formatos de arriba, simples o comprimidos con gzip. Los archivos zip no se admiten. Una descarga puede ser de hasta 32 MiB.
- **Con qué frecuencia.** Una vez al día, y una hora después de una descarga fallida. **Actualizar ahora** la descarga en cualquier momento.
- **Al iniciar.** Rookparcel conserva la última descarga correcta en su carpeta de ajustes (`ipfilter-url.dat`) y la pone en vigor de inmediato en el siguiente inicio, antes de descargar nada.
- **Una dirección nueva.** La copia de la lista de la dirección anterior permanece en la carpeta de ajustes hasta que se haya descargado y comprobado una lista de la dirección nueva, de modo que un error tipográfico o una descarga fallida no pierden nada, y volver a la dirección anterior usa de nuevo su copia. Una copia solo se usa para la dirección de la que procede: al iniciar con una dirección nueva, Rookparcel no tiene ninguna lista de bloqueo en vigor hasta que llega la nueva, y el Registro lo dice. En el programa de Windows, **Actualizar ahora** con una dirección que aún no ha aplicado solo la prueba: la línea muestra el resultado, y nada cambia hasta **Aceptar** o **Aplicar**.
- **Cuando falla una descarga** o trae algo que no es una lista de bloqueo (una página de error, por ejemplo), las reglas que ya están en vigor siguen, y la línea de estado dice «La última descarga ha fallado: ...». El Registro también escribe el error.
- **Línea de estado.** «N reglas, descargadas el» con la fecha y la hora, o «La lista aún no se ha descargado.» hasta que la primera descarga tenga éxito. Hasta entonces, las reglas que ya están en vigor siguen.
- **La comprobación de ajustes.** Mientras Rookparcel retiene las transferencias tras un cambio de versión, hasta que usted confirme los ajustes ([Actualizar Rookparcel](Actualizar-Rookparcel)), la descarga también espera.

Use una dirección de confianza: quien controla la lista decide con qué pares se niega a comunicarse Rookparcel.

En la interfaz web, cambiar el cuadro o la dirección requiere la contraseña actual (**Contraseña actual:** en la página de la interfaz web), porque el servidor descarga cualquier dirección que se defina. La página muestra la dirección sin nombre de usuario, contraseña ni consulta (las credenciales de una suscripción suelen estar ahí); si se deja tal cual, se conserva la dirección guardada. Para quitar las credenciales, vacíe el cuadro, aplique y vuelva a escribir la dirección. La interfaz web que sirve el programa de Windows omite la dirección: pertenece a las Preferencias propias de la ventana. La API compatible con qBittorrent no puede cambiarla.

### Trackers y DHT

De forma predeterminada, el filtro también se aplica a DHT y a los trackers en direcciones bloqueadas (`apply_filter_to_dht` y `apply_ip_filter_to_trackers` en la página Avanzado). Desactive el de los trackers si un tracker que necesita está en un rango bloqueado.

## En la ventana GTK y en la interfaz web

- **Ventana GTK**: las mismas páginas y ajustes. La parte de red de la barra de estado dice lo mismo, y un clic en «Puerto ... en uso» abre Preferencias en Conexión.
- **Aplicación de macOS**: las mismas páginas, que se aplican de inmediato sin **Aceptar** ([Referencia de preferencias](Referencia-de-preferencias#diferencias-en-macos)).
- **Interfaz web**: las mismas páginas en **Preferencias**; su barra de estado muestra la parte de red con la misma información emergente.
- **rookparcel-nox**: `--port N` define el puerto de escucha (6881 de forma predeterminada), `--listen` define las interfaces de escucha por completo, `--proxy TYPE://[USER:PASS@]HOST:PORT` define un proxy, `--ip-filter FILE` carga un archivo de lista de bloqueo (la dirección se define en las Preferencias de la interfaz web), y `--network` define «Usar solo esta red». Un servicio en Windows necesita una regla de cortafuegos para `rookparcel-nox.exe`: consulte [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

## Páginas relacionadas

- [Límites de velocidad y programador](Límites-de-velocidad-y-programador)
- [Perfiles](Perfiles) (puertos para varios Rookparcel)
- [Solución de problemas y preguntas frecuentes](Solución-de-problemas-y-preguntas-frecuentes)
- [Referencia de preferencias](Referencia-de-preferencias)
