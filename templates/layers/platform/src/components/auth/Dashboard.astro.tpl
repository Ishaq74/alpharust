@@meta when=auth@@
---
import AuthShell from './AuthShell.astro';
import { DEFAULT_LOCALE@@if plugin_admin@@, localizedPath@@end@@ } from '@i18n/config';
import { getMessages } from '@i18n/utils';
@@if starwind@@
import { Button } from '@atoms/button';
@@end@@
@@if concordia@@
import Button from '@components/ui/Button.astro';
@@end@@

const user = Astro.locals.user;
const session = Astro.locals.session;
const locale = Astro.currentLocale ?? DEFAULT_LOCALE;
const t = getMessages(locale);
const role = (user as { role?: string | null } | null)?.role ?? t.auth.dashboard.defaultRole;
const until = session ? new Intl.DateTimeFormat(locale, { dateStyle: 'long', timeStyle: 'short' }).format(session.expiresAt) : '';
---

<AuthShell title={t.auth.dashboard.title}>
  <p class="hello">{t.auth.dashboard.welcome} <strong>{user?.name}</strong></p>
  <dl>
    <dt>{t.auth.dashboard.email}</dt>
    <dd>{user?.email}</dd>
    <dt>{t.auth.dashboard.role}</dt>
    <dd>{role}</dd>
    <dt>{t.auth.dashboard.session}</dt>
    <dd><time datetime={session?.expiresAt.toISOString()}>{until}</time></dd>
  </dl>
@@if plugin_admin@@
  {role === 'admin' && <p><a href={localizedPath(locale, '/admin')}>{t.nav.admin}</a></p>}
@@end@@
  <form method="post">
@@if starwind@@
    <Button type="submit" variant="outline">{t.nav.signOut}</Button>
@@else@@
@@if concordia@@
    <Button type="submit">{t.nav.signOut}</Button>
@@else@@
    <button type="submit" class="signout">{t.nav.signOut}</button>
@@end@@
@@end@@
  </form>
</AuthShell>

<style>
  .hello {
    margin: 0 0 var(--space-4);
  }
  dl {
    display: grid;
    grid-template-columns: max-content 1fr;
    gap: var(--space-2) var(--space-4);
    margin: 0 0 var(--space-4);
  }
  dt {
    color: var(--text-muted);
  }
  dd {
    margin: 0;
    overflow-wrap: anywhere;
  }
@@if !starwind&!concordia@@
  .signout {
    font: inherit;
    padding: 0.5rem 1rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    background: transparent;
    color: var(--text);
    cursor: pointer;
  }
@@end@@
</style>
