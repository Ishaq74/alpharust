---
name: gcp-api-key
apply: "*"
severity: error
pattern: "\\bAIza[0-9A-Za-z_-]{35}\\b"
examples:
  bad:
    - "const k = \"AIza¦SyA1234567890abcdefghijklmnopqrstuv\";"
  good:
    - "const k = process.env.GOOGLE_API_KEY;"
---

# Clé d'API Google en clair

Les clés d'API Google (`AIza…`) doivent être restreintes et chargées depuis l'environnement.
