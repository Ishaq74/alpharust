---
name: smtp-tls-required
apply: "*"
severity: error
pattern: "ssl\\s*=\\s*false|starttls\\s*=\\s*false|secure\\s*:\\s*false"
---

# TLS obligatoire pour SMTP

Toute connexion SMTP doit utiliser TLS ou STARTTLS.
Les connexions en clair sont interdites.
