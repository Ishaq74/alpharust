# Composants « étendus » côté serveur : Blade (Laravel) et balises Django (Wagtail), générés depuis UNE description.
#
# Chaque partie (AlertDialogAction, SelectItem…) est décrite par une petite représentation intermédiaire (IR) :
# éléments, attributs, slot principal, conditions, boucles. Les parties « simples » (un élément, attributs littéraux
# ou props, slot) sont lues directement dans les atomes CSS pur (ui-css) ; les autres sont décrites dans MANUAL.
# Deux rendus : composant Blade anonyme (@props, $attributes->merge — https://laravel.com/docs/blade#components)
# et balise Django simple_tag / simple_block_tag (https://docs.djangoproject.com/en/stable/howto/custom-template-tags/).
# Importé par gen_ds.py après ext_write.write() (generate()) ; ne pas éditer les fichiers générés.
import glob
import os
import re

from common import OUT_BLADE, OUT_DJ, ROOT, ext_components, part_rel, w

EXT_DIR = os.path.join(ROOT, 'ui-css', 'src', 'components', 'atoms')

# ---------------------------------------------------------------- IR
class L:            # littéral
    def __init__(s, v): s.v = v
class V:            # variable : prop, valeur calculée ou variable de boucle ("t.pct")
    def __init__(s, n): s.n = n
class IF:
    def __init__(s, c, a, b=None): s.c, s.a, s.b = c, a, b
class EQ:
    def __init__(s, x, lit): s.x, s.lit = x, lit
class NOT:
    def __init__(s, c): s.c = c
class OR:
    def __init__(s, a, b): s.a, s.b = a, b
class AND:
    def __init__(s, a, b): s.a, s.b = a, b
class F:            # chaîne avec {variables}
    def __init__(s, fmt): s.fmt = fmt
class RAW:          # expression écrite dans chaque langage
    def __init__(s, php, py): s.php, s.py = php, py

class El:
    def __init__(s, tag, attrs=(), children=(), rest=False, void=False):
        s.tag, s.attrs, s.children, s.rest, s.void = tag, list(attrs), list(children), rest, void
class Slot:
    def __init__(s, default=None): s.default = default
class T:            # HTML brut (icônes SVG, textes fixes)
    def __init__(s, html): s.html = html
class Out:          # texte échappé
    def __init__(s, e): s.e = e
class C:
    def __init__(s, c, then, other=()): s.c, s.then, s.other = c, list(then), list(other)
class For:
    def __init__(s, var, lst, children): s.var, s.lst, s.children = var, lst, list(children)

class Part:
    def __init__(s, comp, name, props=(), body=(), lets=()):
        s.comp, s.name, s.props, s.body, s.lets = comp, name, list(props), list(body), list(lets)

def lit(v):
    return v if isinstance(v, (L, V, IF, EQ, NOT, OR, AND, F, RAW)) else L(v)

# ---------------------------------------------------------------- noms
def camel(snake):
    p = snake.split('_')
    return p[0] + ''.join(x.capitalize() for x in p[1:])
def snake_of(camel_name):
    return re.sub(r'(?<!^)(?=[A-Z])', '_', camel_name).lower()
PY_RESERVED = {'for', 'as', 'class', 'in', 'is', 'type_'}
def pyname(n):
    return n + '_' if n in ('for', 'as', 'class', 'in', 'is') else n

# ---------------------------------------------------------------- expressions -> PHP
def php_str(s):
    return "'" + str(s).replace('\\', '\\\\').replace("'", "\\'") + "'"
def php_lit(v):
    if v is None: return 'null'
    if v is True: return 'true'
    if v is False: return 'false'
    if isinstance(v, (int, float)): return repr(v)
    return php_str(v)
def php_var(n):
    if '.' in n:
        a, b = n.split('.', 1)
        return f"${camel(a)}['{b}']"
    return '$' + camel(n)
def php(e):
    e = lit(e)
    if isinstance(e, L): return php_lit(e.v)
    if isinstance(e, V): return php_var(e.n)
    if isinstance(e, IF): return f'({php(e.c)} ? {php(e.a)} : {php(e.b)})'
    if isinstance(e, EQ): return f'({php(e.x)} === {php_lit(e.lit)})'
    if isinstance(e, NOT): return f'(! {php(e.c)})'
    if isinstance(e, OR): return f'({php(e.a)} ?: {php(e.b)})'
    if isinstance(e, AND): return f'({php(e.a)} && {php(e.b)})'
    if isinstance(e, F):
        parts = []
        for i, chunk in enumerate(re.split(r'\{([a-z_.]+)\}', e.fmt)):
            if i % 2: parts.append(php_var(chunk))
            elif chunk: parts.append(php_str(chunk))
        return '(' + ' . '.join(parts or ["''"]) + ')'
    if isinstance(e, RAW): return e.php
    raise TypeError(e)

# ---------------------------------------------------------------- expressions -> Python
def py_var(n):
    if '.' in n:
        a, b = n.split('.', 1)
        return f'{pyname(a)}["{b}"]'
    return pyname(n)
def py(e):
    e = lit(e)
    if isinstance(e, L): return repr(e.v)
    if isinstance(e, V): return py_var(e.n)
    if isinstance(e, IF): return f'({py(e.a)} if {py(e.c)} else {py(e.b)})'
    if isinstance(e, EQ): return f'({py(e.x)} == {e.lit!r})'
    if isinstance(e, NOT): return f'(not {py(e.c)})'
    if isinstance(e, OR): return f'({py(e.a)} or {py(e.b)})'
    if isinstance(e, AND): return f'({py(e.a)} and {py(e.b)})'
    if isinstance(e, F):
        parts = []
        for i, chunk in enumerate(re.split(r'\{([a-z_.]+)\}', e.fmt)):
            if i % 2: parts.append(f'str({py_var(chunk)})')
            elif chunk: parts.append(repr(chunk))
        return '(' + ' + '.join(parts or ["''"]) + ')'
    if isinstance(e, RAW): return e.py
    raise TypeError(e)

def is_static(e):
    return isinstance(lit(e), L)

# ---------------------------------------------------------------- rendu Blade
def html_attr(k, v):
    if v is None or v is False: return ''
    if v is True or v == '': return f' {k}'
    return f' {k}="' + str(v).replace('&', '&amp;').replace('"', '&quot;') + '"'

def blade_tag(t):
    return t if isinstance(t, str) else '{{ ' + php(t) + ' }}'

def blade_attrs(el):
    if el.rest:
        inner = ', '.join(f"'{k}' => {php(v)}" for k, v in el.attrs)
        return ' {{ $attributes->merge([' + inner + ']) }}'
    if all(is_static(v) for _, v in el.attrs):
        return ''.join(html_attr(k, lit(v).v) for k, v in el.attrs)
    inner = ', '.join(f"'{k}' => {php(v)}" for k, v in el.attrs)
    return ' {{ new \\Illuminate\\View\\ComponentAttributeBag([' + inner + ']) }}'

def blade_nodes(nodes):
    return ''.join(blade_node(n) for n in nodes)

def blade_node(n):
    if isinstance(n, El):
        o = f'<{blade_tag(n.tag)}{blade_attrs(n)}>'
        if n.void: return o
        return o + blade_nodes(n.children) + f'</{blade_tag(n.tag)}>'
    if isinstance(n, Slot):
        if not n.default: return '{{ $slot }}'
        return "\n@if (trim((string) $slot) === '')\n" + blade_nodes(n.default) + '\n@else\n{{ $slot }}\n@endif\n'
    if isinstance(n, T): return n.html
    if isinstance(n, Out): return '{{ ' + php(n.e) + ' }}'
    if isinstance(n, C):
        s = f'\n@if ({php(n.c)})\n' + blade_nodes(n.then)
        if n.other: s += '\n@else\n' + blade_nodes(n.other)
        return s + '\n@endif\n'
    if isinstance(n, For):
        return f'\n@foreach ({php(n.lst)} as {php_var(n.var)})\n' + blade_nodes(n.children) + '\n@endforeach\n'
    raise TypeError(n)

def blade_part(p):
    out = []
    if p.props:
        out.append('@props([' + ', '.join(f"'{camel(k)}' => {php_lit(d)}" for k, d in p.props) + '])')
    if p.lets:
        out.append('@php\n' + ''.join(f'    {php_var(k)} = {code};\n' for k, code, _ in p.lets) + '@endphp')
    body = blade_nodes(p.body)
    body = re.sub(r'\n{2,}', '\n', body).strip('\n')
    out.append(body)
    return '\n'.join(out) + '\n'

# ---------------------------------------------------------------- rendu Django (Python)
def py_attrs(el):
    d = '{' + ', '.join(f'{k!r}: {py(v)}' for k, v in el.attrs) + '}'
    return '{**' + d + ', **attrs}' if el.rest else d

def py_nodes(nodes):
    if not nodes: return '""'
    if len(nodes) == 1: return py_node(nodes[0])
    return '_join(' + ', '.join(py_node(n) for n in nodes) + ')'

def py_node(n):
    if isinstance(n, El):
        tag = repr(n.tag) if isinstance(n.tag, str) else py(n.tag)
        if n.void: return f'_void({tag}, {py_attrs(n)})'
        return f'_el({tag}, {py_attrs(n)}, {py_nodes(n.children)})'
    if isinstance(n, Slot):
        if not n.default: return 'content'
        return f'(content if str(content).strip() else {py_nodes(n.default)})'
    if isinstance(n, T): return 'mark_safe(' + repr(n.html) + ')'
    if isinstance(n, Out): return py(n.e)
    if isinstance(n, C): return f'({py_nodes(n.then)} if {py(n.c)} else {py_nodes(n.other)})'
    if isinstance(n, For):
        return f'_join(*({py_nodes(n.children)} for {pyname(n.var)} in {py(n.lst)}))'
    raise TypeError(n)

def has_slot(nodes):
    for n in nodes:
        if isinstance(n, Slot): return True
        for sub in ('children', 'then', 'other'):
            if has_slot(getattr(n, sub, []) or []): return True
    return False

def dj_tag_name(p):
    return 'ds_' + snake_of(p.name)

def django_part(p):
    name = dj_tag_name(p)
    block = has_slot(p.body)
    params = ''.join(f', {pyname(k)}={d!r}' for k, d in p.props)
    head = ('content' + params) if block else params.lstrip(', ')
    sep = ', ' if head else ''
    deco = 'simple_block_tag' if block else 'simple_tag'
    lines = [f'@register.{deco}(name="{name}")', f'def {name}({head}{sep}**attrs):', '    attrs = _x(attrs)']
    for k, _, code in p.lets:
        lines.append(f'    {pyname(k)} = {code}')
    lines.append(f'    return {py_nodes(p.body)}')
    # les icônes SVG portent xmlns="http://www.w3.org/…" : ce n'est pas une URL de service
    lines = [l + '  # alpharust-ignore: no-hardcoded-urls' if 'http' in l else l for l in lines]
    return '\n'.join(lines) + '\n'

# ---------------------------------------------------------------- parties simples, lues dans ui-css
SIMPLE_RE = re.compile(r'<([a-z][a-z0-9]*|Tag)((?:\s+(?:[a-zA-Z:-]+(?:="[^"]*"|=\{[A-Za-z]+\})?|\{\.\.\.rest\}))*)\s*(?:/>|>\s*(?:<slot\s*/>)?\s*</\1>)', re.S)
VOID = {'input', 'img', 'br', 'hr', 'track', 'source'}

def astro_split(path):
    src = open(path).read()
    m = re.match(r'---\n(.*?)^---\n(.*)', src, re.S | re.M)
    tpl = re.sub(r'\{/\*.*?\*/\}', '', m.group(2), flags=re.S)
    tpl = re.sub(r'<!--.*?-->', '', tpl, flags=re.S).strip()
    return m.group(1), tpl

def parse_default(s):
    s = s.strip()
    if s in ('true', 'false'): return s == 'true'
    if re.fullmatch(r'-?\d+', s): return int(s)
    if re.fullmatch(r'"[^"]*"', s): return s[1:-1]
    raise ValueError(s)

def auto_part(comp, name, path):
    fm, tpl = astro_split(path)
    if tpl == '<slot />':
        return Part(comp, name, [], [Slot()])
    m = SIMPLE_RE.fullmatch(tpl)
    if not m: return None
    props, renames = [], {}
    pm = re.search(r'const \{(.*?)\} = Astro\.props', fm, re.S)
    for entry in (pm.group(1).split(',') if pm else []):
        entry = entry.strip()
        if not entry or entry.startswith('...') or entry.startswith('class:'): continue
        if entry.startswith('as:'):
            props.append(('as', parse_default(entry.split('=', 1)[1]))); renames['Tag'] = 'as'; continue
        k, _, d = entry.partition('=')
        props.append((snake_of(k.strip()), parse_default(d) if d else None))
    names = {k for k, _ in props}
    attrs, rest = [], False
    for a in re.findall(r'[a-zA-Z:-]+(?:="[^"]*"|=\{[A-Za-z]+\})?|\{\.\.\.rest\}', m.group(2)):
        if a == '{...rest}': rest = True; continue
        k, _, v = a.partition('=')
        if k == 'class': continue
        if not v: attrs.append((k, L(''))); continue
        if v.startswith('"'): attrs.append((k, L(v[1:-1]))); continue
        var = snake_of(v[1:-1])
        assert var in names, (path, var)
        attrs.append((k, V(var)))
    tag = m.group(1)
    tag = V('as') if tag == 'Tag' else tag
    void = isinstance(tag, str) and tag in VOID
    children = [Slot()] if '<slot' in tpl else []
    return Part(comp, name, props, [El(tag, attrs, children, rest=rest, void=void)])

# ---------------------------------------------------------------- icônes (mêmes SVG que les atomes CSS pur)
def svg(attrs, inner):
    return f'<svg xmlns="http://www.w3.org/2000/svg" {attrs} viewBox="0 0 24 24" aria-hidden="true">{inner}</svg>'
STROKE = 'fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"'
def icon_file(path, needle):
    """Extrait le SVG d'une icône depuis l'atome CSS pur (une seule source)."""
    src = open(os.path.join(EXT_DIR, path)).read()
    for m in re.finditer(r'<svg .*?</svg>', src, re.S):
        if needle in m.group(0): return m.group(0)
    raise KeyError((path, needle))

# ---------------------------------------------------------------- parties décrites à la main
M = {}
def part(comp, name, props=(), body=(), lets=()):
    M[name] = Part(comp, name, props, body, lets)

def as_child(comp, name, slot, btn_attrs, child_attrs=(), extra_props=()):
    part(comp, name, [('as_child', False), *extra_props], [C(V('as_child'),
        [El('div', [*child_attrs, ('data-slot', slot), ('data-as-child', '')], [Slot()])],
        [El('button', [('type', 'button'), *btn_attrs, ('data-slot', slot)], [Slot()], rest=True)])])

# alert-dialog
as_child('alert-dialog', 'AlertDialogAction', 'alert-dialog-action', [('data-variant', 'primary')])
as_child('alert-dialog', 'AlertDialogCancel', 'alert-dialog-cancel', [('data-variant', 'outline')])
as_child('alert-dialog', 'AlertDialogTrigger', 'alert-dialog-trigger',
         [('aria-haspopup', 'dialog'), ('data-dialog-for', V('for'))], [('data-dialog-for', V('for'))], [('for', None)])
part('alert-dialog', 'AlertDialogContent', [('animation_duration', 200)], [
    El('div', [('data-state', 'closed'), ('data-slot', 'alert-dialog-backdrop'), ('aria-hidden', 'true')]),
    El('dialog', [('data-state', 'closed'), ('role', 'alertdialog'), ('style', F('--ds-duration: {animation_duration}ms')),
                  ('data-slot', 'alert-dialog-content')], [Slot()], rest=True)])

# aspect-ratio
part('aspect-ratio', 'AspectRatio', [('ratio', 1), ('as', 'div')], [
    El('div', [('style', F('--ds-ratio: {ratio}; padding-bottom: {pad}%')), ('data-slot', 'aspect-ratio-wrapper')], [
        El(V('as'), [('data-slot', 'aspect-ratio')], [Slot()], rest=True)])],
    lets=[('pad', 'round(100 / (float) $ratio, 4)', 'round(100 / float(ratio), 4)')])

# carousel
part('carousel', 'Carousel', [('orientation', 'horizontal'), ('opts', '{}'), ('auto_init', True), ('aria_label', None)], [
    El('div', [('role', 'region'), ('aria-roledescription', 'carousel'), ('aria-label', OR(V('aria_label'), L('Carousel'))),
               ('data-axis', IF(EQ(V('orientation'), 'horizontal'), 'x', 'y')), ('data-opts', V('opts')),
               ('data-auto-init', IF(V('auto_init'), 'true', 'false')), ('data-slot', 'carousel')], [Slot()], rest=True)])
part('carousel', 'CarouselContent', [], [
    El('div', [('data-slot', 'carousel-content')], [El('div', [('data-slot', 'carousel-container')], [Slot()])], rest=True)])
for nm, label, needle in (('CarouselNext', 'Next slide', 'arrow-right'), ('CarouselPrevious', 'Previous slide', 'arrow-left')):
    part('carousel', nm, [('variant', 'outline'), ('size', 'icon')], [
        El('button', [('type', 'button'), ('data-variant', V('variant')), ('data-size', V('size')), ('aria-label', label),
                      ('data-slot', 'carousel-' + nm[8:].lower())], [
            T(icon_file(f'carousel/{nm}.astro', needle)),
            Slot([El('span', [('class', 'sr-only')], [T(label)])])], rest=True)])

# collapsible
part('collapsible', 'Collapsible', [('default_open', False), ('disabled', False)], [
    El('div', [('data-state', IF(V('default_open'), 'open', 'closed')), ('data-disabled', IF(V('disabled'), '', None)),
               ('data-slot', 'collapsible')], [Slot()], rest=True)])
as_child('collapsible', 'CollapsibleTrigger', 'collapsible-trigger', [('aria-expanded', 'false')], [('aria-expanded', 'false')])

# container
CONTAINER_OUTER = [('data-background', V('background')), ('data-padding-y', V('padding_y')), ('data-min-height', V('min_height')),
                   ('data-border', V('border')), ('data-rounded', V('rounded'))]
part('container', 'Container',
     [('as', 'div'), ('bleed', False), ('size', 'lg'), ('gutter', 'md'), ('padding_y', 'none'), ('background', 'none'),
      ('border', 'none'), ('rounded', 'none'), ('min_height', 'none'), ('layout', None), ('centered', False)],
     [C(V('use_wrapper'),
        [El(V('as'), [*CONTAINER_OUTER, ('data-bleed', ''), ('data-slot', 'container')], [
            El('div', [('data-container-inner', ''), ('data-size', V('size')), ('data-gutter', V('gutter')), ('data-layout', V('layout_v'))], [
                El('div', [('data-container-main', '')], [Slot()]), El('div', [('data-container-media', '')])])], rest=True)],
        [El(V('as'), [('data-size', V('size')), ('data-gutter', V('gutter')), *CONTAINER_OUTER, ('data-layout', V('layout_v')),
                      ('data-slot', 'container')], [Slot()], rest=True)])],
     lets=[('layout_v', "$centered ? 'centered' : ($layout ?? 'stack')", '"centered" if centered else (layout or "stack")'),
           ('use_wrapper', "$bleed || $layoutV === 'split'", 'bool(bleed) or layout_v == "split"')])

# dropdown
part('dropdown', 'Dropdown', [('open_on_hover', False), ('close_delay', 200)], [
    El('div', [('data-open-on-hover', IF(V('open_on_hover'), 'true', None)), ('data-close-delay', V('close_delay')),
               ('data-slot', 'dropdown')], [Slot()], rest=True)])
part('dropdown', 'DropdownContent', [('side', 'bottom'), ('align', 'start'), ('side_offset', 4), ('animation_duration', 150)], [
    El('div', [('role', 'menu'), ('data-side', V('side')), ('data-align', V('align')), ('data-state', 'closed'), ('tabindex', '-1'),
               ('aria-orientation', 'vertical'), ('hidden', ''),
               ('style', F('--ds-offset: {side_offset}px; --ds-duration: {animation_duration}ms')),
               ('data-slot', 'dropdown-content')], [Slot()], rest=True)])
part('dropdown', 'DropdownItem', [('inset', False), ('disabled', False), ('as', 'div')], [
    El(V('as'), [('role', 'menuitem'), ('tabindex', IF(V('disabled'), '-1', '0')), ('aria-disabled', IF(V('disabled'), 'true', None)),
                 ('data-disabled', IF(V('disabled'), 'true', None)), ('data-inset', IF(V('inset'), '', None)),
                 ('data-slot', 'dropdown-item')], [Slot()], rest=True)])
part('dropdown', 'DropdownLabel', [('inset', False)], [
    El('div', [('data-inset', IF(V('inset'), '', None)), ('data-slot', 'dropdown-label')], [Slot()], rest=True)])
part('dropdown', 'DropdownSubContent', [('animation_duration', 150)], [
    El('div', [('data-sub-content', ''), ('role', 'menu'), ('data-side', 'right'), ('data-align', 'start'), ('data-state', 'closed'),
               ('tabindex', '-1'), ('aria-orientation', 'vertical'), ('hidden', ''),
               ('style', F('--ds-duration: {animation_duration}ms')), ('data-slot', 'dropdown-content')], [Slot()], rest=True)])
part('dropdown', 'DropdownSubTrigger', [('inset', False), ('disabled', False)], [
    El('div', [('role', 'menuitem'), ('aria-haspopup', 'true'), ('aria-expanded', 'false'), ('tabindex', IF(V('disabled'), '-1', '0')),
               ('data-disabled', IF(V('disabled'), 'true', None)), ('data-inset', IF(V('inset'), '', None)), ('data-sub-trigger', ''),
               ('data-slot', 'dropdown-item')], [Slot(), T(icon_file('dropdown/DropdownSubTrigger.astro', 'chevron-right'))], rest=True)])
part('dropdown', 'DropdownTrigger', [('as_child', False)], [C(V('as_child'),
    [El('div', [('data-slot', 'dropdown-trigger'), ('data-as-child', ''), ('aria-haspopup', 'true'), ('aria-expanded', 'false')], [Slot()])],
    [El('button', [('type', 'button'), ('aria-haspopup', 'true'), ('aria-expanded', 'false'), ('data-state', 'closed'),
                   ('data-slot', 'dropdown-trigger')], [Slot()], rest=True)])])

# dropzone
UPLOAD = El('div', [('data-slot', 'dropzone-upload-indicator')], [
    T(icon_file('dropzone/DropzoneUploadIndicator.astro', 'cloud-upload')), T('<p>Click to upload or drag and drop</p>')])
LOADING = El('div', [('hidden', ''), ('data-slot', 'dropzone-loading-indicator')], [
    T(icon_file('dropzone/DropzoneLoadingIndicator.astro', 'loader')), T('<p>Uploading file(s)...</p>')])
FILES = El('div', [('aria-live', 'polite'), ('aria-label', 'Uploaded files'), ('data-slot', 'dropzone-files-list')])
part('dropzone', 'Dropzone', [('is_uploading', False), ('id', None)], [
    El('label', [('id', V('id')), ('tabindex', '0'), ('data-slot', 'dropzone'), ('data-drag-active', 'false'), ('data-has-files', 'false'),
                 ('data-is-uploading', IF(V('is_uploading'), 'true', 'false'))], [
        Slot([UPLOAD, LOADING, FILES]),
        El('input', [('tabindex', '-1'), ('type', 'file'), ('class', 'sr-only')], rest=True, void=True)])])
part('dropzone', 'DropzoneUploadIndicator', [], [El('div', [('data-slot', 'dropzone-upload-indicator')], [Slot(UPLOAD.children)])])
part('dropzone', 'DropzoneLoadingIndicator', [], [El('div', [('hidden', ''), ('data-slot', 'dropzone-loading-indicator')], [Slot(LOADING.children)])])

# image (atome Astro : <Image> d'astro:assets ; côté serveur : <img> avec chargement différé)
part('image', 'Image', [('src', None), ('alt', '')], [C(V('src'), [
    El('img', [('src', V('src')), ('alt', V('alt')), ('loading', 'lazy'), ('decoding', 'async'), ('data-slot', 'image')], rest=True, void=True)])])

# input-group
part('input-group', 'InputGroupButton', [('type', 'button'), ('variant', 'ghost'), ('size', 'sm')], [
    El('button', [('type', V('type')), ('data-size', V('size')), ('data-input-group-button', 'true'), ('data-slot', 'button'),
                  ('data-variant', V('variant'))], [Slot()], rest=True)])
part('input-group', 'InputGroupInput', [('size', 'md')], [
    El('input', [('data-slot', 'input-group-control'), ('data-size', V('size'))], rest=True, void=True)])
part('input-group', 'InputGroupTextarea', [('size', 'md'), ('default_value', '')], [
    El('textarea', [('data-slot', 'input-group-control'), ('data-size', V('size'))], [Out(V('default_value'))], rest=True)])

# input-otp
part('input-otp', 'InputOtp', [('max_length', 6), ('value', None), ('default_value', None), ('disabled', False), ('pattern', None),
                               ('name', None), ('id', None), ('required', False)], [
    El('div', [('data-max-length', V('max_length')), ('data-value', V('initial')), ('data-disabled', IF(V('disabled'), 'true', 'false')),
               ('data-pattern', V('pattern_str')), ('role', 'group'), ('aria-label', F('OTP input, {max_length} digits')),
               ('data-slot', 'input-otp')], [
        El('input', [('autocomplete', 'one-time-code'), ('inputmode', V('input_mode')), ('name', V('name')), ('id', V('id')),
                     ('class', 'sr-only'), ('required', IF(V('required'), '', None)), ('maxlength', V('max_length')),
                     ('value', V('initial')), ('disabled', IF(V('disabled'), '', None)), ('aria-label', F('OTP, {max_length}')),
                     ('data-otp-hidden-input', '')], void=True),
        Slot()], rest=True)],
    lets=[('pattern_str', "$pattern ?? '^\\\\d+$'", 'pattern or "^\\\\d+$"'),
          ('input_mode', "(str_contains($patternStr, '\\\\d') || str_contains($patternStr, '0-9')) && ! str_contains($patternStr, 'a-z') && ! str_contains($patternStr, 'A-Z') && ! str_contains($patternStr, '\\\\w') ? 'numeric' : 'text'",
           '"numeric" if ("\\\\d" in pattern_str or "0-9" in pattern_str) and not any(s in pattern_str for s in ("a-z", "A-Z", "\\\\w")) else "text"'),
          ('initial', "$value ?: ($defaultValue ?: '')", 'value or default_value or ""')])
part('input-otp', 'InputOtpSeparator', [], [
    El('div', [('aria-hidden', 'true'), ('role', 'separator'), ('data-slot', 'input-otp-separator')],
       [T(icon_file('input-otp/InputOtpSeparator.astro', 'minus'))], rest=True)])
part('input-otp', 'InputOtpSlot', [('size', 'md'), ('index', None)], [
    El('div', [('data-otp-slot', ''), ('data-index', V('index')), ('data-size', V('size')), ('aria-hidden', 'true'),
               ('data-slot', 'input-otp-slot')], [
        T('<span data-otp-char></span><div data-otp-caret hidden><div data-otp-caret-line></div></div>')], rest=True)])

# popover
part('popover', 'Popover', [('open_on_hover', False), ('close_delay', 200), ('default_open', False)], [
    El('div', [('data-open-on-hover', IF(V('open_on_hover'), 'true', None)), ('data-close-delay', V('close_delay')),
               ('data-state', IF(V('default_open'), 'open', 'closed')), ('data-default-open', IF(V('default_open'), 'true', None)),
               ('data-slot', 'popover')], [Slot()], rest=True)])
part('popover', 'PopoverContent', [('side', 'bottom'), ('align', 'center'), ('side_offset', 4), ('animation_duration', 150)], [
    El('div', [('data-side', V('side')), ('data-align', V('align')), ('data-side-offset', V('side_offset')), ('data-state', 'closed'),
               ('role', 'dialog'), ('tabindex', '-1'), ('hidden', ''),
               ('style', F('--ds-offset: {side_offset}px; --ds-duration: {animation_duration}ms')),
               ('data-slot', 'popover-content')], [Slot()], rest=True)])
part('popover', 'PopoverTrigger', [('as_child', False)], [C(V('as_child'),
    [El('div', [('data-slot', 'popover-trigger'), ('data-as-child', '')], [Slot()])],
    [El('button', [('type', 'button'), ('aria-haspopup', 'dialog'), ('aria-expanded', 'false'), ('data-state', 'closed'),
                   ('data-slot', 'popover-trigger')], [Slot()], rest=True)])])

# radio-group
part('radio-group', 'RadioGroup', [('value', None), ('default_value', None), ('legend', None), ('disabled', False),
                                   ('orientation', 'vertical')], [
    El('fieldset', [('role', 'radiogroup'), ('data-value', OR(V('value'), V('default_value'))), ('data-orientation', V('orientation')),
                    ('disabled', IF(V('disabled'), '', None)), ('data-slot', 'radio-group')], [
        C(V('legend'), [El('legend', [('class', 'sr-only')], [Out(V('legend'))])]), Slot()], rest=True)])
part('radio-group', 'RadioGroupItem', [('value', None), ('name', None), ('disabled', False), ('checked', False), ('id', None),
                                       ('size', 'md'), ('variant', 'default')], [
    El('div', [('data-size', V('size')), ('data-slot', 'radio-group-item-wrapper')], [
        El('input', [('type', 'radio'), ('name', V('name')), ('id', V('id')), ('value', V('value')), ('checked', IF(V('checked'), '', None)),
                     ('disabled', IF(V('disabled'), '', None)), ('data-slot', 'radio-group-item')], rest=True, void=True),
        El('span', [('data-variant', V('variant')), ('aria-hidden', 'true'), ('data-slot', 'radio-group-item-control')],
           [T(icon_file('radio-group/RadioGroupItem.astro', 'circle'))])])])

# select
part('select', 'Select', [('name', None), ('default_value', None), ('required', False)], [
    El('div', [('data-name', V('name')), ('data-value', V('default_value')), ('data-required', IF(V('required'), 'true', None)),
               ('data-slot', 'select')], [
        Slot(),
        C(V('name'), [El('input', [('type', 'hidden'), ('name', V('name')), ('value', OR(V('default_value'), L(''))),
                                    ('required', IF(V('required'), '', None)), ('data-select-input', '')], void=True)])], rest=True)])
part('select', 'SelectContent', [('side', 'bottom'), ('align', 'start'), ('size', 'md'), ('side_offset', 4), ('animation_duration', 150)], [
    El('div', [('role', 'listbox'), ('data-side', V('side')), ('data-align', V('align')), ('data-size', V('size')), ('data-state', 'closed'),
               ('tabindex', '-1'), ('hidden', ''), ('style', F('--ds-offset: {side_offset}px; --ds-duration: {animation_duration}ms')),
               ('data-slot', 'select-content')], [El('div', [('data-select-inner', '')], [Slot()])], rest=True)])
part('select', 'SelectItem', [('value', None), ('disabled', False)], [
    El('div', [('data-value', V('value')), ('data-disabled', IF(V('disabled'), 'true', None)), ('aria-disabled', IF(V('disabled'), 'true', None)),
               ('aria-selected', 'false'), ('role', 'option'), ('tabindex', '-1'), ('data-slot', 'select-item')], [
        El('span', [], [Slot()]),
        El('span', [('data-select-check', '')], [T(icon_file('select/SelectItem.astro', 'check'))])], rest=True)])
part('select', 'SelectSearch', [('placeholder', 'Search...'), ('empty_text', 'No results found.')], [
    El('div', [('data-slot', 'select-search-wrapper')], [
        T(icon_file('select/SelectSearch.astro', 'search')),
        El('input', [('type', 'text'), ('placeholder', V('placeholder')), ('autocomplete', 'off'), ('aria-label', V('placeholder')),
                     ('data-slot', 'select-search')], rest=True, void=True)]),
    El('div', [('hidden', ''), ('data-slot', 'select-empty')], [Out(V('empty_text'))])])
part('select', 'SelectTrigger', [('size', 'md')], [
    El('button', [('type', 'button'), ('role', 'combobox'), ('aria-haspopup', 'listbox'), ('aria-label', 'Select field'),
                  ('aria-expanded', 'false'), ('data-state', 'closed'), ('data-size', V('size')), ('data-slot', 'select-trigger')],
       [Slot(), T(icon_file('select/SelectTrigger.astro', 'chevron-down'))], rest=True)])
part('select', 'SelectValue', [('placeholder', 'select')], [
    El('span', [('data-placeholder', V('placeholder')), ('data-slot', 'select-value')], [Out(V('placeholder'))], rest=True)])

# sheet (même moteur que Dialog : <dialog> natif + ds.js)
CLOSE_X = icon_file('sheet/SheetContent.astro', 'M18 6 6 18')
def sheet_content(extra_attrs=(), rest=True, children=None):
    return [El('div', [('data-state', 'closed'), ('data-slot', 'dialog-backdrop'), ('aria-hidden', 'true')]),
            El('dialog', [('data-side', V('side')), ('data-state', 'closed'), ('data-close-duration', '300'), *extra_attrs,
                          ('data-slot', 'sheet-content')], [
                *(children or [Slot()]),
                El('button', [('type', 'button'), ('data-slot', 'sheet-close'), ('aria-label', 'Close dialog')], [T(CLOSE_X)])], rest=rest)]
part('sheet', 'Sheet', [], [El('div', [('data-slot', 'sheet')], [Slot()], rest=True)])
part('sheet', 'SheetContent', [('side', 'right')], sheet_content())
part('sheet', 'SheetClose', [('as_child', False)], [C(V('as_child'),
    [El('span', [('data-slot', 'dialog-close'), ('data-as-child', '')], [Slot()], rest=True)],
    [El('button', [('type', 'button'), ('data-slot', 'dialog-close')], [Slot()], rest=True)])])
part('sheet', 'SheetTrigger', [('for', None), ('as_child', False)], [C(V('as_child'),
    [El('span', [('data-slot', 'dialog-trigger'), ('data-as-child', ''), ('data-for', V('for'))], [Slot()], rest=True)],
    [El('button', [('type', 'button'), ('aria-haspopup', 'dialog'), ('data-slot', 'dialog-trigger'), ('data-for', V('for'))], [Slot()], rest=True)])])

# sidebar
part('sidebar', 'Sidebar', [('side', 'left'), ('variant', 'sidebar'), ('collapsible', 'offcanvas')], [C(EQ(V('collapsible'), 'none'),
    [El('div', [('data-collapsible-mode', 'none'), ('data-slot', 'sidebar')], [Slot()], rest=True)],
    [El('div', [('data-state', 'expanded'), ('data-collapsible', ''), ('data-collapsible-mode', V('collapsible')), ('data-variant', V('variant')),
                ('data-side', V('side')), ('data-slot', 'sidebar')], [
        El('div', [('data-slot', 'sidebar-gap')]),
        El('div', [('data-slot', 'sidebar-container')], [El('div', [('data-slot', 'sidebar-inner')], [Slot()])], rest=True)]),
     El('div', [('data-slot', 'sheet'), ('data-sidebar-mobile', 'true')], sheet_content([('data-sidebar', 'mobile')], rest=False, children=[
         El('div', [('class', 'sr-only'), ('data-slot', 'sheet-header')], [
             El('h2', [('data-slot', 'sheet-title')], [T('Sidebar')]),
             El('p', [('data-slot', 'sheet-description')], [T('Mobile navigation sidebar')])]),
         El('div', [('data-sidebar-mobile-inner', '')], [Slot()])]))])])
part('sidebar', 'SidebarGroupLabel', [('as_child', False)], [
    El('div', [('data-sidebar', 'group-label'), ('data-as-child', IF(V('as_child'), '', None)), ('data-slot', 'sidebar-group-label')], [Slot()], rest=True)])
part('sidebar', 'SidebarInput', [('size', 'md')], [
    El('input', [('data-sidebar', 'input'), ('data-slot', 'sidebar-input'), ('data-size', V('size'))], rest=True, void=True)])
part('sidebar', 'SidebarMenuAction', [('show_on_hover', False), ('variant', 'ghost'), ('size', 'icon-sm')], [
    El('button', [('type', 'button'), ('data-variant', V('variant')), ('data-size', V('size')), ('data-show-on-hover', IF(V('show_on_hover'), '', None)),
                  ('data-sidebar', 'menu-action'), ('data-slot', 'sidebar-menu-action')], [Slot()], rest=True)])
MENU_BTN = [('data-slot', 'sidebar-menu-button'), ('data-sidebar', 'menu-button'), ('data-size', OR(V('size'), L('default'))),
            ('data-variant', V('variant')), ('data-active', IF(V('is_active'), 'true', 'false'))]
def menu_button():
    return C(V('as_child'), [El('div', [*MENU_BTN, ('data-as-child', '')], [Slot()])],
             [El(V('tag_v'), [('type', IF(EQ(V('tag_v'), 'button'), 'button', None)), *MENU_BTN, ('aria-current', V('current'))], [Slot()], rest=True)])
part('sidebar', 'SidebarMenuButton', [('as_child', False), ('is_active', False), ('tooltip', None), ('variant', 'default'), ('size', None)], [
    C(V('tooltip'), [El('span', [('data-sidebar-tooltip', 'true'), ('data-slot', 'tooltip')], [
        El('span', [('data-slot', 'tooltip-trigger')], [menu_button()]),
        El('span', [('role', 'tooltip'), ('data-slot', 'tooltip-content'), ('data-side', 'right')], [Out(V('tooltip'))])])],
      [menu_button()])],
    lets=[('tag_v', "$attributes->has('href') ? 'a' : 'button'", '"a" if "href" in attrs else "button"'),
          ('current', "$isActive && $attributes->has('href') ? 'page' : null", '"page" if is_active and "href" in attrs else None')])
part('sidebar', 'SidebarMenuSkeleton', [('show_icon', False), ('width', None)], [
    El('div', [('data-sidebar', 'menu-skeleton'), ('data-slot', 'sidebar-menu-skeleton')], [
        C(V('show_icon'), [El('div', [('data-sidebar', 'menu-skeleton-icon'), ('data-slot', 'skeleton')])]),
        El('div', [('data-sidebar', 'menu-skeleton-text'), ('style', F('--skeleton-width: {skeleton_width}')), ('data-slot', 'skeleton')])], rest=True)],
    lets=[('skeleton_width', "$width ?? (random_int(50, 89).'%')", 'width or f"{random.randint(50, 89)}%"')])
part('sidebar', 'SidebarMenuSubButton', [('size', None), ('is_active', False)], [
    El('a', [('data-sidebar', 'menu-sub-button'), ('data-size', OR(V('size'), L('md'))), ('data-active', IF(V('is_active'), 'true', 'false')),
             ('aria-current', IF(V('is_active'), 'page', None)), ('data-slot', 'sidebar-menu-sub-button')], [Slot()], rest=True)])
part('sidebar', 'SidebarProvider', [('default_open', True), ('keyboard_shortcut', 'b')], [
    El('div', [('data-state', IF(V('default_open'), 'expanded', 'collapsed')), ('data-mobile-open', 'false'),
               ('data-keyboard-shortcut', V('keyboard_shortcut')), ('style', '--sidebar-width: 18rem; --sidebar-width-icon: 3.5rem;'),
               ('data-slot', 'sidebar-provider')], [Slot()], rest=True)])
part('sidebar', 'SidebarTrigger', [('size', 'icon-sm'), ('variant', 'ghost')], [
    El('button', [('type', 'button'), ('data-variant', V('variant')), ('data-size', V('size')), ('data-sidebar', 'trigger'),
                  ('aria-label', 'Toggle Sidebar'), ('aria-expanded', 'true'), ('data-slot', 'sidebar-trigger')], [
        T(icon_file('sidebar/SidebarTrigger.astro', 'layout-sidebar')), T('<span class="sr-only">Toggle Sidebar</span>')], rest=True)])

# slider (même calcul des positions que l'atome)
part('slider', 'Slider', [('default_value', 0), ('value', None), ('min', 0), ('max', 100), ('step', 1), ('large_step', 10),
                          ('orientation', 'horizontal'), ('disabled', False), ('name', None), ('variant', 'default'),
                          ('aria_label', None), ('aria_labelledby', None)], [
    El('div', [('data-orientation', V('orientation')), ('data-disabled', IF(V('disabled'), '', None)), ('data-variant', V('variant')),
               ('role', 'group'), ('aria-labelledby', V('aria_labelledby')), ('data-min', V('min')), ('data-max', V('max')),
               ('data-step', V('step')), ('data-large-step', V('large_step')), ('data-slot', 'slider')], [
        El('div', [('data-slot', 'slider-control'), ('data-orientation', V('orientation'))], [
            El('div', [('data-slot', 'slider-track'), ('data-orientation', V('orientation'))], [
                El('div', [('data-slot', 'slider-range'), ('data-orientation', V('orientation')), ('style', V('range_style'))])]),
            For('t', V('thumbs'), [
                El('div', [('data-slot', 'slider-thumb'), ('data-index', V('t.i')), ('data-orientation', V('orientation')),
                           ('style', V('t.style')), ('tabindex', IF(V('disabled'), '-1', '0')), ('role', 'slider'),
                           ('aria-label', V('aria_label')), ('aria-labelledby', V('aria_labelledby')), ('aria-valuemin', V('min')),
                           ('aria-valuemax', V('max')), ('aria-valuenow', V('t.v')), ('aria-orientation', V('orientation')),
                           ('aria-disabled', IF(V('disabled'), 'true', 'false'))], [
                    El('input', [('type', 'range'), ('min', V('min')), ('max', V('max')), ('step', V('step')), ('value', V('t.v')),
                                 ('name', V('t.name')), ('disabled', IF(V('disabled'), '', None)), ('tabindex', '-1'),
                                 ('aria-hidden', 'true'), ('class', 'sr-only')], void=True)])])])], rest=True)],
    lets=[('thumbs', "\\App\\View\\DesignKit::sliderThumbs($value ?? $defaultValue, $min, $max, $orientation, $name)",
           '_slider_thumbs(value if value is not None else default_value, min, max, orientation, name)'),
          ('range_style', "\\App\\View\\DesignKit::sliderRange($value ?? $defaultValue, $min, $max, $orientation)",
           '_slider_range(value if value is not None else default_value, min, max, orientation)')])

# theme-toggle (Toggle + icônes)
part('theme-toggle', 'ThemeToggle', [('variant', 'outline'), ('size', 'md'), ('aria_label', 'Toggle theme')], [
    El('button', [('type', 'button'), ('data-state', 'off'), ('data-variant', V('variant')), ('data-size', V('size')),
                  ('aria-pressed', 'false'), ('aria-label', V('aria_label')), ('data-slot', 'theme-toggle')], [
        Slot([El('span', [('data-theme-icon-wrapper', '')], [
            T(icon_file('theme-toggle/ThemeToggle.astro', 'data-theme-icon="light"')),
            T(icon_file('theme-toggle/ThemeToggle.astro', 'data-theme-icon="dark"'))])])], rest=True)])

# toast
TOAST_ICONS = {v: icon_file('toast/ToastTitle.astro', needle) for v, needle in (
    ('success', 'm9 12 2 2 4-4'), ('error', 'm10 10 4 4m0-4-4 4'), ('warning', 'M12 9v4'), ('info', 'M12 8h.01'), ('loading', 'data-icon="loader"'))}
def title_icon(var):
    if isinstance(var, str):
        return [T(TOAST_ICONS[var])] if var in TOAST_ICONS else []
    node = []
    for v in reversed(list(TOAST_ICONS)):
        node = [C(EQ(var, v), [T(TOAST_ICONS[v])], node)]
    return node
CLOSE_TOAST = El('button', [('type', 'button'), ('data-slot', 'toast-close'), ('aria-label', 'Close notification')],
                 [T(icon_file('toast/ToastItem.astro', 'M18 6 6 18'))])
def toast_item(variant, children, rest):
    urgent = (lambda r: IF(OR(EQ(variant, 'error'), EQ(variant, 'warning')), r[0], r[1])) if not isinstance(variant, str) else \
             (lambda r: L(r[0] if variant in ('error', 'warning') else r[1]))
    return El('div', [('data-variant', variant), ('data-state', 'open'), ('role', urgent(('alert', 'status'))),
                      ('aria-live', urgent(('assertive', 'polite'))), ('data-slot', 'toast')], [
        El('div', [('data-slot', 'toast-content')], children), CLOSE_TOAST], rest=rest)
def toast_title(variant, children, rest):
    return El('div', [('data-variant', variant), ('data-slot', 'toast-title'), ('data-toast-title', '')], [
        *title_icon(variant), El('span', [('data-toast-title-text', '')], children)], rest=rest)
def toast_template(variant):
    item_variant = 'default' if variant == 'loading' else variant
    return El('template', [('data-toast-template', variant)], [toast_item(item_variant, [
        toast_title(variant, [T('Title')], False),
        El('div', [('data-toast-description', ''), ('data-slot', 'toast-description')], [T('Description')])], False)])
part('toast', 'ToastItem', [('variant', 'default')], [toast_item(V('variant'), [Slot()], True)])
part('toast', 'ToastTitle', [('variant', 'default')], [toast_title(V('variant'), [Slot()], True)])
part('toast', 'ToastTemplate', [('variant', 'default')], [El('template', [('data-toast-template', V('variant'))], [Slot()])])
part('toast', 'Toaster', [('position', 'bottom-right'), ('gap', '0.5rem'), ('peek', '1rem'), ('limit', 3), ('duration', 5000)], [
    El('div', [('data-position', V('position')), ('data-limit', V('limit')), ('data-duration', V('duration')), ('role', 'region'),
               ('aria-live', 'polite'), ('aria-atomic', 'false'), ('aria-relevant', 'additions text'), ('aria-label', 'Notifications'),
               ('tabindex', '-1'), ('style', F('--gap: {gap}; --peek: {peek};')), ('data-slot', 'toast-viewport')],
       [Slot([toast_template(v) for v in ('default', 'success', 'error', 'warning', 'info', 'loading')])], rest=True)])

# toggle
part('toggle', 'Toggle', [('default_pressed', False), ('sync_group', None), ('variant', 'default'), ('size', 'md')], [
    El('button', [('type', 'button'), ('data-state', IF(V('default_pressed'), 'on', 'off')), ('data-sync-group', V('sync_group')),
                  ('data-variant', V('variant')), ('data-size', V('size')), ('aria-pressed', IF(V('default_pressed'), 'true', 'false')),
                  ('data-slot', 'toggle')], [Slot()], rest=True)])

# video : YouTube (y compris Shorts) en iframe sans cookies, sinon <video> — même détection que l'atome
part('video', 'Video', [('src', ''), ('title', 'Video'), ('autoplay', False), ('muted', False), ('loop', False), ('controls', True),
                        ('poster', None)], [C(V('embed'),
    [El('iframe', [('src', V('embed')), ('title', V('title')),
                   ('allow', 'accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share'),
                   ('referrerpolicy', 'strict-origin-when-cross-origin'), ('allowfullscreen', ''),
                   ('data-video-type', IF(V('is_short'), 'youtube-shorts', 'youtube')), ('data-slot', 'video')], rest=True)],
    [El('video', [('src', V('src')), ('autoplay', IF(V('autoplay'), '', None)), ('muted', IF(V('muted'), '', None)),
                  ('loop', IF(V('loop'), '', None)), ('controls', IF(V('controls'), '', None)), ('poster', V('poster')),
                  ('data-slot', 'video')], [T('<track kind="captions">')], rest=True)])],
    lets=[('embed', "\\App\\View\\DesignKit::youtubeEmbed($src, $autoplay, $muted, $loop, $controls)",
           '_youtube_embed(src, autoplay, muted, loop, controls)'),
          ('is_short', "str_contains($src, '/shorts/')", '"/shorts/" in src')])

# ---------------------------------------------------------------- assemblage
def ext_blade_path(p):
    rel = 'toaster' if p.name == 'Toaster' else part_rel(p.comp, p.name)
    return f'{OUT_BLADE}/{p.comp}/{rel or p.comp}.blade.php'

# helpers PHP partagés (curseur, vidéo) : classe d'application, comme un composant Blade à classe
DESIGN_KIT_PHP = r'''<?php

namespace App\View;

/**
 * Calculs partagés par les composants du design kit (curseur, vidéo) : mêmes résultats que les atomes Astro.
 * Généré par tools/dsgen ; ne pas modifier à la main.
 */
final class DesignKit
{
    /** @return list<array{i:int, v:float|int, style:string, name:?string}> */
    public static function sliderThumbs(mixed $initial, float|int $min, float|int $max, string $orientation, ?string $name): array
    {
        $values = self::values($initial, $min, $max);
        $range = count($values) > 1;
        $thumbs = [];
        foreach ($values as $i => $v) {
            $pct = self::pct($v, $min, $max);
            $thumbs[] = [
                'i' => $i,
                'v' => $v,
                'style' => $orientation === 'horizontal' ? "left: {$pct}%" : "bottom: {$pct}%",
                'name' => $name === null ? null : ($range ? "{$name}[{$i}]" : $name),
            ];
        }

        return $thumbs;
    }

    public static function sliderRange(mixed $initial, float|int $min, float|int $max, string $orientation): string
    {
        $values = self::values($initial, $min, $max);
        $start = count($values) > 1 ? self::pct(min($values), $min, $max) : 0;
        $end = self::pct(max($values), $min, $max);
        $size = $end - $start;

        return $orientation === 'horizontal' ? "left: {$start}%; width: {$size}%" : "bottom: {$start}%; height: {$size}%";
    }

    public static function youtubeEmbed(string $src, bool $autoplay, bool $muted, bool $loop, bool $controls): ?string
    {
        if (! str_contains($src, 'youtube.com') && ! str_contains($src, 'youtu.be') && ! str_contains($src, 'youtube-nocookie.com')) {
            return null;
        }
        $patterns = ['#youtube\.com/shorts/([^?&]+)#', '#youtube\.com/watch\?v=([^&]+)#', '#youtube\.com/embed/([^?&]+)#', '#youtu\.be/([^?&]+)#', '#youtube-nocookie\.com/embed/([^?&]+)#'];
        foreach ($patterns as $pattern) {
            if (preg_match($pattern, $src, $m)) {
                $params = array_filter([
                    'autoplay' => $autoplay ? '1' : null,
                    'mute' => $muted ? '1' : null,
                    'loop' => $loop ? '1' : null,
                    'playlist' => $loop ? $m[1] : null,
                    'controls' => $controls ? null : '0',
                ]);
                $query = http_build_query($params);

                // alpharust-ignore: no-hardcoded-urls
                return 'https://www.youtube-nocookie.com/embed/'.$m[1].($query ? '?'.$query : '');
            }
        }

        return null;
    }

    /** @return list<float|int> */
    private static function values(mixed $initial, float|int $min, float|int $max): array
    {
        if (is_string($initial)) {
            $initial = array_map('floatval', array_filter(explode(',', $initial), 'strlen'));
        }

        return array_values(array_map(fn ($v) => min($max, max($min, $v + 0)), (array) $initial));
    }

    private static function pct(float|int $v, float|int $min, float|int $max): float|int
    {
        $span = ($max - $min) ?: 1;

        return round((($v - $min) / $span) * 100, 4);
    }
}
'''

DJ_EXT_HEADER = r'''

# ---------------------------------------------------------------------------------------------------------------
# Composants étendus (alert-dialog, select, sidebar, toast…) : même balisage que les atomes CSS pur d'Astro.
# Générés par tools/dsgen/ext_server.py ; ne pas modifier à la main.

import random  # noqa: E402
import re  # noqa: E402
from urllib.parse import urlencode  # noqa: E402

from django.utils.html import conditional_escape  # noqa: E402
from django.utils.safestring import mark_safe  # noqa: E402


def _x(attrs):
    """Arguments nommés -> attributs HTML (``aria_label`` -> ``aria-label``, ``class_`` -> ``class``)."""
    return {key.rstrip("_").replace("_", "-"): value for key, value in attrs.items()}


def _join(*parts):
    return mark_safe("".join(str(conditional_escape(p)) for p in parts if p is not None))


def _num(v):
    f = float(v)
    return int(f) if f.is_integer() else round(f, 4)


def _slider_values(initial, lo, hi):
    if isinstance(initial, str):
        initial = [float(x) for x in initial.split(",") if x.strip()]
    if not isinstance(initial, (list, tuple)):
        initial = [initial]
    return [_num(min(float(hi), max(float(lo), float(v)))) for v in initial]


def _pct(v, lo, hi):
    span = (float(hi) - float(lo)) or 1
    return _num((float(v) - float(lo)) / span * 100)


def _slider_thumbs(initial, lo, hi, orientation, name):
    values = _slider_values(initial, lo, hi)
    out = []
    for i, v in enumerate(values):
        pct = _pct(v, lo, hi)
        out.append({
            "i": i,
            "v": v,
            "style": f"left: {pct}%" if orientation == "horizontal" else f"bottom: {pct}%",
            "name": None if name is None else (f"{name}[{i}]" if len(values) > 1 else name),
        })
    return out


def _slider_range(initial, lo, hi, orientation):
    values = _slider_values(initial, lo, hi)
    start = _pct(min(values), lo, hi) if len(values) > 1 else 0
    size = _num(_pct(max(values), lo, hi) - start)
    return f"left: {start}%; width: {size}%" if orientation == "horizontal" else f"bottom: {start}%; height: {size}%"


_YOUTUBE = [r"youtube\.com/shorts/([^?&]+)", r"youtube\.com/watch\?v=([^&]+)", r"youtube\.com/embed/([^?&]+)",
            r"youtu\.be/([^?&]+)", r"youtube-nocookie\.com/embed/([^?&]+)"]


def _youtube_embed(src, autoplay, muted, loop, controls):
    if not any(host in src for host in ("youtube.com", "youtu.be", "youtube-nocookie.com")):
        return None
    for pattern in _YOUTUBE:
        m = re.search(pattern, src)
        if m:
            params = {"autoplay": "1" if autoplay else None, "mute": "1" if muted else None, "loop": "1" if loop else None,
                      "playlist": m.group(1) if loop else None, "controls": None if controls else "0"}
            query = urlencode({k: v for k, v in params.items() if v})
            # alpharust-ignore: no-hardcoded-urls
            return "https://www.youtube-nocookie.com/embed/" + m.group(1) + ("?" + query if query else "")
    return None

'''


def generate():
    """Parties étendues Blade et Django (assemblées depuis les atomes CSS pur), DesignKit.php, ds.py."""
    ext_parts = []
    for comp in ext_components():
        for path in sorted(glob.glob(f'{EXT_DIR}/{comp}/*.astro')):
            name = os.path.basename(path)[:-6]
            p = M.get(name) or auto_part(comp, name, path)
            assert p is not None, f'partie {comp}/{name} : ni simple ni décrite dans MANUAL'
            assert p.comp == comp, (comp, name)
            ext_parts.append(p)
    unused = set(M) - {p.name for p in ext_parts}
    assert not unused, unused
    for p in ext_parts:
        w(ext_blade_path(p), blade_part(p))

    # helpers PHP partagés (curseur, vidéo) : classe d'application, comme un composant Blade à classe
    w(ROOT + '/ui-blade/app/View/DesignKit.php', DESIGN_KIT_PHP)
    dj_core = open(OUT_DJ).read()
    core_tags = set(re.findall(r'name="(ds_[a-z_]+)"', dj_core))
    ext_tags = [dj_tag_name(p) for p in ext_parts]
    assert not core_tags & set(ext_tags), core_tags & set(ext_tags)
    assert len(ext_tags) == len(set(ext_tags))
    w(OUT_DJ, dj_core + DJ_EXT_HEADER + '\n\n'.join(django_part(p) for p in ext_parts))

    # index lisible : partie -> composant Blade -> balise Django (utilisé par les tests de parité et la doc)
    ext_index = {p.comp: [] for p in ext_parts}
    for p in ext_parts:
        ext_index[p.comp].append({'part': p.name,
                                  'blade': 'x-ds.' + ext_blade_path(p)[len(OUT_BLADE) + 1:-len('.blade.php')].replace('/', '.')
                                           .replace(f'{p.comp}.{p.comp}', p.comp),
                                  'django': dj_tag_name(p)})
    print('blade+django étendus ok', len(ext_parts), 'parties')
    return ext_index
