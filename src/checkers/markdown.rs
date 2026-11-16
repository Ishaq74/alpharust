use super::{FileCtx, Finding};

#[derive(Debug, Clone)]
pub struct Heading {
    pub level: usize,
    pub text: String,
    pub line: usize,
}

/// Retire un éventuel frontmatter et renvoie (corps, nombre de lignes retirées).
fn strip_frontmatter(content: &str) -> (&str, usize) {
    if let Some(rest) = content.strip_prefix("---\n") {
        if let Some(end) = rest.find("\n---") {
            let consumed = 4 + end + 4;
            let consumed = consumed.min(content.len());
            let lines = content[..consumed].matches('\n').count();
            return (&content[consumed..], lines);
        }
    }
    (content, 0)
}

/// Titres ATX (`# Titre`), hors blocs de code clôturés.
pub fn parse_headings(content: &str) -> Vec<Heading> {
    let (body, offset) = strip_frontmatter(content);
    let mut out = Vec::new();
    let mut fence: Option<String> = None;
    for (i, line) in body.lines().enumerate() {
        let trimmed = line.trim_start();
        if let Some(f) = &fence {
            if trimmed.starts_with(f.as_str()) {
                fence = None;
            }
            continue;
        }
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            fence = Some(trimmed[..3].to_string());
            continue;
        }
        if line.len() - trimmed.len() > 3 {
            continue; // bloc de code indenté
        }
        let level = trimmed.chars().take_while(|c| *c == '#').count();
        if (1..=6).contains(&level) && trimmed.chars().nth(level).is_none_or(|c| c.is_whitespace())
        {
            out.push(Heading {
                level,
                text: trimmed[level..]
                    .trim()
                    .trim_end_matches('#')
                    .trim()
                    .to_string(),
                line: offset + i + 1,
            });
        }
    }
    out
}

pub fn headings(ctx: &FileCtx) -> Vec<Finding> {
    let hs = parse_headings(ctx.content);
    let mut out = Vec::new();
    let mut prev = 0;
    for h in &hs {
        if prev != 0 && h.level > prev + 1 {
            out.push(Finding::new(
                h.line,
                format!("niveau de titre sauté : H{} après H{}", h.level, prev),
            ));
        }
        prev = h.level;
    }
    out
}

pub fn title(ctx: &FileCtx) -> Vec<Finding> {
    let hs = parse_headings(ctx.content);
    let mut out = Vec::new();
    match hs.first() {
        None => out.push(Finding::new(
            1,
            "aucun titre : le document doit commencer par un H1",
        )),
        Some(h) if h.level != 1 => out.push(Finding::new(
            h.line,
            format!("le premier titre est un H{} (H1 attendu)", h.level),
        )),
        _ => {}
    }
    for h in hs.iter().filter(|h| h.level == 1).skip(1) {
        out.push(Finding::new(h.line, "plusieurs titres H1"));
    }
    out
}

pub fn frontmatter(ctx: &FileCtx) -> Vec<Finding> {
    let content = ctx.content.trim_start_matches('\u{feff}');
    let Some(rest) = content
        .strip_prefix("---\n")
        .or_else(|| content.strip_prefix("---\r\n"))
    else {
        return Vec::new();
    };
    let Some(end) = rest.find("\n---") else {
        return vec![Finding::new(
            1,
            "frontmatter non fermé (`---` final manquant)",
        )];
    };
    match serde_yaml::from_str::<serde_yaml::Value>(&rest[..end]) {
        Ok(serde_yaml::Value::Mapping(_)) => Vec::new(),
        Ok(serde_yaml::Value::Null) => vec![Finding::new(1, "frontmatter vide")],
        Ok(_) => vec![Finding::new(1, "le frontmatter doit être un mapping YAML")],
        Err(e) => {
            let line = e.location().map_or(1, |l| l.line() + 1);
            vec![Finding::new(
                line,
                format!("frontmatter YAML invalide : {}", e),
            )]
        }
    }
}

/// README : doit contenir un titre incluant l'un des mots de `args.any`.
pub fn readme_sections(ctx: &FileCtx) -> Vec<Finding> {
    let wanted: Vec<String> = ctx
        .args
        .strings("any")
        .iter()
        .map(|s| s.to_lowercase())
        .collect();
    if wanted.is_empty() {
        return Vec::new();
    }
    let ok = parse_headings(ctx.content)
        .iter()
        .any(|h| wanted.iter().any(|w| h.text.to_lowercase().contains(w)));
    if ok {
        return Vec::new();
    }
    let msg = ctx.args.string("message").unwrap_or_else(|| {
        format!(
            "section manquante (attendu un titre contenant : {})",
            wanted.join(" / ")
        )
    });
    vec![Finding::new(1, msg)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::Args;
    use std::collections::BTreeMap;
    use std::path::Path;

    fn run(
        f: fn(&FileCtx) -> Vec<Finding>,
        content: &str,
        args: BTreeMap<String, serde_json::Value>,
    ) -> Vec<Finding> {
        f(&FileCtx {
            path: Path::new("README.md"),
            content,
            args: Args(&args),
        })
    }

    #[test]
    fn headings_ignore_code_blocks() {
        let md = "# A\n```\n### not a heading\n```\n## B\n";
        assert!(run(headings, md, BTreeMap::new()).is_empty());
        let skip = "# A\n### C\n";
        let f = run(headings, skip, BTreeMap::new());
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].line, 2);
    }

    #[test]
    fn headings_line_numbers_account_for_frontmatter() {
        let md = "---\nname: x\n---\n# A\n### C\n";
        let f = run(headings, md, BTreeMap::new());
        assert_eq!(f[0].line, 5);
    }

    #[test]
    fn title_rules() {
        assert!(run(title, "# T\n## S\n", BTreeMap::new()).is_empty());
        assert_eq!(run(title, "## S\n", BTreeMap::new()).len(), 1);
        assert_eq!(run(title, "# A\n# B\n", BTreeMap::new()).len(), 1);
        assert_eq!(run(title, "texte seul", BTreeMap::new()).len(), 1);
    }

    #[test]
    fn frontmatter_validation() {
        assert!(run(frontmatter, "# pas de fm", BTreeMap::new()).is_empty());
        assert!(run(frontmatter, "---\na: 1\n---\n# x", BTreeMap::new()).is_empty());
        assert_eq!(run(frontmatter, "---\na: 1\n# x", BTreeMap::new()).len(), 1);
        assert_eq!(
            run(frontmatter, "---\na: [1\n---\n", BTreeMap::new()).len(),
            1
        );
    }

    #[test]
    fn readme_sections_matches_case_insensitively() {
        let mut args = BTreeMap::new();
        args.insert("any".to_string(), serde_json::json!(["install"]));
        assert!(run(readme_sections, "# X\n## Installation\n", args.clone()).is_empty());
        assert_eq!(run(readme_sections, "# X\n## Usage\n", args).len(), 1);
    }
}
