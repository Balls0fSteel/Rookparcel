*Cette page est la traduction de la page anglaise [Connection, ports and firewall](Connection-ports-and-firewall). En cas de divergence, la page anglaise fait foi.*

# Connexion, ports et pare-feu

Cette page explique comment les autres pairs joignent Rookparcel : le port d'écoute, la redirection de port avec UPnP et NAT-PMP, le pare-feu Windows, comment savoir si les connexions entrantes fonctionnent, comment garder le trafic des torrents sur un VPN avec « Utiliser uniquement ce réseau », les proxys, le chiffrement, le filtre IP et sa liste de blocage depuis une adresse web, ainsi que le DHT, l'échange de pairs et la découverte de pairs locaux.

La plupart de ces paramètres se trouvent dans **Options > Préférences > Connexion** et **Options > Préférences > BitTorrent**.

![Préférences, page Connexion](images/prefs-connection.png)

## Pourquoi les connexions entrantes comptent

Rookparcel se connecte aux pairs, et les pairs se connectent à Rookparcel. Les téléchargements fonctionnent avec les seules connexions sortantes, mais un Rookparcel que les autres peuvent joindre trouve plus de pairs, et deux pairs qui ne peuvent ni l'un ni l'autre être joints ne peuvent pas communiquer du tout. Pour de bonnes vitesses, et pour partager, laissez entrer les pairs.

## Le port d'écoute

Sous **Port d'écoute** dans la page Connexion :

| Paramètre | Signification |
|---|---|
| **Port utilisé pour les connexions entrantes :** | Le port auquel les autres pairs se connectent. Une modification s'applique immédiatement. |
| **Port aléatoire** | Remplit un port aléatoire de 10000 à 60000. Il est utilisé une fois que vous l'appliquez. |
| **Port aléatoire à chaque démarrage** | Choisit un nouveau port aléatoire de 10000 à 60000 à chaque démarrage. Un port redirigé à la main dans le routeur ne correspond alors plus. Désactivé par défaut. |
| **Activer la redirection de port UPnP** | Demande à un routeur UPnP de rediriger le port vers cet ordinateur. Activé par défaut. |
| **Activer la redirection de port NAT-PMP** | Demande au routeur de rediriger le port par NAT-PMP ou PCP, que beaucoup de routeurs proposent en plus d'UPnP ou à sa place. Activé par défaut. |

Le profil par défaut écoute sur le port 6881. Un nouveau profil, ou une copie portable, choisit un port aléatoire de 10000 à 60000 à son premier démarrage, de sorte qu'il n'entre pas en collision avec un autre Rookparcel. Voir [Profils](Profils).

Rookparcel écoute en TCP et en UDP sur le même port : l'UDP transporte les pairs uTP et le DHT.

### Rediriger le port dans le routeur

Avec UPnP ou NAT-PMP activés et un routeur qui les prend en charge, il n'y a rien à faire : Rookparcel demande au routeur de rediriger le port et renouvelle la demande. Si votre routeur les a désactivés, ou se comporte mal, décochez les deux cases et redirigez le port à la main dans les réglages propres du routeur : TCP et UDP, vers l'adresse de cet ordinateur. Laissez ensuite **Port aléatoire à chaque démarrage** désactivé, pour que le port reste celui que vous avez redirigé.

## Le pare-feu Windows

Rookparcel n'ajoute pas de règle au pare-feu Windows de lui-même. Windows pose normalement la question la première fois qu'un programme écoute des connexions entrantes ; autorisez Rookparcel à ce moment-là.

Si vous avez écarté cette question, ou si rien n'a été demandé, ajoutez une règle pour le programme depuis une Invite de commandes exécutée en tant qu'administrateur. Pour une installation pour un seul utilisateur :

```bat
netsh advfirewall firewall add rule name="Rookparcel" dir=in action=allow program="%LOCALAPPDATA%\Programs\Rookparcel\rookparcel.exe"
```

Pour une installation pour tous les utilisateurs, le chemin est `C:\Program Files\Rookparcel\rookparcel.exe` ; pour le zip, l'endroit où vous avez décompressé `rookparcel.exe`. Le port de l'interface web demande la même règle quand d'autres ordinateurs doivent la joindre : voir [Interface Web](Interface-Web). Pour [rookparcel-nox](Utiliser-rookparcel-nox) exécuté comme service, voir sa page.

Sous Linux et les BSD, autorisez le port dans votre pare-feu si vous en utilisez un.

## Vérifier que les pairs peuvent se connecter

La partie gauche de la barre d'état vous renseigne :

| Barre d'état | Signification |
|---|---|
| **Réseau OK** | Au moins un pair s'est connecté. Le port fonctionne. |
| **Aucune connexion entrante** | Aucun pair ne s'est encore connecté. Normal pendant les premières minutes après un démarrage ; si cela dure, le port n'est pas joignable. |
| **Port 6881 déjà utilisé** (avec votre numéro de port) | Un autre programme, souvent un autre profil Rookparcel, occupe le port, et Rookparcel écoute sur un port suivant où rien n'est redirigé. Cliquez dessus pour ouvrir les Préférences sur Connexion et choisir un autre port. |

Deux avis peuvent précéder ces mots. **Transferts suspendus : confirmez les préférences** signifie que Rookparcel suspend tous les transferts jusqu'à ce que vous confirmiez les préférences après un changement de version : voir [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel). **En attente du réseau** signifie que « Utiliser uniquement ce réseau » ne trouve aucun adaptateur correspondant : voir la section ci-dessous.

*« Port ... déjà utilisé » est nouveau dans la 1.1.45.*

Placez le pointeur sur cette partie pour voir **Votre adresse vue par les trackers :**, l'adresse publique que les trackers renvoient. L'onglet **Général** montre la même ligne. *Nouveau dans la 1.1.58.*

Rookparcel n'a pas de test de port intégré. Si l'état reste sur « Aucune connexion entrante » avec des torrents actifs :

1. Vérifiez que le pare-feu autorise Rookparcel (ci-dessus).
2. Vérifiez qu'UPnP ou NAT-PMP est activé dans Rookparcel et dans le routeur, ou redirigez le port à la main.
3. Avec « Utiliser uniquement ce réseau » réglé sur un VPN, les pairs ne peuvent vous joindre que par la redirection de port propre au VPN, s'il en a une : Rookparcel ne demande aucune redirection de port à un adaptateur VPN.

Voir aussi [Dépannage et FAQ](Dépannage-et-FAQ).

## Utiliser uniquement ce réseau (VPN)

*Nouveau dans la 1.1.58.*

« Utiliser uniquement ce réseau » garde tout le trafic des torrents sur un seul adaptateur réseau, typiquement un VPN. Les pairs, le DHT, les trackers et les sources web passent tous par lui et par rien d'autre. Quand le VPN tombe, les torrents attendent au lieu de continuer sur votre connexion normale.

1. Connectez votre VPN.
2. Ouvrez **Options > Préférences > Connexion**.
3. Sous **Utiliser uniquement ce réseau**, choisissez dans **Réseau :**
   - **N'importe quel adaptateur VPN** : tout adaptateur qui ressemble à un tunnel VPN (WireGuard, OpenVPN, celui d'une application VPN). C'est ce que la plupart des gens veulent.
   - Un adaptateur par son nom, dans la liste des adaptateurs de cet ordinateur.
   - **Plage d'adresses (ci-dessous)**, puis saisissez la plage dans **Plage d'adresses :**, par exemple `10.2.0.0/16` ou `fd00::/8`.
   - **N'importe quel réseau (désactivé)** le désactive ; c'est le choix par défaut.
4. Laissez **Mettre les torrents en pause tant que ce réseau est absent** coché.
5. Cliquez sur **OK** ou **Appliquer**.

Lequel choisir :

- **N'importe quel adaptateur VPN** résiste à l'application VPN qui renomme ou recrée son adaptateur, comme beaucoup le font à chaque connexion ou mise à jour.
- **Plage d'adresses** convient à un VPN qui attribue toujours des adresses de la même plage : l'adaptateur qui détient une adresse dans cette plage transporte le trafic, quel que soit son nom.
- **Un adaptateur par son nom** est le choix le plus étroit. Un adaptateur qui n'est pas là en ce moment apparaît comme « (non connecté) » dans la liste.

La règle est revérifiée à chaque changement de réseau, de sorte qu'un VPN qui se reconnecte sous une nouvelle adresse est suivi.

**Tant que le réseau est absent :** avec **Mettre les torrents en pause tant que ce réseau est absent** activé, les torrents en cours attendent. Leur état affiche **En attente du réseau**, la barre d'état commence par **En attente du réseau**, et le Journal dit quelle règle n'a rien trouvé. Quand le VPN revient, ils reprennent tout seuls. Arrêter et Démarrer fonctionnent toujours sur eux entre-temps. Avec la case décochée, rien n'est mis en pause, mais aucun trafic de torrent ne peut sortir non plus, puisque chaque connexion a besoin de l'adaptateur correspondant.

**Vérifier que cela fonctionne :** placez le pointeur sur la partie réseau de la barre d'état, ou regardez l'onglet **Général** : **Votre adresse vue par les trackers :** doit afficher l'adresse du VPN, pas la vôtre. Il indique « pas encore connue » tant qu'un tracker n'a pas répondu.

À savoir :

- Si le VPN ne transporte que l'IPv4, Rookparcel n'utilise pas d'IPv6 pour les torrents tant qu'il y est lié, de sorte que l'IPv6 ne puisse pas fuir en contournant le tunnel.
- L'interface web et les connexions distantes à Rookparcel ne sont pas liées, de sorte que vous pouvez toujours les joindre par votre réseau normal.
- Les connexions par proxy ne sont pas liées non plus. L'assistant de recherche est un programme distinct et n'est pas lié ; voir [Rechercher des torrents sous Windows](Rechercher-des-torrents-sous-Windows#bon-à-savoir).
- Dans l'interface web, le même groupe se trouve dans **Préférences > Connexion**. Pour rookparcel-nox en ligne de commande : `--network vpn`, `--network adapter:ID`, `--network range:10.2.0.0/16`, ou `off` ; et `--no-pause-when-missing` pour laisser les torrents s'exécuter sans liaison.

## Serveur proxy

Sous **Serveur proxy** dans la page Connexion. Les champs du proxy sont grisés tant que **Type :** vaut **(aucun)**, le choix par défaut.

| Paramètre | Signification |
|---|---|
| **Type :** | **Socks4**, **Socks5** ou **HTTP** (qui fait passer tout sauf les requêtes http simples par CONNECT). **(aucun)** se connecte directement. |
| **Proxy :** et **Port :** | Le nom d'hôte ou l'adresse IP du serveur proxy, et son port, comme 1080 pour beaucoup de serveurs SOCKS. |
| **Authentification** avec **Nom d'utilisateur :** et **Mot de passe :** | Se connecte au proxy. Un proxy SOCKS4 ne reçoit que le nom d'utilisateur. |
| **Résoudre les noms d'hôte par le proxy** | Laisse le proxy rechercher les noms des trackers et des sources web, de sorte que votre propre serveur DNS ne les voie pas. |
| **Utiliser le proxy pour les connexions entre pairs** | Envoie les connexions aux pairs et aux sources web par le proxy. Les connexions entrantes sont alors refusées, et la redirection de port et la découverte de pairs locaux restent désactivées. uTP et le DHT demandent un proxy SOCKS5. |
| **Utiliser le proxy pour les connexions aux trackers** | Envoie les annonces aux trackers par le proxy, de sorte que les trackers voient l'adresse du proxy. |

Les trois derniers sont cochés par défaut et ne comptent qu'une fois un type de proxy choisi. Un proxy SOCKS5 d'un fournisseur de VPN est une façon d'utiliser un VPN sans adaptateur ; « Utiliser uniquement ce réseau » est l'autre.

## Chiffrement du protocole

Dans **Options > Préférences > BitTorrent**, sous **Chiffrement du protocole** :

| Paramètre | Signification |
|---|---|
| **Sortant :** | Comment les connexions que Rookparcel ouvre sont chiffrées. **Désactivé** : jamais. **Activé** (le choix par défaut) : les tentatives en clair et chiffrées alternent selon le pair, en gardant ce qui a fonctionné. **Forcé** : chiffré seulement, ce qui fait perdre les pairs qui ne savent pas chiffrer. |
| **Autoriser les connexions entrantes non chiffrées** | Accepte les connexions entrantes qui ne sont pas chiffrées. Désactivé, seules les connexions chiffrées sont acceptées, ce qui fait perdre les pairs qui ne savent pas chiffrer. Activé par défaut. |

Changez-les si votre fournisseur ralentit le trafic BitTorrent. La page Avancé a les paramètres plus fins (`out_enc_policy`, `in_enc_policy`, `allowed_enc_level`, `prefer_rc4`).

![Préférences, page BitTorrent](images/prefs-bittorrent.png)

## DHT, échange de pairs, découverte de pairs locaux et uTP

Sous **Fonctions BitTorrent de base** dans la page BitTorrent, tous activés par défaut :

| Paramètre | Signification |
|---|---|
| **Activer le réseau DHT** | Trouve des pairs par le DHT, un réseau de clients BitTorrent, sans tracker. La plupart des liens magnet en ont besoin. |
| **Activer la découverte de pairs locaux** | Trouve des pairs pour les mêmes torrents sur votre réseau local, de sorte qu'ils échangent des données directement à la vitesse du réseau. |
| **Activer l'échange de pairs** | Permet aux pairs connectés de se parler d'autres pairs. Une modification s'applique à tous les torrents. |
| **Activer uTP (transport UDP avec gestion de la bande passante)** | Se connecte aux pairs et en accepte par uTP, un transport UDP qui ralentit quand le réseau est occupé, en plus du TCP. |

Les torrents privés n'utilisent jamais le DHT, l'échange de pairs ni la découverte de pairs locaux, quoi que disent ces cases. Les Propriétés d'un torrent peuvent désactiver le DHT et l'échange de pairs pour ce seul torrent (**Activer le DHT** et **Échange de pairs** sous **Autres paramètres**).

La barre d'état affiche **DHT : N nœuds**, le nombre de nœuds DHT que Rookparcel connaît.

Pour n'autoriser que certains types de connexion, comme le TCP seulement, utilisez `bt.transp_disposition` dans la page Avancé : 1 TCP sortant, 2 uTP sortant, 4 TCP entrant, 8 uTP entrant, additionnés ; 15 (le choix par défaut) autorise tout, 5 correspond au TCP seulement.

## Filtre IP

Sous **Filtre IP** dans la page BitTorrent, Rookparcel peut bloquer des pairs par adresse, à partir d'une liste de blocage au format eMule `ipfilter.dat`, PeerGuardian P2P ou CIDR. La liste peut être un fichier sur votre ordinateur ou un fichier sur le web que Rookparcel télécharge et tient à jour.

Chaque ligne d'une liste est une règle :

| Format | Exemple de ligne |
|---|---|
| eMule `ipfilter.dat` | `001.002.004.000 - 001.002.004.255 , 000 , Some name` |
| PeerGuardian P2P | `Some name:1.2.4.0-1.2.4.255` |
| CIDR, IPv4 ou IPv6 | `10.0.0.0/8` ou `fd00::/8` |

*Les lignes CIDR sont nouvelles dans la 1.2.63.* Elles fonctionnent aussi dans un fichier sur votre ordinateur.

### Un fichier de liste sur votre ordinateur

1. Cochez **Activer le filtre IP**.
2. Choisissez le fichier de la liste de blocage dans la zone en dessous. Laissée vide, Rookparcel utilise `ipfilter.dat` dans son dossier de paramètres.
3. Cliquez sur **Recharger** pour le charger maintenant et voir combien de règles il contient (« N règles chargées. »), ou cliquez sur **OK** pour le charger avec les autres paramètres.

La liste est chargée au démarrage et à l'application. Le Journal écrit « Filtre IP : N règles depuis ... » ou l'erreur. Si une nouvelle liste ne peut pas être lue, les règles chargées auparavant restent en vigueur.

Dans l'interface web, il n'y a pas de bouton Recharger : la liste est chargée quand vous appliquez.

### Une liste depuis une adresse web

*Nouveau dans la 1.2.63.* Rookparcel peut télécharger la liste de blocage depuis le web et la tenir à jour :

1. Cochez **Activer le filtre IP**.
2. Cochez **Obtenir la liste de blocage depuis cette adresse (https) :** et saisissez l'adresse de la liste dans la zone en dessous. Elle doit commencer par `https://`.
3. Cliquez sur **Mettre à jour maintenant** pour la télécharger tout de suite, ou sur **OK**. La ligne sous le bouton montre le résultat.

Tant que l'adresse est utilisée, elle remplace le fichier : la zone du fichier et **Recharger** sont grisés. À savoir :

- **Seulement `https://`.** Une adresse qui renvoie Rookparcel vers une adresse `http://` simple est refusée. Le téléchargement passe par votre proxy quand Préférences > Connexion en règle un.
- **Formats.** Les trois formats ci-dessus, en clair ou compressés en gzip. Les archives zip ne sont pas prises en charge. Un téléchargement peut atteindre 32 Mio.
- **Fréquence.** Une fois par jour, et une heure après un téléchargement qui a échoué. **Mettre à jour maintenant** la télécharge à tout moment.
- **Au démarrage.** Rookparcel conserve le dernier bon téléchargement dans son dossier de paramètres (`ipfilter-url.dat`) et le met en vigueur tout de suite au démarrage suivant, avant de télécharger quoi que ce soit.
- **Une nouvelle adresse.** La copie de la liste de l'ancienne adresse reste dans le dossier de paramètres jusqu'à ce qu'une liste de la nouvelle adresse ait été téléchargée et vérifiée, de sorte qu'une faute de frappe ou un téléchargement qui échoue ne fait rien perdre, et revenir à l'ancienne adresse utilise de nouveau sa copie. Une copie n'est utilisée que pour l'adresse dont elle provient : démarré avec une nouvelle adresse, Rookparcel n'a aucune liste de blocage en vigueur jusqu'à l'arrivée de la nouvelle, et le Journal le dit. Dans le programme Windows, **Mettre à jour maintenant** avec une adresse que vous n'avez pas encore appliquée ne fait que l'essayer : la ligne montre le résultat, et rien ne change avant **OK** ou **Appliquer**.
- **Quand un téléchargement échoue** ou apporte quelque chose qui n'est pas une liste de blocage (une page d'erreur, par exemple), les règles déjà en vigueur restent, et la ligne d'état affiche « Le dernier téléchargement a échoué : ... ». Le Journal écrit aussi l'erreur.
- **Ligne d'état.** « N règles, téléchargées » avec la date et l'heure, ou « La liste n'a pas encore été téléchargée. » jusqu'à ce que le premier téléchargement réussisse. D'ici là, les règles déjà en vigueur restent.
- **La vérification des paramètres.** Tant que Rookparcel suspend les transferts après un changement de version, jusqu'à ce que vous confirmiez les paramètres ([Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel)), le téléchargement attend lui aussi.

Utilisez une adresse en laquelle vous avez confiance : celui qui contrôle la liste décide avec quels pairs Rookparcel refuse de communiquer.

Dans l'interface web, modifier la zone ou l'adresse demande le mot de passe actuel (**Mot de passe actuel :** sur la page Interface web), parce que le serveur télécharge l'adresse qui est réglée, quelle qu'elle soit. La page affiche l'adresse sans nom d'utilisateur, mot de passe ni requête (les identifiants d'un abonnement s'y trouvent souvent) ; laissée telle quelle, l'adresse stockée est conservée. Pour supprimer les identifiants, videz la zone, appliquez, puis saisissez de nouveau l'adresse. L'interface web que sert le programme Windows laisse l'adresse de côté : elle relève des Préférences propres à la fenêtre. L'API compatible avec qBittorrent ne peut pas la modifier.

### Trackers et DHT

Par défaut, le filtre s'applique aussi au DHT et aux trackers situés à des adresses bloquées (`apply_filter_to_dht` et `apply_ip_filter_to_trackers` dans la page Avancé). Désactivez celui des trackers si un tracker dont vous avez besoin se trouve dans une plage bloquée.

## Dans la fenêtre GTK et l'interface web

- **Fenêtre GTK** : les mêmes pages et paramètres. La partie réseau de la barre d'état dit les mêmes choses, et un clic sur « Port ... déjà utilisé » ouvre les Préférences sur Connexion.
- **Application macOS** : les mêmes pages, qui s'appliquent immédiatement, sans **OK** ([Référence des préférences](Référence-des-préférences#différences-sous-macos)).
- **Interface web** : les mêmes pages sous **Préférences** ; sa barre d'état montre la partie réseau avec la même infobulle.
- **rookparcel-nox** : `--port N` règle le port d'écoute (6881 par défaut), `--listen` règle en entier les interfaces d'écoute, `--proxy TYPE://[USER:PASS@]HOST:PORT` règle un proxy, `--ip-filter FILE` charge un fichier de liste de blocage (l'adresse se règle dans les Préférences de l'interface web), et `--network` règle « Utiliser uniquement ce réseau ». Un service sous Windows a besoin d'une règle de pare-feu pour `rookparcel-nox.exe` : voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

## Pages liées

- [Limites de vitesse et planificateur](Limites-de-vitesse-et-planificateur)
- [Profils](Profils) (ports pour plusieurs Rookparcel)
- [Dépannage et FAQ](Dépannage-et-FAQ)
- [Référence des préférences](Référence-des-préférences)
