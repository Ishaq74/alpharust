---
name: no-plaintext-secrets
apply: "*"
severity: error
pattern: "(password|secret|api_key|token)\\s*[:=]\\s*['\"][^'\"]+['\"]"
---

# Interdiction des secrets en clair
