@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Badge } from '@atoms/badge';
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from '@atoms/breadcrumb';
import { Button } from '@atoms/button';
import { Card, CardContent, CardFooter, CardHeader } from '@atoms/card';
import { Prose } from '@atoms/prose';
import SiteIcon from '@/components/site/SiteIcon.astro';
import { render } from 'astro:content';
import BaseLayout from '@layouts/BaseLayout.astro';
import { crumbs, getServices, getSite, href } from '@lib/site';
void href;

@@if ssr@@
// Rendu serveur : l'entrée est résolue à la demande.
export const prerender = false;
const item = (await getServices()).find((x) => x.entry.id === Astro.params.slug);
if (!item) return Astro.rewrite('/404');
@@else@@
export async function getStaticPaths() {
  return (await getServices()).map((item) => ({ params: { slug: item.entry.id }, props: { id: item.entry.id } }));
}
const item = (await getServices()).find((x) => x.entry.id === Astro.props.id)!;
@@end@@
const { Content } = await render(item.entry);
const page = {
  ...item,
  crumbs: crumbs(['Services', '/services']),
};
const site = await getSite(Astro.url.pathname);
---

<BaseLayout title={page.title} description={item.summary} bleed>
<section class="site-page-header">
  <div class="site-container">
    <Breadcrumb>
      <BreadcrumbList>
        {(page.crumbs ?? []).map((crumb: any) => (
          <Fragment>
            <BreadcrumbItem>
              <BreadcrumbLink href={crumb.url}>
                {crumb.label}
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
          </Fragment>
        ))}
        <BreadcrumbItem>
          <BreadcrumbPage>
            {page.title}
          </BreadcrumbPage>
        </BreadcrumbItem>
      </BreadcrumbList>
    </Breadcrumb>
    <span class="site-icon-box site-icon-box--lg">
      <SiteIcon name={page.icon} size={28} />
    </span>
    <h1 class="site-page-header__title">
      {page.title}
    </h1>
    <p class="site-lead">
      {page.summary}
    </p>
    <Badge variant="primary" size="lg">
      {page.price}
    </Badge>
  </div>
</section>
<section class="site-section">
  <div class="site-container site-detail">
    <Prose class="site-prose site-detail__body">
      <Content />
    </Prose>
    <aside class="site-detail__aside">
      <Card class="site-card">
        <CardHeader>
          <h2 class="site-card__title">
            Ce qui est inclus
          </h2>
        </CardHeader>
        <CardContent>
          <ul class="site-checklist">
            {(page.features ?? []).map((feature: any) => (
              <Fragment>
                <li>
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M20 6 9 17l-5-5"/></svg>
                  <span>
                    {feature}
                  </span>
                </li>
              </Fragment>
            ))}
          </ul>
        </CardContent>
        <CardFooter>
          <Button href={site.cta.url} variant="primary" class="site-block">
            {site.cta.label}
          </Button>
        </CardFooter>
      </Card>
    </aside>
  </div>
</section>
</BaseLayout>
