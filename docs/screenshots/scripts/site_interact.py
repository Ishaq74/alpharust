"""Usage : site_interact.py [dossier]. Serveurs attendus : 4401 Astro Starwind, 4402 Astro CSS, 4403 Laravel, 4404 Wagtail.

Vérifie les interactions du gabarit sur les 4 combinaisons : menu déroulant, menu mobile, thème, formulaire de contact."""
import sys
from playwright.sync_api import sync_playwright

OUT = sys.argv[1] if len(sys.argv) > 1 else "."
S=[('astro-starwind','http://localhost:4401','/'),('astro-css','http://localhost:4402','/'),('laravel','http://127.0.0.1:4403',''),('wagtail','http://127.0.0.1:4404','/')]
with sync_playwright() as p:
    b=p.chromium.launch(executable_path='/usr/bin/google-chrome',args=['--no-sandbox'])
    for name,u,sl in S:
        res={}; errs=[]
        pg=b.new_page(viewport={'width':1280,'height':900}); pg.on('console', lambda m: m.type=='error' and errs.append(m.text)); pg.on('pageerror', lambda e: errs.append(str(e)))
        pg.goto(u+'/'); pg.wait_for_load_state('networkidle'); pg.wait_for_timeout(300)
        pg.hover('.site-nav [data-slot=dropdown-trigger]'); pg.wait_for_timeout(600)
        res['dropdown']=pg.evaluate("()=>{const a=[...document.querySelectorAll('.site-header a, [data-slot=dropdown-content] a, [role=menu] a, [role=menuitem]')].filter(x=>x.offsetParent && (x.getAttribute('href')||'').includes('/services/'));return a.length}")
        pg.mouse.move(5, 800); pg.wait_for_timeout(500)
        pg.focus('.site-nav [data-slot=dropdown-trigger]'); pg.keyboard.press('Enter'); pg.wait_for_timeout(500)
        res['dropdown_clavier']=pg.evaluate("()=>{const c=document.querySelector('[data-slot=dropdown-content]');return c.dataset.state}")
        pg.keyboard.press('Escape'); pg.wait_for_timeout(300)
        before=pg.evaluate("document.documentElement.classList.contains('dark')||document.documentElement.dataset.theme==='dark'")
        pg.click('.site-header [data-slot=theme-toggle]'); pg.wait_for_timeout(300)
        after=pg.evaluate("document.documentElement.classList.contains('dark')||document.documentElement.dataset.theme==='dark'")
        res['theme']=f'{before}->{after}'
        pg.close()
        m=b.new_page(viewport={'width':390,'height':844}); m.on('pageerror', lambda e: errs.append(str(e)))
        m.goto(u+'/'); m.wait_for_load_state('networkidle'); m.wait_for_timeout(300)
        m.click('.site-header [aria-label="Ouvrir le menu"]'); m.wait_for_timeout(600)
        res['sheet']=m.evaluate("()=>[...document.querySelectorAll('.site-mobile__link, .site-mobile a')].filter(x=>x.offsetParent).length")
        m.close()
        c=b.new_page(viewport={'width':1280,'height':900}); c.on('pageerror', lambda e: errs.append(str(e)))
        c.goto(u+'/contact'+sl); c.wait_for_load_state('networkidle')
        c.fill('[name=nom]','Test Parité'); c.fill('[name=email]','test@example.org'); c.fill('[name=sujet]','Essai'); c.fill('[name=message]','Bonjour, ceci est un message de test assez long.')
        c.click('form button[type=submit]'); c.wait_for_load_state('networkidle')
        res['contact']=c.evaluate("()=>{const a=document.querySelector('.site-form__notice');return a?a.textContent.trim().slice(0,60):'pas de confirmation'}")
        c.screenshot(path=f'{OUT}/{name}-contact-envoye.png')
        c.close()
        print(name, res, 'erreurs:', errs[:3])
    b.close()
