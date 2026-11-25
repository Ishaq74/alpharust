@@meta mode=replace when=vitrine@@
// Généré par tools/dsgen/vitrine (spec.py) : même site que Laravel et Wagtail.
import { getCollection } from 'astro:content';
import raw from '../data/site.json';
import fr from '../data/locales/fr.json';

/** Lien normalisé : `current` vaut 'page' (page exacte), 'true' (rubrique) ou null — même règle partout. */
export type Link = { label: string; url: string; current?: 'page' | 'true' | null };
export type NavItem = Link & { children: Link[] | null };
export type Language = { code: string; label: string; name: string; flag: string; url: string; current: 'page' | null };
export type Site = {
  locale: string;
  dir: string;
  languages: Language[];
  multilingual: boolean;
  sheet_side: 'left' | 'right';
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
  account: { url: string; label: string } | null;
  copyright: string;
};
// Les sections sont des objets de contenu libres (voir src/data/site.json) : un type par composant de sections/.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type Section = Record<string, any>;
export type PageKey = keyof typeof fr.pages;
type Data = {
  code: string;
  name: string;
  label: string;
  dir: string;
  flag: string;
  dateLocale: string;
  ogLocale: string;
  order: number;
  site: Record<string, any>;
  pages: Record<string, { url: string; title: string; description: string; sections: Section[] }>;
  contactFields: Record<string, any>[];
  ui: Record<string, string>;
};
type AuthField = { name: string; type: string; autocomplete: string };
const all = raw as unknown as {
  default: string;
  auth: { routes: Record<'sign_in' | 'sign_up' | 'account', string>; fields: Record<string, AuthField[]>; passwordMin: number };
};
// Une langue = un fichier src/data/locales/<langue>.json (langues choisies à la génération : --langs).
const files = import.meta.glob<Data>('../data/locales/*.json', { eager: true, import: 'default' });
const locales = Object.values(files).sort((a, b) => a.order - b.order);
const byCode: Record<string, Data> = Object.fromEntries(locales.map((l) => [l.code, l]));
export const LOCALES = locales.map((l) => l.code);
export const DEFAULT_LOCALE = all.default;

/** Langue d'une adresse : `/en/…` -> en, sinon la langue par défaut (routage i18n d'Astro, prefixDefaultLocale: false). */
export function localeOf(pathname: string): string {
  const first = pathname.split('/')[1] ?? '';
  return LOCALES.includes(first) && first !== DEFAULT_LOCALE ? first : DEFAULT_LOCALE;
}
const data = (locale: string) => byCode[locale] ?? byCode[DEFAULT_LOCALE];
/** Adresse sans préfixe de langue (`/en/blog/` -> `/blog/`). */
function unprefixed(pathname: string): string {
  const loc = localeOf(pathname);
  return loc === DEFAULT_LOCALE ? pathname : pathname.slice(loc.length + 1) || '/';
}

/** Date longue dans la langue (dateLocale du fichier de langue, ex. ar-u-nu-latn : chiffres latins comme Laravel et Django). */
export const formatDate = (d: Date, locale = DEFAULT_LOCALE) =>
  new Intl.DateTimeFormat(data(locale).dateLocale, { dateStyle: 'long', timeZone: 'UTC' }).format(d);
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

/** Article affiché (JSON-LD BlogPosting), mêmes champs qu'en Laravel et Wagtail. */
export type Article = { title: string; excerpt: string; author: string; date_iso: string };

/**
 * Référencement commun aux trois frameworks (Laravel : partials/head ; Wagtail : base.html) : URL canonique absolue,
 * alternatives hreflang (+ x-default), og:locale et JSON-LD schema.org (Organization, BlogPosting sur un article).
 * `base` : Astro.site (astro.config `site`, ou SITE_URL au build).
 */
export function seo(site: Site, pathname: string, base: URL | string, article?: Article) {
  const abs = (p: string) => new URL(p, base).href;
  const d = data(site.locale);
  const s = d.site;
  const canonical = abs(href(pathname, DEFAULT_LOCALE));
  const organization = {
    '@context': 'https://schema.org',
    '@type': 'Organization',
    name: s.name,
    url: abs('/'),
    description: s.description,
    email: s.contact.email,
    telephone: s.contact.phone,
    address: s.contact.address,
    sameAs: s.social.map((x: { url: string }) => x.url),
  };
  const graph: Record<string, unknown>[] = [organization];
  if (article) {
    graph.push({
      '@context': 'https://schema.org',
      '@type': 'BlogPosting',
      headline: article.title,
      description: article.excerpt,
      datePublished: article.date_iso,
      author: { '@type': 'Person', name: article.author },
      inLanguage: site.locale,
      mainEntityOfPage: canonical,
      publisher: { '@type': 'Organization', name: s.name },
    });
  }
  return {
    canonical,
    alternates: site.multilingual
      ? [
          ...site.languages.map((l) => ({ hreflang: l.code, href: abs(l.url) })),
          { hreflang: 'x-default', href: abs(href(unprefixed(pathname), DEFAULT_LOCALE)) },
        ]
      : [],
    siteName: s.name,
    ogLocale: d.ogLocale,
    jsonLd: graph.map((g) => JSON.stringify(g).replace(/</g, '\\u003c')),
  };
}

/** Adresses du plan du site (/sitemap.xml) : pages, services et articles de chaque langue, avec leurs alternatives. */
export async function sitemapEntries() {
  const keys = Object.keys(data(DEFAULT_LOCALE).pages).filter((k) => !['not_found'].includes(k));
  const paths = new Set<string>(keys.map((k) => data(DEFAULT_LOCALE).pages[k].url));
  for (const x of await getServices(DEFAULT_LOCALE)) paths.add(`/services/${x.slug}`);
  for (const x of await getPosts(DEFAULT_LOCALE)) paths.add(`/blog/${x.slug}`);
  return [...paths].flatMap((p) =>
    locales.map((l) => ({
      loc: href(p, l.code),
      alternates: locales.length > 1 ? locales.map((a) => ({ hreflang: a.code, href: href(p, a.code) })) : [],
    })),
  );
}

/** Utilisateur connecté (Astro.locals.user avec --auth) : seuls le nom et l'e-mail servent au site. */
export type SiteUser = { name: string; email: string } | null | undefined;

export async function getSite(pathname: string, user?: SiteUser): Promise<Site> {
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
    dir: d.dir,
    languages: locales.map((l) => ({
      code: l.code,
      label: l.label,
      name: l.name,
      flag: l.flag,
      url: href(rest, l.code),
      current: l.code === locale ? 'page' : null,
    })),
    multilingual: locales.length > 1,
    sheet_side: d.dir === 'rtl' ? 'left' : 'right',
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
@@if auth@@
    account: user
      ? { url: href(all.auth.routes.account, locale), label: d.ui.account }
      : { url: href(all.auth.routes.sign_in, locale), label: d.ui.sign_in },
@@else@@
    account: null,
@@end@@
  };
}

export type AuthKind = 'sign_in' | 'sign_up';
export type AuthState = { error?: string | null; values?: Record<string, string> };

/** Pages connexion / inscription (mêmes données que Laravel et Wagtail) ; le mot de passe n'est jamais ré-affiché. */
export function authPage(kind: AuthKind, state: AuthState = {}, locale = DEFAULT_LOCALE) {
  const ui = data(locale).ui;
  const other: AuthKind = kind === 'sign_in' ? 'sign_up' : 'sign_in';
  return {
    title: ui[kind],
    lead: ui[`${kind}_lead`],
    submit: ui[`${kind}_submit`],
    error: state.error ?? null,
    action: href(all.auth.routes[kind], locale),
    fields: all.auth.fields[kind].map((f) => ({
      ...f,
      id: `auth-${f.name}`,
      label: ui[`field_${f.name}`],
      value: f.type === 'password' ? '' : (state.values?.[f.name] ?? ''),
    })),
    alt_text: ui[kind === 'sign_in' ? 'no_account' : 'have_account'],
    alt_label: ui[other],
    alt_url: href(all.auth.routes[other], locale),
  };
}

/** Page « Mon compte ». */
export function accountPage(user: { name: string; email: string }, locale = DEFAULT_LOCALE) {
  const ui = data(locale).ui;
  return { title: ui.account, lead: ui.account_lead, name: user.name, email: user.email, logout_action: href(all.auth.routes.account, locale) };
}

/** Contrôle des formulaires de compte (mêmes règles que Laravel et Wagtail) : libellé d'erreur ou null. */
export function validateAuth(kind: AuthKind, values: Record<string, string>, locale = DEFAULT_LOCALE): string | null {
  const ui = data(locale).ui;
  const v = (k: string) => (values[k] ?? '').trim();
  if (all.auth.fields[kind].some((f) => !v(f.name))) return ui.err_required;
  if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v('email'))) return ui.invalid_email;
  if (kind === 'sign_up' && (values.password ?? '').length < all.auth.passwordMin) return ui.err_password;
  return null;
}

/** Libellé d'une erreur de compte (`err_credentials`, `err_exists`…). */
export const uiText = (key: string, locale = DEFAULT_LOCALE) => data(locale).ui[key] ?? key;
export const AUTH_ROUTES = all.auth.routes;

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

/** Champs déclarés du formulaire seulement (comme `$request->validate()` de Laravel), 5 000 caractères au plus. */
export function contactData(values: Record<string, string>, locale = DEFAULT_LOCALE) {
  return Object.fromEntries(
    data(locale).contactFields.map((f) => [f.name, (values[f.name] ?? '').trim().slice(0, 5000)]),
  ) as Record<string, string>;
}

export const crumbs = (locale: string, ...items: [string, string][]) => [
  { label: data(locale).ui.home, url: href('/', locale) },
  ...items.map(([label, url]) => ({ label, url: href(url, locale) })),
];
