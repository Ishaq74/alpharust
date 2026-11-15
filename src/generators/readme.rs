/// Génère un README de base à partir des métadonnées
pub fn generate(project_name: &str) -> String {
    format!(
        "# {}\n\nProjet généré par AlphaRust.\n\n## Installation\n\n## Usage\n\n## License\n\nMIT\n",
        project_name
    )
}
