@@meta mode=replace@@
# @@title@@

Application [Laravel](https://laravel.com) créée avec [AlphaRust](https://github.com/Ishaq74/alpharust) :
`composer create-project laravel/laravel`@@if breeze@@, kit de démarrage Breeze@@end@@, puis une couche de pages
(modèle `Page`, migration, contrôleur, vues, tests).

## Installation

Prérequis : PHP ≥ 8.3, Composer@@if breeze@@, Node.js ≥ 20@@end@@@@if ui@@, Node.js ≥ 20 (Vite, Tailwind CSS v4)@@end@@.

```bash
composer install
cp .env.example .env
php artisan key:generate
php artisan migrate --seed --seeder=PageSeeder
@@if ui@@
npm install && npm run build   # Vite + Tailwind v4 : feuille et script du design kit
@@end@@
```

## Usage

```bash
php artisan serve        # http://localhost:8000 — pages sur /pages
php artisan test         # tests PHPUnit
./vendor/bin/pint        # style du code
alpharust lint           # règles AlphaRust (profil « web »)
```

## Structure ajoutée par AlphaRust

```text
app/Models/Page.php
app/Http/Controllers/PageController.php
database/migrations/2026_01_01_000000_create_pages_table.php
database/factories/PageFactory.php
database/seeders/PageSeeder.php
resources/views/components/site-layout.blade.php
resources/views/pages/{index,show}.blade.php
routes/pages.php            # inclus depuis routes/web.php
tests/Feature/PageTest.php  tests/Unit/PageTest.php
```

@@if ui@@
## Composants

Catalogue de 23 composants Blade (`<x-ds.button variant="primary">`…) sur la page `/components`, dont le balisage
(`data-slot`) est identique à celui des versions Astro et Wagtail. Style : `resources/css/design/` (jetons dans `tokens/`,
un fichier par composant dans `components/`, vos surcharges dans `components/custom.css`). Détails : [`docs/design.md`](docs/design.md).

@@end@@
Voir [`docs/index.md`](docs/index.md).

## License

MIT.
