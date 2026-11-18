@@meta when=!official@@
/* Mise en page du gabarit : corps de page, rythme vertical, liens, citations, accessibilité. */
body {
  margin: 0;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

main {
  flex: 1;
  width: min(100% - 2 * var(--space-6), var(--page-width));
  margin-inline: auto;
  padding-block: var(--space-8);
}

h1,
h2,
h3 {
  line-height: var(--line-height-heading, 1.2);
  text-wrap: balance;
}

h1 {
  font-size: clamp(2rem, 5vw, 3rem);
  margin-block: 0 var(--space-4);
}

p,
li {
  max-width: var(--measure);
}

a {
  color: var(--brand-text);
  text-underline-offset: 0.2em;
}

a:hover {
  text-decoration-thickness: 2px;
}

:focus-visible {
  outline: 3px solid var(--focus);
  outline-offset: 3px;
  border-radius: 4px;
}

code {
  font-family: var(--font-mono);
  background: var(--surface);
  padding: 0.1em 0.35em;
  border-radius: 0.35rem;
}

/* Rythme vertical : reste correct même si Tailwind (preflight) remet les marges à zéro. */
main :where(h2) {
  font-size: 1.5rem;
  margin-block: var(--space-12) var(--space-4);
}

main :where(h3) {
  font-size: 1.2rem;
  margin-block: var(--space-6) var(--space-2);
}

main :where(section > h2:first-child, article > h1 + p + h2) {
  margin-top: 0;
}

main :where(p, ul, ol, pre, blockquote, table, figure) {
  margin-block: 0 var(--space-4);
}

main :where(ul:not([class]), ol:not([class])) {
  padding-inline-start: 1.5rem;
  list-style: revert;
}

main :where(section + section) {
  margin-top: var(--space-12);
}

pre {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: var(--space-4);
  overflow-x: auto;
  line-height: 1.5;
}

pre code {
  background: none;
  padding: 0;
}

blockquote {
  border-inline-start: 4px solid var(--brand);
  padding-inline-start: var(--space-4);
  color: var(--text-muted);
}

hr {
  border: 0;
  border-top: 1px solid var(--border);
  margin-block: var(--space-8);
}

.meta {
  color: var(--text-muted);
  font-size: 0.95rem;
}

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-3);
  margin-top: var(--space-6);
}

.skip-link {
  position: absolute;
  left: var(--space-4);
  top: -4rem;
  background: var(--brand);
  color: var(--brand-contrast);
  padding: var(--space-2) var(--space-4);
  border-radius: var(--radius);
  z-index: 10;
}

.skip-link:focus {
  top: var(--space-4);
}

.visually-hidden {
  position: absolute;
  width: 1px;
  height: 1px;
  overflow: hidden;
  clip: rect(0 0 0 0);
  white-space: nowrap;
}

@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms;
    animation-iteration-count: 1;
    transition-duration: 0.01ms;
    scroll-behavior: auto;
  }
}
@@if concordia@@

/* Lien d'action `.button` des pages (le composant [data-slot='button'] est dans components/button.css). */
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
@@end@@
