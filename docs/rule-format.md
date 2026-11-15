# Format des règles

Chaque règle est un fichier Markdown avec frontmatter YAML.

```yaml
---
name: nom-de-la-regle
apply: "*"                 # "*" | powershell | php | laravel | postgres | ...
severity: error            # error | warning | info
pattern: "regex optionnelle"
---
```

## Corps

Description claire de la règle.

## Validation

- `apply` doit être quoté si spécial (`"*"`)
- `pattern` vide = règle informative (non exécutée par le runner regex)
- Une regex invalide est ignorée avec un warning sur stderr

## Test

Ajouter une fixture dans `tests/fixtures/violations/` pour vérifier la détection.
