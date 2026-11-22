# Utiliser rookparcel-nox

*Cette page est une traduction de la page anglaise [Running rookparcel-nox](Running-rookparcel-nox). En cas de différence, la page anglaise fait foi.*

`rookparcel-nox` est Rookparcel sans fenêtre : le même moteur, lancé depuis un terminal ou comme service, et contrôlé par son [Interface Web](Interface-Web), depuis un Rookparcel de bureau avec le [Contrôle à distance](Contrôle-à-distance), ou par Sonarr, Radarr et Prowlarr ([Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr)). Cette page explique ses commandes et ses options, comment l'exécuter comme service Windows, sous systemd et sous FreeBSD, où il conserve ses fichiers, et comment l'arrêter.

## Où l'obtenir

`rookparcel-nox` est fourni avec chaque version ([Installer Rookparcel](Installer-Rookparcel)) :

| Plateforme | Fichier |
|---|---|
| Windows | `rookparcel-nox.exe`, à côté de `rookparcel.exe` dans le zip et dans le dossier installé |
| Linux | dans l'archive `rookparcel-<version>-linux-<arch>.tar.gz` (avec les unités systemd), ou seul sous la forme `rookparcel-nox-<version>-<arch>.AppImage` |
| macOS | `rookparcel-nox-<version>-macos-<arch>.tar.gz` |

`rookparcel-nox --help` liste toutes les commandes et options, `rookparcel-nox --version` affiche la version, et `rookparcel-nox --license` affiche la licence et les mentions des composants tiers.

## Un premier lancement

1. Ouvrez un terminal (sous Windows, une Invite de commandes) dans le dossier contenant `rookparcel-nox`.
2. Lancez-le avec un dossier de téléchargement et l'interface Web :

   ```sh
   rookparcel-nox run --save ~/Downloads --webui 8080
   ```

3. Il affiche quelques lignes commençant par `*`. L'une d'elles est l'adresse de l'interface Web et, la première fois, une ligne comme celle-ci :

   ```text
   * Web UI: no password set; temporary password for this run: ... (set one with --webui-pass)
   ```

4. Ouvrez `http://127.0.0.1:8080/` dans un navigateur et connectez-vous en tant que `admin` avec ce mot de passe temporaire.
5. Dans la page, ouvrez **Préférences > Interface web** et définissez votre propre mot de passe. À partir de là, il est conservé (sous forme de hachage), et nox le demande à chaque démarrage.

Pour l'arrêter, tapez `q` et appuyez sur <kbd>Enter</kbd>, ou appuyez sur <kbd>Ctrl</kbd>+<kbd>C</kbd>. Il enregistre l'état de chaque torrent en sortant, si bien que le prochain `run` les reprend sans revérifier les fichiers.

`--webui 8080` écoute sur cet ordinateur seulement (`127.0.0.1`). Pour atteindre la page depuis d'autres ordinateurs ou un téléphone, donnez aussi une adresse, par exemple `--webui 0.0.0.0:8080`, et lisez d'abord la [sécurité de l'interface Web](Interface-Web#sécurité).

## Commandes

| Commande | Ce qu'elle fait |
|---|---|
| `rookparcel-nox run [OPTIONS]` | Reprend chaque torrent enregistré dans le dossier d'état et continue de fonctionner jusqu'à son arrêt. C'est le démon. |
| `rookparcel-nox add <file.torrent>... [--save DIR] [OPTIONS]` | Ajoute les torrents (fichiers ou liens `magnet:`), les télécharge et se termine quand ils sont finis. Avec `--seed`, il continue plutôt de partager. |
| `rookparcel-nox create <file or folder> [-o OUT] [CREATE OPTIONS]` | Fabrique un fichier `.torrent` ([ci-dessous](#créer-un-fichier-torrent)). |
| `rookparcel-nox service ...` | Le service Windows (Windows seulement, [ci-dessous](#le-service-windows)). |
| `rookparcel-nox demo [--torrents N] [--active FRACTION] [--seconds N]` | Charge des torrents simulés et affiche des totaux ; uniquement pour les tests. |

`run` accepte aussi des fichiers `.torrent` et des liens magnet après les options ; ils sont ajoutés au démarrage.

## Options de run et add

### Dossiers et état

| Option | Signification |
|---|---|
| `--save DIR` | Le dossier de téléchargement. Par défaut : le dossier courant. |
| `--state-dir DIR` | Où sont conservés la liste des torrents et les données de reprise ([fichiers](#où-il-conserve-ses-fichiers)) ; c'est aussi le dossier des paramètres, sauf si `--config-dir` est donné. `--state DIR` est équivalent. |
| `--config-dir DIR` | Où sont conservés l'identifiant et les paramètres de l'interface Web (`webui.json`). |
| `--profile NAME` | Lance un nox séparé avec ses propres dossiers, son identifiant et son port d'écoute ([profils](#profils)). |
| `--no-state` | Ne charge ni n'enregistre rien. |
| `--move-on-complete DIR` | Déplace dans DIR chaque torrent qui finit de télécharger. |

### Partage et arrêt

| Option | Signification |
|---|---|
| `--seed` | `add` : continue de partager après le téléchargement au lieu de se terminer. |
| `--super-seed` | Partage initial pour les torrents complets ; implique `--seed`. |
| `--seconds N` | S'arrête après N secondes. |
| `--exit-when-done downloads` ou `all` | Se termine, état enregistré, quand plus rien ne télécharge (`downloads`), ou que plus rien ne télécharge ni ne partage (`all`). Comme **Options > Arrêt automatique** dans le programme de bureau ([Arrêt automatique](Arrêt-automatique)). |

### Réseau

| Option | Signification |
|---|---|
| `--port N` | Le port d'écoute pour les pairs. 6881 par défaut. |
| `--listen LIST` | Interfaces d'écoute à la place de `--port` : `IP:PORT`, `[IPv6]:PORT` ou `DEVICE:PORT`, séparés par des virgules. |
| `--dl-limit KIB`, `--ul-limit KIB` | Limites de réception et d'envoi en Kio/s ; 0 signifie illimité. |
| `--ip-filter FILE` | Bloque les plages d'adresses de FILE (`ipfilter.dat`, format P2P ou CIDR ; CIDR est nouveau dans la version 1.2.63). |
| `--proxy TYPE://[USER:PASS@]HOST:PORT` | Se connecte par un proxy ; TYPE est `socks4`, `socks5` ou `http`. Rien d'entrant n'est accepté tant que les pairs passent par lui. |
| `--no-proxy-peers` | Se connecte directement aux pairs et aux sources web ; seuls les trackers utilisent le proxy. |
| `--no-proxy-trackers` | Contacte directement les trackers. |
| `--no-proxy-hostnames` | Résout les noms d'hôte ici plutôt qu'au niveau du proxy. |
| `--socks5-udp-local-ep` | Pour un proxy SOCKS5 sans NAT entre les deux ; désactivé par défaut. |
| `--network RULE` | N'utilise que ce réseau : le trafic des torrents est lié à l'adaptateur correspondant. RULE vaut `vpn` (n'importe quel adaptateur VPN), `adapter:ID` (un nom de périphérique ; sous Windows, le GUID de l'adaptateur), `range:CIDR` (comme `10.2.0.0/16`), ou `off`. |
| `--no-pause-when-missing` | Tant que la règle `--network` ne correspond à rien, laisse les torrents fonctionner sans liaison au lieu d'attendre le réseau. |

*`--network` est nouveau dans la version 1.1.58.* Les options de proxy et de réseau sont conservées dans les paramètres de l'interface Web comme les cases correspondantes de sa page **Préférences > Connexion**. Dès que vous changez quelque chose dans l'interface Web, la valeur enregistrée l'emporte sur la ligne de commande : modifiez-les donc dans la page à partir de là. Un mot de passe de proxy sur la ligne de commande peut être vu par les autres utilisateurs de l'ordinateur dans la liste des processus ; définissez-le plutôt dans **Préférences > Connexion**. Plus de détails sur ces paramètres : [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu).

### Interface Web

| Option | Signification |
|---|---|
| `--webui [ADDR:]PORT` | Sert l'interface Web et l'API compatible qBittorrent. Sans ADDR, il n'écoute que sur `127.0.0.1`. |
| `--webui-user NAME` | Le nom de connexion. `admin` par défaut. |
| `--webui-pass PASS` | Le mot de passe, conservé sous forme de hachage. Sans mot de passe (et aucun d'enregistré), un mot de passe temporaire est affiché au démarrage. |
| `--webui-save-root DIR` | Peut être donné plusieurs fois. L'interface Web ne peut alors enregistrer, déplacer et lire que dans ces dossiers et en dessous. Sans cette option, quiconque a l'identifiant peut utiliser n'importe quel dossier que le compte de nox peut atteindre. |
| `--webui-https` | Sert en HTTPS avec un certificat autosigné fabriqué à la première utilisation. |
| `--webui-remote-run-program` | Permet à une session de l'interface Web depuis un autre ordinateur de modifier [Exécuter un programme](Référence-des-préférences#exécuter-un-programme), toujours uniquement avec le mot de passe actuel. Sans elle, ces lignes ne se modifient que depuis un navigateur du même ordinateur. Pour les serveurs sans navigateur local, comme un conteneur ; elle n'est jamais enregistrée et doit donc figurer sur la ligne de commande à chaque démarrage. Un proxy inverse sur le serveur lui-même doit ajouter `X-Forwarded-For`, sinon chaque visiteur compte comme un navigateur du même ordinateur ([Exécuter un programme](Référence-des-préférences#exécuter-un-programme)). |
| `--webui-cert FILE [--webui-key FILE]` | Sert en HTTPS avec votre propre certificat et votre propre clé (PEM ; clé non chiffrée). |

Un mot de passe sur la ligne de commande est visible des autres utilisateurs locaux dans la liste des processus. Préférez le définir dans **Préférences > Interface web** de la page. Détails sur HTTPS et les autres paramètres de sécurité : [Interface Web](Interface-Web#https).

### Autres

| Option | Signification |
|---|---|
| `--import qbittorrent` ou `utorrent` | `run` seulement : ajoute au démarrage les torrents de qBittorrent ou µTorrent, avec leurs données là où elles sont (vérifiées d'abord), statistiques, priorités, trackers, étiquettes et tags de qBittorrent. L'autre client ne doit pas fonctionner ; ses fichiers ne sont que lus. Voir [Importer depuis uTorrent et qBittorrent](Importer-depuis-uTorrent-et-qBittorrent). |
| `--import-dir DIR` | Le dossier de l'autre client quand il n'est pas à l'emplacement habituel. |
| `--verbose` (`-v`) | Affiche les événements des trackers et des pairs. |
| `--no-update-check` | `run` seulement : ne demande pas au site web du projet s'il existe des versions plus récentes. |
| `--daemon` | Unix : se détache en arrière-plan ([autres systèmes d'initialisation](#autres-façons-de-lexécuter-en-arrière-plan)). |
| `--pid-file FILE` | Écrit l'identifiant du processus dans FILE et le verrouille ; un second nox avec le même FILE refuse de démarrer. |
| `--log FILE` | Ajoute à FILE tout ce que nox affiche. |

### La vérification des mises à jour

`run` demande au site web du projet quelles versions existent, 10 secondes après le démarrage et toutes les 12 heures. Rien sur votre installation n'est envoyé et rien n'est téléchargé. Une version plus récente se traduit par une ligne dans la sortie et un lien dans la barre d'état de l'interface Web. Désactivez-la avec `--no-update-check`, ou dans l'interface Web sous **Préférences > Général > Confidentialité**, « Rechercher automatiquement les mises à jour ». « Mettre à jour vers les versions bêta » y propose aussi les préversions. Voir [Mettre à jour Rookparcel](Mettre-à-jour-Rookparcel).

## Créer un fichier torrent

```sh
rookparcel-nox create ~/Videos/holiday -t udp://tracker.example:1337/announce --magnet
```

| Option | Signification |
|---|---|
| `-o`, `--output FILE` | Où écrire le torrent. Par défaut : `<name>.torrent`. |
| `-t`, `--tracker URL` | Un tracker ; répétez pour plus de niveaux, ou mettez-en plusieurs dans un niveau avec des virgules. |
| `-w`, `--web-seed URL` | Une source web ; répétable. |
| `--node HOST:PORT` | Un nœud d'amorçage DHT ; répétable. |
| `--private` | Active l'indicateur privé (pas de DHT, d'échange de pairs ni de découverte locale). |
| `--piece-size KIB` | Taille des pièces en Kio. Par défaut : automatique. |
| `--comment TEXT`, `--source TEXT`, `--creator TEXT` | Le commentaire, le champ source (utilisé par les trackers privés) et « créé par ». |
| `--no-date` | Omet la date de création. |
| `--hybrid`, `--v1`, `--v2` | Hybride (par défaut), v1 seulement ou v2 seulement. |
| `--pad` | Aligne les fichiers sur les pièces avec des fichiers de remplissage (toujours actif pour hybride et v2). |
| `--mtime`, `--symlinks`, `--no-attributes`, `--skip-hidden` | Enregistrer les dates de modification ; enregistrer les liens comme des liens (Unix) ; omettre les attributs caché et exécutable ; omettre les noms commençant par un point. |
| `--magnet` | Affiche aussi le lien magnet. |

La fenêtre du programme de bureau pour cela est décrite dans [Créer des torrents](Créer-des-torrents).

## Où il conserve ses fichiers

| | Linux et BSD | Windows | macOS |
|---|---|---|---|
| Liste des torrents et données de reprise (`resume.db`, `metadata.pack`, `dht_state.dat`) | `~/.local/share/rookparcel/nox` | `%APPDATA%\Rookparcel\nox` | `~/Library/Application Support/Rookparcel/nox` |
| Paramètres (`webui.json`, `ipfilter.dat`, le certificat HTTPS) | `~/.config/rookparcel/nox` | le même dossier | le même dossier |
| Le service Windows | | `%ProgramData%\Rookparcel\nox` | |

- Sous Linux, `$XDG_DATA_HOME` et `$XDG_CONFIG_HOME` sont utilisés quand ils sont définis.
- `webui.json` est réécrit par le nox en cours d'exécution chaque fois que quelque chose change dans l'interface Web. Ne le modifiez que lorsque nox est arrêté.
- Un seul nox à la fois peut utiliser un dossier. Un second s'arrête avec « ... is in use by another rookparcel-nox ».
- **Mode portable :** avec un `rookparcel.ini` à côté du programme, les deux vont dans un dossier `nox` à côté de lui, et les dossiers de téléchargement situés dedans sont enregistrés de façon relative, si bien que tout le dossier peut être déplacé. *Nouveau dans la version 1.1.49.* Voir [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel).

## Profils

`--profile NAME` lance un nox séparé avec ses propres torrents, paramètres, identifiant d'interface Web et service. Un nom comporte de 1 à 32 caractères : lettres a-z, chiffres et traits d'union, en commençant par une lettre. Sauf si `--port` ou `--listen` dit autre chose, chaque profil reçoit son propre port d'écoute, calculé à partir de son nom et identique à chaque démarrage.

| | profil par défaut | `--profile work` |
|---|---|---|
| Linux et BSD | `~/.local/share/rookparcel/nox` | `~/.local/share/rookparcel/profiles/work/nox` (et pareil sous `~/.config`) |
| Windows | `%APPDATA%\Rookparcel\nox` | `%APPDATA%\Rookparcel\profiles\work\nox` |
| macOS | `~/Library/Application Support/Rookparcel/nox` | `.../Rookparcel/profiles/work/nox` |
| Service Windows | `RookparcelNox` | `RookparcelNox_work` |
| systemd | `rookparcel-nox.service` | `rookparcel-nox@work.service` |

L'interface Web de chaque profil a besoin de son propre port, par exemple `--webui 127.0.0.1:8081`. Un port déjà utilisé arrête nox avec « port N is in use, perhaps by another Rookparcel profile ». *Les profils sont nouveaux dans la version 1.1.44.* Voir [Profils](Profils).

## Le service Windows

`rookparcel-nox service install` enregistre nox comme un service Windows nommé **RookparcelNox** (« Rookparcel nox »). Il démarre avec Windows et fonctionne sans que personne soit connecté.

Ce n'est pas la même chose que **Préférences > Interface web** dans `rookparcel.exe` : celle-ci sert l'interface Web depuis la fenêtre, à partir des torrents de la fenêtre, uniquement tant que la fenêtre fonctionne ([Interface Web](Interface-Web)).

`install`, `uninstall`, `start` et `stop` nécessitent une Invite de commandes administrateur.

1. Placez `rookparcel-nox.exe` là où le compte du service peut le lire, comme `C:\Program Files\Rookparcel\`. Une copie dans votre dossier utilisateur (Téléchargements, Documents) ne peut pas être ouverte par le compte du service ; `install` vous en avertit.
2. Installez-le avec les options de `run` avec lesquelles il doit démarrer :

   ```bat
   "C:\Program Files\Rookparcel\rookparcel-nox.exe" service install --webui 127.0.0.1:8080
   ```

3. Démarrez-le avec `rookparcel-nox service start`, ou ajoutez `--now` à l'installation. Il démarre avec Windows à partir de là.
4. Lisez le mot de passe temporaire de l'interface Web dans `%ProgramData%\Rookparcel\nox\nox.log` (depuis une invite administrateur), ouvrez la page et définissez votre propre mot de passe. Ou donnez `--webui-user` et `--webui-pass` à `install` : le mot de passe est conservé sous forme de hachage dans le `webui.json` du service, jamais dans la ligne de commande du service.

Options de `install` lui-même :

| Option | Signification |
|---|---|
| `--account A` | `LocalService` (par défaut), `NetworkService` ou `LocalSystem`. |
| `--start TYPE` | `delayed` (par défaut : automatique, après la stabilisation du démarrage), `auto` ou `manual`. |
| `--replace` | Arrête et supprime d'abord un RookparcelNox installé ; son état est conservé. |
| `--now` | Démarre le service une fois installé. |
| `--profile NAME` | Installe le service propre au profil, `RookparcelNox_NAME`. |

Ce qu'il met en place :

- **État et paramètres** dans `%ProgramData%\Rookparcel\nox` (ou `--state-dir`), un dossier que seuls SYSTEM, les administrateurs et le compte du service peuvent ouvrir, puisqu'il contient l'identifiant de l'interface Web.
- **Téléchargements** dans son sous-dossier `Downloads`, sauf si vous donnez `--save`.
- **Journal** dans `nox.log` au même endroit, sauf si vous donnez `--log`.
- **Redémarrages** après un échec : au bout de 5, 30 et 60 secondes.
- À l'arrêt du système, il dispose de temps pour enregistrer l'état des torrents.

Les dossiers dans lesquels le service écrit et que `install` n'a pas créés (un dossier `--save`, un dossier `--move-on-complete`) nécessitent une autorisation pour le compte, depuis une invite administrateur :

```bat
icacls D:\Torrents /grant "NT AUTHORITY\LocalService:(OI)(CI)M"
```

Les connexions entrantes de pairs nécessitent une règle du Pare-feu Windows pour le programme ; un service ne peut pas en demander une :

```bat
netsh advfirewall firewall add rule name="Rookparcel nox" dir=in action=allow program="C:\Program Files\Rookparcel\rookparcel-nox.exe"
```

Les autres commandes du service :

| Commande | Ce qu'elle fait |
|---|---|
| `rookparcel-nox service start`, `service stop` | Le démarre ou l'arrête et attend que ce soit fait. `sc start RookparcelNox`, `sc stop RookparcelNox` et la console Services font la même chose. |
| `rookparcel-nox service status` | Affiche l'état et la ligne de commande du service (pas besoin d'être administrateur). `--all` liste le service de chaque profil. |
| `rookparcel-nox service uninstall` | L'arrête et le supprime. Le dossier d'état et le journal restent. `--all` supprime le service de chaque profil qui exécute ce programme. |
| `rookparcel-nox service debug [OPTIONS]` | Exécute le travail du service dans cette console, pour essayer les options avant l'installation. |
| `sc control RookparcelNox paramchange` | Recharge le filtre IP et rouvre le journal. |

Pour changer les options, installez de nouveau avec `--replace`. Pour supprimer aussi l'état des torrents et les téléchargements, supprimez `%ProgramData%\Rookparcel` après la désinstallation.

`start`, `stop`, `status` et `uninstall` acceptent `--profile NAME` pour le service d'un profil. *Les services par profil sont nouveaux dans la version 1.1.47.*

**Sans le service :** `rookparcel-nox run` dans une Invite de commandes fonctionne aussi et s'arrête proprement avec <kbd>Ctrl</kbd>+<kbd>C</kbd> ou à la fermeture de la fenêtre. Le Planificateur de tâches est une autre voie : une tâche « Au démarrage » qui exécute `rookparcel-nox.exe run --save D:\Torrents --webui 8080 --log C:\Rookparcel\nox.log`. `--daemon` ne fonctionne pas sous Windows.

## systemd (Linux)

L'archive Linux contient des unités prêtes à l'emploi, aussi dans le dossier [`contrib/systemd`](https://github.com/Balls0fSteel/Rookparcel/tree/main/contrib/systemd) du dépôt.

**Comme service système**, exécuté sous un compte `rookparcel`, avec son état dans `/var/lib/rookparcel-nox` :

```sh
sudo install -m 755 rookparcel-nox /usr/local/bin/
sudo install -D -m 644 rookparcel-nox.sysusers /etc/sysusers.d/rookparcel-nox.conf
sudo systemd-sysusers
sudo install -m 644 rookparcel-nox.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now rookparcel-nox
journalctl -u rookparcel-nox
```

Le journal montre le mot de passe temporaire de l'interface Web au premier démarrage. L'unité sert l'interface Web sur `127.0.0.1:8080` et télécharge dans `/var/lib/rookparcel-nox/downloads`. Pour changer cela, et pour l'autoriser à écrire dans un autre dossier :

```ini
# sudo systemctl edit rookparcel-nox
[Service]
ExecStart=
ExecStart=/usr/local/bin/rookparcel-nox run --save /srv/torrents --webui 0.0.0.0:8080
ReadWritePaths=/srv/torrents
```

Le service est verrouillé (il ne peut pas écrire en dehors de son propre dossier sans `ReadWritePaths=`) : un dossier de téléchargement ailleurs nécessite donc toujours cette ligne.

**Comme service utilisateur personnel**, avec vos fichiers dans les dossiers listés [plus haut](#où-il-conserve-ses-fichiers) :

```sh
install -m 755 rookparcel-nox ~/.local/bin/
install -D -m 644 rookparcel-nox.service ~/.config/systemd/user/rookparcel-nox.service
systemctl --user daemon-reload
systemctl --user enable --now rookparcel-nox
loginctl enable-linger "$USER"
```

`loginctl enable-linger` le garde actif après votre déconnexion. Cette unité télécharge dans `~/Downloads`.

**Les profils** utilisent les unités modèles `rookparcel-nox@.service` : `systemctl enable --now rookparcel-nox@work` démarre le profil `work`. Elles n'ont pas d'interface Web par défaut, car deux ne peuvent pas partager un port ; donnez-en une à chacun dans `systemctl edit rookparcel-nox@work`.

`systemctl reload rookparcel-nox` recharge le filtre IP. La sortie va dans le journal.

## FreeBSD

Le script rc.d se trouve dans le dossier [`contrib/freebsd`](https://github.com/Balls0fSteel/Rookparcel/tree/main/contrib/freebsd) du dépôt :

```sh
pw useradd rookparcel -d /var/db/rookparcel-nox -s /usr/sbin/nologin -c "Rookparcel daemon"
install -m 755 rookparcel-nox /usr/local/bin/
install -m 555 rookparcel_nox /usr/local/etc/rc.d/
sysrc rookparcel_nox_enable=YES
sysrc rookparcel_nox_args="--webui 0.0.0.0:8080"
service rookparcel_nox start
```

Il s'exécute sous `rookparcel`, avec son état dans `/var/db/rookparcel-nox` et le journal dans `/var/log/rookparcel-nox.log`, où le premier démarrage écrit le mot de passe temporaire de l'interface Web. Paramètres dans `/etc/rc.conf` : `rookparcel_nox_args` (d'autres options de `run` ; par défaut `--webui 127.0.0.1:8080`), `rookparcel_nox_user`, `rookparcel_nox_dir`, `rookparcel_nox_save`, `rookparcel_nox_log`. Utilisez `rookparcel_nox_args`, pas `rookparcel_nox_flags`. `service rookparcel_nox reload` recharge le filtre IP et rouvre le journal.

Profils : `sysrc rookparcel_nox_profiles="work"` et `sysrc rookparcel_nox_work_args="--webui 127.0.0.1:8081"` ; `service rookparcel_nox start work` en démarre un, `service rookparcel_nox start` les démarre tous.

## Autres façons de l'exécuter en arrière-plan

Sur d'autres systèmes Unix :

- `--daemon` se détache en arrière-plan et rend la main une fois que nox est lancé (code 0) ou qu'il a échoué au démarrage (code 1). Sans `--log`, la sortie après ce point est perdue : lisez donc le mot de passe temporaire avant, ou utilisez `--log`.
- `--pid-file FILE` écrit l'identifiant du processus ; un second nox avec le même fichier refuse de démarrer.
- `--log FILE` ajoute toute la sortie à FILE, lisible seulement par son propriétaire puisqu'il peut contenir le mot de passe temporaire. Un `SIGHUP` le rouvre, si bien que la rotation des journaux fonctionne.

## Arrêt et rechargement

| Quoi | Effet |
|---|---|
| `q` et <kbd>Enter</kbd>, <kbd>Ctrl</kbd>+<kbd>C</kbd>, `SIGTERM` | Un arrêt propre : état des torrents enregistré, redirections de ports du routeur supprimées. |
| Un second <kbd>Ctrl</kbd>+<kbd>C</kbd> ou `SIGTERM` pendant cet arrêt | Quitte immédiatement, sans enregistrer. |
| `SIGHUP` (service Windows : `sc control RookparcelNox paramchange`) | Recharge la liste du filtre IP et rouvre le fichier `--log`. `webui.json` n'est pas relu. Quand la liste vient d'une adresse, la copie téléchargée en dernier est rechargée ; rien n'est téléchargé. |
| Windows : fermeture de la fenêtre de console, fermeture de session, arrêt de l'ordinateur | Un arrêt propre. |

## Voir aussi

- [Interface Web](Interface-Web) : la page que sert nox.
- [Contrôle à distance](Contrôle-à-distance) : piloter nox depuis la fenêtre du programme de bureau.
- [Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr) : nox comme leur client de téléchargement.
- La description technique complète : [DAEMON.md](https://github.com/Balls0fSteel/Rookparcel/blob/main/docs/DAEMON.md).
