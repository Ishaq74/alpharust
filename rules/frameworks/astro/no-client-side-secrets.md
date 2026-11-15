---
name: astro-no-client-secrets
apply: astro
severity: error
pattern: "PUBLIC_.*SECRET|import.meta.env.SECRET"
---

# Pas de secrets côté client Astro
