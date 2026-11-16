use super::smoke::smoke_test;
use super::{readme, write_files, Generated, Kind, TemplateFile, Vars};
use crate::tpl;
use anyhow::Result;
use std::path::Path;

const FILES: &[TemplateFile] = &[
    ("Cargo.toml", tpl!("bootstrap/Cargo.toml.tpl")),
    ("src/main.rs", tpl!("bootstrap/src/main.rs.tpl")),
    ("src/lib.rs", tpl!("bootstrap/src/lib.rs.tpl")),
    (".gitignore", tpl!("bootstrap/dot-gitignore.tpl")),
    (".alpharust.toml", tpl!("bootstrap/dot-alpharust.toml.tpl")),
    (
        ".github/workflows/ci.yml",
        tpl!("bootstrap/dot-github/workflows/ci.yml.tpl"),
    ),
    ("LICENSE", tpl!("bootstrap/LICENSE.tpl")),
];

/// Projet Rust minimal mais complet : lib + binaire + tests + CI + config AlphaRust.
pub fn generate(path: &Path, name: &str, force: bool) -> Result<Generated> {
    let vars = Vars::new(name)?;
    let readme_md = readme::generate_with(
        name,
        &format!(
            "git clone <url-du-depot> {0}\ncd {0}\ncargo build --release",
            vars.slug
        ),
        "cargo run\ncargo test",
    );
    let docs_md = super::docs::generate(name);
    let smoke = smoke_test(&vars.slug);
    let mut files: Vec<TemplateFile> = FILES.to_vec();
    files.push(("README.md", &readme_md));
    files.push(("docs/index.md", &docs_md));
    files.push(("tests/smoke.rs", &smoke));
    write_files(Kind::Bootstrap, path, name, &files, force)
}

/// Compatibilité : ancien nom de la fonction d'initialisation.
pub fn bootstrap(path: &Path, name: &str) -> Result<Generated> {
    generate(path, name, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_template_file_is_registered() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/bootstrap");
        let on_disk = walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .count();
        assert_eq!(
            on_disk,
            FILES.len(),
            "un fichier de template n'est pas enregistré dans FILES"
        );
    }
}
