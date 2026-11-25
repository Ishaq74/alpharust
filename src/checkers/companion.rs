use super::{cached_regex, line_of, FileCtx, Finding};
use std::path::Path;

/// Si `args.trigger` (regex) apparaît dans le fichier, `args.require` (regex) doit
/// aussi apparaître quelque part dans ce fichier. Constat posé sur la première
/// occurrence du déclencheur. Exemple : un webhook sans vérification de signature.
///
/// Exemption par configuration du projet : pour les extensions `args.exempt_ext`, aucun constat
/// si un fichier `args.exempt_files` d'un dossier parent contient `args.exempt_pattern`
/// (exemple : `security.checkOrigin` dans `astro.config.mjs` protège les POST des pages `.astro`).
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
    if require.is_match(ctx.content) || exempted_by_config(ctx) {
        return Vec::new();
    }
    let message = ctx
        .args
        .string("message")
        .unwrap_or_else(|| format!("`{}` présent sans `{}`", trigger.as_str(), require.as_str()));
    vec![Finding::new(line_of(ctx.content, m.start()), message)]
}

fn exempted_by_config(ctx: &FileCtx) -> bool {
    let exts = ctx.args.strings("exempt_ext");
    let ext = ctx.path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if !exts.iter().any(|e| e == ext) {
        return false;
    }
    let Some(pattern) = ctx
        .args
        .string("exempt_pattern")
        .and_then(|p| cached_regex(&p))
    else {
        return false;
    };
    let files = ctx.args.strings("exempt_files");
    let mut dir = ctx.path.parent();
    while let Some(d) = dir {
        let base = if d.as_os_str().is_empty() {
            Path::new(".")
        } else {
            d
        };
        for f in &files {
            if let Ok(text) = std::fs::read_to_string(base.join(f)) {
                return pattern.is_match(&text);
            }
        }
        dir = d.parent();
    }
    false
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

    #[test]
    fn astro_check_origin_exempts_astro_forms_only() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src/pages")).unwrap();
        let mut args = BTreeMap::new();
        args.insert(
            "trigger".into(),
            serde_json::json!("(?i)<form[^>]*method=\"post"),
        );
        args.insert("require".into(), serde_json::json!("(?i)csrf"));
        args.insert("exempt_ext".into(), serde_json::json!(["astro"]));
        args.insert(
            "exempt_files".into(),
            serde_json::json!(["astro.config.mjs"]),
        );
        args.insert(
            "exempt_pattern".into(),
            serde_json::json!("checkOrigin:\\s*true"),
        );
        let form = "<form method=\"post\"></form>";
        let run = |name: &str| {
            let path = dir.path().join("src/pages").join(name);
            requires_companion(&FileCtx {
                path: &path,
                content: form,
                args: Args(&args),
            })
            .len()
        };
        assert_eq!(run("contact.astro"), 1, "sans configuration : signalé");
        std::fs::write(
            dir.path().join("astro.config.mjs"),
            "export default defineConfig({ security: { checkOrigin: true } });",
        )
        .unwrap();
        assert_eq!(run("contact.astro"), 0, "checkOrigin : exempté");
        assert_eq!(run("legacy.html"), 1, "autre extension : toujours signalé");
        std::fs::write(
            dir.path().join("astro.config.mjs"),
            "export default defineConfig({ security: { checkOrigin: false } });",
        )
        .unwrap();
        assert_eq!(run("contact.astro"), 1, "checkOrigin désactivé : signalé");
    }
}
