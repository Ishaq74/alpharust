@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Avatar, AvatarFallback } from '@atoms/avatar';
import { Button } from '@atoms/button';
import { Card, CardContent, CardFooter } from '@atoms/card';
import SiteIcon from '@/components/site/SiteIcon.astro';
import type { Section, Site } from '@lib/site';

interface Props {
  s: Section;
  site: Site;
}

const { s, site } = Astro.props;
void site;
---

<section class="site-section site-section--testimonials">
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
          <Card class="site-card site-quote">
            <CardContent>
              <SiteIcon name="quote" size={24} />
              <blockquote class="site-quote__text">
                <p>
                  {item.quote}
                </p>
              </blockquote>
            </CardContent>
            <CardFooter class="site-person">
              <Avatar size="md">
                <AvatarFallback>
                  {item.initials}
                </AvatarFallback>
              </Avatar>
              <div>
                <p class="site-person__name">
                  {item.name}
                </p>
                <p class="site-person__role">
                  {item.role}
                </p>
              </div>
            </CardFooter>
          </Card>
        </Fragment>
      ))}
    </div>
  </div>
</section>
