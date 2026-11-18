@@meta mode=replace@@
# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
from wagtail.admin.panels import FieldPanel
@@if auth@@
from wagtail.contrib.routable_page.models import RoutablePageMixin, path
@@end@@
from wagtail.fields import StreamField
from wagtail.models import Page

@@if auth@@
from layout import auth
@@end@@
from streams.blocks import SECTION_BLOCKS


@@if auth@@
class HomePage(RoutablePageMixin, Page):
@@else@@
class HomePage(Page):
@@end@@
    body = StreamField(SECTION_BLOCKS, blank=True, use_json_field=True, verbose_name="sections")

    content_panels = Page.content_panels + [FieldPanel("body")]
@@if auth@@

    # Comptes (authentification Django) : /auth/… sous l'accueil de chaque langue, comme Astro et Laravel.
    @path("auth/sign-in/")
    def sign_in(self, request):
        return auth.sign_in(request)

    @path("auth/sign-up/")
    def sign_up(self, request):
        return auth.sign_up(request)

    @path("auth/dashboard/")
    def account(self, request):
        return auth.account(request)
@@end@@
    # une page d’accueil par langue (racine du site et ses traductions)
