*Cette page est la traduction de la page anglaise [RSS Downloader](RSS-Downloader). En cas de divergence, la page anglaise fait foi.*

# Téléchargeur RSS

Cette page explique comment suivre des flux RSS dans Rookparcel et y télécharger : ajouter des flux, télécharger des éléments à la main, les filtres qui téléchargent automatiquement les éléments correspondants, la correspondance des épisodes et de la qualité, et l'historique. Elle couvre d'abord la fenêtre Windows, puis la fenêtre GTK (Linux, BSD) et l'interface web.

Le Téléchargeur RSS fonctionne comme celui de µTorrent 2.2.1. Beaucoup de sites de torrents et de trackers publient un flux de leurs nouvelles publications ; Rookparcel vérifie vos flux toutes les 15 minutes et ajoute ce que vos filtres demandent.

## Ouvrir le Téléchargeur RSS

- **Options > Téléchargeur RSS...**
- Le bouton **Téléchargeur RSS** de la barre d'outils.
- Faites un clic droit sur **Flux** (ou sur un flux) dans la liste des catégories à gauche, puis **Téléchargeur RSS...**.

![La fenêtre du Téléchargeur RSS](images/rss-downloader.png)

La fenêtre a la liste **Flux** à gauche, avec **Ajouter...**, **Modifier...**, **Supprimer** et **Mettre à jour** en dessous, et deux onglets à droite : **Filtres** et **Historique**. Les modifications prennent effet immédiatement ; il n'y a ni OK ni Annuler, seulement **Fermer**. Quand vous fermez la fenêtre, les filtres sont exécutés sur les éléments actuels des flux, de sorte qu'un nouveau filtre peut télécharger tout de suite.

## Ajouter un flux

1. Dans le Téléchargeur RSS, cliquez sur **Ajouter...** sous **Flux**. (Ou faites un clic droit sur **Flux** dans la liste des catégories de la fenêtre principale et choisissez **Ajouter un flux RSS...**.)
2. **URL du flux :** l'adresse du flux. Si une adresse de flux se trouve dans le presse-papiers, elle est saisie pour vous.
3. **Alias personnalisé :** un nom pour le flux. Laissé vide, le titre propre du flux est affiché.
4. Sous **Abonnement**, choisissez une option :
   - **Ne pas télécharger automatiquement tous les éléments** (le choix habituel) : seuls vos filtres, ou vous, téléchargent depuis ce flux.
   - **Télécharger automatiquement tous les éléments publiés dans le flux** : chaque nouvel élément est téléchargé. **Utiliser le filtre d'épisodes intelligent** en dessous ne télécharge chaque épisode qu'une seule fois (voir [Filtre d'épisodes intelligent](#filtre-dépisodes-intelligent)).
5. Cliquez sur **OK**. Le flux est récupéré tout de suite.

Quand **Télécharger automatiquement tous les éléments publiés dans le flux** est choisi, vos filtres ne sont pas utilisés pour ce flux : tout ce qui est nouveau est pris.

**Flux qui demandent un identifiant.** Les flux de certains trackers privés demandent les cookies de votre connexion. Ajoutez-les à l'URL comme le faisait µTorrent, après `:COOKIE:` :

```
https://tracker.example/rss.php:COOKIE:uid=12345;pass=abcdef
```

Les cookies ne sont envoyés qu'au site propre du flux (le même hôte ou l'un de ses sous-domaines), et jamais d'un flux `https` vers un lien `http` simple.

## Gérer les flux

La liste **Flux** commence par **Tous les flux**, puis chaque flux. Sous la liste, une ligne montre l'état du flux sélectionné : « Mis à jour » avec l'heure et le nombre d'éléments, « Pas encore mis à jour », « Désactivé », ou « Erreur : » et la raison. Avec **Tous les flux** sélectionné, elle indique combien de flux vous avez.

| Bouton ou entrée de menu | Ce qu'il fait |
|---|---|
| **Modifier...** / **Modifier le flux...** | Change l'URL, l'alias ou l'abonnement. Une nouvelle URL est un nouveau flux : ses anciens éléments sont abandonnés et il est récupéré de nouveau. |
| **Supprimer** / **Supprimer le flux** | Supprime le flux, après confirmation. Les filtres réglés sur ce flux restent, et ne correspondent à rien tant que vous ne les dirigez pas vers un autre flux. |
| **Mettre à jour** / **Mettre à jour le flux** | Récupère le flux sélectionné maintenant (avec **Tous les flux** sélectionné, chaque flux). |
| **Désactiver le flux** / **Activer le flux** | Arrête ou reprend la vérification du flux, sans le supprimer. Dans le menu contextuel uniquement. |

Faites un clic droit sur un flux dans la liste Flux du Téléchargeur RSS, ou dans la liste des catégories de la fenêtre principale, pour obtenir les mêmes entrées.

Les flux sont vérifiés toutes les 15 minutes. Les téléchargements de flux et des fichiers `.torrent` vers lesquels ils pointent passent par le proxy réglé dans les Préférences, si vous en avez réglé un. Les liens des éléments d'un flux ne peuvent mener qu'à des adresses publiques, sauf si le flux lui-même provient d'un serveur de cet ordinateur ou de votre réseau local.

## Télécharger des éléments à la main

La liste des catégories de la fenêtre principale a une branche **Flux** avec chaque flux en dessous. Cliquez sur un flux (ou sur **Flux** pour tous) et la liste des torrents affiche les éléments du flux à la place de vos torrents, avec les colonnes **Nom**, **Épisode**, **Format**, **Date**, **Flux** et **URL**.

- **Épisode** montre ce que Rookparcel a lu dans le titre : `1x02`, une plage comme `1x02-03`, un numéro absolu comme `12` (anime), ou une date de diffusion comme `2026-10-19`.
- **Format** montre les qualités nommées dans le titre, comme `720p` ou `HDTV`.

Pour télécharger un élément, double-cliquez dessus, ou faites un clic droit dessus et choisissez **Télécharger**. Vous pouvez d'abord en sélectionner plusieurs. Un élément que vous téléchargez à la main reçoit le nom du flux comme étiquette et entre dans l'Historique, comme le ferait un téléchargement par un filtre.

## Filtres

Un filtre télécharge les éléments dont les titres lui correspondent, depuis un flux ou depuis tous. Ouvrez l'onglet **Filtres** et cliquez sur **Ajouter**. Un filtre appelé « Nouveau filtre » apparaît dans la liste, coché. Remplissez ensuite ses champs :

| Champ | Ce qu'il fait |
|---|---|
| **Nom :** | Le nom du filtre dans la liste. La case à cocher de la liste active ou désactive le filtre. |
| **Filtre :** | Ce à quoi le titre doit correspondre. Voir [Écrire un filtre](#écrire-un-filtre). Vide, il ne correspond à rien ; `*` correspond à tout. |
| **Sauf :** | Les titres qui y correspondent sont ignorés, même quand **Filtre** correspond. Même syntaxe. Vide, il n'ignore rien. |
| **Enregistrer dans :** | Le dossier des téléchargements. Vide, le dossier de téléchargement par défaut est utilisé. Le bouton **...** choisit un dossier. |
| **Flux :** | **Tous les flux**, ou un seul flux. |
| **Qualité :** | Cliquez sur le bouton et cochez les qualités que vous acceptez. **Toutes** accepte n'importe quel titre, y compris les titres qui ne nomment aucune qualité. |
| **Numéro d'épisode :** | Quels épisodes prendre, par exemple `1x12-14`. Vide, tous sont pris. Voir [Numéros d'épisode](#numéros-dépisode). |
| **Intervalle minimum :** | Après que le filtre a téléchargé quelque chose, il ne télécharge plus rien jusqu'à ce que ce délai soit écoulé. **0 (ignorer)** signifie aucune attente. Choix de 2 heures à 1 mois. |
| **Étiquette :** | L'étiquette que reçoivent les téléchargements. Vide, le nom du flux est utilisé. Voir [Gérer les torrents](Gérer-les-torrents) pour les étiquettes. |
| **Filtre d'épisodes intelligent** | Ne télécharge chaque épisode qu'une seule fois, quels que soient sa qualité ou son groupe de publication. Voir ci-dessous. |
| **Donner la priorité la plus haute** | Place le téléchargement en tête de la file d'attente. |
| **Ajouter le torrent en pause** | Ajoute le téléchargement sans le démarrer. |

Sous les champs, **Éléments correspondants dans les flux :** liste les éléments de la dernière vérification des flux auxquels le filtre correspond d'après Filtre, Sauf, Flux, Qualité et Numéro d'épisode. Utilisez-la pour tester un filtre avant de fermer la fenêtre. (L'intervalle minimum et le filtre d'épisodes intelligent dépendent de ce qui a été téléchargé auparavant ; la liste ne les applique donc pas.)

**Supprimer** efface le filtre sélectionné. **Monter** et **Descendre** le déplacent dans la liste. L'ordre compte : un élément va au premier filtre qui le prend ; placez donc les filtres plus précis au-dessus des filtres généraux.

Les filtres s'exécutent quand un flux est vérifié, et quand vous fermez le Téléchargeur RSS. Dans chaque flux, les éléments les plus anciens sont examinés en premier ; avec le filtre d'épisodes intelligent ou un intervalle minimum, la première publication d'un épisode l'emporte donc. Un élément n'est jamais téléchargé deux fois depuis le même flux.

### Écrire un filtre

Les champs **Filtre** et **Sauf** utilisent de simples jokers :

| Vous saisissez | Signification |
|---|---|
| `*` | Une suite quelconque de caractères, éventuellement vide. |
| `?` | Exactement un caractère. |
| `\|` | Sépare des alternatives : `show one\|show two` correspond à l'une ou l'autre. |

- La casse n'importe pas.
- Un `.` ou un `_` dans un titre compte comme une espace, de sorte que `the show` correspond à `The.Show.S01E02.720p`.
- Une alternative **sans** joker correspond n'importe où dans le titre : `show` correspond à `The.Show.S01E02`.
- Une alternative **avec** jokers doit correspondre au titre entier : `show*` ne correspond pas à `The.Show.S01E02`, mais `*show*` y correspond.

Exemples :

| Filtre | Sauf | Prend |
|---|---|---|
| `the show` | | tous les titres contenant « the show » |
| `*the show*1080p*` | | cette série en 1080p seulement |
| `the show` | `*german*\|*sample*` | cette série, mais sans les publications allemandes ni les échantillons |

### Numéros d'épisode

Le champ **Numéro d'épisode** accepte une ou plusieurs plages, séparées par `,` ou `;` :

| Vous saisissez | Prend |
|---|---|
| `1x12` ou `S01E12` | Cet épisode seulement. |
| `1x12-14` ou `S01E12-S01E14` | Les épisodes 12 à 14 de la saison 1. |
| `S01E01-10` | Les épisodes 1 à 10 de la saison 1 (la saison peut être omise après le tiret). |
| `S02` | Toute la saison 2. |
| `S01E05-` | L'épisode 5 de la saison 1 et tout ce qui suit. |
| `12` | L'épisode 12 de n'importe quelle saison, pour les séries numérotées d'un bout à l'autre, comme les anime. |
| `1-12`, `100-` | Les épisodes 1 à 12 ; l'épisode 100 et les suivants. |
| `2026.10.19` | L'épisode diffusé à cette date (pour les séries quotidiennes). |

Rookparcel lit les épisodes dans les titres écrits sous la forme `S01E02` (aussi `S01E02E03`, `S01E02-E03`), `1x02`, une date de diffusion (`2026.10.19`, `2026-10-19`), ou un numéro absolu (`Show - 12`, `Episode 12`). Un numéro de version comme `Ubuntu - 22.04` ou un récapitulatif comme `Show - 12.5` n'est pas lu comme un épisode. Un pack de saison comme `Show.S01.Complete` ne nomme aucun épisode ; un filtre avec un numéro d'épisode ne le prend donc pas. Un champ que Rookparcel ne peut pas lire ne correspond à rien.

### Qualités

La liste **Qualité** : HDTV, TVRip, DVDRip, SVCD, DSRip, DVBRip, PDTV, HR.HDTV, HR.PDTV, DVDR, DVDScr, 720p, 1080i, 1080p, WebRip, SatRip, WEB-DL, BluRay, 480p, 2160p. Un titre correspond quand il nomme l'une des qualités que vous avez cochées.

### Filtre d'épisodes intelligent

Avec **Filtre d'épisodes intelligent** coché, le filtre se souvient des épisodes qu'il a téléchargés et ignore les publications ultérieures du même épisode, quels que soient leur qualité ou leur groupe de publication. C'est utile quand un flux contient plusieurs fois le même épisode.

- Un titre qui ne contient aucun épisode est toujours laissé passer.
- Une publication marquée REPACK, PROPER, RERIP ou REAL d'un épisode déjà téléchargé est laissée passer une fois, puisqu'elle corrige la publication précédente.

L'option **Utiliser le filtre d'épisodes intelligent** de l'abonnement du flux fonctionne de la même façon pour un flux réglé pour tout télécharger.

## Historique

L'onglet **Historique** liste ce que le Téléchargeur RSS a ajouté, le plus récent en premier, avec **Nom**, **Date** et **Flux**. Il garde les 1 000 dernières entrées. **Effacer l'historique** vide la liste ; cela ne fait pas télécharger de nouveau ces éléments.

L'onglet Journal de la fenêtre principale montre aussi une ligne pour chaque téléchargement (« RSS : » avec le nom du flux et le titre) et pour chaque erreur de flux. Un téléchargement qui n'a pas pu être récupéré est retenté à la prochaine vérification de son flux.

## GTK

La fenêtre GTK a le même Téléchargeur RSS, ouvert depuis **Options > Téléchargeur RSS...**, le bouton **Téléchargeur RSS** de la barre d'outils, ou le menu contextuel de **Flux** dans le panneau latéral. Les champs, les boutons et les onglets sont les mêmes que ci-dessus. La branche **Flux** du panneau latéral montre les éléments d'un flux dans la liste, et **Télécharger** dans le menu contextuel d'un élément (ou un double-clic) le télécharge. La fenêtre Ajouter un flux RSS propose aussi une adresse de flux prise dans le presse-papiers.

## macOS

L'application macOS a le même Téléchargeur RSS, ouvert depuis **Options > Téléchargeur RSS...**, le bouton **Téléchargeur RSS** de la barre d'outils, ou le menu contextuel de **Flux** dans le panneau latéral. Les champs, les boutons et les onglets sont les mêmes que ci-dessus. Les éléments d'un flux s'affichent dans la liste quand vous choisissez le flux dans le panneau latéral ; double-cliquez sur un élément, appuyez sur <kbd>Return</kbd> ou choisissez **Télécharger** dans son menu contextuel pour le télécharger.

## Interface web

L'interface web a aussi le Téléchargeur RSS. Ouvrez-le avec le bouton **RSS** de la barre d'outils (sur un téléphone, le menu **Plus** de la barre d'outils, **Téléchargeur RSS...**), ou cliquez sur un flux sous **Flux** dans la liste des catégories, ce qui l'ouvre sur les éléments de ce flux.

Le Téléchargeur RSS de la page a trois onglets au lieu de deux :

- **Éléments** : les éléments du flux sélectionné (ou de tous les flux), avec **Télécharger** pour ceux qui sont sélectionnés.
- **Filtres** et **Historique** : comme ci-dessus.

Différences :

- **Enregistrer dans** est un dossier sur le serveur, pas sur l'ordinateur où se trouve le navigateur. Le bouton **...** demande un chemin ; vide, il désigne le dossier de téléchargement par défaut du serveur.
- L'interface web que vous utilisez décide de quels flux vous modifiez :
  - **`rookparcel-nox`** conserve ses propres flux et filtres (dans `rss.dat` à côté de `webui.json`) et les exécute sur le serveur, sans qu'aucune fenêtre soit ouverte. Il n'exécute le Téléchargeur RSS que tant que son interface web est activée. Voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).
  - **L'interface web qu'héberge une fenêtre Windows** modifie les propres flux et filtres de cette fenêtre : il n'y a qu'un seul Téléchargeur RSS, affiché aux deux endroits. Voir [Interface Web](Interface-Web).
- L'interface web répond aussi à la partie RSS de l'API web de qBittorrent, de sorte qu'une application distante qBittorrent peut lister les flux et définir des règles. Ses règles utilisent la syntaxe à jokers ci-dessus ; l'option d'expression régulière est ignorée, et les dossiers de flux ne sont pas pris en charge.

## Fenêtres distantes

Une fenêtre connectée à un autre Rookparcel (Fichier > Se connecter à distance, voir [Contrôle à distance](Contrôle-à-distance)) a ses propres flux et filtres, conservés sur cet ordinateur. Ils sont vérifiés depuis cet ordinateur, et ce qu'ils téléchargent est ajouté au serveur distant.

## Où les flux sont stockés

Les flux, les filtres et l'historique se trouvent dans `rss.dat` dans votre dossier de paramètres, à côté de `rookparcel.ini` (pour une fenêtre distante, dans son propre dossier `remote\<host>_<port>` à cet endroit). Voir [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel).

Pour des recherches ponctuelles plutôt que des flux, voir [Rechercher des torrents](Rechercher-des-torrents). Pour Sonarr et Radarr, qui vérifient eux-mêmes les flux, voir [Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr).
