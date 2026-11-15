pub fn generate(project_name: &str) -> String {
    format!(
        "# Documentation {}\n\n## Overview\n\nGénéré par AlphaRust.\n\n## API\n\n## Examples\n",
        project_name
    )
}
