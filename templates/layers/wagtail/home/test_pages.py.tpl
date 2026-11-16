from django.test import TestCase
from wagtail.models import Site

from home.models import HomePage


class HomePageTests(TestCase):
    def test_default_site_points_to_a_home_page(self):
        site = Site.objects.get(is_default_site=True)
        self.assertIsInstance(site.root_page.specific, HomePage)

    def test_homepage_renders(self):
        response = self.client.get("/")
        self.assertEqual(response.status_code, 200)

    def test_admin_requires_login(self):
        response = self.client.get("/admin/")
        self.assertEqual(response.status_code, 302)
