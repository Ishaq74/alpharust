{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
<section class="site-section site-section--text">
    <div class="site-container site-container--narrow">
        {% if s.title %}
            <h2 class="site-section__title">
                {{ s.title }}
            </h2>
        {% endif %}
        {% ds_prose class_="site-prose" %}
            {{ s.body }}
        {% endds_prose %}
    </div>
</section>
