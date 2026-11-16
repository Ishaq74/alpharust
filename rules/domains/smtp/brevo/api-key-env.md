---
name: brevo-api-key-env
apply: "*"
severity: error
pattern: "(?i)BREVO_API_KEY\\s*[:=]\\s*['\\\"][^'\\\"\\s$]{8,}['\\\"]|\\bxkeysib-[0-9a-f]{40,}"
examples:
  bad:
    - "BREVO_API_KEY = \"abcdefgh12345678\""
    - "k = \"xkeys¦ib-0123456789abcdef0123456789abcdef01234567\""
  good:
    - "BREVO_API_KEY = os.getenv(\"BREVO_API_KEY\")"
---

# Clé Brevo en clair

Les clés Brevo (`xkeysib-…`) doivent venir de l'environnement.
