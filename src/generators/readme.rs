/// README structuré pour un projet généré (sections Installation / Usage / License,
/// exigées par les règles `readme/*`).
pub fn generate_with(project_name: &str, install: &str, usage: &str) -> String {
    format!(
        // alpharust-ignore: no-hardcoded-urls
        "# {name}\n\nProjet généré et gouverné par [AlphaRust](https://github.com/Ishaq74/alpharust).\n\n## Installation\n\n```bash\n{install}\n```\n\n## Usage\n\n```bash\n{usage}\n```\n\n## Qualité\n\n```bash\nalpharust lint --profile strict\n```\n\n## License\n\nMIT\n",
        name = project_name,
        install = install,
        usage = usage
    )
}

/// README générique (commande `alpharust readme <nom>`).
pub fn generate(project_name: &str) -> String {
    generate_with(
        project_name,
        &format!("git clone <url-du-depot> {0}\ncd {0}", project_name),
        "# voir la documentation du projet dans docs/",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_required_sections_and_no_placeholder() {
        let md = generate("demo");
        assert!(md.starts_with("# demo\n"));
        for s in ["## Installation", "## Usage", "## License"] {
            assert!(md.contains(s), "{}", s);
        }
        assert!(!md.contains("À compléter"));
    }
}
