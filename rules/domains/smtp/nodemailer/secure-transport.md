---
name: nodemailer-secure-transport
apply: "nodemailer"
severity: error
pattern: "secure:\\s*false"
examples:
  bad:
    - "nodemailer.createTransport({ secure: false })"
  good:
    - "nodemailer.createTransport({ secure: true })"
---

# Transport Nodemailer sécurisé

Nodemailer doit être configuré avec `secure: true` ou un équivalent TLS.
