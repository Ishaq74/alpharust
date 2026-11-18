@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import Hero from './sections/Hero.astro';
import PageHeader from './sections/PageHeader.astro';
import Stats from './sections/Stats.astro';
import Features from './sections/Features.astro';
import Services from './sections/Services.astro';
import Testimonials from './sections/Testimonials.astro';
import Posts from './sections/Posts.astro';
import Faq from './sections/Faq.astro';
import Cta from './sections/Cta.astro';
import Text from './sections/Text.astro';
import Team from './sections/Team.astro';
import Contact from './sections/Contact.astro';
import type { Section, Site } from '@lib/site';

interface Props {
  sections: Section[];
  site: Site;
}

const { sections, site } = Astro.props;
const components: Record<string, any> = {
  hero: Hero,
  page_header: PageHeader,
  stats: Stats,
  features: Features,
  services: Services,
  testimonials: Testimonials,
  posts: Posts,
  faq: Faq,
  cta: Cta,
  text: Text,
  team: Team,
  contact: Contact,
};
---

{
  sections.map((s) => {
    const Component = components[s.type];
    return Component ? <Component s={s} site={site} /> : null;
  })
}
