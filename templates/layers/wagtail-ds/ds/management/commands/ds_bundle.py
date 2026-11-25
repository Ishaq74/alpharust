"""Regroupe le kit CSS en une seule feuille : ds/static/ds/css/ds.bundle.css.

`index.css` reste l'entrée lisible (jetons, thème, base, composants, dans cet ordre) ; ses `@import` sont remplacés
par le contenu des fichiers, si bien que le navigateur charge une feuille au lieu de 55. `components/custom.css`
(vos surcharges) n'est pas regroupé : base.html le charge à part, juste après. Relancez la commande après avoir
modifié un autre fichier du kit ou la palette (themes/theme.css).
"""
import re
from pathlib import Path

from django.core.management.base import BaseCommand

CSS = Path(__file__).resolve().parents[2] / "static" / "ds" / "css"
IMPORT = re.compile(r"""^@import\s+['"]([^'"]+)['"]\s*;[ \t]*$""", re.MULTILINE)
SEPARATE = {"components/custom.css"}


def bundle(path, seen):
    def inline(match):
        target = (path.parent / match.group(1)).resolve()
        rel = target.relative_to(CSS).as_posix()
        if rel in SEPARATE or target in seen:
            return ""
        seen.add(target)
        return f"/* {rel} */\n" + bundle(target, seen)

    return IMPORT.sub(inline, path.read_text(encoding="utf-8"))


class Command(BaseCommand):
    help = "Regroupe ds/static/ds/css/index.css et ses @import dans ds.bundle.css (une seule requête)."

    def handle(self, *args, **options):
        out = CSS / "ds.bundle.css"
        body = bundle(CSS / "index.css", set())
        out.write_text("/* Généré par `manage.py ds_bundle` depuis index.css : ne pas modifier. */\n" + body, encoding="utf-8")
        self.stdout.write(f"ds/static/ds/css/ds.bundle.css écrit ({len(body.encode()) // 1024} Ko, sans @import)")
