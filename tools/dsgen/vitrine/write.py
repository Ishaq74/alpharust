"""Écrit les couches site-astro, site-laravel et site-wagtail à partir de spec.py (appelé par gen_ds.py)."""
import os, shutil
from . import write_astro, write_laravel, write_wagtail
from .ir import Catalog

HERE = os.path.dirname(os.path.abspath(__file__))

def write_all(root):
    cat = Catalog(root)
    css = open(os.path.join(HERE, 'site.css')).read()
    for layer, mod in (('site-astro', write_astro), ('site-laravel', write_laravel), ('site-wagtail', write_wagtail)):
        out = os.path.join(root, layer)
        shutil.rmtree(out, ignore_errors=True)
        for rel, text in mod.files(cat, css).items():
            path = os.path.join(out, rel)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, 'w') as f:
                f.write(text)
