*Cette page est la traduction de la page anglaise [Tray, notifications and startup](https://github.com/Balls0fSteel/Rookparcel/wiki/Tray-notifications-and-startup). En cas de différence, la page anglaise fait foi.*

# Zone de notification, notifications et démarrage

Cette page traite de l'icône de Rookparcel dans la zone de notification, de la fermeture et de la réduction dans cette zone, de la notification qui signale la fin d'un téléchargement, du lancement de Rookparcel à l'ouverture de session, et de la façon d'en faire le programme qui ouvre les fichiers `.torrent` et les liens `magnet:`.

## L'icône de la zone de notification (Windows)

Tant que Rookparcel fonctionne, son icône se trouve dans la zone de notification de la barre des tâches. Pointer dessus affiche la version ainsi que les vitesses de réception et d'envoi actuelles.

- Un clic gauche (ou un double-clic) masque la fenêtre principale, ou la fait revenir.
- Un clic droit ouvre le menu de la zone de notification :

| Élément | Ce qu'il fait |
|---|---|
| **Masquer Rookparcel** / **Afficher Rookparcel** | Masque la fenêtre principale, ou l'affiche de nouveau |
| **Ajouter un torrent...** | Ouvre un fichier `.torrent`, comme **Fichier > Ajouter un torrent...** |
| **Ajouter un torrent depuis une URL...** | Ajoute un lien magnet ou une adresse web |
| **Démarrer tous les torrents** | Démarre tous les torrents de la liste |
| **Arrêter tous les torrents** | Arrête tous les torrents de la liste |
| **Verrouiller Rookparcel** | Masque toutes les fenêtres de Rookparcel jusqu'à la saisie du code PIN de verrouillage ; affiché dès qu'un code PIN est défini |
| **Quitter** | Quitte Rookparcel |

Pendant que Rookparcel est verrouillé, le menu n'a que **Afficher Rookparcel**, qui demande le code PIN, et **Quitter**. Voir [Verrouiller Rookparcel](Verrouiller-Rookparcel).

## Fermer et réduire dans la zone de notification

Deux paramètres décident de ce qu'il advient de la fenêtre :

| Paramètre | Par défaut | Ce qu'il fait |
|---|---|---|
| **Fermer dans la zone de notification** | Activé | Le bouton de fermeture de la fenêtre (et <kbd>Alt</kbd>+<kbd>F4</kbd>) masque Rookparcel dans la zone de notification au lieu de le quitter, de sorte que les transferts continuent. |
| **Réduire dans la zone de notification** | Désactivé | Réduire la fenêtre la masque dans la zone de notification au lieu de la barre des tâches. |

Réglez-les dans **Options > Préférences > Interface** sous **Zone de notification**, ou cochez-les directement dans le menu **Options** (**Fermer dans la zone de notification** et **Réduire dans la zone de notification**).

Pour quitter Rookparcel quand **Fermer dans la zone de notification** est activé, utilisez **Fichier > Quitter** ou **Quitter** dans le menu de la zone de notification. Rookparcel arrête alors ses torrents et enregistre leur progression. Si cela prend plus qu'un instant (par exemple tant que des fichiers sont encore en cours de déplacement vers un autre lecteur), une fenêtre indique ce qu'il attend, avec un bouton **Quitter maintenant**.

L'interface web que la fenêtre Windows sert ne fonctionne que tant que Rookparcel fonctionne : fermer dans la zone de notification la garde donc disponible. Voir [Interface web](Interface-Web).

## Notification à la fin d'un téléchargement

Lorsque **Afficher les bulles de notification** est activé (c'est le cas par défaut), Rookparcel affiche une notification « Téléchargement terminé » avec le nom du torrent chaque fois qu'un téléchargement se termine. Désactivez-la dans **Options > Préférences > Interface**. Tant que Rookparcel est [verrouillé](Verrouiller-Rookparcel), la notification n'apparaît pas.

Lorsqu'une mise à jour est disponible, Rookparcel le signale plutôt dans la barre d'état ; voir [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel).

## Afficher la vitesse dans la barre de titre

**Afficher la vitesse actuelle dans la barre de titre** (Préférences > Interface, désactivé par défaut) place les vitesses de réception et d'envoi au début du titre de la fenêtre, pour que vous puissiez les lire sur le bouton de la barre des tâches.

## Lancer Rookparcel à l'ouverture de session

1. Ouvrez **Options > Préférences > Général**.
2. Cochez **Lancer Rookparcel au démarrage du système**.
3. Pour qu'il démarre dans la zone de notification sans afficher sa fenêtre, cochez aussi **Démarrer réduit**.
4. Cliquez sur **OK**.

Sous Windows, il s'agit d'une entrée dans les programmes de démarrage de votre compte utilisateur. Aucun droit d'administrateur n'est nécessaire, et une copie portable peut aussi s'en servir. Si vous déplacez une copie portable vers un autre dossier, cochez de nouveau la case : elle n'est cochée que tant que l'entrée lance cette copie précise.

Avec les [profils](Profils), chaque profil a sa propre entrée de démarrage, ce qui permet d'en lancer plusieurs à l'ouverture de session. *Les entrées par profil sont nouvelles dans la 1.1.46.*

L'[Assistant de configuration](Premier-démarrage) propose le même choix sur sa dernière page.

## Ouvrir les fichiers .torrent et les liens magnet avec Rookparcel

1. Ouvrez **Options > Préférences > Général**.
2. Cochez **Associer aux fichiers .torrent** et **Associer aux liens magnet:**.
3. Cliquez sur **OK**.

Un double-clic sur un fichier `.torrent`, ou un clic sur un lien magnet dans votre navigateur, le confie alors à Rookparcel. Si Rookparcel fonctionne déjà, c'est la copie en cours qui le prend. Avec **Afficher une fenêtre avec les fichiers du torrent** activé, la fenêtre Ajouter un nouveau torrent s'ouvre ; avec **Activer la fenêtre du programme** activé, la fenêtre principale passe aussi au premier plan. Voir [Ajouter des torrents](Ajouter-des-torrents).

À savoir sous Windows :

- Les réglages ne valent que pour votre compte utilisateur et ne demandent aucun droit d'administrateur. Le programme d'installation peut aussi les définir.
- Si vous avez choisi un autre programme dans Windows lui-même (« Toujours utiliser cette application », ou Paramètres > Applications par défaut), Windows conserve ce choix, et vous seul pouvez le changer à cet endroit. Rookparcel indique alors quel programme Windows utilise encore, par exemple « Windows ouvre encore les fichiers .torrent avec ... ». Cliquez sur **Applications par défaut...** pour ouvrir le bon endroit dans les Paramètres de Windows et choisissez Rookparcel.
- Avec les profils, un seul profil peut ouvrir les fichiers `.torrent` et les liens magnet. Cocher les cases dans un profil les reprend à l'autre ; la page nomme le profil qui les détient actuellement.
- **Vérifier l'association au démarrage** (désactivé par défaut) fait demander à Rookparcel, à chaque démarrage, s'il doit reprendre les associations lorsqu'un autre programme les détient. Il ne le demande pas quand Rookparcel démarre réduit à l'ouverture de session.

## Version GTK (Linux, BSD)

**La zone de notification.** Rookparcel affiche une icône dans la zone de notification sur les bureaux qui en ont une : KDE Plasma, Xfce, LXQt, Cinnamon, MATE, waybar, et GNOME avec l'extension AppIndicator. Son menu a les mêmes éléments que sous Windows. Si la zone de notification disparaît (le tableau de bord redémarre, par exemple) pendant que Rookparcel y est masqué, Rookparcel affiche la fenêtre principale, ou la fenêtre du code PIN lorsqu'il est verrouillé, pour que rien ne reste sans moyen de retour. *Nouveauté de la 1.2.76.* Sur GNOME simple sans cette extension, il n'y a pas de zone de notification : les cases correspondantes n'apparaissent pas dans Préférences, et fermer la fenêtre quitte Rookparcel. <kbd>Ctrl</kbd>+<kbd>Q</kbd> quitte depuis la fenêtre.

**Réduire dans la zone de notification** demande que le bureau signale que la fenêtre a été réduite ; là où ce n'est pas le cas (certains bureaux Wayland), la réduction reste une réduction normale.

**Les notifications** passent par le service de notification propre au bureau, avec le même texte « Téléchargement terminé ».

**Lancement à l'ouverture de session.** **Lancer Rookparcel au démarrage du système** écrit une entrée de démarrage automatique dans le dossier `~/.config/autostart` de votre utilisateur. Dans un Flatpak, il le demande plutôt au bureau par l'autorisation d'arrière-plan, et le bureau peut vous demander de l'accepter ; dans ce cas, seul le profil par défaut peut démarrer à l'ouverture de session.

**Associations.** Les cases font de Rookparcel le programme par défaut des fichiers `.torrent` et des liens `magnet:` dans les paramètres de votre utilisateur. Décocher une case rend le type au programme par défaut du système ; cela ne rétablit pas un programme que vous utilisiez avant. Si vous déplacez une AppImage, cochez de nouveau les cases. Dans un Flatpak, les cases sont grisées : le bureau propose déjà Rookparcel pour les deux types, et vous choisissez le programme par défaut dans les paramètres de votre bureau.

## macOS

**La barre des menus.** Rookparcel place son icône dans la barre des menus, là où Windows a la zone de notification. Pointer dessus affiche les vitesses. Un clic masque la fenêtre ou la fait revenir ; un clic droit ou Contrôle-clic ouvre le menu : **Masquer Rookparcel** / **Afficher Rookparcel**, **Verrouiller Rookparcel** (dès qu'un code PIN de verrouillage est défini), **Ajouter un torrent...**, **Ajouter un torrent depuis une URL...**, **Démarrer tous les torrents**, **Arrêter tous les torrents** et **Quitter Rookparcel**. Le menu de l'icône du Dock a les mêmes éléments, sans **Quitter Rookparcel**, que le Dock ajoute lui-même. Tant que Rookparcel est [verrouillé](Verrouiller-Rookparcel), les menus ne proposent que **Afficher Rookparcel** (et **Quitter Rookparcel** dans celui de l'icône de la barre des menus).

**Fermer la fenêtre.** Avec **Fermer dans la zone de notification** activé (par défaut), le bouton de fermeture de la fenêtre (<kbd>⌘</kbd>+<kbd>W</kbd>) masque la fenêtre et Rookparcel continue de fonctionner. Cliquez sur l'icône du Dock ou sur celle de la barre des menus pour la faire revenir. Pour quitter, choisissez **Rookparcel > Quitter Rookparcel** (<kbd>⌘</kbd>+<kbd>Q</kbd>) ou **Quitter Rookparcel** dans le menu de l'icône de la barre des menus. **Réduire dans la zone de notification** masque de la même façon une fenêtre réduite.

**Les notifications** de téléchargements terminés sont des notifications macOS, avec le même texte « Téléchargement terminé ». macOS peut demander une fois si Rookparcel a le droit d'afficher des notifications ; si aucune n'apparaît, autorisez Rookparcel dans **Réglages Système > Notifications**.

**Lancement à l'ouverture de session.** **Lancer Rookparcel au démarrage du système** ajoute un élément d'ouverture pour ce profil (un agent de lancement dans `~/Library/LaunchAgents`) ; **Démarrer réduit** le lance masqué. Sous macOS 13 et ultérieur, **Réglages Système > Général > Éléments de connexion** le liste. Un élément d'ouverture créé par l'ancienne version GTK de l'application continue de fonctionner.

**Associations.** Les cases ne fonctionnent que lorsque Rookparcel s'exécute en tant que `Rookparcel.app`, et seulement dans le profil par défaut. On en désactive une dans **Lire les informations > Ouvrir avec** du Finder.

Les Préférences sous macOS n'ont pas de bouton **OK** : chaque changement s'applique aussitôt.

*L'application macOS native est nouvelle dans la 1.2.19.*

## Interface web et rookparcel-nox

L'interface web n'a ni zone de notification, ni notifications, ni réglages de démarrage : ses Préférences affichent les cases d'association et de démarrage automatique grisées, avec la note qu'elles relèvent du client de bureau. Pour lancer `rookparcel-nox` avec l'ordinateur, exécutez-le comme un service ; voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).
