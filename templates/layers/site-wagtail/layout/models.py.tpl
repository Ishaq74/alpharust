# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
"""En-tête et pied de page administrables (modèle ishaq74/wagtail-cms : layout/models.py).

- ``NavigationMenu`` (snippet) : un menu et ses éléments, avec sous-menus (``parent``) ;
- ``HeaderSettings`` / ``FooterSettings`` (réglages du site) : menus affichés, bouton d'appel à l'action.
"""
from django.db import models
from modelcluster.fields import ParentalKey
from modelcluster.models import ClusterableModel
from wagtail.admin.panels import FieldPanel, InlinePanel, PageChooserPanel
from wagtail.contrib.settings.models import BaseGenericSetting, register_setting
from wagtail.models import Orderable, Page, TranslatableMixin
from wagtail.snippets.models import register_snippet


@register_snippet
class NavigationMenu(TranslatableMixin, ClusterableModel):
    """Menu traduisible (internationalisation de Wagtail) : une copie par langue, même ``translation_key``."""

    title = models.CharField("titre", max_length=255, help_text="Nom du menu, affiché comme titre de colonne dans le pied de page.")

    panels = [FieldPanel("title"), InlinePanel("items", label="Éléments du menu")]

    class Meta(TranslatableMixin.Meta):
        verbose_name = "menu de navigation"
        verbose_name_plural = "menus de navigation"

    def __str__(self):
        return self.title


class NavigationItem(Orderable):
    menu = ParentalKey(NavigationMenu, on_delete=models.CASCADE, related_name="items")
    label = models.CharField("libellé", max_length=255)
    page = models.ForeignKey(Page, null=True, blank=True, on_delete=models.SET_NULL, related_name="+")
    custom_url = models.CharField("lien personnalisé", max_length=500, blank=True, help_text="Adresse ou chemin si aucune page n'est choisie.")
    parent = models.ForeignKey(
        "self", null=True, blank=True, on_delete=models.CASCADE, related_name="children", help_text="Élément parent (sous-menu)."
    )

    panels = [FieldPanel("label"), PageChooserPanel("page"), FieldPanel("custom_url"), FieldPanel("parent")]

    def get_url(self):
        return self.page.url if self.page else self.custom_url

    def __str__(self):
        return self.label


@register_setting(icon="list-ul")
class HeaderSettings(BaseGenericSetting):
    selected_menu = models.ForeignKey(
        NavigationMenu, null=True, blank=True, on_delete=models.SET_NULL, related_name="+", verbose_name="menu principal"
    )
    cta_label = models.CharField("bouton : libellé", max_length=60, blank=True)
    cta_page = models.ForeignKey(Page, null=True, blank=True, on_delete=models.SET_NULL, related_name="+", verbose_name="bouton : page")

    panels = [FieldPanel("selected_menu"), FieldPanel("cta_label"), PageChooserPanel("cta_page")]

    class Meta:
        verbose_name = "en-tête"


@register_setting(icon="list-ul")
class FooterSettings(BaseGenericSetting):
    selected_menu_1 = models.ForeignKey(
        NavigationMenu, null=True, blank=True, on_delete=models.SET_NULL, related_name="+", verbose_name="première colonne"
    )
    services_title = models.CharField(
        "colonne des services", max_length=60, blank=True, default="Services", help_text="Vide : colonne masquée."
    )
    selected_menu_2 = models.ForeignKey(
        NavigationMenu, null=True, blank=True, on_delete=models.SET_NULL, related_name="+", verbose_name="troisième colonne"
    )

    panels = [FieldPanel("selected_menu_1"), FieldPanel("services_title"), FieldPanel("selected_menu_2")]

    class Meta:
        verbose_name = "pied de page"
