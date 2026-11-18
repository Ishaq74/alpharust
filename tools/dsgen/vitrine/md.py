"""Markdown minimal (titres ##, paragraphes, listes, **gras**, `code`) -> HTML, aligné sur le rendu d'Astro :
identifiants de titres façon github-slugger et apostrophes typographiques (smartypants) déjà dans la source."""
import html, re

def typo(md):
    """Apostrophes droites -> typographiques hors `code` (la source Markdown elle-même, pour tous les frameworks)."""
    parts = re.split(r'(`[^`]*`)', md)
    return ''.join(p if p.startswith('`') else p.replace("'", '’') for p in parts)

def slug(text):
    t = re.sub(r'<[^>]+>', '', text).lower()
    t = re.sub(r'[^\w\- ]', '', t)
    return t.replace(' ', '-')

def inline(t):
    out = []
    for part in re.split(r'(`[^`]*`)', t):
        if part.startswith('`'):
            out.append('<code>' + html.escape(part[1:-1], quote=False) + '</code>')
        else:
            e = html.escape(part, quote=False)
            out.append(re.sub(r'\*\*(.+?)\*\*', r'<strong>\1</strong>', e))
    return ''.join(out)

def to_html(md):
    blocks, out = re.split(r'\n\s*\n', md.strip()), []
    for b in blocks:
        lines = [l.strip() for l in b.strip().split('\n')]
        if lines[0].startswith('## '):
            text = inline(lines[0][3:])
            out.append(f'<h2 id="{slug(text)}">{text}</h2>')
        elif all(l.startswith('- ') for l in lines):
            out.append('<ul>\n' + '\n'.join(f'<li>{inline(l[2:])}</li>' for l in lines) + '\n</ul>')
        else:
            out.append('<p>' + inline(' '.join(lines)) + '</p>')
    return '\n'.join(out)
