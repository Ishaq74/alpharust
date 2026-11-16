---
name: db-no-hardcoded-credentials
apply: "*"
severity: error
pattern: "(?i)\\b(db_)?(password|passwd|pwd)\\s*[:=]\\s*['\\\"][^'\\\"]+['\\\"]"
examples:
  bad:
    - "DB_PASSWORD = \"abc\""
    - "pwd: \"x1\""
  good:
    - "password = os.getenv(\"DB_PASSWORD\")"
---

# Pas de credentials base de données en dur
