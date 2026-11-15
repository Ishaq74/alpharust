---
name: stripe-secret-key-env
apply: stripe
severity: error
pattern: "sk_(live|test)_"
---

# Clé secrète Stripe via environnement uniquement

Les clés secrètes Stripe (`sk_live_`, `sk_test_`) ne doivent jamais être committées.
