{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
<section class="site-page-header">
    <div class="site-container">
        {% if s.eyebrow %}
            <p class="site-eyebrow">
                {{ s.eyebrow }}
            </p>
        {% endif %}
        <h1 class="site-page-header__title">
            {{ s.title }}
        </h1>
        {% if s.lead %}
            <p class="site-lead">
                {{ s.lead }}
            </p>
        {% endif %}
    </div>
</section>
