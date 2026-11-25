from common import ext_components, kebab, part_rel, rest_first


def test_kebab_and_part_rel():
    assert kebab("AlertDialogAction") == "alert-dialog-action"
    assert part_rel("alert-dialog", "AlertDialog") == ""
    assert part_rel("alert-dialog", "AlertDialogAction") == "action"


def test_rest_first_moves_the_spread_before_data_slot_on_html_elements_only():
    src = '<div class={c} data-slot="card" {...rest}>'
    assert rest_first(src) == '<div class={c} {...rest} data-slot="card">'
    comp = '<Dialog data-slot="sheet" {...rest}>'
    assert rest_first(comp) == comp


def test_extended_components_are_the_23_atoms_outside_core():
    ext = ext_components()
    assert len(ext) == 23
    assert "carousel" in ext and "button" not in ext
