@@meta when=!official@@
---
import { SITE_TITLE } from '../consts';

const year = new Date().getFullYear();
---

<footer>
  <p>© {year} {SITE_TITLE}. Créé avec <a href="https://astro.build">Astro</a> et AlphaRust.</p>
</footer>

<style>
  footer {
    border-top: 1px solid var(--border);
    color: var(--text-muted);
    text-align: center;
    padding: var(--space-6);
    font-size: 0.9rem;
  }
  footer p {
    margin-inline: auto;
  }
</style>
