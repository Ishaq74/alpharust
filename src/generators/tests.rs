/// Génère un squelette de test unitaire
pub fn generate_unit_test(module: &str) -> String {
    format!(
        "#[cfg(test)]\nmod tests {{\n    use super::*;\n\n    #[test]\n    fn test_{}() {{\n        // TODO\n        assert!(true);\n    }}\n}}\n",
        module
    )
}
