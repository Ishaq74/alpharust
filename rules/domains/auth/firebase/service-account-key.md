---
name: firebase-service-account-key
apply: "*.json,*.js,*.ts,*.py,*.php"
severity: error
pattern: "\\\"type\\\"\\s*:\\s*\\\"service_account\\\""
examples:
  bad:
    - "{ \"type\": \"service_account\", \"project_id\": \"x\" }"
  good:
    - "{ \"type\": \"authorized_user\" }"
---

# Compte de service Firebase / Google versionné

Un fichier de compte de service (`"type": "service_account"`) contient une clé privée : ne pas le commiter.
