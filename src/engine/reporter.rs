use crate::engine::Violation;

pub fn report(violations: &Vec<Violation>) {
    if violations.is_empty() {
        println!("✓ aucune violation");
        return;
    }
    let mut errors = 0u32;
    let mut warnings = 0u32;
    for v in violations {
        match v.severity.as_str() {
            "error" => errors += 1,
            _ => warnings += 1,
        }
        if v.line == 0 {
            println!("{}: {} [{}] {}", v.file, v.rule, v.severity, v.message);
        } else {
            println!("{}:{}: {} [{}] {}", v.file, v.line, v.rule, v.severity, v.message);
        }
    }
    println!();
    println!("{} erreur(s), {} avertissement(s)", errors, warnings);
}
