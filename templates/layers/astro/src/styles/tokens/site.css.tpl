@@meta when=!official&starwind|concordia@@
/*
 * Pont de jetons — le CONTRAT lu par les composants (components/*.css), le gabarit et le catalogue.
 * Il relie les noms du contrat (--bg, --surface, --brand, --tone-*) aux jetons du kit choisi :
 * pour changer d'identité, modifiez themes/theme.css ou tokens/colors.css, pas les composants.
 */
:root {
@@if concordia@@
  --bg: var(--bg-default);
  --surface: var(--bg-subtle);
  --surface-2: var(--bg-inset);
  --text: var(--text-default);
  --border: var(--border-default);
  --brand: var(--color-primary);
  --brand-text: var(--color-primary-text, var(--color-primary));
  --brand-contrast: var(--button-primary-color);
  --focus: var(--color-primary);
  --radius: var(--border-radius-lg);
  --font-sans: var(--font-family-sans);
  --font-mono: var(--font-family-mono);
  --measure: 68ch;
  --page-width: 64rem;

  --tone-secondary: var(--color-secondary);
  --tone-secondary-contrast: var(--button-secondary-color, #ffffff);
  --tone-info: var(--color-primary);
  --tone-info-contrast: var(--button-primary-color);
  --tone-success: var(--color-success);
  --tone-success-contrast: #111827;
  --tone-warning: var(--color-warning);
  --tone-warning-contrast: #111827;
  --tone-error: var(--color-danger);
  --tone-error-contrast: #ffffff;
@@else@@
  --bg: var(--background);
  --surface: var(--card);
  --surface-2: var(--muted);
  --text: var(--foreground);
  --text-muted: var(--muted-foreground);
  --brand: var(--primary);
  --brand-text: var(--brand-primary-text, var(--primary-deep)); /* texte de marque lisible (themes/theme.css, WCAG) */
  --brand-contrast: var(--primary-foreground);
  --focus: var(--outline);

  --tone-secondary: var(--secondary);
  --tone-secondary-contrast: var(--secondary-foreground);
  --tone-info: var(--info);
  --tone-info-contrast: var(--info-foreground);
  --tone-success: var(--success);
  --tone-success-contrast: var(--success-foreground);
  --tone-warning: var(--warning);
  --tone-warning-contrast: var(--warning-foreground);
  --tone-error: var(--error);
  --tone-error-contrast: var(--error-foreground);
@@end@@
}
