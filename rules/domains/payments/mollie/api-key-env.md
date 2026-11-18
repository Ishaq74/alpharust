---
name: mollie-api-key-env
apply: "mollie"
severity: error
pattern: "(?i)MOLLIE_API_KEY\\s*[:=]\\s*['\\\"][^'\\\"\\s$]+['\\\"]|\\b(live|test)_[A-Za-z0-9]{30,}\\b"
examples:
  bad:
    - "MOLLIE_API_KEY = \"test_dHar4XY7LxsDOtmnkVtjNVWXLSlXsM\""
  good:
    - "MOLLIE_API_KEY = getenv(\"MOLLIE_API_KEY\")"
---

# Clé Mollie via variable d’environnement
