@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Button } from '@atoms/button';
import SiteIcon from '@/components/site/SiteIcon.astro';
import type { Section, Site } from '@lib/site';

interface Props {
  s: Section;
  site: Site;
}

const { s, site } = Astro.props;
void site;
---

<section class="site-section site-section--cta">
  <div class="site-container">
    <div class="site-cta">
      <div>
        <h2 class="site-cta__title">
          {s.title}
        </h2>
        <p class="site-cta__text">
          {s.text}
        </p>
      </div>
      <Button href={s.action.url} variant="default" size="lg">
        {s.action.label}
        <SiteIcon name="arrow-right" size={18} />
      </Button>
    </div>
  </div>
</section>
