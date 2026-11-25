@@meta mode=replace when=vitrine&lang_ar@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Badge } from '@atoms/badge';
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from '@atoms/breadcrumb';
import { Button } from '@atoms/button';
import { Prose } from '@atoms/prose';
import SiteIcon from '@/components/site/SiteIcon.astro';
import { render } from 'astro:content';
import BaseLayout from '@layouts/BaseLayout.astro';
import { crumbs, getPosts, getSite, href } from '@lib/site';
void href;
const locale = 'ar';

@@if ssr@@
// Rendu serveur : l'entrée est résolue à la demande.
export const prerender = false;
const item = (await getPosts(locale)).find((x) => x.slug === Astro.params.slug);
if (!item) return Astro.rewrite('/404');
@@else@@
export async function getStaticPaths() {
  return (await getPosts('ar')).map((item) => ({ params: { slug: item.slug }, props: { id: item.entry.id } }));
}
const item = (await getPosts(locale)).find((x) => x.entry.id === Astro.props.id)!;
@@end@@
const { Content } = await render(item.entry);
const site = await getSite(Astro.url.pathname);
const page = {
  ...item,
  crumbs: crumbs(locale, [site.ui.blog, '/blog']),
  back_url: href('/blog', locale),
};
---

<BaseLayout title={page.title} description={item.excerpt} bleed>
<article class="site-article">
  <header class="site-page-header">
    <div class="site-container site-container--narrow">
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
      <Badge variant="outline">
        {page.category}
      </Badge>
      <h1 class="site-page-header__title">
        {page.title}
      </h1>
      <p class="site-meta">
        <SiteIcon name="calendar" size={16} />
        <time datetime={page.date_iso}>
          {page.date_label}
        </time>
        <span aria-hidden="true">
          ·
        </span>
        <span>
          {page.author}
        </span>
      </p>
    </div>
  </header>
  <div class="site-section">
    <div class="site-container site-container--narrow">
      <Prose class="site-prose">
        <Content />
      </Prose>
      <p class="site-article__back">
        <Button href={page.back_url} variant="outline">
          <SiteIcon name="arrow-left" size={16} />
          {site.ui.all_posts}
        </Button>
      </p>
    </div>
  </div>
</article>
</BaseLayout>
