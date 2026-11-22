# Rookparcel avec Docker

*Cette page est une traduction de la page anglaise [Docker](Docker). En cas de différence, la page anglaise fait foi.*

`rookparcel-nox`, le démon sans fenêtre, existe aussi sous forme d'image de conteneur pour Docker et d'autres moteurs de conteneurs. Cette page explique comment le lancer avec `docker run` ou Docker Compose, ses dossiers et ses ports, le mot de passe de l'interface Web, l'exécution sous un autre utilisateur, l'utilisation avec Sonarr, Radarr et Prowlarr dans des conteneurs, ainsi que l'arrêt et la mise à jour.

*Nouveau dans la version 1.2.21.*

L'image est `ghcr.io/balls0fsteel/rookparcel-nox`, pour Linux x86-64 (`amd64`) et ARM 64 bits (`arm64`). Chaque version a sa propre étiquette, comme `:1.2.21`, et `:latest` est la version la plus récente (pas une préversion). À l'intérieur, nox fonctionne comme sur n'importe quel ordinateur Linux : tout ce que disent [Utiliser rookparcel-nox](Utiliser-rookparcel-nox) et [Interface Web](Interface-Web) s'applique.

## Un premier démarrage

```sh
docker run -d --name rookparcel --restart unless-stopped --stop-timeout 60 \
  -p 8080:8080 -p 6881:6881 -p 6881:6881/udp \
  -v rookparcel-config:/config -v rookparcel-downloads:/downloads \
  ghcr.io/balls0fsteel/rookparcel-nox
```

1. Lisez le mot de passe temporaire dans le journal :

   ```sh
   docker logs rookparcel
   ```

   ```text
   * Web UI: no password set; temporary password for this run: ... (set one with --webui-pass)
   ```

2. Ouvrez `http://127.0.0.1:8080/` (ou `http://<l'adresse de l'ordinateur>:8080/` depuis un autre ordinateur) et connectez-vous en tant que `admin` avec lui.
3. Dans la page, ouvrez **Préférences > Interface web** et définissez votre propre **Mot de passe :**, avec le mot de passe temporaire comme **Mot de passe actuel :**.

Le mot de passe temporaire n'est affiché qu'au premier démarrage, et il reste valable jusqu'à ce que vous définissiez le vôtre. Si le journal a disparu avant que vous l'ayez lu (le conteneur a été supprimé), définissez un mot de passe avec `ROOKPARCEL_WEBUI_PASS` ([plus bas](#le-mot-de-passe-de-linterface-web)).

Sans commande, l'image exécute :

```sh
rookparcel-nox run --state-dir /config --save /downloads --webui 0.0.0.0:8080 --port 6881
```

Les options placées après le nom de l'image sont ajoutées à cette commande, et une option donnée de nouveau l'emporte sur celle par défaut : `... ghcr.io/balls0fsteel/rookparcel-nox --save /srv/media/downloads --verbose`. Une commande placée après le nom de l'image la remplace en revanche : `... ghcr.io/balls0fsteel/rookparcel-nox run --state-dir /config --save /srv/media/downloads --webui 0.0.0.0:8080 --port 6881`. Chaque option est listée dans [Utiliser rookparcel-nox](Utiliser-rookparcel-nox#options-de-run-et-add).

## Dossiers

| Dossier | Ce qu'il contient |
|---|---|
| `/config` | Tout ce que nox conserve : la liste des torrents et la progression de chacun, l'état DHT, l'identifiant et les préférences de l'interface Web (`webui.json`), les étiquettes, les tags et le Téléchargeur RSS. Gardez-le sur un volume, sinon un redémarrage commence avec une liste vide. |
| `/downloads` | Le dossier de téléchargement par défaut. |

Les deux sont des volumes. Pour Sonarr, Radarr et Prowlarr, montez plutôt votre dossier de médias au même chemin que celui qu'utilisent les applications, et enregistrez dedans ([plus bas](#sonarr-radarr-et-prowlarr-dans-des-conteneurs)).

Pour sauvegarder, arrêtez le conteneur et copiez le volume `/config`. Pour repartir de zéro, supprimez le conteneur et le volume.

## Ports

| Port | Pour |
|---|---|
| `8080/tcp` | L'interface Web et l'API compatible qBittorrent. |
| `6881/tcp`, `6881/udp` | Le port d'écoute pour les pairs (l'UDP porte uTP et le DHT). |

Associez le port d'écoute au même numéro à l'extérieur et à l'intérieur (`-p 6881:6881`) : nox indique aux trackers et aux pairs le port sur lequel il écoute, si bien que `-p 6882:6881` leur donnerait le mauvais. Pour utiliser un autre port, changez les deux, par exemple `-p 51413:51413 -p 51413:51413/udp` et `--port 51413`. Un port modifié plus tard dans **Préférences > Connexion** de l'interface Web nécessite le même changement dans l'association.

**UPnP et NAT-PMP ne fonctionnent pas depuis le réseau habituel (pont) de Docker :** le routeur n'est pas joignable depuis l'intérieur, donc le port n'est pas redirigé pour vous. Redirigez-le à la main dans le routeur, en TCP et UDP, vers l'ordinateur qui exécute Docker ([Rediriger le port dans le routeur](Connexion-ports-et-pare-feu#rediriger-le-port-dans-le-routeur)), ou, sous Linux, exécutez le conteneur sur le réseau de l'hôte (`--network host`), où nox voit le routeur et le réseau local comme le fait un programme hors conteneur, et `-p` n'est pas nécessaire. La découverte locale de pairs ne trouve non plus de pairs sur votre réseau qu'avec le réseau de l'hôte.

## Le mot de passe de l'interface Web

| Variable | Signification |
|---|---|
| `ROOKPARCEL_WEBUI_PASS` | Définit le mot de passe de l'interface Web à chaque démarrage. Il est conservé haché dans `/config`, si bien qu'il reste après que vous avez supprimé la variable. |
| `ROOKPARCEL_WEBUI_USER` | Le nom de connexion qui va avec (`admin` par défaut). Utilisé uniquement avec `ROOKPARCEL_WEBUI_PASS`. |

```sh
docker run -d --name rookparcel ... -e ROOKPARCEL_WEBUI_PASS='a long password' ghcr.io/balls0fsteel/rookparcel-nox
```

Sans `ROOKPARCEL_WEBUI_PASS`, nox garde le mot de passe enregistré, ou en affiche un temporaire quand aucun n'est encore enregistré.

Deux choses à savoir :

- Tant que la variable est définie, elle l'emporte à chaque démarrage : un mot de passe que vous avez changé dans la page revient à celui de la variable au démarrage suivant. Supprimez la variable pour garder celui défini dans la page.
- `docker inspect` affiche la variable, et nox reçoit le mot de passe sur sa ligne de commande, que `docker top` et la liste des processus de l'hôte montrent. Sur un ordinateur auquel d'autres peuvent se connecter, utilisez plutôt le mot de passe temporaire et définissez le vôtre dans la page, ou supprimez la variable après le premier démarrage.

L'interface Web est en HTTP simple. Sur un réseau auquel vous ne faites pas confiance, servez-la en HTTPS (`--webui-https`) ou placez-la derrière un proxy inverse qui le fait : voir [HTTPS](Interface-Web#https) et [Sécurité de l'interface Web](Interface-Web#sécurité).

## Utilisateur et autorisations

nox s'exécute sous l'utilisateur `rookparcel`, d'identifiant utilisateur et groupe 1000, jamais en root. Les nouveaux fichiers sont lisibles par tous et modifiables par leur groupe (umask `0002`), afin que les applications d'autres conteneurs puissent les importer et les supprimer.

| Pour | Faites |
|---|---|
| Écrire dans un dossier de l'hôte qui appartient à un autre utilisateur | Démarrez le conteneur sous cet utilisateur : `--user 1001:1001` (`user: "1001:1001"` dans Compose). Trouvez les identifiants avec `id -u` et `id -g`. |
| Utiliser un autre umask | `-e UMASK=0022` (fichiers lisibles par tous, modifiables seulement par leur propriétaire). |

Avec un autre utilisateur, `/config` doit lui être accessible en écriture aussi. Un nouveau volume nommé appartient à l'utilisateur 1000 : utilisez donc plutôt un dossier de l'hôte que vous donnez à cet utilisateur :

```sh
sudo mkdir -p /srv/rookparcel && sudo chown 1001:1001 /srv/rookparcel
docker run -d --name rookparcel --user 1001:1001 -v /srv/rookparcel:/config ...
```

## Docker Compose

Le dépôt contient un fichier [compose.yaml](https://github.com/Balls0fSteel/Rookparcel/blob/main/contrib/docker/compose.yaml) prêt à l'emploi. Il garde `/config` dans un volume, monte `/srv/media` au même chemin, enregistre dans `/srv/media/downloads` et laisse 60 secondes à nox pour s'arrêter :

```yaml
services:
  rookparcel:
    image: ghcr.io/balls0fsteel/rookparcel-nox:latest
    container_name: rookparcel
    restart: unless-stopped
    user: "1000:1000"
    environment:
      ROOKPARCEL_WEBUI_PASS: ${ROOKPARCEL_WEBUI_PASS:-}
    command: [run, --state-dir, /config, --save, /srv/media/downloads, --webui, "0.0.0.0:8080", --port, "6881"]
    volumes:
      - rookparcel-config:/config
      - /srv/media:/srv/media
    ports:
      - "8080:8080"
      - "6881:6881/tcp"
      - "6881:6881/udp"
    stop_grace_period: 60s

volumes:
  rookparcel-config:
```

1. Mettez le mot de passe dans un fichier `.env` à côté de `compose.yaml` : `ROOKPARCEL_WEBUI_PASS=a long password`. Sans lui, lisez le mot de passe temporaire avec `docker compose logs rookparcel`.
2. Remplacez `/srv/media` et `user:` par votre dossier de médias et son propriétaire.
3. Démarrez-le avec `docker compose up -d`.

## Sonarr, Radarr et Prowlarr dans des conteneurs

Configurez Rookparcel dans les applications comme un client **qBittorrent**, comme le décrit [Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr). Dans des conteneurs, trois choses diffèrent :

1. **Le même chemin dans chaque conteneur.** Montez le dossier de médias au même chemin dans le conteneur de nox et dans ceux des applications (`/srv/media` dans chacun, comme dans le fichier Compose ci-dessus), et enregistrez les téléchargements dedans. Les applications trouvent alors les fichiers là où nox les indique, et les importations peuvent utiliser des liens physiques.
2. **Le nom d'hôte.** Sur le même réseau Compose ou défini par l'utilisateur, les applications joignent nox sous la forme `http://rookparcel:8080` (le nom du service ou du conteneur). nox ne répond qu'aux adresses IP et à `localhost` tant que vous n'avez pas autorisé le nom : ajoutez `rookparcel` à **Domaines du serveur (séparez plusieurs entrées par un point-virgule) :** dans **Préférences > Interface web** de l'interface Web, sous **Connectivité**. Jusque-là, le **Test** des applications échoue avec « Bad Request: unexpected Host header ». Voir [Noms d'hôte et Docker](Sonarr-Radarr-et-Prowlarr#noms-dhôte-et-docker).
3. **Le dossier de téléchargement que demandent les applications** doit être un chemin à l'intérieur du conteneur de nox, comme `/srv/media/downloads`.

`--webui-save-root /srv/media` (ajouté à la commande `run`) limite les dossiers dans lesquels l'interface Web et les applications peuvent enregistrer.

## Journaux, arrêt et rechargement

nox écrit son journal dans la sortie du conteneur : `docker logs rookparcel` (`-f` pour suivre). `--verbose` ajoute les événements des trackers et des pairs.

nox enregistre l'état de chaque torrent quand il s'arrête. `docker stop` envoie `SIGTERM` et attend 10 secondes par défaut avant de tuer le conteneur, ce qui peut être trop court pour une longue liste : laissez-lui donc une minute :

```sh
docker stop -t 60 rookparcel
```

`--stop-timeout 60` avec `docker run`, ou `stop_grace_period: 60s` dans Compose, en fait la valeur par défaut du conteneur. Un arrêt propre se termine par ces lignes dans le journal, et le conteneur se termine avec le code 0 :

```text
* SIGTERM received (again to quit at once, without saving)
* shutting down
* stopped
```

`docker kill -s HUP rookparcel` recharge le filtre IP, comme le fait `SIGHUP` hors conteneur ([Arrêt et rechargement](Utiliser-rookparcel-nox#arrêt-et-rechargement)).

## Mise à jour

L'image ne vérifie pas les mises à jour, et son interface Web n'affiche aucun lien de mise à jour : un conteneur se met à jour en récupérant la nouvelle image et en démarrant un nouveau conteneur sur le même `/config`.

```sh
docker pull ghcr.io/balls0fsteel/rookparcel-nox
docker stop -t 60 rookparcel && docker rm rookparcel
docker run -d --name rookparcel ...    # the same command as before
```

Avec Compose : `docker compose pull`, puis `docker compose up -d`.

Pour la même raison, l'[assistant de recherche](Rechercher-des-torrents#lassistant-de-recherche) n'est pas proposé dans l'image. Votre propre Prowlarr ou Jackett peut toujours être ajouté comme source de recherche ([Rechercher des torrents](Rechercher-des-torrents#ajouter-votre-propre-prowlarr-ou-jackett)).

## Construire l'image soi-même

À partir d'une copie du dépôt :

```sh
docker build -t rookparcel-nox .
```

La construction compile nox dans un conteneur Rust et le copie dans une petite image Debian. Elle prend quelques minutes sur tous les cœurs de l'ordinateur ; `--build-arg CARGO_BUILD_JOBS=1` n'en utilise qu'un. Utilisez ensuite `rookparcel-nox` à la place de `ghcr.io/balls0fsteel/rookparcel-nox` dans les commandes ci-dessus.

## Voir aussi

- [Utiliser rookparcel-nox](Utiliser-rookparcel-nox) : chaque commande et chaque option.
- [Interface Web](Interface-Web) : la page, HTTPS et la sécurité.
- [Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr) : les paramètres des applications, les catégories et Remove Completed.
- [Connexion, ports et pare-feu](Connexion-ports-et-pare-feu) : vérifier que les pairs peuvent se connecter.
- Les fichiers de l'image : [Dockerfile](https://github.com/Balls0fSteel/Rookparcel/blob/main/Dockerfile) et [contrib/docker](https://github.com/Balls0fSteel/Rookparcel/tree/main/contrib/docker).
