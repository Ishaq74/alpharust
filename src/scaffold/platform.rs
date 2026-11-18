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

/// Préréglage de génération : un ensemble cohérent de briques plateforme.
pub struct Preset {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    /// Équivalent en options CLI (affiché par `alpharust presets`).
    pub flags: &'static str,
    ui: UiKit,
    database: Db,
    auth_plugins: Option<&'static [&'static str]>,
    i18n: &'static [&'static str],
    integrations: &'static [&'static str],
    docker: bool,
    ci: bool,
}

pub const PRESETS: &[Preset] = &[
    Preset {
        id: "vitrine",
        label: "Site vitrine",
        description: "Site statique Tailwind + Starwind et MDX, sans serveur (ajoutez `--integration sitemap --site <url>`).",
        flags: "--ui starwind --integration mdx --ci",
        ui: UiKit::Starwind,
        database: Db::None,
        auth_plugins: None,
        i18n: &[],
        integrations: &["mdx"],
        docker: false,
        ci: true,
    },
    Preset {
        id: "blog-i18n",
        label: "Blog multilingue",
        description: "Blog avec MDX et routage i18n (fr, en), rendu statique.",
        flags: "--i18n fr,en --integration mdx --ci",
        ui: UiKit::None,
        database: Db::None,
        auth_plugins: None,
        i18n: &["fr", "en"],
        integrations: &["mdx"],
        docker: false,
        ci: true,
    },
    Preset {
        id: "app-auth",
        label: "Application avec comptes",
        description: "Starwind, SQLite, Better Auth (rôle admin), journal d'audit, Docker et CI.",
        flags: "--ui starwind --db sqlite --auth --auth-plugin admin --docker --ci",
        ui: UiKit::Starwind,
        database: Db::Sqlite,
        auth_plugins: Some(&["admin"]),
        i18n: &[],
        integrations: &[],
        docker: true,
        ci: true,
    },
    Preset {
        id: "saas-pg",
        label: "Plateforme PostgreSQL",
        description: "Starwind, PostgreSQL, Better Auth (admin, username, organization), i18n fr/en, Docker et CI.",
        flags: "--ui starwind --db postgres --auth --auth-plugin admin,username,organization --i18n fr,en --docker --ci",
        ui: UiKit::Starwind,
        database: Db::Postgres,
        auth_plugins: Some(&["admin", "username", "organization"]),
        i18n: &["fr", "en"],
        integrations: &[],
        docker: true,
        ci: true,
    },
    Preset {
        id: "concordia",
        label: "Site Concordia (CSS pur)",
        description: "Système CSS Concordia complet, thème WCAG généré, i18n fr/en, base SQLite et comptes.",
        flags: "--ui concordia --db sqlite --auth --i18n fr,en",
        ui: UiKit::Concordia,
        database: Db::Sqlite,
        auth_plugins: Some(&[]),
        i18n: &["fr", "en"],
        integrations: &[],
        docker: false,
        ci: false,
    },
];

impl Preset {
    pub fn find(id: &str) -> Option<&'static Preset> {
        PRESETS.iter().find(|p| p.id == id.to_lowercase())
    }

    /// Complète les options : seuls les champs encore à leur valeur par défaut sont renseignés
    /// (une option explicite l'emporte toujours sur le préréglage).
    pub fn apply(&self, o: &mut super::astro::AstroOptions) {
        if o.ui == UiKit::None {
            o.ui = self.ui;
        }
        if o.database == Db::None {
            o.database = self.database;
        }
        if let (false, Some(plugins)) = (o.auth, self.auth_plugins) {
            o.auth = true;
            if o.auth_plugins.is_empty() {
                o.auth_plugins = plugins.iter().map(|s| s.to_string()).collect();
            }
        }
        if o.i18n.is_empty() {
            o.i18n = self.i18n.iter().map(|s| s.to_string()).collect();
        }
        if self.ui == UiKit::Concordia && o.ui == UiKit::Concordia && o.theme.is_none() {
            o.theme = Some(super::theme::ThemeColors::default());
        }
        o.docker |= self.docker;
        o.ci |= self.ci;
        for e in self.integrations {
            if !o.integrations.iter().any(|i| i == e) {
                o.integrations.push((*e).to_string());
            }
        }
    }
}

/// Composants Starwind d'atomic (`src/components/atoms/`), tous posés avec le kit `starwind`.
/// `icon-picker` et `media-picker` (liés à la base média d'atomic) ne sont pas repris.
pub const STARWIND_ATOMS: &[&str] = &[
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
    "container",
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
    fn presets_fill_defaults_but_never_override_explicit_options() {
        let p = Preset::find("saas-pg").unwrap();
        let mut o = super::super::astro::AstroOptions::default();
        p.apply(&mut o);
        assert_eq!(o.database, Db::Postgres);
        assert!(o.auth && o.docker && o.ci);
        assert_eq!(o.auth_plugins, ["admin", "username", "organization"]);
        assert_eq!(o.i18n, ["fr", "en"]);

        let mut o = super::super::astro::AstroOptions {
            database: Db::Sqlite,
            i18n: vec!["es".into(), "en".into()],
            ..Default::default()
        };
        p.apply(&mut o);
        assert_eq!(o.database, Db::Sqlite, "l'option explicite l'emporte");
        assert_eq!(o.i18n, ["es", "en"]);
        assert!(o.validate(true).is_ok(), "un préréglage reste valide");
    }

    #[test]
    fn every_preset_produces_a_valid_request() {
        for p in PRESETS {
            let mut o = super::super::astro::AstroOptions::default();
            p.apply(&mut o);
            o.validate(true)
                .unwrap_or_else(|e| panic!("préréglage {} invalide : {e}", p.id));
        }
        assert!(Preset::find("inconnu").is_none());
    }
}
