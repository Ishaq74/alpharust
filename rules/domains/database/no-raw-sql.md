---
name: no-raw-sql
apply: "*"
severity: warning
pattern: "(SELECT|INSERT|UPDATE|DELETE).*FROM"
---

# Éviter le SQL brut non paramétré

Préférer les query builders ou ORM avec paramètres liés.
Le SQL brut doit être justifié et sécurisé.
