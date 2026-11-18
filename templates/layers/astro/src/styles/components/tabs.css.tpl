@@meta when=!official&starwind@@
/* Tabs (Starwind, src/components/atoms/tabs) : le rayon vient de --tabs-border-radius (tokens/components.css). */
@layer overrides {
  [data-slot='tabs-list'] {
    border-radius: var(--tabs-border-radius);
  }
}
