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

    def test_seo_robots_sitemap_and_structured_data(self):
        robots = self.client.get("/robots.txt")
        self.assertContains(robots, "Sitemap: http://testserver/sitemap.xml")
        sitemap = self.client.get("/sitemap.xml")
        self.assertEqual(sitemap.status_code, 200)
        self.assertIn("/blog/choisir-son-framework/</loc>", sitemap.content.decode())
        response = self.client.get("/blog/choisir-son-framework/?utm=x")
        self.assertContains(response, '<link rel="canonical" href="http://testserver/blog/choisir-son-framework/" />')
        self.assertContains(response, '"@type": "Organization"')
        self.assertContains(response, '"@type": "BlogPosting"')

    def test_contact_form_validates_and_stores_submissions(self):
        response = self.client.post("/contact/", {"nom": "", "email": "x"})
        self.assertContains(response, "Ce champ est obligatoire.")
        self.assertContains(response, "Adresse e-mail invalide.")
        response = self.client.post("/contact/", {"nom": "Ada", "email": "ada@example.com", "message": "Bonjour"})
        self.assertContains(response, "Votre message a bien été envoyé")
        self.assertEqual(ContactPage.objects.get(locale__language_code="fr").get_submission_class().objects.count(), 1)

@@if multilingual@@
    # Une ligne par langue choisie (--langs) : (langue, sens, adresse, texte attendu, libellé de la navigation).
    TRANSLATED = [
@@if lang_en@@
        ("en", "ltr", "/en/", "Fast, accessible websites that are easy to grow", "Main navigation"),
        ("en", "ltr", "/en/a-propos/", "A small team with high standards", "Main navigation"),
        ("en", "ltr", "/en/services/accessibilite/", "What’s included", "Main navigation"),
        ("en", "ltr", "/en/blog/choisir-son-framework/", "All articles", "Main navigation"),
        ("en", "ltr", "/en/contact/", "Let’s talk about your project", "Main navigation"),
@@end@@
@@if lang_es@@
        ("es", "ltr", "/es/", "Sitios rápidos, accesibles y fáciles de hacer evolucionar", "Navegación principal"),
        ("es", "ltr", "/es/a-propos/", "Un equipo pequeño, con grandes exigencias", "Navegación principal"),
        ("es", "ltr", "/es/services/accessibilite/", "Qué incluye", "Navegación principal"),
        ("es", "ltr", "/es/blog/choisir-son-framework/", "Todos los artículos", "Navegación principal"),
        ("es", "ltr", "/es/contact/", "Hablemos de su proyecto", "Navegación principal"),
@@end@@
@@if lang_ar@@
        ("ar", "rtl", "/ar/", "مواقع سريعة وسهلة الوصول ويسهل تطويرها", "التنقل الرئيسي"),
        ("ar", "rtl", "/ar/a-propos/", "فريق صغير بمعايير عالية", "التنقل الرئيسي"),
        ("ar", "rtl", "/ar/services/accessibilite/", "ما تتضمنه الخدمة", "التنقل الرئيسي"),
        ("ar", "rtl", "/ar/blog/choisir-son-framework/", "جميع المقالات", "التنقل الرئيسي"),
        ("ar", "rtl", "/ar/contact/", "لنتحدث عن مشروعكم", "التنقل الرئيسي"),
@@end@@
    ]

    def test_translated_pages(self):
        for code, direction, url, text, nav in self.TRANSLATED:
            with self.subTest(url=url):
                response = self.client.get(url)
                self.assertEqual(response.status_code, 200)
                html = response.content.decode()
                self.assertIn(f'<html lang="{code}" dir="{direction}">', html)
                self.assertIn(text, html)
                self.assertIn(f'aria-label="{nav}"', html)
                self.assertIn('hreflang="fr"', html)

    SWITCHER = [
@@if lang_en@@
        ("en", "Page not found"),
@@end@@
@@if lang_es@@
        ("es", "Página no encontrada"),
@@end@@
@@if lang_ar@@
        ("ar", "الصفحة غير موجودة"),
@@end@@
    ]

    def test_the_language_switcher_links_to_the_same_page(self):
        for code, not_found in self.SWITCHER:
            with self.subTest(code=code):
                self.assertContains(self.client.get("/blog/"), f'href="/{code}/blog/"')
                self.assertContains(self.client.get(f"/{code}/blog/"), 'href="/blog/"')
                self.assertContains(self.client.get(f"/{code}/nope/"), not_found, status_code=404)
@@end@@

    def test_seeding_twice_changes_nothing(self):
        from wagtail.models import Page

        before = Page.objects.count()
        call_command("seed_site", stdout=StringIO())
        self.assertEqual(Page.objects.count(), before)
