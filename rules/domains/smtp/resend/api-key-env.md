---
name: resend-api-key-env
apply: "resend"
severity: error
pattern: "(?i)RESEND_API_KEY\\s*[:=]\\s*['\\\"][^'\\\"\\s$]+['\\\"]|\\bre_[A-Za-z0-9]{20,}"
examples:
  bad:
    - "RESEND_API_KEY = \"re_¦abcdefghijklmnopqrstu\""
  good:
    - "RESEND_API_KEY = process.env.RESEND_API_KEY"
---

# Clé Resend uniquement via environnement

La clé API Resend ne doit jamais apparaître en dur dans le code ou les fichiers de configuration versionnés.
