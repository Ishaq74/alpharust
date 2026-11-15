# Format des règles

Chaque règle est un fichier Markdown avec frontmatter YAML :

```yaml
---
name: nom-de-la-regle
apply: * | powershell | php | ...
severity: error | warning | info
pattern: "regex optionnelle"
---
```

Le corps contient la description.
