{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
<section class="site-section site-section--stats">
    <div class="site-container">
        <dl class="site-stats__grid">
            {% for item in s.items %}
                <div class="site-stat">
                    <dt class="site-stat__label">
                        {{ item.label }}
                    </dt>
                    <dd class="site-stat__value">
                        {{ item.value }}
                    </dd>
                </div>
            {% endfor %}
        </dl>
    </div>
</section>
