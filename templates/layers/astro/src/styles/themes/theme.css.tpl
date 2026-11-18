@@meta when=!official@@
/*
 * Palette de marque. Générée par `alpharust generate astro … --theme-primary #… --theme-secondary #… --theme-accent #…`
 * (contrôle WCAG : textes lisibles, variante sombre) ; `npm run theme:check` rejoue le contrôle.
 *
 * Variables lues par les jetons (tokens/colors.css) :
@@if concordia@@
 *   --color-primary / --color-secondary / --color-accent   et   --on-primary… (texte lisible)
@@else@@
 *   --brand-primary / --brand-secondary / --brand-accent   et   --on-brand-primary… (texte lisible)
@@end@@
 *
 * Exemple (à décommenter) :
@@if concordia@@
 *   :root { --color-primary: #0ea5e9; }
 *   :root[data-theme='dark'] { --color-primary: #7dd3fc; }
@@else@@
 *   :root { --brand-primary: #0ea5e9; --on-brand-primary: #0b1220; }
 *   :root[data-theme='dark'], .dark { --brand-primary: #7dd3fc; }
@@end@@
 */
