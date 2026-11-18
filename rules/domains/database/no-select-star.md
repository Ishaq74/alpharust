---
name: no-select-star
apply: "*"
severity: warning
pattern: "(?i)\\bSELECT\\s+\\*"
examples:
  bad:
    - "SELECT * FROM users;"
    - "select * from t"
  good:
    - "SELECT id, name FROM users;"
    - "SELECT COUNT(*) FROM users;"
---

# Éviter SELECT *

Préférer lister explicitement les colonnes nécessaires.
