*Cette page est la traduction de la page anglaise [Keyboard shortcuts](https://github.com/Balls0fSteel/Rookparcel/wiki/Keyboard-shortcuts). En cas de différence, la page anglaise fait foi.*

# Raccourcis clavier

Cette page liste les raccourcis clavier de la version Windows de Rookparcel, puis ceux de la version GTK, de l'application macOS et de l'interface web. La plupart sont aussi écrits à côté de leurs éléments de menu.

## Fenêtre principale (Windows)

| Touches | Ce qu'elles font |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>O</kbd> | **Fichier > Ajouter un torrent...** |
| <kbd>Ctrl</kbd>+<kbd>U</kbd> | **Fichier > Ajouter un torrent depuis une URL...** (un lien magnet ou une adresse web présent dans le presse-papiers est repris) |
| <kbd>Ctrl</kbd>+<kbd>N</kbd> | **Fichier > Créer un nouveau torrent...** |
| <kbd>Ctrl</kbd>+<kbd>P</kbd> | **Options > Préférences** |
| <kbd>Ctrl</kbd>+<kbd>G</kbd> | **Options > Assistant de configuration** |
| <kbd>Ctrl</kbd>+<kbd>L</kbd> | **Fichier > Verrouiller Rookparcel** : masque toutes les fenêtres de Rookparcel jusqu'à la saisie du code PIN de verrouillage (voir [Verrouiller Rookparcel](Verrouiller-Rookparcel)) |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> | Va au champ de recherche de la barre d'outils (lorsque la barre d'outils est affichée). Dans l'onglet Fichiers, va plutôt au champ de filtre de l'onglet Fichiers. |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Sélectionne tous les torrents de la liste. Dans l'onglet Fichiers, sélectionne les fichiers que le filtre affiche. |
| <kbd>Ctrl</kbd>+<kbd>S</kbd> | **Démarrer** les torrents sélectionnés |
| <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd> | **Pause** pour les torrents sélectionnés |
| <kbd>Ctrl</kbd>+<kbd>T</kbd> | **Arrêter** les torrents sélectionnés |
| <kbd>Del</kbd> | **Supprimer** les torrents sélectionnés |
| <kbd>Shift</kbd>+<kbd>Del</kbd> | **Supprimer et > Effacer les données** : supprime les torrents et efface leurs fichiers |
| <kbd>Enter</kbd> | **Propriétés** des torrents sélectionnés (dans la liste des torrents) |
| <kbd>Shift</kbd>+<kbd>F10</kbd> ou la touche Menu | Ouvre le menu du clic droit sous la ligne qui a le focus |
| <kbd>Alt</kbd>+<kbd>F4</kbd> | Ferme la fenêtre. Avec **Fermer dans la zone de notification** activé (par défaut), Rookparcel continue de fonctionner dans la zone de notification. |

Avec **Confirmer la suppression des torrents** activé dans les Préférences, <kbd>Del</kbd> et <kbd>Shift</kbd>+<kbd>Del</kbd> demandent d'abord confirmation. **Fichier > Quitter** n'a pas de raccourci ; voir [Zone de notification, notifications et démarrage](Zone-de-notification-et-démarrage).

*<kbd>Ctrl</kbd>+<kbd>G</kbd> et l'Assistant de configuration sont nouveaux dans la 1.1.62, le champ de filtre de l'onglet Fichiers dans la 1.1.56, et <kbd>Ctrl</kbd>+<kbd>L</kbd> dans la 1.2.54.*

Raccourcis à la souris qui vont avec :

- Double-cliquer sur un torrent terminé ouvre son dossier ; double-cliquer sur un torrent non terminé ouvre **Propriétés**.
- Dans l'onglet Fichiers, double-cliquer sur un fichier ou appuyer sur <kbd>Enter</kbd> dessus l'ouvre.

Dans l'arborescence des dossiers de l'onglet Fichiers, <kbd>Right</kbd> ou <kbd>+</kbd> ouvre le dossier qui a le focus, et <kbd>Left</kbd> ou <kbd>-</kbd> le ferme ; sur un fichier ou un dossier fermé, <kbd>Left</kbd> va au dossier qui le contient. Dans la liste des torrents et dans les onglets de détails, taper les premières lettres d'un nom sélectionne la ligne suivante dont le nom commence par elles, comme dans l'Explorateur ; faites une pause d'une seconde pour recommencer. *Les touches des dossiers et la saisie sont nouvelles dans la 1.2.21.*

## Le champ de recherche et le filtre de l'onglet Fichiers

| Touches | Dans le champ de recherche de la barre d'outils | Dans le champ de filtre de l'onglet Fichiers |
|---|---|---|
| <kbd>Escape</kbd> | Vide le champ et va à la liste des torrents | Vide le champ et va à la liste des fichiers |
| <kbd>Enter</kbd> | Avec **Filtrer les torrents** choisi, va à la liste. Avec une source de recherche choisie, lance la recherche dans cette source. | Va à la liste des fichiers |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Sélectionne le texte du champ | Sélectionne le texte du champ (<kbd>Ctrl</kbd>+<kbd>F</kbd> aussi) |
| <kbd>Alt</kbd>+<kbd>Down</kbd> | Ouvre le menu déroulant du champ (sources, **Gérer les sources...**) | |

Tant que vous tapez dans l'un ou l'autre champ, <kbd>Del</kbd> et <kbd>Ctrl</kbd>+<kbd>A</kbd> modifient le texte, pas la liste. Voir [Rechercher des torrents](Rechercher-des-torrents) et [Détails et fichiers des torrents](Détails-et-fichiers-des-torrents).

## Fenêtre Ajouter un nouveau torrent

Dans la liste des fichiers de la fenêtre Ajouter un nouveau torrent :

| Touches | Ce qu'elles font |
|---|---|
| <kbd>Space</kbd> | Coche ou décoche les fichiers et dossiers sélectionnés |
| <kbd>Right</kbd> ou <kbd>+</kbd> | Ouvre le dossier qui a le focus |
| <kbd>Left</kbd> ou <kbd>-</kbd> | Ferme le dossier qui a le focus ; sur un fichier ou un dossier fermé, <kbd>Left</kbd> va à son dossier parent |
| <kbd>Enter</kbd> | Ouvre ou ferme le dossier qui a le focus |
| Lettres | Sélectionnent la ligne suivante dont le nom commence par elles |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> | Va au champ **Filtrer les fichiers** au-dessus de la liste, avec son texte sélectionné |

Dans le champ de filtre, <kbd>Enter</kbd> va à la liste, et <kbd>Escape</kbd> vide le champ ; quand le champ est vide, <kbd>Escape</kbd> ferme la fenêtre comme **Annuler**. *Le champ de filtre et la saisie sont nouveaux dans la 1.2.21.*

Voir [Ajouter des torrents](Ajouter-des-torrents).

## Préférences et autres fenêtres

| Touches | Ce qu'elles font |
|---|---|
| <kbd>F1</kbd> dans Préférences | Affiche l'aide du réglage qui a le focus à côté de lui, et dans le volet d'aide sous les pages |
| <kbd>Escape</kbd> | Ferme une boîte de dialogue comme **Annuler**. Dans l'Assistant de configuration, c'est **Ignorer**. |
| <kbd>Tab</kbd>, <kbd>Enter</kbd>, <kbd>Escape</kbd> dans la fenêtre Rechercher des torrents | Passent d'un contrôle à l'autre, agissent sur celui qui a le focus, la ferment |

L'aide des Préférences apparaît aussi sous forme d'infobulle quand vous pointez un réglage. Dans la fenêtre principale, un élément de menu ou un bouton de la barre d'outils que vous pointez est décrit dans la barre d'état. *Ces textes d'aide sont nouveaux dans la 1.1.57.*

Voir [Référence des préférences](Référence-des-préférences).

## Touches des menus

Chaque menu est accessible avec <kbd>Alt</kbd> et la lettre soulignée, comme dans les autres programmes Windows : <kbd>Alt</kbd>+<kbd>F</kbd> pour **Fichier**, <kbd>Alt</kbd>+<kbd>O</kbd> pour **Options**, <kbd>Alt</kbd>+<kbd>H</kbd> pour **Aide**, puis la lettre soulignée d'un élément.

## Version GTK (Linux, BSD)

La version GTK a ces raccourcis :

| Touches | Ce qu'elles font |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>O</kbd> | **Ajouter un torrent...** |
| <kbd>Ctrl</kbd>+<kbd>U</kbd> | **Ajouter un torrent depuis une URL...** |
| <kbd>Ctrl</kbd>+<kbd>N</kbd> | **Créer un nouveau torrent...** |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> | Va au champ de recherche |
| <kbd>Ctrl</kbd>+<kbd>P</kbd> | **Préférences** |
| <kbd>Ctrl</kbd>+<kbd>G</kbd> | **Assistant de configuration** |
| <kbd>Ctrl</kbd>+<kbd>L</kbd> | **Verrouiller Rookparcel**. Il fonctionne dans toutes les fenêtres de Rookparcel (Préférences, Propriétés, Ajouter un nouveau torrent et les autres), pas seulement dans la fenêtre principale. *Nouveauté de la 1.2.76 ; auparavant, il ne faisait rien tant qu'une boîte de dialogue était ouverte.* |
| <kbd>Ctrl</kbd>+<kbd>Q</kbd> | **Quitter** : quitte Rookparcel, même quand la fenêtre se fermerait dans la zone de notification |
| <kbd>Ctrl</kbd>+<kbd>S</kbd> | **Démarrer** les torrents sélectionnés |
| <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd> | **Pause** pour les torrents sélectionnés |
| <kbd>Ctrl</kbd>+<kbd>T</kbd> | **Arrêter** les torrents sélectionnés |
| <kbd>Delete</kbd> | **Supprimer** |
| <kbd>Shift</kbd>+<kbd>Delete</kbd> | Supprimer et effacer les données |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Sélectionne tous les torrents ; dans l'onglet Fichiers, les fichiers que son filtre affiche |
| <kbd>Enter</kbd> dans la liste des torrents | **Propriétés** |
| <kbd>F1</kbd> dans Préférences | Affiche l'aide du réglage qui a le focus |
| <kbd>Escape</kbd> | Vide le champ de recherche ou le filtre de l'onglet Fichiers ; ferme les boîtes de dialogue |

Ce qui diffère de Windows :

- Démarrer, Pause, Arrêter, Propriétés et Supprimer ont les mêmes touches que sous Windows.
- <kbd>Ctrl</kbd>+<kbd>F</kbd> va au champ de filtre de l'onglet Fichiers quand le focus est dans l'onglet Fichiers, comme sous Windows.
- Dans l'arborescence des dossiers de l'onglet Fichiers, <kbd>Right</kbd> et <kbd>Left</kbd> ouvrent et ferment les dossiers comme sous Windows.
- <kbd>Ctrl</kbd>+<kbd>Q</kbd> quitte ; sous Windows, il n'y a pas de raccourci pour quitter.
- Les touches de la fenêtre Ajouter un nouveau torrent sont les mêmes que sous Windows.

## macOS

L'application macOS utilise <kbd>⌘</kbd> (Commande) là où Windows utilise <kbd>Ctrl</kbd>, et les touches propres au Mac là où elles diffèrent. Les raccourcis sont écrits à côté des éléments de menu dans la barre des menus.

| Touches | Ce qu'elles font |
|---|---|
| <kbd>⌘</kbd>+<kbd>,</kbd> | **Rookparcel > Préférences** |
| <kbd>⌘</kbd>+<kbd>G</kbd> | **Rookparcel > Assistant de configuration** |
| <kbd>⌘</kbd>+<kbd>H</kbd> | Masque Rookparcel (<kbd>⌥</kbd>+<kbd>⌘</kbd>+<kbd>H</kbd> masque les autres apps) |
| <kbd>⌘</kbd>+<kbd>Q</kbd> | **Quitter Rookparcel**, même quand la fenêtre se fermerait dans la barre des menus |
| <kbd>⌘</kbd>+<kbd>O</kbd> | **Fichier > Ajouter un torrent...** |
| <kbd>⌘</kbd>+<kbd>U</kbd> | **Fichier > Ajouter un torrent depuis une URL...** |
| <kbd>⌘</kbd>+<kbd>N</kbd> | **Fichier > Créer un nouveau torrent...** |
| <kbd>⌘</kbd>+<kbd>L</kbd> | **Fichier > Verrouiller Rookparcel** |
| <kbd>⌘</kbd>+<kbd>W</kbd> | **Fichier > Fermer la fenêtre** : avec **Fermer dans la zone de notification** activé, Rookparcel continue de fonctionner dans la barre des menus |
| <kbd>⌘</kbd>+<kbd>A</kbd> | **Édition > Tout sélectionner** : tous les torrents de la liste, ou le texte d'un champ |
| <kbd>⌘</kbd>+<kbd>F</kbd> | **Édition > Rechercher...** : va au champ de recherche de la barre d'outils ; dans l'onglet Fichiers, à son champ de filtre |
| <kbd>⌥</kbd>+<kbd>⌘</kbd>+<kbd>T</kbd> | **Présentation > Afficher la barre d'outils** |
| <kbd>⌃</kbd>+<kbd>⌘</kbd>+<kbd>S</kbd> | **Présentation > Afficher la barre latérale** (la liste des catégories) |
| <kbd>⌃</kbd>+<kbd>⌘</kbd>+<kbd>F</kbd> | **Présentation > Passer en mode plein écran** |
| <kbd>⌘</kbd>+<kbd>S</kbd> | **Torrent > Démarrer** |
| <kbd>⇧</kbd>+<kbd>⌘</kbd>+<kbd>P</kbd> | **Torrent > Pause** |
| <kbd>⌘</kbd>+<kbd>T</kbd> | **Torrent > Arrêter** |
| <kbd>⌘</kbd>+<kbd>⌫</kbd>, ou <kbd>⌫</kbd> dans la liste | **Torrent > Supprimer** |
| <kbd>⌥</kbd>+<kbd>⌘</kbd>+<kbd>⌫</kbd> | **Torrent > Supprimer et effacer les données** : supprime les torrents et efface leurs fichiers |
| <kbd>⌘</kbd>+<kbd>I</kbd>, ou <kbd>Return</kbd> dans la liste | **Torrent > Propriétés** |
| <kbd>⌘</kbd>+<kbd>M</kbd> | **Fenêtre > Placer dans le Dock** |

Ce qui diffère de Windows :

- Préférences s'obtient par <kbd>⌘</kbd>+<kbd>,</kbd>, comme dans les autres apps Mac, et non par <kbd>Ctrl</kbd>+<kbd>P</kbd>.
- Supprimer est <kbd>⌘</kbd>+<kbd>⌫</kbd> (Suppr.), et Propriétés <kbd>⌘</kbd>+<kbd>I</kbd>.
- Le menu du clic droit de la liste des torrents a les mêmes éléments que le menu **Torrent** ; Contrôle-clic l'ouvre aussi.
- **Édition** a les habituels Annuler, Couper, Copier et Coller pour les champs de texte.

*L'application macOS native est nouvelle dans la 1.2.19.*

## Interface web

Dans la liste des torrents de l'interface web (cliquez d'abord sur la liste pour qu'elle reçoive le clavier) :

| Touches | Ce qu'elles font |
|---|---|
| <kbd>Up</kbd>, <kbd>Down</kbd>, <kbd>Page Up</kbd>, <kbd>Page Down</kbd>, <kbd>Home</kbd>, <kbd>End</kbd> | Déplacent la sélection |
| <kbd>Shift</kbd> avec ces touches | Étend la sélection |
| <kbd>Ctrl</kbd> avec ces touches | Déplace le focus sans changer la sélection |
| <kbd>Ctrl</kbd>+<kbd>Space</kbd> | Ajoute la ligne qui a le focus à la sélection, ou l'en retire |
| <kbd>Space</kbd> | **Pause** pour les torrents sélectionnés |
| <kbd>Delete</kbd> | **Supprimer** |
| <kbd>Shift</kbd>+<kbd>Delete</kbd> | Supprimer et effacer les données |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Sélectionne tous les torrents |
| <kbd>Shift</kbd>+<kbd>F10</kbd> ou la touche Menu | Ouvre le menu du clic droit |
| <kbd>Escape</kbd> | Ferme un menu ouvert |

Ailleurs dans l'interface web :

| Touches | Ce qu'elles font |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>P</kbd> | Ouvre les **Préférences** (pas tant que la fenêtre Ajouter ou Propriétés est ouverte) |
| <kbd>Escape</kbd> dans le champ de filtre | Le vide et va à la liste |
| <kbd>Enter</kbd> dans le champ de filtre | Dans la fenêtre Ajouter : va à la liste |
| <kbd>F1</kbd> dans Préférences | Affiche l'aide du réglage qui a le focus |
| <kbd>Ctrl</kbd>+<kbd>Enter</kbd> dans Préférences | **OK** |
| <kbd>Enter</kbd> dans le champ de valeur de la page Avancé | Définit la valeur |
| <kbd>Escape</kbd> dans Préférences | Les ferme comme **Annuler** |
| <kbd>Up</kbd>, <kbd>Down</kbd>, <kbd>Home</kbd>, <kbd>End</kbd> dans la liste des pages des Préférences | Passent d'une page à l'autre |
| <kbd>Ctrl</kbd>+<kbd>Enter</kbd> dans Propriétés du torrent | **OK** (<kbd>Enter</kbd> seul ajoute une ligne dans la zone des trackers) |

Dans l'onglet Fichiers et dans la liste des fichiers de la fenêtre Ajouter :

| Touches | Ce qu'elles font |
|---|---|
| <kbd>Up</kbd>, <kbd>Down</kbd>, <kbd>Home</kbd>, <kbd>End</kbd> | Déplacent le focus (avec <kbd>Shift</kbd>, étendent la sélection) |
| <kbd>Right</kbd>, <kbd>Left</kbd> | Dans l'arborescence des dossiers : ouvrent ou ferment un dossier ; sur un fichier ou un dossier fermé, <kbd>Left</kbd> va au dossier parent, et <kbd>Right</kbd> sur un dossier ouvert va à sa première entrée |
| <kbd>Space</kbd> | Dans la fenêtre Ajouter : coche ou décoche les fichiers et dossiers sélectionnés |
| Lettres | Sélectionnent la ligne suivante dont le nom commence par elles |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> | Va au champ de filtre |
| <kbd>Ctrl</kbd>+<kbd>A</kbd> | Dans l'onglet Fichiers : sélectionne les fichiers visibles |
| <kbd>Escape</kbd> dans le champ de filtre | Le vide (dans la fenêtre Ajouter, ferme la fenêtre quand il est déjà vide) |
| <kbd>Enter</kbd> dans le champ de filtre | Va à la liste |

Sur un Mac, <kbd>Command</kbd> fonctionne partout où l'interface web indique <kbd>Ctrl</kbd>. Les raccourcis propres à votre navigateur peuvent prendre certaines touches en premier. Voir [Interface web](Interface-Web).
