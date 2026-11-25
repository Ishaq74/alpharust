use super::runner::Violation;

/// Rendu texte (sans couleur) des violations.
pub fn render(violations: &[Violation]) -> String {
    if violations.is_empty() {
        return "✓ No violations\n".to_string();
    }
    let (mut errors, mut warnings, mut infos) = (0, 0, 0);
    let mut out = String::new();
    for v in violations {
        let icon = match v.severity.as_str() {
            "error" => {
                errors += 1;
                "✗"
            }
            "warning" => {
                warnings += 1;
                "!"
            }
            _ => {
                infos += 1;
                "i"
            }
        };
        out.push_str(&format!(
            "{} {}:{}:{}: {}\n",
            icon, v.file, v.line, v.rule, v.message
        ));
    }
    out.push_str(&format!(
        "\n{errors} error(s), {warnings} warning(s), {infos} info(s)\n"
    ));
    out
}

pub fn report(violations: &[Violation]) {
    print!("{}", render(violations));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(sev: &str) -> Violation {
        Violation {
            file: "a.rs".into(),
            line: 3,
            rule: "r".into(),
            rule_id: "core/r".into(),
            message: "m".into(),
            severity: sev.into(),
        }
    }

    #[test]
    fn empty_and_counts() {
        assert_eq!(render(&[]), "✓ No violations\n");
        let out = render(&[v("error"), v("warning"), v("info")]);
        assert!(out.contains("✗ a.rs:3:r: m"));
        assert!(out.contains("1 error(s), 1 warning(s), 1 info(s)"));
    }
}
