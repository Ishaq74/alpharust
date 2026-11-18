---
name: python-no-exec
apply: "python"
severity: error
pattern: "\\b(exec|eval)\\s*\\("
examples:
  bad:
    - "exec(code)"
    - "result = eval(expr)"
  good:
    - "cursor.execute(q)"
    - "ast.literal_eval(s)"
---

# Interdiction d’exec et eval en Python

`exec()` et `eval()` sont interdits sauf cas extrêmement justifié et isolé.
