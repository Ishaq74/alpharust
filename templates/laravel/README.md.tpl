# @@title@@

Projet Laravel généré par [AlphaRust](https://github.com/Ishaq74/alpharust) (profil `web`).

## Installation

Prérequis : PHP ≥ 8.3 et Composer.

```bash
composer install
cp .env.example .env
php artisan key:generate
touch database/database.sqlite
php artisan migrate --seed
```

## Usage

```bash
php artisan serve      # http://localhost:8000
php artisan test       # tests PHPUnit
alpharust lint --profile web .
```

## Structure

- `app/Http/Controllers/PageController.php` + `app/Models/Page.php`
- `database/migrations/` : migrations versionnées (table `pages`)
- `resources/views/` : vues Blade (layout accessible, `lang` renseigné)
- `tests/` : tests Feature (HTTP + base) et Unit

## License

MIT
