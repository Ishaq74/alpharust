"""Fichiers de référence : la sortie des générateurs du site est identique aux couches committées."""
import os

import pytest

from common import ROOT
from vitrine import data as D, write_astro, write_laravel, write_wagtail
from vitrine.ir import Catalog
from vitrine.write import HERE

CSS = open(os.path.join(HERE, "site.css")).read()


@pytest.mark.parametrize("layer,mod", [("site-astro", write_astro), ("site-laravel", write_laravel),
                                       ("site-wagtail", write_wagtail)])
def test_generated_site_layer_matches_the_committed_files(layer, mod):
    out = mod.files(Catalog(ROOT), CSS)
    on_disk = set()
    for base, _, names in os.walk(os.path.join(ROOT, layer)):
        on_disk |= {os.path.relpath(os.path.join(base, n), os.path.join(ROOT, layer)) for n in names}
    assert set(out) == on_disk
    for rel, text in out.items():
        with open(os.path.join(ROOT, layer, rel)) as f:
            assert f.read() == text, f"{layer}/{rel} : relancer python3 tools/dsgen/gen_ds.py"


def test_the_three_frameworks_share_seo_and_csp():
    cat = Catalog(ROOT)
    astro, laravel, wagtail = (m.files(cat, CSS) for m in (write_astro, write_laravel, write_wagtail))
    assert "src/pages/sitemap.xml.ts.tpl" in astro and "src/pages/robots.txt.ts.tpl" in astro
    assert "app/Http/Controllers/SeoController.php.tpl" in laravel
    assert "layout/seo.py.tpl" in wagtail
    assert D.CSP in laravel["app/Http/Middleware/SecurityHeaders.php.tpl"]
    assert D.CSP.replace("; frame-ancestors 'self'", "") in astro["src/layouts/BaseLayout.astro.tpl"]
    for text in (astro["src/lib/site.ts.tpl"], laravel["app/Support/SiteData.php.tpl"],
                 wagtail["layout/context_processors.py.tpl"]):
        assert "x-default" in text and "Organization" in text
