//! Validation des règles et profils (`alpharust check-rules`) : YAML valide, regex compilables,
//! checkers connus, exemples (`examples.bad` / `examples.good`) cohérents avec le comportement réel,
//! motifs de profils non orphelins.

use crate::checkers::{self, Args, Checker, FileCtx};
use crate::parser::Rule;
use crate::profiles::Profile;
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Default)]
pub struct CheckReport {
    pub rules: usize,
    pub profiles: usize,
    pub examples: usize,
    pub problems: Vec<String>,
}

/// Retire le marqueur `¦` (anti-scanners de secrets) des exemples.
fn clean(example: &str) -> String {
    example.replace('¦', "")
}

fn check_examples(rule: &Rule, report: &mut CheckReport) {
    let ex = &rule.examples;
    if ex.bad.is_empty() && ex.good.is_empty() {
        return;
    }
    let id = &rule.id;
    let file = ex.file.clone().unwrap_or_else(|| "example.txt".to_string());
    if let Some(p) = &rule.pattern {
        let Ok(re) = Regex::new(p) else { return };
        for bad in &ex.bad {
            report.examples += 1;
            let text = clean(bad);
            if !text.lines().any(|l| re.is_match(l)) {
                report.problems.push(format!(
                    "{id}: l'exemple `bad` n'est pas détecté : {text:?}"
                ));
            }
        }
        for good in &ex.good {
            report.examples += 1;
            let text = clean(good);
            if text.lines().any(|l| re.is_match(l)) {
                report.problems.push(format!(
                    "{id}: l'exemple `good` est détecté à tort : {text:?}"
                ));
            }
        }
    } else if let Some(Checker::File(check)) = rule.checker.as_deref().and_then(checkers::lookup) {
        let run = |text: &str| {
            check(&FileCtx {
                path: Path::new(&file),
                content: text,
                args: Args(&rule.args),
            })
        };
        for bad in &ex.bad {
            report.examples += 1;
            if run(&clean(bad)).is_empty() {
                report
                    .problems
                    .push(format!("{id}: l'exemple `bad` n'est pas détecté : {bad:?}"));
            }
        }
        for good in &ex.good {
            report.examples += 1;
            let found = run(&clean(good));
            if !found.is_empty() {
                report.problems.push(format!(
                    "{}: l'exemple `good` est détecté à tort ({}) : {:?}",
                    id, found[0].message, good
                ));
            }
        }
    } else {
        report.problems.push(format!(
            "{id}: `examples` n'est supporté que pour les règles regex et checkers fichier"
        ));
    }
}

pub fn check_all() -> CheckReport {
    let mut report = CheckReport::default();
    let loaded = match Rule::load_report(false) {
        Ok(l) => l,
        Err(e) => {
            report
                .problems
                .push(format!("chargement des règles impossible : {e:#}"));
            return report;
        }
    };
    for (path, err) in &loaded.errors {
        report.problems.push(format!("{}: {}", path.display(), err));
    }
    report.rules = loaded.rules.len();

    let mut names = HashSet::new();
    for rule in &loaded.rules {
        if !names.insert(rule.name.clone()) {
            // Même nom dans deux couches : légitime (le composer garde la plus spécifique),
            // mais interdit dans la même couche.
            let same_layer = loaded
                .rules
                .iter()
                .filter(|r| r.name == rule.name)
                .filter(|r| r.id.split('/').next() == rule.id.split('/').next())
                .count();
            if same_layer > 1 {
                report.problems.push(format!(
                    "{}: nom `{}` dupliqué dans la même couche",
                    rule.id, rule.name
                ));
            }
        }
        check_examples(rule, &mut report);
    }

    match Profile::list() {
        Ok(list) => {
            report.profiles = list.len();
            for name in list {
                match Profile::load(&name) {
                    Ok(p) => {
                        for pattern in p.unmatched_patterns(&loaded.rules) {
                            report.problems.push(format!(
                                "profil {name}: le motif `{pattern}` ne correspond à aucune règle"
                            ));
                        }
                    }
                    Err(e) => report.problems.push(format!("profil {name}: {e:#}")),
                }
            }
        }
        Err(e) => report.problems.push(format!("profils illisibles : {e:#}")),
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_rules_and_profiles_are_valid() {
        let report = check_all();
        assert!(
            report.problems.is_empty(),
            "problèmes: {:#?}",
            report.problems
        );
        assert!(
            report.rules >= 90,
            "au moins 90 règles attendues, {} trouvées",
            report.rules
        );
        assert!(
            report.examples >= 60,
            "au moins 60 exemples auto-testés, {} trouvés",
            report.examples
        );
    }

    #[test]
    fn examples_mismatch_is_reported() {
        let md = "---\nname: x\napply: \"*\"\nseverity: error\npattern: \"foo\"\nexamples:\n  bad: [\"nothing\"]\n  good: [\"foo\"]\n---\n";
        let rule = Rule::from_markdown(Path::new("x.md"), md).unwrap();
        let mut report = CheckReport::default();
        check_examples(&rule, &mut report);
        assert_eq!(report.problems.len(), 2);
    }
}
