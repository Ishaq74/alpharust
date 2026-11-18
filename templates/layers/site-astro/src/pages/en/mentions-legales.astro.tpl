@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import BaseLayout from '@layouts/BaseLayout.astro';
import Sections from '@/components/site/Sections.astro';
import { getPage, getSite } from '@lib/site';

const page = await getPage('legal', {}, 'en');
const site = await getSite(Astro.url.pathname);
---

<BaseLayout title={page.title} description={page.description} bleed>
  <Sections sections={page.sections} site={site} />
</BaseLayout>
