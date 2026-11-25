@@meta mode=replace when=vitrine@@
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
// Plan du site XML (protocole sitemaps.org, alternatives hreflang xhtml:link) : mêmes adresses qu'en Laravel et Wagtail.
import type { APIRoute } from 'astro';
import { sitemapEntries } from '@lib/site';

const esc = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;');

export const GET: APIRoute = async ({ site, url }) => {
  const abs = (p: string) => esc(new URL(p, site ?? url).href);
  const body = (await sitemapEntries())
    .map(
      (e) =>
        `  <url>\n    <loc>${abs(e.loc)}</loc>\n` +
        e.alternates.map((a) => `    <xhtml:link rel="alternate" hreflang="${a.hreflang}" href="${abs(a.href)}"/>\n`).join('') +
        '  </url>\n',
    )
    .join('');
  return new Response(
    `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">\n${body}</urlset>\n`,
    { headers: { 'Content-Type': 'application/xml; charset=utf-8' } },
  );
};
