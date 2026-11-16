//! Checkers « projet » : s'exécutent une fois sur la racine analysée.

use super::{cached_regex, Finding, ProjectCtx};
use crate::glob::glob_match;

fn any_match(files: &[String], globs: &[String]) -> bool {
    files.iter().any(|f| globs.iter().any(|g| glob_match(g, f)))
}

/// Le projet est-il concerné ? (`when_any` : au moins un de ces fichiers existe ;
/// `when_contains` : [glob, regex] — un fichier correspondant contient la regex.)
fn applies(ctx: &ProjectCtx) -> bool {
    let markers = ctx.args.strings("when_any");
    if !markers.is_empty() && !any_match(ctx.files, &markers) {
        return false;
    }
    let cond = ctx.args.strings("when_contains");
    if cond.len() == 2 {
        let Some(re) = cached_regex(&cond[1]) else {
            return false;
        };
        let hit = ctx
            .files
            .iter()
            .filter(|f| glob_match(&cond[0], f))
            .any(|f| std::fs::read_to_string(ctx.root.join(f)).is_ok_and(|c| re.is_match(&c)));
        if !hit {
            return false;
        }
    }
    true
}

fn message(ctx: &ProjectCtx, default: &str) -> String {
    ctx.args
        .string("message")
        .unwrap_or_else(|| default.to_string())
}

pub fn require_glob(ctx: &ProjectCtx) -> Vec<Finding> {
    let wanted = ctx.args.strings("any");
    if wanted.is_empty() || !applies(ctx) || any_match(ctx.files, &wanted) {
        return Vec::new();
    }
    vec![Finding::new(
        1,
        message(
            ctx,
            &format!("aucun fichier ne correspond à : {}", wanted.join(", ")),
        ),
    )]
}

pub fn file_contains(ctx: &ProjectCtx) -> Vec<Finding> {
    let Some(file) = ctx.args.string("file") else {
        return Vec::new();
    };
    let regexes: Vec<_> = ctx
        .args
        .strings("any")
        .iter()
        .filter_map(|p| cached_regex(p))
        .collect();
    if regexes.is_empty() || !applies(ctx) {
        return Vec::new();
    }
    let matching: Vec<&String> = ctx.files.iter().filter(|f| glob_match(&file, f)).collect();
    if matching.is_empty() {
        return vec![Finding::new(
            1,
            message(ctx, &format!("fichier `{}` absent", file)),
        )];
    }
    let ok = matching.iter().any(|f| {
        std::fs::read_to_string(ctx.root.join(f))
            .is_ok_and(|c| regexes.iter().any(|re| re.is_match(&c)))
    });
    if ok {
        Vec::new()
    } else {
        vec![Finding::new(
            1,
            message(
                ctx,
                &format!("`{}` ne contient pas l'entrée attendue", file),
            ),
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::Args;
    use std::collections::BTreeMap;
    use std::path::Path;

    fn args(pairs: &[(&str, serde_json::Value)]) -> BTreeMap<String, serde_json::Value> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    fn files(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn require_glob_respects_markers() {
        let a = args(&[
            ("any", serde_json::json!(["README*"])),
            ("when_any", serde_json::json!(["Cargo.toml"])),
        ]);
        let run = |f: &[&str]| {
            let fs = files(f);
            require_glob(&ProjectCtx {
                root: Path::new("."),
                files: &fs,
                args: Args(&a),
            })
        };
        assert_eq!(run(&["Cargo.toml", "src/main.rs"]).len(), 1);
        assert!(run(&["Cargo.toml", "README.md"]).is_empty());
        assert!(run(&["notes.txt"]).is_empty(), "pas un projet → ignoré");
    }

    #[test]
    fn file_contains_reads_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(".gitignore"), "target\n.venv/\n").unwrap();
        let a = args(&[
            ("file", serde_json::json!(".gitignore")),
            ("any", serde_json::json!(["(?m)^\\.?venv/?$"])),
        ]);
        let fs = files(&[".gitignore"]);
        let ctx = ProjectCtx {
            root: dir.path(),
            files: &fs,
            args: Args(&a),
        };
        assert!(file_contains(&ctx).is_empty());
        std::fs::write(dir.path().join(".gitignore"), "target\n").unwrap();
        assert_eq!(file_contains(&ctx).len(), 1);
    }
}
