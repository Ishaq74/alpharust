@@meta when=!official@@
---
import { NAV_LINKS, SITE_TITLE } from '../consts';
import ThemeToggle from './ThemeToggle.astro';

const path = Astro.url.pathname;
const isActive = (href: string) => (href === '/' ? path === '/' : path.startsWith(href));
---

<header>
  <div class="bar">
    <a class="brand" href="/">{SITE_TITLE}</a>
    <nav aria-label="Navigation principale">
      <ul>
        {
          NAV_LINKS.map((link) => (
            <li>
              <a href={link.href} aria-current={isActive(link.href) ? 'page' : undefined}>
                {link.label}
              </a>
            </li>
          ))
        }
      </ul>
    </nav>
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
