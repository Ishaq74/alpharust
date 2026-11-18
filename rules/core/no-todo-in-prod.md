---
name: no-todo-in-prod
apply: "*"
severity: info
pattern: "\\b(TODO|FIXME|XXX)\\b"
examples:
  bad:
    - "// TODO: remove this hack"
    - "# FIXME later"
  good:
    - "fn todo_list() {}"
    - "let xxxl = 1;"
---

# TODOs en production

Les commentaires TODO/FIXME doivent être traités ou justifiés avant une mise en production.
