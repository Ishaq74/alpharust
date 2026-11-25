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

/// Source des réponses de l'assistant : le terminal (dialoguer) en vrai, un script en test.
pub trait Prompter {
    /// Choix unique ; renvoie l'index choisi.
    fn select(&mut self, prompt: &str, labels: &[String], default: usize) -> Result<usize>;
    /// Oui / non.
    fn confirm(&mut self, prompt: &str, default: bool) -> Result<bool>;
    /// Choix multiple ; renvoie les index cochés.
    fn multi(&mut self, prompt: &str, labels: &[String], defaults: &[bool]) -> Result<Vec<usize>>;
    /// Texte libre validé ; sans valeur par défaut, la réponse vide est acceptée.
    fn input(
        &mut self,
        prompt: &str,
        initial: &str,
        default: Option<&str>,
        validate: &dyn Fn(&str) -> Result<(), String>,
    ) -> Result<String>;
}

/// Questions posées dans le terminal avec dialoguer.
#[derive(Default)]
pub struct Terminal {
    theme: ColorfulTheme,
}

impl Prompter for Terminal {
    fn select(&mut self, prompt: &str, labels: &[String], default: usize) -> Result<usize> {
        Select::with_theme(&self.theme)
            .with_prompt(prompt)
            .items(labels)
            .default(default)
            .interact()
            .map_err(err)
    }

    fn confirm(&mut self, prompt: &str, default: bool) -> Result<bool> {
        Confirm::with_theme(&self.theme)
            .with_prompt(prompt)
            .default(default)
            .interact()
            .map_err(err)
    }

    fn multi(&mut self, prompt: &str, labels: &[String], defaults: &[bool]) -> Result<Vec<usize>> {
        let mut m = MultiSelect::with_theme(&self.theme);
        m = m.with_prompt(prompt).items(labels);
        if !defaults.is_empty() {
            m = m.defaults(defaults);
        }
        m.interact().map_err(err)
    }

    fn input(
        &mut self,
        prompt: &str,
        initial: &str,
        default: Option<&str>,
        validate: &dyn Fn(&str) -> Result<(), String>,
    ) -> Result<String> {
        let mut i = Input::<String>::with_theme(&self.theme)
            .with_prompt(prompt)
            .with_initial_text(initial)
            .allow_empty(default.is_none())
            .validate_with(|s: &String| validate(s));
        if let Some(d) = default {
            i = i.default(d.to_string());
        }
        i.interact_text().map_err(err)
    }
}

fn select<T: Copy>(
    p: &mut dyn Prompter,
    prompt: &str,
    items: &[(T, String)],
    default: usize,
) -> Result<T> {
    let labels: Vec<String> = items.iter().map(|(_, l)| l.clone()).collect();
    let i = p.select(prompt, &labels, default)?;
    items
        .get(i)
        .map(|(t, _)| *t)
        .ok_or_else(|| anyhow!("choix hors liste pour « {prompt} »"))
}

fn confirm(p: &mut dyn Prompter, prompt: &str, default: bool) -> Result<bool> {
    p.confirm(prompt, default)
}

/// Pose les questions et renvoie la demande construite (non encore confirmée).
pub fn ask(preset_name: Option<String>, preset_framework: Option<Framework>) -> Result<Request> {
    ask_with(&mut Terminal::default(), preset_name, preset_framework)
}

/// Même assistant, avec une source de réponses au choix (testable).
pub fn ask_with(
    p: &mut dyn Prompter,
    preset_name: Option<String>,
    preset_framework: Option<Framework>,
) -> Result<Request> {
    println!("AlphaRust — nouveau projet (les outils officiels font le gros du travail)\n");

    let framework = match preset_framework {
        Some(f) => f,
        None => {
            let items: Vec<(Framework, String)> = Framework::ALL
                .iter()
                .map(|f| (*f, format!("{:<18} via {}", f.label(), f.official_tool())))
                .collect();
            select(p, "Framework", &items, 0)?
        }
    };

    let name = p.input(
        "Nom du projet",
        &preset_name.unwrap_or_default(),
        Some(&format!("mon-{}", framework.id())),
        &|s: &str| crate::generators::validate_name(s).map_err(|e| e.to_string()),
    )?;

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
            let package_manager = select(p, "Gestionnaire de paquets", &pms, 0)?;
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
            let template = select(p, "Template", &templates, 0)?;
            let ts: Vec<(TsLevel, String)> = vec![
                (TsLevel::Base, "base — permissif".into()),
                (TsLevel::Strict, "strict (recommandé)".into()),
                (TsLevel::Strictest, "strictest — le plus exigeant".into()),
            ];
            let typescript = select(p, "TypeScript", &ts, 1)?;
            let labels: Vec<String> = INTEGRATIONS
                .iter()
                .map(|(n, d)| format!("{n:<9} {d}"))
                .collect();
            let chosen = p.multi(
                "Intégrations (espace pour cocher, entrée pour valider)",
                &labels,
                &[],
            )?;
            let integrations: Vec<String> = chosen
                .iter()
                .map(|i| INTEGRATIONS[*i].0.to_string())
                .collect();
            let needs_site =
                integrations.iter().any(|i| i == "sitemap") || template == Template::Blog;
            let site: Option<String> = if needs_site {
                Some(p.input(
                    "URL publique du site",
                    "",
                    // alpharust-ignore: no-hardcoded-urls
                    Some("https://exemple.com"),
                    &|s: &str| {
                        if s.starts_with("http://") || s.starts_with("https://") {
                            Ok(())
                        } else {
                            Err("l'URL doit commencer par http:// ou https://".to_string())
                        }
                    },
                )?)
            } else {
                None
            };
            let typecheck = confirm(p, "Installer @astrojs/check et vérifier les types ?", true)?;
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
                platform_prompts(p, &mut o)?;
            }
            if confirm(p, "Dockerfile + docker-compose ?", false)? {
                o.docker = true;
            }
            if confirm(p, "Workflow GitHub Actions (check, build, tests) ?", false)? {
                o.ci = true;
            }
            Options::Astro(o)
        }
        Framework::Laravel => {
            let starters: Vec<(Starter, String)> = Starter::ALL
                .iter()
                .map(|s| (*s, s.label().to_string()))
                .collect();
            let starter = select(p, "Kit de démarrage", &starters, 0)?;
            let mut dark = false;
            let mut pest = false;
            if starter != Starter::None {
                if starter.has_frontend() {
                    dark = confirm(p, "Mode sombre (Breeze --dark) ?", false)?;
                }
                pest = confirm(p, "Tests avec Pest au lieu de PHPUnit ?", false)?;
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
            let database = select(p, "Base de données", &dbs, 0)?;
            let frontend = if starter == Starter::None {
                confirm(
                    p,
                    "Installer et compiler les assets (npm install && build) ?",
                    false,
                )?
            } else {
                false
            };
            let ui = starter == Starter::None
                && confirm(
                    p,
                    "Catalogue de composants (design kit CSS, page /components) ?",
                    false,
                )?;
            let starwind = ui
                && select(
                    p,
                    "Kit de composants",
                    &[
                        (false, "CSS pur (design kit)".to_string()),
                        (true, "Tailwind + Starwind".to_string()),
                    ],
                    0,
                )?;
            let auth = ui
                && confirm(
                    p,
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
                langs: if ui { site_langs(p)? } else { vec![] },
            })
        }
        Framework::Wagtail => {
            let extra = p.input(
                "Paquets pip supplémentaires (séparés par des espaces, vide = aucun)",
                "",
                None,
                &|s: &str| {
                    if s.split_whitespace().all(valid_pip_spec) {
                        Ok(())
                    } else {
                        Err("nom de paquet pip invalide".to_string())
                    }
                },
            )?;
            let migrate = confirm(p, "Exécuter `manage.py migrate` ?", true)?;
            let ui = confirm(
                p,
                "Catalogue de composants (design kit CSS, page /components/) ?",
                false,
            )?;
            let starwind = ui
                && select(
                    p,
                    "Kit de composants",
                    &[
                        (false, "CSS pur (design kit)".to_string()),
                        (true, "Tailwind + Starwind".to_string()),
                    ],
                    0,
                )?;
            let auth = ui
                && confirm(
                    p,
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
                langs: if ui { site_langs(p)? } else { vec![] },
            })
        }
        Framework::Rust => {
            let kinds: Vec<(CrateKind, String)> = vec![
                (CrateKind::Bin, "binaire (cargo new --bin)".into()),
                (CrateKind::Lib, "bibliothèque (cargo new --lib)".into()),
            ];
            let kind = select(p, "Type de crate", &kinds, 0)?;
            let labels: Vec<String> = DEPS.iter().map(|(n, _)| (*n).to_string()).collect();
            let chosen = p.multi("Dépendances (cargo add)", &labels, &[])?;
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
        confirm(p, "Installer les dépendances ?", true)?
    };
    req.verify = confirm(p, "Vérifier le résultat (build / tests / lint) ?", true)?;
    req.git = confirm(p, "Initialiser un dépôt Git avec un commit ?", true)?;
    Ok(req)
}

/// Aperçu du plan puis confirmation.
pub fn confirm_plan(req: &Request) -> Result<bool> {
    let plan = crate::scaffold::plan(req)?;
    println!("\n{}", plan.preview());
    Terminal::default().confirm("Lancer la génération ?", true)
}

/// Questions « plateforme » : interface, base de données, authentification, langues.
fn platform_prompts(p: &mut dyn Prompter, o: &mut AstroOptions) -> Result<()> {
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
    o.ui = select(p, "Interface", &kits, 0)?;
    if o.ui == UiKit::Concordia {
        let custom = confirm(
            p,
            "Personnaliser les couleurs de marque (contrôle WCAG) ?",
            false,
        )?;
        if custom {
            let mut ask = |label: &str, default: &str| -> Result<String> {
                p.input(label, "", Some(default), &|s: &str| {
                    if crate::scaffold::theme::parse_hex(s).is_some() {
                        Ok(())
                    } else {
                        Err("couleur hexadécimale attendue, ex. #4f46e5".to_string())
                    }
                })
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
    o.database = select(p, "Base de données (Drizzle ORM)", &dbs, 0)?;
    if o.database != Db::None {
        o.auth = confirm(
            p,
            "Authentification Better Auth (inscription, connexion, audit) ?",
            true,
        )?;
        if o.auth {
            let labels: Vec<String> = AUTH_PLUGINS
                .iter()
                .map(|(n, d)| format!("{n:<13} {d}"))
                .collect();
            let chosen = p.multi("Plugins Better Auth", &labels, &[])?;
            o.auth_plugins = chosen
                .iter()
                .map(|i| AUTH_PLUGINS[*i].0.to_string())
                .collect();
        }
    }
    if o.vitrine() {
        // site vitrine : mêmes langues qu'en Laravel et Wagtail (--langs)
        o.langs = site_langs(p)?;
        return Ok(());
    }
    let labels: Vec<String> = LOCALES.iter().map(|(c, n)| format!("{c}  {n}")).collect();
    let chosen = p.multi(
        "Langues (i18n) — aucune ou au moins deux ; la première cochée est la langue par défaut",
        &labels,
        &[],
    )?;
    if chosen.len() >= 2 {
        o.i18n = chosen.iter().map(|i| LOCALES[*i].0.to_string()).collect();
    }
    Ok(())
}

/// Langues du site vitrine (`--langs`) : fr et en cochés par défaut ; le français (langue par défaut) est toujours inclus.
fn site_langs(p: &mut dyn Prompter) -> Result<Vec<String>> {
    let all = crate::scaffold::site::all();
    let labels: Vec<String> = all
        .iter()
        .map(|l| {
            format!(
                "{:<3} {}{}",
                l.code,
                l.name,
                if l.rtl { " (RTL)" } else { "" }
            )
        })
        .collect();
    let defaults: Vec<bool> = all
        .iter()
        .map(|l| crate::scaffold::site::DEFAULT_LANGS.contains(&l.code.as_str()))
        .collect();
    let chosen = p.multi(
        "Langues du site (le français, langue par défaut, est toujours inclus)",
        &labels,
        &defaults,
    )?;
    let mut langs: Vec<String> = chosen.iter().map(|i| all[*i].code.clone()).collect();
    if !langs
        .iter()
        .any(|c| c == crate::scaffold::site::default_lang())
    {
        langs.insert(0, crate::scaffold::site::default_lang().to_string());
    }
    Ok(langs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Réponse scriptée ; `Default` reprend la valeur proposée par l'assistant.
    #[derive(Debug)]
    enum A {
        Pick(usize),
        Yes,
        No,
        Text(&'static str),
        Many(&'static [usize]),
        Default,
    }

    /// Prompter de test : rejoue des réponses et garde la trace des questions posées.
    struct Script {
        answers: VecDeque<A>,
        asked: Vec<String>,
    }

    impl Script {
        fn new(answers: Vec<A>) -> Self {
            Self {
                answers: answers.into(),
                asked: vec![],
            }
        }
        fn next(&mut self, prompt: &str) -> Result<A> {
            self.asked.push(prompt.to_string());
            self.answers
                .pop_front()
                .ok_or_else(|| anyhow!("pas de réponse scriptée pour « {prompt} »"))
        }
    }

    impl Prompter for Script {
        fn select(&mut self, prompt: &str, labels: &[String], default: usize) -> Result<usize> {
            assert!(!labels.is_empty(), "liste vide pour {prompt}");
            match self.next(prompt)? {
                A::Pick(i) => Ok(i),
                A::Default => Ok(default),
                a => Err(anyhow!("« {prompt} » attend un choix, reçu {a:?}")),
            }
        }
        fn confirm(&mut self, prompt: &str, default: bool) -> Result<bool> {
            match self.next(prompt)? {
                A::Yes => Ok(true),
                A::No => Ok(false),
                A::Default => Ok(default),
                a => Err(anyhow!("« {prompt} » attend oui/non, reçu {a:?}")),
            }
        }
        fn multi(
            &mut self,
            prompt: &str,
            labels: &[String],
            defaults: &[bool],
        ) -> Result<Vec<usize>> {
            match self.next(prompt)? {
                A::Many(v) => {
                    assert!(
                        v.iter().all(|i| *i < labels.len()),
                        "index hors liste pour {prompt}"
                    );
                    Ok(v.to_vec())
                }
                A::Default => Ok(defaults
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| **d)
                    .map(|(i, _)| i)
                    .collect()),
                a => Err(anyhow!("« {prompt} » attend des cases, reçu {a:?}")),
            }
        }
        fn input(
            &mut self,
            prompt: &str,
            initial: &str,
            default: Option<&str>,
            validate: &dyn Fn(&str) -> Result<(), String>,
        ) -> Result<String> {
            let value = match self.next(prompt)? {
                A::Text(t) => t.to_string(),
                A::Default if !initial.is_empty() => initial.to_string(),
                A::Default => default.unwrap_or_default().to_string(),
                a => return Err(anyhow!("« {prompt} » attend du texte, reçu {a:?}")),
            };
            validate(&value).map_err(|e| anyhow!("{prompt} : {e}"))?;
            Ok(value)
        }
    }

    #[test]
    fn rust_bin_with_defaults_builds_a_plan() {
        let mut s = Script::new(vec![
            A::Pick(3), // Framework : Rust (ordre de Framework::ALL)
            A::Default, // nom
            A::Pick(0), // binaire
            A::Many(&[]),
            A::Yes, // installer
            A::No,  // vérifier
            A::No,  // git
        ]);
        let fw = Framework::ALL[3];
        assert_eq!(fw, Framework::Rust, "l'ordre de Framework::ALL a changé");
        let req = ask_with(&mut s, None, None).unwrap();
        assert!(
            s.answers.is_empty(),
            "réponses non consommées : {:?}",
            s.answers
        );
        assert_eq!(req.framework(), Framework::Rust);
        assert_eq!(req.name, "mon-rust");
        assert!(req.install && !req.verify && !req.git);
        assert!(
            s.answers.is_empty(),
            "questions non posées : {:?}",
            s.answers
        );
        crate::scaffold::plan(&req).unwrap();
    }

    #[test]
    fn preset_name_and_framework_skip_the_framework_question() {
        let mut s = Script::new(vec![
            A::Default, // nom pré-rempli
            A::Pick(1), // lib
            A::Many(&[0]),
            A::No,
            A::No,
            A::No,
        ]);
        let req = ask_with(&mut s, Some("ma-lib".into()), Some(Framework::Rust)).unwrap();
        assert!(
            s.answers.is_empty(),
            "réponses non consommées : {:?}",
            s.answers
        );
        assert_eq!(req.name, "ma-lib");
        assert!(!s.asked.iter().any(|q| q == "Framework"));
        let Options::Rust(o) = &req.options else {
            panic!("options Rust attendues")
        };
        assert_eq!(o.kind, CrateKind::Lib);
        assert_eq!(o.deps, vec![DEPS[0].0.to_string()]);
    }

    #[test]
    fn invalid_project_name_is_rejected() {
        let mut s = Script::new(vec![A::Text("../evil")]);
        let e = ask_with(&mut s, None, Some(Framework::Rust)).unwrap_err();
        assert!(e.to_string().contains("Nom du projet"), "{e}");
    }

    #[test]
    fn wagtail_site_always_includes_the_default_language() {
        let mut s = Script::new(vec![
            A::Text("vitrine"),
            A::Text("django-extensions"), // paquets pip
            A::Yes,                       // migrate
            A::Yes,                       // catalogue
            A::Pick(1),                   // Starwind
            A::Yes,                       // comptes
            A::Many(&[1]),                // une seule langue cochée (pas le français)
            A::Yes,                       // vérifier (install forcé en Wagtail)
            A::No,                        // git
        ]);
        let req = ask_with(&mut s, None, Some(Framework::Wagtail)).unwrap();
        assert!(
            s.answers.is_empty(),
            "réponses non consommées : {:?}",
            s.answers
        );
        let Options::Wagtail(o) = &req.options else {
            panic!("options Wagtail attendues")
        };
        assert!(o.ui && o.starwind && o.auth && o.migrate);
        assert_eq!(o.packages, vec!["django-extensions".to_string()]);
        assert_eq!(o.langs[0], crate::scaffold::site::default_lang());
        assert_eq!(o.langs.len(), 2);
        assert!(req.install, "Wagtail installe toujours ses dépendances");
        crate::scaffold::plan(&req).unwrap();
    }

    #[test]
    fn invalid_pip_package_is_rejected() {
        let mut s = Script::new(vec![A::Text("site"), A::Text("bad;rm -rf")]);
        assert!(ask_with(&mut s, None, Some(Framework::Wagtail)).is_err());
    }

    #[test]
    fn laravel_site_with_default_languages() {
        let mut s = Script::new(vec![
            A::Text("vitrine"),
            A::Pick(
                Starter::ALL
                    .iter()
                    .position(|s| *s == Starter::None)
                    .unwrap(),
            ),
            A::Default, // SQLite
            A::No,      // assets npm
            A::Yes,     // catalogue
            A::Pick(0), // CSS pur
            A::No,      // comptes
            A::Default, // langues par défaut
            A::Yes,
            A::Yes,
            A::No,
        ]);
        let req = ask_with(&mut s, None, Some(Framework::Laravel)).unwrap();
        assert!(
            s.answers.is_empty(),
            "réponses non consommées : {:?}",
            s.answers
        );
        let Options::Laravel(o) = &req.options else {
            panic!("options Laravel attendues")
        };
        assert!(o.ui && !o.starwind && !o.auth);
        assert_eq!(o.database, Database::Sqlite);
        let expected: Vec<String> = crate::scaffold::site::DEFAULT_LANGS
            .iter()
            .map(|c| (*c).to_string())
            .collect();
        assert_eq!(o.langs, expected);
        crate::scaffold::plan(&req).unwrap();
    }

    #[test]
    fn astro_platform_with_concordia_custom_colors_and_auth() {
        let mut s = Script::new(vec![
            A::Text("plateforme"),
            A::Default,   // gestionnaire de paquets
            A::Pick(0),   // template alpharust
            A::Default,   // TypeScript strict
            A::Many(&[]), // intégrations
            A::Yes,       // astro check
            A::Pick(2),   // Concordia
            A::Yes,       // couleurs personnalisées
            A::Text("#123456"),
            A::Default,
            A::Default,
            A::Pick(1), // SQLite
            A::Yes,     // Better Auth
            A::Many(&[]),
            A::Default, // langues du site (si vitrine)
            A::No,      // docker
            A::No,      // ci
            A::Yes,
            A::Yes,
            A::No,
        ]);
        let req = ask_with(&mut s, None, Some(Framework::Astro)).unwrap();
        assert!(
            s.answers.is_empty(),
            "réponses non consommées : {:?}",
            s.answers
        );
        let Options::Astro(o) = &req.options else {
            panic!("options Astro attendues")
        };
        assert_eq!(o.template, Template::Alpharust);
        assert_eq!(o.typescript, TsLevel::Strict);
        assert_eq!(o.ui, UiKit::Concordia);
        assert_eq!(o.theme.as_ref().unwrap().primary, "#123456");
        assert_eq!(o.database, Db::Sqlite);
        assert!(o.auth);
        crate::scaffold::plan(&req).unwrap();
    }

    #[test]
    fn invalid_brand_color_is_rejected() {
        let mut s = Script::new(vec![
            A::Text("plateforme"),
            A::Default,
            A::Pick(0),
            A::Default,
            A::Many(&[]),
            A::No,
            A::Pick(2),
            A::Yes,
            A::Text("rouge"),
        ]);
        let e = ask_with(&mut s, None, Some(Framework::Astro)).unwrap_err();
        assert!(e.to_string().contains("hexadécimale"), "{e}");
    }

    #[test]
    fn astro_sitemap_requires_an_http_site_url() {
        let sitemap = INTEGRATIONS
            .iter()
            .position(|(n, _)| *n == "sitemap")
            .unwrap();
        let ok = |url: &'static str| {
            let mut s = Script::new(vec![
                A::Text("blog"),
                A::Default,
                A::Pick(1), // minimal
                A::Default,
                A::Many(Box::leak(vec![sitemap].into_boxed_slice())),
                A::Text(url),
                A::No,
                A::No,
                A::No,
                A::No,
                A::No,
                A::No,
            ]);
            ask_with(&mut s, None, Some(Framework::Astro))
        };
        let req = ok("https://exemple.org").unwrap();
        let Options::Astro(o) = &req.options else {
            panic!()
        };
        assert_eq!(o.site.as_deref(), Some("https://exemple.org"));
        assert!(ok("exemple.org").is_err());
    }

    #[test]
    fn out_of_range_choice_is_an_error_not_a_panic() {
        let mut s = Script::new(vec![A::Pick(99)]);
        assert!(ask_with(&mut s, None, None).is_err());
    }
}
