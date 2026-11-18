"""Parcours de compte sur les 6 combinaisons (comptes : --auth) : inscription, « Mon compte », déconnexion, erreurs,
connexion en anglais — dans un vrai navigateur (Astro refuse les POST d'une autre origine).

Usage : python site_auth.py <dossier-de-sortie>   (mêmes serveurs que site_parity.py)
Écrit <sortie>/auth-<combo>-<étape>.png et site-auth.json ; code de sortie 1 au premier écart.
"""
import asyncio, json, os, sys, time
from playwright.async_api import async_playwright

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from site_parity import STACKS  # noqa: E402


async def run(stack, browser, out, stamp):
    key, _, base, _ = stack
    ctx = await browser.new_context(viewport={'width': 1280, 'height': 900})
    page = await ctx.new_page()
    email = f'ada-{key}-{stamp}@example.com'
    checks = []

    def check(name, ok, detail=''):
        checks.append({'step': name, 'ok': bool(ok), 'detail': detail})

    async def account_label():
        return (await page.locator('.site-header__account').inner_text()).strip()

    async def submit(values):
        for name, value in values.items():
            await page.fill(f'#auth-{name}', value)
        async with page.expect_navigation():
            await page.click('.site-auth form [type=submit]')

    await page.goto(base + '/')
    check('header: Connexion', await account_label() == 'Connexion', await account_label())
    await page.click('.site-header__account')
    check('lien -> /auth/sign-in', page.url.rstrip('/').endswith('/auth/sign-in'), page.url)
    await page.screenshot(path=f'{out}/auth-{key}-sign-in.png', full_page=True)
    await page.click('.site-auth__alt a')
    await submit({'name': 'Ada Lovelace', 'email': email, 'password': 'motdepasse'})
    check('inscription -> /auth/dashboard', page.url.rstrip('/').endswith('/auth/dashboard'), page.url)
    body = await page.inner_text('main')
    check('Mon compte : nom et e-mail', 'Ada Lovelace' in body and email in body, body[:120])
    check('header: Mon compte', await account_label() == 'Mon compte', await account_label())
    await page.screenshot(path=f'{out}/auth-{key}-dashboard.png', full_page=True)
    async with page.expect_navigation():
        await page.click('.site-auth form [type=submit]')
    check('déconnexion -> accueil', page.url.rstrip('/') == base, page.url)
    check('header: Connexion après déconnexion', await account_label() == 'Connexion', await account_label())
    await page.goto(base + '/auth/sign-up')
    await submit({'name': 'Ada', 'email': email, 'password': 'motdepasse'})
    err = await page.locator('.site-auth [data-slot=alert]').inner_text()
    check('inscription en double', 'Un compte existe déjà avec cette adresse.' in err, err)
    await page.goto(base + '/auth/sign-up')
    await submit({'name': 'Bob', 'email': f'bob-{key}-{stamp}@example.com', 'password': 'court'})
    err = await page.locator('.site-auth [data-slot=alert]').inner_text()
    check('mot de passe trop court', '8 caractères' in err, err)
    await page.goto(base + '/en/auth/sign-in')
    await submit({'email': email, 'password': 'mauvais!'})
    err = await page.locator('.site-auth [data-slot=alert]').inner_text()
    check('EN : identifiants refusés', 'Incorrect e-mail or password.' in err, err)
    check('EN : e-mail ré-affiché', await page.input_value('#auth-email') == email)
    await page.screenshot(path=f'{out}/auth-{key}-sign-in-error-en.png', full_page=True)
    await submit({'email': email, 'password': 'motdepasse'})
    check('EN : connexion -> /en/auth/dashboard', page.url.rstrip('/').endswith('/en/auth/dashboard'), page.url)
    check('EN header: My account', await account_label() == 'My account', await account_label())
    await ctx.close()
    return key, checks


async def main(out):
    os.makedirs(out, exist_ok=True)
    stamp = str(int(time.time()))
    async with async_playwright() as p:
        browser = await p.chromium.launch(executable_path=os.environ.get('CHROME', '/usr/bin/google-chrome'), args=['--no-sandbox'])
        results = dict([await run(s, browser, out, stamp) for s in STACKS])
        await browser.close()
    json.dump(results, open(f'{out}/site-auth.json', 'w'), ensure_ascii=False, indent=1)
    bad = 0
    for key, checks in results.items():
        ko = [c for c in checks if not c['ok']]
        bad += len(ko)
        print(f"{key}: {len(checks) - len(ko)}/{len(checks)}" + ''.join(f"\n   ✗ {c['step']} : {c['detail']}" for c in ko))
    sys.exit(1 if bad else 0)


if __name__ == '__main__':
    asyncio.run(main(sys.argv[1]))
