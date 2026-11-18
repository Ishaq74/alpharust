@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Prose } from '@atoms/prose';
import type { Section, Site } from '@lib/site';

interface Props {
  s: Section;
  site: Site;
}

const { s, site } = Astro.props;
void site;
---

<section class="site-section site-section--text">
  <div class="site-container site-container--narrow">
    {s.title && (
      <Fragment>
        <h2 class="site-section__title">
          {s.title}
        </h2>
      </Fragment>
    )}
    <Prose class="site-prose">
      <Fragment set:html={s.body} />
    </Prose>
  </div>
</section>
