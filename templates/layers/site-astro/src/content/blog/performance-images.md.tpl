@@meta when=vitrine@@
---
title: "Des images légères sans perte de qualité"
description: "Formats modernes, dimensions explicites, chargement différé : le trio gagnant."
pubDate: 2026-08-28
category: "Performance"
author: "Camille Martin"
---

Les images représentent souvent plus de la moitié du poids d’une page.

## Les bons formats

AVIF et WebP divisent le poids par deux à cinq par rapport au JPEG, à qualité égale.

## Les bons attributs

- `width` et `height` pour éviter les décalages de mise en page ;
- `loading="lazy"` sous la ligne de flottaison ;
- `srcset` pour servir la bonne taille.
