*Cette page est la traduction de la page anglaise [Locking Rookparcel](https://github.com/Balls0fSteel/Rookparcel/wiki/Locking-Rookparcel). En cas de différence, la page anglaise fait foi.*

# Verrouiller Rookparcel

Cette page explique comment verrouiller les fenêtres de Rookparcel avec un code PIN, pour qu'une autre personne devant votre ordinateur ne voie pas votre liste de torrents, et comment supprimer un code PIN oublié. *Nouveauté de la 1.2.54.*

Le verrouillage sert la confidentialité, pas la sécurité. Il masque les fenêtres ; il ne chiffre rien. Quiconque peut modifier le fichier de paramètres de Rookparcel, ou ouvrir votre dossier de téléchargement, peut le contourner. N'utilisez pas le code PIN de votre carte bancaire.

## Définir un code PIN et verrouiller

1. Ouvrez **Options > Préférences** et choisissez **Avancé > Verrouillage de l'interface** (sur un Mac : **Rookparcel > Préférences**, puis **Verrouillage de l'interface**).
2. Cliquez sur **Définir le code PIN...**.
3. Saisissez un code PIN de 4 à 6 chiffres dans **Nouveau code PIN (4 à 6 chiffres)**, et de nouveau dans **Répétez le nouveau code PIN**.
4. Cliquez sur **OK**. Le code PIN est enregistré aussitôt ; vous n'avez pas besoin de cliquer sur **OK** dans Préférences.

*Avant la 1.2.63, le code PIN se définissait dans **Options > Lock PIN**.*

Pour verrouiller, choisissez **Fichier > Verrouiller Rookparcel** ou appuyez sur <kbd>Ctrl</kbd>+<kbd>L</kbd>. Dès qu'un code PIN est défini, le menu de la zone de notification propose aussi **Verrouiller Rookparcel**. Si aucun code PIN n'est encore défini, **Verrouiller Rookparcel** en demande d'abord un, puis verrouille.

Le verrouillage masque toutes les fenêtres de Rookparcel dans la zone de notification : la fenêtre principale et toute boîte de dialogue qui était ouverte. Vos torrents continuent à se télécharger et à être partagés.

Pour changer le code PIN, cliquez sur **Modifier le code PIN...** sur la même page, saisissez le code PIN actuel, puis le nouveau deux fois. **Supprimer le code PIN...** demande le code PIN actuel et le supprime ; **Verrouiller Rookparcel** demande ensuite un nouveau code PIN avant de verrouiller.

## Désactiver le verrouillage

Si vous ne voulez jamais du verrouillage, décochez **Autoriser le verrouillage de Rookparcel par un code PIN** sur la page **Verrouillage de l'interface**. Si un code PIN est défini, Rookparcel le demande d'abord, puis le supprime. Tant que le verrouillage est désactivé, **Fichier > Verrouiller Rookparcel**, son raccourci et l'entrée **Verrouiller Rookparcel** du menu de la zone de notification disparaissent. Cochez de nouveau la case pour les faire revenir.

## Ouvrir un Rookparcel verrouillé

Cliquez sur l'icône de la zone de notification, choisissez **Afficher Rookparcel** dans son menu, ou relancez Rookparcel. Une fenêtre intitulée **Rookparcel est verrouillé** demande le code PIN. Avec le bon code PIN, les fenêtres qui étaient ouvertes reviennent telles quelles. **Annuler** laisse Rookparcel verrouillé dans la zone de notification.

Au cinquième code PIN incorrect de suite, Rookparcel attend 30 secondes avant d'en accepter un autre, et il fait de même après chaque code incorrect suivant. Le bon code PIN met fin à cette attente.

Rookparcel reste verrouillé quand vous le quittez. Au démarrage suivant, il demande aussitôt le code PIN, sans afficher sa fenêtre principale. Lancé à l'ouverture de session avec **Démarrer réduit**, il attend dans la zone de notification sans rien demander.

## Pendant que Rookparcel est verrouillé

- Le menu de la zone de notification ne propose que **Afficher Rookparcel** et **Quitter**. **Quitter** fonctionne sans le code PIN.
- La notification « Téléchargement terminé » n'apparaît pas, car elle nomme le torrent.
- Les messages qui s'ouvriraient d'eux-mêmes attendent que vous déverrouilliez : messages d'erreur, proposition de mise à jour, et fenêtre **Ajouter un nouveau torrent** d'un lien magnet dont les fichiers sont arrivés.
- Un fichier `.torrent` ou un lien magnet que vous ouvrez pendant le verrouillage demande d'abord le code PIN. Si vous cliquez sur **Annuler**, il n'est pas ajouté.
- Le compte à rebours d'**Arrêt automatique** s'affiche toujours, pour que vous puissiez l'arrêter.
- L'[interface web](Interface-Web) n'est pas verrouillée. Elle a son propre nom d'utilisateur et son propre mot de passe. Si **Pas d'authentification pour les clients sur localhost** est activé, un navigateur du même ordinateur peut toujours voir vos torrents.

## Code PIN oublié

Vous pouvez supprimer un code PIN oublié au démarrage de Rookparcel. Rookparcel demande **Supprimer le code PIN de verrouillage ? Rookparcel s'ouvrira sans le demander.** Répondez **Oui** (**OK** sous Linux, **Supprimer** sous macOS) et il s'ouvre sans code PIN. Vos torrents et vos autres paramètres restent tels quels.

**Windows :** maintenez <kbd>Ctrl</kbd> enfoncée pendant que vous lancez Rookparcel, par exemple en cliquant sur son élément du menu Démarrer ou en double-cliquant sur son icône du bureau. Si Rookparcel fonctionne déjà, c'est la copie en cours qui pose la question.

**macOS :** quittez Rookparcel (**Quitter Rookparcel** dans le menu de l'icône de la barre des menus), puis maintenez <kbd>⌥</kbd> (Option) enfoncée pendant que vous l'ouvrez. Tant que Rookparcel fonctionne, maintenir <kbd>⌥</kbd> en cliquant sur son icône du Dock pose aussi la question.

**Linux et les BSD :** lancez-le depuis un terminal avec `--reset-lock`. Si Rookparcel fonctionne déjà, c'est la copie en cours qui pose la question.

| Copie | Commande |
|---|---|
| Binaires simples (tar.gz) | `rookparcel-gtk --reset-lock` |
| AppImage | `./Rookparcel-<version>-x86_64.AppImage --reset-lock` |
| Flatpak | `flatpak run io.github.balls0fsteel.Rookparcel --reset-lock` |

`--reset-lock` fonctionne aussi sous Windows et macOS : `rookparcel.exe --reset-lock`, ou sur un Mac `/Applications/Rookparcel.app/Contents/MacOS/Rookparcel --reset-lock`. Si Rookparcel fonctionne déjà sur le Mac, sa question peut s'ouvrir derrière la fenêtre au premier plan : cherchez-la.

Avec un [profil](Profils), ajoutez son option : `rookparcel.exe --profile NOM --reset-lock`.

**À la main :** quittez Rookparcel, ouvrez son `rookparcel.ini` dans un éditeur de texte, supprimez les lignes qui commencent par `ui.lock_pin=` et `ui.locked=`, puis enregistrez. [Emplacement des fichiers de Rookparcel](Emplacement-des-fichiers-de-Rookparcel) indique où se trouve le fichier.

## Profils et fenêtres distantes

Chaque [profil](Profils) a son propre code PIN, ou aucun. Il en va de même pour chaque fenêtre connectée à un Rookparcel distant ([Contrôle à distance](Contrôle-à-distance)), puisqu'elle garde ses propres paramètres.

## Version GTK (Linux, BSD)

Le verrouillage fonctionne comme sous Windows. <kbd>Ctrl</kbd>+<kbd>L</kbd> verrouille depuis n'importe quelle fenêtre de Rookparcel, pas seulement la fenêtre principale : Préférences, Propriétés, Créer un nouveau torrent, Ajouter un nouveau torrent, le Téléchargeur RSS, les fenêtres de recherche et les autres boîtes de dialogue (pas les fenêtres du code PIN). *Nouveauté de la 1.2.76 ; auparavant, il ne faisait rien tant qu'une de ces boîtes de dialogue était ouverte.* Si la zone de notification disparaît pendant que Rookparcel est verrouillé (le tableau de bord redémarre, par exemple), la fenêtre **Rookparcel est verrouillé** s'affiche, pour que les fenêtres ne restent pas masquées sans moyen de les retrouver. *Nouveauté de la 1.2.76.*

Sur les bureaux sans zone de notification (GNOME simple sans l'extension AppIndicator), il n'y a nulle part où masquer les fenêtres : la fenêtre **Rookparcel est verrouillé** prend alors leur place. Elle n'a pas de bouton **Annuler**, et la fermer quitte Rookparcel, comme le fait la fermeture de la fenêtre principale à cet endroit.

## macOS

**Fichier > Verrouiller Rookparcel** s'obtient par <kbd>⌘</kbd>+<kbd>L</kbd>. Tant que Rookparcel est verrouillé :

- la barre des menus n'a que **Afficher Rookparcel**, **Masquer Rookparcel** et **Quitter Rookparcel** dans le menu **Rookparcel**, et le menu **Édition** ;
- le menu de l'icône de la barre des menus propose **Afficher Rookparcel** et **Quitter Rookparcel**, et celui de l'icône du Dock **Afficher Rookparcel** ;
- tant que la fenêtre du code PIN est ouverte, **Quitter Rookparcel** est grisé : cliquez d'abord sur **Annuler**.
