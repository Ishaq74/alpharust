---
name: aws-access-key-id
apply: "*"
severity: error
pattern: "\\b(AKIA|ASIA|AGPA|AIDA|AROA)[0-9A-Z]{16}\\b"
examples:
  bad:
    - "key = \"AKIA¦ABCDEFGHIJKLMNOP\""
  good:
    - "key = os.environ[\"AWS_ACCESS_KEY_ID\"]"
    - "AKIA_PLACEHOLDER"
---

# Clé d'accès AWS en clair

Un identifiant de clé d'accès AWS (`AKIA…`) ne doit jamais apparaître dans le code. Utiliser un rôle IAM ou des variables d'environnement.
