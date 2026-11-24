# Disque et stockage

*Cette page est une traduction de la page anglaise [Disk and storage](Disk-and-storage). En cas de différence, la page anglaise fait foi.*

Cette page explique où Rookparcel place les téléchargements et comment il les déplace, ainsi que la façon dont il traite vos disques : E/S disque intelligentes (vérifications en douceur, réglage selon le type de lecteur, ralentissement quand un disque est occupé) et cache de lecture du disque.

## Où vont les téléchargements

Dans **Options > Préférences > Dossiers**, sous **Emplacement des fichiers téléchargés** :

| Paramètre | Signification |
|---|---|
| **Placer les nouveaux téléchargements dans :** | Le dossier où sont enregistrés les nouveaux torrents, sauf si vous en choisissez un autre dans la fenêtre Ajouter un nouveau torrent. |
| **Déplacer les téléchargements terminés vers :** | Quand un torrent a fini de télécharger, ses fichiers sont déplacés vers le dossier indiqué en dessous et il continue de partager depuis cet emplacement. |
| **Ajouter l'étiquette du torrent** | À côté de la case ci-dessus : un torrent terminé qui a une étiquette est déplacé dans un dossier portant le nom de l'étiquette, à l'intérieur du dossier des téléchargements terminés. |
| **Ne déplacer que depuis le dossier de téléchargement par défaut** | Ne déplace que les torrents enregistrés dans le dossier des nouveaux téléchargements ; les torrents que vous avez enregistrés ailleurs restent où ils sont. Activé par défaut. |

L'Assistant de configuration demande le dossier de téléchargement au premier démarrage : voir [Premier démarrage](Premier-démarrage).

La même page conserve aussi des copies des fichiers .torrent et les charge depuis un dossier surveillé (**Emplacement des .torrent**) : voir [Ajouter des torrents](Ajouter-des-torrents).

### Déplacer les téléchargements terminés

1. Cochez **Déplacer les téléchargements terminés vers :** et choisissez le dossier.
2. Cochez **Ajouter l'étiquette du torrent** si vous triez par étiquettes, par exemple `D:\Done\Movies` pour les torrents portant l'étiquette « Movies ». Un torrent sans étiquette va directement dans le dossier.
3. Décidez pour **Ne déplacer que depuis le dossier de téléchargement par défaut** (ci-dessus).
4. Cliquez sur **OK**.

Cela s'applique aux torrents qui se terminent à partir de maintenant ; ceux qui se sont terminés avant restent où ils sont. Pendant le déplacement des fichiers d'un torrent, son état affiche **Déplacement** et le **Journal** indique « déplacement du téléchargement terminé vers ... ». L'étiquette qu'a un torrent au moment où il se termine est prise en compte : réglez donc les étiquettes avant la fin du téléchargement.

### Déplacer un torrent soi-même

Faites un clic droit sur des torrents et choisissez **Avancé > Définir l'emplacement de téléchargement...** pour choisir un dossier et y déplacer leurs fichiers. Les fichiers de même nom déjà présents dans ce dossier sont remplacés. L'élément est grisé tant que le torrent est déjà en cours de déplacement. Un fichier isolé d'un torrent peut être déplacé depuis **Déplacer...** de l'onglet Fichiers : voir [Détails et fichiers des torrents](Détails-et-fichiers-des-torrents).

### Fermer pendant le déplacement de fichiers

Si vous quittez pendant que des fichiers sont déplacés, la fenêtre de fermeture attend la fin des déplacements, avec leur progression, car s'arrêter au milieu pourrait laisser un torrent moitié dans un dossier, moitié dans l'autre. **Quitter maintenant** demande d'abord confirmation et indique ce qui serait annulé : les fichiers du torrent restent dans l'ancien dossier, ce qui a été copié vers le nouveau est supprimé, et vous pouvez le déplacer de nouveau plus tard.

### Fichiers incomplets et .!ut

*Nouveau dans la version 1.2.63.* Quand **Options > Préférences > Général > Ajouter .!ut aux fichiers incomplets** est coché (sous **Pendant le téléchargement**, désactivé par défaut), un fichier encore en cours de téléchargement est stocké avec `.!ut` après son nom, par exemple `film.mkv.!ut`. Dès que tout le fichier a été téléchargé et a passé sa vérification, il reçoit son nom normal, pendant que le reste du torrent continue.

- **Pourquoi :** les lecteurs multimédias, les surveillants de fichiers et [Sonarr et Radarr](Sonarr-Radarr-et-Prowlarr) ne voient un fichier sous son vrai nom qu'une fois qu'il est complet ; rien n'ouvre ni n'importe donc la moitié d'un fichier. C'est ainsi que µTorrent nomme les fichiers inachevés.
- **Activer ou désactiver** renomme tout de suite les fichiers des torrents en cours, et ceux des torrents arrêtés à leur prochain démarrage.
- **Ce qui ne le reçoit jamais :** les fichiers vides, les fichiers de remplissage que certains torrents contiennent, un fichier dont le nom serait trop long avec le suffixe, et un fichier dont le nom avec le suffixe est celui d'un autre fichier ou dossier du même torrent (un torrent contenant à la fois `x` et `x.!ut`) : il garde son nom simple. Un fichier que vous avez renommé dans l'onglet Fichiers reçoit `.!ut` après son nouveau nom.
- **Onglet Fichiers :** **Ouvrir** et **Ouvrir le dossier** trouvent le fichier sous son nom en `.!ut`. L'interface Web et son API affichent les noms simples.
- **Un fichier qu'un autre programme a ouvert** (sous Windows, par exemple un lecteur qui ne le partage pas) garde son nom actuel jusqu'à ce qu'il puisse être renommé. Le Journal l'indique une fois, et Rookparcel réessaie environ toutes les 30 secondes et au prochain démarrage. Le torrent continue de fonctionner pendant ce temps.
- **Un fichier existant n'est jamais écrasé.** Si `film.mkv` et `film.mkv.!ut` sont tous deux dans le dossier, le fichier terminé reste `film.mkv.!ut`. Le Journal l'indique une fois. Supprimez l'autre fichier, puis mettez le torrent en pause et reprenez-le pour qu'il soit renommé.
- **Déplacer...** dans l'onglet Fichiers, pour un fichier pas encore téléchargé, prend le fichier désigné sous le nom qu'il porte là : `film.mkv`, `film.mkv.!ut` ou `film.mkv.!qB`. Lancez **Forcer la vérification** pour que ses morceaux soient comptés.
- **Définir l'emplacement de téléchargement** remplace un fichier de même nom dans le nouveau dossier, y compris un `film.mkv` s'y trouvant alors que le fichier déplacé est encore `film.mkv.!ut`. Un fichier qui n'était pas encore téléchargé est pris dans le nouveau dossier sous celui de ces noms qu'il y porte.

**Si vous venez de µTorrent ou de qBittorrent.** Que la case soit cochée ou non, un fichier absent sous le nom attendu est cherché avec `.!ut` (le suffixe de µTorrent) et avec `.!qB` (l'option « Append .!qB extension to incomplete files » de qBittorrent). Un torrent [importé](Importer-depuis-uTorrent-et-qBittorrent) ou ajouté de nouveau avec **Enregistrer dans** réglé sur son ancien dossier reprend donc à partir de ses fichiers partiels au lieu de recommencer. Ils sont ensuite renommés selon le nom que Rookparcel leur donne : `.!ut` tant qu'ils sont inachevés avec la case cochée, le nom simple sinon. Le dossier séparé de qBittorrent pour les fichiers incomplets et ses dossiers `.unwanted` ne sont pas consultés.

**Supprimer et > Effacer les données** ne supprime un fichier `.!ut` ou `.!qB` que s'il contient des données de ce torrent : Rookparcel y a écrit, ou des morceaux qu'il contient ont passé leur vérification. Un fichier partiel de ce nom que le torrent n'a jamais touché (celui d'un autre programme, ou repéré mais pas encore vérifié) est conservé, et le Journal le nomme.

## Lecteur ou partage absent

*Nouveau dans la version 1.2.76.* Si un torrent qui avait téléchargé des données ne trouve aucun de ses fichiers au démarrage, ou si son dossier d'enregistrement a disparu (disque USB débranché, partage réseau ou point de montage absent, lettre de lecteur pas encore prête), Rookparcel ne télécharge pas les données de nouveau. Le torrent s'arrête avec une erreur qui commence par le chemin manquant et dit **files missing (the drive, share or folder is not available)**, et il conserve les morceaux qu'il avait. Toutes les 30 secondes, Rookparcel regarde de nouveau, et dès que le dossier est de retour, le torrent démarre tout seul et continue sans revérifier les fichiers. Il en va de même quand le lecteur disparaît pendant qu'un torrent télécharge. Fermer Rookparcel entre-temps ne change rien : au prochain démarrage, le torrent est de nouveau trouvé absent, ou de retour. **Démarrer** sur un torrent retenu réessaie tout de suite, et il est de nouveau retenu si les fichiers manquent toujours. Un torrent retenu a une erreur, donc l'[Arrêt automatique](Arrêt-automatique) ne l'attend pas.

Si seuls certains fichiers d'un torrent ont disparu (supprimés à la main ou par un antivirus), seuls leurs morceaux sont téléchargés de nouveau. Si vous supprimez à la main tout le dossier d'un torrent, le torrent est lui aussi retenu. Pour renoncer aux anciennes données d'un torrent retenu et le télécharger à nouveau, ou pour prendre ce qui se trouve maintenant sur le disque, utilisez **Forcer la vérification** : elle vérifie les fichiers présents et télécharge le reste.

## E/S disque intelligentes

Les E/S disque intelligentes gardent l'ordinateur réactif pendant que Rookparcel lit et écrit. Leurs paramètres se trouvent dans **Options > Préférences > Jeux** (une page juste au-dessus d'Avancé dans la liste des pages).

La case **Utiliser les E/S disque intelligentes** de l'Assistant de configuration active ou désactive d'un coup les quatre parties. Le réglage par lecteur est actif dès le départ ; la case ajoute les vérifications en douceur et le ralentissement quand un disque est occupé. *L'Assistant de configuration est nouveau dans la version 1.1.62. Les paramètres existent depuis la version 1.1.40 ; depuis la 1.2.79 ils sont sur la page Jeux, et les vérifications en douceur sont désactivées au départ.*

### Vérification

Une vérification (par exemple après **Forcer la vérification**) lit chaque morceau sur le disque et le compare au torrent. Sous **Vérification** :

| Paramètre | Signification | Par défaut |
|---|---|---|
| **Vérifier les torrents en douceur (basse priorité, laisser la place aux autres programmes)** | Vérifie avec la priorité disque d'arrière-plan du système, laisse tranquilles les fichiers en cache des autres programmes et s'interrompt tant que les lectures sont plus lentes que d'habitude, pour que l'ordinateur reste réactif. Les vérifications durent plus longtemps quand le disque est occupé. | Désactivé |
| **Limiter la vérification à (Mo/s) :** | Plafonne la vitesse de lecture de chaque vérification, pour un disque qui ralentit encore pendant les vérifications en douceur. Ici, 1 Mo vaut 1 000 000 d'octets. | Désactivé, 100 |
| **Lecteurs vérifiés en même temps (un torrent par lecteur) :** | Combien de lecteurs sont vérifiés en même temps, un torrent par lecteur ; les torrents du même lecteur attendent leur tour. De 1 à 8. | 4 |

Les vérifications de torrents situés sur des lecteurs différents se déroulent en parallèle ; deux torrents d'un même lecteur sont vérifiés l'un après l'autre, pour que le lecteur ne saute pas de l'un à l'autre. L'état affiche **En attente de vérification** pour un torrent qui attend son tour, puis **Vérification** avec le pourcentage. *L'état « en attente » est nouveau dans la version 1.2.76 ; auparavant, une vérification en attente affichait « Vérification 0.0 % ».* Une vérification demandée avec **Forcer la vérification** n'attend pas une place libre dans la file des torrents ([Mise en file et partage](Mise-en-file-et-partage)).

*Nouveau dans la version 1.2.76.* Un fichier qui existe mais ne peut pas être lu (un autre programme le tient, ou vous n'en avez pas l'autorisation) arrête la vérification avec une erreur de fichier, au lieu de compter ses morceaux comme manquants et de les télécharger de nouveau. Un fichier qui n'est pas du tout là ne fait toujours perdre que ses propres morceaux.

### Lecteurs

| Paramètre | Signification | Par défaut |
|---|---|---|
| **Adapter au type de lecteur (SSD, disque dur, réseau)** | Détecte si chaque dossier d'enregistrement se trouve sur un SSD, un disque dur, un lecteur amovible ou un partage réseau. Un disque dur ou un lecteur amovible n'obtient alors que deux tâches disque à la fois au maximum, un serveur réseau quatre, afin qu'ils ne s'emballent pas. | Activé |

Le Journal liste les lecteurs trouvés au démarrage, comme « Disk I/O: drive D: (HDD, SATA) ». Les ponts USB, les contrôleurs RAID et les machines virtuelles signalent souvent le mauvais type.

**Corriger une mauvaise détection.** Utilisez `drive_overrides` dans **Options > Préférences > Avancé** : des paires `path=type` séparées par des virgules, le type étant l'un de `ssd`, `hdd`, `network` ou `removable`, par exemple `D:/=hdd, E:/=removable`. Le chemin correspondant le plus long l'emporte.

1. Ouvrez **Avancé** et tapez `drive_overrides` dans **Filtre :**.
2. Sélectionnez la ligne, tapez les paires dans **Valeur :** et cliquez sur **Définir**.
3. Cliquez sur **OK** ou **Appliquer**.

![Préférences, page Avancé](images/prefs-advanced.png)

### Quand le disque est occupé

Sous **Quand le disque est occupé**, les deux désactivés par défaut :

| Paramètre | Signification |
|---|---|
| **Ralentir les téléchargements (écritures sur le disque)** | Tant qu'un lecteur répond aux écritures nettement plus lentement que d'habitude, divise par deux la vitesse de téléchargement pas à pas, sans jamais descendre sous 50 ko/s, puis la laisse remonter quand le disque est calme. |
| **Ralentir les envois (lectures sur le disque)** | La même chose pour les envois, tant qu'un lecteur répond lentement aux lectures. Le partage envoie alors moins, ce qui compte sur les trackers qui comptabilisent votre ratio. |

Ils ne relèvent jamais vos propres limites. Le ralentissement est un plafond par direction, partagé avec [Garder un ping bas](Garder-un-ping-bas) : le plafond le plus bas l'emporte. Tant qu'il retient une direction, la barre d'état l'affiche sous la forme « (auto ...) », et le Journal a une ligne pour cela.

## Cache disque et fichiers

**Options > Préférences > Avancé > Cache disque** contient le cache de lecture et la file disque. La plupart des gens n'ont jamais besoin de les modifier.

Sous **Cache de lecture du disque** :

| Paramètre | Signification | Par défaut |
|---|---|---|
| **Activer la mise en cache des lectures disque** | Garde en mémoire les morceaux lus récemment, en lisant un bloc demandé par les pairs avec ses voisins, ce qui économise des déplacements de tête de disque pendant le partage. | Activé |
| **Remplacer la taille automatique du cache (Mo) :** | Utilise la taille indiquée à côté au lieu des 32 Mo automatiques. Un cache plus grand peut aider un partage intense, au prix de la mémoire. | Désactivé |
| **Retirer les blocs inutilisés depuis plus de (secondes) :** | Abandonne les blocs en cache qu'aucun pair n'a demandés pendant ce temps. Désactivé, ils restent jusqu'à ce que des blocs plus récents les chassent. | Activé, 300 |

Les écritures sont laissées au cache propre du système.

Sous **File disque et fichiers** :

| Paramètre | Signification | Par défaut |
|---|---|---|
| **File d'écriture disque (ko) :** | Quelle quantité de données téléchargées peut attendre d'être écrite sur le disque ; au-delà, Rookparcel cesse de lire depuis les pairs jusqu'à ce que le disque rattrape son retard. Les valeurs inférieures à 4 Mo valent 4 Mo. | 1024 |
| **Nombre maximum de fichiers ouverts :** | Le plus grand nombre de fichiers gardés ouverts en même temps ; les autres sont fermés et rouverts au besoin. Augmentez-le quand de nombreux torrents ou fichiers sont actifs en même temps. | 40 |
| **Threads d'E/S disque (au prochain démarrage) :** | Les threads qui lisent et écrivent les données des torrents. | 10 |
| **Threads de hachage (non utilisé) :** | Conservé pour la compatibilité ; cette version ne l'utilise pas. Les vérifications s'exécutent une par lecteur, réglage sur la page Jeux. | |

Rookparcel n'a pas de paramètre pour préallouer les fichiers. Un fichier est créé quand les premières données qui lui sont destinées arrivent, et sous Windows il est marqué comme fichier fragmenté (sparse), si bien qu'écrire près de sa fin n'amène pas NTFS à remplir de zéros la partie qui précède.

## Dans la fenêtre GTK et l'interface Web

- **Fenêtre GTK** (Linux, BSD), **macOS** et **interface Web** : **Ajouter .!ut aux fichiers incomplets** se trouve aussi sur la page Général, sous **Pendant le téléchargement**.
- **Application macOS** : les mêmes pages et paramètres ; les Préférences s'appliquent immédiatement, sans **OK** ([Référence des préférences](Référence-des-préférences#différences-sous-macos)).
- **Fenêtre GTK** (Linux, BSD) : les mêmes pages Dossiers, Cache disque et Jeux. Les types de lecteur sont aussi détectés sous Linux, macOS et FreeBSD, chacun avec les méthodes propres au système.
- **Interface Web** : les mêmes pages sous **Préférences**, avec **Cache disque** sous **Avancé** et les paramètres de disque sur **Jeux**. Les dossiers y sont ceux de l'ordinateur qui exécute Rookparcel, pas ceux du navigateur.
- **rookparcel-nox** : `--save DIR` définit le dossier de téléchargement et `--move-on-complete DIR` déplace les torrents terminés ; les autres paramètres se trouvent dans les Préférences de l'interface Web.

## Pages connexes

- [Mise en file et partage](Mise-en-file-et-partage)
- [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel) (paramètres et état des torrents, pas les téléchargements)
- [Gérer les torrents](Gérer-les-torrents)
- [Référence des préférences](Référence-des-préférences)
