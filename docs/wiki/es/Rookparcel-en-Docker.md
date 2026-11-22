*Esta página es una traducción de la página inglesa [Docker](Docker). Si hay diferencias, prevalece la página en inglés.*

# Rookparcel en Docker

`rookparcel-nox`, el demonio sin ventana, también se ofrece como imagen de contenedor para Docker y otros motores de contenedores. Esta página explica cómo iniciarlo con `docker run` o Docker Compose, sus carpetas y puertos, la contraseña de la interfaz web, cómo ejecutarlo como otro usuario, cómo usarlo con Sonarr, Radarr y Prowlarr en contenedores, y cómo detenerlo y actualizarlo.

*Nuevo en 1.2.21.*

La imagen es `ghcr.io/balls0fsteel/rookparcel-nox`, para Linux x86-64 (`amd64`) y ARM de 64 bits (`arm64`). Cada versión tiene su propia etiqueta, como `:1.2.21`, y `:latest` es la versión más reciente (no una versión preliminar). Dentro, nox se ejecuta igual que en cualquier equipo Linux: todo lo de [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox) y de [Interfaz web](Interfaz-web) se aplica.

## Un primer inicio

```sh
docker run -d --name rookparcel --restart unless-stopped --stop-timeout 60 \
  -p 8080:8080 -p 6881:6881 -p 6881:6881/udp \
  -v rookparcel-config:/config -v rookparcel-downloads:/downloads \
  ghcr.io/balls0fsteel/rookparcel-nox
```

1. Lea la contraseña temporal en el registro:

   ```sh
   docker logs rookparcel
   ```

   ```text
   * Web UI: no password set; temporary password for this run: ... (set one with --webui-pass)
   ```

2. Abra `http://127.0.0.1:8080/` (o `http://<la dirección del equipo>:8080/` desde otro equipo) e inicie sesión como `admin` con ella.
3. En la página, abra **Preferencias > Interfaz web** y defina su propia **Contraseña:**, con la temporal como **Contraseña actual:**.

La contraseña temporal se muestra solo en el primer inicio, y sigue siendo válida hasta que defina la suya. Si el registro desaparece antes de que lo lea (se eliminó el contenedor), defina una contraseña con `ROOKPARCEL_WEBUI_PASS` ([más abajo](#la-contraseña-de-la-interfaz-web)).

Sin una orden, la imagen ejecuta:

```sh
rookparcel-nox run --state-dir /config --save /downloads --webui 0.0.0.0:8080 --port 6881
```

Las opciones que se escriben después del nombre de la imagen se añaden a esa orden, y una opción dada de nuevo prevalece sobre el valor predeterminado: `... ghcr.io/balls0fsteel/rookparcel-nox --save /srv/media/downloads --verbose`. Una orden escrita después del nombre de la imagen la sustituye en cambio: `... ghcr.io/balls0fsteel/rookparcel-nox run --state-dir /config --save /srv/media/downloads --webui 0.0.0.0:8080 --port 6881`. Todas las opciones están enumeradas en [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox#opciones-de-run-y-add).

## Carpetas

| Carpeta | Qué contiene |
|---|---|
| `/config` | Todo lo que guarda nox: la lista de torrents y el progreso de cada uno, el estado de DHT, el inicio de sesión y las preferencias de la interfaz web (`webui.json`), las etiquetas, las marcas y el Descargador RSS. Manténgala en un volumen, o un reinicio empezará con una lista vacía. |
| `/downloads` | La carpeta de descargas predeterminada. |

Ambas son volúmenes. Para Sonarr, Radarr y Prowlarr, monte en cambio su carpeta de medios en la misma ruta que usan las aplicaciones, y guarde en ella ([más abajo](#sonarr-radarr-y-prowlarr-en-contenedores)).

Para hacer una copia de seguridad, detenga el contenedor y copie el volumen `/config`. Para empezar de nuevo, elimine el contenedor y el volumen.

## Puertos

| Puerto | Para qué |
|---|---|
| `8080/tcp` | La interfaz web y la API compatible con qBittorrent. |
| `6881/tcp`, `6881/udp` | El puerto de escucha para los pares (UDP transporta uTP y el DHT). |

Asigne el puerto de escucha al mismo número fuera y dentro (`-p 6881:6881`): nox comunica a los trackers y a los pares el puerto en el que escucha, así que `-p 6882:6881` les daría uno equivocado. Para usar otro puerto, cambie ambos, por ejemplo `-p 51413:51413 -p 51413:51413/udp` y `--port 51413`. Un puerto que se cambie más tarde en **Preferencias > Conexión** de la interfaz web necesita el mismo cambio en la asignación.

**UPnP y NAT-PMP no funcionan desde la red habitual de Docker (bridge):** el router no es alcanzable desde dentro de ella, así que el puerto no se redirige por usted. Redirija el puerto en el router a mano, TCP y UDP, al equipo que ejecuta Docker ([Redirigir el puerto en el router](Conexión-puertos-y-cortafuegos#reenviar-el-puerto-en-el-router)), o, en Linux, ejecute el contenedor en la red del anfitrión (`--network host`), donde nox ve el router y la red local como lo hace un programa fuera de un contenedor, y `-p` no hace falta. El Descubrimiento local de pares también encuentra pares en su red solo con la red del anfitrión.

## La contraseña de la interfaz web

| Variable | Significado |
|---|---|
| `ROOKPARCEL_WEBUI_PASS` | Define la contraseña de la interfaz web en cada inicio. Se guarda con hash en `/config`, así que permanece después de que quite la variable. |
| `ROOKPARCEL_WEBUI_USER` | El nombre de usuario que la acompaña (`admin` de forma predeterminada). Se usa solo junto con `ROOKPARCEL_WEBUI_PASS`. |

```sh
docker run -d --name rookparcel ... -e ROOKPARCEL_WEBUI_PASS='a long password' ghcr.io/balls0fsteel/rookparcel-nox
```

Sin `ROOKPARCEL_WEBUI_PASS`, nox conserva la contraseña guardada, o muestra una temporal cuando aún no hay ninguna guardada.

Dos cosas que conviene saber:

- Mientras la variable está definida, prevalece en cada inicio: una contraseña que cambió en la página vuelve a la de la variable en el siguiente inicio. Quite la variable para conservar la definida en la página.
- `docker inspect` muestra la variable, y nox recibe la contraseña en su línea de órdenes, que `docker top` y la lista de procesos del anfitrión muestran. En un equipo en el que otros pueden iniciar sesión, use en cambio la contraseña temporal y defina la suya en la página, o quite la variable después del primer inicio.

La interfaz web es HTTP simple. En una red en la que no confíe, sírvala por HTTPS (`--webui-https`) o póngala detrás de un proxy inverso que lo haga: vea [HTTPS](Interfaz-web#https) y [Seguridad de la interfaz web](Interfaz-web#seguridad).

## Usuario y permisos

nox se ejecuta como el usuario `rookparcel`, con ID de usuario y de grupo 1000, nunca como root. Los archivos nuevos son legibles por todos y escribibles por su grupo (umask `0002`), de modo que las aplicaciones de otros contenedores pueden importarlos y quitarlos.

| Para | Haga |
|---|---|
| Escribir en una carpeta del anfitrión que pertenece a otro usuario | Inicie el contenedor como ese usuario: `--user 1001:1001` (`user: "1001:1001"` en Compose). Averigüe los ID con `id -u` y `id -g`. |
| Usar otra umask | `-e UMASK=0022` (archivos legibles por todos, escribibles solo por su propietario). |

Con otro usuario, `/config` también debe ser escribible por él. Un volumen con nombre nuevo pertenece al usuario 1000, así que use en cambio una carpeta del anfitrión que dé a ese usuario:

```sh
sudo mkdir -p /srv/rookparcel && sudo chown 1001:1001 /srv/rookparcel
docker run -d --name rookparcel --user 1001:1001 -v /srv/rookparcel:/config ...
```

## Docker Compose

El repositorio tiene un [compose.yaml](https://github.com/Balls0fSteel/Rookparcel/blob/main/contrib/docker/compose.yaml) ya preparado. Guarda `/config` en un volumen, monta `/srv/media` en la misma ruta, guarda en `/srv/media/downloads` y da a nox 60 segundos para detenerse:

```yaml
services:
  rookparcel:
    image: ghcr.io/balls0fsteel/rookparcel-nox:latest
    container_name: rookparcel
    restart: unless-stopped
    user: "1000:1000"
    environment:
      ROOKPARCEL_WEBUI_PASS: ${ROOKPARCEL_WEBUI_PASS:-}
    command: [run, --state-dir, /config, --save, /srv/media/downloads, --webui, "0.0.0.0:8080", --port, "6881"]
    volumes:
      - rookparcel-config:/config
      - /srv/media:/srv/media
    ports:
      - "8080:8080"
      - "6881:6881/tcp"
      - "6881:6881/udp"
    stop_grace_period: 60s

volumes:
  rookparcel-config:
```

1. Ponga la contraseña en un archivo `.env` junto a `compose.yaml`: `ROOKPARCEL_WEBUI_PASS=a long password`. Sin él, lea la temporal con `docker compose logs rookparcel`.
2. Cambie `/srv/media` y `user:` por su carpeta de medios y su propietario.
3. Inícielo con `docker compose up -d`.

## Sonarr, Radarr y Prowlarr en contenedores

Configure Rookparcel en las aplicaciones como un cliente **qBittorrent**, como describe [Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr). En contenedores, tres cosas cambian:

1. **La misma ruta en cada contenedor.** Monte la carpeta de medios en la misma ruta en el contenedor de nox y en los de las aplicaciones (`/srv/media` en cada uno, como en el archivo Compose de arriba) y guarde las descargas dentro de ella. Las aplicaciones encuentran entonces los archivos donde nox los indica, y las importaciones pueden usar enlaces duros.
2. **El nombre de host.** En la misma red de Compose o en una red definida por el usuario, las aplicaciones llegan a nox como `http://rookparcel:8080` (el nombre del servicio o del contenedor). nox responde solo a direcciones IP y a `localhost` hasta que usted permita el nombre: añada `rookparcel` a **Dominios del servidor (separe varias entradas con punto y coma):** en **Preferencias > Interfaz web** de la interfaz web, en **Conectividad**. Hasta entonces el **Probar** de las aplicaciones falla con «Bad Request: unexpected Host header». Vea [Nombres de host y Docker](Sonarr-Radarr-y-Prowlarr#nombres-de-host-y-docker).
3. **La carpeta de descarga que piden las aplicaciones** debe ser una ruta dentro del contenedor de nox, como `/srv/media/downloads`.

`--webui-save-root /srv/media` (añadido a la orden `run`) limita las carpetas en las que pueden guardar la interfaz web y las aplicaciones.

## Registros, detención y recarga

nox escribe su registro en la salida del contenedor: `docker logs rookparcel` (`-f` para seguirlo). `--verbose` añade los eventos de trackers y pares.

nox guarda el estado de cada torrent cuando se detiene. `docker stop` envía `SIGTERM` y espera 10 segundos de forma predeterminada antes de matar el contenedor, lo que puede ser poco para una lista grande, así que dele un minuto:

```sh
docker stop -t 60 rookparcel
```

`--stop-timeout 60` en `docker run`, o `stop_grace_period: 60s` en Compose, lo convierte en el valor predeterminado del contenedor. Una parada limpia termina con estas líneas en el registro, y el contenedor sale con el código 0:

```text
* SIGTERM received (again to quit at once, without saving)
* shutting down
* stopped
```

`docker kill -s HUP rookparcel` recarga el filtro IP, como hace `SIGHUP` fuera de un contenedor ([Detener y recargar](Ejecutar-rookparcel-nox#detener-y-recargar)).

## Actualizar

La imagen no busca actualizaciones, y su interfaz web no muestra ningún enlace de actualización: un contenedor se actualiza descargando la nueva imagen e iniciando un contenedor nuevo sobre el mismo `/config`.

```sh
docker pull ghcr.io/balls0fsteel/rookparcel-nox
docker stop -t 60 rookparcel && docker rm rookparcel
docker run -d --name rookparcel ...    # la misma orden que antes
```

Con Compose: `docker compose pull`, y después `docker compose up -d`.

Por la misma razón, el [ayudante de búsqueda](Buscar-torrents#el-asistente-de-búsqueda) no se ofrece en la imagen. Su propio Prowlarr o Jackett aún puede añadirse como fuente de búsqueda ([Añadir su propio Prowlarr o Jackett](Buscar-torrents#añadir-su-propio-prowlarr-o-jackett)).

## Construir la imagen usted mismo

Desde una copia del repositorio:

```sh
docker build -t rookparcel-nox .
```

La compilación construye nox en un contenedor de Rust y lo copia en una pequeña imagen de Debian. Tarda unos minutos con todos los núcleos del equipo; `--build-arg CARGO_BUILD_JOBS=1` usa uno. Después use `rookparcel-nox` en lugar de `ghcr.io/balls0fsteel/rookparcel-nox` en las órdenes anteriores.

## Véase también

- [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox): todas las órdenes y opciones.
- [Interfaz web](Interfaz-web): la página, HTTPS y la seguridad.
- [Sonarr, Radarr y Prowlarr](Sonarr-Radarr-y-Prowlarr): los ajustes de las aplicaciones, las categorías y Remove Completed.
- [Conexión, puertos y cortafuegos](Conexión-puertos-y-cortafuegos): comprobar que los pares pueden conectarse.
- Los archivos de la imagen: [Dockerfile](https://github.com/Balls0fSteel/Rookparcel/blob/main/Dockerfile) y [contrib/docker](https://github.com/Balls0fSteel/Rookparcel/tree/main/contrib/docker).
