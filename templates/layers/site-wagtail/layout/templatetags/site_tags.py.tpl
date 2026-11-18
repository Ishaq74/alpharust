# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
from django import template
from django.utils.html import format_html
from django.utils.safestring import mark_safe

from layout.icons import ICONS

register = template.Library()


@register.simple_tag
def site_icon(name, size=20):
    """Icône du site par son nom (``{% site_icon "mail" 18 %}``)."""
    icon = ICONS.get(name)
    if not icon:
        return ""
    return format_html(
        '<svg width="{}" height="{}" viewBox="0 0 {} {}" fill="currentColor" aria-hidden="true" class="site-icon" data-icon="{}">{}</svg>',
        size,
        size,
        icon["width"],
        icon["height"],
        name,
        mark_safe(icon["body"]),
    )
