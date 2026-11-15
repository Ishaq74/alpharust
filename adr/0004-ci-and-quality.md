# ADR 0004 — CI et qualité minimale

## Statut
Accepté

## Contexte
La CI a échoué à plusieurs reprises (dépendance inutilisée, YAML invalide, test cassé). Le projet doit rester buildable en permanence.

## Décision
- CI GitHub Actions obligatoire sur `main` et PR : `cargo build --release` + `cargo test`
- Aucune dépendance inutilisée
- Tout changement de format de règle doit être accompagné d’un test
- Les warnings de compilation sont traités

## Conséquences
Le repo doit toujours être dans un état où `cargo build` et `cargo test` passent sur la CI.
