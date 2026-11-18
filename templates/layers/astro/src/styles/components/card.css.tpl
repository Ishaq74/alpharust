@@meta when=!official&!concordia@@
/*
 * Carte — classe `.card` du gabarit (liste d'articles)
@@if starwind@@
 * et surcharge du composant Starwind `Card` (src/components/atoms/card) via [data-slot="card"].
@@end@@
 */
.card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--card-border-radius);
  padding: var(--card-padding);
  transition:
    border-color 0.15s ease,
    transform 0.15s ease;
}

.card:hover {
  border-color: var(--brand);
  transform: translateY(-2px);
}
@@if starwind@@

@layer overrides {
  [data-slot='card'] {
    border-radius: var(--card-border-radius);
  }
}
@@end@@
