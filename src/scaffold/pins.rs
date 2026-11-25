//! Versions épinglées des outils officiels lancés par les générateurs.
//!
//! Aucune commande ne tire `@latest` : une génération est reproductible et une nouvelle version d'un outil
//! n'entre qu'après passage de la CI (mettre à jour ici, puis `cargo test` et le job e2e).
//! Vérification : `npm view create-astro version`, `npm view starwind version`, `npm view auth version`,
//! https://pypi.org/project/wagtail/ (même série que le `requirements.txt` de `wagtail start`).

/// `npm create astro@<version>` (paquet `create-astro`).
pub const CREATE_ASTRO: &str = "5.3.2";
/// CLI Starwind UI (`starwind init`).
pub const STARWIND_CLI: &str = "3.6.0";
/// CLI Better Auth (`auth generate`), même série que la bibliothèque `better-auth` 1.7 installée.
pub const BETTER_AUTH_CLI: &str = "1.7.28";
/// Spécification pip de Wagtail (série 8.1, correctifs de sécurité compris).
pub const WAGTAIL_PIP: &str = "wagtail>=8.1,<8.2";

/// `create-astro` épinglé, au format `astro@<version>` attendu par `npm|pnpm|bun create`.
pub fn create_astro() -> String {
    format!("astro@{CREATE_ASTRO}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_floating_tags() {
        for v in [
            super::CREATE_ASTRO,
            super::STARWIND_CLI,
            super::BETTER_AUTH_CLI,
        ] {
            assert!(
                v.split('.').count() == 3 && v.split('.').all(|p| p.parse::<u32>().is_ok()),
                "{v}"
            );
        }
        assert_eq!(super::create_astro(), "astro@5.3.2");
    }
}
