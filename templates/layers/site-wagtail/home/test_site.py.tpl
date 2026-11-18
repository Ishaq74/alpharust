# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
from io import StringIO

from django.core.management import call_command
from django.test import TestCase

from contact.models import ContactPage


class SiteTests(TestCase):
    @classmethod
    def setUpTestData(cls):
        call_command("seed_site", stdout=StringIO())

    PAGES = {
        "/": "Des sites rapides, accessibles",
        "/a-propos/": "Une petite équipe",
        "/services/": "Ce que nous faisons",
        "/services/accessibilite/": "Ce qui est inclus",
        "/blog/": "Articles et conseils",
        "/blog/choisir-son-framework/": "Tous les articles",
        "/contact/": "Parlons de votre projet",
        "/mentions-legales/": "Éditeur",
        "/confidentialite/": "Vos droits",
    }

    def test_every_page_renders_inside_the_shared_layout(self):
        for url, text in self.PAGES.items():
            with self.subTest(url=url):
                response = self.client.get(url)
                self.assertEqual(response.status_code, 200)
                html = response.content.decode()
                self.assertIn(text, html)
                for marker in (
                    'href="#contenu"',
                    'aria-label="Navigation principale"',
                    'data-slot="theme-toggle"',
                    'data-slot="sheet-content"',
                    '<footer class="site-footer">',
                ):
                    self.assertIn(marker, html)

    def test_the_current_page_is_marked_in_the_menu(self):
        self.assertContains(self.client.get("/blog/"), 'href="/blog/" aria-current="page"')
        self.assertContains(self.client.get("/services/maintenance/"), 'href="/services/maintenance/" aria-current="page"')

    def test_unknown_pages_use_the_site_404(self):
        response = self.client.get("/nexiste-pas/")
        self.assertEqual(response.status_code, 404)
        self.assertContains(response, "Page introuvable", status_code=404)
        self.assertContains(response, "site-header", status_code=404)

    def test_contact_form_validates_and_stores_submissions(self):
        response = self.client.post("/contact/", {"nom": "", "email": "x"})
        self.assertContains(response, "Ce champ est obligatoire.")
        self.assertContains(response, "Adresse e-mail invalide.")
        response = self.client.post("/contact/", {"nom": "Ada", "email": "ada@example.com", "message": "Bonjour"})
        self.assertContains(response, "Votre message a bien été envoyé")
        self.assertEqual(ContactPage.objects.get().get_submission_class().objects.count(), 1)

    def test_seeding_twice_changes_nothing(self):
        from wagtail.models import Page

        before = Page.objects.count()
        call_command("seed_site", stdout=StringIO())
        self.assertEqual(Page.objects.count(), before)
