---
name: webhook-signature
apply: "*.js,*.jsx,*.ts,*.tsx,*.py,*.php,*.rb,*.go,*.java"
severity: error
checker: requires-companion
args:
  trigger: "(?i)webhook"
  require: "(?i)signature|constructEvent|construct_event|hmac|verify"
  message: "webhook sans vérification de signature"
examples:
  bad:
    - "app.post('/webhook', (req, res) => res.sendStatus(200));"
  good:
    - "app.post('/webhook', (req) => verifySignature(req));"
    - "x = 1"
---

# Signature des webhooks

Toute réception de webhook (Stripe, Mollie, PayPal, etc.) doit vérifier la signature cryptographique avant traitement.
