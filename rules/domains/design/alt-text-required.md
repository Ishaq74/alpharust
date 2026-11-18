---
name: alt-text-required
apply: "*.html,*.htm,*.astro,*.vue,*.svelte,*.jsx,*.tsx,*.php,*.twig"
severity: warning
checker: html-img-alt
examples:
  bad:
    - "<img src=\"a.png\">"
    - "<img\n  src=\"a.png\"\n  class=\"x\"\n>"
  good:
    - "<img src=\"a.png\" alt=\"Logo\">"
    - "<img src=\"deco.png\" alt=\"\">"
    - "<img {...props}>"
---

# Texte alternatif requis

Toute balise `<img>` doit posséder un attribut `alt` descriptif.
