@@meta when=!official&starwind@@
/* Select (Starwind, src/components/atoms/select) : le rayon vient de --select-border-radius (tokens/components.css). */
@layer overrides {
  [data-slot='select-trigger'],
  [data-slot='select-content'] {
    border-radius: var(--select-border-radius);
  }
}
