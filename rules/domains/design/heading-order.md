---
name: heading-order
apply: "*.html,*.htm,*.astro,*.vue,*.svelte"
severity: info
checker: html-heading-order
examples:
  file: "page.html"
  bad:
    - "<h1>Titre</h1><h3>Saut de niveau</h3>"
    - "<h1>Un</h1><h1>Deux</h1>"
  good:
    - "<h1>Titre</h1><h2>Section</h2><h3>Sous-section</h3>"
---

# Ordre des titres

Un seul `<h1>` par page et aucun niveau de titre sauté (`h1` → `h3`).
