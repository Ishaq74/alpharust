pub fn generate(project_name: &str) -> String {
    format!(
        r#"# {name}

Projet généré et gouverné par [AlphaRust](https://github.com/Ishaq74/alpharust).

## Installation

```bash
# À compléter
```

## Usage

```bash
# À compléter
```

## License

MIT
"#,
        name = project_name
    )
}
