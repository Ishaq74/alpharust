@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Alert, AlertTitle } from '@atoms/alert';
import { Button } from '@atoms/button';
import { Card, CardContent, CardHeader } from '@atoms/card';
import { Input } from '@atoms/input';
import { Label } from '@atoms/label';
import { Textarea } from '@atoms/textarea';
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
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="m22 2-7 20-4-9-9-4Z"/><path d="M22 2 11 13"/></svg>
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
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/></svg>
                <a href={site.contact.email_url}>
                  {site.contact.email}
                </a>
              </li>
              <li>
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72c.13.96.36 1.9.7 2.81a2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45c.91.34 1.85.57 2.81.7A2 2 0 0 1 22 16.92z"/></svg>
                <a href={site.contact.phone_url}>
                  {site.contact.phone}
                </a>
              </li>
              <li>
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z"/><circle cx="12" cy="10" r="3"/></svg>
                <span>
                  {site.contact.address}
                </span>
              </li>
              <li>
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
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
