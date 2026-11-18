@@meta when=!official&starwind@@
/* Dialog (Starwind, src/components/atoms/dialog) : le rayon vient de --dialog-border-radius (tokens/components.css). */
@layer overrides {
  [data-slot='dialog-content'] {
    border-radius: var(--dialog-border-radius);
  }
}
