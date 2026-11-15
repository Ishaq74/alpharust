use super::runner::Violation;

pub fn report(violations: &[Violation]) {
    if violations.is_empty() {
        println!("No violations");
        return;
    }
    for v in violations {
        println!("{}:{}:{}: {}", v.file, v.line, v.rule, v.message);
    }
    println!("\n{} violation(s)", violations.len());
}
