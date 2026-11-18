@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Badge } from '@atoms/badge';
import { Button } from '@atoms/button';
import type { Section, Site } from '@lib/site';

interface Props {
  s: Section;
  site: Site;
}

const { s, site } = Astro.props;
void site;
---

<section class="site-hero">
  <div class="site-container site-hero__inner">
    {s.badge && (
      <Fragment>
        <Badge variant="primary">
          {s.badge}
        </Badge>
      </Fragment>
    )}
    <h1 class="site-hero__title">
      {s.title}
    </h1>
    <p class="site-hero__lead">
      {s.lead}
    </p>
    <div class="site-hero__actions">
      <Button href={s.primary.url} variant="primary" size="lg">
        {s.primary.label}
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
      </Button>
      {s.secondary && (
        <Fragment>
          <Button href={s.secondary.url} variant="outline" size="lg">
            {s.secondary.label}
          </Button>
        </Fragment>
      )}
    </div>
  </div>
</section>
