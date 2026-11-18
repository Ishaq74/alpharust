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
                <span class="site-icon-box">
                    {% site_icon "user" size=22 %}
                </span>
                <h1 class="site-auth__title">
                    {{ page.title }}
                </h1>
                <p class="site-lead">
                    {{ page.lead }}
                </p>
            {% endds_card_header %}
            {% ds_card_content %}
                <dl class="site-auth__details">
                    <dt>
                        {{ site.ui.field_name }}
                    </dt>
                    <dd>
                        {{ page.name }}
                    </dd>
                    <dt>
                        {{ site.ui.field_email }}
                    </dt>
                    <dd>
                        {{ page.email }}
                    </dd>
                </dl>
            {% endds_card_content %}
            {% ds_card_footer %}
                <form method="post" action="{{ page.logout_action }}">
                    {% csrf_token %}
                    <input type="hidden" name="locale" value="{{ site.locale }}">
                    {% ds_button type="submit" variant="outline" %}
                        {% site_icon "log-out" size=16 %}
                        {{ site.ui.sign_out }}
                    {% endds_button %}
                </form>
            {% endds_card_footer %}
        {% endds_card %}
    </div>
</section>
{% endblock %}
