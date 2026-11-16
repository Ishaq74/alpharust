//! Exécution des étapes : commandes externes et actions internes (couche, patchs, manifeste, git, lint).

use super::astro::{PackageManager, TsLevel};
use super::exec::{self, Cmd, Cwd};
use super::laravel::Database;
use super::layer::{self, Ctx};
use super::platform::Db;
use super::{Action, Internal, LintSummary, Options, Plan, Request, Step, MANIFEST};
use crate::generators::Vars;
use anyhow::{anyhow, bail, Context, Result};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use std::sync::atomic::AtomicBool;

/// État partagé entre les étapes.
pub struct State {
    pub ctx: Ctx,
    pub lint: Option<LintSummary>,
}

impl State {
    pub fn new(req: &Request, plan: &Plan) -> State {
        State {
            ctx: build_ctx(req, plan),
            lint: None,
        }
    }
}

/// Variables et drapeaux de la couche de templates.
pub fn build_ctx(req: &Request, plan: &Plan) -> Ctx {
    let vars = Vars::new(&req.name).expect("nom validé");
    let mut ctx = Ctx::default();
    ctx.var("name", &vars.name)
        .var("slug", &vars.slug)
        .var("pkg", &vars.pkg)
        .var("title", &vars.title)
        .var("upper", &vars.upper)
        .var("year", current_year().to_string())
        .var("folder", req.folder())
        .var("target", plan.target.display().to_string())
        .var("version", env!("CARGO_PKG_VERSION"));
    ctx.flag("install", req.install);
    ctx.flag(req.framework().id(), true);
    match &req.options {
        Options::Astro(o) => {
            let pm = o.package_manager;
            let (lock, ci) = match pm {
                PackageManager::Pnpm => ("pnpm-lock.yaml", "pnpm install --frozen-lockfile"),
                PackageManager::Yarn => ("yarn.lock", "yarn install --frozen-lockfile"),
                PackageManager::Bun => ("bun.lock", "bun install --frozen-lockfile"),
                PackageManager::Npm => ("package-lock.json", "npm ci"),
            };
            ctx.var("lockfile", lock).var("install_ci", ci);
            ctx.flag("pnpm", pm == PackageManager::Pnpm);
            ctx.flag("bun", pm == PackageManager::Bun);
            ctx.flag(
                "corepack",
                matches!(pm, PackageManager::Pnpm | PackageManager::Yarn),
            );
            ctx.var("pm", o.package_manager.id())
                .var("run", o.package_manager.run_prefix())
                .var("install_cmd", o.package_manager.install_cmd())
                .var("template", o.template.id())
                .var("typescript", o.typescript.id())
                .var("site", o.site.clone().unwrap_or_default());
            for (k, on) in o.flags(req.install) {
                ctx.flag(k, on);
            }
            let db_name = vars.slug.replace('-', "_");
            let default_url = match o.database {
                // identifiants de développement local uniquement (serveur PostgreSQL de la machine)
                Db::Postgres => {
                    let creds = format!("{DEV_PG_USER}:{DEV_PG_USER}");
                    format!("postgresql://{creds}@localhost:5432/{db_name}")
                }
                _ => format!("./data/{db_name}.db"),
            };
            let local = o.db_url.clone().unwrap_or(default_url);
            let test = match o.database {
                Db::Postgres => match local.rsplit_once('/') {
                    Some((head, base)) => {
                        format!("{head}/{}_test", base.split('?').next().unwrap_or(base))
                    }
                    None => local.clone(),
                },
                _ => ":memory:".to_string(),
            };
            ctx.var("db_url", local)
                .var("db_url_test", test)
                .var("db_name", db_name)
                .var("secret", random_secret())
                .var("dialect", o.database.dialect())
                .var(
                    "drizzle_dialect",
                    if o.database == Db::Postgres {
                        "postgresql"
                    } else {
                        "sqlite"
                    },
                )
                .var(
                    "locales_ts",
                    o.locales()
                        .iter()
                        .map(|l| format!("'{l}'"))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
                .var("default_locale", o.default_locale().unwrap_or("fr"));
        }
        Options::Laravel(o) => {
            ctx.var("database", o.starter.id());
            for (k, on) in o.flags() {
                ctx.flag(k, on);
            }
        }
        Options::Wagtail(_) => {}
        Options::Rust(o) => {
            ctx.flag("bin", o.kind == super::rust::CrateKind::Bin);
            ctx.flag("lib", o.kind == super::rust::CrateKind::Lib);
        }
    }
    ctx
}

/// Secret aléatoire (64 caractères hexadécimaux) pour `BETTER_AUTH_SECRET`.
fn random_secret() -> String {
    let mut buf = [0u8; 32];
    if getrandom::fill(&mut buf).is_err() {
        // repli improbable : dérive du temps et du processus (jamais de secret constant)
        let seed = unix_now() as u64 ^ (std::process::id() as u64) << 20;
        for (i, b) in buf.iter_mut().enumerate() {
            *b = (seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(i as u64 * 1442695040888963407)
                >> 24) as u8;
        }
    }
    buf.iter().map(|b| format!("{b:02x}")).collect()
}

fn current_year() -> i64 {
    civil(unix_now()).0
}

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// (année, mois, jour, heure, minute, seconde) UTC depuis un timestamp Unix.
pub fn civil(ts: i64) -> (i64, u32, u32, u32, u32, u32) {
    let days = ts.div_euclid(86_400);
    let secs = ts.rem_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (
        y,
        m,
        d,
        (secs / 3600) as u32,
        ((secs % 3600) / 60) as u32,
        (secs % 60) as u32,
    )
}

pub fn iso_now() -> String {
    let (y, mo, d, h, mi, s) = civil(unix_now());
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{s:02}Z")
}

/// Fichiers du projet (chemins relatifs triés), sans dossiers de dépendances ni de build.
pub fn list_files(target: &Path) -> Vec<String> {
    const SKIP: &[&str] = &[
        "node_modules",
        ".git",
        "vendor",
        ".venv",
        "target",
        "dist",
        ".astro",
        "__pycache__",
        ".cache",
        ".pytest_cache",
    ];
    let mut out = Vec::new();
    for e in walkdir::WalkDir::new(target)
        .into_iter()
        .filter_entry(|e| {
            e.depth() == 0
                || !e.file_type().is_dir()
                || !SKIP.contains(&e.file_name().to_string_lossy().as_ref())
        })
        .filter_map(|e| e.ok())
    {
        if e.file_type().is_file() {
            if let Ok(rel) = e.path().strip_prefix(target) {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
        if out.len() >= 5000 {
            break;
        }
    }
    out.sort();
    out
}

fn short_tail(tail: &[String]) -> String {
    let n = tail.len().saturating_sub(8);
    tail[n..].join("\n")
}

/// Exécute une étape ; renvoie une note éventuelle.
pub fn run_step(
    req: &Request,
    plan: &Plan,
    step: &Step,
    state: &mut State,
    cancel: &AtomicBool,
    on_line: &mut dyn FnMut(&str),
) -> Result<Option<String>> {
    match &step.action {
        Action::Run(cmd) => run_cmd(plan, cmd, cancel, on_line).map(|_| None),
        Action::Do(what) => run_internal(req, plan, what, state, cancel, on_line),
    }
}

fn run_cmd(
    plan: &Plan,
    cmd: &Cmd,
    cancel: &AtomicBool,
    on_line: &mut dyn FnMut(&str),
) -> Result<()> {
    let cwd = match cmd.cwd {
        Cwd::Parent => plan
            .target
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| ".".into()),
        Cwd::Project => plan.target.clone(),
    };
    if !cwd.is_dir() {
        bail!("dossier de travail introuvable : {}", cwd.display());
    }
    on_line(&format!("$ {}", cmd.display()));
    let outcome = exec::run(
        &cmd.program,
        &cmd.args,
        &cwd,
        cancel,
        exec::step_timeout(),
        on_line,
    )
    .map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            anyhow!(
                "outil introuvable : `{}` (installer l'outil ou lancer `alpharust doctor`)",
                cmd.program
            )
        } else {
            anyhow!("impossible de lancer `{}` : {}", cmd.program, e)
        }
    })?;
    if outcome.cancelled {
        bail!("annulé");
    }
    if outcome.timed_out {
        bail!(
            "délai dépassé ({} s) — variable ALPHARUST_STEP_TIMEOUT pour l'augmenter",
            exec::step_timeout().as_secs()
        );
    }
    if !outcome.success {
        bail!(
            "`{}` a échoué (code {}){}",
            cmd.display(),
            outcome
                .code
                .map(|c| c.to_string())
                .unwrap_or_else(|| "signal".into()),
            if outcome.tail.is_empty() {
                String::new()
            } else {
                format!("\n{}", short_tail(&outcome.tail))
            }
        );
    }
    Ok(())
}

fn run_internal(
    req: &Request,
    plan: &Plan,
    what: &Internal,
    state: &mut State,
    cancel: &AtomicBool,
    on_line: &mut dyn FnMut(&str),
) -> Result<Option<String>> {
    let target = &plan.target;
    match what {
        Internal::Prepare => prepare(req, target, on_line).map(Some),
        Internal::Mkdir => {
            fs::create_dir_all(target)
                .with_context(|| format!("création de {}", target.display()))?;
            Ok(None)
        }
        Internal::Layer(names) => {
            let applied = layer::apply(names, &state.ctx, target)?;
            for f in &applied.written {
                on_line(&format!("+ {f}"));
            }
            for f in &applied.replaced {
                on_line(&format!("~ {f} (fichier officiel remplacé)"));
            }
            for f in &applied.kept {
                on_line(&format!("= {f} (fichier officiel conservé)"));
            }
            Ok(Some(format!(
                "{} créé(s), {} remplacé(s), {} conservé(s)",
                applied.written.len(),
                applied.replaced.len(),
                applied.kept.len()
            )))
        }
        Internal::AstroTsconfig(level) => patch_tsconfig(target, *level).map(Some),
        Internal::AstroSite(url) => patch_astro_site(target, url).map(Some),
        Internal::AstroPaths => patch_tsconfig_paths(target).map(Some),
        Internal::AstroConfig {
            ssr,
            locales,
            concordia_icons,
        } => patch_astro_config(target, *ssr, locales, *concordia_icons).map(Some),
        Internal::CssTheme(t) => write_css_theme(target, t).map(Some),
        Internal::PackageScripts(scripts) => patch_package_scripts(target, scripts).map(Some),
        Internal::LaravelEnv(db) => patch_laravel_env(target, *db, &req.name).map(Some),
        Internal::LaravelRoutes => patch_laravel_routes(target).map(Some),
        Internal::WriteManifest => write_manifest(req, target).map(|_| None),
        Internal::GitInit => git_init(target, cancel, on_line).map(Some),
        Internal::Lint => lint(target, state, on_line).map(Some),
        Internal::Freeze => freeze(target, cancel, on_line).map(Some),
        Internal::Requirements(pkgs) => append_requirements(target, pkgs).map(Some),
    }
}

/// Contrôle la destination ; supprime un ancien projet AlphaRust si `force`.
fn prepare(req: &Request, target: &Path, on_line: &mut dyn FnMut(&str)) -> Result<String> {
    if target.parent().is_none() || target.file_name().is_none() {
        bail!("destination invalide : {}", target.display());
    }
    if target.exists() {
        if !target.is_dir() {
            bail!("{} existe et n'est pas un dossier", target.display());
        }
        let empty = fs::read_dir(target)?.next().is_none();
        if empty {
            // les outils officiels (cargo new…) veulent créer le dossier eux-mêmes
            if !matches!(req.options, Options::Wagtail(_)) {
                fs::remove_dir(target)?;
            }
        } else if !req.force {
            bail!(
                "{} existe déjà et n'est pas vide (utiliser --force pour remplacer un projet AlphaRust existant)",
                target.display()
            );
        } else if !target.join(MANIFEST).is_file() {
            bail!(
                "{} n'est pas un projet généré par AlphaRust ({} absent) : suppression refusée",
                target.display(),
                MANIFEST
            );
        } else {
            on_line(&format!(
                "suppression de l'ancien projet {}",
                target.display()
            ));
            fs::remove_dir_all(target)
                .with_context(|| format!("suppression de {}", target.display()))?;
        }
    }
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).with_context(|| format!("création de {}", parent.display()))?;
    }
    Ok(format!("destination prête : {}", target.display()))
}

fn read_json(path: &Path) -> Result<Value> {
    let text =
        fs::read_to_string(path).with_context(|| format!("lecture de {}", path.display()))?;
    serde_json::from_str(&text)
        .with_context(|| format!("{} n'est pas du JSON valide", path.display()))
}

fn write_json(path: &Path, v: &Value) -> Result<()> {
    let mut text = serde_json::to_string_pretty(v)?;
    text.push('\n');
    fs::write(path, text).with_context(|| format!("écriture de {}", path.display()))
}

pub fn patch_tsconfig(target: &Path, level: TsLevel) -> Result<String> {
    let path = target.join("tsconfig.json");
    let mut v = read_json(&path)?;
    let extends = format!("astro/tsconfigs/{}", level.id());
    v["extends"] = json!(extends);
    write_json(&path, &v)?;
    Ok(format!("extends = {extends}"))
}

pub fn patch_package_scripts(target: &Path, scripts: &[(&str, &str)]) -> Result<String> {
    let path = target.join("package.json");
    let mut v = read_json(&path)?;
    let obj = v
        .as_object_mut()
        .ok_or_else(|| anyhow!("package.json invalide"))?;
    let entry = obj.entry("scripts").or_insert_with(|| json!({}));
    let map = entry
        .as_object_mut()
        .ok_or_else(|| anyhow!("`scripts` invalide dans package.json"))?;
    for (k, val) in scripts {
        map.insert((*k).to_string(), json!(val));
    }
    write_json(&path, &v)?;
    Ok(scripts
        .iter()
        .map(|(k, _)| *k)
        .collect::<Vec<_>>()
        .join(", "))
}

/// Alias de chemins, comme dans le dépôt atomic.
pub const TS_PATHS: &[(&str, &str)] = &[
    ("@/*", "src/*"),
    ("@styles/*", "src/styles/*"),
    ("@layouts/*", "src/layouts/*"),
    ("@components/*", "src/components/*"),
    ("@i18n/*", "src/i18n/*"),
    ("@lib/*", "src/lib/*"),
    ("@database/*", "src/database/*"),
];

pub fn patch_tsconfig_paths(target: &Path) -> Result<String> {
    let path = target.join("tsconfig.json");
    let mut v = read_json(&path)?;
    let opts = v
        .as_object_mut()
        .ok_or_else(|| anyhow!("tsconfig.json invalide"))?
        .entry("compilerOptions")
        .or_insert_with(|| json!({}));
    let o = opts
        .as_object_mut()
        .ok_or_else(|| anyhow!("`compilerOptions` invalide"))?;
    o.entry("baseUrl").or_insert(json!("."));
    let paths = o.entry("paths").or_insert_with(|| json!({}));
    let p = paths
        .as_object_mut()
        .ok_or_else(|| anyhow!("`paths` invalide"))?;
    for (alias, dir) in TS_PATHS {
        p.insert((*alias).to_string(), json!([dir]));
    }
    write_json(&path, &v)?;
    Ok(format!("{} alias", TS_PATHS.len()))
}

fn astro_config_path(target: &Path) -> Result<std::path::PathBuf> {
    ["astro.config.mjs", "astro.config.ts", "astro.config.js"]
        .iter()
        .map(|f| target.join(f))
        .find(|p| p.is_file())
        .ok_or_else(|| anyhow!("astro.config.* introuvable"))
}

/// Insère `text` juste après `defineConfig({`.
fn insert_in_define_config(src: &str, text: &str) -> Result<String> {
    let i = src
        .find("defineConfig({")
        .ok_or_else(|| anyhow!("`defineConfig({{` introuvable dans astro.config"))?;
    let at = i + "defineConfig({".len();
    let mut s = src.to_string();
    s.insert_str(at, text);
    Ok(s)
}

/// Rendu serveur (`output: 'server'`), i18n natif d'Astro, `checkOrigin` et icônes.
/// L'adaptateur Node est ajouté par `astro add node` ; ici on ne touche qu'aux options.
pub fn patch_astro_config(
    target: &Path,
    ssr: bool,
    locales: &[String],
    icons: bool,
) -> Result<String> {
    let path = astro_config_path(target)?;
    let mut text = fs::read_to_string(&path)?;
    let mut done: Vec<&str> = vec![];
    if ssr && !text.contains("output:") {
        text = insert_in_define_config(&text, "\n  output: 'server',")?;
        done.push("output server");
    }
    if !locales.is_empty() && !text.contains("i18n:") {
        let list = locales
            .iter()
            .map(|l| format!("'{l}'"))
            .collect::<Vec<_>>()
            .join(", ");
        text = insert_in_define_config(
            &text,
            &format!(
                "\n  i18n: {{\n    locales: [{list}],\n    defaultLocale: '{}',\n    routing: {{ prefixDefaultLocale: false }},\n  }},",
                locales[0]
            ),
        )?;
        done.push("i18n");
    }
    if !text.contains("security:") {
        // `checkOrigin` est actif par défaut depuis Astro 5 : on l'explicite (CSRF des formulaires et API)
        text = insert_in_define_config(&text, "\n  security: { checkOrigin: true },")?;
        done.push("checkOrigin");
    }
    if icons && !text.contains("astro-icon") {
        if let Some(i) = text.find("integrations: [") {
            let at = i + "integrations: [".len();
            text.insert_str(at, "icon(), ");
        } else {
            text = insert_in_define_config(&text, "\n  integrations: [icon()],")?;
        }
        // import après le dernier `import` existant
        let last_import_end = text
            .lines()
            .scan(0usize, |pos, l| {
                let start = *pos;
                *pos += l.len() + 1;
                Some((start, l))
            })
            .filter(|(_, l)| l.trim_start().starts_with("import "))
            .map(|(start, l)| start + l.len() + 1)
            .last()
            .unwrap_or(0);
        text.insert_str(
            last_import_end.min(text.len()),
            "import icon from 'astro-icon';\n",
        );
        done.push("astro-icon");
    }
    fs::write(&path, text)?;
    Ok(if done.is_empty() {
        "déjà configuré".into()
    } else {
        done.join(", ")
    })
}

pub fn write_css_theme(target: &Path, t: &super::theme::ThemeColors) -> Result<String> {
    let css = t.css()?;
    let dir = target.join("src/styles");
    fs::create_dir_all(&dir)?;
    fs::write(dir.join("theme.css"), css)?;
    t.summary()
}

pub fn patch_astro_site(target: &Path, url: &str) -> Result<String> {
    if url
        .chars()
        .any(|c| c == '\'' || c == '"' || c == '\\' || c.is_whitespace())
    {
        bail!("URL du site invalide : {url}");
    }
    let path = ["astro.config.mjs", "astro.config.ts", "astro.config.js"]
        .iter()
        .map(|f| target.join(f))
        .find(|p| p.is_file())
        .ok_or_else(|| anyhow!("astro.config.* introuvable"))?;
    let text = fs::read_to_string(&path)?;
    let re_existing = regex::Regex::new(r#"site:\s*['"][^'"]*['"]"#).unwrap();
    let new = if re_existing.is_match(&text) {
        re_existing
            .replace(&text, format!("site: '{url}'"))
            .into_owned()
    } else if let Some(i) = text.find("defineConfig({") {
        let at = i + "defineConfig({".len();
        let mut s = text.clone();
        s.insert_str(at, &format!("\n  site: '{url}',"));
        s
    } else {
        bail!("`defineConfig({{` introuvable dans {}", path.display());
    };
    fs::write(&path, new)?;
    Ok(format!("site = {url}"))
}

pub fn patch_laravel_env(target: &Path, db: Database, name: &str) -> Result<String> {
    let pkg = Vars::new(name)?.pkg;
    let (port, user) = match db {
        Database::Mysql => ("3306", "root"),
        Database::Pgsql => ("5432", "postgres"),
        Database::Sqlite => return Ok("sqlite : aucune modification".into()),
    };
    let settings = [
        ("DB_CONNECTION", db.id().to_string()),
        ("DB_HOST", "127.0.0.1".into()),
        ("DB_PORT", port.into()),
        ("DB_DATABASE", pkg),
        ("DB_USERNAME", user.into()),
        ("DB_PASSWORD", String::new()),
    ];
    let mut touched = 0;
    for file in [".env", ".env.example"] {
        let path = target.join(file);
        if !path.is_file() {
            continue;
        }
        let mut lines: Vec<String> = fs::read_to_string(&path)?
            .lines()
            .map(String::from)
            .collect();
        for (key, value) in &settings {
            let new = format!("{key}={value}");
            let mut done = false;
            for l in lines.iter_mut() {
                let t = l.trim_start_matches('#').trim();
                if t.starts_with(&format!("{key}=")) {
                    *l = new.clone();
                    done = true;
                    break;
                }
            }
            if !done {
                lines.push(new);
            }
        }
        fs::write(&path, lines.join("\n") + "\n")?;
        touched += 1;
    }
    Ok(format!(
        "{touched} fichier(s) .env mis à jour pour {}",
        db.id()
    ))
}

pub fn patch_laravel_routes(target: &Path) -> Result<String> {
    let path = target.join("routes/web.php");
    let text =
        fs::read_to_string(&path).with_context(|| format!("lecture de {}", path.display()))?;
    let line = "require __DIR__.'/pages.php';";
    if text.contains(line) {
        return Ok("déjà branché".into());
    }
    let mut s = text;
    if !s.ends_with('\n') {
        s.push('\n');
    }
    s.push_str(&format!("\n{line}\n"));
    fs::write(&path, s)?;
    Ok("routes/pages.php inclus dans routes/web.php".into())
}

fn append_requirements(target: &Path, pkgs: &[String]) -> Result<String> {
    let path = target.join("requirements.txt");
    let mut text = fs::read_to_string(&path).unwrap_or_default();
    let mut added = 0;
    for p in pkgs {
        let name = p
            .split(['[', '=', '<', '>', '~', '!'])
            .next()
            .unwrap_or(p)
            .to_lowercase();
        let exists = text.lines().any(|l| {
            l.split(['[', '=', '<', '>', '~', '!'])
                .next()
                .map(|n| n.trim().to_lowercase() == name)
                .unwrap_or(false)
        });
        if !exists {
            if !text.is_empty() && !text.ends_with('\n') {
                text.push('\n');
            }
            text.push_str(p);
            text.push('\n');
            added += 1;
        }
    }
    fs::write(&path, text)?;
    Ok(format!("{added} paquet(s) ajouté(s)"))
}

fn freeze(target: &Path, cancel: &AtomicBool, on_line: &mut dyn FnMut(&str)) -> Result<String> {
    let py = if cfg!(windows) {
        "./.venv/Scripts/python"
    } else {
        "./.venv/bin/python"
    };
    let mut lines: Vec<String> = Vec::new();
    let out = exec::run(
        py,
        &["-m".into(), "pip".into(), "freeze".into()],
        target,
        cancel,
        std::time::Duration::from_secs(120),
        &mut |l| lines.push(l.to_string()),
    )?;
    if !out.success {
        bail!("pip freeze a échoué");
    }
    lines.retain(|l| l.contains("=="));
    lines.sort_by_key(|l| l.to_lowercase());
    fs::write(
        target.join("requirements-lock.txt"),
        lines.join("\n") + "\n",
    )?;
    on_line(&format!("{} paquets figés", lines.len()));
    Ok(format!("{} paquets figés", lines.len()))
}

/// Masque les identifiants d'une URL de base de données (`://utilisateur:motdepasse@` → `://***@`).
/// Utilisateur (et mot de passe) PostgreSQL par défaut d'une installation locale.
const DEV_PG_USER: &str = "postgres";

pub fn mask_url_credentials(url: &str) -> String {
    match (url.find("://"), url.find('@')) {
        (Some(scheme), Some(at)) if at > scheme + 3 && !url[scheme + 3..at].contains('/') => {
            format!("{}://***@{}", &url[..scheme], &url[at + 1..])
        }
        _ => url.to_string(),
    }
}

fn redact_secrets(v: &mut Value) {
    match v {
        Value::Object(map) => {
            for (k, val) in map.iter_mut() {
                if k == "db_url" {
                    if let Value::String(s) = val {
                        *s = mask_url_credentials(s);
                    }
                } else {
                    redact_secrets(val);
                }
            }
        }
        Value::Array(a) => a.iter_mut().for_each(redact_secrets),
        _ => {}
    }
}

pub fn write_manifest(req: &Request, target: &Path) -> Result<()> {
    let mut request = serde_json::to_value(req)?;
    redact_secrets(&mut request);
    let manifest = json!({
        "generator": "alpharust",
        "version": env!("CARGO_PKG_VERSION"),
        "created_at": iso_now(),
        "name": req.name,
        "framework": req.framework().id(),
        "official_tool": req.framework().official_tool(),
        "request": request,
    });
    write_json(&target.join(MANIFEST), &manifest)
}

fn git_cfg(target: &Path, key: &str) -> bool {
    super::tools::which("git")
        .and_then(|g| {
            std::process::Command::new(g)
                .args(["-C", &target.display().to_string(), "config", key])
                .output()
                .ok()
        })
        .map(|o| o.status.success() && !o.stdout.is_empty())
        .unwrap_or(false)
}

fn git_init(target: &Path, cancel: &AtomicBool, on_line: &mut dyn FnMut(&str)) -> Result<String> {
    if super::tools::which("git").is_none() {
        bail!("git introuvable (dépôt non initialisé)");
    }
    let t = std::time::Duration::from_secs(120);
    let step = |args: Vec<String>, on_line: &mut dyn FnMut(&str)| -> Result<()> {
        on_line(&format!("$ git {}", args.join(" ")));
        let o = exec::run("git", &args, target, cancel, t, on_line)?;
        if !o.success {
            bail!("`git {}` a échoué\n{}", args.join(" "), short_tail(&o.tail));
        }
        Ok(())
    };
    step(vec!["init".into(), "-q".into()], on_line)?;
    // branche `main` quelle que soit la version de git
    step(
        vec![
            "symbolic-ref".into(),
            "HEAD".into(),
            "refs/heads/main".into(),
        ],
        on_line,
    )?;
    step(vec!["add".into(), "-A".into()], on_line)?;
    let mut commit: Vec<String> = Vec::new();
    if !git_cfg(target, "user.name") || !git_cfg(target, "user.email") {
        commit.extend(
            [
                "-c",
                "user.name=AlphaRust",
                "-c",
                "user.email=alpharust@users.noreply.local",
            ]
            .map(String::from),
        );
    }
    commit.extend(
        [
            "commit",
            "-q",
            "-m",
            "chore: projet initial généré par AlphaRust",
        ]
        .map(String::from),
    );
    step(commit, on_line)?;
    Ok("dépôt initialisé (branche main)".into())
}

fn lint(target: &Path, state: &mut State, on_line: &mut dyn FnMut(&str)) -> Result<String> {
    let report = crate::engine::lint(target, None)?;
    let (mut e, mut w, mut i) = (0, 0, 0);
    let mut sample = Vec::new();
    for v in &report.violations {
        match v.severity.as_str() {
            "error" => e += 1,
            "warning" => w += 1,
            _ => i += 1,
        }
        if sample.len() < 12 {
            sample.push(format!("{}:{} [{}] {}", v.file, v.line, v.rule, v.message));
        }
    }
    for s in &sample {
        on_line(s);
    }
    let summary = LintSummary {
        files: report.files_scanned,
        rules: report.rules_used,
        errors: e,
        warnings: w,
        infos: i,
        profile: report.profile.clone(),
        sample,
    };
    let note = format!(
        "{} fichier(s), {} règle(s) : {} erreur(s), {} avertissement(s), {} info(s)",
        summary.files, summary.rules, e, w, i
    );
    on_line(&note);
    state.lint = Some(summary);
    if e > 0 || w > 0 {
        bail!("{note}");
    }
    Ok(note)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_dates() {
        assert_eq!(civil(0), (1970, 1, 1, 0, 0, 0));
        assert_eq!(civil(1_000_000_000), (2001, 9, 9, 1, 46, 40));
        assert_eq!(civil(1_788_000_000).0, 2026);
        assert!(iso_now().ends_with('Z') && iso_now().len() == 20);
    }

    #[test]
    fn tsconfig_and_scripts_patches_keep_other_keys_in_order() {
        let d = tempfile::tempdir().unwrap();
        fs::write(
            d.path().join("tsconfig.json"),
            r#"{"extends":"astro/tsconfigs/base","include":[".astro/types.d.ts","**/*"]}"#,
        )
        .unwrap();
        patch_tsconfig(d.path(), TsLevel::Strictest).unwrap();
        let t = fs::read_to_string(d.path().join("tsconfig.json")).unwrap();
        assert!(t.contains("astro/tsconfigs/strictest") && t.contains("include"));

        fs::write(
            d.path().join("package.json"),
            r#"{"name":"x","scripts":{"dev":"astro dev"},"dependencies":{"astro":"^7"}}"#,
        )
        .unwrap();
        patch_package_scripts(d.path(), &[("check", "astro check")]).unwrap();
        let p = fs::read_to_string(d.path().join("package.json")).unwrap();
        assert!(p.find("\"name\"").unwrap() < p.find("\"scripts\"").unwrap());
        assert!(p.find("\"scripts\"").unwrap() < p.find("\"dependencies\"").unwrap());
        assert!(p.contains("\"check\": \"astro check\"") && p.contains("\"dev\""));
    }

    #[test]
    fn astro_site_is_inserted_or_replaced() {
        let d = tempfile::tempdir().unwrap();
        let cfg = d.path().join("astro.config.mjs");
        fs::write(
            &cfg,
            "import { defineConfig } from 'astro/config';\nexport default defineConfig({});\n",
        )
        .unwrap();
        // alpharust-ignore: no-hardcoded-urls
        patch_astro_site(d.path(), "https://exemple.com").unwrap();
        assert!(fs::read_to_string(&cfg)
            .unwrap()
            // alpharust-ignore: no-hardcoded-urls
            .contains("site: 'https://exemple.com',"));
        // alpharust-ignore: no-hardcoded-urls
        patch_astro_site(d.path(), "https://autre.org").unwrap();
        let t = fs::read_to_string(&cfg).unwrap();
        // alpharust-ignore: no-hardcoded-urls
        assert!(t.contains("site: 'https://autre.org'") && !t.contains("exemple.com"));
        assert!(patch_astro_site(d.path(), "https://a'b").is_err());
    }

    #[test]
    fn laravel_env_and_routes_patches() {
        let d = tempfile::tempdir().unwrap();
        fs::create_dir_all(d.path().join("routes")).unwrap();
        fs::write(
            d.path().join(".env.example"),
            "APP_NAME=Laravel\nDB_CONNECTION=sqlite\n# DB_HOST=127.0.0.1\n# DB_PORT=3306\n# DB_DATABASE=laravel\n# DB_USERNAME=root\n# DB_PASSWORD=\n",
        )
        .unwrap();
        patch_laravel_env(d.path(), Database::Pgsql, "Mon-Site").unwrap();
        let e = fs::read_to_string(d.path().join(".env.example")).unwrap();
        assert!(
            e.contains("DB_CONNECTION=pgsql")
                && e.contains("DB_PORT=5432")
                && e.contains("DB_DATABASE=mon_site")
        );
        assert!(e.contains("DB_USERNAME=postgres") && !e.contains("# DB_HOST"));
        assert_eq!(e.matches("DB_CONNECTION").count(), 1);

        fs::write(d.path().join("routes/web.php"), "<?php\n\nuse X;\n").unwrap();
        patch_laravel_routes(d.path()).unwrap();
        patch_laravel_routes(d.path()).unwrap();
        let w = fs::read_to_string(d.path().join("routes/web.php")).unwrap();
        assert_eq!(w.matches("pages.php").count(), 1);
    }

    #[test]
    fn requirements_are_appended_once() {
        let d = tempfile::tempdir().unwrap();
        fs::write(d.path().join("requirements.txt"), "Django>=6\nwagtail>=8").unwrap();
        append_requirements(d.path(), &["psycopg[binary]".into(), "django".into()]).unwrap();
        let t = fs::read_to_string(d.path().join("requirements.txt")).unwrap();
        assert_eq!(t, "Django>=6\nwagtail>=8\npsycopg[binary]\n");
    }

    #[test]
    fn prepare_protects_foreign_directories() {
        let d = tempfile::tempdir().unwrap();
        let target = d.path().join("proj");
        fs::create_dir_all(&target).unwrap();
        fs::write(target.join("precieux.txt"), "x").unwrap();
        let mut req = Request::new(super::super::Framework::Rust, "proj");
        req.path = Some(target.clone());
        // sans --force : refus
        assert!(prepare(&req, &target, &mut |_| {}).is_err());
        // avec --force mais sans manifeste : refus (dossier étranger)
        req.force = true;
        let e = prepare(&req, &target, &mut |_| {}).unwrap_err().to_string();
        assert!(
            e.contains("n'est pas un projet généré par AlphaRust"),
            "{e}"
        );
        assert!(target.join("precieux.txt").exists());
        // avec manifeste : remplacement autorisé
        fs::write(target.join(MANIFEST), "{}").unwrap();
        prepare(&req, &target, &mut |_| {}).unwrap();
        assert!(!target.exists());
        // dossier vide : accepté
        fs::create_dir_all(&target).unwrap();
        req.force = false;
        prepare(&req, &target, &mut |_| {}).unwrap();
    }
    #[test]
    fn database_urls_are_masked_in_the_manifest() {
        let creds = format!("{}:{}", "admin", "secret");
        let url = format!("postgresql://{creds}@db.example:5432/app");
        assert_eq!(
            mask_url_credentials(&url),
            "postgresql://***@db.example:5432/app"
        );
        assert_eq!(mask_url_credentials("./data/app.db"), "./data/app.db");
        let mut v = json!({"options": {"db_url": url, "other": "x"}});
        redact_secrets(&mut v);
        assert_eq!(
            v["options"]["db_url"],
            "postgresql://***@db.example:5432/app"
        );
        assert_eq!(v["options"]["other"], "x");
    }

    #[test]
    fn tsconfig_paths_and_astro_config_are_patched() {
        let d = tempfile::tempdir().unwrap();
        fs::write(
            d.path().join("tsconfig.json"),
            r#"{"extends":"astro/tsconfigs/strict"}"#,
        )
        .unwrap();
        patch_tsconfig_paths(d.path()).unwrap();
        let t: Value =
            serde_json::from_str(&fs::read_to_string(d.path().join("tsconfig.json")).unwrap())
                .unwrap();
        assert_eq!(t["compilerOptions"]["paths"]["@lib/*"][0], "src/lib/*");
        assert_eq!(
            t["compilerOptions"]["paths"]["@database/*"][0],
            "src/database/*"
        );
        assert_eq!(t["extends"], "astro/tsconfigs/strict");
    }

    #[test]
    fn css_theme_file_is_written_and_validated() {
        let d = tempfile::tempdir().unwrap();
        let summary =
            write_css_theme(d.path(), &super::super::theme::ThemeColors::default()).unwrap();
        assert!(summary.contains("primary"));
        let css = fs::read_to_string(d.path().join("src/styles/theme.css")).unwrap();
        assert!(css.contains("--color-primary") && css.contains(":root[data-theme='dark']"));
        let bad = super::super::theme::ThemeColors {
            primary: "pas-une-couleur".into(),
            ..Default::default()
        };
        assert!(write_css_theme(d.path(), &bad).is_err());
    }
}
