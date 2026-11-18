{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
<section class="site-section site-section--cta">
    <div class="site-container">
        <div class="site-cta">
            <div>
                <h2 class="site-cta__title">
                    {{ s.title }}
                </h2>
                <p class="site-cta__text">
                    {{ s.text }}
                </p>
            </div>
            {% ds_button href=s.action.url variant="default" size="lg" %}
                {{ s.action.label }}
                {% site_icon "arrow-right" size=18 %}
            {% endds_button %}
        </div>
    </div>
</section>
