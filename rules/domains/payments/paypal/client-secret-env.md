---
name: paypal-client-secret-env
apply: "paypal"
severity: error
pattern: "(?i)PAYPAL_CLIENT_SECRET\\s*[:=]\\s*['\\\"][^'\\\"\\s$]+['\\\"]"
examples:
  bad:
    - "PAYPAL_CLIENT_SECRET = \"EJx9abc\""
  good:
    - "PAYPAL_CLIENT_SECRET = os.environ[\"X\"]"
---

# Secret PayPal via variable d’environnement
