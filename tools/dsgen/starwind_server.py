"""Kit Tailwind + Starwind pour Laravel et Wagtail (couche `starwind-server`).

Les composants serveur (Blade `<x-ds.*>`, Django `{% ds_* %}`) posent le même balisage que les atomes Starwind d'atomic
(`data-slot`, `data-variant`, `data-size`, `data-side`…). Ce script lit les définitions `tv()` des atomes Starwind
(classes Tailwind verbatim) et écrit une feuille où chaque slot reçoit ces classes par `@apply` :

    [data-slot="button"] { @apply inline-flex items-center … }
    [data-slot="button"][data-variant="primary"] { @apply bg-primary text-primary-foreground … }

Tailwind v4 compile ensuite cette feuille : `@tailwindcss/vite` côté Laravel (doc Laravel « Vite »), la CLI autonome
de Tailwind côté Wagtail (doc Tailwind « Standalone CLI »). Exclus : les classes de tw-animate-css (animations
d'ouverture) et les marqueurs `group`/`peer` sans utilitaire (voir DROP).
"""
import json, os, re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ATOMS = ROOT / 'templates/layers/ui-atomic/src/components/atoms'
OUT = ROOT / 'templates/layers/starwind-server'

# Marqueurs sans utilitaire, classes de tw-animate-css (non fournie hors npm) et classes de script Starwind.
# Utilitaires inconnus du thème (Tailwind les ignore dans une classe, mais @apply les refuse) : on les écarte aussi.
DROP_EXACT = {'font-heading', 'dark', 'from-gradient-warm/40', 'from-gradient-cool/40', 'focus-visible:border-ring', 'sw-prose',
              'group', 'peer', 'animate-in', 'animate-out', 'fade-in', 'fade-out', 'zoom-in', 'zoom-out'}
DROP_RE = re.compile(r'^(group/|peer/|starwind-)|(^|:)(animate-in|animate-out|fade-in|fade-out|fade-in-\d+|fade-out-\d+|'
                     r'zoom-in-\d+|zoom-out-\d+|zoom-in|zoom-out|slide-in-from-[\w-]+|slide-out-to-[\w-]+|spin-in-\d+|spin-out-\d+|'
                     r'fill-mode-\w+|running|paused|repeat-\w+|delay-\d+|ease-\w+|duration-\d+|direction-\w+)$')

def js_literal(obj: str):
    """Objet littéral JS (clés nues, virgules finales, commentaires) -> Python."""
    s = re.sub(r'//[^\n"]*\n', '\n', obj)
    s = re.sub(r'/\*.*?\*/', '', s, flags=re.S)
    s = re.sub(r'([{,]\s*)([A-Za-z_$][\w$]*)\s*:', r'\1"\2":', s)
    s = re.sub(r',(\s*[}\]])', r'\1', s)
    return json.loads(s)

def tv_blocks(src: str):
    """[(nom de la constante, objet tv)] d'un fichier .astro."""
    out, i = [], 0
    while (i := src.find('tv({', i)) != -1:
        m = re.search(r'(?:const|let)\s+(\w+)\s*=\s*$', src[max(0, i - 80):i])
        depth, j = 0, i + 3
        while j < len(src):
            depth += {'{': 1, '}': -1}.get(src[j], 0)
            if depth == 0: break
            j += 1
        try: out.append((m.group(1) if m else '', js_literal(src[i + 3:j + 1])))
        except Exception: pass
        i = j
    return out

def kebab(name): return re.sub(r'(?<!^)(?=[A-Z])', '-', name).lower()

def slot_for(name, slots, comp):
    """Slot d'une définition : nom de la constante (dialogContent -> dialog-content), sinon le seul slot du fichier."""
    k = kebab(name)
    for cand in (k, f'{comp}-{k}', k.replace('-variants', '')):
        if cand in slots: return cand
    tail = [s for s in slots if s.endswith('-' + k.split('-')[-1])]
    if len(tail) == 1: return tail[0]
    return slots[0] if len(slots) == 1 else None

def classes(v):
    if isinstance(v, list): v = ' '.join(x for x in v if isinstance(x, str))
    if not isinstance(v, str): return []
    return [c for c in v.split() if c not in DROP_EXACT and not DROP_RE.search(c)]

def rules():
    seen, out = set(), []
    for comp_dir in sorted(ATOMS.iterdir()):
        for f in sorted(comp_dir.glob('*.astro')):
            src = f.read_text()
            slots = list(dict.fromkeys(re.findall(r'data-slot="([a-z0-9-]+)"', src)))
            for name, tv in tv_blocks(src):
                slot = slot_for(name, slots, comp_dir.name)
                if not slot or slot in seen or not isinstance(tv, dict): continue
                seen.add(slot)
                sel = f'[data-slot="{slot}"]'
                block = [f'/* {comp_dir.name}/{f.name} : {name} */']
                base = classes(tv.get('base', ''))
                if base: block.append(f'{sel} {{ @apply {" ".join(base)}; }}')
                defaults = tv.get('defaultVariants', {}) or {}
                for key, values in (tv.get('variants') or {}).items():
                    if not isinstance(values, dict): continue
                    attr = 'data-' + kebab(key)
                    for val, cls in values.items():
                        c = classes(cls)
                        if not c: continue
                        conds = f'[{attr}="{val}"]'
                        if str(defaults.get(key)) == str(val):
                            conds = f':is({conds}, :not([{attr}]))'
                        block.append(f'{sel}{conds} {{ @apply {" ".join(c)}; }}')
                if len(block) > 1: out.append('\n'.join(block))
    return out

THEME = '''/* Thème Starwind (sortie de `starwind init`, sans tw-animate-css ni @tailwindcss/forms) : jetons -> utilitaires. */
@custom-variant dark (&:where(.dark, .dark *, [data-theme='dark'], [data-theme='dark'] *));

@theme {
  --animate-accordion-down: accordion-down 0.2s ease-out;
  --animate-accordion-up: accordion-up 0.2s ease-out;
  @keyframes accordion-down { from { height: 0; } to { height: var(--starwind-accordion-content-height); } }
  @keyframes accordion-up { from { height: var(--starwind-accordion-content-height); } to { height: 0; } }
}

@theme inline {
''' + ''.join(f'  --color-{n}: var(--{n});\n' for n in [
    'background', 'foreground', 'card', 'card-foreground', 'popover', 'popover-foreground', 'primary', 'primary-foreground',
    'primary-accent', 'secondary', 'secondary-foreground', 'secondary-accent', 'muted', 'muted-foreground', 'accent',
    'accent-foreground', 'info', 'info-foreground', 'success', 'success-foreground', 'warning', 'warning-foreground', 'error',
    'error-foreground', 'border', 'input', 'outline']) + '''
  --radius-xs: calc(var(--radius) - 0.375rem);
  --radius-sm: calc(var(--radius) - 0.25rem);
  --radius-md: calc(var(--radius) - 0.125rem);
  --radius-lg: var(--radius);
  --radius-xl: calc(var(--radius) + 0.25rem);
  --radius-2xl: calc(var(--radius) + 0.5rem);
  --radius-3xl: calc(var(--radius) + 1rem);
  --color-sidebar: var(--sidebar-background);
  --color-sidebar-foreground: var(--sidebar-foreground);
  --color-sidebar-primary: var(--sidebar-primary);
  --color-sidebar-primary-foreground: var(--sidebar-primary-foreground);
  --color-sidebar-accent: var(--sidebar-accent);
  --color-sidebar-accent-foreground: var(--sidebar-accent-foreground);
  --color-sidebar-border: var(--sidebar-border);
  --color-sidebar-outline: var(--sidebar-outline);
}

:root { --radius: 0.625rem; }
'''

BASE = '''/* Valeurs de départ de Tailwind reprises d'atomic (bordures, anneau de focus, curseur). */
@layer base {
  * { @apply border-border outline-outline/50; }
  body { @apply bg-background text-foreground scheme-light dark:scheme-dark; }
  button { @apply cursor-pointer; }
}
'''

LARAVEL_APP = """@@meta mode=replace@@
/*
 * Feuille d'entrée Vite (laravel-vite-plugin) — Tailwind CSS v4 + kit Starwind (classes des atomes d'atomic).
 * Même ordre qu'Astro + Starwind : jetons, thème de marque, base, composants, puis gabarit du site.
 * alpharust-ignore: no-hardcoded-urls (liens de documentation)
 * Docs : https://laravel.com/docs/vite#working-with-stylesheets · https://tailwindcss.com/docs/installation/framework-guides/laravel/vite
 */
@import 'tailwindcss';
@import './starwind/theme.css';
@layer theme, base, components, utilities, overrides;

@import './starwind/colors.css';
@import './design/tokens/typography.css';
@import './design/tokens/spacing.css';
@import './design/tokens/site.css';
@import './design/themes/theme.css';
@import './design/base.css' layer(base);
@import './starwind/base.css';
@import './starwind/components.css';
@import './site.css'; /* gabarit du site (en-tête, pied de page, sections) : identique à Astro et Wagtail */

@source '../../vendor/laravel/framework/src/Illuminate/Pagination/resources/views/*.blade.php';
@source '../../storage/framework/views/*.php';
"""

WAGTAIL_INPUT = """/*
 * Entrée Tailwind CSS v4 du kit Starwind (Wagtail), compilée par la CLI autonome de Tailwind :
 *   .tailwind/tailwindcss -i tailwind/input.css -o @@pkg@@/static/css/tailwind.css --minify   (ajouter --watch en dev)
 * alpharust-ignore: no-hardcoded-urls (liens de documentation)
 * Doc : https://tailwindcss.com/blog/standalone-cli · https://tailwindcss.com/docs/installation/tailwind-cli
 * base.html charge la sortie avant css/site.css (gabarit du site, identique à Astro et Laravel).
 */
@import 'tailwindcss';
@import './starwind/theme.css';
@layer theme, base, components, utilities, overrides;

@import './starwind/colors.css';
@import '../ds/static/ds/css/tokens/typography.css';
@import '../ds/static/ds/css/tokens/spacing.css';
@import '../ds/static/ds/css/tokens/site.css';
@import '../ds/static/ds/css/themes/theme.css';
@import '../ds/static/ds/css/base.css' layer(base);
@import './starwind/base.css';
@import './starwind/components.css';

@source '../ds/templates';
@source '../@@pkg@@/templates';
"""

def write(root=ROOT):
    root = Path(root)
    colors = (root / 'templates/layers/astro/src/styles/tokens/colors.css.tpl').read_text().split('\n', 1)[1]
    kit = {
        'theme.css': THEME,
        'colors.css': "/* Jetons de couleur d'atomic (mêmes valeurs qu'Astro + Starwind). */\n" + colors,
        'base.css': BASE,
        'components.css': ("/* GÉNÉRÉ par tools/dsgen/starwind_server.py depuis les atomes Starwind d'atomic : ne pas modifier. */\n"
                           '@layer components {\n\n' + '\n\n'.join(rules()) + '\n\n}\n'),
    }
    files = {}
    for name, text in kit.items():
        files[f'laravel-starwind/resources/css/starwind/{name}'] = text
        files[f'wagtail-starwind/tailwind/starwind/{name}'] = text
    files['laravel-starwind/resources/css/app.css.tpl'] = LARAVEL_APP
    files['wagtail-starwind/tailwind/input.css.tpl'] = WAGTAIL_INPUT
    for rel, text in files.items():
        p = root / 'templates/layers' / rel; p.parent.mkdir(parents=True, exist_ok=True); p.write_text(text)
    return len(files)

if __name__ == '__main__':
    print('starwind-server', write(), 'fichiers')
