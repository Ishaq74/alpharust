# ADR 0002 — Format des règles et validité YAML

## Statut
Accepté

## Contexte
Le frontmatter des règles est du YAML. La valeur `apply: *` est invalide (le `*` démarre un alias YAML) et faisait échouer le parseur et la CI.

## Décision
- Toutes les valeurs potentiellement spéciales doivent être quotées : `apply: "*"`
- Le parseur reste strict (serde_yaml)
- Le template officiel (`rules/_template.md`) impose le format correct

## Conséquences
Toute nouvelle règle doit suivre le format quoté. Les règles existantes non quotées doivent être corrigées progressivement.
