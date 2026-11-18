# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
from django.db import models
from django.template.defaultfilters import date as date_filter
from django.utils.translation import get_language, gettext
from wagtail.admin.panels import FieldPanel
from wagtail.fields import RichTextField, StreamField
from wagtail.models import Page

from streams.blocks import SECTION_BLOCKS

# Format des dates d'articles par langue (filtre date de Django ; mêmes rendus que Carbon et Intl).
DATE_FORMATS = {'fr': 'j F Y', 'en': 'F j, Y', 'es': 'j \\d\\e F \\d\\e Y', 'ar': 'j F Y'}


class BlogIndexPage(Page):
    body = StreamField(SECTION_BLOCKS, blank=True, use_json_field=True, verbose_name="sections")

    content_panels = Page.content_panels + [FieldPanel("body")]
    subpage_types = ["blog.BlogPage"]
    max_count_per_parent = 1  # une par langue (une arborescence par langue)

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
        return date_filter(self.date, DATE_FORMATS.get((get_language() or "fr").split("-")[0], "j F Y"))

    @property
    def crumbs(self):
        parent = self.get_parent()
        home = parent.get_parent()
        return [{"label": gettext("Accueil"), "url": home.url}, {"label": parent.title, "url": parent.url}]

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
