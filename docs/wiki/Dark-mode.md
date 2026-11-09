# Dark mode

Rookparcel follows your system's light or dark setting on every platform, and the Windows program and the Web UI let you choose.

## Windows

By default the window follows Windows' **Settings > Personalization > Colors > Choose your mode** for apps. **Options > Preferences > UI Settings > Theme** has three choices:

- **System (light or dark)**: follow Windows, and change with it while the program runs.
- **Light** and **Dark**: fix the colours whatever Windows says.

Everything follows: the window and its menus, the toolbar, the torrent list, the categories, the detail tabs, the status bar and every dialog, Preferences included. The icons come in a light and a dark set. Windows **High Contrast** always wins: with it on, Rookparcel keeps the system's colours.

Dark needs **Windows 10 version 1809** or later. An older Windows, or one without the parts dark mode uses, shows a note under the Theme box and stays light. Two things stay the system's in dark: message boxes, and the progress bar of the update download while it has no size yet.

## Linux (GTK)

The GTK program follows its GTK theme: a dark theme such as Adwaita-dark or Breeze-Dark, `gtk-application-prefer-dark-theme`, or `GTK_THEME=Adwaita:dark`. A desktop's dark preference on its own (GNOME's or KDE's *Dark* style) does not darken plain GTK 4 windows, so Rookparcel stays light with it until the GTK theme is dark too. Its own drawn parts (the progress bars, the piece map, the speed graph, the scheduler) and icons switch with the theme.

## macOS

The Mac app follows the system appearance.

## Web UI

The page follows your browser's or system's setting by default and offers themes of its own, including your own CSS files: see [Web UI](Web-UI).
