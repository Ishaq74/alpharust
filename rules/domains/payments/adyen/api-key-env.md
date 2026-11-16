---
name: adyen-api-key-env
apply: "*"
severity: error
pattern: "(?i)ADYEN_(API_)?KEY\\s*[:=]\\s*['\\\"][^'\\\"\\s$]{8,}['\\\"]"
examples:
  bad:
    - "ADYEN_API_KEY = \"AQEabcdefgh1234\""
  good:
    - "ADYEN_API_KEY = os.environ[\"ADYEN_API_KEY\"]"
---

# Clé Adyen en clair

La clé d'API Adyen doit venir de l'environnement.
