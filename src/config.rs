use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub profile: Option<String>,
    pub rules: Option<Vec<String>>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Path::new(".alpharust.toml");
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path).context("reading .alpharust.toml")?;
        toml::from_str(&content).context("parsing .alpharust.toml")
    }
}
