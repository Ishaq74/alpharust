---
name: no-hardcoded-urls
apply: "*"
severity: warning
pattern: "https?://[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
---

# Éviter les URLs en dur

Les URLs d’API ou de services externes doivent venir de la configuration ou de variables d’environnement.
