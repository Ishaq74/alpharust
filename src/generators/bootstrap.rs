use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Crée un projet minimal : README, .gitignore, dossiers standards, CI.
pub fn bootstrap(path: &Path, name: &str) -> Result<()> {
    fs::create_dir_all(path).with_context(|| format!("création de {})", path.display()))?;

    let gitignore = "/target\n.DS_Store\n*.swp\n.env\n.env.*\n!.env.example\n";
    fs::write(path.join(".gitignore"), gitignore).context("écriture .gitignore");

    let readme = format!(
        "# {}\n\nProjet généré par AlphaRust.\n\n## Installation\n\n```bash\ncargo install --path .\n```\n\n## Usage\n\n```bash\n./alpharust lint .\n```\n",
        name
    );
    fs::write(path.join("README.md"), readme).context("écriture README.md");

    for d in &[".github/workflows", "docs", "src"] {
        fs::create_dir_all(path.join(d)).ok();
    }

    let ci = "name: CI\n\non:\n  push:\n    branches: [main]\n  pull_request:\n\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n      - name: alpharust lint\n        run: |\n          curl -sL https://github.com/Ishaq74/alpharust/releases/latest/download/alpharust-x86_64-unknown-linux-gnu -o alpharust\n          chmod +x alpharust\n          ./alpharust lint .\n";
    fs::write(path.join(".github/workflows/ci.yml"), ci).context("écriture CI");

    println!("projet {} initialisé dans {})", name, path.display());
    Ok(())
}
