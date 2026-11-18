---
name: no-eval
apply: "*"
severity: error
pattern: "\\beval\\s*\\("
examples:
  bad:
    - "eval(code)"
    - "x = eval (\"1+1\")"
  good:
    - "evaluate(x)"
    - "retrieval(y)"
---

# Interdiction d’eval / évaluation dynamique

Aucun appel à eval, exec, Invoke-Expression ou équivalent.
