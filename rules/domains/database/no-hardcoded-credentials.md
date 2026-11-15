---
name: db-no-hardcoded-credentials
apply: "*"
severity: error
pattern: "(password|passwd|pwd)\\s*[:=]\\s*['\"][^'\"]+['\"]"
---

# Pas de credentials base de données en dur
