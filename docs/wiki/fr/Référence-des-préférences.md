*Cette page est la traduction de la page anglaise [Preferences reference](https://github.com/Balls0fSteel/Rookparcel/wiki/Preferences-reference). En cas de différence, la page anglaise fait foi.*

# Référence des préférences

Cette page liste chaque page de la fenêtre Préférences et ce que fait chaque réglage qui s'y trouve, dans l'ordre où la fenêtre les montre. Chaque section renvoie à la page qui explique le sujet en détail.

## Ouvrir les Préférences

Ouvrez **Options > Préférences**, appuyez sur <kbd>Ctrl</kbd>+<kbd>P</kbd>, ou cliquez sur **Préférences** dans la barre d'outils. Les pages sont listées à gauche ; **Cache disque**, **E/S disque**, **Trackers**, **Exécuter un programme** et **Verrouillage de l'interface** se trouvent sous **Avancé**. **OK** enregistre et ferme, **Appliquer** enregistre et laisse la fenêtre ouverte, **Annuler** oublie les changements.

L'aide est intégrée :

- Pointez un réglage et une infobulle l'explique. Le volet sous les pages montre le même texte pour le réglage sous le pointeur, ou à défaut pour celui qui a le focus du clavier.
- Appuyez sur <kbd>F1</kbd> et l'aide du réglage qui a le focus apparaît à côté de lui.

*Les textes d'aide, les infobulles et F1 sont nouveaux dans la 1.1.57.*

Les textes ci-dessous sont des versions courtes de cette aide intégrée. Les vitesses sur les pages sont en ko/s ; 0 signifie illimité, sauf si le libellé dit autre chose.

## Général

![Préférences, page Général](images/prefs-general.png)

| Réglage | Ce qu'il fait |
|---|---|
| **Interface :** (Langue) | La langue des menus, des boîtes de dialogue et de l'interface web. Le premier choix suit la langue du système. Les fenêtres de bureau changent à leur prochain démarrage. |
| **Associer aux fichiers .torrent** | Fait de cette copie de Rookparcel le programme qui ouvre les fichiers `.torrent`, pour votre compte utilisateur seulement. Un programme choisi dans les réglages d'applications par défaut du système l'emporte sur ce réglage. |
| **Associer aux liens magnet:** | Fait de cette copie de Rookparcel le programme qui ouvre les liens `magnet:` cliqués dans un navigateur, pour votre compte utilisateur seulement. Avec les profils, un seul profil peut les détenir. |
| **Vérifier l'association au démarrage** | Au démarrage, demande s'il faut reprendre les fichiers `.torrent` et les liens `magnet:` lorsque Rookparcel n'est pas le programme qui les ouvre. Désactivé par défaut. |
| **Lancer Rookparcel au démarrage du système** | Lance Rookparcel à l'ouverture de session. Chaque profil a sa propre entrée. |
| **Démarrer réduit** | Avec le lancement automatique, Rookparcel démarre sans afficher sa fenêtre, dans la zone de notification là où il y en a une. |
| **Applications par défaut...** (Windows seulement) | Ouvre les Paramètres de Windows sur Applications par défaut. Un programme choisi à cet endroit l'emporte sur les cases ci-dessus, et vous seul pouvez changer ce choix. |
| **Rechercher automatiquement les mises à jour** (Confidentialité) | Environ dix secondes après le démarrage, puis toutes les 12 heures, demande au site web du projet si une version plus récente est sortie. Rien n'est téléchargé ni installé sans demander. **Aide > Rechercher des mises à jour** fonctionne dans les deux cas. |
| **Mettre à jour vers les versions bêta** (Confidentialité) | Propose aussi les versions préliminaires (bêta) lors de la recherche de mises à jour. |
| **Ajouter .!ut aux fichiers incomplets** (Pendant le téléchargement) | Un fichier encore en cours de téléchargement est stocké avec `.!ut` après son nom (`film.mkv.!ut`) et reprend son nom normal dès qu'il est entièrement téléchargé et a passé sa vérification. L'activer ou le désactiver renomme aussitôt les fichiers des torrents en cours, et ceux des torrents arrêtés quand ils démarrent. Désactivé par défaut. |

Sous Windows, les cases d'intégration se trouvent sous **Intégration à Windows** ; dans la version GTK, le groupe s'appelle **Intégration au bureau**.

*Ajouter .!ut aux fichiers incomplets est nouveau dans la 1.2.63.* Les fichiers laissés en `.!ut` par µTorrent ou en `.!qB` par qBittorrent sont pris en compte que la case soit cochée ou non. Un fichier qu'un autre programme tient ouvert garde son nom jusqu'à ce qu'il puisse être renommé. Un fichier dont le nom avec `.!ut` est celui d'un autre fichier du même torrent garde son nom simple, et Supprimer et > Effacer les données n'efface un fichier `.!ut` ou `.!qB` que lorsqu'il contient les données de ce torrent.

Voir aussi : [Zone de notification, notifications et démarrage](Zone-de-notification-et-démarrage), [Ajouter des torrents](Ajouter-des-torrents), [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel), [Profils](Profils), [Disque et stockage](Disque-et-stockage#fichiers-incomplets-et-ut).

## Interface

| Réglage | Ce qu'il fait |
|---|---|
| **Confirmer la suppression des torrents** | Demande avant de retirer des torrents de la liste, en disant si leurs fichiers sont aussi effacés. Désactivé, **Supprimer** agit aussitôt. |
| **Thème** (Windows) | Les couleurs de la fenêtre : **Système (clair ou sombre)** suit *Choisir votre mode* de Windows pour les applications, **Clair** et **Sombre** les fixent. Le Contraste élevé de Windows garde ses propres couleurs. Le mode sombre demande Windows 10 version 1809 ou plus récent ; un Windows plus ancien le dit sous la liste et reste clair ([Mode sombre](Mode-sombre)). |
| **Thème** (interface web seulement) | Les couleurs de la page de l'interface web pour tous ceux qui l'ouvrent : Système (clair ou sombre), Clair, Sombre, Minuit, Ardoise, Solarized sombre, Solarized clair, Contraste élevé, Freux postier, et vos propres fichiers CSS du dossier `webui/themes` sous le dossier des paramètres ([Interface web](Interface-Web)). |
| **Thème dans ce navigateur** (interface web seulement) | Un thème pour ce navigateur seulement, conservé sur cet appareil et utilisé à la place du choix du serveur. **Par défaut du serveur** suit de nouveau le thème du serveur. |
| **Alterner la couleur de fond des listes** | Ombre une ligne sur deux des listes. |
| **Afficher la vitesse actuelle dans la barre de titre** | Place les vitesses de réception et d'envoi actuelles au début du titre de la fenêtre principale, ce qui s'affiche aussi sur la barre des tâches. |
| **Afficher les bulles de notification** | Affiche une notification, avec le nom du torrent, quand un téléchargement se termine. |
| **Fermer dans la zone de notification** (Zone de notification) | Le bouton de fermeture de la fenêtre masque Rookparcel dans la zone de notification au lieu de le quitter, de sorte que les transferts continuent. Quittez depuis le menu ou depuis le menu de l'icône de la zone de notification. |
| **Réduire dans la zone de notification** (Zone de notification) | Réduire la fenêtre la masque dans la zone de notification au lieu de la barre des tâches. Cliquez sur l'icône de la zone de notification pour la faire revenir. |
| **Ne pas démarrer le téléchargement automatiquement** (À l'ajout de torrents) | Les nouveaux torrents sont ajoutés à l'arrêt et en dehors de la file, de sorte que rien n'est téléchargé tant que vous ne les démarrez pas. |
| **Activer la fenêtre du programme** | Amène la fenêtre principale au premier plan quand un autre programme, tel qu'un navigateur, confie un torrent à Rookparcel. Désactivé, seule la fenêtre Ajouter un nouveau torrent apparaît. |
| **Afficher une fenêtre avec les fichiers du torrent** | Ouvre la fenêtre Ajouter un nouveau torrent pour chaque torrent que vous ajoutez, afin de choisir d'abord le dossier et les fichiers ; pour un lien magnet, une fois que sa liste de fichiers est arrivée. Désactivé, les torrents sont ajoutés aussitôt avec les valeurs par défaut. |
| **Ne pas télécharger les fichiers nommés :** (À l'ajout de torrents) | Les fichiers dont le nom correspond à l'un des motifs de la zone ci-dessous démarrent décochés, donc ne sont pas téléchargés, dans les torrents que vous ajoutez à partir de maintenant. Séparez les motifs par des points-virgules ; `*` remplace n'importe quels caractères et `?` un seul, et la casse ne compte pas, comme dans `*.lnk; Thumbs.db; *sample*`. Seul le nom propre du fichier compte, pas ses dossiers. Désactivé par défaut. |

*Ne pas télécharger les fichiers nommés est nouveau dans la 1.2.63.* Vous pouvez toujours cocher un tel fichier dans la fenêtre Ajouter un nouveau torrent, ou plus tard dans l'onglet Fichiers. Un torrent dont tous les fichiers correspondent est laissé tel quel. La liste s'applique aussi aux torrents ajoutés sans la fenêtre Ajouter un nouveau torrent, par exemple depuis le dossier surveillé, le RSS ou Sonarr et Radarr, mais pas aux torrents déjà présents dans la liste. Voir [Ajouter des torrents](Ajouter-des-torrents#fichiers-exclus-par-leur-nom).

Voir aussi : [Zone de notification, notifications et démarrage](Zone-de-notification-et-démarrage), [Ajouter des torrents](Ajouter-des-torrents), [Gérer les torrents](Gérer-les-torrents), [La fenêtre principale](La-fenêtre-principale).

## Dossiers

| Réglage | Ce qu'il fait |
|---|---|
| **Placer les nouveaux téléchargements dans :** | Le dossier où les nouveaux torrents sont enregistrés, sauf si vous en choisissez un autre à l'ajout. |
| **Déplacer les téléchargements terminés vers :** | Quand un torrent finit de se télécharger, ses fichiers sont déplacés dans ce dossier et il continue d'être partagé à partir de là. S'applique aux torrents qui se terminent à partir de maintenant. |
| **Ajouter l'étiquette du torrent** | Déplace un torrent terminé qui a une étiquette dans un dossier nommé d'après l'étiquette, à l'intérieur du dossier des téléchargements terminés. |
| **Ne déplacer que depuis le dossier de téléchargement par défaut** | Ne déplace que les torrents enregistrés dans le dossier des nouveaux téléchargements ; les torrents que vous avez enregistrés ailleurs restent où ils sont. |
| **Stocker les .torrent dans :** | Garde dans ce dossier une copie de chaque fichier `.torrent` ajouté, nommée d'après le torrent. La copie d'un lien magnet est stockée une fois ses métadonnées arrivées. |
| **Déplacer les .torrent des tâches terminées vers :** | Quand un torrent finit de se télécharger, sa copie `.torrent` stockée est déplacée dans ce dossier. |
| **Charger automatiquement les .torrent depuis :** | Surveille ce dossier et ajoute chaque nouveau fichier `.torrent` qu'on y dépose, sans la fenêtre Ajouter un nouveau torrent. Un fichier chargé est renommé en `.torrent.loaded`. |
| **Effacer les .torrent chargés** | Efface un fichier `.torrent` chargé automatiquement dès que son torrent est ajouté, au lieu de le renommer. |

Voir aussi : [Disque et stockage](Disque-et-stockage), [Ajouter des torrents](Ajouter-des-torrents), [Mise en file et partage](Mise-en-file-et-partage).

## Connexion

![Préférences, page Connexion](images/prefs-connection.png)

| Réglage | Ce qu'il fait |
|---|---|
| **Port utilisé pour les connexions entrantes :** | Le port auquel les autres pairs se connectent. Autorisez-le dans le pare-feu et redirigez-le dans le routeur, ou laissez UPnP ou NAT-PMP le faire. Un changement s'applique aussitôt. |
| **Port aléatoire** (bouton) | Remplit un port aléatoire de 10000 à 60000. Il est utilisé dès que vous appliquez. |
| **Port aléatoire à chaque démarrage** | Choisit un nouveau port aléatoire à chaque démarrage. Un port redirigé à la main dans le routeur ne correspond alors plus. |
| **Activer la redirection de port UPnP** | Demande à un routeur UPnP de rediriger le port d'écoute vers cet ordinateur. Désactivez-le quand vous redirigez le port vous-même ou quand le routeur se comporte mal. |
| **Activer la redirection de port NAT-PMP** | Demande au routeur de rediriger le port par NAT-PMP ou PCP, que beaucoup de routeurs proposent en plus d'UPnP ou à sa place. |
| **Type :** (Serveur proxy) | Le genre de proxy : **Socks4**, **Socks5** ou **HTTP**. **(aucun)** se connecte directement. |
| **Proxy :** et **Port :** | Le nom d'hôte ou l'adresse IP du serveur proxy, et le port sur lequel il écoute (comme 1080 pour de nombreux serveurs SOCKS). |
| **Authentification**, **Nom d'utilisateur :**, **Mot de passe :** | Se connecte au proxy. Un proxy SOCKS4 ne reçoit que le nom d'utilisateur. |
| **Résoudre les noms d'hôte par le proxy** | Laisse le proxy chercher les noms des trackers et des sources web, pour que votre propre serveur DNS ne les voie pas. |
| **Utiliser le proxy pour les connexions entre pairs** | Envoie les connexions vers les pairs et les sources web par le proxy. Les connexions entrantes sont alors refusées, et la redirection de port et la découverte de pairs locaux restent désactivées ; uTP et le DHT demandent un proxy SOCKS5. |
| **Utiliser le proxy pour les connexions aux trackers** | Envoie les annonces aux trackers par le proxy, pour que les trackers voient l'adresse du proxy plutôt que la vôtre. |
| **Réseau :** (Utiliser uniquement ce réseau) | Garde tout le trafic des torrents sur un seul adaptateur réseau : **N'importe quel adaptateur VPN**, un adaptateur par son nom, ou **Plage d'adresses (ci-dessous)**. **N'importe quel réseau (désactivé)** désactive cette fonction. L'interface web et les connexions distantes ne sont pas liées. |
| **Plage d'adresses :** | Une plage telle que `10.2.0.0/16` ou `fd00::/8`, pour le choix **Plage d'adresses** : l'adaptateur qui détient une adresse dans cette plage porte le trafic des torrents. |
| **Mettre les torrents en pause tant que ce réseau est absent** | Tant qu'aucun adaptateur ne correspond (le VPN est coupé), les torrents en cours attendent et affichent « En attente du réseau », puis reprennent d'eux-mêmes. |

*Le groupe Utiliser uniquement ce réseau est nouveau dans la 1.1.58.*

Voir aussi : [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu).

## Bande passante

![Préférences, page Bande passante](images/prefs-bandwidth.png)

| Réglage | Ce qu'il fait |
|---|---|
| **Vitesse d'envoi maximale (ko/s)** | Le maximum que tous les torrents réunis envoient par seconde. 0 signifie illimité. Les heures Limité du planificateur et Garder un ping bas peuvent l'abaisser pour un temps. |
| **Vitesse de réception maximale (ko/s)** | La même chose pour les téléchargements. |
| **Appliquer la limitation de vitesse au surdébit de transport** | Compte les en-têtes TCP/IP du trafic des pairs dans les limites, pour qu'elles correspondent à ce que transporte la ligne. Désactivé par défaut. |
| **Appliquer la limitation de vitesse aux connexions uTP** | Applique aussi les limites aux pairs uTP. Désactivé, les pairs uTP ne sont ralentis que par le contrôle de délai propre à uTP. Activé par défaut. |
| **Garder un ping bas (limiter les vitesses automatiquement)** | Mesure le délai sur votre ligne et plafonne automatiquement les envois et les téléchargements tant qu'il augmente, pour les jeux et les appels. Il n'élève jamais vos propres limites ; la barre d'état montre le plafond sous la forme « auto ». |
| **Nombre maximum global de connexions :** | Le maximum de connexions de pairs ouvertes en même temps pour tous les torrents. Abaissez-le si le routeur peine. |
| **Nombre maximum de pairs connectés par torrent** | Le maximum de pairs auxquels un torrent se connecte ; 0 signifie sans limite. S'applique aux torrents ajoutés à partir de maintenant. |
| **Nombre d'emplacements d'envoi par torrent** | Le nombre de pairs auxquels un torrent envoie en même temps ; 0 signifie sans limite. S'applique aux torrents ajoutés à partir de maintenant. |
| **Nombre global d'emplacements d'envoi** | Le nombre de pairs auxquels tous les torrents réunis envoient en même temps ; -1 signifie sans limite. Moins d'emplacements donnent plus de vitesse à chaque pair. |

Voir aussi : [Limites de vitesse et planificateur](Limites-de-vitesse-et-planificateur), [Garder un ping bas](Garder-un-ping-bas).

## BitTorrent

![Préférences, page BitTorrent](images/prefs-bittorrent.png)

| Réglage | Ce qu'il fait |
|---|---|
| **Activer le réseau DHT** | Trouve des pairs par le DHT, sans tracker ; la plupart des liens magnet en ont besoin. Les torrents privés ne l'utilisent jamais. |
| **Activer la découverte de pairs locaux** | Trouve des pairs pour les mêmes torrents sur votre réseau local. Les torrents privés ne l'utilisent jamais. |
| **Activer l'échange de pairs** | Permet aux pairs connectés de se parler d'autres pairs. Les torrents privés ne l'utilisent jamais. |
| **Activer uTP (transport UDP avec gestion de la bande passante)** | Se connecte aux pairs et en accepte par uTP en plus de TCP. uTP ralentit quand le réseau est chargé. |
| **Sortant :** (Chiffrement du protocole) | **Désactivé** : ne jamais chiffrer. **Activé** : les tentatives en clair et chiffrées alternent pour chaque pair, en gardant ce qui a fonctionné. **Forcé** : chiffré seulement, ce qui fait perdre les pairs qui ne peuvent pas chiffrer. |
| **Autoriser les connexions entrantes non chiffrées** | Accepte les connexions entrantes qui ne sont pas chiffrées. Désactivé, seules les connexions chiffrées sont acceptées. |
| **Activer le filtre IP** | Bloque les pairs dont les adresses figurent dans la liste de blocage, au format `ipfilter.dat` d'eMule ou P2P de PeerGuardian, ou en lignes CIDR telles que `10.0.0.0/8`. Un chemin vide désigne `ipfilter.dat` dans le dossier des paramètres. |
| **Recharger** | Charge la liste de blocage maintenant et montre combien de règles elle contient. Le filtre IP doit être activé. |
| **Obtenir la liste de blocage depuis cette adresse (https) :** | Prend la liste de blocage à l'adresse `https://` ci-dessous au lieu du fichier ci-dessus, par le proxy quand il y en a un. La copie téléchargée en dernier depuis cette adresse est utilisée aussitôt au démarrage, et la liste est téléchargée de nouveau une fois par jour. Tant que la case est cochée, le fichier et **Recharger** sont grisés. |
| **Mettre à jour maintenant** | Télécharge la liste de blocage depuis l'adresse maintenant. La ligne en dessous montre combien de règles elle contient et quand elle a été téléchargée, ou pourquoi le dernier téléchargement a échoué. |

*La liste de blocage depuis une adresse est nouvelle dans la 1.2.63.* La liste peut être au format `ipfilter.dat`, P2P de PeerGuardian ou CIDR (`10.0.0.0/8`), brute ou compressée en gzip ; les archives zip ne sont pas prises en charge. Quand un téléchargement échoue ou n'est pas une telle liste, les règles en cours d'utilisation restent. Une nouvelle adresse garde la copie de l'ancienne jusqu'à ce qu'une liste de la nouvelle ait été téléchargée, de sorte qu'un premier téléchargement raté ne fait rien perdre ; dans le programme Windows, **Mettre à jour maintenant** avec une adresse pas encore appliquée ne fait que l'essayer jusqu'à **OK** ou **Appliquer**.

Voir aussi : [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu#filtre-ip).

## File d'attente

![Préférences, page File d'attente](images/prefs-queueing.png)

| Réglage | Ce qu'il fait |
|---|---|
| **Nombre maximum de torrents actifs (envoi ou réception)** | Le maximum de torrents en file qui fonctionnent en même temps ; les autres attendent. -1 signifie sans limite. Les torrents forcés fonctionnent en dehors de la file. |
| **Nombre maximum de téléchargements actifs** | Le maximum de torrents en file qui se téléchargent en même temps. -1 signifie sans limite. |
| **Nombre maximum de partages actifs** | Le maximum de torrents en file terminés qui sont partagés en même temps. -1 signifie sans limite. |
| **Ne pas compter les torrents lents dans ces limites** | Un torrent qui transfère moins d'environ 2 ko/s pendant une minute cesse de prendre un emplacement, de sorte qu'un autre peut démarrer. |
| **Ratio minimum (%)** (Objectif de partage) | Envoyer ce pourcentage de ce qui a été reçu ; 150 signifie une fois et demie. 0 signifie pas d'objectif de ratio. Sans effet tant que la case ci-dessous n'est pas cochée. |
| **Durée de partage minimum (minutes)** | Partager pendant ce nombre de minutes. 0 signifie pas d'objectif de durée. Quand les deux objectifs sont définis, les deux doivent être atteints. Sans effet tant que la case ci-dessous n'est pas cochée. |
| **Limiter la vitesse d'envoi à (ko/s) : [0 : arrêter]** | Met l'objectif de partage en vigueur. Cochée, un torrent qui a atteint l'objectif est arrêté (0) ou continue d'être partagé avec sa vitesse d'envoi plafonnée au nombre indiqué. Désactivée, comme au début, les torrents continuent d'être partagés au-delà de l'objectif. |

Voir aussi : [Mise en file et partage](Mise-en-file-et-partage).

## Planificateur

![Préférences, page Planificateur](images/prefs-scheduler.png)

| Réglage | Ce qu'il fait |
|---|---|
| **Activer le planificateur** | Suit le tableau hebdomadaire heure par heure, en heure locale. Désactivé, Rookparcel fonctionne toujours à pleine vitesse. |
| **Tableau du planificateur** | Une cellule par heure de la semaine ; cliquez ou faites glisser pour passer cycliquement par **Pleine vitesse**, **Limité**, **Arrêt** et **Partage seul**. |
| **Vitesse d'envoi limitée (ko/s)** et **Vitesse de réception limitée (ko/s)** | Les limites pendant les heures Limité, à la place des limites normales. 0 signifie illimité. |
| **Désactiver le DHT à l'arrêt** | Désactive aussi le DHT pendant les heures Arrêt. Activé par défaut. |

Voir aussi : [Limites de vitesse et planificateur](Limites-de-vitesse-et-planificateur).

## Interface web

![Préférences, page Interface web](images/prefs-webui.png)

Cette page n'existe que dans la version Windows. Elle permet à un navigateur de contrôler les torrents de cette fenêtre.

| Réglage | Ce qu'il fait |
|---|---|
| **Activer l'interface web** | Sert l'interface web depuis cette fenêtre tant que Rookparcel fonctionne. Elle demande un nom d'utilisateur et un mot de passe. |
| **Nom d'utilisateur :** et **Mot de passe :** | L'identifiant. Le mot de passe n'est stocké que sous forme de hachage ; laissez-le vide pour garder celui qui est stocké. Un nouveau mot de passe déconnecte tous les autres navigateurs. |
| **Accepter une clé d'API (Authorization: Bearer)** | Permet à des applications telles que Sonarr ou Radarr d'utiliser l'interface web avec la clé ci-dessous, sans se connecter. Désactivé par défaut. |
| **Clé d'API (au moins 16 caractères) :** | La clé que les applications envoient. Stockée seulement sous forme de hachage et jamais montrée de nouveau. |
| **Ignorer l'authentification pour les clients de ces sous-réseaux** | Les clients de ces réseaux utilisent l'interface web sans se connecter. Ne listez que des réseaux auxquels vous faites autant confiance qu'au mot de passe. |
| **Port d'écoute :** | Le port sur lequel l'interface web écoute ; 8080 est proposé, ou le port libre suivant. |
| **Autoriser les connexions depuis d'autres ordinateurs** | Écoute sur chaque adresse réseau, pour que d'autres ordinateurs et des téléphones puissent la joindre. Ils ont aussi besoin d'une règle du Pare-feu Windows pour `rookparcel.exe`. |
| **Utiliser HTTPS (certificat autosigné)** | Sert l'interface web en HTTPS avec un certificat que Rookparcel fabrique lui-même. Les navigateurs l'avertissent tant que vous ne l'avez pas accepté. |
| **Domaines du serveur** | Les noms d'hôte auxquels l'interface web répond en plus des adresses IP et de localhost, tels que le nom de cet ordinateur pour une application dans Docker. |
| **Ouvrir l'interface web** | Ouvre dans votre navigateur l'interface web en cours d'exécution. |

Voir aussi : [Interface web](Interface-Web), [Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr).

## Avancé

![Préférences, page Avancé](images/prefs-advanced.png)

La page Avancé est un tableau des réglages propres au moteur, sous leurs noms de libtorrent. Tapez une partie d'un nom dans **Filtre :** pour réduire la liste. Sélectionnez une ligne, tapez une nouvelle valeur dans **Valeur :** (ou choisissez **Vrai** ou **Faux** pour un interrupteur), puis cliquez sur **Définir**. **Réinitialiser** remet le réglage sélectionné à sa valeur par défaut. Un `*` marque une valeur qui diffère de la valeur par défaut. Comme partout dans les Préférences, le changement prend effet avec **OK** ou **Appliquer**. Les vitesses sont ici en octets par seconde, pas en ko/s.

La plupart des lignes sont les mêmes réglages que les autres pages montrent sous des noms plus parlants. Les autres se répartissent dans ces groupes :

| Groupe | Exemples |
|---|---|
| Identité | `user_agent`, `peer_fingerprint` (ont rarement besoin d'être changés) |
| Écoute et redirection de port | `listen_interfaces`, `max_retry_port_bind`, `upnp_lease_duration`, `natpmp_gateway` |
| File d'attente | `active_dht_limit`, `active_tracker_limit`, `inactive_down_rate`, `auto_manage_interval` |
| Connexions et pairs | `connection_speed`, `max_peerlist_size`, `peer_timeout`, `peer_turnover` |
| DHT | `dht_announce_interval`, `dht_upload_rate_limit`, `dht_bootstrap_nodes` et ses interrupteurs de protection |
| uTP et TCP | `enable_incoming_tcp`, `utp_target_delay`, `mixed_mode_algorithm`, `bt.transp_disposition` |
| Chiffrement | `out_enc_policy`, `in_enc_policy`, `allowed_enc_level`, `prefer_rc4` |
| Trackers | `announce_to_all_tiers`, `announce_to_all_trackers`, `num_want`, `stop_tracker_timeout` |
| Disque | `cache_size`, `max_queued_disk_bytes`, `file_pool_size`, `drive_overrides` |
| Sources web | `max_web_seed_connections`, `urlseed_timeout`, `ssrf_mitigation` |
| Proxy | `proxy_type`, `proxy_send_host_in_connect`, `socks5_udp_send_local_ep` |
| Garder un ping bas | `auto_limit_delay_ms`, `auto_limit_probe_hosts` |

Lignes souvent utiles à connaître :

| Réglage | Ce qu'il fait |
|---|---|
| `listen_interfaces` | Adresses et ports, séparés par des virgules, sur lesquels accepter des pairs. `0.0.0.0` et `[::]` désignent toutes les interfaces. |
| `connection_speed` | Nouvelles tentatives de connexion sortante par seconde, partagées par tous les torrents. Plus élevé, il trouve des pairs plus vite mais peut surcharger des routeurs faibles. 30 par défaut. |
| `announce_to_all_trackers` | Activé : tous les trackers d'un niveau sont contactés, pas seulement le premier qui fonctionne. Peut trouver plus de pairs au prix de plus de trafic vers les trackers. Désactivé par défaut. |
| `upnp_lease_duration` | Nombre de secondes pour lesquelles une redirection de port UPnP est demandée. 0 demande une redirection permanente, dont certains routeurs ont besoin. 3600 par défaut. |
| `stop_tracker_timeout` | Nombre de secondes que Rookparcel attend à la sortie pour les trackers et les routeurs. 5 par défaut. Abaissez-le pour quitter plus vite. |
| `auto_limit_delay_ms` | Millisecondes de ping supplémentaire que Garder un ping bas autorise au-dessus du ping au repos de la ligne. Plus bas, les jeux sont plus fluides ; plus haut, la vitesse peut être plus grande. 15 par défaut. |
| `drive_overrides` | Corrige les lecteurs dont le type est mal détecté, sous forme de paires chemin=type telles que `D:/=hdd, /mnt/nas=network`. |
| `bt.transp_disposition` | Types de connexion autorisés, sous forme de bits additionnés : 1 TCP sortant, 2 uTP sortant, 4 TCP entrant, 8 uTP entrant. 15 par défaut autorise tout ; 5 est TCP seulement. |

L'aide complète de chaque ligne s'affiche dans le volet sous le tableau quand vous la sélectionnez.

## Cache disque

| Réglage | Ce qu'il fait |
|---|---|
| **Activer la mise en cache des lectures disque** | Garde en mémoire les pièces lues récemment, ce qui épargne des déplacements de tête du disque pendant le partage. Les écritures sont laissées au cache du système. |
| **Remplacer la taille automatique du cache (Mo) :** | Utilise cette taille à la place des 32 Mo automatiques pour le cache de lecture. Un cache plus grand peut aider pour un partage intense, au prix de la mémoire. |
| **Retirer les blocs inutilisés depuis plus de (secondes) :** | Abandonne les blocs en cache qu'aucun pair n'a demandés pendant ce temps. Désactivé, ils restent jusqu'à ce que des blocs plus récents les chassent. |
| **File d'écriture disque (ko) :** | La quantité de données téléchargées qui peut attendre d'être écrite ; au-delà, Rookparcel cesse de lire les pairs jusqu'à ce que le disque rattrape son retard. Les valeurs inférieures à 4 Mo comptent pour 4 Mo. |
| **Nombre maximum de fichiers ouverts :** | Le maximum de fichiers gardés ouverts en même temps. Augmentez-le quand beaucoup de torrents ou de fichiers sont actifs en même temps. |
| **Threads d'E/S disque (au prochain démarrage) :** | Les threads qui lisent et écrivent les données des torrents. |
| **Threads de hachage (non utilisé) :** | Conservé pour la compatibilité ; cette version ne l'utilise pas. Les vérifications s'exécutent une par lecteur, réglées sur la page E/S disque. |

Voir aussi : [Disque et stockage](Disque-et-stockage).

## E/S disque

| Réglage | Ce qu'il fait |
|---|---|
| **Vérifier les torrents en douceur (basse priorité, laisser la place aux autres programmes)** | Vérifie les torrents à la priorité disque d'arrière-plan du système et fait une pause tant que les lectures sont plus lentes que d'habitude, pour que l'ordinateur reste réactif. Les vérifications durent plus longtemps tant que le disque est occupé. |
| **Limiter la vérification à (Mo/s) :** | Plafonne la vitesse de lecture de chaque vérification, pour un disque qui rame encore pendant les vérifications en douceur. Désactivé par défaut. |
| **Lecteurs vérifiés en même temps (un torrent par lecteur) :** | Le nombre de lecteurs vérifiés en même temps ; les torrents d'un même lecteur attendent leur tour. -1 signifie sans limite. |
| **Adapter au type de lecteur (SSD, disque dur, réseau)** | Détecte si chaque dossier d'enregistrement se trouve sur un SSD, un disque dur, un lecteur amovible ou un partage réseau, et ajuste le nombre de tâches disque exécutées en même temps. |
| **Ralentir les téléchargements (écritures sur le disque)** | Tant qu'un lecteur répond aux écritures nettement plus lentement que d'habitude, réduit de moitié la vitesse de réception par étapes, jamais en dessous de 50 ko/s, et la laisse remonter une fois le disque au calme. |
| **Ralentir les envois (lectures sur le disque)** | La même chose pour les envois tant qu'un lecteur répond lentement aux lectures. |

Voir aussi : [Disque et stockage](Disque-et-stockage).

## Trackers

*Nouveauté de la 1.2.40.* Des trackers que chaque torrent que vous ajoutez à partir de maintenant reçoit en plus des siens. Les deux cases sont désactivées au début.

| Réglage | Ce qu'il fait |
|---|---|
| **Ajouter ces trackers aux nouveaux téléchargements :** | Ajoute les trackers de la zone, une adresse par ligne (`http://`, `https://` ou `udp://`) ; une ligne vide commence un nouveau niveau. Ils viennent après les propres trackers du torrent. |
| **Ajouter les trackers de cette liste sur le Web (https) :** | Lit un fichier texte d'adresses de trackers à l'adresse `https://` ci-dessous, au démarrage puis une fois par jour, et ajoute ses trackers après ceux de la zone. |
| **Mettre à jour maintenant** | Lit la liste à l'adresse maintenant. La ligne sous le bouton dit combien de trackers la liste contient et quand elle a été lue, ou pourquoi le dernier essai a échoué. |

À savoir :

- **Les torrents privés ne reçoivent jamais ces trackers**, pas plus que les torrents déjà présents dans la liste, les torrents que vous créez ou ceux importés depuis un autre client. Un lien magnet les reçoit une fois ses détails arrivés et montrant qu'il n'est pas privé.
- **La liste sur le Web n'est jamais lue que comme une liste.** L'adresse doit commencer par `https://`. Seules les lignes qui sont des adresses de trackers sont utilisées, 200 au plus, et un fichier qui est autre chose (une page d'erreur, un programme) est ignoré : la dernière bonne liste reste en usage. Rien dans le fichier n'est jamais exécuté.
- **Celui qui gère cette adresse choisit ses trackers**, et ces trackers voient alors votre adresse et vos téléchargements publics. Utilisez une liste en laquelle vous avez confiance.
- Dans l'interface web, les réglages de la page ne changent qu'avec le mot de passe actuel, et l'API compatible qBittorrent ne peut pas les changer.
- `rookparcel-nox` garde ces réglages avec ceux de l'interface web : il a donc besoin de `--webui` pour les utiliser.

## Exécuter un programme

*Nouveauté de la 1.2.41.* Lance un programme à vous quand un torrent est ajouté ou se termine. Les deux cases sont désactivées au début.

| Réglage | Ce qu'il fait |
|---|---|
| **Exécuter ce programme à l'ajout d'un torrent :** | Lance le programme de la ligne ci-dessous pour chaque nouveau torrent. Pas pour un torrent qui était déjà dans la liste, ni pour les torrents chargés au démarrage. |
| **Exécuter ce programme à la fin du téléchargement d'un torrent :** | Lance le programme pour chaque torrent qui se termine. Pas après une revérification ou un redémarrage, ni pour un torrent dont les données étaient complètes quand vous l'avez ajouté. |

Écrivez d'abord le chemin du programme, entre guillemets doubles s'il contient des espaces, puis ses arguments, par exemple `"C:\Tools\notify.exe" --name "%N" --path "%F"`. Dans les arguments, ces codes remplacent les détails du torrent :

| Code | Remplace |
|---|---|
| `%N` | le nom du torrent |
| `%L` | son étiquette |
| `%F` | l'endroit où se trouvent ses données : le fichier, ou le dossier d'un torrent à plusieurs fichiers |
| `%D` | le dossier où il a été enregistré |
| `%Z` | sa taille en octets |
| `%T` | son tracker actuel |
| `%I`, `%J` | son info hash, v1 et v2 (`-` quand il n'en a pas) |
| `%K` | son identifiant : le hash v1, ou le hash v2 raccourci |
| `%%` | un signe pourcent |

À savoir :

- **Le programme est lancé directement, sans interpréteur de commandes.** Les tubes, `>` et `&&` ne font rien ; pour les utiliser, lancez un script et mettez-les dans le script. Pour lancer un script, nommez d'abord son interpréteur là où le système en demande un (`powershell -File ...`, `/bin/sh /path/script.sh`).
- **Le nom d'un torrent ne peut jamais devenir une commande.** Chaque code est placé à l'intérieur de son propre argument, avec ou sans guillemets, et le chemin du programme lui-même n'admet aucun code. Un nom peut tout de même commencer par un tiret, que certains programmes lisent comme une option : écrivez `--name=%N` ou mettez `--` devant là où cela compte.
- **Pas de codes pour un interpréteur de commandes ou de langage chargé d'exécuter une chaîne.** Un tel programme lirait le nom comme des commandes ; une ligne comme `cmd /c ... %N`, `powershell -Command ...`, `sh -c "... %N"`, `python -c`, `perl -e`, `node -e`, `osascript -e`, `ssh host ... %N` ou `wsl bash -c` est donc refusée dès qu'elle contient un code, y compris derrière `env`, `nohup`, `nice`, `timeout`, `sudo` et des enveloppes semblables (dont le programme n'admet pas non plus de code). Il en va de même d'un code dans le texte d'un programme `awk` ou `sed`, dans une commande `find -exec` qui lance un interpréteur, et après un interpréteur qu'un autre programme lance (`gnome-terminal -- bash -c "... %N"`). Les programmes qui exécutent comme du code une chaîne de leur ligne sont refusés avec un code n'importe où dans la ligne : `script`, `expect`, `forfiles`, `wmic`, des éditeurs tels que `vim` et `emacs`, et des débogueurs tels que `gdb`. Nommez plutôt le script et passez-lui les codes : `powershell -File C:\Tools\done.ps1 "%N"`, `/bin/sh /path/done.sh "%N"`, `python /path/done.py "%N"`, `wsl -e /path/done.sh "%N"`. Les options propres aux autres programmes sont les leurs : `notify-send -e "%N"`, `rsync -c "%F" /backup` et `/path/done.sh -c "%L"` sont acceptés. Une ligne enregistrée par une ancienne version et refusée désormais est désactivée, et le Journal le dit (la ligne et pourquoi) au démarrage.
- Le programme n'a ni fenêtre ni entrée, quatre au plus s'exécutent en même temps, et chaque lancement et chaque fin sont écrits dans le Journal.
- Le programme s'exécute sur l'ordinateur qui a les torrents : dans une fenêtre connectée à un `rookparcel-nox` distant, les lignes de cette page ne font rien ; réglez-les sur le serveur.
- **Qui peut changer les lignes.** Quiconque peut les changer peut exécuter des programmes sous votre compte ; donc : les programmes de bureau sur leur propre ordinateur ; l'interface web seulement depuis un navigateur du même ordinateur, avec le mot de passe actuel ; jamais l'API compatible qBittorrent, et jamais une connexion distante. Un serveur que vous ne pouvez joindre que d'ailleurs (Docker, par exemple) demande `--webui-remote-run-program` sur sa ligne de commande : voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox). Réfléchissez avant de le donner : avec lui, le mot de passe de l'interface web est tout ce qui sépare un visiteur de l'exécution de programmes sur le serveur.
- **Derrière un proxy inverse.** Si l'interface web est atteinte par un proxy ou un tunnel sur le serveur lui-même (nginx, HAProxy, stunnel, un tunnel SSH), le serveur voit chaque visiteur comme un navigateur du même ordinateur, sauf si le proxy dit autre chose. Faites ajouter par le proxy l'adresse du visiteur (`X-Forwarded-For` ; pour nginx : `proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;`). Sans cet en-tête, quiconque atteint le proxy et connaît le mot de passe peut changer ces lignes, exactement comme avec `--webui-remote-run-program`.

## Verrouillage de l'interface

Le code PIN qui verrouille les fenêtres de Rookparcel, et l'interrupteur qui désactive le verrouillage. Contrairement au reste des Préférences, tout ce qui se trouve sur cette page prend effet aussitôt : **OK**, **Appliquer** et **Annuler** n'y changent rien. *Nouveauté de la 1.2.63 ; auparavant, le code PIN se définissait dans **Options > Lock PIN**.*

| Réglage | Ce qu'il fait |
|---|---|
| **Autoriser le verrouillage de Rookparcel par un code PIN** | Activé par défaut. Désactivé, il n'y a ni **Fichier > Verrouiller Rookparcel**, ni <kbd>Ctrl</kbd>+<kbd>L</kbd>, ni **Verrouiller** dans le menu de la zone de notification. Le désactiver demande le code PIN, s'il y en a un, et le supprime. |
| **Définir le code PIN...** | Demande un nouveau code PIN de 4 à 6 chiffres, deux fois. Seulement tant qu'aucun code PIN n'est défini. |
| **Modifier le code PIN...** | Demande le code PIN actuel, puis un nouveau deux fois. |
| **Supprimer le code PIN...** | Demande le code PIN actuel, puis le supprime. **Verrouiller Rookparcel** demande ensuite un nouveau code PIN avant de verrouiller. |

La ligne sous les boutons dit si un code PIN est défini. Voir aussi : [Verrouiller Rookparcel](Verrouiller-Rookparcel).

## Différences dans la version GTK

La version GTK (Linux, BSD) a les mêmes pages et les mêmes réglages, avec ces différences :

- Il n'y a pas de page **Interface web**.
- **Interface** n'a pas de réglage **Thème** : GTK suit le thème du bureau ([Mode sombre](Mode-sombre)). Les cases de la zone de notification et des notifications (**Afficher les bulles de notification**, **Fermer dans la zone de notification**, **Réduire dans la zone de notification**) ne s'affichent que tant qu'une icône est à l'écran dans la zone de notification.
- **Général** a **Intégration au bureau** au lieu d'Intégration à Windows, et pas de bouton **Applications par défaut...**. Dans un Flatpak, les cases d'association sont grisées sous une note : choisissez le programme par défaut dans les paramètres de votre bureau. Le groupe **Confidentialité** n'est pas affiché dans un Flatpak, que Flathub met à jour.
- <kbd>Escape</kbd> ferme la fenêtre comme **Annuler**.

## Différences sous macOS

L'application macOS ouvre les Préférences depuis **Rookparcel > Préférences** (<kbd>⌘</kbd>+<kbd>,</kbd>). Elle a les pages de la version GTK, listées dans une barre latérale, et ces différences :

- Chaque changement s'applique aussitôt, comme dans les autres apps Mac : il n'y a ni **OK**, ni **Annuler**, ni **Appliquer**. Une case ou une liste s'applique quand vous la changez ; un champ de texte ou de nombre quand vous appuyez sur <kbd>Return</kbd>, quittez le champ ou fermez la fenêtre.
- Une valeur non autorisée, telle qu'un port hors plage, s'affiche en rouge sous son champ, et la dernière bonne valeur reste en usage. Fermer la fenêtre remet la bonne valeur dans le champ.
- Les cases d'association ne fonctionnent que dans `Rookparcel.app` et seulement dans le profil par défaut.

*L'application macOS native est nouvelle dans la 1.2.19.*

## Différences dans l'interface web

L'interface web (celle de `rookparcel-nox`, ou celle que la fenêtre Windows sert) a une fenêtre **Préférences** avec les mêmes pages. Les valeurs modifiées de sa page Avancé s'affichent en gras au lieu d'un `*`. Autres différences :

- **Général** : les cases d'association et de démarrage automatique sont grisées avec la note « Les associations de fichiers et le démarrage automatique relèvent du client de bureau. » `rookparcel-nox` a son propre groupe **Confidentialité** pour sa recherche de mises à jour ; l'interface web qu'une fenêtre Windows sert n'en a pas.
- **Interface** n'a que **Confirmer la suppression des torrents**, les deux réglages de thème, **Ne pas démarrer le téléchargement automatiquement**, **Afficher une fenêtre avec les fichiers du torrent** et **Ne pas télécharger les fichiers nommés :**.
- **BitTorrent** : changer l'adresse du filtre IP, ou la case au-dessus, demande le mot de passe actuel, saisi dans **Mot de passe actuel :** sur la page Interface web. L'interface web que sert la fenêtre Windows n'a ni adresse, ni **Mettre à jour maintenant**, ni ligne d'état : les Préférences de la fenêtre elle-même les contiennent.
- **Bande passante** dans `rookparcel-nox` a **Limites alternatives** : **Vitesse d'envoi alternative**, **Vitesse de réception alternative** et **Utiliser maintenant les limites alternatives**, que les applications qui basculent le mode de limites de vitesse de qBittorrent font aussi basculer.
- **File d'attente** a la case **Arrêter le torrent ou limiter sa vitesse d'envoi** au-dessus de **Limiter la vitesse d'envoi à (ko/s) : [0 : arrêter]** ; la case met l'objectif de partage en vigueur.
- **Interface web** ajoute **Mot de passe actuel :** (nécessaire pour changer l'identifiant et les réglages de sécurité), **Pas d'authentification pour les clients sur localhost**, **Adresse d'écoute :** (affichée à titre indicatif), **Activer la validation de l'en-tête Host**, **Activer la protection contre la falsification de requête intersite (CSRF)**, et un groupe **Sécurité** avec **Bannir le client après échecs consécutifs :**, **Durée du bannissement (secondes) :** et **Expiration de la session (secondes) :**.
- <kbd>Ctrl</kbd>+<kbd>Enter</kbd> fonctionne comme **OK** ; <kbd>Enter</kbd> dans le champ de valeur d'Avancé définit la valeur.

Voir [Interface web](Interface-Web) et [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).
