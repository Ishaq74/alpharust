---
name: braintree-private-key-env
apply: "*"
severity: error
pattern: "(?i)BRAINTREE_PRIVATE_KEY\\s*[:=]\\s*['\\\"][^'\\\"\\s$]{8,}['\\\"]"
examples:
  bad:
    - "BRAINTREE_PRIVATE_KEY = \"abcdefgh12345678\""
  good:
    - "BRAINTREE_PRIVATE_KEY = os.environ[\"BRAINTREE_PRIVATE_KEY\"]"
---

# Clé privée Braintree en clair

`BRAINTREE_PRIVATE_KEY` doit venir de l'environnement.
