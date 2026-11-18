#!/usr/bin/env python3
"""Compare, élément par élément (dans l'ordre), les attributs de contrat (tag, data-*, role, aria-*, type, tabindex,
hidden, disabled, name, value) des éléments portant data-slot entre une page de référence et d'autres pages.
usage: attrdiff.py REF_URL URL...  (ignore id/aria-controls/aria-labelledby générés et le style ;
un attribut booléen écrit disabled="disabled" équivaut à disabled, comme en HTML)"""
import sys, urllib.request
from html.parser import HTMLParser
SKIP = {"id", "aria-controls", "aria-labelledby", "aria-describedby", "for", "style", "class", "data-astro-cid", "href", "src", "data-toast-id", "aria-activedescendant"}
class P(HTMLParser):
    def __init__(s):
        super().__init__(); s.els = []
    def handle_starttag(s, tag, attrs):
        a = {}
        for k, v in attrs: a.setdefault(k, v)
        if "data-slot" not in a: return
        keep = {k: ("" if (v or "") == k else (v or "")) for k, v in a.items() if k not in SKIP and not k.startswith("data-astro") and (k.startswith(("data-", "aria-")) or k in ("role", "type", "tabindex", "hidden", "disabled", "name", "value", "open", "inert", "aria-hidden"))}
        s.els.append((tag, tuple(sorted(keep.items()))))
def load(u):
    p = P(); p.feed(urllib.request.urlopen(u).read().decode()); return p.els
ref = load(sys.argv[1]); bad = 0
for u in sys.argv[2:]:
    o = load(u); diffs = []
    for i, (a, b) in enumerate(zip(ref, o)):
        if a != b: diffs.append((i, a, b))
    print(f"== {u}: {len(o)} éléments (réf {len(ref)}), {len(diffs)} écart(s)")
    for i, a, b in diffs[:15]:
        da, db = dict(a[1]), dict(b[1])
        print(f"  #{i} {a[0]}/{b[0]} slot={da.get('data-slot')}: réf-seulement={ {k: v for k, v in da.items() if db.get(k) != v} } autre-seulement={ {k: v for k, v in db.items() if da.get(k) != v} }")
    bad += len(diffs) + abs(len(o) - len(ref))
print("RÉSULTAT :", "ATTRIBUTS IDENTIQUES" if not bad else f"{bad} ÉCART(S)")
sys.exit(1 if bad else 0)
