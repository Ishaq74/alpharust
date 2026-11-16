//! Lecture des règles (Markdown + frontmatter YAML) et cache sur disque.
pub mod cache;
pub mod rule;

pub use rule::{Examples, LoadReport, Rule, SEVERITIES};
