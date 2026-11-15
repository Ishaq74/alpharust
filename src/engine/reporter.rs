use super::runner::Violation;

pub fn report(violations: &[Violation]) {
    if violations.is_empty() {
        println!("✓ No violations");
        return;
    }

    let mut errors = 0;
    let mut warnings = 0;

    for v in violations {
        match v.severity.as_str() {
            "error" => {
                errors += 1;
                println!("✗ {}:{}:{}: {}", v.file, v.line, v.rule, v.message);
            }
            _ => {
                warnings += 1;
                println!("! {}:{}:{}: {}", v.file, v.line, v.rule, v.message);
            }
        }
    }

    println!("\n{} error(s), {} warning(s)", errors, warnings);
}
