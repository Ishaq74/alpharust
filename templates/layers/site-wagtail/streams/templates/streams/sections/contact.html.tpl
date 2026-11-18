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
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="m22 2-7 20-4-9-9-4Z"/><path d="M22 2 11 13"/></svg>
                        {% endds_button %}
                    </form>
                {% endds_card_content %}
            {% endds_card %}
            <aside class="site-contact__info">
                {% ds_card class_="site-card" %}
                    {% ds_card_header %}
                        <h2 class="site-card__title">
                            Coordonnées
                        </h2>
                    {% endds_card_header %}
                    {% ds_card_content %}
                        <ul class="site-contact__list">
                            <li>
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/></svg>
                                <a href="{{ site.contact.email_url }}">
                                    {{ site.contact.email }}
                                </a>
                            </li>
                            <li>
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72c.13.96.36 1.9.7 2.81a2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45c.91.34 1.85.57 2.81.7A2 2 0 0 1 22 16.92z"/></svg>
                                <a href="{{ site.contact.phone_url }}">
                                    {{ site.contact.phone }}
                                </a>
                            </li>
                            <li>
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z"/><circle cx="12" cy="10" r="3"/></svg>
                                <span>
                                    {{ site.contact.address }}
                                </span>
                            </li>
                            <li>
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
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
