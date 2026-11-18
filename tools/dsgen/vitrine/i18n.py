"""Langues du site : chargées depuis vitrine/locales/<code>.py (voir locales/__init__.py pour en ajouter une).

LOCALES : [(code, nom, libellé)] dans l'ordre du sélecteur ; META[code] : métadonnées (sens d'écriture, drapeau,
formats de date) ; UI[clé][code] : libellés d'interface ; TR[code] : traductions des textes de la spec.
Une clé d'interface ou un texte de la spec sans traduction fait échouer gen_ds.py.
"""
from .locales import load

DEFAULT = 'fr'
_MODS = load()
assert _MODS[0].META['code'] == DEFAULT, 'le français (langue source) doit être la première langue (order=1)'
LOCALES = [(m.META['code'], m.META['name'], m.META['label']) for m in _MODS]
CODES = [c for c, _, _ in LOCALES]
META = {m.META['code']: m.META for m in _MODS}
_KEYS = list(_MODS[0].UI)
for _m in _MODS:
    _missing = [k for k in _KEYS if k not in _m.UI]
    if _missing:
        raise KeyError(f"locales/{_m.META['code']}.py : libellés d'interface manquants {_missing}")
UI = {k: {m.META['code']: m.UI[k] for m in _MODS} for k in _KEYS}
TR = {m.META['code']: m.TEXT for m in _MODS if m.META['code'] != DEFAULT}
EN = TR.get('en', {})
