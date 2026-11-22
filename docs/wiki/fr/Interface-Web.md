# Interface Web

*Cette page est une traduction de la page anglaise [Web UI](Web-UI). En cas de différence, la page anglaise fait foi.*

L'interface Web est la fenêtre principale de Rookparcel dans un navigateur : la liste des torrents, les catégories, les onglets de détails, le menu contextuel, Ajouter, les Préférences et le Téléchargeur RSS. Cette page explique comment l'activer dans le programme Windows et dans `rookparcel-nox`, comment se connecter, HTTPS, ce que la page sait faire, et comment l'utiliser sur un téléphone.

Il y a deux façons de l'obtenir :

- **Depuis le programme Windows.** `rookparcel.exe` sert lui-même la page, à partir des torrents de la fenêtre, tant qu'il fonctionne (**Options > Préférences > Interface web**).
- **Depuis `rookparcel-nox`**, le Rookparcel sans fenêtre, lancé avec `--webui` ([Utiliser rookparcel-nox](Utiliser-rookparcel-nox)). C'est la voie à suivre sur les serveurs Linux, BSD et macOS, et sur un PC Windows qui doit continuer à télécharger sans fenêtre.

Le programme GTK (Linux, BSD) et l'application macOS ne servent pas d'interface Web de leur propre chef. Tous deux peuvent en revanche contrôler un `rookparcel-nox` grâce au [Contrôle à distance](Contrôle-à-distance).

## L'activer dans le programme Windows

![Préférences > Interface web](images/prefs-webui.png)

1. Ouvrez **Options > Préférences** et choisissez **Interface web**.
2. Cochez **Activer l'interface web**.
3. Sous **Authentification**, saisissez un **Nom d'utilisateur :** (`admin` est proposé) et un **Mot de passe :**. Les deux sont nécessaires ; sans eux, la page affiche « Veuillez saisir un nom d'utilisateur et un mot de passe pour l'interface web. » Le mot de passe n'est conservé que sous forme de hachage. Plus tard, laissez le champ vide pour garder celui qui est enregistré.
4. Sous **Connectivité**, vérifiez le **Port d'écoute :**. 8080 est proposé, ou le port libre suivant quand un autre programme ou un autre [profil](Profils) l'occupe.
5. Cliquez sur **Appliquer** ou **OK**. La ligne en bas indique « L'interface web est en cours d'exécution : ... » avec son adresse.
6. Cliquez sur **Ouvrir l'interface web** pour l'ouvrir dans votre navigateur.

Si l'interface Web ne peut pas démarrer (le port est utilisé, par exemple), la ligne d'état de la page en donne la raison. Les paramètres des autres pages sont appliqués malgré tout.

L'interface Web ne fonctionne que tant que Rookparcel fonctionne. Fermez la fenêtre vers la zone de notification pour la garder active ([Zone de notification et démarrage](Zone-de-notification-et-démarrage)). Elle redémarre quand vous appliquez des modifications sur cette page.

Les autres paramètres de la page :

| Paramètre | Ce qu'il fait |
|---|---|
| **Autoriser les connexions depuis d'autres ordinateurs** | Écoute sur toutes les adresses réseau, si bien que d'autres ordinateurs et des téléphones peuvent atteindre la page, pas seulement celui-ci. Ils ont aussi besoin d'une règle du Pare-feu Windows pour `rookparcel.exe` ([Connexion, ports et pare-feu](Connexion-ports-et-pare-feu)). |
| **Utiliser HTTPS (certificat autosigné)** | Sert la page en HTTPS avec un certificat que Rookparcel fabrique lui-même et conserve dans le dossier des paramètres. Les navigateurs avertissent à propos d'un tel certificat tant que vous ne l'avez pas accepté. |
| **Domaines du serveur (séparez plusieurs entrées par un point-virgule) :** | Les noms d'hôte auxquels la page répond en plus des adresses IP et de `localhost`, comme le nom de cet ordinateur. Les autres noms sont refusés. Voir [Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr#noms-dhôte-et-docker). |
| **Accepter une clé d'API (Authorization: Bearer)** et **Clé d'API (au moins 16 caractères) :** | Permet à des scripts et à des applications comme Sonarr ou Radarr d'utiliser l'interface Web avec cette clé au lieu de se connecter. Désactivé par défaut. La clé n'est conservée que sous forme de hachage et n'est plus jamais affichée ; laissez le champ vide pour la garder. |
| **Ignorer l'authentification pour les clients de ces sous-réseaux (...)** | Les clients de ces réseaux (comme `192.168.1.0/24`) utilisent l'interface Web sans se connecter, avec un contrôle total. N'indiquez que des réseaux auxquels vous faites autant confiance qu'au mot de passe. |

Ce qui change par rapport à la page de nox quand c'est la fenêtre qui la sert :

- Les torrents, étiquettes, tags, paramètres et flux RSS sont ceux de la fenêtre. Une modification faite dans le navigateur se voit dans la fenêtre, et inversement. Le Téléchargeur RSS de la page modifie les flux et les filtres de la fenêtre.
- Le planificateur, l'objectif de partage et le dossier de déplacement des téléchargements terminés de la fenêtre font le travail, comme sans l'interface Web.
- Il n'y a pas de limites de vitesse alternatives.
- Une fenêtre qui contrôle un nox distant ([Contrôle à distance](Contrôle-à-distance)) ne peut pas servir l'interface Web : « Une fenêtre distante ne peut pas servir l'interface web ; utilisez les Préférences de la fenêtre locale. »

## L'activer dans rookparcel-nox

Lancez nox avec `--webui` et un port :

```sh
rookparcel-nox run --save ~/Downloads --webui 8080
```

| Valeur de `--webui` | Qui peut atteindre la page |
|---|---|
| `8080` | Cet ordinateur seulement (`127.0.0.1:8080`). |
| `0.0.0.0:8080` | Tous les ordinateurs qui peuvent atteindre celui-ci. |
| `192.168.1.10:8080` | Les ordinateurs qui atteignent celui-ci à cette adresse. |

Le nom de connexion est `admin`, sauf si vous donnez `--webui-user`. Au premier démarrage, sans mot de passe enregistré, un mot de passe temporaire est affiché :

```text
* Web UI: no password set; temporary password for this run: ... (set one with --webui-pass)
```

Connectez-vous avec lui, puis définissez le vôtre dans **Préférences > Interface web** de la page (le champ **Mot de passe :**, avec le mot de passe temporaire comme **Mot de passe actuel :**). `--webui-pass` le définit aussi, mais un mot de passe sur la ligne de commande peut être vu par les autres utilisateurs de l'ordinateur.

Un nox qui fonctionne comme service écrit le mot de passe temporaire dans son journal ou dans le journal systemd : voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox#interface-web).

## Connexion

Ouvrez l'adresse dans un navigateur : `http://127.0.0.1:8080/` sur le même ordinateur, ou `http://<l'adresse de l'ordinateur>:8080/` depuis un autre (`https://` quand HTTPS est activé). Saisissez le **Nom d'utilisateur** et le **Mot de passe** et cliquez sur **Se connecter**. **Se déconnecter** se trouve à l'extrémité droite de la barre d'outils.

- Après cinq connexions erronées de suite depuis une même adresse, cette adresse est verrouillée pendant une heure. Les deux nombres se trouvent dans **Préférences > Interface web**.
- Une connexion prend fin après une heure sans requête de ce navigateur (**Expiration de la session (secondes) :**).
- Définir un nouveau mot de passe déconnecte tous les autres navigateurs et fenêtres distantes ; celui qui l'a changé reste connecté.
- Quand la session se termine (après l'expiration, ou **Se déconnecter**), la page ferme ses boîtes de dialogue ouvertes et affiche le formulaire de connexion. Un fichier `.torrent` qui attendait d'être ajouté reste en file d'attente et ouvre sa boîte de dialogue d'ajout après votre nouvelle connexion.
- Quand la connexion à Rookparcel est perdue, la barre d'état affiche « Déconnecté, nouvelle tentative dans ... s » et la page se reconnecte d'elle-même.

## HTTPS

Sans HTTPS, le mot de passe et tout ce que la page affiche transitent sur le réseau sans chiffrement. Sur votre propre ordinateur, cela n'a pas d'importance ; à travers un réseau, activez HTTPS.

- **Programme Windows :** cochez **Utiliser HTTPS (certificat autosigné)** dans **Préférences > Interface web**.
- **nox :** ajoutez `--webui-https`. nox fabrique un certificat à la première utilisation, le conserve dans son dossier de paramètres (`webui-cert.pem`, `webui-key.pem`) et le renouvelle un mois avant son expiration. À chaque démarrage, il affiche une ligne comme `* Web UI: certificate SHA-256 ...`.
- **nox avec votre propre certificat :** `--webui-cert FILE --webui-key FILE` (PEM ; la clé non chiffrée).

Le navigateur avertit à propos d'un certificat autosigné la première fois. Comparez l'empreinte que le navigateur affiche avec la ligne `certificate SHA-256` que nox a affichée, puis acceptez-le. Avec HTTPS activé, une adresse `http://` simple vers le même port est redirigée vers `https://`.

## Ce que la page sait faire

La page suit la fenêtre du bureau : mêmes colonnes, mêmes menus, mêmes mots et même ordre.

- **Barre d'outils :** **Ajouter un torrent**, **Ajouter une URL**, **Supprimer**, **Démarrer**, **Pause**, **Arrêter**, **Monter** et **Descendre** (ordre de la file), **RSS**, **Préférences** (<kbd>Ctrl</kbd>+<kbd>P</kbd>), la zone de recherche qui filtre la liste, **Rechercher**, et **Se déconnecter**.
- **Catégories** à gauche avec leurs compteurs, y compris les étiquettes, les tags et les flux RSS. Faites un clic droit sur la branche **Tags** (un appui long sur un téléphone) pour **Nouveau tag...**, ou sur un tag pour **Supprimer le tag** (*les tags sont nouveaux dans la version 1.2.63*).
- **La liste des torrents :** cliquez sur un en-tête pour trier, faites glisser les en-têtes pour réordonner ou redimensionner, faites un clic droit sur un en-tête pour choisir les colonnes (**Réinitialiser les colonnes** les remet en place).
- **Les onglets de détails** sous la liste : Général, Trackers, Pairs, Pièces, Fichiers, Vitesse et Journal ([Détails et fichiers des torrents](Détails-et-fichiers-des-torrents)).
- **Le menu contextuel** contient ce que contient le menu de la fenêtre : **Forcer le démarrage**, **Démarrer**, **Pause**, **Arrêter**, les déplacements dans la file, **Supprimer** et **Supprimer et > Effacer les données**, **Forcer la vérification** (elle démarre un torrent arrêté, comme dans le programme de bureau ; *nouveau dans la version 1.2.76*), **Mettre à jour le tracker**, **Copier le lien magnet**, **Exporter le .torrent...**, **Avancé > Définir l'emplacement de téléchargement...**, **Partage initial**, **Téléchargement séquentiel** et **Télécharger d'abord les premières et dernières pièces**, **Allocation de bande passante**, **Étiquette**, **Tags**, et **Propriétés** ([Gérer les torrents](Gérer-les-torrents)). **Ouvrir le dossier** est grisé : un navigateur ne peut pas ouvrir un dossier sur le serveur. *Copier le lien magnet, Exporter le .torrent et les deux éléments d'ordre de téléchargement sont nouveaux dans la version 1.2.63.* Exporter télécharge le fichier `.torrent` par le navigateur. Là où le navigateur ne donne pas de presse-papiers à la page (`http://` simple depuis un autre ordinateur), Copier le lien magnet affiche les liens pour que vous les copiiez à la main.
- **Ajouter des torrents :** **Ajouter un torrent** envoie des fichiers `.torrent` depuis l'ordinateur sur lequel le navigateur fonctionne. **Ajouter une URL** accepte un lien magnet, un info-hash ou un lien `http://` vers un torrent. Vous pouvez aussi déposer des fichiers `.torrent` ou un lien magnet sur la page, ou coller un lien magnet n'importe où en dehors d'une zone de texte. Quand « Afficher une fenêtre avec les fichiers du torrent » est activé (**Préférences > Interface**, activé par défaut), une boîte de dialogue d'ajout demande la même chose que celle du bureau : dossier d'enregistrement, nom, étiquette, quels fichiers, démarrer ou non. Pour un lien magnet, elle s'ouvre une fois que la liste des fichiers du torrent est arrivée. Les fichiers correspondant à **Ne pas télécharger les fichiers nommés :** (**Préférences > Interface**) y sont décochés au départ, et les torrents ajoutés sans cette boîte les laissent de côté (*nouveau dans la version 1.2.63*). Voir [Ajouter des torrents](Ajouter-des-torrents#fichiers-exclus-par-leur-nom).
- **Propriétés du torrent :** trackers, limites de vitesse, places d'envoi, objectif de partage et ordre de téléchargement des torrents sélectionnés.
- **Préférences :** les pages Général, Interface, Dossiers, Connexion, Bande passante, BitTorrent, File d'attente, Planificateur, Interface web, Avancé, Cache disque et E/S disque. Survolez un paramètre pour voir son aide, ou appuyez sur <kbd>F1</kbd> (*les textes d'aide sont nouveaux dans la version 1.1.57*). Les dossiers sont des dossiers du serveur, saisis sous forme de texte. L'interface Web que sert le programme Windows n'a, sur sa page BitTorrent, ni adresse de filtre IP, ni **Mettre à jour maintenant**, ni ligne d'état : les Préférences propres à la fenêtre les contiennent. Voir [Référence des préférences](Référence-des-préférences).
- **Téléchargeur RSS :** flux, filtres et historique, comme dans le programme de bureau ([Téléchargeur RSS](Téléchargeur-RSS)).
- **Rechercher** ouvre Rechercher des torrents, qui interroge votre Prowlarr ou Jackett ; **Gérer les sources...** y permet de les ajouter ([Rechercher des torrents](Rechercher-des-torrents)). Ajouter une source, ou changer son adresse ou sa clé, demande le mot de passe actuel de l'interface Web (comme l'installation de l'assistant de recherche) ; un mauvais mot de passe déconnecte ce navigateur et compte comme une connexion échouée.

Touches dans la liste : les flèches, <kbd>Home</kbd>, <kbd>End</kbd>, <kbd>Page Up</kbd>, <kbd>Page Down</kbd> déplacent ; <kbd>Shift</kbd> et <kbd>Ctrl</kbd> avec un clic ou une touche étendent la sélection ; <kbd>Ctrl</kbd>+<kbd>A</kbd> sélectionne tous les torrents que le filtre affiche ; <kbd>Space</kbd> met en pause ; <kbd>Delete</kbd> supprime ; <kbd>Shift</kbd>+<kbd>Delete</kbd> supprime et efface les données ; la touche <kbd>Menu</kbd> ou <kbd>Shift</kbd>+<kbd>F10</kbd> ouvre le menu contextuel.

La page utilise la langue réglée dans **Préférences > Général**, « Interface ». Par défaut, elle passe en sombre quand votre navigateur ou votre système est réglé sur un thème sombre. **Préférences > Interface > Thème** choisit un thème pour tous ceux qui ouvrent cette interface Web : **Système (clair ou sombre)**, **Clair**, **Sombre**, **Minuit**, **Ardoise**, **Solarized sombre**, **Solarized clair**, **Contraste élevé** ou **Freux postier**. **Thème dans ce navigateur** sur la même page (sur un téléphone, **Thème** dans le menu Plus) en choisit un pour ce navigateur seulement, conservé sur l'appareil ; **Par défaut du serveur** suit de nouveau le choix du serveur. Votre propre thème est un fichier CSS déposé dans le dossier `webui/themes` sous le dossier des paramètres (`--config-dir` pour `rookparcel-nox` ; le dossier des paramètres du programme Windows quand il sert l'interface Web) : copiez l'un des fichiers intégrés depuis l'arborescence des sources ([crates/nox/src/webui/themes](https://github.com/Balls0fSteel/Rookparcel/tree/main/crates/nox/src/webui/themes)), changez les couleurs et la ligne `/* name: ... */`, nommez le fichier avec des lettres minuscules, des chiffres et des tirets, et il est listé sous ce nom.

## Sur un téléphone

La même adresse fonctionne sur un téléphone ; la page change sa mise en page sur un écran étroit (moins de 768 pixels de large).

- La liste affiche le nom de chaque torrent au-dessus d'une barre de progression avec son état, ses vitesses et le temps restant.
- Les catégories se trouvent dans un tiroir derrière le bouton de menu au début de la barre d'outils.
- Touchez un torrent pour voir ses détails dans une feuille qui monte du bas.
- Le bouton de chaque ligne ouvre son menu. Un appui long commence la sélection de plusieurs torrents ; touchez-en d'autres pour les ajouter, et **Terminé** met fin à la sélection.
- La barre d'outils garde **Ajouter**, **Démarrer**, **Pause** et **Supprimer** ; le reste, et le tri, se trouvent sous **Plus**.
- Sous Android, le bouton Retour ferme le tiroir, la feuille ou la boîte de dialogue la plus haute.

Pour que le téléphone atteigne la page, l'interface Web doit accepter les connexions d'autres ordinateurs (**Autoriser les connexions depuis d'autres ordinateurs**, ou `--webui 0.0.0.0:PORT` de nox), et le téléphone doit être sur un réseau qui atteint l'ordinateur.

## Sécurité

L'interface Web contrôle vos torrents et peut écrire des fichiers partout où le compte de Rookparcel le peut. Gardez ceci à l'esprit avant de l'ouvrir à un réseau :

- Utilisez un mot de passe long, et activez HTTPS pour tout ce qui dépasse votre propre ordinateur.
- **Activer la validation de l'en-tête Host** (activée par défaut) refuse les requêtes adressées à un nom autre qu'une adresse IP, `localhost` ou un domaine de serveur listé. **Activer la protection contre la falsification de requête intersite (CSRF)** (activée par défaut) refuse les modifications envoyées depuis la page d'un autre site web. Laissez les deux activées.
- **Pas d'authentification pour les clients sur localhost** (page de nox ; désactivé par défaut) laisse entrer sans connexion les programmes du même ordinateur. Cette option et la liste de sous-réseaux ne s'appliquent jamais aux requêtes qui passent par un proxy inverse.
- Changer l'identifiant, la clé d'API, les sous-réseaux ou les autres paramètres de sécurité dans la page demande le **Mot de passe actuel :**. Un mauvais mot de passe déconnecte ce navigateur et compte comme une connexion échouée ; des essais répétés verrouillent donc l'adresse comme le font des connexions échouées. Il en va de même pour les adresses depuis lesquelles le serveur télécharge, comme l'adresse de la liste de trackers et l'adresse de la liste de blocage du filtre IP (Préférences > BitTorrent, *nouveau dans la version 1.2.63* ; voir [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu#une-liste-depuis-une-adresse-web)).
- Avec nox, `--webui-save-root DIR` limite les dossiers dans lesquels l'interface Web peut enregistrer, déplacer et lire ([Utiliser rookparcel-nox](Utiliser-rookparcel-nox#interface-web)).

Derrière un proxy inverse sur le même ordinateur, faites en sorte que le proxy définisse l'en-tête `X-Forwarded-For`, afin que les connexions échouées verrouillent le bon client et pas tout le monde. Avec **Accepter une clé d'API (Authorization: Bearer)** activé, le proxy ne doit pas ajouter d'en-tête `Authorization` de lui-même : avant qu'un navigateur se soit connecté, un tel en-tête compte comme une mauvaise clé.

## Voir aussi

- [Contrôle à distance](Contrôle-à-distance) : la fenêtre du programme de bureau qui pilote un nox, comme alternative à la page.
- [Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr) : utiliser l'API compatible qBittorrent de l'interface Web.
- La conception derrière la page : [WEBUI.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/WEBUI.md).
