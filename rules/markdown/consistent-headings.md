---
name: consistent-headings
apply: "markdown"
severity: info
checker: markdown-headings
examples:
  bad:
    - "# A\n### C\n"
  good:
    - "# A\n## B\n```\n### code\n```\n"
---

# Headings cohérents
