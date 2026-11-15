---
name: stripe-webhook-signature
apply: stripe
severity: error
pattern: ""
---

# Signature webhook Stripe

Toute route webhook Stripe doit vérifier la signature avec `stripe.webhooks.constructEvent`.
