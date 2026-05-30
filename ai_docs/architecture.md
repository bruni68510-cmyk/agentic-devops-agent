# Architecture envisagée

## Vue d'ensemble

L'agent est composé de quatre couches principales :

1. **Interface utilisateur** : CLI, API locale ou interface conversationnelle future.
2. **Noyau Rust** : orchestre les intentions, charge les plugins, valide les politiques et journalise les actions.
3. **Framework d'enforcement** : décide si une action est autorisée avant toute exécution.
4. **Plugins** : implémentent des capacités système précises, comme la gestion de services.

## Flux d'exécution

1. L'utilisateur exprime une demande.
2. Le noyau la mappe vers une capacité connue.
3. Le plugin prépare une action typée, sans l'exécuter directement si elle est sensible.
4. Le framework d'enforcement vérifie la politique : utilisateur, plugin, action, arguments, cible et contexte.
5. L'action est exécutée par un adaptateur système borné.
6. Le résultat et les décisions sont journalisés.

## Processus et privilèges

L'agent doit tourner comme un utilisateur Linux dédié, par exemple `agentic-devops`, sans privilèges root permanents.
Les opérations privilégiées doivent passer par un mécanisme limité : policy engine, helper séparé, `polkit`, capabilities Linux, règles `sudoers` très spécifiques, ou autre mécanisme équivalent.

## Adaptateurs système

Les plugins ne devraient pas appeler un shell générique.
Ils devraient utiliser des adaptateurs typés, par exemple :

- `SystemdAdapter.list_units()`
- `SystemdAdapter.get_unit_status(unit_name)`
- `SystemdAdapter.start_unit(unit_name)`
- `SystemdAdapter.stop_unit(unit_name)`

Chaque méthode doit avoir des paramètres validés et une politique associée.

## Questions ouvertes

- Quel framework d'enforcement choisir : moteur de politiques embarqué, OPA/Rego, Cedar, policy engine maison, `polkit`, ou combinaison de plusieurs mécanismes ?
- Les plugins doivent-ils être chargés dynamiquement, compilés avec l'agent, ou exécutés comme processus isolés ?
- Faut-il viser WebAssembly pour isoler les plugins dès la première version ?
