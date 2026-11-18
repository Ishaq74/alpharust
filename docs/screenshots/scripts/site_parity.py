"""Parité du site généré : mêmes pages sur 4 combinaisons (Astro + Starwind, Astro + CSS pur, Laravel, Wagtail).

Pour chaque page : capture pleine page (bureau et mobile), erreurs console, et une SIGNATURE structurelle comparée
entre combinaisons — repères (header/nav/main/footer), titres h1-h3, libellés du menu et du pied de page, champs de
formulaire, boutons, nombre de cartes. Produit des planches côte à côte (`site-<page>.png`) et `site-parity.json`.

Usage : python site_parity.py <dossier-de-sortie>  (serveurs : 4401 Starwind, 4402 CSS pur, 4403 Laravel, 4404 Wagtail)
"""
import asyncio, json, os, re, sys
from playwright.async_api import async_playwright
from PIL import Image, ImageDraw

STACKS = [
    ('astro-starwind', 'Astro · Tailwind + Starwind', 'http://localhost:4401', True),
    ('astro-css', 'Astro · CSS pur (Concordia)', 'http://localhost:4402', True),
    ('laravel', 'Laravel · Blade', 'http://127.0.0.1:4403', False),
    ('wagtail', 'Wagtail · Django', 'http://127.0.0.1:4404', True),
]
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
        ref = by['astro-css']
        for stack, sig in by.items():
            for k in keys + ['status']:
                if norm(sig[k]) != norm(ref[k]):
                    diffs.setdefault(page, []).append({'stack': stack, 'key': k, 'ref': ref[k], 'got': sig[k]})
    return diffs

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
    report = asyncio.run(capture(out))
    diffs = compare(report)
    for page in report:
        board(out, page); board(out, page, '-mobile', 0.7)
    json.dump({'pages': report, 'diffs': diffs}, open(f'{out}/site-parity.json', 'w'), ensure_ascii=False, indent=1)
    errors = {p: {s: v['errors'] for s, v in by.items() if v['errors']} for p, by in report.items()}
    errors = {p: e for p, e in errors.items() if e}
    print('pages', len(report), '· combinaisons', len(STACKS), '· différences de structure', sum(map(len, diffs.values())),
          '· erreurs console', errors or 0)
    for page, ds in diffs.items():
        for d in ds: print(' ', page, d['stack'], d['key'], '\n     ref', d['ref'], '\n     got', d['got'])
