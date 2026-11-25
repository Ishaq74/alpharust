@@meta mode=replace when=vitrine&auth@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Button } from '@atoms/button';
import { Card, CardContent, CardFooter, CardHeader } from '@atoms/card';
import SiteIcon from '@/components/site/SiteIcon.astro';
import BaseLayout from '@layouts/BaseLayout.astro';
import { AUTH_ROUTES, accountPage, getSite, href } from '@lib/site';
import { signOut } from '@lib/site-auth';

export const prerender = false;
const locale = 'fr';
const user = Astro.locals.user;
if (!user) return Astro.redirect(href(AUTH_ROUTES.sign_in, locale));
// Déconnexion : formulaire POST de cette page.
if (Astro.request.method === 'POST') return signOut(Astro, href('/', locale));
const page = accountPage(user, locale);
const site = await getSite(Astro.url.pathname, user);
---

<BaseLayout title={page.title} bleed>
<section class="site-section site-auth">
  <div class="site-container site-container--narrow">
    <Card class="site-card site-auth__card">
      <CardHeader>
        <span class="site-icon-box">
          <SiteIcon name="user" size={22} />
        </span>
        <h1 class="site-auth__title">
          {page.title}
        </h1>
        <p class="site-lead">
          {page.lead}
        </p>
      </CardHeader>
      <CardContent>
        <dl class="site-auth__details">
          <dt>
            {site.ui.field_name}
          </dt>
          <dd>
            {page.name}
          </dd>
          <dt>
            {site.ui.field_email}
          </dt>
          <dd>
            {page.email}
          </dd>
        </dl>
      </CardContent>
      <CardFooter>
        <form method="post" action={page.logout_action}>
          <input type="hidden" name="locale" value={site.locale} />
          <Button type="submit" variant="outline">
            <SiteIcon name="log-out" size={16} />
            {site.ui.sign_out}
          </Button>
        </form>
      </CardFooter>
    </Card>
  </div>
</section>
</BaseLayout>
