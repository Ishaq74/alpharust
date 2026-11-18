//! Assistant interactif (`alpharust new`) : questions guidées, aperçu des commandes exactes, confirmation.

use crate::scaffold::astro::{AstroOptions, PackageManager, Template, TsLevel, INTEGRATIONS};
use crate::scaffold::laravel::{Database, LaravelOptions, Starter};
use crate::scaffold::platform::{Db, UiKit, AUTH_PLUGINS, LOCALES};
use crate::scaffold::rust::{CrateKind, RustOptions, DEPS};
use crate::scaffold::theme::ThemeColors;
use crate::scaffold::wagtail::{valid_pip_spec, WagtailOptions};
use crate::scaffold::{Framework, Options, Request};
use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};

fn err<E: std::fmt::Display>(e: E) -> anyhow::Error {
    anyhow!("saisie interrompue : {e}")
}

fn select<T: Copy>(
    theme: &ColorfulTheme,
    prompt: &str,
    items: &[(T, String)],
    default: usize,
) -> Result<T> {
    let labels: Vec<&str> = items.iter().map(|(_, l)| l.as_str()).collect();
    let i = Select::with_theme(theme)
        .with_prompt(prompt)
        .items(&labels)
        .default(default)
        .interact()
        .map_err(err)?;
    Ok(items[i].0)
}

fn confirm(theme: &ColorfulTheme, prompt: &str, default: bool) -> Result<bool> {
    Confirm::with_theme(theme)
        .with_prompt(prompt)
        .default(default)
        .interact()
        .map_err(err)
}

/// Pose les questions et renvoie la demande construite (non encore confirmée).
pub fn ask(preset_name: Option<String>, preset_framework: Option<Framework>) -> Result<Request> {
    let theme = ColorfulTheme::default();
    println!("AlphaRust — nouveau projet (les outils officiels font le gros du travail)\n");

    let framework = match preset_framework {
        Some(f) => f,
        None => {
            let items: Vec<(Framework, String)> = Framework::ALL
                .iter()
                .map(|f| (*f, format!("{:<18} via {}", f.label(), f.official_tool())))
                .collect();
            select(&theme, "Framework", &items, 0)?
        }
    };

    let name: String = Input::with_theme(&theme)
        .with_prompt("Nom du projet")
        .with_initial_text(preset_name.unwrap_or_default())
        .default(format!("mon-{}", framework.id()))
        .validate_with(|s: &String| crate::generators::validate_name(s).map_err(|e| e.to_string()))
        .interact_text()
        .map_err(err)?;

    let mut req = Request::new(framework, &name);

    req.options = match framework {
        Framework::Astro => {
            let pms: Vec<(PackageManager, String)> = PackageManager::ALL
                .iter()
                .map(|p| {
                    let found = crate::scaffold::tools::which(p.id()).is_some();
                    (
                        *p,
                        format!("{}{}", p.id(), if found { "" } else { "  (non installé)" }),
                    )
                })
                .collect();
            let package_manager = select(&theme, "Gestionnaire de paquets", &pms, 0)?;
            let templates: Vec<(Template, String)> = vec![
                (
                    Template::Alpharust,
                    "alpharust — minimal officiel + structure AlphaRust (layout, thème, blog typé)"
                        .into(),
                ),
                (Template::Minimal, "minimal   — officiel, vide".into()),
                (Template::Basics, "basics    — officiel, exemples".into()),
                (Template::Blog, "blog      — officiel, blog complet".into()),
            ];
            let template = select(&theme, "Template", &templates, 0)?;
            let ts: Vec<(TsLevel, String)> = vec![
                (TsLevel::Base, "base — permissif".into()),
                (TsLevel::Strict, "strict (recommandé)".into()),
                (TsLevel::Strictest, "strictest — le plus exigeant".into()),
            ];
            let typescript = select(&theme, "TypeScript", &ts, 1)?;
            let labels: Vec<String> = INTEGRATIONS
                .iter()
                .map(|(n, d)| format!("{n:<9} {d}"))
                .collect();
            let chosen = MultiSelect::with_theme(&theme)
                .with_prompt("Intégrations (espace pour cocher, entrée pour valider)")
                .items(&labels)
                .interact()
                .map_err(err)?;
            let integrations: Vec<String> = chosen
                .iter()
                .map(|i| INTEGRATIONS[*i].0.to_string())
                .collect();
            let needs_site =
                integrations.iter().any(|i| i == "sitemap") || template == Template::Blog;
            let site: Option<String> = if needs_site {
                Some(
                    Input::with_theme(&theme)
                        .with_prompt("URL publique du site")
                        // alpharust-ignore: no-hardcoded-urls
                        .default("https://exemple.com".to_string())
                        .validate_with(|s: &String| {
                            if s.starts_with("http://") || s.starts_with("https://") {
                                Ok(())
                            } else {
                                Err("l'URL doit commencer par http:// ou https://".to_string())
                            }
                        })
                        .interact_text()
                        .map_err(err)?,
                )
            } else {
                None
            };
            let typecheck = confirm(
                &theme,
                "Installer @astrojs/check et vérifier les types ?",
                true,
            )?;
            let mut o = AstroOptions {
                package_manager,
                template,
                typescript,
                integrations,
                site,
                typecheck,
                ..AstroOptions::default()
            };
            if template == Template::Alpharust {
                platform_prompts(&theme, &mut o)?;
            }
            if confirm(&theme, "Dockerfile + docker-compose ?", false)? {
                o.docker = true;
            }
            if confirm(
                &theme,
                "Workflow GitHub Actions (check, build, tests) ?",
                false,
            )? {
                o.ci = true;
            }
            Options::Astro(o)
        }
        Framework::Laravel => {
            let starters: Vec<(Starter, String)> = Starter::ALL
                .iter()
                .map(|s| (*s, s.label().to_string()))
                .collect();
            let starter = select(&theme, "Kit de démarrage", &starters, 0)?;
            let mut dark = false;
            let mut pest = false;
            if starter != Starter::None {
                if starter.has_frontend() {
                    dark = confirm(&theme, "Mode sombre (Breeze --dark) ?", false)?;
                }
                pest = confirm(&theme, "Tests avec Pest au lieu de PHPUnit ?", false)?;
            }
            let dbs: Vec<(Database, String)> = vec![
                (
                    Database::Sqlite,
                    "SQLite (zéro configuration, migrations exécutées)".into(),
                ),
                (
                    Database::Mysql,
                    "MySQL (.env configuré, migration à lancer)".into(),
                ),
                (
                    Database::Pgsql,
                    "PostgreSQL (.env configuré, migration à lancer)".into(),
                ),
            ];
            let database = select(&theme, "Base de données", &dbs, 0)?;
            let frontend = if starter == Starter::None {
                confirm(
                    &theme,
                    "Installer et compiler les assets (npm install && build) ?",
                    false,
                )?
            } else {
                false
            };
            let ui = starter == Starter::None
                && confirm(
                    &theme,
                    "Catalogue de composants (design kit CSS, page /components) ?",
                    false,
                )?;
            let starwind = ui
                && select(
                    &theme,
                    "Kit de composants",
                    &[
                        (false, "CSS pur (design kit)".to_string()),
                        (true, "Tailwind + Starwind".to_string()),
                    ],
                    0,
                )?;
            let auth = ui
                && confirm(
                    &theme,
                    "Comptes du site (Laravel Fortify : connexion, inscription, compte) ?",
                    false,
                )?;
            Options::Laravel(LaravelOptions {
                starter,
                dark,
                pest,
                database,
                frontend,
                ui,
                starwind,
                theme: None,
                auth,
                icons: None,
            })
        }
        Framework::Wagtail => {
            let extra: String = Input::with_theme(&theme)
                .with_prompt("Paquets pip supplémentaires (séparés par des espaces, vide = aucun)")
                .allow_empty(true)
                .validate_with(|s: &String| {
                    if s.split_whitespace().all(valid_pip_spec) {
                        Ok(())
                    } else {
                        Err("nom de paquet pip invalide".to_string())
                    }
                })
                .interact_text()
                .map_err(err)?;
            let migrate = confirm(&theme, "Exécuter `manage.py migrate` ?", true)?;
            let ui = confirm(
                &theme,
                "Catalogue de composants (design kit CSS, page /components/) ?",
                false,
            )?;
            let starwind = ui
                && select(
                    &theme,
                    "Kit de composants",
                    &[
                        (false, "CSS pur (design kit)".to_string()),
                        (true, "Tailwind + Starwind".to_string()),
                    ],
                    0,
                )?;
            let auth = ui
                && confirm(
                    &theme,
                    "Comptes du site (authentification Django : connexion, inscription, compte) ?",
                    false,
                )?;
            Options::Wagtail(WagtailOptions {
                packages: extra.split_whitespace().map(String::from).collect(),
                migrate,
                ui,
                starwind,
                theme: None,
                auth,
                icons: None,
            })
        }
        Framework::Rust => {
            let kinds: Vec<(CrateKind, String)> = vec![
                (CrateKind::Bin, "binaire (cargo new --bin)".into()),
                (CrateKind::Lib, "bibliothèque (cargo new --lib)".into()),
            ];
            let kind = select(&theme, "Type de crate", &kinds, 0)?;
            let labels: Vec<&str> = DEPS.iter().map(|(n, _)| *n).collect();
            let chosen = MultiSelect::with_theme(&theme)
                .with_prompt("Dépendances (cargo add)")
                .items(&labels)
                .interact()
                .map_err(err)?;
            Options::Rust(RustOptions {
                kind,
                edition: None,
                deps: chosen.iter().map(|i| DEPS[*i].0.to_string()).collect(),
            })
        }
    };

    req.install = if framework == Framework::Wagtail {
        true
    } else {
        confirm(&theme, "Installer les dépendances ?", true)?
    };
    req.verify = confirm(
        &theme,
        "Vérifier le résultat (build / tests / lint) ?",
        true,
    )?;
    req.git = confirm(&theme, "Initialiser un dépôt Git avec un commit ?", true)?;
    Ok(req)
}

/// Aperçu du plan puis confirmation.
pub fn confirm_plan(req: &Request) -> Result<bool> {
    let plan = crate::scaffold::plan(req)?;
    println!("\n{}", plan.preview());
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Lancer la génération ?")
        .default(true)
        .interact()
        .map_err(err)
}

/// Questions « plateforme » : interface, base de données, authentification, langues.
fn platform_prompts(theme: &ColorfulTheme, o: &mut AstroOptions) -> Result<()> {
    let kits = vec![
        (
            UiKit::None,
            "Thème AlphaRust par défaut (CSS simple)".to_string(),
        ),
        (
            UiKit::Starwind,
            "Tailwind 4 + Starwind UI (outil officiel `starwind`)".to_string(),
        ),
        (
            UiKit::Concordia,
            "CSS pur Concordia : jetons, 4 variantes, ~90 composants + thème WCAG".to_string(),
        ),
    ];
    o.ui = select(theme, "Interface", &kits, 0)?;
    if o.ui == UiKit::Concordia {
        let custom = confirm(
            theme,
            "Personnaliser les couleurs de marque (contrôle WCAG) ?",
            false,
        )?;
        if custom {
            let ask = |label: &str, default: &str| -> Result<String> {
                Input::with_theme(theme)
                    .with_prompt(label)
                    .default(default.to_string())
                    .validate_with(|s: &String| {
                        if crate::scaffold::theme::parse_hex(s).is_some() {
                            Ok(())
                        } else {
                            Err("couleur hexadécimale attendue, ex. #4f46e5".to_string())
                        }
                    })
                    .interact_text()
                    .map_err(err)
            };
            o.theme = Some(ThemeColors {
                primary: ask("Couleur primaire", crate::scaffold::theme::DEFAULT_PRIMARY)?,
                secondary: ask(
                    "Couleur secondaire",
                    crate::scaffold::theme::DEFAULT_SECONDARY,
                )?,
                accent: ask("Couleur d'accent", crate::scaffold::theme::DEFAULT_ACCENT)?,
            });
        }
    }
    let dbs = vec![
        (Db::None, "Aucune (site statique)".to_string()),
        (Db::Sqlite, "SQLite local (zéro service)".to_string()),
        (Db::Postgres, "PostgreSQL (comme atomic)".to_string()),
    ];
    o.database = select(theme, "Base de données (Drizzle ORM)", &dbs, 0)?;
    if o.database != Db::None {
        o.auth = confirm(
            theme,
            "Authentification Better Auth (inscription, connexion, audit) ?",
            true,
        )?;
        if o.auth {
            let labels: Vec<String> = AUTH_PLUGINS
                .iter()
                .map(|(n, d)| format!("{n:<13} {d}"))
                .collect();
            let chosen = MultiSelect::with_theme(theme)
                .with_prompt("Plugins Better Auth")
                .items(&labels)
                .interact()
                .map_err(err)?;
            o.auth_plugins = chosen
                .iter()
                .map(|i| AUTH_PLUGINS[*i].0.to_string())
                .collect();
        }
    }
    let labels: Vec<String> = LOCALES.iter().map(|(c, n)| format!("{c}  {n}")).collect();
    let chosen = MultiSelect::with_theme(theme)
        .with_prompt("Langues (i18n) — aucune ou au moins deux ; la première cochée est la langue par défaut")
        .items(&labels)
        .interact()
        .map_err(err)?;
    if chosen.len() >= 2 {
        o.i18n = chosen.iter().map(|i| LOCALES[*i].0.to_string()).collect();
    }
    Ok(())
}
