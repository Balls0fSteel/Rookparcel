*Ceci est la traduction française de la page [The main window](The-main-window) du wiki anglais. La page anglaise fait foi.*

# La fenêtre principale

Cette page passe en revue la fenêtre principale de Rookparcel : les menus, la barre d'outils et sa case de recherche, la liste des catégories, la liste des torrents et ses colonnes, les onglets de détails et la barre d'état. Elle décrit d'abord la version Windows, puis ce qui diffère dans la version GTK et dans l'interface web.

![La fenêtre principale avec des torrents et l'onglet Général](images/main-window.png)

## Les éléments en un coup d'œil

| Élément | Où | À quoi il sert |
|---|---|---|
| Barre des menus | en haut | **Fichier**, **Options** et **Aide** |
| Barre d'outils | sous les menus | des boutons pour les actions courantes, et la case de recherche à droite |
| Liste des catégories | à gauche | montre les torrents par état, étiquette ou tag, et vos flux RSS |
| Liste des torrents | au milieu | une ligne par torrent |
| Onglets de détails | sous la liste | les détails du torrent sélectionné : Général, Trackers, Pairs, Pièces, Fichiers, Vitesse, Journal |
| Barre d'état | en bas | l'état du réseau, le DHT et les vitesses totales |

Chaque élément, sauf les menus et la liste des torrents, peut être masqué depuis le menu **Options** : **Afficher la barre d'outils**, **Afficher les détails**, **Afficher la barre d'état** et **Afficher la liste des catégories**. Faites glisser les bordures entre la liste des catégories, la liste des torrents et les onglets de détails pour les redimensionner. Rookparcel retient ces choix, les tailles, les colonnes, l'ordre de tri et l'onglet ouvert.

## Les menus

- **Fichier :** **Ajouter un torrent...**, **Ajouter un torrent depuis une URL...**, **Créer un nouveau torrent...**, **Importer depuis qBittorrent...**, **Importer depuis µTorrent...**, **Profils**, **Se connecter à distance...**, **Verrouiller Rookparcel** (<kbd>Ctrl</kbd>+<kbd>L</kbd> ; voir [Verrouiller Rookparcel](Verrouiller-Rookparcel)) et **Quitter**.
- **Options :** **Préférences**, **Assistant de configuration**, **Téléchargeur RSS...**, les quatre éléments **Afficher** ci-dessus, **Arrêt automatique**, **Fermer dans la zone de notification** et **Réduire dans la zone de notification**.
- **Aide :** **Site web de Rookparcel**, **Rechercher des mises à jour**, **Statistiques**, **Faire un don**, **À propos de Rookparcel** et **Mentions des tiers**. **Statistiques** (nouveau dans la 1.2.39) montre ce que Rookparcel a envoyé et reçu pendant toutes ses sessions et pendant celle-ci, le ratio de partage, les données jetées, les pairs connectés ainsi que la file du disque et le cache de lecture ; les totaux de tous les temps restent quand des torrents sont supprimés. **Faire un don** (nouveau dans la 1.2.34) montre une courte note et un bouton qui ouvre la page de dons du site dans votre navigateur.

*Fichier > Profils existe depuis la 1.1.48 et Options > Assistant de configuration depuis la 1.1.62.*

Pages : [Verrouiller Rookparcel](Verrouiller-Rookparcel), [Ajouter des torrents](Ajouter-des-torrents), [Créer des torrents](Créer-des-torrents), [Importer depuis µTorrent et qBittorrent](Importer-depuis-uTorrent-et-qBittorrent), [Profils](Profils), [Contrôle à distance](Contrôle-à-distance), [Référence des préférences](Référence-des-préférences), [Premier démarrage et assistant de configuration](Premier-démarrage), [Téléchargeur RSS](Téléchargeur-RSS), [Arrêt automatique](Arrêt-automatique), [Zone de notification et démarrage](Zone-de-notification-et-démarrage), [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel).

## La barre d'outils

De gauche à droite :

| Bouton | Fonction |
|---|---|
| **Ajouter un torrent** | ajoute des fichiers `.torrent` (<kbd>Ctrl</kbd>+<kbd>O</kbd>) |
| **Ajouter un torrent depuis une URL** | ajoute un lien magnet ou une adresse web (<kbd>Ctrl</kbd>+<kbd>U</kbd>) |
| **Créer un nouveau torrent** | crée un nouveau `.torrent` (<kbd>Ctrl</kbd>+<kbd>N</kbd>) |
| **Supprimer** | retire de la liste les torrents sélectionnés (<kbd>Del</kbd>) |
| **Démarrer**, **Pause**, **Arrêter** | démarrent, mettent en pause ou arrêtent les torrents sélectionnés |
| **Monter dans la file**, **Descendre dans la file** | déplacent les torrents sélectionnés d'une place dans la file |
| **Téléchargeur RSS** | ouvre le Téléchargeur RSS |
| **Préférences** | ouvre les Préférences (<kbd>Ctrl</kbd>+<kbd>P</kbd>) |

Survolez un bouton pour voir son nom. À partir de la 1.1.57, la barre d'état affiche aussi une ligne sur ce que fait le bouton sous le pointeur, ou l'élément de menu en surbrillance. Ce que signifient Démarrer, Pause et Arrêter est expliqué sur [Gérer les torrents](Gérer-les-torrents#démarrer-pause-arrêter-et-forcer-le-démarrage).

### La case de recherche

La case à l'extrémité droite de la barre d'outils a deux usages, choisis avec la petite flèche à côté :

- **Filtrer les torrents** (le mode habituel) : pendant que vous tapez, la liste ne montre que les torrents dont le nom contient le texte, sans tenir compte de la casse. Le filtre s'ajoute à la catégorie choisie à gauche. <kbd>Esc</kbd> vide la case et revient à la liste. <kbd>Ctrl</kbd>+<kbd>F</kbd> place le curseur dans la case.
- **Une source de recherche** : <kbd>Enter</kbd> recherche des torrents sur le web par Prowlarr, Jackett ou l'assistant de recherche. Voir [Rechercher des torrents](Rechercher-des-torrents).

Le choix est retenu d'un démarrage à l'autre.

## La liste des catégories

La liste de gauche restreint la liste des torrents. Chaque entrée indique combien de torrents elle contient.

| Entrée | Montre |
|---|---|
| **Torrents** | tous les torrents |
| **En téléchargement** | les torrents pas encore terminés |
| **Terminés** | les torrents terminés |
| **Actifs** | les torrents qui reçoivent ou envoient des données en ce moment |
| **Inactifs** | les torrents qui ne déplacent aucune donnée en ce moment |
| **Étiquettes** > **Sans étiquette** | les torrents sans étiquette |
| **Étiquettes** > *une étiquette* | les torrents portant cette étiquette ; une entrée par étiquette utilisée |
| **Tags** > **Sans tag** | les torrents sans aucun tag |
| **Tags** > *un tag* | les torrents portant ce tag ; une entrée par tag, y compris pour un tag qu'aucun torrent n'a (0) |
| **Flux** | vos flux RSS ; en choisir un montre ses éléments dans la liste |

Les étiquettes se créent et s'attribuent depuis le menu du clic droit de la liste des torrents. Voir [Gérer les torrents](Gérer-les-torrents#étiquettes). Les tags s'attribuent de la même façon ; faites un clic droit sur la branche **Tags** pour **Nouveau tag...**, ou sur un tag pour **Supprimer le tag**. Voir [Gérer les torrents](Gérer-les-torrents#tags) (*les tags existent depuis la 1.2.63*). La branche Flux a son propre menu du clic droit ; voir [Téléchargeur RSS](Téléchargeur-RSS).

## La liste des torrents

Une ligne par torrent. Cliquez sur une ligne pour la sélectionner ; <kbd>Ctrl</kbd>+clic et <kbd>Shift</kbd>+clic en sélectionnent plusieurs ; <kbd>Ctrl</kbd>+<kbd>A</kbd> sélectionne tout. Faites un clic droit pour le menu du torrent, décrit sur [Gérer les torrents](Gérer-les-torrents#le-menu-contextuel).

Double-cliquez sur un torrent terminé pour ouvrir son dossier dans l'Explorateur. Double-cliquez sur un torrent qui n'est pas terminé, ou appuyez sur <kbd>Enter</kbd>, pour ouvrir ses [Propriétés](Détails-et-fichiers-des-torrents#propriétés-du-torrent).

### Colonnes

Faites un clic droit sur les en-têtes de colonnes pour afficher ou masquer des colonnes. **Nom** est toujours affichée. Faites glisser un en-tête pour déplacer la colonne ; faites glisser son bord pour changer sa largeur.

| Colonne | Montre | Affichée au départ |
|---|---|---|
| **Nom** | le nom du torrent | oui |
| **#** | sa place dans la file ; `*` quand il n'en a pas | oui |
| **Taille** | la taille des fichiers que vous avez choisi de télécharger | oui |
| **Fait** | la part téléchargée, en pourcentage | oui |
| **État** | ce que fait le torrent (voir plus bas) | oui |
| **Sources** | les sources connectées, et entre parenthèses les sources de tout l'essaim quand le tracker les indique | oui |
| **Pairs** | les pairs connectés, et entre parenthèses les pairs de l'essaim | oui |
| **Réception**, **Envoi** | les vitesses actuelles | oui |
| **Temps restant** | le temps restant estimé | oui |
| **Envoyé** | le total envoyé | oui |
| **Reçu** | le total reçu | non |
| **Ratio** | le total envoyé divisé par le total reçu | oui |
| **Dispo.** | combien de copies complètes détiennent ensemble les pairs connectés | oui |
| **Étiquette** | son étiquette | oui |
| **Ajouté le**, **Terminé le** | les dates | non |
| **Tracker** | le nom d'hôte du tracker utilisé | non |
| **Limite réception**, **Limite envoi** | ses propres limites de vitesse | non |
| **Restant** | ce qu'il reste à télécharger | non |
| **Sources/Pairs** | les sources par pair dans l'essaim | non |
| **Enregistré sous** | le dossier où se trouvent ses fichiers | non |
| **Gaspillé** | les données jetées (vérifications échouées, doublons) | non |
| **Tags** | ses tags, séparés par des virgules (*nouveau dans la 1.2.63*) | non |

### Tri

Cliquez sur un en-tête de colonne pour trier selon elle. Le premier clic trie dans le sens généralement le plus utile : les vitesses, les sources, les pairs, le ratio et la part faite vont du plus haut au plus bas, les autres du plus bas au plus haut. Cliquez de nouveau pour inverser. Les noms sont triés comme le fait l'Explorateur, si bien que « Episode 2 » vient avant « Episode 10 ».

### Ce que dit la colonne État

| État | Signification |
|---|---|
| **Téléchargement**, **Partage** | en cours |
| **En attente**, **Partage en attente** | démarré, mais en attente d'une place libre dans la file |
| **[F] Téléchargement**, **[F] Partage** | démarré avec **Forcer le démarrage**, en dehors de la file |
| **[S] Partage** | partage avec **Partage initial** activé |
| **En pause** | mis en pause par vous |
| **Arrêté** | arrêté par vous avant d'être terminé |
| **Terminé** | arrêté après avoir été terminé |
| **Téléchargement des métadonnées** | un lien magnet qui récupère la liste des fichiers du torrent |
| **Vérification 45.3 %**, **En attente de vérification** | vérification des fichiers sur le disque. **En attente de vérification** signifie que la vérification attend son lecteur, qui ne vérifie qu'un torrent à la fois. *En attente de vérification existe depuis la 1.2.76.* |
| **Déplacement** | déplacement de ses fichiers vers un autre dossier |
| **En attente du réseau** | **Utiliser uniquement ce réseau** ne trouve aucune carte réseau correspondante (le VPN est coupé) ; il reprend tout seul quand elle revient. *Nouveau dans la 1.1.58.* |
| **Erreur : ...** | quelque chose s'est mal passé ; le message dit quoi. Quand un lecteur, un partage ou un dossier contenant les fichiers du torrent a disparu, le message indique « files missing » et le torrent attend que les fichiers reviennent (*nouveau dans la 1.2.76* ; voir [Disque et stockage](Disque-et-stockage#lecteur-ou-partage-absent)). |

Voir [Gérer les torrents](Gérer-les-torrents) pour les actions qui mènent à ces états.

## Les onglets de détails

Les onglets sous la liste montrent le torrent sélectionné : **Général**, **Trackers**, **Pairs**, **Pièces** et **Fichiers**. **Vitesse** et **Journal** concernent tout le programme. Chaque onglet est décrit sur [Détails et fichiers des torrents](Détails-et-fichiers-des-torrents).

## La barre d'état

De gauche à droite :

| Élément | Montre |
|---|---|
| Avis de mise à jour | « Rookparcel ... est disponible - cliquez ici pour le télécharger », seulement quand une version plus récente est sortie. Voir [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel). |
| Réseau | **Réseau OK** quand d'autres pairs peuvent se connecter à vous, **Aucune connexion entrante** quand aucun ne l'a encore fait. **Port ... déjà utilisé** signifie qu'un autre programme, souvent un autre profil Rookparcel, occupe le port d'écoute ; cliquez dessus pour ouvrir Préférences > Connexion. L'état du planificateur suit quand il limite la vitesse, par exemple « Planificateur : limité ». **Restauration des torrents...** s'affiche au démarrage jusqu'au retour des torrents enregistrés dans la liste. **Transferts suspendus : confirmez les préférences** signifie que Rookparcel n'a pas pu se fier aux réglages qu'il a lus (un réglage de proxy, de règle de réseau ou d'interface d'écoute après une mise à jour, ou un fichier de réglages illisible), donc rien ne se connecte ; vérifiez **Options > Préférences** et cliquez sur **OK**. Voir [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel). |
| **DHT : ... nœuds** | combien de nœuds DHT Rookparcel connaît |
| **R : ... T : ...** | la vitesse de réception, et le total reçu pendant cette session |
| **E : ... T : ...** | la vitesse d'envoi, et le total envoyé pendant cette session |

Quand **Garder un ping bas** freine les vitesses, les éléments de vitesse ajoutent la limite automatique, par exemple « R : 400 kB/s (auto 450 kB/s) T : 2.10 GB ».

*Nouveau dans la 1.1.58 :* tant que **Utiliser uniquement ce réseau** ne trouve aucune carte correspondante, l'élément réseau indique « En attente du réseau », et le survoler montre votre adresse telle que les trackers la voient. Le texte **Port ... déjà utilisé** est venu avec les profils dans la 1.1.45.

Voir [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu), [Limites de vitesse et planificateur](Limites-de-vitesse-et-planificateur) et [Garder un ping bas](Garder-un-ping-bas).

## La barre de titre et la zone de notification

Avec **Afficher la vitesse actuelle dans la barre de titre** cochée (Préférences > Interface), le titre de la fenêtre commence par les vitesses de réception et d'envoi, qui s'affichent aussi dans la barre des tâches.

Rookparcel place une icône dans la zone de notification. La survoler montre la version et les vitesses. Un clic affiche ou masque la fenêtre ; un clic droit ouvre un menu avec **Masquer Rookparcel** ou **Afficher Rookparcel**, **Ajouter un torrent...**, **Ajouter un torrent depuis une URL...**, **Démarrer tous les torrents**, **Arrêter tous les torrents**, **Verrouiller Rookparcel** (seulement une fois un code PIN défini) et **Quitter**. Par défaut, le bouton de fermeture masque la fenêtre dans la zone de notification et les torrents continuent ; **Quitter** quitte. Voir [Zone de notification et démarrage](Zone-de-notification-et-démarrage).

## Dans la version GTK (Linux, BSD)

- La même disposition : barre d'outils, liste des catégories, liste des torrents, onglets de détails et barre d'état. La barre d'outils n'a pas de bouton **Préférences** ; utilisez **Options > Préférences** ou <kbd>Ctrl</kbd>+<kbd>P</kbd>.
- Le menu **Options** contient **Préférences**, **Assistant de configuration**, **Téléchargeur RSS...** et **Arrêt automatique**. Il n'y a ni éléments **Afficher** ni éléments de la zone de notification. **Fichier** contient **Verrouiller Rookparcel** comme sous Windows.
- **Fichier > Quitter** est aussi <kbd>Ctrl</kbd>+<kbd>Q</kbd>.
- Un clic droit sur un en-tête de colonne affiche ou masque des colonnes ; les en-têtes peuvent être glissés pour réordonner les colonnes.
- Le double-clic fonctionne comme sous Windows : un torrent terminé ouvre son dossier, un autre ouvre les Propriétés.

## Dans l'interface web

- La page a une barre d'outils (**Ajouter un torrent**, **Ajouter une URL**, **Supprimer**, **Démarrer**, **Pause**, **Arrêter**, **Monter**, **Descendre**, **RSS**, **Préférences** et **Rechercher**), la liste des catégories, la liste des torrents et les mêmes sept onglets de détails, plus une case de recherche qui filtre la liste par nom.
- Cliquez sur un en-tête pour trier, faites-le glisser pour réordonner, faites glisser son bord pour redimensionner. Faites un clic droit sur les en-têtes pour afficher ou masquer des colonnes ; **Réinitialiser les colonnes** revient au réglage par défaut.
- Sur un téléphone, la liste des catégories s'ouvre avec le bouton ☰, et le bouton ⋯ contient les actions qui ne tiennent pas, **Trier par** et **Tri décroissant**.

Voir [Interface web](Interface-Web).

## Pages connexes

- [Raccourcis clavier](Raccourcis-clavier)
- [Gérer les torrents](Gérer-les-torrents)
- [Détails et fichiers des torrents](Détails-et-fichiers-des-torrents)
