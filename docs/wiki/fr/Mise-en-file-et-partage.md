# Mise en file et partage

*Cette page est une traduction de la page anglaise [Queueing and seeding](Queueing-and-seeding). En cas de différence, la page anglaise fait foi.*

Cette page explique comment Rookparcel décide quels torrents fonctionnent en même temps (la file d'attente), ce qui se passe quand un torrent terminé a assez partagé (l'objectif de partage), le partage initial des nouveaux torrents, et le déplacement des téléchargements terminés vers un autre dossier.

Les paramètres se trouvent dans **Options > Préférences > File d'attente**.

![Préférences, page File d'attente](images/prefs-queueing.png)

## Fonctionnement de la file

Un torrent que vous démarrez est confié à la file. La file fait fonctionner autant de torrents que ses limites le permettent et laisse les autres attendre :

| État | Signification |
|---|---|
| **Téléchargement**, **Partage** | En cours, dans une place de la file. |
| **En attente** | Attend une place de téléchargement. |
| **Partage en attente** | Terminé, attend une place de partage. |
| **[F] Téléchargement**, **[F] Partage** | Démarré avec **Forcer le démarrage**, en dehors de la file. |
| **En attente de vérification**, **Vérification** | Attend que son lecteur soit disponible pour être vérifié, puis est en cours de vérification. Voir [Disque et stockage](Disque-et-stockage#vérification). |
| **Arrêté**, **En pause**, **Terminé** | Sorti de la file par vous (ou, pour Terminé, par l'objectif de partage). |

- **Démarrer** confie les torrents sélectionnés à la file.
- **Forcer le démarrage** les lance tout de suite, en dehors de la file, même quand ses limites sont atteintes. Les torrents forcés n'atteignent jamais d'objectif de partage : ils partagent jusqu'à ce que vous les arrêtiez.
- **Arrêter** et **Pause** sortent les torrents de la file jusqu'à ce que vous les démarriez à nouveau.
- **Forcer la vérification** fonctionne aussi en dehors de la file : *nouveau dans la version 1.2.76.* Un torrent que vous revérifiez est vérifié immédiatement, même quand la file est pleine, puis retourne attendre son tour. Sur un torrent arrêté ou en pause, la commande démarre aussi le torrent, que vous pouvez arrêter de nouveau.
- **Monter dans la file**, **Descendre dans la file**, **Mettre en tête de file** et **Mettre en fin de file** dans le menu contextuel changent l'ordre. La colonne **#** indique la place de chaque torrent. Les torrents situés plus haut obtiennent une place en premier.

Voir [Gérer les torrents](Gérer-les-torrents) pour ces commandes et leurs touches. Un nouveau torrent peut être ajouté en tête de la file avec **Ajouter en tête de file** dans la fenêtre Ajouter un nouveau torrent ([Ajouter des torrents](Ajouter-des-torrents)).

## Paramètres de la file

Sous **Paramètres de la file** :

| Paramètre | Signification | Par défaut |
|---|---|---|
| **Nombre maximum de torrents actifs (envoi ou réception) : [-1 : pas de limite]** | Le plus grand nombre de torrents de la file qui fonctionnent en même temps, téléchargements et partages confondus. | 500 |
| **Nombre maximum de téléchargements actifs : [-1 : pas de limite]** | Le plus grand nombre de torrents de la file qui téléchargent en même temps. Le suivant démarre quand l'un se termine ou s'arrête. | 3 |
| **Nombre maximum de partages actifs : [-1 : pas de limite]** | Le plus grand nombre de torrents terminés de la file qui partagent en même temps. | 5 |
| **Ne pas compter les torrents lents dans ces limites** | Un torrent qui transfère moins d'environ 2 ko/s pendant une minute cesse d'occuper une place de téléchargement ou de partage, ce qui permet à un autre de démarrer. Il compte toujours dans le total. | Activé |

Pour laisser tout fonctionner en même temps, réglez les limites sur -1. Pour télécharger un torrent à la fois, réglez **Nombre maximum de téléchargements actifs** sur 1.

« Lent » se règle sur la page Avancé : `inactive_down_rate` et `inactive_up_rate` (octets par seconde, 2048 par défaut) et `auto_manage_startup` (secondes pendant lesquelles un torrent doit rester en dessous, 60 par défaut).

## Objectif de partage

L'objectif de partage décide quand un torrent terminé a assez partagé. Sous **Objectif de partage** :

| Paramètre | Signification | Par défaut |
|---|---|---|
| **Ratio minimum (%) : [0 : pas d'objectif de ratio]** | Envoyer ce pourcentage de ce que le torrent a téléchargé. 150 signifie une fois et demie. | 150 |
| **Durée de partage minimum (minutes) : [0 : pas d'objectif de durée]** | Partager pendant au moins ce nombre de minutes. | 0 |

Quand les deux sont définis, les deux doivent être atteints. Avec les deux à 0, il n'y a pas d'objectif.

Ces deux nombres ne font rien par eux-mêmes. Au départ, les torrents partagent jusqu'à ce que vous les arrêtiez, quel que soit leur ratio.

**Quand un torrent atteint l'objectif de partage (jamais les torrents forcés) :**

- **Limiter la vitesse d'envoi à (ko/s) : [0 : arrêter]** met l'objectif en vigueur. Il est désactivé au départ.
- Cochée avec 0, un torrent qui a atteint l'objectif est arrêté. Il quitte la file et s'affiche comme **Terminé**.
- Cochée avec une vitesse, le torrent continue de partager lentement : sa propre limite d'envoi est réglée sur cette vitesse. Une limite plus basse que le torrent a déjà est conservée.

L'onglet **Journal** écrit le nom du torrent et ce qui a été atteint, par exemple « objectif de partage atteint, arrêté (ratio 1.523, objectif 1.500) » ou « objectif de partage atteint, envoi limité à ... ».

Seuls les torrents de la file sont vérifiés : un torrent démarré de force, arrêté, en vérification ou encore en téléchargement est laissé tranquille.

Un objectif n'agit qu'une fois sur un torrent. Si vous redémarrez un torrent après que son objectif l'a arrêté, il partage ensuite jusqu'à ce que vous l'arrêtiez. Il n'est arrêté de nouveau qu'après que vous avez modifié l'objectif et que le torrent a atteint le nouveau.

Jusqu'à la version 1.2.44, l'objectif était toujours en vigueur et arrêtait chaque torrent à 150 %. Une mise à jour laisse désactivé un objectif que vous n'aviez jamais modifié. Un objectif que vous aviez modifié continue d'arrêter les torrents : la case est cochée pour vous, avec 0.

### Un objectif pour un seul torrent

1. Sélectionnez le torrent et choisissez **Propriétés** (<kbd>Enter</kbd>).
2. Sous **Objectif de partage**, cochez **Remplacer les paramètres par défaut**.
3. Réglez **Ratio minimum (%) : [0 : pas d'objectif de ratio]** et **Durée de partage minimum (minutes) : [0 : pas d'objectif de durée]**. Avec les deux à 0, ce torrent partage indéfiniment.
4. Cliquez sur **OK**.

L'objectif propre à un torrent est toujours en vigueur, même quand la case des Préférences est décochée : le torrent est arrêté quand il l'atteint (ou son envoi est limité, quand la case est cochée avec une vitesse).

![La boîte de dialogue Propriétés du torrent](images/properties.png)

## Partage initial

Le partage initial (super-seeding) vous aide à diffuser un torrent que vous venez de créer, quand vous êtes le seul à le partager : chaque pair reçoit des morceaux que personne d'autre n'a encore, si bien que l'essaim obtient plus vite une copie complète.

- Faites un clic droit sur un torrent en partage et choisissez **Avancé > Partage initial**. L'élément est grisé sauf si tous les torrents sélectionnés sont en partage ; choisissez-le de nouveau pour le désactiver.
- Ou cochez **Partage initial** sous **Autres paramètres** dans les Propriétés du torrent.

Tant qu'il est activé, l'état affiche **[S] Partage**. Voir [Créer des torrents](Créer-des-torrents) pour fabriquer un torrent à partager. Pour rookparcel-nox, `--super-seed` fait la même chose pour les torrents complets.

## Déplacer les téléchargements terminés

Rookparcel peut déplacer les fichiers d'un torrent vers un autre dossier quand il a fini de télécharger, par exemple d'un disque rapide vers un disque volumineux. Il continue de partager depuis le nouvel emplacement. Réglez cela dans **Options > Préférences > Dossiers**, sous **Emplacement des fichiers téléchargés** :

1. Cochez **Déplacer les téléchargements terminés vers :** et choisissez le dossier en dessous.
2. Cochez éventuellement **Ajouter l'étiquette du torrent** : un torrent qui a une étiquette va dans un dossier portant le nom de l'étiquette, à l'intérieur de ce dossier. L'étiquette qu'il a au moment où il se termine est prise en compte.
3. Laissez **Ne déplacer que depuis le dossier de téléchargement par défaut** coché pour ne déplacer que les torrents enregistrés dans le dossier des nouveaux téléchargements (**Placer les nouveaux téléchargements dans :**) lui-même ; les torrents que vous avez enregistrés ailleurs, même dans un dossier situé à l'intérieur, restent alors où ils sont. Décochez-le pour déplacer tous les torrents terminés.
4. Cliquez sur **OK**.

Cela s'applique aux torrents qui se terminent à partir de maintenant. Plus de détails sur les dossiers et le déplacement de fichiers : [Disque et stockage](Disque-et-stockage#déplacer-les-téléchargements-terminés).

## Arrêt automatique et partage

**Options > Arrêt automatique > ... Quand tout est terminé** attend que tous les partages s'arrêtent. Un partage sans objectif de partage, ou un partage forcé, continue et le retient. Voir [Arrêt automatique](Arrêt-automatique).

## Dans la fenêtre GTK et l'interface Web

- **Fenêtre GTK** : la même page File d'attente, les mêmes Propriétés et les mêmes menus.
- **Application macOS** : la même page File d'attente, les mêmes Propriétés et les mêmes menus ; les Préférences s'appliquent immédiatement, sans **OK**.
- **Interface Web** : la même page File d'attente. Sa case d'objectif de partage s'intitule **Arrêter le torrent ou limiter sa vitesse d'envoi**, avec la vitesse à côté ; 0 arrête le torrent. Le menu du torrent contient les mêmes éléments de file et de Partage initial.
- **rookparcel-nox** : les limites de la file et l'objectif de partage se règlent dans les Préférences de son interface Web. `--move-on-complete DIR` déplace dans un dossier chaque torrent qui se termine.

## Pages connexes

- [Gérer les torrents](Gérer-les-torrents)
- [Limites de vitesse et planificateur](Limites-de-vitesse-et-planificateur)
- [Disque et stockage](Disque-et-stockage)
- [Référence des préférences](Référence-des-préférences)
