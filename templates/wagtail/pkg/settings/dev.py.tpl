"""Développement local : DEBUG actif, clé de développement uniquement."""
from .base import *  # noqa: F401,F403

DEBUG = True
SECRET_KEY = "dev-only-insecure-key"  # alpharust-ignore
ALLOWED_HOSTS = ["localhost", "127.0.0.1", "testserver"]
EMAIL_BACKEND = "django.core.mail.backends.console.EmailBackend"
