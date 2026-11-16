# Wagtail Generator

Crée un venv Python (`.venv`), installe Wagtail (`pip`), exécute `wagtail start`, installe `requirements.txt`, migre,
fige `requirements-lock.txt`, puis ajoute la couche AlphaRust (README, docs, `.gitignore`, `.env.example`, test d'accueil,
`.alpharust.toml`). Vérifié par `manage.py check`, `makemigrations --check` et `manage.py test`.
Couche : `templates/layers/wagtail/`. Code : `src/scaffold/wagtail.rs`.
