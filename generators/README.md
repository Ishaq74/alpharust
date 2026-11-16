# Generators

Documentation des générateurs intégrés à AlphaRust. Le code vit dans `src/generators/`, les fichiers de
projet dans `templates/` (embarqués dans le binaire). Voir aussi `docs/generators.md`.

| Générateur | Commande | Résultat |
|------------|----------|----------|
| bootstrap | `alpharust init <nom>` | projet Rust complet (lib, binaire, tests, CI) |
| wagtail | `alpharust generate wagtail <nom>` | projet Django/Wagtail (migrations, tests) |
| astro | `alpharust generate astro <nom>` | site Astro (content collection, pages, layout accessible) |
| laravel | `alpharust generate laravel <nom>` | application Laravel (migration, modèle, PHPUnit) |
| readme | `alpharust readme <nom>` | README de départ (stdout) |
| docs | `alpharust docs <nom>` | documentation de départ (stdout) |
| checks | `alpharust checks [--profile p]` | checklist Markdown des règles actives (stdout) |
