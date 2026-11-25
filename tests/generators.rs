//! Pipeline de génération : outils officiels (simulés ou réels) + couche de modèles + vérification.
//!
//! * Tests *hermétiques* : un faux `npm`/`node` (scripts shell dans un PATH jetable) joue le rôle de
//!   `create-astro` ; on vérifie l'ordre des étapes, les correctifs (tsconfig, scripts, site), la couche
//!   de modèles, le manifeste, le lint du résultat, les erreurs de pré-vérification et les garde-fous.
//! * Tests *réels* (`ALPHARUST_E2E=1`) : de vraies générations avec create-astro, composer, wagtail, cargo.
//!   Ils s'ignorent proprement (message) quand une chaîne d'outils manque.

#![cfg(unix)]

use alpharust::engine;
use alpharust::scaffold::astro::{AstroOptions, PackageManager, Template, TsLevel};
use alpharust::scaffold::laravel::{LaravelOptions, Starter};
use alpharust::scaffold::platform::{Db, UiKit};
use alpharust::scaffold::theme::ThemeColors;
use alpharust::scaffold::wagtail::WagtailOptions;
use alpharust::scaffold::{self, Event, Framework, Options, Report, Request};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Mutex, MutexGuard, OnceLock};

/// `PATH` est global au processus : les tests qui le modifient (ou en dépendent) sont sérialisés.
static LOCK: Mutex<()> = Mutex::new(());
static ORIGINAL_PATH: OnceLock<std::ffi::OsString> = OnceLock::new();

fn lock() -> MutexGuard<'static, ()> {
    let g = LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    ORIGINAL_PATH.get_or_init(|| std::env::var_os("PATH").unwrap_or_default());
    std::env::set_var("ALPHARUST_NO_CACHE", "1");
    g
}

fn set_path(shims: Option<&Path>) {
    let orig = ORIGINAL_PATH.get().unwrap().clone();
    match shims {
        Some(dir) => {
            let mut paths = vec![dir.to_path_buf()];
            paths.extend(std::env::split_paths(&orig));
            std::env::set_var("PATH", std::env::join_paths(paths).unwrap());
        }
        None => std::env::set_var("PATH", orig),
    }
}

fn write_exe(dir: &Path, name: &str, body: &str) {
    let p = dir.join(name);
    std::fs::write(&p, format!("#!/bin/sh\n{body}\n")).unwrap();
    std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
}

/// Faux `node` + faux `npm` qui imitent `npm create astro@5.3.2`, `astro add`, `npm install`, `npm run`.
fn fake_astro_toolchain() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    write_exe(dir.path(), "node", "echo v24.0.0");
    write_exe(
        dir.path(),
        "npm",
        r#"
if [ "$1" = "--version" ]; then echo 11.0.0; exit 0; fi
echo "npm $*" >> "$FAKE_NPM_LOG"
case "$1" in
  create)
    d="$3"
    mkdir -p "$d/src/pages" "$d/public"
    printf '{\n  "name": "%s",\n  "type": "module",\n  "version": "0.0.1",\n  "scripts": {\n    "dev": "astro dev",\n    "build": "astro build",\n    "preview": "astro preview",\n    "astro": "astro"\n  },\n  "dependencies": {\n    "astro": "^5.0.0"\n  }\n}\n' "$d" > "$d/package.json"
    printf '{\n  "extends": "astro/tsconfigs/strict",\n  "include": [".astro/types.d.ts", "**/*"],\n  "exclude": ["dist"]\n}\n' > "$d/tsconfig.json"
    printf "import { defineConfig } from 'astro/config';\n\n// https://astro.build/config\nexport default defineConfig({});\n" > "$d/astro.config.mjs"
    printf -- '---\n---\n<h1>Astro</h1>\n' > "$d/src/pages/index.astro"
    printf 'node_modules\ndist\n.astro\n' > "$d/.gitignore"
    printf '# official\n' > "$d/README.md"
    echo '<svg xmlns="http://www.w3.org/2000/svg"/>' > "$d/public/favicon.svg"
    echo "created $d"
    ;;
  exec) echo "astro add done" ;;
  install) echo "installed" ;;
  run) echo "ran $2" ;;
esac
exit 0
"#,
    );
    dir
}

fn run(req: &Request) -> (anyhow::Result<Report>, Vec<Event>) {
    let mut events = Vec::new();
    let cancel = AtomicBool::new(false);
    let r = scaffold::execute(req, &mut |e| events.push(e), &cancel);
    (r, events)
}

fn astro_request(dir: &Path, name: &str, f: impl FnOnce(&mut AstroOptions)) -> Request {
    let mut o = AstroOptions::default();
    f(&mut o);
    let mut req = Request::new(Framework::Astro, name);
    req.path = Some(dir.join(name));
    req.options = Options::Astro(o);
    req
}

fn step_ids(events: &[Event]) -> Vec<String> {
    events
        .iter()
        .filter_map(|e| match e {
            Event::StepStart { id, .. } => Some(id.clone()),
            _ => None,
        })
        .collect()
}

fn read(p: &Path) -> String {
    std::fs::read_to_string(p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

fn json(p: &Path) -> serde_json::Value {
    serde_json::from_str(&read(p)).unwrap()
}

#[test]
fn astro_pipeline_runs_official_tool_then_layers_templates_then_verifies() {
    let _g = lock();
    let shims = fake_astro_toolchain();
    let log = shims.path().join("npm.log");
    std::env::set_var("FAKE_NPM_LOG", &log);
    set_path(Some(shims.path()));

    let out = tempfile::tempdir().unwrap();
    let req = astro_request(out.path(), "vitrine", |o| {
        o.integrations = vec!["tailwind".into(), "mdx".into(), "sitemap".into()];
        o.site = Some("https://vitrine.example.com".into());
        o.typescript = TsLevel::Strictest;
    });
    let (result, events) = run(&req);
    set_path(None);
    let report = result.unwrap_or_else(|e| panic!("{e:#}"));
    let t = out.path().join("vitrine");

    // 1. ordre des étapes : préparation → outil officiel → configuration → couche → vérifications
    let ids = step_ids(&events);
    assert_eq!(
        ids,
        [
            "prepare",
            "create",
            "tsconfig",
            "allow-scripts",
            "integrations",
            "typecheck-deps",
            "scripts",
            "layer",
            "design-kit",
            "site",
            "manifest",
            "check",
            "build",
            "lint",
            "git"
        ],
        "{ids:?}"
    );
    assert!(matches!(events.first(), Some(Event::Preflight { .. })));
    assert!(matches!(events.last(), Some(Event::Done { .. })));

    // 2. les vraies commandes ont été lancées, dans l'ordre
    let npm = read(&log);
    let lines: Vec<&str> = npm.lines().collect();
    assert!(
        lines[0].starts_with(
            "npm create astro@5.3.2 vitrine -- --template minimal --install --no-git --no-ai --yes"
        ),
        "{lines:?}"
    );
    assert!(
        lines[1].contains("exec -- astro add tailwind mdx sitemap --yes"),
        "{lines:?}"
    );
    assert!(
        lines[2].contains("install -D @astrojs/check typescript@^6"),
        "{lines:?}"
    );
    assert_eq!(lines[3], "npm run check");
    assert_eq!(lines[4], "npm run build");

    // 3. correctifs appliqués sur les fichiers officiels (pas de réécriture à la main)
    assert_eq!(
        json(&t.join("tsconfig.json"))["extends"],
        "astro/tsconfigs/strictest"
    );
    assert_eq!(
        json(&t.join("package.json"))["scripts"]["check"],
        "astro check"
    );
    assert!(read(&t.join("astro.config.mjs")).contains("site: 'https://vitrine.example.com'"));

    // 4. couche de modèles : fichiers ajoutés, fichiers officiels remplacés, marqueurs résolus
    for f in [
        "src/layouts/BaseLayout.astro",
        "src/components/SEO.astro",
        "src/components/ThemeToggle.astro",
        "src/content.config.ts",
        "src/content/blog/bienvenue.md",
        "src/content/blog/exemple-mdx.mdx",
        "src/pages/blog/[...slug].astro",
        "src/styles/tokens/colors.css",
        "src/styles/themes/theme.css",
        ".alpharust.toml",
        "docs/index.md",
    ] {
        assert!(t.join(f).is_file(), "{f} manquant");
    }
    assert!(
        read(&t.join("src/pages/index.astro")).contains("BaseLayout"),
        "index officiel remplacé"
    );
    let entry = read(&t.join("src/styles/global.css"));
    assert!(
        entry.find("@import \"tailwindcss\"").unwrap() < entry.find("./tokens/colors.css").unwrap(),
        "tailwind avant les jetons"
    );
    assert!(
        read(&t.join(".gitignore")).contains("node_modules"),
        "fichier officiel conservé"
    );
    for f in &report.files {
        assert!(!f.contains("@@"), "chemin non résolu : {f}");
        if let Ok(c) = std::fs::read_to_string(t.join(f)) {
            assert!(!c.contains("@@"), "marqueur non résolu dans {f}");
        }
    }

    // 5. manifeste, lint du résultat, rapport
    let m = json(&t.join(".alpharust-project.json"));
    assert_eq!(m["framework"], "astro");
    assert_eq!(m["official_tool"], "create-astro");
    assert_eq!(
        m["request"]["integrations"],
        serde_json::json!(["tailwind", "mdx", "sitemap"])
    );
    let lint = engine::lint(&t, None).unwrap();
    assert_eq!(lint.count("error"), 0, "{:#?}", lint.violations);
    assert!(report.ok && report.lint.as_ref().unwrap().errors == 0);
    assert!(report.next_steps.iter().any(|s| s.contains("npm run dev")));
    assert!(t.join(".git").is_dir(), "dépôt git initialisé");
}

#[test]
fn astro_without_install_skips_installation_and_verification() {
    let _g = lock();
    let shims = fake_astro_toolchain();
    let log = shims.path().join("npm.log");
    std::env::set_var("FAKE_NPM_LOG", &log);
    set_path(Some(shims.path()));
    let out = tempfile::tempdir().unwrap();
    let mut req = astro_request(out.path(), "lite", |o| o.template = Template::Minimal);
    req.install = false;
    req.git = false;
    let (result, events) = run(&req);
    set_path(None);
    result.unwrap_or_else(|e| panic!("{e:#}"));
    let ids = step_ids(&events);
    assert!(
        !ids.iter()
            .any(|i| ["build", "check", "typecheck-deps", "integrations"].contains(&i.as_str())),
        "{ids:?}"
    );
    assert!(read(&log).contains("--no-install"));
    assert!(!out.path().join("lite/.git").exists());
}

#[test]
fn missing_toolchain_fails_early_with_install_hints() {
    let _g = lock();
    let empty = tempfile::tempdir().unwrap();
    std::env::set_var("PATH", empty.path());
    let out = tempfile::tempdir().unwrap();
    let req = astro_request(out.path(), "nope", |_| {});
    let (result, events) = run(&req);
    let tools = scaffold::preflight(&req);
    set_path(None);
    let err = format!("{:#}", result.unwrap_err());
    assert!(
        err.contains("Node.js") && err.contains("nodejs.org"),
        "{err}"
    );
    assert!(matches!(events.last(), Some(Event::Failed { .. })));
    assert!(
        !out.path().join("nope").exists(),
        "rien n'est créé quand un outil manque"
    );
    assert!(tools
        .iter()
        .any(|t| t.id == "node" && !t.found && !t.hint.is_empty()));
    let ids = step_ids(&events);
    assert!(ids.is_empty(), "aucune étape exécutée : {ids:?}");
}

#[test]
fn failing_official_command_stops_the_pipeline_with_its_output() {
    let _g = lock();
    let shims = tempfile::tempdir().unwrap();
    write_exe(shims.path(), "node", "echo v24.0.0");
    write_exe(
        shims.path(),
        "npm",
        r#"if [ "$1" = "--version" ]; then echo 11.0.0; exit 0; fi
echo "npm ERR! network unreachable" >&2
exit 7"#,
    );
    set_path(Some(shims.path()));
    let out = tempfile::tempdir().unwrap();
    let req = astro_request(out.path(), "broken", |_| {});
    let (result, events) = run(&req);
    set_path(None);
    let err = format!("{:#}", result.unwrap_err());
    assert!(err.contains("code 7") || err.contains("7"), "{err}");
    assert!(
        err.contains("network unreachable"),
        "la sortie de l'outil est conservée : {err}"
    );
    let ids = step_ids(&events);
    assert_eq!(
        ids,
        ["prepare", "create"],
        "pas d'étape après l'échec : {ids:?}"
    );
    assert!(events
        .iter()
        .any(|e| matches!(e, Event::Log { line, .. } if line.contains("network unreachable"))));
}

#[test]
fn cancellation_stops_before_running_anything() {
    let _g = lock();
    let shims = fake_astro_toolchain();
    std::env::set_var("FAKE_NPM_LOG", shims.path().join("npm.log"));
    set_path(Some(shims.path()));
    let out = tempfile::tempdir().unwrap();
    let req = astro_request(out.path(), "cancelled", |_| {});
    let cancel = AtomicBool::new(true);
    let mut events = Vec::new();
    let r = scaffold::execute(&req, &mut |e| events.push(e), &cancel);
    set_path(None);
    let err = format!("{:#}", r.unwrap_err());
    assert!(err.to_lowercase().contains("annul"), "{err}");
    assert!(
        !shims.path().join("npm.log").exists(),
        "aucune commande lancée"
    );
}

#[test]
fn force_only_replaces_projects_generated_by_alpharust() {
    let _g = lock();
    let shims = fake_astro_toolchain();
    std::env::set_var("FAKE_NPM_LOG", shims.path().join("npm.log"));
    set_path(Some(shims.path()));
    let out = tempfile::tempdir().unwrap();

    // dossier étranger non vide : refus, même avec --force, et rien n'est supprimé
    let foreign = out.path().join("foreign");
    std::fs::create_dir_all(&foreign).unwrap();
    std::fs::write(foreign.join("precious.txt"), "à garder").unwrap();
    let mut req = astro_request(out.path(), "foreign", |o| o.template = Template::Minimal);
    req.install = false;
    let (r, _) = run(&req);
    assert!(format!("{:#}", r.unwrap_err()).contains("--force"));
    req.force = true;
    let (r, _) = run(&req);
    let err = format!("{:#}", r.unwrap_err());
    assert!(
        err.contains("n'est pas un projet généré par AlphaRust"),
        "{err}"
    );
    assert_eq!(read(&foreign.join("precious.txt")), "à garder");

    // projet AlphaRust existant : refus sans --force, remplacement avec
    let mut ok = astro_request(out.path(), "mine", |o| o.template = Template::Minimal);
    ok.install = false;
    ok.git = false;
    run(&ok).0.unwrap_or_else(|e| panic!("{e:#}"));
    std::fs::write(out.path().join("mine/stale.txt"), "x").unwrap();
    assert!(run(&ok).0.is_err());
    ok.force = true;
    run(&ok).0.unwrap_or_else(|e| panic!("{e:#}"));
    set_path(None);
    assert!(
        !out.path().join("mine/stale.txt").exists(),
        "ancien contenu supprimé"
    );
}

#[test]
fn invalid_requests_are_rejected_before_any_work() {
    let _g = lock();
    for bad in ["../evil", "a/b", ".hidden", "..", "", "-rf", "x y"] {
        let r = Request::new(Framework::Rust, bad);
        assert!(r.validate().is_err(), "{bad:?}");
    }
    let mut r = Request::new(Framework::Wagtail, "cms");
    r.install = false;
    assert!(r
        .validate()
        .unwrap_err()
        .to_string()
        .contains("Wagtail exige l'installation"));
    let mut r = astro_request(Path::new("/tmp"), "x", |o| {
        o.integrations = vec!["sitemap".into()];
    });
    assert!(r.validate().unwrap_err().to_string().contains("--site"));
    r = astro_request(Path::new("/tmp"), "x", |o| {
        o.integrations = vec!["react".into(), "svelte".into()];
    });
    assert!(r.validate().is_err());
    let mut lv = Request::new(Framework::Laravel, "x");
    lv.options = Options::Laravel(LaravelOptions {
        pest: true,
        ..Default::default()
    });
    assert!(lv.validate().is_err(), "--pest sans kit");
    lv.options = Options::Laravel(LaravelOptions {
        starter: Starter::BreezeBlade,
        pest: true,
        ..Default::default()
    });
    assert!(lv.validate().is_ok());
    let mut w = Request::new(Framework::Wagtail, "x");
    w.options = Options::Wagtail(WagtailOptions {
        packages: vec!["bad;rm -rf /".into()],
        migrate: true,
        ..Default::default()
    });
    assert!(w.validate().is_err(), "spécification pip invalide");
}

#[test]
fn plans_are_pure_previews_and_never_touch_the_disk() {
    let _g = lock();
    let out = tempfile::tempdir().unwrap();
    for f in Framework::ALL {
        let mut req = Request::new(f, "preview");
        req.path = Some(out.path().join("preview"));
        let plan = scaffold::plan(&req).unwrap();
        let text = plan.preview();
        assert!(
            text.contains(f.official_tool().split_whitespace().next().unwrap())
                || text.contains(f.official_tool()),
            "{f:?}: {text}"
        );
        assert!(plan.steps.len() >= 8, "{f:?}");
        assert!(!out.path().join("preview").exists());
    }
    let mut req = astro_request(out.path(), "pm", |o| {
        o.package_manager = PackageManager::Bun;
    });
    req.install = true;
    let p = scaffold::plan(&req).unwrap().preview();
    assert!(p.contains("bun create astro@5.3.2 pm"), "{p}");
}

#[test]
fn rust_project_is_created_by_cargo_new_and_verified_for_real() {
    let _g = lock();
    set_path(None);
    if alpharust::scaffold::tools::which("cargo").is_none() {
        eprintln!("cargo absent : test ignoré");
        return;
    }
    let out = tempfile::tempdir().unwrap();
    let mut req = Request::new(Framework::Rust, "vrai-crate");
    req.path = Some(out.path().join("vrai-crate"));
    // pas de dépendance réseau : on vérifie sans installer d'extras
    let (result, events) = run(&req);
    let report = result.unwrap_or_else(|e| panic!("{e:#}"));
    let t = out.path().join("vrai-crate");
    let ids = step_ids(&events);
    assert_eq!(ids[1], "create");
    assert!(t.join("Cargo.toml").is_file() && t.join("src/main.rs").is_file());
    assert!(
        read(&t.join("Cargo.toml")).contains("name = \"vrai-crate\""),
        "généré par cargo new"
    );
    assert!(t.join("tests/smoke.rs").is_file() && t.join(".github/workflows/ci.yml").is_file());
    assert!(ids.contains(&"test".to_string()));
    assert!(
        report.steps.iter().find(|s| s.id == "test").unwrap().ok,
        "cargo test passe dans le projet généré"
    );
    assert_eq!(report.lint.as_ref().unwrap().errors, 0);
    let fmt = report.steps.iter().find(|s| s.id == "fmt");
    if let Some(f) = fmt {
        assert!(f.ok, "cargo fmt --check : {:?}", f.note);
    }
}

// ---------------------------------------------------------------------------
// Générations réelles (ALPHARUST_E2E=1) : create-astro, composer, wagtail… avec réseau
// ---------------------------------------------------------------------------

fn e2e_enabled() -> bool {
    std::env::var("ALPHARUST_E2E").is_ok_and(|v| v == "1")
}

/// Génère pour de vrai ; renvoie `None` (test ignoré) si une chaîne d'outils manque.
fn e2e(req: Request) -> Option<(PathBuf, Report, tempfile::TempDir)> {
    if !e2e_enabled() {
        eprintln!("ALPHARUST_E2E != 1 : génération réelle ignorée");
        return None;
    }
    set_path(None);
    let tools = scaffold::preflight(&req);
    if let Err(e) = scaffold::preflight_check(&tools) {
        eprintln!("chaîne d'outils absente, test ignoré : {e:#}");
        return None;
    }
    let out = tempfile::tempdir().unwrap();
    let mut req = req;
    req.path = Some(out.path().join(&req.name));
    let (result, events) = run(&req);
    let report = result.unwrap_or_else(|e| {
        let logs: Vec<String> = events
            .iter()
            .filter_map(|e| match e {
                Event::Log { line, .. } => Some(line.clone()),
                _ => None,
            })
            .collect();
        let tail: Vec<&String> = logs.iter().rev().take(40).collect();
        panic!("génération en échec : {e:#}\n{tail:#?}")
    });
    for s in &report.steps {
        assert!(
            s.ok || !s.fatal,
            "étape en échec : {} ({:?})",
            s.label,
            s.note
        );
    }
    let t = out.path().join(&req.name);
    Some((t, report, out))
}

fn assert_lint_clean(t: &Path) {
    let r = engine::lint(t, None).unwrap();
    assert_eq!(r.count("error"), 0, "{:#?}", r.violations);
}

#[test]
#[ignore = "génération réelle : ALPHARUST_E2E=1 cargo test --test generators -- --ignored"]
fn e2e_astro_npm_with_integrations() {
    let _g = lock();
    let mut req = Request::new(Framework::Astro, "e2e-astro");
    req.options = Options::Astro(AstroOptions {
        integrations: vec![
            "tailwind".into(),
            "mdx".into(),
            "sitemap".into(),
            "react".into(),
        ],
        site: Some("https://e2e.example.com".into()),
        ..Default::default()
    });
    let Some((t, report, _keep)) = e2e(req) else {
        return;
    };
    assert!(
        t.join("node_modules/astro").is_dir(),
        "dépendances installées"
    );
    assert!(t.join("dist/index.html").is_file(), "build effectué");
    assert!(t.join("dist/sitemap-index.xml").is_file(), "sitemap généré");
    for id in ["check", "build"] {
        assert!(report.steps.iter().find(|s| s.id == id).unwrap().ok, "{id}");
    }
    let pkg = read(&t.join("package.json"));
    for dep in [
        "@astrojs/react",
        "@astrojs/mdx",
        "@astrojs/sitemap",
        "tailwindcss",
        "@astrojs/check",
    ] {
        assert!(pkg.contains(dep), "{dep} installé via astro add");
    }
    assert_lint_clean(&t);
}

#[test]
#[ignore = "génération réelle : ALPHARUST_E2E=1 cargo test --test generators -- --ignored"]
fn e2e_astro_official_blog_template_with_pnpm() {
    let _g = lock();
    if alpharust::scaffold::tools::which("pnpm").is_none() {
        eprintln!("pnpm absent : test ignoré");
        return;
    }
    let mut req = Request::new(Framework::Astro, "e2e-blog");
    req.options = Options::Astro(AstroOptions {
        package_manager: PackageManager::Pnpm,
        template: Template::Blog,
        typescript: TsLevel::Strictest,
        site: Some("https://blog.example.com".into()),
        ..Default::default()
    });
    let Some((t, _r, _keep)) = e2e(req) else {
        return;
    };
    assert!(t.join("pnpm-lock.yaml").is_file());
    assert!(t.join("dist/index.html").is_file());
    assert_eq!(
        json(&t.join("tsconfig.json"))["extends"],
        "astro/tsconfigs/strictest"
    );
    assert_lint_clean(&t);
}

#[test]
#[ignore = "génération réelle : ALPHARUST_E2E=1 cargo test --test generators -- --ignored"]
fn e2e_laravel_with_breeze_and_pest() {
    let _g = lock();
    let mut req = Request::new(Framework::Laravel, "e2e-laravel");
    req.options = Options::Laravel(LaravelOptions {
        starter: Starter::BreezeBlade,
        pest: true,
        dark: true,
        ..Default::default()
    });
    let Some((t, report, _keep)) = e2e(req) else {
        return;
    };
    assert!(t.join("artisan").is_file() && t.join("vendor/autoload.php").is_file());
    assert!(
        t.join("public/build/manifest.json").is_file(),
        "front compilé"
    );
    assert!(t.join("routes/pages.php").is_file());
    assert!(report.steps.iter().find(|s| s.id == "test").unwrap().ok);
    assert_lint_clean(&t);
}

#[test]
#[ignore = "génération réelle : ALPHARUST_E2E=1 cargo test --test generators -- --ignored"]
fn e2e_wagtail_in_a_venv() {
    let _g = lock();
    let Some((t, report, _keep)) = e2e(Request::new(Framework::Wagtail, "e2e-cms")) else {
        return;
    };
    assert!(t.join(".venv/bin/python").is_file() && t.join("manage.py").is_file());
    assert!(
        t.join("e2e_cms/settings/base.py").is_file(),
        "créé par `wagtail start`"
    );
    for id in ["check", "migrations", "test"] {
        assert!(report.steps.iter().find(|s| s.id == id).unwrap().ok, "{id}");
    }
    assert_lint_clean(&t);
}

#[test]
#[ignore = "génération réelle : ALPHARUST_E2E=1 cargo test --test generators -- --ignored"]
fn e2e_laravel_design_kit_builds_with_vite_and_passes_its_tests() {
    let _g = lock();
    let mut req = Request::new(Framework::Laravel, "e2e-lv-kit");
    req.options = Options::Laravel(LaravelOptions {
        ui: true,
        ..Default::default()
    });
    let Some((t, report, _keep)) = e2e(req) else {
        return;
    };
    assert!(
        t.join("public/build/manifest.json").is_file(),
        "Vite a compilé le kit"
    );
    let css: String = std::fs::read_dir(t.join("public/build/assets"))
        .unwrap()
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "css"))
        .map(|e| read(&e.path()))
        .collect();
    assert!(
        css.contains("data-slot"),
        "les composants sont dans la feuille compilée"
    );
    // les composants étendus (CSS pur, mêmes slots qu'atomic) font partie de la même feuille
    for slot in ["select-content", "sidebar-provider", "toast-viewport"] {
        assert!(css.contains(slot), "slot étendu {slot} compilé");
    }
    for id in ["npm-build", "test"] {
        assert!(report.steps.iter().find(|s| s.id == id).unwrap().ok, "{id}");
    }
    // les 46 composants Blade (core + étendus) et la page avancée compilent tous
    assert!(t
        .join("resources/views/pages/components-advanced.blade.php")
        .is_file());
    assert!(t.join("app/View/DesignKit.php").is_file());
    let parts = walk_count(&t.join("resources/views/components/ds"), "blade.php");
    assert!(
        parts >= 184,
        "parties Blade (70 core + 114 étendues) : {parts}"
    );
    let cache = std::process::Command::new("php")
        .args(["artisan", "view:cache"])
        .current_dir(&t)
        .output()
        .expect("php artisan view:cache");
    assert!(
        cache.status.success(),
        "view:cache : {}",
        String::from_utf8_lossy(&cache.stderr)
    );
    assert_lint_clean(&t);
}

fn walk_count(dir: &Path, suffix: &str) -> usize {
    std::fs::read_dir(dir)
        .unwrap()
        .flatten()
        .map(|e| {
            let p = e.path();
            if p.is_dir() {
                walk_count(&p, suffix)
            } else {
                usize::from(p.to_string_lossy().ends_with(suffix))
            }
        })
        .sum()
}

#[test]
#[ignore = "génération réelle : ALPHARUST_E2E=1 cargo test --test generators -- --ignored"]
fn e2e_wagtail_design_kit_serves_components_and_passes_its_tests() {
    let _g = lock();
    let mut req = Request::new(Framework::Wagtail, "e2e-wg-kit");
    req.options = Options::Wagtail(WagtailOptions {
        ui: true,
        ..Default::default()
    });
    let Some((t, report, _keep)) = e2e(req) else {
        return;
    };
    assert!(t.join("ds/static/ds/css/index.css").is_file());
    for f in ["select", "sidebar", "toast", "utilities"] {
        assert!(
            t.join(format!("ds/static/ds/css/components/{f}.css"))
                .is_file(),
            "{f}.css"
        );
    }
    assert!(read(&t.join("e2e_wg_kit/templates/base.html")).contains("ds/css/index.css"));
    for id in ["design-wire", "check", "test"] {
        assert!(report.steps.iter().find(|s| s.id == id).unwrap().ok, "{id}");
    }
    // les 46 composants Django : balises core et étendues enregistrées, page avancée rendue sans erreur
    assert!(t.join("ds/templates/ds/components_advanced.html").is_file());
    let probe = std::process::Command::new(t.join(".venv/bin/python"))
        .args([
            "manage.py",
            "shell",
            "-c",
            "from ds.templatetags.ds import register; from django.test import Client; \
             r = Client().get('/components/advanced/'); \
             print('PROBE', len(register.tags), r.status_code, r.content.decode().count('data-slot='))",
        ])
        .current_dir(&t)
        .output()
        .expect("manage.py shell");
    let out = String::from_utf8_lossy(&probe.stdout).to_string();
    let n: Vec<usize> = out
        .lines()
        .find_map(|l| l.strip_prefix("PROBE "))
        .unwrap_or_default()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    assert!(
        n.len() == 3 && n[0] >= 180 && n[1] == 200 && n[2] >= 250,
        "balises / page avancée : {out} {}",
        String::from_utf8_lossy(&probe.stderr)
    );
    assert_lint_clean(&t);
}

fn step_ok(report: &Report, id: &str) {
    let s = report
        .steps
        .iter()
        .find(|s| s.id == id)
        .unwrap_or_else(|| panic!("étape {id} absente"));
    assert!(s.ok, "étape {id} en échec : {:?}", s.note);
}

#[test]
#[ignore = "génération réelle : ALPHARUST_E2E=1 cargo test --test generators -- --ignored"]
fn e2e_astro_platform_sqlite_auth_starwind_i18n() {
    let _g = lock();
    let mut req = Request::new(Framework::Astro, "e2e-platform");
    req.options = Options::Astro(AstroOptions {
        ui: UiKit::Starwind,
        database: Db::Sqlite,
        auth: true,
        auth_plugins: vec!["admin".into(), "username".into()],
        i18n: vec!["fr".into(), "en".into()],
        docker: true,
        ci: true,
        theme: Some(ThemeColors::default()),
        ..Default::default()
    });
    let Some((t, report, _keep)) = e2e(req) else {
        return;
    };
    for id in [
        "starwind-init",
        "starwind-config",
        "atoms-deps",
        "auth-schema",
        "db-generate",
        "db-migrate",
        "check",
        "build",
        "test",
    ] {
        step_ok(&report, id);
    }
    assert_design_layout(&t);
    assert!(read(&t.join("src/styles/themes/theme.css")).contains("--brand-primary: #4f46e5;"));
    assert!(
        walk(&t.join("src/components/atoms")) >= 200,
        "tous les composants atomic"
    );
    for f in [
        "src/components/atoms/button/Button.astro",
        "src/lib/starwind/positioning.ts",
        "src/styles/tokens/components.css",
        "src/styles/components/button.css",
        "src/styles/themes/theme.css",
        "src/pages/components.astro",
        "src/lib/auth.ts",
        "src/database/schemas/auth.schema.ts",
        "src/pages/[lang]/auth/sign-in.astro",
        "src/i18n/messages/en.json",
        "Dockerfile",
        ".github/workflows/ci.yml",
        "dist/server/entry.mjs",
    ] {
        assert!(t.join(f).exists(), "{f} attendu");
    }
    let schema = read(&t.join("src/database/schemas/auth.schema.ts"));
    assert!(
        schema.contains("sqliteTable") && schema.contains("username"),
        "schéma généré par `auth generate`"
    );
    assert!(!read(&t.join(".alpharust-project.json")).contains("BETTER_AUTH_SECRET"));
    assert_lint_clean(&t);
}

#[test]
#[ignore = "génération réelle : ALPHARUST_E2E=1 cargo test --test generators -- --ignored"]
fn e2e_astro_platform_concordia_theme_all_components() {
    let _g = lock();
    let mut req = Request::new(Framework::Astro, "e2e-concordia");
    req.options = Options::Astro(AstroOptions {
        ui: UiKit::Concordia,
        theme: Some(ThemeColors {
            primary: "#0ea5e9".into(),
            secondary: "#7c3aed".into(),
            accent: "#f59e0b".into(),
        }),
        ..Default::default()
    });
    let Some((t, report, _keep)) = e2e(req) else {
        return;
    };
    for id in ["icons-deps", "theme", "check", "build"] {
        step_ok(&report, id);
    }
    // tous les composants du dépôt Concordia sont présents
    let count = walk(&t.join("src/components/ui"));
    assert!(count >= 80, "{count} fichiers de composants Concordia");
    let theme = read(&t.join("src/styles/themes/theme.css"));
    assert!(theme.contains("--color-primary: #0ea5e9;") && theme.contains("--on-primary"));
    for v in ["retro", "modern", "futuristic"] {
        assert!(t.join(format!("src/styles/components/{v}.css")).is_file());
    }
    assert_design_layout(&t);
    assert_lint_clean(&t);
}

/// Arborescence `src/styles/` commune à tous les kits.
fn assert_design_layout(t: &Path) {
    for f in [
        "global.css",
        "base.css",
        "tokens/colors.css",
        "tokens/typography.css",
        "tokens/spacing.css",
        "tokens/components.css",
        "tokens/site.css",
        "themes/theme.css",
        "components/layout.css",
        "components/button.css",
        "components/custom.css",
    ] {
        assert!(t.join("src/styles").join(f).is_file(), "src/styles/{f}");
    }
    assert!(read(&t.join("src/layouts/BaseLayout.astro")).contains("styles/global.css"));
    assert!(read(&t.join("README.md")).contains("Personnaliser le design"));
}

/// CSS simple et Tailwind seul : même arborescence, palette de marque générée, build réel.
#[test]
#[ignore = "génération réelle : ALPHARUST_E2E=1 cargo test --test generators -- --ignored"]
fn e2e_astro_design_system_plain_and_tailwind() {
    let _g = lock();
    for (name, integrations) in [
        ("e2e-design-plain", vec![]),
        ("e2e-design-tw", vec!["tailwind".to_string()]),
    ] {
        let mut req = Request::new(Framework::Astro, name);
        req.options = Options::Astro(AstroOptions {
            integrations,
            theme: Some(ThemeColors::default()),
            ..Default::default()
        });
        let Some((t, report, _keep)) = e2e(req) else {
            return;
        };
        for id in ["theme", "check", "build"] {
            step_ok(&report, id);
        }
        assert_design_layout(&t);
        let theme = read(&t.join("src/styles/themes/theme.css"));
        assert!(
            theme.contains("--brand-primary: #4f46e5;") && theme.contains("--on-brand-primary")
        );
        assert!(t.join("scripts/theme-check.mjs").is_file());
        assert_lint_clean(&t);
    }
}

fn walk(dir: &Path) -> usize {
    std::fs::read_dir(dir)
        .map(|rd| {
            rd.flatten()
                .map(|e| {
                    if e.path().is_dir() {
                        walk(&e.path())
                    } else {
                        1
                    }
                })
                .sum()
        })
        .unwrap_or(0)
}

/// PostgreSQL réel : `ALPHARUST_E2E_PG_URL=postgresql://…/base` (la base `<base>_test` doit exister aussi).
#[test]
#[ignore = "génération réelle : ALPHARUST_E2E=1 cargo test --test generators -- --ignored"]
fn e2e_astro_platform_postgres() {
    let _g = lock();
    let Ok(url) = std::env::var("ALPHARUST_E2E_PG_URL") else {
        eprintln!("ALPHARUST_E2E_PG_URL absent : test PostgreSQL ignoré");
        return;
    };
    let mut req = Request::new(Framework::Astro, "e2e-postgres");
    req.options = Options::Astro(AstroOptions {
        database: Db::Postgres,
        db_url: Some(url),
        auth: true,
        ..Default::default()
    });
    let Some((t, report, _keep)) = e2e(req) else {
        return;
    };
    for id in ["auth-schema", "db-generate", "db-migrate", "test", "build"] {
        step_ok(&report, id);
    }
    assert!(read(&t.join("src/database/schemas/auth.schema.ts")).contains("pgTable"));
    assert!(!read(&t.join(".alpharust-project.json")).contains("://postgres:"));
}
