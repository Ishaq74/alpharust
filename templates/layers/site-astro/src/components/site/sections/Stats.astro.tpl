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

<section class="site-section site-section--stats">
  <div class="site-container">
    <dl class="site-stats__grid">
      {(s.items ?? []).map((item: any) => (
        <Fragment>
          <div class="site-stat">
            <dt class="site-stat__label">
              {item.label}
            </dt>
            <dd class="site-stat__value">
              {item.value}
            </dd>
          </div>
        </Fragment>
      ))}
    </dl>
  </div>
</section>
