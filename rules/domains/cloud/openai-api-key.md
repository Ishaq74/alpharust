---
name: openai-api-key
apply: "*"
severity: error
pattern: "\\bsk-(proj-|svcacct-)?[A-Za-z0-9_-]{32,}\\b"
examples:
  bad:
    - "OPENAI = \"s¦k-proj-abcdefghijklmnopqrstuvwxyz0123456789\""
  good:
    - "OPENAI = os.environ[\"OPENAI_API_KEY\"]"
    - "sk-court"
---

# Clé d'API OpenAI en clair

Une clé `sk-…` doit être lue depuis l'environnement.
