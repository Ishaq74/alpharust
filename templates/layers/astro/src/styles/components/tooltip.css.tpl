@@meta when=!official&starwind@@
/* Tooltip (Starwind, src/components/atoms/tooltip) : le rayon vient de --tooltip-border-radius (tokens/components.css). */
@layer overrides {
  [data-slot='tooltip-content'] {
    border-radius: var(--tooltip-border-radius);
  }
}
