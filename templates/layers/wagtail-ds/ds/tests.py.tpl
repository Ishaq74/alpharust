import re

from django.test import TestCase


def tag_for(html, slot, variant):
    """Balise ouvrante portant data-slot et data-variant (l'ordre des attributs n'est pas garanti)."""
    for tag in re.findall(r"<[a-z]+ [^>]*>", html):
        if f'data-slot="{slot}"' in tag and f'data-variant="{variant}"' in tag:
            return tag
    raise AssertionError(f"balise {slot}/{variant} introuvable")


class ComponentCatalogTests(TestCase):
    def test_catalog_renders_every_core_slot(self):
        html = self.client.get("/components/").content.decode()
        for slot in ["button", "card", "alert", "tabs", "dialog-content", "native-select", "checkbox-input", "switch-button", "table", "pagination-link"]:
            self.assertIn(f'data-slot="{slot}"', html)

    def test_alert_role_follows_the_variant(self):
        html = self.client.get("/components/").content.decode()
        self.assertIn('role="alert"', tag_for(html, "alert", "error"))
        self.assertIn('role="status"', tag_for(html, "alert", "info"))

    def test_button_with_href_is_a_link(self):
        html = self.client.get("/components/").content.decode()
        self.assertRegex(html, r'<a [^>]*data-slot="button"')

    def test_design_assets_are_linked(self):
        html = self.client.get("/components/").content.decode()
@@if starwind@@
        self.assertIn("css/tailwind.css", html)  # kit Starwind compilé par la CLI Tailwind
@@else@@
        self.assertIn("ds/css/index.css", html)
@@end@@
        self.assertIn("ds/js/ds.js", html)


class AdvancedCatalogTests(TestCase):
    EXTENDED = ["alert-dialog", "aspect-ratio", "button-group", "carousel", "collapsible", "container", "dropdown", "dropzone", "image", "input-group", "input-otp", "item", "popover", "prose", "radio-group", "select", "sheet", "sidebar", "slider", "theme-toggle", "toast", "toggle", "video"]

    def setUp(self):
        self.html = self.client.get("/components/advanced/").content.decode()

    def test_renders_the_23_extended_components(self):
        for slot in self.EXTENDED + ["alert-dialog-content", "select-content", "select-item", "sheet-content", "sidebar-menu-button", "toast-viewport", "input-otp-slot", "carousel-next"]:
            self.assertIn(f'data-slot="{slot}"', self.html)

    def test_extended_components_keep_their_accessibility_contract(self):
        self.assertRegex(self.html, r'role="combobox"[^>]*aria-label="Fruit"')
        self.assertRegex(self.html, r'data-slot="slider-thumb"[^>]*role="slider"[^>]*aria-valuenow="40"')
        self.assertRegex(self.html, r'<a [^>]*data-slot="sidebar-menu-button"[^>]*aria-current="page"')
        self.assertIn("youtube-nocookie.com/embed/aqz-KE-bpKQ", self.html)
        self.assertIn('<template data-toast-template="success"', self.html)
