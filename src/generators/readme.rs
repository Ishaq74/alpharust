pub fn generate(project_name: &str) -> String {
    format!(
        r#"# {name}

Projet généré et gouverné par [AlphaRust](https://github.com/Ishaq74/alpharust).

## Installation

## Usage

## License

MIT
"#,
        name = project_name
    )
}
