---
name: session-secure
apply: "*"
severity: error
pattern: "(?i)(^|[^a-z])(secure|httponly)\\s*[:=]\\s*false"
examples:
  bad:
    - "cookie: { secure: false }"
    - "SESSION_COOKIE_HTTPONLY = false"
  good:
    - "cookie: { secure: true, httpOnly: true }"
---

# Sessions sécurisées

Les cookies de session doivent avoir les flags `Secure` et `HttpOnly` activés.
