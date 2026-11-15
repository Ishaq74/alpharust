use super::runner::Violation;

pub fn report(violations: &[Violation]) {
    if violations.is_empty() {
        println!("✓ No violations");
        return;
    }
    let errors = violations.iter().filter(|v| v.severity == "error").count();
    let warnings = violations.len() - errors;
    for v in violations {
        let icon = if v.severity == "error" { "✗" } else { "!" };
        println!("{} {}:{}:{}: {}", icon, v.file, v.line, v.rule, v.message);
    }
    println!("\n{} error(s), {} warning(s)", errors, warnings);
}
