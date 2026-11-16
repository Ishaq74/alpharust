---
name: planetscale-token
apply: "*"
severity: error
pattern: "\\bpscale_(pw|tkn|oauth)_[A-Za-z0-9_-]{16,}"
examples:
  bad:
    - "PW = \"pscale_¦pw_abcdefghijklmnopqrstuvwxyz\""
  good:
    - "PW = os.environ[\"PLANETSCALE_PASSWORD\"]"
---

# Jeton PlanetScale en clair

Les mots de passe et jetons PlanetScale (`pscale_pw_…`) ne doivent pas être versionnés.
