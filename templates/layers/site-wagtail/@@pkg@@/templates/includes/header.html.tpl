{% load ds site_tags wagtailcore_tags %}
{# Généré par tools/dsgen/vitrine (spec.py) : même site qu’Astro et Laravel. #}
<header class="site-header">
    <div class="site-container site-header__bar">
        <a class="site-brand" href="{{ site.home_url }}">
            <span class="site-brand__mark" aria-hidden="true">
                {{ site.initial }}
            </span>
            <span class="site-brand__name">
                {{ site.name }}
            </span>
        </a>
        <nav class="site-nav" aria-label="Navigation principale">
            <ul class="site-nav__list">
                {% for item in site.nav %}
                    <li>
                        {% if item.children %}
                            {% ds_dropdown open_on_hover=True class_="site-nav__dropdown" %}
                                {% ds_dropdown_trigger class_="site-nav__link" aria_current=item.current %}
                                    {{ item.label }}
                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="m6 9 6 6 6-6"/></svg>
                                {% endds_dropdown_trigger %}
                                {% ds_dropdown_content class_="site-nav__menu" %}
                                    {% ds_dropdown_item as_="a" href=item.url %}
                                        {{ item.label }}
                                    {% endds_dropdown_item %}
                                    {% for child in item.children %}
                                        {% ds_dropdown_item as_="a" href=child.url aria_current=child.current %}
                                            {{ child.label }}
                                        {% endds_dropdown_item %}
                                    {% endfor %}
                                {% endds_dropdown_content %}
                            {% endds_dropdown %}
                        {% else %}
                            <a class="site-nav__link" href="{{ item.url }}"{% if item.current %} aria-current="{{ item.current }}"{% endif %}>
                                {{ item.label }}
                            </a>
                        {% endif %}
                    </li>
                {% endfor %}
            </ul>
        </nav>
        <div class="site-header__actions">
            {% ds_button href=site.cta.url variant="primary" size="sm" class_="site-header__cta" %}
                {{ site.cta.label }}
            {% endds_button %}
            {% ds_theme_toggle size="sm" aria_label="Changer de thème" %}{% endds_theme_toggle %}
            {% ds_sheet class_="site-header__mobile" %}
                {% ds_sheet_trigger as_child=True %}
                    {% ds_button variant="ghost" size="icon" aria_label="Ouvrir le menu" %}
                        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="M4 6h16M4 12h16M4 18h16"/></svg>
                    {% endds_button %}
                {% endds_sheet_trigger %}
                {% ds_sheet_content side="right" class_="site-mobile" %}
                    {% ds_sheet_header %}
                        {% ds_sheet_title %}
                            {{ site.name }}
                        {% endds_sheet_title %}
                    {% endds_sheet_header %}
                    <nav class="site-mobile__nav" aria-label="Navigation mobile">
                        <ul class="site-mobile__list">
                            {% for item in site.nav %}
                                <li>
                                    {% if item.children %}
                                        {% ds_collapsible class_="site-mobile__group" %}
                                            {% ds_collapsible_trigger class_="site-mobile__link" %}
                                                {{ item.label }}
                                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class="site-icon"><path d="m6 9 6 6 6-6"/></svg>
                                            {% endds_collapsible_trigger %}
                                            {% ds_collapsible_content %}
                                                <ul class="site-mobile__sub">
                                                    <li>
                                                        <a href="{{ item.url }}">
                                                            {{ item.label }}
                                                        </a>
                                                    </li>
                                                    {% for child in item.children %}
                                                        <li>
                                                            <a href="{{ child.url }}"{% if child.current %} aria-current="{{ child.current }}"{% endif %}>
                                                                {{ child.label }}
                                                            </a>
                                                        </li>
                                                    {% endfor %}
                                                </ul>
                                            {% endds_collapsible_content %}
                                        {% endds_collapsible %}
                                    {% else %}
                                        <a class="site-mobile__link" href="{{ item.url }}"{% if item.current %} aria-current="{{ item.current }}"{% endif %}>
                                            {{ item.label }}
                                        </a>
                                    {% endif %}
                                </li>
                            {% endfor %}
                        </ul>
                    </nav>
                    {% ds_sheet_footer %}
                        {% ds_button href=site.cta.url variant="primary" class_="site-mobile__cta" %}
                            {{ site.cta.label }}
                        {% endds_button %}
                    {% endds_sheet_footer %}
                {% endds_sheet_content %}
            {% endds_sheet %}
        </div>
    </div>
</header>
