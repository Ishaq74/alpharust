---
name: php-no-eval
apply: php
severity: error
pattern: "\\beval\\s*\\("
---

# Interdiction d’eval en PHP

`eval()` est interdit. Utiliser des alternatives sûres (templates, configuration, etc.).
