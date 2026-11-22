*Ceci est la traduction française de la page [Adding torrents](Adding-torrents) du wiki anglais. La page anglaise fait foi.*

# Ajouter des torrents

Cette page montre toutes les façons de donner un torrent à Rookparcel : un fichier `.torrent`, un lien magnet, une adresse web, le glisser-déposer, votre navigateur et votre gestionnaire de fichiers, et un dossier surveillé. Elle explique aussi la fenêtre Ajouter un nouveau torrent, où vous choisissez le dossier et les fichiers avant le début du téléchargement.

## Les moyens rapides

| Ce que vous avez | Ce qu'il faut faire |
|---|---|
| Un fichier `.torrent` sur votre ordinateur | **Fichier > Ajouter un torrent...** (<kbd>Ctrl</kbd>+<kbd>O</kbd>), ou le premier bouton de la barre d'outils. Vous pouvez choisir plusieurs fichiers à la fois. |
| Un lien magnet | **Fichier > Ajouter un torrent depuis une URL...** (<kbd>Ctrl</kbd>+<kbd>U</kbd>) et collez-le. |
| L'adresse web d'un fichier `.torrent` (`http://` ou `https://`) | **Fichier > Ajouter un torrent depuis une URL...** et collez-la. Rookparcel télécharge le fichier et l'ajoute. |
| Un fichier `.torrent` dans l'Explorateur | Faites-le glisser sur la fenêtre de Rookparcel, ou double-cliquez dessus si Rookparcel ouvre les fichiers `.torrent`. |
| Un lien magnet dans votre navigateur | Cliquez dessus, si Rookparcel ouvre les liens magnet. |
| De nombreux fichiers `.torrent` qui arrivent au fil du temps | Laissez Rookparcel surveiller un dossier. Voir [Un dossier surveillé](#un-dossier-surveillé). |
| Un résultat de recherche | Voir [Rechercher des torrents](Rechercher-des-torrents). |

Le menu du clic droit sur l'icône de la zone de notification propose aussi **Ajouter un torrent...** et **Ajouter un torrent depuis une URL...**.

## Ajouter un torrent depuis une URL

![La fenêtre Ajouter un torrent depuis une URL](images/add-url.png)

1. Copiez le lien magnet ou l'adresse du fichier `.torrent`.
2. Choisissez **Fichier > Ajouter un torrent depuis une URL...**. Quand le presse-papiers contient un lien qui commence par `magnet:`, `http://` ou `https://`, la case est déjà remplie.
3. Cliquez sur **OK**.

Une adresse est téléchargée en arrière-plan, par votre proxy quand Préférences > Connexion y fait passer les connexions aux trackers. Certains sites répondent à l'adresse par un lien magnet au lieu d'un fichier ; Rookparcel le suit aussi. Si le téléchargement échoue, un message indique « Impossible de télécharger » avec l'adresse et la raison.

## Glisser-déposer

Déposez un ou plusieurs fichiers `.torrent` de l'Explorateur sur la fenêtre principale. Chacun est ajouté comme si vous l'aviez choisi avec **Ajouter un torrent...**.

## Depuis votre navigateur et votre gestionnaire de fichiers

Rookparcel peut être le programme que Windows utilise pour les fichiers `.torrent` et les liens `magnet:`. Activez cela dans **Options > Préférences > Général** avec **Associer aux fichiers .torrent** et **Associer aux liens magnet:**, ou dans l'[assistant de configuration](Premier-démarrage). Voir [Zone de notification et démarrage](Zone-de-notification-et-démarrage) pour les détails, y compris ce qu'il faut faire quand Windows garde un autre programme.

Quand Rookparcel tourne déjà et que vous ouvrez un torrent depuis votre navigateur ou l'Explorateur, c'est le Rookparcel en cours qui le prend. Avec **Activer la fenêtre du programme** cochée (Préférences > Interface, cochée par défaut), la fenêtre principale passe aussi au premier plan ; décochée, seule la fenêtre Ajouter un nouveau torrent apparaît.

Vous pouvez aussi lancer Rookparcel avec des torrents sur la ligne de commande : `rookparcel.exe file.torrent "magnet:?xt=..."`. Chaque chemin `.torrent`, lien `magnet:` et adresse `http(s)://` est ajouté. Si Rookparcel tourne déjà, ils vont à la fenêtre en cours.

Si le Rookparcel en cours a été lancé avec les droits d'administrateur, Windows ne laisse pas une seconde copie lui transmettre un torrent. Rookparcel vous le dit alors ; ajoutez le torrent depuis la fenêtre en cours, ou lancez Rookparcel sans les droits d'administrateur.

## La fenêtre Ajouter un nouveau torrent

![La fenêtre Ajouter un nouveau torrent](images/add-new-torrent.png)

Avec **Afficher une fenêtre avec les fichiers du torrent** cochée (Préférences > Interface, cochée par défaut), cette fenêtre s'ouvre pour chaque torrent que vous ajoutez. Décochée, les torrents sont ajoutés aussitôt avec le dossier par défaut et tous les fichiers.

Pour un lien magnet, la fenêtre s'ouvre quand la liste des fichiers du torrent est arrivée d'autres pairs. D'ici là, le torrent attend dans la liste sous la mention « Téléchargement des métadonnées ». Il n'écrit rien sur le disque pendant qu'il attend. Si vous quittez Rookparcel avant l'arrivée de la liste, la fenêtre revient au démarrage suivant.

### Enregistrer dans

Le dossier où vont les fichiers du torrent. La liste contient le dossier de téléchargement par défaut (celui de **Préférences > Dossiers > Placer les nouveaux téléchargements dans :**) et les dix derniers dossiers utilisés. Saisissez un chemin ou cliquez sur **...** pour en choisir un. Un dossier qui n'existe pas encore est créé. L'espace libre de ce disque est affiché après la taille, plus bas.

**Nom :** le nom du dossier du torrent, ou de son fichier unique. Le modifier renomme le dossier ou le fichier sur le disque.

### Options du torrent

| Option | Ce qu'elle fait |
|---|---|
| **Ignorer la vérification du hash** | Considère les fichiers déjà présents dans le dossier comme complets, sans les vérifier. À n'utiliser que si vous savez que les données sont complètes et correctes, par exemple après les avoir copiées d'un autre ordinateur. Grisée pour un lien magnet : ses fichiers ont déjà été vérifiés à l'arrivée de la liste des fichiers. |
| **Démarrer le torrent** | Cochée : le torrent entre dans la file et démarre comme la file le permet. Décochée : il est ajouté arrêté. Elle est cochée au départ, sauf si **Ne pas démarrer le téléchargement automatiquement** est activée dans Préférences > Interface. |
| **Ajouter en tête de file** | Place le torrent en premier dans la file, pour qu'il démarre avant les autres. |
| **Étiquette :** | Donne une étiquette au torrent. Choisissez-en une déjà utilisée ou saisissez-en une nouvelle. Voir [Gérer les torrents](Gérer-les-torrents#étiquettes). |
| **Téléchargement séquentiel** | Télécharge les pièces dans l'ordre, du début des données à leur fin, pour qu'une vidéo puisse être lue pendant le téléchargement. À réserver à ce que vous voulez prévisualiser : l'ordre habituel, du plus rare au plus courant, est meilleur pour les autres pairs. |
| **Télécharger d'abord les premières et dernières pièces** | Télécharge les premières et dernières pièces de chaque fichier voulu avant le reste, pour qu'un lecteur puisse lire tôt l'en-tête et l'index d'une vidéo. |

*Les deux cases d'ordre de téléchargement existent depuis la 1.2.63.* Toutes deux sont décochées au départ pour chaque torrent ; Rookparcel ne les mémorise pas. Pour les changer après l'ajout, utilisez **Avancé** dans le menu du clic droit ou dans les Propriétés. Voir [Gérer les torrents](Gérer-les-torrents#ordre-de-téléchargement).

### Contenu du torrent

Les premières lignes montrent le **Nom :**, le **Commentaire :**, la **Taille :** et la **Date :** du torrent. La taille indique quelle part du torrent vous avez sélectionnée, par exemple « 1.20 GB sur 3.40 GB sélectionnés », suivie de l'espace disque libre.

La liste des fichiers montre les dossiers et les fichiers du torrent sous forme d'arbre, chacun avec une case à cocher, sa taille et sa priorité.

- Décochez un fichier pour l'ignorer. Un dossier dont les fichiers sont cochés en partie affiche « Mixte ».
- **Tout sélectionner** et **Ne rien sélectionner** cochent ou décochent tout.
- Cliquez sur l'icône d'un dossier, ou double-cliquez dessus, pour l'ouvrir ou le fermer. <kbd>Right</kbd> et <kbd>Left</kbd> (ou <kbd>+</kbd> et <kbd>-</kbd> du pavé numérique) font de même ; <kbd>Space</kbd> coche ou décoche les lignes sélectionnées.
- Faites un clic droit sur des lignes sélectionnées pour **Priorité haute**, **Priorité normale**, **Priorité basse**, **Ne pas télécharger**, **Cocher** et **Décocher**. Choisir une priorité coche les fichiers ; **Ne pas télécharger** les décoche.

Les fichiers dont le nom correspond à **Ne pas télécharger les fichiers nommés :** (Préférences > Interface) sont décochés au départ ici. Cochez-les si vous les voulez finalement : ce que vous choisissez dans cette fenêtre l'emporte toujours. Voir [Fichiers exclus par leur nom](#fichiers-exclus-par-leur-nom).

Vous pouvez modifier plus tard les choix de fichiers et les priorités dans l'[onglet Fichiers](Détails-et-fichiers-des-torrents#fichiers).

### OK, Annuler et Ne plus afficher ceci

**OK** ajoute le torrent avec vos choix. **Annuler** n'ajoute rien ; pour un lien magnet, cela supprime le torrent en attente. Cocher **Ne plus afficher ceci** désactive **Afficher une fenêtre avec les fichiers du torrent**, si bien que les torrents suivants sont ajoutés sans cette fenêtre. Réactivez-la dans Préférences > Interface.

Rookparcel retient la taille de la fenêtre.

## Ajouter des torrents arrêtés

Pour ajouter des torrents sans rien télécharger tant que vous ne le dites pas, activez **Ne pas démarrer le téléchargement automatiquement** dans **Options > Préférences > Interface**. Les nouveaux torrents sont ajoutés arrêtés et hors de la file. Démarrez-les avec **Démarrer** quand vous êtes prêt. Dans la fenêtre Ajouter un nouveau torrent, cela se voit par **Démarrer le torrent** décochée.

## Fichiers exclus par leur nom

*Nouveau dans la 1.2.63.* Certains torrents contiennent des fichiers dont vous ne voulez jamais, comme des raccourcis, des caches de miniatures ou des vidéos d'échantillon. Rookparcel peut les exclure par leur nom :

1. Ouvrez **Options > Préférences > Interface**.
2. Sous **À l'ajout de torrents**, cochez **Ne pas télécharger les fichiers nommés :**.
3. Dans la case en dessous, saisissez les noms, séparés par des points-virgules. `*` remplace n'importe quels caractères et `?` un seul, et la casse n'a pas d'importance. Par exemple : `*.lnk; Thumbs.db; *sample*`.
4. Cliquez sur **OK**.

Seul le nom propre d'un fichier est comparé, pas les dossiers où il se trouve. La liste s'applique aux torrents que vous ajoutez à partir de maintenant :

- Dans la fenêtre Ajouter un nouveau torrent, les fichiers correspondants sont décochés au départ, et vous pouvez les recocher.
- Les torrents ajoutés sans la fenêtre voient leurs fichiers correspondants réglés sur **Ne pas télécharger** : avec la fenêtre désactivée, depuis le [dossier surveillé](#un-dossier-surveillé), depuis RSS, depuis un lien, depuis l'interface web, et depuis des applications comme Sonarr et Radarr. Pour un lien magnet, cela se produit une fois sa liste de fichiers arrivée.
- Un torrent dont tous les fichiers correspondent est ajouté avec tous ses fichiers, comme si la liste était vide.
- Les torrents déjà dans la liste, les torrents restaurés au démarrage et les torrents importés ne sont pas modifiés.

Un lien magnet ajouté sans la fenêtre, dont la liste de fichiers n'arrive qu'après un redémarrage de Rookparcel, télécharge tous les fichiers, et il en va de même d'un torrent confié à `rookparcel-nox` sur sa ligne de commande (`add`, ou `run` avec un fichier ou un lien). Pour obtenir plus tard un fichier ignoré, donnez-lui une priorité dans l'[onglet Fichiers](Détails-et-fichiers-des-torrents#fichiers).

## Un torrent déjà dans la liste

Si vous ajoutez un torrent qui est déjà dans la liste, Rookparcel dit « ... est déjà dans la liste des torrents. » et n'ajoute rien.

## Un dossier surveillé

Rookparcel peut surveiller un dossier et ajouter chaque nouveau fichier `.torrent` qu'on y dépose :

1. Ouvrez **Options > Préférences > Dossiers**.
2. Cochez **Charger automatiquement les .torrent depuis :** et choisissez le dossier.
3. Si vous le voulez, cochez **Effacer les .torrent chargés**.
4. Cliquez sur **OK**.

Rookparcel ajoute un nouveau fichier quelques secondes après qu'il a cessé de changer, sans la fenêtre Ajouter un nouveau torrent. Un fichier chargé est renommé en `.torrent.loaded`, ou supprimé quand **Effacer les .torrent chargés** est cochée. Les problèmes sont écrits dans l'[onglet Journal](Détails-et-fichiers-des-torrents#journal).

La même page peut conserver une copie de chaque fichier `.torrent` ajouté (**Stocker les .torrent dans :**). Voir [Disque et stockage](Disque-et-stockage) et la [Référence des préférences](Référence-des-préférences).

## Dans la version GTK (Linux, BSD)

- Les mêmes éléments de menu, les mêmes raccourcis et la même fenêtre Ajouter un nouveau torrent.
- **Ajouter un torrent depuis une URL** remplit de la même façon un lien venant du presse-papiers. Un texte qui n'est ni un lien `magnet:` ni une adresse `http(s)://` donne « Ce n'est pas un lien que Rookparcel peut ajouter ».
- La fenêtre GTK n'accepte pas les fichiers déposés. Utilisez **Fichier > Ajouter un torrent...**, ou ouvrez le fichier avec Rookparcel depuis votre gestionnaire de fichiers.
- Sous Linux et BSD, les associations de fichiers se trouvent sous **Intégration au bureau** dans Préférences > Général. Dans un Flatpak, choisissez plutôt le programme par défaut dans les paramètres de votre bureau. Voir [Zone de notification et démarrage](Zone-de-notification-et-démarrage).

## Dans l'interface web

- **Ajouter un torrent** envoie un ou plusieurs fichiers `.torrent` depuis l'ordinateur sur lequel tourne le navigateur ; **Ajouter une URL** demande un lien magnet ou une adresse. Un simple info-hash est pris lui aussi pour un lien magnet.
- Vous pouvez déposer des fichiers `.torrent` ou un lien magnet sur la page, ou coller un lien magnet n'importe où en dehors d'une zone de texte.
- La même fenêtre Ajouter un nouveau torrent s'ouvre dans la page. **Enregistrer dans** est un dossier du serveur, saisi sous forme de chemin.
- Sur un téléphone, **Ajouter une URL** se trouve dans le menu **Plus** (⋯).

Voir [Interface web](Interface-Web). Dans une fenêtre connectée avec **Fichier > Se connecter à distance...**, le bouton **...** d'Ajouter un nouveau torrent est grisé : saisissez un dossier du serveur. Voir [Contrôle à distance](Contrôle-à-distance).

## Pages connexes

- [La fenêtre principale](La-fenêtre-principale)
- [Gérer les torrents](Gérer-les-torrents)
- [Importer depuis µTorrent et qBittorrent](Importer-depuis-uTorrent-et-qBittorrent)
- [Téléchargeur RSS](Téléchargeur-RSS)
