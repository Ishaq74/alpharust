@@meta mode=replace@@
{% extends "base.html" %}
{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
{% block title %}Page introuvable · {{ site.name }}{% endblock %}
{% block og_title %}Page introuvable · {{ site.name }}{% endblock %}
{% block content %}
<section class="site-section site-404">
    <div class="site-container site-container--narrow">
        <p class="site-404__code">
            404
        </p>
        <h1 class="site-page-header__title">
            Page introuvable
        </h1>
        <p class="site-lead">
            La page demandée n’existe pas ou a été déplacée.
        </p>
        {% ds_button href=site.home_url variant="primary" %}
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
            Retour à l’accueil
        {% endds_button %}
    </div>
</section>
{% endblock %}
