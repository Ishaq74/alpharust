@@meta when=!official&starwind@@
/* Badge (Starwind, src/components/atoms/badge) : le rayon vient de --badge-border-radius (tokens/components.css). */
@layer overrides {
  [data-slot='badge'] {
    border-radius: var(--badge-border-radius);
  }
}
