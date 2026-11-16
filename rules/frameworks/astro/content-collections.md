---
name: astro-content-collections
apply: "*"
severity: info
checker: require-glob
args:
  any: ["src/content.config.*", "src/content/config.*"]
  when_any: ["astro.config.*"]
  message: "projet Astro sans content collections"
---

# Content collections recommandées
