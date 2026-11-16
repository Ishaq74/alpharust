---
name: stripe-webhook-signature
apply: "*.js,*.jsx,*.ts,*.tsx,*.py,*.php,*.rb,*.go,*.java"
severity: error
checker: requires-companion
args:
  trigger: "(?i)/stripe[-_/]?webhooks?\\b|stripe_webhooks?\\b|stripeWebhooks?\\b"
  require: "constructEvent|construct_event|Webhook::constructEvent"
  message: "webhook Stripe sans constructEvent"
examples:
  bad:
    - "app.post('/stripe/webhook', handler);"
  good:
    - "app.post('/stripe/webhook', (r) => stripe.webhooks.constructEvent(r.body, sig, secret));"
---

# Signature webhook Stripe

Toute route webhook Stripe doit vérifier la signature avec `stripe.webhooks.constructEvent`.
