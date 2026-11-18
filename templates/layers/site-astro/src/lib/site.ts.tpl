@@meta mode=replace when=vitrine@@
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { getCollection } from 'astro:content';
import raw from '../data/site.json';

/** Lien normalisé : `current` vaut 'page' (page exacte), 'true' (rubrique) ou null — même règle partout. */
export type Link = { label: string; url: string; current?: 'page' | 'true' | null };
export type NavItem = Link & { children: Link[] | null };
export type Language = { code: string; label: string; name: string; url: string; current: 'page' | null };
export type Site = {
  locale: string;
  languages: Language[];
  ui: Record<string, string>;
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
export type PageKey = keyof typeof raw.data.fr.pages;
type Data = {
  site: Record<string, any>;
  pages: Record<string, { url: string; title: string; description: string; sections: Section[] }>;
  contactFields: Record<string, any>[];
  ui: Record<string, string>;
};
const all = raw as unknown as { locales: { code: string; name: string; label: string }[]; default: string; data: Record<string, Data> };
export const LOCALES = all.locales.map((l) => l.code);
export const DEFAULT_LOCALE = all.default;

/** Langue d'une adresse : `/en/…` -> en, sinon la langue par défaut (routage i18n d'Astro, prefixDefaultLocale: false). */
export function localeOf(pathname: string): string {
  const first = pathname.split('/')[1] ?? '';
  return LOCALES.includes(first) && first !== DEFAULT_LOCALE ? first : DEFAULT_LOCALE;
}
const data = (locale: string) => all.data[locale] ?? all.data[DEFAULT_LOCALE];
/** Adresse sans préfixe de langue (`/en/blog/` -> `/blog/`). */
function unprefixed(pathname: string): string {
  const loc = localeOf(pathname);
  return loc === DEFAULT_LOCALE ? pathname : pathname.slice(loc.length + 1) || '/';
}

const DATE_LOCALES: Record<string, string> = { fr: 'fr-FR', en: 'en-US' };
export const formatDate = (d: Date, locale = DEFAULT_LOCALE) =>
  new Intl.DateTimeFormat(DATE_LOCALES[locale] ?? locale, { dateStyle: 'long', timeZone: 'UTC' }).format(d);
export const isoDate = (d: Date) => d.toISOString().slice(0, 10);

/** Adresse interne au format Astro (barre finale), préfixée par la langue hors langue par défaut. */
export function href(url: string, locale = DEFAULT_LOCALE): string {
  if (/^[a-z]+:/.test(url)) return url;
  const prefix = locale === DEFAULT_LOCALE ? '' : `/${locale}`;
  if (url === '/') return prefix ? `${prefix}/` : '/';
  return `${prefix}${url.replace(/\/$/, '')}/`;
}

function current(url: string, pathname: string): 'page' | 'true' | null {
  const u = href(url);
  const p = href(pathname);
  if (p === u) return 'page';
  return u !== href('/', localeOf(u)) && p.startsWith(u) ? 'true' : null;
}

/** Entrées d'une langue : src/content/<collection>/<langue>/<slug>.md. */
const inLocale = (id: string, locale: string) => id.startsWith(`${locale}/`);
const slugOf = (id: string) => id.slice(id.indexOf('/') + 1);

export async function getServices(locale = DEFAULT_LOCALE) {
  const entries = (await getCollection('services', (e) => inLocale(e.id, locale))).sort((a, b) => a.data.order - b.data.order);
  return entries.map((e) => ({
    entry: e,
    slug: slugOf(e.id),
    title: e.data.title,
    url: href(`/services/${slugOf(e.id)}`, locale),
    summary: e.data.summary,
    icon: e.data.icon,
    price: e.data.price,
    features: e.data.features,
    more_label: `${data(locale).ui.more_label}${e.data.title}`,
  }));
}

export async function getPosts(locale = DEFAULT_LOCALE) {
  const entries = (await getCollection('blog', (e) => inLocale(e.id, locale))).sort(
    (a, b) => b.data.pubDate.valueOf() - a.data.pubDate.valueOf(),
  );
  return entries.map((e) => ({
    entry: e,
    slug: slugOf(e.id),
    title: e.data.title,
    url: href(`/blog/${slugOf(e.id)}`, locale),
    excerpt: e.data.description,
    category: e.data.category,
    author: e.data.author,
    date_iso: isoDate(e.data.pubDate),
    date_label: formatDate(e.data.pubDate, locale),
  }));
}

export async function getSite(pathname: string): Promise<Site> {
  const locale = localeOf(pathname);
  const d = data(locale);
  const s = d.site;
  const services = await getServices(locale);
  const serviceLinks = services.map((x) => ({ label: x.title, url: x.url, current: current(x.url, pathname) }));
  const links = (v: any): Link[] =>
    v === 'services' ? serviceLinks : (v as Link[]).map((l) => ({ label: l.label, url: href(l.url, locale) }));
  const rest = unprefixed(pathname);
  return {
    locale,
    languages: all.locales.map((l) => ({
      code: l.code,
      label: l.label,
      name: l.name,
      url: href(rest, l.code),
      current: l.code === locale ? 'page' : null,
    })),
    ui: d.ui,
    name: s.name,
    initial: s.name.charAt(0).toUpperCase(),
    tagline: s.tagline,
    description: s.description,
    home_url: href('/', locale),
    cta: { label: s.cta.label, url: href(s.cta.url, locale) },
    nav: s.nav.map((i: any) => ({
      label: i.label,
      url: href(i.url, locale),
      current: current(href(i.url, locale), pathname),
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
export async function getPage(key: PageKey, state: ContactState = {}, locale = DEFAULT_LOCALE) {
  const d = data(locale);
  const page = d.pages[key];
  const services = await getServices(locale);
  const posts = await getPosts(locale);
  const sections = page.sections.map((s) => {
    const link = s.link ? { label: s.link.label, url: href(s.link.url, locale) } : null;
    const fix = (a: any) => (a ? { label: a.label, url: href(a.url, locale) } : null);
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
          action: href(page.url, locale),
          sent: Boolean(state.sent),
          fields: d.contactFields.map((f) => {
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
export function validateContact(values: Record<string, string>, locale = DEFAULT_LOCALE) {
  const d = data(locale);
  const errors: Record<string, string> = {};
  for (const f of d.contactFields) {
    const v = (values[f.name] ?? '').trim();
    if (f.required && !v) errors[f.name] = d.ui.required;
    else if (f.type === 'email' && v && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v)) errors[f.name] = d.ui.invalid_email;
  }
  return errors;
}

export const crumbs = (locale: string, ...items: [string, string][]) => [
  { label: data(locale).ui.home, url: href('/', locale) },
  ...items.map(([label, url]) => ({ label, url: href(url, locale) })),
];
