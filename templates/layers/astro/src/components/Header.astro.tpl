@@meta when=!official@@
---
import { SITE_TITLE@@if !messages@@, NAV_LINKS@@end@@ } from '../consts';
import ThemeToggle from './ThemeToggle.astro';
@@if messages@@
import { DEFAULT_LOCALE, localizedPath } from '@i18n/config';
import { getMessages } from '@i18n/utils';
@@end@@
@@if i18n@@
import LanguageSwitcher from './LanguageSwitcher.astro';
@@end@@

const path = Astro.url.pathname;
const isActive = (href: string) => (href.length <= 4 && href.endsWith('/') ? path === href : path.startsWith(href));
@@if messages@@
const locale = Astro.currentLocale ?? DEFAULT_LOCALE;
const t = getMessages(locale);
const home = localizedPath(locale, '/');
const links: { href: string; label: string }[] = [
  { href: home, label: t.nav.home },
  { href: '/blog/', label: t.nav.blog },
  { href: localizedPath(locale, '/about/'), label: t.nav.about },
];
@@if auth@@
const user = Astro.locals.user;
if (user) {
  links.push({ href: localizedPath(locale, '/auth/dashboard'), label: t.nav.dashboard });
} else {
  links.push({ href: localizedPath(locale, '/auth/sign-in'), label: t.nav.signIn });
  links.push({ href: localizedPath(locale, '/auth/sign-up'), label: t.nav.signUp });
}
@@end@@
@@else@@
const home = '/';
const links = NAV_LINKS.map((l) => ({ href: l.href as string, label: l.label as string }));
@@end@@
---

<header>
  <div class="bar">
    <a class="brand" href={home}>{SITE_TITLE}</a>
    <nav aria-label="Navigation principale">
      <ul>
        {
          links.map((link) => (
            <li>
              <a href={link.href} aria-current={isActive(link.href) ? 'page' : undefined}>
                {link.label}
              </a>
            </li>
          ))
        }
      </ul>
    </nav>
@@if i18n@@
    <LanguageSwitcher />
@@end@@
    <ThemeToggle />
  </div>
</header>

<style>
  header {
    border-bottom: 1px solid var(--border);
    background: var(--bg);
  }
  .bar {
    width: min(100% - 2 * var(--space-6), var(--page-width));
    margin-inline: auto;
    display: flex;
    align-items: center;
    gap: var(--space-6);
    padding-block: var(--space-3);
  }
  .brand {
    font-weight: 700;
    font-size: 1.15rem;
    color: var(--text);
    text-decoration: none;
    margin-right: auto;
  }
  ul {
    display: flex;
    gap: var(--space-4);
    list-style: none;
    margin: 0;
    padding: 0;
  }
  nav a {
    color: var(--text-muted);
    text-decoration: none;
    padding: var(--space-1) 0;
    border-bottom: 2px solid transparent;
  }
  nav a:hover,
  nav a[aria-current='page'] {
    color: var(--text);
    border-bottom-color: var(--accent);
  }
</style>
