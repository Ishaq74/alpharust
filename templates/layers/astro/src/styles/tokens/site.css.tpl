@@meta when=!official@@
/*
 * Variables du gabarit AlphaRust (en-tête, pied, pages) : un pont vers les jetons du kit choisi,
 * pour que le site suive vos jetons et votre thème sans rien réécrire.
 */
:root {
@@if concordia@@
  --bg: var(--bg-default);
  --surface: var(--bg-subtle);
  --text: var(--text-default);
  --border: var(--border-default);
  --brand: var(--color-primary);
  --brand-text: var(--color-primary);
  --brand-contrast: var(--button-primary-color);
  --focus: var(--color-primary);
  --radius: var(--border-radius-lg);
  --font-sans: var(--font-family-sans);
  --font-mono: var(--font-family-mono);
  --measure: 68ch;
  --page-width: 64rem;
@@else@@
  --bg: var(--background);
  --surface: var(--card);
  --text: var(--foreground);
  --text-muted: var(--muted-foreground);
  --brand: var(--primary);
@@if starwind@@
  --brand-text: var(--primary-deep); /* lisible sur fond clair (atomic) */
@@else@@
  --brand-text: var(--primary);
@@end@@
  --brand-contrast: var(--primary-foreground);
  --focus: var(--outline);
@@end@@
}
