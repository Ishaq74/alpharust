---
name: no-inline-styles
apply: "*"
severity: warning
pattern: "style\\s*="
---

# Pas de styles inline

Les styles doivent être externalisés (CSS, modules, Tailwind, etc.).
Les attributs `style=` sont interdits sauf cas exceptionnel justifié.
