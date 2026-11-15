use std::fs;
use std::path::Path;

pub fn bootstrap(path: &Path, name: &str) -> anyhow::Result<()> {
    fs::create_dir_all(path)?;
    fs::write(path.join("README.md"), crate::generators::readme::generate(name))?;
    fs::write(path.join(".gitignore"), "/target\n")?;
    println!("Projet {} initialisé dans {}", name, path.display());
    Ok(())
}
