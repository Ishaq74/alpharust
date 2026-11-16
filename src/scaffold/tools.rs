//! Détection des outils externes (preflight) : présence, version minimale, indice d'installation.

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Résultat de la détection d'un outil.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ToolStatus {
    pub id: String,
    pub label: String,
    /// L'outil est nécessaire pour la configuration choisie (sinon : recommandé).
    pub required: bool,
    pub found: bool,
    pub path: Option<String>,
    pub version: Option<String>,
    pub min: Option<String>,
    /// Présent et assez récent.
    pub ok: bool,
    pub hint: String,
}

/// Version `x.y.z` (les composantes manquantes valent 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub u32, pub u32, pub u32);

impl Version {
    pub fn parse(s: &str) -> Option<Version> {
        let mut parts = [0u32; 3];
        let start = s.find(|c: char| c.is_ascii_digit())?;
        let mut n = 0;
        for chunk in s[start..].split(|c: char| !(c.is_ascii_digit() || c == '.')) {
            if chunk.is_empty() {
                continue;
            }
            for p in chunk.split('.') {
                if n < 3 {
                    if let Ok(v) = p.parse::<u32>() {
                        parts[n] = v;
                        n += 1;
                        continue;
                    }
                }
                break;
            }
            break;
        }
        (n > 0).then_some(Version(parts[0], parts[1], parts[2]))
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

/// Description statique d'un outil.
pub struct ToolSpec {
    pub id: &'static str,
    pub label: &'static str,
    /// Exécutables candidats (le premier trouvé dans le PATH gagne).
    pub programs: &'static [&'static str],
    pub version_args: &'static [&'static str],
    pub min: Option<&'static str>,
    pub hint: &'static str,
}

pub const NODE: ToolSpec = ToolSpec {
    id: "node",
    label: "Node.js",
    programs: &["node"],
    version_args: &["--version"],
    min: Some("22.12.0"),
    // alpharust-ignore: no-hardcoded-urls
    hint: "Installer Node.js ≥ 22 : https://nodejs.org (ou `nvm install 22`)",
};
pub const NPM: ToolSpec = ToolSpec {
    id: "npm",
    label: "npm",
    programs: &["npm"],
    version_args: &["--version"],
    min: Some("9.6.5"),
    // alpharust-ignore: no-hardcoded-urls
    hint: "npm est fourni avec Node.js : https://nodejs.org",
};
pub const PNPM: ToolSpec = ToolSpec {
    id: "pnpm",
    label: "pnpm",
    programs: &["pnpm"],
    version_args: &["--version"],
    min: Some("8.0.0"),
    // alpharust-ignore: no-hardcoded-urls
    hint: "Installer pnpm : `corepack enable pnpm` ou https://pnpm.io/installation",
};
pub const YARN: ToolSpec = ToolSpec {
    id: "yarn",
    label: "Yarn",
    programs: &["yarn"],
    version_args: &["--version"],
    min: Some("1.22.0"),
    // alpharust-ignore: no-hardcoded-urls
    hint: "Installer Yarn : `corepack enable yarn` ou https://yarnpkg.com",
};
pub const BUN: ToolSpec = ToolSpec {
    id: "bun",
    label: "Bun",
    programs: &["bun"],
    version_args: &["--version"],
    min: Some("1.1.0"),
    // alpharust-ignore: no-hardcoded-urls
    hint: "Installer Bun : https://bun.sh",
};
pub const PHP: ToolSpec = ToolSpec {
    id: "php",
    label: "PHP",
    programs: &["php"],
    version_args: &["--version"],
    min: Some("8.3.0"),
    // alpharust-ignore: no-hardcoded-urls
    hint: "Installer PHP ≥ 8.3 : https://www.php.net/downloads (ou `brew install php`, `apt install php-cli`)",
};
pub const COMPOSER: ToolSpec = ToolSpec {
    id: "composer",
    label: "Composer",
    programs: &["composer", "composer.phar"],
    version_args: &["--version"],
    min: Some("2.0.0"),
    // alpharust-ignore: no-hardcoded-urls
    hint: "Installer Composer : https://getcomposer.org/download/",
};
pub const PYTHON: ToolSpec = ToolSpec {
    id: "python",
    label: "Python",
    programs: &["python3", "python"],
    version_args: &["--version"],
    min: Some("3.10.0"),
    // alpharust-ignore: no-hardcoded-urls
    hint: "Installer Python ≥ 3.10 (3.12 recommandé) : https://www.python.org/downloads/",
};
pub const GIT: ToolSpec = ToolSpec {
    id: "git",
    label: "Git",
    programs: &["git"],
    version_args: &["--version"],
    min: None,
    // alpharust-ignore: no-hardcoded-urls
    hint: "Installer Git : https://git-scm.com/downloads",
};
pub const CARGO: ToolSpec = ToolSpec {
    id: "cargo",
    label: "Cargo",
    programs: &["cargo"],
    version_args: &["--version"],
    min: Some("1.85.0"),
    // alpharust-ignore: no-hardcoded-urls
    hint: "Installer Rust ≥ 1.85 : https://rustup.rs",
};

/// Cherche `program` dans le PATH (et renvoie son chemin complet).
pub fn which(program: &str) -> Option<PathBuf> {
    let p = Path::new(program);
    if p.components().count() > 1 {
        return p.is_file().then(|| p.to_path_buf());
    }
    let path = std::env::var_os("PATH")?;
    let exts: Vec<String> = if cfg!(windows) {
        std::env::var("PATHEXT")
            .unwrap_or(".EXE;.CMD;.BAT".into())
            .split(';')
            .map(|s| s.to_lowercase())
            .collect()
    } else {
        vec![String::new()]
    };
    for dir in std::env::split_paths(&path) {
        for ext in &exts {
            let cand = dir.join(format!("{program}{ext}"));
            if cand.is_file() && is_executable(&cand) {
                return Some(cand);
            }
        }
    }
    None
}

#[cfg(unix)]
fn is_executable(p: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    p.metadata()
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}
#[cfg(not(unix))]
fn is_executable(_: &Path) -> bool {
    true
}

/// Exécute `program args…` et renvoie la sortie combinée (stdout puis stderr).
pub fn capture(program: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .env("NO_COLOR", "1")
        .output()
        .ok()?;
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    out.status.success().then_some(text)
}

/// Détecte un outil. `required` indique s'il est nécessaire à la configuration choisie.
pub fn detect(spec: &ToolSpec, required: bool) -> ToolStatus {
    let mut status = ToolStatus {
        id: spec.id.into(),
        label: spec.label.into(),
        required,
        found: false,
        path: None,
        version: None,
        min: spec.min.map(String::from),
        ok: false,
        hint: spec.hint.into(),
    };
    for prog in spec.programs {
        if let Some(path) = which(prog) {
            status.found = true;
            status.path = Some(path.display().to_string());
            if let Some(out) = capture(&path, spec.version_args) {
                status.version = Version::parse(&out).map(|v| v.to_string());
            }
            let min = spec.min.and_then(Version::parse);
            status.ok = match (min, status.version.as_deref().and_then(Version::parse)) {
                (None, _) => true,
                (Some(_), None) => true, // version illisible : on ne bloque pas
                (Some(m), Some(v)) => v >= m,
            };
            break;
        }
    }
    status
}

/// Extensions PHP chargées (`php -m`), en minuscules.
pub fn php_extensions() -> Vec<String> {
    which("php")
        .and_then(|p| capture(&p, &["-m"]))
        .map(|out| {
            out.lines()
                .map(|l| l.trim().to_lowercase())
                .filter(|l| !l.is_empty() && !l.starts_with('['))
                .collect()
        })
        .unwrap_or_default()
}

/// `python -m venv` est-il utilisable (module `venv` + `ensurepip`) ?
pub fn python_venv_available(python: &Path) -> bool {
    capture(python, &["-c", "import venv, ensurepip"]).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_versions() {
        assert_eq!(Version::parse("v22.12.1"), Some(Version(22, 12, 1)));
        assert_eq!(Version::parse("PHP 8.5.8 (cli)"), Some(Version(8, 5, 8)));
        assert_eq!(Version::parse("Python 3.13.5"), Some(Version(3, 13, 5)));
        assert_eq!(
            Version::parse("Composer version 2.8.4 2025"),
            Some(Version(2, 8, 4))
        );
        assert_eq!(
            Version::parse("cargo 1.85.1 (d73d2caf9 2024-12-31)"),
            Some(Version(1, 85, 1))
        );
        assert_eq!(Version::parse("rien"), None);
        assert!(Version(22, 12, 0) < Version(22, 23, 4));
        assert!(Version(8, 3, 0) <= Version(8, 5, 8));
    }

    #[test]
    fn unknown_tool_is_reported_missing() {
        let spec = ToolSpec {
            id: "nope",
            label: "Nope",
            programs: &["alpharust-does-not-exist-xyz"],
            version_args: &["--version"],
            min: Some("1.0.0"),
            hint: "installer nope",
        };
        let s = detect(&spec, true);
        assert!(!s.found && !s.ok && s.required);
        assert_eq!(s.hint, "installer nope");
    }

    #[test]
    fn which_finds_sh_and_rejects_junk() {
        #[cfg(unix)]
        assert!(which("sh").is_some());
        assert!(which("alpharust-does-not-exist-xyz").is_none());
    }
}
