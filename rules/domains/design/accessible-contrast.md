---
name: accessible-contrast
apply: "*.css,*.scss,*.sass,*.html,*.astro,*.vue,*.svelte"
severity: warning
checker: css-contrast
examples:
  bad:
    - ".a { color: #777777; background: #888888; }"
    - ".b {\n  color: #fff;\n  background-color: #ffff00;\n}"
  good:
    - ".a { color: #000; background: #fff; }"
    - ".b { color: var(--fg); background: var(--bg); }"
---

# Contraste accessible

Le contraste texte/fond doit respecter les critères WCAG AA minimum (4.5:1 pour le texte normal).
