# Vision

## Idée générale

Créer un agent DevOps local écrit en Rust pour Linux, capable d'effectuer des actions système via des plugins spécialisés.
L'agent se comporte comme un restricted shell applicatif : il n'offre pas un terminal libre, mais une collection de capacités contrôlées et vérifiables.

## Objectifs

- Fournir une interface sûre pour interroger et administrer un système Linux.
- Permettre l'ajout progressif de capacités via des plugins.
- Empêcher les plugins et l'utilisateur de dépasser le périmètre prévu.
- Produire des réponses utiles pour le diagnostic DevOps : état des services, erreurs récentes, actions possibles.
- Rendre chaque action auditable et reproductible.

## Non-objectifs initiaux

- Remplacer un shell Unix complet.
- Donner un accès root permanent à l'agent.
- Exécuter des commandes arbitraires fournies par l'utilisateur ou par un plugin.
- Couvrir Windows ou macOS dans la première phase.
- Gérer l'orchestration distante multi-machines dès le départ.

## Utilisateur cible

L'utilisateur cible est un administrateur, un développeur DevOps ou un opérateur qui veut déléguer des diagnostics et des actions simples à un agent local, sans ouvrir un accès shell complet.
