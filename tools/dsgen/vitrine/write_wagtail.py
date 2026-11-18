"""Rendu Wagtail : modèles de page + StreamField + snippets/réglages + base.html (modèle ishaq74/wagtail-cms)."""
import json
from . import data as D, templates as TPL
from .ir import Django
from .icons import ICONS

GEN = '{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}\n'
PYGEN = '# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.\n'
LOAD = '{% load ds site_tags wagtailcore_tags %}\n'

def dj(cat, nodes, ind=0): return Django(cat).nodes(nodes, ind)

BASE = """@@meta mode=replace@@
{% load static wagtailcore_tags wagtailuserbar %}
""" + GEN + """<!DOCTYPE html>
<html lang="fr">
    <head>
        <meta charset="utf-8" />
        <title>{% block title %}{% if page.seo_title %}{{ page.seo_title }}{% else %}{{ page.title }}{% endif %} · {{ site.name }}{% endblock %}</title>
        <meta name="description" content="{% block description %}{% if page.search_description %}{{ page.search_description }}{% else %}{{ site.description }}{% endif %}{% endblock %}" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="canonical" href="{{ request.build_absolute_uri }}" />
        <meta property="og:type" content="website" />
        <meta property="og:title" content="{% block og_title %}{% if page.seo_title %}{{ page.seo_title }}{% else %}{{ page.title }}{% endif %} · {{ site.name }}{% endblock %}" />
        <meta property="og:description" content="{% if page.search_description %}{{ page.search_description }}{% else %}{{ site.description }}{% endif %}" />
        <script>
            // Applique le thème choisi avant le premier rendu (évite le flash).
            try {
                const saved = localStorage.getItem('theme');
                const theme = saved === 'light' || saved === 'dark' ? saved : (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
                document.documentElement.dataset.theme = theme;
                document.documentElement.classList.toggle('dark', theme === 'dark');
            } catch (e) {}
        </script>

        {# Force all links in the live preview panel to be opened in a new tab #}
        {% if request.in_preview_panel %}
        <base target="_blank">
        {% endif %}

        {# Global stylesheets #}
        <link rel="stylesheet" type="text/css" href="{% static 'css/site.css' %}">

        {% block extra_css %}
        {# Override this in templates to add extra stylesheets #}
        {% endblock %}
    </head>

    <body class="site-body{% block body_class %}{% endblock %}">
        {% wagtailuserbar %}
        <a class="site-skip" href="#contenu">Aller au contenu</a>
        {% include "includes/header.html" %}
        <main id="contenu" class="site-main" tabindex="-1">
            {% block content %}{% endblock %}
        </main>
        {% include "includes/footer.html" %}

        {# Global javascript #}

        {% block extra_js %}
        {# Override this in templates to add extra javascript #}
        {% endblock %}
    </body>
</html>
"""

def stream_page(title_block=''):
    return ('{% extends "base.html" %}\n{% load wagtailcore_tags %}\n' + GEN + title_block +
            '{% block content %}\n{% for block in page.body %}{% include_block block %}{% endfor %}\n{% endblock %}\n')

def detail(cat, nodes):
    return '{% extends "base.html" %}\n' + LOAD + GEN + '{% block content %}\n' + dj(cat, nodes) + '{% endblock %}\n'

def not_found(cat):
    return ('@@meta mode=replace@@\n{% extends "base.html" %}\n' + LOAD + GEN +
            '{% block title %}Page introuvable · {{ site.name }}{% endblock %}\n{% block og_title %}Page introuvable · {{ site.name }}{% endblock %}\n'
            '{% block content %}\n' + dj(cat, TPL.NOT_FOUND) + '{% endblock %}\n')

APP = lambda name, label: f'''from django.apps import AppConfig


class {label}Config(AppConfig):
    default_auto_field = "django.db.models.BigAutoField"
    name = "{name}"
'''

ICONS_PY = PYGEN + '"""Tracés Lucide (ISC) : mêmes icônes qu’Astro (SiteIcon.astro) et Blade (<x-site.icon>)."""\n\nICONS = {\n' + \
    ''.join(f'    {json.dumps(k)}: {json.dumps(v)},\n' for k, v in ICONS.items()) + '}\n'

SITE_TAGS = PYGEN + '''from django import template
from django.utils.html import format_html
from django.utils.safestring import mark_safe

from layout.icons import ICONS

register = template.Library()


@register.simple_tag
def site_icon(name, size=20):
    """Icône du site par son nom (``{% site_icon "mail" 18 %}``)."""
    return format_html(
        '<svg width="{}" height="{}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" '
        'stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon">{}</svg>',
        size,
        size,
        mark_safe(ICONS.get(name, "")),
    )
'''

LAYOUT_MODELS = PYGEN + '''"""En-tête et pied de page administrables (modèle ishaq74/wagtail-cms : layout/models.py).

- ``NavigationMenu`` (snippet) : un menu et ses éléments, avec sous-menus (``parent``) ;
- ``HeaderSettings`` / ``FooterSettings`` (réglages du site) : menus affichés, bouton d'appel à l'action.
"""
from django.db import models
from modelcluster.fields import ParentalKey
from modelcluster.models import ClusterableModel
from wagtail.admin.panels import FieldPanel, InlinePanel, PageChooserPanel
from wagtail.contrib.settings.models import BaseGenericSetting, register_setting
from wagtail.models import Orderable, Page
from wagtail.snippets.models import register_snippet


@register_snippet
class NavigationMenu(ClusterableModel):
    title = models.CharField("titre", max_length=255, help_text="Nom du menu, affiché comme titre de colonne dans le pied de page.")

    panels = [FieldPanel("title"), InlinePanel("items", label="Éléments du menu")]

    class Meta:
        verbose_name = "menu de navigation"
        verbose_name_plural = "menus de navigation"

    def __str__(self):
        return self.title


class NavigationItem(Orderable):
    menu = ParentalKey(NavigationMenu, on_delete=models.CASCADE, related_name="items")
    label = models.CharField("libellé", max_length=255)
    page = models.ForeignKey(Page, null=True, blank=True, on_delete=models.SET_NULL, related_name="+")
    custom_url = models.CharField("lien personnalisé", max_length=500, blank=True, help_text="Adresse ou chemin si aucune page n'est choisie.")
    parent = models.ForeignKey(
        "self", null=True, blank=True, on_delete=models.CASCADE, related_name="children", help_text="Élément parent (sous-menu)."
    )

    panels = [FieldPanel("label"), PageChooserPanel("page"), FieldPanel("custom_url"), FieldPanel("parent")]

    def get_url(self):
        return self.page.url if self.page else self.custom_url

    def __str__(self):
        return self.label


@register_setting(icon="list-ul")
class HeaderSettings(BaseGenericSetting):
    selected_menu = models.ForeignKey(
        NavigationMenu, null=True, blank=True, on_delete=models.SET_NULL, related_name="+", verbose_name="menu principal"
    )
    cta_label = models.CharField("bouton : libellé", max_length=60, blank=True)
    cta_page = models.ForeignKey(Page, null=True, blank=True, on_delete=models.SET_NULL, related_name="+", verbose_name="bouton : page")

    panels = [FieldPanel("selected_menu"), FieldPanel("cta_label"), PageChooserPanel("cta_page")]

    class Meta:
        verbose_name = "en-tête"


@register_setting(icon="list-ul")
class FooterSettings(BaseGenericSetting):
    selected_menu_1 = models.ForeignKey(
        NavigationMenu, null=True, blank=True, on_delete=models.SET_NULL, related_name="+", verbose_name="première colonne"
    )
    services_title = models.CharField(
        "colonne des services", max_length=60, blank=True, default="Services", help_text="Vide : colonne masquée."
    )
    selected_menu_2 = models.ForeignKey(
        NavigationMenu, null=True, blank=True, on_delete=models.SET_NULL, related_name="+", verbose_name="troisième colonne"
    )

    panels = [FieldPanel("selected_menu_1"), FieldPanel("services_title"), FieldPanel("selected_menu_2")]

    class Meta:
        verbose_name = "pied de page"
'''

SETTINGS_MODELS = PYGEN + '''"""Réglages du site (modèle ishaq74/wagtail-cms : site_settings/models.py) : organisation, coordonnées, réseaux."""
from django.db import models
from wagtail.contrib.settings.models import BaseGenericSetting, register_setting


@register_setting(icon="site")
class OrganisationSettings(BaseGenericSetting):
    tagline = models.CharField("accroche", max_length=255, blank=True)
    description = models.TextField("description (SEO par défaut)", blank=True)
    copyright = models.CharField("mention de copyright", max_length=255, blank=True)

    class Meta:
        verbose_name = "organisation"


@register_setting(icon="mail")
class ContactSettings(BaseGenericSetting):
    email = models.EmailField("e-mail", blank=True)
    phone = models.CharField("téléphone", max_length=40, blank=True)
    address = models.CharField("adresse", max_length=255, blank=True)
    hours = models.CharField("horaires", max_length=255, blank=True)

    class Meta:
        verbose_name = "coordonnées"


@register_setting(icon="link")
class SocialMediaSettings(BaseGenericSetting):
    github = models.URLField("GitHub", blank=True)
    linkedin = models.URLField("LinkedIn", blank=True)
    mastodon = models.URLField("Mastodon", blank=True)

    class Meta:
        verbose_name = "réseaux sociaux"
'''

CONTEXT = PYGEN + '''"""Processeur de contexte : ``site`` (navigation, pied de page, coordonnées) pour base.html et les blocs.

Mêmes clés que src/lib/site.ts (Astro) et App\\\\Support\\\\SiteData (Laravel) : le balisage produit est identique.
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
            "phone_url": "tel:" + re.sub(r"\\s+", "", contact.phone),
            "address": contact.address,
            "hours": contact.hours,
        },
        "year": date.today().year,
        "copyright": org.copyright,
    }


def site(request):
    return {"site": SimpleLazyObject(lambda: build_site(request))}
'''

ICON_CHOICES = sorted({s['icon'] for s in D.SERVICES} | {i['icon'] for p in D.PAGES.values() for s in p['sections'] if s['type'] == 'features' for i in s['items']})

BLOCKS = PYGEN + '''"""Blocs StreamField : une section du site par bloc (mêmes types que src/data/site.json et config/site.php).

Chaque bloc expose à son gabarit (streams/sections/<type>.html) une variable ``s`` normalisée, identique
aux autres frameworks ; les blocs « services » et « posts » y ajoutent les pages correspondantes.
"""
from wagtail import blocks
from wagtail.rich_text import RichText

from layout.context_processors import href

ICONS = [''' + ', '.join(f'("{i}", "{i}")' for i in ICON_CHOICES) + ''']


class LinkBlock(blocks.StructBlock):
    label = blocks.CharBlock(label="libellé", required=False)
    page = blocks.PageChooserBlock(required=False)
    url = blocks.CharBlock(label="adresse ou chemin", required=False)

    class Meta:
        icon = "link"


def plain(value):
    """StructValue/ListValue -> dict/list ; un lien vide devient None, un lien rempli {label, url}."""
    if isinstance(value, blocks.StructValue):
        if set(value.keys()) == {"label", "page", "url"}:
            if not value.get("label"):
                return None
            target = value["page"].url if value.get("page") else value.get("url")
            return {"label": value["label"], "url": href(target)}
        return {k: plain(v) for k, v in value.items()}
    if isinstance(value, (list, blocks.list_block.ListValue)):
        return [plain(v) for v in value]
    return value


class SectionBlock(blocks.StructBlock):
    def section(self, value, parent_context):
        return plain(value)

    def get_context(self, value, parent_context=None):
        context = super().get_context(value, parent_context=parent_context)
        context["s"] = self.section(value, parent_context or {})
        return context


class HeroBlock(SectionBlock):
    badge = blocks.CharBlock(required=False)
    title = blocks.CharBlock(label="titre")
    lead = blocks.TextBlock(label="chapeau")
    primary = LinkBlock(label="bouton principal")
    secondary = LinkBlock(label="bouton secondaire", required=False)

    class Meta:
        template = "streams/sections/hero.html"
        icon = "pick"


class PageHeaderBlock(SectionBlock):
    eyebrow = blocks.CharBlock(label="surtitre", required=False)
    title = blocks.CharBlock(label="titre")
    lead = blocks.TextBlock(label="chapeau", required=False)

    class Meta:
        template = "streams/sections/page_header.html"
        icon = "title"


class StatsBlock(SectionBlock):
    items = blocks.ListBlock(blocks.StructBlock([("value", blocks.CharBlock()), ("label", blocks.CharBlock())]))

    class Meta:
        template = "streams/sections/stats.html"
        icon = "order"


class FeaturesBlock(SectionBlock):
    title = blocks.CharBlock(label="titre", required=False)
    lead = blocks.TextBlock(label="chapeau", required=False)
    items = blocks.ListBlock(
        blocks.StructBlock(
            [("icon", blocks.ChoiceBlock(choices=ICONS)), ("title", blocks.CharBlock()), ("text", blocks.TextBlock())]
        )
    )

    class Meta:
        template = "streams/sections/features.html"
        icon = "list-ul"


class ServicesBlock(SectionBlock):
    title = blocks.CharBlock(label="titre", required=False)
    lead = blocks.TextBlock(label="chapeau", required=False)
    limit = blocks.IntegerBlock(label="nombre (0 = tous)", min_value=0, default=0, required=False)
    link = LinkBlock(label="lien « voir tout »", required=False)

    def section(self, value, parent_context):
        from service.models import ServicePage

        s = plain(value)
        pages = ServicePage.objects.live().order_by("path")
        if s.get("limit"):
            pages = pages[: s["limit"]]
        s["items"] = [p.card() for p in pages]
        return s

    class Meta:
        template = "streams/sections/services.html"
        icon = "cogs"


class TestimonialsBlock(SectionBlock):
    title = blocks.CharBlock(label="titre", required=False)
    lead = blocks.TextBlock(label="chapeau", required=False)
    items = blocks.ListBlock(
        blocks.StructBlock(
            [
                ("quote", blocks.TextBlock()),
                ("name", blocks.CharBlock()),
                ("role", blocks.CharBlock()),
                ("initials", blocks.CharBlock(max_length=3)),
            ]
        )
    )

    class Meta:
        template = "streams/sections/testimonials.html"
        icon = "openquote"


class PostsBlock(SectionBlock):
    title = blocks.CharBlock(label="titre", required=False)
    lead = blocks.TextBlock(label="chapeau", required=False)
    limit = blocks.IntegerBlock(label="nombre (0 = tous)", min_value=0, default=0, required=False)
    link = LinkBlock(label="lien « voir tout »", required=False)

    def section(self, value, parent_context):
        from blog.models import BlogPage

        s = plain(value)
        pages = BlogPage.objects.live().order_by("-date")
        if s.get("limit"):
            pages = pages[: s["limit"]]
        s["items"] = [p.card() for p in pages]
        return s

    class Meta:
        template = "streams/sections/posts.html"
        icon = "doc-full"


class FaqBlock(SectionBlock):
    title = blocks.CharBlock(label="titre", required=False)
    lead = blocks.TextBlock(label="chapeau", required=False)
    items = blocks.ListBlock(blocks.StructBlock([("question", blocks.CharBlock()), ("answer", blocks.TextBlock())]))

    class Meta:
        template = "streams/sections/faq.html"
        icon = "help"


class CtaBlock(SectionBlock):
    title = blocks.CharBlock(label="titre")
    text = blocks.TextBlock(label="texte")
    action = LinkBlock(label="bouton")

    class Meta:
        template = "streams/sections/cta.html"
        icon = "pick"


class TextBlock(SectionBlock):
    title = blocks.CharBlock(label="titre", required=False)
    body = blocks.RichTextBlock(label="texte")

    def section(self, value, parent_context):
        s = plain(value)
        s["body"] = value["body"] if isinstance(value["body"], RichText) else RichText(value["body"])
        return s

    class Meta:
        template = "streams/sections/text.html"
        icon = "pilcrow"


class TeamBlock(SectionBlock):
    title = blocks.CharBlock(label="titre", required=False)
    lead = blocks.TextBlock(label="chapeau", required=False)
    items = blocks.ListBlock(
        blocks.StructBlock([("name", blocks.CharBlock()), ("role", blocks.CharBlock()), ("initials", blocks.CharBlock(max_length=3))])
    )

    class Meta:
        template = "streams/sections/team.html"
        icon = "group"


class ContactBlock(SectionBlock):
    """Formulaire de la ContactPage (wagtail.contrib.forms) + coordonnées ; n'a de sens que sur cette page."""

    title = blocks.CharBlock(label="titre")
    success = blocks.CharBlock(label="message de confirmation")
    submit = blocks.CharBlock(label="bouton d'envoi")

    def section(self, value, parent_context):
        s = plain(value)
        page, form = parent_context.get("page"), parent_context.get("form")
        s["action"] = page.url if page else ""
        s["sent"] = bool(parent_context.get("sent"))
        s["fields"] = page.contact_fields(form) if form is not None and hasattr(page, "contact_fields") else []
        return s

    class Meta:
        template = "streams/sections/contact.html"
        icon = "mail"


SECTION_BLOCKS = [
    ("hero", HeroBlock()),
    ("page_header", PageHeaderBlock()),
    ("stats", StatsBlock()),
    ("features", FeaturesBlock()),
    ("services", ServicesBlock()),
    ("testimonials", TestimonialsBlock()),
    ("posts", PostsBlock()),
    ("faq", FaqBlock()),
    ("cta", CtaBlock()),
    ("text", TextBlock()),
    ("team", TeamBlock()),
]
'''

HOME_MODELS = '@@meta mode=replace@@\n' + PYGEN + '''from wagtail.admin.panels import FieldPanel
from wagtail.fields import StreamField
from wagtail.models import Page

from streams.blocks import SECTION_BLOCKS


class HomePage(Page):
    body = StreamField(SECTION_BLOCKS, blank=True, use_json_field=True, verbose_name="sections")

    content_panels = Page.content_panels + [FieldPanel("body")]
    max_count = 1
'''

FLEX_MODELS = PYGEN + '''from wagtail.admin.panels import FieldPanel
from wagtail.fields import StreamField
from wagtail.models import Page

from streams.blocks import SECTION_BLOCKS


class FlexPage(Page):
    """Page libre composée de sections : à propos, mentions légales, confidentialité…"""

    body = StreamField(SECTION_BLOCKS, blank=True, use_json_field=True, verbose_name="sections")

    content_panels = Page.content_panels + [FieldPanel("body")]

    class Meta:
        verbose_name = "page libre"
'''

SERVICE_MODELS = PYGEN + '''from django.db import models
from wagtail.admin.panels import FieldPanel
from wagtail.blocks import CharBlock
from wagtail.fields import RichTextField, StreamField
from wagtail.models import Page

from streams.blocks import ICONS, SECTION_BLOCKS


class ServiceIndexPage(Page):
    body = StreamField(SECTION_BLOCKS, blank=True, use_json_field=True, verbose_name="sections")

    content_panels = Page.content_panels + [FieldPanel("body")]
    subpage_types = ["service.ServicePage"]
    max_count = 1

    class Meta:
        verbose_name = "index des services"


class ServicePage(Page):
    summary = models.TextField("résumé")
    icon = models.CharField("icône", max_length=40, choices=ICONS, default="code")
    price = models.CharField("tarif", max_length=60, blank=True)
    features = StreamField([("feature", CharBlock(label="élément"))], blank=True, use_json_field=True, verbose_name="ce qui est inclus")
    body = RichTextField("contenu", features=["h2", "h3", "bold", "italic", "code", "ol", "ul", "link"])

    content_panels = Page.content_panels + [
        FieldPanel("summary"),
        FieldPanel("icon"),
        FieldPanel("price"),
        FieldPanel("features"),
        FieldPanel("body"),
    ]
    parent_page_types = ["service.ServiceIndexPage"]
    subpage_types = []

    class Meta:
        verbose_name = "service"

    @property
    def crumbs(self):
        parent = self.get_parent()
        return [{"label": "Accueil", "url": "/"}, {"label": parent.title, "url": parent.url}]

    def card(self):
        return {
            "title": self.title,
            "url": self.url,
            "summary": self.summary,
            "icon": self.icon,
            "price": self.price,
            "more_label": f"En savoir plus : {self.title}",
        }
'''

BLOG_MODELS = PYGEN + '''from django.db import models
from django.template.defaultfilters import date as date_filter
from wagtail.admin.panels import FieldPanel
from wagtail.fields import RichTextField, StreamField
from wagtail.models import Page

from streams.blocks import SECTION_BLOCKS


class BlogIndexPage(Page):
    body = StreamField(SECTION_BLOCKS, blank=True, use_json_field=True, verbose_name="sections")

    content_panels = Page.content_panels + [FieldPanel("body")]
    subpage_types = ["blog.BlogPage"]
    max_count = 1

    class Meta:
        verbose_name = "index du blog"


class BlogPage(Page):
    date = models.DateField("date de publication")
    category = models.CharField("catégorie", max_length=60, default="Actualités")
    author = models.CharField("auteur", max_length=120, default="La rédaction")
    excerpt = models.TextField("extrait")
    body = RichTextField("contenu", features=["h2", "h3", "bold", "italic", "code", "ol", "ul", "link"])

    content_panels = Page.content_panels + [
        FieldPanel("date"),
        FieldPanel("category"),
        FieldPanel("author"),
        FieldPanel("excerpt"),
        FieldPanel("body"),
    ]
    parent_page_types = ["blog.BlogIndexPage"]
    subpage_types = []

    class Meta:
        verbose_name = "article"

    @property
    def date_iso(self):
        return self.date.isoformat()

    @property
    def date_label(self):
        return date_filter(self.date, "j F Y")

    @property
    def crumbs(self):
        parent = self.get_parent()
        return [{"label": "Accueil", "url": "/"}, {"label": parent.title, "url": parent.url}]

    @property
    def back_url(self):
        return self.get_parent().url

    def card(self):
        return {
            "title": self.title,
            "url": self.url,
            "excerpt": self.excerpt,
            "category": self.category,
            "author": self.author,
            "date_iso": self.date_iso,
            "date_label": self.date_label,
        }
'''

CONTACT_MODELS = PYGEN + '''from django.db import models
from django.template.response import TemplateResponse
from modelcluster.fields import ParentalKey
from wagtail.admin.panels import FieldPanel, FieldRowPanel, InlinePanel, MultiFieldPanel
from wagtail.contrib.forms.models import AbstractEmailForm, AbstractFormField
from wagtail.fields import StreamField

from streams.blocks import SECTION_BLOCKS, ContactBlock

TYPES = {"singleline": "text", "email": "email", "multiline": "textarea"}


class FormField(AbstractFormField):
    page = ParentalKey("ContactPage", on_delete=models.CASCADE, related_name="form_fields")
    autocomplete = models.CharField(max_length=40, blank=True, default="off")

    panels = AbstractFormField.panels + [FieldPanel("autocomplete")]


class ContactPage(AbstractEmailForm):
    """Formulaire de contact (wagtail.contrib.forms) : les envois sont enregistrés (Formulaires dans l'admin)."""

    body = StreamField(SECTION_BLOCKS + [("contact", ContactBlock())], blank=True, use_json_field=True, verbose_name="sections")

    content_panels = AbstractEmailForm.content_panels + [
        FieldPanel("body"),
        InlinePanel("form_fields", label="Champs du formulaire"),
        MultiFieldPanel(
            [FieldRowPanel([FieldPanel("from_address"), FieldPanel("to_address")]), FieldPanel("subject")],
            "E-mail de notification",
        ),
    ]
    max_count = 1

    class Meta:
        verbose_name = "page de contact"

    def get_form(self, *args, **kwargs):
        form = super().get_form(*args, **kwargs)
        for field in form.fields.values():
            field.error_messages["required"] = "Ce champ est obligatoire."
            field.error_messages["invalid"] = "Adresse e-mail invalide."
        return form

    def contact_fields(self, form):
        """Champs normalisés pour le gabarit commun (mêmes clés qu'Astro et Laravel)."""
        out = []
        for f in self.get_form_fields():
            bound = form[f.clean_name]
            errors = form.errors.get(f.clean_name) if form.is_bound else None
            error = errors[0] if errors else None
            field_id = f"contact-{f.clean_name}"
            out.append(
                {
                    "id": field_id,
                    "name": f.clean_name,
                    "label": f.label,
                    "type": TYPES.get(f.field_type, "text"),
                    "autocomplete": f.autocomplete or "off",
                    "required": f.required,
                    "textarea": f.field_type == "multiline",
                    "value": (bound.value() or "") if form.is_bound else "",
                    "error": error,
                    "invalid": "true" if error else None,
                    "error_id": f"{field_id}-error" if error else None,
                }
            )
        return out

    def render_landing_page(self, request, form_submission=None, *args, **kwargs):
        # Même page, avec le message de confirmation et un formulaire vide (comme Astro et Laravel).
        context = self.get_context(request)
        context["form"] = self.get_form(page=self, user=request.user)
        context["sent"] = True
        return TemplateResponse(request, self.get_template(request), context)
'''

def stream_json(sections):
    """Sections de la spec -> données brutes StreamField (liens {label, page, url})."""
    def link(a): return {'label': a['label'], 'page': None, 'url': a['url']} if a else {'label': '', 'page': None, 'url': ''}
    out = []
    for s in sections:
        v = {k: x for k, x in s.items() if k != 'type'}
        for k in ('primary', 'secondary', 'action', 'link'):
            if k in v: v[k] = link(v[k])
        for k in list(v):
            if v[k] is None: v[k] = ''
        if s['type'] == 'contact':
            v = {'title': s['title'], 'success': s['success'], 'submit': s['submit']}
        out.append({'type': s['type'], 'value': v})
    return out

def seed():
    pages = {k: {'title': p['title'], 'description': p['description'], 'slug': p['url'].strip('/'),
                 'body': stream_json(p['sections'])} for k, p in D.PAGES.items()}
    services = [{'slug': s['slug'], 'title': s['title'], 'summary': s['summary'], 'icon': s['icon'], 'price': s['price'],
                 'features': s['features'], 'body': s['body_html']} for s in D.SERVICES]
    posts = [{'slug': p['slug'], 'title': p['title'], 'excerpt': p['excerpt'], 'category': p['category'], 'author': p['author'],
              'date': p['date'], 'body': p['body_html']} for p in sorted(D.POSTS, key=lambda p: p['date'])]
    site = {k: D.SITE[k] for k in ('tagline', 'description', 'copyright', 'cta', 'contact')}
    social = ''.join(f'    {{"icon": {json.dumps(x["icon"])}, "url": {json.dumps(x["url"])}}},  # alpharust-ignore: no-hardcoded-urls\n'
                     for x in D.SITE['social'])
    nav = D.SITE['nav']
    footer = [c for c in D.SITE['footer'] if c['links'] != 'services']
    return PYGEN + '''"""Crée l'arborescence et le contenu du site — mêmes données qu'Astro (src/data, src/content) et Laravel (config/site.php,
SiteSeeder). Idempotent : une page existante (même slug) est conservée.

    python manage.py seed_site
"""
import json
from datetime import date

from django.core.management.base import BaseCommand
from django.db import transaction
from wagtail.models import Site

from blog.models import BlogIndexPage, BlogPage
from contact.models import ContactPage, FormField
from flex.models import FlexPage
from home.models import HomePage
from layout.models import FooterSettings, HeaderSettings, NavigationItem, NavigationMenu
from service.models import ServiceIndexPage, ServicePage
from site_settings.models import ContactSettings, OrganisationSettings, SocialMediaSettings

PAGES = json.loads(r"""''' + json.dumps(pages, ensure_ascii=False, indent=1) + '''""")
SERVICES = json.loads(r"""''' + json.dumps(services, ensure_ascii=False, indent=1) + '''""")
POSTS = json.loads(r"""''' + json.dumps(posts, ensure_ascii=False, indent=1) + '''""")
SITE = json.loads(r"""''' + json.dumps(site, ensure_ascii=False, indent=1) + '''""")
NAV = json.loads(r"""''' + json.dumps(nav, ensure_ascii=False, indent=1) + '''""")
FOOTER = json.loads(r"""''' + json.dumps(footer, ensure_ascii=False, indent=1) + '''""")
SOCIAL = [
''' + social + ''']
FIELDS = json.loads(r"""''' + json.dumps(D.FIELDS, ensure_ascii=False, indent=1) + '''""")
FIELD_TYPES = {"text": "singleline", "email": "email", "textarea": "multiline"}


def publish(page):
    page.save_revision().publish()
    return page


def child(parent, model, slug, **fields):
    existing = parent.get_children().filter(slug=slug).first()
    if existing:
        return existing.specific
    page = model(slug=slug, **fields)
    parent.add_child(instance=page)
    return publish(page)


def section_page(parent, model, key, **extra):
    p = PAGES[key]
    return child(parent, model, p["slug"], title=p["title"], search_description=p["description"],
                 body=json.dumps(p["body"]), **extra)


class Command(BaseCommand):
    help = "Crée les pages, menus et réglages du site (mêmes contenus qu'Astro et Laravel)."

    @transaction.atomic
    def handle(self, *args, **options):
        site = Site.objects.get(is_default_site=True)
        home = site.root_page.specific
        if not isinstance(home, HomePage):
            self.stderr.write("La page racine du site n'est pas une HomePage : rien à faire.")
            return
        if not home.body:
            home.title = PAGES["home"]["title"]
            home.search_description = PAGES["home"]["description"]
            home.body = json.dumps(PAGES["home"]["body"])
            publish(home)
        site.site_name = site.site_name or "@@title@@"
        site.save()

        pages = {"home": home}
        pages["services"] = section_page(home, ServiceIndexPage, "services")
        for s in SERVICES:
            pages[s["slug"]] = child(
                pages["services"], ServicePage, s["slug"], title=s["title"], search_description=s["summary"],
                summary=s["summary"], icon=s["icon"], price=s["price"], body=s["body"],
                features=json.dumps([{"type": "feature", "value": f} for f in s["features"]]),
            )
        pages["blog"] = section_page(home, BlogIndexPage, "blog")
        for p in POSTS:
            child(
                pages["blog"], BlogPage, p["slug"], title=p["title"], search_description=p["excerpt"],
                date=date.fromisoformat(p["date"]), category=p["category"], author=p["author"], excerpt=p["excerpt"],
                body=p["body"],
            )
        pages["about"] = section_page(home, FlexPage, "about")
        contact = PAGES["contact"]
        if not home.get_children().filter(slug=contact["slug"]).exists():
            page = section_page(home, ContactPage, "contact", subject="Nouveau message depuis le site")
            for i, f in enumerate(FIELDS):
                FormField.objects.create(
                    page=page, sort_order=i, label=f["label"], clean_name=f["name"], field_type=FIELD_TYPES[f["type"]],
                    required=f["required"], autocomplete=f["autocomplete"],
                )
        pages["contact"] = home.get_children().get(slug=contact["slug"]).specific
        pages["legal"] = section_page(home, FlexPage, "legal", show_in_menus=False)
        pages["privacy"] = section_page(home, FlexPage, "privacy", show_in_menus=False)

        by_url = {"/" + p.url.strip("/") if p.url != "/" else "/": p for p in pages.values() if hasattr(p, "url")}

        def item(menu, entry, order, parent=None):
            page = by_url.get(entry["url"])
            return NavigationItem.objects.create(
                menu=menu, label=entry["label"], page=page, custom_url="" if page else entry["url"],
                parent=parent, sort_order=order,
            )

        if not NavigationMenu.objects.exists():
            main = NavigationMenu.objects.create(title="Principal")
            for i, entry in enumerate(NAV):
                parent = item(main, entry, i)
                if entry.get("children") == "services":
                    for j, s in enumerate(SERVICES):
                        NavigationItem.objects.create(
                            menu=main, label=s["title"], page=pages[s["slug"]], parent=parent, sort_order=100 + j
                        )
            columns = []
            for col in FOOTER:
                menu = NavigationMenu.objects.create(title=col["title"])
                for i, entry in enumerate(col["links"]):
                    item(menu, entry, i)
                columns.append(menu)
            header = HeaderSettings.load()
            header.selected_menu = main
            header.cta_label = SITE["cta"]["label"]
            header.cta_page = by_url.get(SITE["cta"]["url"])
            header.save()
            footer = FooterSettings.load()
            footer.selected_menu_1, footer.selected_menu_2 = columns[0], columns[-1]
            footer.save()

        org = OrganisationSettings.load()
        if not org.tagline:
            org.tagline, org.description, org.copyright = SITE["tagline"], SITE["description"], SITE["copyright"]
            org.save()
        contact_settings = ContactSettings.load()
        if not contact_settings.email:
            for k in ("email", "phone", "address", "hours"):
                setattr(contact_settings, k, SITE["contact"][k])
            contact_settings.save()
        social = SocialMediaSettings.load()
        if not (social.github or social.linkedin or social.mastodon):
            for s in SOCIAL:
                setattr(social, s["icon"], s["url"])
            social.save()
        self.stdout.write(self.style.SUCCESS("Site prêt : %d pages." % home.get_descendants(inclusive=True).count()))
'''

TESTS = PYGEN + '''from io import StringIO

from django.core.management import call_command
from django.test import TestCase

from contact.models import ContactPage


class SiteTests(TestCase):
    @classmethod
    def setUpTestData(cls):
        call_command("seed_site", stdout=StringIO())

    PAGES = {
        "/": "Des sites rapides, accessibles",
        "/a-propos/": "Une petite équipe",
        "/services/": "Ce que nous faisons",
        "/services/accessibilite/": "Ce qui est inclus",
        "/blog/": "Articles et conseils",
        "/blog/choisir-son-framework/": "Tous les articles",
        "/contact/": "Parlons de votre projet",
        "/mentions-legales/": "Éditeur",
        "/confidentialite/": "Vos droits",
    }

    def test_every_page_renders_inside_the_shared_layout(self):
        for url, text in self.PAGES.items():
            with self.subTest(url=url):
                response = self.client.get(url)
                self.assertEqual(response.status_code, 200)
                html = response.content.decode()
                self.assertIn(text, html)
                for marker in (
                    'href="#contenu"',
                    'aria-label="Navigation principale"',
                    'data-slot="theme-toggle"',
                    'data-slot="sheet-content"',
                    '<footer class="site-footer">',
                ):
                    self.assertIn(marker, html)

    def test_the_current_page_is_marked_in_the_menu(self):
        self.assertContains(self.client.get("/blog/"), 'href="/blog/" aria-current="page"')
        self.assertContains(self.client.get("/services/maintenance/"), 'href="/services/maintenance/" aria-current="page"')

    def test_unknown_pages_use_the_site_404(self):
        response = self.client.get("/nexiste-pas/")
        self.assertEqual(response.status_code, 404)
        self.assertContains(response, "Page introuvable", status_code=404)
        self.assertContains(response, "site-header", status_code=404)

    def test_contact_form_validates_and_stores_submissions(self):
        response = self.client.post("/contact/", {"nom": "", "email": "x"})
        self.assertContains(response, "Ce champ est obligatoire.")
        self.assertContains(response, "Adresse e-mail invalide.")
        response = self.client.post("/contact/", {"nom": "Ada", "email": "ada@example.com", "message": "Bonjour"})
        self.assertContains(response, "Votre message a bien été envoyé")
        self.assertEqual(ContactPage.objects.get().get_submission_class().objects.count(), 1)

    def test_seeding_twice_changes_nothing(self):
        from wagtail.models import Page

        before = Page.objects.count()
        call_command("seed_site", stdout=StringIO())
        self.assertEqual(Page.objects.count(), before)
'''

def files(cat, css):
    out = {
        '@@pkg@@/templates/base.html.tpl': BASE,
        '@@pkg@@/templates/404.html.tpl': not_found(cat),
        '@@pkg@@/templates/includes/header.html.tpl': LOAD + GEN + dj(cat, TPL.HEADER),
        '@@pkg@@/templates/includes/footer.html.tpl': LOAD + GEN + dj(cat, TPL.FOOTER),
        '@@pkg@@/static/css/site.css.tpl': css,
        'home/models.py.tpl': HOME_MODELS,
        'home/templates/home/home_page.html.tpl': '@@meta mode=replace@@\n' + stream_page('{% block title %}{{ site.name }}{% endblock %}\n{% block og_title %}{{ site.name }}{% endblock %}\n'),
        'home/test_site.py.tpl': TESTS,
        'layout/__init__.py.tpl': '',
        'layout/apps.py.tpl': APP('layout', 'Layout'),
        'layout/models.py.tpl': LAYOUT_MODELS,
        'layout/icons.py.tpl': ICONS_PY,
        'layout/context_processors.py.tpl': CONTEXT,
        'layout/migrations/__init__.py.tpl': '',
        'layout/templatetags/__init__.py.tpl': '',
        'layout/templatetags/site_tags.py.tpl': SITE_TAGS,
        'layout/management/__init__.py.tpl': '',
        'layout/management/commands/__init__.py.tpl': '',
        'layout/management/commands/seed_site.py.tpl': seed(),
        'site_settings/__init__.py.tpl': '',
        'site_settings/apps.py.tpl': APP('site_settings', 'SiteSettings'),
        'site_settings/models.py.tpl': SETTINGS_MODELS,
        'site_settings/migrations/__init__.py.tpl': '',
        'streams/__init__.py.tpl': '',
        'streams/apps.py.tpl': APP('streams', 'Streams'),
        'streams/blocks.py.tpl': BLOCKS,
        'flex/__init__.py.tpl': '',
        'flex/apps.py.tpl': APP('flex', 'Flex'),
        'flex/models.py.tpl': FLEX_MODELS,
        'flex/migrations/__init__.py.tpl': '',
        'flex/templates/flex/flex_page.html.tpl': stream_page(),
        'service/__init__.py.tpl': '',
        'service/apps.py.tpl': APP('service', 'Service'),
        'service/models.py.tpl': SERVICE_MODELS,
        'service/migrations/__init__.py.tpl': '',
        'service/templates/service/service_index_page.html.tpl': stream_page(),
        'service/templates/service/service_page.html.tpl': detail(cat, TPL.SERVICE_DETAIL),
        'blog/__init__.py.tpl': '',
        'blog/apps.py.tpl': APP('blog', 'Blog'),
        'blog/models.py.tpl': BLOG_MODELS,
        'blog/migrations/__init__.py.tpl': '',
        'blog/templates/blog/blog_index_page.html.tpl': stream_page(),
        'blog/templates/blog/blog_page.html.tpl': detail(cat, TPL.POST_DETAIL),
        'contact/__init__.py.tpl': '',
        'contact/apps.py.tpl': APP('contact', 'Contact'),
        'contact/models.py.tpl': CONTACT_MODELS,
        'contact/migrations/__init__.py.tpl': '',
        'contact/templates/contact/contact_page.html.tpl': stream_page(),
    }
    for t, nodes in list(TPL.SECTIONS.items()):
        out[f'streams/templates/streams/sections/{t}.html.tpl'] = LOAD + GEN + dj(cat, nodes)
    return out
