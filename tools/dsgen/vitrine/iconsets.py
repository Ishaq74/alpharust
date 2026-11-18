"""Jeux d'icônes du site (Iconify) : nom logique -> nom dans chaque jeu.

Défaut : MDI (Material Design Icons, @iconify-json/mdi) pour les pictogrammes, Circle Flags pour les drapeaux du
sélecteur de langue. Lucide est fourni aussi ; tout autre jeu Iconify se choisit à la génération
(`alpharust generate … --icons <jeu>`) : chaque nom logique est cherché parmi CANDIDATES, et l'icône MDI est gardée
quand le jeu n'en a aucune.

Les tracés sont copiés dans iconsets.json (sous-ensemble utile, licences comprises) par fetch_icons.py : la génération
des couches (gen_ds.py) ne dépend ni du réseau ni de node_modules.
"""
import json, os
from .locales import load as _locales

DEFAULT_SET = 'mdi'
FLAG_SET = 'circle-flags'
# Jeux copiés dans iconsets.json (le premier est le défaut).
BUNDLED = ['mdi', 'lucide']

# nom logique -> nom par jeu ; les noms absents d'un jeu tombent sur CANDIDATES puis sur MDI.
MAP = {
    'menu':          {'mdi': 'menu',                    'lucide': 'menu'},
    'chevron-down':  {'mdi': 'chevron-down',            'lucide': 'chevron-down'},
    'arrow-right':   {'mdi': 'arrow-right',             'lucide': 'arrow-right'},
    'arrow-left':    {'mdi': 'arrow-left',              'lucide': 'arrow-left'},
    'check':         {'mdi': 'check',                   'lucide': 'check'},
    'quote':         {'mdi': 'format-quote-open',       'lucide': 'quote'},
    'mail':          {'mdi': 'email-outline',           'lucide': 'mail'},
    'phone':         {'mdi': 'phone-outline',           'lucide': 'phone'},
    'map-pin':       {'mdi': 'map-marker-outline',      'lucide': 'map-pin'},
    'clock':         {'mdi': 'clock-outline',           'lucide': 'clock'},
    'calendar':      {'mdi': 'calendar-blank-outline',  'lucide': 'calendar'},
    'send':          {'mdi': 'send-outline',            'lucide': 'send'},
    'code':          {'mdi': 'code-tags',               'lucide': 'code'},
    'zap':           {'mdi': 'lightning-bolt-outline',  'lucide': 'zap'},
    'accessibility': {'mdi': 'human',                   'lucide': 'accessibility'},
    'shield':        {'mdi': 'shield-check-outline',    'lucide': 'shield'},
    'pencil':        {'mdi': 'pencil-outline',          'lucide': 'pencil'},
    'heart':         {'mdi': 'heart-outline',           'lucide': 'heart'},
    'leaf':          {'mdi': 'leaf',                    'lucide': 'leaf'},
    'users':         {'mdi': 'account-group-outline',   'lucide': 'users'},
    'user':          {'mdi': 'account-circle-outline',  'lucide': 'circle-user'},
    'log-out':       {'mdi': 'logout',                  'lucide': 'log-out'},
    'github':        {'mdi': 'github',                  'lucide': 'github'},
    'linkedin':      {'mdi': 'linkedin',                'lucide': 'linkedin'},
    'mastodon':      {'mdi': 'mastodon',                'lucide': 'mastodon'},
}
# Noms essayés, dans l'ordre, pour un jeu Iconify quelconque (tabler, ph, heroicons, openmoji…).
CANDIDATES = {
    'menu': ['menu', 'menu-2', 'hamburger', 'list'], 'chevron-down': ['chevron-down', 'caret-down', 'angle-down'],
    'arrow-right': ['arrow-right'], 'arrow-left': ['arrow-left'], 'check': ['check', 'checkmark', 'tick'],
    'quote': ['quote', 'quotes', 'blockquote', 'format-quote'], 'mail': ['mail', 'email', 'envelope', 'envelope-simple'],
    'phone': ['phone', 'telephone'], 'map-pin': ['map-pin', 'location', 'map-marker', 'pin'], 'clock': ['clock', 'time'],
    'calendar': ['calendar', 'calendar-blank'], 'send': ['send', 'paper-plane', 'paper-airplane'],
    'code': ['code', 'code-tags', 'brackets'], 'zap': ['zap', 'bolt', 'lightning', 'flash'],
    'accessibility': ['accessibility', 'accessible', 'human', 'person'], 'shield': ['shield', 'shield-check'],
    'pencil': ['pencil', 'edit', 'pen'], 'heart': ['heart'], 'leaf': ['leaf', 'plant'], 'users': ['users', 'people', 'user-group'],
    'user': ['user', 'user-circle', 'account', 'person'], 'log-out': ['log-out', 'logout', 'sign-out', 'exit'],
    'github': ['github', 'brand-github', 'github-logo'], 'linkedin': ['linkedin', 'brand-linkedin', 'linkedin-logo'],
    'mastodon': ['mastodon', 'brand-mastodon', 'mastodon-logo'],
}
# Drapeau de chaque langue : META['flag'] de locales/<code>.py (l'arabe, langue de nombreux pays, prend le drapeau
# de langue lang-ar plutôt qu'un drapeau national).
FLAGS = {m.META['code']: m.META['flag'] for m in _locales()}

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, 'iconsets.json')

def load():
    with open(DATA) as f:
        return json.load(f)

def resolve(data, set_name=DEFAULT_SET):
    """Icônes logiques d'un jeu copié : {nom: {body, width, height}} (repli MDI)."""
    out = {}
    for logical, names in MAP.items():
        for s in (set_name, DEFAULT_SET):
            entry = data['sets'][s]['icons'].get(names.get(s, ''))
            if entry:
                out[logical] = entry
                break
        else:
            raise KeyError(f'icône {logical!r} absente de {set_name} et de {DEFAULT_SET}')
    return out

def flags(data, codes):
    return {f'flag-{c}': data['sets'][FLAG_SET]['icons'][FLAGS[c]] for c in codes}
