# Documentation — @@title@@

## Génération

Projet créé par AlphaRust (v@@version@@) avec les outils officiels :

1. `python -m venv .venv` ;
2. `pip install wagtail` ;
3. `wagtail start @@pkg@@ .` (réglages `base` / `dev` / `production`, app `home`, app `search`) ;
4. couche AlphaRust : configuration qualité, documentation, tests supplémentaires ;
5. vérification : `manage.py check`, `makemigrations --check`, `manage.py test` et lint AlphaRust.

La configuration exacte est conservée dans `.alpharust-project.json`.

## Réglages

- `@@pkg@@/settings/base.py` : réglages communs ;
- `@@pkg@@/settings/dev.py` : développement (utilisé par `manage.py`) ;
- `@@pkg@@/settings/production.py` : production (définir `SECRET_KEY` et `ALLOWED_HOSTS` par l'environnement).

## Tests

`home/tests.py` (fourni par Wagtail) et `home/test_pages.py` (AlphaRust) : `python manage.py test`.
