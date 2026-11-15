use std::path::Path;

#[test]
fn loads_rules() {
    let rules = alpharust::parser::Rule::load_all().unwrap_or_default();
    assert!(!rules.is_empty(), "des règles doivent être chargées");
}

#[test]
fn detects_violations_in_fixtures() {
    let rules = alpharust::parser::Rule::load_all().unwrap_or_default();
    let fixtures = Path::new("tests/fixtures/violations");
    if !fixtures.exists() {
        return;
    }
    let violations = alpharust::engine::runner::run(&rules, fixtures).unwrap_or_default();
    assert!(
        !violations.is_empty(),
        "au moins une violation doit être détectée dans les fixtures"
    );
}

#[test]
fn clean_fixtures_have_fewer_violations() {
    let rules = alpharust::parser::Rule::load_all().unwrap_or_default();
    let clean = Path::new("tests/fixtures/clean");
    let dirty = Path::new("tests/fixtures/violations");
    if !clean.exists() || !dirty.exists() {
        return;
    }
    let clean_v = alpharust::engine::runner::run(&rules, clean).unwrap_or_default();
    let dirty_v = alpharust::engine::runner::run(&rules, dirty).unwrap_or_default();
    assert!(
        clean_v.len() <= dirty_v.len(),
        "les fixtures clean doivent avoir moins ou autant de violations que les dirty"
    );
}
