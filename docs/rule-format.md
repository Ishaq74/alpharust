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

Description claire de la règle, pourquoi elle existe, et comment la respecter.

## Bonnes pratiques

- Toujours quoter `apply` si la valeur contient des caractères spéciaux
- Pattern vide = règle informative ou future checker sémantique
- Une règle = un fichier
