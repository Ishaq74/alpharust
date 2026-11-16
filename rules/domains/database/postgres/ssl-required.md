---
name: postgres-ssl-required
apply: "postgres"
severity: error
pattern: "(?i)sslmode\\s*=\\s*(disable|allow)|ssl\\s*=\\s*false"
examples:
  bad:
    - "postgres://h/db?sslmode=disable"
    - "ssl=false"
  good:
    - "postgres://h/db?sslmode=require"
---

# SSL obligatoire pour PostgreSQL

Toute connexion PostgreSQL doit utiliser SSL (`sslmode=require` ou équivalent).
Les connexions non chiffrées sont interdites.
