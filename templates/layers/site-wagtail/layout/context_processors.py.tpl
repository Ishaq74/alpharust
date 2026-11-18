# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
"""Processeur de contexte : ``site`` (navigation, pied de page, coordonnées) pour base.html et les blocs.

Mêmes clés que src/lib/site.ts (Astro) et App\\Support\\SiteData (Laravel) : le balisage produit est identique.
"""
import re
from datetime import date

from django.utils.functional import SimpleLazyObject
from wagtail.models import Site

from layout.models import FooterSettings, HeaderSettings
from site_settings.models import ContactSettings, OrganisationSettings, SocialMediaSettings

SOCIAL = [("github", "GitHub"), ("linkedin", "LinkedIn"), ("mastodon", "Mastodon")]


def href(url):
    """Adresse interne au format Wagtail (barre finale)."""
    if not url or url == "/" or re.match(r"^[a-z]+:", url):
        return url or "/"
    return "/" + url.strip("/") + "/"


def current(url, path):
    """'page' (page exacte), 'true' (rubrique) ou None — même règle que les autres frameworks."""
    u, p = href(url), href(path)
    if p == u:
        return "page"
    return "true" if u != "/" and p.startswith(u) else None


def link(label, url, path=None):
    out = {"label": label, "url": href(url)}
    if path is not None:
        out["current"] = current(url, path)
    return out


def services(path=None):
    from service.models import ServicePage

    return [link(p.title, p.url, path) for p in ServicePage.objects.live().order_by("path")]


def menu_links(menu, path=None):
    return [link(i.label, i.get_url(), path) for i in menu.items.filter(parent__isnull=True)] if menu else []


def build_site(request):
    path = request.path
    wsite = Site.find_for_request(request)
    header = HeaderSettings.load(request_or_site=request)
    footer = FooterSettings.load(request_or_site=request)
    org = OrganisationSettings.load(request_or_site=request)
    contact = ContactSettings.load(request_or_site=request)
    social = SocialMediaSettings.load(request_or_site=request)
    name = wsite.site_name if wsite and wsite.site_name else "@@title@@"
    nav = []
    if header.selected_menu:
        for item in header.selected_menu.items.filter(parent__isnull=True):
            children = [link(c.label, c.get_url(), path) for c in item.children.all().order_by("sort_order")]
            nav.append({**link(item.label, item.get_url(), path), "children": children or None})
    columns = []
    if footer.selected_menu_1:
        columns.append({"title": footer.selected_menu_1.title, "links": menu_links(footer.selected_menu_1)})
    if footer.services_title:
        columns.append({"title": footer.services_title, "links": [link(s["label"], s["url"]) for s in services()]})
    if footer.selected_menu_2:
        columns.append({"title": footer.selected_menu_2.title, "links": menu_links(footer.selected_menu_2)})
    return {
        "name": name,
        "initial": name[:1].upper(),
        "tagline": org.tagline,
        "description": org.description,
        "home_url": "/",
        "cta": link(header.cta_label, header.cta_page.url if header.cta_page else "/"),
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
            "address": contact.address,
            "hours": contact.hours,
        },
        "year": date.today().year,
        "copyright": org.copyright,
    }


def site(request):
    return {"site": SimpleLazyObject(lambda: build_site(request))}
