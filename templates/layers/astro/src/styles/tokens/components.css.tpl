@@meta when=!official&!concordia@@
/*
 * Jetons de composants — mêmes noms que Concordia (--button-border-radius, --card-padding…).
 * Modifier une valeur ici change tous les composants concernés ; les fichiers components/*.css les lisent.
 */
:root {
  /* Boutons */
  --button-padding-y: var(--space-3);
  --button-padding-x: var(--space-6);
  --button-font-weight: 600;
@@if starwind@@
  --button-border-radius: var(--radius-md);
  --button-letter-spacing: normal;
  --button-text-transform: none;
@@else@@
  --button-border-radius: var(--radius);
@@end@@

  /* Cartes */
  --card-padding: var(--space-6);
@@if starwind@@
  --card-border-radius: var(--radius-xl);
@@else@@
  --card-border-radius: var(--radius);
@@end@@
@@if starwind@@

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
@@end@@
}
