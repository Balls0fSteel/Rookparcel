*Ceci est la traduction française de la page [First start and the Setup Guide](First-start-and-the-Setup-Guide) du wiki anglais. La page anglaise fait foi.*

# Premier démarrage et assistant de configuration

Cette page décrit ce qui se passe la première fois que vous lancez Rookparcel : l'assistant de configuration, une courte fenêtre de quatre questions, ce que fixe chaque réponse, et comment rouvrir l'assistant plus tard.

*Nouveau dans la 1.1.62.* Jusqu'à la 1.1.40, il n'y a pas d'assistant : Rookparcel s'ouvre avec ses réglages par défaut, et tout ce qui suit se règle dans **Options > Préférences**.

## Quand l'assistant s'ouvre

L'assistant de configuration s'ouvre tout seul au premier démarrage de Rookparcel : quand son fichier de réglages ne contient encore aucun réglage. C'est le cas d'une nouvelle installation, d'un nouveau dossier portable et de chaque nouveau [profil](Profils). Il s'ouvre avant la fenêtre principale, donc rien ne commence à se télécharger avant que vous ayez répondu. Un fichier de réglages qui existe mais ne peut pas être lu ne compte pas comme un premier démarrage.

Il ne s'ouvre pas tout seul pour quelqu'un qui a déjà utilisé une version plus ancienne : les réglages existent. Ouvrez-le alors depuis le menu (voir plus bas).

![La première page de l'assistant de configuration](images/setup-guide.png)

## Se déplacer dans l'assistant

Chaque page a un titre, « Étape 1 sur 4 » au-dessus, un court texte et ses réglages. Les boutons :

| Bouton | Ce qu'il fait |
|---|---|
| **< Précédent** | la page d'avant |
| **Suivant >** | la page suivante |
| **Terminer** | enregistre vos réponses et ferme l'assistant |
| **Ignorer** | ferme l'assistant et garde les réglages tels qu'ils sont. <kbd>Esc</kbd> et la case de fermeture de la fenêtre font de même |

Rien n'est enregistré avant **Terminer**, et **Terminer** ne change que les réponses que vous avez modifiées. Posez la souris sur un réglage pour voir ce qu'il fait ; l'aide s'affiche aussi sous les réglages.

Après **Terminer** ou **Ignorer**, l'assistant ne s'ouvre plus tout seul.

## Page 1 : où vont les téléchargements

Choisissez le dossier où les nouveaux téléchargements sont enregistrés.

| Réglage | Signification |
|---|---|
| **Placer les nouveaux téléchargements dans :** | le dossier des nouveaux téléchargements. Par défaut, c'est votre dossier Téléchargements ; dans un dossier portable, c'est le dossier `Downloads` qu'il contient |
| **Déplacer les téléchargements terminés vers :** | quand elle est cochée, les téléchargements terminés passent d'eux-mêmes dans le dossier voisin, d'un disque rapide vers un grand disque, par exemple |

**Options > Préférences > Dossiers** propose d'autres réglages de dossiers. Voir [Disque et stockage](Disque-et-stockage).

## Page 2 : votre routeur

Chaque connexion à un pair occupe une place dans la table de connexions de votre routeur. Les routeurs anciens et les box de base des fournisseurs d'accès ont de petites tables ; quand l'une est pleine, le routeur coupe des connexions, ralentit toute la maison ou redémarre.

**Quel routeur avez-vous ?**

- **Récent (acheté après 2015 environ), ou je ne sais pas :** les nombres de connexions habituels de Rookparcel.
- **Ancien, ou la box de base du fournisseur d'accès :** moins de connexions : 100 au total, 40 par torrent et 8 nouvelles par seconde.

La réponse « Ancien » est un point de départ. Vous pouvez changer ces nombres plus tard dans **Options > Préférences > Bande passante** et **Avancé**. Répondre « Récent » plus tard remet les nombres habituels.

La même page propose **Garder un ping bas (limiter les vitesses automatiquement)**. Cochez-la si vous jouez ou téléphonez pendant que Rookparcel tourne : elle freine les vitesses tant que votre ligne est chargée. Voir [Garder un ping bas](Garder-un-ping-bas).

## Page 3 : vos disques

**Utiliser les E/S disque intelligentes** garde l'ordinateur réactif pendant que Rookparcel lit et écrit :

- les vérifications tournent avec la priorité disque d'arrière-plan du système ;
- chaque lecteur est réglé selon son type (SSD, disque dur, partage réseau) ;
- les transferts ralentissent tant qu'un disque répond lentement.

Les vérifications en douceur et le réglage par lecteur sont actifs dès le départ. La case ajoute le ralentissement quand un disque est occupé. Les vérifications durent plus longtemps quand autre chose utilise le disque. **Options > Préférences > Avancé > E/S disque** règle chaque élément séparément. Voir [Disque et stockage](Disque-et-stockage).

## Page 4 : cet ordinateur

Rookparcel peut démarrer à l'ouverture de votre session, et ouvrir les fichiers `.torrent` et les liens `magnet:` depuis votre navigateur et votre gestionnaire de fichiers.

| Réglage | Signification |
|---|---|
| **Lancer Rookparcel au démarrage du système** | lance Rookparcel à l'ouverture de votre session |
| **Associer aux fichiers .torrent** | un double-clic sur un fichier `.torrent` l'ouvre dans Rookparcel |
| **Associer aux liens magnet:** | un clic sur un lien `magnet:` dans le navigateur l'ouvre dans Rookparcel |

**Options > Préférences > Général** permet de les changer plus tard. Voir [Zone de notification et démarrage](Zone-de-notification-et-démarrage).

Cette page est omise là où elle ne s'applique pas :

- **L'installeur Windows** a déjà posé la question des associations, donc une copie installée affiche trois pages à son premier démarrage. La page est là quand vous ouvrez l'assistant depuis le menu.
- **Une fenêtre connectée à un `rookparcel-nox` distant** affiche trois pages quand vous ouvrez l'assistant depuis le menu.
- **Le Flatpak** n'affiche que la case du démarrage : c'est le bureau qui décide quel programme ouvre les fichiers `.torrent` et les liens `magnet:`. Il en va de même sous macOS là où Rookparcel ne peut pas poser lui-même les associations (un profil nommé, ou un binaire seul en dehors de `Rookparcel.app`).

## Rouvrir l'assistant

Choisissez **Options > Assistant de configuration**, ou appuyez sur <kbd>Ctrl</kbd>+<kbd>G</kbd>. L'assistant montre vos réglages actuels, et **Terminer** applique les réponses que vous avez modifiées, comme **OK** dans les Préférences.

## GTK et interface web

- La version GTK (Linux, BSD) a le même assistant, avec les mêmes pages, au premier démarrage de chaque profil et sous **Options > Assistant de configuration** (<kbd>Ctrl</kbd>+<kbd>G</kbd>).
- L'application macOS l'a aussi, sous **Rookparcel > Assistant de configuration** (<kbd>⌘</kbd>+<kbd>G</kbd>).
- `rookparcel-nox` et l'interface web n'ont pas d'assistant. Réglez-les dans les Préférences de l'interface web : voir [Interface web](Interface-Web) et [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

## Après l'assistant

La fenêtre principale s'ouvre. Étapes suivantes :

- [Ajouter des torrents](Ajouter-des-torrents)
- [La fenêtre principale](La-fenêtre-principale)
- [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu), si les téléchargements restent lents
- [Référence des préférences](Référence-des-préférences), pour tous les autres réglages
