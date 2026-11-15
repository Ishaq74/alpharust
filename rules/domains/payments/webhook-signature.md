---
name: webhook-signature
apply: "*"
severity: error
pattern: ""
---

# Signature des webhooks

Toute réception de webhook (Stripe, Mollie, PayPal, etc.) doit vérifier la signature cryptographique avant traitement.
