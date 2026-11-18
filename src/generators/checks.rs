use crate::parser::Rule;

/// Checklist Markdown des règles actives (une case par règle exécutable ou documentaire).
pub fn generate(title: &str, rules: &[Rule]) -> String {
    let mut out = format!("# Checks — {}\n\n{} règle(s).\n\n", title, rules.len());
    for severity in ["error", "warning", "info"] {
        let group: Vec<&Rule> = rules.iter().filter(|r| r.severity == severity).collect();
        if group.is_empty() {
            continue;
        }
        out.push_str(&format!("## {} ({})\n\n", severity, group.len()));
        for r in group {
            out.push_str(&format!(
                "- [ ] `{}` — {} _(apply: {}, {})_\n",
                r.id,
                r.display_message(),
                r.apply,
                r.kind()
            ));
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_by_severity() {
        let rules = Rule::load_all().unwrap();
        let md = generate("tout", &rules);
        assert!(md.contains("## error"));
        assert!(md.contains("- [ ] `core/no-eval`"));
        assert!(md.contains(&format!("{} règle(s)", rules.len())));
    }
}
