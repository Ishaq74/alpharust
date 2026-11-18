{% extends "base.html" %}
{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
{% block content %}
<article class="site-article">
    <header class="site-page-header">
        <div class="site-container site-container--narrow">
            {% ds_breadcrumb %}
                {% ds_breadcrumb_list %}
                    {% for crumb in page.crumbs %}
                        {% ds_breadcrumb_item %}
                            {% ds_breadcrumb_link href=crumb.url %}
                                {{ crumb.label }}
                            {% endds_breadcrumb_link %}
                        {% endds_breadcrumb_item %}
                        {% ds_breadcrumb_separator %}{% endds_breadcrumb_separator %}
                    {% endfor %}
                    {% ds_breadcrumb_item %}
                        {% ds_breadcrumb_page %}
                            {{ page.title }}
                        {% endds_breadcrumb_page %}
                    {% endds_breadcrumb_item %}
                {% endds_breadcrumb_list %}
            {% endds_breadcrumb %}
            {% ds_badge variant="outline" %}
                {{ page.category }}
            {% endds_badge %}
            <h1 class="site-page-header__title">
                {{ page.title }}
            </h1>
            <p class="site-meta">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><rect width="18" height="18" x="3" y="4" rx="2"/><path d="M16 2v4M8 2v4M3 10h18"/></svg>
                <time datetime="{{ page.date_iso }}">
                    {{ page.date_label }}
                </time>
                <span aria-hidden="true">
                    ·
                </span>
                <span>
                    {{ page.author }}
                </span>
            </p>
        </div>
    </header>
    <div class="site-section">
        <div class="site-container site-container--narrow">
            {% ds_prose class_="site-prose" %}
                {{ page.body|richtext }}
            {% endds_prose %}
            <p class="site-article__back">
                {% ds_button href=page.back_url variant="outline" %}
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
                    Tous les articles
                {% endds_button %}
            </p>
        </div>
    </div>
</article>
{% endblock %}
