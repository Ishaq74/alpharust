---
name: has-installation
apply: "/readme*"
severity: warning
checker: readme-sections
args:
  any: ["install"]
  message: "section « Installation » manquante"
examples:
  file: "README.md"
  bad:
    - "# X\n## Usage\n"
  good:
    - "# X\n## Installation\n"
    - "# X\n## Installation rapide\n"
---

# Section Installation requise

Le README doit contenir une section claire expliquant comment installer le projet.
