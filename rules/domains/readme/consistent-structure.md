---
name: consistent-structure
apply: "/readme*"
severity: info
checker: markdown-title
examples:
  file: "README.md"
  bad:
    - "## Pas de titre H1\n"
    - "# A\n# B\n"
  good:
    - "# Titre\n## Section\n"
---

# Structure de README cohérente
