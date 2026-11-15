---
name: astro-no-client-secrets
apply: astro
severity: error
pattern: "PUBLIC_.*SECRET|import.meta.env\\.SECRET"
---

# Pas de secrets côté client Astro

Les variables `PUBLIC_` et `import.meta.env` ne doivent jamais contenir de secrets.
