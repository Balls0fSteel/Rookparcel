*Ceci est la traduction française de la page [Where Rookparcel keeps its files](Where-Rookparcel-keeps-its-files) du wiki anglais. La page anglaise fait foi.*

# Emplacement des fichiers de Rookparcel

Cette page indique où Rookparcel garde ses réglages et sa liste de torrents sur chaque plateforme, comment le mode portable les garde à côté du programme, et comment les sauvegarder, les déplacer vers un autre ordinateur ou repartir de zéro. Vos téléchargements n'en font pas partie : ils sont là où vous les avez enregistrés.

## Trouver le dossier

**Aide > À propos de Rookparcel** (sous macOS **Rookparcel > À propos de Rookparcel**) affiche en bas le chemin du fichier de réglages. **Fichier > Profils > Ouvrir le dossier du profil** ouvre le dossier (*nouveau dans la 1.1.48*).

## Windows

La fenêtre Windows garde tout dans un seul dossier : `%APPDATA%\Rookparcel`, qui est en général `C:\Users\<vous>\AppData\Roaming\Rookparcel`. Les copies installées et celles du zip utilisent le même dossier, sauf si la copie du zip est portable.

| Fichier ou dossier | Ce qu'il contient |
|---|---|
| `rookparcel.ini` | les réglages |
| `state\` | la liste des torrents et la progression de chacun |
| `rss.dat` | les flux RSS et leurs filtres |
| `remote.ini`, `remote\` | les connexions à distance, et les réglages de chaque fenêtre distante |
| `ipfilter.dat` | le fichier du filtre IP par défaut, si vous en utilisez un |
| `ipfilter-url.dat`, `ipfilter-url.txt` | la liste de blocage du filtre IP telle qu'elle a été téléchargée la dernière fois depuis son adresse web, et une note disant d'où et quand, si vous en utilisez une (*nouveau dans la 1.2.63*) |
| `nox\` | les fichiers de `rookparcel-nox` quand il a tourné depuis une console |
| `search-helper\` | l'assistant de recherche, si vous l'avez installé ([Rechercher des torrents sous Windows](Rechercher-des-torrents-sous-Windows)) |
| `profiles\` | les autres [profils](Profils), chacun dans un dossier organisé comme celui-ci |
| `rookparcel.lock` | marque le dossier comme utilisé pendant que Rookparcel tourne |

Le service Windows de `rookparcel-nox` garde ses fichiers dans `%ProgramData%\Rookparcel\nox`. Voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

## Linux et les BSD

La version GTK suit les dossiers XDG :

| Quoi | Dossier |
|---|---|
| réglages : `rookparcel.ini`, `rss.dat`, `remote.ini`, `remote/`, `ipfilter.dat` et la liste téléchargée du filtre IP (`ipfilter-url.dat`, `ipfilter-url.txt`) si vous les utilisez | `$XDG_CONFIG_HOME/rookparcel/gtk`, sinon `~/.config/rookparcel/gtk` |
| liste des torrents : `resume.db`, `resume.log`, `metadata.pack`, `dht_state.dat`, `stats.dat` (les totaux de tous les temps d'Aide > Statistiques) | `$XDG_DATA_HOME/rookparcel/gtk`, sinon `~/.local/share/rookparcel/gtk` |

`rookparcel.lock`, dans le dossier de la liste des torrents, le marque comme utilisé. `rookparcel-nox` utilise `nox` à la place de `gtk` dans les deux chemins.

Le Flatpak garde ses dossiers sous `~/.var/app/io.github.balls0fsteel.Rookparcel/`. Sous systemd, les dossiers sont ceux du service ; voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

## macOS

Les réglages et la liste des torrents sont tous deux dans `~/Library/Application Support/Rookparcel/mac`, organisés comme le dossier Windows ci-dessus ; `rookparcel-nox` utilise `~/Library/Application Support/Rookparcel/nox`. Un [profil](Profils) `work` se trouve dans `~/Library/Application Support/Rookparcel/profiles/work/mac`.

La version GTK de `Rookparcel.app` des versions précédentes utilisait un dossier `gtk` aux mêmes endroits. Le premier démarrage de l'application native renomme chaque dossier `gtk` en `mac` (celui du profil par défaut et celui de chaque profil) et corrige les chemins qui y mènent dans ses réglages, si bien que tout est repris. Elle ne le fait qu'une fois : seulement là où il n'y a pas encore de dossier `mac`, et pas tant que l'ancienne application utilise le dossier. Les dossiers portables sont laissés tels quels.

*L'application macOS native et son dossier `mac` existent depuis la version 1.2.19 ; les versions précédentes utilisent `gtk`.*

## Mode portable

En mode portable, Rookparcel garde ses réglages et sa liste de torrents dans son propre dossier, comme le faisait µTorrent avec `settings.dat` à côté de `uTorrent.exe`. Le dossier peut se trouver sur une clé USB ou dans un dossier synchronisé.

**Pour l'activer**, placez un fichier nommé `rookparcel.ini` au bon endroit avant le premier démarrage. Il peut être vide. Ce doit être un fichier : un dossier portant ce nom ne compte pas.

| Copie | Où va `rookparcel.ini` |
|---|---|
| Zip Windows | à côté de `rookparcel.exe` |
| tar.gz Linux ou BSD | à côté de `rookparcel-gtk` (ou de `rookparcel-nox`) |
| AppImage Linux | à côté du fichier `.AppImage` |
| macOS | à côté de `Rookparcel.app`, jamais à l'intérieur ; à côté de `rookparcel-nox` pour le démon |
| Flatpak, installeur Windows | pas de mode portable |

*Le mode portable sous Windows est présent dans toutes les versions. Sous Linux et les BSD, il existe depuis la 1.1.49, sous macOS depuis la 1.1.50.*

L'organisation à l'intérieur d'un dossier portable est la même que celle de `%APPDATA%\Rookparcel` ci-dessus : les réglages et `state` à côté du programme, `nox` pour `rookparcel-nox`, `profiles` pour les autres profils.

### Ce qui change en mode portable

- **Le premier démarrage télécharge dans le dossier.** Les nouveaux téléchargements vont dans le dossier `Downloads` qu'il contient, et Rookparcel choisit un port d'écoute au hasard, pour ne pas entrer en conflit avec un autre Rookparcel sur 6881. *Nouveau dans la 1.1.45 et la 1.1.49.*
- **Les chemins situés dans le dossier sont enregistrés relativement à lui.** Les dossiers d'enregistrement, les dossiers des Préférences, le fichier du filtre IP et les dossiers d'enregistrement RSS qui se trouvent dans le dossier portable sont écrits sous la forme `./Downloads/...`. Le dossier peut alors passer sur une autre lettre de lecteur, ou sur un autre ordinateur, et les torrents continuent sans nouvelle vérification. Les chemins extérieurs au dossier restent tels quels. *Nouveau dans la 1.1.49.* Les chemins écrits par une version plus ancienne sont convertis à leur prochain enregistrement.
- **Rien n'est écrit dans le système sauf si vous le demandez.** Les associations, le démarrage avec le système et les services ne sont réglés que si vous les cochez dans les Préférences. Ils désignent alors le programme de cette copie, et cessent de fonctionner si le dossier est déplacé.
- **Il ne se met pas à jour lui-même.** L'avis de mise à jour d'une copie portable ouvre la page de téléchargement. Voir [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel).

Un dossier portable écrit par la 1.1.49 ou plus récente demande la 1.1.49 ou plus récente : une version plus ancienne lirait mal les chemins relatifs.

Si le dossier ne peut pas être écrit (une clé en lecture seule, `C:\Program Files`), Rookparcel s'arrête avec « Ce Rookparcel portable ne peut pas écrire dans son dossier » au lieu de mettre vos réglages ailleurs. Déplacez le dossier, ou supprimez `rookparcel.ini` pour utiliser les dossiers habituels.

Sous macOS, un `Rookparcel.app` lancé directement depuis le dossier Téléchargements après son téléchargement tourne depuis une copie temporaire, où le `rookparcel.ini` placé à côté n'est pas vu. Rookparcel le dit. Déplacez `Rookparcel.app` avec le Finder dans le dossier voulu et relancez-le.

## Un dossier, un Rookparcel

Deux Rookparcel n'utilisent jamais un même dossier en même temps. Lancer Rookparcel une seconde fois amène la fenêtre déjà ouverte au premier plan et lui transmet ce que vous avez ouvert. Une copie qui trouve le dossier utilisé par une autre session, ou par un autre ordinateur via un partage réseau, affiche « Un autre Rookparcel utilise déjà ce dossier » et se ferme. Fermez d'abord l'autre. (*La vérification entre sessions et entre ordinateurs existe depuis la 1.1.43.*)

Pour faire tourner volontairement deux Rookparcel côte à côte, utilisez les [profils](Profils).

## Sauvegarder

1. Quittez Rookparcel (**Fichier > Quitter** ; sous macOS **Rookparcel > Quitter Rookparcel**), pour que tout soit enregistré.
2. Copiez le dossier entier : `%APPDATA%\Rookparcel` sous Windows, à la fois `~/.config/rookparcel` et `~/.local/share/rookparcel` sous Linux, `~/Library/Application Support/Rookparcel` sous macOS, ou le dossier portable.

La sauvegarde contient les réglages, la liste des torrents avec la progression de chacun, les flux RSS et tous les profils. Elle ne contient pas les fichiers téléchargés.

## Passer à un autre ordinateur

1. Quittez Rookparcel sur l'ancien ordinateur et copiez le dossier, comme pour une sauvegarde.
2. Installez Rookparcel sur le nouvel ordinateur, mais ne le lancez pas encore.
3. Placez le dossier au même endroit là-bas.
4. Copiez ou déplacez vos téléchargements.
5. Lancez Rookparcel.

Hors du mode portable, la liste des torrents garde des chemins complets. Placez les téléchargements aux mêmes chemins sur le nouvel ordinateur (la même lettre de lecteur et les mêmes dossiers sous Windows), sinon les torrents ne retrouveront pas leurs fichiers.

Un dossier portable est plus simple : copiez le dossier entier, téléchargements compris, et lancez le programme qu'il contient. Ses chemins sont relatifs, donc une autre lettre de lecteur ou un autre nom d'utilisateur n'a aucune importance.

## Repartir de zéro

Pour réinitialiser complètement Rookparcel :

1. Quittez Rookparcel.
2. Renommez le dossier (par exemple en `Rookparcel.old`), pour pouvoir revenir en arrière.
3. Lancez Rookparcel. Il démarre comme une nouvelle installation, avec l'[assistant de configuration](Premier-démarrage) et une liste de torrents vide.

Vos fichiers téléchargés restent où ils sont ; ajoutez de nouveau leurs torrents pour les partager. Le désinstalleur Windows peut aussi supprimer le dossier : son option **Paramètres et liste des torrents** ([Installer Rookparcel](Installer-Rookparcel#désinstaller)).
