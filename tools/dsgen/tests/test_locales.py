from vitrine import i18n


def test_every_locale_has_the_same_interface_keys_and_seo_metadata():
    assert i18n.CODES[0] == "fr"
    for code, meta in i18n.META.items():
        for key in ("code", "name", "label", "dir", "og_locale"):
            assert key in meta, (code, key)
        assert meta["dir"] in ("ltr", "rtl")
    for key, values in i18n.UI.items():
        assert set(values) == set(i18n.CODES), key


def test_arabic_is_rtl():
    assert i18n.META["ar"]["dir"] == "rtl"
