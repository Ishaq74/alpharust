@@meta mode=replace@@
"""Réglages de production durcis par AlphaRust (liste de contrôle Django :
https://docs.djangoproject.com/en/stable/howto/deployment/checklist/ ; `python manage.py check --deploy`).

Variables d'environnement : DJANGO_SECRET_KEY et DJANGO_ALLOWED_HOSTS (obligatoires), DJANGO_CSRF_TRUSTED_ORIGINS,
WAGTAILADMIN_BASE_URL, DJANGO_SECURE_HSTS_SECONDS (3600 par défaut ; à augmenter une fois HTTPS validé),
DJANGO_SECURE_SSL_REDIRECT (1 par défaut), DJANGO_BEHIND_PROXY=1 derrière un proxy HTTPS qui envoie X-Forwarded-Proto.
"""
import os

import django
from django.core.exceptions import ImproperlyConfigured

from .base import *  # noqa: F403

DEBUG = False


def env(name, default=None):
    value = os.environ.get(name, default)
    if value is None or value == "":
        raise ImproperlyConfigured(f"Variable d'environnement {name} requise en production")
    return value


def env_list(name, default=None):
    return [item.strip() for item in env(name, default).split(",") if item.strip()]


SECRET_KEY = env("DJANGO_SECRET_KEY")
ALLOWED_HOSTS = env_list("DJANGO_ALLOWED_HOSTS")
CSRF_TRUSTED_ORIGINS = env_list("DJANGO_CSRF_TRUSTED_ORIGINS", ",")
WAGTAILADMIN_BASE_URL = os.environ.get("WAGTAILADMIN_BASE_URL", WAGTAILADMIN_BASE_URL)  # noqa: F405

# HTTPS et cookies (réglages SECURE_* de la liste de contrôle)
SECURE_SSL_REDIRECT = os.environ.get("DJANGO_SECURE_SSL_REDIRECT", "1") == "1"
if os.environ.get("DJANGO_BEHIND_PROXY") == "1":
    SECURE_PROXY_SSL_HEADER = ("HTTP_X_FORWARDED_PROTO", "https")
SESSION_COOKIE_SECURE = True
CSRF_COOKIE_SECURE = True
SECURE_HSTS_SECONDS = int(os.environ.get("DJANGO_SECURE_HSTS_SECONDS", "3600"))
SECURE_CONTENT_TYPE_NOSNIFF = True
SECURE_REFERRER_POLICY = "same-origin"
# L'aperçu de l'admin Wagtail affiche la page dans un cadre de même origine.
X_FRAME_OPTIONS = "SAMEORIGIN"
SILENCED_SYSTEM_CHECKS = ["security.W019"]  # SAMEORIGIN voulu (aperçu Wagtail)

# CSP de base, la même qu'Astro et Laravel (middleware CSP intégré à Django depuis la 6.0).
CSP = "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob: https:; font-src 'self' data:; connect-src 'self'; object-src 'none'; base-uri 'self'; form-action 'self'; frame-ancestors 'self'"
if django.VERSION >= (6, 0):
    MIDDLEWARE = [*MIDDLEWARE, "django.middleware.csp.ContentSecurityPolicyMiddleware"]  # noqa: F405
    SECURE_CSP = {
        directive: sources.split()
        for directive, sources in (part.strip().split(" ", 1) for part in CSP.split(";"))
    }

# ManifestStaticFilesStorage is recommended in production, to prevent
# outdated JavaScript / CSS assets being served from cache
# (e.g. after a Wagtail upgrade).
# See https://docs.djangoproject.com/en/stable/ref/contrib/staticfiles/#manifeststaticfilesstorage
STORAGES["staticfiles"]["BACKEND"] = "django.contrib.staticfiles.storage.ManifestStaticFilesStorage"  # noqa: F405

try:
    from .local import *  # noqa: F403
except ImportError:
    pass
