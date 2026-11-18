---
name: no-plaintext-keys
apply: "*"
severity: error
pattern: "\\b(sk|rk)_(live|test)_[A-Za-z0-9]{6,}"
examples:
  bad:
    - "key = \"sk_¦live_ABCDEF123456\""
    - "rk_¦test_ABCDEF123456"
  good:
    - "key = process.env.STRIPE_KEY"
    - "pk_live_ABCDEF123456"
---

# Clés de paiement jamais en clair

Les clés Stripe, Mollie, PayPal, etc. doivent exclusivement venir de variables d’environnement.
