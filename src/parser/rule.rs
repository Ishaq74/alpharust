use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Sévérités acceptées dans le frontmatter.
pub const SEVERITIES: [&str; 3] = ["error", "warning", "info"];

/// Exemples auto-testés d'une règle (`alpharust check-rules`).
///
/// Le caractère `¦` est supprimé avant le test : il permet d'écrire des exemples
/// qui ressemblent à de vrais secrets sans déclencher les scanners de secrets.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Examples {
    #[serde(default)]
    pub bad: Vec<String>,
    #[serde(default)]
    pub good: Vec<String>,
    /// Nom de fichier fictif (utile aux checkers qui dépendent du nom, ex. `package.json`).
    #[serde(default)]
    pub file: Option<String>,
}

/// Frontmatter tel qu'écrit dans les fichiers de règles.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RuleMeta {
    name: String,
    apply: String,
    severity: String,
    #[serde(default)]
    pattern: Option<String>,
    #[serde(default)]
    checker: Option<String>,
    #[serde(default)]
    args: BTreeMap<String, serde_json::Value>,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    examples: Examples,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Identifiant = chemin relatif sous `rules/` sans extension (ex. `core/no-eval`).
    pub id: String,
    pub name: String,
    pub apply: String,
    pub severity: String,
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(default)]
    pub checker: Option<String>,
    #[serde(default)]
    pub args: BTreeMap<String, serde_json::Value>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub examples: Examples,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub path: PathBuf,
}

/// Résultat d'un chargement du dossier `rules/`.
#[derive(Debug, Default)]
pub struct LoadReport {
    pub rules: Vec<Rule>,
    /// Fichiers de règles invalides : (chemin, erreur).
    pub errors: Vec<(PathBuf, String)>,
    /// Vrai si les règles viennent du cache sur disque.
    pub from_cache: bool,
}

impl Rule {
    /// Charge toutes les règles valides (cache activé). Les fichiers invalides
    /// sont signalés sur stderr (jamais ignorés silencieusement).
    pub fn load_all() -> Result<Vec<Self>> {
        let report = Self::load_report(true)?;
        for (path, err) in &report.errors {
            eprintln!("warning: règle ignorée {}: {}", path.display(), err);
        }
        Ok(report.rules)
    }

    /// Charge `rules/` (racine résolue par [`crate::paths`]).
    pub fn load_report(use_cache: bool) -> Result<LoadReport> {
        Self::load_dir(&crate::paths::rules_dir(), use_cache)
    }

    pub fn load_dir(root: &Path, use_cache: bool) -> Result<LoadReport> {
        if !root.exists() {
            return Ok(LoadReport::default());
        }
        let use_cache = use_cache && std::env::var_os("ALPHARUST_NO_CACHE").is_none();
        let fingerprint = super::cache::fingerprint(root);
        if use_cache {
            if let Some(rules) = super::cache::load(fingerprint) {
                return Ok(LoadReport {
                    rules,
                    errors: Vec::new(),
                    from_cache: true,
                });
            }
        }

        let mut report = LoadReport::default();
        for entry in WalkDir::new(root)
            .sort_by_file_name()
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !entry.file_type().is_file() || !is_rule_file(path) {
                continue;
            }
            match Self::from_file_in(root, path) {
                Ok(rule) => report.rules.push(rule),
                Err(e) => report.errors.push((path.to_path_buf(), format!("{:#}", e))),
            }
        }
        if use_cache && report.errors.is_empty() {
            super::cache::store(fingerprint, &report.rules);
        }
        Ok(report)
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        Self::from_file_in(&crate::paths::rules_dir(), path)
    }

    fn from_file_in(root: &Path, path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("lecture de {}", path.display()))?;
        let mut rule = Self::from_markdown(path, &content)?;
        let rel = path.strip_prefix(root).unwrap_or(path).with_extension("");
        rule.id = rel
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/");
        Ok(rule)
    }

    pub fn from_markdown(path: &Path, content: &str) -> Result<Self> {
        let (front, body) = split_frontmatter(content)?;
        let meta: RuleMeta = serde_yaml::from_str(&front)
            .with_context(|| format!("frontmatter invalide dans {}", path.display()))?;
        let id = path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| meta.name.clone());
        let rule = Rule {
            id,
            name: meta.name,
            apply: meta.apply,
            severity: meta.severity,
            pattern: meta.pattern.filter(|p| !p.is_empty()),
            checker: meta.checker,
            args: meta.args,
            message: meta.message,
            examples: meta.examples,
            description: body.trim().to_string(),
            path: path.to_path_buf(),
        };
        let problems = rule.validate();
        if !problems.is_empty() {
            bail!("{}", problems.join("; "));
        }
        Ok(rule)
    }

    /// Liste des problèmes de définition (vide = règle valide).
    pub fn validate(&self) -> Vec<String> {
        let mut problems = Vec::new();
        if self.name.trim().is_empty() {
            problems.push("`name` vide".to_string());
        }
        if self.apply.trim().is_empty() {
            problems.push("`apply` vide".to_string());
        }
        if !SEVERITIES.contains(&self.severity.as_str()) {
            problems.push(format!(
                "severity `{}` invalide (attendu: {})",
                self.severity,
                SEVERITIES.join(" | ")
            ));
        }
        if let Some(p) = &self.pattern {
            if let Err(e) = Regex::new(p) {
                problems.push(format!("regex invalide: {}", e));
            }
        }
        if let Some(c) = &self.checker {
            if !crate::checkers::is_known(c) {
                problems.push(format!(
                    "checker inconnu `{}` (disponibles: {})",
                    c,
                    crate::checkers::names().join(", ")
                ));
            }
            if self.pattern.is_some() {
                problems.push("`pattern` et `checker` sont exclusifs".to_string());
            }
        }
        problems
    }

    /// `regex`, `checker` ou `info` (règle documentaire, non exécutée).
    pub fn kind(&self) -> &'static str {
        if self.checker.is_some() {
            "checker"
        } else if self.pattern.is_some() {
            "regex"
        } else {
            "info"
        }
    }

    /// Message affiché dans les violations : `message` du frontmatter,
    /// sinon le premier titre Markdown de la description, sinon le nom.
    pub fn display_message(&self) -> String {
        if let Some(m) = &self.message {
            return m.clone();
        }
        self.description
            .lines()
            .map(str::trim)
            .find(|l| l.starts_with('#'))
            .map(|l| l.trim_start_matches('#').trim().to_string())
            .filter(|l| !l.is_empty())
            .unwrap_or_else(|| self.name.clone())
    }
}

fn is_rule_file(path: &Path) -> bool {
    if path.extension().and_then(|s| s.to_str()) != Some("md") {
        return false;
    }
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    !(name.starts_with('_') || name.eq_ignore_ascii_case("readme.md"))
}

fn split_frontmatter(content: &str) -> Result<(String, String)> {
    let content = content.replace("\r\n", "\n");
    let content = content.trim_start_matches('\u{feff}').trim_start();
    if !content.starts_with("---") {
        bail!("frontmatter manquant");
    }
    let rest = &content[3..];
    let end = rest
        .find("\n---")
        .ok_or_else(|| anyhow!("frontmatter non fermé"))?;
    let front = rest[..end].trim().to_string();
    let body = rest[end + 4..].to_string();
    Ok((front, body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_rule() {
        let md = "---\nname: test-rule\napply: \"*\"\nseverity: error\npattern: \"foo\"\n---\n\n# Titre\n\nDescription here.\n";
        let rule = Rule::from_markdown(Path::new("test.md"), md).unwrap();
        assert_eq!(rule.name, "test-rule");
        assert_eq!(rule.severity, "error");
        assert_eq!(rule.apply, "*");
        assert!(rule.pattern.is_some());
        assert_eq!(rule.kind(), "regex");
        assert_eq!(rule.display_message(), "Titre");
    }

    #[test]
    fn empty_pattern_is_info_rule() {
        let md = "---\nname: x\napply: \"*\"\nseverity: info\npattern: \"\"\n---\nbody";
        let rule = Rule::from_markdown(Path::new("x.md"), md).unwrap();
        assert_eq!(rule.kind(), "info");
    }

    #[test]
    fn rejects_bad_severity_regex_and_checker() {
        let bad_sev = "---\nname: x\napply: \"*\"\nseverity: fatal\n---\n";
        assert!(Rule::from_markdown(Path::new("x.md"), bad_sev).is_err());
        let bad_re = "---\nname: x\napply: \"*\"\nseverity: error\npattern: \"(\"\n---\n";
        assert!(Rule::from_markdown(Path::new("x.md"), bad_re).is_err());
        let bad_checker = "---\nname: x\napply: \"*\"\nseverity: error\nchecker: nope\n---\n";
        assert!(Rule::from_markdown(Path::new("x.md"), bad_checker).is_err());
        let both =
            "---\nname: x\napply: \"*\"\nseverity: error\npattern: a\nchecker: rust-unsafe\n---\n";
        assert!(Rule::from_markdown(Path::new("x.md"), both).is_err());
    }

    #[test]
    fn rejects_unknown_fields_and_missing_frontmatter() {
        let typo = "---\nname: x\napply: \"*\"\nseverity: error\npatern: a\n---\n";
        assert!(Rule::from_markdown(Path::new("x.md"), typo).is_err());
        assert!(Rule::from_markdown(Path::new("x.md"), "# pas de frontmatter").is_err());
        assert!(Rule::from_markdown(Path::new("x.md"), "---\nname: x\n").is_err());
    }

    #[test]
    fn handles_crlf() {
        let md = "---\r\nname: crlf\r\napply: \"*\"\r\nseverity: info\r\n---\r\n\r\n# T\r\n";
        assert_eq!(
            Rule::from_markdown(Path::new("c.md"), md).unwrap().name,
            "crlf"
        );
    }
}
