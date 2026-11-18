{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
<section class="site-section site-section--faq">
    <div class="site-container site-container--narrow">
        {% if s.title %}
            <div class="site-section__head">
                <div>
                    <h2 class="site-section__title">
                        {{ s.title }}
                    </h2>
                    {% if s.lead %}
                        <p class="site-lead">
                            {{ s.lead }}
                        </p>
                    {% endif %}
                </div>
                {% if s.link %}
                    {% ds_button href=s.link.url variant="ghost" %}
                        {{ s.link.label }}
                        {% site_icon "arrow-right" size=16 %}
                    {% endds_button %}
                {% endif %}
            </div>
        {% endif %}
        <div class="site-faq__list">
            {% for item in s.items %}
                <details class="site-faq__item" name="faq">
                    <summary class="site-faq__question">
                        <span>
                            {{ item.question }}
                        </span>
                        {% site_icon "chevron-down" size=18 %}
                    </summary>
                    <div class="site-faq__answer">
                        <p>
                            {{ item.answer }}
                        </p>
                    </div>
                </details>
            {% endfor %}
        </div>
    </div>
</section>
