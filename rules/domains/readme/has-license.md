---
name: has-license
apply: "/readme*"
severity: warning
checker: readme-sections
args:
  any: ["license", "licence"]
  message: "section « License » manquante"
examples:
  file: "README.md"
  bad:
    - "# X\n## Usage\n"
  good:
    - "# X\n## License\n"
    - "# X\n## Licence\n"
---

# Licence requise

Le README doit mentionner la licence du projet.
