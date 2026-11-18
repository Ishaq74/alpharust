# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
"""Blocs StreamField : une section du site par bloc (mêmes types que src/data/site.json et config/site.php).

Chaque bloc expose à son gabarit (streams/sections/<type>.html) une variable ``s`` normalisée, identique
aux autres frameworks ; les blocs « services » et « posts » y ajoutent les pages correspondantes.
"""
from wagtail import blocks
from wagtail.models import Locale
from wagtail.rich_text import RichText

from layout.context_processors import href, localize

ICONS = [("accessibility", "accessibility"), ("code", "code"), ("heart", "heart"), ("leaf", "leaf"), ("pencil", "pencil"), ("shield", "shield"), ("users", "users"), ("zap", "zap")]


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
            target = value["page"].localized.url if value.get("page") else localize(value.get("url"))
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
        pages = ServicePage.objects.live().filter(locale=Locale.get_active()).order_by("path")
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
        pages = BlogPage.objects.live().filter(locale=Locale.get_active()).order_by("-date")
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
