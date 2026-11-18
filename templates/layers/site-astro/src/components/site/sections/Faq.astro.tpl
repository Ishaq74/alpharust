@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '@atoms/accordion';
import { Button } from '@atoms/button';
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
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
              </Button>
            </Fragment>
          )}
        </div>
      </Fragment>
    )}
    <Accordion type="single" class="site-faq__list">
      {(s.items ?? []).map((item: any) => (
        <Fragment>
          <AccordionItem value={item.question}>
            <AccordionTrigger>
              {item.question}
            </AccordionTrigger>
            <AccordionContent>
              <p>
                {item.answer}
              </p>
            </AccordionContent>
          </AccordionItem>
        </Fragment>
      ))}
    </Accordion>
  </div>
</section>
