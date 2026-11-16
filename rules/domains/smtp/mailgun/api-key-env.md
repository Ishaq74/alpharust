---
name: mailgun-api-key-env
apply: "mailgun"
severity: error
pattern: "(?i)MAILGUN_API_KEY\\s*[:=]\\s*['\\\"][^'\\\"\\s$]+['\\\"]|\\bkey-[0-9a-f]{32}\\b"
examples:
  bad:
    - "MAILGUN_API_KEY = \"abc123\""
  good:
    - "MAILGUN_API_KEY = os.getenv(\"MAILGUN_API_KEY\")"
---

# Clé Mailgun via variable d’environnement
