---
name: resend-api-key-env
apply: *
severity: error
pattern: "RESEND_API_KEY\\s*=\\s*['\"][^'\"]+['\"]"
---

# Clé Resend uniquement via variable d’environnement
