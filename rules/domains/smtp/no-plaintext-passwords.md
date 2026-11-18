---
name: smtp-no-plaintext-passwords
apply: "*"
severity: error
pattern: "(?i)\\b(smtp|mail)[_.]?(pass|password)\\s*[:=]\\s*['\\\"]"
examples:
  bad:
    - "smtp.password = \"abc\""
  good:
    - "smtp.host = \"localhost\""
---

# Pas de mots de passe SMTP en clair
