{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
<section class="site-section site-section--team">
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
        <ul class="site-grid site-grid--4 site-team">
            {% for item in s.items %}
                <li class="site-team__member">
                    {% ds_avatar size="lg" %}
                        {% ds_avatar_fallback %}
                            {{ item.initials }}
                        {% endds_avatar_fallback %}
                    {% endds_avatar %}
                    <p class="site-person__name">
                        {{ item.name }}
                    </p>
                    <p class="site-person__role">
                        {{ item.role }}
                    </p>
                </li>
            {% endfor %}
        </ul>
    </div>
</section>
