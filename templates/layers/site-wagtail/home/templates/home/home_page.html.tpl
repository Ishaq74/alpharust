@@meta mode=replace@@
{% extends "base.html" %}
{% load wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
{% block title %}{{ site.name }}{% endblock %}
{% block og_title %}{{ site.name }}{% endblock %}
{% block content %}
{% for block in page.body %}{% include_block block %}{% endfor %}
{% endblock %}
