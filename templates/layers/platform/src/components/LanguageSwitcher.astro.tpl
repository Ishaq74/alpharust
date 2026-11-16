@@meta when=i18n@@
---
import { DEFAULT_LOCALE, LOCALES, LOCALE_LABELS, localizedPath } from '@i18n/config';
import { getMessages } from '@i18n/utils';

const current = Astro.currentLocale ?? DEFAULT_LOCALE;
const t = getMessages(current);

/** Chemin courant sans préfixe de langue. */
function unprefixed(pathname: string): string {
  for (const l of LOCALES) {
    if (l !== DEFAULT_LOCALE && (pathname === `/${l}` || pathname.startsWith(`/${l}/`))) {
      return pathname.slice(l.length + 1) || '/';
    }
  }
  return pathname;
}

const base = unprefixed(Astro.url.pathname);
// Le blog n'est disponible que dans la langue par défaut : on renvoie vers l'accueil de la langue choisie.
const target = base.startsWith('/blog') ? '/' : base;
---

<nav class="lang" aria-label={t.nav.language}>
  <ul>
    {
      LOCALES.map((l) => (
        <li>
          <a
            href={localizedPath(l, target)}
            hreflang={l}
            lang={l}
            aria-current={l === current ? 'true' : undefined}
          >
            {LOCALE_LABELS[l] ?? l}
          </a>
        </li>
      ))
    }
  </ul>
</nav>

<style>
  .lang ul {
    display: flex;
    gap: var(--space-2);
    list-style: none;
    margin: 0;
    padding: 0;
  }
  .lang a {
    font-size: 0.85rem;
    text-decoration: none;
    color: var(--text-muted);
    padding: var(--space-1) var(--space-2);
    border-radius: 0.4rem;
  }
  .lang a[aria-current='true'] {
    color: var(--text);
    background: var(--surface);
    font-weight: 600;
  }
</style>
