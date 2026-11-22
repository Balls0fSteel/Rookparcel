*Cette page est la traduction de la page anglaise [Searching for torrents](Searching-for-torrents). En cas de divergence, la page anglaise fait foi.*

# Rechercher des torrents

Cette page explique comment trouver de nouveaux torrents depuis Rookparcel : la différence entre filtrer votre propre liste et faire une recherche, comment ajouter des sources de recherche (l'assistant de recherche, ou votre propre Prowlarr ou Jackett), et comment ajouter ce qu'une recherche trouve. Elle couvre la fenêtre Windows, la fenêtre GTK (Linux, BSD) et l'interface web.

Pour un pas-à-pas de la version Windows avec des captures d'écran, voir [Rechercher des torrents sous Windows](Rechercher-des-torrents-sous-Windows).

## Comment fonctionne la recherche

Rookparcel n'a pas de moteur de recherche à lui et ne parcourt pas lui-même les sites. Il interroge une *source de recherche* : un programme qui sait chercher sur les sites de torrents et répond dans un format standard appelé Torznab. Deux programmes gratuits font cela :

- **Jackett** et **Prowlarr**, que vous exécutez peut-être déjà pour Sonarr ou Radarr, sur cet ordinateur, un NAS ou un serveur.
- L'**assistant de recherche**, qui est Jackett téléchargé et exécuté pour vous par Rookparcel. C'est la solution facile si vous n'avez ni l'un ni l'autre.

Une recherche part vers toutes les sources que vous choisissez en même temps. Les réponses sont fusionnées, les doublons (le même torrent venant de deux sources) ne sont affichés qu'une fois, et la liste est triée par sources (seeds).

*La zone de recherche et Gérer les sources figurent dans la version 1.1.40. L'assistant de recherche est nouveau dans la 1.1.59.*

## Filtrer votre liste ou faire une recherche

La zone de recherche en haut à droite de la fenêtre principale a deux rôles :

| Mode | Ce que fait Entrée |
|---|---|
| Filtre (le mode par défaut) | La saisie réduit votre propre liste de torrents aux noms qui contiennent le texte. Rien n'est recherché en ligne. |
| Une source, ou **Toutes les sources** | La saisie ne filtre pas la liste. Entrée envoie le texte à la source choisie (ou à toutes les sources) et ouvre la fenêtre **Rechercher des torrents** avec les résultats. |

<kbd>Ctrl</kbd>+<kbd>F</kbd> place le curseur dans la zone dans les deux interfaces. L'indication grise dans la zone vide dit dans quel mode elle se trouve.

### Windows

La petite flèche à côté de la zone ouvre un menu (<kbd>Alt</kbd>+<kbd>Down</kbd> dans la zone l'ouvre aussi) :

- **Filtrer les torrents** : le filtre de la liste. L'indication affiche « Rechercher des noms de torrents ».
- Chaque source de recherche activée, par son nom. L'indication affiche « Rechercher » et le nom de la source.
- **Toutes les sources**. L'indication affiche « Rechercher dans toutes les sources ».
- **Gérer les sources...**, qui ouvre la fenêtre Sources de recherche.

Le choix est conservé d'un démarrage à l'autre. Si la source choisie est supprimée ou désactivée, la zone revient à **Filtrer les torrents**.

### GTK

Le bouton de menu à côté de la zone a les mêmes entrées, sauf que l'entrée de filtre s'appelle **Rechercher des noms de torrents**. Quand une source est choisie, l'indication devient « Rechercher avec Prowlarr ou Jackett ».

### macOS

Le menu du champ de recherche a les mêmes entrées que celui de GTK : **Rechercher des noms de torrents**, chaque source activée, **Toutes les sources** et **Gérer les sources...**.

### Interface web

La zone de recherche de l'interface web ne fait que filtrer la liste. La recherche a son propre bouton, **Rechercher**, à côté (sur un téléphone, dans le menu **Plus** de la barre d'outils, sous le nom **Rechercher des torrents**). Ce bouton ouvre la fenêtre Rechercher des torrents avec le texte de la zone de filtre déjà saisi.

## Gérer les sources

Les sources sont conservées dans la fenêtre **Sources de recherche** :

- Windows : la flèche à côté de la zone de recherche, puis **Gérer les sources...**.
- GTK : le bouton de menu à côté de la zone de recherche, puis **Gérer les sources...**, ou le bouton **Gérer les sources...** de la fenêtre Rechercher des torrents.
- Interface web : **Rechercher**, puis **Gérer les sources...** dans la fenêtre Rechercher des torrents.

![La fenêtre Sources de recherche](images/search-sources.png)

La fenêtre liste vos sources, chacune avec une case à cocher. Une source cochée est interrogée quand vous cherchez dans **Toutes les sources** ; une source décochée est conservée mais laissée de côté. Les boutons :

| Bouton | Ce qu'il fait |
|---|---|
| **Ajouter...** | Ajoute une source de votre choix (voir ci-dessous). |
| **Modifier...** | Change le nom, l'URL, la clé d'API ou la case à cocher de la source sélectionnée. |
| **Supprimer** | Supprime la source sélectionnée, après confirmation. |
| **Tester** | Demande à la source sa liste de catégories. « OK » et un nombre de catégories signifient que l'URL et la clé d'API fonctionnent ; sinon, la ligne sous la liste affiche l'erreur. |

Dans les fenêtres Windows et GTK, les boutons de l'assistant de recherche sont aussi présents ici ; voir [L'assistant de recherche](#lassistant-de-recherche).

Tant que vous n'avez aucune source, une recherche affiche « Aucune source de recherche : ajoutez votre Prowlarr ou Jackett dans Gérer les sources. »

## Ajouter votre propre Prowlarr ou Jackett

Si vous exécutez déjà Prowlarr ou Jackett, ajoutez-le comme source :

1. Ouvrez **Sources de recherche** et cliquez sur **Ajouter...**. La fenêtre **Ajouter une source de recherche** s'ouvre.
2. **Nom :** ce que vous voulez. Laissé vide, la source est affichée sous le nom de son hôte.
3. **URL Torznab :** l'adresse du flux Torznab de la source :
   - Jackett : `http://host:9117/api/v2.0/indexers/all/results/torznab/`. La partie `all` interroge tous les indexeurs que vous avez configurés dans Jackett, donc une seule source suffit.
   - Prowlarr : `http://host:9696/1/api`, où `1` est le numéro de l'indexeur. Prowlarr n'a pas de flux « tous les indexeurs », ajoutez donc une source par indexeur.

   Remplacez `host` par `localhost` quand il s'exécute sur le même ordinateur, ou par l'adresse de l'ordinateur sur votre réseau.
4. **Clé d'API :** la clé affichée par la page de Jackett, ou par **Settings > General** de Prowlarr.
5. Laissez **Rechercher dans cette source** coché, et cliquez sur **OK**.
6. Sélectionnez la nouvelle source et cliquez sur **Tester**. Elle doit afficher « OK » et un nombre de catégories.

L'URL doit commencer par `http://` ou `https://`.

**La clé d'API** est stockée chiffrée (sous Windows) et n'est plus jamais affichée. Quand vous modifiez une source, la zone de la clé démarre vide avec l'indication grise « Inchangée » ; laissez-la vide pour conserver la clé stockée, ou saisissez-en une nouvelle. La clé est retirée de toute adresse affichée dans une erreur ou une ligne d'état.

**Proxy.** Une source sur cet ordinateur ou sur votre réseau local est jointe directement : une adresse IP ou `localhost` qui n'est pas publique, ou un nom d'hôte dont toutes les adresses le sont. Une source sur une adresse publique passe par le proxy réglé dans les Préférences, si vous en avez réglé un. Si le proxy résout lui-même les noms d'hôte (un proxy SOCKS réglé pour recevoir les noms d'hôte, ou un proxy HTTP pour une adresse `http://` simple), Rookparcel ne résout pas d'abord un nom sur cet ordinateur ; une source donnée par un nom d'hôte autre que `localhost` passe donc aussi par le proxy ; donnez l'adresse IP de la source pour qu'elle reste directe. Voir [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu).

### Dans l'interface web

Les fenêtres Ajouter et Modifier de l'interface web ont un champ de plus, **Mot de passe actuel de l'interface web :**. Ajouter une source, ou changer l'URL ou la clé d'API d'une source, demande le mot de passe actuel de l'interface web. Renommer une source, la cocher ou la décocher, la supprimer, la tester et chercher n'en demandent pas. Un mauvais mot de passe vous déconnecte, comme ailleurs dans l'interface web.

L'interface web n'affiche jamais une clé d'API stockée, et n'envoie jamais au navigateur les liens des résultats (qui contiennent la clé de l'indexeur).

## L'assistant de recherche

L'assistant de recherche est [Jackett](https://github.com/Jackett/Jackett), un programme gratuit distinct de ses propres auteurs, sous licence GPL-2.0. Rookparcel télécharge la version publiée de Jackett depuis GitHub quand vous cliquez sur le bouton, la vérifie avec la somme de contrôle que GitHub publie, la conserve dans un dossier `search-helper` de votre profil, et ne l'exécute que pendant que vous cherchez. Rien n'est téléchargé ni exécuté avant que vous cliquiez.

*Nouveau dans la 1.1.59.*

### Où il est proposé

| Où | Assistant de recherche |
|---|---|
| Fenêtre Windows (x64 et ARM64) | Oui |
| Fenêtre GTK sous Linux (x86-64 et ARM64) | Oui |
| Application macOS (Intel et Apple silicon) | Oui |
| Flatpak | Non |
| FreeBSD | Non (Jackett n'a pas de version pour ce système) |
| Versions créées par un gestionnaire de paquets sans la vérification des mises à jour | Non |
| Interface web et `rookparcel-nox` | Oui, quand le serveur est Windows, Linux ou macOS (pas FreeBSD, pas un Flatpak). *Nouveau dans la 1.1.67.* |

Là où il n'est pas proposé, la fenêtre Sources de recherche n'a pas de boutons de l'assistant, et **Ajouter...** pour votre propre Prowlarr ou Jackett fonctionne toujours.

### L'obtenir

1. Ouvrez **Sources de recherche** et cliquez sur **Obtenir l'assistant de recherche (Jackett)...**.
2. Rookparcel recherche la dernière version de Jackett et demande confirmation avant de télécharger. La question indique la version, la taille (environ 50 Mo) et le dossier où il sera placé. Cliquez sur **Oui**.
3. Une fenêtre de progression montre le téléchargement. Rookparcel vérifie ensuite le fichier, le décompresse et le démarre une fois pour lire sa liste de sites.
4. La fenêtre **Sites de l'assistant de recherche** s'ouvre. Rien n'est coché pour vous. Cochez les sites publics que vous voulez interroger et cliquez sur **OK**. Les sites marqués « Nécessite FlareSolverr, que l'assistant n'inclut pas » sont grisés et ne peuvent pas être cochés.
5. De retour dans Sources de recherche, l'assistant apparaît sous le nom **Assistant de recherche (tous)**. La ligne du bas indique combien de sites sont cochés.

Choisissez ensuite **Assistant de recherche (tous)** (ou **Toutes les sources**) dans le menu de la zone de recherche et lancez une recherche.

Deux ou trois sites suffisent pour commencer. Chaque site coché est interrogé à chaque recherche : plus de sites ralentissent donc les recherches et ajoutent des doublons.

Une fois l'assistant installé, le bouton **Obtenir** est remplacé par quatre autres :

| Bouton | Ce qu'il fait |
|---|---|
| **Choisir les sites...** | Rouvre la fenêtre Sites de l'assistant de recherche, pour cocher ou décocher des sites. |
| **Ouvrir la page de l'assistant** | Ouvre la page propre de Jackett dans votre navigateur et copie son mot de passe dans le presse-papiers (la page le demande). Utilisez-la pour ajouter des sites privés avec votre identifiant ; ils sont ensuite interrogés avec les sites cochés. |
| **Mettre à jour l'assistant de recherche** | Cherche une version plus récente de Jackett et demande confirmation avant de la télécharger. Vos sites et réglages restent. Rien ne se met à jour tout seul. |
| **Supprimer l'assistant de recherche** | Demande confirmation, puis arrête l'assistant, supprime son dossier et retire sa source de la liste. |

### Comment il fonctionne

- Il démarre à votre première recherche dans sa source et s'arrête tout seul dix minutes après la dernière. Il s'arrête aussi quand Rookparcel se ferme. Il ne démarre jamais de lui-même ni au démarrage de l'ordinateur.
- Il n'écoute que sur cet ordinateur (`127.0.0.1`), jamais sur votre réseau.
- Il n'a besoin d'aucun droit d'administrateur.
- Si vous réglez un proxy dans les Préférences, l'assistant l'utilise pour les sites. « Utiliser uniquement ce réseau » ne lie pas l'assistant, car c'est un programme distinct : ses requêtes empruntent la route normale de l'ordinateur.

### Dans l'interface web et `rookparcel-nox`

**Gérer les sources** dans l'interface web a les mêmes boutons et la même liste de sites. L'assistant est téléchargé et s'exécute sur le serveur, pas sur l'ordinateur où se trouve le navigateur, et avec `rookparcel-nox --no-state` il n'est pas proposé. Chaque étape qui prend du temps affiche sa progression, et le téléchargement a un bouton **Annuler**. Le télécharger demande le mot de passe actuel de l'interface web, comme l'ajout d'une source. La page propre de l'assistant écoute sur le `127.0.0.1` du serveur, donc **Ouvrir la page de l'assistant** n'est affiché qu'à un navigateur situé sur l'ordinateur où Rookparcel s'exécute ; ailleurs, la fenêtre le dit. Une fenêtre connectée avec **Fichier > Se connecter à distance...** utilise son propre assistant, sur cet ordinateur.

### Où il se trouve

Le dossier `search-helper` dans le dossier de votre profil :

| Système | Dossier |
|---|---|
| Windows | `%APPDATA%\Rookparcel\search-helper` ; en mode portable, le dossier contenant `rookparcel.exe` ; avec un profil, le dossier de ce profil |
| Linux (GTK) | `~/.local/share/rookparcel/gtk/search-helper` (avec un profil, sous `profiles/<name>/`) |
| macOS | `~/Library/Application Support/Rookparcel/mac/search-helper` (avec un profil, sous `profiles/<name>/`) |
| `rookparcel-nox` | `search-helper` dans son dossier d'état : `%APPDATA%\Rookparcel\nox\search-helper`, `~/.local/share/rookparcel/nox/search-helper`, ou `/var/lib/rookparcel-nox/search-helper` avec le service système fourni |

Chaque profil a son propre assistant. Voir [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel) et [Profils](Profils).

## La fenêtre Rechercher des torrents

La fenêtre s'ouvre quand vous appuyez sur Entrée dans la zone de recherche avec une source choisie (Windows, GTK), ou quand vous cliquez sur **Rechercher** (interface web). Elle contient :

- **La requête** et **Rechercher**, pour chercher de nouveau depuis la fenêtre.
- **Une catégorie** : **Toutes les catégories**, **Films**, **Audio**, **PC**, **TV**, **Livres** ou **Autres**.
- **Une source** : **Toutes les sources**, ou une seule source.
- **Arrêter**, pour arrêter les sources qui cherchent encore. Les résultats obtenus jusque-là restent.
- **Une ligne d'état par source** : « recherche en cours... », le nombre de résultats, ou l'erreur. Une source qui échoue n'arrête pas les autres.
- **Les résultats**, avec les colonnes **Nom**, **Taille**, **Sources**, **Pairs**, **Catégorie**, **Âge** et **Source**. Cliquez sur l'en-tête d'une colonne pour trier ; la liste est d'abord triée par sources (seeds). Quand deux sources trouvent le même torrent, il n'est listé qu'une fois, et **Source** nomme les deux.
- Le nombre de résultats, **Ajouter** et **Fermer**.

Sur un téléphone, les résultats de l'interface web n'affichent que Nom, Taille et Sources.

Fermer la fenêtre arrête la recherche (Windows : <kbd>Esc</kbd> la ferme aussi).

## Ajouter un résultat

Sélectionnez un résultat et cliquez sur **Ajouter**, double-cliquez dessus, ou appuyez sur <kbd>Enter</kbd> dans la liste.

Rookparcel récupère le résultat auprès de sa source (un fichier `.torrent` ou un lien magnet) et l'ajoute de la même façon qu'un fichier `.torrent` ouvert ou qu'un lien magnet cliqué. Si **Afficher une fenêtre avec les fichiers du torrent** est activé dans **Options > Préférences > Interface**, la fenêtre Ajouter un nouveau torrent s'ouvre d'abord. Voir [Ajouter des torrents](Ajouter-des-torrents).

La ligne du bas affiche « Ajouté : » et le nom du torrent. La fenêtre GTK et l'interface web refusent un résultat qui est déjà dans votre liste de torrents.

Dans l'interface web, le torrent est ajouté sur le serveur.

## Rechercher depuis une fenêtre distante ou l'interface web

- **Une fenêtre distante** (Fichier > Se connecter à distance, voir [Contrôle à distance](Contrôle-à-distance)) cherche depuis cet ordinateur, avec sa propre liste de sources, et ajoute ce que vous choisissez au serveur distant.
- **L'interface web de `rookparcel-nox`** conserve ses propres sources sur le serveur. Les recherches et les téléchargements s'exécutent sur le serveur ; chaque page ouverte voit la même recherche.
- **L'interface web qu'héberge une fenêtre Windows** utilise les sources de cette fenêtre : une source ajoutée dans la fenêtre apparaît dans la page, et inversement. Voir [Interface Web](Interface-Web).

L'interface web répond aussi à la partie recherche de l'API web de qBittorrent, de sorte que l'onglet de recherche d'une application distante qBittorrent interroge aussi vos sources.

## Si quelque chose ne va pas

| Ce que vous voyez | Ce que cela signifie |
|---|---|
| La fenêtre de résultats ne s'ouvre jamais | La zone est en mode filtre. Choisissez d'abord une source ou **Toutes les sources** dans son menu. |
| « Aucune source de recherche : ... » | Ajoutez une source dans Gérer les sources, ou obtenez l'assistant de recherche. |
| La ligne d'une source affiche une erreur | Cette source a échoué ; les autres répondent toujours. Cliquez sur **Tester** dans Sources de recherche pour vérifier son URL et sa clé d'API. |
| Le test affiche `HTTP 401` ou une erreur sur la clé d'API | La clé d'API est incorrecte. Modifiez la source et saisissez de nouveau la clé. |
| L'assistant de recherche ne trouve rien | Aucun site n'est coché, ou les sites cochés sont hors service ou ont changé. **Choisir les sites...** montre ce qui est coché ; essayez un autre site. |
| « Impossible d'obtenir l'assistant de recherche : ... » | GitHub n'a pas pu être joint ou le téléchargement a été interrompu. Réessayez. |
| « L'assistant de recherche n'a pas démarré : ... » | Le programme du dossier `search-helper` n'a pas pu s'exécuter : un téléchargement endommagé, ou un antivirus qui a supprimé des fichiers. Supprimez-le et obtenez-le de nouveau. |

Plus d'aide : [Dépannage et FAQ](Dépannage-et-FAQ). Pour le téléchargement automatique de nouveaux épisodes à partir de flux, voir [Téléchargeur RSS](Téléchargeur-RSS). Pour utiliser Rookparcel comme client de téléchargement de Prowlarr, voir [Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr).
