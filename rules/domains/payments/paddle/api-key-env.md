---
name: paddle-api-key-env
apply: "*"
severity: error
pattern: "(?i)PADDLE_(API_KEY|WEBHOOK_SECRET)\\s*[:=]\\s*['\\\"][^'\\\"\\s$]{8,}['\\\"]"
examples:
  bad:
    - "PADDLE_API_KEY = \"pdl_live_apikey_abcdef\""
    - "PADDLE_WEBHOOK_SECRET: \"pdl_ntfset_abcdef\""
  good:
    - "PADDLE_API_KEY = os.environ[\"PADDLE_API_KEY\"]"
---

# Secret Paddle en clair

Les clés d'API et secrets de webhook Paddle doivent venir de l'environnement.
