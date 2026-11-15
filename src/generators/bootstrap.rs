use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn bootstrap(path: &Path, name: &str) -> Result<()> {
    fs::create_dir_all(path)?;
    fs::create_dir_all(path.join("src"))?;

    let readme = super::readme::generate(name);
    fs::write(path.join("README.md"), readme)?;

    fs::write(
        path.join(".gitignore"),
        "/target\n.DS_Store\n*.swp\n.env\n",
    )?;

    fs::write(
        path.join("src/main.rs"),
        "fn main() {\n    println!(\"Hello from {}\");\n}\n",
    )?;

    println!("✓ Projet '{}' initialisé dans {}", name, path.display());
    println!("  - README.md");
    println!("  - .gitignore");
    println!("  - src/main.rs");
    Ok(())
}
