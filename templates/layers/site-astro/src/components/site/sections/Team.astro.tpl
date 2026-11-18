@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Avatar, AvatarFallback } from '@atoms/avatar';
import { Button } from '@atoms/button';
import type { Section, Site } from '@lib/site';

interface Props {
  s: Section;
  site: Site;
}

const { s, site } = Astro.props;
void site;
---

<section class="site-section site-section--team">
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
    <ul class="site-grid site-grid--4 site-team">
      {(s.items ?? []).map((item: any) => (
        <Fragment>
          <li class="site-team__member">
            <Avatar size="lg">
              <AvatarFallback>
                {item.initials}
              </AvatarFallback>
            </Avatar>
            <p class="site-person__name">
              {item.name}
            </p>
            <p class="site-person__role">
              {item.role}
            </p>
          </li>
        </Fragment>
      ))}
    </ul>
  </div>
</section>
