use crate::glob::glob_match_loose;
use crate::parser::Rule;
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

pub const CONFIG_FILE: &str = ".alpharust.toml";

/// Configuration optionnelle `.alpharust.toml`.
///
/// ```toml
/// profile  = "strict"                 # profil par défaut de `lint`
/// exclude  = ["rules/**", "vendor/**"]  # globs relatifs à la racine analysée
/// disable  = ["core/no-hardcoded-urls"] # ids/noms de règles (globs acceptés)
/// fail_on  = "error"                  # error | warning | info
/// [severity]                          # surcharges de sévérité
/// "core/no-todo-in-prod" = "warning"
/// ```
#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub profile: Option<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub disable: Vec<String>,
    pub fail_on: Option<String>,
    #[serde(default)]
    pub severity: BTreeMap<String, String>,
}

impl Config {
    /// Charge `.alpharust.toml` depuis le répertoire courant.
    pub fn load() -> Result<Self> {
        Self::load_from(Path::new("."))
    }

    /// Charge `.alpharust.toml` depuis `dir` (config par défaut si absent).
    pub fn load_from(dir: &Path) -> Result<Self> {
        let path = dir.join(CONFIG_FILE);
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("lecture de {}", path.display()))?;
        let config: Config =
            toml::from_str(&content).with_context(|| format!("{} invalide", path.display()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if let Some(f) = &self.fail_on {
            if !crate::parser::SEVERITIES.contains(&f.as_str()) {
                bail!("fail_on `{}` invalide (error | warning | info)", f);
            }
        }
        for (rule, sev) in &self.severity {
            if !crate::parser::SEVERITIES.contains(&sev.as_str()) {
                bail!("severity `{}` invalide pour la règle `{}`", sev, rule);
            }
        }
        Ok(())
    }

    /// Retire les règles désactivées et applique les surcharges de sévérité.
    pub fn apply(&self, rules: Vec<Rule>) -> Vec<Rule> {
        rules
            .into_iter()
            .filter(|r| {
                !self
                    .disable
                    .iter()
                    .any(|d| glob_match_loose(d, &r.id) || glob_match_loose(d, &r.name))
            })
            .map(|mut r| {
                if let Some(sev) = self
                    .severity
                    .get(&r.id)
                    .or_else(|| self.severity.get(&r.name))
                {
                    r.severity = sev.clone();
                }
                r
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_config() {
        let toml = "profile=\"strict\"\nexclude=[\"a/**\"]\ndisable=[\"core/*\"]\nfail_on=\"warning\"\n[severity]\n\"x/y\"=\"info\"\n";
        let c: Config = toml::from_str(toml).unwrap();
        c.validate().unwrap();
        assert_eq!(c.profile.as_deref(), Some("strict"));
        assert_eq!(c.severity["x/y"], "info");
    }

    #[test]
    fn rejects_unknown_keys_and_bad_values() {
        assert!(toml::from_str::<Config>("profil=\"x\"").is_err());
        let c: Config = toml::from_str("fail_on=\"fatal\"").unwrap();
        assert!(c.validate().is_err());
    }

    #[test]
    fn load_from_missing_dir_is_default() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(Config::load_from(dir.path()).unwrap(), Config::default());
    }

    #[test]
    fn apply_disables_and_overrides() {
        let mk = |id: &str, name: &str| {
            Rule::from_markdown(
                Path::new(&format!("{}.md", id)),
                &format!("---\nname: {}\napply: \"*\"\nseverity: error\n---\n", name),
            )
            .map(|mut r| {
                r.id = id.to_string();
                r
            })
            .unwrap()
        };
        let c = Config {
            disable: vec!["core/a".into()],
            severity: BTreeMap::from([("core/b".to_string(), "info".to_string())]),
            ..Default::default()
        };
        let out = c.apply(vec![mk("core/a", "a"), mk("core/b", "b")]);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].severity, "info");
    }
}
