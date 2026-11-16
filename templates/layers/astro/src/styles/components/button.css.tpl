@@meta when=!official@@
/*
 * Bouton — classe `.button` du gabarit (liens d'action des pages)
@@if starwind@@
 * et surcharge du composant Starwind `Button` (src/components/atoms/button) via son attribut [data-slot="button"].
@@end@@
 * Les valeurs viennent de tokens/components.css : modifiez les jetons, pas les composants.
 */
.button {
  display: inline-block;
  background: var(--brand);
  color: var(--brand-contrast);
  padding: var(--button-padding-y) var(--button-padding-x);
  border-radius: var(--button-border-radius);
  text-decoration: none;
  font-weight: var(--button-font-weight, 600);
  border: 0;
  cursor: pointer;
}

.button:hover {
  filter: brightness(1.1);
}

.button.secondary {
  background: transparent;
  color: var(--text);
  border: 1px solid var(--border);
}

.button.secondary:hover {
  border-color: var(--brand);
  filter: none;
}
@@if starwind@@

/* Composant Starwind : la couche `overrides` passe après les utilitaires de ses classes Tailwind. */
@layer overrides {
  [data-slot='button'] {
    border-radius: var(--button-border-radius);
    letter-spacing: var(--button-letter-spacing);
    text-transform: var(--button-text-transform);
  }
}
@@end@@
