use super::{cached_regex, line_of, FileCtx, Finding};

/// Si `args.trigger` (regex) apparaît dans le fichier, `args.require` (regex) doit
/// aussi apparaître quelque part dans ce fichier. Constat posé sur la première
/// occurrence du déclencheur. Exemple : un webhook sans vérification de signature.
pub fn requires_companion(ctx: &FileCtx) -> Vec<Finding> {
    let (Some(trigger), Some(require)) = (ctx.args.string("trigger"), ctx.args.string("require"))
    else {
        return Vec::new();
    };
    let (Some(trigger), Some(require)) = (cached_regex(&trigger), cached_regex(&require)) else {
        return Vec::new();
    };
    let Some(m) = trigger.find(ctx.content) else {
        return Vec::new();
    };
    if require.is_match(ctx.content) {
        return Vec::new();
    }
    let message = ctx
        .args
        .string("message")
        .unwrap_or_else(|| format!("`{}` présent sans `{}`", trigger.as_str(), require.as_str()));
    vec![Finding::new(line_of(ctx.content, m.start()), message)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::Args;
    use std::collections::BTreeMap;
    use std::path::Path;

    #[test]
    fn trigger_without_companion_is_flagged() {
        let mut args = BTreeMap::new();
        args.insert("trigger".into(), serde_json::json!("(?i)webhook"));
        args.insert("require".into(), serde_json::json!("(?i)signature"));
        let run = |c: &str| {
            requires_companion(&FileCtx {
                path: Path::new("a.js"),
                content: c,
                args: Args(&args),
            })
        };
        let f = run("ok\napp.post('/webhook', h)");
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].line, 2);
        assert!(run("webhook + verifySignature()").is_empty());
        assert!(run("rien à signaler").is_empty());
    }
}
