---
name: no-raw-sql
apply: *
severity: warning
pattern: "(SELECT|INSERT|UPDATE|DELETE).*FROM"
---

# Éviter le SQL brut non paramétré
