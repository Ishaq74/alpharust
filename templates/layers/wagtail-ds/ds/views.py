from django.views.generic import TemplateView

VARIANTS = ["default", "primary", "secondary", "outline", "ghost", "info", "success", "warning", "error"]
PIXEL = "data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7"


class ComponentsView(TemplateView):
    """Catalogue de composants (même contenu que les pages Astro et Blade)."""

    template_name = "ds/components.html"
    extra_context = {"variants": VARIANTS, "pixel": PIXEL}
