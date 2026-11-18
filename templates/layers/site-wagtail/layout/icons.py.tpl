# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
"""Icônes du site : layout/icons.json (MDI par défaut, drapeaux Circle Flags), le même fichier qu’Astro et Laravel."""
import json
from pathlib import Path

ICONS = json.loads((Path(__file__).parent / "icons.json").read_text(encoding="utf-8"))["icons"]
