---
name: html-lang-required
apply: "*.html,*.htm,*.astro,*.vue,*.svelte"
severity: warning
checker: html-lang
examples:
  file: "page.html"
  bad:
    - "<html><head></head><body></body></html>"
  good:
    - "<html lang=\"fr\"><head></head><body></body></html>"
    - "<div>fragment sans balise html</div>"
---

# Langue du document requise

La balise `<html>` doit déclarer la langue de la page (`lang="fr"`) pour les lecteurs d'écran et le SEO.
