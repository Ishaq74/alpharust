---
name: session-secure
apply: "*"
severity: error
pattern: "secure\\s*[:=]\\s*false|httponly\\s*[:=]\\s*false"
---

# Sessions sécurisées

Les cookies de session doivent avoir les flags `Secure` et `HttpOnly` activés.
