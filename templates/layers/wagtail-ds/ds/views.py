from django.views.generic import TemplateView

VARIANTS = ["default", "primary", "secondary", "outline", "ghost", "info", "success", "warning", "error"]
SLIDES = ["Concevoir", "Construire", "Vérifier", "Publier"]
LOGO = (
    "data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20viewBox%3D%220%200%2048%2048%22%3E"
    "%3Crect%20width%3D%2248%22%20height%3D%2248%22%20rx%3D%2210%22%20fill%3D%22%233b4cca%22%2F%3E"
    "%3Cpath%20d%3D%22M15%2036%2024%2012l9%2024m-15-8h12%22%20stroke%3D%22%23fff%22%20stroke-width%3D%224%22%20fill%3D%22none%22%2F%3E%3C%2Fsvg%3E"
)
PIXEL = "data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7"


class ComponentsView(TemplateView):
    """Catalogue de composants (même contenu que les pages Astro et Blade)."""

    template_name = "ds/components.html"
    extra_context = {"variants": VARIANTS, "pixel": PIXEL}


class AdvancedComponentsView(TemplateView):
    """Composants étendus (fenêtres, saisie, barre latérale, notifications…) : même contenu que /components/advanced en Astro et Blade."""

    template_name = "ds/components_advanced.html"
    extra_context = {"slides": SLIDES, "logo": LOGO}
