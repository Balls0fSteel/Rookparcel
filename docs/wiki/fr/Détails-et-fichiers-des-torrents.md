*Cette page est la traduction de la page anglaise [Torrent details and files](Torrent-details-and-files). En cas de divergence, la page anglaise fait foi.*

# Détails et fichiers des torrents

Cette page explique les onglets de détails situés sous la liste des torrents (Général, Trackers, Pairs, Pièces, Fichiers, Vitesse et Journal), comment choisir les fichiers à télécharger et dans quel ordre, et la fenêtre Propriétés du torrent avec les trackers, les limites et l'objectif de partage d'un torrent.

Sélectionnez un torrent dans la liste pour remplir les onglets. **Options > Afficher les détails** les affiche ou les masque ; faites glisser la bordure au-dessus d'eux pour modifier leur hauteur. Rookparcel se souvient de l'onglet ouvert.

## Général

![L'onglet Général](images/tab-general.png)

La vue d'ensemble d'un torrent.

- **Reçu :** une barre avec une bande par pièce que vous avez, et le pourcentage accompli.
- **Disponibilité :** une barre indiquant quelles pièces les pairs connectés possèdent, et combien de copies complètes ils détiennent à eux tous. En dessous de 1, aucun pair connecté n'a toutes les pièces.

**Transfert**

| Champ | Signification |
|---|---|
| **Temps écoulé :** | depuis combien de temps le torrent est en cours d'exécution |
| **Restant :** | temps restant estimé |
| **Gaspillé :** | données jetées : pièces qui ont échoué à leur vérification, et doublons |
| **Reçu :**, **Envoyé :** | totaux pour ce torrent |
| **Sources :**, **Pairs :** | « 3 sur 120 connectés » : connectés, et le total indiqué par le tracker |
| **Vitesse de réception :**, **Vitesse d'envoi :** | vitesses actuelles |
| **Limite de réception :**, **Limite d'envoi :** | les limites propres au torrent ; ∞ s'il n'y en a pas |
| **Ratio de partage :** | envoyé divisé par reçu |
| **État :** | le même texte que la colonne État |

**Général**

| Champ | Signification |
|---|---|
| **Enregistré sous :** | le dossier où se trouvent les fichiers |
| **Restant :** | ce qu'il reste à télécharger |
| **Taille totale :** | le torrent entier, fichiers ignorés compris |
| **Ajouté le :**, **Terminé le :** | dates |
| **Pièces :** | « 1200 x 1.00 MB (have 800) » : le nombre de pièces, leur taille, et combien vous en avez |
| **Hash :** | le hash d'information (info-hash) du torrent |
| **Votre adresse vue par les trackers :** | votre adresse publique, pour que vous puissiez savoir si votre VPN est utilisé. *Nouveau dans la 1.1.58.* |

## Trackers

![L'onglet Trackers](images/tab-trackers.png)

Les trackers du torrent, une ligne chacun.

| Colonne | Signification |
|---|---|
| **Nom** | l'adresse du tracker |
| **État** | **Fonctionne**, **Fonctionne :** avec le message du tracker, **Mise à jour...**, ou **Erreur** / le message d'erreur du tracker ; vide s'il n'a pas encore été contacté |
| **Mise à jour dans** | temps avant la prochaine annonce |
| **Sources**, **Pairs**, **Reçu** | les chiffres de l'essaim tels que le tracker les indique |

Pour contacter les trackers maintenant, utilisez **Mettre à jour le tracker** dans le menu contextuel. Pour ajouter, retirer ou réordonner des trackers, utilisez les [Propriétés](#propriétés-du-torrent).

Cliquez sur l'en-tête d'une colonne pour trier les trackers selon celle-ci, comme dans la [liste des fichiers](#trier-la-liste-des-fichiers) : **Sources**, **Pairs** et **Reçu** commencent par le plus grand nombre, les autres colonnes par la plus petite valeur ; un deuxième clic inverse l'ordre, un troisième revient à l'ordre propre du torrent (ses niveaux). Un tracker qui n'a pas indiqué de nombre reste en bas dans les deux cas. Rookparcel se souvient du tri. *Nouveau dans la 1.2.69.*

## Pairs

![L'onglet Pairs](images/tab-peers.png)

Les pairs auxquels vous êtes connecté. Une source web affiche son adresse dans la colonne **IP**.

| Colonne | Signification |
|---|---|
| **IP** | l'adresse du pair |
| **Client** | le programme BitTorrent du pair |
| **Drapeaux** | voir ci-dessous |
| **%** | la part du torrent que le pair possède |
| **Réception**, **Envoi** | vitesse depuis et vers ce pair |
| **Req.** | requêtes en attente, de nous et vers nous |
| **Reçu**, **Envoyé** | totaux avec ce pair |

Cliquez sur l'en-tête d'une colonne pour trier les pairs selon celle-ci : **%**, les vitesses, **Req.** et les totaux commencent par le plus grand nombre, **IP**, **Client** et **Drapeaux** par la plus petite valeur (les adresses par numéro) ; un deuxième clic inverse l'ordre, un troisième revient à l'ordre d'arrivée des pairs. La liste conserve son tri pendant que des pairs arrivent et partent et que leurs chiffres changent, et les pairs sélectionnés restent sélectionnés. Rookparcel se souvient du tri. *Nouveau dans la 1.2.69.*

Les **Drapeaux** sont les lettres de µTorrent :

| Drapeau | Signification |
|---|---|
| D / d | vous recevez des données du pair / vous voulez en recevoir, mais il n'en envoie pas pour le moment |
| U / u | vous envoyez des données au pair / il veut des données, mais vous n'en envoyez pas pour le moment |
| O | déblocage optimiste (optimistic unchoke) : vous donnez sa chance au pair |
| S | délaissé (snubbed) : le pair a cessé d'envoyer |
| I | le pair s'est connecté à vous |
| E / e | la connexion est chiffrée / seule sa prise de contact l'est |
| P | la connexion utilise uTP |
| H | le pair a été trouvé par le DHT |
| X | le pair a été trouvé par l'échange de pairs |
| L | le pair a été trouvé sur votre réseau local |

## Pièces

Une carte avec un carré par pièce. Les carrés bleus sont les pièces que vous avez. Les orange sont en cours de téléchargement. Les autres sont des nuances de vert : plus il est foncé, plus il y a de pairs connectés qui ont cette pièce ; le blanc signifie qu'aucun ne l'a. La ligne au-dessus de la carte compte les pièces. Quand il y en a plus que la carte n'en peut contenir, elle se termine par « ... N de plus ».

La liste sous la carte montre les pièces en cours de téléchargement : leur numéro (**#**), leur **Taille**, **Nb de blocs**, une barre **Blocs**, et combien de blocs sont **Terminés**. Dans la barre, les blocs bleus sont sur le disque, les verts sont en cours d'écriture, les orange sont demandés à un pair, les blancs sont encore libres.

## Fichiers

![L'onglet Fichiers](images/tab-files.png)

Les fichiers du torrent dans leurs dossiers, comme les montre la [fenêtre Ajouter un nouveau torrent](Ajouter-des-torrents#contenu-du-torrent).

| Colonne | Signification |
|---|---|
| **Nom** | le nom du fichier ou du dossier ; les dossiers s'ouvrent et se ferment avec la flèche à côté |
| **Taille** | sa taille |
| **Fait** | la part téléchargée |
| **%** | la même chose en pourcentage |
| **Nb de pièces** | combien de pièces il couvre |
| **Première pièce** | le numéro de sa première pièce |
| **Priorité** | **Haute**, **Normale**, **Basse** ou **Ne pas télécharger** ; **Mixte** pour un dossier dont les fichiers diffèrent |

Une ligne de dossier additionne la **Taille** et le **Fait** de ses fichiers ; elle n'a pas de colonnes de pièces.

### Trier la liste des fichiers

*Nouveau dans la 1.2.65 dans la fenêtre Windows ; dans la fenêtre GTK et l'application macOS, nouveau dans la 1.2.67.*

Cliquez sur l'en-tête d'une colonne pour trier les fichiers selon celle-ci ; une flèche dans l'en-tête indique la colonne et le sens.

- Le premier clic trie les noms, les tailles et les pièces à partir de la plus petite valeur, et **Fait**, **%** et **Priorité** à partir du haut. Un deuxième clic inverse l'ordre, et un troisième revient à l'ordre propre du torrent.
- Dans l'arborescence, les fichiers et les dossiers à l'intérieur de chaque dossier sont triés, les dossiers d'abord.
- Une liste triée par **Fait** ou **%** bouge au fil du téléchargement, et une liste triée par **Priorité** bouge quand vous changez les priorités. Les fichiers sélectionnés restent sélectionnés.
- Rookparcel se souvient du tri. **Prioriser selon l'ordre des fichiers** suit toujours l'ordre dans le torrent, quel que soit le tri.

### Arborescence ou liste unique

*Nouveau dans la 1.2.21.*

Le bouton **Arborescence** à côté de la zone de filtre, ou **Arborescence** dans le menu contextuel, bascule entre les dossiers et la liste unique de chemins de µTorrent 2.2.1 (dossier propre du torrent compris). Rookparcel se souvient du choix. Dans l'arborescence :

- Cliquez sur la flèche, double-cliquez sur un dossier, ou appuyez sur <kbd>Right</kbd> / <kbd>Left</kbd> pour l'ouvrir ou le fermer. <kbd>Left</kbd> sur un fichier va à son dossier.
- Les dossiers restent ouverts ou fermés pendant la mise à jour de la liste et après un changement de priorités. Sélectionner un autre torrent rouvre tous les dossiers.
- Une priorité, **Ne pas télécharger** ou **Prioriser selon l'ordre des fichiers** choisie sur un dossier s'applique à tous les fichiers qu'il contient (seulement à ceux affichés quand un filtre est actif). **Ouvrir** et **Ouvrir le dossier** sur un dossier ouvrent ce dossier ; **Déplacer...** est réservé aux fichiers seuls.

### Choisir les fichiers et leur ordre

Sélectionnez un ou plusieurs fichiers et faites un clic droit :

| Entrée | Action |
|---|---|
| **Ouvrir** | ouvre le fichier dans son programme par défaut, comme le ferait un double-clic dans l'Explorateur ; sans effet tant qu'il n'est pas encore sur le disque |
| **Ouvrir le dossier** | ouvre son dossier dans l'Explorateur, avec le fichier sélectionné une fois qu'il existe ; d'ici là, le dossier existant le plus proche du torrent |
| **Prioriser selon l'ordre des fichiers** | donne aux fichiers sélectionnés des priorités décroissantes selon leur ordre dans le torrent, de sorte qu'ils se terminent l'un après l'autre ; les fichiers que vous aviez ignorés sont téléchargés à nouveau |
| **Priorité > Priorité haute / Priorité normale / Priorité basse** | règle la priorité ; les fichiers de priorité plus élevée reçoivent leurs pièces en premier |
| **Ne pas télécharger** | ignore les fichiers |
| **Déplacer...** | choisit un nouvel emplacement pour un fichier et l'y déplace ; pour un fichier qui n'est pas sur le disque, indique au torrent de le chercher là |

Double-cliquer sur un fichier, ou appuyer sur <kbd>Enter</kbd> dessus, l'ouvre comme **Ouvrir**.

**Ne pas télécharger** est la façon d'ignorer des fichiers après l'ajout d'un torrent. Choisir de nouveau une priorité télécharge un fichier ignoré. Pour ignorer des fichiers avant le début du téléchargement, décochez-les dans la [fenêtre Ajouter un nouveau torrent](Ajouter-des-torrents#contenu-du-torrent). Pour ignorer les fichiers portant certains noms dans tous les torrents que vous ajoutez, utilisez **Ne pas télécharger les fichiers nommés :** dans Préférences > Interface ([Ajouter des torrents](Ajouter-des-torrents#fichiers-exclus-par-leur-nom) ; *nouveau dans la 1.2.63*).

Après **Déplacer...** vers un fichier qui n'était pas sur le disque, lancez **Forcer la vérification** pour que Rookparcel compte ce qu'il y a trouvé. Déplacer ne remplace jamais un fichier déjà présent au nouvel emplacement.

### Filtrer la liste des fichiers

*Nouveau dans la 1.1.56.*

Un torrent de plusieurs milliers de fichiers se gère plus facilement avec la zone **Filtrer les fichiers** en haut à droite de l'onglet Fichiers.

1. Cliquez dans la zone, ou appuyez sur <kbd>Ctrl</kbd>+<kbd>F</kbd> pendant que l'onglet Fichiers a le focus.
2. Saisissez un ou plusieurs mots. La liste n'affiche que les fichiers dont le chemin, dossiers compris, contient tous les mots, sans tenir compte de la casse ; dans l'arborescence, leurs dossiers sont affichés ouverts. « 12 sur 3,400 fichiers » à côté de la zone indique combien correspondent.
3. Sélectionnez les fichiers voulus et utilisez le menu contextuel. <kbd>Ctrl</kbd>+<kbd>A</kbd> ne sélectionne que les fichiers affichés.
4. <kbd>Esc</kbd> vide la zone ; <kbd>Enter</kbd> passe à la liste.

Le texte reste quand vous sélectionnez un autre torrent, de sorte que vous pouvez appliquer le même filtre à plusieurs. Il n'est pas enregistré à la fermeture de Rookparcel. **Prioriser selon l'ordre des fichiers** ordonne tous les fichiers du torrent, même quand un filtre est actif.

Par exemple, saisissez `sample` et réglez les fichiers correspondants sur **Ne pas télécharger**, ou saisissez `s01 mkv` pour trouver les fichiers vidéo de la première saison.

## Vitesse

Un graphique de la vitesse de réception (vert) et d'envoi (rouge) de tous les torrents ensemble sur les dix dernières minutes, avec les valeurs actuelles en haut.

## Journal

Le journal de Rookparcel pour cette session, chaque ligne avec l'heure. Il montre les torrents ajoutés et terminés, les déplacements, les changements du planificateur et de l'arrêt automatique, les problèmes avec les dossiers surveillés et le filtre IP, les téléchargements RSS, les importations et plus encore. Il garde les 1000 dernières lignes.

Un tracker qui ne répond pas est indiqué dans l'[onglet Trackers](#trackers), pas ici : avec des torrents publics, c'est courant et sans gravité, car les pairs viennent aussi du DHT et de l'échange de pairs. L'exception est un torrent privé, qui n'a que ses trackers : quand aucun ne répond, le Journal le signale une fois, avec la dernière erreur.

Quand quelque chose ne fonctionne pas comme prévu, regardez ici en premier. Voir aussi [Dépannage et FAQ](Dépannage-et-FAQ).

## Propriétés du torrent

![La fenêtre Propriétés du torrent](images/properties.png)

Ouvrez-la avec **Propriétés** dans le menu contextuel, <kbd>Enter</kbd> dans la liste des torrents, ou un double-clic sur un torrent qui n'est pas terminé. Le titre montre le nom du torrent, ou « Propriétés - 3 torrents » quand plusieurs sont sélectionnés.

**Trackers (une ligne vide entre les niveaux)**

Une adresse de tracker par ligne. Une ligne vide commence un nouveau niveau : Rookparcel essaie d'abord les trackers du premier niveau et ne passe au niveau suivant que s'ils échouent tous. Chaque ligne doit commencer par `http://`, `https://` ou `udp://` ; une adresse répétée est ignorée.

**Paramètres de bande passante**

| Paramètre | Signification |
|---|---|
| **Vitesse d'envoi maximale (ko/s) : [0 : illimitée]** | la limite d'envoi de ce torrent |
| **Vitesse de réception maximale (ko/s) : [0 : illimitée]** | la limite de réception de ce torrent |
| **Remplacer le nombre d'emplacements d'envoi par défaut** et **Nombre d'emplacements d'envoi : [0 : illimité]** | à combien de pairs ce torrent envoie en même temps, au lieu de la valeur par défaut de Préférences > BitTorrent |

**Objectif de partage**

Cochez **Remplacer les paramètres par défaut** pour donner à ce torrent son propre objectif au lieu de celui de Préférences > File d'attente :

| Paramètre | Signification |
|---|---|
| **Ratio minimum (%) : [0 : pas d'objectif de ratio]** | envoyer ce pourcentage de ce qui a été reçu ; 150 signifie une fois et demie |
| **Durée de partage minimum (minutes) : [0 : pas d'objectif de durée]** | partager au moins aussi longtemps |

Quand les deux sont réglés, les deux doivent être atteints. Ce qui se passe ensuite se règle dans Préférences > File d'attente ; voir [Mise en file et partage](Mise-en-file-et-partage). Un torrent démarré avec Forcer le démarrage n'atteint jamais son objectif.

**Autres paramètres**

| Paramètre | Signification |
|---|---|
| **Partage initial** | comme dans le menu contextuel ; seulement pour les torrents en partage |
| **Activer le DHT** | permet à ce torrent de trouver des pairs par le DHT |
| **Échange de pairs** | permet à ce torrent de connaître des pairs grâce à d'autres pairs |
| **Téléchargement séquentiel** | télécharge les pièces dans l'ordre, comme le fait **Avancé > Téléchargement séquentiel** dans le menu contextuel ; grisé quand tous les torrents sont terminés |
| **Télécharger d'abord les premières et dernières pièces** | télécharge d'abord les deux extrémités de chaque fichier voulu, comme le fait **Avancé > Télécharger d'abord les premières et dernières pièces** |

*Les deux cases d'ordre de téléchargement sont nouvelles dans la 1.2.63.* Voir [Gérer les torrents](Gérer-les-torrents#ordre-de-téléchargement) pour savoir quand elles aident.

Les torrents privés n'utilisent jamais le DHT ni l'échange de pairs.

**OK** applique ce que vous avez modifié.

### Plusieurs torrents à la fois

Sélectionnez plusieurs torrents et ouvrez les Propriétés pour les modifier ensemble. Un champ dont la valeur diffère d'un torrent à l'autre se charge vide, et une case à cocher affiche un troisième état, mixte. Un champ que vous laissez tel qu'il s'est chargé n'est pas touché sur chaque torrent ; seul ce que vous modifiez est appliqué à tous.

## Dans la version GTK (Linux, BSD)

- Les mêmes sept onglets avec les mêmes colonnes, le même menu et la même zone de filtre de l'onglet Fichiers, et la même fenêtre Propriétés.
- **Ouvrir** et **Ouvrir le dossier** utilisent les programmes par défaut et le gestionnaire de fichiers de votre bureau.

## Dans l'application macOS

- Les mêmes sept onglets et colonnes, le même menu de l'onglet Fichiers (avec **Ouvrir** et **Ouvrir le dossier** dans le Finder), **Arborescence** et la zone **Filtrer les fichiers**, et la même fenêtre Propriétés.

## Dans l'interface web

- Les mêmes sept onglets. L'onglet Général affiche aussi **Créé le :**, **Créé par :**, **Trackers :** et **Commentaire :**.
- Cliquez sur l'en-tête d'une colonne dans le tableau d'un onglet pour trier selon celle-ci.
- L'onglet Fichiers a aussi la zone **Filtrer les fichiers**. Son menu contextuel propose les quatre priorités : **Priorité haute**, **Priorité normale**, **Priorité basse** et **Ne pas télécharger**. L'ouverture des fichiers et Déplacer ne sont pas disponibles depuis un navigateur.
- **Propriétés** dans le menu du torrent ouvre la même fenêtre Propriétés du torrent.

Dans une fenêtre connectée avec **Fichier > Se connecter à distance...**, le menu de l'onglet Fichiers n'a ni **Ouvrir**, ni **Ouvrir le dossier**, ni **Déplacer...**, puisque les fichiers se trouvent sur l'autre ordinateur. Voir [Contrôle à distance](Contrôle-à-distance).

## Pages liées

- [Gérer les torrents](Gérer-les-torrents)
- [La fenêtre principale](La-fenêtre-principale)
- [Mise en file et partage](Mise-en-file-et-partage)
- [Limites de vitesse et planificateur](Limites-de-vitesse-et-planificateur)
