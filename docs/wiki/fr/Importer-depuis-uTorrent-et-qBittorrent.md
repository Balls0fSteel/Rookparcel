*Ceci est la traduction française de la page [Importing from µTorrent and qBittorrent](Importing-from-uTorrent-and-qBittorrent) du wiki anglais. La page anglaise fait foi.*

# Importer depuis µTorrent et qBittorrent

Cette page explique comment reprendre vos torrents de µTorrent ou de qBittorrent installé sur le même ordinateur, pour qu'ils continuent à télécharger et à partager dans Rookparcel là où ils en sont, sans rien retélécharger.

## Ce qui est repris

Pour chaque torrent, Rookparcel reprend :

- les données, qui restent où elles sont : rien n'est copié ni déplacé ;
- les statistiques, les priorités des fichiers (les fichiers ignorés restent ignorés) et les trackers ;
- l'étiquette : la catégorie de qBittorrent, ou l'étiquette de µTorrent ;
- les tags de qBittorrent, sous forme de tags (*nouveau dans la 1.2.63* ; auparavant, le premier tag devenait l'étiquette d'un torrent sans catégorie). Un nom de tag contenant une virgule ou un caractère de contrôle est écarté ;
- un état arrêté ou en pause ;
- depuis qBittorrent, les limites de partage propres au torrent (ratio et durée de partage), qui deviennent son propre [objectif de partage](Détails-et-fichiers-des-torrents#propriétés-du-torrent).

Rookparcel ne se fie pas à l'enregistrement de l'autre programme indiquant quelles pièces sont complètes : il vérifie les fichiers avant de s'y fier. Les gros torrents affichent donc **Vérification** pendant un moment après l'importation.

Les fichiers de l'autre programme sont seulement lus, jamais modifiés. Les fichiers de données partiels que µTorrent a laissés en `.!ut` ou qBittorrent en `.!qB` sont repris, et renommés quand le torrent tourne : voir [Disque et stockage](Disque-et-stockage#fichiers-incomplets-et-ut) (*nouveau dans la 1.2.63*). Les torrents déjà présents dans la liste de Rookparcel sont écartés, donc vous pouvez relancer l'importation sans risque.

## Avant de commencer

1. **Fermez l'autre programme**, icône de la zone de notification comprise. Les deux programmes réécrivent leurs fichiers en se fermant, donc Rookparcel refuse d'importer tant qu'il voit µTorrent ou qBittorrent en cours d'exécution : « ... est en cours d'exécution. Fermez-le, puis relancez l'importation : il réécrit ses fichiers en se fermant. » Sous macOS et BSD, Rookparcel ne peut pas savoir s'il tourne, donc assurez-vous-en vous-même.
2. Assurez-vous que les disques contenant les données téléchargées sont connectés, sous les mêmes lettres ou les mêmes chemins qu'avant.

Après l'importation, ne faites pas tourner les deux programmes sur les mêmes torrents : ils écriraient tous deux dans les mêmes fichiers.

## Importer

1. Choisissez **Fichier > Importer depuis qBittorrent...** ou **Fichier > Importer depuis µTorrent...**.
2. Rookparcel cherche dans le dossier habituel du programme (voir plus bas). S'il n'y est pas, un sélecteur de dossier s'ouvre : « Sélectionner le dossier de qBittorrent » ou « Sélectionner le dossier de µTorrent ». Choisissez le dossier décrit plus bas.
3. Rookparcel demande : « Importer 47 torrents depuis qBittorrent ? Leurs données restent où elles sont et les fichiers de qBittorrent ne sont pas modifiés. Les torrents déjà dans la liste sont ignorés. » La question liste les dossiers où se trouvent les données. Vérifiez-les, puis cliquez sur **Oui**.
4. Un message indique combien ont été ajoutés, par exemple « 45 torrents ajoutés depuis qBittorrent. 2 ont été ignorés ; l'onglet Journal en donne la raison. »

Les torrents apparaissent dans la liste et commencent leur vérification. Leurs étiquettes s'affichent sous **Étiquettes** dans la liste des catégories, et les tags de qBittorrent sous **Tags**.

## Où Rookparcel cherche

| Programme | Dossier habituel | Ce qu'il faut choisir si on vous le demande |
|---|---|---|
| qBittorrent sous Windows | `%LOCALAPPDATA%\qBittorrent\BT_backup` | le dossier `BT_backup`, ou le dossier `qBittorrent` au-dessus |
| qBittorrent sous Linux et BSD | `~/.local/share/qBittorrent/BT_backup` (ou sous `$XDG_DATA_HOME`) | le même |
| qBittorrent sous macOS | `~/Library/Application Support/qBittorrent/BT_backup` | le même |
| µTorrent sous Windows | `%APPDATA%\uTorrent` | le dossier qui contient `resume.dat` |

qBittorrent garde un fichier `.fastresume` et un fichier `.torrent` par torrent dans `BT_backup` ; µTorrent garde `resume.dat` à côté des fichiers `.torrent`. Si votre copie les garde ailleurs, choisissez ce dossier. Les données restent dans les dossiers que nomment les enregistrements, donc ces dossiers doivent exister sur cet ordinateur.

## Torrents écartés

Chaque torrent importé ou écarté a une ligne dans l'[onglet Journal](Détails-et-fichiers-des-torrents#journal) : « Imported from qBittorrent: ... » ou « Not imported from qBittorrent: ...: » suivi de la raison.

| Raison | Que faire |
|---|---|
| déjà dans la liste | rien ; il est déjà dans Rookparcel |
| son fichier .torrent est introuvable | ajoutez le torrent vous-même, depuis son fichier `.torrent` ou son lien magnet, avec **Enregistrer dans** réglé sur son dossier de données |
| son enregistrement de reprise est illisible | la même chose |
| son fichier .torrent est illisible : ... | la même chose, avec une bonne copie du fichier `.torrent` |

Quand Rookparcel ne trouve aucun torrent, il dit « Aucun torrent de qBittorrent n'a été trouvé dans ... » avec le dossier qu'il a lu.

## Dans la version GTK (Linux, BSD)

Les deux mêmes éléments du menu **Fichier**, avec les mêmes étapes. µTorrent n'a pas de dossier habituel en dehors de Windows, donc Rookparcel demande le dossier contenant `resume.dat`, par exemple un dossier copié depuis un ordinateur Windows.

## Avec rookparcel-nox

Le démon sans fenêtre importe au démarrage :

```
rookparcel-nox run --import qbittorrent
rookparcel-nox run --import utorrent --import-dir "D:\Backup\uTorrent"
```

`--import` accepte `qbittorrent` ou `utorrent` ; `--import-dir` désigne le dossier quand ce n'est pas le dossier habituel (le `BT_backup` de qBittorrent ou son dossier de données, le dossier de µTorrent contenant `resume.dat`). L'autre programme ne doit pas tourner, et les torrents déjà dans la liste de nox sont laissés tels quels. Voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

L'interface web n'a pas d'importation, et dans une fenêtre connectée avec **Fichier > Se connecter à distance...** les éléments d'importation sont grisés : le serveur ne pourrait pas lire les dossiers de cet ordinateur. Voir [Contrôle à distance](Contrôle-à-distance).

## Pages connexes

- [Ajouter des torrents](Ajouter-des-torrents)
- [Gérer les torrents](Gérer-les-torrents)
- [Mise en file et partage](Mise-en-file-et-partage)
