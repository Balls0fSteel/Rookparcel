*Ceci est la traduction française de la page [Updating Rookparcel](Updating-Rookparcel) du wiki anglais. La page anglaise fait foi.*

# Mettre à jour Rookparcel

Rookparcel recherche lui-même les nouvelles versions et vous prévient quand l'une d'elles est sortie. Cette page décrit la recherche, comment une copie installée sous Windows se met à jour toute seule, la mise à jour à la main des copies zip, Linux et macOS, les versions bêta, la désactivation de la recherche, et `rookparcel-nox`.

## La recherche de mises à jour

Environ dix secondes après son démarrage, puis toutes les 12 heures tant qu'il tourne, Rookparcel demande au site du projet quelle est la version la plus récente. C'est une petite requête HTTPS pour une courte liste de numéros de version. Rien sur votre ordinateur, vos torrents ou vos réglages n'est envoyé, et rien n'est téléchargé ni installé sans votre accord. Si Rookparcel utilise un proxy (**Options > Préférences > Connexion**), la recherche passe aussi par lui.

Quand une version plus récente est sortie :

- Une boîte de question apparaît, une fois pour chaque nouvelle version.
- Ensuite, un avis reste dans la barre d'état jusqu'à ce que vous cliquiez dessus : « Rookparcel 1.2.76 est disponible - cliquez ici pour le télécharger ».
- L'onglet Journal contient aussi une ligne avec le lien.

Pour rechercher tout de suite, choisissez **Aide > Rechercher des mises à jour** (sous macOS **Rookparcel > Rechercher des mises à jour**). Elle répond dans les deux cas, avec la version plus récente ou avec « Vous avez la version la plus récente ».

## Une copie installée sous Windows se met à jour toute seule

Un Rookparcel installé par l'installeur Windows peut installer lui-même la nouvelle version. *Nouveau dans la 1.1.40.*

1. La question indique « Rookparcel 1.2.76 est disponible. Vous avez la version 1.2.69. Le télécharger et l'installer maintenant ? ». Cliquez sur **Télécharger et installer**. (**Ouvrir la page de téléchargement** ouvre à la place la page de la version dans votre navigateur ; **Annuler** ne fait rien.)
2. Une petite fenêtre montre la progression du téléchargement. Rookparcel vérifie l'installeur téléchargé avec le `SHA256SUMS` de la version avant de l'utiliser.
3. Rookparcel demande : « Pour l'installer, Rookparcel se ferme maintenant et redémarre une fois l'installation terminée. Fermer et installer maintenant ? ». Cliquez sur **Oui**.
4. Rookparcel enregistre tout et se ferme. L'installeur installe la nouvelle version en silence dans le même dossier et relance Rookparcel.

Vos réglages, vos torrents et vos associations restent tels qu'ils étaient. Si l'installation échoue, l'ancienne version reste et redémarre.

Pour une installation pour tous (dans `C:\Program Files`), Windows demande les droits d'administrateur pendant l'installation.

**Mise à jour depuis la 1.1.17 ou plus ancienne :** ces versions ne peuvent pas installer de mises à jour ; leur question ouvre la page de téléchargement. Téléchargez le nouvel installeur et lancez-le une fois à la main, comme dans [Installer Rookparcel](Installer-Rookparcel#installer-par-dessus-une-ancienne-version). Ensuite, Rookparcel se met à jour lui-même.

## Mettre à jour à la main

### Installeur Windows

Téléchargez le nouveau `rookparcel-<version>-windows-x64-setup.exe` et lancez-le. Il trouve le Rookparcel installé et s'installe par-dessus, en gardant le dossier, le mode (pour vous ou pour tous), les réglages et les torrents. Quittez d'abord Rookparcel, sinon l'installeur vous le demande. Voir [Installer Rookparcel](Installer-Rookparcel#installer-par-dessus-une-ancienne-version).

### Zip Windows

Une copie du zip ne se met pas à jour toute seule. Sa question propose d'ouvrir la page de téléchargement.

1. Téléchargez le nouveau zip.
2. Quittez Rookparcel (**Fichier > Quitter**).
3. Décompressez les nouveaux `rookparcel.exe` et `rookparcel-nox.exe` par-dessus les anciens.
4. Lancez Rookparcel.

Gardez `rookparcel.ini` dans le dossier si la copie est portable. Les réglages et les torrents ne sont pas dans le zip, donc rien de ce qui est à vous n'est écrasé.

### Linux et macOS

Dans la version GTK et l'application macOS, la question ouvre la page de la version.

- **AppImage :** téléchargez le nouveau `.AppImage`, rendez-le exécutable (`chmod +x`) et lancez-le à la place de l'ancien. Supprimez l'ancien fichier. Les réglages et les torrents ne sont pas dans l'AppImage, donc ils sont conservés. Pour une AppImage portable, placez le nouveau fichier à côté de `rookparcel.ini`.
- **tar.gz :** quittez Rookparcel et décompressez les nouveaux binaires par-dessus les anciens.
- **Flatpak :** le Flatpak n'a pas de recherche de mises à jour. Téléchargez le paquet plus récent depuis la [page des versions](https://github.com/Balls0fSteel/Rookparcel/releases) et installez-le comme dans [Installer Rookparcel](Installer-Rookparcel#flatpak).
- **macOS :** quittez Rookparcel (**Rookparcel > Quitter Rookparcel**), puis faites glisser le nouveau `Rookparcel.app` de `Rookparcel-<version>-macos.dmg` dans Applications et remplacez l'ancien. Si Gatekeeper refuse la nouvelle copie, ouvrez-la comme dans [Installer Rookparcel](Installer-Rookparcel#macos). Le même `Rookparcel.app` fonctionne sur les Mac Apple silicon et Intel.
- **macOS, depuis la version GTK :** les versions précédentes avaient un `Rookparcel.app` GTK pour Apple silicon. Remplacez-le de la même façon ; à son premier démarrage, la nouvelle application reprend les réglages et les torrents de l'ancienne. Voir [Installer Rookparcel](Installer-Rookparcel#passer-de-la-version-gtk).

## Versions bêta

Rookparcel publie parfois une préversion (une version candidate) avant une version. Pour qu'elles vous soient proposées aussi, cochez **Mettre à jour vers les versions bêta** dans **Options > Préférences > Général**, sous **Confidentialité**. Elle est décochée par défaut.

Une préversion est toujours informée de la préversion suivante, quel que soit l'état de la case.

## Désactiver la recherche

Décochez **Rechercher automatiquement les mises à jour** dans **Options > Préférences > Général**, sous **Confidentialité**. Rookparcel ne questionne alors plus jamais le site de lui-même. **Aide > Rechercher des mises à jour** fonctionne toujours quand vous le choisissez.

Certaines versions n'ont aucune recherche de mises à jour : le Flatpak, et les paquets conçus pour un gestionnaire de paquets qui se charge des mises à jour.

## rookparcel-nox

`rookparcel-nox` recherche de la même façon, dix secondes après le démarrage de `run` et toutes les 12 heures. *Nouveau dans la 1.1.17.* Il ne télécharge ni n'installe jamais rien. Quand une version plus récente est sortie :

- il affiche une ligne, par exemple `* Rookparcel 1.2.0 is available: https://github.com/Balls0fSteel/Rookparcel/releases/tag/v1.2.0` ;
- son interface web montre la ligne dans son Journal et un lien dans la barre d'état de la page.

Pour la désactiver, lancez-le avec `--no-update-check`, ou décochez **Rechercher automatiquement les mises à jour** dans **Général > Confidentialité** de l'interface web. **Mettre à jour vers les versions bêta** s'y trouve aussi.

Pour mettre le démon à jour, arrêtez-le, remplacez `rookparcel-nox` par le nouveau et relancez-le. Pour le service Windows, arrêtez-le d'abord depuis une invite de commandes d'administrateur (`sc stop RookparcelNox`) et relancez-le ensuite ; l'installeur Windows le demande quand il trouve le service en cours d'exécution. Voir [Utiliser rookparcel-nox](Utiliser-rookparcel-nox).

## Après la mise à jour

- **Contrôle à distance :** quand une version change le protocole distant, une fenêtre et le `rookparcel-nox` qu'elle pilote doivent tous deux être dans cette version. Mettez les deux à jour. Les notes de version indiquent quand cela s'applique ([Contrôle à distance](Contrôle-à-distance)).
- **Revenir à une version plus ancienne :** les réglages et les torrents sont conservés entre les versions 1.1.x. La 1.1.40 stocke les données de reprise sous une forme plus petite, donc revenir de celle-ci à une version plus ancienne revérifie chaque torrent une fois. Un [dossier portable](Emplacement-des-fichiers-de-Rookparcel#mode-portable) utilisé par la 1.1.49 ou plus récente demande la 1.1.49 ou plus récente.
- Ce qui a changé dans chaque version : les notes de version sur la [page des versions](https://github.com/Balls0fSteel/Rookparcel/releases), ou la page [Actualités](https://balls0fsteel.github.io/Rookparcel/news.html) du site.

## Quand une version lit vos réglages autrement

*Nouveau dans la 1.2.57.*

À chaque enregistrement de ses réglages, Rookparcel note aussi sa version et la façon dont il a lu les réglages qui décident par où passe votre trafic : le proxy, **Utiliser uniquement ce réseau**, les interfaces d'écoute, le chiffrement et le filtre IP. Quand une autre version démarre, elle compare cette note avec sa propre lecture du même fichier :

- Elle garde d'abord une copie du fichier tel qu'elle l'a trouvé, à côté de lui : `rookparcel.ini.<ancienne version>.bak` (`webui.json.<ancienne version>.bak` pour `rookparcel-nox`). Les trois dernières sont conservées.
- Chaque réglage lu différemment est écrit dans l'onglet **Journal**, par exemple `proxy: socks5 10.8.0.1:1080 peers trackers names → none`.
- Si le proxy, la règle de réseau ou les interfaces d'écoute étaient perdus, rien ne se connecte : la barre d'état affiche **Transferts suspendus : confirmez les préférences**, et les torrents attendent. Vérifiez **Options > Préférences > Connexion**, puis cliquez sur **OK**, et les transferts démarrent. Dans l'interface web, enregistrez sa page Préférences. Les flux RSS, les liens `.torrent` et la recherche de mises à jour attendent eux aussi. Pour revenir en arrière à la place, quittez Rookparcel et copiez le fichier `.bak` par-dessus le fichier de réglages.

Les transferts sont suspendus de la même façon quand le fichier de réglages ne peut pas être lu du tout (il n'est alors jamais écrasé), quand il manque ou est vide alors qu'il y a des torrents enregistrés, ou quand cette version ne peut pas lire le type de proxy qu'une version plus récente a écrit ; cette ligne reste dans le fichier pour la version plus récente. Un fichier de réglages qui n'est pas du texte valide (enregistré par un éditeur dans un autre encodage) est lu du mieux possible, et l'original est conservé sous le nom `rookparcel.ini.damaged-<nombre>`. `rookparcel-nox` ne démarre pas avec un `webui.json` qu'il ne peut pas lire ; il nomme le fichier, pour que vous puissiez le corriger ou l'écarter.
