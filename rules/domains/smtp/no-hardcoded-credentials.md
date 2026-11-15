---
name: smtp-no-hardcoded-credentials
apply: "*"
severity: error
pattern: "(smtp_password|mail_password)\\s*[:=]"
---

# Pas de credentials SMTP en dur
