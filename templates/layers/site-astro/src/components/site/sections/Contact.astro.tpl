@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Alert, AlertTitle } from '@atoms/alert';
import { Button } from '@atoms/button';
import { Card, CardContent, CardHeader } from '@atoms/card';
import { Input } from '@atoms/input';
import { Label } from '@atoms/label';
import { Textarea } from '@atoms/textarea';
import SiteIcon from '@/components/site/SiteIcon.astro';
import type { Section, Site } from '@lib/site';

interface Props {
  s: Section;
  site: Site;
}

const { s, site } = Astro.props;
void site;
---

<section class="site-section site-section--contact">
  <div class="site-container">
    <div class="site-contact__grid">
      <Card class="site-card">
        <CardHeader>
          <h2 class="site-card__title">
            {s.title}
          </h2>
        </CardHeader>
        <CardContent>
          {s.sent && (
            <Fragment>
              <Alert variant="success" class="site-form__notice">
                <AlertTitle>
                  {s.success}
                </AlertTitle>
              </Alert>
            </Fragment>
          )}
          <form class="site-form" method="post" action={s.action}>
            {(s.fields ?? []).map((field: any) => (
              <Fragment>
                <div class="site-field">
                  <Label for={field.id}>
                    {field.label}
                    {field.required && (
                      <Fragment>
                        <span class="site-required" aria-hidden="true">
                          *
                        </span>
                      </Fragment>
                    )}
                  </Label>
                  {field.textarea ? (
                    <Fragment>
                      <Textarea id={field.id} name={field.name} rows="6" required={field.required || undefined} aria-invalid={field.invalid || undefined} aria-describedby={field.error_id || undefined} defaultValue={field.value} />
                    </Fragment>
                  ) : (
                    <Fragment>
                      <Input id={field.id} name={field.name} type={field.type} autocomplete={field.autocomplete} value={field.value} required={field.required || undefined} aria-invalid={field.invalid || undefined} aria-describedby={field.error_id || undefined} />
                    </Fragment>
                  )}
                  {field.error && (
                    <Fragment>
                      <p class="site-field__error" id={field.error_id}>
                        {field.error}
                      </p>
                    </Fragment>
                  )}
                </div>
              </Fragment>
            ))}
            <Button type="submit" variant="primary">
              {s.submit}
              <SiteIcon name="send" size={16} />
            </Button>
          </form>
        </CardContent>
      </Card>
      <aside class="site-contact__info">
        <Card class="site-card">
          <CardHeader>
            <h2 class="site-card__title">
              {site.ui.details}
            </h2>
          </CardHeader>
          <CardContent>
            <ul class="site-contact__list">
              <li>
                <SiteIcon name="mail" size={18} />
                <a href={site.contact.email_url}>
                  {site.contact.email}
                </a>
              </li>
              <li>
                <SiteIcon name="phone" size={18} />
                <a href={site.contact.phone_url}>
                  {site.contact.phone}
                </a>
              </li>
              <li>
                <SiteIcon name="map-pin" size={18} />
                <span>
                  {site.contact.address}
                </span>
              </li>
              <li>
                <SiteIcon name="clock" size={18} />
                <span>
                  {site.contact.hours}
                </span>
              </li>
            </ul>
          </CardContent>
        </Card>
      </aside>
    </div>
  </div>
</section>
