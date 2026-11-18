---
name: no-plaintext-secrets
apply: "*"
severity: error
pattern: "(?i)(password|secret|api_key|token)\\s*[:=]\\s*['\\\"][^'\\\"]+['\\\"]"
examples:
  bad:
    - "password = \"hunter2\""
    - "API_KEY: 'abcdef123'"
  good:
    - "password = std::env::var(\"PW\")"
    - "let token = \"\";"
---

# Interdiction des secrets en clair
