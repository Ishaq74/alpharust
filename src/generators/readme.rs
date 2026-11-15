pub fn generate(name: &str) -> String {
    format!(
        "# {}\n\n> Généré par AlphaRust — prêt-à-porter sur mesure.\n\n## Installation\n\n```bash\ngit clone <repo>\ncd {}\n```\n\n## Utilisation\n\n```bash\n# Lancer le lint\n./alpharust lint .\n\n# Avec un profil\n./alpharust lint . --profile strict\n```\n\n## Règles actives\n\nVoir `rules/` pour la liste complète des primitives.\n\n## Licence\n\nMIT\n",
        name, name
    )
}
