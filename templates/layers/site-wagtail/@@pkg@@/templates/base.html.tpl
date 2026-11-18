@@meta mode=replace@@
{% load i18n static wagtailcore_tags wagtailuserbar %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
{% get_current_language as LANGUAGE_CODE %}<!DOCTYPE html>
<html lang="{{ LANGUAGE_CODE }}">
    <head>
        <meta charset="utf-8" />
        <title>{% block title %}{% if page.seo_title %}{{ page.seo_title }}{% else %}{{ page.title }}{% endif %} · {{ site.name }}{% endblock %}</title>
        <meta name="description" content="{% block description %}{% if page.search_description %}{{ page.search_description }}{% else %}{{ site.description }}{% endif %}{% endblock %}" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="canonical" href="{{ request.build_absolute_uri }}" />
        <meta property="og:type" content="website" />
        <meta property="og:title" content="{% block og_title %}{% if page.seo_title %}{{ page.seo_title }}{% else %}{{ page.title }}{% endif %} · {{ site.name }}{% endblock %}" />
        <meta property="og:description" content="{% if page.search_description %}{{ page.search_description }}{% else %}{{ site.description }}{% endif %}" />
        <script>
            // Applique le thème choisi avant le premier rendu (évite le flash).
            try {
                const saved = localStorage.getItem('theme');
                const theme = saved === 'light' || saved === 'dark' ? saved : (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
                document.documentElement.dataset.theme = theme;
                document.documentElement.classList.toggle('dark', theme === 'dark');
            } catch (e) {}
        </script>

        {# Force all links in the live preview panel to be opened in a new tab #}
        {% if request.in_preview_panel %}
        <base target="_blank">
        {% endif %}

        {# Global stylesheets #}
        <link rel="stylesheet" type="text/css" href="{% static 'css/site.css' %}">

        {% block extra_css %}
        {# Override this in templates to add extra stylesheets #}
        {% endblock %}
    </head>

    <body class="site-body{% block body_class %}{% endblock %}">
        {% wagtailuserbar %}
        <a class="site-skip" href="#contenu">{{ site.ui.skip }}</a>
        {% include "includes/header.html" %}
        <main id="contenu" class="site-main" tabindex="-1">
            {% block content %}{% endblock %}
        </main>
        {% include "includes/footer.html" %}

        {# Global javascript #}

        {% block extra_js %}
        {# Override this in templates to add extra javascript #}
        {% endblock %}
    </body>
</html>
