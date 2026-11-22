*Cette page est la traduction de la page anglaise [Searching for torrents on Windows](Searching-for-torrents-on-Windows). En cas de divergence, la page anglaise fait foi.*

# Rechercher des torrents sous Windows

Un guide court de la zone de recherche de la version Windows de Rookparcel : comment obtenir l'assistant de recherche, choisir les sites qu'il interroge et ajouter ce qu'il trouve. Dix minutes et un téléchargement de 55 Mo, à faire une seule fois.

**Version :** l'assistant de recherche est présent dans Rookparcel 1.1.59 et les versions plus récentes. La version 1.1.40 a la zone de recherche et Gérer les sources, mais pas le bouton « Obtenir l'assistant de recherche » ; dans ce cas, c'est la dernière section (votre propre Prowlarr ou Jackett) qui s'applique.

## Ce que vous obtenez

Rookparcel n'a pas de moteur de recherche à lui. Il interroge une *source de recherche* : un petit programme qui sait chercher sur les sites de torrents. La source la plus simple est l'**assistant de recherche**, qui est [Jackett](https://github.com/Jackett/Jackett), un programme gratuit distinct de ses propres auteurs. Rookparcel le télécharge pour vous en un clic, le conserve dans son propre dossier et ne l'exécute que pendant que vous cherchez. Vous choisissez quels sites publics il interroge. Rien n'est téléchargé ni exécuté tant que vous n'avez pas cliqué sur le bouton.

Si vous exécutez déjà Prowlarr ou Jackett vous-même, vous pouvez l'utiliser à la place : voir la dernière section.

## 1. Ouvrir Gérer les sources

La zone de recherche se trouve en haut à droite de la fenêtre principale. La petite flèche à côté ouvre un menu. Choisissez **Gérer les sources...**.

![Le menu déroulant de la zone de recherche avec Gérer les sources](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-dropdown.png)

La fenêtre Sources de recherche s'ouvre. Elle est vide au départ.

![La fenêtre Sources de recherche avec le bouton Obtenir l'assistant de recherche](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-sources.png)

## 2. Obtenir l'assistant de recherche

Cliquez sur **Obtenir l'assistant de recherche (Jackett)...**. Rookparcel recherche la dernière version de Jackett et demande confirmation avant de télécharger quoi que ce soit :

> L'assistant de recherche est Jackett, un programme distinct créé par ses propres auteurs, sous licence GPL-2.0, téléchargé depuis github.com/Jackett/Jackett (v0.24.3009, 54.5 Mo).
>
> Il est placé dans C:\Users\you\AppData\Roaming\Rookparcel\search-helper et ne s'exécute que sur cet ordinateur, uniquement pendant vos recherches. Vous choisissez ensuite les sites publics qu'il interroge.
>
> Le télécharger maintenant ?

Cliquez sur **Oui**. Le téléchargement prend de quelques secondes à une minute, selon votre connexion.

![La fenêtre de progression du téléchargement](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-download.png)

Rookparcel vérifie le fichier avec la somme de contrôle de la version, le décompresse et le démarre une fois pour lire sa liste de sites. Aucune invite de Windows n'apparaît : rien n'a besoin de droits d'administrateur, et l'assistant n'écoute que sur cet ordinateur, de sorte que le pare-feu n'a rien à demander.

## 3. Cocher les sites à interroger

La liste des sites s'ouvre toute seule (plus tard, c'est le bouton **Choisir les sites...**). Rien n'est coché pour vous. Cochez les sites publics que vous voulez interroger, puis cliquez sur **OK**.

![La fenêtre Sites de l'assistant de recherche avec The Pirate Bay coché](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-sites.png)

- Les lignes grisées indiquent « Nécessite FlareSolverr, que l'assistant n'inclut pas ». Ces sites se trouvent derrière une vérification anti-robots que l'assistant ne peut pas passer seul ; laissez-les.
- Deux ou trois sites suffisent pour commencer. Chaque site coché est interrogé à chaque recherche : plus de sites signifient donc des recherches plus lentes et plus de doublons.
- Les sites privés demandent leurs identifiants. La page propre de l'assistant s'en occupe : voir « Sites privés » ci-dessous.

De retour dans Sources de recherche, l'assistant apparaît maintenant sous le nom **Assistant de recherche (tous)** avec ses boutons en dessous, et la ligne du bas indique combien de sites sont cochés. Cliquez sur **Fermer**.

![Sources de recherche avec l'assistant installé et deux sites cochés](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-sources-installed.png)

## 4. Rechercher

Rouvrez le menu de la flèche et choisissez **Assistant de recherche (tous)**. L'indication de la zone de recherche change en conséquence. (La première entrée, **Filtrer les torrents**, est l'autre rôle de la zone : réduire votre propre liste de torrents par nom. **Toutes les sources** interroge en une fois toutes les sources que vous avez.)

![Le menu déroulant avec Assistant de recherche (tous) choisi](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-dropdown-helper.png)

Saisissez ce que vous cherchez et appuyez sur **Entrée**. La fenêtre Rechercher des torrents s'ouvre et les résultats arrivent en quelques secondes.

![La fenêtre Rechercher des torrents avec les résultats pour « ubuntu »](https://raw.githubusercontent.com/Balls0fSteel/Rookparcel/main/docs/screenshots/search-win-results.png)

- Cliquez sur l'en-tête d'une colonne pour trier : les sources (Seeds) du plus grand au plus petit nombre permettent de trouver les torrents en bonne santé.
- La zone de catégorie limite la recherche à Films, TV, Audio, PC, Livres ou Autres. La zone de source et le bouton Rechercher relancent la recherche depuis cette fenêtre.
- Sélectionnez un résultat et cliquez sur **Ajouter**, ou double-cliquez dessus. Le torrent est ajouté comme l'est un lien magnet ; si « Afficher une fenêtre avec les fichiers du torrent » est activé dans les Préférences, la fenêtre Ajouter un nouveau torrent s'ouvre d'abord.
- Pour que la zone filtre de nouveau votre liste de torrents, choisissez **Filtrer les torrents** dans le menu de la flèche. Le choix est conservé d'un démarrage à l'autre.

C'est toute la configuration. Les sections ci-dessous serviront plus tard.

## Bon à savoir

**Il ne s'exécute que pendant que vous cherchez.** L'assistant démarre à votre première recherche et s'arrête tout seul dix minutes après la dernière. Il se ferme avec Rookparcel, quelle que soit la façon dont Rookparcel se termine. Il ne démarre jamais de lui-même ni au démarrage de l'ordinateur, et il n'écoute que sur cet ordinateur (`127.0.0.1`), jamais sur votre réseau.

**Où il se trouve.** Le dossier `search-helper` dans le dossier des paramètres de Rookparcel : `%APPDATA%\Rookparcel\search-helper` normalement, le dossier contenant `rookparcel.exe` en mode portable, et le dossier du profil quand vous utilisez des profils. Les réglages et le journal propres à Jackett se trouvent dans son sous-dossier `data`.

**Proxy et VPN.** Si vous réglez un proxy dans Préférences > Connexion, l'assistant l'utilise aussi pour les sites. L'assistant est un programme distinct : « Utiliser uniquement ce réseau » ne le lie donc pas, et ses requêtes vers les sites empruntent la route normale de Windows, qui est votre VPN quand le VPN transporte tout votre trafic.

**Mise à jour.** Rien ne se met à jour tout seul. **Mettre à jour l'assistant de recherche** dans Sources de recherche cherche une version plus récente de Jackett et demande confirmation avant de la télécharger ; vos sites et réglages restent.

**Suppression.** **Supprimer l'assistant de recherche** l'arrête, supprime son dossier et retire sa source de la liste. Le bouton **Obtenir** revient. Supprimer le dossier des paramètres de Rookparcel le supprime aussi.

**Sites privés.** **Ouvrir la page de l'assistant** ouvre la page propre de Jackett dans votre navigateur et copie son mot de passe dans le presse-papiers (la page le demande). Vous pouvez y ajouter un site privé avec votre identifiant, comme dans n'importe quel Jackett. Les sites ajoutés là sont interrogés avec les sites cochés.

**Antivirus.** Le téléchargement est la version publiée de Jackett elle-même, vérifiée avec la somme de contrôle que GitHub publie pour elle. Si un antivirus met en quarantaine des fichiers du dossier `search-helper`, les recherches échouent avec « L'assistant de recherche n'a pas démarré » ; restaurez les fichiers, ou supprimez l'assistant et obtenez-le de nouveau.

## Si quelque chose ne va pas

| Ce que vous voyez | Ce que cela signifie |
|---|---|
| « Impossible d'obtenir l'assistant de recherche : ... » | GitHub n'a pas pu être joint (pas de connexion, un proxy qui le bloque) ou le téléchargement a été interrompu. Réessayez. |
| « L'assistant de recherche n'a pas démarré : ... » | Le programme du dossier `search-helper` n'a pas pu s'exécuter : un téléchargement endommagé ou un antivirus. Supprimez-le et obtenez-le de nouveau. |
| Une recherche ne trouve rien | Aucun site n'est coché, ou les sites cochés sont hors service ou ont changé. Choisir les sites... montre ce qui est coché ; essayez un autre site. |
| « Assistant de recherche (tous) : ... » avec une erreur | Un site a échoué ; les autres répondent toujours. Le message nomme le site. |
| La fenêtre de résultats ne s'ouvre jamais | La zone est en mode **Filtrer les torrents**. Choisissez d'abord **Assistant de recherche (tous)** ou une source dans le menu de la flèche. |

## Utiliser plutôt votre propre Prowlarr ou Jackett

Si vous exécutez déjà Prowlarr ou Jackett (sur ce PC, un NAS, un serveur), ajoutez-le comme source et passez l'assistant :

1. Dans Sources de recherche, cliquez sur **Ajouter...**.
2. **Nom :** ce que vous voulez. **URL Torznab :** pour Jackett, `http://host:9117/api/v2.0/indexers/all/results/torznab/` ; pour Prowlarr, l'URL du flux Torznab d'un indexeur, `http://host:9696/1/api`, une source par indexeur. **Clé d'API :** celle de la page de Jackett, ou Settings > General de Prowlarr.
3. Cliquez sur **Tester** : elle doit afficher « OK » et un nombre de catégories. Puis **OK**.

La clé d'API est stockée chiffrée et n'est plus jamais affichée. Une source sur votre propre ordinateur ou réseau est jointe directement ; une source publique passe par votre proxy, si vous en avez réglé un. (Si le proxy résout lui-même les noms d'hôte, une source donnée par un nom d'hôte autre que `localhost` passe aussi par lui ; donnez l'adresse IP de la source pour qu'elle reste directe.) L'assistant et vos propres sources peuvent être utilisés côte à côte : **Toutes les sources** dans le menu de la flèche les interroge toutes.

Pour savoir comment l'assistant est construit et ce qu'il fait sur le réseau, voir [SEARCH_HELPER.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/SEARCH_HELPER.md) ; pour la recherche Torznab elle-même, la partie 2 de [INTEGRATIONS.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/INTEGRATIONS.md).
