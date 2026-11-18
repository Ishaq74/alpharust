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
        <nav class="site-nav" aria-label="{{ site.ui.main_nav }}">
            <ul class="site-nav__list">
                {% for item in site.nav %}
                    <li>
                        {% if item.children %}
                            {% ds_dropdown open_on_hover=True class_="site-nav__dropdown" %}
                                {% ds_dropdown_trigger class_="site-nav__link" aria_current=item.current %}
                                    {{ item.label }}
                                    {% site_icon "chevron-down" size=16 %}
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
            {% if site.account %}
                {% ds_button href=site.account.url variant="ghost" size="sm" class_="site-header__account" %}
                    {% site_icon "user" size=18 %}
                    <span>
                        {{ site.account.label }}
                    </span>
                {% endds_button %}
            {% endif %}
            {% ds_button href=site.cta.url variant="primary" size="sm" class_="site-header__cta" %}
                {{ site.cta.label }}
            {% endds_button %}
            <nav class="site-lang" aria-label="{{ site.ui.language }}">
                <ul class="site-lang__list">
                    {% for lang in site.languages %}
                        <li>
                            <a class="site-lang__link" href="{{ lang.url }}" hreflang="{{ lang.code }}" lang="{{ lang.code }}"{% if lang.current %} aria-current="{{ lang.current }}"{% endif %} title="{{ lang.name }}">
                                {% site_icon lang.flag size=16 %}
                                <span>
                                    {{ lang.label }}
                                </span>
                            </a>
                        </li>
                    {% endfor %}
                </ul>
            </nav>
            {% ds_theme_toggle size="sm" aria_label=site.ui.theme %}{% endds_theme_toggle %}
            {% ds_sheet class_="site-header__mobile" %}
                {% ds_sheet_trigger as_child=True %}
                    {% ds_button variant="ghost" size="icon" aria_label=site.ui.open_menu %}
                        {% site_icon "menu" size=22 %}
                    {% endds_button %}
                {% endds_sheet_trigger %}
                {% ds_sheet_content side="right" class_="site-mobile" %}
                    {% ds_sheet_header %}
                        {% ds_sheet_title %}
                            {{ site.name }}
                        {% endds_sheet_title %}
                    {% endds_sheet_header %}
                    <nav class="site-mobile__nav" aria-label="{{ site.ui.mobile_nav }}">
                        <ul class="site-mobile__list">
                            {% for item in site.nav %}
                                <li>
                                    {% if item.children %}
                                        {% ds_collapsible class_="site-mobile__group" %}
                                            {% ds_collapsible_trigger class_="site-mobile__link" %}
                                                {{ item.label }}
                                                {% site_icon "chevron-down" size=16 %}
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
                        {% if site.account %}
                            {% ds_button href=site.account.url variant="outline" class_="site-mobile__cta" %}
                                {% site_icon "user" size=18 %}
                                {{ site.account.label }}
                            {% endds_button %}
                        {% endif %}
                        {% ds_button href=site.cta.url variant="primary" class_="site-mobile__cta" %}
                            {{ site.cta.label }}
                        {% endds_button %}
                    {% endds_sheet_footer %}
                {% endds_sheet_content %}
            {% endds_sheet %}
        </div>
    </div>
</header>
