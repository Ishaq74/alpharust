# Templates

Fichiers sources des générateurs, embarqués dans le binaire par `include_str!` (voir `src/generators/`).

| Dossier | Générateur |
|---------|------------|
| `bootstrap/` | `alpharust init` (Rust) |
| `wagtail/` | `alpharust generate wagtail` |
| `astro/` | `alpharust generate astro` |
| `laravel/` | `alpharust generate laravel` |

Chaque fichier se termine par `.tpl`. Marqueurs : `@@name@@`, `@@slug@@`, `@@pkg@@`, `@@title@@`, `@@upper@@`
(substitués dans le contenu et dans les chemins). Un dossier ou fichier `dot-xxx` devient `.xxx`.
Un test (`src/generators`) échoue si un template n'est pas référencé.
