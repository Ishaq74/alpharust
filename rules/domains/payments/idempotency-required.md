---
name: idempotency-required
apply: "*.js,*.jsx,*.ts,*.tsx,*.mjs,*.py,*.php,*.rb,*.go,*.java,*.rs"
severity: warning
checker: requires-companion
args:
  trigger: "(?i)\\b(paymentIntents|charges|refunds|payouts)\\.create\\b|stripe\\.(PaymentIntent|Charge|Refund)\\.create"
  require: "(?i)idempoten"
  message: "création de paiement sans clé d'idempotence"
examples:
  bad:
    - "await stripe.paymentIntents.create({ amount: 100 });"
    - "stripe.Charge.create(amount=100)"
  good:
    - "await stripe.paymentIntents.create({ amount: 1 }, { idempotencyKey: key });"
---

# Idempotence requise pour les paiements
