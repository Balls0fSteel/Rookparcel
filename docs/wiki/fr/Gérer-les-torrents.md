*Cette page est la traduction de la page anglaise [Managing torrents](Managing-torrents). En cas de divergence, la page anglaise fait foi.*

# Gérer les torrents

Cette page explique ce que vous faites des torrents une fois qu'ils sont dans la liste : les démarrer et les arrêter, la file d'attente, les supprimer avec ou sans leurs fichiers, les vérifier à nouveau, déplacer leurs fichiers, les étiquettes et les tags, les limites de vitesse propres à un torrent, et le menu contextuel qui réunit tout cela.

Chaque action s'applique aux torrents sélectionnés. Sélectionnez-en plusieurs avec <kbd>Ctrl</kbd>+clic ou <kbd>Shift</kbd>+clic, ou tous avec <kbd>Ctrl</kbd>+<kbd>A</kbd>, et agissez sur eux en une seule fois.

## Le menu contextuel

![Le menu contextuel de la liste des torrents](images/torrent-menu.png)

Faites un clic droit sur un torrent (ou appuyez sur <kbd>Shift</kbd>+<kbd>F10</kbd>) pour afficher ce menu :

| Entrée | Action | Touche |
|---|---|---|
| **Ouvrir le dossier** | ouvre le dossier du torrent dans l'Explorateur, avec ses données sélectionnées | |
| **Copier le lien magnet** | place le lien magnet du torrent dans le presse-papiers | |
| **Exporter le .torrent...** | enregistre le fichier `.torrent` du torrent | |
| **Forcer le démarrage** | démarre tout de suite, en dehors de la file d'attente | |
| **Démarrer** | confie le torrent à la file d'attente | <kbd>Ctrl</kbd>+<kbd>S</kbd> |
| **Pause** | le suspend et le retire de la file d'attente ; affiche **En pause** | <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd> |
| **Arrêter** | l'arrête et le retire de la file d'attente ; affiche **Arrêté** ou **Terminé** | <kbd>Ctrl</kbd>+<kbd>T</kbd> |
| **Monter dans la file**, **Descendre dans la file** | d'une place vers le haut ou vers le bas | |
| **Mettre en tête de file**, **Mettre en fin de file** | tout en haut ou tout en bas, en conservant leur ordre | |
| **Supprimer** | le retire de la liste ; les fichiers restent | <kbd>Del</kbd> |
| **Supprimer et > Effacer les données** | le retire et efface ses fichiers | <kbd>Shift</kbd>+<kbd>Del</kbd> |
| **Forcer la vérification** | vérifie à nouveau les fichiers sur le disque | |
| **Mettre à jour le tracker** | contacte les trackers maintenant | |
| **Avancé > Définir l'emplacement de téléchargement...** | déplace ses fichiers dans un autre dossier | |
| **Avancé > Partage initial** | active ou désactive le partage initial | |
| **Avancé > Téléchargement séquentiel** | télécharge les pièces dans l'ordre, du début à la fin | |
| **Avancé > Télécharger d'abord les premières et dernières pièces** | télécharge d'abord les deux extrémités de chaque fichier voulu | |
| **Allocation de bande passante > Définir la limite de réception / Définir la limite d'envoi** | ses propres limites de vitesse | |
| **Étiquette** | lui donne une étiquette, une nouvelle, ou aucune | |
| **Tags** | lui donne des tags ou les retire | |
| **Propriétés** | trackers, limites, objectif de partage et plus | <kbd>Enter</kbd> |

Les sections ci-dessous expliquent chaque groupe. La plupart de ces actions figurent aussi dans la barre d'outils ; voir [La fenêtre principale](La-fenêtre-principale#la-barre-doutils).

## Démarrer, Pause, Arrêter et Forcer le démarrage

Rookparcel tient une file d'attente, comme µTorrent. Le nombre de torrents qui peuvent s'exécuter en même temps se règle dans **Options > Préférences > File d'attente** ; voir [Mise en file et partage](Mise-en-file-et-partage).

- **Démarrer** confie le torrent à la file d'attente. S'il y a de la place, il s'exécute (**Téléchargement** ou **Partage**). Sinon, il attend, avec l'état **En attente** ou **Partage en attente**, jusqu'à ce qu'une place se libère.
- **Forcer le démarrage** exécute le torrent tout de suite, même si la file d'attente est pleine. Son état commence par **[F]**, par exemple **[F] Partage**. Un torrent forcé n'atteint jamais d'objectif de partage : il partage jusqu'à ce que vous l'arrêtiez.
- **Pause** et **Arrêter** suspendent tous deux le torrent et le retirent de la file d'attente. La seule différence est le mot affiché dans la colonne État : **En pause**, ou **Arrêté** (**Terminé** pour un torrent complet). Les deux sont conservés après un redémarrage. Utilisez Démarrer ou Forcer le démarrage pour le reprendre.

Le menu de l'icône de la zone de notification propose **Démarrer tous les torrents** et **Arrêter tous les torrents**, qui agissent sur tous les torrents de la liste.

Pour ajouter de nouveaux torrents à l'arrêt, voir [Ajouter des torrents](Ajouter-des-torrents#ajouter-des-torrents-arrêtés).

## La file d'attente

La colonne **#** indique la place de chaque torrent dans la file d'attente. Les nombres les plus bas passent en premier. Utilisez **Monter dans la file** et **Descendre dans la file** (aussi dans la barre d'outils), ou **Mettre en tête de file** et **Mettre en fin de file**, pour changer l'ordre. Quand vous déplacez plusieurs torrents, leur ordre relatif est conservé.

Dans la fenêtre Ajouter un nouveau torrent, **Ajouter en tête de file** place un nouveau torrent en premier.

## Supprimer des torrents

- **Supprimer** (<kbd>Del</kbd>) retire les torrents de la liste. Les données téléchargées restent sur le disque.
- **Supprimer et > Effacer les données** (<kbd>Shift</kbd>+<kbd>Del</kbd>) les retire et efface leurs fichiers téléchargés. Sous Windows, les fichiers ne passent pas par la Corbeille et ne peuvent pas y être récupérés. Un fichier partiel dont le nom se termine par `.!ut` ou `.!qB` et dont Rookparcel ne peut pas montrer qu'il contient les données de ce torrent est conservé ; voir [Disque et stockage](Disque-et-stockage#fichiers-incomplets-et-ut).

Si **Confirmer la suppression des torrents** est activé (Préférences > Interface, activé par défaut), Rookparcel demande d'abord confirmation et précise si les fichiers sont aussi supprimés. Désactivée, la commande Supprimer agit immédiatement.

## Forcer la vérification

**Forcer la vérification** lit les fichiers du torrent sur le disque pièce par pièce et ne compte que les pièces complètes et correctes. L'état affiche **En attente de vérification** tant que la vérification attend son lecteur (les vérifications s'exécutent à raison d'un torrent par lecteur ; *nouveau dans la 1.2.76*), puis **Vérification** avec un pourcentage. Utilisez-la après avoir déplacé ou restauré des fichiers à la main, ou si vous soupçonnez des données endommagées.

*Nouveau dans la 1.2.76 :* un torrent arrêté démarre avec la vérification ; arrêtez-le de nouveau ensuite si vous voulez qu'il reste arrêté. Un torrent en attente est vérifié tout de suite, sans attendre une place dans la file, puis remis en attente. Un torrent suspendu avec l'état **fichiers manquants** (voir [Disque et stockage](Disque-et-stockage#lecteur-ou-partage-absent)) est vérifié par rapport à ce qui est présent maintenant, et tout ce qui manque est téléchargé à nouveau.

La douceur avec laquelle les vérifications sollicitent le disque se règle dans **Préférences > Avancé > E/S disque** ; voir [Disque et stockage](Disque-et-stockage).

## Mettre à jour le tracker

**Mettre à jour le tracker** contacte les trackers du torrent maintenant, sans attendre la prochaine annonce, pour signaler la progression et obtenir plus de pairs. L'[onglet Trackers](Détails-et-fichiers-des-torrents#trackers) montre la réponse.

## Déplacer les fichiers : Définir l'emplacement de téléchargement

1. Sélectionnez les torrents.
2. Choisissez **Avancé > Définir l'emplacement de téléchargement...**.
3. Choisissez le nouveau dossier.

Rookparcel y déplace les fichiers et continue de partager ou de télécharger depuis le nouvel emplacement. L'état affiche **Déplacement** pendant ce temps, et l'entrée est grisée pour un torrent déjà en cours de déplacement. Les fichiers de même nom déjà présents dans le nouveau dossier sont remplacés.

Si vous quittez pendant un déplacement, Rookparcel attend la fin des déplacements et en affiche la progression. **Quitter maintenant** interrompt un déplacement : ses fichiers restent dans l'ancien dossier, et ce qui avait déjà été copié est effacé. Déplacez-le de nouveau plus tard.

Pour déplacer automatiquement les téléchargements terminés, voir **Déplacer les téléchargements terminés vers :** sur la page [Disque et stockage](Disque-et-stockage).

## Ouvrir le dossier

**Ouvrir le dossier** ouvre le dossier du torrent dans l'Explorateur, avec son fichier ou son dossier sélectionné une fois qu'il existe. Quand plusieurs torrents sont sélectionnés, il ouvre celui du premier. Un double-clic sur un torrent terminé fait la même chose.

Pour ouvrir un seul fichier, utilisez l'[onglet Fichiers](Détails-et-fichiers-des-torrents#fichiers).

## Copier le lien magnet et exporter le .torrent

*Nouveau dans la 1.2.63.*

**Copier le lien magnet** place dans le presse-papiers les liens magnet des torrents sélectionnés, un par ligne. Chaque lien contient le hash d'information du torrent (info-hash), son nom et ses trackers. Collez-le dans un autre programme de torrents, ou envoyez-le à quelqu'un, pour partager le torrent sans fichier.

Comme le lien contient les trackers, ne partagez pas un lien copié depuis le torrent d'un tracker privé. L'adresse d'un tracker privé contient généralement votre clé personnelle (un passkey), et toute personne qui a le lien pourrait s'annoncer à votre place.

**Exporter le .torrent...** enregistre les fichiers `.torrent` des torrents sélectionnés. Le fichier contient le torrent tel que Rookparcel l'a reçu, avec les trackers et les sources web qu'il utilise maintenant. Vous pouvez l'ajouter sur un autre ordinateur, ou le garder comme copie.

- Avec un seul torrent sélectionné, Rookparcel demande un nom de fichier.
- Avec plusieurs, il demande un dossier et enregistre chacun sous le nom `<nom>.torrent`. Quand un nom est déjà pris, il ajoute un numéro, par exemple ` (2)`, au lieu de remplacer le fichier.
- Un torrent ajouté à partir d'un lien magnet n'a pas de fichier `.torrent` tant que sa liste de fichiers n'est pas arrivée (« Téléchargement des métadonnées »). L'entrée est grisée quand aucun des torrents sélectionnés n'en a un.

Si le fichier que vous avez nommé ne peut pas être écrit, Rookparcel affiche « Impossible d'exporter » avec le nom du torrent et la raison. Les fichiers qui échouent lors d'une exportation vers un dossier sont signalés dans l'[onglet Journal](Détails-et-fichiers-des-torrents#journal).

## Étiquettes

Une étiquette est un nom que vous donnez à des torrents pour les regrouper, comme « Films » ou « Linux ». Chaque étiquette utilisée apparaît sous **Étiquettes** dans la liste des catégories, avec un compteur, de sorte qu'un clic n'affiche que ces torrents.

- **Étiquette > Nouvelle étiquette...** demande un nom et le donne aux torrents sélectionnés.
- **Étiquette >** *une étiquette* donne une étiquette existante.
- **Étiquette > Retirer l'étiquette** retire l'étiquette.

Le menu coche l'étiquette que les torrents sélectionnés ont en commun. Vous pouvez aussi donner une étiquette dans la fenêtre Ajouter un nouveau torrent, et les filtres RSS peuvent en donner une.

Quand **Ajouter l'étiquette du torrent** est activé à côté de **Déplacer les téléchargements terminés vers :** (Préférences > Dossiers), un torrent terminé qui a une étiquette est déplacé dans un dossier portant le nom de son étiquette. Voir [Disque et stockage](Disque-et-stockage).

## Tags

*Nouveau dans la 1.2.63.*

Un torrent a une seule étiquette, mais il peut avoir en plus un nombre quelconque de tags, comme « 4K », « garder » et « ISO Linux ». Les tags fonctionnent comme ceux de qBittorrent. Chaque tag apparaît sous **Tags** dans la liste des catégories, sous les étiquettes, avec un compteur ; un torrent qui a plusieurs tags est compté sous chacun. **Sans tag** affiche les torrents qui n'ont aucun tag.

- **Tags > Nouveau tag...** demande un nom et donne le nouveau tag aux torrents sélectionnés.
- **Tags >** *un tag* donne ce tag aux torrents sélectionnés. Un tag que tous les torrents sélectionnés ont est coché ; le choisir le retire à tous. Un tag que seuls certains d'entre eux ont n'est pas coché (l'application macOS affiche une marque mixte) ; le choisir le donne à tous.
- **Tags > Retirer tous les tags** retire tous les tags des torrents sélectionnés. Les tags eux-mêmes restent.

Un tag reste dans la liste des catégories quand plus aucun torrent ne l'a, avec un compteur à 0. Pour vous en débarrasser, faites un clic droit dessus dans la liste des catégories et choisissez **Supprimer le tag** : Rookparcel demande « Supprimer le tag « *nom* » ? Tous les torrents qui l'ont le perdront. » Les torrents eux-mêmes restent. **Nouveau tag...** dans ce menu crée un tag sans le donner à aucun torrent.

Un nom ne peut pas être vide ni contenir de virgule, de tabulation ou de saut de ligne. Les espaces aux deux extrémités sont supprimées, et un nom de plus de 128 octets est tronqué. Les noms distinguent les majuscules des minuscules : `tv` et `TV` sont deux tags. Il peut y avoir au plus 1024 tags.

La colonne **Tags** affiche les tags d'un torrent ; elle est masquée au départ (voir [La fenêtre principale](La-fenêtre-principale#colonnes)). La zone de recherche ne cherche que dans les noms des torrents, pas dans les tags ; cliquez plutôt sur un tag dans la liste des catégories. La fenêtre Ajouter un nouveau torrent n'a pas de tags : donnez-les depuis le menu contextuel une fois le torrent dans la liste. Les applications qui utilisent l'API de qBittorrent, comme Sonarr, peuvent aussi donner des tags ([Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr#catégories-étiquettes-et-tags)).

## Limites de vitesse pour un torrent

**Allocation de bande passante > Définir la limite de réception** et **Définir la limite d'envoi** proposent **Illimité**, 10, 25, 50, 100, 250, 500, 1000, 2500 et 5000 ko/s. Le choix actuel est coché. Pour tout autre nombre, utilisez les [Propriétés](Détails-et-fichiers-des-torrents#propriétés-du-torrent). Les limites globales des Préférences s'appliquent toujours en plus. Voir [Limites de vitesse et planificateur](Limites-de-vitesse-et-planificateur).

## Partage initial

**Avancé > Partage initial** sert pour un torrent que vous êtes le premier à partager. Chaque pair reçoit des pièces que personne d'autre n'a encore, de sorte qu'un nouveau torrent se répand plus vite. Il ne peut être activé que si tous les torrents sélectionnés sont en partage. L'état affiche **[S] Partage** tant qu'il est activé. Choisissez-le de nouveau pour le désactiver.

## Ordre de téléchargement

*Nouveau dans la 1.2.63.*

Par défaut, Rookparcel télécharge d'abord les pièces les plus rares parmi les pairs. Cela maintient chaque pièce disponible dans l'essaim et convient le mieux à tous ceux qui partagent le torrent. Deux entrées sous **Avancé** changent l'ordre, surtout pour que vous puissiez regarder une vidéo pendant son téléchargement. Chacune est cochée quand elle est activée ; choisissez-la de nouveau pour la désactiver.

- **Téléchargement séquentiel** télécharge les pièces dans l'ordre, du début des données à leur fin, de sorte qu'un lecteur peut lire la vidéo au fur et à mesure de son arrivée. Les fichiers de priorité plus élevée passent toujours en premier. Cela a un coût pour l'essaim : les pièces rares que vous auriez pu transmettre arrivent tard. Utilisez-le pour un torrent que vous voulez prévisualiser, et laissez-le désactivé sinon.
- **Télécharger d'abord les premières et dernières pièces** télécharge les premières et les dernières pièces de chaque fichier voulu avant le reste : 1 % du fichier à chaque extrémité, au moins une pièce. Beaucoup de formats vidéo y conservent leur en-tête et leur index, de sorte qu'un lecteur peut ouvrir le fichier inachevé. Les fichiers ignorés ne sont pas touchés.

Les deux sont grisés quand tous les torrents sélectionnés sont terminés. Chaque torrent conserve son choix après un redémarrage. Vous pouvez aussi les régler à l'ajout d'un torrent, dans la [fenêtre Ajouter un nouveau torrent](Ajouter-des-torrents#options-du-torrent), et dans les [Propriétés](Détails-et-fichiers-des-torrents#propriétés-du-torrent).

## Propriétés

**Propriétés** (<kbd>Enter</kbd>, ou double-clic sur un torrent qui n'est pas terminé) ouvre la fenêtre Propriétés du torrent : trackers, limites de vitesse, emplacements d'envoi, objectif de partage, DHT et échange de pairs, et ordre de téléchargement de ce torrent. Voir [Détails et fichiers des torrents](Détails-et-fichiers-des-torrents#propriétés-du-torrent).

## Dans la version GTK (Linux, BSD)

- Le même menu contextuel, dans le même ordre, sauf que **Copier le lien magnet** et **Exporter le .torrent...** ont leur propre groupe juste avant **Avancé**. L'application macOS les place là aussi.
- Raccourcis : <kbd>Del</kbd> supprime, <kbd>Shift</kbd>+<kbd>Del</kbd> supprime et efface les fichiers, <kbd>Ctrl</kbd>+<kbd>A</kbd> sélectionne tout. Démarrer, Pause et Arrêter n'ont pas de raccourci.
- **Ouvrir le dossier** ouvre le dossier dans votre gestionnaire de fichiers.
- Sous Linux et BSD, les fichiers supprimés sont effacés du disque, pas déplacés dans la corbeille.

## Dans l'interface web

- Le même menu contextuel, sauf que **Ouvrir le dossier** est grisé (« pas depuis un navigateur »), et que **Copier le lien magnet** et **Exporter le .torrent...** se trouvent juste avant **Avancé**. **Tags** liste d'abord **Nouveau tag...** et **Retirer tous les tags**, puis les tags.
- **Exporter le .torrent...** télécharge le fichier par le navigateur, dans son dossier de téléchargement. Avec plusieurs torrents, le navigateur télécharge chaque fichier à tour de rôle, 50 au plus par clic.
- **Copier le lien magnet** a besoin de l'accès du navigateur au presse-papiers. Une page ouverte en `http://` simple depuis un autre ordinateur n'en a pas ; la page affiche donc les liens, sous « Copiez le lien ci-dessous : », pour que vous les copiiez à la main.
- **Définir l'emplacement de téléchargement** demande un dossier sur le serveur, saisi sous forme de chemin.
- Les fichiers supprimés sont effacés du disque, pas déplacés dans la corbeille.
- Sur un téléphone, le menu omet **Ouvrir le dossier** et commence par **Détails**.

## Dans une fenêtre distante

Dans une fenêtre connectée avec **Fichier > Se connecter à distance...**, les fichiers se trouvent sur l'autre ordinateur. **Ouvrir le dossier** est grisé, et **Définir l'emplacement de téléchargement...** demande un dossier sur le serveur. Voir [Contrôle à distance](Contrôle-à-distance).

## Pages liées

- [La fenêtre principale](La-fenêtre-principale)
- [Détails et fichiers des torrents](Détails-et-fichiers-des-torrents)
- [Mise en file et partage](Mise-en-file-et-partage)
- [Raccourcis clavier](Raccourcis-clavier)
