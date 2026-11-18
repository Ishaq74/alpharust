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
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
            {% endds_button %}
            {% if s.secondary %}
                {% ds_button href=s.secondary.url variant="outline" size="lg" %}
                    {{ s.secondary.label }}
                {% endds_button %}
            {% endif %}
        </div>
    </div>
</section>
