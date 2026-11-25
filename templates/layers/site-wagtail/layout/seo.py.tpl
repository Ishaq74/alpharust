# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
"""robots.txt servi par le site (comme Astro et Laravel) ; le plan du site est wagtail.contrib.sitemaps (/sitemap.xml)."""
from django.http import HttpResponse
from django.views.decorators.http import require_GET


@require_GET
def robots_txt(request):
    sitemap = request.build_absolute_uri("/sitemap.xml")
    return HttpResponse(f"User-agent: *\nAllow: /\n\nSitemap: {sitemap}\n", content_type="text/plain; charset=utf-8")
