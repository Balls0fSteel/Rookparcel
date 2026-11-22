*Ceci est la traduction française de la page [Installing Rookparcel](Installing-Rookparcel) du wiki anglais. La page anglaise fait foi.*

# Installer Rookparcel

Cette page liste les téléchargements pour chaque plateforme, explique comment installer et lancer chacun la première fois, et comment retirer Rookparcel. Après le premier démarrage, voyez [Premier démarrage et assistant de configuration](Premier-démarrage).

## Où télécharger

Chaque version se trouve sur la [page des versions](https://github.com/Balls0fSteel/Rookparcel/releases). La dernière version publiée est la 1.2.76. Chaque version a des notes avec les étapes du premier lancement pour chaque plateforme, et un fichier `SHA256SUMS` avec la somme de contrôle de chaque téléchargement.

| Plateforme | Fichier | Contenu |
|---|---|---|
| Windows 10 / 11, x64, installeur | `rookparcel-<version>-windows-x64-setup.exe` | `rookparcel.exe` (la fenêtre) et `rookparcel-nox.exe` (le démon sans fenêtre), installés avec une entrée du menu Démarrer et un désinstalleur |
| Windows 10 / 11, x64, portable | `rookparcel-<version>-windows-x64.zip` | les deux mêmes programmes ; rien à installer |
| Bureau Linux, x86_64 ou aarch64 | `Rookparcel-<version>-<arch>.AppImage` | la fenêtre GTK 4 avec ses bibliothèques incluses (demande glibc 2.39 ou plus récent : Ubuntu 24.04, Fedora 40, Debian 13 ou plus récent) |
| Bureau Linux, Flatpak | `Rookparcel-<version>-<arch>.flatpak` | la fenêtre GTK 4 sur l'environnement d'exécution GNOME 51 de Flathub |
| Linux, binaires simples | `rookparcel-<version>-linux-<arch>.tar.gz` | `rookparcel-gtk`, `rookparcel-nox`, l'entrée de bureau et les unités systemd ; ils utilisent le GTK 4 (4.12 ou plus récent) et l'OpenSSL 3 du système |
| Serveur Linux, sans fenêtre | `rookparcel-nox-<version>-<arch>.AppImage` | `rookparcel-nox` avec OpenSSL inclus |
| macOS, Apple silicon ou Intel | `Rookparcel-<version>-macos.dmg` | `Rookparcel.app`, la fenêtre macOS native, une seule application pour les deux types de Mac (macOS 11 Big Sur ou plus récent) |
| macOS, sans fenêtre | `rookparcel-nox-<version>-macos-<arch>.tar.gz` | `rookparcel-nox` pour Apple silicon (`arm64`, macOS 11 ou plus récent) et Intel (`x86_64`, macOS 10.14 ou plus récent) |

La fenêtre sous Linux et les BSD est la version GTK ; sous macOS, c'est une application Mac native. Les deux fonctionnent comme la version Windows ; là où elles diffèrent, les pages du wiki l'indiquent. Pour faire tourner le démon sans fenêtre, voyez [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

*L'application macOS native existe depuis la version 1.2.19. Les versions précédentes proposent à la place `Rookparcel-<version>-arm64.dmg` : la fenêtre GTK incluse pour Apple silicon, macOS 15 ou plus récent.*

## Windows

### Installeur ou zip

- **L'installeur** place Rookparcel dans votre menu Démarrer, peut l'associer aux fichiers `.torrent` et aux liens `magnet:`, ajoute une entrée sous Paramètres > Applications pour le désinstaller, et permet à Rookparcel de se mettre à jour lui-même (voir [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel)).
- **Le zip** n'a rien à installer : décompressez-le où vous voulez et lancez `rookparcel.exe`. Il peut fonctionner en mode portable, avec ses réglages et ses torrents gardés dans son propre dossier (voir [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel#mode-portable)). Il ne se met pas à jour lui-même ; une version plus récente est un nouveau zip.

Les deux contiennent les mêmes programmes. Rookparcel demande un Windows 10 ou plus récent en 64 bits. Windows 11 sur Arm exécute la version x64.

### Lancer l'installeur

1. Lancez `rookparcel-<version>-windows-x64-setup.exe`.
2. Windows SmartScreen peut afficher « Windows a protégé votre ordinateur », parce que l'installeur n'est pas signé. Cliquez sur **Informations complémentaires**, puis sur **Exécuter quand même**.
3. Choisissez la langue de l'installeur, lisez la page d'accueil et acceptez la licence (la PolyForm Noncommercial License 1.0.0).
4. **Choix des utilisateurs :**
   - **Installer seulement pour moi** (le choix par défaut) ne demande aucun droit d'administrateur. Rookparcel est placé dans `%LOCALAPPDATA%\Programs\Rookparcel`.
   - **Installer pour tous les utilisateurs de cet ordinateur** installe dans `C:\Program Files\Rookparcel`. Windows demande les droits d'administrateur, et la fenêtre indique « Éditeur inconnu ». Si vous refusez, vous revenez à cette page.
5. **Composants :** Rookparcel lui-même, **Raccourci sur le Bureau**, **Associer aux fichiers .torrent** et **Associer aux liens magnet:**. Les trois dernières sont cochées pour une nouvelle installation.
6. Choisissez le dossier, ou gardez celui qui est affiché, et laissez l'installation se faire.
7. Sur la dernière page, laissez cochée la case « Exécuter Rookparcel » pour le lancer.

Les associations sont écrites pour l'utilisateur qui lance l'installeur. Les autres utilisateurs d'une installation pour tous peuvent les activer dans **Options > Préférences > Général**.

L'installeur refuse un dossier qui contient un fichier `rookparcel.ini` : ce dossier est un Rookparcel portable. Choisissez un autre dossier.

### Installer par-dessus une ancienne version

Lancez le nouvel installeur. Il trouve le Rookparcel installé, saute les pages des utilisateurs et du dossier, et installe par-dessus, dans le même dossier. Vos réglages et vos torrents restent où ils sont. Lors d'une mise à jour, les cases d'association sont décochées au départ, ce qui laisse les associations telles qu'elles sont.

Si Rookparcel est en cours d'exécution, l'installeur vous demande de le quitter (**Fichier > Quitter**) et de cliquer sur **Recommencer**. Si `rookparcel-nox.exe` du même dossier est en cours d'exécution, par exemple comme service Rookparcel nox, il vous demande de l'arrêter d'abord. Depuis une invite de commandes d'administrateur :

```
sc stop RookparcelNox
```

Relancez le service après la mise à jour. Une copie installée peut aussi faire tout cela toute seule : voir [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel).

### Installation silencieuse

L'installeur accepte ces options, pour les scripts et les administrateurs :

| Option | Effet |
|---|---|
| `/S` | silencieux : aucune page, rien n'est lancé à la fin |
| `/D=<dossier>` | le dossier d'installation ; doit être le dernier argument, sans guillemets |
| `/CURRENTUSER`, `/ALLUSERS` | pour moi ou pour tous, sans la page des utilisateurs. `/ALLUSERS` sans droits d'administrateur les demande par l'invite de Windows, même en mode silencieux |
| `/DESKTOPSHORTCUT` | crée le raccourci sur le Bureau (une installation silencieuse n'en crée pas autrement) |
| `/ASSOCIATE=torrent,magnet` | les associations : `torrent`, `magnet` ou les deux |

Par exemple, depuis une invite de commandes d'administrateur :

```
rookparcel-1.2.76-windows-x64-setup.exe /S /ALLUSERS /ASSOCIATE=torrent,magnet
```

### Le zip

1. Décompressez `rookparcel-<version>-windows-x64.zip` dans un dossier où vous pouvez écrire.
2. Lancez `rookparcel.exe`. SmartScreen avertit la première fois, comme ci-dessus : **Informations complémentaires**, puis **Exécuter quand même**.

Les réglages et les torrents vont dans `%APPDATA%\Rookparcel`, au même endroit qu'une copie installée. Pour les garder plutôt dans le dossier du zip, placez un fichier vide nommé `rookparcel.ini` à côté de `rookparcel.exe` avant le premier démarrage. Voir [Mode portable](Emplacement-des-fichiers-de-Rookparcel#mode-portable).

Pour ouvrir les fichiers `.torrent` et les liens `magnet:` avec la copie du zip, cochez les associations dans **Options > Préférences > Général**. Elles désignent alors le `rookparcel.exe` de cette copie, et cessent de fonctionner si vous déplacez ou supprimez son dossier.

### Vérifier un téléchargement

Le `SHA256SUMS` de chaque version donne le SHA-256 de chaque fichier. Dans PowerShell :

```
Get-FileHash .\rookparcel-1.2.76-windows-x64-setup.exe -Algorithm SHA256
```

Le hash doit correspondre à la ligne du fichier dans `SHA256SUMS`.

## Linux

### AppImage

1. Téléchargez `Rookparcel-<version>-x86_64.AppImage` (ou `-aarch64` sur ARM).
2. Rendez-le exécutable et lancez-le :

   ```bash
   chmod +x Rookparcel-1.2.76-x86_64.AppImage
   ./Rookparcel-1.2.76-x86_64.AppImage
   ```

3. Sans FUSE 2 sur le système, lancez-le avec `--appimage-extract-and-run` en plus.

L'AppImage demande glibc 2.39 ou plus récent (Ubuntu 24.04, Fedora 40, Debian 13 ou plus récent). Les réglages vont dans `~/.config/rookparcel/gtk`, les torrents dans `~/.local/share/rookparcel/gtk`. Un fichier `rookparcel.ini` à côté du fichier `.AppImage` le rend portable à la place (*nouveau dans la 1.1.49*).

### Flatpak

Le paquet contient l'application ; l'environnement d'exécution vient de Flathub. Ajoutez Flathub une seule fois, puis installez le paquet :

```bash
flatpak remote-add --if-not-exists --user flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install --user Rookparcel-1.2.76-x86_64.flatpak
flatpak run io.github.balls0fsteel.Rookparcel
```

Ensuite, il est dans le menu de vos applications. Le Flatpak peut écrire dans votre dossier personnel ; pour un dossier de téléchargement sur un autre disque, donnez-lui l'accès une fois, par exemple `flatpak override --user --filesystem=/mnt/data io.github.balls0fsteel.Rookparcel` (avec votre dossier). Le Flatpak n'a pas de recherche de mises à jour propre et pas de mode portable. Ses associations des fichiers `.torrent` et des liens `magnet:` relèvent du bureau : choisissez Rookparcel comme programme par défaut dans les paramètres de votre bureau.

### Binaires simples (tar.gz)

`rookparcel-<version>-linux-<arch>.tar.gz` contient `rookparcel-gtk`, `rookparcel-nox`, l'entrée de bureau et les unités systemd (`systemd/system`, `systemd/user`). Les binaires utilisent le GTK 4 (4.12 ou plus récent) et l'OpenSSL 3 du système, tels qu'Ubuntu 24.04 les fournit, donc ils fonctionnent sur des distributions de cet âge ou plus récentes.

Décompressez l'archive et lancez `rookparcel-gtk`. Un fichier `rookparcel.ini` à côté du binaire le rend portable (*nouveau dans la 1.1.49*). Pour les unités systemd, voyez [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

### Serveur sans fenêtre

`rookparcel-nox-<version>-<arch>.AppImage` est le démon seul, avec OpenSSL inclus ; il reste en dehors des menus. Voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

## macOS

1. Ouvrez `Rookparcel-<version>-macos.dmg` et faites glisser `Rookparcel.app` dans Applications.
2. L'application est signée ad hoc et non notariée, donc Gatekeeper la refuse la première fois. Autorisez-la une fois :
   - **macOS 15 Sequoia et plus récent :** double-cliquez sur l'application et cliquez sur **Terminé** dans l'avertissement. Ouvrez **Réglages Système > Confidentialité et sécurité**, faites défiler jusqu'à **Sécurité** et cliquez sur **Ouvrir quand même** (le bouton reste là environ une heure). Confirmez avec **Ouvrir quand même** et votre mot de passe ou Touch ID.
   - **macOS 11 à 14 :** Contrôle-clic (ou clic droit) sur l'application, choisissez **Ouvrir**, puis cliquez sur **Ouvrir**.
3. Ou, y compris quand macOS dit que l'application « est endommagée », retirez l'indicateur de quarantaine dans Terminal :

   ```bash
   xattr -dr com.apple.quarantine /Applications/Rookparcel.app
   ```

L'image disque contient ces étapes dans `How to open Rookparcel.html`, et le [guide d'installation](https://balls0fsteel.github.io/Rookparcel/install.html#macos) du site les montre avec des images.

L'application fonctionne sous macOS 11 Big Sur ou plus récent, sur les Mac Apple silicon comme Intel : l'unique `Rookparcel.app` contient les deux. Les réglages et les torrents vont dans `~/Library/Application Support/Rookparcel/mac`.

### Passer de la version GTK

Les versions précédentes avaient une version GTK de `Rookparcel.app` pour Apple silicon. Remplacez-la par la nouvelle `Rookparcel.app` dans Applications. À son premier démarrage, la nouvelle application renomme le dossier `gtk` de l'ancienne dans `~/Library/Application Support/Rookparcel` en `mac`, et celui de chaque profil aussi, si bien que vos réglages, vos torrents et vos profils sont repris. Cela n'arrive qu'une fois, et seulement tant qu'aucun dossier `mac` n'existe. Quittez d'abord l'ancienne application : tant qu'elle tourne, la nouvelle affiche « Un autre Rookparcel utilise déjà ce dossier » et se ferme. Voir [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel#macos).

Une entrée « Lancer Rookparcel au démarrage du système » créée par l'ancienne application continue de fonctionner. Un dossier portable est utilisé tel quel.

Pour le démon sans fenêtre, décompressez `rookparcel-nox-<version>-macos-<arch>.tar.gz` et lancez `rookparcel-nox` depuis Terminal ([Utiliser rookparcel-nox](Utiliser-rookparcel-nox)).

## Désinstaller

### Installeur Windows

1. Quittez Rookparcel (**Fichier > Quitter**).
2. Ouvrez **Paramètres > Applications**, trouvez **Rookparcel** et choisissez **Désinstaller**.
3. Le désinstalleur a une seule option, **Paramètres et liste des torrents**, décochée par défaut. Cochez-la pour supprimer aussi ce que Rookparcel garde dans `%APPDATA%\Rookparcel` : les réglages, la liste des torrents et leur progression, les flux RSS, les connexions à distance, le filtre IP par défaut, les fichiers de nox et les profils.

Le désinstalleur retire les associations, l'entrée Applications par défaut et l'entrée « Lancer Rookparcel au démarrage du système » qui désignent le `rookparcel.exe` de cette installation. Celles d'une autre copie, une copie portable par exemple, restent. Il ne touche jamais à vos fichiers téléchargés.

Si le service Rookparcel nox tourne depuis le dossier d'installation, le désinstalleur vous demande de le supprimer d'abord. Depuis une invite de commandes d'administrateur :

```
"%LOCALAPPDATA%\Programs\Rookparcel\rookparcel-nox.exe" service uninstall --all
```

(avec votre propre dossier d'installation). Une désinstallation silencieuse : `"%LOCALAPPDATA%\Programs\Rookparcel\uninstall.exe" /S`.

Les réglages appartiennent à chaque utilisateur Windows. Désinstaller une installation pour tous ne les nettoie que pour l'utilisateur qui lance le désinstalleur.

### Zip Windows

1. Si vous avez coché des associations ou « Lancer Rookparcel au démarrage du système », décochez-les d'abord dans **Options > Préférences > Général**.
2. Quittez Rookparcel et supprimez son dossier.
3. Supprimez aussi `%APPDATA%\Rookparcel`, sauf si la copie était portable (tout était alors dans son dossier).

### Linux et macOS

- **AppImage ou tar.gz :** supprimez le fichier ou le dossier. Pour supprimer les réglages et la liste des torrents, supprimez `~/.config/rookparcel` et `~/.local/share/rookparcel`.
- **Flatpak :** `flatpak uninstall --user io.github.balls0fsteel.Rookparcel`.
- **macOS :** faites glisser `Rookparcel.app` dans la Corbeille. Les réglages et la liste des torrents sont dans `~/Library/Application Support/Rookparcel`.

Les fichiers téléchargés restent où ils sont dans tous les cas. Pour la liste complète des dossiers, voyez [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel).
