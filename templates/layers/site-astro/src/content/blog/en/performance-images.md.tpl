@@meta when=vitrine&lang_en@@
---
title: "Lightweight images without quality loss"
description: "Modern formats, explicit dimensions, lazy loading: the winning trio."
pubDate: 2026-08-28
category: "Performance"
author: "Camille Martin"
---

Images often account for more than half of a page’s weight.

## The right formats

AVIF and WebP cut the weight by two to five compared with JPEG, at equal quality.

## The right attributes

- `width` and `height` to avoid layout shifts;
- `loading="lazy"` below the fold;
- `srcset` to serve the right size.
