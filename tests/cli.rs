//! Tests de bout en bout : le vrai binaire `alpharust`.

use std::path::Path;
use std::process::{Command, Output};

fn bin() -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_alpharust"));
    c.env("ALPHARUST_NO_CACHE", "1");
    c
}

fn run(args: &[&str]) -> Output {
    bin().args(args).output().expect("binaire exécutable")
}

fn text(o: &Output) -> (String, String, i32) {
    (
        String::from_utf8_lossy(&o.stdout).into_owned(),
        String::from_utf8_lossy(&o.stderr).into_owned(),
        o.status.code().unwrap_or(-1),
    )
}

#[test]
fn version_and_help() {
    let (out, _, code) = text(&run(&["--version"]));
    assert_eq!(code, 0);
    assert!(out.starts_with("alpharust "));
    let (out, _, code) = text(&run(&["--help"]));
    assert_eq!(code, 0);
    for cmd in [
        "lint",
        "list",
        "generate",
        "init",
        "tui",
        "gui",
        "check-rules",
        "cache",
    ] {
        assert!(out.contains(cmd), "{} absent de l'aide", cmd);
    }
}

#[test]
fn list_rules_profiles_and_checkers() {
    let (out, _, code) = text(&run(&["list"]));
    assert_eq!(code, 0);
    assert!(out.contains("règles") && out.contains("core/no-eval"));
    let (out, _, _) = text(&run(&["list", "--profiles"]));
    for p in [
        "core",
        "strict",
        "web",
        "ecommerce",
        "cms",
        "starter",
        "secrets",
    ] {
        assert!(out.contains(p), "profil {} absent", p);
    }
    let (out, _, _) = text(&run(&["list", "--checkers"]));
    assert!(
        out.contains("html-img-alt") && out.contains("rust-unsafe") && out.contains("css-contrast")
    );
}

#[test]
fn lint_exit_codes_and_profiles() {
    let (out, _, code) = text(&run(&[
        "lint",
        "tests/fixtures/violations",
        "--profile",
        "strict",
    ]));
    assert_eq!(code, 1, "des erreurs → code 1");
    assert!(out.contains("no-eval") && out.contains("error(s)"));

    let (out, _, code) = text(&run(&["lint", "tests/fixtures/clean"]));
    assert_eq!(code, 0, "{}", out);
    assert!(out.contains("No violations"));

    let (_, _, code) = text(&run(&[
        "lint",
        "tests/fixtures/violations",
        "--profile",
        "core",
        "--fail-on",
        "info",
    ]));
    assert_eq!(code, 1);

    let (_, err, code) = text(&run(&["lint", "/chemin/inexistant"]));
    assert_eq!(code, 2);
    assert!(err.contains("introuvable"));

    let (_, err, code) = text(&run(&["lint", ".", "--profile", "nope"]));
    assert_eq!(code, 2);
    assert!(err.contains("introuvable") && err.contains("strict"));
}

#[test]
fn lint_sarif_and_json_outputs_are_pure_json() {
    let (out, _, code) = text(&run(&[
        "lint",
        "tests/fixtures/violations",
        "--profile",
        "core",
        "--sarif",
    ]));
    assert_eq!(code, 1);
    let json: serde_json::Value = serde_json::from_str(&out).expect("stdout = SARIF pur");
    assert_eq!(json["version"], "2.1.0");
    assert!(!json["runs"][0]["results"].as_array().unwrap().is_empty());

    let (out, _, _) = text(&run(&[
        "lint",
        "tests/fixtures/violations",
        "--profile",
        "core",
        "--json",
    ]));
    let json: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert!(json[0]["rule_id"].as_str().unwrap().starts_with("core/"));
}

#[test]
fn works_from_any_directory() {
    let dir = tempfile::tempdir().unwrap();
    let (out, _, code) = text(
        &bin()
            .current_dir(dir.path())
            .args(["list"])
            .output()
            .unwrap(),
    );
    assert_eq!(code, 0);
    assert!(
        out.contains("core/no-eval"),
        "les règles sont trouvées depuis l'exécutable"
    );
}

#[test]
fn check_rules_passes() {
    let (out, _, code) = text(&run(&["check-rules"]));
    assert_eq!(code, 0, "{}", out);
    assert!(out.contains("✓"));
}

#[test]
fn generate_each_kind_then_refuse_overwrite() {
    let dir = tempfile::tempdir().unwrap();
    for (kind, marker) in [
        ("wagtail", "manage.py"),
        ("astro", "package.json"),
        ("laravel", "artisan"),
    ] {
        let target = dir.path().join(format!("{}-demo", kind));
        let (out, _, code) = text(&run(&[
            "generate",
            kind,
            "demo",
            "--path",
            target.to_str().unwrap(),
        ]));
        assert_eq!(code, 0, "{}: {}", kind, out);
        assert!(out.contains("Suite :"));
        assert!(target.join(marker).exists());
        let (_, err, code) = text(&run(&[
            "generate",
            kind,
            "demo",
            "--path",
            target.to_str().unwrap(),
        ]));
        assert_eq!(code, 2);
        assert!(err.contains("--force"));
        let (_, _, code) = text(&run(&[
            "generate",
            kind,
            "demo",
            "--path",
            target.to_str().unwrap(),
            "--force",
        ]));
        assert_eq!(code, 0);
    }
    let init = dir.path().join("rust-demo");
    let (_, _, code) = text(&run(&[
        "init",
        "rust-demo",
        "--path",
        init.to_str().unwrap(),
    ]));
    assert_eq!(code, 0);
    assert!(init.join("Cargo.toml").exists() && init.join("tests/smoke.rs").exists());
}

#[test]
fn generate_rejects_dangerous_names() {
    for bad in ["../evil", "a/b", ".hidden", "..", ""] {
        let (_, err, code) = text(&run(&["generate", "astro", bad]));
        assert_eq!(code, 2, "{:?}", bad);
        assert!(
            err.contains("invalide") || err.contains("error"),
            "{:?}: {}",
            bad,
            err
        );
    }
    assert!(!Path::new("sites/evil").exists());
}

#[test]
fn readme_docs_checks_and_cache_commands() {
    let (out, _, code) = text(&run(&["readme", "demo"]));
    assert_eq!(code, 0);
    assert!(out.contains("## Installation") && out.contains("## Usage"));
    let (out, _, _) = text(&run(&["docs", "demo"]));
    assert!(out.starts_with("# Documentation demo"));
    let (out, _, _) = text(&run(&["checks", "--profile", "core"]));
    assert!(out.contains("- [ ] `core/no-eval`"));
    let (out, _, code) = text(&run(&["cache", "status"]));
    assert_eq!(code, 0);
    assert!(out.contains("cache"));
    let (_, _, code) = text(&run(&["readme", "../x"]));
    assert_eq!(code, 2);
}

#[test]
fn tui_refuses_to_run_without_a_terminal() {
    let (_, err, code) = text(&run(&["tui"]));
    assert_eq!(code, 2);
    assert!(err.contains("terminal"), "{}", err);
}

#[test]
fn closed_pipe_does_not_panic() {
    let mut child = bin()
        .args(["list"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    let out = child.wait_with_output().unwrap();
    assert!(!String::from_utf8_lossy(&out.stderr).contains("panicked"));
}
