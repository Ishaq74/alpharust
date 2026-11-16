@@meta when=i18n@@
---
@@if ssr@@
export const prerender = false;
@@else@@
export function getStaticPaths() {
  return LOCALES.filter((l) => l !== DEFAULT_LOCALE).map((lang) => ({ params: { lang } }));
}
@@end@@
import BaseLayout from '../../layouts/BaseLayout.astro';
import { SITE_TITLE } from '../../consts';
import { DEFAULT_LOCALE, @@if !ssr@@LOCALES, @@end@@isLocale, localizedPath } from '@i18n/config';
import { getMessages } from '@i18n/utils';

// Seules les langues non par défaut sont préfixées (prefixDefaultLocale: false).
if (!isLocale(Astro.params.lang) || Astro.params.lang === DEFAULT_LOCALE) return new Response(null, { status: 404 });
const locale = Astro.currentLocale ?? DEFAULT_LOCALE;
const t = getMessages(locale);
---

<BaseLayout title={SITE_TITLE} description={t.home.description}>
  <section class="hero">
    <h1>{SITE_TITLE}</h1>
    <p class="lead">{t.home.description}</p>
    <div class="actions">
      <a class="button" href="/blog/">{t.home.readBlog}</a>
      <a class="button secondary" href={localizedPath(locale, '/about/')}>{t.home.about}</a>
    </div>
  </section>
</BaseLayout>

<style>
  .hero {
    padding: var(--space-12) var(--space-8);
    border: 1px solid var(--border);
    border-radius: calc(var(--radius) * 2);
    background: linear-gradient(135deg, color-mix(in srgb, var(--brand) 14%, var(--bg)), var(--bg) 70%);
  }
  .lead {
    font-size: 1.25rem;
    color: var(--text-muted);
    margin-bottom: 0;
  }
</style>
