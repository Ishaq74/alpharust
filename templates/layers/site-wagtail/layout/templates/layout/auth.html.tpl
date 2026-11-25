@@meta when=auth@@
{% extends "base.html" %}
{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
{% block title %}{{ page.title }} · {{ site.name }}{% endblock %}
{% block og_title %}{{ page.title }} · {{ site.name }}{% endblock %}
{% block content %}
<section class="site-section site-auth">
    <div class="site-container site-container--narrow">
        {% ds_card class_="site-card site-auth__card" %}
            {% ds_card_header %}
                <h1 class="site-auth__title">
                    {{ page.title }}
                </h1>
                <p class="site-lead">
                    {{ page.lead }}
                </p>
            {% endds_card_header %}
            {% ds_card_content %}
                {% if page.error %}
                    {% ds_alert variant="error" class_="site-form__notice" %}
                        {% ds_alert_title %}
                            {{ page.error }}
                        {% endds_alert_title %}
                    {% endds_alert %}
                {% endif %}
                <form class="site-form" method="post" action="{{ page.action }}">
                    {% csrf_token %}
                    <input type="hidden" name="locale" value="{{ site.locale }}">
                    {% for field in page.fields %}
                        <div class="site-field">
                            {% ds_label for_=field.id %}
                                {{ field.label }}
                            {% endds_label %}
                            {% ds_input id=field.id name=field.name type=field.type autocomplete=field.autocomplete value=field.value required=True %}
                        </div>
                    {% endfor %}
                    {% ds_button type="submit" variant="primary" class_="site-block" %}
                        {{ page.submit }}
                    {% endds_button %}
                </form>
            {% endds_card_content %}
            {% ds_card_footer class_="site-auth__alt" %}
                <span>
                    {{ page.alt_text }}
                </span>
                <a href="{{ page.alt_url }}">
                    {{ page.alt_label }}
                </a>
            {% endds_card_footer %}
        {% endds_card %}
    </div>
</section>
{% endblock %}
