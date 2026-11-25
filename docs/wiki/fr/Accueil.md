*Ceci est la traduction française de la page [Home](Home) du wiki anglais. La page anglaise fait foi.*

# Wiki de Rookparcel

Rookparcel est un client BitTorrent léger dans l'esprit de µTorrent 2.2.1 : petit, rapide et natif. Ce wiki explique comment l'utiliser : l'installer, ajouter et gérer des torrents, les réglages, l'interface web et le démon sans fenêtre.

Rookparcel existe sous la forme de quatre programmes :

| Programme | Où | Ce que c'est |
|---|---|---|
| `rookparcel.exe` | Windows 10 et 11 | la fenêtre Windows (interface graphique) |
| `rookparcel-gtk` | Linux, BSD | la fenêtre GTK 4 (interface graphique) : la même fenêtre principale et les mêmes boîtes de dialogue, avec quelques différences indiquées sur chaque page |
| `Rookparcel.app` | macOS 11 ou plus récent, Apple silicon et Intel | la fenêtre macOS native (interface graphique) : la même fenêtre principale et les mêmes boîtes de dialogue, avec la barre des menus du Mac et les raccourcis <kbd>⌘</kbd> |
| `rookparcel-nox` | tous | fonctionne sans fenêtre, piloté depuis un navigateur par son interface web |

La plupart des pages décrivent la fenêtre Windows, puis indiquent ce qui diffère dans la fenêtre GTK et dans l'interface web. L'application macOS fonctionne comme la fenêtre GTK ; là où elle diffère, la page a une section macOS : [Installer Rookparcel](Installer-Rookparcel#macos), [Raccourcis clavier](Raccourcis-clavier#macos), [Zone de notification et démarrage](Zone-de-notification-et-démarrage#macos), [Référence des préférences](Référence-des-préférences#différences-sous-macos) et [Arrêt automatique](Arrêt-automatique). *L'application macOS native existe depuis la version 1.2.19 ; les versions précédentes proposent la fenêtre GTK pour Apple silicon sous macOS.*

**Version :** le wiki suit le code actuel (1.2.81), qui est aussi la dernière version publiée : [1.2.81](https://github.com/Balls0fSteel/Rookparcel/releases/latest). Les pages indiquent dans quelle version une fonction récente est apparue.

**Autres langues :** [English](Home) · [Français](Accueil) · [Español](Inicio) · [Deutsch](Startseite) · [Magyar](Kezdőlap). Les traductions suivent ce wiki anglais ; en cas de différence, la page anglaise fait foi.

## Pour commencer

- [Installer Rookparcel](Installer-Rookparcel)
- [Premier démarrage et assistant de configuration](Premier-démarrage)
- [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel) (et mode portable)
- [Profils](Profils) : plusieurs Rookparcel indépendants côte à côte
- [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel)

## Utilisation courante

- [Ajouter des torrents](Ajouter-des-torrents) : fichiers, liens magnet, URL, la fenêtre Ajouter un nouveau torrent
- [La fenêtre principale](La-fenêtre-principale)
- [Gérer les torrents](Gérer-les-torrents) : démarrer, arrêter, supprimer, étiquettes et tags, ordre de la file
- [Détails et fichiers des torrents](Détails-et-fichiers-des-torrents) : les onglets de détails, le choix des fichiers, les propriétés du torrent
- [Créer des torrents](Créer-des-torrents)
- [Importer depuis µTorrent et qBittorrent](Importer-depuis-uTorrent-et-qBittorrent)
- [Rechercher des torrents](Rechercher-des-torrents) et, pas à pas, [Rechercher des torrents sous Windows](Rechercher-des-torrents-sous-Windows)
- [Téléchargeur RSS](Téléchargeur-RSS)

## Vitesse, réseau et disque

- [Limites de vitesse et planificateur](Limites-de-vitesse-et-planificateur)
- [Garder un ping bas](Garder-un-ping-bas) (mode joueur)
- [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu), VPN et proxys
- [Mise en file et partage](Mise-en-file-et-partage)
- [Disque et stockage](Disque-et-stockage)
- [Arrêt automatique](Arrêt-automatique)

## À distance et sans fenêtre

- [Interface web](Interface-Web)
- [Contrôle à distance](Contrôle-à-distance) : une fenêtre de bureau qui pilote un Rookparcel installé ailleurs
- [Utiliser rookparcel-nox](Utiliser-rookparcel-nox) : le démon sans fenêtre et les services
- [Rookparcel avec Docker](Rookparcel-avec-Docker) : rookparcel-nox comme image de conteneur
- [Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr)

## Référence

- [Référence des préférences](Référence-des-préférences)
- [Mode sombre](Mode-sombre)
- [Raccourcis clavier](Raccourcis-clavier)
- [Zone de notification et démarrage](Zone-de-notification-et-démarrage)
- [Verrouiller Rookparcel](Verrouiller-Rookparcel) : masquer les fenêtres derrière un code PIN, et supprimer un code oublié
- [Dépannage et FAQ](Dépannage-et-FAQ)

## Autres liens

- [Site web](https://balls0fsteel.github.io/Rookparcel/) : téléchargements, actualités, FAQ, benchmarks
- [Versions publiées](https://github.com/Balls0fSteel/Rookparcel/releases)
- [Rapports de bogues et questions](https://github.com/Balls0fSteel/Rookparcel/issues)
- Documentation pour les développeurs : le dossier [docs](https://github.com/Balls0fSteel/Rookparcel/tree/main/docs) du dépôt
