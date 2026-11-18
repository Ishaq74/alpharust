{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
<section class="site-hero">
    <div class="site-container site-hero__inner">
        {% if s.badge %}
            {% ds_badge variant="primary" %}
                {{ s.badge }}
            {% endds_badge %}
        {% endif %}
        <h1 class="site-hero__title">
            {{ s.title }}
        </h1>
        <p class="site-hero__lead">
            {{ s.lead }}
        </p>
        <div class="site-hero__actions">
            {% ds_button href=s.primary.url variant="primary" size="lg" %}
                {{ s.primary.label }}
                {% site_icon "arrow-right" size=18 %}
            {% endds_button %}
            {% if s.secondary %}
                {% ds_button href=s.secondary.url variant="outline" size="lg" %}
                    {{ s.secondary.label }}
                {% endds_button %}
            {% endif %}
        </div>
    </div>
</section>
