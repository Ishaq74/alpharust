---
name: stripe-secret-key-env
apply: "stripe"
severity: error
pattern: "\\b(sk|rk)_(live|test)_[A-Za-z0-9]{6,}"
examples:
  bad:
    - "Stripe(\"sk_¦test_ABCDEF123456\")"
  good:
    - "Stripe(process.env.STRIPE_SECRET_KEY)"
---

# Clé secrète Stripe via environnement uniquement

Les clés secrètes Stripe (`sk_live_`, `sk_test_`) ne doivent jamais être committées.
