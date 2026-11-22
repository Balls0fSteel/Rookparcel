# Contrôle à distance

*Cette page est une traduction de la page anglaise [Remote control](Remote-control). En cas de différence, la page anglaise fait foi.*

Le programme de bureau peut contrôler un `rookparcel-nox` sur un autre ordinateur : un NAS, un serveur domestique, un PC dans une autre pièce. La fenêtre a l'apparence et le fonctionnement habituels, mais les torrents, les téléchargements et le moteur se trouvent sur le serveur. Cette page explique comment se connecter avec **Fichier > Se connecter à distance...** ou `--remote`, ce qui fonctionne dans une fenêtre distante, et comment la connexion est protégée.

L'alternative est l'[Interface Web](Interface-Web) dans un navigateur, qui n'exige rien d'installé. Une fenêtre distante vous donne à la place le programme de bureau complet : ses boîtes de dialogue, ses raccourcis clavier et ses onglets de détails.

## Ce qu'il vous faut

- Un `rookparcel-nox` en fonctionnement avec son interface Web activée, par exemple `rookparcel-nox run --webui 0.0.0.0:8443 --webui-https` ([Utiliser rookparcel-nox](Utiliser-rookparcel-nox)). La fenêtre distante utilise le port et l'identifiant de l'interface Web.
- Le nom d'utilisateur et le mot de passe de l'interface Web.
- La même version de Rookparcel des deux côtés. Un serveur qui parle une autre version refuse la connexion avec « the server refused the connection (HTTP 426): this server speaks remote protocol ... ». Le protocole distant change de temps en temps (dernièrement dans la version 1.2.70), et la version 1.2.76 ne se connecte pas aux versions 1.2.63 à 1.2.69 : mettez donc à jour ensemble le serveur et les ordinateurs ayant des fenêtres distantes. L'interface Web et l'API compatible qBittorrent ne sont pas concernées.

Une fenêtre distante se connecte à `rookparcel-nox`. L'interface Web propre au programme Windows (**Préférences > Interface web**) est destinée aux navigateurs.

## Se connecter

1. Dans le programme Windows, choisissez **Fichier > Se connecter à distance...**. Une seconde fenêtre Rookparcel démarre à côté de la vôtre et affiche la boîte de dialogue **Se connecter à distance**. (Ou lancez `rookparcel.exe --remote`, qui n'ouvre que la fenêtre distante.)
2. Remplissez :

   | Champ | Ce qu'il faut saisir |
   |---|---|
   | **Hôte :** | Le nom ou l'adresse IP du serveur. |
   | **Port :** | Le port de l'interface Web (8080 est proposé). |
   | **Nom d'utilisateur :** | Le nom d'utilisateur de l'interface Web (`admin` est proposé). |
   | **Mot de passe :** | Le mot de passe de l'interface Web. |
   | **Utiliser HTTPS** | Coché par défaut. Laissez-le coché quand le serveur fonctionne avec `--webui-https` ou son propre certificat. |

3. Cliquez sur **Se connecter**.
4. La première fois, un serveur avec un certificat autosigné (celui que fabrique `--webui-https`) est présenté par son empreinte :

   > ... présente un certificat auquel cet ordinateur ne fait pas confiance (un certificat auto-signé, comme celui que crée rookparcel-nox --webui-https).

   Comparez le long nombre avec la ligne `certificate SHA-256` que nox a affichée à son démarrage (dans son terminal, son journal ou le journal systemd). S'ils correspondent, cliquez sur **Oui**. Rookparcel mémorise le certificat pour ce serveur et ne le redemande plus.
5. La fenêtre s'ouvre avec les torrents du serveur. Sa barre de titre indique le `host:port` du serveur.

La boîte de dialogue mémorise pour la fois suivante l'hôte, le port, le nom d'utilisateur et le choix HTTPS. Le mot de passe n'est jamais enregistré ; vous le saisissez à chaque connexion.

Dans le programme GTK (Linux, BSD), c'est pareil : **Fichier > Se connecter à distance...**, ou `rookparcel-gtk --remote`. L'application macOS a elle aussi **Fichier > Se connecter à distance...**. Chaque fenêtre distante est un programme à part entière : vous pouvez donc avoir en même temps votre fenêtre locale et plusieurs serveurs ouverts.

## Sans HTTPS

Si vous décochez **Utiliser HTTPS** et que l'hôte est un autre ordinateur, Rookparcel demande d'abord :

> ... est un autre ordinateur et, sans HTTPS, le nom d'utilisateur, le mot de passe et chaque commande transitent sur le réseau sans chiffrement : n'importe qui sur ce réseau peut les lire ou les modifier.

Répondez **Non** et activez plutôt HTTPS sur le serveur (`--webui-https`), sauf si le réseau entre les deux est entièrement à vous. Une connexion vers le même ordinateur (`localhost`, `127.0.0.1`) ne donne lieu à aucune question.

## Ce qui fonctionne dans une fenêtre distante

Presque tout fonctionne comme dans une fenêtre locale, en agissant sur le serveur :

- Ajouter des torrents : les fichiers `.torrent` de cet ordinateur sont envoyés au serveur ; les liens magnet aussi.
- Démarrer, arrêter, mettre en pause, forcer le démarrage, supprimer (aussi avec les données, effacées sur le serveur), revérifier, ordre de la file, étiquettes, tags, limites de vitesse, trackers, priorités des fichiers, renommage, Propriétés du torrent.
- Les onglets de détails, avec les pairs, les pièces, les fichiers et le graphique de vitesse du serveur.
- **Copier le lien magnet** et **Exporter le .torrent...** (*nouveau dans la version 1.2.63*). Un fichier `.torrent` exporté est enregistré sur cet ordinateur.
- **Préférences :** les paramètres du moteur (connexion, bande passante, BitTorrent, file d'attente et autres) sont ceux du serveur. Ce que vous y modifiez est appliqué sur le serveur et conservé là-bas.

Ce qui est différent :

| | Dans une fenêtre distante |
|---|---|
| Dossiers | Les dossiers sont sur le serveur. Les sélecteurs de dossier pour les torrents sont désactivés ; **Définir l'emplacement de téléchargement...** demande un dossier du serveur sous forme de texte. Les nouveaux téléchargements vont dans le dossier par défaut du serveur. |
| **Ouvrir le dossier** | Désactivé : le dossier est sur le serveur. |
| Création de torrents | Un torrent que vous créez ici ne peut pas être partagé par le serveur, puisque les fichiers sont sur cet ordinateur. |
| Filtre IP | Le filtre IP propre au serveur s'applique ; celui de la fenêtre n'est pas envoyé. |
| Importation | **Fichier > Importer** ne fait rien (grisé dans les programmes Windows et macOS) : le serveur ne reprend pas les torrents de l'état enregistré d'un autre client. |
| Limites de dossiers | Un serveur lancé avec `--webui-save-root` refuse un ajout ou un déplacement vers un dossier extérieur à ces racines, comme il le fait pour l'interface Web. Un renommage qui sortirait du dossier du torrent est refusé aussi. |
| **Préférences > Interface web** | Une fenêtre distante ne peut pas servir l'interface Web : « Une fenêtre distante ne peut pas servir l'interface web ; utilisez les Préférences de la fenêtre locale. » |
| Étiquettes, tags, flux RSS, paramètres de la fenêtre | Conservés sur cet ordinateur, séparément pour chaque serveur (voir plus bas). |
| Planificateur, Téléchargeur RSS, objectif de partage, dossiers `.torrent` | Ceux de la fenêtre fonctionnent tant qu'elle est ouverte et agissent sur les torrents du serveur, en plus de tout ce que nox fait lui-même. |

Le serveur continue de télécharger après la fermeture de la fenêtre.

## Perdre la connexion

Quand la connexion tombe, Rookparcel le signale une fois :

> La connexion à ... a été perdue : ...
>
> La liste montre les torrents tels qu'ils étaient. Fermez cette fenêtre et utilisez Fichier > Se connecter à distance... pour vous reconnecter.

La barre de titre se termine alors par « (déconnecté) ». Les commandes ne vont plus nulle part à partir de là. Fermez la fenêtre et reconnectez-vous.

## Quand la connexion échoue

| Message | Ce qu'il faut vérifier |
|---|---|
| « Saisissez le nom d'hôte ou l'adresse du serveur, et son port (1 à 65535). » | L'hôte est vide ou le port n'est pas un nombre valide. |
| « Impossible de se connecter à ... : wrong user name or password » | L'identifiant de l'interface Web. Trop d'essais erronés verrouillent l'adresse pendant un moment ([Interface Web](Interface-Web#connexion)). |
| « ... Est-ce bien un rookparcel-nox avec son interface web sur ce port (--webui) ? » | Autre chose a répondu sur ce port. |
| « ... Si l'interface web du serveur utilise HTTPS (--webui-https), cochez Utiliser HTTPS. » | Le serveur parle HTTPS mais **Utiliser HTTPS** était décoché. |
| « Impossible de joindre ... en HTTPS : ... » | **Utiliser HTTPS** est coché, mais le serveur ne parle pas HTTPS, ou n'est pas joignable du tout. |
| « Le certificat de ... n'est pas celui que vous aviez accepté : ... » | Le certificat du serveur a changé. Si vous l'avez remplacé, ou si nox a renouvelé le sien, reconnectez-vous pour vérifier le nouveau. Sinon, quelqu'un s'est peut-être interposé : n'allez pas plus loin. |
| « ... (HTTP 400): ... » nommant l'en-tête Host | Le serveur refuse le nom d'hôte que vous avez saisi. Utilisez son adresse IP, ou ajoutez le nom aux **Préférences > Interface web > Domaines du serveur** du serveur ([Interface Web](Interface-Web#sécurité)). |
| « ... (HTTP 426): this server speaks remote protocol ... » | Les deux programmes sont de versions différentes. Mettez à jour le plus ancien ([Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel)). |

Le pare-feu du serveur doit laisser entrer le port de l'interface Web.

## Où la fenêtre distante conserve ses fichiers

| | Windows | GTK (Linux, BSD) |
|---|---|---|
| Détails de connexion et certificats acceptés | `remote.ini` à côté de `rookparcel.ini` dans le dossier des paramètres | `remote.ini` dans `~/.config/rookparcel/gtk` |
| Les paramètres de la fenêtre, étiquettes, tags et flux RSS par serveur | `remote\<host>_<port>` dans le dossier des paramètres | `~/.config/rookparcel/gtk/remote/<host>_<port>` |

Ceux-ci ne se mélangent jamais aux paramètres de votre fenêtre locale. Voir [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel).

## Sécurité

- La fenêtre distante se connecte avec l'identifiant de l'interface Web ; tout ce qui figure dans [Sécurité de l'interface Web](Interface-Web#sécurité) s'applique donc : un mot de passe long, HTTPS à travers un réseau.
- Les mauvais mots de passe comptent pour le même verrouillage que le formulaire de connexion de l'interface Web.
- Définir un nouveau mot de passe de l'interface Web déconnecte toutes les fenêtres distantes.
- Un certificat qui change après que vous l'avez accepté est refusé avec un avertissement ; Rookparcel redemande à la tentative suivante.
- Le mot de passe n'est jamais écrit sur le disque de cet ordinateur.

## Voir aussi

- [Utiliser rookparcel-nox](Utiliser-rookparcel-nox) : lancer le serveur, options HTTPS et services.
- [Interface Web](Interface-Web) : le même serveur dans un navigateur.
- La description technique : [WEBUI.md, Remote mode](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/WEBUI.md#remote-mode).
