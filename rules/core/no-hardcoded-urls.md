---
name: no-hardcoded-urls
apply: "*.rs,*.py,*.php,*.js,*.jsx,*.ts,*.tsx,*.mjs,*.rb,*.go,*.java,*.ps1,*.sh"
severity: warning
pattern: "https?://[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
examples:
  bad:
    - "let u = \"https://api.service.io/v1\";"
  good:
    - "let u = std::env::var(\"API_URL\");"
---

# Éviter les URLs en dur

Les URLs d’API ou de services externes doivent venir de la configuration ou de variables d’environnement.
