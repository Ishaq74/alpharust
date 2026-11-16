/// Documentation technique de départ : plan + règles AlphaRust applicables.
pub fn generate(project_name: &str) -> String {
    format!(
        "# Documentation {name}\n\n## Vue d'ensemble\n\n`{name}` est gouverné par AlphaRust : les règles du dossier `rules/` (profil `strict` recommandé) \
         sont vérifiées par `alpharust lint`.\n\n## Installation\n\nVoir le README du projet.\n\n## Usage\n\n```bash\nalpharust lint --profile strict\nalpharust lint --sarif > alpharust.sarif\n```\n\n## Architecture\n\n- `src/` ou dossier applicatif : code du projet\n- `tests/` : tests automatisés\n- `docs/` : documentation (ce dossier)\n\n## Contribuer\n\n1. Créer une branche\n2. `alpharust lint --profile strict` doit passer sans erreur\n3. Ouvrir une pull request\n",
        name = project_name
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mentions_project_and_sections() {
        let md = generate("demo");
        assert!(md.starts_with("# Documentation demo"));
        assert!(md.contains("## Installation") && md.contains("## Usage"));
        assert!(!md.contains("à compléter"));
    }
}
