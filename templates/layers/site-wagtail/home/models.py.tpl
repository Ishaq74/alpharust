@@meta mode=replace@@
# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
from wagtail.admin.panels import FieldPanel
from wagtail.fields import StreamField
from wagtail.models import Page

from streams.blocks import SECTION_BLOCKS


class HomePage(Page):
    body = StreamField(SECTION_BLOCKS, blank=True, use_json_field=True, verbose_name="sections")

    content_panels = Page.content_panels + [FieldPanel("body")]
    max_count = 1
