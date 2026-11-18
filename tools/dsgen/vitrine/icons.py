"""Icônes du site : MDI par défaut (pictogrammes) et Circle Flags (drapeaux des langues), voir iconsets.py.

ICONS : {nom logique: {body, width, height}} — le même fichier icons.json est écrit pour Astro, Laravel et Wagtail ;
`alpharust generate … --icons <jeu>` le réécrit avec un autre jeu Iconify.
"""
from . import iconsets, i18n

_DATA = iconsets.load()
ICONS = {**iconsets.resolve(_DATA), **iconsets.flags(_DATA, [c for c, _, _ in i18n.LOCALES])}


def icons_json():
    """Contenu de icons.json (trié, une icône par ligne) : lu par chaque framework, réécrit par --icons."""
    import json
    sets = _DATA['sets']
    head = {'set': iconsets.DEFAULT_SET, 'flags': iconsets.FLAG_SET,
            'licenses': {k: f"{sets[k]['name']} ({sets[k]['license']})" for k in (iconsets.DEFAULT_SET, iconsets.FLAG_SET)}}
    lines = ',\n'.join(f'  {json.dumps(k)}: {json.dumps(v, ensure_ascii=False, sort_keys=True)}' for k, v in sorted(ICONS.items()))
    return json.dumps(head, ensure_ascii=False)[:-1] + ', "icons": {\n' + lines + '\n}}\n'
