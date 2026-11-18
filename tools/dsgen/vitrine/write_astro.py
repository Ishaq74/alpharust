"""Rendu Astro : layout + composants + pages + collections de contenu (couche site-astro)."""
import json
from . import data as D, templates as TPL, i18n as I18N
from .ir import Astro
from .icons import icons_json

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

def locale_meta(loc):
    m = D.META[loc]
    return {'code': loc, 'name': m['name'], 'label': m['label'], 'dir': m['dir'], 'flag': f'flag-{loc}',
            'dateLocale': m['date_intl'], 'order': m['order']}

def site_json():
    # Données communes ; les textes de chaque langue sont dans src/data/locales/<langue>.json (une langue = un fichier).
    out = {
        'default': D.DEFAULT,
        'auth': {'routes': D.AUTH_ROUTES, 'fields': {k: [{'name': n, 'type': t, 'autocomplete': a} for n, t, a in v]
                                                     for k, v in D.AUTH_FIELDS.items()}, 'passwordMin': D.PASSWORD_MIN},
    }
    return META + json.dumps(out, ensure_ascii=False, indent=2) + '\n'

def locale_json(loc):
    """Textes d'une langue (copié seulement si la langue est choisie : --langs)."""
    L = D.L[loc]
    out = {**locale_meta(loc),
           'site': {k: L.SITE[k] for k in ('name', 'tagline', 'description', 'cta', 'contact', 'social', 'nav', 'footer', 'copyright')},
           'pages': L.PAGES, 'contactFields': L.FIELDS, 'ui': L.UI}
    return lang_meta(loc) + json.dumps(out, ensure_ascii=False, indent=2) + '\n'

def lang_meta(loc, extra=''):
    """En-tête de couche d'un fichier propre à une langue : le français (langue par défaut) est toujours là."""
    cond = 'vitrine' + extra + ('' if loc == D.DEFAULT else f'&lang_{loc}')
    return f'@@meta mode=replace when={cond}@@\n'

LIB = META + HEAD + r"""import { getCollection } from 'astro:content';
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

export const crumbs = (locale: string, ...items: [string, string][]) => [
  { label: data(locale).ui.home, url: href('/', locale) },
  ...items.map(([label, url]) => ({ label, url: href(url, locale) })),
];
"""

def icon_component():
    return (META + "---\n" + HEAD + "// Icônes du site : src/data/icons.json (MDI par défaut, drapeaux Circle Flags ; `--icons <jeu>` le réécrit).\n"
            "// Même fichier et même balisage qu’en Blade (<x-site.icon>) et Django ({% site_icon %}).\n"
            "import data from '../../data/icons.json';\n\n"
            "interface Props {\n  name: string;\n  size?: number;\n}\n\nconst { name, size = 20 } = Astro.props;\n"
            "const icon = (data.icons as Record<string, { body: string; width: number; height: number }>)[name];\n---\n\n"
            "{\n  icon && (\n    <svg width={size} height={size} viewBox={`0 0 ${icon.width} ${icon.height}`} fill=\"currentColor\" "
            "aria-hidden=\"true\" class=\"site-icon\" data-icon={name} set:html={icon.body} />\n  )\n}\n")

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

interface Props {
  title: string;
  description?: string;
  /** Pages du site (sections pleine largeur) ; sinon le contenu est centré (catalogue, compte…). */
  bleed?: boolean;
}

const { title, description, bleed = false } = Astro.props;
@@if auth@@
// Lien « Connexion » ou « Mon compte » de l'en-tête (session lue par src/middleware.ts).
const site = await getSite(Astro.url.pathname, Astro.locals.user);
@@else@@
const site = await getSite(Astro.url.pathname);
@@end@@
---

<!doctype html>
<html lang={site.locale} dir={site.dir}>
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
    <a class="site-skip" href="#contenu">{site.ui.skip}</a>
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

def simple_page(key, locale='fr'):
    return lang_meta(locale) + f"""---
{HEAD}import BaseLayout from '@layouts/BaseLayout.astro';
import Sections from '@/components/site/Sections.astro';
import {{ getPage, getSite }} from '@lib/site';

const page = await getPage('{key}', {{}}, '{locale}');
const site = await getSite(Astro.url.pathname);
---

<BaseLayout title={{page.title}} description={{page.description}} bleed>
  <Sections sections={{page.sections}} site={{site}} />
</BaseLayout>
"""

def home_page(locale='fr'):
    return lang_meta(locale) + f"""---
{HEAD}import BaseLayout from '@layouts/BaseLayout.astro';
import Sections from '@/components/site/Sections.astro';
import {{ getPage, getSite }} from '@lib/site';

const page = await getPage('home', {{}}, '{locale}');
const site = await getSite(Astro.url.pathname);
---

<BaseLayout title={{site.name}} description={{page.description}} bleed>
  <Sections sections={{page.sections}} site={{site}} />
</BaseLayout>
"""

def contact_page(locale='fr'):
    return lang_meta(locale) + f"""---
{HEAD}import BaseLayout from '@layouts/BaseLayout.astro';
import Sections from '@/components/site/Sections.astro';
import {{ type ContactState, getPage, getSite, validateContact }} from '@lib/site';

// Rendu à la demande, même dans un site statique (adaptateur Node) : le formulaire est validé ici
// (Astro vérifie l'origine des POST : security.checkOrigin).
export const prerender = false;

const locale = '{locale}';
let state: ContactState = {{}};
if (Astro.request.method === 'POST') {{
  const form = await Astro.request.formData();
  const values = Object.fromEntries([...form.entries()].map(([k, v]) => [k, String(v)]));
  const errors = validateContact(values, locale);
  state = Object.keys(errors).length ? {{ values, errors }} : {{ sent: true }};
  if (Object.keys(errors).length) Astro.response.status = 422;
}}
const page = await getPage('contact', state, locale);
const site = await getSite(Astro.url.pathname);
---

<BaseLayout title={{page.title}} description={{page.description}} bleed>
  <Sections sections={{page.sections}} site={{site}} />
</BaseLayout>
"""

def detail_page(cat, kind, locale='fr'):
    """services/[slug].astro et blog/[...slug].astro : rendu serveur (ssr) ou pages statiques."""
    coll = 'services' if kind == 'service' else 'blog'
    nodes = TPL.SERVICE_DETAIL if kind == 'service' else TPL.POST_DETAIL
    r = Astro(cat)
    body = r.nodes(nodes)
    getter = 'getServices' if kind == 'service' else 'getPosts'
    if kind == 'service':
        page_expr = """{
  ...item,
  crumbs: crumbs(locale, [site.ui.services, '/services']),
}"""
        desc = 'item.summary'
    else:
        page_expr = """{
  ...item,
  crumbs: crumbs(locale, [site.ui.blog, '/blog']),
  back_url: href('/blog', locale),
}"""
        desc = 'item.excerpt'
    return lang_meta(locale) + f"""---
{HEAD}{r.imports()}
import {{ render }} from 'astro:content';
import BaseLayout from '@layouts/BaseLayout.astro';
import {{ crumbs, {getter}, getSite, href }} from '@lib/site';
void href;
const locale = '{locale}';

@@if ssr@@
// Rendu serveur : l'entrée est résolue à la demande.
export const prerender = false;
const item = (await {getter}(locale)).find((x) => x.slug === Astro.params.slug);
if (!item) return Astro.rewrite('/404');
@@else@@
export async function getStaticPaths() {{
  return (await {getter}('{locale}')).map((item) => ({{ params: {{ slug: item.slug }}, props: {{ id: item.entry.id }} }}));
}}
const item = (await {getter}(locale)).find((x) => x.entry.id === Astro.props.id)!;
@@end@@
const {{ Content }} = await render(item.entry);
const site = await getSite(Astro.url.pathname);
const page = {page_expr};
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

<BaseLayout title={{site.ui.not_found}} bleed>
{body}</BaseLayout>
"""

AUTH_META = '@@meta mode=replace when=vitrine&auth@@\n'

SITE_AUTH = (AUTH_META + HEAD + """// Comptes du site avec Better Auth (API serveur : aucun JavaScript client), comme Fortify (Laravel) et l'auth Django.
import type { AstroGlobal } from 'astro';
import { APIError } from 'better-auth/api';
import { auth } from '@lib/auth';

/** Redirection 303 qui transmet les cookies de session posés par Better Auth. */
function redirect(location: string, source?: Headers): Response {
  const headers = new Headers({ Location: location });
  for (const cookie of source?.getSetCookie() ?? []) headers.append('Set-Cookie', cookie);
  return new Response(null, { status: 303, headers });
}

/** Clé de libellé (site.ui) d'une erreur Better Auth. */
function errorKey(err: unknown, fallback: string): string {
  if (err instanceof APIError) {
    const code = String((err.body as { code?: string } | undefined)?.code ?? '');
    if (code.includes('USER_ALREADY_EXISTS')) return 'err_exists';
    if (code.includes('PASSWORD_TOO_SHORT')) return 'err_password';
    if (code.includes('INVALID_EMAIL_OR_PASSWORD') || code.includes('INVALID_PASSWORD')) return 'err_credentials';
    if (code.includes('INVALID_EMAIL')) return 'invalid_email';
  }
  return fallback;
}

export type AuthResult = { response?: Response; error?: string };

export async function signIn(astro: AstroGlobal, values: Record<string, string>, next: string): Promise<AuthResult> {
  try {
    const { headers } = await auth.api.signInEmail({
      body: { email: values.email.trim(), password: values.password },
      headers: astro.request.headers,
      returnHeaders: true,
    });
    return { response: redirect(next, headers) };
  } catch (err) {
    return { error: errorKey(err, 'err_credentials') };
  }
}

export async function signUp(astro: AstroGlobal, values: Record<string, string>, next: string): Promise<AuthResult> {
  try {
    const { headers } = await auth.api.signUpEmail({
      body: { name: values.name.trim(), email: values.email.trim(), password: values.password },
      headers: astro.request.headers,
      returnHeaders: true,
    });
    return { response: redirect(next, headers) };
  } catch (err) {
    return { error: errorKey(err, 'err_generic') };
  }
}

export async function signOut(astro: AstroGlobal, next: string): Promise<Response> {
  try {
    const { headers } = await auth.api.signOut({ headers: astro.request.headers, returnHeaders: true });
    return redirect(next, headers);
  } catch {
    return redirect(next);
  }
}
""")


def auth_page(cat, kind, locale='fr'):
    r = Astro(cat)
    body = r.nodes(TPL.AUTH_FORM)
    fn = 'signIn' if kind == 'sign_in' else 'signUp'
    return lang_meta(locale, '&auth') + f"""---
{HEAD}{r.imports()}
import BaseLayout from '@layouts/BaseLayout.astro';
import {{ AUTH_ROUTES, type AuthState, authPage, getSite, href, uiText, validateAuth }} from '@lib/site';
import {{ {fn} }} from '@lib/site-auth';

// Formulaire traité côté serveur (Better Auth) ; Astro vérifie l'origine des POST (security.checkOrigin).
export const prerender = false;
const locale = '{locale}';
if (Astro.locals.user) return Astro.redirect(href(AUTH_ROUTES.account, locale));
let state: AuthState = {{}};
if (Astro.request.method === 'POST') {{
  const form = await Astro.request.formData();
  const values = Object.fromEntries([...form.entries()].map(([k, v]) => [k, String(v)]));
  let error = validateAuth('{kind}', values, locale);
  if (!error) {{
    const result = await {fn}(Astro, values, href(AUTH_ROUTES.account, locale));
    if (result.response) return result.response;
    error = uiText(result.error ?? 'err_generic', locale);
  }}
  state = {{ error, values }};
  Astro.response.status = 422;
}}
const page = authPage('{kind}', state, locale);
const site = await getSite(Astro.url.pathname, Astro.locals.user);
---

<BaseLayout title={{page.title}} bleed>
{body}</BaseLayout>
"""


def account_page(cat, locale='fr'):
    r = Astro(cat)
    body = r.nodes(TPL.ACCOUNT)
    return lang_meta(locale, '&auth') + f"""---
{HEAD}{r.imports()}
import BaseLayout from '@layouts/BaseLayout.astro';
import {{ AUTH_ROUTES, accountPage, getSite, href }} from '@lib/site';
import {{ signOut }} from '@lib/site-auth';

export const prerender = false;
const locale = '{locale}';
const user = Astro.locals.user;
if (!user) return Astro.redirect(href(AUTH_ROUTES.sign_in, locale));
// Déconnexion : formulaire POST de cette page.
if (Astro.request.method === 'POST') return signOut(Astro, href('/', locale));
const page = accountPage(user, locale);
const site = await getSite(Astro.url.pathname, user);
---

<BaseLayout title={{page.title}} bleed>
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
    for loc, L in D.L.items():
      for p in L.POSTS:
        files[f"src/content/blog/{loc}/{p['slug']}.md.tpl"] = (
            lang_meta(loc).replace('mode=replace ', '') + "---\n"
            f"title: {yaml_str(p['title'])}\ndescription: {yaml_str(p['excerpt'])}\npubDate: {p['date']}\n"
            f"category: {yaml_str(p['category'])}\nauthor: {yaml_str(p['author'])}\n---\n\n{p['body_md'].strip()}\n")
      for s in L.SERVICES:
        feats = ''.join(f"\n  - {yaml_str(f)}" for f in s['features'])
        files[f"src/content/services/{loc}/{s['slug']}.md.tpl"] = (
            lang_meta(loc).replace('mode=replace ', '') + "---\n"
            f"title: {yaml_str(s['title'])}\nsummary: {yaml_str(s['summary'])}\nicon: {s['icon']}\nprice: {yaml_str(s['price'])}\n"
            f"order: {s['order']}\nfeatures:{feats}\n---\n\n{s['body_md'].strip()}\n")
    return files

def files(cat, css):
    out = {
        'src/data/site.json.tpl': site_json(),
        'src/lib/site.ts.tpl': LIB,
        'src/lib/site-auth.ts.tpl': SITE_AUTH,
        'src/content.config.ts.tpl': CONTENT_CONFIG,
        'src/layouts/BaseLayout.astro.tpl': layout(),
        'src/components/site/SiteHeader.astro.tpl': comp(cat, TPL.HEADER, SITE_PROPS),
        'src/components/site/SiteFooter.astro.tpl': comp(cat, TPL.FOOTER, SITE_PROPS),
        'src/components/site/SiteIcon.astro.tpl': icon_component(),
        'src/data/icons.json.tpl': META + icons_json(),
        'src/components/site/Sections.astro.tpl': sections_component(),
        'src/pages/404.astro.tpl': not_found(cat),
        'src/styles/site.css.tpl': META + css,
    }
    # Pages de chaque langue : src/pages/ (fr, langue par défaut) et src/pages/en/ (routage i18n d'Astro, dossiers par langue).
    for loc in D.LOCALES:
        out[f'src/data/locales/{loc}.json.tpl'] = locale_json(loc)
        d = 'src/pages/' if loc == D.DEFAULT else f'src/pages/{loc}/'
        out.update({
            f'{d}index.astro.tpl': home_page(loc),
            f'{d}a-propos.astro.tpl': simple_page('about', loc),
            f'{d}mentions-legales.astro.tpl': simple_page('legal', loc),
            f'{d}confidentialite.astro.tpl': simple_page('privacy', loc),
            f'{d}services/index.astro.tpl': simple_page('services', loc),
            f'{d}blog/index.astro.tpl': simple_page('blog', loc),
            f'{d}contact.astro.tpl': contact_page(loc),
            f'{d}services/[slug].astro.tpl': detail_page(cat, 'service', loc),
            f'{d}blog/[...slug].astro.tpl': detail_page(cat, 'post', loc),
            # Comptes (--auth) : remplacent les pages de la couche astro par celles du site.
            f'{d}auth/sign-in.astro.tpl': auth_page(cat, 'sign_in', loc),
            f'{d}auth/sign-up.astro.tpl': auth_page(cat, 'sign_up', loc),
            f'{d}auth/dashboard.astro.tpl': account_page(cat, loc),
        })
    for t, nodes in TPL.SECTIONS.items():
        out[f'src/components/site/sections/{pascal(t)}.astro.tpl'] = comp(cat, nodes, SECTION_PROPS)
    out.update(content_files())
    return out
