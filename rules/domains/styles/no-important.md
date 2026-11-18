---
name: no-important
apply: "*.css,*.scss,*.sass,*.less,*.html,*.astro,*.vue,*.svelte"
severity: warning
pattern: "!important"
examples:
  bad:
    - ".a { color: red !important; }"
  good:
    - ".a { color: red; }"
---

# Éviter !important

`!important` doit être évité. Préférer une meilleure spécificité ou une architecture CSS plus claire.
