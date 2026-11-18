"""Rendu Astro : layout + composants + pages + collections de contenu (couche site-astro)."""
import json
from . import data as D, templates as TPL
from .ir import Astro
from .icons import ICONS

META = '@@meta mode=replace when=vitrine@@\n'
HEAD = '// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.\n'

def comp(cat, nodes, props, extra_imports='', script=''):
    r = Astro(cat)
    body = r.nodes(nodes)
    imports = r.imports().replace("'@/components/site/SiteIcon.astro'", "'@/components/site/SiteIcon.astro'")
    fm = f"---\n{HEAD}{imports}\n{extra_imports}{props}{script}---\n\n"
    return META + fm + body

SITE_PROPS = "import type { Site } from '@lib/site';\n\ninterface Props {\n  site: Site;\n}\n\nconst { site } = Astro.props;\n"
SECTION_PROPS = ("import type { Section, Site } from '@lib/site';\n\ninterface Props {\n  s: Section;\n  site: Site;\n}\n\n"
                 "const { s, site } = Astro.props;\nvoid site;\n")

def site_json():
    out = {
        'site': {k: D.SITE[k] for k in ('name', 'tagline', 'description', 'cta', 'contact', 'social', 'nav', 'footer', 'copyright')},
        'pages': D.PAGES,
        'contactFields': D.FIELDS,
    }
    return META + json.dumps(out, ensure_ascii=False, indent=2) + '\n'

LIB = META + HEAD + r"""import { getCollection } from 'astro:content';
import raw from '../data/site.json';

/** Lien normalisé : `current` vaut 'page' (page exacte), 'true' (rubrique) ou null — même règle partout. */
export type Link = { label: string; url: string; current?: 'page' | 'true' | null };
export type NavItem = Link & { children: Link[] | null };
export type Site = {
  name: string;
  initial: string;
  tagline: string;
  description: string;
  home_url: string;
  cta: Link;
  nav: NavItem[];
  footer: { title: string; links: Link[] }[];
  social: { label: string; url: string; icon: string }[];
  contact: Record<string, string>;
  year: number;
  copyright_line: string;
  copyright: string;
};
// Les sections sont des objets de contenu libres (voir src/data/site.json) : un type par composant de sections/.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type Section = Record<string, any>;
export type PageKey = keyof typeof raw.pages;

const data = raw as unknown as {
  site: Record<string, any>;
  pages: Record<string, { url: string; title: string; description: string; sections: Section[] }>;
  contactFields: Record<string, any>[];
};

const dateFormat = new Intl.DateTimeFormat('fr-FR', { dateStyle: 'long', timeZone: 'UTC' });
export const formatDate = (d: Date) => dateFormat.format(d);
export const isoDate = (d: Date) => d.toISOString().slice(0, 10);

/** Adresse interne au format Astro (barre finale). */
export function href(url: string): string {
  if (url === '/' || /^[a-z]+:/.test(url)) return url;
  return `${url.replace(/\/$/, '')}/`;
}

function current(url: string, pathname: string): 'page' | 'true' | null {
  const u = href(url);
  const p = href(pathname);
  if (p === u) return 'page';
  return u !== '/' && p.startsWith(u) ? 'true' : null;
}

export async function getServices() {
  const entries = (await getCollection('services')).sort((a, b) => a.data.order - b.data.order);
  return entries.map((e) => ({
    entry: e,
    title: e.data.title,
    url: href(`/services/${e.id}`),
    summary: e.data.summary,
    icon: e.data.icon,
    price: e.data.price,
    features: e.data.features,
    more_label: `En savoir plus : ${e.data.title}`,
  }));
}

export async function getPosts() {
  const entries = (await getCollection('blog')).sort((a, b) => b.data.pubDate.valueOf() - a.data.pubDate.valueOf());
  return entries.map((e) => ({
    entry: e,
    title: e.data.title,
    url: href(`/blog/${e.id}`),
    excerpt: e.data.description,
    category: e.data.category,
    author: e.data.author,
    date_iso: isoDate(e.data.pubDate),
    date_label: formatDate(e.data.pubDate),
  }));
}

export async function getSite(pathname: string): Promise<Site> {
  const s = data.site;
  const services = await getServices();
  const serviceLinks = services.map((x) => ({ label: x.title, url: x.url, current: current(x.url, pathname) }));
  const links = (v: any): Link[] =>
    v === 'services' ? serviceLinks : (v as Link[]).map((l) => ({ label: l.label, url: href(l.url) }));
  return {
    name: s.name,
    initial: s.name.charAt(0).toUpperCase(),
    tagline: s.tagline,
    description: s.description,
    home_url: '/',
    cta: { label: s.cta.label, url: href(s.cta.url) },
    nav: s.nav.map((i: any) => ({
      label: i.label,
      url: href(i.url),
      current: current(i.url, pathname),
      children: i.children ? links(i.children) : null,
    })),
    footer: s.footer.map((c: any) => ({ title: c.title, links: links(c.links) })),
    social: s.social,
    contact: {
      ...s.contact,
      email_url: `mailto:${s.contact.email}`,
      phone_url: `tel:${s.contact.phone.replace(/\s/g, '')}`,
    },
    year: new Date().getFullYear(),
    copyright: s.copyright,
    copyright_line: `© ${new Date().getFullYear()} ${s.name} · ${s.copyright}`,
  };
}

export type ContactState = { sent?: boolean; values?: Record<string, string>; errors?: Record<string, string> };

/** Sections d'une page, listes (services, articles) et formulaire résolus. */
export async function getPage(key: PageKey, state: ContactState = {}) {
  const page = data.pages[key];
  const services = await getServices();
  const posts = await getPosts();
  const sections = page.sections.map((s) => {
    const link = s.link ? { label: s.link.label, url: href(s.link.url) } : null;
    const fix = (a: any) => (a ? { label: a.label, url: href(a.url) } : null);
    switch (s.type) {
      case 'services':
        return { ...s, link, items: s.limit ? services.slice(0, s.limit) : services };
      case 'posts':
        return { ...s, link, items: s.limit ? posts.slice(0, s.limit) : posts };
      case 'hero':
        return { ...s, primary: fix(s.primary), secondary: fix(s.secondary) };
      case 'cta':
        return { ...s, action: fix(s.action) };
      case 'contact':
        return {
          ...s,
          action: href(page.url),
          sent: Boolean(state.sent),
          fields: data.contactFields.map((f) => {
            const error = state.errors?.[f.name] ?? null;
            const id = `contact-${f.name}`;
            return {
              ...f,
              id,
              textarea: f.type === 'textarea',
              value: state.sent ? '' : (state.values?.[f.name] ?? ''),
              error,
              invalid: error ? 'true' : null,
              error_id: error ? `${id}-error` : null,
            };
          }),
        };
      default:
        return { ...s, link };
    }
  });
  return { ...page, sections };
}

/** Validation du formulaire de contact (mêmes règles que Laravel et Wagtail). */
export function validateContact(values: Record<string, string>) {
  const errors: Record<string, string> = {};
  for (const f of data.contactFields) {
    const v = (values[f.name] ?? '').trim();
    if (f.required && !v) errors[f.name] = 'Ce champ est obligatoire.';
    else if (f.type === 'email' && v && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v)) errors[f.name] = 'Adresse e-mail invalide.';
  }
  return errors;
}

export const crumbs = (...items: [string, string][]) => [
  { label: 'Accueil', url: '/' },
  ...items.map(([label, url]) => ({ label, url: href(url) })),
];
"""

def icon_component():
    entries = ',\n'.join(f"  {json.dumps(k)}: '{v}'" for k, v in ICONS.items())
    return (META + "---\n" + HEAD + "// Tracés Lucide (ISC). Les mêmes icônes existent en Blade (<x-site.icon>) et Django ({% site_icon %}).\n"
            f"const icons: Record<string, string> = {{\n{entries},\n}};\n\n"
            "interface Props {\n  name: string;\n  size?: number;\n}\n\nconst { name, size = 20 } = Astro.props;\n---\n\n"
            '<svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" '
            'stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon" set:html={icons[name] ?? \'\'} />\n')

def pascal(t): return ''.join(w.capitalize() for w in t.split('_'))

def sections_component():
    imports = ''.join(f"import {pascal(t)} from './sections/{pascal(t)}.astro';\n" for t in TPL.SECTIONS)
    mapping = ',\n'.join(f'  {t}: {pascal(t)}' for t in TPL.SECTIONS)
    return (META + "---\n" + HEAD + imports + "import type { Section, Site } from '@lib/site';\n\n"
            "interface Props {\n  sections: Section[];\n  site: Site;\n}\n\nconst { sections, site } = Astro.props;\n"
            f"const components: Record<string, any> = {{\n{mapping},\n}};\n---\n\n"
            "{\n  sections.map((s) => {\n    const Component = components[s.type];\n"
            "    return Component ? <Component s={s} site={site} /> : null;\n  })\n}\n")

def layout():
    return META + """---
""" + HEAD + """// Gabarit commun : en-tête, pied de page, SEO, thème — le même que resources/views/components/layouts/site.blade.php
// (Laravel) et templates/base.html (Wagtail).
import '../styles/global.css';
import '../styles/site.css';
import SEO from '../components/SEO.astro';
import SiteFooter from '../components/site/SiteFooter.astro';
import SiteHeader from '../components/site/SiteHeader.astro';
import { getSite } from '@lib/site';
@@if messages@@
import { DEFAULT_LOCALE, dirOf } from '@i18n/config';
@@end@@

interface Props {
  title: string;
  description?: string;
  /** Pages du site (sections pleine largeur) ; sinon le contenu est centré (catalogue, compte…). */
  bleed?: boolean;
}

const { title, description, bleed = false } = Astro.props;
const site = await getSite(Astro.url.pathname);
@@if messages@@
const locale = Astro.currentLocale ?? DEFAULT_LOCALE;
@@end@@
---

<!doctype html>
@@if messages@@
<html lang={locale} dir={dirOf(locale)}>
@@else@@
<html lang="fr">
@@end@@
  <head>
    <SEO title={title} description={description ?? site.description} />
    <script is:inline>
      // Applique le thème choisi avant le premier rendu (évite le flash).
      try {
        const saved = localStorage.getItem('theme');
        const theme =
          saved === 'light' || saved === 'dark'
            ? saved
            : window.matchMedia('(prefers-color-scheme: dark)').matches
              ? 'dark'
              : 'light';
        document.documentElement.dataset.theme = theme;
        document.documentElement.classList.toggle('dark', theme === 'dark');
      } catch {}
    </script>
  </head>
  <body class="site-body">
    <a class="site-skip" href="#contenu">Aller au contenu</a>
    <SiteHeader site={site} />
    <main id="contenu" class:list={['site-main', { 'site-main--page': !bleed }]} tabindex="-1">
      <slot />
    </main>
    <SiteFooter site={site} />
@@if concordia@@
    <script>
      // Comportements du design kit (onglets, dialogue, accordéon…) : même script que Laravel et Wagtail.
      import '../scripts/ds.js';
    </script>
@@end@@
  </body>
</html>
"""

def simple_page(key, depth=0):
    up = '../' * (depth + 1)
    return META + f"""---
{HEAD}import BaseLayout from '@layouts/BaseLayout.astro';
import Sections from '@/components/site/Sections.astro';
import {{ getPage, getSite }} from '@lib/site';

const page = await getPage('{key}');
const site = await getSite(Astro.url.pathname);
---

<BaseLayout title={{page.title}} description={{page.description}} bleed>
  <Sections sections={{page.sections}} site={{site}} />
</BaseLayout>
"""

def home_page():
    return META + f"""---
{HEAD}import BaseLayout from '@layouts/BaseLayout.astro';
import Sections from '@/components/site/Sections.astro';
import {{ getPage, getSite }} from '@lib/site';

const page = await getPage('home');
const site = await getSite(Astro.url.pathname);
---

<BaseLayout title={{site.name}} description={{page.description}} bleed>
  <Sections sections={{page.sections}} site={{site}} />
</BaseLayout>
"""

def contact_page():
    return META + f"""---
{HEAD}import BaseLayout from '@layouts/BaseLayout.astro';
import Sections from '@/components/site/Sections.astro';
import {{ type ContactState, getPage, getSite, validateContact }} from '@lib/site';

// Rendu à la demande, même dans un site statique (adaptateur Node) : le formulaire est validé ici
// (Astro vérifie l'origine des POST : security.checkOrigin).
export const prerender = false;

let state: ContactState = {{}};
if (Astro.request.method === 'POST') {{
  const form = await Astro.request.formData();
  const values = Object.fromEntries([...form.entries()].map(([k, v]) => [k, String(v)]));
  const errors = validateContact(values);
  state = Object.keys(errors).length ? {{ values, errors }} : {{ sent: true }};
  if (Object.keys(errors).length) Astro.response.status = 422;
}}
const page = await getPage('contact', state);
const site = await getSite(Astro.url.pathname);
---

<BaseLayout title={{page.title}} description={{page.description}} bleed>
  <Sections sections={{page.sections}} site={{site}} />
</BaseLayout>
"""

def detail_page(cat, kind):
    """services/[slug].astro et blog/[...slug].astro : rendu serveur (ssr) ou pages statiques."""
    coll = 'services' if kind == 'service' else 'blog'
    nodes = TPL.SERVICE_DETAIL if kind == 'service' else TPL.POST_DETAIL
    r = Astro(cat)
    body = r.nodes(nodes)
    getter = 'getServices' if kind == 'service' else 'getPosts'
    if kind == 'service':
        page_expr = """{
  ...item,
  crumbs: crumbs(['Services', '/services']),
}"""
        desc = 'item.summary'
    else:
        page_expr = """{
  ...item,
  crumbs: crumbs(['Blog', '/blog']),
  back_url: href('/blog'),
}"""
        desc = 'item.excerpt'
    return META + f"""---
{HEAD}{r.imports()}
import {{ render }} from 'astro:content';
import BaseLayout from '@layouts/BaseLayout.astro';
import {{ crumbs, {getter}, getSite, href }} from '@lib/site';
void href;

@@if ssr@@
// Rendu serveur : l'entrée est résolue à la demande.
export const prerender = false;
const item = (await {getter}()).find((x) => x.entry.id === Astro.params.slug);
if (!item) return Astro.rewrite('/404');
@@else@@
export async function getStaticPaths() {{
  return (await {getter}()).map((item) => ({{ params: {{ slug: item.entry.id }}, props: {{ id: item.entry.id }} }}));
}}
const item = (await {getter}()).find((x) => x.entry.id === Astro.props.id)!;
@@end@@
const {{ Content }} = await render(item.entry);
const page = {page_expr};
const site = await getSite(Astro.url.pathname);
---

<BaseLayout title={{page.title}} description={{{desc}}} bleed>
{body}</BaseLayout>
"""

def not_found(cat):
    r = Astro(cat)
    body = r.nodes(TPL.NOT_FOUND)
    return META + f"""---
{HEAD}{r.imports()}
import BaseLayout from '@layouts/BaseLayout.astro';
import {{ getSite }} from '@lib/site';

const site = await getSite(Astro.url.pathname);
---

<BaseLayout title="Page introuvable" bleed>
{body}</BaseLayout>
"""

CONTENT_CONFIG = META + """import { defineCollection } from 'astro:content';
import { glob } from 'astro/loaders';
import { z } from 'astro/zod';

// Articles (src/content/blog) et services (src/content/services) : mêmes champs que les modèles Laravel
// (Post, Service) et les pages Wagtail (BlogPage, ServicePage).
const blog = defineCollection({
@@if mdx@@
  loader: glob({ base: './src/content/blog', pattern: '**/*.{md,mdx}' }),
@@else@@
  loader: glob({ base: './src/content/blog', pattern: '**/*.md' }),
@@end@@
  schema: z.object({
    title: z.string(),
    description: z.string(),
    pubDate: z.coerce.date(),
    updatedDate: z.coerce.date().optional(),
    category: z.string().default('Actualités'),
    author: z.string().default('La rédaction'),
  }),
});

const services = defineCollection({
  loader: glob({ base: './src/content/services', pattern: '**/*.md' }),
  schema: z.object({
    title: z.string(),
    summary: z.string(),
    icon: z.string(),
    price: z.string(),
    order: z.number(),
    features: z.array(z.string()),
  }),
});

export const collections = { blog, services };
"""

def yaml_str(s): return json.dumps(s, ensure_ascii=False)

def content_files():
    files = {}
    for p in D.POSTS:
        files[f"src/content/blog/{p['slug']}.md.tpl"] = (
            "@@meta when=vitrine@@\n---\n"
            f"title: {yaml_str(p['title'])}\ndescription: {yaml_str(p['excerpt'])}\npubDate: {p['date']}\n"
            f"category: {yaml_str(p['category'])}\nauthor: {yaml_str(p['author'])}\n---\n\n{p['body_md'].strip()}\n")
    for s in D.SERVICES:
        feats = ''.join(f"\n  - {yaml_str(f)}" for f in s['features'])
        files[f"src/content/services/{s['slug']}.md.tpl"] = (
            "@@meta when=vitrine@@\n---\n"
            f"title: {yaml_str(s['title'])}\nsummary: {yaml_str(s['summary'])}\nicon: {s['icon']}\nprice: {yaml_str(s['price'])}\n"
            f"order: {s['order']}\nfeatures:{feats}\n---\n\n{s['body_md'].strip()}\n")
    return files

def files(cat, css):
    out = {
        'src/data/site.json.tpl': site_json(),
        'src/lib/site.ts.tpl': LIB,
        'src/content.config.ts.tpl': CONTENT_CONFIG,
        'src/layouts/BaseLayout.astro.tpl': layout(),
        'src/components/site/SiteHeader.astro.tpl': comp(cat, TPL.HEADER, SITE_PROPS),
        'src/components/site/SiteFooter.astro.tpl': comp(cat, TPL.FOOTER, SITE_PROPS),
        'src/components/site/SiteIcon.astro.tpl': icon_component(),
        'src/components/site/Sections.astro.tpl': sections_component(),
        'src/pages/index.astro.tpl': home_page(),
        'src/pages/a-propos.astro.tpl': simple_page('about'),
        'src/pages/mentions-legales.astro.tpl': simple_page('legal'),
        'src/pages/confidentialite.astro.tpl': simple_page('privacy'),
        'src/pages/services/index.astro.tpl': simple_page('services'),
        'src/pages/blog/index.astro.tpl': simple_page('blog'),
        'src/pages/contact.astro.tpl': contact_page(),
        'src/pages/services/[slug].astro.tpl': detail_page(cat, 'service'),
        'src/pages/blog/[...slug].astro.tpl': detail_page(cat, 'post'),
        'src/pages/404.astro.tpl': not_found(cat),
        'src/styles/site.css.tpl': META + css,
    }
    for t, nodes in TPL.SECTIONS.items():
        out[f'src/components/site/sections/{pascal(t)}.astro.tpl'] = comp(cat, nodes, SECTION_PROPS)
    out.update(content_files())
    return out
