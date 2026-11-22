*Dies ist eine Übersetzung der englischen Seite [Dark mode](Dark-mode). Maßgeblich ist die englische Seite.*

# Dunkler Modus

Rookparcel folgt auf jeder Plattform der hellen oder dunklen Einstellung Ihres Systems; im Windows-Programm und in der Weboberfläche können Sie das Design außerdem selbst wählen.

## Windows

Standardmäßig folgt das Fenster der Windows-Einstellung **Einstellungen > Personalisierung > Farben > Modus auswählen** für Apps. Unter **Optionen > Einstellungen > Oberfläche > Design** gibt es drei Möglichkeiten:

- **System (hell oder dunkel)**: folgt Windows und wechselt mit, auch während das Programm läuft.
- **Hell** und **Dunkel**: legen die Farben fest, was Windows auch einstellt.

Alles folgt dem Design: das Fenster und seine Menüs, die Symbolleiste, die Torrent-Liste, die Kategorien, die Detail-Registerkarten, die Statusleiste und jeder Dialog, auch die Einstellungen. Die Symbole gibt es in einem hellen und einem dunklen Satz. Der **Hohe Kontrast** von Windows hat immer Vorrang: Ist er eingeschaltet, behält Rookparcel die Farben des Systems.

Der dunkle Modus braucht **Windows 10 Version 1809** oder neuer. Ein älteres Windows, oder eines ohne die Teile, die der dunkle Modus verwendet, zeigt unter dem Feld „Design“ einen Hinweis an und bleibt hell. Drei Dinge behalten die Farben des Systems: Meldungsfenster, der Dateiauswahldialog (er folgt dem eigenen Modus von Windows, nicht der Einstellung „Design“) und der Fortschrittsbalken des Update-Downloads, solange noch keine Größe bekannt ist.

## Linux (GTK)

Das GTK-Programm folgt seinem GTK-Design: einem dunklen Design wie Adwaita-dark oder Breeze-Dark, `gtk-application-prefer-dark-theme` oder `GTK_THEME=Adwaita:dark`. Die dunkle Vorgabe eines Desktops allein (der Stil *Dunkel* von GNOME oder KDE) macht einfache GTK-4-Fenster nicht dunkel; Rookparcel bleibt dann hell, bis auch das GTK-Design dunkel ist. Die selbst gezeichneten Teile (die Fortschrittsbalken, die Teileübersicht, das Geschwindigkeitsdiagramm, der Zeitplaner) und die Symbole wechseln mit dem Design.

## macOS

Die Mac-App folgt dem Erscheinungsbild des Systems.

## Weboberfläche

Die Seite folgt standardmäßig der Einstellung Ihres Browsers oder Systems und bietet eigene Designs an, auch Ihre eigenen CSS-Dateien: siehe [Weboberfläche](Weboberfläche).
