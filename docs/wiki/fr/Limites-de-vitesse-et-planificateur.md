*Cette page est la traduction de la page anglaise [Speed limits and scheduler](Speed-limits-and-scheduler). En cas de divergence, la page anglaise fait foi.*

# Limites de vitesse et planificateur

Cette page explique comment plafonner les vitesses de réception et d'envoi de Rookparcel : pour tous les torrents ensemble, pour chaque torrent, et heure par heure avec le planificateur hebdomadaire. Pour des limites qui suivent d'elles-mêmes le ping de votre ligne, voir [Garder un ping bas](Garder-un-ping-bas).

Toutes les vitesses sont en ko/s, où 1 ko vaut 1024 octets. Dans chaque zone de vitesse, 0 signifie illimité.

## Limites globales

Les limites globales plafonnent tous les torrents ensemble.

1. Ouvrez **Options > Préférences** (<kbd>Ctrl</kbd>+<kbd>P</kbd>) et choisissez **Bande passante**.
2. Sous **Limitation globale de la vitesse d'envoi**, réglez **Vitesse d'envoi maximale (ko/s) : [0 : illimitée]**.
3. Sous **Limitation globale de la vitesse de réception**, réglez **Vitesse de réception maximale (ko/s) : [0 : illimitée]**.
4. Cliquez sur **OK** ou **Appliquer**. Les nouvelles limites s'appliquent immédiatement.

![Préférences, page Bande passante](images/prefs-bandwidth.png)

Les deux sont illimitées par défaut. Limiter l'envoi est la raison habituelle d'en régler une : quand Rookparcel envoie aussi vite que la ligne le permet, les pages web, les jeux et les appels ralentissent pour tout le monde dans la maison.

### Options de limitation globale de la vitesse

La même page a deux cases qui changent ce que les limites comptent :

| Option | Ce qu'elle fait | Par défaut |
|---|---|---|
| **Appliquer la limitation de vitesse au surdébit de transport** | Compte les en-têtes TCP/IP du trafic des pairs, estimés par paquet, dans les limites globales, de sorte que les limites correspondent à ce que la ligne transporte. | Désactivée |
| **Appliquer la limitation de vitesse aux connexions uTP** | Applique aussi les limites globales aux pairs uTP. Désactivée, les pairs uTP ne sont ralentis que par le contrôle de délai propre à uTP, qui cède la place aux autres trafics. | Activée |

### Connexions et emplacements d'envoi

Le groupe **Nombre de connexions** de la même page n'est pas une limite de vitesse, mais il influe sur la vitesse :

| Paramètre | Signification | Par défaut |
|---|---|---|
| **Nombre maximum global de connexions :** | Le plus grand nombre de connexions de pairs ouvertes en même temps pour tous les torrents. Réduisez-le si le routeur peine. | 200 |
| **Nombre maximum de pairs connectés par torrent : [0 : pas de limite]** | Le plus grand nombre de pairs auxquels un torrent se connecte. | 0 |
| **Nombre d'emplacements d'envoi par torrent : [0 : pas de limite]** | À combien de pairs un torrent envoie en même temps. | 0 |
| **Nombre global d'emplacements d'envoi : [-1 : pas de limite]** | À combien de pairs tous les torrents ensemble envoient en même temps, les plus rapides étant choisis. Moins d'emplacements donnent plus de vitesse à chaque pair. | 8 |

Les deux valeurs par torrent s'appliquent aux torrents ajoutés à partir de maintenant ; les Propriétés d'un torrent peuvent changer ses propres emplacements d'envoi. La question sur le routeur de l'Assistant de configuration peut réduire les nombres de connexions pour un routeur ancien : voir [Premier démarrage](Premier-démarrage).

## Limites pour un torrent

Un torrent peut avoir ses propres limites, inférieures aux limites globales. Les limites globales s'appliquent toujours en plus.

**Depuis le menu contextuel :**

1. Sélectionnez un ou plusieurs torrents dans la liste.
2. Faites un clic droit et choisissez **Allocation de bande passante > Définir la limite de réception** ou **Allocation de bande passante > Définir la limite d'envoi**.
3. Choisissez une vitesse : **Illimité**, ou l'une des valeurs 10, 25, 50, 100, 250, 500, 1000, 2500 et 5000 ko/s.

La vitesse actuelle du premier torrent sélectionné est cochée.

![Le menu contextuel de la liste des torrents](images/torrent-menu.png)

**Depuis les Propriétés du torrent**, pour n'importe quelle vitesse :

1. Sélectionnez les torrents et choisissez **Propriétés** (<kbd>Enter</kbd>).
2. Sous **Paramètres de bande passante**, réglez **Vitesse d'envoi maximale (ko/s) : [0 : illimitée]** et **Vitesse de réception maximale (ko/s) : [0 : illimitée]**.
3. Pour donner au torrent son propre nombre d'emplacements d'envoi, cochez **Remplacer le nombre d'emplacements d'envoi par défaut** et remplissez **Nombre d'emplacements d'envoi : [0 : illimité]**.
4. Cliquez sur **OK**.

![La boîte de dialogue Propriétés du torrent](images/properties.png)

Les limites propres à un torrent apparaissent dans les colonnes **Limite réception** et **Limite envoi** de la liste des torrents et dans l'onglet **Général**. Voir [Détails et fichiers des torrents](Détails-et-fichiers-des-torrents) pour le reste de la boîte de dialogue Propriétés.

Un torrent qui atteint son objectif de partage peut aussi recevoir tout seul une limite d'envoi : voir [Mise en file et partage](Mise-en-file-et-partage#objectif-de-partage).

## La barre d'état

La barre d'état en bas de la fenêtre principale montre les vitesses actuelles et les totaux de cette session, sous la forme **R : 1.2 MB/s T : 3.4 GB** pour la réception et **E : ... T : ...** pour l'envoi. Tant que Garder un ping bas ou le ralentissement dû à un disque occupé retient une direction, sa partie montre aussi le plafond automatique, sous la forme **E : 412.0 kB/s (auto 450.0 kB/s) T : ...**.

La barre d'état n'a pas de menu de vitesse. Réglez les limites globales dans les Préférences, ou les limites propres aux torrents depuis leur menu.

Avec **Afficher la vitesse actuelle dans la barre de titre** activé (**Options > Préférences > Interface**), le titre de la fenêtre et le bouton de la barre des tâches commencent eux aussi par les vitesses.

## Le planificateur

Le planificateur suit un tableau hebdomadaire, heure par heure, et fait passer Rookparcel entre quatre modes. Il est désactivé par défaut.

1. Ouvrez **Options > Préférences > Planificateur**.
2. Cochez **Activer le planificateur**. Le tableau est grisé tant que vous ne l'avez pas fait.
3. Cliquez sur une cellule du **Tableau du planificateur** pour faire passer cette heure au mode suivant. Maintenez le bouton enfoncé et faites glisser pour peindre les cellules survolées avec le même mode.
4. Sous **Paramètres du planificateur**, réglez les vitesses des heures **Limité**.
5. Cliquez sur **OK** ou **Appliquer**.

![Préférences, page Planificateur](images/prefs-scheduler.png)

Le tableau a une ligne par jour, le lundi en premier, et une colonne par heure, selon l'heure locale de votre ordinateur. Un clic fait défiler **Pleine vitesse**, **Limité**, **Arrêt** et **Partage seul**. La ligne sous le tableau nomme l'heure sous le pointeur et son mode, comme « Lun 09:00 - 10:00 : Pleine vitesse ».

| Mode | Couleur | Ce que fait Rookparcel pendant cette heure |
|---|---|---|
| **Pleine vitesse** | vert foncé | Fonctionne avec vos réglages normaux. |
| **Limité** | vert clair | Utilise les deux vitesses Limité ci-dessous à la place des limites globales. |
| **Arrêt** | blanc | Met tous les torrents en pause. Avec **Désactiver le DHT à l'arrêt**, le DHT est aussi désactivé, de sorte que Rookparcel reste silencieux sur le réseau. |
| **Partage seul** | rouge clair | Envoie, mais ne télécharge rien. Les torrents ajoutés pendant ces heures attendent aussi. |

| Paramètre du planificateur | Signification | Par défaut |
|---|---|---|
| **Vitesse d'envoi limitée (ko/s) : [0 : illimitée]** | La limite d'envoi globale pendant les heures Limité. | 0 |
| **Vitesse de réception limitée (ko/s) : [0 : illimitée]** | La limite de réception globale pendant les heures Limité. | 0 |
| **Désactiver le DHT à l'arrêt** | Désactive aussi le DHT pendant les heures Arrêt. | Activé |

À quoi s'attendre :

- Quand l'heure change de mode, la partie réseau de la barre d'état ajoute **Planificateur : limité**, **Planificateur : arrêté** ou **Planificateur : partage seul**, et l'onglet **Journal** écrit une ligne comme « Planificateur : Limité ». Pleine vitesse n'affiche rien.
- Arrêt ne change pas l'état propre des torrents. Quand Pleine vitesse revient, ils reprennent comme avant, et un torrent que vous aviez arrêté reste arrêté.
- Décochez **Activer le planificateur** pour revenir immédiatement à la pleine vitesse avec vos réglages normaux.
- Garder un ping bas et le ralentissement dû à un disque occupé peuvent encore abaisser les vitesses pendant n'importe quelle heure ; ils ne les augmentent jamais.

## Limites de vitesse alternatives (interface web de rookparcel-nox)

Les fenêtres Windows et GTK n'ont pas de limites de vitesse alternatives ; utilisez à la place les heures Limité du planificateur.

L'interface web de [rookparcel-nox](Utiliser-rookparcel-nox) en a, sous **Préférences > Bande passante > Limites alternatives** :

| Paramètre | Signification | Par défaut |
|---|---|---|
| **Vitesse d'envoi alternative (ko/s) : [0 : illimitée]** | La limite d'envoi globale tant que les limites alternatives sont activées. | 10 |
| **Vitesse de réception alternative (ko/s) : [0 : illimitée]** | La limite de réception globale tant qu'elles sont activées. | 10 |
| **Utiliser maintenant les limites alternatives** | Passe aux limites alternatives jusqu'à ce que vous reveniez en arrière. | Désactivé |

Les applications qui changent le mode des limites de vitesse de qBittorrent par l'interface web (voir [Sonarr, Radarr et Prowlarr](Sonarr-Radarr-et-Prowlarr)) changent la même chose. Les heures Limité du planificateur l'emportent toujours sur les deux jeux tant qu'elles durent.

Quand une fenêtre de bureau sert l'interface web (Préférences > Interface web dans la fenêtre), ce groupe n'est pas affiché et les limites alternatives ne peuvent pas être activées : les limites et le planificateur propres à la fenêtre s'appliquent.

## Dans la fenêtre GTK et l'interface web

- **Fenêtre GTK** (Linux, BSD) : les mêmes pages de Préférences, le même sous-menu **Allocation de bande passante** dans le menu des torrents, le même tableau du planificateur, modifié de la même façon.
- **Application macOS** : les mêmes pages, le même sous-menu **Allocation de bande passante** et un tableau du planificateur que vous peignez en faisant glisser. Les Préférences s'appliquent immédiatement, sans **OK** : voir [Référence des préférences](Référence-des-préférences#différences-sous-macos).
- **Interface web** : les mêmes pages sous ses **Préférences**, et le même sous-menu **Allocation de bande passante**. Sa barre d'état montre une limite globale entre parenthèses après la vitesse tant qu'une est réglée, et le plafond automatique de la même façon que la fenêtre.
- **rookparcel-nox** sans l'interface web : `--dl-limit KIB` et `--ul-limit KIB` règlent les limites globales au démarrage (0 = illimité). Le planificateur, l'objectif de partage et les limites alternatives se règlent dans les **Préférences** de l'interface web.

## Pages liées

- [Garder un ping bas](Garder-un-ping-bas)
- [Mise en file et partage](Mise-en-file-et-partage)
- [Disque et stockage](Disque-et-stockage#quand-le-disque-est-occupé)
- [Référence des préférences](Référence-des-préférences)
