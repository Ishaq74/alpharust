---
name: python-no-exec
apply: python
severity: error
pattern: "\\b(exec|eval)\\s*\\("
---

# Interdiction d’exec et eval en Python

`exec()` et `eval()` sont interdits sauf cas extrêmement justifié et isolé.
