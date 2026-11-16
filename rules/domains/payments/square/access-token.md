---
name: square-access-token
apply: "*"
severity: error
pattern: "\\b(EAAA[A-Za-z0-9_-]{40,}|sq0(atp|csp)-[A-Za-z0-9_-]{22,})"
examples:
  bad:
    - "token = \"EA¦AAabcdefghijklmnopqrstuvwxyz0123456789ABCDEFGH\""
    - "t = \"sq0¦atp-abcdefghijklmnopqrstuvwx\""
  good:
    - "token = os.environ[\"SQUARE_ACCESS_TOKEN\"]"
---

# Jeton Square en clair

Les jetons d'accès Square (`EAAA…`, `sq0atp-…`) ne doivent pas être versionnés.
