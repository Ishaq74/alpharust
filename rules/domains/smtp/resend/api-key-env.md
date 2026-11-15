---
name: resend-api-key-env
apply: resend
severity: error
pattern: "RESEND_API_KEY\\s*=\\s*['\"][^'\"]+['\"]"
---

# Clé Resend uniquement via environnement

La clé API Resend ne doit jamais apparaître en dur dans le code ou les fichiers de configuration versionnés.
