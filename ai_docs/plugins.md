# Modèle de plugins

## Rôle d'un plugin

Un plugin apporte un skill borné à l'agent.
Il ne fournit pas un accès libre au système : il expose une liste de capacités typées, documentées et soumises au framework d'enforcement.

## Contrat minimal

Chaque plugin doit déclarer :

- son nom ;
- sa version ;
- ses capacités ;
- ses actions en lecture seule ;
- ses actions mutantes ;
- les ressources ciblées ;
- les permissions nécessaires ;
- les risques connus ;
- les événements d'audit produits.

## Exemple de capacités

Pour un plugin `service` :

- `service.list` : lister les services visibles.
- `service.status` : obtenir l'état d'un service.
- `service.failed` : lister les services en échec.
- `service.start` : démarrer un service autorisé.
- `service.stop` : arrêter un service autorisé.
- `service.restart` : redémarrer un service autorisé.

## Interfaces sûres

Les plugins doivent produire des commandes internes typées ou appeler des adaptateurs système typés.
Ils ne doivent pas construire des chaînes shell libres à partir de l'entrée utilisateur.

## Isolation

Trois approches sont envisageables :

1. **Plugins compilés avec le binaire** : simple, mais moins flexible.
2. **Plugins processus séparés** : meilleure isolation, contrat IPC nécessaire.
3. **Plugins WebAssembly** : bonne sandbox potentielle, mais complexité initiale plus élevée.

## Décision initiale proposée

Commencer avec des plugins compilés ou des modules Rust internes pour valider le modèle de capacités, puis isoler progressivement les plugins lorsque les contrats seront stables.
