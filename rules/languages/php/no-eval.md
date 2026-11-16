---
name: php-no-eval
apply: "php"
severity: error
pattern: "\\beval\\s*\\("
examples:
  bad:
    - "eval($_GET[\"code\"]);"
  good:
    - "$x = evaluate($y);"
---

# Interdiction d’eval en PHP

`eval()` est interdit. Utiliser des alternatives sûres (templates, configuration, etc.).
