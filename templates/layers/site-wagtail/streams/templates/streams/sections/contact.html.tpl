{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
<section class="site-section site-section--contact">
    <div class="site-container">
        <div class="site-contact__grid">
            {% ds_card class_="site-card" %}
                {% ds_card_header %}
                    <h2 class="site-card__title">
                        {{ s.title }}
                    </h2>
                {% endds_card_header %}
                {% ds_card_content %}
                    {% if s.sent %}
                        {% ds_alert variant="success" class_="site-form__notice" %}
                            {% ds_alert_title %}
                                {{ s.success }}
                            {% endds_alert_title %}
                        {% endds_alert %}
                    {% endif %}
                    <form class="site-form" method="post" action="{{ s.action }}">
                        {% csrf_token %}
                        {% for field in s.fields %}
                            <div class="site-field">
                                {% ds_label for_=field.id %}
                                    {{ field.label }}
                                    {% if field.required %}
                                        <span class="site-required" aria-hidden="true">
                                            *
                                        </span>
                                    {% endif %}
                                {% endds_label %}
                                {% if field.textarea %}
                                    {% ds_textarea id=field.id name=field.name rows="6" required=field.required aria_invalid=field.invalid aria_describedby=field.error_id default_value=field.value %}{% endds_textarea %}
                                {% else %}
                                    {% ds_input id=field.id name=field.name type=field.type autocomplete=field.autocomplete value=field.value required=field.required aria_invalid=field.invalid aria_describedby=field.error_id %}
                                {% endif %}
                                {% if field.error %}
                                    <p class="site-field__error" id="{{ field.error_id }}">
                                        {{ field.error }}
                                    </p>
                                {% endif %}
                            </div>
                        {% endfor %}
                        {% ds_button type="submit" variant="primary" %}
                            {{ s.submit }}
                            {% site_icon "send" size=16 %}
                        {% endds_button %}
                    </form>
                {% endds_card_content %}
            {% endds_card %}
            <aside class="site-contact__info">
                {% ds_card class_="site-card" %}
                    {% ds_card_header %}
                        <h2 class="site-card__title">
                            {{ site.ui.details }}
                        </h2>
                    {% endds_card_header %}
                    {% ds_card_content %}
                        <ul class="site-contact__list">
                            <li>
                                {% site_icon "mail" size=18 %}
                                <a href="{{ site.contact.email_url }}">
                                    {{ site.contact.email }}
                                </a>
                            </li>
                            <li>
                                {% site_icon "phone" size=18 %}
                                <a href="{{ site.contact.phone_url }}">
                                    {{ site.contact.phone }}
                                </a>
                            </li>
                            <li>
                                {% site_icon "map-pin" size=18 %}
                                <span>
                                    {{ site.contact.address }}
                                </span>
                            </li>
                            <li>
                                {% site_icon "clock" size=18 %}
                                <span>
                                    {{ site.contact.hours }}
                                </span>
                            </li>
                        </ul>
                    {% endds_card_content %}
                {% endds_card %}
            </aside>
        </div>
    </div>
</section>
