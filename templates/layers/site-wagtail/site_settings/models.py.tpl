# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel.
"""Réglages du site (modèle ishaq74/wagtail-cms : site_settings/models.py) : organisation, coordonnées, réseaux."""
from django.db import models
from wagtail.contrib.settings.models import BaseGenericSetting, register_setting


@register_setting(icon="site")
class OrganisationSettings(BaseGenericSetting):
    tagline = models.CharField("accroche", max_length=255, blank=True)
    description = models.TextField("description (SEO par défaut)", blank=True)
    copyright = models.CharField("mention de copyright", max_length=255, blank=True)

    class Meta:
        verbose_name = "organisation"


@register_setting(icon="mail")
class ContactSettings(BaseGenericSetting):
    email = models.EmailField("e-mail", blank=True)
    phone = models.CharField("téléphone", max_length=40, blank=True)
    address = models.CharField("adresse", max_length=255, blank=True)
    hours = models.CharField("horaires", max_length=255, blank=True)

    class Meta:
        verbose_name = "coordonnées"


@register_setting(icon="link")
class SocialMediaSettings(BaseGenericSetting):
    github = models.URLField("GitHub", blank=True)
    linkedin = models.URLField("LinkedIn", blank=True)
    mastodon = models.URLField("Mastodon", blank=True)

    class Meta:
        verbose_name = "réseaux sociaux"
