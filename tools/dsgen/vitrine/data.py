"""Données du site prêtes à écrire, dans chaque langue : spec typographiée + corps Markdown convertis en HTML.

Les valeurs du module (SITE, SERVICES, …) sont celles de la langue par défaut (fr) ; `L['en']` porte les mêmes
structures traduites (locales/<langue>.py), de même pour chaque langue. Les adresses restent celles de la spec : chaque framework les préfixe (`/en/…`).
"""
import copy
from types import SimpleNamespace
from . import spec, md, i18n

RAW_KEYS = {'url', 'email', 'icon', 'slug', 'type', 'name', 'autocomplete', 'date', 'phone', 'body'}
NO_TR = {'url', 'email', 'icon', 'slug', 'type', 'name', 'autocomplete', 'date', 'phone', 'code'}

def _tr(v, locale, key=None):
    """Traduit un texte de la spec ; erreur si locales/<langue>.py n'en a pas la traduction."""
    if isinstance(v, str):
        if key in NO_TR or v == 'services':
            return v
        if v not in i18n.TR[locale]:
            raise KeyError(f'traduction « {locale} » manquante dans vitrine/locales/{locale}.py : {v[:70]!r}')
        return i18n.TR[locale][v]
    if isinstance(v, list): return [_tr(x, locale, key) for x in v]
    if isinstance(v, dict): return {k: _tr(x, locale, k) for k, x in v.items()}
    return v

def _typo(v, key=None):
    if isinstance(v, str):
        return v if key in RAW_KEYS else md.typo(v)
    if isinstance(v, list): return [_typo(x, key) for x in v]
    if isinstance(v, dict): return {k: _typo(x, k) for k, x in v.items()}
    return v

def build(locale):
    src = (lambda x: copy.deepcopy(x)) if locale == i18n.DEFAULT else (lambda x: _tr(copy.deepcopy(x), locale))
    site = _typo(src(spec.SITE)); site['name'] = '@@title@@'
    services = _typo(src(spec.SERVICES)); posts = _typo(src(spec.POSTS))
    pages = _typo(src(spec.PAGES)); fields = _typo(src(spec.CONTACT_FIELDS))
    for coll in (services, posts):
        for e in coll:
            e['body_md'] = md.typo(e['body'])
            e['body_html'] = md.to_html(e['body_md'])
    for p in pages.values():
        for s in p['sections']:
            if s['type'] == 'text':
                s['body'] = md.to_html(md.typo(s['body']))
    # Ordre d'affichage : services dans l'ordre de la spec, articles du plus récent au plus ancien.
    for i, e in enumerate(services): e['order'] = i + 1
    posts.sort(key=lambda p: p['date'], reverse=True)
    ui = {k: v[locale] for k, v in i18n.UI.items()}
    return SimpleNamespace(SITE=site, SERVICES=services, POSTS=posts, PAGES=pages, FIELDS=fields, UI=ui)

LOCALES = i18n.CODES
META = i18n.META
DEFAULT = i18n.DEFAULT
L = {code: build(code) for code in LOCALES}
SITE, SERVICES, POSTS, PAGES, FIELDS, UI = (getattr(L[DEFAULT], k) for k in ('SITE', 'SERVICES', 'POSTS', 'PAGES', 'FIELDS', 'UI'))

# Pages simples (sections seules) et leur adresse ; services/blog ont en plus des pages de détail.
SIMPLE_PAGES = ['about', 'legal', 'privacy']

# Comptes (--auth) : mêmes adresses et mêmes champs dans les trois frameworks (Better Auth, Fortify, auth Django).
AUTH_ROUTES = {'sign_in': '/auth/sign-in', 'sign_up': '/auth/sign-up', 'account': '/auth/dashboard'}
# champ : (nom, type, autocomplete) ; le libellé est site.ui.field_<nom>
AUTH_FIELDS = {
    'sign_in': [('email', 'email', 'email'), ('password', 'password', 'current-password')],
    'sign_up': [('name', 'text', 'name'), ('email', 'email', 'email'), ('password', 'password', 'new-password')],
}
PASSWORD_MIN = 8

# Politique de sécurité du contenu (CSP) de base, identique sur les 3 frameworks, envoyée en production seulement
# (Astro : src/middleware.ts ; Laravel : App\Http\Middleware\SecurityHeaders ; Wagtail : SECURE_CSP de Django 6,
# settings/production.py). 'unsafe-inline' : scripts et styles en ligne du thème, des îlots Astro et de l'admin Wagtail.
CSP = ("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; "
       "img-src 'self' data: blob: https:; font-src 'self' data:; connect-src 'self'; object-src 'none'; "
       "base-uri 'self'; form-action 'self'; frame-ancestors 'self'")
