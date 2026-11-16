---
name: github-token
apply: "*"
severity: error
pattern: "\\b(gh[pousr]_[A-Za-z0-9]{36,}|github_pat_[A-Za-z0-9_]{22,})\\b"
examples:
  bad:
    - "token = \"gh¦p_abcdefghijklmnopqrstuvwxyz0123456789\""
  good:
    - "token = os.environ[\"GITHUB_TOKEN\"]"
    - "ghp_court"
---

# Jeton GitHub en clair

Un jeton d'accès GitHub (`ghp_…`, `github_pat_…`) donne accès aux dépôts : le révoquer et utiliser un secret CI.
