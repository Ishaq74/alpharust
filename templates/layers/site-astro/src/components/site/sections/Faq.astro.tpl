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

<section class="site-section site-section--faq">
  <div class="site-container site-container--narrow">
    {s.title && (
      <Fragment>
        <div class="site-section__head">
          <div>
            <h2 class="site-section__title">
              {s.title}
            </h2>
            {s.lead && (
              <Fragment>
                <p class="site-lead">
                  {s.lead}
                </p>
              </Fragment>
            )}
          </div>
          {s.link && (
            <Fragment>
              <Button href={s.link.url} variant="ghost">
                {s.link.label}
                <SiteIcon name="arrow-right" size={16} />
              </Button>
            </Fragment>
          )}
        </div>
      </Fragment>
    )}
    <div class="site-faq__list">
      {(s.items ?? []).map((item: any) => (
        <Fragment>
          <details class="site-faq__item" name="faq">
            <summary class="site-faq__question">
              <span>
                {item.question}
              </span>
              <SiteIcon name="chevron-down" size={18} />
            </summary>
            <div class="site-faq__answer">
              <p>
                {item.answer}
              </p>
            </div>
          </details>
        </Fragment>
      ))}
    </div>
  </div>
</section>
