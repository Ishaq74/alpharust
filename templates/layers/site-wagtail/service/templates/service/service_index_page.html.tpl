{% extends "base.html" %}
{% load wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
{% block content %}
{% for block in page.body %}{% include_block block %}{% endfor %}
{% endblock %}
