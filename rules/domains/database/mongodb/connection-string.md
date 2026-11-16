---
name: mongodb-connection-string
apply: "*"
severity: error
pattern: "mongodb(\\+srv)?://[^/\\s:@'\\\"$]+:[^@\\s'\\\"$]+@"
examples:
  bad:
    - "uri = \"mongodb+srv://user:p4ss@cluster0.example.net/db\""
  good:
    - "uri = \"mongodb://localhost:27017/db\""
    - "uri = \"mongodb://${USER}:${PASS}@host/db\""
---

# Chaîne MongoDB avec mot de passe en clair

Les identifiants MongoDB doivent venir de l'environnement.
