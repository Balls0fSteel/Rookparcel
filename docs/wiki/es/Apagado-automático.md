*Esta página es una traducción de la página inglesa [Auto Shutdown](Auto-shutdown). Si hay diferencias, prevalece la página en inglés.*

# Apagado automático

El Apagado automático cierra Rookparcel, o suspende, hiberna o apaga el equipo, cuando sus torrents han terminado. Esta página explica las opciones, qué cuenta como «terminado» y la cuenta atrás antes de actuar.

## Elegir qué ocurre

Abra **Opciones > Apagado automático** y elija un elemento:

| Elemento | Qué ocurre cuando el trabajo ha terminado |
|---|---|
| **Desactivado** | Nada. Es el valor predeterminado. |
| **Salir al terminar las descargas** | Rookparcel se cierra cuando ya no descarga ningún torrent iniciado. |
| **Salir al terminar todo** | Rookparcel se cierra cuando ya no descarga ni siembra ningún torrent iniciado. |
| **Suspender al terminar las descargas** | El equipo se suspende (modo de suspensión) cuando las descargas han terminado. |
| **Suspender al terminar todo** | Lo mismo, cuando no se está descargando ni sembrando nada. |
| **Hibernar al terminar las descargas** | El equipo hiberna cuando las descargas han terminado. |
| **Hibernar al terminar todo** | Lo mismo, cuando no se está descargando ni sembrando nada. |
| **Apagar al terminar las descargas** | El equipo se apaga cuando las descargas han terminado. |
| **Apagar al terminar todo** | Lo mismo, cuando no se está descargando ni sembrando nada. |

![El menú Opciones > Apagado automático](images/auto-shutdown.png)

El elemento elegido tiene una marca al lado. Suspender o Hibernar aparecen en gris cuando Windows no los ofrece en este equipo (por ejemplo, con la hibernación desactivada).

La elección dura solo esta ejecución: cada inicio de Rookparcel empieza en **Desactivado**, y después de actuar vuelve a estar en **Desactivado**.

## Qué cuenta como terminado

**Al terminar las descargas** espera a cada torrent iniciado que no haya terminado de descargarse: los que están en ejecución, forzados o en cola, incluido uno que se está comprobando, que está obteniendo la lista de archivos de un enlace magnet o que el programador retiene. Un torrent cuyos archivos deseados están todos hechos cuenta como terminado, aunque queden archivos omitidos.

**Al terminar todo** espera además a cada torrent iniciado que está sembrando o en cola para sembrar. Una semilla termina cuando su [objetivo de siembra](Cola-y-siembra#objetivo-de-siembra) la detiene. Una semilla sin objetivo en vigor (la casilla del objetivo en Preferencias > Cola está desactivada al principio), o una iniciada con Forzar inicio, sigue sembrando y retrasa el apagado, así que ponga un objetivo en vigor antes de elegir un elemento «todo».

En ambos casos:

- Un torrent detenido o en pausa, o con un error, no cuenta: espera a que usted actúe. Un torrent retenido porque falta su unidad o su carpeta también tiene un error ([Disco y almacenamiento](Disco-y-almacenamiento#cuando-falta-una-unidad-o-un-recurso-compartido)).
- Un torrent cuyos archivos se están moviendo cuenta como trabajo hasta que termina el movimiento.
- El Apagado automático actúa solo después de haber visto trabajo mientras el elemento estaba elegido. Si lo elige cuando no queda nada por hacer, espera a la siguiente descarga en lugar de actuar de inmediato.

## La cuenta atrás

Cuando el trabajo ha terminado, aparece una pequeña ventana **Apagado automático** por encima de todas las ventanas, incluso cuando Rookparcel está oculto en la bandeja. Dice qué se completó y cuenta hacia atrás desde 30 segundos, por ejemplo:

> Todas las descargas han terminado.
> El equipo se apagará en 0:30.

- Haga clic en **Cancelar** (o pulse <kbd>Esc</kbd>) para anularlo. El Apagado automático queda entonces **Desactivado**.
- Con las ventanas bloqueadas por un PIN ([Bloquear Rookparcel](Bloquear-Rookparcel)), la cuenta atrás se muestra igualmente, de modo que puede cancelarla.
- Si llega trabajo nuevo durante la cuenta atrás, como una descarga por RSS, la cuenta atrás se cierra y espera a que también termine ese trabajo; después empieza de nuevo una cuenta atrás completa.
- Cuando se acaba el tiempo, actúa.

La pestaña **Registro** anota cada paso: «Apagado automático: hecho, ... en 30 s», «Apagado automático: cancelado, ahora desactivado», «Apagado automático: nuevas tareas, la cuenta atrás espera a que terminen».

## Cómo se lleva a cabo cada acción

- **Salir**: Rookparcel se cierra como con **Archivo > Salir**, guardando la lista de torrents y el progreso al salir.
- **Apagar**: Rookparcel se cierra primero del mismo modo y después pide a Windows que apague y corte la alimentación. Windows no es forzado: un programa con trabajo sin guardar aún puede retener el apagado.
- **Suspender** e **Hibernar**: Rookparcel guarda primero la lista de torrents y el progreso, y después pone el equipo en suspensión. Sigue ejecutándose y continúa cuando el equipo se reactiva. El Apagado automático queda entonces **Desactivado**.

Si Windows se niega, un mensaje lo dice, como «No se pudo apagar el equipo:» seguido del motivo. Una cuenta de Windows a la que no se permite apagar el equipo recibe este mensaje.

## En la ventana GTK, en macOS y en rookparcel-nox

- **Ventana GTK** (Linux, BSD): el mismo menú **Opciones > Apagado automático** y la misma cuenta atrás. Suspender, Hibernar y Apagar pasan por el gestor de inicio de sesión del sistema (systemd-logind), que puede pedirle su contraseña. Donde falta (los BSD, un sistema sin systemd) o no permite una acción, esos elementos están en gris y solo quedan los elementos Salir.
- **macOS**: el mismo menú **Opciones > Apagado automático** y la misma cuenta atrás. Suspender y Apagar piden a macOS que suspenda o apague, como hace el menú Apple, con eventos de Apple dirigidos al «loginwindow» del sistema. La primera vez, macOS puede preguntar si Rookparcel puede controlar «loginwindow»; haga clic en **OK**. Si lo rechazó, permítalo en **Ajustes del Sistema > Privacidad y seguridad > Automatización**. Los elementos Hibernar están en gris: macOS no tiene una hibernación propia que pedir. *La aplicación nativa de macOS es nueva en 1.2.19.*
- **Interfaz web**: no tiene Apagado automático.
- **rookparcel-nox**: inícielo con `--exit-when-done downloads` o `--exit-when-done all` para que se cierre, con su estado guardado, cuando no se esté descargando nada, o no se esté descargando ni sembrando nada. Espera hasta que algo se haya ejecutado primero, como hace el menú, y se cierra unos segundos después de terminar el trabajo. Nunca suspende ni apaga el equipo. Vea [Ejecutar rookparcel-nox](Ejecutar-rookparcel-nox).

## Páginas relacionadas

- [Cola y siembra](Cola-y-siembra)
- [Límites de velocidad y programador](Límites-de-velocidad-y-programador)
- [Bandeja, notificaciones e inicio](Bandeja-notificaciones-e-inicio)
