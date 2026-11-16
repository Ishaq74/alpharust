---
name: auth0-client-secret-env
apply: "*"
severity: error
pattern: "(?i)AUTH0_CLIENT_SECRET\\s*[:=]\\s*['\\\"][^'\\\"\\s$]{8,}['\\\"]"
examples:
  bad:
    - "AUTH0_CLIENT_SECRET = \"abcdefgh12345678\""
  good:
    - "AUTH0_CLIENT_SECRET = process.env.AUTH0_CLIENT_SECRET"
---

# Secret client Auth0 en clair

`AUTH0_CLIENT_SECRET` doit venir de l'environnement.
