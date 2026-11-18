---
name: smtp-no-hardcoded-credentials
apply: "*"
severity: error
pattern: "(?i)(smtp_password|mail_password|smtp_pass)\\s*[:=]\\s*['\\\"][^'\\\"\\s$]+['\\\"]"
examples:
  bad:
    - "SMTP_PASSWORD = \"hunter2\""
    - "mail_password: \"x\""
  good:
    - "SMTP_PASSWORD = os.environ[\"SMTP_PASSWORD\"]"
---

# Pas de credentials SMTP en dur
