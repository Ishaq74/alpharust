{% extends "base.html" %}
{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
{% block content %}
<section class="site-page-header">
    <div class="site-container">
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
        <span class="site-icon-box site-icon-box--lg">
            {% site_icon page.icon size=28 %}
        </span>
        <h1 class="site-page-header__title">
            {{ page.title }}
        </h1>
        <p class="site-lead">
            {{ page.summary }}
        </p>
        {% ds_badge variant="primary" size="lg" %}
            {{ page.price }}
        {% endds_badge %}
    </div>
</section>
<section class="site-section">
    <div class="site-container site-detail">
        {% ds_prose class_="site-prose site-detail__body" %}
            {{ page.body|richtext }}
        {% endds_prose %}
        <aside class="site-detail__aside">
            {% ds_card class_="site-card" %}
                {% ds_card_header %}
                    <h2 class="site-card__title">
                        Ce qui est inclus
                    </h2>
                {% endds_card_header %}
                {% ds_card_content %}
                    <ul class="site-checklist">
                        {% for feature in page.features %}
                            <li>
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M20 6 9 17l-5-5"/></svg>
                                <span>
                                    {{ feature }}
                                </span>
                            </li>
                        {% endfor %}
                    </ul>
                {% endds_card_content %}
                {% ds_card_footer %}
                    {% ds_button href=site.cta.url variant="primary" class_="site-block" %}
                        {{ site.cta.label }}
                    {% endds_button %}
                {% endds_card_footer %}
            {% endds_card %}
        </aside>
    </div>
</section>
{% endblock %}
