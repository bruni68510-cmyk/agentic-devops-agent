# Décision 0004 — Skill service en lecture seule pour la V1

## Statut

Acceptée.

## Contexte

Le skill `service` est le premier cas d'usage concret.
Il doit permettre de diagnostiquer les services Linux sans donner à l'agent plus de pouvoir que prévu.
Les actions `start`, `stop`, `restart` et `reload` sont utiles, mais sensibles.

## Décision

La V1 du skill `service` est **lecture seule**.
Elle cible Linux avec `systemd` et expose uniquement des capacités de diagnostic bornées.

Capacités initiales :

- `service.list` : lister les services visibles ;
- `service.status` : afficher l'état d'un service ;
- `service.failed` : lister les services en échec ;
- `service.logs_recent` : lire un extrait borné des logs récents d'un service.

## Restrictions

- Aucun `start`, `stop`, `restart` ou `reload` en V1.
- Les noms d'unités doivent être validés comme identifiants `systemd` attendus.
- Le nombre de lignes de logs doit être borné.
- Les délais d'exécution doivent être bornés.
- Les résultats doivent être structurés autant que possible.

## Actions mutantes futures

Les actions mutantes seront ajoutées seulement après validation du modèle d'enforcement.
Elles devront respecter au minimum :

- allowlist explicite de services ;
- refus par défaut ;
- confirmation possible ;
- audit systématique ;
- mécanisme d'élévation minimal et documenté si nécessaire.

## Conséquences

- La première implémentation peut valider le flux complet sans risque de modification système.
- Les tests de sécurité se concentrent d'abord sur la validation d'entrées, le refus d'actions inconnues et l'absence de shell arbitraire.
- Les choix `polkit`, `sudoers`, helper root ou D-Bus privilégié sont repoussés à la phase actions mutantes.
