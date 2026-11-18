# Documentation — @@title@@

## Génération

Projet créé par AlphaRust (v@@version@@) :

1. `composer create-project laravel/laravel` (outil officiel) ;
@@if breeze@@
2. `composer require laravel/breeze --dev` puis `php artisan breeze:install` ;
@@end@@
3. couche AlphaRust : pages (modèle, migration, factory, seeder, contrôleur, vues, tests), configuration qualité ;
4. vérification : `php artisan test` et lint AlphaRust.

La configuration exacte est conservée dans `.alpharust-project.json`.

## Pages

| Route | Contrôleur | Vue |
|-------|------------|-----|
| `GET /pages` | `PageController@index` | `pages.index` |
| `GET /pages/{slug}` | `PageController@show` | `pages.show` |

Le modèle `Page` possède `title`, `slug` (unique) et `body`. `php artisan db:seed --class=PageSeeder` crée une page d'exemple.
