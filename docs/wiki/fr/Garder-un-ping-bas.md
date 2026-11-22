*Cette page est la traduction de la page anglaise [Keep ping low](Keep-ping-low). En cas de divergence, la page anglaise fait foi.*

# Garder un ping bas

Garder un ping bas est le « mode joueur » de Rookparcel : il surveille le délai de votre ligne internet et retient les vitesses de Rookparcel tant que ce délai augmente, afin que les jeux, les appels vidéo et la navigation web restent réactifs pendant que les torrents s'exécutent. Cette page explique ce qu'il fait, comment l'activer et ce que vous voyez pendant qu'il travaille.

## Pourquoi les torrents augmentent votre ping

L'envoi de votre ligne en est généralement la partie la plus étroite. Quand Rookparcel envoie plus vite que la ligne ne peut transporter, les paquets attendent dans une file dans le modem ou le routeur, et tous les autres paquets attendent derrière eux : ceux d'un jeu, d'un appel, d'une page web. Cette file est ce que vous remarquez comme du lag. Envoyer un peu moins vite que la ligne garde la file vide et le ping à sa valeur au repos. La direction de réception a le même problème, du côté de votre fournisseur d'accès.

Garder un ping bas n'a pas besoin de savoir quel jeu s'exécute. Il mesure la file lui-même, de sorte qu'il fonctionne pour n'importe quel jeu, appel ou autre programme.

## L'activer

1. Ouvrez **Options > Préférences** (<kbd>Ctrl</kbd>+<kbd>P</kbd>) et choisissez **Bande passante**.
2. Sous **Options de limitation globale de la vitesse**, cochez **Garder un ping bas (limiter les vitesses automatiquement)**.
3. Cliquez sur **OK** ou **Appliquer**.

![Préférences, page Bande passante](images/prefs-bandwidth.png)

Il est désactivé par défaut. Pour le désactiver de nouveau, décochez la case : vos propres limites reviennent immédiatement.

Quand vous l'activez pour la première fois, Rookparcel apprend le ping de votre ligne au repos. Cela prend jusqu'à environ 20 secondes, pendant lesquelles les vitesses peuvent démarrer plus bas.

## Ce qu'il fait

- **Il mesure le délai.** Rookparcel envoie un ping au premier routeur situé après votre ligne domestique environ quatre fois par seconde (quelques centaines d'octets par seconde). Si aucun routeur situé après la ligne ne répond, il envoie plutôt des pings aux hôtes du paramètre Avancé `auto_limit_probe_hosts`, par défaut `1.1.1.1,8.8.8.8,9.9.9.9`. Les pairs uTP occupés fournissent une seconde mesure sans trafic supplémentaire.
- **Il plafonne les deux directions.** Tant que le délai reste supérieur à sa valeur au repos de plus que la marge admise, le plafond d'envoi ou de réception est abaissé ; quand la ligne est de nouveau calme, le plafond remonte pas à pas.
- **Il n'augmente jamais vos propres limites.** C'est la plus basse des deux valeurs, le plafond automatique et votre propre limite globale, qui est utilisée. Vos réglages ne sont pas modifiés.
- **Il a des planchers.** Le plafond ne descend pas sous environ 10 ko/s, ni sous une petite part de la vitesse la plus élevée observée, de sorte qu'un Wi-Fi chargé ou l'envoi d'un autre programme ne puisse pas priver complètement Rookparcel de débit.
- **uTP cède plus tôt.** Tant qu'il est activé, les connexions uTP visent au plus 25 ms de délai de mise en file au lieu des 100 ms habituelles.

La note sous la case le dit aussi : ces pings vont au premier routeur situé après votre ligne, ou sinon aux hôtes de repli, dont les exploitants les voient.

## Ce que vous voyez

- **La barre d'état.** Tant qu'une direction est plafonnée en dessous de votre propre limite, sa partie montre le plafond entre parenthèses, par exemple **E : 412.0 kB/s (auto 450.0 kB/s) T : 2.1 GB**. Sans plafond, elle s'affiche comme d'habitude.
- **L'onglet Journal.** Les lignes qui commencent par « Keep ping low: » (en anglais) indiquent quel routeur ou quels hôtes reçoivent des pings, et quand une direction est limitée ou ne l'est plus.
- **L'interface web** montre le même « (auto ...) » dans sa barre d'état.

Des vitesses inférieures au maximum de votre ligne pendant qu'il est activé sont normales : cette marge est ce qui garde le ping bas.

## Réglage fin

Deux paramètres de **Options > Préférences > Avancé** s'y rattachent. Sélectionnez la ligne, saisissez la valeur, cliquez sur **Définir**, puis sur **OK** ou **Appliquer**.

| Paramètre | Signification | Par défaut |
|---|---|---|
| `auto_limit_delay_ms` | Millisecondes de ping supplémentaire admises au-dessus du ping de la ligne au repos avant que les plafonds soient abaissés. Plus bas, les jeux et les appels sont plus fluides ; plus haut, la vitesse permise est plus grande. | 15 |
| `auto_limit_probe_hosts` | Adresses IP, séparées par des virgules, auxquelles envoyer des pings quand aucun routeur situé après la ligne ne répond. Lu quand la fonction est activée. Il est rarement nécessaire de le changer. | `1.1.1.1,8.8.8.8,9.9.9.9` |

`utp_target_delay` (100 par défaut) est la cible de délai uTP que la fonction abaisse à 25 tant qu'elle est activée.

## Plateformes

| Système | Comment il mesure |
|---|---|
| Windows | Des pings, sans droits d'administrateur, plus les pairs uTP. |
| Linux | Des pings par les sockets ping sans privilège du système, plus les pairs uTP. Si le système ne les autorise pas, le Journal indique « this system does not let Rookparcel ping (its group must be in net.ipv4.ping_group_range) » et seuls les pairs uTP sont utilisés. |
| macOS | Des pings, plus les pairs uTP. |
| FreeBSD et autres systèmes sans pings sans privilège | Les pairs uTP seulement. Le Journal indique que les pings ne sont pas disponibles. |

Avec les pairs uTP seuls, il lui faut au moins trois pairs uTP occupés pour mesurer quoi que ce soit ; un partage uniquement vers des pairs TCP ne lui donne rien sur quoi s'appuyer.

## Quand autre chose vaut mieux

Un routeur avec SQM (comme CAKE) ou une QoS de jeu qui fonctionne bien fait cela mieux, pour tous les appareils de la maison, pas seulement pour Rookparcel. Si vous en avez un, vous pouvez laisser Garder un ping bas désactivé.

## Fonctions apparentées

- Le ralentissement dû à un disque occupé, dans **Avancé > E/S disque**, utilise le même genre de plafond automatique, pour un disque lent au lieu d'une ligne lente ; le plus bas des deux plafonds l'emporte. Voir [Disque et stockage](Disque-et-stockage#quand-le-disque-est-occupé).
- La page du routeur de l'Assistant de configuration mentionne Garder un ping bas pour les jeux et les appels : voir [Premier démarrage](Premier-démarrage).
- Limites fixes et planificateur hebdomadaire : [Limites de vitesse et planificateur](Limites-de-vitesse-et-planificateur).

## Dans la fenêtre GTK et l'interface web

La fenêtre GTK (Linux, BSD) et l'application macOS ont la même case dans **Préférences > Bande passante** (sur le Mac, elle s'applique dès que vous la cochez), et l'interface web l'a sur sa page **Préférences > Bande passante**, avec la même note. Dans [rookparcel-nox](Utiliser-rookparcel-nox), c'est le même paramètre, `auto_rate_limit`, réglé depuis les Préférences de l'interface web.
