"""Rafraîchit iconsets.json depuis les paquets npm @iconify-json/<jeu> (réseau requis, pas la génération).

Usage : python3 tools/dsgen/vitrine/fetch_icons.py
"""
import json, os, subprocess, sys, tarfile, tempfile

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from vitrine import iconsets as S  # noqa: E402


def package(name, tmp):
    out = subprocess.run(['npm', 'pack', f'@iconify-json/{name}', '--silent'], cwd=tmp, check=True,
                         capture_output=True, text=True).stdout.strip().splitlines()[-1]
    with tarfile.open(os.path.join(tmp, out)) as t:
        read = lambda f: json.load(t.extractfile(f'package/{f}'))  # noqa: E731
        return read('icons.json'), read('info.json'), read('package.json')['version']


def subset(icons, names):
    w, h = icons.get('width', 16), icons.get('height', 16)
    out = {}
    for n in sorted(set(names)):
        src = icons['icons'].get(n) or icons['icons'].get(icons.get('aliases', {}).get(n, {}).get('parent', ''))
        if src:
            out[n] = {'body': src['body'], 'width': src.get('width', w), 'height': src.get('height', h)}
    return out


def main():
    sets = {}
    with tempfile.TemporaryDirectory() as tmp:
        for name in S.BUNDLED + [S.FLAG_SET]:
            icons, info, version = package(name, tmp)
            names = list(S.FLAGS.values()) if name == S.FLAG_SET else [m[name] for m in S.MAP.values() if name in m]
            sets[name] = {'name': info['name'], 'license': info['license']['spdx'], 'version': version,
                          'icons': subset(icons, names)}
    with open(S.DATA, 'w') as f:
        json.dump({'_': 'Généré par fetch_icons.py (sous-ensemble des paquets @iconify-json) ; lu par gen_ds.py et par alpharust --icons',
                   'default': S.DEFAULT_SET, 'flag_set': S.FLAG_SET, 'flags': S.FLAGS, 'map': S.MAP,
                   'candidates': S.CANDIDATES, 'sets': sets}, f,
                  ensure_ascii=False, indent=1, sort_keys=True)
        f.write('\n')
    for name, s in sets.items():
        print(f"{name} {s['version']} ({s['license']}) : {len(s['icons'])} icônes")


if __name__ == '__main__':
    main()
