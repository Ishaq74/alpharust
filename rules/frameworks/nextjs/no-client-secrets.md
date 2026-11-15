---
name: nextjs-no-client-secrets
apply: nextjs
severity: error
pattern: "NEXT_PUBLIC_.*SECRET|process\\.env\\.SECRET"
---

# Pas de secrets côté client Next.js

Les variables `NEXT_PUBLIC_` ne doivent jamais contenir de secrets.
