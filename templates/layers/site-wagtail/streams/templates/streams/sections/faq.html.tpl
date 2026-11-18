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
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
                    {% endds_button %}
                {% endif %}
            </div>
        {% endif %}
        {% ds_accordion type="single" class_="site-faq__list" %}
            {% for item in s.items %}
                {% ds_accordion_item value=item.question %}
                    {% ds_accordion_trigger %}
                        {{ item.question }}
                    {% endds_accordion_trigger %}
                    {% ds_accordion_content %}
                        <p>
                            {{ item.answer }}
                        </p>
                    {% endds_accordion_content %}
                {% endds_accordion_item %}
            {% endfor %}
        {% endds_accordion %}
    </div>
</section>
