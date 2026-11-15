pub fn generate_unit_test(name: &str) -> String {
    format!(
        "#[cfg(test)]\nmod tests {{\n    #[test]\n    fn test_{}() {{\n        assert!(true);\n    }}\n}}\n",
        name
    )
}
