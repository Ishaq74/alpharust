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
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
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
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="m6 9 6 6 6-6"/></svg>
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
