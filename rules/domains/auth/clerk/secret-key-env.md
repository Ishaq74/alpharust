---
name: clerk-secret-key-env
apply: "*"
severity: error
pattern: "(?i)CLERK_SECRET_KEY\\s*[:=]\\s*['\\\"][^'\\\"\\s$]{8,}['\\\"]"
examples:
  bad:
    - "CLERK_SECRET_KEY = \"abcdefgh12345678\""
  good:
    - "CLERK_SECRET_KEY = process.env.CLERK_SECRET_KEY"
---

# Clé secrète Clerk en clair

`CLERK_SECRET_KEY` doit venir de l'environnement.
