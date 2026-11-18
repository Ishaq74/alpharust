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
@@if lang_en@@
        self.assertContains(self.client.get("/en/"), 'href="/en/auth/sign-in/"')
@@end@@
@@if lang_ar@@
        self.assertContains(self.client.get("/ar/"), 'href="/ar/auth/sign-in/"')
@@end@@

    def test_auth_pages_render_in_the_site_layout(self):
        self.assertContains(self.client.get("/auth/sign-in/"), "Accédez à votre espace client.")
        self.assertContains(self.client.get("/auth/sign-up/"), 'autocomplete="new-password"')
@@if lang_en@@
        self.assertContains(self.client.get("/en/auth/sign-up/"), "Create my account")
@@end@@
@@if lang_es@@
        self.assertContains(self.client.get("/es/auth/sign-up/"), "Crear mi cuenta")
@@end@@
        self.assertRedirects(self.client.get("/auth/dashboard/"), "/auth/sign-in/", fetch_redirect_response=False)

    def test_sign_up_sign_out_and_sign_in(self):
        r = self.client.post("/auth/sign-up/", {"name": "Ada", "email": "Ada@example.com", "password": "motdepasse"})
        self.assertRedirects(r, "/auth/dashboard/", fetch_redirect_response=False)
        self.assertContains(self.client.get("/auth/dashboard/"), "ada@example.com")
@@if lang_en@@
        self.assertRedirects(self.client.post("/en/auth/dashboard/"), "/en/", fetch_redirect_response=False)
        r = self.client.post("/en/auth/sign-in/", {"email": "ada@example.com", "password": "motdepasse"})
        self.assertRedirects(r, "/en/auth/dashboard/", fetch_redirect_response=False)
@@else@@
        self.assertRedirects(self.client.post("/auth/dashboard/"), "/", fetch_redirect_response=False)
        r = self.client.post("/auth/sign-in/", {"email": "ada@example.com", "password": "motdepasse"})
        self.assertRedirects(r, "/auth/dashboard/", fetch_redirect_response=False)
@@end@@

    def test_errors_use_the_site_messages(self):
        get_user_model().objects.create_user("ada@example.com", "ada@example.com", "motdepasse")
        r = self.client.post("/auth/sign-in/", {"email": "ada@example.com", "password": "mauvais"})
        self.assertContains(r, "E-mail ou mot de passe incorrect.", status_code=422)
        r = self.client.post("/auth/sign-up/", {"name": "Ada", "email": "ada@example.com", "password": "motdepasse"})
        self.assertContains(r, "Un compte existe déjà avec cette adresse.", status_code=422)
@@if lang_en@@
        r = self.client.post("/en/auth/sign-up/", {"name": "Bob", "email": "bob@example.com", "password": "court"})
        self.assertContains(r, "The password must be at least 8 characters.", status_code=422)
@@end@@
@@if lang_ar@@
        r = self.client.post("/ar/auth/sign-up/", {"name": "Bob", "email": "bob@example.com", "password": "court"})
        self.assertContains(r, "يجب أن تتكوّن كلمة المرور من 8 أحرف على الأقل.", status_code=422)
@@end@@
