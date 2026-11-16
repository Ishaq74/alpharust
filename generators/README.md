# Generators

Documentation des générateurs intégrés à AlphaRust. Le pipeline vit dans `src/scaffold/`, la couche de modèles dans
`templates/layers/` (embarquée dans le binaire). Voir `docs/generators.md`.

| Générateur | Commande | Outil officiel | Résultat vérifié |
|------------|----------|----------------|------------------|
| astro | `alpharust generate astro <nom>` | `create-astro` + `astro add` | `astro check` + `astro build` |
| laravel | `alpharust generate laravel <nom>` | `composer create-project` + Breeze | `php artisan test` |
| wagtail | `alpharust generate wagtail <nom>` | `wagtail start` dans un venv | `manage.py check` / `test` |
| rust | `alpharust generate rust <nom>` (alias `init`) | `cargo new` | `cargo test` / `fmt` / `clippy` |
| readme | `alpharust readme <nom>` | — | README de départ (stdout) |
| docs | `alpharust docs <nom>` | — | documentation de départ (stdout) |
| checks | `alpharust checks [--profile p]` | — | checklist Markdown des règles actives (stdout) |

`alpharust new` lance l'assistant interactif ; `alpharust doctor` vérifie les outils.
