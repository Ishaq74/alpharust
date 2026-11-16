---
name: js-no-eval
apply: "javascript"
severity: error
pattern: "\\beval\\s*\\("
examples:
  bad:
    - "const r = eval(input);"
  good:
    - "const r = JSON.parse(input);"
---

# Interdiction d’eval en JavaScript

`eval()` et `new Function()` avec du code dynamique sont interdits.
