use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn generate(path: &Path, name: &str) -> Result<()> {
    fs::create_dir_all(path.join("src/pages"))?;
    fs::create_dir_all(path.join("src/components"))?;

    fs::write(
        path.join("package.json"),
        format!(r#"{{
  "name": "{}",
  "type": "module",
  "scripts": {{
    "dev": "astro dev",
    "build": "astro build"
  }},
  "dependencies": {{
    "astro": "^4.0.0"
  }}
}}"#, name),
    )?;

    fs::write(
        path.join("src/pages/index.astro"),
        "---\n---\n<html>\n  <body>\n    <h1>Hello from Astro</h1>\n  </body>\n</html>\n",
    )?;

    println!("✓ Projet Astro '{}' généré", name);
    Ok(())
}
