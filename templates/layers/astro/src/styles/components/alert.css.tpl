@@meta when=!official&starwind@@
/* Alert (Starwind, src/components/atoms/alert) : le rayon vient de --alert-border-radius (tokens/components.css). */
@layer overrides {
  [data-slot='alert'] {
    border-radius: var(--alert-border-radius);
  }
}
