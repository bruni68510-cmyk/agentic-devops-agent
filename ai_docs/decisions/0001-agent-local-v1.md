# Décision 0001 — Agent local Linux en V1

## Statut

Acceptée.

## Contexte

Le projet vise un agent DevOps Rust pour Linux qui fonctionne comme un restricted shell applicatif.
Avant d'ajouter des capacités réseau, multi-machines ou distribuées, il faut stabiliser le modèle local : privilèges, plugins, enforcement, audit et premiers adaptateurs système.

## Décision

La V1 cible un **agent local Linux uniquement**, utilisé d'abord via une **CLI locale**.

Le processus principal doit tourner comme un utilisateur Linux non privilégié.
Il ne doit pas supposer un accès `root`, ni ouvrir d'interface réseau par défaut.

## Conséquences

- Le premier prototype peut se concentrer sur la sûreté du modèle d'exécution local.
- Les questions d'authentification réseau, chiffrement, exposition HTTP/gRPC et orchestration distante sont repoussées.
- Les interactions système passent par des adaptateurs typés au lieu de commandes shell arbitraires.
- Le déploiement initial reste simple : un binaire Rust et une configuration locale.

## Hors périmètre V1

- Agent distant piloté depuis une autre machine.
- API HTTP publique.
- Orchestration multi-hôtes.
- Gestion centralisée de flotte.
