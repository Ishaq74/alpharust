---
name: redis-connection-string
apply: "*"
severity: error
pattern: "rediss?://[^/\\s:@'\\\"$]*:[^@\\s'\\\"$]+@"
examples:
  bad:
    - "REDIS_URL = \"redis://default:secret@cache:6379\""
  good:
    - "REDIS_URL = \"rediss://cache:6379\""
    - "REDIS_URL = os.environ[\"REDIS_URL\"]"
---

# URL Redis avec mot de passe en clair

Les mots de passe Redis doivent venir de l'environnement.
