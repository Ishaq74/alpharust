---
name: has-usage
apply: "/readme*"
severity: warning
checker: readme-sections
args:
  any: ["usage", "utilisation"]
  message: "section « Usage » manquante"
examples:
  file: "README.md"
  bad:
    - "# X\n## Installation\n"
  good:
    - "# X\n## Usage\n"
    - "# X\n## Utilisation\n"
---

# Section Usage requise

Le README doit contenir une section expliquant comment utiliser le projet.
