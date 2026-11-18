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
import { DEFAULT_LOCALE, @@if !ssr@@LOCALES, @@end@@isLocale } from '@i18n/config';
import { getMessages } from '@i18n/utils';

if (!isLocale(Astro.params.lang) || Astro.params.lang === DEFAULT_LOCALE) return new Response(null, { status: 404 });
const t = getMessages(Astro.currentLocale);
---

<BaseLayout title={t.about.title} description={t.about.body}>
  <h1>{t.about.title}</h1>
  <p>{t.about.body}</p>
</BaseLayout>
