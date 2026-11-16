use crate::parser::Rule;
use std::collections::HashSet;

/// Priorité d'une règle selon sa couche : framework > domaine > langage > core.
pub fn priority(rule: &Rule) -> u8 {
    match rule.id.split('/').next().unwrap_or("") {
        "frameworks" => 4,
        "domains" => 3,
        "languages" => 2,
        "markdown" => 2,
        _ => 1,
    }
}

/// Trie les règles par priorité décroissante (ordre stable) et supprime les
/// doublons de nom : en cas de conflit, la règle la plus spécifique l'emporte.
pub fn compose(mut rules: Vec<Rule>) -> Vec<Rule> {
    rules.sort_by_key(|r| std::cmp::Reverse(priority(r)));
    let mut seen = HashSet::new();
    rules.retain(|r| seen.insert(r.name.clone()));
    rules
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn rule(id: &str, name: &str) -> Rule {
        let mut r = Rule::from_markdown(
            Path::new("x.md"),
            &format!("---\nname: {}\napply: \"*\"\nseverity: error\n---\n", name),
        )
        .unwrap();
        r.id = id.to_string();
        r
    }

    #[test]
    fn most_specific_layer_first() {
        let out = compose(vec![
            rule("core/a", "a"),
            rule("frameworks/x/b", "b"),
            rule("languages/y/c", "c"),
            rule("domains/z/d", "d"),
        ]);
        let ids: Vec<_> = out.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(
            ids,
            ["frameworks/x/b", "domains/z/d", "languages/y/c", "core/a"]
        );
    }

    #[test]
    fn same_name_keeps_most_specific() {
        let out = compose(vec![
            rule("core/no-eval", "no-eval"),
            rule("languages/js/no-eval", "no-eval"),
        ]);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, "languages/js/no-eval");
    }
}
