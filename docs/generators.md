# Générateurs

Les générateurs créent des projets **complets et testés** dans `sites/<nom>/` (ou `--path`).
Les fichiers sont des templates `templates/<type>/**/*.tpl` embarqués dans le binaire ;
les marqueurs `@@name@@`, `@@slug@@`, `@@pkg@@` (identifiant Python/Rust), `@@title@@` sont substitués
dans les contenus **et** les chemins. Les dossiers préfixés `dot-` deviennent des dotfiles
(`dot-gitignore.tpl` → `.gitignore`).

```bash
alpharust init mon-projet                  # Rust : lib + binaire + tests + CI + .alpharust.toml
alpharust generate wagtail mon-cms         # Django/Wagtail
alpharust generate astro mon-site          # Astro
alpharust generate laravel mon-app         # Laravel
alpharust generate astro mon-site --path ./autre --force
```

Le nom du projet est un seul segment de chemin (`[A-Za-z0-9._-]`, sans `..`) ; un dossier existant non vide
est refusé sans `--force`.

| Type | Contenu | Vérifié par |
|------|---------|-------------|
| `init` (Rust) | `Cargo.toml`, `src/lib.rs` + tests unitaires, `src/main.rs`, `tests/smoke.rs`, CI GitHub, LICENSE, README, `docs/` | `cargo test` dans les tests d'intégration |
| `wagtail` | réglages `base/dev/production`, `HomePage` + migrations `0001`/`0002` (page d'accueil et site créés), templates, CSS, tests Django, `.env.example` | `manage.py check`, `makemigrations --check`, `manage.py test` (CI) |
| `astro` | layout accessible, header, pages (accueil, à propos, blog), content collection typée + article, `tsconfig` strict | `npm run build` (CI) |
| `laravel` | squelette 11+ (bootstrap, routes, contrôleur, modèle, migration, seeder, vues Blade, PHPUnit), `.env.example` | `php artisan test` (CI) |

Tous les projets générés incluent `README.md` (Installation / Usage / License), `docs/index.md`,
`.gitignore` et `.alpharust.toml` ; ils passent les profils `web` / `cms` / `strict` d'AlphaRust sans violation
(testé dans `tests/generators.rs`).

Autres générateurs de texte : `readme`, `docs`, `checks` (checklist Markdown d'un profil).

## Ajouter un générateur

1. Déposer les fichiers dans `templates/<type>/` (suffixe `.tpl`).
2. Créer `src/generators/<type>.rs` avec la liste `FILES` (un test vérifie qu'aucun fichier n'est oublié).
3. Brancher `Kind::<Type>` dans `generators::generate`, la CLI, la TUI et la GUI.
4. Ajouter un test dans `tests/generators.rs`.
