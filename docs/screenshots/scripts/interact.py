"""Vérifie les composants avancés (Starwind et CSS pur) : rendu, erreurs console et interactions clés.
Usage : interact.py <nom> <url> <dossier_sortie>"""
import sys, json
from playwright.sync_api import sync_playwright
name, url, out = sys.argv[1:4]
res = {}
with sync_playwright() as p:
    b = p.chromium.launch(executable_path="/usr/bin/google-chrome", args=["--no-sandbox"])
    pg = b.new_page(viewport={"width": 1280, "height": 900})
    errs = []
    pg.on("pageerror", lambda e: errs.append(str(e)))
    pg.on("console", lambda m: errs.append(m.text) if m.type == "error" else None)
    pg.goto(url); pg.wait_for_load_state("networkidle")
    pg.screenshot(path=f"{out}/{name}-advanced.png", full_page=True)
    def step(k, fn):
        try:
            res[k] = fn()
        except Exception as e:
            res[k] = "ERREUR " + str(e).splitlines()[0][:160]
    def esc():
        pg.keyboard.press("Escape"); pg.wait_for_timeout(250)
    # alert dialog
    def ad():
        pg.get_by_role("button", name="Supprimer le projet").click(); pg.wait_for_timeout(400)
        d = pg.locator("dialog[data-slot='alert-dialog-content']")
        o = d.evaluate("d => d.open")
        pg.screenshot(path=f"{out}/{name}-alert-dialog.png")
        pg.get_by_role("button", name="Annuler").click(); pg.wait_for_timeout(400)
        return {"ouvert": o, "fermé après Annuler": not d.evaluate("d => d.open")}
    step("alert-dialog", ad)
    def sh():
        pg.get_by_role("button", name="Ouvrir le panneau").click(); pg.wait_for_timeout(500)
        d = pg.locator("dialog[data-slot='sheet-content']").first
        o = d.evaluate("d => d.open")
        pg.screenshot(path=f"{out}/{name}-sheet.png")
        esc()
        return {"ouvert": o, "fermé par Échap": not d.evaluate("d => d.open")}
    step("sheet", sh)
    def dd():
        pg.get_by_role("button", name="Menu", exact=True).click(); pg.wait_for_timeout(300)
        c = pg.locator("[data-slot='dropdown-content']").first
        vis = c.is_visible()
        pg.screenshot(path=f"{out}/{name}-dropdown.png")
        esc()
        return {"visible": vis, "masqué après Échap": not c.is_visible()}
    step("dropdown", dd)
    def po():
        pg.get_by_role("button", name="Popover").click(); pg.wait_for_timeout(300)
        c = pg.locator("[data-slot='popover-content']").first
        vis = c.is_visible(); esc()
        return {"visible": vis}
    step("popover", po)
    def se():
        ev = pg.evaluate("""() => new Promise(r => { document.addEventListener('starwind-select:change', e => r(e.detail), {once:true}); setTimeout(() => r(null), 3000); window.__sel = 1; })""") if False else None
        pg.evaluate("window.__selEv = null; document.addEventListener('starwind-select:change', e => { window.__selEv = e.detail; }, {once:true})")
        pg.locator("[data-slot='select-trigger']").first.click(); pg.wait_for_timeout(300)
        pg.screenshot(path=f"{out}/{name}-select.png")
        pg.locator("[data-slot='select-item']", has_text="Poire").first.click(); pg.wait_for_timeout(300)
        return {"valeur affichée": pg.locator("[data-slot='select-value']").first.inner_text(), "événement": pg.evaluate("window.__selEv")}
    step("select", se)
    def otp():
        inp = pg.locator("[data-slot='input-otp'] input").first
        inp.focus(); pg.keyboard.type("123456"); pg.wait_for_timeout(200)
        return {"valeur": inp.input_value(), "cases": pg.locator("[data-slot='input-otp-slot']").all_inner_texts()}
    step("input-otp", otp)
    def sl():
        t = pg.locator("[data-slot='slider-thumb']").first
        before = t.get_attribute("aria-valuenow"); t.focus(); pg.keyboard.press("ArrowRight"); pg.keyboard.press("ArrowRight")
        return {"avant": before, "après": t.get_attribute("aria-valuenow")}
    step("slider", sl)
    def rg():
        pg.locator("label[for='plan-team']").click()
        return pg.locator("[data-slot='radio-group-item']:checked").first.get_attribute("value", timeout=3000)
    step("radio-group", rg)
    def tg():
        t = pg.get_by_role("button", name="Gras"); t.click()
        return t.get_attribute("aria-pressed")
    step("toggle", tg)
    def co():
        pg.get_by_text("Afficher les détails").click(); pg.wait_for_timeout(200)
        return pg.locator("[data-slot='collapsible-content']").first.is_visible()
    step("collapsible", co)
    def ca():
        pg.locator("[data-slot='carousel-next']").first.click(); pg.wait_for_timeout(600)
        return pg.locator("[data-slot='carousel-previous']").first.is_enabled()
    step("carousel (précédent actif après suivant)", ca)
    def sb():
        root = pg.locator("[data-slot='sidebar']").first
        before = root.get_attribute("data-state")
        pg.locator("[data-slot='sidebar-trigger']").first.click(); pg.wait_for_timeout(400)
        after = root.get_attribute("data-state")
        pg.locator("[data-slot='sidebar-trigger']").first.scroll_into_view_if_needed()
        pg.screenshot(path=f"{out}/{name}-sidebar-collapsed.png")
        pg.locator("[data-slot='sidebar-trigger']").first.click(); pg.wait_for_timeout(300)
        return {"avant": before, "après": after}
    step("sidebar", sb)
    def to():
        pg.get_by_role("button", name="Afficher une notification").click(); pg.wait_for_timeout(700)
        n = pg.locator("[data-slot='toast-viewport'] [data-slot='toast']").count()
        pg.screenshot(path=f"{out}/{name}-toast.png")
        return {"toasts affichés": n}
    step("toast", to)
    def th():
        before = pg.evaluate("document.documentElement.classList.contains('dark')")
        pg.locator("[data-slot='theme-toggle']").first.click(); pg.wait_for_timeout(300)
        return {"dark avant": before, "dark après": pg.evaluate("document.documentElement.classList.contains('dark')")}
    step("theme-toggle", th)
    pg.screenshot(path=f"{out}/{name}-advanced-dark.png", full_page=True)
    res["erreurs console"] = errs
    b.close()
print(json.dumps(res, ensure_ascii=False, indent=1))
