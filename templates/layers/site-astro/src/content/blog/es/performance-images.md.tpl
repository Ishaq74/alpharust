@@meta when=vitrine&lang_es@@
---
title: "Imágenes ligeras sin perder calidad"
description: "Formatos modernos, dimensiones explícitas y carga diferida: el trío ganador."
pubDate: 2026-08-28
category: "Rendimiento"
author: "Camille Martin"
---

Las imágenes suelen representar más de la mitad del peso de una página.

## Los formatos adecuados

AVIF y WebP dividen el peso entre dos y cinco respecto al JPEG, con la misma calidad.

## Los atributos adecuados

- `width` y `height` para evitar desplazamientos del diseño;
- `loading="lazy"` por debajo de la línea de flotación;
- `srcset` para servir el tamaño adecuado.
