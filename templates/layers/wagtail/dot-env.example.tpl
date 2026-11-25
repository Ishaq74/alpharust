# Copier vers .env et adapter. Jamais de secret réel dans le dépôt.
DJANGO_SETTINGS_MODULE=@@pkg@@.settings.dev

# Production (settings/production.py, `python manage.py check --deploy`) :
# DJANGO_SETTINGS_MODULE=@@pkg@@.settings.production
# DJANGO_SECRET_KEY=une-longue-valeur-aleatoire
# DJANGO_ALLOWED_HOSTS=exemple.fr,www.exemple.fr
# DJANGO_CSRF_TRUSTED_ORIGINS=https://exemple.fr
# WAGTAILADMIN_BASE_URL=https://exemple.fr
# DJANGO_BEHIND_PROXY=1
