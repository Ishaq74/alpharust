use super::runner::Violation;

pub fn report(violations: &[Violation]) {
    if violations.is_empty() {
        println!("No violations found.");
        return;
    }
    for v in violations {
        println!("{}:{}:{}: {}", v.file, v.line, v.rule, v.message);
    }
}
