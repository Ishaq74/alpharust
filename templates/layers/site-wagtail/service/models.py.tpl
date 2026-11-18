# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
from django.db import models
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
