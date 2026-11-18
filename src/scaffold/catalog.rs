//! Parité du catalogue de composants : un contrat (`templates/layers/design-kit/catalog.toml`),
//! quatre réalisations (Starwind/Tailwind, CSS pur Astro, Blade, balises Django) et des tests qui
//! comparent le contrat aux fichiers réels. Rien n'est supposé : chaque test lit les templates embarqués.
//! Composants `core` : les quatre cibles. Composants `css` : atomes Starwind + équivalents CSS pur (Astro),
//! style partagé dans design-kit (donc servi aussi à Laravel et Wagtail). Générateur : `tools/dsgen/gen_ds.py`.

#[cfg(test)]
mod tests {
    use crate::scaffold::astro::AstroOptions;
    use crate::scaffold::internal::{build_ctx, write_design_kit};
    use crate::scaffold::laravel::LaravelOptions;
    use crate::scaffold::layer;
    use crate::scaffold::platform::UiKit;
    use crate::scaffold::wagtail::WagtailOptions;
    use crate::scaffold::{plan, Action, Framework, Internal, Options, Request};
    use std::collections::BTreeSet;
    use std::fs;

    struct Component {
        name: String,
        kind: String,
        slots: Vec<String>,
        atom_only: Vec<String>,
    }

    impl Component {
        fn is_core(&self) -> bool {
            self.kind == "core"
        }
    }

    /// Tous les composants du contrat (core puis css).
    fn catalog() -> Vec<Component> {
        let raw = layer::files("design-kit")
            .into_iter()
            .find(|(p, _)| p == "catalog.toml")
            .expect("catalog.toml")
            .1;
        let v: toml::Value = toml::from_str(raw).expect("catalog.toml valide");
        let strings = |t: &toml::Value, k: &str| -> Vec<String> {
            t.get(k)
                .and_then(|a| a.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default()
        };
        let comps = v["component"]
            .as_array()
            .expect("[[component]]")
            .iter()
            .map(|c| Component {
                name: c["name"].as_str().unwrap().to_string(),
                kind: c["kind"].as_str().unwrap().to_string(),
                slots: strings(c, "slots"),
                atom_only: strings(c, "atom_only"),
            })
            .collect();
        comps
    }

    /// Catalogue core : décliné dans les quatre cibles (Starwind, CSS pur, Blade, Django).
    fn core() -> Vec<Component> {
        catalog().into_iter().filter(Component::is_core).collect()
    }

    /// Un nom de slot apparaît entre guillemets (`"x"` ou `'x'`) : quelle que soit la syntaxe de la cible.
    fn quoted(text: &str, slot: &str) -> bool {
        text.contains(&format!("\"{slot}\"")) || text.contains(&format!("'{slot}'"))
    }

    fn layer_tree(name: &str) -> String {
        layer::files(name)
            .iter()
            .map(|(_, b)| *b)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn render(req: &Request) -> tempfile::TempDir {
        let p = plan(req).expect("plan valide");
        let ctx = build_ctx(req, &p);
        let dir = tempfile::tempdir().unwrap();
        for s in &p.steps {
            match &s.action {
                Action::Do(Internal::Layer(l)) => {
                    layer::apply(l, &ctx, dir.path()).unwrap();
                }
                Action::Do(Internal::DesignKit(t)) => {
                    write_design_kit(dir.path(), *t).unwrap();
                }
                _ => {}
            }
        }
        dir
    }

    fn astro(ui: UiKit) -> tempfile::TempDir {
        let o = AstroOptions {
            ui,
            ..Default::default()
        };
        let mut req = Request::new(Framework::Astro, "demo");
        req.options = Options::Astro(o);
        render(&req)
    }

    fn laravel() -> tempfile::TempDir {
        let mut req = Request::new(Framework::Laravel, "demo");
        req.options = Options::Laravel(LaravelOptions {
            ui: true,
            ..Default::default()
        });
        render(&req)
    }

    fn wagtail() -> tempfile::TempDir {
        let mut req = Request::new(Framework::Wagtail, "demo");
        req.options = Options::Wagtail(WagtailOptions {
            ui: true,
            ..Default::default()
        });
        render(&req)
    }

    #[test]
    fn catalog_lists_23_core_and_23_css_components_with_unique_slots() {
        let comps = catalog();
        assert_eq!(comps.iter().filter(|c| c.is_core()).count(), 23);
        assert_eq!(comps.iter().filter(|c| c.kind == "css").count(), 23);
        assert!(comps.iter().all(|c| c.is_core() || c.kind == "css"));
        let mut seen = BTreeSet::new();
        for c in &comps {
            assert!(!c.slots.is_empty(), "{} sans slot", c.name);
            assert!(
                c.slots
                    .iter()
                    .any(|s| *s == c.name || s.starts_with(&c.name)),
                "{}",
                c.name
            );
            for s in &c.slots {
                assert!(seen.insert(s.clone()), "slot en double : {s}");
            }
        }
    }

    #[test]
    fn every_component_has_css_and_every_slot_is_styled() {
        let comps = catalog();
        let kit: Vec<(String, &str)> = layer::files("design-kit");
        let kit_slots: BTreeSet<&str> = comps
            .iter()
            .flat_map(|c| c.slots.iter().chain(c.atom_only.iter()).map(String::as_str))
            .collect();
        for c in &comps {
            let css = kit
                .iter()
                .find(|(p, _)| *p == format!("css/components/{}.css", c.name))
                .unwrap_or_else(|| panic!("css/components/{}.css manquant", c.name))
                .1;
            // au moins un slot du composant est stylé (un conteneur purement structurel peut n'avoir aucune règle)
            assert!(
                c.slots
                    .iter()
                    .any(|s| css.contains(&format!("[data-slot='{s}']"))),
                "{}.css ne cible aucun slot du composant",
                c.name
            );
            // et aucun slot cité n'est inventé
            for slot in css.split("[data-slot='").skip(1) {
                let slot = slot.split('\'').next().unwrap();
                assert!(
                    kit_slots.contains(slot),
                    "{}.css cible [data-slot='{slot}'] hors contrat",
                    c.name
                );
            }
        }
        let index = kit.iter().find(|(p, _)| p == "css/index.css").unwrap().1;
        for c in &comps {
            assert!(
                index.contains(&format!("components/{}.css", c.name)),
                "index.css n'importe pas {}",
                c.name
            );
        }
    }

    #[test]
    fn css_astro_blade_and_django_cover_every_slot_of_the_46_components() {
        // core et étendus : un dossier Blade par composant, chaque slot posé par Blade et par Django
        let comps = catalog();
        assert_eq!(comps.len(), 46);
        let django = layer_tree("ui-django");
        for c in &comps {
            let astro = concat_layer_dir("ui-css", &format!("src/components/atoms/{}/", c.name));
            let blade = concat_layer_dir(
                "ui-blade",
                &format!("resources/views/components/ds/{}/", c.name),
            );
            assert!(!astro.is_empty(), "Astro CSS : dossier {} absent", c.name);
            assert!(!blade.is_empty(), "Blade : dossier {} absent", c.name);
            for s in &c.slots {
                assert!(
                    quoted(&astro, s) || astro.contains(&format!("data-slot=\"{s}\"")),
                    "Astro CSS : slot {s}"
                );
                // sidebar-mobile : panneau Sheet (data-slot="sheet") repéré par data-sidebar-mobile, comme l'atome
                let marker = |t: &str| {
                    t.contains(&format!("'data-{s}'")) || t.contains(&format!("data-{s}=\""))
                };
                assert!(
                    quoted(&blade, s) || marker(&blade),
                    "Blade {} : slot {s}",
                    c.name
                );
                assert!(
                    quoted(&django, s) || marker(&django),
                    "Django {} : slot {s}",
                    c.name
                );
            }
        }
    }

    fn concat_layer_dir(layer_name: &str, prefix: &str) -> String {
        layer::files(layer_name)
            .iter()
            .filter(|(p, _)| p.starts_with(prefix))
            .map(|(_, b)| *b)
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn css_components_mirror_atomic_files_and_slots() {
        let atomic = layer::files("ui-atomic");
        let css = layer::files("ui-css");
        let names = |files: &[(String, &str)], dir: &str| -> BTreeSet<String> {
            let prefix = format!("src/components/atoms/{dir}/");
            files
                .iter()
                .filter_map(|(p, _)| p.strip_prefix(&prefix).map(String::from))
                .collect()
        };
        for c in catalog().iter().filter(|c| !c.is_core()) {
            // mêmes fichiers (composants, index.ts, types et scripts) que l'atome Starwind
            assert_eq!(
                names(&atomic, &c.name),
                names(&css, &c.name),
                "fichiers de {}",
                c.name
            );
            let astro = concat_layer_dir("ui-css", &format!("src/components/atoms/{}/", c.name));
            let atom = concat_layer_dir("ui-atomic", &format!("src/components/atoms/{}/", c.name));
            for s in &c.slots {
                let has = |t: &str| {
                    t.contains(&format!("data-slot=\"{s}\""))
                        || t.contains(&format!("\"data-slot\": \"{s}\""))
                };
                assert!(has(&astro), "CSS pur {} : slot {s}", c.name);
                assert!(has(&atom), "Starwind {} : slot {s}", c.name);
            }
            // mêmes exports publics (hors objets *Variants, propres à tailwind-variants)
            let index = |files: &[(String, &str)]| -> String {
                let p = format!("src/components/atoms/{}/index.ts", c.name);
                files
                    .iter()
                    .find(|(f, _)| *f == p)
                    .map(|(_, b)| *b)
                    .unwrap_or_default()
                    .to_string()
            };
            let (ia, ic) = (index(&atomic), index(&css));
            let exported = |src: &str| -> BTreeSet<String> {
                let body = src.split("export {").nth(1).unwrap_or_default();
                body.split('}')
                    .next()
                    .unwrap_or_default()
                    .split(',')
                    .map(|n| n.trim().trim_start_matches("type ").to_string())
                    .filter(|n| !n.is_empty() && !n.ends_with("Variants"))
                    .collect()
            };
            assert_eq!(exported(&ia), exported(&ic), "exports de {}", c.name);
        }
    }

    #[test]
    fn starwind_atoms_use_only_contract_slots_and_every_atom_is_classified() {
        let comps = catalog();
        let allowed: BTreeSet<String> = comps
            .iter()
            .flat_map(|c| c.slots.iter().chain(c.atom_only.iter()).cloned())
            .collect();
        let atoms_root = "src/components/atoms/";
        let files = layer::files("ui-atomic");
        let mut dirs = BTreeSet::new();
        for (p, body) in &files {
            let Some(rest) = p.strip_prefix(atoms_root) else {
                continue;
            };
            let dir = rest.split('/').next().unwrap().to_string();
            dirs.insert(dir.clone());
            if !comps.iter().any(|c| c.name == dir) {
                continue;
            }
            for slot in body.split("data-slot=\"").skip(1) {
                let slot = slot.split('"').next().unwrap();
                if slot.contains('{') || slot.contains('$') {
                    continue;
                }
                assert!(
                    allowed.contains(slot),
                    "atome {dir} : slot `{slot}` hors contrat (ajoutez-le à atom_only ou au contrat)"
                );
            }
        }
        // chaque atome est déclaré au contrat (core ou css) : aucun composant Starwind sans équivalent CSS pur
        for d in &dirs {
            assert!(
                comps.iter().any(|c| &c.name == d),
                "atome `{d}` absent de catalog.toml"
            );
        }
        for c in &comps {
            assert!(
                dirs.contains(&c.name),
                "composant `{}` absent des atomes Starwind",
                c.name
            );
        }
    }

    #[test]
    fn catalogue_pages_show_all_46_components_in_all_four_targets() {
        let comps = core();
        let sw = astro(UiKit::Starwind);
        let cc = astro(UiKit::Concordia);
        let lv = laravel();
        let wg = wagtail();
        let page_sw = fs::read_to_string(sw.path().join("src/pages/components.astro")).unwrap();
        let page_cc = fs::read_to_string(cc.path().join("src/pages/components.astro")).unwrap();
        let page_lv =
            fs::read_to_string(lv.path().join("resources/views/pages/components.blade.php"))
                .unwrap();
        let page_wg =
            fs::read_to_string(wg.path().join("ds/templates/ds/components.html")).unwrap();
        for c in &comps {
            let import = format!("@atoms/{}'", c.name);
            assert!(page_sw.contains(&import), "page Starwind : {}", c.name);
            assert!(page_cc.contains(&import), "page Concordia : {}", c.name);
            assert!(
                page_lv.contains(&format!("<x-ds.{}", c.name)),
                "page Laravel : {}",
                c.name
            );
            assert!(
                page_wg.contains(&format!("ds_{}", c.name.replace('-', "_"))),
                "page Wagtail : {}",
                c.name
            );
        }
        // composants étendus : page commune aux deux kits Astro, liée depuis /components
        for (kit, dir) in [("Starwind", &sw), ("CSS", &cc)] {
            let page = dir.path().join("src/pages/components/advanced.astro");
            let page =
                fs::read_to_string(&page).unwrap_or_else(|_| panic!("{kit} : advanced.astro"));
            for c in catalog().iter().filter(|c| !c.is_core()) {
                assert!(
                    page.contains(&format!("@atoms/{}'", c.name)),
                    "{kit} avancé : {}",
                    c.name
                );
            }
        }
        assert!(
            page_sw.contains("/components/advanced") && page_cc.contains("/components/advanced")
        );
        // côté serveur : page « composants avancés » Blade et Django, liée depuis /components
        let adv_lv = fs::read_to_string(
            lv.path()
                .join("resources/views/pages/components-advanced.blade.php"),
        )
        .unwrap();
        let adv_wg =
            fs::read_to_string(wg.path().join("ds/templates/ds/components_advanced.html")).unwrap();
        for c in catalog().iter().filter(|c| !c.is_core()) {
            assert!(
                adv_lv.contains(&format!("<x-ds.{}", c.name)),
                "Laravel avancé : {}",
                c.name
            );
            assert!(
                adv_wg.contains(&format!("{{% ds_{}", c.name.replace('-', "_"))),
                "Wagtail avancé : {}",
                c.name
            );
        }
        assert!(page_lv.contains("/components/advanced"));
        assert!(page_wg.contains("ds-components-advanced"));
        let routes = fs::read_to_string(lv.path().join("routes/pages.php")).unwrap();
        assert!(routes.contains("pages.components-advanced"));
        let urls = fs::read_to_string(wg.path().join("ds/urls.py")).unwrap();
        assert!(urls.contains("AdvancedComponentsView"));
    }

    #[test]
    fn astro_css_kit_installs_components_script_and_keeps_custom_css() {
        let comps = catalog();
        let cc = astro(UiKit::Concordia);
        let styles = cc.path().join("src/styles/components");
        for c in &comps {
            assert!(
                styles.join(format!("{}.css", c.name)).is_file(),
                "{}",
                c.name
            );
        }
        assert!(styles.join("custom.css").is_file());
        assert!(cc.path().join("src/scripts/ds.js").is_file());
        assert!(cc
            .path()
            .join("src/components/atoms/button/Button.astro")
            .is_file());
        let global = fs::read_to_string(cc.path().join("src/styles/global.css")).unwrap();
        for c in &comps {
            assert!(
                global.contains(&format!("./components/{}.css", c.name)),
                "global.css n'importe pas {}",
                c.name
            );
        }
        assert!(global.contains("./components/utilities.css"));
        let js = fs::read_to_string(cc.path().join("src/scripts/ds.js")).unwrap();
        for f in [
            "initSelects",
            "initSidebars",
            "initCarousels",
            "__starwind__",
        ] {
            assert!(js.contains(f), "ds.js : {f}");
        }
        // Concordia garde ses fichiers dans une couche : les composants du kit passent après
        assert!(global.contains("layer(concordia)"));
        // Starwind n'embarque pas les feuilles du kit (ses composants sont en Tailwind)
        let sw = astro(UiKit::Starwind);
        assert!(!sw
            .path()
            .join("src/styles/components/accordion.css")
            .exists());
        assert!(!sw.path().join("src/scripts/ds.js").exists());
        assert!(sw.path().join("src/pages/components.astro").is_file());
        assert!(!sw.path().join("src/pages/design.astro").exists());
    }

    #[test]
    fn plain_and_tailwind_astro_take_their_tokens_from_the_kit() {
        for tailwind in [false, true] {
            let mut o = AstroOptions::default();
            if tailwind {
                o.integrations.push("tailwind".into());
            }
            let mut req = Request::new(Framework::Astro, "demo");
            req.options = Options::Astro(o);
            let d = render(&req);
            let tokens = d.path().join("src/styles/tokens");
            for f in ["colors", "typography", "spacing", "components", "site"] {
                assert!(
                    tokens.join(format!("{f}.css")).is_file(),
                    "{f} (tailwind={tailwind})"
                );
            }
            assert_eq!(tokens.join("tailwind.css").is_file(), tailwind);
            assert!(!d.path().join("src/pages/components.astro").exists());
        }
    }

    #[test]
    fn laravel_kit_goes_to_resources_and_wires_routes_and_layout() {
        let lv = laravel();
        let root = lv.path();
        for f in [
            "resources/css/design/index.css",
            "resources/css/design/components/button.css",
            "resources/css/design/tokens/tailwind.css",
            "resources/css/design/components/custom.css",
            "resources/js/design/ds.js",
            "resources/css/app.css",
            "resources/js/app.js",
            "resources/views/components/ds/button/button.blade.php",
            "tests/Feature/ComponentsTest.php",
        ] {
            assert!(root.join(f).is_file(), "{f}");
        }
        let routes = fs::read_to_string(root.join("routes/pages.php")).unwrap();
        assert!(routes.contains("/components"));
        let layout =
            fs::read_to_string(root.join("resources/views/components/site-layout.blade.php"))
                .unwrap();
        assert!(layout.contains("@vite(['resources/css/app.css', 'resources/js/app.js'])"));
        assert!(layout.contains("public_path('build/manifest.json')"));
        let app = fs::read_to_string(root.join("resources/css/app.css")).unwrap();
        assert!(app.contains("@layer theme, base, ds, components, utilities;"));
        assert!(app.contains("@import './design/index.css' layer(ds);"));
        // sans --ui, ni route ni catalogue
        let mut req = Request::new(Framework::Laravel, "demo");
        req.options = Options::Laravel(LaravelOptions::default());
        let plain = render(&req);
        assert!(!fs::read_to_string(plain.path().join("routes/pages.php"))
            .unwrap()
            .contains("/components"));
        assert!(!plain.path().join("resources/views/components/ds").exists());
    }

    #[test]
    fn laravel_ui_refuses_breeze_starters() {
        use crate::scaffold::laravel::Starter;
        let o = LaravelOptions {
            ui: true,
            starter: Starter::BreezeBlade,
            ..Default::default()
        };
        assert!(o.validate().is_err());
    }

    #[test]
    fn wagtail_kit_is_an_app_with_static_files_and_gets_wired_idempotently() {
        let wg = wagtail();
        let root = wg.path();
        for f in [
            "ds/__init__.py",
            "ds/apps.py",
            "ds/templatetags/ds.py",
            "ds/urls.py",
            "ds/views.py",
            "ds/tests.py",
            "ds/templates/ds/components.html",
            "ds/static/ds/css/index.css",
            "ds/static/ds/css/components/tabs.css",
            "ds/static/ds/js/ds.js",
        ] {
            assert!(root.join(f).is_file(), "{f}");
        }
        // squelette `wagtail start` minimal, puis branchement
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path();
        fs::create_dir_all(p.join("demo/settings")).unwrap();
        fs::create_dir_all(p.join("demo/templates")).unwrap();
        fs::write(
            p.join("demo/settings/base.py"),
            "INSTALLED_APPS = [\n    \"home\",\n]\nMIDDLEWARE = [\n    \"django.contrib.sessions.middleware.SessionMiddleware\",\n]\nTEMPLATES = [{\"OPTIONS\": {\"context_processors\": [\n                \"django.contrib.messages.context_processors.messages\",\n]}}]\nLANGUAGE_CODE = \"en-us\"\n",
        )
        .unwrap();
        fs::write(
            p.join("demo/urls.py"),
            "urlpatterns = [\n    path(\"search/\", search_views.search, name=\"search\"),\n]\nurlpatterns = urlpatterns + [\n    path(\"\", include(wagtail_urls)),\n]\n",
        )
        .unwrap();
        fs::write(
            p.join("demo/templates/base.html"),
            "{# Global stylesheets #}\n{# Global javascript #}\n",
        )
        .unwrap();
        for _ in 0..2 {
            crate::scaffold::internal::patch_wagtail_design(p, "demo", false).unwrap();
        }
        let settings = fs::read_to_string(p.join("demo/settings/base.py")).unwrap();
        assert_eq!(settings.matches("\"ds\",").count(), 1);
        assert_eq!(
            settings.matches("layout.context_processors.site").count(),
            1
        );
        assert_eq!(settings.matches("\"blog\",").count(), 1);
        assert!(settings.contains("LANGUAGE_CODE = \"fr\""));
        let urls = fs::read_to_string(p.join("demo/urls.py")).unwrap();
        assert_eq!(urls.matches("ds.urls").count(), 1);
        assert_eq!(urls.matches("i18n_patterns(").count(), 1);
        assert!(urls.contains("prefix_default_language=False"));
        assert_eq!(settings.matches("wagtail.locales").count(), 1);
        assert_eq!(settings.matches("LocaleMiddleware").count(), 1);
        assert_eq!(settings.matches("WAGTAIL_I18N_ENABLED").count(), 1);
        let base = fs::read_to_string(p.join("demo/templates/base.html")).unwrap();
        assert_eq!(base.matches("ds/css/index.css").count(), 1);
        assert_eq!(base.matches("ds/js/ds.js").count(), 1);
        assert_eq!(base.matches("ds/favicon.svg").count(), 1);
        // un squelette modifié est signalé, jamais patché à l'aveugle
        fs::write(p.join("demo/urls.py"), "urlpatterns = []\n").unwrap();
        assert!(crate::scaffold::internal::patch_wagtail_design(p, "demo", false).is_err());
    }
}
