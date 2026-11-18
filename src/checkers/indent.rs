use super::{FileCtx, Finding};

/// Indentation mixte : lignes indentées par tabulation *et* par espaces (≥ 2).
/// Les lignes de continuation de commentaires (` * ...`) sont ignorées.
pub fn consistency(ctx: &FileCtx) -> Vec<Finding> {
    let mut first_tab = None;
    let mut first_space = None;
    let (mut tabs, mut spaces) = (0usize, 0usize);
    for (i, line) in ctx.content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with('\t') {
            tabs += 1;
            first_tab.get_or_insert(i + 1);
        } else if line.starts_with("  ") && !line.trim_start().starts_with('*') {
            spaces += 1;
            first_space.get_or_insert(i + 1);
        }
    }
    if tabs == 0 || spaces == 0 {
        return Vec::new();
    }
    let (line, minority) = if tabs < spaces {
        (first_tab.unwrap_or(1), "tabulations")
    } else {
        (first_space.unwrap_or(1), "espaces")
    };
    vec![Finding::new(
        line,
        format!(
            "indentation mixte ({} lignes à tabulations, {} à espaces) : première ligne en {}",
            tabs, spaces, minority
        ),
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::Args;
    use std::collections::BTreeMap;
    use std::path::Path;

    fn run(content: &str) -> Vec<Finding> {
        let args = BTreeMap::new();
        consistency(&FileCtx {
            path: Path::new("a.js"),
            content,
            args: Args(&args),
        })
    }

    #[test]
    fn detects_mixed_indentation() {
        let f = run("a\n  b\n  c\n\td\n");
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].line, 4);
    }

    #[test]
    fn uniform_files_pass() {
        assert!(run("a\n\tb\n\tc\n").is_empty());
        assert!(run("a\n  b\n    c\n").is_empty());
        assert!(run("/*\n * doc\n */\n\tx\n").is_empty());
    }
}
