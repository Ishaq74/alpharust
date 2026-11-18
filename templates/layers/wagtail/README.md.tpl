@@meta mode=replace@@
# @@title@@

Site [Wagtail](https://wagtail.org) créé avec [AlphaRust](https://github.com/Ishaq74/alpharust) :
environnement virtuel, `pip install wagtail`, `wagtail start @@pkg@@`, puis couche qualité (tests, docs, configuration).

## Installation

Prérequis : Python ≥ 3.10 (3.12 recommandé).

```bash
python3 -m venv .venv
. .venv/bin/activate
pip install -r requirements.txt
python manage.py migrate
python manage.py createsuperuser
```

`requirements-lock.txt` contient les versions exactes installées lors de la génération.

## Usage

```bash
python manage.py runserver      # http://localhost:8000 — admin sur /admin/
python manage.py test           # tests
python manage.py check          # contrôles Django
alpharust lint                  # règles AlphaRust (profil « cms »)
```

@@if ui@@
## Composants

Catalogue de 23 composants (`{% ds_button variant="primary" %}`…) sur la page `/components/`, dont le balisage (`data-slot`)
est identique à celui des versions Astro et Laravel. Application Django `ds` : balises dans `ds/templatetags/ds.py`, style dans
`ds/static/ds/css/` (jetons dans `tokens/`, un fichier par composant, vos surcharges dans `components/custom.css`).
Détails : [`docs/design.md`](docs/design.md).

@@end@@
Voir [`docs/index.md`](docs/index.md).

## License

MIT.
