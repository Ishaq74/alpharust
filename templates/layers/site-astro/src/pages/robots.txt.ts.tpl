@@meta mode=replace when=vitrine@@
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
// robots.txt servi par le site (comme Laravel et Wagtail) : tout est indexable, plan du site en adresse absolue.
import type { APIRoute } from 'astro';

export const GET: APIRoute = ({ site, url }) => {
  const sitemap = new URL('/sitemap.xml', site ?? url).href;
  return new Response(`User-agent: *\nAllow: /\n\nSitemap: ${sitemap}\n`, {
    headers: { 'Content-Type': 'text/plain; charset=utf-8' },
  });
};
