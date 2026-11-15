# Format des règles

Chaque règle est un fichier `.md` avec frontmatter YAML.

```yaml
---
name: nom-de-la-regle
apply: "*"          # ou powershell, php, laravel, etc. Toujours quoté si spécial
severity: error     # error | warning | info
pattern: "regex"    # optionnel
---
```

Le corps Markdown contient la description humaine.

## Règles

- `apply: "*"` pour universel
- Pattern vide = règle informative ou sémantique future
- Toujours YAML valide
