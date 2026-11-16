# @@title@@

Projet Wagtail généré par [AlphaRust](https://github.com/Ishaq74/alpharust) (profil `cms`).

## Installation

Prérequis : Python ≥ 3.12.

```bash
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
python manage.py migrate
python manage.py createsuperuser
```

## Usage

```bash
python manage.py runserver      # http://localhost:8000  — admin : /admin/
python manage.py test           # tests
alpharust lint --profile cms .  # règles AlphaRust
```

## Structure

- `@@pkg@@/settings/` : réglages séparés `base` / `dev` / `production` (secrets via l'environnement)
- `home/` : application avec `HomePage` et ses migrations versionnées
- `.env.example` : variables à définir en production

## License

MIT
