# Sonarr, Radarr et Prowlarr

*Cette page est une traduction de la page anglaise [Sonarr, Radarr and Prowlarr](Sonarr-Radarr-and-Prowlarr). En cas de différence, la page anglaise fait foi.*

Sonarr, Radarr et Prowlarr peuvent utiliser Rookparcel comme client de téléchargement. L'interface Web de Rookparcel répond à la même API que celle de qBittorrent : dans ces applications, vous ajoutez donc Rookparcel comme un client **qBittorrent**. Cette page explique comment le configurer avec `rookparcel-nox` ou avec l'interface Web propre au programme Windows, les catégories, les dossiers de téléchargement et la suppression des téléchargements terminés.

Pour rechercher dans votre Prowlarr ou Jackett depuis Rookparcel (l'autre sens), voir [Rechercher des torrents](Rechercher-des-torrents).

## Ce qui fonctionne

Ces points ont été vérifiés avec les vraies applications (Sonarr 4, Radarr et Prowlarr dans Docker), à la fois avec `rookparcel-nox` sous Linux et avec l'interface Web du programme Windows :

- Le **Test** du client de téléchargement dans chaque application.
- Les récupérations (grabs) depuis Sonarr, Radarr et Prowlarr, chacune dans sa propre catégorie.
- La durée de partage d'un indexeur arrivant avec le torrent et l'arrêtant à l'heure.
- Sonarr et Radarr important le fichier terminé (par lien physique, ou par un mappage de chemin distant) puis supprimant le torrent avec **Remove Completed**.
- La recherche et la récupération de Prowlarr.

Non vérifiés : Lidarr et Readarr (Lidarr utilise le même code client que Radarr), et les récupérations de liens magnet.

## 1. Activer l'interface Web de Rookparcel

**Avec rookparcel-nox :**

```sh
rookparcel-nox run --save /srv/media/downloads --webui 0.0.0.0:8080
```

Connectez-vous une fois dans un navigateur avec le mot de passe temporaire que nox affiche et définissez le vôtre dans **Préférences > Interface web**. Détails : [Interface Web](Interface-Web#lactiver-dans-rookparcel-nox) et [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

`0.0.0.0` permet à d'autres ordinateurs et à des conteneurs de l'atteindre. Quand les applications fonctionnent sur le même ordinateur, pas dans des conteneurs, `--webui 8080` (cet ordinateur seulement) suffit.

**Avec le programme Windows :** **Options > Préférences > Interface web**, cochez **Activer l'interface web**, définissez un **Nom d'utilisateur :** et un **Mot de passe :**, et cochez **Autoriser les connexions depuis d'autres ordinateurs** si les applications fonctionnent sur un autre ordinateur. Le Pare-feu Windows a alors besoin d'une règle pour `rookparcel.exe`. Détails : [Interface Web](Interface-Web#lactiver-dans-le-programme-windows). Rookparcel doit fonctionner pour que les applications l'atteignent.

## 2. Ajouter Rookparcel dans l'application

Dans Sonarr, Radarr ou Prowlarr, ajoutez un nouveau client de téléchargement de type **qBittorrent** et remplissez :

| Champ | Valeur |
|---|---|
| Host | L'adresse IP de l'ordinateur qui exécute Rookparcel (ou un nom d'hôte que vous avez autorisé, voir [plus bas](#noms-dhôte-et-docker)). |
| Port | Le port de l'interface Web, comme `8080`. |
| SSL | Désactivé, sauf si l'interface Web utilise HTTPS avec un certificat auquel l'application fait confiance. Les vérifications ci-dessus ont été faites en HTTP simple. |
| Username, Password | L'identifiant de l'interface Web. |
| Category | Une catégorie de votre choix, comme `tv-sonarr`, `radarr` ou `prowlarr`. |

Cliquez sur **Test**, puis enregistrez. Le Test crée la catégorie dans Rookparcel si elle n'existe pas encore.

Au lieu d'un nom d'utilisateur et d'un mot de passe, les applications peuvent envoyer une clé d'API. Activez **Accepter une clé d'API (Authorization: Bearer)** dans **Préférences > Interface web** de Rookparcel, saisissez-y une clé d'au moins 16 caractères, et donnez la même clé à l'application. Une troisième voie est **Ignorer l'authentification pour les clients de ces sous-réseaux (...)** avec le réseau de l'application, ce qui laisse entrer sans connexion tous les clients de ce réseau : n'indiquez que des réseaux auxquels vous faites autant confiance qu'au mot de passe ([Sécurité de l'interface Web](Interface-Web#sécurité)).

## Noms d'hôte et Docker

Rookparcel ne répond qu'aux adresses IP et à `localhost`, sauf si vous autorisez d'autres noms. Une application qui l'atteint par un nom, comme `http://rookparcel:8080` depuis un conteneur Docker ou le nom réseau d'un PC, reçoit « Bad Request: unexpected Host header », et son Test signale un échec de connexion.

Utilisez soit l'adresse IP dans l'application, soit ajoutez le nom à **Domaines du serveur (séparez plusieurs entrées par un point-virgule) :** :

- Dans l'interface Web de nox : **Préférences > Interface web**, sous **Connectivité**. La modifier là demande le **Mot de passe actuel :**.
- Dans le programme Windows : **Options > Préférences > Interface web**.

Plusieurs noms se séparent par `;` ; `*` correspond à tout. Préférez nommer les hôtes plutôt que `*` : la vérification protège contre les pages web qui atteignent l'interface Web sous un nom qui leur est propre.

Pour exécuter nox lui-même dans un conteneur à côté des applications, voir [Rookparcel avec Docker](Rookparcel-avec-Docker) : son image, un fichier Compose avec le dossier de médias partagé, et les identifiants d'utilisateur.

## Catégories, étiquettes et tags

Une catégorie qBittorrent est une **étiquette** dans Rookparcel. Les torrents qu'une application ajoute reçoivent sa catégorie comme étiquette ; vous les voyez donc sous cette étiquette dans la liste des catégories ([Gérer les torrents](Gérer-les-torrents)). Changer l'étiquette d'un torrent dans Rookparcel change aussi sa catégorie pour l'application ; une application ne suit que les torrents de sa propre catégorie.

Quand l'application ne donne pas de dossier de téléchargement propre, un torrent va dans le dossier d'enregistrement de sa catégorie si la catégorie en a un, et sinon dans le dossier de téléchargement par défaut de Rookparcel.

Un tag qBittorrent est un **tag** dans Rookparcel (*nouveau dans la version 1.2.63* ; auparavant, les tags étaient acceptés puis abandonnés). Un torrent peut en avoir autant qu'on veut en plus de son étiquette ; ils s'affichent sous **Tags** dans la liste des catégories et dans la colonne **Tags** ([Gérer les torrents](Gérer-les-torrents#tags)). Avec **Add Series Tags** de Sonarr activé, une récupération reçoit les tags de la série. Radarr n'envoie aucun tag. Un nom contenant une virgule ou un caractère de contrôle est ignoré sans erreur, et les noms sont sensibles à la casse, comme dans qBittorrent.

## Dossiers de téléchargement et chemins

L'application importe les fichiers terminés en les lisant au chemin que Rookparcel indique. Ce chemin doit désigner le même fichier pour les deux :

- **Même ordinateur, sans conteneurs :** rien à faire.
- **Conteneurs :** montez le dossier de téléchargement au même chemin dans le conteneur de l'application que celui qu'utilise Rookparcel, par exemple `/srv/media` dans les deux. Les liens physiques fonctionnent alors aussi.
- **Ordinateurs différents** (par exemple Rookparcel sous Windows enregistrant dans un partage du serveur où fonctionne Sonarr) : ajoutez un **Remote Path Mapping** dans Sonarr et Radarr, du chemin de Rookparcel vers le chemin de l'application, comme `\\server\media\downloads\` vers `/srv/media/downloads/`. C'était la configuration vérifiée avec le programme Windows.

Avec nox, `--webui-save-root DIR` limite les dossiers dans lesquels l'interface Web, et donc les applications, peuvent enregistrer ([Utiliser rookparcel-nox](Utiliser-rookparcel-nox#interface-web)). Incluez les dossiers que les applications utilisent.

## Laisser de côté les fichiers indésirables

*Nouveau dans la version 1.2.63.* **Ne pas télécharger les fichiers nommés :** dans **Préférences > Interface** de Rookparcel s'applique aussi aux récupérations des applications. Quand il est activé, les fichiers dont les noms correspondent aux motifs, comme `*.lnk; Thumbs.db; *sample*`, sont réglés pour ne pas être téléchargés quand l'application ajoute le torrent ; pour un lien magnet, une fois que sa liste de fichiers est arrivée. Un torrent dont tous les fichiers correspondent est téléchargé en entier. Voir [Ajouter des torrents](Ajouter-des-torrents#fichiers-exclus-par-leur-nom).

Les applications qui règlent les « Excluded file names » de qBittorrent par son API atteignent le même paramètre : `excluded_file_names_enabled` et `excluded_file_names` (un motif par ligne).

## Limites de partage et Remove Completed

**Remove Completed** dans Sonarr et Radarr supprime un torrent une fois qu'il est arrêté et que sa limite de partage est atteinte. Rookparcel signale les deux :

- **Un ratio ou une durée de partage défini sur l'indexeur** (dans Sonarr, Radarr ou Prowlarr) arrive avec chaque torrent et devient l'objectif de partage propre à ce torrent. Une valeur que l'indexeur laisse vide est prise dans les paramètres de Rookparcel.
- **L'objectif de partage propre à Rookparcel** se trouve dans **Préférences > File d'attente**, sous **Objectif de partage** : « Ratio minimum (%) : [0 : pas d'objectif de ratio] » (150 % par défaut) et « Durée de partage minimum (minutes) : [0 : pas d'objectif de durée] ». Quand un torrent l'atteint, Rookparcel l'arrête, et l'application peut le supprimer. Voir [Mise en file et partage](Mise-en-file-et-partage).

Deux choses à savoir :

- Avec **Limiter la vitesse d'envoi à (ko/s) : [0 : arrêter]** activé, un torrent continue de partager après son objectif, si bien que l'application ne le voit jamais comme terminé et ne le supprime pas.
- L'objectif de Rookparcel est atteint quand chaque cible définie est atteinte (ratio et durée), comme dans µTorrent ; qBittorrent s'arrête à la première. Un torrent peut donc partager un peu plus longtemps que l'application ne l'attend, mais il n'est jamais supprimé avant que les deux soient satisfaits.

Un torrent que vous démarrez avec **Forcer le démarrage** n'atteint jamais son objectif de partage.

## Prowlarr

Prowlarr ne fait qu'ajouter des torrents ; il ne les suit pas et ne les importe pas. Ajoutez Rookparcel dans Prowlarr de la même manière (un client qBittorrent avec sa propre catégorie, comme `prowlarr`) pour envoyer à Rookparcel les récupérations de la recherche propre à Prowlarr.

## Limites connues

- Rookparcel se présente comme la version 2.11.2 de l'API de qBittorrent.
- Ignorés quand une application les envoie : le choix de la disposition du contenu et la limite de durée de partage inactif de Sonarr. « Première et dernière pièces d'abord » est respecté depuis la version 1.2.63 ; avant, il était ignoré.
- La demande de revérification d'une application conserve le comportement de qBittorrent : un torrent arrêté reste arrêté et est vérifié à son démarrage. **Forcer la vérification** dans l'interface Web et les fenêtres démarre un torrent arrêté depuis la version 1.2.76.
- Ajouter un torrent qui est déjà dans la liste reçoit une réponse de succès ; l'application suit alors celui qui existe.
- L'API affiche `ip_filter_enabled` et `ip_filter_path` du filtre IP mais ne les modifie pas, et elle n'a pas de paramètre pour l'adresse web de la liste de blocage. Réglez-les dans les Préférences propres à Rookparcel.

## En cas de problème

| Ce que vous voyez | Ce que cela signifie |
|---|---|
| Le Test échoue, et l'adresse de Rookparcel est un nom | La vérification de l'hôte a refusé le nom. Utilisez l'adresse IP, ou ajoutez le nom aux **Domaines du serveur** ([plus haut](#noms-dhôte-et-docker)). |
| Le Test échoue avec une adresse IP | L'interface Web n'est pas activée, elle n'écoute que sur `127.0.0.1` (`--webui 8080` de nox, ou **Autoriser les connexions depuis d'autres ordinateurs** désactivé), un pare-feu bloque le port, ou l'identifiant est faux. Trop de mauvaises connexions verrouillent l'adresse de l'application pendant une heure. |
| L'importation échoue : le fichier est introuvable | L'application voit le téléchargement sous un autre chemin. Utilisez le même chemin de montage, ou un Remote Path Mapping ([plus haut](#dossiers-de-téléchargement-et-chemins)). |
| Les torrents terminés restent dans la file de l'application | Rookparcel continue de les partager : aucun objectif de partage n'est défini, l'objectif n'est pas encore atteint, ou **Limiter la vitesse d'envoi à (ko/s) : [0 : arrêter]** est activé. |

## Voir aussi

- [Interface Web](Interface-Web) : l'activer, HTTPS et la sécurité.
- [Utiliser rookparcel-nox](Utiliser-rookparcel-nox) : nox comme service qui démarre avec l'ordinateur.
- [Rookparcel avec Docker](Rookparcel-avec-Docker) : nox dans un conteneur.
- Les notes techniques sur ce que les applications appellent : [INTEGRATIONS.md, part 1](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/INTEGRATIONS.md#part-1-the-shim-as-an-arr-download-client).
