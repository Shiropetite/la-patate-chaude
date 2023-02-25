# La Patate Chaude

## Groupe

AL1 Groupe 8 : Sofia DA SILVA RIBEIRO et Léo SALLARD

## Organisation et Démarche d'élaboration

Pour ce projet, nous avons commencé par établir la communication entre le client et le serveur en permettant tout d'abord au client
d'envoyer le message `Hello` et de recevoir le message `Welcome` du serveur.

Sofia a mis ensuite en place le reste du début du processus de communication en allant jusqu'à `Start Game`.
À ce moment là, notre code était encore réuni dans un seul fichier global.

Ensuite, Léo à travailler sur l'implémentation du challenge **HashCash MD5**. Nous sommes partis sur une approche itérative ou le module testent toutes les clés possibles de 0 à MAX. Le code du challenge à été la première partie du code séparée du reste. Le trait `Challenge` n'a été implémenté que plus tard par Léo toujours.

Pendant ce temps Sofia a mis en place tous les `enums` ainsi que les `structs` nécessaire pour représenter correctement le flux de communication jusqu'à la réception d'un challenge côté client.

Nous avons ensuite décidé de l'architecture finale du projet, le but était de fragmenter au maximum les informations et la responsabilité de chaque fichier, tout en mutualisant le plus possible le code commun au client et au serveur.

Enfin, Léo a appliqué le traitement `rustfmt` et `rustdoc` sur toute la workspace, ainsi que rédigé le fichier que vous êtes en train de lire.

*Notes : Nous n'avions au départ pas compris qu'il fallait fork le git du projet, d'où le premier commit "massif", nous avions travaillé durant la majeure partie du semestre sur un repository privé.*

### Spécificités et Bonus

Le seul bonus que nous pourrions considéré avoir tenté est celui de réduire au maximum l'utilisation des `unwrap()`, des `expect()`, des `panic!()` et des `mut`.

Nous souhaitons au départ tenter d'implémenter nous-mêmes la bibliothèque `md5` mais avons dû y renoncer par manque de temps.

En dehors de cela le projet n'a pas de spécificité qui sort du cadre du sujet de base.
