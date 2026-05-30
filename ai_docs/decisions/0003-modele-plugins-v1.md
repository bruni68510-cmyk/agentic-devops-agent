# Décision 0003 — Plugins Rust internes en V1

## Statut

Acceptée.

## Contexte

Les plugins doivent apporter des skills bornés sans transformer l'agent en shell généraliste.
Le chargement dynamique, les ABI stables, la signature de plugins et l'isolation forte ajoutent beaucoup de complexité au démarrage.

## Décision

La V1 implémente les plugins comme **modules Rust internes compilés avec le binaire**.

Chaque plugin doit exposer un contrat typé :

- nom ;
- version ;
- capacités ;
- actions en lecture seule ;
- actions mutantes ;
- ressources ciblées ;
- exigences de permissions ;
- événements d'audit.

## Interdictions

- Pas de chargement de bibliothèques dynamiques `.so` en V1.
- Pas de plugins installables à chaud.
- Pas d'interface plugin basée sur une commande shell libre.
- Pas d'exécution arbitraire de chaînes fournies par l'utilisateur.

## Conséquences

- Le contrat plugin peut évoluer rapidement pendant le prototype.
- Les tests restent simples et intégrés au workspace Rust.
- L'isolation forte est repoussée à une phase ultérieure, une fois les contrats stabilisés.

## Évolution prévue

Après la V1, les options à évaluer sont :

1. plugins processus séparés avec IPC typée ;
2. plugins WebAssembly ;
3. sandbox système avec seccomp, AppArmor ou SELinux ;
4. combinaison d'un protocole plugin stable et d'un mécanisme de signature.
