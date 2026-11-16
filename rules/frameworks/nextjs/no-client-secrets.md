---
name: nextjs-no-client-secrets
apply: "*.js,*.jsx,*.ts,*.tsx,*.mjs,.env*"
severity: error
pattern: "\\bNEXT_PUBLIC_\\w*(SECRET|PRIVATE|PASSWORD|TOKEN)\\w*"
examples:
  bad:
    - "NEXT_PUBLIC_API_SECRET=abc"
  good:
    - "NEXT_PUBLIC_SITE_URL=https://x.org"
    - "process.env.API_SECRET"
---

# Pas de secrets côté client Next.js

Les variables `NEXT_PUBLIC_` ne doivent jamais contenir de secrets.
