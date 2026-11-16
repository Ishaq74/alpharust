@@meta when=!official&tailwind@@
@@if starwind@@
/* Tailwind : compléments aux jetons Starwind (dégradés d'atomic, couleurs de marque). */
@theme inline {
  --color-primary-deep: var(--primary-deep);
  --color-gradient-warm: var(--gradient-warm);
  --color-gradient-cool: var(--gradient-cool);
  --color-brand-secondary: var(--brand-secondary, var(--secondary));
  --color-brand-accent: var(--brand-accent, var(--accent));
}
@@else@@
/* Tailwind : variante `dark:` pilotée par data-theme, et jetons exposés (bg-primary, text-foreground, border-border…) via @theme. */
@custom-variant dark (&:where([data-theme='dark'], [data-theme='dark'] *));

@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-card: var(--card);
  --color-card-foreground: var(--card-foreground);
  --color-muted: var(--muted);
  --color-muted-foreground: var(--muted-foreground);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
  --color-border: var(--border);
  --color-outline: var(--outline);
  --color-brand-secondary: var(--brand-secondary, var(--muted));
  --color-brand-accent: var(--brand-accent, var(--primary));
  --font-sans: var(--font-sans);
  --font-mono: var(--font-mono);
}
@@end@@
