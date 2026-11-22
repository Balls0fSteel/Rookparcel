*Cette page est la traduction de la page anglaise [Creating torrents](Creating-torrents). En cas de divergence, la page anglaise fait foi.*

# Créer des torrents

Cette page montre comment créer un nouveau fichier `.torrent` à partir d'un fichier ou d'un dossier de votre ordinateur avec la fenêtre Créer un nouveau torrent, ce que signifie chaque option, et comment en commencer le partage tout de suite.

![La fenêtre Créer un nouveau torrent](images/create-torrent.png)

## Créer un torrent en cinq étapes

1. Choisissez **Fichier > Créer un nouveau torrent...** (<kbd>Ctrl</kbd>+<kbd>N</kbd>), ou le bouton **Créer un nouveau torrent** de la barre d'outils.
2. Sous **Choisir la source**, cliquez sur **Ajouter un fichier** pour un fichier seul ou sur **Ajouter un dossier** pour un dossier. Vous pouvez aussi saisir ou coller le chemin.
3. Sous **Propriétés du torrent**, ajoutez les trackers voulus, un par ligne. Sans trackers, les pairs ne peuvent se trouver que par le DHT et l'échange de pairs.
4. Laissez **Commencer le partage** coché pour le partager tout de suite.
5. Cliquez sur **Créer et enregistrer sous...**, choisissez où enregistrer le fichier `.torrent`, et attendez la barre de progression.

Le nom de fichier proposé est le nom de la source suivi de `.torrent`. Quand le calcul des empreintes est terminé, la fenêtre se ferme. Partagez le fichier `.torrent` avec les personnes qui doivent le télécharger.

## Les options

### Choisir la source et Ignorer des fichiers

**Choisir la source** est le fichier ou le dossier à partir duquel le torrent est créé. Le nom d'un dossier devient le nom du torrent.

**Ignorer des fichiers** exclut du torrent des fichiers et des dossiers selon leur nom. Écrivez des motifs avec jokers, `*` pour une suite quelconque de caractères et `?` pour un seul, séparés par `|` ou `;`. La casse n'importe pas, et un motif doit correspondre au nom entier d'un fichier ou d'un dossier, pas à son chemin. Par exemple, `*.nfo|Thumbs.db|*.tmp` ignore les fichiers NFO, les caches de miniatures de Windows et les fichiers temporaires.

### Propriétés du torrent

| Option | Signification |
|---|---|
| **Trackers :** | une adresse de tracker par ligne. Une ligne vide commence un nouveau niveau : les niveaux suivants sont utilisés quand les trackers qui les précèdent échouent. Les adresses répétées sont ignorées. |
| **Sources web :** | adresses de serveurs web qui détiennent les mêmes fichiers, une par ligne. Les pairs peuvent y télécharger comme auprès d'une source. |
| **Commentaire :** | un texte libre stocké dans le torrent ; d'autres programmes l'affichent, par exemple dans la fenêtre Ajouter un nouveau torrent. |
| **Taille des pièces :** | la taille de chaque pièce, de 16 ko à 16 Mo. **(détection automatique)** en choisit une d'après la taille totale et convient à la plupart des torrents. |
| **Version :** | la version de BitTorrent, voir ci-dessous. |

### Version : v1, v2 ou hybride

| Choix | Ce que vous obtenez |
|---|---|
| **Hybride (v1 + v2)** | le choix par défaut. Se charge dans les anciens programmes qui ne gèrent que la v1 comme dans les plus récents qui comprennent la v2. |
| **v1** | le format classique, tel que le créait µTorrent 2.2.1. Tous les programmes BitTorrent le lisent. |
| **v2 seulement** | uniquement le format plus récent. Les programmes qui ne comprennent que la v1 ne peuvent pas le charger. |

Dans le doute, gardez **Hybride (v1 + v2)**.

### Autres

| Option | Signification |
|---|---|
| **Commencer le partage** | ajoute le nouveau torrent à votre liste dès qu'il est créé, avec le dossier qui contient la source comme dossier de téléchargement. Rookparcel vérifie les fichiers puis partage. Les trackers de **Trackers pour les nouveaux téléchargements** dans les Préférences ne lui sont pas ajoutés ; il garde ceux que vous avez saisis. |
| **Torrent privé** | marque le torrent comme privé : les pairs ne sont trouvés que par ses trackers, jamais par le DHT, l'échange de pairs ni la découverte de pairs locaux. Les trackers privés l'exigent. |
| **Conserver l'ordre des fichiers** | conserve les fichiers d'un dossier dans l'ordre où Windows les liste au lieu de les trier par nom. Disponible seulement pour la **v1** : les torrents v2 et hybrides trient toujours leurs fichiers. |

## Progression et annulation

Pendant que Rookparcel lit les fichiers et calcule leurs empreintes, la barre de progression et le pourcentage avancent et les autres commandes sont grisées. Le bouton **Fermer** devient **Annuler** : cliquez dessus, appuyez sur <kbd>Esc</kbd> ou fermez la fenêtre pour arrêter. Rien n'est enregistré si vous annulez.

Si quelque chose se passe mal, un message l'indique. Les plus courants :

| Message | Signification |
|---|---|
| « Veuillez choisir un fichier ou un dossier à partir duquel créer le torrent. » | **Choisir la source** est vide. |
| « ... : aucun fichier à mettre dans le torrent (vide, ou tous ignorés). » | le dossier est vide, ou **Ignorer des fichiers** n'a rien laissé. |

## Ce que Rookparcel mémorise

La prochaine fois que vous ouvrez la fenêtre, elle contient les trackers, la taille des pièces, la version, **Commencer le partage**, **Torrent privé** et **Conserver l'ordre des fichiers** que vous aviez utilisés en dernier. La source, les motifs à ignorer, les sources web et le commentaire démarrent vides.

## Partager un torrent créé plus tôt

Si vous avez créé un torrent sans **Commencer le partage**, ajoutez le fichier `.torrent` comme n'importe quel autre ([Ajouter des torrents](Ajouter-des-torrents)) et réglez **Enregistrer dans** sur le dossier qui contient le fichier ou le dossier source. Rookparcel vérifie les fichiers et les partage.

Pour aider un tout nouveau torrent à se répandre, vous pouvez activer **Partage initial** pour lui ; voir [Gérer les torrents](Gérer-les-torrents#partage-initial).

## Dans la version GTK (Linux, BSD)

La même fenêtre avec les mêmes options, sous **Fichier > Créer un nouveau torrent...** (<kbd>Ctrl</kbd>+<kbd>N</kbd>).

## Dans l'application macOS

La même fenêtre avec les mêmes options, sous **Fichier > Créer un nouveau torrent...** (<kbd>⌘</kbd>+<kbd>N</kbd>).

## Dans l'interface web et rookparcel-nox

L'interface web ne peut pas créer de torrents : les fichiers devraient se trouver sur le serveur. Sur le serveur lui-même, `rookparcel-nox create` crée un torrent en ligne de commande :

```
rookparcel-nox create "D:\Share\My folder" -o my-folder.torrent -t udp://tracker.example.org:6969/announce
```

`--v1`, `--v2` et `--hybrid` choisissent la version (hybride par défaut), `--private` le marque comme privé, `--piece-size` règle la taille des pièces en Kio, `-w` ajoute une source web, et `--magnet` affiche aussi le lien magnet. `rookparcel-nox --help` liste toutes les options. Voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

Dans une fenêtre connectée avec **Fichier > Se connecter à distance...**, **Commencer le partage** est décoché et grisé : le torrent est créé à partir de fichiers de cet ordinateur, que le serveur n'a pas. Voir [Contrôle à distance](Contrôle-à-distance).

## Pages liées

- [Ajouter des torrents](Ajouter-des-torrents)
- [Gérer les torrents](Gérer-les-torrents)
- [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu)
