use super::{write_with_docs, Generated, Kind, TemplateFile};
use crate::tpl;
use anyhow::Result;
use std::path::Path;

/// Fichiers du template `templates/astro/` (chemin généré, contenu embarqué).
pub const FILES: &[TemplateFile] = &[
    (".alpharust.toml", tpl!("astro/dot-alpharust.toml.tpl")),
    (".gitignore", tpl!("astro/dot-gitignore.tpl")),
    ("README.md", tpl!("astro/README.md.tpl")),
    ("astro.config.mjs", tpl!("astro/astro.config.mjs.tpl")),
    ("package.json", tpl!("astro/package.json.tpl")),
    ("public/favicon.svg", tpl!("astro/public/favicon.svg.tpl")),
    (
        "src/components/Header.astro",
        tpl!("astro/src/components/Header.astro.tpl"),
    ),
    (
        "src/content.config.ts",
        tpl!("astro/src/content.config.ts.tpl"),
    ),
    (
        "src/content/blog/premier-article.md",
        tpl!("astro/src/content/blog/premier-article.md.tpl"),
    ),
    (
        "src/layouts/Layout.astro",
        tpl!("astro/src/layouts/Layout.astro.tpl"),
    ),
    (
        "src/pages/about.astro",
        tpl!("astro/src/pages/about.astro.tpl"),
    ),
    (
        "src/pages/blog/[id].astro",
        tpl!("astro/src/pages/blog/[id].astro.tpl"),
    ),
    (
        "src/pages/blog/index.astro",
        tpl!("astro/src/pages/blog/index.astro.tpl"),
    ),
    (
        "src/pages/index.astro",
        tpl!("astro/src/pages/index.astro.tpl"),
    ),
    (
        "src/styles/global.css",
        tpl!("astro/src/styles/global.css.tpl"),
    ),
    ("tsconfig.json", tpl!("astro/tsconfig.json.tpl")),
];

pub fn generate(path: &Path, name: &str, force: bool) -> Result<Generated> {
    write_with_docs(Kind::Astro, path, name, FILES, force)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_template_file_is_registered() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/astro");
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
