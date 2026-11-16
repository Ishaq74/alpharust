use super::{write_with_docs, Generated, Kind, TemplateFile};
use crate::tpl;
use anyhow::Result;
use std::path::Path;

/// Fichiers du template `templates/laravel/` (chemin généré, contenu embarqué).
pub const FILES: &[TemplateFile] = &[
    (".alpharust.toml", tpl!("laravel/dot-alpharust.toml.tpl")),
    (".env.example", tpl!("laravel/dot-env.example.tpl")),
    (".gitignore", tpl!("laravel/dot-gitignore.tpl")),
    ("README.md", tpl!("laravel/README.md.tpl")),
    (
        "app/Http/Controllers/Controller.php",
        tpl!("laravel/app/Http/Controllers/Controller.php.tpl"),
    ),
    (
        "app/Http/Controllers/PageController.php",
        tpl!("laravel/app/Http/Controllers/PageController.php.tpl"),
    ),
    (
        "app/Models/Page.php",
        tpl!("laravel/app/Models/Page.php.tpl"),
    ),
    (
        "app/Providers/AppServiceProvider.php",
        tpl!("laravel/app/Providers/AppServiceProvider.php.tpl"),
    ),
    ("artisan", tpl!("laravel/artisan.tpl")),
    ("bootstrap/app.php", tpl!("laravel/bootstrap/app.php.tpl")),
    (
        "bootstrap/cache/.gitignore",
        tpl!("laravel/bootstrap/cache/dot-gitignore.tpl"),
    ),
    (
        "bootstrap/providers.php",
        tpl!("laravel/bootstrap/providers.php.tpl"),
    ),
    ("composer.json", tpl!("laravel/composer.json.tpl")),
    (
        "database/.gitignore",
        tpl!("laravel/database/dot-gitignore.tpl"),
    ),
    (
        "database/migrations/2026_01_01_000000_create_pages_table.php",
        tpl!("laravel/database/migrations/2026_01_01_000000_create_pages_table.php.tpl"),
    ),
    (
        "database/seeders/DatabaseSeeder.php",
        tpl!("laravel/database/seeders/DatabaseSeeder.php.tpl"),
    ),
    ("phpunit.xml", tpl!("laravel/phpunit.xml.tpl")),
    ("public/index.php", tpl!("laravel/public/index.php.tpl")),
    (
        "resources/views/home.blade.php",
        tpl!("laravel/resources/views/home.blade.php.tpl"),
    ),
    (
        "resources/views/layouts/app.blade.php",
        tpl!("laravel/resources/views/layouts/app.blade.php.tpl"),
    ),
    (
        "resources/views/page.blade.php",
        tpl!("laravel/resources/views/page.blade.php.tpl"),
    ),
    ("routes/console.php", tpl!("laravel/routes/console.php.tpl")),
    ("routes/web.php", tpl!("laravel/routes/web.php.tpl")),
    (
        "storage/app/.gitignore",
        tpl!("laravel/storage/app/dot-gitignore.tpl"),
    ),
    (
        "storage/framework/cache/.gitignore",
        tpl!("laravel/storage/framework/cache/dot-gitignore.tpl"),
    ),
    (
        "storage/framework/sessions/.gitignore",
        tpl!("laravel/storage/framework/sessions/dot-gitignore.tpl"),
    ),
    (
        "storage/framework/views/.gitignore",
        tpl!("laravel/storage/framework/views/dot-gitignore.tpl"),
    ),
    (
        "storage/logs/.gitignore",
        tpl!("laravel/storage/logs/dot-gitignore.tpl"),
    ),
    (
        "tests/Feature/PageTest.php",
        tpl!("laravel/tests/Feature/PageTest.php.tpl"),
    ),
    ("tests/TestCase.php", tpl!("laravel/tests/TestCase.php.tpl")),
    (
        "tests/Unit/PageTest.php",
        tpl!("laravel/tests/Unit/PageTest.php.tpl"),
    ),
];

pub fn generate(path: &Path, name: &str, force: bool) -> Result<Generated> {
    write_with_docs(Kind::Laravel, path, name, FILES, force)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_template_file_is_registered() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/laravel");
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
