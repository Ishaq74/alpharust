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
        "new",
        "doctor",
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
fn generate_dry_run_prints_the_official_commands_for_each_framework() {
    for (fw, expect) in [
        ("astro", "npm create astro@5.3.2 demo"),
        ("laravel", "composer create-project laravel/laravel"),
        ("wagtail", "wagtail start"),
        ("rust", "cargo new"),
    ] {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("demo");
        let (out, err, code) = text(&run(&[
            "generate",
            fw,
            "demo",
            "--path",
            target.to_str().unwrap(),
            "--dry-run",
        ]));
        // code 1 = outil manquant sur cette machine ; le plan est affiché dans les deux cas
        assert!(code == 0 || code == 1, "{fw}: {code} {err}");
        assert!(out.contains(expect), "{fw}: {out}");
        assert!(out.contains("Outils détectés"), "{fw}: {out}");
        assert!(!target.exists(), "--dry-run n'écrit rien");
    }
}

#[test]
fn generate_dry_run_json_is_machine_readable() {
    let (out, _, _) = text(&run(&[
        "generate",
        "astro",
        "demo",
        "--dry-run",
        "--json",
        "--pm",
        "pnpm",
        "-i",
        "tailwind,mdx",
        "--ts",
        "strictest",
    ]));
    let v: serde_json::Value =
        serde_json::from_str(out.trim()).unwrap_or_else(|e| panic!("{e}: {out}"));
    let cmds: Vec<&str> = v["steps"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|s| s["command"].as_str())
        .collect();
    assert!(
        cmds.iter()
            .any(|c| c.starts_with("pnpm create astro@5.3.2 demo")),
        "{cmds:?}"
    );
    assert!(
        cmds.iter()
            .any(|c| c == &"pnpm astro add tailwind mdx --yes"),
        "{cmds:?}"
    );
    assert!(v["tools"]
        .as_array()
        .unwrap()
        .iter()
        .any(|t| t["id"] == "node"));
}

#[test]
fn generate_rejects_options_of_another_framework_and_bad_values() {
    let (_, err, code) = text(&run(&[
        "generate",
        "rust",
        "x",
        "--pm",
        "pnpm",
        "--dry-run",
    ]));
    assert_eq!(code, 2);
    assert!(err.contains("astro") && err.contains("--pm"), "{err}");
    let (_, err, code) = text(&run(&[
        "generate",
        "astro",
        "x",
        "--starter",
        "none",
        "--dry-run",
    ]));
    assert_eq!(code, 2);
    assert!(err.contains("laravel"), "{err}");
    let (_, err, code) = text(&run(&[
        "generate",
        "astro",
        "x",
        "--pm",
        "pip",
        "--dry-run",
    ]));
    assert_eq!(code, 2);
    assert!(err.contains("npm, pnpm, yarn, bun"), "{err}");
    let (_, err, code) = text(&run(&[
        "generate",
        "astro",
        "x",
        "-i",
        "sitemap",
        "--dry-run",
    ]));
    assert_eq!(code, 2);
    assert!(err.contains("--site"), "{err}");
    let (_, err, code) = text(&run(&[
        "generate",
        "astro",
        "x",
        "-i",
        "react,vue",
        "--dry-run",
    ]));
    assert_eq!(code, 2);
    assert!(err.contains("un seul framework"), "{err}");
    let (_, err, code) = text(&run(&["generate", "laravel", "x", "--pest", "--dry-run"]));
    assert_eq!(code, 2);
    assert!(err.contains("kit de démarrage"), "{err}");
    let (_, _, code) = text(&run(&[
        "generate",
        "wagtail",
        "x",
        "--no-install",
        "--dry-run",
    ]));
    assert_eq!(code, 2);
}

#[test]
fn generate_rust_for_real_then_refuse_overwrite_unless_forced() {
    if which_cargo().is_none() {
        eprintln!("cargo absent : test ignoré");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("rust-demo");
    let t = target.to_str().unwrap();
    let args = [
        "generate",
        "rust",
        "rust-demo",
        "--path",
        t,
        "--no-verify",
        "--no-git",
    ];
    let (out, _, code) = text(&run(&args));
    assert_eq!(code, 0, "{out}");
    assert!(
        out.contains("Suite :") && out.contains("cargo run"),
        "{out}"
    );
    assert!(target.join("Cargo.toml").exists() && target.join("tests/smoke.rs").exists());
    assert!(target.join(".alpharust-project.json").exists());
    // l'étape de création est bien l'outil officiel
    let manifest = std::fs::read_to_string(target.join(".alpharust-project.json")).unwrap();
    assert!(
        manifest.contains("\"official_tool\": \"cargo new\""),
        "{manifest}"
    );

    let (out, err, code) = text(&run(&args));
    assert_eq!(code, 1, "échec d'exécution = code 1 : {out}{err}");
    assert!(
        out.contains("--force") || err.contains("--force"),
        "{out}{err}"
    );
    assert!(
        !err.contains("projet partiel"),
        "rien de partiel : la destination n'a pas été touchée"
    );
    let mut forced = args.to_vec();
    forced.push("--force");
    let (_, _, code) = text(&run(&forced));
    assert_eq!(code, 0);

    // `init` est un alias de `generate rust`
    let init = dir.path().join("rust-init");
    let (_, _, code) = text(&run(&[
        "init",
        "rust-init",
        "--path",
        init.to_str().unwrap(),
        "--no-verify",
        "--no-git",
    ]));
    assert_eq!(code, 0);
    assert!(init.join("Cargo.toml").exists());
}

#[test]
fn generate_json_streams_events_one_per_line() {
    if which_cargo().is_none() {
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("evt");
    let (out, _, code) = text(&run(&[
        "generate",
        "rust",
        "evt",
        "--path",
        target.to_str().unwrap(),
        "--no-verify",
        "--no-git",
        "--json",
    ]));
    assert_eq!(code, 0, "{out}");
    let types: Vec<String> = out
        .lines()
        .map(|l| {
            serde_json::from_str::<serde_json::Value>(l).unwrap_or_else(|e| panic!("{e}: {l}"))
                ["type"]
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect();
    assert_eq!(types.first().map(String::as_str), Some("preflight"));
    assert_eq!(types.last().map(String::as_str), Some("done"));
    assert!(types.contains(&"step_start".to_string()) && types.contains(&"step_end".to_string()));
}

fn which_cargo() -> Option<std::path::PathBuf> {
    alpharust::scaffold::tools::which("cargo")
}

#[test]
fn doctor_lists_toolchains() {
    let (out, _, code) = text(&run(&["doctor"]));
    assert_eq!(code, 0, "{out}");
    for needle in ["Astro", "Laravel", "Wagtail", "Rust", "Node.js", "Cargo"] {
        assert!(out.contains(needle), "{needle} absent : {out}");
    }
    let (out, _, code) = text(&run(&["doctor", "--framework", "rust"]));
    assert!(code == 0 || code == 1);
    assert!(out.contains("Cargo") && !out.contains("Node.js"), "{out}");
}

#[test]
fn doctor_reports_missing_tools_with_hints() {
    let empty = tempfile::tempdir().unwrap();
    let o = bin()
        .env("PATH", empty.path())
        .args(["doctor", "--framework", "astro"])
        .output()
        .unwrap();
    let (out, _, code) = text(&o);
    assert_eq!(code, 1, "{out}");
    assert!(
        out.contains("outils manquants") && out.contains("nodejs.org"),
        "{out}"
    );
}

#[test]
fn new_wizard_requires_a_terminal() {
    let (_, err, code) = text(&run(&["new"]));
    assert_eq!(code, 2);
    assert!(
        err.contains("interactif") && err.contains("generate"),
        "{err}"
    );
}

#[test]
fn generate_rejects_dangerous_names() {
    for bad in ["../evil", "a/b", ".hidden", "..", ""] {
        let (_, err, code) = text(&run(&["generate", "astro", bad, "--dry-run"]));
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
