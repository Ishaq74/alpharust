# Générateur du kit partagé : atomes CSS pur (Astro), composants Blade (Laravel) et balises Django (Wagtail)
# à partir des atomes Starwind d'atomic (templates/layers/ui-atomic) et de spec.py, puis catalog.toml.
# Usage : python3 tools/dsgen/gen_ds.py  — la CI régénère et vérifie `git diff --exit-code templates/layers`.
import os, re, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from spec import SIMPLE, SPECIAL

ROOT = os.path.normpath(os.path.join(HERE, '..', '..', 'templates', 'layers'))
ATOMIC = ROOT + '/ui-atomic/src/components/atoms'
OUT_ASTRO = ROOT + '/ui-css/src/components/atoms'
OUT_BLADE = ROOT + '/ui-blade/resources/views/components/ds'
OUT_DJ = ROOT + '/ui-django/ds/templatetags/ds.py'

for d in (ROOT + '/ui-css', ROOT + '/ui-blade', ROOT + '/ui-django'):
    shutil.rmtree(d, ignore_errors=True)

def kebab(s): return re.sub(r'(?<!^)(?=[A-Z])', '-', s).lower()
def snake(s): return kebab(s).replace('-', '_')
def pascal_comp(c): return ''.join(w.capitalize() for w in c.split('-'))
def attr_name_astro(k): return k
def rest_first(text):
    # Astro ne fusionne pas les attributs en double et le navigateur garde le PREMIER : comme dans atomic,
    # `{...rest}` passe avant `data-slot` pour qu'un composant parent puisse le surcharger
    # (Sheet -> Dialog, InputGroupInput -> Input, SidebarInput -> Input…).
    # Éléments HTML seulement (minuscule ou `Tag` dynamique) : pour un composant, la dernière prop l'emporte.
    return re.sub(r'(<(?:[a-z][a-z0-9]*|Tag)\b[^<>]*?)(data-slot="[a-z-]+")([^<>]*?) \{\.\.\.rest\}',
                  r'\1{...rest} \2\3', text)

def w(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    if path.endswith('.astro'):
        text = rest_first(text)
    open(path, 'w').write(text)

def part_rel(comp, name):
    base = pascal_comp(comp)
    if name == base: return ''
    assert name.startswith(base), (comp, name)
    return kebab(name[len(base):])

# ---------- ASTRO ----------
def astro_simple(p):
    tag = p['tag']; data = p['data']
    types = f'HTMLAttributes<"{tag}">'
    if p['linkable']:
        types = 'HTMLAttributes<"button"> & Omit<HTMLAttributes<"a">, "type">'
    def union(comp, k):
        if k == 'variant': return ' | '.join(f'"{x}"' for x in ['default','primary','secondary','outline','ghost','info','success','warning','error'])
        if k == 'size' and comp == 'button': return ' | '.join(f'"{x}"' for x in ['sm','md','lg','icon-sm','icon','icon-lg'])
        if k == 'size' and comp == 'card': return '"default" | "sm"'
        if k == 'size': return '"sm" | "md" | "lg"'
        if k == 'side': return '"top" | "right" | "bottom" | "left"'
        return 'string'
    extra = ''.join(f'  {k}?: {union(p["comp"], k)};\n' for k, _ in data)
    ptype = f'type Props = {types}' + (' & {\n' + extra + '};' if extra else ';')
    names = ''.join(f'{k} = "{d}", ' for k, d in data)
    attrs = ''.join(f' {k}="{v}"' for k, v in p['attrs'].items() if not p['linkable'] or k != 'type')
    dattrs = ''.join(f' data-{k}={{{k}}}' for k, _ in data)
    if p['linkable']:
        head = f'const Tag = Astro.props.href ? "a" : "{tag}";\n'
        open_t = f'<Tag class={{className}}{attrs}'
        # type="button" seulement pour <button>
        typ = ' type={Astro.props.href ? undefined : "button"}' if tag == 'button' else ''
        open_t = f'<Tag class={{className}}{typ} data-slot="{p["slot"]}"{dattrs} {{...rest}}>'
        close_t = '</Tag>'
    else:
        head = ''
        open_t = f'<{tag} class={{className}}{attrs} data-slot="{p["slot"]}"{dattrs} {{...rest}}'
        close_t = f'</{tag}>'
    fm = f'---\nimport type {{ HTMLAttributes }} from "astro/types";\n\n{ptype}\n\nconst {{ {names}class: className, ...rest }} = Astro.props;\n{head}---\n\n'
    if p['void']:
        body = open_t + ' />\n'
    elif p['empty']:
        body = open_t + '></' + tag + '>\n'
    elif p['linkable']:
        body = open_t + '\n  <slot />\n' + close_t + '\n'
    else:
        inner = f'<slot>{p["text"]}</slot>' if p['text'] else '<slot />'
        body = open_t + '>\n  ' + inner + '\n' + close_t + '\n'
    return fm + body

ASTRO_SPECIAL = {}
ASTRO_SPECIAL['Alert'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"div"> & {
  variant?: "default" | "primary" | "secondary" | "info" | "success" | "warning" | "error";
};

const { variant = "default", class: className, ...rest } = Astro.props;

// error / warning : lecteurs d'écran interrompus (role="alert"), sinon annonce polie (role="status")
const inferredRole = rest.role ?? (variant === "error" || variant === "warning" ? "alert" : "status");
---

<div class={className} role={inferredRole} data-slot="alert" data-variant={variant} {...rest}>
  <slot />
</div>
'''
ASTRO_SPECIAL['Separator'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = Omit<HTMLAttributes<"div">, "role" | "aria-orientation"> & {
  /** @default "horizontal" */
  orientation?: "horizontal" | "vertical";
};

const { class: className, orientation = "horizontal", ...rest } = Astro.props;
---

<div
  role="separator"
  aria-orientation={orientation}
  class={className}
  data-slot="separator"
  {...rest}
>
</div>
'''
ASTRO_SPECIAL['Textarea'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"textarea"> & {
  size?: "sm" | "md" | "lg";
  /** Texte initial. */
  defaultValue?: string;
};

const { size = "md", class: className, defaultValue, ...rest } = Astro.props;
---

<textarea class={className} data-slot="textarea" data-size={size} {...rest}>{defaultValue ?? ''}</textarea>
'''
ASTRO_SPECIAL['Checkbox'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = Omit<HTMLAttributes<"input">, "type"> & {
  /** Texte de l'étiquette affichée à côté de la case (liée par `for` / `id`). */
  label?: string;
  size?: "sm" | "md" | "lg";
  variant?: "default" | "primary" | "secondary" | "info" | "success" | "warning" | "error";
};

const { id, label, checked, size = "md", variant = "default", class: className, ...rest } = Astro.props;
---

<div data-slot="checkbox" data-size={size} data-variant={variant}>
  <input type="checkbox" id={id} class={className} checked={checked} data-slot="checkbox-input" {...rest} />
  {label && <label for={id} data-slot="checkbox-label">{label}</label>}
</div>
'''
ASTRO_SPECIAL['Switch'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = Omit<HTMLAttributes<"button">, "role" | "type" | "aria-checked"> & {
  id: string;
  label?: string;
  checked?: boolean;
  size?: "sm" | "md" | "lg";
  variant?: "default" | "primary" | "secondary" | "info" | "success" | "warning" | "error";
};

const { id, label, checked = false, size = "md", variant = "default", class: className, ...rest } = Astro.props;
const ariaLabel = rest["aria-label"] ?? label ?? "switch";
delete rest["aria-label"];
---

<div data-slot="switch">
  <button
    type="button"
    id={id}
    role="switch"
    aria-checked={checked ? "true" : "false"}
    aria-label={ariaLabel}
    class={className}
    data-slot="switch-button"
    data-size={size}
    data-variant={variant}
    {...rest}
  >
    <span data-slot="switch-toggle"></span>
  </button>
  {label && <label for={id} data-slot="switch-label">{label}</label>}
</div>
'''
ASTRO_SPECIAL['Progress'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = Omit<HTMLAttributes<"progress">, "value" | "max"> & {
  /** Valeur courante ; omise = état indéterminé. */
  value?: number;
  max?: number;
  variant?: "default" | "primary" | "secondary" | "info" | "success" | "warning" | "error";
};

const { class: className, value, max = 100, variant = "default", ...rest } = Astro.props;
---

<progress class={className} value={value} max={max} data-slot="progress" data-variant={variant} {...rest}></progress>
'''
ASTRO_SPECIAL['NativeSelect'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = Omit<HTMLAttributes<"select">, "size"> & {
  size?: "sm" | "md" | "lg";
};

const { size = "md", class: className, ...rest } = Astro.props;
---

<div data-slot="native-select-wrapper">
  <select class={className} data-slot="native-select" data-size={size} {...rest}>
    <slot />
  </select>
  <span data-slot="native-select-icon" aria-hidden="true"></span>
</div>
'''
ASTRO_SPECIAL['AvatarImage'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"img"> & {
  src?: string;
  alt?: string;
};

const { src, alt = "", class: className, ...rest } = Astro.props;
---

{src && <img src={src} alt={alt} class={className} data-slot="avatar-image" {...rest} />}
'''
ASTRO_SPECIAL['Tabs'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"div"> & {
  /** Valeur de l'onglet actif au départ. */
  defaultValue?: string;
};

const { defaultValue, class: className, ...rest } = Astro.props;
---

<div class={className} data-slot="tabs" data-default={defaultValue} {...rest}>
  <slot />
</div>
'''
ASTRO_SPECIAL['TabsTrigger'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = Omit<HTMLAttributes<"button">, "type" | "role"> & {
  value: string;
  /** Rendu actif côté serveur (évite le saut de mise en page). */
  defaultActive?: boolean;
};

const { value, defaultActive = false, class: className, ...rest } = Astro.props;
---

<button
  type="button"
  role="tab"
  class={className}
  aria-selected={defaultActive ? "true" : "false"}
  tabindex={defaultActive ? 0 : -1}
  data-slot="tabs-trigger"
  data-value={value}
  {...rest}
>
  <slot />
</button>
'''
ASTRO_SPECIAL['TabsContent'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = Omit<HTMLAttributes<"div">, "role" | "hidden"> & {
  value: string;
  defaultActive?: boolean;
};

const { value, defaultActive = false, class: className, ...rest } = Astro.props;
---

<div role="tabpanel" class={className} hidden={!defaultActive} data-slot="tabs-content" data-value={value} {...rest}>
  <slot />
</div>
'''
ASTRO_SPECIAL['Accordion'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"div"> & {
  /** "single" : un seul élément ouvert à la fois. */
  type?: "single" | "multiple";
  /** Valeur de l'élément ouvert au départ. */
  defaultValue?: string;
};

const { type = "single", defaultValue, class: className, ...rest } = Astro.props;
---

<div class={className} data-slot="accordion" data-type={type} data-default={defaultValue} {...rest}>
  <slot />
</div>
'''
ASTRO_SPECIAL['AccordionItem'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"details"> & {
  value: string;
};

const { value, class: className, ...rest } = Astro.props;
---

<details class={className} data-slot="accordion-item" data-value={value} {...rest}>
  <slot />
</details>
'''
ASTRO_SPECIAL['Dialog'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"div">;

const { class: className, ...rest } = Astro.props;
---

<div class={className} data-slot="dialog" {...rest}>
  <slot />
</div>
'''
ASTRO_SPECIAL['DialogTrigger'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"button"> & {
  /** `id` du Dialog à ouvrir ; inutile à l'intérieur d'un Dialog. */
  for?: string;
  /** Habille un élément enfant (ex. <Button>) au lieu de rendre son propre <button>. */
  asChild?: boolean;
};

const { for: target, asChild = false, class: className, ...rest } = Astro.props;
---

{
  asChild ? (
    <span class={className} data-slot="dialog-trigger" data-as-child data-for={target} {...rest}>
      <slot />
    </span>
  ) : (
    <button type="button" class={className} aria-haspopup="dialog" data-slot="dialog-trigger" data-for={target} {...rest}>
      <slot />
    </button>
  )
}
'''
ASTRO_SPECIAL['DialogClose'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"button"> & {
  /** Habille un élément enfant (ex. <Button>) au lieu de rendre son propre <button>. */
  asChild?: boolean;
};

const { asChild = false, class: className, ...rest } = Astro.props;
---

{
  asChild ? (
    <span class={className} data-slot="dialog-close" data-as-child {...rest}>
      <slot />
    </span>
  ) : (
    <button type="button" class={className} data-slot="dialog-close" {...rest}>
      <slot />
    </button>
  )
}
'''
ASTRO_SPECIAL['DialogContent'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"dialog">;

const { class: className, ...rest } = Astro.props;
---

<dialog class={className} data-slot="dialog-content" {...rest}>
  <slot />
</dialog>
'''
ASTRO_SPECIAL['PaginationLink'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"a"> & {
  /** Page courante : aria-current="page". */
  isActive?: boolean;
};

const { isActive = false, class: className, ...rest } = Astro.props;
---

<a class={className} aria-current={isActive ? "page" : undefined} data-slot="pagination-link" {...rest}>
  <slot />
</a>
'''
ASTRO_SPECIAL['Table'] = '''---
import type { HTMLAttributes } from "astro/types";

type Props = HTMLAttributes<"table">;

const { class: className, ...rest } = Astro.props;
---

<div data-slot="table-container">
  <table class={className} data-slot="table" {...rest}>
    <slot />
  </table>
</div>
'''

# ---------- index.ts (mêmes clés que atomic) ----------
def atomic_default_map(comp):
    src = open(f'{ATOMIC}/{comp}/index.ts').read()
    m = re.search(r'export default \{(.*?)\};', src, re.S)
    out = []
    if not m:
        m1 = re.search(r'export default (\w+);', src)
        return [('', m1.group(1))]
    for line in m.group(1).splitlines():
        line = line.strip().rstrip(',')
        if ':' in line:
            k, v = [x.strip() for x in line.split(':', 1)]
            out.append((k, v))
    return out

parts_by_comp = {}
for p in SIMPLE:
    parts_by_comp.setdefault(p['comp'], []).append(p['name'])
for comp, names in SPECIAL.items():
    parts_by_comp.setdefault(comp, []).extend(names)

# composants sans tag simple ni spécial mais présents (separator, textarea...) déjà dans SPECIAL
all_comps = sorted(parts_by_comp)

for comp in all_comps:
    names = parts_by_comp[comp]
    atomic_files = sorted(f[:-6] for f in os.listdir(f'{ATOMIC}/{comp}') if f.endswith('.astro'))
    # chaque fichier atomic doit avoir son équivalent CSS (sauf parties non applicables, listées explicitement)
    missing = [f for f in atomic_files if f not in names]
    extra = [n for n in names if n not in atomic_files]
    if missing or extra:
        print('ÉCART', comp, 'manquants:', missing, 'en plus:', extra)
    for p in [x for x in SIMPLE if x['comp'] == comp]:
        w(f'{OUT_ASTRO}/{comp}/{p["name"]}.astro', astro_simple(p))
    for n in SPECIAL.get(comp, []):
        w(f'{OUT_ASTRO}/{comp}/{n}.astro', ASTRO_SPECIAL[n])
    dm = [(k, v) for k, v in atomic_default_map(comp) if v in names]
    imports = ''.join(f'import {n} from "./{n}.astro";\n' for n in sorted(names))
    exports = ', '.join(sorted(names))
    if len(dm) == 1 and dm[0][0] == '':
        dflt = f'export default {dm[0][1]};\n'
    else:
        dflt = 'export default {\n' + ''.join(f'  {k}: {v},\n' for k, v in dm) + '};\n'
    w(f'{OUT_ASTRO}/{comp}/index.ts', f'{imports}\nexport {{ {exports} }};\n\n{dflt}')

print('astro ok', len(all_comps), 'composants')

# ---------- BLADE ----------
def blade_path(comp, name):
    rel = part_rel(comp, name)
    return f'{OUT_BLADE}/{comp}/{rel or comp}.blade.php'

def blade_simple(p):
    tag = p['tag']
    props = ', '.join(f"'{k}' => '{d}'" for k, d in p['data'])
    head = f"@props([{props}])\n" if props else ''
    merge = [f"'data-slot' => '{p['slot']}'"] + [f"'data-{k}' => ${k}" for k, _ in p['data']]
    fixed = [f"'{k}' => '{v}'" for k, v in p['attrs'].items() if not (p['linkable'] and k == 'type')]
    def el(t, extra=()):
        m = ', '.join(merge + fixed + list(extra))
        o = f"<{t} {{{{ $attributes->merge([{m}]) }}}}"
        if p['void']:
            return o + '>'
        if p['empty']:
            return o + f'></{t}>'
        if p['text']:
            return o + f">@if ($slot->isEmpty()) {p['text']} @else {{{{ $slot }}}} @endif</{t}>"
        return o + f">{{{{ $slot }}}}</{t}>"
    if p['linkable']:
        has_type = 'type' in p['attrs']
        other = 'a' if tag != 'a' else 'button'
        return head + "@if ($attributes->has('href'))\n    " + el('a') + "\n@else\n    " + el(tag, ["'type' => 'button'"] if has_type else []) + "\n@endif\n"
    return head + el(tag) + '\n'

BLADE_SPECIAL = {}
BLADE_SPECIAL['Alert'] = """@props(['variant' => 'default'])
@php
    $role = $attributes->get('role') ?? (in_array($variant, ['error', 'warning'], true) ? 'alert' : 'status');
@endphp
<div {{ $attributes->merge(['role' => $role, 'data-slot' => 'alert', 'data-variant' => $variant]) }}>{{ $slot }}</div>
"""
BLADE_SPECIAL['Separator'] = """@props(['orientation' => 'horizontal'])
<div {{ $attributes->merge(['role' => 'separator', 'aria-orientation' => $orientation, 'data-slot' => 'separator']) }}></div>
"""
BLADE_SPECIAL['Textarea'] = """@props(['size' => 'md', 'defaultValue' => ''])
<textarea {{ $attributes->merge(['data-slot' => 'textarea', 'data-size' => $size]) }}>{{ $defaultValue }}{{ $slot }}</textarea>
"""
BLADE_SPECIAL['Checkbox'] = """@props(['id' => null, 'label' => null, 'checked' => false, 'size' => 'md', 'variant' => 'default'])
<div data-slot="checkbox" data-size="{{ $size }}" data-variant="{{ $variant }}">
    <input type="checkbox" @if ($id) id="{{ $id }}" @endif @checked($checked) {{ $attributes->merge(['data-slot' => 'checkbox-input']) }}>
    @if ($label)
        <label @if ($id) for="{{ $id }}" @endif data-slot="checkbox-label">{{ $label }}</label>
    @endif
</div>
"""
BLADE_SPECIAL['Switch'] = """@props(['id', 'label' => null, 'checked' => false, 'size' => 'md', 'variant' => 'default'])
@php
    $ariaLabel = $attributes->get('aria-label') ?? $label ?? 'switch';
@endphp
<div data-slot="switch">
    <button type="button" id="{{ $id }}" role="switch" aria-checked="{{ $checked ? 'true' : 'false' }}" aria-label="{{ $ariaLabel }}" {{ $attributes->except('aria-label')->merge(['data-slot' => 'switch-button', 'data-size' => $size, 'data-variant' => $variant]) }}>
        <span data-slot="switch-toggle"></span>
    </button>
    @if ($label)
        <label for="{{ $id }}" data-slot="switch-label">{{ $label }}</label>
    @endif
</div>
"""
BLADE_SPECIAL['Progress'] = """@props(['value' => null, 'max' => 100, 'variant' => 'default'])
@php
    $progress = ['data-slot' => 'progress', 'data-variant' => $variant, 'max' => $max];
    if (! is_null($value)) {
        $progress['value'] = $value;
    }
@endphp
<progress {{ $attributes->merge($progress) }}></progress>
"""
BLADE_SPECIAL['NativeSelect'] = """@props(['size' => 'md'])
<div data-slot="native-select-wrapper">
    <select {{ $attributes->merge(['data-slot' => 'native-select', 'data-size' => $size]) }}>{{ $slot }}</select>
    <span data-slot="native-select-icon" aria-hidden="true"></span>
</div>
"""
BLADE_SPECIAL['AvatarImage'] = """@props(['src' => null, 'alt' => ''])
@if ($src)
    <img src="{{ $src }}" alt="{{ $alt }}" {{ $attributes->merge(['data-slot' => 'avatar-image']) }}>
@endif
"""
BLADE_SPECIAL['Tabs'] = """@props(['defaultValue' => null])
<div {{ $attributes->merge(['data-slot' => 'tabs', 'data-default' => $defaultValue]) }}>{{ $slot }}</div>
"""
BLADE_SPECIAL['TabsTrigger'] = """@props(['value', 'defaultActive' => false])
<button {{ $attributes->merge(['type' => 'button', 'role' => 'tab', 'aria-selected' => $defaultActive ? 'true' : 'false', 'tabindex' => $defaultActive ? 0 : -1, 'data-slot' => 'tabs-trigger', 'data-value' => $value]) }}>{{ $slot }}</button>
"""
BLADE_SPECIAL['TabsContent'] = """@props(['value', 'defaultActive' => false])
<div {{ $attributes->merge(['role' => 'tabpanel', 'data-slot' => 'tabs-content', 'data-value' => $value]) }} @unless ($defaultActive) hidden @endunless>{{ $slot }}</div>
"""
BLADE_SPECIAL['Accordion'] = """@props(['type' => 'single', 'defaultValue' => null])
<div {{ $attributes->merge(['data-slot' => 'accordion', 'data-type' => $type, 'data-default' => $defaultValue]) }}>{{ $slot }}</div>
"""
BLADE_SPECIAL['AccordionItem'] = """@props(['value'])
<details {{ $attributes->merge(['data-slot' => 'accordion-item', 'data-value' => $value]) }}>{{ $slot }}</details>
"""
BLADE_SPECIAL['Dialog'] = """<div {{ $attributes->merge(['data-slot' => 'dialog']) }}>{{ $slot }}</div>
"""
BLADE_SPECIAL['DialogTrigger'] = """@props(['for' => null, 'asChild' => false])
@if ($asChild)
    <span {{ $attributes->merge(['data-slot' => 'dialog-trigger', 'data-as-child' => true, 'data-for' => $for]) }}>{{ $slot }}</span>
@else
    <button {{ $attributes->merge(['type' => 'button', 'aria-haspopup' => 'dialog', 'data-slot' => 'dialog-trigger', 'data-for' => $for]) }}>{{ $slot }}</button>
@endif
"""
BLADE_SPECIAL['DialogClose'] = """@props(['asChild' => false])
@if ($asChild)
    <span {{ $attributes->merge(['data-slot' => 'dialog-close', 'data-as-child' => true]) }}>{{ $slot }}</span>
@else
    <button {{ $attributes->merge(['type' => 'button', 'data-slot' => 'dialog-close']) }}>{{ $slot }}</button>
@endif
"""
BLADE_SPECIAL['DialogContent'] = """<dialog {{ $attributes->merge(['data-slot' => 'dialog-content']) }}>{{ $slot }}</dialog>
"""
BLADE_SPECIAL['PaginationLink'] = """@props(['isActive' => false])
<a {{ $attributes->merge(['aria-current' => $isActive ? 'page' : null, 'data-slot' => 'pagination-link']) }}>{{ $slot }}</a>
"""
BLADE_SPECIAL['Table'] = """<div data-slot="table-container">
    <table {{ $attributes->merge(['data-slot' => 'table']) }}>{{ $slot }}</table>
</div>
"""

for p in SIMPLE:
    w(blade_path(p['comp'], p['name']), blade_simple(p))
for comp, names in SPECIAL.items():
    for n in names:
        w(blade_path(comp, n), BLADE_SPECIAL[n])

# ---------- DJANGO ----------
HEADER = '''"""Composants du catalogue AlphaRust pour les gabarits Django/Wagtail.

Même balisage (attributs ``data-slot``) que les composants Astro et Blade ; le style vient de
``static/ds/css`` (voir README du projet). Usage dans un gabarit ::

    {% load ds %}
    {% ds_button variant="primary" %}Enregistrer{% endds_button %}

Les balises à contenu utilisent ``simple_block_tag`` (Django 5.2+), les éléments vides ``simple_tag``.
Les attributs HTML supplémentaires sont passés en arguments nommés ; ``_`` devient ``-``
(``aria_label="x"`` -> ``aria-label="x"``) et ``class_`` devient ``class``.
"""

from django import template
from django.utils.html import format_html, format_html_join

register = template.Library()


def _attrs(slot, data, extra):
    out = {"data-slot": slot}
    for key, value in data.items():
        if value is not None:
            out["data-" + key.replace("_", "-")] = value
    for key, value in extra.items():
        out[key.rstrip("_").replace("_", "-")] = value
    return out


def _flat(attrs):
    """Attributs HTML échappés (format_html), dans l'ordre donné ; True = attribut booléen, False/None = omis."""
    parts = []
    for key, value in attrs.items():
        if value is True:
            parts.append(format_html(" {}", key))
        elif value is not False and value is not None:
            parts.append(format_html(' {}="{}"', key, value))
    return format_html_join("", "{}", ((p,) for p in parts))


def _el(tag, attrs, content=""):
    return format_html("<{tag}{attrs}>{content}</{tag}>", tag=tag, attrs=_flat(attrs), content=content)


def _void(tag, attrs):
    return format_html("<{tag}{attrs}>", tag=tag, attrs=_flat(attrs))

'''

def dj_name(p): return 'ds_' + p['slot'].replace('-', '_')

def dj_simple(p):
    name = dj_name(p)
    sig_data = ''.join(f', {k}="{d}"' for k, d in p['data'])
    data_dict = '{' + ', '.join(f'"{k}": {k}' for k, _ in p['data']) + '}'
    fixed = p['attrs']
    fx = ('{' + ', '.join(f'"{k}": "{v}"' for k, v in fixed.items()) + '}') if fixed else '{}'
    if p['void'] or p['empty']:
        dec = f'@register.simple_tag(name="{name}")'
        fn = f'def {name}({sig_data.lstrip(", ")}{", " if sig_data else ""}**attrs):'
        if p['void']:
            body = f'    return _void("{p["tag"]}", _attrs("{p["slot"]}", {data_dict}, {{**{fx}, **attrs}}))'
        else:
            body = f'    return _el("{p["tag"]}", _attrs("{p["slot"]}", {data_dict}, {{**{fx}, **attrs}}))'
        return f'{dec}\n{fn}\n{body}\n'
    dec = f'@register.simple_block_tag(name="{name}")'
    fn = f'def {name}(content{sig_data}, **attrs):'
    if p['linkable']:
        typ = '    if tag != "a":\n        attrs.setdefault("type", "button")\n' if 'type' in p['attrs'] else ''
        body = (f'    tag = "a" if "href" in attrs else "{p["tag"]}"\n'
                + typ
                + f'    return _el(tag, _attrs("{p["slot"]}", {data_dict}, attrs), content)')
    elif p['text']:
        body = f'    return _el("{p["tag"]}", _attrs("{p["slot"]}", {data_dict}, {{**{fx}, **attrs}}), content or "{p["text"]}")'
    else:
        body = f'    return _el("{p["tag"]}", _attrs("{p["slot"]}", {data_dict}, {{**{fx}, **attrs}}), content)'
    return f'{dec}\n{fn}\n{body}\n'

DJ_SPECIAL = '''
@register.simple_block_tag(name="ds_alert")
def ds_alert(content, variant="default", **attrs):
    # error / warning : role="alert" ; sinon role="status"
    attrs.setdefault("role", "alert" if variant in ("error", "warning") else "status")
    return _el("div", _attrs("alert", {"variant": variant}, attrs), content)


@register.simple_tag(name="ds_separator")
def ds_separator(orientation="horizontal", **attrs):
    attrs = {"role": "separator", "aria-orientation": orientation, **attrs}
    return _el("div", _attrs("separator", {}, attrs))


@register.simple_block_tag(name="ds_textarea")
def ds_textarea(content, size="md", default_value="", **attrs):
    return _el("textarea", _attrs("textarea", {"size": size}, attrs), f"{default_value}{content}")


@register.simple_tag(name="ds_checkbox")
def ds_checkbox(id=None, label=None, checked=False, size="md", variant="default", **attrs):
    if id:
        attrs["id"] = id
    attrs["checked"] = bool(checked)
    box = _void("input", {"type": "checkbox", **_attrs("checkbox-input", {}, attrs)})
    text = _el("label", {"for": id, "data-slot": "checkbox-label"} if id else {"data-slot": "checkbox-label"}, label) if label else ""
    return _el("div", {"data-slot": "checkbox", "data-size": size, "data-variant": variant}, box + text)


@register.simple_tag(name="ds_switch")
def ds_switch(id, label=None, checked=False, size="md", variant="default", **attrs):
    aria_label = attrs.pop("aria-label", None) or attrs.pop("aria_label", None) or label or "switch"
    button = _el(
        "button",
        {
            "type": "button",
            "id": id,
            "role": "switch",
            "aria-checked": "true" if checked else "false",
            "aria-label": aria_label,
            **_attrs("switch-button", {"size": size, "variant": variant}, attrs),
        },
        _el("span", {"data-slot": "switch-toggle"}),
    )
    text = _el("label", {"for": id, "data-slot": "switch-label"}, label) if label else ""
    return _el("div", {"data-slot": "switch"}, button + text)


@register.simple_tag(name="ds_progress")
def ds_progress(value=None, max=100, variant="default", **attrs):
    attrs = {"max": max, **attrs}
    if value is not None:
        attrs["value"] = value
    return _el("progress", _attrs("progress", {"variant": variant}, attrs))


@register.simple_block_tag(name="ds_native_select")
def ds_native_select(content, size="md", **attrs):
    select = _el("select", _attrs("native-select", {"size": size}, attrs), content)
    icon = _el("span", {"data-slot": "native-select-icon", "aria-hidden": "true"})
    return _el("div", {"data-slot": "native-select-wrapper"}, select + icon)


@register.simple_tag(name="ds_avatar_image")
def ds_avatar_image(src=None, alt="", **attrs):
    if not src:
        return ""
    return _void("img", {"src": src, "alt": alt, **_attrs("avatar-image", {}, attrs)})


@register.simple_block_tag(name="ds_tabs")
def ds_tabs(content, default_value=None, **attrs):
    return _el("div", _attrs("tabs", {"default": default_value}, attrs), content)


@register.simple_block_tag(name="ds_tabs_trigger")
def ds_tabs_trigger(content, value, default_active=False, **attrs):
    attrs = {
        "type": "button",
        "role": "tab",
        "aria-selected": "true" if default_active else "false",
        "tabindex": 0 if default_active else -1,
        **attrs,
    }
    return _el("button", _attrs("tabs-trigger", {"value": value}, attrs), content)


@register.simple_block_tag(name="ds_tabs_content")
def ds_tabs_content(content, value, default_active=False, **attrs):
    attrs = {"role": "tabpanel", "hidden": not default_active, **attrs}
    return _el("div", _attrs("tabs-content", {"value": value}, attrs), content)


@register.simple_block_tag(name="ds_accordion")
def ds_accordion(content, type="single", default_value=None, **attrs):
    return _el("div", _attrs("accordion", {"type": type, "default": default_value}, attrs), content)


@register.simple_block_tag(name="ds_accordion_item")
def ds_accordion_item(content, value, **attrs):
    return _el("details", _attrs("accordion-item", {"value": value}, attrs), content)


@register.simple_block_tag(name="ds_dialog")
def ds_dialog(content, **attrs):
    return _el("div", _attrs("dialog", {}, attrs), content)


@register.simple_block_tag(name="ds_dialog_trigger")
def ds_dialog_trigger(content, as_child=False, **attrs):
    # `for="id-du-dialogue"` : ouvrir un dialogue situé ailleurs dans la page
    target = attrs.pop("for", None)
    if as_child:
        return _el("span", _attrs("dialog-trigger", {"as-child": True, "for": target}, attrs), content)
    attrs = {"type": "button", "aria-haspopup": "dialog", **attrs}
    return _el("button", _attrs("dialog-trigger", {"for": target}, attrs), content)


@register.simple_block_tag(name="ds_dialog_close")
def ds_dialog_close(content, as_child=False, **attrs):
    if as_child:
        return _el("span", _attrs("dialog-close", {"as-child": True}, attrs), content)
    attrs = {"type": "button", **attrs}
    return _el("button", _attrs("dialog-close", {}, attrs), content)


@register.simple_block_tag(name="ds_dialog_content")
def ds_dialog_content(content, **attrs):
    return _el("dialog", _attrs("dialog-content", {}, attrs), content)


@register.simple_block_tag(name="ds_pagination_link")
def ds_pagination_link(content, is_active=False, **attrs):
    if is_active:
        attrs["aria-current"] = "page"
    return _el("a", _attrs("pagination-link", {}, attrs), content)


@register.simple_block_tag(name="ds_table")
def ds_table(content, **attrs):
    return _el("div", {"data-slot": "table-container"}, _el("table", _attrs("table", {}, attrs), content))
'''

dj = HEADER + '\n\n'.join(dj_simple(p) for p in SIMPLE) + '\n' + DJ_SPECIAL
w(OUT_DJ, dj)
w(ROOT + '/ui-django/ds/__init__.py', '')
w(ROOT + '/ui-django/ds/templatetags/__init__.py', '')
w(ROOT + '/ui-django/ds/apps.py', '''from django.apps import AppConfig


class DsConfig(AppConfig):
    """Catalogue de composants (balises de gabarit `{% load ds %}` + fichiers statiques `ds/`)."""

    name = "ds"
    verbose_name = "Design system"
''')
print('blade+django ok')

# ---------- composants étendus (CSS pur, Astro) ----------
exec(open(os.path.join(HERE, 'ext_write.py')).read())

# ---------- composants étendus côté serveur (Blade, Django) ----------
ext = sorted(d for d in os.listdir(ATOMIC) if os.path.isdir(f'{ATOMIC}/{d}') and d not in parts_by_comp)
exec(open(os.path.join(HERE, 'ext_server.py')).read())

# ---------- site complet (même site en Astro, Laravel et Wagtail : tools/dsgen/vitrine/spec.py) ----------
from vitrine.write import write_all as write_site
write_site(ROOT)
print('site-astro + site-laravel + site-wagtail ok')

# ---------- catalog.toml ----------
import glob
slot_of = {}
for p in SIMPLE: slot_of[p['name']] = p['slot']
special_slots = {
 'Alert':'alert','Separator':'separator','Textarea':'textarea','Checkbox':'checkbox','Switch':'switch','Progress':'progress',
 'NativeSelect':'native-select','AvatarImage':'avatar-image','Tabs':'tabs','TabsTrigger':'tabs-trigger','TabsContent':'tabs-content',
 'Accordion':'accordion','AccordionItem':'accordion-item','Dialog':'dialog','DialogTrigger':'dialog-trigger','DialogContent':'dialog-content','DialogClose':'dialog-close',
 'PaginationLink':'pagination-link','Table':'table'}
slot_of.update(special_slots)
# slots additionnels du contrat (éléments internes posés par les composants racine)
extra_slots = {'checkbox':['checkbox-input','checkbox-label'], 'switch':['switch-button','switch-toggle','switch-label'],
               'native-select':['native-select-wrapper','native-select-icon'], 'table':['table-container']}
atoms = sorted(d for d in os.listdir(ATOMIC) if os.path.isdir(f'{ATOMIC}/{d}'))
core = sorted(parts_by_comp)
ext = [a for a in atoms if a not in core]
lines = ['# Catalogue de composants AlphaRust — source de vérité du contrat de balisage.',
 '# Chaque composant « core » existe dans les quatre cibles : Starwind (Tailwind), CSS pur (Astro),',
 '# Blade (Laravel) et balises de gabarit (Django/Wagtail). Le test de parité (src/scaffold/catalog.rs)',
 '# vérifie ce fichier contre les fichiers réels.', '', '[meta]', 'contract = "data-slot"', '']
for comp in core:
    slots = sorted({slot_of[n] for n in parts_by_comp[comp]} | set(extra_slots.get(comp, [])))
    atom_slots = set()
    for f in glob.glob(f'{ATOMIC}/{comp}/*.astro'):
        txt = open(f).read()
        atom_slots |= set(re.findall(r'data-slot="([a-z-]+)"', txt))
        atom_slots |= set(re.findall(r'data-slot=\{?`?([a-z-]+)', txt))
    atom_only = sorted(atom_slots - set(slots))
    lines += ['[[component]]', f'name = "{comp}"', 'kind = "core"',
              'parts = [' + ', '.join(f'"{n}"' for n in sorted(parts_by_comp[comp])) + ']',
              'slots = [' + ', '.join(f'"{s}"' for s in slots) + ']']
    if atom_only:
        lines.append('atom_only = [' + ', '.join(f'"{s}"' for s in atom_only) + ']')
    lines.append('')
def slots_in(d):
    found = set()
    for f in glob.glob(f'{d}/*.astro'):
        txt = open(f).read()
        found |= set(re.findall(r'data-slot="([a-z-]+)"', txt))
        found |= set(re.findall(r'"data-slot":\s*"([a-z-]+)"', txt))
    return found

lines += ['# Composants « étendus » : atomes Starwind ET équivalents CSS pur (Astro) aux mêmes noms, props et data-slot.',
          '# Leur style vit dans design-kit/css/components (donc aussi servi à Laravel et Wagtail) ;',
          '# Blade et Django couvrent les 46 composants. Tout dossier d\'atomes doit être core ou css (test de parité).', '']
for comp in ext:
    a, c = slots_in(f'{ATOMIC}/{comp}'), slots_in(f'{OUT_ASTRO}/{comp}')
    lines += ['[[component]]', f'name = "{comp}"', 'kind = "css"',
              'slots = [' + ', '.join(f'"{s}"' for s in sorted(a & c)) + ']']
    if a - c:
        lines.append('atom_only = [' + ', '.join(f'"{s}"' for s in sorted(a - c)) + ']')
    if c - a:
        lines.append('css_only = [' + ', '.join(f'"{s}"' for s in sorted(c - a)) + ']')
    lines.append('')
lines += ['# Écarts connus entre le DOM rendu par les atomes Starwind (dépôt atomic, repris tels quels) et le contrat.',
          '# Mesurés par le script de parité runtime (docs/tests.md) ; tout autre écart est une erreur.',
          '[known_gaps]',
          'atoms_missing_slots = ["checkbox", "switch", "tooltip-trigger"]',
          'atoms_extra_slots = ["dialog-backdrop"]',
          'atoms_slot_count_differs = []', '']
open(ROOT + '/design-kit/catalog.toml', 'w').write('\n'.join(lines))
print('catalog.toml', len(core), 'core', len(ext), 'css')
