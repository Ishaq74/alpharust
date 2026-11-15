---
name: js-no-eval
apply: javascript
severity: error
pattern: "\\beval\\s*\\("
---

# Interdiction d’eval en JavaScript

`eval()` et `new Function()` avec du code dynamique sont interdits.
