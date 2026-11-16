---
name: astro-no-client-secrets
apply: "*.astro,*.ts,*.js,*.mjs,*.jsx,*.tsx,.env*"
severity: error
pattern: "\\bPUBLIC_\\w*(SECRET|PRIVATE|PASSWORD|TOKEN)\\w*"
examples:
  bad:
    - "PUBLIC_API_SECRET=abc"
    - "import.meta.env.PUBLIC_DB_PASSWORD"
  good:
    - "PUBLIC_SITE_URL=https://x.org"
    - "import.meta.env.API_SECRET"
---

# Pas de secrets côté client Astro

Les variables `PUBLIC_` et `import.meta.env` ne doivent jamais contenir de secrets.
