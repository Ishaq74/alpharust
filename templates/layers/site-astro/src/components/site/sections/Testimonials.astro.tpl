@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Avatar, AvatarFallback } from '@atoms/avatar';
import { Button } from '@atoms/button';
import { Card, CardContent, CardFooter } from '@atoms/card';
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
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
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
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M16 3a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2 1 1 0 0 1 1 1v1a2 2 0 0 1-2 2 1 1 0 0 0-1 1v2a1 1 0 0 0 1 1 6 6 0 0 0 6-6V5a2 2 0 0 0-2-2z"/><path d="M5 3a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2 1 1 0 0 1 1 1v1a2 2 0 0 1-2 2 1 1 0 0 0-1 1v2a1 1 0 0 0 1 1 6 6 0 0 0 6-6V5a2 2 0 0 0-2-2z"/></svg>
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
