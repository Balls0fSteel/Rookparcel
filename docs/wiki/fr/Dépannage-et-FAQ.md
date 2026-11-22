*Cette page est la traduction de la page anglaise [Troubleshooting and FAQ](https://github.com/Balls0fSteel/Rookparcel/wiki/Troubleshooting-and-FAQ). En cas de différence, la page anglaise fait foi.*

# Dépannage et FAQ

Cette page aide lorsque quelque chose ne fonctionne pas : téléchargements lents ou bloqués, port inaccessible, avertissements de Windows ou de macOS, trackers privés. Elle indique aussi où se trouvent les messages de Rookparcel, comment réinitialiser ses paramètres et comment signaler un problème.

## Les téléchargements sont lents ou ne démarrent pas

Regardez la colonne **État** du torrent et la barre d'état en bas de la fenêtre. Elles indiquent en général la raison.

| Ce que vous voyez | Ce que cela signifie et ce qu'il faut faire |
|---|---|
| **En attente** | La file est pleine : le nombre de torrents ou de téléchargements actifs défini dans **Préférences > File d'attente** est atteint. Le torrent démarre quand un autre se termine ou s'arrête. **Forcer le démarrage** dans le menu du clic droit le lance en dehors de la file. Voir [Mise en file et partage](Mise-en-file-et-partage). |
| **Arrêté** ou **En pause** | Démarrez-le avec **Démarrer** (<kbd>Ctrl</kbd>+<kbd>S</kbd>). Avec **Ne pas démarrer le téléchargement automatiquement** activé dans Préférences > Interface, les nouveaux torrents sont ajoutés à l'arrêt. |
| **En attente de vérification**, **Vérification** | Rookparcel vérifie les fichiers déjà présents sur le disque. Les torrents situés sur un même lecteur sont vérifiés l'un après l'autre : ceux qui attendent leur tour affichent **En attente de vérification**. Avec **Vérifier les torrents en douceur** activé, une vérification laisse la place aux autres programmes et dure plus longtemps tant que le disque est occupé. Voir [Disque et stockage](Disque-et-stockage). *Les vérifications en attente l'indiquent depuis la 1.2.76 ; auparavant, elles affichaient **Vérification** à 0 %.* |
| **En attente du réseau** | **Utiliser uniquement ce réseau** est activé dans Préférences > Connexion et son réseau (en général votre VPN) est absent. Le torrent reprend de lui-même quand le réseau revient. Voir [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu). |
| **Erreur : ...** | Le message après « Erreur : » dit ce qui s'est mal passé. Pour « files missing (the drive, share or folder is not available) », voir la section suivante. Si une vérification s'arrête sur une erreur qui nomme un fichier, c'est qu'un autre programme tient ce fichier ouvert ou que vous n'avez pas le droit de le lire : fermez le programme ou corrigez l'autorisation, puis **Démarrez** le torrent. *Depuis la 1.2.76, une telle vérification s'arrête sur l'erreur ; auparavant, elle comptait les pièces du fichier comme manquantes et les téléchargeait de nouveau.* |
| « Planificateur : limité », « Planificateur : arrêté » ou « Planificateur : partage seul » dans la barre d'état | Le planificateur retient les vitesses à cette heure. Voir [Limites de vitesse et planificateur](Limites-de-vitesse-et-planificateur). |
| « (auto ...) » à côté des vitesses dans la barre d'état | **Garder un ping bas** plafonne la vitesse parce que le délai sur votre ligne a augmenté. Voir [Garder un ping bas](Garder-un-ping-bas). |

Autres points à vérifier :

- **Sources et pairs.** Un torrent sans source ne peut pas se terminer. L'onglet **Trackers** montre si les trackers répondent ; l'onglet **Pairs** montre à qui vous êtes connecté. Voir [Détails et fichiers des torrents](Détails-et-fichiers-des-torrents).
- **Les liens magnet** ont d'abord besoin de la liste de fichiers du torrent, obtenue auprès d'autres pairs, et la plupart ont besoin du DHT : gardez **Activer le réseau DHT** activé dans Préférences > BitTorrent.
- **Limites de vitesse.** Vérifiez **Préférences > Bande passante** et les limites propres au torrent (**Allocation de bande passante** dans le menu du clic droit, ou **Propriétés**). 0 signifie illimité.
- **Un disque occupé.** Avec **Ralentir les téléchargements (écritures sur le disque)** activé (Préférences > Avancé > E/S disque), Rookparcel baisse la vitesse de réception tant que le disque répond lentement.
- **Un proxy pour les connexions entre pairs.** Avec **Utiliser le proxy pour les connexions entre pairs** activé, personne ne peut se connecter à vous, ce qui laisse moins de pairs.
- **Aucune connexion entrante** (ci-dessous) coûte aussi de la vitesse : vous ne pouvez joindre que les pairs qui acceptent eux-mêmes les connexions.

## « Files missing (the drive, share or folder is not available) »

Ce message s'affiche en anglais. Lorsque le dossier d'enregistrement d'un torrent est absent (un lecteur USB ou réseau débranché, un partage ou un point de montage non connecté), Rookparcel arrête le torrent avec cette erreur au lieu de le télécharger de nouveau. Les pièces qu'il avait restent enregistrées, et elles le restent d'un redémarrage à l'autre.

1. Reconnectez le lecteur ou le partage. Toutes les 30 secondes, Rookparcel cherche le dossier ; quand il est de retour, le torrent démarre de lui-même et retrouve ses fichiers tels qu'ils étaient.
2. Pour essayer tout de suite, choisissez **Démarrer**. Si le dossier est toujours absent, le torrent s'arrête de nouveau avec l'erreur.
3. Si vous avez déplacé ou supprimé les fichiers pour de bon, **Forcer la vérification** prend ce qui se trouve maintenant dans le dossier : les pièces qui n'y sont pas sont téléchargées de nouveau.

Un lecteur retiré pendant que le torrent se télécharge est traité de la même façon. Lorsque seuls certains fichiers d'un torrent ont disparu, il les télécharge de nouveau à la place. Voir [Gérer les torrents](Gérer-les-torrents). *Nouveauté de la 1.2.76 ; auparavant, le torrent téléchargeait tout de nouveau.*

**Forcer la vérification** sur un torrent arrêté le démarre désormais, comme **Démarrer**, et la vérification s'exécute aussitôt même si le torrent était en file d'attente ; arrêtez-le de nouveau si vous voulez qu'il reste arrêté. *Nouveauté de la 1.2.76 ; auparavant, la vérification attendait un Démarrer.*

## « Transferts suspendus : confirmez les préférences »

Après une mise à jour, ou quand le fichier de paramètres ne peut pas être lu, Rookparcel suspend tous les transferts jusqu'à ce que vous confirmiez que ses paramètres de proxy, de réseau et d'écoute sont ceux que vous voulez. Ouvrez **Options > Préférences > Connexion**, vérifiez-les et cliquez sur **OK**. L'onglet Journal liste ce qui a changé. Voir [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel#quand-une-version-lit-vos-réglages-autrement).

## « Aucune connexion entrante » ou « Port ... déjà utilisé »

La partie gauche de la barre d'état indique l'état du réseau :

| Texte | Signification |
|---|---|
| **Réseau OK** | D'autres pairs se sont connectés à vous : votre port d'écoute est donc joignable. |
| **Aucune connexion entrante** | Aucun pair ne s'est encore connecté à vous. Juste après le démarrage, c'est normal ; si cela dure, votre port est probablement bloqué par le routeur ou le pare-feu. |
| **Port 6881 déjà utilisé** (avec votre numéro de port) | Un autre programme, souvent un autre profil Rookparcel, tient le port : Rookparcel écoute donc sur un autre, où rien n'est redirigé. Cliquez sur le texte pour ouvrir **Préférences > Connexion** et choisissez un autre port. *Nouveauté de la 1.1.45.* |

Pour rendre le port joignable :

1. Ouvrez **Options > Préférences > Connexion** et notez le **Port utilisé pour les connexions entrantes**.
2. Cochez **Activer la redirection de port UPnP** et **Activer la redirection de port NAT-PMP**, pour que le routeur soit prié de rediriger le port. Ou redirigez le port à la main dans le routeur (TCP et UDP) vers cet ordinateur, et laissez **Port aléatoire à chaque démarrage** désactivé.
3. Autorisez `rookparcel.exe` à traverser le pare-feu (sous Linux, `rookparcel-gtk`), ou ouvrez-y le port.
4. Derrière un VPN, les connexions entrantes ne fonctionnent que si le VPN redirige un port vers vous.

L'onglet **Général** affiche « Votre adresse vue par les trackers : », ce qui vous dit si votre trafic passe par un VPN. *Nouveauté de la 1.1.58.* Plus de détails dans [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu).

## Windows SmartScreen ou macOS Gatekeeper avertit à propos de Rookparcel

Les programmes ne sont pas signés, et les systèmes avertissent donc la première fois :

- **Windows :** « Windows a protégé votre ordinateur ». Cliquez sur **Informations complémentaires**, puis sur **Exécuter quand même**.
- **macOS :** l'app est refusée tant que vous ne l'avez pas autorisée une fois. Sous macOS 15 et plus récent, essayez de l'ouvrir, cliquez sur **Terminé**, puis sur **Ouvrir quand même** dans **Réglages Système > Confidentialité et sécurité**. Sous macOS 11 à 14, faites Contrôle-clic sur l'app et choisissez **Ouvrir**. Voir [Installer Rookparcel](Installer-Rookparcel#macos).

Pour vous assurer qu'un fichier est bien celui qui a été publié, comparez sa somme de contrôle avec le fichier `SHA256SUMS` de la version. Les détails de chaque plateforme sont dans [Installer Rookparcel](Installer-Rookparcel).

## Puis-je utiliser Rookparcel sur des trackers privés ?

Pas encore. Les trackers privés n'acceptent que les clients qu'ils ont mis sur liste blanche, et l'identifiant client de Rookparcel (son identifiant de pair commence par `-PA`, et il envoie `Rookparcel/<version>` aux trackers) n'est encore sur la liste blanche d'aucun. Rookparcel se déclare toujours honnêtement comme Rookparcel : l'utiliser sur un tracker privé pourrait donc causer des ennuis à votre compte. Consultez d'abord la liste des clients autorisés de votre tracker.

Les torrents marqués privés n'utilisent jamais le DHT, la découverte de pairs locaux ni l'échange de pairs, quels que soient les réglages des Préférences.

## Double-cliquer sur un fichier .torrent ou sur un lien magnet n'ouvre pas Rookparcel

Cochez **Associer aux fichiers .torrent** et **Associer aux liens magnet:** dans **Préférences > Général**. Si Windows les ouvre toujours avec un autre programme, c'est qu'un programme a été choisi dans Windows lui-même, et vous seul pouvez le changer : cliquez sur **Applications par défaut...** et choisissez Rookparcel à cet endroit. Voir [Zone de notification, notifications et démarrage](Zone-de-notification-et-démarrage#ouvrir-les-fichiers-torrent-et-les-liens-magnet-avec-rookparcel).

## L'interface web est inaccessible depuis un autre appareil

- Cochez **Autoriser les connexions depuis d'autres ordinateurs** dans **Préférences > Interface web** ; sinon elle n'écoute que sur cet ordinateur.
- Autorisez `rookparcel.exe` dans le Pare-feu Windows pour le port de l'interface web.
- « le port ... est déjà utilisé, peut-être par un autre profil Rookparcel » : choisissez un autre **Port d'écoute**.
- Une application qui joint Rookparcel par un nom d'hôte (comme dans Docker) échoue avec « unexpected Host header » (message affiché en anglais). Ajoutez le nom dans **Domaines du serveur**, ou utilisez l'adresse IP.

Voir [Interface web](Interface-Web) et [Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr).

## Une fenêtre distante affiche « the server refused the connection (HTTP 426) »

Une fenêtre connectée avec **Fichier > Se connecter à distance...** et le `rookparcel-nox` auquel elle se connecte doivent avoir la même version : le message (affiché en anglais) se termine par « this server speaks remote protocol ... ». Certaines versions changent le protocole distant (la dernière fois dans la 1.2.70) : après avoir mis l'un des côtés à jour vers la 1.2.76, mettez aussi l'autre à jour : la fenêtre de bureau, et le programme `rookparcel-nox` ou son image Docker. Un navigateur qui utilise l'interface web n'a pas cette limite. Voir [Contrôle à distance](Contrôle-à-distance) et [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel).

## La fermeture de Rookparcel prend beaucoup de temps

Rookparcel enregistre la liste des torrents et leur progression, prévient les trackers qu'il s'arrête et attend les fichiers qui sont encore en cours de déplacement. Quand cela prend plus qu'un instant, la fenêtre « Fermeture de Rookparcel » indique ce qu'il attend. **Quitter maintenant** cesse d'attendre ; elle demande d'abord confirmation et liste ce à quoi vous renoncez. La liste des torrents et la progression sont de toute façon enregistrées en premier.

Sous Linux, la fenêtre GTK se ferme de la même façon lorsqu'elle reçoit <kbd>Ctrl</kbd>+<kbd>C</kbd> dans son terminal ou un `SIGTERM`, par exemple envoyé par `kill`. Renvoyer le signal n'abrège pas la fermeture. Pour s'arrêter tout de suite, utilisez **Quitter maintenant**, ou `kill -KILL` avec l'identifiant du processus. *Depuis la 1.2.54 ; auparavant, un second signal quittait aussitôt sans enregistrer.*

## Une copie portable ne peut pas écrire dans son dossier

« Ce Rookparcel portable ne peut pas écrire dans son dossier » signifie que le dossier qui contient `rookparcel.exe` et `rookparcel.ini` est en lecture seule pour vous, par exemple sous `C:\Program Files`. Déplacez le dossier là où vous pouvez écrire, ou supprimez `rookparcel.ini` à côté du programme pour garder les paramètres dans votre dossier utilisateur. Voir [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel).

## L'assistant de recherche ne fonctionne pas

Voir le tableau à la fin de [Rechercher des torrents sous Windows](Rechercher-des-torrents-sous-Windows) et [Rechercher des torrents](Rechercher-des-torrents).

## Remarques pour Linux

- **Pas d'icône dans la zone de notification sous GNOME.** GNOME simple n'a pas de zone de notification ; installez l'extension AppIndicator, ou utilisez Rookparcel sans elle (fermer la fenêtre le quitte alors). Voir [Zone de notification, notifications et démarrage](Zone-de-notification-et-démarrage#version-gtk-linux-bsd).
- **Réduire dans la zone de notification ne fait rien** sur certains bureaux Wayland, qui ne signalent pas qu'une fenêtre a été réduite.
- **Se déconnecter sous Xfce** n'attend pas que Rookparcel ait fini d'enregistrer ; quittez-le d'abord.

## Remarques pour macOS

- **Fermer la fenêtre ne quitte pas.** Avec **Fermer dans la zone de notification** activé (par défaut), la fenêtre se masque et Rookparcel continue de fonctionner dans la barre des menus. Quittez avec <kbd>⌘</kbd>+<kbd>Q</kbd>. Voir [Zone de notification, notifications et démarrage](Zone-de-notification-et-démarrage#macos).
- **Pas de notifications.** Autorisez Rookparcel dans **Réglages Système > Notifications**.
- **Arrêt automatique ne met pas le Mac en veille et ne l'éteint pas.** Autorisez Rookparcel à contrôler « loginwindow » dans **Réglages Système > Confidentialité et sécurité > Automatisation**. Voir [Arrêt automatique](Arrêt-automatique).
- **Paramètres disparus après le remplacement de la version GTK.** La nouvelle app ne reprend l'ancien dossier `gtk` que tant qu'aucun dossier `mac` n'existe. Si un dossier `mac` a été créé en premier, quittez Rookparcel et consultez [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel#macos).

## Où se trouvent les journaux

**Les versions Windows et GTK** n'écrivent aucun fichier journal. Leurs messages (erreurs, filtre IP, recherche de mises à jour, planificateur, téléchargements terminés) vont dans l'onglet **Journal** en bas de la fenêtre principale, qui garde les 1000 dernières lignes tant que Rookparcel fonctionne. Regardez-le juste après que le problème se produit, avant de quitter, et notez les lignes qui s'y rapportent.

**rookparcel-nox** affiche ses messages dans le terminal. Pour plus de détails, lancez-le avec `--verbose`, qui ajoute les événements des trackers et des pairs (chaque déconnexion avec sa raison). `--log FICHIER` ajoute tout à la fin d'un fichier. Quand il fonctionne comme service :

| Où | Journal |
|---|---|
| Service Windows | `nox.log` dans le dossier d'état, `%ProgramData%\Rookparcel\nox` sauf si `--state-dir` en décide autrement (à lire depuis une invite élevée) |
| systemd | le journal : `journalctl -u rookparcel-nox` |
| FreeBSD rc.d | `/var/log/rookparcel-nox.log` |

Voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

**L'assistant de recherche** (Jackett) garde ses propres paramètres et son journal dans le dossier `data` situé dans `search-helper`, dans votre dossier de paramètres de Rookparcel.

## Réinitialiser les paramètres

- **Un réglage :** dans **Préférences > Avancé**, sélectionnez-le et cliquez sur **Réinitialiser**, puis sur **OK**. Les lignes marquées `*` diffèrent de la valeur par défaut.
- **Un code PIN de verrouillage oublié :** maintenez <kbd>Ctrl</kbd> (Windows) ou <kbd>⌥</kbd> (macOS) pendant que vous lancez Rookparcel, ou lancez-le avec `--reset-lock`, et confirmez. Voir [Verrouiller Rookparcel](Verrouiller-Rookparcel#code-pin-oublié).
- **Les questions du premier démarrage :** **Options > Assistant de configuration** (<kbd>Ctrl</kbd>+<kbd>G</kbd>) les pose de nouveau. *Nouveauté de la 1.1.62.*
- **Tout :** quittez Rookparcel (**Fichier > Quitter**), puis renommez le dossier où il garde ses paramètres et ses torrents (sous Linux et BSD, le dossier des paramètres et le dossier des données) ; Rookparcel démarre alors comme neuf, avec une liste de torrents vide. Sous Windows, **Aide > À propos de Rookparcel** indique le dossier. Lui rendre son nom d'origine restaure tout. Voir [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel).

## Signaler un bogue

Ouvrez un ticket sur le suivi des tickets : <https://github.com/Balls0fSteel/Rookparcel/issues>. Veuillez y indiquer :

1. Votre système (Windows 11, Ubuntu 24.04, macOS 15...) et le fichier que vous avez téléchargé.
2. La version de Rookparcel, depuis **Aide > À propos de Rookparcel**.
3. Ce que vous avez fait, ce que vous attendiez et ce qui s'est passé à la place, étape par étape.
4. Pour un plantage ou un torrent bloqué, les lignes de l'onglet **Journal**, ou la sortie de `rookparcel-nox run --verbose`.

Ne mentionnez ni mots de passe, ni clés d'API, ni rien d'autre de privé ; le suivi des tickets est public.

## Signaler un problème de sécurité

N'ouvrez pas de ticket public. Utilisez le signalement privé de vulnérabilités de GitHub : <https://github.com/Balls0fSteel/Rookparcel/security/advisories/new>. Voir [SECURITY.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/SECURITY.md).

## FAQ

**Rookparcel est-il gratuit ?** Gratuit pour un usage non commercial sous la licence PolyForm Noncommercial 1.0.0 : usage personnel, étude, projets de loisir, associations, écoles et institutions publiques, sans frais. L'usage commercial demande une autorisation séparée. Voir [LICENSE.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/LICENSE.md).

**Est-il lié à µTorrent ou à qBittorrent ?** Non. Rookparcel recrée l'apparence et le comportement de µTorrent 2.2.1 mais n'utilise aucun de son code. Son moteur est un portage indépendant en Rust de libtorrent-rasterbar 2.0.11. Son API web suit la documentation publiée de qBittorrent pour que des outils tels que Sonarr puissent dialoguer avec lui.

**Quelles langues parle-t-il ?** L'anglais et 34 autres, toutes intégrées. Il suit la langue de votre système ; choisissez-en une autre dans **Préférences > Général**. Les traductions sont automatiques ; les corrections sont bienvenues sous forme de tickets.

**Qu'envoie Rookparcel, et à qui ?** La recherche de mises à jour télécharge un petit fichier depuis le site web du projet, sans rien sur vous ni sur vos torrents ; désactivez-la dans **Préférences > Général > Confidentialité**. Les recherches ne vont qu'aux sources de recherche que vous avez ajoutées. Comme tout client BitTorrent, Rookparcel montre votre adresse IP aux trackers, au DHT et aux pairs de vos torrents. Voir [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel).

**Où sont mes paramètres et mes torrents ?** Voir [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel).

**Puis-je lancer plusieurs copies ?** Oui, sous forme de profils. Voir [Profils](Profils).

**Où obtenir de l'aide ?** Lisez la [FAQ du site web](https://balls0fsteel.github.io/Rookparcel/faq.html) et les pages de ce wiki, ou posez la question sur le [suivi des tickets](https://github.com/Balls0fSteel/Rookparcel/issues).
