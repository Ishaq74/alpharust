@@meta mode=replace when=vitrine@@
---
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { Badge } from '@atoms/badge';
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from '@atoms/breadcrumb';
import { Button } from '@atoms/button';
import { Prose } from '@atoms/prose';
import { render } from 'astro:content';
import BaseLayout from '@layouts/BaseLayout.astro';
import { crumbs, getPosts, getSite, href } from '@lib/site';
void href;

@@if ssr@@
// Rendu serveur : l'entrée est résolue à la demande.
export const prerender = false;
const item = (await getPosts()).find((x) => x.entry.id === Astro.params.slug);
if (!item) return Astro.rewrite('/404');
@@else@@
export async function getStaticPaths() {
  return (await getPosts()).map((item) => ({ params: { slug: item.entry.id }, props: { id: item.entry.id } }));
}
const item = (await getPosts()).find((x) => x.entry.id === Astro.props.id)!;
@@end@@
const { Content } = await render(item.entry);
const page = {
  ...item,
  crumbs: crumbs(['Blog', '/blog']),
  back_url: href('/blog'),
};
const site = await getSite(Astro.url.pathname);
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
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><rect width="18" height="18" x="3" y="4" rx="2"/><path d="M16 2v4M8 2v4M3 10h18"/></svg>
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
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
          Tous les articles
        </Button>
      </p>
    </div>
  </div>
</article>
</BaseLayout>
