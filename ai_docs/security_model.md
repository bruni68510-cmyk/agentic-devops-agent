# Modèle de sécurité

## Principe central

L'agent ne doit pouvoir faire que ce qui a été explicitement programmé, déclaré et autorisé.
Une demande utilisateur ou un plugin ne doit jamais pouvoir transformer l'agent en shell généraliste.

## Défense en profondeur

Le modèle de sécurité doit combiner plusieurs barrières :

- **Utilisateur Linux non privilégié** pour le processus principal.
- **Allowlist de capacités** exposées par les plugins.
- **Validation stricte des arguments** avant exécution.
- **Policy engine indépendant** du code métier du plugin.
- **Journalisation** des décisions et actions.
- **Isolation optionnelle des plugins** par processus, sandbox, WebAssembly, seccomp, AppArmor ou SELinux.
- **Élévation contrôlée** uniquement pour des opérations précises.

## Actions interdites par défaut

- Exécution de commandes shell arbitraires.
- Écriture libre dans le système de fichiers.
- Modification de fichiers système hors périmètre déclaré.
- Installation de paquets sans capacité dédiée et politique explicite.
- Changement d'utilisateur, de groupe ou de permissions sans capacité dédiée.
- Accès réseau sortant non documenté.

## Enforcement

Le framework d'enforcement doit recevoir une décision structurée, par exemple :

- identité de l'appelant ;
- plugin demandé ;
- capacité demandée ;
- action exacte ;
- arguments normalisés ;
- ressource cible ;
- niveau de risque ;
- contexte d'exécution.

Le plugin ne doit pas être l'autorité finale pour décider si son action est permise.

## Politique déclarative

Une politique pourrait exprimer :

- quels services peuvent être listés ;
- quels services peuvent être démarrés ou arrêtés ;
- quels utilisateurs peuvent lancer ces actions ;
- quelles actions nécessitent confirmation ;
- quelles actions sont uniquement disponibles en lecture seule.

## Question ouverte

Le choix exact entre `polkit`, `sudoers` restrictif, capabilities Linux, OPA, Cedar, seccomp, AppArmor, SELinux ou WebAssembly doit être évalué par prototype.
La décision doit se baser sur la capacité à garantir le moindre privilège, l'auditabilité et la simplicité d'exploitation.
