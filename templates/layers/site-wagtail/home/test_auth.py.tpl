@@meta when=auth@@
# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
from django.contrib.auth import get_user_model
from django.core.management import call_command
from django.test import TestCase


class AuthTests(TestCase):
    """Comptes (authentification Django) : mêmes pages, adresses et messages qu'Astro et Laravel."""

    @classmethod
    def setUpTestData(cls):
        call_command("seed_site", verbosity=0)

    def test_the_header_links_to_the_sign_in_page(self):
        self.assertContains(self.client.get("/"), 'href="/auth/sign-in/"')
        self.assertContains(self.client.get("/en/"), 'href="/en/auth/sign-in/"')

    def test_auth_pages_render_in_the_site_layout(self):
        self.assertContains(self.client.get("/auth/sign-in/"), "Accédez à votre espace client.")
        self.assertContains(self.client.get("/auth/sign-up/"), 'autocomplete="new-password"')
        self.assertContains(self.client.get("/en/auth/sign-up/"), "Create my account")
        self.assertRedirects(self.client.get("/auth/dashboard/"), "/auth/sign-in/", fetch_redirect_response=False)

    def test_sign_up_sign_out_and_sign_in(self):
        r = self.client.post("/auth/sign-up/", {"name": "Ada", "email": "Ada@example.com", "password": "motdepasse"})
        self.assertRedirects(r, "/auth/dashboard/", fetch_redirect_response=False)
        self.assertContains(self.client.get("/auth/dashboard/"), "ada@example.com")
        self.assertRedirects(self.client.post("/en/auth/dashboard/"), "/en/", fetch_redirect_response=False)
        r = self.client.post("/en/auth/sign-in/", {"email": "ada@example.com", "password": "motdepasse"})
        self.assertRedirects(r, "/en/auth/dashboard/", fetch_redirect_response=False)

    def test_errors_use_the_site_messages(self):
        get_user_model().objects.create_user("ada@example.com", "ada@example.com", "motdepasse")
        r = self.client.post("/auth/sign-in/", {"email": "ada@example.com", "password": "mauvais"})
        self.assertContains(r, "E-mail ou mot de passe incorrect.", status_code=422)
        r = self.client.post("/auth/sign-up/", {"name": "Ada", "email": "ada@example.com", "password": "motdepasse"})
        self.assertContains(r, "Un compte existe déjà avec cette adresse.", status_code=422)
        r = self.client.post("/en/auth/sign-up/", {"name": "Bob", "email": "bob@example.com", "password": "court"})
        self.assertContains(r, "The password must be at least 8 characters.", status_code=422)
