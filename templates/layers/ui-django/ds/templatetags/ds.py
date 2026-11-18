"""Composants du catalogue AlphaRust pour les gabarits Django/Wagtail.

Même balisage (attributs ``data-slot``) que les composants Astro et Blade ; le style vient de
``static/ds/css`` (voir README du projet). Usage dans un gabarit ::

    {% load ds %}
    {% ds_button variant="primary" %}Enregistrer{% endds_button %}

Les balises à contenu utilisent ``simple_block_tag`` (Django 5.2+), les éléments vides ``simple_tag``.
Les attributs HTML supplémentaires sont passés en arguments nommés ; ``_`` devient ``-``
(``aria_label="x"`` -> ``aria-label="x"``) et ``class_`` devient ``class``.
"""

from django import template
from django.utils.html import format_html, format_html_join

register = template.Library()


def _attrs(slot, data, extra):
    out = {"data-slot": slot}
    for key, value in data.items():
        if value is not None:
            out["data-" + key.replace("_", "-")] = value
    for key, value in extra.items():
        out[key.rstrip("_").replace("_", "-")] = value
    return out


def _flat(attrs):
    """Attributs HTML échappés (format_html), dans l'ordre donné ; True = attribut booléen, False/None = omis."""
    parts = []
    for key, value in attrs.items():
        if value is True:
            parts.append(format_html(" {}", key))
        elif value is not False and value is not None:
            parts.append(format_html(' {}="{}"', key, value))
    return format_html_join("", "{}", ((p,) for p in parts))


def _el(tag, attrs, content=""):
    return format_html("<{tag}{attrs}>{content}</{tag}>", tag=tag, attrs=_flat(attrs), content=content)


def _void(tag, attrs):
    return format_html("<{tag}{attrs}>", tag=tag, attrs=_flat(attrs))

@register.simple_block_tag(name="ds_button")
def ds_button(content, variant="default", size="md", **attrs):
    tag = "a" if "href" in attrs else "button"
    if tag != "a":
        attrs.setdefault("type", "button")
    return _el(tag, _attrs("button", {"variant": variant, "size": size}, attrs), content)


@register.simple_block_tag(name="ds_badge")
def ds_badge(content, variant="default", size="md", **attrs):
    tag = "a" if "href" in attrs else "div"
    return _el(tag, _attrs("badge", {"variant": variant, "size": size}, attrs), content)


@register.simple_block_tag(name="ds_alert_title")
def ds_alert_title(content, **attrs):
    return _el("h5", _attrs("alert-title", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_alert_description")
def ds_alert_description(content, **attrs):
    return _el("p", _attrs("alert-description", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_card")
def ds_card(content, size="default", **attrs):
    return _el("div", _attrs("card", {"size": size}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_card_header")
def ds_card_header(content, **attrs):
    return _el("div", _attrs("card-header", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_card_title")
def ds_card_title(content, **attrs):
    return _el("div", _attrs("card-title", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_card_description")
def ds_card_description(content, **attrs):
    return _el("div", _attrs("card-description", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_card_action")
def ds_card_action(content, **attrs):
    return _el("div", _attrs("card-action", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_card_content")
def ds_card_content(content, **attrs):
    return _el("div", _attrs("card-content", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_card_footer")
def ds_card_footer(content, **attrs):
    return _el("div", _attrs("card-footer", {}, {**{}, **attrs}), content)


@register.simple_tag(name="ds_input")
def ds_input(size="md", **attrs):
    return _void("input", _attrs("input", {"size": size}, {**{}, **attrs}))


@register.simple_block_tag(name="ds_label")
def ds_label(content, size="md", **attrs):
    return _el("label", _attrs("label", {"size": size}, {**{}, **attrs}), content)


@register.simple_tag(name="ds_skeleton")
def ds_skeleton(**attrs):
    return _el("div", _attrs("skeleton", {}, {**{}, **attrs}))


@register.simple_tag(name="ds_spinner")
def ds_spinner(**attrs):
    return _el("span", _attrs("spinner", {}, {**{"role": "status", "aria-label": "Loading"}, **attrs}))


@register.simple_block_tag(name="ds_kbd")
def ds_kbd(content, **attrs):
    return _el("kbd", _attrs("kbd", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_kbd_group")
def ds_kbd_group(content, **attrs):
    return _el("kbd", _attrs("kbd-group", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_table_header")
def ds_table_header(content, **attrs):
    return _el("thead", _attrs("table-header", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_table_body")
def ds_table_body(content, **attrs):
    return _el("tbody", _attrs("table-body", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_table_foot")
def ds_table_foot(content, **attrs):
    return _el("tfoot", _attrs("table-foot", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_table_row")
def ds_table_row(content, **attrs):
    return _el("tr", _attrs("table-row", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_table_head")
def ds_table_head(content, **attrs):
    return _el("th", _attrs("table-head", {}, {**{"scope": "col"}, **attrs}), content)


@register.simple_block_tag(name="ds_table_cell")
def ds_table_cell(content, **attrs):
    return _el("td", _attrs("table-cell", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_table_caption")
def ds_table_caption(content, **attrs):
    return _el("caption", _attrs("table-caption", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_breadcrumb")
def ds_breadcrumb(content, **attrs):
    return _el("nav", _attrs("breadcrumb", {}, {**{"aria-label": "breadcrumb"}, **attrs}), content)


@register.simple_block_tag(name="ds_breadcrumb_list")
def ds_breadcrumb_list(content, **attrs):
    return _el("ol", _attrs("breadcrumb-list", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_breadcrumb_item")
def ds_breadcrumb_item(content, **attrs):
    return _el("li", _attrs("breadcrumb-item", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_breadcrumb_link")
def ds_breadcrumb_link(content, **attrs):
    return _el("a", _attrs("breadcrumb-link", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_breadcrumb_page")
def ds_breadcrumb_page(content, **attrs):
    return _el("span", _attrs("breadcrumb-page", {}, {**{"aria-current": "page"}, **attrs}), content)


@register.simple_block_tag(name="ds_breadcrumb_separator")
def ds_breadcrumb_separator(content, **attrs):
    return _el("li", _attrs("breadcrumb-separator", {}, {**{"role": "presentation", "aria-hidden": "true"}, **attrs}), content or "›")


@register.simple_block_tag(name="ds_breadcrumb_ellipsis")
def ds_breadcrumb_ellipsis(content, **attrs):
    return _el("span", _attrs("breadcrumb-ellipsis", {}, {**{"role": "presentation", "aria-hidden": "true"}, **attrs}), content or "…")


@register.simple_block_tag(name="ds_pagination")
def ds_pagination(content, **attrs):
    return _el("nav", _attrs("pagination", {}, {**{"aria-label": "pagination"}, **attrs}), content)


@register.simple_block_tag(name="ds_pagination_content")
def ds_pagination_content(content, **attrs):
    return _el("ul", _attrs("pagination-content", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_pagination_item")
def ds_pagination_item(content, **attrs):
    return _el("li", _attrs("pagination-item", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_pagination_ellipsis")
def ds_pagination_ellipsis(content, **attrs):
    return _el("span", _attrs("pagination-ellipsis", {}, {**{"aria-hidden": "true"}, **attrs}), content or "…")


@register.simple_block_tag(name="ds_pagination_previous")
def ds_pagination_previous(content, **attrs):
    return _el("a", _attrs("pagination-previous", {}, {**{"aria-label": "Go to previous page"}, **attrs}), content or "‹ Previous")


@register.simple_block_tag(name="ds_pagination_next")
def ds_pagination_next(content, **attrs):
    return _el("a", _attrs("pagination-next", {}, {**{"aria-label": "Go to next page"}, **attrs}), content or "Next ›")


@register.simple_block_tag(name="ds_avatar")
def ds_avatar(content, size="md", **attrs):
    return _el("figure", _attrs("avatar", {"size": size}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_avatar_fallback")
def ds_avatar_fallback(content, **attrs):
    return _el("div", _attrs("avatar-fallback", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_accordion_trigger")
def ds_accordion_trigger(content, **attrs):
    return _el("summary", _attrs("accordion-trigger", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_accordion_content")
def ds_accordion_content(content, **attrs):
    return _el("div", _attrs("accordion-content", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_dialog_header")
def ds_dialog_header(content, **attrs):
    return _el("div", _attrs("dialog-header", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_dialog_title")
def ds_dialog_title(content, **attrs):
    return _el("h2", _attrs("dialog-title", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_dialog_description")
def ds_dialog_description(content, **attrs):
    return _el("p", _attrs("dialog-description", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_dialog_footer")
def ds_dialog_footer(content, **attrs):
    return _el("div", _attrs("dialog-footer", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_tooltip_trigger")
def ds_tooltip_trigger(content, **attrs):
    return _el("span", _attrs("tooltip-trigger", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_tooltip")
def ds_tooltip(content, **attrs):
    return _el("span", _attrs("tooltip", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_tooltip_content")
def ds_tooltip_content(content, side="top", **attrs):
    return _el("span", _attrs("tooltip-content", {"side": side}, {**{"role": "tooltip"}, **attrs}), content)


@register.simple_block_tag(name="ds_tabs_list")
def ds_tabs_list(content, **attrs):
    return _el("div", _attrs("tabs-list", {}, {**{"role": "tablist"}, **attrs}), content)


@register.simple_block_tag(name="ds_native_select_option")
def ds_native_select_option(content, **attrs):
    return _el("option", _attrs("native-select-option", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_native_select_optgroup")
def ds_native_select_optgroup(content, **attrs):
    return _el("optgroup", _attrs("native-select-optgroup", {}, {**{}, **attrs}), content)


@register.simple_block_tag(name="ds_alert")
def ds_alert(content, variant="default", **attrs):
    # error / warning : role="alert" ; sinon role="status"
    attrs.setdefault("role", "alert" if variant in ("error", "warning") else "status")
    return _el("div", _attrs("alert", {"variant": variant}, attrs), content)


@register.simple_tag(name="ds_separator")
def ds_separator(orientation="horizontal", **attrs):
    attrs = {"role": "separator", "aria-orientation": orientation, **attrs}
    return _el("div", _attrs("separator", {}, attrs))


@register.simple_block_tag(name="ds_textarea")
def ds_textarea(content, size="md", default_value="", **attrs):
    return _el("textarea", _attrs("textarea", {"size": size}, attrs), f"{default_value}{content}")


@register.simple_tag(name="ds_checkbox")
def ds_checkbox(id=None, label=None, checked=False, size="md", variant="default", **attrs):
    if id:
        attrs["id"] = id
    attrs["checked"] = bool(checked)
    box = _void("input", {"type": "checkbox", **_attrs("checkbox-input", {}, attrs)})
    text = _el("label", {"for": id, "data-slot": "checkbox-label"} if id else {"data-slot": "checkbox-label"}, label) if label else ""
    return _el("div", {"data-slot": "checkbox", "data-size": size, "data-variant": variant}, box + text)


@register.simple_tag(name="ds_switch")
def ds_switch(id, label=None, checked=False, size="md", variant="default", **attrs):
    aria_label = attrs.pop("aria-label", None) or attrs.pop("aria_label", None) or label or "switch"
    button = _el(
        "button",
        {
            "type": "button",
            "id": id,
            "role": "switch",
            "aria-checked": "true" if checked else "false",
            "aria-label": aria_label,
            **_attrs("switch-button", {"size": size, "variant": variant}, attrs),
        },
        _el("span", {"data-slot": "switch-toggle"}),
    )
    text = _el("label", {"for": id, "data-slot": "switch-label"}, label) if label else ""
    return _el("div", {"data-slot": "switch"}, button + text)


@register.simple_tag(name="ds_progress")
def ds_progress(value=None, max=100, variant="default", **attrs):
    attrs = {"max": max, **attrs}
    if value is not None:
        attrs["value"] = value
    return _el("progress", _attrs("progress", {"variant": variant}, attrs))


@register.simple_block_tag(name="ds_native_select")
def ds_native_select(content, size="md", **attrs):
    select = _el("select", _attrs("native-select", {"size": size}, attrs), content)
    icon = _el("span", {"data-slot": "native-select-icon", "aria-hidden": "true"})
    return _el("div", {"data-slot": "native-select-wrapper"}, select + icon)


@register.simple_tag(name="ds_avatar_image")
def ds_avatar_image(src=None, alt="", **attrs):
    if not src:
        return ""
    return _void("img", {"src": src, "alt": alt, **_attrs("avatar-image", {}, attrs)})


@register.simple_block_tag(name="ds_tabs")
def ds_tabs(content, default_value=None, **attrs):
    return _el("div", _attrs("tabs", {"default": default_value}, attrs), content)


@register.simple_block_tag(name="ds_tabs_trigger")
def ds_tabs_trigger(content, value, default_active=False, **attrs):
    attrs = {
        "type": "button",
        "role": "tab",
        "aria-selected": "true" if default_active else "false",
        "tabindex": 0 if default_active else -1,
        **attrs,
    }
    return _el("button", _attrs("tabs-trigger", {"value": value}, attrs), content)


@register.simple_block_tag(name="ds_tabs_content")
def ds_tabs_content(content, value, default_active=False, **attrs):
    attrs = {"role": "tabpanel", "hidden": not default_active, **attrs}
    return _el("div", _attrs("tabs-content", {"value": value}, attrs), content)


@register.simple_block_tag(name="ds_accordion")
def ds_accordion(content, type="single", default_value=None, **attrs):
    return _el("div", _attrs("accordion", {"type": type, "default": default_value}, attrs), content)


@register.simple_block_tag(name="ds_accordion_item")
def ds_accordion_item(content, value, **attrs):
    return _el("details", _attrs("accordion-item", {"value": value}, attrs), content)


@register.simple_block_tag(name="ds_dialog")
def ds_dialog(content, **attrs):
    return _el("div", _attrs("dialog", {}, attrs), content)


@register.simple_block_tag(name="ds_dialog_trigger")
def ds_dialog_trigger(content, as_child=False, **attrs):
    # `for="id-du-dialogue"` : ouvrir un dialogue situé ailleurs dans la page
    target = attrs.pop("for", None)
    if as_child:
        return _el("span", _attrs("dialog-trigger", {"as-child": True, "for": target}, attrs), content)
    attrs = {"type": "button", "aria-haspopup": "dialog", **attrs}
    return _el("button", _attrs("dialog-trigger", {"for": target}, attrs), content)


@register.simple_block_tag(name="ds_dialog_close")
def ds_dialog_close(content, as_child=False, **attrs):
    if as_child:
        return _el("span", _attrs("dialog-close", {"as-child": True}, attrs), content)
    attrs = {"type": "button", **attrs}
    return _el("button", _attrs("dialog-close", {}, attrs), content)


@register.simple_block_tag(name="ds_dialog_content")
def ds_dialog_content(content, **attrs):
    return _el("dialog", _attrs("dialog-content", {}, attrs), content)


@register.simple_block_tag(name="ds_pagination_link")
def ds_pagination_link(content, is_active=False, **attrs):
    if is_active:
        attrs["aria-current"] = "page"
    return _el("a", _attrs("pagination-link", {}, attrs), content)


@register.simple_block_tag(name="ds_table")
def ds_table(content, **attrs):
    return _el("div", {"data-slot": "table-container"}, _el("table", _attrs("table", {}, attrs), content))
