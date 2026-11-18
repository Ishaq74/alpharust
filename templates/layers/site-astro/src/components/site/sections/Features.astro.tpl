@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Button } from '@atoms/button';
import { Card, CardContent, CardHeader } from '@atoms/card';
import SiteIcon from '@/components/site/SiteIcon.astro';
import type { Section, Site } from '@lib/site';

interface Props {
  s: Section;
  site: Site;
}

const { s, site } = Astro.props;
void site;
---

<section class="site-section site-section--features">
  <div class="site-container">
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
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
              </Button>
            </Fragment>
          )}
        </div>
      </Fragment>
    )}
    <div class="site-grid site-grid--4">
      {(s.items ?? []).map((item: any) => (
        <Fragment>
          <Card class="site-card">
            <CardHeader>
              <span class="site-icon-box">
                <SiteIcon name={item.icon} size={22} />
              </span>
              <h3 class="site-card__title">
                {item.title}
              </h3>
            </CardHeader>
            <CardContent>
              <p class="site-card__text">
                {item.text}
              </p>
            </CardContent>
          </Card>
        </Fragment>
      ))}
    </div>
  </div>
</section>
