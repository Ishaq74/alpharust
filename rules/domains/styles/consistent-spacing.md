---
name: consistent-spacing
apply: "*.rs,*.js,*.jsx,*.ts,*.tsx,*.css,*.scss,*.py,*.php,*.html,*.astro,*.vue,*.json,*.yml,*.yaml"
severity: info
checker: indent-consistency
examples:
  bad:
    - "a\n  b\n  c\n\td\n"
  good:
    - "a\n\tb\n\tc\n"
    - "a\n  b\n    c\n"
---

# Espacement cohérent

Utiliser une échelle d’espacement cohérente (4px, 8px, etc.) plutôt que des valeurs arbitraires.
