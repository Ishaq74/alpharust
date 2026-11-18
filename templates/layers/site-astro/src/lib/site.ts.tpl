@@meta mode=replace when=vitrine@@
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { getCollection } from 'astro:content';
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
