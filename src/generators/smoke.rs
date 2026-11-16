/// Test d'intégration de base pour un projet Rust généré : le binaire doit se lancer.
pub fn smoke_test(crate_name: &str) -> String {
    format!(
        "use std::process::Command;\n\n#[test]\nfn binary_prints_greeting() {{\n    let out = Command::new(env!(\"CARGO_BIN_EXE_{bin}\"))\n        .output()\n        .expect(\"le binaire doit se lancer\");\n    assert!(out.status.success());\n    let stdout = String::from_utf8_lossy(&out.stdout);\n    assert!(stdout.contains(\"Hello from {name}\"), \"sortie inattendue: {{stdout}}\");\n}}\n",
        bin = crate_name,
        name = crate_name
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test_references_binary() {
        let t = smoke_test("demo");
        assert!(t.contains("CARGO_BIN_EXE_demo") && t.contains("Hello from demo"));
    }
}
