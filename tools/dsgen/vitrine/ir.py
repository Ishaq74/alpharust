"""Représentation intermédiaire des gabarits du site et ses trois rendus : Astro, Blade, Django.

Un gabarit est décrit UNE fois (éléments HTML, composants du design kit, boucles, conditions, sorties) puis rendu :
  - Astro  : `<Button href={x.url}>`, `{list.map((x) => (<Fragment>…</Fragment>))}`, `{cond && (…)}` ;
  - Blade  : `<x-ds.button :href="$x['url']">`, `@foreach`, `@if (! empty(…))` ;
  - Django : `{% ds_button href=x.url %}…{% endds_button %}`, `{% for %}`, `{% if %}`.
Les valeurs dynamiques sont des chemins (`post.title`) : les données arrivent déjà normalisées (URL, dates formatées,
`current` du menu) depuis chaque framework, si bien que le balisage rendu est identique partout.
"""
import json, os, re

# ------------------------------------------------------------------ expressions et nœuds
class V:
    """Chemin de données : `post.title`, `site.cta.url`."""
    def __init__(self, path): self.path = path
class Opt:
    """Attribut HTML posé seulement si la valeur est vraie (aria-current, open…)."""
    def __init__(self, v): self.v = v

class E:
    def __init__(self, tag, attrs=None, children=None, void=False):
        self.tag, self.attrs, self.children, self.void = tag, attrs or [], children or [], void
class C:
    """Composant du design kit (nom Astro : Button, CardHeader, DropdownItem…)."""
    def __init__(self, name, props=None, children=None):
        self.name, self.props, self.children = name, props or [], children or []
class T:
    def __init__(self, text): self.text = text
class O:
    def __init__(self, v): self.v = v
class Html:
    """HTML déjà sûr (corps riche) ; `astro` : rendu Astro spécifique (ex. `<Content />` d'une collection)."""
    def __init__(self, v, astro=None, django=None): self.v, self.astro, self.django = v, astro, django
class For:
    def __init__(self, var, v, children): self.var, self.v, self.children = var, v, children
class If:
    def __init__(self, v, then, other=None, negate=False): self.v, self.then, self.other, self.negate = v, then, other or [], negate
class Icon:
    """Icône SVG : nom littéral (SVG en ligne) ou chemin (composant d'icône du site)."""
    def __init__(self, name, size=None): self.name, self.size = name, size
class Csrf: pass
class Raw:
    """Fragment propre à un framework (ex. champ de formulaire) : dict astro/blade/django."""
    def __init__(self, **by): self.by = by

def kebab(s): return re.sub(r'(?<!^)(?=[A-Z])', '-', s).lower()
def snake(s): return kebab(s).replace('-', '_')
def camel_to_kebab(k): return re.sub(r'([a-z0-9])([A-Z])', r'\1-\2', k).lower()

from .icons import ICONS
def svg(name, size=None, cls=None):
    s = size or 20
    c = f' class="{cls}"' if cls else ''
    return (f'<svg width="{s}" height="{s}" viewBox="0 0 24 24" fill="none" stroke="currentColor" '
            f'stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"{c}>{ICONS[name]}</svg>')

# ------------------------------------------------------------------ catalogue des composants (nom -> module, tag Blade, balise Django)
class Catalog:
    def __init__(self, root):
        self.mod, self.blade, self.dj = {}, {}, {}
        atoms = root + '/ui-css/src/components/atoms'
        blade_root = root + '/ui-blade/resources/views/components/ds'
        ds_py = open(root + '/ui-django/ds/templatetags/ds.py').read()
        self.block = set(re.findall(r'simple_block_tag\(name="(\w+)"\)', ds_py))
        self.simple = set(re.findall(r'simple_tag\(name="(\w+)"\)', ds_py))
        for comp in sorted(os.listdir(atoms)):
            pas = ''.join(w.capitalize() for w in comp.split('-'))
            for f in os.listdir(f'{atoms}/{comp}'):
                if not f.endswith('.astro'): continue
                name = f[:-6]
                self.mod[name] = comp
                part = kebab(name[len(pas):]) if name.startswith(pas) and name != pas else ('' if name == pas else kebab(name))
                tag = comp + ('.' + part if part else '')
                path = f'{blade_root}/{comp}/{part or comp}.blade.php'
                if os.path.exists(path): self.blade[name] = tag
                dj = 'ds_' + snake(name)
                if dj in self.block or dj in self.simple: self.dj[name] = dj

    def check(self, name):
        assert name in self.mod and name in self.blade and name in self.dj, f'composant inconnu ou incomplet : {name}'

# ------------------------------------------------------------------ rendu Astro
def js_path(p): return p
def a_attr_lit(v): return v.replace('"', '&quot;')

class Astro:
    def __init__(self, cat):
        self.cat, self.used, self.icons_dyn = cat, set(), False
    def expr(self, v):
        if isinstance(v, V): return js_path(v.path)
        return json.dumps(v, ensure_ascii=False)
    def attrs(self, attrs, comp=False):
        out = []
        for k, v in attrs:
            if v is True: out.append(f' {k}')
            elif isinstance(v, Opt): out.append(f' {k}={{{self.expr(v.v)} || undefined}}')
            elif isinstance(v, V): out.append(f' {k}={{{self.expr(v)}}}')
            elif isinstance(v, (int, float)) and not isinstance(v, bool): out.append(f' {k}={{{v}}}')
            else: out.append(f' {k}="{a_attr_lit(v)}"')
        return ''.join(out)
    def nodes(self, ns, ind=0): return ''.join(self.node(n, ind) for n in ns)
    def node(self, n, ind):
        p = '  ' * ind
        if isinstance(n, str): n = T(n)
        if isinstance(n, T): return p + n.text + '\n'
        if isinstance(n, O): return p + '{' + self.expr(n.v) + '}\n'
        if isinstance(n, Html):
            return p + (n.astro or f'<Fragment set:html={{{self.expr(n.v)}}} />') + '\n'
        if isinstance(n, Icon):
            if isinstance(n.name, V):
                self.icons_dyn = True
                sz = f' size={{{n.size}}}' if n.size else ''
                return p + f'<SiteIcon name={{{self.expr(n.name)}}}{sz} />\n'
            return p + svg(n.name, n.size) + '\n'
        if isinstance(n, Csrf): return ''
        if isinstance(n, Raw): return p + n.by['astro'] + '\n' if n.by.get('astro') else ''
        if isinstance(n, E):
            a = self.attrs(n.attrs)
            if n.void: return f'{p}<{n.tag}{a} />\n'
            if not n.children: return f'{p}<{n.tag}{a}></{n.tag}>\n'
            return f'{p}<{n.tag}{a}>\n{self.nodes(n.children, ind + 1)}{p}</{n.tag}>\n'
        if isinstance(n, C):
            self.cat.check(n.name); self.used.add(n.name)
            a = self.attrs(n.props, comp=True)
            if not n.children: return f'{p}<{n.name}{a} />\n'
            return f'{p}<{n.name}{a}>\n{self.nodes(n.children, ind + 1)}{p}</{n.name}>\n'
        if isinstance(n, For):
            return (f'{p}{{({self.expr(n.v)} ?? []).map(({n.var}: any) => (\n{p}  <Fragment>\n'
                    f'{self.nodes(n.children, ind + 2)}{p}  </Fragment>\n{p}))}}\n')
        if isinstance(n, If):
            c = self.expr(n.v)
            c = f'!{c}' if n.negate else c
            then = f'{p}  <Fragment>\n{self.nodes(n.then, ind + 2)}{p}  </Fragment>\n'
            if n.other:
                other = f'{p}  <Fragment>\n{self.nodes(n.other, ind + 2)}{p}  </Fragment>\n'
                return f'{p}{{{c} ? (\n{then}{p}) : (\n{other}{p})}}\n'
            return f'{p}{{{c} && (\n{then}{p})}}\n'
        raise TypeError(n)
    def imports(self):
        mods = {}
        for name in sorted(self.used): mods.setdefault(self.cat.mod[name], []).append(name)
        lines = [f"import {{ {', '.join(names)} }} from '@atoms/{m}';" for m, names in sorted(mods.items())]
        if self.icons_dyn: lines.append("import SiteIcon from '@/components/site/SiteIcon.astro';")
        return '\n'.join(lines)

# ------------------------------------------------------------------ rendu Blade
def php_path(path):
    head, *rest = path.split('.')
    return '$' + head + ''.join(f"['{r}']" for r in rest)
def php_lit(v):
    if v is True: return 'true'
    if v is False: return 'false'
    if isinstance(v, (int, float)): return str(v)
    return "'" + str(v).replace('\\', '\\\\').replace("'", "\\'") + "'"
def blade_text(s): return s.replace('{{', '&#123;&#123;').replace('@', '&#64;') if '{{' in s or '@' in s else s

class Blade:
    def __init__(self, cat): self.cat = cat
    def val(self, v):
        if isinstance(v, V): return php_path(v.path) + (" ?? ''" if '.' in v.path else '')
        return php_lit(v)
    def cond(self, v, negate=False):
        c = f'! empty({php_path(v.path)})'
        return f'empty({php_path(v.path)})' if negate else c
    def attrs(self, attrs):
        out = []
        for k, v in attrs:
            if v is True: out.append(f' {k}')
            elif isinstance(v, Opt): out.append(f' @if ({self.cond(v.v)}){k}="{{{{ {php_path(v.v.path)} }}}}"@endif')
            elif isinstance(v, V): out.append(f' {k}="{{{{ {self.val(v)} }}}}"')
            else: out.append(f' {k}="{blade_text(str(v))}"')
        return ''.join(out)
    def props(self, props):
        out = []
        for k, v in props:
            kk = camel_to_kebab(k) if not k.startswith(('aria-', 'data-')) else k
            if v is True: out.append(f' :{kk}="true"')
            elif isinstance(v, Opt): out.append(f' :{kk}="{php_path(v.v.path)} ?? null"')
            elif isinstance(v, V): out.append(f' :{kk}="{self.val(v)}"')
            elif isinstance(v, (int, float)) and not isinstance(v, bool): out.append(f' :{kk}="{v}"')
            else: out.append(f' {kk}="{blade_text(str(v))}"')
        return ''.join(out)
    def nodes(self, ns, ind=0): return ''.join(self.node(n, ind) for n in ns)
    def node(self, n, ind):
        p = '    ' * ind
        if isinstance(n, str): n = T(n)
        if isinstance(n, T): return p + blade_text(n.text) + '\n'
        if isinstance(n, O): return p + '{{ ' + self.val(n.v) + ' }}\n'
        if isinstance(n, Html): return p + '{!! ' + self.val(n.v) + ' !!}\n'
        if isinstance(n, Icon):
            if isinstance(n.name, V):
                sz = f' :size="{n.size}"' if n.size else ''
                return p + f'<x-site.icon :name="{self.val(n.name)}"{sz} />\n'
            return p + svg(n.name, n.size) + '\n'
        if isinstance(n, Csrf): return p + '@csrf\n'
        if isinstance(n, Raw): return p + n.by['blade'] + '\n' if n.by.get('blade') else ''
        if isinstance(n, E):
            a = self.attrs(n.attrs)
            if n.void: return f'{p}<{n.tag}{a}>\n'
            if not n.children: return f'{p}<{n.tag}{a}></{n.tag}>\n'
            return f'{p}<{n.tag}{a}>\n{self.nodes(n.children, ind + 1)}{p}</{n.tag}>\n'
        if isinstance(n, C):
            self.cat.check(n.name)
            tag = 'x-ds.' + self.cat.blade[n.name]
            a = self.props(n.props)
            if not n.children: return f'{p}<{tag}{a} />\n'
            return f'{p}<{tag}{a}>\n{self.nodes(n.children, ind + 1)}{p}</{tag}>\n'
        if isinstance(n, For):
            return f"{p}@foreach ({php_path(n.v.path)} ?? [] as ${n.var})\n{self.nodes(n.children, ind + 1)}{p}@endforeach\n"
        if isinstance(n, If):
            s = f'{p}@if ({self.cond(n.v, n.negate)})\n{self.nodes(n.then, ind + 1)}'
            if n.other: s += f'{p}@else\n{self.nodes(n.other, ind + 1)}'
            return s + f'{p}@endif\n'
        raise TypeError(n)

# ------------------------------------------------------------------ rendu Django
def dj_lit(v):
    if v is True: return 'True'
    if v is False: return 'False'
    if isinstance(v, (int, float)): return str(v)
    return '"' + str(v).replace('"', '&quot;') + '"'
def dj_text(s): return s.replace('{%', '{&#37;').replace('{{', '&#123;&#123;')

class Django:
    def __init__(self, cat): self.cat, self.icons_dyn = cat, False
    def attrs(self, attrs):
        out = []
        for k, v in attrs:
            if v is True: out.append(f' {k}')
            elif isinstance(v, Opt): out.append(f'{{% if {v.v.path} %}} {k}="{{{{ {v.v.path} }}}}"{{% endif %}}')
            elif isinstance(v, V): out.append(f' {k}="{{{{ {v.path} }}}}"')
            else: out.append(f' {k}="{dj_text(str(v))}"')
        return ''.join(out)
    def props(self, props):
        out = []
        for k, v in props:
            kk = snake(k) if not k.startswith(('aria-', 'data-')) else k.replace('-', '_')
            if kk in ('class', 'for', 'as', 'type', 'id'): kk += '_' if kk in ('class', 'for', 'as') else ''
            if v is True: out.append(f' {kk}=True')
            elif isinstance(v, Opt): out.append(f' {kk}={v.v.path}')
            elif isinstance(v, V): out.append(f' {kk}={v.path}')
            else: out.append(f' {kk}={dj_lit(v)}')
        return ''.join(out)
    def nodes(self, ns, ind=0): return ''.join(self.node(n, ind) for n in ns)
    def node(self, n, ind):
        p = '    ' * ind
        if isinstance(n, str): n = T(n)
        if isinstance(n, T): return p + dj_text(n.text) + '\n'
        if isinstance(n, O): return p + '{{ ' + n.v.path + ' }}\n' if isinstance(n.v, V) else p + dj_text(str(n.v)) + '\n'
        if isinstance(n, Html): return p + (n.django or '{{ ' + n.v.path + ' }}') + '\n'
        if isinstance(n, Icon):
            if isinstance(n.name, V):
                self.icons_dyn = True
                sz = f' size={n.size}' if n.size else ''
                return p + f'{{% site_icon {n.name.path}{sz} %}}\n'
            return p + svg(n.name, n.size) + '\n'
        if isinstance(n, Csrf): return p + '{% csrf_token %}\n'
        if isinstance(n, Raw): return p + n.by['django'] + '\n' if n.by.get('django') else ''
        if isinstance(n, E):
            a = self.attrs(n.attrs)
            if n.void: return f'{p}<{n.tag}{a}>\n'
            if not n.children: return f'{p}<{n.tag}{a}></{n.tag}>\n'
            return f'{p}<{n.tag}{a}>\n{self.nodes(n.children, ind + 1)}{p}</{n.tag}>\n'
        if isinstance(n, C):
            self.cat.check(n.name)
            tag = self.cat.dj[n.name]
            a = self.props(n.props)
            if tag in self.cat.block:
                if not n.children: return f'{p}{{% {tag}{a} %}}{{% end{tag} %}}\n'
                return f'{p}{{% {tag}{a} %}}\n{self.nodes(n.children, ind + 1)}{p}{{% end{tag} %}}\n'
            assert not n.children, f'{n.name} : balise Django sans contenu'
            return f'{p}{{% {tag}{a} %}}\n'
        if isinstance(n, For):
            return f'{p}{{% for {n.var} in {n.v.path} %}}\n{self.nodes(n.children, ind + 1)}{p}{{% endfor %}}\n'
        if isinstance(n, If):
            c = ('not ' if n.negate else '') + n.v.path
            s = f'{p}{{% if {c} %}}\n{self.nodes(n.then, ind + 1)}'
            if n.other: s += f'{p}{{% else %}}\n{self.nodes(n.other, ind + 1)}'
            return s + f'{p}{{% endif %}}\n'
        raise TypeError(n)
