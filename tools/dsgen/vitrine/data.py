"""Données du site prêtes à écrire : spec typographiée + corps Markdown convertis en HTML."""
import copy
from . import spec, md

RAW_KEYS = {'url', 'email', 'icon', 'slug', 'type', 'name', 'autocomplete', 'date', 'phone', 'body'}

def _typo(v, key=None):
    if isinstance(v, str):
        return v if key in RAW_KEYS else md.typo(v)
    if isinstance(v, list): return [_typo(x, key) for x in v]
    if isinstance(v, dict): return {k: _typo(x, k) for k, x in v.items()}
    return v

SITE = _typo(copy.deepcopy(spec.SITE))
SITE['name'] = '@@title@@'
SERVICES = _typo(copy.deepcopy(spec.SERVICES))
POSTS = _typo(copy.deepcopy(spec.POSTS))
PAGES = _typo(copy.deepcopy(spec.PAGES))
FIELDS = _typo(copy.deepcopy(spec.CONTACT_FIELDS))
for coll in (SERVICES, POSTS):
    for e in coll:
        e['body_md'] = md.typo(e['body'])
        e['body_html'] = md.to_html(e['body_md'])
for p in PAGES.values():
    for s in p['sections']:
        if s['type'] == 'text':
            s['body'] = md.to_html(md.typo(s['body']))
# Ordre d'affichage : services dans l'ordre de la spec, articles du plus récent au plus ancien.
for i, e in enumerate(SERVICES): e['order'] = i + 1
POSTS.sort(key=lambda p: p['date'], reverse=True)

# Pages simples (sections seules) et leur adresse ; services/blog ont en plus des pages de détail.
SIMPLE_PAGES = ['about', 'legal', 'privacy']
