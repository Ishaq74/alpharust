# Générateurs

AlphaRust ne **recopie plus de fichiers écrits à la main** : chaque projet est créé par l'**outil officiel**
du framework, ses dépendances sont **réellement installées**, une **couche de modèles** propre est posée par-dessus,
puis le résultat est **vérifié** (types, build, tests, lint). Le tout est le même pipeline pour la CLI, la TUI et la GUI.

```text
preflight ─► plan ─► exécution (événements en direct) ─► couche de modèles ─► vérification ─► rapport
 outils?     commandes   create-astro, composer,           templates/layers     check, build,    fichiers,
 versions    exactes     wagtail start, cargo new          (create/replace)     test, lint       durées, suite
```

| Framework | Outil officiel | Installation | Vérification réelle |
|-----------|----------------|--------------|---------------------|
| `astro` | `<pm> create astro@latest` (+ `astro add <intégrations> --yes`) | npm · pnpm · yarn · bun | `astro check` (types), `astro build`, lint AlphaRust |
| `laravel` | `composer create-project laravel/laravel` (+ `laravel/breeze` : blade, livewire, react, vue, api) | composer (+ npm pour un kit front) | `php artisan test` (PHPUnit ou Pest), migrations SQLite, lint |
| `wagtail` | `python -m venv .venv` + `pip install wagtail` + `wagtail start` | pip dans le venv, `requirements-lock.txt` | `manage.py check`, `makemigrations --check`, `manage.py test`, lint |
| `rust` | `cargo new` (+ `cargo add`) | cargo | `cargo test`, `cargo fmt --check`, `cargo clippy -D warnings`, lint |

## Utilisation

```bash
alpharust new                                  # assistant interactif (questions, aperçu des commandes, confirmation)
alpharust generate astro vitrine -i tailwind,mdx,sitemap,react --site https://exemple.com --pm pnpm --ts strictest
alpharust generate astro blog --template blog --site https://blog.exemple.com
alpharust generate laravel boutique --starter breeze-livewire --pest --dark
alpharust generate wagtail cms --package django-debug-toolbar
alpharust generate rust outil --kind bin --dep clap,anyhow --edition 2024
alpharust generate astro demo --dry-run        # affiche les commandes exactes, n'écrit rien
alpharust generate astro demo --json           # événements JSON (un par ligne) pour d'autres outils
alpharust doctor                               # outils détectés, versions minimales, conseils d'installation
```

Options communes : `--path`, `--force`, `--no-install`, `--no-verify`, `--no-git`, `--dry-run`, `--json`, `-v/--verbose`.
Les options d'un autre framework sont refusées (`generate rust x --pm pnpm` → erreur claire).

### Options par framework

| Astro | Valeurs |
|-------|---------|
| `--pm` | `npm` (défaut), `pnpm`, `yarn`, `bun` |
| `--template` | `alpharust` (défaut : `minimal` officiel + structure complète), `minimal`, `basics`, `blog` (officiels) |
| `--ts` | `base`, `strict` (défaut), `strictest` — preset officiel `astro/tsconfigs/*` |
| `-i/--integration` | `tailwind`, `mdx`, `sitemap`, `react`, `vue`, `svelte` (via `astro add`) |
| `--site` | URL publique (exigée par `sitemap`, écrite dans `astro.config.mjs`) |
| `--no-typecheck` | n'installe pas `@astrojs/check` |
| `--preset` | `vitrine`, `blog-i18n`, `app-auth`, `saas-pg`, `concordia` (voir `alpharust presets`) |
| `--ui` | `none`, `starwind` (46 composants atomic), `concordia` (+ `--css-variant` ; les 46 composants d'atomic en CSS pur, même API que Starwind) |
| `--theme-primary/-secondary/-accent` | palette de marque WCAG → `src/styles/themes/theme.css`, **tous les kits** |
| `--db`, `--db-url` | `none`, `sqlite`, `postgres` (+ URL de développement : migrations appliquées) |
| `--auth`, `--auth-plugin` | Better Auth ; plugins `admin`, `username`, `organization` (site vitrine : pages de compte du site) |
| `--icons` | site vitrine : jeu d'icônes Iconify (MDI par défaut ; drapeaux Circle Flags) |
| `--i18n` | `fr,en,es,ar` (au moins deux, la première est la langue par défaut) |
| `--langs` | langues du site vitrine (`--ui`) : `fr,en,es,ar`, défaut `fr,en` (ou celles de `--i18n`), français obligatoire ; ADR 0017 |
| `--docker`, `--ci` | Dockerfile + compose ; workflow GitHub Actions |

Les briques plateforme sont détaillées dans [`platform.md`](platform.md).

| Laravel | Valeurs |
|---------|---------|
| `--starter` | `none` (défaut), `breeze-blade`, `breeze-livewire`, `breeze-react`, `breeze-vue`, `breeze-api` |
| `--dark`, `--pest` | mode sombre Breeze, Pest au lieu de PHPUnit (exigent un kit) |
| `--db` | `sqlite` (défaut : migrations + données d'exemple exécutées), `mysql`, `pgsql` |
| `--frontend` | `npm install && npm run build` même sans kit |
| `--ui` | `none`, `css` (design kit CSS pur) ou `starwind` (Tailwind v4 + règles des atomes Starwind) : composants Blade `<x-ds.*>`, page `/components` et **site vitrine** FR/EN (squelette sans Breeze) |
| `--auth` | comptes du site avec **Laravel Fortify** (`/auth/sign-in`, `/auth/sign-up`, `/auth/dashboard`) ; exige `--ui` |
| `--icons` | jeu d'icônes Iconify du site (MDI par défaut, `lucide`, tout jeu Iconify) ; exige `--ui` |
| `--langs` | langues du site vitrine : `fr,en,es,ar` (défaut `fr,en`, français obligatoire) ; exige `--ui` |

Wagtail : `--package` (paquets pip en plus), `--no-migrate`, `--ui css|starwind` (application `ds` : balises `{% ds_* %}`, page `/components/`, site vitrine dans les langues de `--langs` ; Starwind compilé par la CLI autonome Tailwind), `--auth` (authentification Django), `--icons <jeu>`, `--langs fr,en,es,ar`. Rust : `--kind bin|lib`, `--edition`, `--dep`.

## Contrôles et garde-fous

* **Pré-vérification** : chaque outil requis est détecté avec sa version minimale (Node ≥ 22.12, PHP ≥ 8.3 + extensions,
  Composer, Python ≥ 3.10 avec `venv`, Cargo ≥ 1.85…). S'il manque quelque chose, rien n'est créé et l'erreur indique
  comment l'installer.
* **Compatibilités validées** avant tout travail : un seul framework d'îlots (react/vue/svelte), `sitemap` exige `--site`,
  les intégrations exigent l'installation, `--dark/--pest` exigent un kit Breeze, Wagtail exige l'installation…
* **Aucune surprise sur le disque** : `--force` ne supprime qu'un dossier portant le manifeste `.alpharust-project.json`
  (un dossier étranger est refusé même avec `--force`) ; les noms sont validés (un seul segment, pas de `..`).
* **Processus enfants maîtrisés** : `CI=1`, couleurs et télémétrie désactivées, stdin fermé, délai par étape
  (`ALPHARUST_STEP_TIMEOUT`, 20 min par défaut), annulation propre (GUI : bouton « Annuler »).
* **Sortie claire** : chaque étape est numérotée avec sa durée ; en cas d'échec la fin du journal de l'outil est reproduite.
  Codes de sortie : `0` succès, `1` échec d'exécution (outil manquant, commande en échec), `2` erreur d'usage.

## La couche de modèles (`templates/layers/`)

Les fichiers sont embarqués dans le binaire (`include_dir!`) et s'appliquent **après** l'outil officiel :

* `common/` (`.alpharust.toml`, `docs/`…) puis `<framework>/` ;
* première ligne facultative `@@meta mode=create|replace when=<expr>@@` : `create` ne **n'écrase jamais** un fichier
  officiel, `replace` le remplace ; `when` combine des drapeaux avec `|` (ou), `&` (et), `!` (non) ;
* blocs `@@if expr@@ … @@else@@ … @@end@@` sur leurs propres lignes, ou en ligne (`@@if typecheck@@, check@@end@@`) ;
* variables `@@name@@ @@slug@@ @@pkg@@ @@title@@ @@pm@@ @@run@@ @@typescript@@…` dans les contenus **et** les chemins ;
  une variable inconnue est conservée (donc détectable) ; `dot-xxx` devient `.xxx`, `.tpl` est retiré.

Contenu de la couche Astro `alpharust` : `BaseLayout` (SEO, lien d'évitement, thème clair/sombre sans flash),
`Header`/`Footer`/`SEO`/`ThemeToggle`, pages (accueil, à propos, 404, blog + `[...slug]`), collection `blog` typée (Zod,
loader `glob`), articles Markdown (+ MDX si `mdx`), îlot de démonstration React/Vue/Svelte, `tokens.css` + `global.css`
(Tailwind importé en premier si choisi). Laravel : pages + modèle + migration + seeder + tests, branchés **sans conflit**
avec Breeze (`routes/pages.php`, `components/site-layout`). Wagtail : `.alpharust.toml`, README, docs, `.gitignore`,
`.env.example` et un test d'accueil par-dessus le projet créé par `wagtail start`.

## Tests

* `tests/generators.rs` — **hermétiques** : un faux `npm`/`node` simule `create-astro` ; ordre des étapes, correctifs
  (tsconfig, scripts, `site`), couche, manifeste, lint, échec d'un outil, outil absent, annulation, `--force`.
  `cargo new` est lui **réel**.
* **Générations réelles** : `ALPHARUST_E2E=1 cargo test --test generators e2e -- --test-threads=1` lance create-astro
  (npm et pnpm, intégrations, templates officiels), composer + Breeze + Pest, venv + wagtail ; chaque test s'ignore
  avec un message si l'outil manque. La CI les exécute (job `generated-projects`).

Autres générateurs de texte : `readme`, `docs`, `checks` (checklist Markdown d'un profil).

## Ajouter un framework

1. `src/scaffold/<fw>.rs` : options (serde), `validate`, `tools` (preflight) et `steps` (commandes officielles).
2. `templates/layers/<fw>/` : couche de modèles.
3. L'enregistrer dans `Framework`/`Options` (`src/scaffold/mod.rs`) : la CLI, la TUI et la GUI le proposent via `Options`.
4. Tests hermétiques (shims) et un test `e2e_*`.
