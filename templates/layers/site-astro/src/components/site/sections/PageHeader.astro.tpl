@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.

import type { Section, Site } from '@lib/site';

interface Props {
  s: Section;
  site: Site;
}

const { s, site } = Astro.props;
void site;
---

<section class="site-page-header">
  <div class="site-container">
    {s.eyebrow && (
      <Fragment>
        <p class="site-eyebrow">
          {s.eyebrow}
        </p>
      </Fragment>
    )}
    <h1 class="site-page-header__title">
      {s.title}
    </h1>
    {s.lead && (
      <Fragment>
        <p class="site-lead">
          {s.lead}
        </p>
      </Fragment>
    )}
  </div>
</section>
