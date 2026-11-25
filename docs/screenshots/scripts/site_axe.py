"""Accessibilité (axe-core, règles WCAG 2.x A et AA) des 6 combinaisons, dans les 4 langues.

Usage : AXE_JS=<chemin de axe.min.js> python site_axe.py <dossier-de-sortie>   (mêmes serveurs que site_parity.py)
axe-core : `npm i axe-core@4` puis AXE_JS=node_modules/axe-core/axe.min.js. Écrit site-axe.json ;
code de sortie 1 dès qu'une violation « serious » ou « critical » est trouvée (contraste compris), en clair et en sombre.
"""
import asyncio, json, os, sys
from playwright.async_api import async_playwright

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from site_parity import STACKS  # noqa: E402

PAGES = ['/', '/a-propos', '/services/accessibilite', '/blog/choisir-son-framework', '/contact', '/auth/sign-in',
         '/en', '/en/contact', '/es', '/es/blog', '/ar', '/ar/contact']
TAGS = ['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa']


async def main(out):
    axe = open(os.environ['AXE_JS']).read()
    report, blocking = [], 0
    async with async_playwright() as p:
        browser = await p.chromium.launch(executable_path=os.environ.get('CHROME', '/usr/bin/google-chrome'), args=['--no-sandbox'])
        for key, _, base, slash in STACKS:
            for theme in ('light', 'dark'):
                ctx = await browser.new_context(viewport={'width': 1280, 'height': 900}, color_scheme=theme)
                page = await ctx.new_page()
                for path in PAGES:
                    url = base + path + ('/' if slash and path != '/' else '')
                    await page.goto(url, wait_until='networkidle')
                    await page.add_script_tag(content=axe)
                    res = await page.evaluate('(tags) => axe.run(document, { runOnly: { type: "tag", values: tags } })', TAGS)
                    for v in res['violations']:
                        bad = v['impact'] in ('serious', 'critical')
                        blocking += bad
                        report.append({'combo': key, 'theme': theme, 'page': path, 'rule': v['id'], 'impact': v['impact'],
                                       'nodes': len(v['nodes']), 'example': v['nodes'][0]['target'] if v['nodes'] else None,
                                       'summary': v['nodes'][0].get('failureSummary', '')[:300] if v['nodes'] else ''})
                await ctx.close()
        await browser.close()
    os.makedirs(out, exist_ok=True)
    with open(os.path.join(out, 'site-axe.json'), 'w') as f:
        json.dump({'pages': len(PAGES), 'combos': [s[0] for s in STACKS], 'violations': report}, f, ensure_ascii=False, indent=1)
    by = {}
    for r in report:
        by.setdefault((r['combo'], r['rule'], r['impact']), 0)
        by[(r['combo'], r['rule'], r['impact'])] += 1
    for (combo, rule, impact), n in sorted(by.items()):
        print(f'{combo:18} {impact:9} {rule} ×{n}')
    print(f'axe : {len(report)} violation(s), dont {blocking} serious/critical, {len(PAGES)} pages × {len(STACKS)} combinaisons × 2 thèmes')
    if blocking:
        sys.exit(1)


if __name__ == '__main__':
    asyncio.run(main(sys.argv[1] if len(sys.argv) > 1 else 'axe'))
