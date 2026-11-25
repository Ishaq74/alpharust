//! Tests d'intégration du moteur : règles réelles du dépôt contre les fixtures.

use alpharust::engine::{self, runner};
use alpharust::parser::Rule;
use std::collections::BTreeSet;
use std::path::Path;

fn rules() -> Vec<Rule> {
    Rule::load_all().expect("règles chargeables")
}

fn found(dir: &str, rules: &[Rule]) -> BTreeSet<(String, String)> {
    runner::run(rules, Path::new(dir))
        .unwrap()
        .into_iter()
        .map(|v| {
            let file = v.file.rsplit('/').next().unwrap().to_string();
            (file, v.rule)
        })
        .collect()
}

fn has(set: &BTreeSet<(String, String)>, file: &str, rule: &str) -> bool {
    set.contains(&(file.to_string(), rule.to_string()))
}

#[test]
fn loads_a_complete_rule_set() {
    let rules = rules();
    assert!(rules.len() >= 90, "{} règles", rules.len());
    let kinds: BTreeSet<_> = rules.iter().map(alpharust::parser::Rule::kind).collect();
    assert_eq!(kinds, BTreeSet::from(["checker", "info", "regex"]));
    // Les ids sont uniques.
    let ids: BTreeSet<_> = rules.iter().map(|r| &r.id).collect();
    assert_eq!(ids.len(), rules.len());
}

#[test]
fn violations_fixtures_trigger_the_expected_rules() {
    let rules = rules();
    let f = found("tests/fixtures/violations", &rules);
    let expected = [
        ("eval.ps1", "no-eval"),
        ("eval.ps1", "no-invoke-expression"),
        ("no-eval.php", "php-no-eval"),
        ("select-star.sql", "no-select-star"),
        ("hardcoded-secret.rs", "no-plaintext-secrets"),
        ("page.html", "alt-text-required"),
        ("page.html", "no-inline-styles"),
        ("style.css", "accessible-contrast"),
        ("style.css", "no-important"),
        ("unsafe.rs", "rust-no-unsafe"),
        ("package.json", "libs-consistency"),
        ("notes.md", "consistent-headings"),
        ("webhook.js", "stripe-webhook-signature"),
        ("webhook.js", "webhook-signature"),
    ];
    for (file, rule) in expected {
        assert!(
            has(&f, file, rule),
            "{file} devrait déclencher {rule} — trouvé: {f:#?}"
        );
    }
}

#[test]
fn clean_fixtures_have_zero_violations() {
    let violations = runner::run(&rules(), Path::new("tests/fixtures/clean")).unwrap();
    assert!(violations.is_empty(), "{violations:#?}");
}

#[test]
fn semantic_checkers_report_precise_lines() {
    let rules = rules();
    let v = runner::run(&rules, Path::new("tests/fixtures/violations/page.html")).unwrap();
    let line = |rule: &str| v.iter().find(|x| x.rule == rule).map(|x| x.line);
    assert_eq!(line("alt-text-required"), Some(6), "{v:#?}");
    assert_eq!(line("no-inline-styles"), Some(7));
    let v = runner::run(&rules, Path::new("tests/fixtures/violations/unsafe.rs")).unwrap();
    assert_eq!(
        v.iter().find(|x| x.rule == "rust-no-unsafe").unwrap().line,
        4
    );
}

#[test]
fn project_checkers_detect_missing_readme_and_tests() {
    let rules = rules();
    let bad = found("tests/fixtures/projects/bad-project", &rules);
    for rule in ["required-readme", "unit-tests-required", "api-docs-present"] {
        assert!(
            bad.iter().any(|(_, r)| r == rule),
            "bad-project: {rule} manquant dans {bad:?}"
        );
    }
    let good = found("tests/fixtures/projects/good-project", &rules);
    assert!(good.is_empty(), "good-project doit être propre : {good:?}");
}

#[test]
fn profiles_actually_filter_rules() {
    let core = engine::lint(Path::new("tests/fixtures/violations"), Some("core")).unwrap();
    let strict = engine::lint(Path::new("tests/fixtures/violations"), Some("strict")).unwrap();
    let every_rule = engine::composer::compose(rules());
    let all = runner::run_with(
        &every_rule,
        Path::new("tests/fixtures/violations"),
        &Default::default(),
    )
    .unwrap();
    assert!(core.rules_used >= 3 && core.rules_used < strict.rules_used);
    assert!(strict.rules_used < every_rule.len());
    assert!(
        !core.violations.is_empty(),
        "le profil core doit détecter eval"
    );
    assert!(core
        .violations
        .iter()
        .all(|v| v.rule_id.starts_with("core/")));
    assert!(all.violations.len() > strict.violations.len());
    assert!(strict.violations.len() > core.violations.len());
    assert!(engine::lint(Path::new("tests/fixtures"), Some("inconnu")).is_err());
}

#[test]
fn secrets_profile_detects_real_looking_tokens() {
    // Les jetons sont assemblés à l'exécution pour ne pas ressembler à de vrais secrets dans le dépôt.
    let dir = tempfile::tempdir().unwrap();
    let samples = [
        (
            "aws.py",
            format!("key = \"{}{}\"", "AK", "IAABCDEFGHIJKLMNOP"),
            "aws-access-key-id",
        ),
        (
            "gh.js",
            format!("t = '{}_{}'", "ghp", "a".repeat(36)),
            "github-token",
        ),
        (
            "db.env",
            format!("URL=mongodb+srv://user:p4ss{}cluster.example.net/db", "@"),
            "mongodb-connection-string",
        ),
        (
            "k.pem",
            format!("-----BEGIN {} KEY-----", "RSA PRIVATE"),
            "private-key-block",
        ),
        (
            "mail.js",
            format!("SENDGRID_API_KEY = \"{}\"", "abcdefgh12345678"),
            "sendgrid-api-key-env",
        ),
    ];
    for (name, content, _) in &samples {
        std::fs::write(dir.path().join(name), content).unwrap();
    }
    let report = engine::lint(dir.path(), Some("secrets")).unwrap();
    for (file, _, rule) in &samples {
        assert!(
            report
                .violations
                .iter()
                .any(|v| v.rule == *rule && v.file.ends_with(file)),
            "{} non détecté dans {}: {:#?}",
            rule,
            file,
            report.violations
        );
    }
    assert!(report
        .violations
        .iter()
        .all(|v| v.severity == "error" || v.severity == "warning"));
}

#[test]
fn config_file_excludes_disables_and_overrides() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.py"), "x = eval(s)\n").unwrap();
    std::fs::create_dir(dir.path().join("third")).unwrap();
    std::fs::write(dir.path().join("third/b.py"), "x = eval(s)\n").unwrap();

    let base = engine::lint(dir.path(), Some("core")).unwrap();
    assert_eq!(base.violations.len(), 2);

    std::fs::write(
        dir.path().join(".alpharust.toml"),
        "profile = \"core\"\nexclude = [\"third/**\"]\n[severity]\n\"core/no-eval\" = \"warning\"\n",
    )
    .unwrap();
    let cfg = engine::lint(dir.path(), None).unwrap();
    assert_eq!(cfg.violations.len(), 1);
    assert_eq!(cfg.violations[0].severity, "warning");
    assert_eq!(cfg.exit_code(None), 0, "warning < error");
    assert_eq!(cfg.exit_code(Some("warning")), 1);

    std::fs::write(
        dir.path().join(".alpharust.toml"),
        "profile = \"core\"\ndisable = [\"core/no-eval\"]\n",
    )
    .unwrap();
    assert!(engine::lint(dir.path(), None)
        .unwrap()
        .violations
        .is_empty());

    std::fs::write(dir.path().join(".alpharust.toml"), "profil = \"core\"\n").unwrap();
    assert!(
        engine::lint(dir.path(), None).is_err(),
        "clé inconnue refusée"
    );
}

#[test]
fn composer_prefers_specific_rules_when_names_collide() {
    let rules = engine::composer::compose(rules());
    let names: Vec<_> = rules.iter().map(|r| r.name.clone()).collect();
    let unique: BTreeSet<_> = names.iter().collect();
    assert_eq!(
        names.len(),
        unique.len(),
        "noms de règles dupliqués après composition"
    );
}

#[test]
fn sarif_is_valid_for_real_findings() {
    let report = engine::lint(Path::new("tests/fixtures/violations"), Some("strict")).unwrap();
    let sarif = alpharust::sarif::to_sarif(&report.violations);
    let json: serde_json::Value = serde_json::from_str(&sarif).unwrap();
    assert_eq!(json["version"], "2.1.0");
    let run = &json["runs"][0];
    let rules = run["tool"]["driver"]["rules"].as_array().unwrap();
    let results = run["results"].as_array().unwrap();
    assert_eq!(results.len(), report.violations.len());
    for r in results {
        let idx = r["ruleIndex"].as_u64().unwrap() as usize;
        assert_eq!(rules[idx]["id"], r["ruleId"]);
        assert!(["error", "warning", "note"].contains(&r["level"].as_str().unwrap()));
        assert!(
            r["locations"][0]["physicalLocation"]["region"]["startLine"]
                .as_u64()
                .unwrap()
                >= 1
        );
    }
}

#[test]
fn repository_passes_its_own_lint() {
    let report = engine::lint(Path::new("."), None).unwrap();
    assert_eq!(report.count("error"), 0, "{:#?}", report.violations);
    assert_eq!(report.profile.as_deref(), Some("strict"));
}
