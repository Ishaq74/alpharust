"""Planche 3×2 des 6 combinaisons pour une page capturée par site_parity.py.

Usage : python montage.py <dossier site_parity> <page> <sortie.png> [hauteur max]
"""
import sys

from PIL import Image, ImageDraw

COMBOS = ['astro-css', 'laravel-css', 'wagtail-css', 'astro-starwind', 'laravel-starwind', 'wagtail-starwind']


def main(src, page, out, max_h=1400):
    shots = [Image.open(f'{src}/{c}-{page}.png').convert('RGB') for c in COMBOS]
    w = max(s.width for s in shots) // 2
    h = min(max(s.height for s in shots) // 2, max_h)
    cell = [s.resize((s.width // 2, s.height // 2)).crop((0, 0, w, h)) for s in shots]
    pad, label = 16, 28
    sheet = Image.new('RGB', (3 * w + 4 * pad, 2 * (h + label) + 3 * pad), 'white')
    draw = ImageDraw.Draw(sheet)
    for i, (name, img) in enumerate(zip(COMBOS, cell)):
        x = pad + (i % 3) * (w + pad)
        y = pad + (i // 3) * (h + label + pad)
        draw.text((x, y + 6), name, fill='black')
        sheet.paste(img, (x, y + label))
    sheet.save(out, optimize=True)


if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2], sys.argv[3], *(int(a) for a in sys.argv[4:]))
