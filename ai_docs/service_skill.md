# Skill service

## Objectif

Le premier skill cible permet de diagnostiquer et gérer des services Linux, probablement via `systemd`.
Il doit être conçu pour empêcher l'agent Rust de faire plus que les actions prévues.

## Cas d'usage initiaux

- Lister les services.
- Afficher l'état d'un service.
- Lister les services en échec.
- Expliquer pourquoi un service a échoué à partir d'informations disponibles.
- Démarrer un service autorisé.
- Arrêter un service autorisé.
- Redémarrer un service autorisé.

## Actions en lecture seule

Les actions suivantes devraient être disponibles avant les actions mutantes :

- `list_services`
- `get_service_status`
- `list_failed_services`
- `get_recent_service_logs`

Elles doivent tout de même être bornées : nombre de lignes de logs, unités visibles, formats de sortie et délais d'exécution.

## Actions mutantes

Les actions suivantes sont sensibles :

- `start_service`
- `stop_service`
- `restart_service`
- `reload_service`

Elles doivent nécessiter une politique explicite indiquant quels services sont autorisés.
Par défaut, aucun service ne doit être modifiable.

## Validation des entrées

Le nom d'un service doit être validé comme identifiant `systemd` attendu, et non interprété comme une commande.
Exemples de règles :

- accepter uniquement des noms d'unités normalisés ;
- refuser les caractères de shell, espaces non attendus et substitutions ;
- résoudre les alias de manière contrôlée ;
- comparer la cible finale à une allowlist.

## Enforcement spécifique

Avant une action mutante, le noyau doit demander une décision au framework d'enforcement avec au minimum :

- action demandée ;
- unité systemd cible ;
- plugin source ;
- utilisateur demandeur ;
- politique applicable ;
- nécessité éventuelle d'une confirmation.

## Exemples de politiques

- Autoriser la lecture de l'état de tous les services.
- Autoriser le redémarrage de `nginx.service` et `my-app.service` uniquement.
- Interdire l'arrêt de `ssh.service`.
- Exiger confirmation pour tout redémarrage.
- Interdire toute action mutante en mode diagnostic.

## Risques à traiter

- Arrêt accidentel d'un service critique.
- Contournement via un nom d'unité malformé.
- Escalade de privilèges via un helper trop permissif.
- Fuite d'informations sensibles dans les logs.
- Déni de service par répétition de redémarrages.
