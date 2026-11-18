{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
<section class="site-section site-section--testimonials">
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
                {% ds_card class_="site-card site-quote" %}
                    {% ds_card_content %}
                        {% site_icon "quote" size=24 %}
                        <blockquote class="site-quote__text">
                            <p>
                                {{ item.quote }}
                            </p>
                        </blockquote>
                    {% endds_card_content %}
                    {% ds_card_footer class_="site-person" %}
                        {% ds_avatar size="md" %}
                            {% ds_avatar_fallback %}
                                {{ item.initials }}
                            {% endds_avatar_fallback %}
                        {% endds_avatar %}
                        <div>
                            <p class="site-person__name">
                                {{ item.name }}
                            </p>
                            <p class="site-person__role">
                                {{ item.role }}
                            </p>
                        </div>
                    {% endds_card_footer %}
                {% endds_card %}
            {% endfor %}
        </div>
    </div>
</section>
