# Arrêt automatique

*Cette page est une traduction de la page anglaise [Auto Shutdown](Auto-shutdown). En cas de différence, la page anglaise fait foi.*

L'arrêt automatique ferme Rookparcel, ou met l'ordinateur en veille, en veille prolongée ou l'éteint, une fois vos torrents terminés. Cette page explique les choix possibles, ce qui compte comme « terminé », et le compte à rebours avant l'action.

## Choisir ce qui se passe

Ouvrez **Options > Arrêt automatique** et choisissez un élément :

| Élément | Ce qui se passe quand le travail est terminé |
|---|---|
| **Désactivé** | Rien. C'est la valeur par défaut. |
| **Quitter à la fin des téléchargements** | Rookparcel se ferme dès qu'aucun torrent démarré ne télécharge plus. |
| **Quitter quand tout est terminé** | Rookparcel se ferme dès qu'aucun torrent démarré ne télécharge ni ne partage plus. |
| **Veille à la fin des téléchargements** | L'ordinateur se met en veille une fois les téléchargements terminés. |
| **Veille quand tout est terminé** | Pareil, une fois que rien ne télécharge ni ne partage. |
| **Veille prolongée à la fin des téléchargements** | L'ordinateur se met en veille prolongée une fois les téléchargements terminés. |
| **Veille prolongée quand tout est terminé** | Pareil, une fois que rien ne télécharge ni ne partage. |
| **Éteindre à la fin des téléchargements** | L'ordinateur s'éteint une fois les téléchargements terminés. |
| **Éteindre quand tout est terminé** | Pareil, une fois que rien ne télécharge ni ne partage. |

![Le menu Options > Arrêt automatique](images/auto-shutdown.png)

L'élément choisi est marqué d'une coche à côté de lui. Veille et Veille prolongée sont grisés quand Windows ne les propose pas sur cet ordinateur (veille prolongée désactivée, par exemple).

Le choix ne dure que pour cette exécution : chaque démarrage de Rookparcel commence à **Désactivé**, et après avoir agi, il repasse à **Désactivé**.

## Ce qui compte comme terminé

**Fin des téléchargements** attend chaque torrent démarré qui n'a pas fini de télécharger : en cours, forcés ou en file d'attente, y compris un torrent en vérification, en train de récupérer la liste de fichiers d'un lien magnet, ou retenu par le planificateur. Un torrent dont tous les fichiers voulus sont terminés compte comme terminé, même s'il reste des fichiers ignorés.

**Tout est terminé** attend aussi chaque torrent démarré qui partage ou qui est en file d'attente pour partager. Un partage se termine quand son [objectif de partage](Mise-en-file-et-partage#objectif-de-partage) l'arrête. Un partage sans objectif en vigueur (la case de l'objectif dans Préférences > File d'attente est décochée au départ), ou un partage démarré de force, continue et retient l'arrêt : mettez donc un objectif en vigueur avant de choisir un élément « tout est terminé ».

Dans les deux cas :

- Un torrent arrêté ou en pause, ou un torrent en erreur, ne compte pas : il vous attend. Un torrent retenu parce que son lecteur ou son dossier est absent a aussi une erreur ([Disque et stockage](Disque-et-stockage#lecteur-ou-partage-absent)).
- Un torrent dont les fichiers sont en cours de déplacement compte comme du travail jusqu'à la fin du déplacement.
- L'arrêt automatique n'agit qu'après avoir vu du travail pendant que l'élément était choisi. Le choisir quand il ne reste rien à faire attend le prochain téléchargement au lieu d'agir tout de suite.

## Le compte à rebours

Quand le travail est terminé, une petite fenêtre **Arrêt automatique** apparaît au-dessus de toutes les fenêtres, même quand Rookparcel est caché dans la zone de notification. Elle indique ce qui s'est terminé et décompte à partir de 30 secondes, par exemple :

> Tous les téléchargements sont terminés.
> L'ordinateur s'éteindra dans 0:30.

- Cliquez sur **Annuler** (ou appuyez sur <kbd>Esc</kbd>) pour l'annuler. L'arrêt automatique est alors **Désactivé**.
- Quand les fenêtres sont verrouillées par un code PIN ([Verrouiller Rookparcel](Verrouiller-Rookparcel)), le compte à rebours s'affiche quand même, ce qui permet de l'annuler.
- Si de nouveaux travaux arrivent pendant le compte à rebours, comme un téléchargement RSS, le compte à rebours se ferme et attend que ce travail soit terminé lui aussi ; puis un compte à rebours complet recommence.
- Quand le temps est écoulé, il agit.

L'onglet **Journal** enregistre chaque étape : « Arrêt automatique : terminé, ... dans 30 s », « Arrêt automatique : annulé, maintenant désactivé », « Arrêt automatique : nouvelles tâches, le compte à rebours attend qu'elles soient terminées ».

## Comment chaque action est exécutée

- **Quitter** : Rookparcel se ferme comme avec **Fichier > Quitter**, en enregistrant au passage la liste des torrents et leur progression.
- **Éteindre** : Rookparcel se ferme d'abord de la même manière, puis demande à Windows d'arrêter et d'éteindre l'ordinateur. Windows n'est pas forcé : un programme contenant du travail non enregistré peut encore retenir l'arrêt.
- **Veille** et **Veille prolongée** : Rookparcel enregistre d'abord la liste des torrents et leur progression, puis met l'ordinateur en veille. Il continue de fonctionner et reprend quand l'ordinateur se réveille. L'arrêt automatique est alors **Désactivé**.

Si Windows refuse, un message l'indique, par exemple « Impossible d'éteindre l'ordinateur : » suivi de la raison. Un compte Windows qui n'a pas le droit d'éteindre l'ordinateur reçoit ce message.

## Dans la fenêtre GTK, sous macOS et dans rookparcel-nox

- **Fenêtre GTK** (Linux, BSD) : le même menu **Options > Arrêt automatique** et le même compte à rebours. Veille, Veille prolongée et Éteindre passent par le gestionnaire de session du système (systemd-logind), qui peut demander votre mot de passe. Là où il manque (les BSD, un système sans systemd) ou n'autorise pas une action, ces éléments sont grisés et il ne reste que les éléments Quitter.
- **macOS** : le même menu **Options > Arrêt automatique** et le même compte à rebours. Veille et Éteindre demandent à macOS de se mettre en veille ou de s'éteindre comme le fait le menu Pomme, avec des événements Apple envoyés à « loginwindow » du système. La première fois, macOS peut demander si Rookparcel a le droit de contrôler « loginwindow » ; cliquez sur **OK**. Si vous avez refusé, autorisez-le dans **Réglages Système > Confidentialité et sécurité > Automatisation**. Les éléments Veille prolongée sont grisés : macOS n'a pas de veille prolongée propre à demander. *L'application macOS native est nouvelle dans la version 1.2.19.*
- **Interface Web** : n'a pas d'arrêt automatique.
- **rookparcel-nox** : lancez-le avec `--exit-when-done downloads` ou `--exit-when-done all` pour qu'il se ferme, son état enregistré, quand plus rien ne télécharge, ou que plus rien ne télécharge ni ne partage. Il attend que quelque chose ait fonctionné d'abord, comme le fait le menu, et se ferme quelques secondes après la fin du travail. Il ne met jamais l'ordinateur en veille ni ne l'éteint. Voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

## Pages connexes

- [Mise en file et partage](Mise-en-file-et-partage)
- [Limites de vitesse et planificateur](Limites-de-vitesse-et-planificateur)
- [Zone de notification et démarrage](Zone-de-notification-et-démarrage)
