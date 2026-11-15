# CLI AlphaRust

## Commandes

| Commande | Description |
|----------|-------------|
| `lint [path]` | Lint un chemin (défaut: .) |
| `lint --profile strict` | Lint avec un profil |
| `lint --sarif` | Sortie SARIF |
| `list` | Lister les règles |
| `list --profiles` | Lister les profils |
| `init <name>` | Initialiser un projet dans sites/ |
| `generate wagtail\|astro\|laravel <name>` | Générer un projet framework |
| `readme <name>` | Générer un README |
| `docs <name>` | Générer de la documentation |
| `tui` | Mode TUI interactif |
| `gui` | Mode GUI web (port 3000) |

## Exemples

```bash
alpharust lint --profile strict
alpharust generate astro mon-site
alpharust tui
```
