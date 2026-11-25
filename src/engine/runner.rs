use crate::checkers::{self, Args, Checker, FileCtx, ProjectCtx};
use crate::glob::glob_match;
use crate::parser::Rule;
use regex::Regex;
use serde::Serialize;
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

/// Dossiers jamais analysés.
const SKIP_DIRS: [&str; 14] = [
    "target",
    ".git",
    "node_modules",
    ".alpharust-cache",
    ".venv",
    "__pycache__",
    "vendor",
    "dist",
    ".astro",
    ".next",
    ".pytest_cache",
    ".cache",
    "storage",
    "staticfiles",
];
/// Fichiers plus gros que ça sont ignorés (binaires, bundles…).
const MAX_FILE_SIZE: u64 = 2 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Violation {
    pub file: String,
    pub line: usize,
    /// Nom de la règle.
    pub rule: String,
    /// Identifiant de la règle (chemin sous `rules/`).
    pub rule_id: String,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Clone, Default)]
pub struct Options {
    /// Globs (relatifs à la racine analysée) à exclure.
    pub exclude: Vec<String>,
}

#[derive(Debug, Default)]
pub struct RunReport {
    pub violations: Vec<Violation>,
    pub files_scanned: usize,
}

// ---------------------------------------------------------------------------
// Sélecteur `apply`
// ---------------------------------------------------------------------------

/// Alias de langages → extensions.
fn language_extensions(name: &str) -> Option<&'static [&'static str]> {
    Some(match name {
        "powershell" => &["ps1", "psm1", "psd1"],
        "php" => &["php", "phtml"],
        "python" => &["py"],
        "rust" => &["rs"],
        "javascript" => &["js", "mjs", "cjs", "jsx", "ts", "tsx"],
        "typescript" => &["ts", "tsx"],
        "markdown" => &["md", "markdown", "mdx"],
        "html" => &["html", "htm"],
        "css" => &["css", "scss", "sass", "less"],
        "sql" => &["sql"],
        "json" => &["json"],
        "yaml" => &["yml", "yaml"],
        "shell" => &["sh", "bash", "zsh"],
        _ => return None,
    })
}

#[derive(Debug, Clone)]
enum Selector {
    All,
    Extension(String),
    Glob(String),
    /// Glob ancré à la racine analysée (préfixe `/` dans `apply`).
    Root(String),
    Language(&'static [&'static str]),
    Readme,
    DocsDir,
    /// Nom de fichier exact (ex. `package.json`).
    Basename(String),
    /// Mot-clé de contexte (ex. `stripe`, `mysql`) : le chemin ou le contenu le mentionne.
    Context(String),
}

fn parse_apply(apply: &str) -> Vec<Selector> {
    apply
        .split(',')
        .map(|p| p.trim().to_lowercase())
        .filter(|p| !p.is_empty())
        .map(|p| {
            if p == "*" {
                Selector::All
            } else if let Some(root) = p.strip_prefix('/') {
                Selector::Root(root.to_string())
            } else if let Some(ext) = p
                .strip_prefix("*.")
                .filter(|e| !e.contains(['*', '/', '?']))
            {
                Selector::Extension(ext.to_string())
            } else if p.contains(['*', '?', '/']) {
                Selector::Glob(p)
            } else if let Some(exts) = language_extensions(&p) {
                Selector::Language(exts)
            } else if p == "readme" {
                Selector::Readme
            } else if p == "docs" {
                Selector::DocsDir
            } else if p.contains('.') {
                Selector::Basename(p)
            } else {
                Selector::Context(p)
            }
        })
        .collect()
}

/// Informations calculées une fois par fichier.
struct FileInfo<'a> {
    lower_rel: String,
    basename: String,
    ext: String,
    content: &'a str,
    lower_content: std::cell::OnceCell<String>,
}

impl<'a> FileInfo<'a> {
    fn new(rel: &str, content: &'a str) -> Self {
        let lower_rel = rel.to_lowercase();
        let basename = lower_rel.rsplit('/').next().unwrap_or("").to_string();
        let ext = basename
            .rsplit_once('.')
            .map(|(_, e)| e.to_string())
            .unwrap_or_default();
        Self {
            lower_rel,
            basename,
            ext,
            content,
            lower_content: std::cell::OnceCell::new(),
        }
    }

    fn lower_content(&self) -> &str {
        self.lower_content
            .get_or_init(|| self.content.to_lowercase())
    }

    fn selected(&self, sel: &Selector) -> bool {
        match sel {
            Selector::All => true,
            Selector::Extension(e) => &self.ext == e,
            Selector::Glob(g) => {
                glob_match(g, &self.lower_rel)
                    || (!g.contains('/') && glob_match(g, &self.basename))
                    || glob_match(&format!("**/{g}"), &self.lower_rel)
            }
            Selector::Root(g) => glob_match(g, &self.lower_rel),
            Selector::Language(exts) => exts.contains(&self.ext.as_str()),
            Selector::Readme => self.basename.starts_with("readme"),
            Selector::DocsDir => self.lower_rel.split('/').any(|c| c == "docs"),
            Selector::Basename(b) => &self.basename == b,
            Selector::Context(word) => {
                self.lower_rel.contains(word.as_str())
                    || self.lower_content().contains(word.as_str())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Règles compilées (regex compilée une seule fois)
// ---------------------------------------------------------------------------

pub struct CompiledRule<'a> {
    pub rule: &'a Rule,
    selectors: Vec<Selector>,
    regex: Option<Regex>,
    checker: Option<Checker>,
    message: String,
}

impl<'a> CompiledRule<'a> {
    /// Compile une règle ; `None` si elle n'est pas exécutable (informative ou invalide).
    pub fn new(rule: &'a Rule) -> Option<Self> {
        let regex = match &rule.pattern {
            Some(p) => match Regex::new(p) {
                Ok(r) => Some(r),
                Err(e) => {
                    eprintln!(
                        "warning: regex invalide dans la règle '{}': {}",
                        rule.name, e
                    );
                    return None;
                }
            },
            None => None,
        };
        let checker = rule.checker.as_deref().and_then(checkers::lookup);
        if regex.is_none() && checker.is_none() {
            return None;
        }
        Some(Self {
            rule,
            selectors: parse_apply(&rule.apply),
            regex,
            checker,
            message: rule.display_message(),
        })
    }

    fn applies_to(&self, info: &FileInfo) -> bool {
        self.selectors.iter().any(|s| info.selected(s))
    }

    fn violation(&self, file: &str, line: usize, message: &str) -> Violation {
        Violation {
            file: file.to_string(),
            line,
            rule: self.rule.name.clone(),
            rule_id: self.rule.id.clone(),
            message: message.to_string(),
            severity: self.rule.severity.clone(),
        }
    }
}

/// `alpharust-ignore` ou `alpharust-ignore: règle1, règle2` sur la ligne ou la précédente.
fn suppressed(lines: &[&str], line: usize, rule: &Rule) -> bool {
    let check = |text: &str| -> bool {
        let Some(idx) = text.find("alpharust-ignore") else {
            return false;
        };
        let rest = &text[idx + "alpharust-ignore".len()..];
        let Some(list) = rest.strip_prefix(':') else {
            return true; // sans liste : tout est ignoré
        };
        let names: Vec<&str> = list
            .split([',', ' '])
            .map(|s| s.trim().trim_end_matches("*/").trim_end_matches("-->"))
            .filter(|s| !s.is_empty())
            .collect();
        names.is_empty() || names.iter().any(|n| *n == rule.name || *n == rule.id)
    };
    let here = lines.get(line.wrapping_sub(1)).is_some_and(|l| check(l));
    let prev = line >= 2 && lines.get(line - 2).is_some_and(|l| check(l));
    here || prev
}

/// Exécute les règles sur `target` (fichier ou dossier) avec les options par défaut.
pub fn run(rules: &[Rule], target: &Path) -> anyhow::Result<Vec<Violation>> {
    Ok(run_with(rules, target, &Options::default())?.violations)
}

pub fn run_with(rules: &[Rule], target: &Path, opts: &Options) -> anyhow::Result<RunReport> {
    if !target.exists() {
        anyhow::bail!("chemin introuvable : {}", target.display());
    }
    let compiled: Vec<CompiledRule> = rules.iter().filter_map(CompiledRule::new).collect();
    let (file_rules, project_rules): (Vec<_>, Vec<_>) = compiled
        .iter()
        .partition(|c| !matches!(c.checker, Some(Checker::Project(_))));

    let mut report = RunReport::default();
    let mut rel_files: Vec<String> = Vec::new();
    let is_dir = target.is_dir();

    let walker = WalkDir::new(target)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| {
            e.depth() == 0
                || !(e.file_type().is_dir()
                    && e.file_name()
                        .to_str()
                        .is_some_and(|n| SKIP_DIRS.contains(&n)))
        });
    for entry in walker.filter_map(std::result::Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let rel = if is_dir {
            path.strip_prefix(target).unwrap_or(path)
        } else {
            Path::new(path.file_name().unwrap_or_default())
        };
        let rel = rel
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/");
        // Les fichiers exclus restent visibles des checkers « projet » (présence de tests, README…).
        rel_files.push(rel.clone());
        if opts
            .exclude
            .iter()
            .any(|g| glob_match(g.trim_start_matches("./"), &rel))
        {
            continue;
        }

        if entry
            .metadata()
            .map(|m| m.len() > MAX_FILE_SIZE)
            .unwrap_or(true)
        {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(path) else {
            continue; // binaire / non UTF-8
        };
        report.files_scanned += 1;
        let info = FileInfo::new(&rel, &content);
        let lines: Vec<&str> = content.lines().collect();
        let display = display_path(path);
        let mut seen: HashSet<(usize, String)> = HashSet::new();

        for cr in &file_rules {
            if !cr.applies_to(&info) {
                continue;
            }
            let mut push = |line: usize, message: &str, report: &mut RunReport| {
                if suppressed(&lines, line, cr.rule) || !seen.insert((line, cr.rule.name.clone())) {
                    return;
                }
                report
                    .violations
                    .push(cr.violation(&display, line, message));
            };
            if let Some(re) = &cr.regex {
                for (i, line) in lines.iter().enumerate() {
                    if re.is_match(line) {
                        push(i + 1, &cr.message, &mut report);
                    }
                }
            } else if let Some(Checker::File(check)) = cr.checker {
                let ctx = FileCtx {
                    path,
                    content: &content,
                    args: Args(&cr.rule.args),
                };
                for f in check(&ctx) {
                    let message = format!("{} — {}", cr.message, f.message);
                    push(f.line.max(1), &message, &mut report);
                }
            }
        }
    }

    if is_dir && !project_rules.is_empty() {
        let display = display_path(target);
        for cr in &project_rules {
            if let Some(Checker::Project(check)) = cr.checker {
                let ctx = ProjectCtx {
                    root: target,
                    files: &rel_files,
                    args: Args(&cr.rule.args),
                };
                for f in check(&ctx) {
                    let message = format!("{} — {}", cr.message, f.message);
                    report
                        .violations
                        .push(cr.violation(&display, f.line.max(1), &message));
                }
            }
        }
    }

    report
        .violations
        .sort_by(|a, b| (&a.file, a.line, &a.rule).cmp(&(&b.file, b.line, &b.rule)));
    Ok(report)
}

/// Chemin affiché dans les rapports : séparateur `/` sur tous les systèmes (SARIF, JSON et texte identiques
/// sous Windows ; sous Unix, `\\` est un caractère de nom de fichier valide et reste tel quel).
fn display_path(path: &Path) -> String {
    let s = path.display().to_string();
    if cfg!(windows) {
        s.replace('\\', "/")
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn rule(front: &str) -> Rule {
        let md = format!("---\n{front}\n---\n\n# Message de test\n");
        let mut r = Rule::from_markdown(Path::new("t.md"), &md).unwrap();
        r.id = format!("core/{}", r.name);
        r
    }

    fn lint_str(rules: &[Rule], file: &str, content: &str) -> Vec<Violation> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(file);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, content).unwrap();
        run(rules, dir.path()).unwrap()
    }

    #[test]
    fn apply_selectors() {
        let r = rule("name: t\napply: \"powershell\"\nseverity: error\npattern: \"bad\"");
        assert_eq!(lint_str(std::slice::from_ref(&r), "a.ps1", "bad").len(), 1);
        assert!(lint_str(&[r], "a.py", "bad").is_empty());

        let r = rule("name: t\napply: \"stripe\"\nseverity: error\npattern: \"bad\"");
        assert_eq!(
            lint_str(std::slice::from_ref(&r), "pay.js", "import stripe\nbad").len(),
            1
        );
        assert!(
            lint_str(&[r], "pay.js", "bad").is_empty(),
            "contexte absent"
        );

        let r = rule("name: t\napply: \"package.json,*.toml\"\nseverity: error\npattern: \"x\"");
        assert_eq!(
            lint_str(std::slice::from_ref(&r), "sub/package.json", "x").len(),
            1
        );
        assert_eq!(lint_str(std::slice::from_ref(&r), "a.toml", "x").len(), 1);
        assert!(lint_str(&[r], "a.json", "x").is_empty());

        let r = rule("name: t\napply: \"readme\"\nseverity: error\npattern: \"x\"");
        assert_eq!(
            lint_str(std::slice::from_ref(&r), "README.md", "x").len(),
            1
        );
        assert!(lint_str(&[r], "notes.md", "x").is_empty());

        let r = rule("name: t\napply: \"/readme*\"\nseverity: error\npattern: \"x\"");
        assert_eq!(
            lint_str(std::slice::from_ref(&r), "README.md", "x").len(),
            1
        );
        assert!(
            lint_str(&[r], "sub/README.md", "x").is_empty(),
            "ancré à la racine"
        );
    }

    #[test]
    fn regex_message_comes_from_title_and_line_is_right() {
        let r = rule("name: t\napply: \"*\"\nseverity: warning\npattern: \"foo\"");
        let v = lint_str(&[r], "a.txt", "ok\nfoo bar\n");
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].line, 2);
        assert_eq!(v[0].message, "Message de test");
        assert_eq!(v[0].rule_id, "core/t");
    }

    #[test]
    fn inline_suppression() {
        let r = rule("name: t\napply: \"*\"\nseverity: error\npattern: \"foo\"");
        let rules = [r];
        assert!(lint_str(&rules, "a.txt", "foo // alpharust-ignore").is_empty());
        assert!(lint_str(&rules, "a.txt", "# alpharust-ignore: t\nfoo").is_empty());
        assert_eq!(
            lint_str(&rules, "a.txt", "foo // alpharust-ignore: autre-regle").len(),
            1
        );
        assert_eq!(lint_str(&rules, "a.txt", "foo\n\nfoo").len(), 2);
    }

    #[test]
    fn exclude_and_skip_dirs() {
        let r = rule("name: t\napply: \"*\"\nseverity: error\npattern: \"foo\"");
        let dir = tempfile::tempdir().unwrap();
        for p in [
            "a.txt",
            "lib/b.txt",
            "node_modules/c.txt",
            "target/d.txt",
            "vendor/e.txt",
            "dist/f.txt",
            ".astro/g.txt",
        ] {
            let path = dir.path().join(p);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, "foo").unwrap();
        }
        let all = run_with(std::slice::from_ref(&r), dir.path(), &Options::default()).unwrap();
        assert_eq!(
            all.violations.len(),
            2,
            "node_modules, target, vendor, dist et .astro sont ignorés"
        );
        let opts = Options {
            exclude: vec!["lib/**".into()],
        };
        let some = run_with(&[r], dir.path(), &opts).unwrap();
        assert_eq!(some.violations.len(), 1);
    }

    #[test]
    fn checker_rules_run_and_project_rules_run_once() {
        let img = rule("name: alt\napply: \"*.html\"\nseverity: warning\nchecker: html-img-alt");
        let v = lint_str(&[img], "p.html", "<img src=a>");
        assert_eq!(v.len(), 1);
        assert!(v[0].message.contains("Message de test —"));

        let readme = rule(
            "name: readme\napply: \"*\"\nseverity: warning\nchecker: require-glob\nargs:\n  any: [\"README*\"]\n  when_any: [\"Cargo.toml\"]",
        );
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("Cargo.toml"), "[package]").unwrap();
        assert_eq!(
            run(std::slice::from_ref(&readme), dir.path())
                .unwrap()
                .len(),
            1
        );
        fs::write(dir.path().join("README.md"), "# x").unwrap();
        assert!(run(&[readme], dir.path()).unwrap().is_empty());
    }

    #[test]
    fn missing_path_is_an_error() {
        assert!(run(&[], Path::new("/definitely/not/here")).is_err());
    }

    #[test]
    fn binary_files_are_skipped() {
        let r = rule("name: t\napply: \"*\"\nseverity: error\npattern: \"foo\"");
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("a.bin"), [0xff, 0xfe, b'f', b'o', b'o']).unwrap();
        assert!(run(&[r], dir.path()).unwrap().is_empty());
    }

    #[test]
    fn single_file_target() {
        let r = rule("name: t\napply: \"*.rs\"\nseverity: error\npattern: \"foo\"");
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path().join("x.rs");
        fs::write(&f, "foo").unwrap();
        assert_eq!(run(&[r], &f).unwrap().len(), 1);
    }
}
