"""Chemins et utilitaires partagés par gen_ds.py et ses modules (ext_astro*, ext_write, ext_server)."""
import os
import re

from spec import SIMPLE, SPECIAL

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, '..', '..', 'templates', 'layers'))
ATOMIC = ROOT + '/ui-atomic/src/components/atoms'
OUT_ASTRO = ROOT + '/ui-css/src/components/atoms'
OUT_BLADE = ROOT + '/ui-blade/resources/views/components/ds'
OUT_DJ = ROOT + '/ui-django/ds/templatetags/ds.py'


def kebab(s): return re.sub(r'(?<!^)(?=[A-Z])', '-', s).lower()
def snake(s): return kebab(s).replace('-', '_')
def pascal_comp(c): return ''.join(w.capitalize() for w in c.split('-'))
def attr_name_astro(k): return k
def rest_first(text):
    # Astro ne fusionne pas les attributs en double et le navigateur garde le PREMIER : comme dans atomic,
    # `{...rest}` passe avant `data-slot` pour qu'un composant parent puisse le surcharger
    # (Sheet -> Dialog, InputGroupInput -> Input, SidebarInput -> Input…).
    # Éléments HTML seulement (minuscule ou `Tag` dynamique) : pour un composant, la dernière prop l'emporte.
    return re.sub(r'(<(?:[a-z][a-z0-9]*|Tag)\b[^<>]*?)(data-slot="[a-z-]+")([^<>]*?) \{\.\.\.rest\}',
                  r'\1{...rest} \2\3', text)

def w(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    if path.endswith('.astro'):
        text = rest_first(text)
    open(path, 'w').write(text)

def part_rel(comp, name):
    base = pascal_comp(comp)
    if name == base: return ''
    assert name.startswith(base), (comp, name)
    return kebab(name[len(base):])


def core_components():
    """Composants « core » (spec.py) et leurs parties : {composant: [parties]}."""
    parts_by_comp = {}
    for p in SIMPLE:
        parts_by_comp.setdefault(p['comp'], []).append(p['name'])
    for comp, names in SPECIAL.items():
        parts_by_comp.setdefault(comp, []).extend(names)
    return parts_by_comp


def ext_components():
    """Composants « étendus » : présents dans les atomes d'atomic, hors core."""
    core = core_components()
    return sorted(d for d in os.listdir(ATOMIC) if os.path.isdir(f'{ATOMIC}/{d}') and d not in core)
