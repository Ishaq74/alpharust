use crate::parser::Rule;

/// Applique un ordre de priorité simple.
pub fn compose(mut rules: Vec<Rule>) -> Vec<Rule> {
    rules.sort_by(|a, b| {
        let score = |r: &Rule| {
            let p = r.path.to_string_lossy();
            if p.contains("frameworks") { 4 }
            else if p.contains("domains") { 3 }
            else if p.contains("languages") { 2 }
            else { 1 }
        };
        score(b).cmp(&score(a))
    });
    rules
}
