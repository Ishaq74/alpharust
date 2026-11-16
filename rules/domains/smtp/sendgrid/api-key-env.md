---
name: sendgrid-api-key-env
apply: "*"
severity: error
pattern: "(?i)SENDGRID_API_KEY\\s*[:=]\\s*['\\\"][^'\\\"\\s$]{8,}['\\\"]|\\bSG\\.[A-Za-z0-9_-]{16,}\\.[A-Za-z0-9_-]{16,}"
examples:
  bad:
    - "SENDGRID_API_KEY = \"SG.¦abcdefghijklmnop.abcdefghijklmnop\""
    - "SENDGRID_API_KEY=\"abcdefgh1234\""
  good:
    - "SENDGRID_API_KEY = os.environ[\"SENDGRID_API_KEY\"]"
---

# Clé SendGrid en clair

La clé `SENDGRID_API_KEY` doit venir de l'environnement, jamais du code.
