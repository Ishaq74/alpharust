@@meta mode=replace when=vitrine&lang_es@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import BaseLayout from '@layouts/BaseLayout.astro';
import Sections from '@/components/site/Sections.astro';
import { type ContactState, getPage, getSite, validateContact } from '@lib/site';

// Rendu à la demande, même dans un site statique (adaptateur Node) : le formulaire est validé ici
// (Astro vérifie l'origine des POST : security.checkOrigin).
export const prerender = false;

const locale = 'es';
let state: ContactState = {};
if (Astro.request.method === 'POST') {
  const form = await Astro.request.formData();
  const values = Object.fromEntries([...form.entries()].map(([k, v]) => [k, String(v)]));
  const errors = validateContact(values, locale);
  state = Object.keys(errors).length ? { values, errors } : { sent: true };
  if (Object.keys(errors).length) Astro.response.status = 422;
}
const page = await getPage('contact', state, locale);
const site = await getSite(Astro.url.pathname);
---

<BaseLayout title={page.title} description={page.description} bleed>
  <Sections sections={page.sections} site={site} />
</BaseLayout>
