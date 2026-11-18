# Composants « étendus » en CSS pur (Astro) : mêmes fichiers, props, data-slot et événements que les atomes d'atomic.
# Appelé par gen_ds.py (python3 tools/dsgen/gen_ds.py) ; ne pas éditer les fichiers générés à la main.
import os, re, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
for _part in ('ext_astro1.py', 'ext_astro2.py', 'ext_astro3.py'):
    exec(open(os.path.join(HERE, _part)).read())
ROOT = os.path.normpath(os.path.join(HERE, '..', '..', 'templates', 'layers'))
ATOMIC=ROOT+'/ui-atomic/src/components/atoms'
OUT=ROOT+'/ui-css/src/components/atoms'
comps = sorted({p.split('/')[0] for p in F})
for c in comps:
    shutil.rmtree(f'{OUT}/{c}', ignore_errors=True)
for p, body in F.items():
    dst=f'{OUT}/{p}'; os.makedirs(os.path.dirname(dst), exist_ok=True)
    if body is None: shutil.copy(f'{ATOMIC}/{p}', dst)
    else: open(dst,'w').write(rest_first(body) if dst.endswith('.astro') else body)
import re
def strip_variants(src):
    # Les objets *Variants d'atomic sont des fonctions tailwind-variants : sans objet en CSS pur
    # (comme pour les 23 composants core, l'allure vient des attributs data-*).
    src=re.sub(r'import (\w+),\s*\{[^}]*\}\s*from ("\./[^"]+\.astro");', r'import \1 from \2;', src)
    names=re.findall(r'const (\w+Variants) = \{[^}]*\};\n*', src)
    src=re.sub(r'const (\w+Variants) = \{[^}]*\};\n*', '', src)
    for n in names:
        src=re.sub(r'\n\s*'+n+r',?(?=\s*[\n}])', '', src)
        src=re.sub(r',\s*'+n+r'\b', '', src)
    return src
# index.ts : mêmes exports que l'index d'atomic (on retire seulement ce qui n'existe pas en CSS pur)
for c in comps:
    src=open(f'{ATOMIC}/{c}/index.ts').read()
    if c=='carousel':
        src=src.replace('import { initCarousel } from "./carousel-script";','import { type CarouselApi, type CarouselOptions, initCarousel } from "./carousel-script";')
    src=strip_variants(src)
    open(f'{OUT}/{c}/index.ts','w').write(src)
    af=sorted(f for f in os.listdir(f'{ATOMIC}/{c}'))
    of=sorted(f for f in os.listdir(f'{OUT}/{c}'))
    if af!=of: print('FICHIERS', c, set(af)^set(of))
print(len(comps),'composants', len(F),'fichiers')
