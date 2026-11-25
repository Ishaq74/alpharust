@@meta mode=replace when=vitrine&lang_en@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import BaseLayout from '@layouts/BaseLayout.astro';
import Sections from '@/components/site/Sections.astro';
import { type ContactState, contactData, getPage, getSite, validateContact } from '@lib/site';
import { saveContactMessage } from '@lib/contact-store';

// Rendu à la demande, même dans un site statique (adaptateur Node) : le formulaire est validé puis le message
// est enregistré (comme Laravel et Wagtail) ; Astro vérifie l'origine des POST (security.checkOrigin).
export const prerender = false;

const locale = 'en';
let state: ContactState = {};
if (Astro.request.method === 'POST') {
  const form = await Astro.request.formData();
  const values = Object.fromEntries([...form.entries()].map(([k, v]) => [k, String(v)]));
  const errors = validateContact(values, locale);
  if (Object.keys(errors).length) {
    state = { values, errors };
    Astro.response.status = 422;
  } else {
    await saveContactMessage(contactData(values, locale));
    state = { sent: true };
  }
}
const page = await getPage('contact', state, locale);
const site = await getSite(Astro.url.pathname);
---

<BaseLayout title={page.title} description={page.description} bleed>
  <Sections sections={page.sections} site={site} />
</BaseLayout>
