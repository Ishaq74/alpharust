---
name: postgres-ssl-required
apply: postgres
severity: error
pattern: "sslmode=disable|ssl=false"
---

# SSL obligatoire pour PostgreSQL

Toute connexion PostgreSQL doit utiliser SSL (`sslmode=require` ou équivalent).
Les connexions non chiffrées sont interdites.
