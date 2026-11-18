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
                {% site_icon "calendar" size=16 %}
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
                    {% site_icon "arrow-left" size=16 %}
                    {{ site.ui.all_posts }}
                {% endds_button %}
            </p>
        </div>
    </div>
</article>
{% endblock %}
