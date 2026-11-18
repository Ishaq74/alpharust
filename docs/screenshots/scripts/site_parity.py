"""Parité du site généré : mêmes pages sur les combinaisons framework × kit (Astro, Laravel, Wagtail × Starwind, CSS pur).

Pour chaque page : capture pleine page (bureau et mobile), erreurs console, et une SIGNATURE structurelle comparée
entre combinaisons — repères (header/nav/main/footer), titres h1-h3, libellés du menu et du pied de page, champs de
formulaire, boutons, nombre de cartes — et un ÉCART EN PIXELS de chaque capture avec la référence (part des pixels
qui diffèrent de plus de 24/255 sur un canal ; les lignes en plus ou en moins comptent comme différentes).
Produit des planches côte à côte (`site-<page>.png`) et `site-parity.json` ; code de sortie 1 au-delà du seuil.

Usage : python site_parity.py <dossier-de-sortie> [seuil-en-%]
Serveurs : 4401 Astro Starwind, 4402 Astro CSS, 4403 Laravel CSS, 4404 Wagtail CSS, 4405 Laravel Starwind,
4406 Wagtail Starwind (SITE_STACKS=clé1,clé2… pour n'en garder que certaines). Les démos portent le même nom de
projet (ex. `sites/parity/<combo>/vitrine`) pour que le texte soit identique au pixel près.
"""
import asyncio, json, os, re, sys
from playwright.async_api import async_playwright
from PIL import Image, ImageDraw

ALL_STACKS = [
    ('astro-starwind', 'Astro · Tailwind + Starwind', 'http://localhost:4401', True),
    ('astro-css', 'Astro · CSS pur', 'http://localhost:4402', True),
    ('laravel-css', 'Laravel · CSS pur', 'http://127.0.0.1:4403', False),
    ('wagtail-css', 'Wagtail · CSS pur', 'http://127.0.0.1:4404', True),
    ('laravel-starwind', 'Laravel · Tailwind + Starwind', 'http://127.0.0.1:4405', False),
    ('wagtail-starwind', 'Wagtail · Tailwind + Starwind', 'http://127.0.0.1:4406', True),
]
_only = [k for k in os.environ.get('SITE_STACKS', '').split(',') if k]
STACKS = [s for s in ALL_STACKS if not _only or s[0] in _only]
REF = 'astro-css' if any(s[0] == 'astro-css' for s in STACKS) else STACKS[0][0]
PAGES = ['/', '/a-propos', '/services', '/services/accessibilite', '/blog', '/blog/choisir-son-framework', '/contact',
         '/mentions-legales', '/confidentialite', '/page-inexistante']

SIGNATURE = r"""() => {
  const txt = (e) => (e.textContent || '').replace(/\s+/g, ' ').trim();
  const vis = (e) => !!(e.offsetWidth || e.offsetHeight || e.getClientRects().length);
  const q = (s, root = document) => [...root.querySelectorAll(s)];
  return {
    title: document.title,
    lang: document.documentElement.lang,
    landmarks: q('header, nav, main, footer, aside, section, article').map((e) => e.tagName.toLowerCase() + (e.getAttribute('aria-label') ? `[${e.getAttribute('aria-label')}]` : '')),
    headings: q('main h1, main h2, main h3').map((e) => e.tagName.toLowerCase() + ':' + txt(e)),
    nav: q('.site-nav > ul > li').map((li) => txt(li.querySelector('a, button'))),
    menu: q('.site-nav [role=menu] a').map(txt),
    mobile: q('.site-mobile__nav a, .site-mobile__nav button').map(txt),
    footer: q('footer a').map(txt),
    fields: q('main form input:not([type=hidden]), main form textarea').map((e) => `${e.name}:${e.type}:${e.required}`),
    buttons: q('main a[data-slot=button], main button').filter(vis).map(txt),
    cards: q('main [data-slot=card]').length,
    skip: !!document.querySelector('a[href="#contenu"]'),
    current: q('.site-nav [aria-current]').map(txt),
  };
}"""

def url(base, page, slash):
    if page == '/' or not slash: return base + page
    return base + page + '/'

async def capture(out):
    report = {}
    async with async_playwright() as p:
        b = await p.chromium.launch(executable_path='/usr/bin/google-chrome', args=['--no-sandbox'])
        for key, label, base, slash in STACKS:
            for width, suffix in ((1280, ''), (390, '-mobile')):
                ctx = await b.new_context(viewport={'width': width, 'height': 900})
                pg = await ctx.new_page()
                for page in PAGES:
                    errs = []
                    pg.on('console', lambda m, e=errs: e.append(m.text) if m.type == 'error' else None)
                    pg.on('pageerror', lambda x, e=errs: e.append(str(x)))
                    resp = await pg.goto(url(base, page, slash), wait_until='networkidle')
                    name = (page.strip('/').replace('/', '-') or 'accueil')
                    await pg.screenshot(path=f'{out}/{key}-{name}{suffix}.png', full_page=True)
                    if not suffix:
                        sig = await pg.evaluate(SIGNATURE)
                        # Le 404 attendu : la ressource principale répond 404, sans autre erreur.
                        errs = [e for e in errs if not ('404' in e and page == '/page-inexistante')]
                        report.setdefault(name, {})[key] = {'status': resp.status, 'errors': errs, **sig}
                await ctx.close()
        await b.close()
    return report

def norm(v):
    """Le nom du projet diffère (demo-sw, demo-lv…) et Astro retire les blancs entre éléments : on les neutralise."""
    if isinstance(v, list): return [norm(x) for x in v]
    if isinstance(v, str): return re.sub(r'\s+', '', re.sub(r'Demo ?\w\w', 'SITE', v))
    return v

def compare(report):
    """Différences de signature par page (référence : astro-css). Titre de l'onglet et casse des URL ignorés."""
    diffs = {}
    keys = ['lang', 'landmarks', 'headings', 'nav', 'menu', 'mobile', 'footer', 'fields', 'buttons', 'cards', 'skip', 'current']
    for page, by in report.items():
        ref = by[REF]
        for stack, sig in by.items():
            for k in keys + ['status']:
                if norm(sig[k]) != norm(ref[k]):
                    diffs.setdefault(page, []).append({'stack': stack, 'key': k, 'ref': ref[k], 'got': sig[k]})
    return diffs

def pixel_diff(a_path, b_path, tol=24):
    """Part (en %) des pixels qui diffèrent de plus de `tol` sur un canal ; la différence de hauteur compte en entier."""
    a = Image.open(a_path).convert('RGB'); b = Image.open(b_path).convert('RGB')
    w, h = min(a.width, b.width), min(a.height, b.height)
    H = max(a.height, b.height)
    from PIL import ImageChops
    d = ImageChops.difference(a.crop((0, 0, w, h)), b.crop((0, 0, w, h))).convert('L').point(lambda v: 255 if v > tol else 0)
    differing = sum(d.histogram()[255:]) + (H - h) * w
    return round(100 * differing / (w * H), 2)

def pixels(out, report):
    """Écart en pixels de chaque combinaison avec la référence, bureau et mobile."""
    res = {}
    for name in report:
        for suffix in ('', '-mobile'):
            for key, *_ in STACKS:
                if key == REF: continue
                res.setdefault(name + suffix, {})[key] = pixel_diff(f'{out}/{REF}-{name}{suffix}.png', f'{out}/{key}-{name}{suffix}.png')
    return res

def board(out, name, suffix='', scale=0.5):
    ims = [Image.open(f'{out}/{k}-{name}{suffix}.png').convert('RGB') for k, *_ in STACKS]
    ims = [i.resize((int(i.width * scale), int(i.height * scale))) for i in ims]
    h, gap = max(i.height for i in ims), 16
    W = sum(i.width for i in ims) + gap * (len(ims) - 1)
    im = Image.new('RGB', (W, h + 36), 'white'); d = ImageDraw.Draw(im); x = 0
    for i, (_, label, *_r) in zip(ims, STACKS):
        im.paste(i, (x, 36)); d.text((x + 8, 12), label, fill='black'); x += i.width + gap
        if x < W: d.line([(x - gap // 2, 0), (x - gap // 2, h + 36)], fill='#999', width=2)
    im.save(f'{out}/site-{name}{suffix}.png')

if __name__ == '__main__':
    out = sys.argv[1]; os.makedirs(out, exist_ok=True)
    limit = float(sys.argv[2]) if len(sys.argv) > 2 else None
    report = asyncio.run(capture(out))
    diffs = compare(report)
    for page in report:
        board(out, page); board(out, page, '-mobile', 0.7)
    px = pixels(out, report)
    json.dump({'reference': REF, 'pages': report, 'diffs': diffs, 'pixels': px}, open(f'{out}/site-parity.json', 'w'), ensure_ascii=False, indent=1)
    errors = {p: {s: v['errors'] for s, v in by.items() if v['errors']} for p, by in report.items()}
    errors = {p: e for p, e in errors.items() if e}
    print('pages', len(report), '· combinaisons', len(STACKS), '· différences de structure', sum(map(len, diffs.values())),
          '· erreurs console', errors or 0)
    for page, ds in diffs.items():
        for d in ds: print(' ', page, d['stack'], d['key'], '\n     ref', d['ref'], '\n     got', d['got'])
    worst = max((v, p, k) for p, by in px.items() for k, v in by.items()) if px else (0, '', '')
    print(f'écart en pixels (réf. {REF}) : pire {worst[0]} % ({worst[2]}, {worst[1]})')
    for p, by in px.items(): print('  ', p, by)
    if limit is not None and (worst[0] > limit or diffs or errors):
        sys.exit(1)
