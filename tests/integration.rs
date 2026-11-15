use std::path::Path;

#[test]
fn detects_violations_in_fixtures() {
    let rules = alpharust::parser::Rule::load_all().unwrap_or_default();
    assert!(!rules.is_empty(), "des règles doivent être chargées");

    let fixtures = Path::new("tests/fixtures/violations");
    if fixtures.exists() {
        let violations = alpharust::engine::runner::run(&rules, fixtures).unwrap_or_default();
        assert!(
            !violations.is_empty(),
            "au moins une violation doit être détectée dans les fixtures"
        );
    }
}
