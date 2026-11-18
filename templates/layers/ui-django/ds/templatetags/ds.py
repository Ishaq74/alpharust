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


# ---------------------------------------------------------------------------------------------------------------
# Composants étendus (alert-dialog, select, sidebar, toast…) : même balisage que les atomes CSS pur d'Astro.
# Générés par tools/dsgen/ext_server.py ; ne pas modifier à la main.

import random  # noqa: E402
import re  # noqa: E402
from urllib.parse import urlencode  # noqa: E402

from django.utils.html import conditional_escape  # noqa: E402
from django.utils.safestring import mark_safe  # noqa: E402


def _x(attrs):
    """Arguments nommés -> attributs HTML (``aria_label`` -> ``aria-label``, ``class_`` -> ``class``)."""
    return {key.rstrip("_").replace("_", "-"): value for key, value in attrs.items()}


def _join(*parts):
    return mark_safe("".join(str(conditional_escape(p)) for p in parts if p is not None))


def _num(v):
    f = float(v)
    return int(f) if f.is_integer() else round(f, 4)


def _slider_values(initial, lo, hi):
    if isinstance(initial, str):
        initial = [float(x) for x in initial.split(",") if x.strip()]
    if not isinstance(initial, (list, tuple)):
        initial = [initial]
    return [_num(min(float(hi), max(float(lo), float(v)))) for v in initial]


def _pct(v, lo, hi):
    span = (float(hi) - float(lo)) or 1
    return _num((float(v) - float(lo)) / span * 100)


def _slider_thumbs(initial, lo, hi, orientation, name):
    values = _slider_values(initial, lo, hi)
    out = []
    for i, v in enumerate(values):
        pct = _pct(v, lo, hi)
        out.append({
            "i": i,
            "v": v,
            "style": f"left: {pct}%" if orientation == "horizontal" else f"bottom: {pct}%",
            "name": None if name is None else (f"{name}[{i}]" if len(values) > 1 else name),
        })
    return out


def _slider_range(initial, lo, hi, orientation):
    values = _slider_values(initial, lo, hi)
    start = _pct(min(values), lo, hi) if len(values) > 1 else 0
    size = _num(_pct(max(values), lo, hi) - start)
    return f"left: {start}%; width: {size}%" if orientation == "horizontal" else f"bottom: {start}%; height: {size}%"


_YOUTUBE = [r"youtube\.com/shorts/([^?&]+)", r"youtube\.com/watch\?v=([^&]+)", r"youtube\.com/embed/([^?&]+)",
            r"youtu\.be/([^?&]+)", r"youtube-nocookie\.com/embed/([^?&]+)"]


def _youtube_embed(src, autoplay, muted, loop, controls):
    if not any(host in src for host in ("youtube.com", "youtu.be", "youtube-nocookie.com")):
        return None
    for pattern in _YOUTUBE:
        m = re.search(pattern, src)
        if m:
            params = {"autoplay": "1" if autoplay else None, "mute": "1" if muted else None, "loop": "1" if loop else None,
                      "playlist": m.group(1) if loop else None, "controls": None if controls else "0"}
            query = urlencode({k: v for k, v in params.items() if v})
            # alpharust-ignore: no-hardcoded-urls
            return "https://www.youtube-nocookie.com/embed/" + m.group(1) + ("?" + query if query else "")
    return None

@register.simple_block_tag(name="ds_alert_dialog")
def ds_alert_dialog(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'alert-dialog'}, **attrs}, content)


@register.simple_block_tag(name="ds_alert_dialog_action")
def ds_alert_dialog_action(content, as_child=False, **attrs):
    attrs = _x(attrs)
    return (_el('div', {'data-slot': 'alert-dialog-action', 'data-as-child': ''}, content) if as_child else _el('button', {**{'type': 'button', 'data-variant': 'primary', 'data-slot': 'alert-dialog-action'}, **attrs}, content))


@register.simple_block_tag(name="ds_alert_dialog_cancel")
def ds_alert_dialog_cancel(content, as_child=False, **attrs):
    attrs = _x(attrs)
    return (_el('div', {'data-slot': 'alert-dialog-cancel', 'data-as-child': ''}, content) if as_child else _el('button', {**{'type': 'button', 'data-variant': 'outline', 'data-slot': 'alert-dialog-cancel'}, **attrs}, content))


@register.simple_block_tag(name="ds_alert_dialog_content")
def ds_alert_dialog_content(content, animation_duration=200, **attrs):
    attrs = _x(attrs)
    return _join(_el('div', {'data-state': 'closed', 'data-slot': 'alert-dialog-backdrop', 'aria-hidden': 'true'}, ""), _el('dialog', {**{'data-state': 'closed', 'role': 'alertdialog', 'style': ('--ds-duration: ' + str(animation_duration) + 'ms'), 'data-slot': 'alert-dialog-content'}, **attrs}, content))


@register.simple_block_tag(name="ds_alert_dialog_description")
def ds_alert_dialog_description(content, **attrs):
    attrs = _x(attrs)
    return _el('p', {**{'data-slot': 'alert-dialog-description'}, **attrs}, content)


@register.simple_block_tag(name="ds_alert_dialog_footer")
def ds_alert_dialog_footer(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'alert-dialog-footer'}, **attrs}, content)


@register.simple_block_tag(name="ds_alert_dialog_header")
def ds_alert_dialog_header(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'alert-dialog-header'}, **attrs}, content)


@register.simple_block_tag(name="ds_alert_dialog_title")
def ds_alert_dialog_title(content, **attrs):
    attrs = _x(attrs)
    return _el('h2', {**{'data-slot': 'alert-dialog-title'}, **attrs}, content)


@register.simple_block_tag(name="ds_alert_dialog_trigger")
def ds_alert_dialog_trigger(content, as_child=False, for_=None, **attrs):
    attrs = _x(attrs)
    return (_el('div', {'data-dialog-for': for_, 'data-slot': 'alert-dialog-trigger', 'data-as-child': ''}, content) if as_child else _el('button', {**{'type': 'button', 'aria-haspopup': 'dialog', 'data-dialog-for': for_, 'data-slot': 'alert-dialog-trigger'}, **attrs}, content))


@register.simple_block_tag(name="ds_aspect_ratio")
def ds_aspect_ratio(content, ratio=1, as_='div', **attrs):
    attrs = _x(attrs)
    pad = round(100 / float(ratio), 4)
    return _el('div', {'style': ('--ds-ratio: ' + str(ratio) + '; padding-bottom: ' + str(pad) + '%'), 'data-slot': 'aspect-ratio-wrapper'}, _el(as_, {**{'data-slot': 'aspect-ratio'}, **attrs}, content))


@register.simple_block_tag(name="ds_button_group")
def ds_button_group(content, orientation='horizontal', **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'group', 'data-orientation': orientation, 'data-slot': 'button-group'}, **attrs}, content)


@register.simple_tag(name="ds_button_group_separator")
def ds_button_group_separator(orientation='vertical', **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'separator', 'aria-orientation': orientation, 'data-orientation': orientation, 'data-slot': 'button-group-separator'}, **attrs}, "")


@register.simple_block_tag(name="ds_button_group_text")
def ds_button_group_text(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'button-group-text'}, **attrs}, content)


@register.simple_block_tag(name="ds_carousel")
def ds_carousel(content, orientation='horizontal', opts='{}', auto_init=True, aria_label=None, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'region', 'aria-roledescription': 'carousel', 'aria-label': (aria_label or 'Carousel'), 'data-axis': ('x' if (orientation == 'horizontal') else 'y'), 'data-opts': opts, 'data-auto-init': ('true' if auto_init else 'false'), 'data-slot': 'carousel'}, **attrs}, content)


@register.simple_block_tag(name="ds_carousel_content")
def ds_carousel_content(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'carousel-content'}, **attrs}, _el('div', {'data-slot': 'carousel-container'}, content))


@register.simple_block_tag(name="ds_carousel_item")
def ds_carousel_item(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'group', 'aria-roledescription': 'slide', 'data-slot': 'carousel-item'}, **attrs}, content)


@register.simple_block_tag(name="ds_carousel_next")
def ds_carousel_next(content, variant='outline', size='icon', **attrs):
    attrs = _x(attrs)
    return _el('button', {**{'type': 'button', 'data-variant': variant, 'data-size': size, 'aria-label': 'Next slide', 'data-slot': 'carousel-next'}, **attrs}, _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="arrow-right"><path d="M5 12h14M13 18l6-6M13 6l6 6"/></svg>'), (content if str(content).strip() else _el('span', {'class': 'sr-only'}, mark_safe('Next slide')))))  # alpharust-ignore: no-hardcoded-urls


@register.simple_block_tag(name="ds_carousel_previous")
def ds_carousel_previous(content, variant='outline', size='icon', **attrs):
    attrs = _x(attrs)
    return _el('button', {**{'type': 'button', 'data-variant': variant, 'data-size': size, 'aria-label': 'Previous slide', 'data-slot': 'carousel-previous'}, **attrs}, _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="arrow-left"><path d="M5 12h14M5 12l6 6M5 12l6-6"/></svg>'), (content if str(content).strip() else _el('span', {'class': 'sr-only'}, mark_safe('Previous slide')))))  # alpharust-ignore: no-hardcoded-urls


@register.simple_block_tag(name="ds_collapsible")
def ds_collapsible(content, default_open=False, disabled=False, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-state': ('open' if default_open else 'closed'), 'data-disabled': ('' if disabled else None), 'data-slot': 'collapsible'}, **attrs}, content)


@register.simple_block_tag(name="ds_collapsible_content")
def ds_collapsible_content(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-state': 'closed', 'hidden': '', 'data-slot': 'collapsible-content'}, **attrs}, content)


@register.simple_block_tag(name="ds_collapsible_trigger")
def ds_collapsible_trigger(content, as_child=False, **attrs):
    attrs = _x(attrs)
    return (_el('div', {'aria-expanded': 'false', 'data-slot': 'collapsible-trigger', 'data-as-child': ''}, content) if as_child else _el('button', {**{'type': 'button', 'aria-expanded': 'false', 'data-slot': 'collapsible-trigger'}, **attrs}, content))


@register.simple_block_tag(name="ds_container")
def ds_container(content, as_='div', bleed=False, size='lg', gutter='md', padding_y='none', background='none', border='none', rounded='none', min_height='none', layout=None, centered=False, **attrs):
    attrs = _x(attrs)
    layout_v = "centered" if centered else (layout or "stack")
    use_wrapper = bool(bleed) or layout_v == "split"
    return (_el(as_, {**{'data-background': background, 'data-padding-y': padding_y, 'data-min-height': min_height, 'data-border': border, 'data-rounded': rounded, 'data-bleed': '', 'data-slot': 'container'}, **attrs}, _el('div', {'data-container-inner': '', 'data-size': size, 'data-gutter': gutter, 'data-layout': layout_v}, _join(_el('div', {'data-container-main': ''}, content), _el('div', {'data-container-media': ''}, "")))) if use_wrapper else _el(as_, {**{'data-size': size, 'data-gutter': gutter, 'data-background': background, 'data-padding-y': padding_y, 'data-min-height': min_height, 'data-border': border, 'data-rounded': rounded, 'data-layout': layout_v, 'data-slot': 'container'}, **attrs}, content))


@register.simple_block_tag(name="ds_dropdown")
def ds_dropdown(content, open_on_hover=False, close_delay=200, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-open-on-hover': ('true' if open_on_hover else None), 'data-close-delay': close_delay, 'data-slot': 'dropdown'}, **attrs}, content)


@register.simple_block_tag(name="ds_dropdown_content")
def ds_dropdown_content(content, side='bottom', align='start', side_offset=4, animation_duration=150, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'menu', 'data-side': side, 'data-align': align, 'data-state': 'closed', 'tabindex': '-1', 'aria-orientation': 'vertical', 'hidden': '', 'style': ('--ds-offset: ' + str(side_offset) + 'px; --ds-duration: ' + str(animation_duration) + 'ms'), 'data-slot': 'dropdown-content'}, **attrs}, content)


@register.simple_block_tag(name="ds_dropdown_item")
def ds_dropdown_item(content, inset=False, disabled=False, as_='div', **attrs):
    attrs = _x(attrs)
    return _el(as_, {**{'role': 'menuitem', 'tabindex': ('-1' if disabled else '0'), 'aria-disabled': ('true' if disabled else None), 'data-disabled': ('true' if disabled else None), 'data-inset': ('' if inset else None), 'data-slot': 'dropdown-item'}, **attrs}, content)


@register.simple_block_tag(name="ds_dropdown_label")
def ds_dropdown_label(content, inset=False, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-inset': ('' if inset else None), 'data-slot': 'dropdown-label'}, **attrs}, content)


@register.simple_tag(name="ds_dropdown_separator")
def ds_dropdown_separator(**attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'separator', 'aria-orientation': 'horizontal', 'data-slot': 'dropdown-separator'}, **attrs}, "")


@register.simple_block_tag(name="ds_dropdown_shortcut")
def ds_dropdown_shortcut(content, **attrs):
    attrs = _x(attrs)
    return _el('span', {**{'data-dropdown-shortcut': ''}, **attrs}, content)


@register.simple_block_tag(name="ds_dropdown_sub")
def ds_dropdown_sub(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'dropdown-sub'}, **attrs}, content)


@register.simple_block_tag(name="ds_dropdown_sub_content")
def ds_dropdown_sub_content(content, animation_duration=150, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-sub-content': '', 'role': 'menu', 'data-side': 'right', 'data-align': 'start', 'data-state': 'closed', 'tabindex': '-1', 'aria-orientation': 'vertical', 'hidden': '', 'style': ('--ds-duration: ' + str(animation_duration) + 'ms'), 'data-slot': 'dropdown-content'}, **attrs}, content)


@register.simple_block_tag(name="ds_dropdown_sub_trigger")
def ds_dropdown_sub_trigger(content, inset=False, disabled=False, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'menuitem', 'aria-haspopup': 'true', 'aria-expanded': 'false', 'tabindex': ('-1' if disabled else '0'), 'data-disabled': ('true' if disabled else None), 'data-inset': ('' if inset else None), 'data-sub-trigger': '', 'data-slot': 'dropdown-item'}, **attrs}, _join(content, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="chevron-right"><path d="m9 6 6 6-6 6"/></svg>')))  # alpharust-ignore: no-hardcoded-urls


@register.simple_block_tag(name="ds_dropdown_trigger")
def ds_dropdown_trigger(content, as_child=False, **attrs):
    attrs = _x(attrs)
    return (_el('div', {'data-slot': 'dropdown-trigger', 'data-as-child': '', 'aria-haspopup': 'true', 'aria-expanded': 'false'}, content) if as_child else _el('button', {**{'type': 'button', 'aria-haspopup': 'true', 'aria-expanded': 'false', 'data-state': 'closed', 'data-slot': 'dropdown-trigger'}, **attrs}, content))


@register.simple_block_tag(name="ds_dropzone")
def ds_dropzone(content, is_uploading=False, id=None, **attrs):
    attrs = _x(attrs)
    return _el('label', {'id': id, 'tabindex': '0', 'data-slot': 'dropzone', 'data-drag-active': 'false', 'data-has-files': 'false', 'data-is-uploading': ('true' if is_uploading else 'false')}, _join((content if str(content).strip() else _join(_el('div', {'data-slot': 'dropzone-upload-indicator'}, _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="2.5rem" height="2.5rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="cloud-upload"><path d="M7 18a4.6 4.4 0 0 1 0-9 5 4.5 0 0 1 11 2h1a3.5 3.5 0 0 1 0 7h-1"/><path d="m9 15 3-3 3 3M12 12v9"/></svg>'), mark_safe('<p>Click to upload or drag and drop</p>'))), _el('div', {'hidden': '', 'data-slot': 'dropzone-loading-indicator'}, _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="2.5rem" height="2.5rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="loader"><path d="M12 3a9 9 0 1 0 9 9"/></svg>'), mark_safe('<p>Uploading file(s)...</p>'))), _el('div', {'aria-live': 'polite', 'aria-label': 'Uploaded files', 'data-slot': 'dropzone-files-list'}, ""))), _void('input', {**{'tabindex': '-1', 'type': 'file', 'class': 'sr-only'}, **attrs})))  # alpharust-ignore: no-hardcoded-urls


@register.simple_tag(name="ds_dropzone_files_list")
def ds_dropzone_files_list(**attrs):
    attrs = _x(attrs)
    return _el('div', {**{'aria-live': 'polite', 'aria-label': 'Uploaded files', 'data-slot': 'dropzone-files-list'}, **attrs}, "")


@register.simple_block_tag(name="ds_dropzone_loading_indicator")
def ds_dropzone_loading_indicator(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {'hidden': '', 'data-slot': 'dropzone-loading-indicator'}, (content if str(content).strip() else _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="2.5rem" height="2.5rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="loader"><path d="M12 3a9 9 0 1 0 9 9"/></svg>'), mark_safe('<p>Uploading file(s)...</p>'))))  # alpharust-ignore: no-hardcoded-urls


@register.simple_block_tag(name="ds_dropzone_upload_indicator")
def ds_dropzone_upload_indicator(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {'data-slot': 'dropzone-upload-indicator'}, (content if str(content).strip() else _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="2.5rem" height="2.5rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="cloud-upload"><path d="M7 18a4.6 4.4 0 0 1 0-9 5 4.5 0 0 1 11 2h1a3.5 3.5 0 0 1 0 7h-1"/><path d="m9 15 3-3 3 3M12 12v9"/></svg>'), mark_safe('<p>Click to upload or drag and drop</p>'))))  # alpharust-ignore: no-hardcoded-urls


@register.simple_tag(name="ds_image")
def ds_image(src=None, alt='', **attrs):
    attrs = _x(attrs)
    return (_void('img', {**{'src': src, 'alt': alt, 'loading': 'lazy', 'decoding': 'async', 'data-slot': 'image'}, **attrs}) if src else "")


@register.simple_block_tag(name="ds_input_group")
def ds_input_group(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'group', 'data-slot': 'input-group'}, **attrs}, content)


@register.simple_block_tag(name="ds_input_group_addon")
def ds_input_group_addon(content, align='inline-start', **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'group', 'data-align': align, 'data-slot': 'input-group-addon'}, **attrs}, content)


@register.simple_block_tag(name="ds_input_group_button")
def ds_input_group_button(content, type='button', variant='ghost', size='sm', **attrs):
    attrs = _x(attrs)
    return _el('button', {**{'type': type, 'data-size': size, 'data-input-group-button': 'true', 'data-slot': 'button', 'data-variant': variant}, **attrs}, content)


@register.simple_tag(name="ds_input_group_input")
def ds_input_group_input(size='md', **attrs):
    attrs = _x(attrs)
    return _void('input', {**{'data-slot': 'input-group-control', 'data-size': size}, **attrs})


@register.simple_block_tag(name="ds_input_group_text")
def ds_input_group_text(content, **attrs):
    attrs = _x(attrs)
    return _el('span', {**{'data-input-group-text': ''}, **attrs}, content)


@register.simple_tag(name="ds_input_group_textarea")
def ds_input_group_textarea(size='md', default_value='', **attrs):
    attrs = _x(attrs)
    return _el('textarea', {**{'data-slot': 'input-group-control', 'data-size': size}, **attrs}, default_value)


@register.simple_block_tag(name="ds_input_otp")
def ds_input_otp(content, max_length=6, value=None, default_value=None, disabled=False, pattern=None, name=None, id=None, required=False, **attrs):
    attrs = _x(attrs)
    pattern_str = pattern or "^\\d+$"
    input_mode = "numeric" if ("\\d" in pattern_str or "0-9" in pattern_str) and not any(s in pattern_str for s in ("a-z", "A-Z", "\\w")) else "text"
    initial = value or default_value or ""
    return _el('div', {**{'data-max-length': max_length, 'data-value': initial, 'data-disabled': ('true' if disabled else 'false'), 'data-pattern': pattern_str, 'role': 'group', 'aria-label': ('OTP input, ' + str(max_length) + ' digits'), 'data-slot': 'input-otp'}, **attrs}, _join(_void('input', {'autocomplete': 'one-time-code', 'inputmode': input_mode, 'name': name, 'id': id, 'class': 'sr-only', 'required': ('' if required else None), 'maxlength': max_length, 'value': initial, 'disabled': ('' if disabled else None), 'aria-label': ('OTP, ' + str(max_length)), 'data-otp-hidden-input': ''}), content))


@register.simple_block_tag(name="ds_input_otp_group")
def ds_input_otp_group(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'input-otp-group'}, **attrs}, content)


@register.simple_tag(name="ds_input_otp_separator")
def ds_input_otp_separator(**attrs):
    attrs = _x(attrs)
    return _el('div', {**{'aria-hidden': 'true', 'role': 'separator', 'data-slot': 'input-otp-separator'}, **attrs}, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.5rem" height="1.5rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true" data-icon="minus"><path d="M5 12h14"/></svg>'))  # alpharust-ignore: no-hardcoded-urls


@register.simple_tag(name="ds_input_otp_slot")
def ds_input_otp_slot(size='md', index=None, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-otp-slot': '', 'data-index': index, 'data-size': size, 'aria-hidden': 'true', 'data-slot': 'input-otp-slot'}, **attrs}, mark_safe('<span data-otp-char></span><div data-otp-caret hidden><div data-otp-caret-line></div></div>'))


@register.simple_block_tag(name="ds_item")
def ds_item(content, variant='default', size='default', as_='div', **attrs):
    attrs = _x(attrs)
    return _el(as_, {**{'data-variant': variant, 'data-size': size, 'data-slot': 'item'}, **attrs}, content)


@register.simple_block_tag(name="ds_item_actions")
def ds_item_actions(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'item-actions'}, **attrs}, content)


@register.simple_block_tag(name="ds_item_content")
def ds_item_content(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'item-content'}, **attrs}, content)


@register.simple_block_tag(name="ds_item_description")
def ds_item_description(content, **attrs):
    attrs = _x(attrs)
    return _el('p', {**{'data-slot': 'item-description'}, **attrs}, content)


@register.simple_block_tag(name="ds_item_footer")
def ds_item_footer(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'item-footer'}, **attrs}, content)


@register.simple_block_tag(name="ds_item_group")
def ds_item_group(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'list', 'data-slot': 'item-group'}, **attrs}, content)


@register.simple_block_tag(name="ds_item_header")
def ds_item_header(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'item-header'}, **attrs}, content)


@register.simple_block_tag(name="ds_item_media")
def ds_item_media(content, variant='default', **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-variant': variant, 'data-slot': 'item-media'}, **attrs}, content)


@register.simple_tag(name="ds_item_separator")
def ds_item_separator(orientation='horizontal', **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'none', 'data-orientation': orientation, 'data-slot': 'item-separator'}, **attrs}, "")


@register.simple_block_tag(name="ds_item_title")
def ds_item_title(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'item-title'}, **attrs}, content)


@register.simple_block_tag(name="ds_popover")
def ds_popover(content, open_on_hover=False, close_delay=200, default_open=False, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-open-on-hover': ('true' if open_on_hover else None), 'data-close-delay': close_delay, 'data-state': ('open' if default_open else 'closed'), 'data-default-open': ('true' if default_open else None), 'data-slot': 'popover'}, **attrs}, content)


@register.simple_block_tag(name="ds_popover_content")
def ds_popover_content(content, side='bottom', align='center', side_offset=4, animation_duration=150, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-side': side, 'data-align': align, 'data-side-offset': side_offset, 'data-state': 'closed', 'role': 'dialog', 'tabindex': '-1', 'hidden': '', 'style': ('--ds-offset: ' + str(side_offset) + 'px; --ds-duration: ' + str(animation_duration) + 'ms'), 'data-slot': 'popover-content'}, **attrs}, content)


@register.simple_block_tag(name="ds_popover_description")
def ds_popover_description(content, **attrs):
    attrs = _x(attrs)
    return _el('p', {**{'data-slot': 'popover-description'}, **attrs}, content)


@register.simple_block_tag(name="ds_popover_header")
def ds_popover_header(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'popover-header'}, **attrs}, content)


@register.simple_block_tag(name="ds_popover_title")
def ds_popover_title(content, **attrs):
    attrs = _x(attrs)
    return _el('h3', {**{'data-slot': 'popover-title'}, **attrs}, content)


@register.simple_block_tag(name="ds_popover_trigger")
def ds_popover_trigger(content, as_child=False, **attrs):
    attrs = _x(attrs)
    return (_el('div', {'data-slot': 'popover-trigger', 'data-as-child': ''}, content) if as_child else _el('button', {**{'type': 'button', 'aria-haspopup': 'dialog', 'aria-expanded': 'false', 'data-state': 'closed', 'data-slot': 'popover-trigger'}, **attrs}, content))


@register.simple_block_tag(name="ds_prose")
def ds_prose(content, size='default', **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-size': size, 'data-slot': 'prose'}, **attrs}, content)


@register.simple_block_tag(name="ds_radio_group")
def ds_radio_group(content, value=None, default_value=None, legend=None, disabled=False, orientation='vertical', **attrs):
    attrs = _x(attrs)
    return _el('fieldset', {**{'role': 'radiogroup', 'data-value': (value or default_value), 'data-orientation': orientation, 'disabled': ('' if disabled else None), 'data-slot': 'radio-group'}, **attrs}, _join((_el('legend', {'class': 'sr-only'}, legend) if legend else ""), content))


@register.simple_tag(name="ds_radio_group_item")
def ds_radio_group_item(value=None, name=None, disabled=False, checked=False, id=None, size='md', variant='default', **attrs):
    attrs = _x(attrs)
    return _el('div', {'data-size': size, 'data-slot': 'radio-group-item-wrapper'}, _join(_void('input', {**{'type': 'radio', 'name': name, 'id': id, 'value': value, 'checked': ('' if checked else None), 'disabled': ('' if disabled else None), 'data-slot': 'radio-group-item'}, **attrs}), _el('span', {'data-variant': variant, 'aria-hidden': 'true', 'data-slot': 'radio-group-item-control'}, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true" data-icon="circle"><circle cx="12" cy="12" r="6"/></svg>'))))  # alpharust-ignore: no-hardcoded-urls


@register.simple_block_tag(name="ds_select")
def ds_select(content, name=None, default_value=None, required=False, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-name': name, 'data-value': default_value, 'data-required': ('true' if required else None), 'data-slot': 'select'}, **attrs}, _join(content, (_void('input', {'type': 'hidden', 'name': name, 'value': (default_value or ''), 'required': ('' if required else None), 'data-select-input': ''}) if name else "")))


@register.simple_block_tag(name="ds_select_content")
def ds_select_content(content, side='bottom', align='start', size='md', side_offset=4, animation_duration=150, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'listbox', 'data-side': side, 'data-align': align, 'data-size': size, 'data-state': 'closed', 'tabindex': '-1', 'hidden': '', 'style': ('--ds-offset: ' + str(side_offset) + 'px; --ds-duration: ' + str(animation_duration) + 'ms'), 'data-slot': 'select-content'}, **attrs}, _el('div', {'data-select-inner': ''}, content))


@register.simple_block_tag(name="ds_select_group")
def ds_select_group(content, **attrs):
    attrs = _x(attrs)
    return content


@register.simple_block_tag(name="ds_select_item")
def ds_select_item(content, value=None, disabled=False, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-value': value, 'data-disabled': ('true' if disabled else None), 'aria-disabled': ('true' if disabled else None), 'aria-selected': 'false', 'role': 'option', 'tabindex': '-1', 'data-slot': 'select-item'}, **attrs}, _join(_el('span', {}, content), _el('span', {'data-select-check': ''}, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="check"><path d="m5 12 5 5L20 7"/></svg>'))))  # alpharust-ignore: no-hardcoded-urls


@register.simple_block_tag(name="ds_select_label")
def ds_select_label(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'select-label'}, **attrs}, content)


@register.simple_tag(name="ds_select_search")
def ds_select_search(placeholder='Search...', empty_text='No results found.', **attrs):
    attrs = _x(attrs)
    return _join(_el('div', {'data-slot': 'select-search-wrapper'}, _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="search"><circle cx="10" cy="10" r="7"/><path d="m21 21-6-6"/></svg>'), _void('input', {**{'type': 'text', 'placeholder': placeholder, 'autocomplete': 'off', 'aria-label': placeholder, 'data-slot': 'select-search'}, **attrs}))), _el('div', {'hidden': '', 'data-slot': 'select-empty'}, empty_text))  # alpharust-ignore: no-hardcoded-urls


@register.simple_tag(name="ds_select_separator")
def ds_select_separator(**attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'separator', 'data-slot': 'select-separator'}, **attrs}, "")


@register.simple_block_tag(name="ds_select_trigger")
def ds_select_trigger(content, size='md', **attrs):
    attrs = _x(attrs)
    return _el('button', {**{'type': 'button', 'role': 'combobox', 'aria-haspopup': 'listbox', 'aria-label': 'Select field', 'aria-expanded': 'false', 'data-state': 'closed', 'data-size': size, 'data-slot': 'select-trigger'}, **attrs}, _join(content, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="chevron-down"><path d="m6 9 6 6 6-6"/></svg>')))  # alpharust-ignore: no-hardcoded-urls


@register.simple_tag(name="ds_select_value")
def ds_select_value(placeholder='select', **attrs):
    attrs = _x(attrs)
    return _el('span', {**{'data-placeholder': placeholder, 'data-slot': 'select-value'}, **attrs}, placeholder)


@register.simple_block_tag(name="ds_sheet")
def ds_sheet(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'sheet'}, **attrs}, content)


@register.simple_block_tag(name="ds_sheet_close")
def ds_sheet_close(content, as_child=False, **attrs):
    attrs = _x(attrs)
    return (_el('span', {**{'data-slot': 'dialog-close', 'data-as-child': ''}, **attrs}, content) if as_child else _el('button', {**{'type': 'button', 'data-slot': 'dialog-close'}, **attrs}, content))


@register.simple_block_tag(name="ds_sheet_content")
def ds_sheet_content(content, side='right', **attrs):
    attrs = _x(attrs)
    return _join(_el('div', {'data-state': 'closed', 'data-slot': 'dialog-backdrop', 'aria-hidden': 'true'}, ""), _el('dialog', {**{'data-side': side, 'data-state': 'closed', 'data-close-duration': '300', 'data-slot': 'sheet-content'}, **attrs}, _join(content, _el('button', {'type': 'button', 'data-slot': 'sheet-close', 'aria-label': 'Close dialog'}, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M18 6 6 18M6 6l12 12"/></svg>')))))  # alpharust-ignore: no-hardcoded-urls


@register.simple_block_tag(name="ds_sheet_description")
def ds_sheet_description(content, **attrs):
    attrs = _x(attrs)
    return _el('p', {**{'data-slot': 'sheet-description'}, **attrs}, content)


@register.simple_block_tag(name="ds_sheet_footer")
def ds_sheet_footer(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'sheet-footer'}, **attrs}, content)


@register.simple_block_tag(name="ds_sheet_header")
def ds_sheet_header(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-slot': 'sheet-header'}, **attrs}, content)


@register.simple_block_tag(name="ds_sheet_title")
def ds_sheet_title(content, **attrs):
    attrs = _x(attrs)
    return _el('h2', {**{'data-slot': 'sheet-title'}, **attrs}, content)


@register.simple_block_tag(name="ds_sheet_trigger")
def ds_sheet_trigger(content, for_=None, as_child=False, **attrs):
    attrs = _x(attrs)
    return (_el('span', {**{'data-slot': 'dialog-trigger', 'data-as-child': '', 'data-for': for_}, **attrs}, content) if as_child else _el('button', {**{'type': 'button', 'aria-haspopup': 'dialog', 'data-slot': 'dialog-trigger', 'data-for': for_}, **attrs}, content))


@register.simple_block_tag(name="ds_sidebar")
def ds_sidebar(content, side='left', variant='sidebar', collapsible='offcanvas', **attrs):
    attrs = _x(attrs)
    return (_el('div', {**{'data-collapsible-mode': 'none', 'data-slot': 'sidebar'}, **attrs}, content) if (collapsible == 'none') else _join(_el('div', {'data-state': 'expanded', 'data-collapsible': '', 'data-collapsible-mode': collapsible, 'data-variant': variant, 'data-side': side, 'data-slot': 'sidebar'}, _join(_el('div', {'data-slot': 'sidebar-gap'}, ""), _el('div', {**{'data-slot': 'sidebar-container'}, **attrs}, _el('div', {'data-slot': 'sidebar-inner'}, content)))), _el('div', {'data-slot': 'sheet', 'data-sidebar-mobile': 'true'}, _join(_el('div', {'data-state': 'closed', 'data-slot': 'dialog-backdrop', 'aria-hidden': 'true'}, ""), _el('dialog', {'data-side': side, 'data-state': 'closed', 'data-close-duration': '300', 'data-sidebar': 'mobile', 'data-slot': 'sheet-content'}, _join(_el('div', {'class': 'sr-only', 'data-slot': 'sheet-header'}, _join(_el('h2', {'data-slot': 'sheet-title'}, mark_safe('Sidebar')), _el('p', {'data-slot': 'sheet-description'}, mark_safe('Mobile navigation sidebar')))), _el('div', {'data-sidebar-mobile-inner': ''}, content), _el('button', {'type': 'button', 'data-slot': 'sheet-close', 'aria-label': 'Close dialog'}, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M18 6 6 18M6 6l12 12"/></svg>'))))))))  # alpharust-ignore: no-hardcoded-urls


@register.simple_block_tag(name="ds_sidebar_content")
def ds_sidebar_content(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-sidebar': 'content', 'data-slot': 'sidebar-content'}, **attrs}, content)


@register.simple_block_tag(name="ds_sidebar_footer")
def ds_sidebar_footer(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-sidebar': 'footer', 'data-slot': 'sidebar-footer'}, **attrs}, content)


@register.simple_block_tag(name="ds_sidebar_group")
def ds_sidebar_group(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-sidebar': 'group', 'data-slot': 'sidebar-group'}, **attrs}, content)


@register.simple_block_tag(name="ds_sidebar_group_content")
def ds_sidebar_group_content(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-sidebar': 'group-content', 'data-slot': 'sidebar-group-content'}, **attrs}, content)


@register.simple_block_tag(name="ds_sidebar_group_label")
def ds_sidebar_group_label(content, as_child=False, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-sidebar': 'group-label', 'data-as-child': ('' if as_child else None), 'data-slot': 'sidebar-group-label'}, **attrs}, content)


@register.simple_block_tag(name="ds_sidebar_header")
def ds_sidebar_header(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-sidebar': 'header', 'data-slot': 'sidebar-header'}, **attrs}, content)


@register.simple_tag(name="ds_sidebar_input")
def ds_sidebar_input(size='md', **attrs):
    attrs = _x(attrs)
    return _void('input', {**{'data-sidebar': 'input', 'data-slot': 'sidebar-input', 'data-size': size}, **attrs})


@register.simple_block_tag(name="ds_sidebar_inset")
def ds_sidebar_inset(content, **attrs):
    attrs = _x(attrs)
    return _el('main', {**{'id': 'main', 'data-slot': 'sidebar-inset'}, **attrs}, content)


@register.simple_block_tag(name="ds_sidebar_menu")
def ds_sidebar_menu(content, **attrs):
    attrs = _x(attrs)
    return _el('ul', {**{'data-sidebar': 'menu', 'data-slot': 'sidebar-menu'}, **attrs}, content)


@register.simple_block_tag(name="ds_sidebar_menu_action")
def ds_sidebar_menu_action(content, show_on_hover=False, variant='ghost', size='icon-sm', **attrs):
    attrs = _x(attrs)
    return _el('button', {**{'type': 'button', 'data-variant': variant, 'data-size': size, 'data-show-on-hover': ('' if show_on_hover else None), 'data-sidebar': 'menu-action', 'data-slot': 'sidebar-menu-action'}, **attrs}, content)


@register.simple_block_tag(name="ds_sidebar_menu_badge")
def ds_sidebar_menu_badge(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-sidebar': 'menu-badge', 'data-slot': 'sidebar-menu-badge'}, **attrs}, content)


@register.simple_block_tag(name="ds_sidebar_menu_button")
def ds_sidebar_menu_button(content, as_child=False, is_active=False, tooltip=None, variant='default', size=None, **attrs):
    attrs = _x(attrs)
    tag_v = "a" if "href" in attrs else "button"
    current = "page" if is_active and "href" in attrs else None
    return (_el('span', {'data-sidebar-tooltip': 'true', 'data-slot': 'tooltip'}, _join(_el('span', {'data-slot': 'tooltip-trigger'}, (_el('div', {'data-slot': 'sidebar-menu-button', 'data-sidebar': 'menu-button', 'data-size': (size or 'default'), 'data-variant': variant, 'data-active': ('true' if is_active else 'false'), 'data-as-child': ''}, content) if as_child else _el(tag_v, {**{'type': ('button' if (tag_v == 'button') else None), 'data-slot': 'sidebar-menu-button', 'data-sidebar': 'menu-button', 'data-size': (size or 'default'), 'data-variant': variant, 'data-active': ('true' if is_active else 'false'), 'aria-current': current}, **attrs}, content))), _el('span', {'role': 'tooltip', 'data-slot': 'tooltip-content', 'data-side': 'right'}, tooltip))) if tooltip else (_el('div', {'data-slot': 'sidebar-menu-button', 'data-sidebar': 'menu-button', 'data-size': (size or 'default'), 'data-variant': variant, 'data-active': ('true' if is_active else 'false'), 'data-as-child': ''}, content) if as_child else _el(tag_v, {**{'type': ('button' if (tag_v == 'button') else None), 'data-slot': 'sidebar-menu-button', 'data-sidebar': 'menu-button', 'data-size': (size or 'default'), 'data-variant': variant, 'data-active': ('true' if is_active else 'false'), 'aria-current': current}, **attrs}, content)))


@register.simple_block_tag(name="ds_sidebar_menu_item")
def ds_sidebar_menu_item(content, **attrs):
    attrs = _x(attrs)
    return _el('li', {**{'data-sidebar': 'menu-item', 'data-slot': 'sidebar-menu-item'}, **attrs}, content)


@register.simple_tag(name="ds_sidebar_menu_skeleton")
def ds_sidebar_menu_skeleton(show_icon=False, width=None, **attrs):
    attrs = _x(attrs)
    skeleton_width = width or f"{random.randint(50, 89)}%"
    return _el('div', {**{'data-sidebar': 'menu-skeleton', 'data-slot': 'sidebar-menu-skeleton'}, **attrs}, _join((_el('div', {'data-sidebar': 'menu-skeleton-icon', 'data-slot': 'skeleton'}, "") if show_icon else ""), _el('div', {'data-sidebar': 'menu-skeleton-text', 'style': ('--skeleton-width: ' + str(skeleton_width)), 'data-slot': 'skeleton'}, "")))


@register.simple_block_tag(name="ds_sidebar_menu_sub")
def ds_sidebar_menu_sub(content, **attrs):
    attrs = _x(attrs)
    return _el('ul', {**{'data-sidebar': 'menu-sub', 'data-slot': 'sidebar-menu-sub'}, **attrs}, content)


@register.simple_block_tag(name="ds_sidebar_menu_sub_button")
def ds_sidebar_menu_sub_button(content, size=None, is_active=False, **attrs):
    attrs = _x(attrs)
    return _el('a', {**{'data-sidebar': 'menu-sub-button', 'data-size': (size or 'md'), 'data-active': ('true' if is_active else 'false'), 'aria-current': ('page' if is_active else None), 'data-slot': 'sidebar-menu-sub-button'}, **attrs}, content)


@register.simple_block_tag(name="ds_sidebar_menu_sub_item")
def ds_sidebar_menu_sub_item(content, **attrs):
    attrs = _x(attrs)
    return _el('li', {**{'data-sidebar': 'menu-sub-item', 'data-slot': 'sidebar-menu-sub-item'}, **attrs}, content)


@register.simple_block_tag(name="ds_sidebar_provider")
def ds_sidebar_provider(content, default_open=True, keyboard_shortcut='b', **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-state': ('expanded' if default_open else 'collapsed'), 'data-mobile-open': 'false', 'data-keyboard-shortcut': keyboard_shortcut, 'style': '--sidebar-width: 18rem; --sidebar-width-icon: 3.5rem;', 'data-slot': 'sidebar-provider'}, **attrs}, content)


@register.simple_tag(name="ds_sidebar_rail")
def ds_sidebar_rail(**attrs):
    attrs = _x(attrs)
    return _el('button', {**{'type': 'button', 'data-sidebar': 'rail', 'aria-label': 'Toggle Sidebar', 'tabindex': '-1', 'data-slot': 'sidebar-rail'}, **attrs}, "")


@register.simple_tag(name="ds_sidebar_separator")
def ds_sidebar_separator(orientation='horizontal', **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'role': 'none', 'data-orientation': orientation, 'data-sidebar': 'separator', 'data-slot': 'sidebar-separator'}, **attrs}, "")


@register.simple_tag(name="ds_sidebar_trigger")
def ds_sidebar_trigger(size='icon-sm', variant='ghost', **attrs):
    attrs = _x(attrs)
    return _el('button', {**{'type': 'button', 'data-variant': variant, 'data-size': size, 'data-sidebar': 'trigger', 'aria-label': 'Toggle Sidebar', 'aria-expanded': 'true', 'data-slot': 'sidebar-trigger'}, **attrs}, _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.25em" height="1.25em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="layout-sidebar"><rect x="4" y="4" width="16" height="16" rx="2"/><path d="M9 4v16"/></svg>'), mark_safe('<span class="sr-only">Toggle Sidebar</span>')))  # alpharust-ignore: no-hardcoded-urls


@register.simple_tag(name="ds_slider")
def ds_slider(default_value=0, value=None, min=0, max=100, step=1, large_step=10, orientation='horizontal', disabled=False, name=None, variant='default', aria_label=None, aria_labelledby=None, **attrs):
    attrs = _x(attrs)
    thumbs = _slider_thumbs(value if value is not None else default_value, min, max, orientation, name)
    range_style = _slider_range(value if value is not None else default_value, min, max, orientation)
    return _el('div', {**{'data-orientation': orientation, 'data-disabled': ('' if disabled else None), 'data-variant': variant, 'role': 'group', 'aria-labelledby': aria_labelledby, 'data-min': min, 'data-max': max, 'data-step': step, 'data-large-step': large_step, 'data-slot': 'slider'}, **attrs}, _el('div', {'data-slot': 'slider-control', 'data-orientation': orientation}, _join(_el('div', {'data-slot': 'slider-track', 'data-orientation': orientation}, _el('div', {'data-slot': 'slider-range', 'data-orientation': orientation, 'style': range_style}, "")), _join(*(_el('div', {'data-slot': 'slider-thumb', 'data-index': t["i"], 'data-orientation': orientation, 'style': t["style"], 'tabindex': ('-1' if disabled else '0'), 'role': 'slider', 'aria-label': aria_label, 'aria-labelledby': aria_labelledby, 'aria-valuemin': min, 'aria-valuemax': max, 'aria-valuenow': t["v"], 'aria-orientation': orientation, 'aria-disabled': ('true' if disabled else 'false')}, _void('input', {'type': 'range', 'min': min, 'max': max, 'step': step, 'value': t["v"], 'name': t["name"], 'disabled': ('' if disabled else None), 'tabindex': '-1', 'aria-hidden': 'true', 'class': 'sr-only'})) for t in thumbs)))))


@register.simple_block_tag(name="ds_theme_toggle")
def ds_theme_toggle(content, variant='outline', size='md', aria_label='Toggle theme', **attrs):
    attrs = _x(attrs)
    return _el('button', {**{'type': 'button', 'data-state': 'off', 'data-variant': variant, 'data-size': size, 'aria-pressed': 'false', 'aria-label': aria_label, 'data-slot': 'theme-toggle'}, **attrs}, (content if str(content).strip() else _el('span', {'data-theme-icon-wrapper': ''}, _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.25rem" height="1.25rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-theme-icon="light"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/></svg>'), mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.25rem" height="1.25rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-theme-icon="dark"><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9z"/></svg>')))))  # alpharust-ignore: no-hardcoded-urls


@register.simple_block_tag(name="ds_toast_description")
def ds_toast_description(content, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-toast-description': '', 'data-slot': 'toast-description'}, **attrs}, content)


@register.simple_block_tag(name="ds_toast_item")
def ds_toast_item(content, variant='default', **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-variant': variant, 'data-state': 'open', 'role': ('alert' if ((variant == 'error') or (variant == 'warning')) else 'status'), 'aria-live': ('assertive' if ((variant == 'error') or (variant == 'warning')) else 'polite'), 'data-slot': 'toast'}, **attrs}, _join(_el('div', {'data-slot': 'toast-content'}, content), _el('button', {'type': 'button', 'data-slot': 'toast-close', 'aria-label': 'Close notification'}, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M18 6 6 18M6 6l12 12"/></svg>'))))  # alpharust-ignore: no-hardcoded-urls


@register.simple_block_tag(name="ds_toast_template")
def ds_toast_template(content, variant='default', **attrs):
    attrs = _x(attrs)
    return _el('template', {'data-toast-template': variant}, content)


@register.simple_block_tag(name="ds_toast_title")
def ds_toast_title(content, variant='default', **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-variant': variant, 'data-slot': 'toast-title', 'data-toast-title': ''}, **attrs}, _join((mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="m9 12 2 2 4-4"/></svg>') if (variant == 'success') else (mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="m10 10 4 4m0-4-4 4"/></svg>') if (variant == 'error') else (mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 9v4M12 17h.01M10.3 3.9 1.8 18a2 2 0 0 0 1.7 3h17a2 2 0 0 0 1.7-3L13.7 3.9a2 2 0 0 0-3.4 0z"/></svg>') if (variant == 'warning') else (mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="M12 8h.01M11 12h1v4h1"/></svg>') if (variant == 'info') else (mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="loader"><path d="M12 3a9 9 0 1 0 9 9"/></svg>') if (variant == 'loading') else ""))))), _el('span', {'data-toast-title-text': ''}, content)))  # alpharust-ignore: no-hardcoded-urls


@register.simple_block_tag(name="ds_toaster")
def ds_toaster(content, position='bottom-right', gap='0.5rem', peek='1rem', limit=3, duration=5000, **attrs):
    attrs = _x(attrs)
    return _el('div', {**{'data-position': position, 'data-limit': limit, 'data-duration': duration, 'role': 'region', 'aria-live': 'polite', 'aria-atomic': 'false', 'aria-relevant': 'additions text', 'aria-label': 'Notifications', 'tabindex': '-1', 'style': ('--gap: ' + str(gap) + '; --peek: ' + str(peek) + ';'), 'data-slot': 'toast-viewport'}, **attrs}, (content if str(content).strip() else _join(_el('template', {'data-toast-template': 'default'}, _el('div', {'data-variant': 'default', 'data-state': 'open', 'role': 'status', 'aria-live': 'polite', 'data-slot': 'toast'}, _join(_el('div', {'data-slot': 'toast-content'}, _join(_el('div', {'data-variant': 'default', 'data-slot': 'toast-title', 'data-toast-title': ''}, _el('span', {'data-toast-title-text': ''}, mark_safe('Title'))), _el('div', {'data-toast-description': '', 'data-slot': 'toast-description'}, mark_safe('Description')))), _el('button', {'type': 'button', 'data-slot': 'toast-close', 'aria-label': 'Close notification'}, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M18 6 6 18M6 6l12 12"/></svg>'))))), _el('template', {'data-toast-template': 'success'}, _el('div', {'data-variant': 'success', 'data-state': 'open', 'role': 'status', 'aria-live': 'polite', 'data-slot': 'toast'}, _join(_el('div', {'data-slot': 'toast-content'}, _join(_el('div', {'data-variant': 'success', 'data-slot': 'toast-title', 'data-toast-title': ''}, _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="m9 12 2 2 4-4"/></svg>'), _el('span', {'data-toast-title-text': ''}, mark_safe('Title')))), _el('div', {'data-toast-description': '', 'data-slot': 'toast-description'}, mark_safe('Description')))), _el('button', {'type': 'button', 'data-slot': 'toast-close', 'aria-label': 'Close notification'}, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M18 6 6 18M6 6l12 12"/></svg>'))))), _el('template', {'data-toast-template': 'error'}, _el('div', {'data-variant': 'error', 'data-state': 'open', 'role': 'alert', 'aria-live': 'assertive', 'data-slot': 'toast'}, _join(_el('div', {'data-slot': 'toast-content'}, _join(_el('div', {'data-variant': 'error', 'data-slot': 'toast-title', 'data-toast-title': ''}, _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="m10 10 4 4m0-4-4 4"/></svg>'), _el('span', {'data-toast-title-text': ''}, mark_safe('Title')))), _el('div', {'data-toast-description': '', 'data-slot': 'toast-description'}, mark_safe('Description')))), _el('button', {'type': 'button', 'data-slot': 'toast-close', 'aria-label': 'Close notification'}, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M18 6 6 18M6 6l12 12"/></svg>'))))), _el('template', {'data-toast-template': 'warning'}, _el('div', {'data-variant': 'warning', 'data-state': 'open', 'role': 'alert', 'aria-live': 'assertive', 'data-slot': 'toast'}, _join(_el('div', {'data-slot': 'toast-content'}, _join(_el('div', {'data-variant': 'warning', 'data-slot': 'toast-title', 'data-toast-title': ''}, _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 9v4M12 17h.01M10.3 3.9 1.8 18a2 2 0 0 0 1.7 3h17a2 2 0 0 0 1.7-3L13.7 3.9a2 2 0 0 0-3.4 0z"/></svg>'), _el('span', {'data-toast-title-text': ''}, mark_safe('Title')))), _el('div', {'data-toast-description': '', 'data-slot': 'toast-description'}, mark_safe('Description')))), _el('button', {'type': 'button', 'data-slot': 'toast-close', 'aria-label': 'Close notification'}, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M18 6 6 18M6 6l12 12"/></svg>'))))), _el('template', {'data-toast-template': 'info'}, _el('div', {'data-variant': 'info', 'data-state': 'open', 'role': 'status', 'aria-live': 'polite', 'data-slot': 'toast'}, _join(_el('div', {'data-slot': 'toast-content'}, _join(_el('div', {'data-variant': 'info', 'data-slot': 'toast-title', 'data-toast-title': ''}, _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="M12 8h.01M11 12h1v4h1"/></svg>'), _el('span', {'data-toast-title-text': ''}, mark_safe('Title')))), _el('div', {'data-toast-description': '', 'data-slot': 'toast-description'}, mark_safe('Description')))), _el('button', {'type': 'button', 'data-slot': 'toast-close', 'aria-label': 'Close notification'}, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M18 6 6 18M6 6l12 12"/></svg>'))))), _el('template', {'data-toast-template': 'loading'}, _el('div', {'data-variant': 'default', 'data-state': 'open', 'role': 'status', 'aria-live': 'polite', 'data-slot': 'toast'}, _join(_el('div', {'data-slot': 'toast-content'}, _join(_el('div', {'data-variant': 'loading', 'data-slot': 'toast-title', 'data-toast-title': ''}, _join(mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1.1em" height="1.1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="loader"><path d="M12 3a9 9 0 1 0 9 9"/></svg>'), _el('span', {'data-toast-title-text': ''}, mark_safe('Title')))), _el('div', {'data-toast-description': '', 'data-slot': 'toast-description'}, mark_safe('Description')))), _el('button', {'type': 'button', 'data-slot': 'toast-close', 'aria-label': 'Close notification'}, mark_safe('<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M18 6 6 18M6 6l12 12"/></svg>'))))))))  # alpharust-ignore: no-hardcoded-urls


@register.simple_block_tag(name="ds_toggle")
def ds_toggle(content, default_pressed=False, sync_group=None, variant='default', size='md', **attrs):
    attrs = _x(attrs)
    return _el('button', {**{'type': 'button', 'data-state': ('on' if default_pressed else 'off'), 'data-sync-group': sync_group, 'data-variant': variant, 'data-size': size, 'aria-pressed': ('true' if default_pressed else 'false'), 'data-slot': 'toggle'}, **attrs}, content)


@register.simple_tag(name="ds_video")
def ds_video(src='', title='Video', autoplay=False, muted=False, loop=False, controls=True, poster=None, **attrs):
    attrs = _x(attrs)
    embed = _youtube_embed(src, autoplay, muted, loop, controls)
    is_short = "/shorts/" in src
    return (_el('iframe', {**{'src': embed, 'title': title, 'allow': 'accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share', 'referrerpolicy': 'strict-origin-when-cross-origin', 'allowfullscreen': '', 'data-video-type': ('youtube-shorts' if is_short else 'youtube'), 'data-slot': 'video'}, **attrs}, "") if embed else _el('video', {**{'src': src, 'autoplay': ('' if autoplay else None), 'muted': ('' if muted else None), 'loop': ('' if loop else None), 'controls': ('' if controls else None), 'poster': poster, 'data-slot': 'video'}, **attrs}, mark_safe('<track kind="captions">')))
