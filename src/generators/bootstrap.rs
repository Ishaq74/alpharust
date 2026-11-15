use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn bootstrap(path: &Path, name: &str) -> Result<()> {
    fs::create_dir_all(path)?;
    let readme = super::readme::generate(name);
    fs::write(path.join("README.md"), readme)?;
    fs::write(
        path.join(".gitignore"),
        "/target\n.DS_Store\n*.swp\n",
    )?;
    println!("✓ Projet '{}' initialisé dans {}", name, path.display());
    Ok(())
}
