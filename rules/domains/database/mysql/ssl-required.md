---
name: mysql-ssl-required
apply: "mysql"
severity: error
pattern: "(?i)ssl\\s*=\\s*false|useSSL\\s*=\\s*false"
examples:
  bad:
    - "jdbc:mysql://db/app?useSSL=false"
    - "mysql ssl=false"
  good:
    - "jdbc:mysql://db/app?useSSL=true"
---

# MySQL SSL requis
