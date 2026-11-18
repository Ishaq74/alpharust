@@meta when=!official&!vitrine@@
---
title: 'Structure du projet'
description: 'Où se trouvent les composants, layouts, pages et styles.'
pubDate: 2026-01-15
---

- `src/layouts/BaseLayout.astro` : en-tête HTML, SEO, lien d'évitement, thème.
- `src/components/` : `Header`, `Footer`, `SEO`, `ThemeToggle`.
- `src/pages/` : routes du site.
- `src/styles/` : design system : `global.css` en entrée, puis `tokens/`, `themes/`, `components/`.
