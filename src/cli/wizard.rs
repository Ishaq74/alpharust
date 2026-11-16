//! Assistant interactif (`alpharust new`) : questions guidées, aperçu des commandes exactes, confirmation.

use crate::scaffold::astro::{AstroOptions, PackageManager, Template, TsLevel, INTEGRATIONS};
use crate::scaffold::laravel::{Database, LaravelOptions, Starter};
use crate::scaffold::rust::{CrateKind, RustOptions, DEPS};
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
            Options::Astro(AstroOptions {
                package_manager,
                template,
                typescript,
                integrations,
                site,
                typecheck,
            })
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
            Options::Laravel(LaravelOptions {
                starter,
                dark,
                pest,
                database,
                frontend,
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
            Options::Wagtail(WagtailOptions {
                packages: extra.split_whitespace().map(String::from).collect(),
                migrate,
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
