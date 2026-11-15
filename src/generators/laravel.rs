use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn generate(path: &Path, name: &str) -> Result<()> {
    fs::create_dir_all(path.join("app/Http/Controllers"))?;
    fs::create_dir_all(path.join("database/migrations"))?;

    fs::write(
        path.join("composer.json"),
        format!(r#"{{
  "name": "{}/{}",
  "require": {{
    "php": "^8.2",
    "laravel/framework": "^11.0"
  }}
}}"#, name.to_lowercase(), name.to_lowercase()),
    )?;

    println!("✓ Projet Laravel '{}' généré", name);
    Ok(())
}
