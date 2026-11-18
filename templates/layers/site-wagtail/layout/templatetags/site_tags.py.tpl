# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
from django import template
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
