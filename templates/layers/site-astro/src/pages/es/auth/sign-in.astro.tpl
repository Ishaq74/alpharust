@@meta mode=replace when=vitrine&auth&lang_es@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Alert, AlertTitle } from '@atoms/alert';
import { Button } from '@atoms/button';
import { Card, CardContent, CardFooter, CardHeader } from '@atoms/card';
import { Input } from '@atoms/input';
import { Label } from '@atoms/label';
import BaseLayout from '@layouts/BaseLayout.astro';
import { AUTH_ROUTES, type AuthState, authPage, getSite, href, uiText, validateAuth } from '@lib/site';
import { signIn } from '@lib/site-auth';

// Formulaire traité côté serveur (Better Auth) ; Astro vérifie l'origine des POST (security.checkOrigin).
export const prerender = false;
const locale = 'es';
if (Astro.locals.user) return Astro.redirect(href(AUTH_ROUTES.account, locale));
let state: AuthState = {};
if (Astro.request.method === 'POST') {
  const form = await Astro.request.formData();
  const values = Object.fromEntries([...form.entries()].map(([k, v]) => [k, String(v)]));
  let error = validateAuth('sign_in', values, locale);
  if (!error) {
    const result = await signIn(Astro, values, href(AUTH_ROUTES.account, locale));
    if (result.response) return result.response;
    error = uiText(result.error ?? 'err_generic', locale);
  }
  state = { error, values };
  Astro.response.status = 422;
}
const page = authPage('sign_in', state, locale);
const site = await getSite(Astro.url.pathname, Astro.locals.user);
---

<BaseLayout title={page.title} bleed>
<section class="site-section site-auth">
  <div class="site-container site-container--narrow">
    <Card class="site-card site-auth__card">
      <CardHeader>
        <h1 class="site-auth__title">
          {page.title}
        </h1>
        <p class="site-lead">
          {page.lead}
        </p>
      </CardHeader>
      <CardContent>
        {page.error && (
          <Fragment>
            <Alert variant="error" class="site-form__notice">
              <AlertTitle>
                {page.error}
              </AlertTitle>
            </Alert>
          </Fragment>
        )}
        <form class="site-form" method="post" action={page.action}>
          <input type="hidden" name="locale" value={site.locale} />
          {(page.fields ?? []).map((field: any) => (
            <Fragment>
              <div class="site-field">
                <Label for={field.id}>
                  {field.label}
                </Label>
                <Input id={field.id} name={field.name} type={field.type} autocomplete={field.autocomplete} value={field.value} required />
              </div>
            </Fragment>
          ))}
          <Button type="submit" variant="primary" class="site-block">
            {page.submit}
          </Button>
        </form>
      </CardContent>
      <CardFooter class="site-auth__alt">
        <span>
          {page.alt_text}
        </span>
        <a href={page.alt_url}>
          {page.alt_label}
        </a>
      </CardFooter>
    </Card>
  </div>
</section>
</BaseLayout>
