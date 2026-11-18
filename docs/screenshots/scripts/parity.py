#!/usr/bin/env python3
"""Parité runtime : compare les data-slot réellement rendus par les pages /components (4 cibles) ou /components/advanced (2 kits).

usage: parity.py starwind=URL concordia=URL laravel=URL wagtail=URL
"""
import sys, tomllib, os, urllib.request
from collections import Counter
from html.parser import HTMLParser


class P(HTMLParser):
    def __init__(self):
        super().__init__()
        self.slots = Counter()
        self.variants = Counter()

    def handle_starttag(self, tag, attrs):
        # comme le navigateur : en cas d'attribut en double, le PREMIER l'emporte
        a = {}
        for k, v in attrs:
            a.setdefault(k, v)
        s = a.get("data-slot")
        if s:
            self.slots[s] += 1
            if a.get("data-variant"):
                self.variants[(s, a["data-variant"])] += 1


def load(url):
    p = P()
    p.feed(urllib.request.urlopen(url).read().decode())
    return p


CATALOG = os.environ.get("CATALOG", "templates/layers/design-kit/catalog.toml")
cat = tomllib.load(open(CATALOG, "rb"))
gaps = cat["known_gaps"]
# data-variant fait partie du contrat des composants core ; les atomes étendus d'atomic (verbatim) n'en posent pas,
# leurs équivalents CSS l'utilisent comme crochet de style : comparaison informative seulement.
core_slots = {s for c in cat["component"] if c["kind"] == "core" for s in c["slots"]}
pages = {k: load(v) for k, v in (a.split("=", 1) for a in sys.argv[1:])}
names = list(pages)
ref = pages.get("concordia") or pages[names[0]]
ok = True
for n, p in pages.items():
    print(f"{n:10} {sum(p.slots.values()):4} éléments, {len(p.slots):3} slots distincts, {len(p.variants)} (slot, variant)")
for n, p in pages.items():
    if n in ("concordia",):
        continue
    missing = set(ref.slots) - set(p.slots)
    extra = set(p.slots) - set(ref.slots)
    cnt = {s: (ref.slots[s], p.slots[s]) for s in ref.slots if s in p.slots and ref.slots[s] != p.slots[s]}
    vm = set(ref.variants) - set(p.variants)
    print(f"\n== {n} vs concordia")
    print("  slots absents :", sorted(missing) or "aucun")
    print("  slots en plus :", sorted(extra) or "aucun")
    print("  effectifs différents (concordia, autre) :", cnt or "aucun")
    print("  (slot, variant) absents :", sorted(vm) or "aucun")
    if n == "starwind":
        # Starwind : seuls les écarts DÉCLARÉS dans [known_gaps] sont admis ; les variantes passent par data-variant
        unknown = (
            (missing - set(gaps["atoms_missing_slots"]))
            | (extra - set(gaps["atoms_extra_slots"]))
            | (set(cnt) - set(gaps["atoms_slot_count_differs"]))
        )
        # une variante portée par un slot déjà déclaré absent n'est pas un nouvel écart
        info = {v for v in vm if v[0] not in core_slots}
        if info:
            print("  data-variant propres au CSS (composants étendus, informatif) :", len(info))
        vm = {v for v in vm if v[0] in core_slots and v[0] not in gaps["atoms_missing_slots"]}
        print("  écarts non déclarés :", sorted(map(str, unknown | vm)) or "aucun")
        if unknown or vm:
            ok = False
    elif missing or extra or cnt or vm:
        ok = False
print("\nRÉSULTAT :", "PARITÉ OK" if ok else "ÉCARTS")
sys.exit(0 if ok else 1)
