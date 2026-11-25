{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
<section class="site-section site-section--services">
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
                        {% site_icon "arrow-right" size=16 %}
                    {% endds_button %}
                {% endif %}
            </div>
        {% endif %}
        <div class="site-grid site-grid--3">
            {% for item in s.items %}
                {% ds_card class_="site-card" %}
                    {% ds_card_header %}
                        <span class="site-icon-box">
                            {% site_icon item.icon size=22 %}
                        </span>
                        <h3 class="site-card__title">
                            <a href="{{ item.url }}">
                                {{ item.title }}
                            </a>
                        </h3>
                    {% endds_card_header %}
                    {% ds_card_content %}
                        <p class="site-card__text">
                            {{ item.summary }}
                        </p>
                    {% endds_card_content %}
                    {% ds_card_footer class_="site-card__footer" %}
                        {% ds_badge variant="outline" %}
                            {{ item.price }}
                        {% endds_badge %}
                        {% ds_button href=item.url variant="outline" size="sm" aria_label=item.more_label %}
                            {{ site.ui.more }}
                        {% endds_button %}
                    {% endds_card_footer %}
                {% endds_card %}
            {% endfor %}
        </div>
    </div>
</section>
