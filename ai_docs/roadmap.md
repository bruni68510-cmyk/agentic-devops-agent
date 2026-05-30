# Roadmap

## Phase 1 — Documentation et cadrage

- Décrire la vision dans `AGENTS.md`.
- Structurer les documents dans `ai_docs/`.
- Identifier les premiers skills et leurs limites.
- Définir les principes de sécurité non négociables.
- Formaliser les décisions V1 dans `ai_docs/decisions/`.

## Phase 2 — Prototype noyau Rust

- Appliquer les décisions V1 documentées dans `ai_docs/decisions/`.
=======
- Créer une CLI minimale.
- Définir les traits Rust pour les plugins.
- Définir un format de capacité typé.
- Ajouter une journalisation structurée.
- Implémenter un mode lecture seule pour le skill service.

## Phase 3 — Enforcement

- Prototyper une politique déclarative.
- Séparer décision d'autorisation et exécution.
- Refuser toute action non déclarée.
- Ajouter des tests d'abus : noms malformés, action inconnue, service non autorisé.

## Phase 4 — Actions mutantes contrôlées

- Ajouter `start`, `stop` et `restart` pour une allowlist de services.
- Tester un mécanisme d'élévation minimale.
- Journaliser chaque décision et chaque action.
- Ajouter confirmations ou dry-run selon le niveau de risque.

## Phase 5 — Isolation des plugins

- Évaluer processus séparés, WebAssembly, seccomp, AppArmor et SELinux.
- Choisir une stratégie d'isolation adaptée à l'exploitation Linux.
- Stabiliser le contrat plugin.

## Phase 6 — Extension progressive

- Ajouter de nouveaux skills uniquement après documentation.
- Exemples possibles : logs, disque, réseau, paquets, utilisateurs.
- Pour chaque skill, définir d'abord les actions autorisées et interdites.
