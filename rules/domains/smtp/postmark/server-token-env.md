---
name: postmark-server-token-env
apply: "*"
severity: error
pattern: "(?i)POSTMARK_(SERVER_|API_)?TOKEN\\s*[:=]\\s*['\\\"][0-9a-f-]{36}['\\\"]"
examples:
  bad:
    - "POSTMARK_SERVER_TOKEN = \"123e4567-e89b-12d3-a456-426614174000\""
  good:
    - "POSTMARK_SERVER_TOKEN = os.environ[\"POSTMARK_SERVER_TOKEN\"]"
---

# Jeton Postmark en clair

Le jeton serveur Postmark doit venir de l'environnement.
