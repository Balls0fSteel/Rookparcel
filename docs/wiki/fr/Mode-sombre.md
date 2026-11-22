*Cette page est la traduction de la page anglaise [Dark mode](https://github.com/Balls0fSteel/Rookparcel/wiki/Dark-mode). En cas de différence, la page anglaise fait foi.*

# Mode sombre

Rookparcel suit le réglage clair ou sombre de votre système sur toutes les plateformes, et le programme Windows ainsi que l'interface web vous laissent choisir.

## Windows

Par défaut, la fenêtre suit le réglage **Paramètres > Personnalisation > Couleurs > Choisir votre mode** de Windows pour les applications. **Options > Préférences > Interface > Thème** propose trois choix :

- **Système (clair ou sombre)** : suit Windows, et change avec lui pendant que le programme fonctionne.
- **Clair** et **Sombre** : fixent les couleurs quel que soit le réglage de Windows.

Tout suit : la fenêtre et ses menus, la barre d'outils, la liste des torrents, les catégories, les onglets de détails, la barre d'état et chaque boîte de dialogue, Préférences comprises. Les icônes existent en un jeu clair et un jeu sombre. Le mode **Contraste élevé** de Windows l'emporte toujours : lorsqu'il est activé, Rookparcel garde les couleurs du système.

Le mode sombre demande **Windows 10 version 1809** ou plus récent. Un Windows plus ancien, ou sans les composants que le mode sombre utilise, affiche une note sous la liste Thème et reste clair. Trois éléments restent ceux du système : les boîtes de message, le sélecteur de fichiers (il suit le mode propre à Windows, pas le réglage Thème) et la barre de progression du téléchargement de la mise à jour tant qu'elle n'a pas encore de taille.

## Linux (GTK)

Le programme GTK suit son thème GTK : un thème sombre tel qu'Adwaita-dark ou Breeze-Dark, `gtk-application-prefer-dark-theme`, ou `GTK_THEME=Adwaita:dark`. La préférence sombre d'un bureau seule (le style *Sombre* de GNOME ou de KDE) n'assombrit pas les fenêtres GTK 4 simples : Rookparcel reste donc clair avec elle, jusqu'à ce que le thème GTK soit lui aussi sombre. Ses éléments dessinés par lui-même (les barres de progression, la carte des pièces, le graphique des vitesses, le planificateur) et ses icônes changent avec le thème.

## macOS

L'application Mac suit l'apparence du système.

## Interface web

La page suit par défaut le réglage de votre navigateur ou de votre système, et propose ses propres thèmes, y compris vos propres fichiers CSS : voir [Interface web](Interface-Web).
