"""Langues du site vitrine : un fichier par langue (`<code>.py`), chargé automatiquement.

Chaque fichier déclare :
  META : code, name (nom dans la langue), label (sélecteur), dir ('ltr' ou 'rtl'), flag (icône Circle Flags),
         order (position dans le sélecteur), date_intl (Astro, Intl.DateTimeFormat), date_php (Carbon translatedFormat),
         date_django (filtre date) ;
  UI   : libellés d'interface (mêmes clés que fr.py) ;
  TEXT : textes de la spec (clé : texte français de spec.py) -> traduction (absent de fr.py, langue source).

Ajouter une langue : copier en.py en `<code>.py`, traduire, ajouter le drapeau dans iconsets.FLAGS, puis lancer
`python3 tools/dsgen/gen_ds.py` (une traduction manquante fait échouer la génération).
"""
import importlib
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def load():
    codes = sorted(f[:-3] for f in os.listdir(HERE) if f.endswith('.py') and not f.startswith('_'))
    mods = [importlib.import_module(f'{__name__}.{c}') for c in codes]
    for c, m in zip(codes, mods):
        assert m.META['code'] == c, f'locales/{c}.py : META.code doit valoir {c!r}'
    return sorted(mods, key=lambda m: m.META['order'])
