---
name: no-inline-styles
apply: "*.html,*.htm,*.astro,*.vue,*.svelte,*.jsx,*.tsx,*.php,*.twig"
severity: warning
pattern: "<[a-zA-Z][^>]*\\sstyle\\s*="
examples:
  bad:
    - "<div style=\"color:red\">x</div>"
    - "<p class=\"a\" style={{margin: 0}}>"
  good:
    - "<div class=\"a\">x</div>"
    - "const style = 1;"
---

# Pas de styles inline

Les styles doivent être externalisés (CSS, modules, Tailwind, etc.).
Les attributs `style=` sont interdits sauf cas exceptionnel justifié.
