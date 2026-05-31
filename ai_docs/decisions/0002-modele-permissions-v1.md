# Décision 0002 — Modèle de permissions déclaratif en V1

## Statut

Acceptée.

## Contexte

L'agent doit empêcher un utilisateur ou un plugin de dépasser le périmètre prévu.
Le plugin ne doit pas être l'autorité finale qui décide si une action sensible est permise.

## Décision

La V1 utilise un **policy engine simple et déclaratif**, maintenu dans la configuration locale du projet.
Le format privilégié pour commencer est **TOML**, car il est lisible, courant dans l'écosystème Rust et suffisamment strict pour une première version.

Toute action doit être évaluée comme une demande structurée contenant au minimum :

- le plugin demandé ;
- la capacité demandée ;
- l'action exacte ;
- la ressource cible ;
- les arguments normalisés ;
- le mode d'exécution ;
- l'identité locale disponible.

## Règle par défaut

Toute action non déclarée est refusée.
Toute action mutante est refusée tant qu'une politique explicite ne l'autorise pas.

## Exemple indicatif

```toml
[service.read]
allow = ["*"]

[service.restart]
allow = ["nginx.service", "my-app.service"]
require_confirmation = true

[service.stop]
allow = ["my-app.service"]
deny = ["ssh.service"]
```

## Conséquences

- La V1 peut être testée sans intégrer immédiatement OPA, Cedar ou un autre moteur externe.
- Le modèle reste migrable vers un moteur plus avancé si les règles deviennent complexes.
- Les tests d'abus doivent vérifier les actions inconnues, ressources non autorisées et arguments malformés.

## Questions repoussées

- Choix final entre OPA/Rego, Cedar ou policy engine maison durable.
- Modèle de délégation multi-utilisateurs.
- Synchronisation de policies entre machines.
