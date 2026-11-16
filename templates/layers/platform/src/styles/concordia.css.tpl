@@meta when=concordia@@
/*
 * Système CSS Concordia (jetons, base, variantes) — copie fidèle du dépôt Ishaq74/concordia.
 * Les variantes de composants activées au moment de la génération sont importées ci-dessous.
 */
@import './base.css';
@@if css_retro@@
@import './components/retro.css';
@@end@@
@@if css_modern@@
@import './components/modern.css';
@@end@@
@@if css_futuristic@@
@import './components/futuristic.css';
@@end@@

/* Passerelle : les composants AlphaRust (en-tête, pied, cartes) suivent la couleur primaire Concordia. */
:root {
  --accent: var(--color-primary);
}
