//! Couche de templates appliquée **par-dessus** le projet créé par l'outil officiel.
//!
//! Les fichiers vivent dans `templates/layers/<framework>/…` (suffixe `.tpl`) et sont embarqués dans le binaire.
//!
//! Syntaxe d'un template :
//! - première ligne facultative `@@meta mode=replace when=tailwind@@` :
//!   `mode=create` (défaut : n'écrase jamais un fichier existant), `mode=replace` (écrase le fichier officiel),
//!   `when=flag` / `when=!flag` / `when=a|b` / `when=a&!b` : n'applique le fichier que si le drapeau est actif ;
//! - blocs `@@if flag@@ … @@else@@ … @@end@@` (directives sur leur propre ligne, imbriquables) ;
//! - variables `@@name@@`, `@@slug@@`, `@@pkg@@`, `@@title@@`, `@@upper@@`, `@@pm@@`, `@@run@@`, …
//!
//! Dans les chemins : un segment `dot-xxx` devient `.xxx` et `@@var@@` est substitué.

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

static LAYERS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/layers");

/// Variables et drapeaux de rendu.
#[derive(Debug, Clone, Default)]
pub struct Ctx {
    pub vars: BTreeMap<String, String>,
    pub flags: BTreeSet<String>,
}

impl Ctx {
    pub fn var(&mut self, k: &str, v: impl Into<String>) -> &mut Self {
        self.vars.insert(k.to_string(), v.into());
        self
    }
    pub fn flag(&mut self, k: &str, on: bool) -> &mut Self {
        if on {
            self.flags.insert(k.to_string());
        }
        self
    }
    pub fn has(&self, k: &str) -> bool {
        self.flags.contains(k)
    }

    /// Substitue les variables `@@x@@` (les clés inconnues sont conservées, ce qui les rend détectables).
    pub fn subst(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let mut rest = text;
        while let Some(i) = rest.find("@@") {
            out.push_str(&rest[..i]);
            let after = &rest[i + 2..];
            match after.find("@@") {
                Some(j)
                    if !after[..j].is_empty()
                        && after[..j]
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || c == '_')
                        && self.vars.contains_key(&after[..j]) =>
                {
                    out.push_str(&self.vars[&after[..j]]);
                    rest = &after[j + 2..];
                }
                _ => {
                    out.push_str("@@");
                    rest = after;
                }
            }
        }
        out.push_str(rest);
        out
    }

    /// Évalue une expression : `a|b` (OU), `a&b` (ET, prioritaire sur OU), `!a` (NON, par terme).
    pub fn holds(&self, expr: &str) -> bool {
        let expr = expr.trim();
        if expr.is_empty() {
            return true;
        }
        expr.split('|').any(|group| {
            group.split('&').all(|term| {
                let term = term.trim();
                match term.strip_prefix('!') {
                    Some(n) => !self.has(n.trim()),
                    None => self.has(term),
                }
            })
        })
    }

    /// Rend un template : blocs conditionnels puis variables.
    pub fn render(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        // pile des conditions : (branche courante active, parent actif)
        let mut stack: Vec<(bool, bool)> = Vec::new();
        let active = |stack: &Vec<(bool, bool)>| stack.iter().all(|(on, _)| *on);
        for line in text.split_inclusive('\n') {
            let t = line.trim();
            if let Some(expr) = t
                .strip_prefix("@@if ")
                .and_then(|r| r.strip_suffix("@@"))
                .filter(|e| !e.contains("@@"))
            {
                stack.push((self.holds(expr), active(&stack)));
            } else if t == "@@else@@" {
                if let Some(top) = stack.last_mut() {
                    top.0 = !top.0;
                }
            } else if t == "@@end@@" {
                stack.pop();
            } else if active(&stack) {
                out.push_str(&self.inline(line));
            }
        }
        self.subst(&out)
    }

    /// Directives en ligne `texte @@if flag@@A@@else@@B@@end@@ suite` (non imbriquables, sur une seule ligne).
    fn inline(&self, line: &str) -> String {
        let mut out = String::with_capacity(line.len());
        let mut rest = line;
        while let Some(i) = rest.find("@@if ") {
            let after = &rest[i + 5..];
            let (Some(e), true) = (after.find("@@"), after.contains("@@end@@")) else {
                break;
            };
            let expr = &after[..e];
            let body_start = &after[e + 2..];
            let Some(end) = body_start.find("@@end@@") else {
                break;
            };
            let body = &body_start[..end];
            let (yes, no) = body.split_once("@@else@@").unwrap_or((body, ""));
            out.push_str(&rest[..i]);
            out.push_str(if self.holds(expr) { yes } else { no });
            rest = &body_start[end + 7..];
        }
        out.push_str(rest);
        out
    }
}

/// Mode d'écriture d'un fichier de la couche.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Create,
    Replace,
}

#[derive(Debug, Clone)]
pub struct Meta {
    pub mode: Mode,
    pub when: String,
}

/// Sépare l'en-tête `@@meta … @@` du corps.
pub fn parse_meta(text: &str) -> (Meta, &str) {
    let mut meta = Meta {
        mode: Mode::Create,
        when: String::new(),
    };
    if let Some(rest) = text.strip_prefix("@@meta ") {
        if let Some(end) = rest.find("@@") {
            for kv in rest[..end].split_whitespace() {
                match kv.split_once('=') {
                    Some(("mode", "replace")) => meta.mode = Mode::Replace,
                    Some(("mode", _)) => meta.mode = Mode::Create,
                    Some(("when", w)) => meta.when = w.to_string(),
                    _ => {}
                }
            }
            let body = &rest[end + 2..];
            return (meta, body.strip_prefix('\n').unwrap_or(body));
        }
    }
    (meta, text)
}

/// Chemin cible : retire `.tpl`, convertit `dot-` et substitue les variables.
pub fn target_path(ctx: &Ctx, rel: &str) -> String {
    let rel = rel.strip_suffix(".tpl").unwrap_or(rel);
    let converted: Vec<String> = rel
        .split('/')
        .map(|seg| match seg.strip_prefix("dot-") {
            Some(r) => format!(".{r}"),
            None => seg.to_string(),
        })
        .collect();
    ctx.subst(&converted.join("/"))
}

fn collect<'a>(dir: &'a Dir<'a>, out: &mut Vec<(&'a str, &'a str)>) {
    for f in dir.files() {
        if let Some(body) = f.contents_utf8() {
            out.push((f.path().to_str().unwrap_or(""), body));
        }
    }
    for d in dir.dirs() {
        collect(d, out);
    }
}

/// Tous les templates d'une couche (`astro`, `laravel`, …) : (chemin relatif à la couche, contenu brut).
pub fn files(layer: &str) -> Vec<(String, &'static str)> {
    let mut all = Vec::new();
    if let Some(dir) = LAYERS.get_dir(layer) {
        collect(dir, &mut all);
    }
    let prefix = format!("{layer}/");
    let mut v: Vec<(String, &'static str)> = all
        .into_iter()
        .map(|(p, b)| (p.strip_prefix(&prefix).unwrap_or(p).to_string(), b))
        .collect();
    v.sort();
    v
}

/// Résultat de l'application d'une couche.
#[derive(Debug, Clone, Default)]
pub struct Applied {
    pub written: Vec<String>,
    pub replaced: Vec<String>,
    pub kept: Vec<String>,
}

/// Applique les couches `names` (dans l'ordre) à `target`.
pub fn apply(names: &[&str], ctx: &Ctx, target: &Path) -> Result<Applied> {
    let mut applied = Applied::default();
    for name in names {
        for (rel, raw) in files(name) {
            let (meta, body) = parse_meta(raw);
            if !ctx.holds(&meta.when) {
                continue;
            }
            let rel_out = target_path(ctx, &rel);
            let dest = target.join(&rel_out);
            let existed = dest.exists();
            if existed && meta.mode == Mode::Create {
                applied.kept.push(rel_out);
                continue;
            }
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("création de {}", parent.display()))?;
            }
            fs::write(&dest, ctx.render(body))
                .with_context(|| format!("écriture de {}", dest.display()))?;
            #[cfg(unix)]
            if rel_out.ends_with(".sh") {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(&dest, fs::Permissions::from_mode(0o755));
            }
            if existed {
                applied.replaced.push(rel_out);
            } else {
                applied.written.push(rel_out);
            }
        }
    }
    Ok(applied)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> Ctx {
        let mut c = Ctx::default();
        c.var("name", "demo").var("pm", "pnpm");
        c.flag("tailwind", true);
        c
    }

    #[test]
    fn substitutes_known_variables_only() {
        let c = ctx();
        assert_eq!(c.subst("a @@name@@ b @@pm@@"), "a demo b pnpm");
        assert_eq!(c.subst("@@inconnu@@ et @@name@@"), "@@inconnu@@ et demo");
        assert_eq!(c.subst("email@@x"), "email@@x");
    }

    #[test]
    fn conditional_blocks_nest() {
        let c = ctx();
        let t = "a\n@@if tailwind@@\nb\n@@if react@@\nc\n@@else@@\nd\n@@end@@\n@@else@@\ne\n@@end@@\nf\n";
        assert_eq!(c.render(t), "a\nb\nd\nf\n");
        let t2 = "@@if !tailwind@@\nx\n@@else@@\ny\n@@end@@\n";
        assert_eq!(c.render(t2), "y\n");
        assert!(c.holds("react|tailwind") && !c.holds("react|vue") && c.holds(""));
        assert!(
            c.holds("tailwind&!react")
                && !c.holds("tailwind&react")
                && c.holds("react|tailwind&!vue")
        );
    }

    #[test]
    fn inline_directives_work_on_a_single_line() {
        let mut c = ctx();
        c.flag("tailwind", true).flag("react", false);
        assert_eq!(c.render("a@@if tailwind@@, tw@@end@@ b\n"), "a, tw b\n");
        assert_eq!(
            c.render("a@@if react@@, r@@else@@, !r@@end@@ b @@if tailwind@@T@@end@@\n"),
            "a, !r b T\n"
        );
        assert_eq!(c.render("x@@if react@@y@@end@@z\n"), "xz\n");
        assert_eq!(c.render("sans directive\n"), "sans directive\n");
    }

    #[test]
    fn meta_header_is_parsed() {
        let (m, body) = parse_meta("@@meta mode=replace when=react|vue@@\ncorps\n");
        assert_eq!(m.mode, Mode::Replace);
        assert_eq!(m.when, "react|vue");
        assert_eq!(body, "corps\n");
        let (m, body) = parse_meta("sans meta\n");
        assert_eq!(m.mode, Mode::Create);
        assert_eq!(body, "sans meta\n");
    }

    #[test]
    fn target_paths() {
        let mut c = ctx();
        c.var("pkg", "mon_site");
        assert_eq!(
            target_path(&c, "dot-github/workflows/ci.yml.tpl"),
            ".github/workflows/ci.yml"
        );
        assert_eq!(target_path(&c, "@@pkg@@/x.py.tpl"), "mon_site/x.py");
    }

    #[test]
    fn create_mode_never_clobbers_and_replace_does() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("README.md"), "officiel").unwrap();
        fs::write(dir.path().join(".gitignore"), "officiel").unwrap();
        let c = ctx();
        let applied = apply(&["common", "astro"], &c, dir.path()).unwrap();
        assert_eq!(
            fs::read_to_string(dir.path().join(".gitignore")).unwrap(),
            "officiel",
            "mode create : le fichier officiel est conservé"
        );
        assert!(
            applied.kept.iter().any(|f| f == ".gitignore"),
            "{applied:?}"
        );
        assert_ne!(
            fs::read_to_string(dir.path().join("README.md")).unwrap(),
            "officiel",
            "mode replace : le fichier officiel est remplacé"
        );
        assert!(
            applied.written.iter().any(|f| f == ".alpharust.toml"),
            "{applied:?}"
        );
    }

    #[test]
    fn every_layer_template_has_valid_syntax() {
        // les blocs @@if/@@end doivent être équilibrés dans tous les templates embarqués
        for layer in ["common", "astro", "laravel", "wagtail", "rust"] {
            for (path, raw) in files(layer) {
                let ifs = raw
                    .lines()
                    .filter(|l| l.trim().starts_with("@@if ") && !l.contains("@@end@@"))
                    .count();
                let ends = raw.lines().filter(|l| l.trim() == "@@end@@").count();
                assert_eq!(ifs, ends, "blocs déséquilibrés dans {layer}/{path}");
            }
        }
    }
}
