---
name: no-plaintext-passwords
apply: "*"
severity: error
pattern: "(?i)password\\s*[:=]\\s*['\\\"][^'\\\"]+['\\\"]"
examples:
  bad:
    - "password = \"secret123\""
  good:
    - "password = os.environ[\"DB_PASSWORD\"]"
---

# Mots de passe jamais en clair
