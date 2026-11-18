# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
"""Processeur de contexte : ``site`` (navigation, pied de page, coordonnées, langues) pour base.html et les blocs.

Mêmes clés que src/lib/site.ts (Astro) et App\\Support\\SiteData (Laravel) : le balisage produit est identique.
Langues : internationalisation de Wagtail (pages et menus traduits, même ``translation_key``) ; les réglages et
libellés d'interface passent par gettext (layout/locale/<langue>/LC_MESSAGES/django.po).
"""
import re
from datetime import date

from django.conf import settings
from django.utils.functional import SimpleLazyObject
from django.utils.translation import get_language, gettext
from wagtail.models import Locale, Site

from layout.models import FooterSettings, HeaderSettings
from site_settings.models import ContactSettings, OrganisationSettings, SocialMediaSettings

SOCIAL = [("github", "GitHub"), ("linkedin", "LinkedIn"), ("mastodon", "Mastodon")]
UI = {'skip': 'Aller au contenu', 'main_nav': 'Navigation principale', 'mobile_nav': 'Navigation mobile', 'theme': 'Changer de thème', 'open_menu': 'Ouvrir le menu', 'social': 'Réseaux sociaux', 'language': 'Langue', 'contact': 'Contact', 'more': 'En savoir plus', 'more_label': 'En savoir plus : ', 'details': 'Coordonnées', 'included': 'Ce qui est inclus', 'all_posts': 'Tous les articles', 'home': 'Accueil', 'not_found': 'Page introuvable', 'not_found_text': 'La page demandée n’existe pas ou a été déplacée.', 'back_home': 'Retour à l’accueil', 'required': 'Ce champ est obligatoire.', 'invalid_email': 'Adresse e-mail invalide.'}
LABELS = {code: label for code, _, label in [('fr', 'Français', 'FR'), ('en', 'English', 'EN')]}


def locale_code():
    code = (get_language() or settings.LANGUAGE_CODE).split("-")[0]
    return code if code in dict(settings.LANGUAGES) else settings.LANGUAGE_CODE


def prefix(code):
    return "" if code == settings.LANGUAGE_CODE else "/" + code


def href(url, code=None):
    """Adresse interne au format Wagtail (barre finale) ; les chemins de la spec reçoivent le préfixe de langue."""
    if not url or re.match(r"^[a-z]+:", url):
        return url or "/"
    return "/" + url.strip("/") + "/" if url != "/" else "/"


def localize(url):
    """Chemin de la spec (`/contact`) dans la langue active (`/en/contact/`) ; adresses externes inchangées."""
    if not url or re.match(r"^[a-z]+:", url):
        return url
    code = locale_code()
    if code != settings.LANGUAGE_CODE and not (url + "/").startswith("/" + code + "/"):
        url = "/" + code + ("" if url == "/" else url)
    return href(url)


def home_url(code=None):
    return prefix(code or locale_code()) + "/"


def current(url, path):
    """'page' (page exacte), 'true' (rubrique) ou None — même règle que les autres frameworks."""
    u, p = href(url), href(path)
    if p == u:
        return "page"
    return "true" if u != home_url() and p.startswith(u) else None


def link(label, url, path=None):
    out = {"label": label, "url": href(url)}
    if path is not None:
        out["current"] = current(url, path)
    return out


def localized_url(page):
    return page.localized.url if page else None


def services(path=None):
    from service.models import ServicePage

    pages = ServicePage.objects.live().filter(locale=Locale.get_active()).order_by("path")
    return [link(p.title, p.url, path) for p in pages]


def menu_links(menu, path=None):
    return [link(i.label, i.get_url(), path) for i in menu.items.filter(parent__isnull=True)] if menu else []


def unprefixed(path):
    for code, _ in settings.LANGUAGES:
        if code != settings.LANGUAGE_CODE and (path == "/" + code + "/" or path.startswith("/" + code + "/")):
            return path[len(code) + 1:] or "/"
    return path


def build_site(request):
    path = request.path
    code = locale_code()
    wsite = Site.find_for_request(request)
    header = HeaderSettings.load(request_or_site=request)
    footer = FooterSettings.load(request_or_site=request)
    org = OrganisationSettings.load(request_or_site=request)
    contact = ContactSettings.load(request_or_site=request)
    social = SocialMediaSettings.load(request_or_site=request)
    name = wsite.site_name if wsite and wsite.site_name else "@@title@@"
    main_menu = header.selected_menu.localized if header.selected_menu else None
    nav = []
    if main_menu:
        for item in main_menu.items.filter(parent__isnull=True):
            children = [link(c.label, c.get_url(), path) for c in item.children.all().order_by("sort_order")]
            nav.append({**link(item.label, item.get_url(), path), "children": children or None})
    columns = []
    for menu in (footer.selected_menu_1, None, footer.selected_menu_2):
        if menu is None and footer.services_title:
            columns.append({"title": gettext(footer.services_title), "links": [link(s["label"], s["url"]) for s in services()]})
        elif menu is not None:
            menu = menu.localized
            columns.append({"title": menu.title, "links": menu_links(menu)})
    rest = unprefixed(path)
    copyright = gettext(org.copyright) if org.copyright else ""
    return {
        "locale": code,
        "languages": [
            {"code": c, "label": LABELS.get(c, c.upper()), "name": n, "url": prefix(c) + rest, "current": "page" if c == code else None}
            for c, n in settings.LANGUAGES
        ],
        "ui": {k: gettext(v) for k, v in UI.items()},
        "name": name,
        "initial": name[:1].upper(),
        "tagline": gettext(org.tagline) if org.tagline else "",
        "description": gettext(org.description) if org.description else "",
        "home_url": home_url(code),
        "cta": link(gettext(header.cta_label) if header.cta_label else "", localized_url(header.cta_page) or home_url(code)),
        "nav": nav,
        "footer": columns,
        "social": [
            {"label": label, "url": getattr(social, key), "icon": key} for key, label in SOCIAL if getattr(social, key)
        ],
        "contact": {
            "email": contact.email,
            "email_url": f"mailto:{contact.email}",
            "phone": contact.phone,
            "phone_url": "tel:" + re.sub(r"\s+", "", contact.phone),
            "address": gettext(contact.address) if contact.address else "",
            "hours": gettext(contact.hours) if contact.hours else "",
        },
        "year": date.today().year,
        "copyright": copyright,
        "copyright_line": f"© {date.today().year} {name} · {copyright}",
    }


def site(request):
    return {"site": SimpleLazyObject(lambda: build_site(request))}
