{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
<section class="site-section site-section--posts">
    <div class="site-container">
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
        <div class="site-grid site-grid--3">
            {% for item in s.items %}
                {% ds_card class_="site-card" %}
                    {% ds_card_header %}
                        <div class="site-meta">
                            {% ds_badge variant="outline" %}
                                {{ item.category }}
                            {% endds_badge %}
                            <time datetime="{{ item.date_iso }}">
                                {{ item.date_label }}
                            </time>
                        </div>
                        <h3 class="site-card__title">
                            <a href="{{ item.url }}">
                                {{ item.title }}
                            </a>
                        </h3>
                    {% endds_card_header %}
                    {% ds_card_content %}
                        <p class="site-card__text">
                            {{ item.excerpt }}
                        </p>
                    {% endds_card_content %}
                    {% ds_card_footer %}
                        <span class="site-muted">
                            {{ item.author }}
                        </span>
                    {% endds_card_footer %}
                {% endds_card %}
            {% endfor %}
        </div>
    </div>
</section>
