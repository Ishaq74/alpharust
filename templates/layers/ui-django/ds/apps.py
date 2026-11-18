from django.apps import AppConfig


class DsConfig(AppConfig):
    """Catalogue de composants (balises de gabarit `{% load ds %}` + fichiers statiques `ds/`)."""

    name = "ds"
    verbose_name = "Design system"
