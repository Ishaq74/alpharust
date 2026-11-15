pub fn generate_unit_test(module_name: &str) -> String {
    format!(
        r#"#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_{name}_basic() {{
        // TODO: implement real assertion
        assert!(true);
    }}

    #[test]
    fn test_{name}_edge_case() {{
        // TODO
        assert!(true);
    }}
}}
"#,
        name = module_name
    )
}
