# ADR 0001 — Fondation AlphaRust

## Statut
Accepté

## Contexte
Création de la fondation unique pour primitives, règles, génération et orchestration. Zéro dette technique. Prêt-à-porter sur mesure.

## Décision
- Rust comme moteur unique (binaire statique).
- Règles en Markdown comme source de vérité.
- Profils pour l’orchestration.
- Domaines (database, smtp, payments, styles, design, auth, docs, readme, checks, tests) avec third-parties.
- Générateurs intégrés (bootstrap, readme, docs, tests unitaires).

## Conséquences
Tout le reste (starters, forge, etc.) se construit sur cette base ou est absorbé.
