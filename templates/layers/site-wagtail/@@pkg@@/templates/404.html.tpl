@@meta mode=replace@@
{% extends "base.html" %}
{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
{% block title %}{{ site.ui.not_found }} · {{ site.name }}{% endblock %}
{% block og_title %}{{ site.ui.not_found }} · {{ site.name }}{% endblock %}
{% block content %}
<section class="site-section site-404">
    <div class="site-container site-container--narrow">
        <p class="site-404__code">
            404
        </p>
        <h1 class="site-page-header__title">
            {{ site.ui.not_found }}
        </h1>
        <p class="site-lead">
            {{ site.ui.not_found_text }}
        </p>
        {% ds_button href=site.home_url variant="primary" %}
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
            {{ site.ui.back_home }}
        {% endds_button %}
    </div>
</section>
{% endblock %}
