//! Checkers sémantiques : analyses plus fines que la simple regex ligne à ligne.
//!
//! Un règle peut déclarer `checker: <nom>` (et `args:`) à la place de `pattern:`.
//! Deux familles :
//!
//! * **fichier** — reçoit le contenu d'un fichier et renvoie des constats
//!   (analyse de balises HTML, titres Markdown, lexer Rust, contraste CSS, manifestes de dépendances…) ;
//! * **projet** — reçoit la liste des fichiers du projet (présence d'un README, de tests, de migrations…).

use regex::Regex;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};

pub mod companion;
pub mod css;
pub mod deps;
pub mod html;
pub mod indent;
pub mod markdown;
pub mod project;
pub mod rust;

/// Un constat produit par un checker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub line: usize,
    pub message: String,
}

impl Finding {
    pub fn new(line: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

/// Accès typé aux `args:` d'une règle.
#[derive(Debug, Clone, Copy)]
pub struct Args<'a>(pub &'a BTreeMap<String, Value>);

impl Args<'_> {
    pub fn string(&self, key: &str) -> Option<String> {
        self.0.get(key).and_then(|v| v.as_str()).map(str::to_string)
    }

    /// Accepte une chaîne seule ou une liste de chaînes.
    pub fn strings(&self, key: &str) -> Vec<String> {
        match self.0.get(key) {
            Some(Value::String(s)) => vec![s.clone()],
            Some(Value::Array(a)) => a
                .iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect(),
            _ => Vec::new(),
        }
    }
}

pub struct FileCtx<'a> {
    pub path: &'a Path,
    pub content: &'a str,
    pub args: Args<'a>,
}

pub struct ProjectCtx<'a> {
    /// Racine analysée.
    pub root: &'a Path,
    /// Fichiers du projet, chemins relatifs à la racine, séparateur `/`.
    pub files: &'a [String],
    pub args: Args<'a>,
}

pub type FileFn = fn(&FileCtx) -> Vec<Finding>;
pub type ProjectFn = fn(&ProjectCtx) -> Vec<Finding>;

#[derive(Clone, Copy)]
pub enum Checker {
    File(FileFn),
    Project(ProjectFn),
}

/// Registre (nom, description, implémentation).
const REGISTRY: &[(&str, &str, Checker)] = &[
    (
        "html-img-alt",
        "balises <img> sans attribut alt (analyse de balises, multi-lignes)",
        Checker::File(html::img_alt),
    ),
    (
        "html-lang",
        "<html> sans attribut lang",
        Checker::File(html::html_lang),
    ),
    (
        "html-heading-order",
        "hiérarchie h1..h6 : un seul h1, pas de niveau sauté",
        Checker::File(html::heading_order),
    ),
    (
        "html-semantic",
        "page avec <body> mais sans balise sémantique (main/header/nav/footer…)",
        Checker::File(html::semantic),
    ),
    (
        "markdown-headings",
        "hiérarchie des titres Markdown (blocs de code ignorés)",
        Checker::File(markdown::headings),
    ),
    (
        "markdown-title",
        "le premier titre d'un document Markdown doit être un H1 unique",
        Checker::File(markdown::title),
    ),
    (
        "markdown-frontmatter",
        "frontmatter YAML présent doit être fermé et parsable",
        Checker::File(markdown::frontmatter),
    ),
    (
        "readme-sections",
        "README contenant une section (args.any = mots-clés de titre)",
        Checker::File(markdown::readme_sections),
    ),
    (
        "css-contrast",
        "contraste WCAG AA (4.5:1) entre color et background d'un même bloc",
        Checker::File(css::contrast),
    ),
    (
        "rust-unsafe",
        "mot-clé `unsafe` hors commentaires et chaînes (lexer Rust léger)",
        Checker::File(rust::unsafe_usage),
    ),
    (
        "indent-consistency",
        "indentation mixte tabulations / espaces dans un même fichier",
        Checker::File(indent::consistency),
    ),
    (
        "deps-consistency",
        "manifestes de dépendances : versions joker, doublons, versions divergentes",
        Checker::File(deps::consistency),
    ),
    (
        "requires-companion",
        "si `trigger` apparaît, `require` doit aussi apparaître dans le fichier",
        Checker::File(companion::requires_companion),
    ),
    (
        "require-glob",
        "projet : au moins un fichier doit correspondre à args.any",
        Checker::Project(project::require_glob),
    ),
    (
        "file-contains",
        "projet : un fichier (args.file) doit contenir une des regex args.any",
        Checker::Project(project::file_contains),
    ),
];

pub fn lookup(name: &str) -> Option<Checker> {
    REGISTRY
        .iter()
        .find(|(n, _, _)| *n == name)
        .map(|(_, _, c)| *c)
}

pub fn is_known(name: &str) -> bool {
    lookup(name).is_some()
}

pub fn names() -> Vec<&'static str> {
    REGISTRY.iter().map(|(n, _, _)| *n).collect()
}

/// (nom, description) de chaque checker, pour la documentation et l'interface.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    REGISTRY.iter().map(|(n, d, _)| (*n, *d)).collect()
}

/// Compile une regex avec cache global (les checkers sont appelés fichier par fichier).
pub fn cached_regex(pattern: &str) -> Option<Arc<Regex>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Option<Arc<Regex>>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = cache.lock().ok()?;
    guard
        .entry(pattern.to_string())
        .or_insert_with(|| Regex::new(pattern).ok().map(Arc::new))
        .clone()
}

/// Numéro de ligne (1-indexé) d'un offset d'octet.
pub fn line_of(content: &str, offset: usize) -> usize {
    content.as_bytes()[..offset.min(content.len())]
        .iter()
        .filter(|b| **b == b'\n')
        .count()
        + 1
}
