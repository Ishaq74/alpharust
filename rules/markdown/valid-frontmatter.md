---
name: valid-frontmatter
apply: "markdown"
severity: error
checker: markdown-frontmatter
examples:
  bad:
    - "---\nname: x\n# pas fermé\n"
    - "---\na: [1\n---\nbody"
  good:
    - "---\nname: x\n---\n# T\n"
    - "# sans frontmatter\n"
---

# Frontmatter YAML valide requis
