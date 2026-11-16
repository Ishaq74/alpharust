//! Les projets générés doivent être complets, sans marqueur résiduel, et passer leurs propres règles.

use alpharust::engine;
use alpharust::generators::{self, Kind};
use std::path::Path;
use std::process::Command;

fn generate(
    kind: Kind,
    name: &str,
) -> (tempfile::TempDir, std::path::PathBuf, generators::Generated) {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join(name);
    let g = generators::generate(kind, &target, name, false).unwrap();
    (dir, target, g)
}

fn assert_no_markers(target: &Path, files: &[String]) {
    for f in files {
        assert!(!f.contains("@@"), "chemin non substitué : {}", f);
        let content = std::fs::read_to_string(target.join(f)).unwrap();
        assert!(!content.contains("@@"), "marqueur non substitué dans {}", f);
    }
}

fn has_all(target: &Path, files: &[&str]) {
    for f in files {
        assert!(target.join(f).exists(), "{} manquant", f);
    }
}

fn lint_clean(target: &Path, profile: &str) {
    let report = engine::lint(target, Some(profile)).unwrap();
    assert!(
        report.violations.is_empty(),
        "profil {} : {:#?}",
        profile,
        report.violations
    );
}

#[test]
fn wagtail_project_is_complete() {
    let (_d, t, g) = generate(Kind::Wagtail, "mon-cms");
    assert!(g.files.len() >= 20, "{} fichiers", g.files.len());
    has_all(
        &t,
        &[
            "manage.py",
            "requirements.txt",
            "README.md",
            ".gitignore",
            ".env.example",
            "mon_cms/settings/base.py",
            "mon_cms/settings/dev.py",
            "mon_cms/settings/production.py",
            "mon_cms/urls.py",
            "mon_cms/wsgi.py",
            "mon_cms/templates/base.html",
            "home/models.py",
            "home/migrations/0001_initial.py",
            "home/migrations/0002_create_homepage.py",
            "home/templates/home/home_page.html",
            "home/tests.py",
        ],
    );
    assert_no_markers(&t, &g.files);
    assert!(std::fs::read_to_string(t.join("manage.py"))
        .unwrap()
        .contains("mon_cms.settings.dev"));
    lint_clean(&t, "cms");
    lint_clean(&t, "strict");
}

#[test]
fn astro_project_is_complete() {
    let (_d, t, g) = generate(Kind::Astro, "mon-astro");
    has_all(
        &t,
        &[
            "package.json",
            "astro.config.mjs",
            "tsconfig.json",
            "src/content.config.ts",
            "src/layouts/Layout.astro",
            "src/pages/index.astro",
            "src/pages/blog/[id].astro",
            "src/content/blog/premier-article.md",
            "README.md",
            ".gitignore",
        ],
    );
    assert_no_markers(&t, &g.files);
    let pkg: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(t.join("package.json")).unwrap()).unwrap();
    assert_eq!(pkg["name"], "mon-astro");
    assert!(pkg["scripts"]["build"].is_string());
    lint_clean(&t, "web");
    lint_clean(&t, "strict");
}

#[test]
fn laravel_project_is_complete() {
    let (_d, t, g) = generate(Kind::Laravel, "mon-laravel");
    has_all(
        &t,
        &[
            "composer.json",
            "artisan",
            "bootstrap/app.php",
            "public/index.php",
            "routes/web.php",
            "app/Http/Controllers/PageController.php",
            "app/Models/Page.php",
            "database/migrations/2026_01_01_000000_create_pages_table.php",
            "resources/views/layouts/app.blade.php",
            "tests/Feature/PageTest.php",
            "phpunit.xml",
            ".env.example",
            "storage/logs/.gitignore",
        ],
    );
    assert_no_markers(&t, &g.files);
    let composer: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(t.join("composer.json")).unwrap()).unwrap();
    assert_eq!(composer["name"], "app/mon-laravel");
    lint_clean(&t, "web");
    lint_clean(&t, "strict");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_ne!(
            std::fs::metadata(t.join("artisan"))
                .unwrap()
                .permissions()
                .mode()
                & 0o111,
            0,
            "artisan exécutable"
        );
    }
}

#[test]
fn bootstrap_project_builds_and_passes_its_tests() {
    let (d, t, g) = generate(Kind::Bootstrap, "demo-app");
    has_all(
        &t,
        &[
            "Cargo.toml",
            "src/main.rs",
            "src/lib.rs",
            "tests/smoke.rs",
            "README.md",
            "docs/index.md",
            ".github/workflows/ci.yml",
            "LICENSE",
            ".alpharust.toml",
        ],
    );
    assert_no_markers(&t, &g.files);
    lint_clean(&t, "core");
    lint_clean(&t, "starter");
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let out = Command::new(cargo)
        .args(["test", "--quiet"])
        .current_dir(&t)
        .env("CARGO_TARGET_DIR", d.path().join("target"))
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("test result: ok"), "{}", stdout);
}

#[test]
fn generated_python_and_php_are_syntactically_valid() {
    let (_d, w, _) = generate(Kind::Wagtail, "syn-check");
    if let Ok(out) = Command::new("python3")
        .args(["-m", "compileall", "-q", "."])
        .current_dir(&w)
        .output()
    {
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let (_d2, l, _) = generate(Kind::Laravel, "syn-check");
    if Command::new("php")
        .arg("-v")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        for f in [
            "artisan",
            "routes/web.php",
            "app/Models/Page.php",
            "app/Http/Controllers/PageController.php",
        ] {
            let out = Command::new("php")
                .args(["-l", f])
                .current_dir(&l)
                .output()
                .unwrap();
            assert!(
                out.status.success(),
                "{}: {}",
                f,
                String::from_utf8_lossy(&out.stdout)
            );
        }
    }
}

#[test]
fn names_are_sanitised_for_each_ecosystem() {
    let (_d, t, _) = generate(Kind::Wagtail, "9-Mon.Site");
    assert!(t.join("site_9_mon_site/settings/base.py").exists());
    assert!(generators::generate(Kind::Astro, &_d.path().join("x"), "../x", false).is_err());
}

#[test]
fn refuses_to_overwrite_without_force() {
    let dir = tempfile::tempdir().unwrap();
    let t = dir.path().join("p");
    generators::generate(Kind::Astro, &t, "p", false).unwrap();
    std::fs::write(t.join("package.json"), "MODIFIÉ").unwrap();
    assert!(generators::generate(Kind::Astro, &t, "p", false).is_err());
    assert_eq!(
        std::fs::read_to_string(t.join("package.json")).unwrap(),
        "MODIFIÉ"
    );
    generators::generate(Kind::Astro, &t, "p", true).unwrap();
    assert_ne!(
        std::fs::read_to_string(t.join("package.json")).unwrap(),
        "MODIFIÉ"
    );
}
