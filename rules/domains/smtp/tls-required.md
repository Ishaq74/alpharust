---
name: smtp-tls-required
apply: "*"
severity: error
pattern: "(?i)\\bssl\\s*=\\s*false|\\bstarttls\\s*=\\s*false|\\bsecure\\s*:\\s*false"
examples:
  bad:
    - "starttls = false"
    - "{ secure: false }"
  good:
    - "starttls = true"
    - "{ secure: true }"
---

# TLS obligatoire pour SMTP

Toute connexion SMTP doit utiliser TLS ou STARTTLS.
Les connexions en clair sont interdites.
