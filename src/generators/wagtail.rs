use super::{write_with_docs, Generated, Kind, TemplateFile};
use crate::tpl;
use anyhow::Result;
use std::path::Path;

/// Fichiers du template `templates/wagtail/` (chemin généré, contenu embarqué).
pub const FILES: &[TemplateFile] = &[
    (".alpharust.toml", tpl!("wagtail/dot-alpharust.toml.tpl")),
    (".env.example", tpl!("wagtail/dot-env.example.tpl")),
    (".gitignore", tpl!("wagtail/dot-gitignore.tpl")),
    ("@@pkg@@/__init__.py", tpl!("wagtail/pkg/__init__.py.tpl")),
    ("@@pkg@@/asgi.py", tpl!("wagtail/pkg/asgi.py.tpl")),
    (
        "@@pkg@@/settings/__init__.py",
        tpl!("wagtail/pkg/settings/__init__.py.tpl"),
    ),
    (
        "@@pkg@@/settings/base.py",
        tpl!("wagtail/pkg/settings/base.py.tpl"),
    ),
    (
        "@@pkg@@/settings/dev.py",
        tpl!("wagtail/pkg/settings/dev.py.tpl"),
    ),
    (
        "@@pkg@@/settings/production.py",
        tpl!("wagtail/pkg/settings/production.py.tpl"),
    ),
    (
        "@@pkg@@/static/css/site.css",
        tpl!("wagtail/pkg/static/css/site.css.tpl"),
    ),
    (
        "@@pkg@@/templates/base.html",
        tpl!("wagtail/pkg/templates/base.html.tpl"),
    ),
    ("@@pkg@@/urls.py", tpl!("wagtail/pkg/urls.py.tpl")),
    ("@@pkg@@/wsgi.py", tpl!("wagtail/pkg/wsgi.py.tpl")),
    ("README.md", tpl!("wagtail/README.md.tpl")),
    ("home/__init__.py", tpl!("wagtail/home/__init__.py.tpl")),
    ("home/apps.py", tpl!("wagtail/home/apps.py.tpl")),
    (
        "home/migrations/0001_initial.py",
        tpl!("wagtail/home/migrations/0001_initial.py.tpl"),
    ),
    (
        "home/migrations/0002_create_homepage.py",
        tpl!("wagtail/home/migrations/0002_create_homepage.py.tpl"),
    ),
    (
        "home/migrations/__init__.py",
        tpl!("wagtail/home/migrations/__init__.py.tpl"),
    ),
    ("home/models.py", tpl!("wagtail/home/models.py.tpl")),
    (
        "home/templates/home/home_page.html",
        tpl!("wagtail/home/templates/home/home_page.html.tpl"),
    ),
    ("home/tests.py", tpl!("wagtail/home/tests.py.tpl")),
    ("manage.py", tpl!("wagtail/manage.py.tpl")),
    ("requirements.txt", tpl!("wagtail/requirements.txt.tpl")),
];

pub fn generate(path: &Path, name: &str, force: bool) -> Result<Generated> {
    write_with_docs(Kind::Wagtail, path, name, FILES, force)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_template_file_is_registered() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/wagtail");
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
