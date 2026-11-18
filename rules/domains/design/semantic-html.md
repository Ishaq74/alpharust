---
name: semantic-html
apply: "*.html,*.htm,*.astro,*.vue,*.svelte"
severity: warning
checker: html-semantic
examples:
  bad:
    - "<body><div><div>x</div></div></body>"
  good:
    - "<body><header>h</header><main>m</main></body>"
    - "<div>fragment</div>"
---

# HTML sémantique

Utiliser les balises sémantiques (`<header>`, `<nav>`, `<main>`, `<article>`, `<footer>`, etc.) plutôt que des `<div>` génériques.
