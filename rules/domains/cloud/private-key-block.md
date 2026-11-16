---
name: private-key-block
apply: "*"
severity: error
pattern: "-----BEGIN (RSA |EC |DSA |OPENSSH |PGP |ENCRYPTED )?PRIVATE KEY( BLOCK)?-----"
examples:
  bad:
    - "-----BEGIN RSA PRIV¦ATE KEY-----"
    - "-----BEGIN PRIV¦ATE KEY-----"
  good:
    - "-----BEGIN PUBLIC KEY-----"
    - "-----BEGIN CERTIFICATE-----"
---

# Clé privée dans le dépôt

Aucune clé privée (PEM/OpenSSH/PGP) ne doit être commitée.
