# AGENTS.md — Agentic DevOps Agent

## Vision du projet

Ce dépôt documente et prépare la création d'un agent DevOps écrit en Rust, ciblant Linux en premier lieu.
L'agent doit fonctionner comme un **restricted shell applicatif** : il reçoit des intentions utilisateur, les traduit en actions système autorisées, puis exécute uniquement les opérations explicitement prévues par son modèle de permissions.

L'agent ne doit pas tourner en `root` par défaut. Il doit fonctionner comme un utilisateur Linux dédié et non privilégié, avec des élévations limitées, traçables et déclaratives lorsque certaines opérations système l'exigent.

## Principes de conception

- **Sécurité par défaut** : toute action non déclarée est refusée.
- **Moindre privilège** : l'agent tourne comme utilisateur normal et n'obtient que les droits strictement nécessaires.
- **Plugins contrôlés** : chaque plugin expose un ensemble borné de capacités, jamais un accès shell générique.
- **Contrats explicites** : les entrées, sorties, permissions et effets de bord de chaque plugin doivent être documentés.
- **Auditabilité** : chaque décision d'autorisation et chaque action système doit pouvoir être journalisée.
- **Linux d'abord** : la première cible est Linux avec `systemd` pour les capacités liées aux services.
- **Rust d'abord** : l'implémentation doit privilégier la sûreté mémoire, les types forts et des erreurs explicites.

## Structure documentaire

Les décisions et idées doivent être maintenues dans `ai_docs/` :

- `ai_docs/vision.md` : description produit et objectifs.
- `ai_docs/architecture.md` : architecture technique envisagée.
- `ai_docs/security_model.md` : modèle de sécurité et d'enforcement.
- `ai_docs/plugins.md` : modèle de plugins et contrats de capacités.
- `ai_docs/service_skill.md` : premier skill cible autour des services Linux.
- `ai_docs/roadmap.md` : étapes de réalisation.

## Contraintes importantes pour les futurs agents

1. Ne pas introduire d'exécution de commandes arbitraires comme interface plugin.
2. Ne pas supposer que l'agent peut tourner en `root`.
3. Documenter toute nouvelle capacité avant de proposer son implémentation.
4. Préférer des allowlists, des politiques déclaratives et des API typées aux chaînes de commandes libres.
5. Pour les actions sensibles, prévoir une couche d'autorisation indépendante du plugin.
6. Toute proposition d'intégration avec `sudo`, `polkit`, capabilities Linux, seccomp, AppArmor ou SELinux doit expliquer le périmètre exact accordé.

## Style de documentation

- Écrire en français, sauf noms d'API, commandes, concepts techniques ou citations de projet.
- Utiliser des titres Markdown courts et hiérarchisés.
- Séparer clairement les objectifs, les décisions prises et les questions ouvertes.
- Marquer les éléments incertains avec `Question ouverte` plutôt que de les présenter comme acquis.
