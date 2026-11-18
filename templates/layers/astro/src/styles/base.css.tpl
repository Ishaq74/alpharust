@@meta when=!official&!concordia@@
/* Base : remise à zéro minimale et valeurs globales (utilise les jetons de tokens/). */
*,
*::before,
*::after {
  box-sizing: border-box;
}

html {
  font-family: var(--font-sans);
  line-height: 1.6;
  background: var(--bg);
  color: var(--text);
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
}

img {
  max-width: 100%;
  height: auto;
}
@@if tailwind@@

/* Valeurs de départ de Tailwind (reprises d'atomic) : bordures, anneau de focus, curseur. */
@layer base {
  * {
    @apply border-border outline-outline/50;
  }
  body {
    @apply bg-background text-foreground scheme-light dark:scheme-dark;
  }
  button {
    @apply cursor-pointer;
  }
}
@@end@@
