@@meta when=!official&starwind@@
/*
 * Jetons de composants — mêmes noms que Concordia (--button-border-radius, --card-padding…).
 * Modifier une valeur ici change tous les composants concernés ; les fichiers components/*.css les lisent.
 */
:root {
  /* Boutons */
  --button-padding-y: var(--space-3);
  --button-padding-x: var(--space-6);
  --button-font-weight: 600;
  --button-border-radius: var(--radius-md);
  --button-letter-spacing: normal;
  --button-text-transform: none;

  /* Cartes */
  --card-padding: var(--space-6);
  --card-border-radius: var(--radius-xl);

  /* Composants Starwind (atoms/) : valeurs par défaut identiques à celles des composants. */
  --input-border-radius: var(--radius-md);
  --textarea-border-radius: var(--radius-md);
  --select-border-radius: var(--radius-md);
  --badge-border-radius: 9999px;
  --alert-border-radius: var(--radius-lg);
  --dialog-border-radius: var(--radius-lg);
  --tabs-border-radius: var(--radius-lg);
  --table-border-radius: var(--radius-lg);
  --tooltip-border-radius: var(--radius-md);
}
