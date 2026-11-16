//! Briques « plateforme » d'un projet Astro, calquées sur le dépôt de référence `Ishaq74/atomic` :
//! base de données (Drizzle + PostgreSQL ou SQLite), authentification (Better Auth + plugins),
//! i18n (routage natif d'Astro), kit d'interface (Starwind officiel ou système CSS de Concordia),
//! Docker et CI. Les paquets sont installés par les outils officiels ; seule la glue est écrite par la couche.

use serde::{Deserialize, Serialize};

/// Langues livrées avec leurs traductions (mêmes que le dépôt atomic).
pub const LOCALES: &[(&str, &str)] = &[
    ("fr", "Français"),
    ("en", "English"),
    ("es", "Español"),
    ("ar", "العربية (RTL)"),
];

/// Plugins Better Auth proposés.
pub const AUTH_PLUGINS: &[(&str, &str)] = &[
    ("username", "Connexion par nom d'utilisateur"),
    ("admin", "Rôles, bannissement, page /admin"),
    ("organization", "Organisations, membres, invitations"),
];

/// Composants Starwind installés par défaut (`starwind add`).
pub const STARWIND_DEFAULT: &[&str] = &["button", "card", "input", "label", "alert"];

/// Composants Starwind connus (catalogue officiel, mêmes noms que `starwind add`).
pub const STARWIND_COMPONENTS: &[&str] = &[
    "accordion",
    "alert",
    "alert-dialog",
    "aspect-ratio",
    "avatar",
    "badge",
    "breadcrumb",
    "button",
    "button-group",
    "card",
    "carousel",
    "checkbox",
    "collapsible",
    "dialog",
    "dropdown",
    "dropzone",
    "image",
    "input",
    "input-group",
    "input-otp",
    "item",
    "kbd",
    "label",
    "native-select",
    "pagination",
    "popover",
    "progress",
    "prose",
    "radio-group",
    "select",
    "separator",
    "sheet",
    "sidebar",
    "skeleton",
    "slider",
    "spinner",
    "switch",
    "table",
    "tabs",
    "textarea",
    "theme-toggle",
    "toast",
    "toggle",
    "tooltip",
    "video",
];

/// Kit d'interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum UiKit {
    /// Thème AlphaRust par défaut (CSS simple, variables).
    #[default]
    None,
    /// Tailwind 4 + Starwind (`starwind init` / `starwind add`), comme atomic.
    Starwind,
    /// Système CSS de Concordia : jetons + 4 variantes (initial, retro, modern, futuristic), sans Tailwind.
    Concordia,
}

impl UiKit {
    pub const ALL: [UiKit; 3] = [UiKit::None, UiKit::Starwind, UiKit::Concordia];
    pub fn id(self) -> &'static str {
        match self {
            UiKit::None => "none",
            UiKit::Starwind => "starwind",
            UiKit::Concordia => "concordia",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|u| u.id() == s.to_lowercase())
    }
}

/// Base de données (Drizzle ORM).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Db {
    #[default]
    None,
    /// PostgreSQL (`pg`), comme atomic.
    Postgres,
    /// SQLite local (`better-sqlite3`) : zéro service à lancer.
    Sqlite,
}

impl Db {
    pub const ALL: [Db; 3] = [Db::None, Db::Postgres, Db::Sqlite];
    pub fn id(self) -> &'static str {
        match self {
            Db::None => "none",
            Db::Postgres => "postgres",
            Db::Sqlite => "sqlite",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pg" | "postgresql" => Some(Db::Postgres),
            other => Self::ALL.into_iter().find(|d| d.id() == other),
        }
    }
    /// Dialecte pour `auth generate --dialect` et drizzle-kit.
    pub fn dialect(self) -> &'static str {
        match self {
            Db::Postgres => "pg",
            _ => "sqlite",
        }
    }
    /// Paquets d'exécution (hors drizzle-orm).
    pub fn driver(self) -> &'static str {
        match self {
            Db::Postgres => "pg",
            _ => "better-sqlite3",
        }
    }
    pub fn driver_types(self) -> &'static str {
        match self {
            Db::Postgres => "@types/pg",
            _ => "@types/better-sqlite3",
        }
    }
}

/// Langue valide ? (`fr`, `en`, `es`, `ar`)
pub fn is_locale(code: &str) -> bool {
    LOCALES.iter().any(|(c, _)| *c == code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(UiKit::parse("Starwind"), Some(UiKit::Starwind));
        assert_eq!(Db::parse("PostgreSQL"), Some(Db::Postgres));
        assert_eq!(Db::parse("pg"), Some(Db::Postgres));
        assert_eq!(Db::parse("mysql"), None);
        assert_eq!(Db::Sqlite.dialect(), "sqlite");
        assert_eq!(Db::Postgres.driver(), "pg");
        assert!(is_locale("ar") && !is_locale("de"));
    }

    #[test]
    fn starwind_defaults_are_known() {
        for c in STARWIND_DEFAULT {
            assert!(STARWIND_COMPONENTS.contains(c), "{c}");
        }
    }
}
