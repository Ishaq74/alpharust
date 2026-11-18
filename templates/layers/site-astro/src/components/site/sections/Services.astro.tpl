@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Badge } from '@atoms/badge';
import { Button } from '@atoms/button';
import { Card, CardContent, CardFooter, CardHeader } from '@atoms/card';
import SiteIcon from '@/components/site/SiteIcon.astro';
import type { Section, Site } from '@lib/site';

interface Props {
  s: Section;
  site: Site;
}

const { s, site } = Astro.props;
void site;
---

<section class="site-section site-section--services">
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
                <SiteIcon name="arrow-right" size={16} />
              </Button>
            </Fragment>
          )}
        </div>
      </Fragment>
    )}
    <div class="site-grid site-grid--3">
      {(s.items ?? []).map((item: any) => (
        <Fragment>
          <Card class="site-card">
            <CardHeader>
              <span class="site-icon-box">
                <SiteIcon name={item.icon} size={22} />
              </span>
              <h3 class="site-card__title">
                <a href={item.url}>
                  {item.title}
                </a>
              </h3>
            </CardHeader>
            <CardContent>
              <p class="site-card__text">
                {item.summary}
              </p>
            </CardContent>
            <CardFooter class="site-card__footer">
              <Badge variant="outline">
                {item.price}
              </Badge>
              <Button href={item.url} variant="outline" size="sm" aria-label={item.more_label}>
                {site.ui.more}
              </Button>
            </CardFooter>
          </Card>
        </Fragment>
      ))}
    </div>
  </div>
</section>
