---
name: no-plaintext-keys
apply: "*"
severity: error
pattern: "(sk_live_|sk_test_|pk_live_|rk_live_)"
---

# Clés de paiement jamais en clair

Les clés Stripe, Mollie, PayPal, etc. doivent exclusivement venir de variables d’environnement.
