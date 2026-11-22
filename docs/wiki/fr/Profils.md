*Ceci est la traduction française de la page [Profiles](Profiles) du wiki anglais. La page anglaise fait foi.*

# Profils

Un profil est un Rookparcel complet à part : ses propres réglages, sa liste de torrents, ses flux RSS, ses ports et sa fenêtre. Cette page explique comment créer et lancer des profils, ce que chacun garde séparé, et l'option `--profile` pour les raccourcis et la ligne de commande.

*Nouveau dans la 1.2.0.* Jusqu'à la 1.1.40, le seul moyen de faire tourner deux Rookparcel est d'avoir deux copies portables dans des dossiers séparés ([mode portable](Emplacement-des-fichiers-de-Rookparcel#mode-portable)).

## À quoi servent les profils

- Un Rookparcel qui n'utilise que votre VPN, à côté d'un Rookparcel normal : chaque profil a son propre réglage **Utiliser uniquement ce réseau**, son proxy et ses ports ([Connexion, ports et pare-feu](Connexion-ports-et-pare-feu)).
- Des trackers privés séparés des torrents publics, chacun avec ses objectifs de partage et ses sources de recherche.
- Essayer un réglage sans toucher à votre configuration réelle : `--profile test`.

Plusieurs profils peuvent tourner en même temps, chacun dans sa fenêtre. Ils ne partagent rien : un torrent d'un profil n'est pas visible dans un autre.

## Le profil par défaut

Sans option de profil, Rookparcel utilise le **profil par défaut** : les dossiers décrits dans [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel), exactement comme avant les profils. Rien ne bouge quand vous commencez à utiliser les profils.

Un profil nommé se trouve dans un dossier `profiles` à l'intérieur du dossier par défaut :

| | profil par défaut | profil `work` |
|---|---|---|
| Windows | `%APPDATA%\Rookparcel` | `%APPDATA%\Rookparcel\profiles\work` |
| Linux, BSD | `~/.config/rookparcel/gtk`, `~/.local/share/rookparcel/gtk` | `~/.config/rookparcel/profiles/work/gtk`, `~/.local/share/rookparcel/profiles/work/gtk` |
| macOS | `~/Library/Application Support/Rookparcel/mac` | `~/Library/Application Support/Rookparcel/profiles/work/mac` |
| dossier portable | le dossier | `profiles\work` à l'intérieur |

## Noms de profils

Un nom compte de 1 à 32 caractères : lettres a-z, chiffres et traits d'union (-), et commence par une lettre. Les majuscules sont prises pour des minuscules, donc `Work` et `work` sont le même profil. `default` désigne le profil par défaut. Tout autre nom est refusé avec un message qui le dit.

## Créer et lancer des profils depuis le menu

**Fichier > Profils** liste le profil par défaut (sous le nom **Par défaut**) et chaque profil nommé. Celui que cette fenêtre utilise est coché.

| Élément | Ce qu'il fait |
|---|---|
| le nom d'un profil | lance ce profil dans sa propre fenêtre. S'il tourne déjà, sa fenêtre passe au premier plan |
| **Nouveau profil...** | demande un nom (« Nom (lettres a-z, chiffres et traits d'union) : »), crée le profil et le lance |
| **Créer un raccourci...** | Windows : place sur votre bureau un raccourci qui lance ce profil, portant son nom, par exemple `Rookparcel (work).lnk`. Linux : place le profil dans le menu de vos applications |
| **Ouvrir le dossier du profil** | ouvre le dossier de ce profil |

Un nouveau profil démarre vide, comme une nouvelle installation, et s'ouvre avec l'[assistant de configuration](Premier-démarrage).

## Distinguer les fenêtres

La fenêtre d'un profil nommé affiche son nom dans la barre de titre et dans l'info-bulle de l'icône de la zone de notification : `Rookparcel 1.2.63 [work]`. Le profil par défaut n'affiche pas de nom. Sous Windows, chaque profil a son propre groupe de boutons dans la barre des tâches, ce qui permet de les épingler séparément.

## Ce que chaque profil garde à part

**Les ports.** Un nouveau profil choisit à son premier démarrage un port d'écoute au hasard entre 10000 et 60000, pour ne pas se disputer le 6881 avec le profil par défaut. Le profil par défaut garde son port. Si deux profils sont réglés malgré tout sur le même port, la barre d'état affiche « Port N déjà utilisé » ; cliquez dessus pour ouvrir **Options > Préférences > Connexion**. Voir [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu).

Chaque profil ouvre son propre port sur le routeur (UPnP, NAT-PMP). La redirection porte le nom du profil, par exemple `Rookparcel (work)`, ce qui permet de les distinguer dans la liste du routeur.

**L'interface web.** L'interface web de chaque profil a besoin de son propre port. Dans la fenêtre Windows, quand vous activez l'interface web dans un profil, **Options > Préférences > Interface web** propose le premier port libre à partir de 8080. Si le port est quand même pris, elle indique « le port N est déjà utilisé, peut-être par un autre profil Rookparcel ». Voir [Interface web](Interface-Web).

**Les fichiers `.torrent` et les liens `magnet:`.** Un seul programme peut les ouvrir, donc ils appartiennent à un profil à la fois. Cocher **Associer aux fichiers .torrent** ou **Associer aux liens magnet:** dans **Options > Préférences > Général** d'un profil les donne à ce profil. Sous Windows, quand un autre profil les possède, la page le dit : « Le profil Rookparcel « work » les ouvre actuellement ; cocher une case les fait ouvrir par ce profil. » Un double-clic sur un fichier `.torrent` l'ouvre ensuite dans le profil qui possède l'association, en lançant ce profil s'il ne tourne pas.

Les cases d'association de l'installeur Windows les donnent au profil par défaut.

**Le démarrage avec le système.** **Lancer Rookparcel au démarrage du système** se règle profil par profil, donc chaque profil peut démarrer seul. Sous Windows, ils sont listés sous les noms `Rookparcel` et `Rookparcel (work)`.

**Les fenêtres distantes.** Une fenêtre qui pilote un `rookparcel-nox` distant ([Contrôle à distance](Contrôle-à-distance)) garde ses réglages dans le profil depuis lequel elle a été ouverte.

## Lancer un profil depuis la ligne de commande

Ajoutez `--profile NOM` (ou `--profile=NOM`) à la commande :

```
rookparcel.exe --profile work
rookparcel-gtk --profile work
```

- Elle peut figurer n'importe où parmi les arguments, et le nom n'est jamais pris pour un torrent : `rookparcel.exe --profile work file.torrent` ajoute `file.torrent` au profil `work`.
- `--profile default`, ou aucune option, désigne le profil par défaut.
- Quand le profil n'existe pas encore, la fenêtre demande d'abord : « Le profil « work » n'existe pas encore. Le créer ? », et affiche le dossier qu'elle va créer. **Non** ferme sans rien créer, si bien qu'un nom mal tapé ne lance pas un Rookparcel vide.
- Elle se combine avec les autres options, par exemple `--minimized` pour démarrer dans la zone de notification, ou `--remote` pour se connecter à un `rookparcel-nox` distant.

Pour créer un raccourci Windows à la main, ajoutez `--profile work` après `rookparcel.exe` dans la case **Cible** du raccourci. **Fichier > Profils > Créer un raccourci...** le fait pour vous.

## Profils et mode portable

Un Rookparcel portable a aussi des profils, dans le dossier `profiles` à l'intérieur du dossier portable. Ils le suivent dans ses déplacements. Voir [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel#mode-portable).

## Différences avec GTK

- **Fichier > Profils** fonctionne de la même façon. Le profil actuel apparaît comme l'élément sélectionné.
- **Créer un raccourci...** place le profil dans le menu de vos applications (« Rookparcel (work) »). Le profil a sa propre entrée de bureau, si bien que le dock et la barre des tâches montrent sa fenêtre sous son propre nom et sa propre icône.
- Dans le **Flatpak**, les profils fonctionnent, mais seul le profil par défaut peut démarrer avec le système.

## Différences avec macOS

- **Fichier > Profils** liste les profils, **Nouveau profil...** et **Ouvrir le dossier du profil** ; il n'y a pas de **Créer un raccourci...**.
- Ouvrir un fichier `.torrent` ou un lien `magnet:` atteint toujours le profil par défaut : c'est macOS qui choisit l'application, pas un profil. Le démarrage avec le système se règle par profil.

## Profils de rookparcel-nox

`rookparcel-nox` accepte aussi `--profile NOM`, pour faire tourner plusieurs démons sur un même ordinateur, chacun avec ses torrents, son interface web et son service. Un profil de démon ne demande pas de confirmation avant d'être créé. Son port d'écoute découle de son nom, donc il reste le même à chaque démarrage. Un profil d'interface graphique et un profil `rookparcel-nox` de même nom sont des programmes distincts qui ne partagent que le dossier `profiles/<nom>`. Voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

## Supprimer un profil

1. Fermez la fenêtre du profil (**Fichier > Quitter**).
2. S'il possède les associations ou démarre avec le système, décochez-les d'abord dans ses Préférences, ou donnez-les à un autre profil.
3. Supprimez son dossier, `profiles\<nom>`. **Fichier > Profils > Ouvrir le dossier du profil** indique où il se trouve.

Les téléchargements du profil restent où ils sont.
