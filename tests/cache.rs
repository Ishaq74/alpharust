//! Cache des règles : isolé dans son propre binaire de test (variables d'environnement globales).

use alpharust::parser::{cache, Rule};
use std::fs;

#[test]
fn rules_cache_hits_and_invalidates() {
    let dir = tempfile::tempdir().unwrap();
    let cache_dir = tempfile::tempdir().unwrap();
    std::env::set_var("ALPHARUST_CACHE_DIR", cache_dir.path());
    std::env::remove_var("ALPHARUST_NO_CACHE");

    let rules = dir.path().join("rules");
    fs::create_dir_all(rules.join("core")).unwrap();
    fs::write(
        rules.join("core/a.md"),
        "---\nname: a\napply: \"*\"\nseverity: error\npattern: \"foo\"\n---\n# A\n",
    )
    .unwrap();

    let first = Rule::load_dir(&rules, true).unwrap();
    assert!(!first.from_cache);
    assert_eq!(first.rules.len(), 1);
    assert!(cache::status().0, "le cache doit être écrit");

    let second = Rule::load_dir(&rules, true).unwrap();
    assert!(second.from_cache);
    assert_eq!(second.rules[0].id, "core/a");
    assert_eq!(second.rules[0].pattern.as_deref(), Some("foo"));

    // Une modification invalide le cache.
    fs::write(
        rules.join("core/b.md"),
        "---\nname: b\napply: \"*\"\nseverity: info\npattern: \"bar\"\n---\n# B\n",
    )
    .unwrap();
    let third = Rule::load_dir(&rules, true).unwrap();
    assert!(!third.from_cache);
    assert_eq!(third.rules.len(), 2);

    // Désactivation explicite.
    let bypass = Rule::load_dir(&rules, false).unwrap();
    assert!(!bypass.from_cache);

    // Un cache corrompu est ignoré, jamais fatal.
    fs::write(cache::cache_file(), b"{ pas du json").unwrap();
    assert!(!Rule::load_dir(&rules, true).unwrap().from_cache);

    assert!(cache::clear());
    assert!(!cache::status().0);
}

#[test]
fn invalid_rules_are_reported_not_cached() {
    let dir = tempfile::tempdir().unwrap();
    let cache_dir = tempfile::tempdir().unwrap();
    std::env::set_var("ALPHARUST_CACHE_DIR", cache_dir.path());
    let rules = dir.path().join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(
        rules.join("bad.md"),
        "---\nname: x\napply: \"*\"\nseverity: nope\n---\n",
    )
    .unwrap();
    fs::write(rules.join("_template.md"), "ignoré").unwrap();
    let report = Rule::load_dir(&rules, true).unwrap();
    assert_eq!(report.errors.len(), 1);
    assert!(report.errors[0].1.contains("severity"));
    assert!(report.rules.is_empty());
}
